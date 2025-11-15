use std::{net::ToSocketAddrs, pin::Pin, sync::Arc};

use anyhow::{anyhow, Error, Result};
use bonsol_bonfire::{
    BonfireMessage, BonsolInstruction, Gpu, HardwareSpecs, LogEvent, LogSource, Pong,
};
use bonsol_prover::util::EventChannelRx;
use futures::{future::try_join3, stream::Peekable, SinkExt, StreamExt};
use quinn::{ClientConfig, Endpoint};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use sysinfo::System;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    Mutex,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, warn};

use crate::ingest::{Ingester, TxChannel};

impl From<bonsol_bonfire::BonsolInstruction> for crate::types::BonsolInstruction {
    fn from(other: bonsol_bonfire::BonsolInstruction) -> crate::types::BonsolInstruction {
        crate::types::BonsolInstruction {
            accounts: other
                .accounts
                .into_iter()
                .map(|acc_bytes| acc_bytes.try_into().unwrap())
                .collect(),
            cpi: other.cpi,
            data: other.data,
            last_known_block: other.last_known_block,
        }
    }
}

type LogEventStream = Pin<Box<Peekable<UnboundedReceiverStream<LogEvent>>>>;

pub struct BonfireIngester {
    server_addr: String,
    op_handle: Option<tokio::task::JoinHandle<Result<()>>>,
    keypair: Arc<Keypair>,
    logs_rx: Arc<Mutex<LogEventStream>>,
}

impl BonfireIngester {
    pub fn new(
        server_addr: String,
        keypair: Arc<Keypair>,
        stdout: EventChannelRx,
        stderr: EventChannelRx,
    ) -> Result<BonfireIngester> {
        let (logs_tx, logs_rx) = unbounded_channel();
        let log_pump =
            |rx: EventChannelRx, source: LogSource, logs_tx: UnboundedSender<LogEvent>| {
                tokio::task::spawn_blocking(move || {
                    while let Ok(msg) = rx.recv() {
                        logs_tx
                            .send(bonsol_bonfire::LogEvent {
                                source,
                                image_id: msg.image_id,
                                job_id: msg.job_id,
                                log: msg.log,
                            })
                            .expect("Log channel dropped. This shouldn't happen!");
                    }
                    Ok::<(), Error>(())
                });
            };
        log_pump(stdout, LogSource::Stdout, logs_tx.clone());
        log_pump(stderr, LogSource::Stderr, logs_tx);

        Ok(BonfireIngester {
            server_addr,
            op_handle: None,
            keypair,
            logs_rx: Arc::new(Mutex::new(Box::pin(
                UnboundedReceiverStream::new(logs_rx).peekable(),
            ))),
        })
    }

    async fn connect(
        txchan: UnboundedSender<Vec<crate::types::BonsolInstruction>>,
        logs_rx: Arc<Mutex<LogEventStream>>,
        server_addr: String,
        keypair: Arc<Keypair>,
    ) -> Result<()> {
        let server_name = server_addr
            .split(":")
            .next()
            .ok_or_else(|| anyhow!("Can't parse server address"))?;
        let server_socket_addr = server_addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow!("Can't parse server address"))?;

        // Configure TLS roots

        let client_config = ClientConfig::try_with_platform_verifier()?;
        let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        endpoint.set_default_client_config(client_config);
        debug!("Connecting to Bonfire server");
        let conn = endpoint.connect(server_socket_addr, server_name)?.await?;
        let (mut sig_writer, mut sig_reader) =
            bonsol_bonfire::framed::<_, _, BonfireMessage>(conn.accept_bi().await?);

        debug!("Connection successful, logging in...");

        // Handshake: Challenge -> LoginResponse -> send HardwareSpecs -> SpecsAck
        // 1) Receive server challenge
        let challenge = sig_reader
            .next()
            .await
            .ok_or_else(|| anyhow!("Can't get challenge"))??
            .as_challenge()?
            .clone();
        debug!("Got challenge {:?}", challenge);

        // 2) Sign challenge with ephemeral keypair and send login
        let response = challenge.sign(&*keypair);
        sig_writer.send(response.into()).await?;
        debug!("Response sent");

        // 3) Expect LoginResponse(true)
        let login_ok = sig_reader
            .next()
            .await
            .ok_or_else(|| anyhow!("Can't get login response"))??
            .as_login_ok()?
            .clone();
        if !login_ok.0 {
            return Err(anyhow!("bonfire login rejected by server"));
        }

        // 4) Send basic hardware specs, then wait for SpecsAck
        let specs = Self::get_hw_specs()?;
        sig_writer.send(specs.into()).await?;
        let _ack = sig_reader
            .next()
            .await
            .ok_or_else(|| anyhow!("Can't get challenge"))??
            .as_specs_ack()?;

        debug!("Login successful!");

        let mut log_writer = bonsol_bonfire::writer(conn.open_uni().await?);
        let log_future = async move {
            loop {
                // We peek first to avoid losing messages in case of reconnection
                let mut logs_rx = logs_rx.lock().await;
                let event = logs_rx
                    .as_mut()
                    .peek()
                    .await
                    .expect("channel disconnected, this should never happen!");
                if log_writer.send(event.clone()).await.is_err() {
                    return Err::<(), _>(anyhow!("Log QUIC stream closed"));
                }

                // Now that the log was safely sent, we can consume the message
                logs_rx.next().await;
            }
        };

        let ping_future = async move {
            loop {
                sig_reader
                    .next()
                    .await
                    .ok_or_else(|| anyhow!("Signalling stream closed"))??
                    .as_ping()?;
                sig_writer.send(Pong.into()).await?;
            }
            #[allow(unreachable_code)]
            Ok(())
        };

        let bix_future = async move {
            let mut event_reader =
                bonsol_bonfire::reader::<_, Vec<BonsolInstruction>>(conn.accept_uni().await?);
            loop {
                let msg = event_reader.next().await;
                if let Some(Ok(bix)) = msg {
                    let mut out = Vec::with_capacity(bix.len());
                    for ix in bix.iter() {
                        // Convert Vec<u8> -> Pubkey for each account
                        let accounts = ix
                            .accounts
                            .iter()
                            .filter_map(|a| {
                                if a.len() == 32 {
                                    let mut arr = [0u8; 32];
                                    for (i, b) in a.iter().enumerate() {
                                        arr[i] = *b;
                                    }
                                    Some(Pubkey::new_from_array(arr))
                                } else {
                                    None
                                }
                            })
                            .collect();

                        let data: Vec<u8> = ix.data.iter().copied().collect();

                        out.push(crate::types::BonsolInstruction {
                            cpi: ix.cpi,
                            accounts,
                            data,
                            last_known_block: ix.last_known_block.into(),
                        });
                    }
                    let _ = txchan.send(out);
                }
            }
            #[allow(unreachable_code)]
            Ok(())
        };

        // "Guillotine" join, if one future fails, everything else is aborted, and we reconnect
        try_join3(ping_future, log_future, bix_future)
            .await
            .unwrap();

        Ok(())
    }

    fn get_hw_specs() -> Result<HardwareSpecs> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let gpus = if let Ok(nvml) = nvml_wrapper::Nvml::init() {
            (0..nvml.device_count().unwrap_or(0))
                .filter_map(|i| {
                    nvml.device_by_index(i).ok().and_then(|d| {
                        Some(Gpu {
                            gpu_model: d.name().ok()?,
                            gpu_memory_bytes: d.memory_info().ok()?.total,
                        })
                    })
                })
                .collect()
        } else {
            vec![]
        };

        let cpu_cores = sys.cpus().len() as u32;
        let first_cpu = sys
            .cpus()
            .first()
            .ok_or_else(|| anyhow!("Can't detect any CPU"))?;
        let cpu_type = first_cpu.brand().to_owned();
        let cpu_mhz = first_cpu.frequency() as u32;
        let memory_bytes = sys.total_memory();

        Ok(HardwareSpecs {
            cpu_type,
            cpu_mhz,
            cpu_cores,
            memory_bytes,
            gpus,
        })
    }
}

impl Ingester for BonfireIngester {
    fn start(&mut self, _program: Pubkey) -> Result<TxChannel> {
        let (txchan, rx) = unbounded_channel();

        let keypair = self.keypair.clone();
        let server_addr = self.server_addr.clone();
        let logs_rx = self.logs_rx.clone();

        self.op_handle = Some(tokio::spawn(async move {
            loop {
                let r = Self::connect(
                    txchan.clone(),
                    logs_rx.clone(),
                    server_addr.clone(),
                    keypair.clone(),
                )
                .await;
                warn!("Disconnected! Reason: {:?}\nTrying to reconnect...", r);
            }
        }));

        Ok(rx)
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(t) = self.op_handle.as_mut() {
            t.abort()
        }
        Ok(())
    }
}
