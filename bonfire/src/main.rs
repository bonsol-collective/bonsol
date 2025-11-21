use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, LazyLock, Weak,
    },
    time::{Duration, SystemTime},
};

use anyhow::{anyhow, Error, Result};
use bonsol_schema::{parse_ix_data, ChannelInstructionIxType};
use chrono::{DateTime, Utc};
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    stream, SinkExt, StreamExt,
};
use quinn::{
    rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    Connection, Endpoint, ServerConfig, TransportConfig,
};
use serde::{Deserialize, Serialize};
use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};
use solana_program::pubkey;
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use solana_sdk::{bs58, commitment_config::CommitmentConfig, pubkey::Pubkey};
use solana_transaction_status::{
    option_serializer::OptionSerializer, UiInstruction, UiTransactionEncoding,
};
use tokio::{
    sync::{
        broadcast::{self, Sender},
        Mutex,
    },
    time::sleep,
};
use tracing::{debug, info, trace};

use crate::protocol::{
    BonfireMessage, BonsolInstruction, Challenge, HardwareSpecs, LogEvent, LoginResponse, Ping,
    SpecsAck,
};

mod protocol;
mod web;

const BONSOL_PROGRAM: Pubkey = pubkey!("BoNsHRcyLLNdtnoDf8hiCNZpyehMC4FDMxs6NTxFi3ew");

const TLS_KEY_FILE: LazyLock<String> =
    LazyLock::new(|| std::env::var("TLS_KEY_FILE").expect("TLS_KEY_FILE must be set!"));

const TLS_CERT_FILE: LazyLock<String> =
    LazyLock::new(|| std::env::var("TLS_CERT_FILE").expect("TLS_CERT_FILE must be set!"));

const WEBSOCKET_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("WEBSOCKET_URL").expect("WEBSOCKET_URL must be set!"));

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum JobStatus {
    Submitted,
    Claimed,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    status: JobStatus,
    execution_id: String,
    node: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()) // Respect RUST_LOG env
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .init();

    info!("Bonfire {} is starting!", env!("CARGO_PKG_VERSION"));

    let clients = BonfireClientList::new();
    let jobs = Arc::new(Mutex::new(HashMap::new()));

    let (log_tx, log_rx) = broadcast::channel(100);
    let (bix_tx, bix_rx) = unbounded();

    tokio::spawn(subscription(bix_tx));
    tokio::spawn(quic_server(clients.clone(), log_tx));
    tokio::spawn(bix_thread(clients.clone(), bix_rx, jobs.clone()));
    web::web_server(clients, jobs, log_rx).await?;
    Ok(())
}

async fn quic_server(clients: BonfireClientList, log_tx: Sender<LogEvent>) -> Result<()> {
    debug!("QUIC thread starting...");
    let key = PrivateKeyDer::from_pem_file(&*TLS_KEY_FILE)?;
    let cert: Result<_, _> = CertificateDer::pem_file_iter(&*TLS_CERT_FILE)?.collect();
    let mut transport = TransportConfig::default();
    transport.max_idle_timeout(Some(Duration::from_secs(15).try_into()?)); // 15 sec timeout
    let mut config = ServerConfig::with_single_cert(cert?, key)?;
    config.transport_config(Arc::new(transport));

    let endpoint = Endpoint::server(config, "0.0.0.0:8041".parse()?)?;

    loop {
        if let Some(incoming) = endpoint.accept().await {
            let clients = clients.clone();
            let log_tx = log_tx.clone();
            tokio::spawn(async move {
                // accept connection in dedicated thread
                let conn = incoming.await?;

                let client = BonfireClientBuilder::new()
                    .with_connection(conn)
                    .with_log_channel(log_tx)
                    .build()
                    .await?;

                clients.push(client).await;

                Ok::<(), anyhow::Error>(())
            });
        }
    }
}

macro_rules! some_or_continue {
    ($i:expr) => {
        if let Some(v) = $i {
            v
        } else {
            continue;
        }
    };
}

async fn bix_thread(
    clients: BonfireClientList,
    rx_chan: UnboundedReceiver<Vec<BonsolInstruction>>,
    jobs: Arc<Mutex<HashMap<String, Job>>>,
) {
    debug!("BIX thread starting...");

    rx_chan
        .for_each(|bixes| {
            let clients = clients.clone();
            let jobs = jobs.clone();

            async move {
                // Send events to all clients
                clients
                    .get_all()
                    .await
                    .iter()
                    .for_each(|c| c.send_event(bixes.clone().into()));

                // Parse instructions
                let parsed_bixes = bixes
                    .iter()
                    .filter_map(|bix| Some((parse_ix_data(&bix.data).ok()?, bix.accounts.clone())));

                let mut jobs = jobs.lock().await;

                for (pix, accounts) in parsed_bixes {
                    let (execution_id, new_status) = match pix.ix_type() {
                        ChannelInstructionIxType::ExecuteV1 => {
                            let ix = some_or_continue!(pix.execute_v1_nested_flatbuffer());
                            (some_or_continue!(ix.execution_id()), JobStatus::Submitted)
                        }
                        ChannelInstructionIxType::ClaimV1 => {
                            let ix = some_or_continue!(pix.claim_v1_nested_flatbuffer());
                            (some_or_continue!(ix.execution_id()), JobStatus::Claimed)
                        }
                        ChannelInstructionIxType::StatusV1 => {
                            let ix = some_or_continue!(pix.status_v1_nested_flatbuffer());
                            (some_or_continue!(ix.execution_id()), JobStatus::Done)
                        }
                        _ => continue,
                    };

                    // extract claimer/node only for ClaimV1 and StatusV1
                    let claimer: Option<String> =
                        if matches!(pix.ix_type(), ChannelInstructionIxType::ExecuteV1) {
                            None
                        } else {
                            accounts
                                .get(3)
                                .and_then(|data| -> Option<Pubkey> { data.clone().try_into().ok() })
                                .map(|c| c.to_string())
                        };

                    // Apply valid state transitions only
                    jobs.entry(execution_id.to_owned())
                        .and_modify(|job| {
                            if is_valid_transition(job.status, new_status) {
                                job.status = new_status;
                                job.node = claimer.clone();
                                job.updated_at = Utc::now();
                            }
                        })
                        .or_insert_with(|| Job {
                            execution_id: execution_id.to_owned(),
                            node: claimer,
                            status: new_status,
                            updated_at: Utc::now(),
                            created_at: Utc::now(),
                        });
                }
            }
        })
        .await;
}

// ---------------------------------------------------------
// Allowed transitions: Submitted → Claimed → Done
// ---------------------------------------------------------
fn is_valid_transition(old: JobStatus, new: JobStatus) -> bool {
    match (old, new) {
        (JobStatus::Submitted, JobStatus::Claimed) => true,
        (JobStatus::Claimed, JobStatus::Done) => true,
        (JobStatus::Submitted, JobStatus::Done) => true, // allows jump if job didn’t exist before
        (_, _) => false,                                 // backwards or same → not allowed
    }
}

async fn subscription(tx_chan: UnboundedSender<Vec<BonsolInstruction>>) -> Result<()> {
    debug!("Websocket subscription thread starting...");

    let s = move || {
        let tx_chan = tx_chan.clone();
        async move {
            let client = PubsubClient::new(&*WEBSOCKET_URL).await.unwrap();

            client
                .block_subscribe(
                    RpcBlockSubscribeFilter::MentionsAccountOrProgram(BONSOL_PROGRAM.to_string()),
                    Some(RpcBlockSubscribeConfig {
                        encoding: Some(UiTransactionEncoding::Base64),
                        max_supported_transaction_version: Some(0),
                        show_rewards: Some(false),
                        commitment: Some(CommitmentConfig::confirmed()),
                        transaction_details: Some(
                            solana_transaction_status::TransactionDetails::Full,
                        ),
                    }),
                )
                .await?
                .0
                .filter_map(async |block| block.value.block)
                .flat_map(|block| {
                    let bh = block.block_height.unwrap_or(block.parent_slot);
                    stream::iter(
                        block
                            .transactions
                            .into_iter()
                            .flatten()
                            .filter_map(move |tx| {
                                tx.transaction
                                    .decode()
                                    .map(|dtx| tx.meta.map(|meta| (dtx, meta, bh)))
                                    .flatten()
                            }),
                    )
                })
                .map(move |(tx, meta, bh)| {
                    let accounts = tx.message.static_account_keys().to_owned();
                    let mut out_vec = Vec::new();

                    tx.message
                        .instructions()
                        .into_iter()
                        .filter(|ix| ix.program_id(&accounts) == &BONSOL_PROGRAM)
                        .map(|ix| BonsolInstruction {
                            cpi: false,
                            last_known_block: bh,
                            data: ix.data.clone(),
                            accounts: ix
                                .accounts
                                .iter()
                                .map(|a| accounts[*a as usize].to_bytes().to_vec())
                                .collect(),
                        })
                        .for_each(|bix| {
                            out_vec.push(bix);
                        });

                    if let OptionSerializer::Some(itxs) = meta.inner_instructions {
                        itxs.into_iter()
                            .flat_map(|x| x.instructions)
                            .filter_map(|x| match x {
                                UiInstruction::Compiled(ix) => Some(ix),
                                _ => None,
                            })
                            .filter(|ix| {
                                accounts.get(ix.program_id_index as usize) == Some(&BONSOL_PROGRAM)
                            })
                            .filter_map(|ix| {
                                bs58::decode(ix.data)
                                    .into_vec()
                                    .ok()
                                    .map(|data| (data, ix.accounts))
                            })
                            .map(|(data, acc)| BonsolInstruction {
                                cpi: true,
                                accounts: acc
                                    .iter()
                                    .map(|a| accounts[*a as usize].to_bytes().to_vec())
                                    .collect(),
                                data,
                                last_known_block: bh,
                            })
                            .for_each(|bix| {
                                out_vec.push(bix);
                            });
                    }
                    Ok(out_vec)
                })
                .forward(tx_chan)
                .await?;

            Ok::<_, Error>(())
        }
    };

    loop {
        let r = s().await;
        info!("Subscription ended, reason {:?}. Reconnecting", r);
    }
}

#[derive(Default)]
struct BonfireClientBuilder {
    conn: Option<Connection>,
    log_tx: Option<Sender<LogEvent>>,
}

impl BonfireClientBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn with_connection(mut self, conn: Connection) -> Self {
        self.conn = Some(conn);
        self
    }

    fn with_log_channel(mut self, log_tx: Sender<LogEvent>) -> Self {
        self.log_tx = Some(log_tx);
        self
    }

    async fn build(self) -> Result<Arc<BonfireClient>> {
        let conn = self.conn.unwrap();
        let (mut sig_writer, mut sig_reader) =
            protocol::framed::<_, _, BonfireMessage>(conn.open_bi().await?);

        // Start handshake
        let challenge = Challenge::new();
        sig_writer.send(challenge.clone().into()).await?;

        let challenge_response = sig_reader
            .next()
            .await
            .ok_or_else(|| anyhow!("Can't read challenge"))??;

        let challenge_response = challenge_response.as_challenge_response()?;
        let verified = challenge.verify(&challenge_response);

        sig_writer.send(LoginResponse(verified).into()).await?;
        if !verified {
            return Err(anyhow!("Invalid challenge response"));
        }

        let hw = sig_reader
            .next()
            .await
            .ok_or_else(|| anyhow!("Can't read Hardware specs"))??
            .as_client_hardware_specs()?
            .clone();
        debug!("New client connected! Specs {:?}", hw);
        sig_writer.send(SpecsAck.into()).await?;

        // Handshake done!

        let mut event_writer = protocol::writer(conn.open_uni().await?);
        let pubkey: Pubkey = challenge_response.public_key.into();
        let (bix_tx, mut bix_rx) = unbounded();

        let client = Arc::new(BonfireClient {
            hw,
            pubkey,
            tx: bix_tx,
            latency: AtomicU64::new(0),
        });

        // Ping loop
        let client_clone = client.clone();
        let ping_future = async move {
            loop {
                trace!("Pinging...");
                let time = SystemTime::now();
                sig_writer.send(Ping.into()).await?;
                sig_reader
                    .next()
                    .await
                    .ok_or_else(|| anyhow!("QUIC signalling closed"))??
                    .as_pong()?;
                let latency_duration = time.elapsed()?;
                client_clone
                    .latency
                    .store(latency_duration.as_micros() as u64, Ordering::Relaxed);
                trace!("Ping done, latency: {}", latency_duration.as_micros());

                sleep(Duration::from_secs(5)).await;
            }
            #[allow(unreachable_code)]
            Err::<(), _>(anyhow!("Connection dropped"))
        };

        let event_future = async move {
            while let Some(msg) = bix_rx.next().await {
                event_writer.send(msg).await?;
            }
            Err::<(), _>(anyhow!("Connection dropped"))
        };

        let log_future = async move {
            let mut log_reader = protocol::reader::<_, LogEvent>(conn.accept_uni().await?);
            let log_tx = self.log_tx.unwrap();
            while let Some(Ok(msg)) = log_reader.next().await {
                log_tx.send(msg)?;
            }
            Err::<(), _>(anyhow!("Connection dropped"))
        };

        let client_clone = client.clone();
        tokio::spawn(async move {
            let r = futures::future::try_join3(log_future, event_future, ping_future).await;

            debug!(
                "Client dropped: {}, reason: {:?}",
                client_clone.pubkey.to_string(),
                r
            );
        });

        Ok(client)
    }
}

struct BonfireClient {
    hw: HardwareSpecs,
    pubkey: Pubkey,
    tx: UnboundedSender<Vec<BonsolInstruction>>,
    latency: AtomicU64,
}

impl BonfireClient {
    pub fn send_event(&self, msg: Vec<BonsolInstruction>) {
        self.tx.unbounded_send(msg).unwrap();
    }
}

#[derive(Clone)]
struct BonfireClientList {
    list: Arc<Mutex<Vec<Weak<BonfireClient>>>>,
}

impl BonfireClientList {
    pub fn new() -> Self {
        BonfireClientList {
            list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn push(&self, client: Arc<BonfireClient>) {
        self.list.lock().await.push(Arc::downgrade(&client));
    }

    pub async fn get_all(&self) -> Vec<Arc<BonfireClient>> {
        let mut list = self.list.lock().await;
        list.retain(|c| c.upgrade().is_some());
        list.iter().filter_map(|c| c.upgrade()).collect()
    }
}
