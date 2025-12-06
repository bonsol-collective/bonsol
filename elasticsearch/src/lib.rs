use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use elasticsearch::{
    Elasticsearch,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        StatusCode,
    },
    indices::IndicesCreateParts,
    IndexParts,
    BulkParts,
    BulkOperations,
    BulkOperation,
    SearchParts,
};
use serde::{Deserialize, Serialize};
use url::Url;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogType {
    Stdout,
    Stderr,
    System,
}

// DTO for historical logs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub kind: LogType,
    pub job_id: Option<String>,
    pub image_id: Option<String>,
    pub node_id: Option<String>,
    pub meta: Option<serde_json::Value>,
}

pub struct BonsolStore {
    client: Elasticsearch,
    index_name: String,
}

impl BonsolStore{
    pub fn new(url_str:&str,index_name:&str)->Result<Self>{
        let url = Url::parse(url_str).context("Invalid ElasticSearch Url")?;

        // Single node connection pool, underlying cluster can be multi-node behind a load-balancer.
        let conn_pool = SingleNodeConnectionPool::new(url);

        let transport = TransportBuilder::new(conn_pool)
            // timeout for the request; avoid hanging on network partitions 
            .timeout(Duration::from_secs(5))
            .build()
            .context("Failed To build ElasticSearch Transport")?;

        let client = Elasticsearch::new(transport);

        Ok(Self { 
            client, 
            index_name: index_name.to_owned() 
        })
    }

    pub async fn health_check(&self)->Result<()>{
        let res = self
            .client
            .ping()
            .send()
            .await
            .context("Failed to send ping to the ElasticSearch")?;

        if !res.status_code().is_success() {
            anyhow::bail!("Elastic Search ping failed with status {}", res.status_code());
        }

        Ok(())
    }

    // Checks if the index exists and only creates if missing
    pub async fn ensure_index(&self)->Result<()>{
        let body = serde_json::json!({
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 1
            },
            "mappings": {
                "properties": {
                    "timestamp": { "type": "date" },
                    "level":     { "type": "keyword" },
                    "message":   { "type": "text" },
                    "kind":      { "type": "keyword" },
                    "job_id":    { "type": "keyword" },
                    "image_id":  { "type": "keyword" },
                    "node_id":   { "type": "keyword" },
                    "meta":      { "type": "object", "enabled": false }
                }
            }
        });

        let res = self.client
            .indices()
            .create(IndicesCreateParts::Index(&self.index_name))
            .body(body)
            .send()
            .await
        .context("Failed to send create-index request")?;

        match res.status_code(){
            StatusCode::OK | StatusCode::CREATED => Ok(()),
            StatusCode::BAD_REQUEST =>{
                // resource already exists
                Ok(())
            },
            other=> anyhow::bail!("Unexpected status creating index : {}",other),
        }
    }

    pub fn from_env_optional()-> Result<Option<BonsolStore>>{
        let url = match std::env::var("ELASTICSEARCH_URL"){
            Ok(url)=>url,
            Err(_)=> return Ok(None),
        };

        let index_name = std::env::var("ELASTICSEARCH_LOG_INDEX")
            .unwrap_or_else(|_| "bonsol_logs_v1".to_string());
        
        let store = BonsolStore::new(&url, &index_name)?;

        Ok(Some(store))
    }

    // Index Single log entry into elasticSearch
    pub async fn index_log(&self,log:&LogEntry)->Result<()>{
        let res = self
            .client
            .index(IndexParts::Index(&self.index_name))
            .body(log)
            .send()
            .await
           .context("Failed to send index request for LogEntry")?;

        if !res.status_code().is_success(){
            anyhow::bail!("Indexing log failed with status {}",res.status_code());
        }

        Ok(())
    }

    // Indexes multiple log entries in a single bulk request.
    pub async fn index_log_bulk(&self, logs: &[LogEntry]) -> Result<()> {
        if logs.is_empty() {
            return Ok(());
        }

        // Build bulk operations
        let mut ops = BulkOperations::new();

        for log in logs {
            // Create an index operation for each log
            ops.push(BulkOperation::index(log.clone()))?;
        }

        let res = self
            .client
            .bulk(BulkParts::Index(&self.index_name))
            .body(vec![ops])
            .send()
            .await
            .context("Failed to send bulk index request")?;

        if !res.status_code().is_success() {
            anyhow::bail!("Bulk indexing logs failed with status {}",res.status_code());
        }

        Ok(())
    }

}