use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc},
};

use actix_web::{
    get,
    web::{Data, Query},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::sse;
use anyhow::{Error, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast::Receiver, Mutex};
use tokio_stream::wrappers::BroadcastStream;
use tracing::debug;

use crate::{
    protocol::{HardwareSpecs, LogEvent},
    BonfireClientList, Job,
};

#[derive(Serialize)]
struct Node {
    pubkey: String,
    hw: HardwareSpecs,
    latency: u64,
}

#[get("/nodes")]
async fn nodes(clients: Data<BonfireClientList>) -> impl Responder {
    HttpResponse::Ok().json(
        clients
            .get_all()
            .await
            .iter()
            .map(|client| Node {
                pubkey: client.pubkey.to_string(),
                latency: client.latency.load(Ordering::Relaxed),
                hw: client.hw.clone(),
            })
            .collect::<Vec<_>>(),
    )
}

#[get("/jobs")]
async fn jobs(jobs: Data<Arc<Mutex<HashMap<String, Job>>>>) -> impl Responder {
    HttpResponse::Ok().json(&jobs.lock().await.values().collect::<Vec<_>>())
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

#[get("/health")]
async fn health() -> impl Responder {
    "Healthy!"
}

pub async fn web_server(
    clients: BonfireClientList,
    jobs_list: Arc<Mutex<HashMap<String, Job>>>,
    log_rx: Receiver<LogEvent>,
) -> Result<()> {
    debug!("Web thread starting...");
    let log_rx = Data::new(log_rx);
    let jobs_list = Data::new(jobs_list);
    let clients = Data::new(clients);
    HttpServer::new(move || {
        App::new()
            .service(nodes)
            .service(logs)
            .service(health)
            .service(jobs)
            .app_data(clients.clone())
            .app_data(log_rx.clone())
            .app_data(jobs_list.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
