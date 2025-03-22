use std::time::Duration;

use anyhow::Result;
use bonsol_interface::bonsol_schema::{root_as_deploy_v1, root_as_execution_request_v1};
pub use bonsol_interface::bonsol_schema::{
    ClaimV1T, DeployV1T, ExecutionRequestV1T, ExitCode, InputT, InputType, ProgramInputType,
    StatusTypes,
};
use bonsol_interface::claim_state::ClaimStateHolder;
use bonsol_interface::prover_version::ProverVersion;
pub use bonsol_interface::util::*;
pub use bonsol_interface::{instructions, ID};
use bytes::Bytes;
use futures_util::TryFutureExt;
use instructions::{CallbackConfig, ExecutionConfig, InputRef};
use num_traits::FromPrimitive;
use rand::Rng;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::config::RpcSendTransactionConfig;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::{v0, VersionedMessage};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;
use tokio::time::Instant;

pub use flatbuffers;

mod client;
pub use client::*;

pub fn rand_id(chars: usize) -> String {
    let mut rng = rand::thread_rng();
    (&mut rng)
        .sample_iter(rand::distributions::Alphanumeric)
        .take(chars)
        .map(char::from)
        .collect()
}
