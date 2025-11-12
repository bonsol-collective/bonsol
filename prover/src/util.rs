use std::{
    io::{self, Write},
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use anyhow::Result;
use bytes::{Bytes, BytesMut};
use futures_util::{Stream, StreamExt};

pub async fn get_body_max_size(
    stream: impl Stream<Item = reqwest::Result<Bytes>> + 'static,
    max_size: usize,
) -> Result<Bytes> {
    let mut max = 0;
    let mut b = BytesMut::new();
    let mut stream = Box::pin(stream);
    while let Some(chunk) = stream.as_mut().next().await {
        let chunk_res = chunk?;
        let chunk = BytesMut::from(chunk_res.as_ref());
        let l = chunk.len();
        max += l;
        if max > max_size {
            return Err(anyhow::anyhow!("Max size exceeded"));
        }
        b.extend_from_slice(&chunk);
    }
    Ok(b.into())
}

#[derive(Debug, Clone)]
pub struct LogEvent {
    pub log: Vec<u8>,
    pub image_id: Arc<str>,
    pub job_id: Arc<str>,
}

pub struct LogShipper {
    image_id: Arc<str>,
    job_id: Arc<str>,
    tx: Sender<LogEvent>,
}

pub type EventChannelTx = Sender<LogEvent>;
pub type EventChannelRx = Receiver<LogEvent>;

impl LogShipper {
    pub fn new(tx: EventChannelTx, image_id: &str, job_id: &str) -> LogShipper {
        LogShipper {
            tx,
            image_id: Arc::from(image_id),
            job_id: Arc::from(job_id),
        }
    }
}

impl Write for LogShipper {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let event = LogEvent {
            log: buf.to_vec(),
            image_id: self.image_id.clone(),
            job_id: self.job_id.clone(),
        };
        self.tx
            .send(event)
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, format!("send error: {}", e)))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // mpsc has no flush mechanism — so it's a no-op
        Ok(())
    }
}
