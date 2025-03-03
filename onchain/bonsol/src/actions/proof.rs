use crate::{
    assertions::*,
    error::ChannelError,
    proof_handling::{
        output_digest_v1_0_1, output_digest_v1_2_1, prepare_inputs_v1_0_1, prepare_inputs_v1_2_1,
        verify_risc0_v1_0_1, verify_risc0_v1_2_1,
    },
    utilities::*,
};

use bonsol_interface::{
    bonsol_schema::{
        root_as_execution_request_v1, ChannelInstruction, ExecutionRequestV1, ExitCode, StatusV1,
    },
    prover_version::{ProverVersion, VERSION_V1_0_1, VERSION_V1_2_1},
    util::execution_address_seeds,
};

use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_memory::sol_memcmp,
    sysvar::Sysvar,
};

struct ProofAccounts<'a> {
    pub requester: &'a AccountInfo<'a>,
    pub exec: &'a AccountInfo<'a>, // lets store if the proof is verified or not here --
    pub prover: &'a AccountInfo<'a>,
}

impl<'a> ProofAccounts<'a> {
    fn from_instruction(
        accounts: &'a [AccountInfo<'a>],
        data: &[u8],
    ) -> Result<Self, ChannelError> {
        let er_ref = pa.exec.try_borrow_data()?;
        let er = root_as_execution_request_v1(&er_ref)
            .map_err(|_| ChannelError::InvalidExecutionAccount)?;

        let ea = &accounts[1];
        let prover = &accounts[3];
        let callback_program = &accounts[2];

        // get execution id
        /*
        let bmp = Some(check_pda(
            &execution_address_seeds(accounts[0].key, eid.as_bytes()),
            ea.key,
            ChannelError::InvalidExecutionAccount,
        )?);
        */

        Ok(ProofAccounts {
            requester: &accounts[0],
            exec: &accounts[1],
            prover: &accounts[2],
        })
    }
}

fn verify_with_prover(
    input_digest: &[u8],
    co: &[u8],
    asud: &[u8],
    er: ExecutionRequestV1,
    exed: &[u8],
    proof: &[u8; 256],
) -> Result<bool, ProgramError> {
    let er_ref = sa.exec.try_borrow_data()?;
    let er =
        root_as_execution_request_v1(&er_ref).map_err(|_| ChannelError::InvalidExecutionAccount)?;

    if er.max_block_height() < Clock::get()?.slot {
        return Err(ChannelError::ExecutionExpired.into());
    }
    let prover_version =
        ProverVersion::try_from(er.prover_version()).unwrap_or(ProverVersion::default());

    let verified = match prover_version {
        VERSION_V1_0_1 => {
            let output_digest = output_digest_v1_0_1(input_digest, co, asud);
            let proof_inputs = prepare_inputs_v1_0_1(
                er.image_id().unwrap(),
                exed,
                output_digest.as_ref(),
                st.exit_code_system(),
                st.exit_code_user(),
            )?;
            verify_risc0_v1_0_1(proof, &proof_inputs)?
        }
        VERSION_V1_2_1 => {
            let output_digest = output_digest_v1_2_1(input_digest, co, asud);
            let proof_inputs = prepare_inputs_v1_2_1(
                er.image_id().unwrap(),
                exed,
                output_digest.as_ref(),
                st.exit_code_system(),
                st.exit_code_user(),
            )?;
            verify_risc0_v1_2_1(proof, &proof_inputs)?
        }
        _ => false,
    };

    // store prover state here

    Ok(verified)
}

/*
et ix_data = &[]; // todo: create new instruction data here
        let instruction = Instruction::new_with_bytes(self.bonsol_program, ix_data, accounts);
        let (blockhash, last_valid) = self
            .rpc_client
            .get_latest_blockhash_with_commitment(self.rpc_client.commitment())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get blockhash: {:?}", e))?;

        let msg = v0::Message::try_compile(&self.signer.pubkey(), &[instruction], &[], blockhash)?;
        let tx = VersionedTransaction::try_new(VersionedMessage::V0(msg), &[&self.signer])?;

        let sig = self
            .rpc_client
            .send_and_confirm_transaction_with_spinner_and_config(
                &tx,
                CommitmentConfig::confirmed(),
                RpcSendTransactionConfig {
                    skip_preflight: true,
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send transaction: {:?}", e))?;
        self.sigs
            .insert(sig, TransactionStatus::Pending { expiry: last_valid });
*/
