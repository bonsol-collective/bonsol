use std::str::FromStr;

use crate::command::PdaCommand;
use anyhow::Result;
use bonsol_sdk::{deployment_address, execution_address, execution_claim_address, BonsolClient};
use solana_sdk::pubkey::Pubkey;

pub async fn get_pda(rpc_url: String, pda_command: PdaCommand) -> Result<()> {
    let client = BonsolClient::new(rpc_url);
    match pda_command {
        PdaCommand::Deployment { image_id } => {
            let deployment_pda = deployment_address(image_id.as_str()).0;
            println!("Deployment PDA: {:?}", deployment_pda);

            let deployment = client.get_deployment_v1(&image_id).await?;
            println!("deployment: {:?}", deployment);
        }
        PdaCommand::Execution {
            requester,
            execution_id,
        } => {
            let requester = Pubkey::from_str(requester.as_str())?;
            let execution_pda = execution_address(&requester, execution_id.as_bytes()).0;
            println!("Execution PDA: {:?}", execution_pda);

            let execution_request = client
                .get_execution_request_v1(&requester, &execution_id)
                .await?;
            println!("execution_request: {:?}", execution_request);
        }
        PdaCommand::Claim {
            requester,
            execution_id,
        } => {
            let requester = Pubkey::from_str(requester.as_str())?;
            let execution_address = execution_address(&requester, execution_id.as_bytes()).0;
            let claim_pda = execution_claim_address(&execution_address.to_bytes()).0;
            println!("Claim PDA: {:?}", claim_pda);

            let claim_request = client.get_claim_state_v1(&requester, &execution_id).await?;
            println!("claim_request: {:?}", claim_request);
        }
    }

    Ok(())
}
