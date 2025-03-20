use anyhow::Result;
use bonsol_sdk::{deployment_address, execution_address, rand_id};
use risc0_zkvm::sha::{Impl, Sha256};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::{hash, hashv},
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

const PROGRAM_ID: Pubkey = pubkey!("exay1T7QqsJPNcwzMiWubR6vZnqrgM16jZRraHgqBGG");
const SIMPLE_IMAGE_ID: &str = "d69d68a076b618b053446e7a055ef9638b1ad623ce91f2155bd9b588bd1f069e";

#[tokio::main]
async fn main() -> Result<()> {
    let (rpc_url, keypair) = try_load_from_config()?;
    let client = RpcClient::new(rpc_url);

    // Prepare instruction data
    let execution_id = rand_id(16);
    let (requester_pda, requester_bump) =
        Pubkey::find_program_address(&[execution_id.as_bytes()], &PROGRAM_ID);
    let (execution_account, _execution_bump) =
        execution_address(&requester_pda, &execution_id.as_bytes());
    let input_hash = [
        227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65,
        228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85,
    ];
    let expiration = 1000u64.to_le_bytes();
    let private_input_url = "https://echoserver.dev/server?response=N4IgFgpghgJhBOBnEAuA2mkIA0WC6euARgPYwCeqoALuQA4SojUQAe1OIMU1UTLiagEdEMAGYjxQmIgmi5U+SAC+uQTwCuyFACYADHuVA";
    let mut data = vec![0];
    data.extend_from_slice(&execution_id.as_bytes());
    data.extend_from_slice(&input_hash);
    data.extend_from_slice(&expiration);
    data.push(requester_bump);
    data.extend_from_slice(private_input_url.as_bytes());

    let deployment_account = deployment_address(SIMPLE_IMAGE_ID).0;
    
    let signature = client
        .send_transaction(&Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new(keypair.pubkey(), true),
                    AccountMeta::new(requester_pda, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                    AccountMeta::new(execution_account, false),
                    AccountMeta::new_readonly(deployment_account, false),
                    AccountMeta::new_readonly(PROGRAM_ID, false),
                    AccountMeta::new_readonly(bonsol_sdk::ID, false),
                ],
                data,
            }],
            Some(&keypair.pubkey()),
            &[&keypair],
            client.get_latest_blockhash().await?,
        ))
        .await?;
    client.confirm_transaction(&signature).await?;

    println!("Transaction sent: {}", signature);
    Ok(())
}

fn try_load_from_config() -> anyhow::Result<(String, Keypair)> {
    let config = solana_cli_config::Config::load(solana_cli_config::CONFIG_FILE.as_ref().unwrap())
        .unwrap_or_default();
    Ok((
        config.json_rpc_url,
        solana_sdk::signature::read_keypair_file(std::path::Path::new(&config.keypair_path))
            .unwrap(),
    ))
}
