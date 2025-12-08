use std::time::Duration;

use anyhow::{Context, Result};
use elasticsearch::{
    BulkOperation, BulkOperations, BulkParts, Elasticsearch, IndexParts, SearchParts,
    http::{StatusCode, transport::{SingleNodeConnectionPool, TransportBuilder}},
    indices::IndicesCreateParts,
};
use url::Url;

use crate::{LogEntry, LogSearchQuery, LogSearchResponse, Pagination};


pub struct BonsolStore {
    client: Elasticsearch,
    index_name: String,
}

impl BonsolStore {
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

        match res.status_code() {
            StatusCode::OK | StatusCode::CREATED => {
                tracing::info!("Created Elasticsearch index: {}", self.index_name);
                Ok(())
            },
            StatusCode::BAD_REQUEST => {
                // Index already exists - this is fine
                tracing::debug!("Index {} already exists", self.index_name);
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

    pub async fn search_log(&self,query:LogSearchQuery)->Result<LogSearchResponse>{
        let page = query.page.max(1);
        let limit = query.limit.min(100).max(1);
        let from = ((page-1)*limit) as i64;

        let order = if query.order == "asc" { "asc" } else { "desc" };

        // ElasticSearch Query
        let mut must_clauses:Vec<serde_json::Value> = Vec::new();
        let mut filter_clauses:Vec<serde_json::Value> = Vec::new();

        // Full text search on message 
        if let Some(ref search_text) = query.search {
            must_clauses.push(serde_json::json!({
                "match":{
                    "message":{
                        "query": search_text,
                        "operator":"and"
                    }
                }
            }));
        }

        // Filter By Source
        if let Some(ref source) = query.source {
            filter_clauses.push(serde_json::json!({
                "term":{
                    "kind": source.to_lowercase()
                }
            }));
        }

        // Filter By Job Id (prefix match)
        if let Some(ref job_id) = query.job_id {
            filter_clauses.push(serde_json::json!({
                "prefix":{
                    "job_id": job_id
                }
            }));
        }

        // Filter by Image Id (prefix match)
        if let Some(ref image_id) = query.image_id{
            filter_clauses.push(serde_json::json!({
                "prefix":{
                    "image_id":image_id
                }
            }));
        }

        // Filter by node id 
        if let Some(ref node_id) = query.node_id{
            filter_clauses.push(serde_json::json!({
                "term":{
                    "node_id":node_id
                }
            }));
        }

        // Filter by level
        if let Some(ref level) = query.level {
            filter_clauses.push(serde_json::json!({
                "term":{
                    "level":level.to_uppercase()
                }
            }));
        }

        // Time range filter
        let mut range_filter = serde_json::Map::new();

        if let Some(from_time) = query.from {
            range_filter.insert("gte".to_string(), serde_json::json!(from_time.to_rfc3339()));
        }

        if let Some(to_time) = query.to {
            range_filter.insert("lte".to_string(), serde_json::json!(to_time.to_rfc3339()));
        }
        if !range_filter.is_empty() {
            filter_clauses.push(serde_json::json!({
                "range": {
                    "timestamp": range_filter
                }
            }));
        }

        // Final Query 
        let es_query = serde_json::json!({
            "query":{
                "bool":{
                    "must" : if must_clauses.is_empty(){
                        vec![serde_json::json!({
                            "match_all":{}
                        })]
                    }else{
                        must_clauses
                    },
                    "filter":filter_clauses
                }
            },
            "sort":[
                {
                    "timestamp": {
                        "order": order
                    }
                }
            ],
            "from":from,
            "size":limit,
            "track_total_hits":true
        });

        let response = self.client
            .search(SearchParts::Index(&[&self.index_name]))
            .body(es_query)
            .send()
            .await
        .context("Failed to execute search query")?;

        let status = response.status_code();

        let response_body: serde_json::Value = response.json().await
            .context("Failed to parse search response")?;

        if !status.is_success() {
            anyhow::bail!("Search failed with status {}: {:?}", status, response_body);
        }

        // Parsing hits 
        let hits = response_body["hits"]["hits"]
            .as_array()
            .map(|arr| arr.to_vec())
            .unwrap_or_default();

        let total = response_body["hits"]["total"]["value"]
            .as_u64()
            .unwrap_or(0);

        let took_ms = response_body["took"]
            .as_u64()
            .unwrap_or(0);

        // converting hits to LogEntry 
        let data: Vec<LogEntry> = hits
            .iter()
            .filter_map(|h| {
                serde_json::from_value(h["_source"].clone()).ok()
            })
            .collect();

        let total_pages = ((total as f64)/(limit as f64)).ceil() as u32;

        Ok(LogSearchResponse { 
            data, 
            pagination: Pagination { 
                page, 
                limit, 
                total, 
                total_pages 
            }, 
            took_ms 
        })

    }

    // Get logs for a specific job
    pub async fn get_logs_by_job(&self, job_id:&str)-> Result<Vec<LogEntry>>{
        let query = LogSearchQuery{
            job_id: Some(job_id.to_string()),
            limit: 1000,
            order:"asc".to_string(),
            ..Default::default()
        };

        let response = self.search_log(query).await?;

        Ok(response.data)
    }

    // Get Logs for a specific node 
    pub async fn get_logs_by_node(&self,node_id:&str,limit:u32)->Result<Vec<LogEntry>>{
        let query = LogSearchQuery{
            node_id : Some(node_id.to_string()),
            limit,
            order:"desc".to_string(),
            ..Default::default()
        };

        let response = self.search_log(query).await?;
        Ok(response.data)
    }

}