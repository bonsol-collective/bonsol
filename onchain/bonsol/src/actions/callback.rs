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
        root_as_execution_request_v1, ChannelInstruction, ExecutionRequestV1, ExitCode,
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

struct CallbackAccount<'a> {
    requester: &'a AccountInfo<'a>, // requester is prover
    exec_account: &'a AccountInfo<'a>,
    callback_accounts: &'a AccountInfo<'a>,
}
