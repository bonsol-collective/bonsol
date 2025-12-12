use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

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
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub kind: LogType,
    pub job_id: Option<String>,
    pub image_id: Option<String>,
    pub node_id: Option<String>,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogSearchQuery {
    /// Filter by log source (stdout/stderr)
    pub source: Option<String>,
    /// Filter by job ID (prefix match)
    pub job_id: Option<String>,
    /// Filter by image ID (prefix match)
    pub image_id: Option<String>,
    /// Filter by node ID
    pub node_id: Option<String>,
    /// Full-text search in message
    pub search: Option<String>,
    /// Start time (ISO8601)
    pub from: Option<DateTime<Utc>>,
    /// End time (ISO8601)
    pub to: Option<DateTime<Utc>>,
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: u32,
    /// Results per page (max 100)
    #[serde(default = "default_limit")]
    pub limit: u32,
    /// Sort order: "asc" or "desc"
    #[serde(default = "default_order")]
    pub order: String,
}

impl Default for LogSearchQuery {
    fn default() -> Self {
        Self {
            source: None,
            job_id: None,
            image_id: None,
            node_id: None,
            search: None,
            from: None,
            to: None,
            page: default_page(),
            limit: default_limit(),
            order: default_order(),
        }
    }
}

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    50
}
fn default_order() -> String {
    "desc".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct LogSearchResponse {
    pub data: Vec<LogEntry>,
    pub pagination: Pagination,
}
