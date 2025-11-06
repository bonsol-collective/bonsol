use std::{pin::Pin, sync::Arc};

use anyhow::{Result, anyhow};
use bincode::{Decode, Encode};
use futures::{Sink, SinkExt, Stream, StreamExt};
use rand::RngCore;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

#[derive(Encode, Decode, Clone, Debug)]
pub struct BonsolInstruction {
    pub cpi: bool,
    pub accounts: Vec<Vec<u8>>,
    pub data: Vec<u8>,
    pub last_known_block: u64,
}

#[derive(Encode, Decode, Debug)]
pub enum BonfireMessage {
    /// Server → Client messages
    Server(ServerMessage),

    /// Client → Server messages
    Client(ClientMessage),
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Challenge {
    pub id: u64,
    pub nonce: [u8; 32],
}

impl Challenge {
    pub fn new() -> Self {
        let id = rand::rng().next_u64();
        let mut nonce = [0u8; 32];
        rand::rng().fill_bytes(&mut nonce);
        Self { id, nonce }
    }

    pub fn sign(&self, keypair: &Keypair) -> ChallengeResponse {
        let signed = keypair.sign_message(&self.nonce);
        ChallengeResponse {
            public_key: keypair.pubkey().to_bytes(),
            challenge_id: self.id,
            signature: signed.as_array().to_owned(),
        }
    }

    pub fn verify(&self, response: &ChallengeResponse) -> bool {
        let signature = Signature::from(response.signature);
        signature.verify(&response.public_key, &self.nonce)
    }
}

#[derive(Encode, Decode, Debug)]
pub enum ServerMessage {
    /// Server challenges client to prove identity
    Challenge(Challenge),
    /// Server confirms the client's login (challenge response) was valid
    LoginResponse(LoginResponse),
    /// Server confirms receipt of hardware specs and that the client is now fully logged in
    SpecsAck(SpecsAck),
    Ping(Ping),
}

#[derive(Encode, Decode, Debug)]
pub struct ChallengeResponse {
    pub public_key: [u8; 32], // e.g. Ed25519 public key
    pub challenge_id: u64,    // same as the server’s
    pub signature: [u8; 64],  // signature over the challenge nonce
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct LoginResponse(pub bool);

#[derive(Encode, Decode, Debug)]
pub struct SpecsAck;

#[derive(Encode, Decode, Debug)]
pub struct Ping;

#[derive(Encode, Decode, Debug)]
pub struct Pong;

#[derive(Encode, Decode, Debug, Clone)]
pub struct HardwareSpecs {
    /// CPU model or architecture string, e.g. "AMD Ryzen 7 5800X" or "Apple M2"
    pub cpu_type: String,

    /// CPU frequency in MHz
    pub cpu_mhz: u32,

    /// Number of cores
    pub cpu_cores: u32,

    /// Total system memory in bytes
    pub memory_bytes: u64,

    /// Gpu info
    pub gpus: Vec<Gpu>,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Gpu {
    /// GPU model string, e.g. "NVIDIA RTX 3080"
    pub gpu_model: String,
    /// Dedicated GPU memory in bytes
    pub gpu_memory_bytes: u64,
}

#[derive(Encode, Decode, Debug)]
pub enum ClientMessage {
    Login(ChallengeResponse),
    /// Client sends its hardware specifications
    HardwareSpecs(HardwareSpecs),
    Pong(Pong),
}

pub type BonfireWriter<P> = Pin<Box<dyn Sink<P, Error = anyhow::Error> + Send>>;
pub type BonfireReader<P> = Pin<Box<dyn Stream<Item = Result<P>> + Send>>;

pub fn writer<W, P>(w: W) -> BonfireWriter<P>
where
    W: AsyncWrite + Send + 'static,
    P: Encode + Send + 'static,
{
    Box::pin(
        FramedWrite::new(w, LengthDelimitedCodec::new()).with(|msg| async move {
            let config = bincode::config::standard();
            let bytes = bincode::encode_to_vec(msg, config)?;
            Ok::<_, anyhow::Error>(bytes.into())
        }),
    )
}

pub fn reader<R, P>(r: R) -> BonfireReader<P>
where
    R: AsyncRead + Send + 'static,
    P: Decode<()> + Send + 'static,
{
    Box::pin(FramedRead::new(r, LengthDelimitedCodec::new()).map(|buf| {
        let config = bincode::config::standard();
        let buf = buf?;
        let (msg, _bytes) = bincode::decode_from_slice(&buf, config)?;
        Ok(msg)
    }))
}

pub fn framed<W, R, P>((w, r): (W, R)) -> (BonfireWriter<P>, BonfireReader<P>)
where
    W: AsyncWrite + Send + 'static,
    R: AsyncRead + Send + 'static,
    P: Encode + Decode<()> + Send + 'static,
{
    (writer(w), reader(r))
}

// Conversions remain the same, now targeting non-archived types
impl From<Challenge> for BonfireMessage {
    fn from(challenge: Challenge) -> Self {
        BonfireMessage::Server(ServerMessage::Challenge(challenge))
    }
}

impl From<ChallengeResponse> for BonfireMessage {
    fn from(response: ChallengeResponse) -> Self {
        BonfireMessage::Client(ClientMessage::Login(response))
    }
}

impl From<HardwareSpecs> for BonfireMessage {
    fn from(specs: HardwareSpecs) -> Self {
        BonfireMessage::Client(ClientMessage::HardwareSpecs(specs))
    }
}

impl From<LoginResponse> for BonfireMessage {
    fn from(r: LoginResponse) -> Self {
        BonfireMessage::Server(ServerMessage::LoginResponse(r))
    }
}

impl From<SpecsAck> for BonfireMessage {
    fn from(_: SpecsAck) -> Self {
        BonfireMessage::Server(ServerMessage::SpecsAck(SpecsAck))
    }
}
impl From<Ping> for BonfireMessage {
    fn from(_: Ping) -> Self {
        BonfireMessage::Server(ServerMessage::Ping(Ping))
    }
}
impl From<Pong> for BonfireMessage {
    fn from(_: Pong) -> Self {
        BonfireMessage::Client(ClientMessage::Pong(Pong))
    }
}

// Helper pattern-matchers on the non-archived BonfireMessage
impl BonfireMessage {
    pub fn as_login_ok(&self) -> Result<&LoginResponse> {
        match self {
            BonfireMessage::Server(ServerMessage::LoginResponse(ok)) => Ok(ok),
            _ => Err(anyhow!("BonfireMessage is not a Server LoginOk")),
        }
    }

    pub fn as_client_hardware_specs(&self) -> Result<&HardwareSpecs> {
        match self {
            BonfireMessage::Client(ClientMessage::HardwareSpecs(specs)) => Ok(specs),
            _ => Err(anyhow!("BonfireMessage is not a Client HardwareSpecs")),
        }
    }

    pub fn as_specs_ack(&self) -> Result<&SpecsAck> {
        match self {
            BonfireMessage::Server(ServerMessage::SpecsAck(ack)) => Ok(ack),
            _ => Err(anyhow!("BonfireMessage is not a Server SpecsAck")),
        }
    }

    pub fn as_challenge(&self) -> Result<&Challenge> {
        match self {
            BonfireMessage::Server(ServerMessage::Challenge(c)) => Ok(c),
            _ => Err(anyhow!("BonfireMessage is not a Server Challenge")),
        }
    }

    pub fn as_challenge_response(&self) -> Result<&ChallengeResponse> {
        match self {
            BonfireMessage::Client(ClientMessage::Login(resp)) => Ok(resp),
            _ => Err(anyhow!("BonfireMessage is not a Client ChallengeResponse")),
        }
    }

    pub fn as_pong(&self) -> Result<&Pong> {
        match self {
            BonfireMessage::Client(ClientMessage::Pong(resp)) => Ok(resp),
            _ => Err(anyhow!("BonfireMessage is not a Client ChallengeResponse")),
        }
    }
    pub fn as_ping(&self) -> Result<&Ping> {
        match self {
            BonfireMessage::Server(ServerMessage::Ping(resp)) => Ok(resp),
            _ => Err(anyhow!("BonfireMessage is not a Client ChallengeResponse")),
        }
    }
}

#[derive(Encode, Decode, Debug, Clone, Copy)]
pub enum LogSource {
    Stdout,
    Stderr,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct LogEvent {
    pub source: LogSource,
    pub image_id: Arc<str>,
    pub job_id: Arc<str>,
    pub log: Vec<u8>,
}
