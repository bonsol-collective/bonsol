mod utils;
pub mod verify_prover_version;

use {
    reqwest::Url,
    crate::{
        config::ProverNodeConfig,
        observe::*,
        risc0_runner::utils::async_to_json,
        transaction_sender::{RpcTransactionSender, TransactionSender, TransactionStatus},
        MissingImageStrategy,        
    },
    bonsol_interface::{
        bonsol_schema::{ClaimV1, DeployV1, ExecutionRequestV1},
        prover_version::{ProverVersion, VERSION_V1_2_1},
    },
    dashmap::DashMap,
    risc0_binfmt::MemoryImage,
    risc0_zkvm::{
        ExitCode, Journal, SuccinctReceipt, Receipt,
        recursion::{identity_p254},
        sha::{Digest, Digestible},
        InnerReceipt, MaybePruned, ReceiptClaim, VerifierContext,
        FakeReceipt, ExecutorEnv, ApiClient, ProverOpts, AssetRequest,
        Prover, get_prover_server
    },
    solana_sdk::{pubkey::Pubkey, signature::Signature, instruction::AccountMeta,},
    std::{
        convert::TryInto, env::consts::ARCH, fs, io::Cursor, path::Path, sync::Arc, time::Duration,
        str, rc::Rc,
    },
    utils::{check_stark_compression_tools_path, check_x86_64arch},
};

use {
    crate::types::{BonsolInstruction, ProgramExec},
    anyhow::Result,
    bonsol_interface::bonsol_schema::{parse_ix_data, root_as_deploy_v1, ChannelInstructionIxType},
    bonsol_prover::{
        image::Image,
        input_resolver::{InputResolver, ProgramInput},
        prover::{get_risc0_prover, new_risc0_exec_env},
        util::get_body_max_size,
    },
    risc0_groth16::{ProofJson, Seal},    
    tempfile::tempdir,
    thiserror::Error,
    tokio::{
        fs::File, io::AsyncReadExt, process::Command, sync::mpsc::UnboundedSender, task::JoinHandle,
    },
    tracing::{debug, error, info, warn},
    verify_prover_version::verify_prover_version,
};

const REQUIRED_PROVER: ProverVersion = VERSION_V1_2_1;

#[derive(Debug, Error)]
pub enum Risc0RunnerError {
    #[error("Empty instruction")]
    EmptyInstruction,
    #[error("Invalid data")]
    InvalidData,
    #[error("Img too large")]
    ImgTooLarge,
    #[error("Img load error")]
    ImgLoadError,
    #[error("Image Data Unavailable")]
    ImageDataUnavailable,
    #[error("Image download error")]
    ImageDownloadError(#[from] anyhow::Error),
    #[error("Transaction error: {0}")]
    TransactionError(String),
    #[error("Error with proof compression")]
    ProofCompressionError,
    #[error("Error with proof generation")]
    ProofGenerationError,
    #[error("Invalid prover version {0}, expected {1}")]
    InvalidProverVersion(ProverVersion, ProverVersion),
    #[error("Proof verification failed: {0}")]
    ProofVerificationError(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimStatus {
    Claiming,
    Submitted,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InflightProof {
    pub execution_id: String,
    pub image_id: String,
    pub status: ClaimStatus,
    pub claim_signature: Signature,
    pub submission_signature: Option<Signature>,
    pub expiry: u64,
    pub requester: Pubkey,
    pub program_callback: Option<ProgramExec>,
    pub additional_accounts: Vec<AccountMeta>,
}

type InflightProofs = Arc<DashMap<String, InflightProof>>;
type InflightProofRef<'a> = &'a DashMap<String, InflightProof>;

type LoadedImageMap = Arc<DashMap<String, Image>>;
type LoadedImageMapRef<'a> = &'a DashMap<String, Image>;

type InputStagingArea = Arc<DashMap<String, Vec<ProgramInput>>>;
type InputStagingAreaRef<'a> = &'a DashMap<String, Vec<ProgramInput>>;

pub struct Risc0Runner {
    config: Arc<ProverNodeConfig>,
    loaded_images: LoadedImageMap,
    worker_handle: Option<JoinHandle<Result<()>>>,
    inflight_proof_worker_handle: Option<JoinHandle<Result<()>>>,
    txn_sender: Arc<RpcTransactionSender>,
    input_staging_area: InputStagingArea,
    self_identity: Arc<Pubkey>,
    inflight_proofs: InflightProofs,
    input_resolver: Arc<dyn InputResolver + 'static>,
}

impl Risc0Runner {
    pub async fn new(
        config: ProverNodeConfig,
        self_identity: Pubkey,
        txn_sender: Arc<RpcTransactionSender>,
        input_resolver: Arc<dyn InputResolver + 'static>,
    ) -> Result<Risc0Runner> {
        let dir = fs::read_dir(&config.risc0_image_folder)?;
        let loaded_images = DashMap::new();
        for entry in dir {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let img = Image::new(entry.path()).await?;
                info!("Loaded image: {}", &img.id);
                loaded_images.insert(img.id.clone(), img);
            }
        }

        if !check_x86_64arch() {
            warn!("Bonsol node will not compress STARKs to SNARKs after successful risc0vm\nproving due to stark compression tooling requiring x86_64 architectures - virtualization will also fail");
        }

        check_stark_compression_tools_path(&config.stark_compression_tools_path)?;

        Ok(Risc0Runner {
            config: Arc::new(config),
            loaded_images: Arc::new(loaded_images),
            worker_handle: None,
            inflight_proof_worker_handle: None,
            txn_sender,
            input_staging_area: Arc::new(DashMap::new()),
            self_identity: Arc::new(self_identity),
            inflight_proofs: Arc::new(DashMap::new()),
            input_resolver,
        })
    }

    // TODO: break up pipleine into smaller domains to make it easier to test
    // Break into Image handling, Input handling, Execution Request
    // Inputs and Image should be service used by this prover.
    pub fn start(&mut self) -> Result<UnboundedSender<BonsolInstruction>> {
        verify_prover_version(REQUIRED_PROVER)
            .expect("Bonsol build conflict: prover version is not supported");
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<BonsolInstruction>();
        let loaded_images = self.loaded_images.clone();
        // TODO: move image handling out of prover
        let img_client = Arc::new(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(
                    self.config.image_download_timeout_secs as u64,
                ))
                .build()?,
        );
        let config = self.config.clone();
        let self_id = self.self_identity.clone();
        let input_staging_area = self.input_staging_area.clone();
        let inflight_proofs = self.inflight_proofs.clone();
        let txn_sender = self.txn_sender.clone();
        self.inflight_proof_worker_handle = Some(tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let current_block = txn_sender.get_current_block().await.unwrap_or(0);
                inflight_proofs.retain(|_, v| {
                    if v.expiry < current_block {
                        emit_event!(MetricEvents::ProofExpired, execution_id => v.execution_id.clone());
                        return false;
                    }
                    match &v.status {
                        ClaimStatus::Claiming => {
                            let sig = v.claim_signature;
                            let inner_status = txn_sender.get_signature_status(&sig);
                            return match inner_status {
                                None => false,
                                Some(status) => {
                                    match status {
                                        TransactionStatus::Confirmed(status) => {
                                            txn_sender.clear_signature_status(&sig);
                                            if status.err.is_some() {
                                                info!("Claim Transaction Failed");

                                            }
                                            status.err.is_none()
                                        },
                                        _ => true
                                    }
                                }
                            };
                        }
                        ClaimStatus::Submitted => {
                            if let Some(sig) = v.submission_signature.as_ref() {
                                let inner_status = txn_sender.get_signature_status(sig);
                                return match inner_status {
                                    None => false,
                                    Some(status) => {
                                        match status {
                                            TransactionStatus::Confirmed(status) => {
                                                txn_sender.clear_signature_status(sig);
                                                if status.err.is_some() {
                                                    emit_event!(MetricEvents::ProofSubmissionError, sig => sig.to_string());
                                                }
                                                status.err.is_none()
                                            },
                                            _ => true
                                        }
                                    }
                                };
                            }
                        }
                    };
                    true
                });
            }
        }));

        let inflight_proofs = self.inflight_proofs.clone();
        let txn_sender = self.txn_sender.clone();
        let input_resolver = self.input_resolver.clone();
        self.worker_handle = Some(tokio::spawn(async move {
            while let Some(bix) = rx.recv().await {
                let txn_sender = txn_sender.clone();
                let loaded_images = loaded_images.clone();
                let config = config.clone();
                let img_client = img_client.clone();
                let input_resolver = input_resolver.clone();
                let self_id = self_id.clone();
                let input_staging_area = input_staging_area.clone();
                let inflight_proofs = inflight_proofs.clone();
                tokio::spawn(async move {
                    let bonsol_ix_type =
                        parse_ix_data(&bix.data).map_err(|_| Risc0RunnerError::InvalidData)?;
                    let result = match bonsol_ix_type.ix_type() {
                        ChannelInstructionIxType::DeployV1 => {
                            let payload = bonsol_ix_type
                                .deploy_v1_nested_flatbuffer()
                                .ok_or::<anyhow::Error>(
                                Risc0RunnerError::EmptyInstruction.into(),
                            )?;
                            emit_counter!(MetricEvents::ImageDeployment, 1, "image_id" => payload.image_id().unwrap_or_default());
                            handle_image_deployment(&config, &img_client, payload, &loaded_images)
                                .await
                        }
                        ChannelInstructionIxType::ExecuteV1 => {
                            info!("Received execution request");
                            // Evaluate the execution request and decide if it should be claimed
                            let payload = bonsol_ix_type
                                .execute_v1_nested_flatbuffer()
                                .ok_or::<anyhow::Error>(
                                Risc0RunnerError::EmptyInstruction.into(),
                            )?;
                            let er_prover_version: ProverVersion = payload
                                .prover_version()
                                .try_into()
                                .map_err::<anyhow::Error, _>(|_| {
                                    Risc0RunnerError::InvalidProverVersion(
                                        ProverVersion::UnsupportedVersion,
                                        REQUIRED_PROVER,
                                    )
                                    .into()
                                })?;
                            if er_prover_version != REQUIRED_PROVER {
                                return Err(Risc0RunnerError::InvalidProverVersion(
                                    er_prover_version,
                                    REQUIRED_PROVER,
                                )
                                .into());
                            }
                            handle_execution_request(
                                &config,
                                &inflight_proofs,
                                input_resolver.clone(),
                                img_client.clone(),
                                &txn_sender,
                                &loaded_images,
                                &input_staging_area,
                                bix.last_known_block,
                                payload,
                                &bix.accounts,
                            )
                            .await
                        }
                        ChannelInstructionIxType::ClaimV1 => {
                            info!("Claim Event");
                            let payload = bonsol_ix_type
                                .claim_v1_nested_flatbuffer()
                                .ok_or::<anyhow::Error>(
                                Risc0RunnerError::EmptyInstruction.into(),
                            )?;
                            handle_claim(
                                &config,
                                &self_id,
                                &inflight_proofs,
                                input_resolver.clone(),
                                &txn_sender,
                                &loaded_images,
                                &input_staging_area,
                                payload,
                                &bix.accounts,
                            )
                            .await
                        }
                        ChannelInstructionIxType::StatusV1 => Ok(()),
                        _ => {
                            info!("Unknown instruction type");
                            Ok(())
                        }
                    };
                    if result.is_err() {
                        info!("Error: {:?}", result);
                    }
                    result
                });
            }
            Ok(())
        }));
        Ok(tx)
    }

    pub fn stop(&mut self) -> Result<()> {
        self.worker_handle.take().unwrap().abort();
        Ok(())
    }
}

pub async fn handle_claim<'a>(
    config: &ProverNodeConfig,
    self_identity: &Pubkey,
    in_flight_proofs: InflightProofRef<'a>,
    input_resolver: Arc<dyn InputResolver + 'static>,
    transaction_sender: &RpcTransactionSender,
    loaded_images: LoadedImageMapRef<'a>,
    input_staging_area: InputStagingAreaRef<'a>,
    claim: ClaimV1<'a>,
    accounts: &[Pubkey],
) -> Result<()> {
    info!("Received claim event");
    let claimer = accounts[3];
    let execution_id = claim.execution_id().ok_or(Risc0RunnerError::InvalidData)?;
    
    // Early return if not our claim
    if &claimer != self_identity {
        let attempt = in_flight_proofs.remove(execution_id);
        if let Some((ifp, claim)) = attempt {
            if let ClaimStatus::Claiming = claim.status {
                transaction_sender.clear_signature_status(&claim.claim_signature);
                emit_event!(MetricEvents::ClaimMissed, 
                    execution_id => ifp, 
                    signature => &claim.claim_signature.to_string()
                );
            }
        }
        return Ok(());
    }

    let claim_status = in_flight_proofs
        .get(execution_id)
        .map(|v| v.value().to_owned());
        
    if let Some(mut claim) = claim_status {
        emit_event!(MetricEvents::ClaimReceived, execution_id => execution_id);
        
        if let ClaimStatus::Claiming = claim.status {
            let result: anyhow::Result<()> = async {
                // Get and validate image
                let image = loaded_images.get(&claim.image_id)
                    .ok_or(anyhow::anyhow!("Image not loaded, fatal error aborting execution"))?;
                
                if image.data.is_none() {
                    return Err(Risc0RunnerError::ImageDataUnavailable.into());
                }

                // Handle inputs
                let mut inputs = input_staging_area
                    .get(execution_id)
                    .ok_or(Risc0RunnerError::InvalidData)?
                    .value()
                    .clone();

                let unresolved_count = inputs.iter()
                    .filter(|i| matches!(i, ProgramInput::Unresolved(_)))
                    .count();

                if unresolved_count > 0 {
                    info!("{} outstanding inputs", unresolved_count);
                    // Note: We are not guaranteed to have inputs at claim time
                    emit_event_with_duration!(MetricEvents::InputDownload, {
                        input_resolver.resolve_private_inputs(
                            execution_id,
                            &mut inputs,
                            Arc::new(transaction_sender)
                        ).await?;
                    }, execution_id => execution_id, stage => "private");
                    
                    input_staging_area.insert(execution_id.to_string(), inputs);
                }
                
                info!("{} inputs resolved", unresolved_count);

                // Take ownership of inputs
                let (eid, inputs) = input_staging_area
                    .remove(execution_id)
                    .ok_or(Risc0RunnerError::InvalidData)?;
                
                let mem_image = image.get_memory_image()?;
                
                // Generate proof
                let result: Result<(Journal, Digest, SuccinctReceipt<ReceiptClaim>), Risc0RunnerError> = 
                    tokio::task::spawn_blocking(move || {
                        risc0_prove(mem_image, inputs).map_err(|e| {
                            info!("Error generating proof: {:?}", e);
                            Risc0RunnerError::ProofGenerationError
                        })
                    })
                    .await?;

                let (journal, assumptions_digest, succinct_receipt) = result?;

                // Create Receipt with proper metadata
                let journal_bytes = journal.bytes.clone();
                let receipt = Receipt::new(
                    InnerReceipt::Succinct(succinct_receipt),
                    journal_bytes
                );

                // Compress proof
                let compressed_receipt = risc0_compress_proof(
                    config.stark_compression_tools_path.as_str(),
                    receipt,
                )
                .await
                .map_err(|e| {
                    error!("Proof compression failed: {:?}", e);
                    Risc0RunnerError::ProofCompressionError
                })?;

                // Submit proof
                let (input_digest, committed_outputs) = journal.bytes.split_at(32);
                let sig = transaction_sender
                    .submit_proof(
                        &eid,
                        claim.requester,
                        claim.program_callback.clone(),
                        &compressed_receipt.proof,
                        &compressed_receipt.execution_digest,
                        input_digest,
                        assumptions_digest.as_bytes(),
                        committed_outputs,
                        claim.additional_accounts.clone(),
                        compressed_receipt.exit_code_system,
                        compressed_receipt.exit_code_user,
                    )
                    .await
                    .map_err(|e| {
                        error!("Proof submission failed: {:?}", e);
                        Risc0RunnerError::TransactionError(e.to_string())
                    })?;

                info!("Proof submitted successfully - ID: {}, Signature: {}", 
                    execution_id, sig);
                
                claim.status = ClaimStatus::Submitted;
                claim.submission_signature = Some(sig);
                in_flight_proofs.insert(eid.clone(), claim);
                
                Ok(())
            }.await;

            // Always clean up in-flight proof on completion
            if result.is_err() {
                in_flight_proofs.remove(execution_id);
            }
            
            result?;
        }
    }
    
    Ok(())
}

async fn handle_execution_request<'a>(
    config: &ProverNodeConfig,
    in_flight_proofs: InflightProofRef<'a>,
    input_resolver: Arc<dyn InputResolver + 'static>,
    img_client: Arc<reqwest::Client>,
    transaction_sender: &RpcTransactionSender,
    loaded_images: LoadedImageMapRef<'a>,
    input_staging_area: InputStagingAreaRef<'a>,
    _execution_block: u64,
    exec: ExecutionRequestV1<'a>,
    accounts: &[Pubkey],
) -> Result<()> {
    debug!("Processing execution request");
    
    if !can_execute(exec) {
        warn!(
            "Execution request for incompatible prover version: {:?}",
            exec.prover_version()
        );
        emit_event!(MetricEvents::IncompatibleProverVersion, execution_id => exec.execution_id().unwrap_or_default());
        return Ok(());
    }

    let inflight = in_flight_proofs.len();
    debug!("Current in-flight proofs: {}", inflight);
    debug!("Maximum concurrent proofs allowed: {}", config.maximum_concurrent_proofs);
    
    emit_event!(MetricEvents::ExecutionRequest, execution_id => exec.execution_id().unwrap_or_default());
    
    if inflight < config.maximum_concurrent_proofs as usize {
        let eid = exec
            .execution_id()
            .map(|d| d.to_string())
            .ok_or(Risc0RunnerError::InvalidData)?;
        let image_id = exec
            .image_id()
            .map(|d| d.to_string())
            .ok_or(Risc0RunnerError::InvalidData)?;
            
        debug!("Processing execution ID: {}", eid);
        debug!("Image ID: {}", image_id);
        
        let expiry = exec.max_block_height();
        debug!("Execution expiry block: {}", expiry);
        
        let img = loaded_images.get(&image_id);
        debug!("Image loaded status: {}", img.is_some());
        
        // Check deployment data from chain
        debug!("Fetching deployment account data for image: {}", image_id);
        let account = transaction_sender
            .get_deployment_account(&image_id)
            .await
            .map_err(|e| {
                error!("Failed to get deployment account during execution: {:?}", e);
                Risc0RunnerError::ImageDownloadError(e)
            })?;
        
        debug!("Got deployment account data, length: {}", account.data.len());
        
        if let Ok(deploy_data) = root_as_deploy_v1(&account.data) {
            debug!("Successfully parsed deployment data during execution");
            if let Some(url_bytes) = deploy_data.url() {
                let bytes: Vec<u8> = url_bytes.bytes().collect();
                debug!("Raw URL bytes during execution: {:?}", bytes);
                if let Ok(url_str) = str::from_utf8(&bytes) {
                    debug!("Raw URL string during execution: {}", url_str);
                    // Try to parse and validate the URL
                    match Url::parse(url_str) {
                        Ok(parsed_url) => debug!("Parsed URL during execution: {}", parsed_url),
                        Err(e) => {
                            debug!("URL parsing error during execution: {:?} for URL: {}", e, url_str);
                            if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
                                debug!("URL missing scheme during execution");
                            }
                            if url_str.contains("://") {
                                debug!("URL contains scheme but still invalid");
                            }
                        }
                    }
                } else {
                    error!("URL contains invalid UTF-8 during execution");
                }
            } else {
                error!("No URL found in deployment data during execution");
            }
        } else {
            error!("Failed to parse deployment data during execution");
        }
        
        let img = if img.is_none() {
            debug!("Image not found in loaded images, checking strategy: {:?}", config.missing_image_strategy);
            match config.missing_image_strategy {
                MissingImageStrategy::DownloadAndClaim => {
                    debug!("Attempting to download and claim image");
                    load_image(
                        config,
                        transaction_sender,
                        &img_client,
                        &image_id,
                        loaded_images,
                    )
                    .await?;
                    loaded_images.get(&image_id)
                }
                MissingImageStrategy::DownloadAndMiss => {
                    debug!("Downloading image but will reject claim");
                    load_image(
                        config,
                        transaction_sender,
                        &img_client,
                        &image_id,
                        loaded_images,
                    )
                    .await?;
                    None
                }
                MissingImageStrategy::Fail => {
                    debug!("Rejecting claim due to missing image");
                    None
                }
            }
        } else {
            img
        }
        .ok_or(Risc0RunnerError::ImgLoadError)?;

        emit_histogram!(MetricEvents::ImageComputeEstimate, img.size as f64, image_id => image_id.clone());
        let computable_by = expiry / 2;
        debug!("Compute deadline block: {}", computable_by);

        if computable_by < expiry {
            debug!("Processing inputs for execution");
            let inputs = exec.input().ok_or(Risc0RunnerError::InvalidData)?;
            let program_inputs = emit_event_with_duration!(MetricEvents::InputDownload, {
                debug!("Resolving public inputs");
                input_resolver.resolve_public_inputs(
                    inputs.iter().map(|i| i.unpack()).collect()
                ).await?
            }, execution_id => eid, stage => "public");
            
            debug!("Storing inputs in staging area");
            input_staging_area.insert(eid.clone(), program_inputs);
            
            debug!("Attempting to claim execution");
            let sig = transaction_sender
                .claim(&eid, accounts[0], accounts[2], computable_by)
                .await
                .map_err(|e| Risc0RunnerError::TransactionError(e.to_string()));
                
            match sig {
                Ok(sig) => {
                    debug!("Claim successful, signature: {}", sig);
                    let callback_program = exec
                        .callback_program_id()
                        .and_then::<[u8; 32], _>(|v| v.bytes().try_into().ok())
                        .map(Pubkey::from);
                        
                    debug!("Callback program: {:?}", callback_program);
                    
                    let callback = if callback_program.is_some() {
                        Some(ProgramExec {
                            program_id: callback_program.unwrap(),
                            instruction_prefix: exec
                                .callback_instruction_prefix()
                                .map(|v| v.bytes().to_vec())
                                .unwrap_or(vec![0x1]),
                        })
                    } else {
                        None
                    };

                    in_flight_proofs.insert(
                        eid.clone(),
                        InflightProof {
                            execution_id: eid.clone(),
                            image_id: image_id.clone(),
                            status: ClaimStatus::Claiming,
                            expiry,
                            claim_signature: sig,
                            submission_signature: None,
                            requester: accounts[0],
                            program_callback: callback,
                            additional_accounts: exec
                                .callback_extra_accounts()
                                .unwrap_or_default()
                                .into_iter()
                                .map(|a| {
                                    let pkbytes: [u8; 32] = a.pubkey().into();
                                    let pubkey = Pubkey::try_from(pkbytes).unwrap_or_default();
                                    let writable = a.writable();
                                    AccountMeta {
                                        pubkey,
                                        is_writable: writable == 1,
                                        is_signer: false,
                                    }
                                })
                                .collect(),
                        },
                    );
                    emit_event!(MetricEvents::ClaimAttempt, execution_id => eid);
                    debug!("Successfully registered in-flight proof");
                }
                Err(e) => {
                    error!("Error claiming: {:?}", e);
                    in_flight_proofs.remove(&eid);
                }
            }
        } else {
            debug!("Skipping execution - cannot compute before expiry");
        }
    } else {
        debug!("Rejecting execution - maximum concurrent proofs reached");
    }
    Ok(())
}

async fn load_image<'a>(
    config: &ProverNodeConfig,
    transaction_sender: &RpcTransactionSender,
    http_client: &reqwest::Client,
    image_id: &str,
    loaded_images: LoadedImageMapRef<'a>,
) -> Result<()> {
    debug!("Loading image with ID: {}", image_id);
    let account = transaction_sender
        .get_deployment_account(image_id)
        .await
        .map_err(|e| {
            error!("Failed to get deployment account: {:?}", e);
            Risc0RunnerError::ImageDownloadError(e)
        })?;
    
    debug!("Got deployment account data, length: {}", account.data.len());
    let deploy_data = match root_as_deploy_v1(&account.data) {
        Ok(data) => {
            debug!("Successfully parsed deployment data");
            if let Some(url_bytes) = data.url() {
                // Print the raw URL bytes for debugging
                let bytes: Vec<u8> = url_bytes.bytes().collect();
                debug!("Raw URL bytes: {:?}", bytes);
                if let Ok(url_str) = str::from_utf8(&bytes) {
                    debug!("URL as string: {}", url_str);
                    // Try to parse and validate the URL
                    match Url::parse(url_str) {
                        Ok(parsed_url) => debug!("Parsed URL: {}", parsed_url),
                        Err(e) => error!("URL parsing error: {}", e),
                    }
                } else {
                    error!("URL contains invalid UTF-8");
                }
            } else {
                error!("No URL found in deployment data");
            }
            data
        }
        Err(e) => {
            error!("Failed to parse deployment data: {:?}", e);
            return Err(anyhow::anyhow!("Failed to parse account data").into());
        }
    };

    debug!("Attempting to handle image deployment");
    handle_image_deployment(config, http_client, deploy_data, loaded_images).await?;
    debug!("Image deployment handled successfully");
    Ok(())
}

async fn handle_image_deployment<'a>(
    config: &ProverNodeConfig,
    http_client: &reqwest::Client,
    deploy: DeployV1<'a>,
    loaded_images: LoadedImageMapRef<'a>,
) -> Result<()> {
    let url_bytes = match deploy.url() {
        Some(bytes) => bytes,
        None => {
            error!("Failed to get URL from deployment data");
            return Err(Risc0RunnerError::InvalidData.into());
        }
    };
    
    // Convert URL bytes to string and validate
    let bytes: Vec<u8> = url_bytes.bytes().collect();
    debug!("Raw URL bytes from deployment: {:?}", bytes);
    
    let url_str = str::from_utf8(&bytes).map_err(|e| {
        error!("Invalid UTF-8 in URL: {:?}", e);
        anyhow::anyhow!("Invalid UTF-8 in URL")
    })?;
    
    debug!("URL string before parsing: {}", url_str);
    
    // Try to fix relative URLs by prepending https:// if needed
    let url_str = if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
        debug!("Attempting to fix relative URL by prepending https://");
        format!("https://{}", url_str.trim_start_matches('/'))
    } else {
        url_str.to_string()
    };
    
    let url = Url::parse(&url_str).map_err(|e| {
        error!("URL parsing error: {:?} for URL: {}", e, url_str);
        if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
            error!("URL must be absolute and start with http:// or https://");
        }
        anyhow::anyhow!("Invalid URL format: {}", e)
    })?;

    // Additional validation for absolute URLs
    if !url.has_host() {
        error!("URL must have a host: {}", url_str);
        return Err(anyhow::anyhow!("URL must be absolute with a valid host").into());
    }
    
    let size = deploy.size_();
    let image_id = deploy.image_id().unwrap_or_default();
    let program_name = deploy.program_name().unwrap_or_default();
    
    debug!("Starting image deployment process");
    debug!("URL (parsed): {}", url);
    debug!("Image ID: {}", image_id);
    debug!("Size: {}", size);
    debug!("Program name: {}", program_name);
    
    emit_histogram!(MetricEvents::ImageDownload, size as f64, url => url.to_string());
    emit_event_with_duration!(MetricEvents::ImageDownload, {
        debug!("Initiating HTTP GET request to: {}", url);
        let resp = match http_client.get(url.as_str()).send().await {
            Ok(r) => {
                debug!("Received HTTP response");
                debug!("Response status: {}", r.status());
                debug!("Response headers: {:?}", r.headers());
                r
            }
            Err(e) => {
                error!("HTTP request failed: {:?}", e);
                return Err(e.into());
            }
        };
        
        let resp = match resp.error_for_status() {
            Ok(r) => r,
            Err(e) => {
                error!("HTTP error status: {:?}", e);
                return Err(e.into());
            }
        };
        
        let min = std::cmp::min(size, (config.max_image_size_mb * 1024 * 1024) as u64) as usize;
        debug!("Download size limits - requested: {}, max: {}, using: {}", 
            size, config.max_image_size_mb * 1024 * 1024, min);
        
        if resp.status().is_success() {
            let stream = resp.bytes_stream();
            debug!("Starting to read response body stream");
            
            let resp_data = match get_body_max_size(stream, min).await {
                Ok(data) => {
                    debug!("Successfully read {} bytes from response", data.len());
                    data
                }
                Err(e) => {
                    error!("Failed to read response body: {:?}", e);
                    return Err(Risc0RunnerError::ImgTooLarge.into());
                }
            };
            
            let img = match Image::from_bytes(resp_data) {
                Ok(i) => {
                    debug!("Successfully created image from bytes");
                    i
                }
                Err(e) => {
                    error!("Failed to create image from bytes: {:?}", e);
                    return Err(e.into());
                }
            };
            
            if let Some(bytes) = img.bytes() {
                let path = Path::new(&config.risc0_image_folder).join(img.id.clone());
                debug!("Writing image to path: {:?}", path);
                if let Err(e) = tokio::fs::write(&path, bytes).await {
                    error!("Failed to write image file: {:?}", e);
                    return Err(e.into());
                }
                debug!("Successfully wrote image file");
            }
            
            if img.id != image_id {
                error!("Image ID mismatch - Expected: {}, Got: {}", image_id, img.id);
                return Err(Risc0RunnerError::InvalidData.into());
            }
            
            loaded_images.insert(img.id.clone(), img);
            info!("Successfully downloaded and stored image: {}", image_id);
        } else {
            error!("Download failed with status: {}", resp.status());
            return Err(Risc0RunnerError::InvalidData.into());
        }
        Ok::<_, anyhow::Error>(())
    }, url => url.to_string())
}

pub async fn risc0_compress_proof(
    tools_path: &str,
    receipt: Receipt,
) -> Result<CompressedReceipt> {
    // Validate journal structure
    let journal_bytes = &receipt.journal.bytes;
    if journal_bytes.len() < 32 {
        error!("Invalid journal size");
        return Err(Risc0RunnerError::ProofCompressionError.into());
    }

    match &receipt.inner {
        InnerReceipt::Succinct(sr) => {
            let seal_bytes = sr.get_seal_bytes();
            if !(ARCH == "x86_64" || ARCH == "x86") {
                panic!("X86 only");
            }

            let tmp = tempdir()?;
            let prove_dir = tmp.path();
            let root_path = Path::new(tools_path);
            let mut cursor = Cursor::new(&seal_bytes);
            let inputs = prove_dir.join("input.json");
            let witness = prove_dir.join("out.wtns");
            let input_file = File::create(&inputs).await?;

            emit_event_with_duration!(MetricEvents::ProofConversion, {
                async_to_json(&mut cursor, input_file).await
            }, system => "groth16json")?;

            let zkey = root_path.join("stark_verify_final.zkey");
            let proof_out = prove_dir.join("proof.json");
            let public = prove_dir.join("public.json");

            emit_event_with_duration!(MetricEvents::ProofCompression, {
                let status = Command::new(root_path.join("stark_verify"))
                    .arg(inputs.clone())
                    .arg(witness.clone())
                    .output()
                    .await?;

                if !status.status.success() {
                    info!("witness {:?}", status);
                    return Err(Risc0RunnerError::ProofCompressionError.into());
                }

                let snark_status = Command::new(root_path.join("rapidsnark"))
                    .arg(zkey)
                    .arg(witness)
                    .arg(proof_out.clone())
                    .arg(public)
                    .output()
                    .await?;

                if !snark_status.status.success() {
                    info!("snark {:?}", snark_status);
                    return Err(Risc0RunnerError::ProofCompressionError.into());
                }
            }, system => "risc0");

            let mut proof_fd = File::open(proof_out).await?;
            let mt = proof_fd.metadata().await?;
            let mut bytes = Vec::with_capacity(mt.len() as usize);
            proof_fd.read_to_end(&mut bytes).await?;
            let proof: ProofJson = serde_json::from_slice(&bytes)?;
            let seal: Seal = proof.try_into()?;

            // Get claim using direct field access for SuccinctReceipt
            match &sr.claim {
                MaybePruned::Value(rc) => {
                    let (system, user) = match rc.exit_code {
                        ExitCode::Halted(user_exit) => (0, user_exit),
                        ExitCode::Paused(user_exit) => (1, user_exit),
                        ExitCode::SystemSplit => (2, 0),
                        ExitCode::SessionLimit => (2, 2),
                    };

                    Ok(CompressedReceipt {
                        execution_digest: rc.post.digest().as_bytes().to_vec(),
                        exit_code_system: system,
                        exit_code_user: user,
                        proof: seal.to_vec(),
                    })
                }
                MaybePruned::Pruned(digest) => {
                    Ok(CompressedReceipt {
                        execution_digest: digest.as_bytes().to_vec(),
                        exit_code_system: 0,
                        exit_code_user: 0,
                        proof: seal.to_vec(),
                    })
                }
            }
        }
        InnerReceipt::Composite(cr) => {
            debug!("Converting Composite receipt to Succinct before compression");
            // Convert composite to succinct in blocking context
            let cr = cr.clone(); // Clone for move into spawn_blocking
            let sr = emit_event_with_duration!(MetricEvents::ProofConversion, {
                tokio::task::spawn_blocking(move || {
                    let prover = get_risc0_prover()?;
                    prover.composite_to_succinct(&cr)
                }).await?
            }, system => "risc0")?;
            
            // Create new receipt with Succinct variant
            let succinct_receipt = Receipt::new(
                InnerReceipt::Succinct(sr),
                receipt.journal.bytes
            );
            
            // Use Box::pin for async recursion
            Box::pin(risc0_compress_proof(tools_path, succinct_receipt)).await
        }
        InnerReceipt::Fake(_) => {
            debug!("Dev mode: Creating mock compressed receipt");
            // Get claim using method call for regular Receipt
            let claim = receipt.claim()?;
            
            match claim {
                MaybePruned::Value(rc) => {
                    // Get exit codes even in dev mode
                    let (system, user) = match rc.exit_code {
                        ExitCode::Halted(user_exit) => (0, user_exit),
                        ExitCode::Paused(user_exit) => (1, user_exit),
                        ExitCode::SystemSplit => (2, 0),
                        ExitCode::SessionLimit => (2, 2),
                    };
                    
                    // Create minimal mock proof data
                    let mock_proof = vec![0u8; 32]; // Minimal mock proof
                    
                    emit_event!(MetricEvents::ProofCompression,
                        mode => "dev",
                        details => "Using mock proof data"
                    );
                    
                    Ok(CompressedReceipt {
                        execution_digest: rc.post.digest().as_bytes().to_vec(),
                        exit_code_system: system,
                        exit_code_user: user,
                        proof: mock_proof,
                    })
                }
                MaybePruned::Pruned(digest) => {
                    debug!("Dev mode with pruned digest: {:?}", digest);
                    Ok(CompressedReceipt {
                        execution_digest: digest.as_bytes().to_vec(),
                        exit_code_system: 0,
                        exit_code_user: 0,
                        proof: vec![0u8; 32], // Minimal mock proof
                    })
                }
            }
        }
        _ => {
            error!("Unexpected receipt type");
            emit_event!(MetricEvents::ProvingFailed,
                error => "unexpected_receipt_type",
                details => "Receipt type not supported"
            );
            Err(Risc0RunnerError::ProofCompressionError.into())
        }
    }
}

// proving function, no async this is cpu/gpu intesive
fn risc0_prove(
    memory_image: MemoryImage,
    sorted_inputs: Vec<ProgramInput>,
) -> Result<(Journal, Digest, SuccinctReceipt<ReceiptClaim>)> {
    let image_id = memory_image.compute_id();
    let mut exec = new_risc0_exec_env(memory_image, sorted_inputs)?;
    let session = exec.run()?;
    
    // Check if we're in dev mode
    if option_env!("RISC0_DEV_MODE").is_some() {
        debug!("Dev mode: Creating succinct receipt directly");
        
        // Unwrap journal early since we need it multiple times
        let journal = session.journal.ok_or_else(|| {
            error!("Missing journal in dev mode");
            anyhow::anyhow!("Journal must be provided")
        })?;

        // Create a prover with succinct options
        let prover = get_prover_server(&ProverOpts::succinct())?;
        
        // Create minimal execution environment with just the journal
        let env = ExecutorEnv::builder()
            .write(&journal.bytes)?
            .build()?;

        // Get succinct receipt directly from prover
        let receipt = prover.prove(env, &[])?;
        
        // Extract the SuccinctReceipt
        let succinct = match receipt.receipt.inner {
            InnerReceipt::Succinct(sr) => sr,
            _ => {
                error!("Expected succinct receipt in dev mode");
                return Err(Risc0RunnerError::ProofGenerationError.into());
            }
        };

        // Get digest from the claim field
        let digest = succinct.claim.digest();

        return Ok((journal, digest, succinct));
    }
    
    // Production mode - use real prover
    debug!("Production mode: Generating real proof");
    let prover = get_risc0_prover()?;
    let ctx = VerifierContext::default();
    
    let info = emit_event_with_duration!(MetricEvents::ProofGeneration, {
        prover.prove_session(&ctx, &session)
    }, system => "risc0", image_id => image_id.to_string())?;
    
    emit_histogram!(MetricEvents::ProofSegments, info.stats.segments as f64, system => "risc0", image_id => image_id.to_string());
    emit_histogram!(MetricEvents::ProofCycles, info.stats.total_cycles as f64, system => "risc0", cycle_type => "total", image_id => image_id.to_string());
    emit_histogram!(MetricEvents::ProofCycles, info.stats.user_cycles as f64, system => "risc0", cycle_type => "user", image_id => image_id.to_string());
    
    // Validate journal structure
    let journal = info.receipt.journal.clone();
    if journal.bytes.len() < 32 {
        error!("Invalid journal size in production mode");
        return Err(Risc0RunnerError::ProofGenerationError.into());
    }

    // Create API client for compression
    let client = ApiClient::from_env()?;
    let opts = ProverOpts::default();
    
    // Convert to succinct receipt using compression
    let succinct_receipt = client.compress(
        &opts,
        info.receipt.try_into()?,
        AssetRequest::Inline,
    )?;

    // Extract the SuccinctReceipt and digest
    let (digest, succinct) = match &succinct_receipt.inner {
        InnerReceipt::Succinct(sr) => {
            let digest = match &sr.claim {
                MaybePruned::Value(rc) => {
                    match &rc.output {
                        MaybePruned::Value(Some(output)) => {
                            match &output.assumptions {
                                MaybePruned::Value(ass) => ass.digest(),
                                _ => {
                                    error!("Pruned assumptions in production mode");
                                    return Err(Risc0RunnerError::ProofGenerationError.into());
                                }
                            }
                        }
                        _ => {
                            error!("Invalid output in production mode");
                            return Err(Risc0RunnerError::ProofGenerationError.into());
                        }
                    }
                }
                MaybePruned::Pruned(_) => {
                    error!("Pruned claim in production mode");
                    return Err(Risc0RunnerError::ProofGenerationError.into());
                }
            };
            (digest, sr.clone())
        }
        _ => {
            error!("Expected succinct receipt after compression");
            return Err(Risc0RunnerError::ProofGenerationError.into());
        }
    };
    
    Ok((journal, digest, succinct))
}

pub struct CompressedReceipt {
    pub execution_digest: Vec<u8>,
    pub exit_code_system: u32,
    pub exit_code_user: u32,
    pub proof: Vec<u8>,
}

fn can_execute(exec: ExecutionRequestV1) -> bool {
    let version = exec.prover_version().try_into();
    if version.is_ok() {
        match version.unwrap() {
            REQUIRED_PROVER => true,
            _ => false,
        }
    } else {
        false
    }
}
