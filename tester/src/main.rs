use std::str::FromStr;

use anyhow::Result;

use rand::distributions::Alphanumeric;
use rand::Rng;

use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::config::RpcSendTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::hashv;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::{v0, VersionedMessage};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_sdk::transaction::VersionedTransaction;

use bonsol_sdk::instructions::{CallbackConfig, ExecutionConfig, InputRef};
use bonsol_sdk::{deployment_address, execution_address, BonsolClient, ExitCode, InputType};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "http://127.0.0.1:8899".to_string();
    let rpc_client = RpcClient::new(rpc_url.clone());
    let bonsol_client = BonsolClient::new(rpc_url);
    let signer = Keypair::new();
    let args: Vec<String> = env::args().collect();
    rpc_client
        .request_airdrop(&signer.pubkey(), 100_000_000_000)
        .await?;
    let timeout = args.get(1).map(|s| s.parse::<u64>().unwrap()).unwrap_or(60);
    example_bonsol_program_test(&bonsol_client, &rpc_client, &signer, timeout).await?;
    example_sdk_test(&bonsol_client, &rpc_client, &signer, timeout).await?;
    example_sdk_no_callback_test(&bonsol_client, &rpc_client, &signer, timeout).await?;
    Ok(())
}

const SIMPLE_IMAGE_ID: &str = "68f4b0c5f9ce034aa60ceb264a18d6c410a3af68fafd931bcfd9ebe7c1e42960";

async fn example_sdk_no_callback_test(
    bonsol_client: &BonsolClient,
    client: &RpcClient,
    signer: &dyn Signer,
    timeout: u64,
) -> Result<()> {
    println!("Running sdk test, no callback");
    let expiration: u64 = 10000000000;
    let execution_id = rand_id(16);
    let input_1 = "{\"attestation\":\"test\"}";
    let input_2 = "https://echoserver.dev/server?response=N4IgFgpghgJhBOBnEAuA2mkBjA9gOwBcJCBaAgTwAcIQAaEIgDwIHpKAbKASzxAF0+9AEY4Y5VKArVUDCMzogYUAlBlFEBEAF96G5QFdkKAEwAGU1qA";
    let input_hash = hashv(&[input_1.as_bytes(), input_2.as_bytes()]);
    println!("Execution expiry {}", expiration);
    let slot = bonsol_client.get_current_slot().await?;
    let ixs = bonsol_client
        .execute_v1(
            &signer.pubkey(),
            SIMPLE_IMAGE_ID,
            &execution_id,
            vec![
                InputRef::new(InputType::PublicData, input_1.as_bytes()),
                InputRef::new(InputType::Private, input_2.as_bytes()),
            ],
            10000,
            slot + expiration,
            ExecutionConfig {
                verify_input_hash: true,
                input_hash: Some(input_hash.as_ref()),
                forward_output: true,
            },
            None,
            None,
        )
        .await?;
    let bh = client.get_latest_blockhash().await?;
    let tsx = v0::Message::try_compile(
        &signer.pubkey(),
        &ixs,
        &[],
        client.get_latest_blockhash().await?,
    )?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(tsx), &[signer])?;
    let signature = client
        .send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await?;
    client
        .confirm_transaction_with_spinner(&signature, &bh, CommitmentConfig::confirmed())
        .await?;
    bonsol_client
        .wait_for_claim(signer.pubkey(), &execution_id, Some(timeout))
        .await?;
    let status = bonsol_client
        .wait_for_proof(signer.pubkey(), &execution_id, Some(timeout))
        .await?;
    if status != ExitCode::Success {
        return Err(anyhow::anyhow!("Execution failed"));
    }
    println!("SDK Execution succeeded");
    Ok(())
}

async fn example_sdk_test(
    bonsol_client: &BonsolClient,
    client: &RpcClient,
    signer: &dyn Signer,
    timeout: u64,
) -> Result<()> {
    println!("Running sdk test");
    let ea1 = Pubkey::from_str("3b6DR2gbTJwrrX27VLEZ2FJcHrDvTSLKEcTLVhdxCoaf")?;
    let ea2 = Pubkey::from_str("g7dD1FHSemkUQrX1Eak37wzvDjscgBW2pFCENwjLdMX")?;
    let ea3 = Pubkey::from_str("FHab8zDcP1DooZqXHWQowikqtXJb1eNHc46FEh1KejmX")?;
    let example_program = Pubkey::from_str("exay1T7QqsJPNcwzMiWubR6vZnqrgM16jZRraHgqBGG")?;
    let expiration: u64 = 10000000000;
    let execution_id = rand_id(16);
    let input_1 = "{\"attestation\":\"test\"}";
    let input_2 = "https://echoserver.dev/server?response=N4IgFgpghgJhBOBnEAuA2mkBjA9gOwBcJCBaAgTwAcIQAaEIgDwIHpKAbKASzxAF0+9AEY4Y5VKArVUDCMzogYUAlBlFEBEAF96G5QFdkKAEwAGU1qA";
    let input_hash = hashv(&[input_1.as_bytes(), input_2.as_bytes()]);
    println!("Execution expiry {}", expiration);
    let slot = bonsol_client.get_current_slot().await?;
    let ixs = bonsol_client
        .execute_v1(
            &signer.pubkey(),
            SIMPLE_IMAGE_ID,
            &execution_id,
            vec![
                InputRef::new(InputType::PublicData, input_1.as_bytes()),
                InputRef::new(InputType::Private, input_2.as_bytes()),
            ],
            10000,
            slot + expiration,
            ExecutionConfig {
                verify_input_hash: true,
                input_hash: Some(input_hash.as_ref()),
                forward_output: true,
            },
            Some(CallbackConfig {
                program_id: example_program,
                instruction_prefix: vec![2],
                extra_accounts: vec![
                    AccountMeta::new(ea1, false),
                    AccountMeta::new_readonly(ea2, false),
                    AccountMeta::new_readonly(ea3, false),
                ],
            }),
            None,
        )
        .await?;
    let bh = client.get_latest_blockhash().await?;
    let tsx = v0::Message::try_compile(
        &signer.pubkey(),
        &ixs,
        &[],
        client.get_latest_blockhash().await?,
    )?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(tsx), &[signer])?;
    let signature = client
        .send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await?;
    client
        .confirm_transaction_with_spinner(&signature, &bh, CommitmentConfig::confirmed())
        .await?;
    bonsol_client
        .wait_for_claim(signer.pubkey(), &execution_id, Some(timeout))
        .await?;
    let status = bonsol_client
        .wait_for_proof(signer.pubkey(), &execution_id, Some(timeout))
        .await?;
    if status != ExitCode::Success {
        return Err(anyhow::anyhow!("Execution failed"));
    }
    println!("SDK Execution succeeded");
    Ok(())
}

async fn example_bonsol_program_test(
    bonsol_client: &BonsolClient,
    client: &RpcClient,
    signer: &dyn Signer,
    timeout: u64,
) -> Result<()> {
    println!("Running Bonsol program test");
    let example_program = Pubkey::from_str("exay1T7QqsJPNcwzMiWubR6vZnqrgM16jZRraHgqBGG")?;
    let bonsol_program = Pubkey::from_str("BoNsHRcyLLNdtnoDf8hiCNZpyehMC4FDMxs6NTxFi3ew")?;
    let expiration: u64 = 10000000000;
    let execution_id = rand_id(16);
    let (requester, bump) =
        Pubkey::find_program_address(&[execution_id.as_bytes()], &example_program);
    let input_1 = "{\"attestation\":\"test\"}";
    let input_2 = "https://echoserver.dev/server?response=N4IgFgpghgJhBOBnEAuA2mkBjA9gOwBcJCBaAgTwAcIQAaEIgDwIHpKAbKASzxAF0+9AEY4Y5VKArVUDCMzogYUAlBlFEBEAF96G5QFdkKAEwAGU1qA";
    let input_hash = hashv(&[input_1.as_bytes(), input_2.as_bytes()]);
    let execution_account = execution_address(&requester, execution_id.as_bytes()).0;
    let deployment_account = deployment_address(SIMPLE_IMAGE_ID).0;
    let ix = Instruction {
        program_id: example_program,
        accounts: vec![
            AccountMeta::new_readonly(signer.pubkey(), true),
            AccountMeta::new(requester, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new(execution_account, false),
            AccountMeta::new_readonly(deployment_account, false),
            AccountMeta::new_readonly(example_program, false),
            AccountMeta::new_readonly(bonsol_program, false),
        ],
        data: [
            &[0],
            execution_id.as_bytes(),
            input_hash.as_ref(),
            &expiration.to_le_bytes(),
            &[bump],
            input_2.as_bytes(),
        ]
        .concat(),
    };
    let bh = client.get_latest_blockhash().await?;
    let tsx = v0::Message::try_compile(
        &signer.pubkey(),
        &[ix],
        &[],
        client.get_latest_blockhash().await?,
    )?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(tsx), &[signer])?;
    let signature = client
        .send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )
        .await?;
    client
        .confirm_transaction_with_spinner(&signature, &bh, CommitmentConfig::confirmed())
        .await?;
    bonsol_client
        .wait_for_claim(requester, &execution_id, Some(timeout))
        .await?;
    let status = bonsol_client
        .wait_for_proof(requester, &execution_id, Some(timeout))
        .await?;
    if status != ExitCode::Success {
        return Err(anyhow::anyhow!("Execution failed"));
    }
    println!("Bonsol Program Execution succeeded");
    Ok(())
}

pub fn rand_id(chars: usize) -> String {
    let mut rng = rand::thread_rng();
    (&mut rng)
        .sample_iter(Alphanumeric)
        .take(chars)
        .map(char::from)
        .collect()
}
