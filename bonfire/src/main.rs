use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime},
};

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{Data, Query},
};
use actix_web_lab::sse;
use anyhow::{Error, Result, anyhow};
use futures::{SinkExt, Stream, StreamExt, stream};
use quinn::{
    Connection, Endpoint, ServerConfig,
    rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use serde::{Deserialize, Serialize};
use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use solana_sdk::{bs58, commitment_config::CommitmentConfig, pubkey::Pubkey};
use solana_transaction_status::{
    UiInstruction, UiTransactionEncoding, option_serializer::OptionSerializer,
};
use tokio::{
    sync::{
        Mutex,
        broadcast::{self, Receiver, Sender},
        mpsc::{self, UnboundedSender},
    },
    time::sleep,
};
use tokio_stream::wrappers::BroadcastStream;

use crate::protocol::{
    BonfireMessage, BonfireReader, BonfireWriter, BonsolInstruction, Challenge, HardwareSpecs,
    LogEvent, LoginResponse, Ping, SpecsAck,
};

mod protocol;

type ClientsList = Arc<Mutex<Vec<BonfireConnectedClient>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let clients = Arc::new(Mutex::new(Vec::new()));

    let (log_tx, log_rx) = broadcast::channel(100);

    let quic_future = quic_server(clients.clone(), log_tx);
    let web_future = web_server(clients, log_rx);

    futures::try_join!(quic_future, web_future).unwrap();
    Ok(())
}

#[derive(Serialize)]
struct Node {
    pubkey: String,
    hw: HardwareSpecs,
    latency: u64,
}
#[get("/nodes")]
async fn nodes(clients: Data<ClientsList>) -> impl Responder {
    HttpResponse::Ok().json(
        clients
            .lock()
            .await
            .iter()
            .map(|client| Node {
                pubkey: client.key.to_string(),
                latency: client.latency.load(Ordering::Relaxed),
                hw: client.hw.clone(),
            })
            .collect::<Vec<_>>(),
    )
}

#[derive(Deserialize, Clone)]
struct LogsQuery {
    image_id: Option<String>,
    job_id: Option<String>,
}

#[get("/logs")]
async fn logs(log_rx: Data<Receiver<LogEvent>>, params: Query<LogsQuery>) -> impl Responder {
    let stream = BroadcastStream::new(log_rx.resubscribe()).filter_map(move |log| {
        let params = params.clone();
        async move {
            log.ok()
                .filter(|log| match &params.image_id {
                    None => true,
                    Some(image_id) => &*log.image_id == image_id,
                })
                .filter(|log| match &params.job_id {
                    None => true,
                    Some(job_id) => &*log.job_id == job_id,
                })
                .map(|log| {
                    let json = serde_json::to_string(&log)?;
                    Ok::<_, Error>(sse::Event::Data(sse::Data::new(json)))
                })
        }
    });

    sse::Sse::from_stream(stream)
}

async fn web_server(clients: ClientsList, log_rx: Receiver<LogEvent>) -> Result<()> {
    let log_rx = Data::new(log_rx);
    HttpServer::new(move || {
        App::new()
            .service(nodes)
            .service(logs)
            .app_data(Data::new(clients.clone()))
            .app_data(log_rx.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

async fn quic_server(clients: ClientsList, log_tx: Sender<LogEvent>) -> Result<()> {
    let key = PrivateKeyDer::from_pem_file("server.key.pem")?;
    let cert = CertificateDer::from_pem_file("server.cert.pem")?;
    let config = ServerConfig::with_single_cert(vec![cert], key)?;

    let endpoint = Endpoint::server(config, "0.0.0.0:8041".parse()?)?;

    let clients_clone = clients.clone();
    tokio::spawn(async move {
        let client = PubsubClient::new("ws://localhost:8900").await.unwrap();
        subscription(
            &client,
            &"BoNsHRcyLLNdtnoDf8hiCNZpyehMC4FDMxs6NTxFi3ew"
                .parse()
                .unwrap(),
        )
        .await
        .unwrap()
        .for_each(|bix| {
            let clients = clients_clone.clone();
            async move {
                let clients = clients.lock().await;
                clients
                    .iter()
                    .for_each(|c| c.send_event(bix.clone().into()));
            }
        })
        .await;
    });

    loop {
        if let Some(incoming) = endpoint.accept().await {
            let clients = clients.clone();
            let log_tx = log_tx.clone();
            tokio::spawn(async move {
                // accept connection in dedicated thread
                let conn = incoming.await?;
                let client = BonfireConnectedClient::connect(conn, log_tx).await?;
                clients.lock().await.push(client);
                Ok::<(), anyhow::Error>(())
            });
        }
    }
}

async fn subscription(
    client: &PubsubClient,
    program: &Pubkey,
) -> Result<impl Stream<Item = Vec<BonsolInstruction>>> {
    let (stream, _unsub) = client
        .block_subscribe(
            RpcBlockSubscribeFilter::MentionsAccountOrProgram(program.to_string()),
            Some(RpcBlockSubscribeConfig {
                encoding: Some(UiTransactionEncoding::Base64),
                max_supported_transaction_version: Some(0),
                show_rewards: Some(false),
                commitment: Some(CommitmentConfig::confirmed()),
                transaction_details: Some(solana_transaction_status::TransactionDetails::Full),
            }),
        )
        .await?;

    let out = stream
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
                .filter(|ix| ix.program_id(&accounts) == program)
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
                    .filter(|ix| accounts.get(ix.program_id_index as usize) == Some(program))
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
            out_vec
        });
    Ok(out)
}

struct BonfireConnectedClient {
    hw: HardwareSpecs,
    key: Pubkey,
    tx: UnboundedSender<Vec<BonsolInstruction>>,
    latency: Arc<AtomicU64>,
}

impl BonfireConnectedClient {
    pub async fn connect(
        conn: Connection,
        log_tx: Sender<LogEvent>,
    ) -> Result<BonfireConnectedClient> {
        //        let conn = Arc::new(Mutex::new(BonfireConnection::new(writer, reader)));
        let (mut sig_writer, mut sig_reader) = protocol::framed(conn.open_bi().await?);
        let (hw, key) = Self::handshake(&mut sig_writer, &mut sig_reader).await?;

        // Event Stream
        let mut event_writer = protocol::writer(conn.open_uni().await?);

        // Ping loop
        let latency = Arc::new(AtomicU64::new(0));
        let latency_clone = latency.clone();
        let ping_future = async move {
            loop {
                let time = SystemTime::now();
                sig_writer.send(Ping.into()).await.unwrap();
                sig_reader
                    .next()
                    .await
                    .ok_or_else(|| anyhow!("QUIC signalling closed"))??
                    .as_pong()
                    .unwrap();
                let latency_duration = time.elapsed()?;
                latency_clone.store(latency_duration.as_micros() as u64, Ordering::Relaxed);

                sleep(Duration::from_secs(10)).await;
            }
            #[allow(unreachable_code)]
            Ok::<_, Error>(())
        };

        let (tx_out, mut rx_out) = mpsc::unbounded_channel();
        let event_future = async move {
            while let Some(msg) = rx_out.recv().await {
                event_writer.send(msg).await?;
            }

            Ok::<_, Error>(())
        };

        let log_future = async move {
            let mut log_reader = protocol::reader::<_, LogEvent>(conn.accept_uni().await?);

            while let Some(Ok(msg)) = log_reader.next().await {
                log_tx.send(msg)?;
            }
            Ok::<_, Error>(())
        };

        tokio::spawn(futures::future::try_join3(
            log_future,
            event_future,
            ping_future,
        ));

        Ok(BonfireConnectedClient {
            hw,
            key,
            latency,
            tx: tx_out,
        })
    }

    async fn handshake(
        writer: &mut BonfireWriter<BonfireMessage>,
        reader: &mut BonfireReader<BonfireMessage>,
    ) -> Result<(HardwareSpecs, Pubkey)> {
        println!("Sending challenge");
        let challenge = Challenge::new();
        writer.send(challenge.clone().into()).await?;

        let challenge_response = reader
            .next()
            .await
            .ok_or_else(|| anyhow!("Can't read challenge"))??;

        let challenge_response = challenge_response.as_challenge_response()?;
        let verified = challenge.verify(&challenge_response);

        writer.send(LoginResponse(verified).into()).await?;
        if !verified {
            return Err(anyhow!("Invalid challenge response"));
        }

        let hw = reader
            .next()
            .await
            .ok_or_else(|| anyhow!("Can't read Hardware specs"))??
            .as_client_hardware_specs()?
            .clone();
        println!("New client connected! Specs {:?}", hw);
        writer.send(SpecsAck.into()).await?;

        Ok((hw, challenge_response.public_key.into()))
    }

    fn send_event(&self, msg: Vec<BonsolInstruction>) {
        self.tx.send(msg).unwrap();
    }
}
