use crate::command::PdaCommand;
use anyhow::Result;
use bonsol_interface::bonsol_schema::{
    root_as_claim_v1, root_as_deploy_v1, root_as_execution_request_v1,
};
use bonsol_sdk::{deployment_address, execution_address, execution_claim_address};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn get_pda(rpc_url: String, pda_command: PdaCommand) -> Result<()> {
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    match pda_command {
        PdaCommand::Deployment { image_id } => {
            let (deployment_pda, deployment_bump) = deployment_address(image_id.as_str());
            println!("Deployment PDA: {:?}", deployment_pda);
            println!("Deployment Bump: {:?}", deployment_bump);

            let account_info = rpc_client.get_account(&deployment_pda).await?;
            println!("Account Info: {:?}", account_info);

            let dp = root_as_deploy_v1(&account_info.data)?;
            println!("DP: {:?}", dp);
        }
        PdaCommand::Execution {
            requester,
            execution_id,
        } => {
            let (execution_pda, execution_bump) = execution_address(
                &Pubkey::from_str(requester.as_str())?,
                execution_id.as_bytes(),
            );
            println!("Execution PDA: {:?}", execution_pda);
            println!("Execution Bump: {:?}", execution_bump);

            let account_info = rpc_client.get_account(&execution_pda).await?;
            println!("Account Info: {:?}", account_info);

            let execution_request = root_as_execution_request_v1(&account_info.data)?;
            println!("execution_request: {:?}", execution_request);
        }
        PdaCommand::Claim { execution_address } => {
            let (claim_pda, claim_bump) = execution_claim_address(execution_address.as_bytes());
            println!("Claim PDA: {:?}", claim_pda);
            println!("Claim Bump: {:?}", claim_bump);

            let account_info = rpc_client.get_account(&claim_pda).await?;
            println!("Account Info: {:?}", account_info);

            let claim_request = root_as_claim_v1(&account_info.data)?;
            println!("claim_request: {:?}", claim_request);
        }
    }

    Ok(())
}

// pub fn execution_address_seeds<'a>(requester: &'a Pubkey, execution_id: &'a [u8]) -> Vec<&'a [u8]> {
//     vec!["execution".as_bytes(), requester.as_ref(), execution_id]
// }

// pub fn deployment_address_seeds(hash: &Hash) -> Vec<&[u8]> {
//     vec!["deployment".as_bytes(), hash.as_ref()]
// }

// pub fn execution_claim_address_seeds(execution_address: &[u8]) -> Vec<&[u8]> {
//     vec!["execution_claim".as_bytes(), execution_address]
// }
