use std::fs::{self, File};
use std::path::Path;

use anyhow::Result;
use bonsol_sdk::{BonsolClient, ProgramInputType};
use indicatif::ProgressBar;
use log::debug;
use object_store::aws::AmazonS3Builder;
use object_store::ObjectStore;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::command::{DeployArgs, S3UploadArgs, SharedDeployArgs};
use crate::common::ZkProgramManifest;
use crate::error::{BonsolCliError, S3ClientError, ZkManifestError};

pub async fn deploy(rpc_url: String, signer: Keypair, deploy_args: DeployArgs) -> Result<()> {
    let bar = ProgressBar::new_spinner();
    let rpc_client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());
    let SharedDeployArgs {
        manifest_path,
        auto_confirm,
    } = deploy_args.shared_args();

    let manifest_file = File::open(Path::new(&manifest_path)).map_err(|err| {
        BonsolCliError::ZkManifestError(ZkManifestError::FailedToOpen {
            manifest_path: manifest_path.clone(),
            err,
        })
    })?;
    let manifest: ZkProgramManifest = serde_json::from_reader(manifest_file).map_err(|err| {
        BonsolCliError::ZkManifestError(ZkManifestError::FailedDeserialization {
            manifest_path,
            err,
        })
    })?;
    let loaded_binary = fs::read(&manifest.binary_path).map_err(|err| {
        BonsolCliError::ZkManifestError(ZkManifestError::FailedToLoadBinary {
            binary_path: manifest.binary_path.clone(),
            err,
        })
    })?;
    let url: String = match deploy_args {
        DeployArgs::S3(s3_upload) => {
            let S3UploadArgs {
                bucket,
                access_key,
                secret_key,
                region,
                endpoint,
                ..
            } = s3_upload;

            let dest = format!("{}-{}", manifest.name, manifest.image_id);
            let store_path = object_store::path::Path::from(dest.clone());

            // Use conventional S3 endpoint URL format
            let endpoint_url = endpoint.unwrap_or(format!("https://s3.{}.amazonaws.com", region));

            // Create the S3 client with the proper configuration
            let s3_client = AmazonS3Builder::new()
                .with_bucket_name(&bucket)
                .with_region(&region)
                .with_access_key_id(&access_key)
                .with_secret_access_key(&secret_key)
                .with_endpoint(&endpoint_url)
                .build()
                .map_err(|err| {
                    BonsolCliError::S3ClientError(S3ClientError::FailedToBuildClient {
                        args: vec![
                            format!("bucket: {bucket}"),
                            format!("access_key: {access_key}"),
                            format!(
                                "secret_key: {}..{}",
                                &secret_key[..4],
                                &secret_key[secret_key.len() - 4..]
                            ),
                            format!("region: {region}"),
                        ],
                        err,
                    })
                })?;

            // get the file to see if it exists
            if s3_client.head(&store_path).await.is_ok() {
                bar.set_message("File already exists, skipping upload");
            } else {
                s3_client
                    .put(&store_path, loaded_binary.into())
                    .await
                    .map_err(|err| {
                        BonsolCliError::S3ClientError(S3ClientError::UploadFailed {
                            dest: store_path.clone(),
                            err,
                        })
                    })?;
            }

            bar.finish_and_clear();

            // Create a properly formatted HTTPS URL for the S3 object
            // This follows the AWS S3 URL convention
            let https_url = format!("https://{}.s3.{}.amazonaws.com/{}", bucket, region, dest);
            println!("Image uploaded to S3");
            debug!("S3 path: s3://{}/{}", bucket, dest);
            debug!("HTTPS URL (used for download): {}", https_url);

            // Return the HTTPS URL for compatibility with the HTTP client
            https_url
        }
        DeployArgs::Url(url_upload) => {
            let url = if url_upload.post {
                // Post the binary to the URL endpoint
                // TODO: use command args to determine the method/URL
                let url = format!(
                    "{}/{}-{}",
                    url_upload.url.trim_end_matches("/"),
                    manifest.name,
                    manifest.image_id
                );
                let client = reqwest::Client::new();
                client
                    .post(&url)
                    .header("Content-Type", "application/octet-stream")
                    .body(loaded_binary.clone())
                    .send()
                    .await?;

                url
            } else {
                // Not posting assumes the data is already at the URL, check it
                let req = reqwest::get(&url_upload.url).await?;
                let bytes = req.bytes().await?;
                if bytes != loaded_binary {
                    return Err(BonsolCliError::OriginBinaryMismatch {
                        url: url_upload.url,
                        binary_path: manifest.binary_path,
                    }
                    .into());
                }

                url_upload.url
            };

            bar.finish_and_clear();
            println!("Program available at URL {}", url);
            url
        }
    };

    if !auto_confirm {
        bar.finish_and_clear();
        println!("Deploying to Solana, which will cost real money. Are you sure you want to continue? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let response = input.trim();
        if response != "y" {
            bar.finish_and_clear();
            println!("Response: {response}\nAborting...");
            return Ok(());
        }
    }
    let bonsol_client = BonsolClient::with_rpc_client(rpc_client);
    let image_id = manifest.image_id;
    let deploy = bonsol_client.get_deployment(&image_id).await;
    match deploy {
        Ok(Some(account)) => {
            bar.finish_and_clear();
            println!(
                "Deployment for account '{}' already exists, deployments are immutable",
                account.owner
            );
            Ok(())
        }
        Ok(None) => {
            let deploy_txn = bonsol_client
                .deploy_v1(
                    &signer.pubkey(),
                    &image_id,
                    manifest.size,
                    &manifest.name,
                    &url,
                    manifest
                        .input_order
                        .iter()
                        .map(|i| match i.as_str() {
                            "Public" => ProgramInputType::Public,
                            "Private" => ProgramInputType::Private,
                            _ => ProgramInputType::Unknown,
                        })
                        .collect(),
                )
                .await?;
            if let Err(err) = bonsol_client.send_txn_standard(signer, deploy_txn).await {
                bar.finish_and_clear();
                anyhow::bail!(err)
            }

            bar.finish_and_clear();
            println!("{} deployed", image_id);
            Ok(())
        }
        Err(e) => {
            bar.finish_with_message(format!("Error getting deployment: {:?}", e));
            Ok(())
        }
    }
}
