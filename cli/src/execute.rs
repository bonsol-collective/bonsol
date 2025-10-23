use crate::common::*;
use anyhow::Result;
use bonsol_prover::input_resolver::{DefaultInputResolver, InputResolver, ProgramInput};
use bonsol_sdk::instructions::{ExecutionConfig, InputRef};
use bonsol_sdk::{BonsolClient, ExecutionAccountStatus, InputT, InputType};
use hex;
use indicatif::ProgressBar;
use sha2::{Digest, Sha256};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::bs58;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::fs::File;
use std::sync::Arc;
use tokio::time::Instant;

pub async fn execution_waiter(
    sdk: &BonsolClient,
    requester: Pubkey,
    execution_id: String,
    expiry: u64,
    timeout: Option<u64>,
) -> Result<()> {
    let indicator = ProgressBar::new_spinner();

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
    let now = Instant::now();
    loop {
        if let Some(timeout) = timeout {
            if now.elapsed().as_secs() > timeout {
                return Err(anyhow::anyhow!("Timeout"));
            }
        }
        interval.tick().await;

        let current_block = sdk.get_current_slot().await?;
        indicator.set_message(format!(
            "Waiting for execution to be claimed, current block {} expiry {}",
            current_block, expiry
        ));
        if current_block > expiry {
            indicator.finish_with_message("Execution expired");
            return Err(anyhow::anyhow!("Execution expired"));
        }

        let claim_state = sdk.get_claim_state_v1(&requester, &execution_id).await;
        if let Ok(claim_state) = claim_state {
            let claim = claim_state.claim()?;
            indicator.finish_with_message(format!(
                "Claimed by {} at slot {}, committed {}",
                bs58::encode(claim.claimer).into_string(),
                claim.claimed_at,
                claim.block_commitment
            ));
            break;
        }
    }
    //now we are looking for execution request finished
    loop {
        if let Some(timeout) = timeout {
            if now.elapsed().as_secs() > timeout {
                indicator.finish_with_message("Execution timed out");
                return Err(anyhow::anyhow!("Timeout"));
            }
        }
        interval.tick().await;
        let exec_status = sdk
            .get_execution_request_v1(&requester, &execution_id)
            .await?;
        match exec_status {
            ExecutionAccountStatus::Completed(ec) => {
                indicator.finish_with_message(format!("Execution completed with exit code {}", ec));
                return Ok(());
            }
            ExecutionAccountStatus::Pending(_) => {
                indicator.tick();
                continue;
            }
        }
    }
}

pub async fn execute(
    sdk: &BonsolClient,
    rpc_url: String,
    keypair: impl Signer,
    execution_request_file: Option<String>,
    image_id: Option<String>,
    execution_id: Option<String>,
    timeout: Option<u64>,
    inputs_file: Option<String>,
    tip: Option<u64>,
    expiry: Option<u64>,
    stdin: Option<String>,
    wait: bool,
) -> Result<()> {
    let indicator = ProgressBar::new_spinner();
    let erstr =
        execution_request_file.ok_or(anyhow::anyhow!("Execution request file not provided"))?;
    let erfile = File::open(erstr)?;
    let execution_request_file: ExecutionRequestFile = serde_json::from_reader(erfile)?;
    let inputs = if let Some(inputs) = execution_request_file.inputs {
        inputs
    } else {
        execute_get_inputs(inputs_file, stdin)?
    };
    let execution_id = execution_id
        .or(execution_request_file.execution_id)
        .or(Some(rand_id(8)))
        .ok_or(anyhow::anyhow!("Execution id not provided"))?;
    let image_id = image_id
        .or(execution_request_file.image_id)
        .ok_or(anyhow::anyhow!("Image id not provided"))?;
    let tip = tip
        .or(execution_request_file.tip)
        .ok_or(anyhow::anyhow!("Tip not provided"))?;
    let expiry = expiry
        .or(execution_request_file.expiry)
        .ok_or(anyhow::anyhow!("Expiry not provided"))?;
    let callback_config = execution_request_file.callback_config;
    let mut input_hash =
        if let Some(input_hash) = execution_request_file.execution_config.input_hash {
            hex::decode(&input_hash)
                .map_err(|_| anyhow::anyhow!("Invalid input hash, must be hex encoded"))?
        } else {
            vec![]
        };

    let signer = keypair.pubkey();
    let transformed_cli_inputs = execute_transform_cli_inputs(inputs)?;

    // Declare data storage that will live long enough
    let single_concatenated_input_data: Option<Vec<u8>> = if transformed_cli_inputs
        .iter()
        .all(|i| i.input_type == InputType::PublicData)
        && !transformed_cli_inputs.is_empty()
    {
        let mut concatenated_bytes: Vec<u8> = Vec::new();
        for input_t_val in &transformed_cli_inputs {
            if let Some(bytes) = &input_t_val.data {
                // Assuming InputT { input_type: InputType, data: Option<Vec<u8>> }
                concatenated_bytes.extend_from_slice(bytes);
            }
        }
        if !concatenated_bytes.is_empty() {
            Some(concatenated_bytes)
        } else {
            None // Or Some(Vec::new()) if an empty input ref is desired for empty concatenation
        }
    } else {
        None
    };

    let final_input_refs: Vec<InputRef> =
        if let Some(ref data_bytes) = single_concatenated_input_data {
            // We have concatenated data, create a single InputRef
            // Ensure data_bytes is not empty before creating InputRef if that's a requirement
            if data_bytes.is_empty() && !transformed_cli_inputs.is_empty() {
                // This handles the case where concatenation was attempted but resulted in empty (e.g. all inputs had None data)
                // Depending on desired behavior, could be an empty Vec or an error.
                // For now, if concatenation yields empty but there WERE inputs, maybe it should be an empty Vec<InputRef>
                // or an error. If it must be one input ref, an empty one might be InputRef::new(InputType::PublicData, &[])
                // Sticking to one ref if single_concatenated_input_data is Some:
                vec![InputRef::new(InputType::PublicData, data_bytes.as_slice())]
            } else if !data_bytes.is_empty() {
                vec![InputRef::new(InputType::PublicData, data_bytes.as_slice())]
            } else {
                // single_concatenated_input_data was Some(empty_vec) and transformed_cli_inputs was empty
                vec![]
            }
        } else {
            // No concatenation, or concatenation resulted in None (e.g. no public data inputs, or mixed inputs)
            // Use original logic for multiple inputs
            transformed_cli_inputs
                .iter()
                .map(|input_t_val| {
                    InputRef::new(
                        input_t_val.input_type,
                        input_t_val.data.as_deref().unwrap_or_default(),
                    )
                })
                .collect()
        };

    // The verify_input_hash logic was here, it might need adjustment if it relied on the structure of transformed_inputs
    // For now, let's assume it's handled or less critical than the main execution flow.
    // Re-inserting the verify_input_hash logic based on transformed_cli_inputs (pre-concatenation)
    let verify_input_hash = execution_request_file
        .execution_config
        .verify_input_hash
        .unwrap_or(false);

    // Hash inputs logic needs to operate on Vec<InputT> (transformed_cli_inputs)
    // because DefaultInputResolver::resolve_public_inputs expects something compatible with ProgramInputType.
    // The current transformed_cli_inputs IS Vec<InputT> (from bonsol_sdk which is ProgramInputType<Vec<u8>, Option<String>>)
    // So this part should be fine if resolve_public_inputs can take Vec<InputT>
    let hash_inputs = verify_input_hash
        && transformed_cli_inputs
            .iter()
            .all(|i| i.input_type != InputType::Private);

    if hash_inputs {
        // ... (hashing logic as it was, using transformed_cli_inputs) ...
        // This part needs to be correct based on what resolve_public_inputs expects.
        // If it expects Vec<ProgramInputType<Vec<u8>, Option<String>>>, then transformed_cli_inputs is fine.
        indicator.set_message("Getting/Hashing inputs");
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            rpc_url.clone(),
            CommitmentConfig::confirmed(),
        ));
        let input_resolver =
            DefaultInputResolver::new(Arc::new(reqwest::Client::new()), rpc_client);

        // THIS CLONE IS IMPORTANT for ProgramInputType if it's not Copy.
        // Use InputT alias which is bonsol_sdk::ProgramInputType<Vec<u8>, Option<String>>
        let resolvable_inputs_for_hashing: Vec<InputT> = transformed_cli_inputs
            .iter()
            .map(
                |i_t|
            // Assuming i_t is ProgramInputType<Vec<u8>, Option<String>> from bonsol_sdk::InputT
            // We need to ensure it's cloned correctly if resolve_public_inputs takes ownership or modifies.
            // Or if ProgramInputType itself is not directly cloneable in this form for that function.
            // Let's assume a direct clone works for now, or that resolve_public_inputs takes references.
            i_t.clone(), // This requires InputT to be Clone.
            )
            .collect();

        let hashing_inputs = input_resolver
            .resolve_public_inputs(resolvable_inputs_for_hashing)
            .await?;
        let mut hash_obj = Sha256::new(); // Renamed from 'hash' to avoid conflict if input_hash is in scope
        for prog_input in hashing_inputs {
            if let ProgramInput::Resolved(ri) = prog_input {
                hash_obj.update(&ri.data);
            } else {
                return Err(anyhow::anyhow!("Unresolved input during hashing"));
            }
        }
        input_hash = hash_obj.finalize().to_vec();
    }

    let execution_config = ExecutionConfig {
        verify_input_hash: execution_request_file
            .execution_config
            .verify_input_hash
            .unwrap_or(false),
        input_hash: Some(&input_hash),
        forward_output: execution_request_file
            .execution_config
            .forward_output
            .unwrap_or(false),
    };
    let current_block = sdk.get_current_slot().await?;
    let expiry = expiry + current_block;
    println!("Execution expiry {}", expiry);
    println!("current block {}", current_block);
    indicator.set_message("Building transaction");
    let ixs = sdk
        .execute_v1(
            &signer,
            &image_id,
            &execution_id,
            final_input_refs,
            tip,
            expiry,
            execution_config,
            callback_config.map(|c| c.into()),
            None, // A future cli change can implement prover version selection
            vec![],
        )
        .await?;
    indicator.finish_with_message("Sending transaction");
    sdk.send_txn_standard(&keypair, ixs).await?;
    indicator.finish_with_message("Waiting for execution");
    if wait {
        execution_waiter(sdk, keypair.pubkey(), execution_id, expiry, timeout).await?;
    }
    Ok(())
}
