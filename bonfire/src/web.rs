use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc},
};

use actix_web::{
    get,
    web::{Data, Path, Query},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::sse;
use anyhow::{Error, Result};
use bonsol_elasticsearch::{BonsolStore, LogSearchQuery, LogSearchResponse};
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


#[derive(Debug, Deserialize)]
pub struct HistoryLogsQuery {
    /// Filter by source: "stdout" or "stderr"
    pub source: Option<String>,
    /// Filter by job ID (prefix match)
    pub job_id: Option<String>,
    /// Filter by image ID (prefix match)  
    pub image_id: Option<String>,
    /// Filter by node public key
    pub node_id: Option<String>,
    /// Full-text search in log message
    pub search: Option<String>,
    /// Filter by log level
    pub level: Option<String>,
    /// Start time (ISO8601 format)
    pub from: Option<String>,
    /// End time (ISO8601 format)
    pub to: Option<String>,
    /// Page number (default: 1)
    pub page: Option<u32>,
    /// Results per page (default: 50, max: 100)
    pub limit: Option<u32>,
    /// Sort order: "asc" or "desc" (default: desc)
    pub order: Option<String>,
}

/// Response for historical logs
#[derive(Serialize)]
pub struct HistoryLogsResponse {
    pub success: bool,
    #[serde(flatten)]
    pub data: LogSearchResponse,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

/// Helper to extract BonsolStore or return a 503 error response.
fn require_es_store(
    es_store: &Data<Option<Arc<BonsolStore>>>,
) -> Result<&Arc<BonsolStore>, HttpResponse> {
    es_store.as_ref().as_ref().ok_or_else(|| {
        HttpResponse::ServiceUnavailable().json(ErrorResponse {
            success: false,
            error: "Historical logs not available - Elasticsearch not configured".to_string(),
        })
    })
}

#[get("/logs/history")]
async fn logs_history(
    es_store: Data<Option<Arc<BonsolStore>>>,
    query: Query<HistoryLogsQuery>,
) -> impl Responder {
    let store = match require_es_store(&es_store) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    // parse time filters 
    let from = query.from.as_ref().and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    let to = query.to.as_ref().and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    // Build Search Query
    let search_query = LogSearchQuery{
        source: query.source.clone(),
        job_id: query.job_id.clone(),
        image_id: query.image_id.clone(),
        node_id: query.node_id.clone(),
        search: query.search.clone(),
        level: query.level.clone(),
        from,
        to,
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(50),
        order: query.order.clone().unwrap_or_else(|| "desc".to_string()),
    };

    // executing search 
    match store.search_log(search_query).await {
        Ok(response)=> HttpResponse::Ok().json(HistoryLogsResponse{
            success:true,
            data:response
        }),
        Err(e) => {
            tracing::error!("Failed to search logs: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Search failed: {}", e),
            })
        }
    }
}


#[get("/logs/history/job/{job_id}")]
async fn logs_by_job(
    es_store: Data<Option<Arc<BonsolStore>>>,
    job_id: Path<String>,
) -> impl Responder {
    let store = match require_es_store(&es_store) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    match store.get_logs_by_job(&job_id).await {
        Ok(data) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "job_id": job_id.as_str(),
            "count": data.len(),
            "data": data
        })),
        Err(e) => {
            tracing::error!("Failed to get logs for job {}: {}", job_id, e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to get logs: {}", e),
            })
        }
    }
}

#[get("/logs/history/node/{node_id}")]
async fn logs_by_node(
    es_store: Data<Option<Arc<BonsolStore>>>,
    node_id: Path<String>,
    query: Query<HashMap<String, String>>,
) -> impl Responder {
    let store = match require_es_store(&es_store) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    let limit = query
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100)
        .min(1000);

    match store.get_logs_by_node(&node_id, limit).await {
        Ok(logs_vec) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "node_id": node_id.as_str(),
            "count": logs_vec.len(),
            "data": logs_vec
        })),
        Err(e) => {
            tracing::error!("Failed to get logs for node {}: {}", node_id, e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to get logs: {}", e),
            })
        }
    }
}

#[get("/logs/history/stats")]
async fn logs_stats(
    es_store: Data<Option<Arc<BonsolStore>>>,
) -> impl Responder {
    let store = match require_es_store(&es_store) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    // Get total count
    let query = LogSearchQuery {
        limit: 0,
        ..Default::default()
    };

    match store.search_log(query).await {
        Ok(response) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "total_logs": response.pagination.total,
            "elasticsearch_available": true
        })),
        Err(e) => {
            tracing::error!("Failed to get log stats: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to get stats: {}", e),
            })
        }
    }
}


pub async fn web_server(
    clients: BonfireClientList,
    jobs_list: Arc<Mutex<HashMap<String, Job>>>,
    log_rx: Receiver<LogEvent>,
    es_store: Option<Arc<BonsolStore>>
) -> Result<()> {
    debug!("Web thread starting...");
    let log_rx = Data::new(log_rx);
    let jobs_list = Data::new(jobs_list);
    let clients = Data::new(clients);
    let es_store = Data::new(es_store);

    HttpServer::new(move || {
        App::new()
            .service(nodes)
            .service(logs)
            .service(health)
            .service(jobs)
            .service(logs_history)
            .service(logs_by_job)
            .service(logs_by_node)
            .service(logs_stats)
            .app_data(clients.clone())
            .app_data(log_rx.clone())
            .app_data(jobs_list.clone())
            .app_data(es_store.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
