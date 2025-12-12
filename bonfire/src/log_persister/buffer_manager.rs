use std::sync::Arc;
use std::time::Duration;

use bonsol_elasticsearch::{BonsolStore, LogEntry};
use tokio::sync::mpsc;
use tracing::{debug, trace, warn};

pub struct LogBufferManager {
    log_store: Arc<BonsolStore>,
    batch_size: usize,
    flush_interval: Duration,
    channel_capacity: usize,
}

impl LogBufferManager {
    pub fn new(log_store: Arc<BonsolStore>, batch_size: usize, flush_interval: Duration) -> Self {
        Self {
            log_store,
            batch_size,
            flush_interval,
            channel_capacity: 10_000,
        }
    }

    #[allow(dead_code)]
    pub fn with_channel_capacity(mut self, capacity: usize) -> Self {
        self.channel_capacity = capacity;
        self
    }

    // Spawns the log buffer background task and returns a sender for submitting logs.
    //
    // The spawned task runs a `tokio::select!` loop that:
    // 1. Collects incoming logs from the returned channel
    // 2. Flushes to ES when batch_size is reached
    // 3. Periodically flushes partial batches based on flush_interval
    // 4. Flushes remaining logs on channel close (graceful shutdown)
    //
    // # Returns
    // An `mpsc::Sender<LogEntry>` that can be used to submit logs for persistence.
    pub fn spawn(self) -> mpsc::Sender<LogEntry> {
        let (tx, rx) = mpsc::channel(self.channel_capacity);

        tokio::spawn(self.run_buffer_loop(rx));

        tx
    }

    // buffer loop that handles log batching and flushing.
    async fn run_buffer_loop(self, mut rx: mpsc::Receiver<LogEntry>) {
        let mut batch: Vec<LogEntry> = Vec::with_capacity(self.batch_size);
        let mut interval = tokio::time::interval(self.flush_interval);

        loop {
            tokio::select! {
                log_opt = rx.recv() => {
                    match log_opt {
                        Some(log) => {
                            batch.push(log);

                            // Flush when batch is full
                            if batch.len() >= self.batch_size {
                                self.flush_batch(&mut batch).await;
                            }
                        }
                        None => {
                            // Channel closed - flush remaining logs and exit
                            if !batch.is_empty() {
                                debug!(
                                    "Channel closed, flushing {} remaining logs",
                                    batch.len()
                                );
                                self.flush_batch(&mut batch).await;
                            }
                            debug!("Log buffer manager shutting down");
                            break;
                        }
                    }
                }
                _ = interval.tick() => {
                    // Periodic flush for partial batches
                    if !batch.is_empty() {
                        trace!("Periodic flush of {} logs", batch.len());
                        self.flush_batch(&mut batch).await;
                    }
                }
            }
        }
    }

    /// Flushes the current batch to Elasticsearch.
    async fn flush_batch(&self, batch: &mut Vec<LogEntry>) {
        if batch.is_empty() {
            return;
        }

        match self.log_store.index_log_bulk(batch).await {
            Ok(_) => {
                trace!("Successfully indexed {} logs", batch.len());
            }
            Err(e) => {
                warn!("Failed to bulk index {} logs: {}", batch.len(), e);
            }
        }

        batch.clear();
    }
}
