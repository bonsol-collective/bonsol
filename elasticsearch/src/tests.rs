use crate::{BonsolStore, LogEntry, LogSearchQuery, LogType};
use chrono::Utc;

// ============================================================================
// Unit Tests (no ES required)
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_log_entry_deserialization() {
        let json = r#"{
            "timestamp": "2025-12-07T10:00:00Z",
            "level": "ERROR",
            "message": "Something went wrong",
            "kind": "stderr",
            "job_id": "job-abc",
            "image_id": null,
            "node_id": "node-xyz",
            "meta": null
        }"#;

        let entry: LogEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.level, "ERROR");
        assert_eq!(entry.message, "Something went wrong");
        assert!(matches!(entry.kind, LogType::Stderr));
        assert_eq!(entry.job_id, Some("job-abc".to_string()));
        assert!(entry.image_id.is_none());
    }

    #[test]
    fn test_log_type_serialization() {
        assert_eq!(serde_json::to_string(&LogType::Stdout).unwrap(), "\"stdout\"");
        assert_eq!(serde_json::to_string(&LogType::Stderr).unwrap(), "\"stderr\"");
        assert_eq!(serde_json::to_string(&LogType::System).unwrap(), "\"system\"");
    }

    #[test]
    fn test_log_search_query_defaults() {
        let query = LogSearchQuery::default();
        
        // Check defaults match expected values
        assert_eq!(query.page, 1);
        assert_eq!(query.limit, 50);
        assert_eq!(query.order, "desc");
        
        // These should be None
        assert!(query.source.is_none());
        assert!(query.job_id.is_none());
        assert!(query.image_id.is_none());
        assert!(query.node_id.is_none());
        assert!(query.search.is_none());
        assert!(query.level.is_none());
        assert!(query.from.is_none());
        assert!(query.to.is_none());
    }

    #[test]
    fn test_log_search_query_with_all_fields() {
        let from_time = Utc::now() - chrono::Duration::hours(24);
        let to_time = Utc::now();

        let query = LogSearchQuery {
            source: Some("stdout".to_string()),
            job_id: Some("job-prefix".to_string()),
            image_id: Some("image-prefix".to_string()),
            node_id: Some("node-123".to_string()),
            search: Some("error occurred".to_string()),
            level: Some("ERROR".to_string()),
            from: Some(from_time),
            to: Some(to_time),
            page: 3,
            limit: 25,
            order: "asc".to_string(),
        };

        assert_eq!(query.source.as_deref(), Some("stdout"));
        assert_eq!(query.job_id.as_deref(), Some("job-prefix"));
        assert_eq!(query.page, 3);
        assert_eq!(query.limit, 25);
        assert_eq!(query.order, "asc");
        assert!(query.from.is_some());
        assert!(query.to.is_some());
    }

    #[test]
    fn test_log_entry_with_special_characters_in_message() {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: "INFO".to_string(),
            message: "Message with \"quotes\" and 'apostrophes' and \nnewlines".to_string(),
            kind: LogType::Stdout,
            job_id: Some("job-with-special-chars-!@#$%".to_string()),
            image_id: None,
            node_id: None,
            meta: None,
        };

        let json = serde_json::to_string(&entry).unwrap();
        let parsed: LogEntry = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.message, entry.message);
        assert_eq!(parsed.job_id, entry.job_id);
    }

    #[test]
    fn test_log_entry_with_complex_meta() {
        let meta = serde_json::json!({
            "nested": {
                "field": "value",
                "array": [1, 2, 3],
                "deep": {
                    "deeper": {
                        "deepest": true
                    }
                }
            },
            "tags": ["tag1", "tag2", "tag3"],
            "count": 42,
            "enabled": true,
            "nullable": null
        });

        let entry = LogEntry {
            timestamp: Utc::now(),
            level: "DEBUG".to_string(),
            message: "Complex meta test".to_string(),
            kind: LogType::System,
            job_id: None,
            image_id: None,
            node_id: None,
            meta: Some(meta),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let parsed: LogEntry = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.meta.is_some());
        let parsed_meta = parsed.meta.unwrap();
        assert_eq!(parsed_meta["nested"]["field"], "value");
        assert_eq!(parsed_meta["count"], 42);
    }

    #[test]
    fn test_log_search_query_json_deserialization() {
        let json = r#"{
            "source": "stderr",
            "job_id": "job-123",
            "page": 2,
            "limit": 25,
            "order": "asc"
        }"#;

        let query: LogSearchQuery = serde_json::from_str(json).unwrap();
        
        assert_eq!(query.source.as_deref(), Some("stderr"));
        assert_eq!(query.job_id.as_deref(), Some("job-123"));
        assert_eq!(query.page, 2);
        assert_eq!(query.limit, 25);
        assert_eq!(query.order, "asc");
    }

    #[test]
    fn test_log_search_query_partial_json() {
        // Test with minimal fields - should use defaults
        let json = r#"{}"#;

        let query: LogSearchQuery = serde_json::from_str(json).unwrap();
        
        assert!(query.source.is_none());
        assert!(query.job_id.is_none());
        assert_eq!(query.page, 1);      // default
        assert_eq!(query.limit, 50);    // default
        assert_eq!(query.order, "desc"); // default
    }

    #[test]
    fn test_bonsolstore_url_parsing() {
        // Valid URLs
        assert!(BonsolStore::new("http://localhost:9200", "test").is_ok());
        assert!(BonsolStore::new("https://es.example.com:9200", "test").is_ok());
        assert!(BonsolStore::new("http://user:pass@localhost:9200", "test").is_ok());
        
        // Invalid URLs  
        assert!(BonsolStore::new("not-a-url", "test").is_err());
        assert!(BonsolStore::new("", "test").is_err());
        // Note: ftp:// URLs can be parsed but would fail on actual connection
    }
}

// ============================================================================
// Integration Tests (requires running ES)
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn get_test_store() -> Option<BonsolStore> {
        INIT.call_once(|| {
            // Initialize tracing for tests
            let _ = tracing_subscriber::fmt()
                .with_test_writer()
                .with_max_level(tracing::Level::DEBUG)
                .try_init();
        });

        let url = std::env::var("ELASTICSEARCH_URL").ok()?;
        let index = format!("bonsol_test_{}", std::process::id());
        BonsolStore::new(&url, &index).ok()
    }

    fn create_test_log(job_id: &str, message: &str, kind: LogType) -> LogEntry {
        LogEntry {
            timestamp: Utc::now(),
            level: "INFO".to_string(),
            message: message.to_string(),
            kind,
            job_id: Some(job_id.to_string()),
            image_id: Some("test-image".to_string()),
            node_id: Some("test-node".to_string()),
            meta: None,
        }
    }

    #[tokio::test]
    async fn test_health_check() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        let result = store.health_check().await;
        assert!(result.is_ok(), "Health check failed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_ensure_index_creates_index() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        let result = store.ensure_index().await;
        assert!(result.is_ok(), "Failed to create index: {:?}", result.err());

        // Calling again should succeed (index already exists)
        let result2 = store.ensure_index().await;
        assert!(result2.is_ok(), "Second ensure_index failed: {:?}", result2.err());
    }

    #[tokio::test]
    async fn test_index_single_log() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        let log = create_test_log("single-job-1", "Test single log entry", LogType::Stdout);
        let result = store.index_log(&log).await;
        
        assert!(result.is_ok(), "Failed to index log: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_index_log_bulk() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        let logs: Vec<LogEntry> = (0..10)
            .map(|i| create_test_log(
                &format!("bulk-job-{}", i),
                &format!("Bulk log message {}", i),
                if i % 2 == 0 { LogType::Stdout } else { LogType::Stderr },
            ))
            .collect();

        let result = store.index_log_bulk(&logs).await;
        assert!(result.is_ok(), "Failed to bulk index: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_index_log_bulk_empty() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        // Empty batch should succeed without making a request
        let result = store.index_log_bulk(&[]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_logs_basic() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        // Index some test data
        let logs: Vec<LogEntry> = (0..5)
            .map(|i| create_test_log(
                "search-test-job",
                &format!("Searchable message number {}", i),
                LogType::Stdout,
            ))
            .collect();
        store.index_log_bulk(&logs).await.unwrap();

        // Wait for ES to refresh
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Search
        let query = LogSearchQuery {
            job_id: Some("search-test-job".to_string()),
            limit: 10,
            ..Default::default()
        };

        let result = store.search_log(query).await;
        assert!(result.is_ok(), "Search failed: {:?}", result.err());

        let response = result.unwrap();
        assert!(response.data.len() > 0, "Expected some results");
        assert!(response.pagination.total > 0);
    }

    #[tokio::test]
    async fn test_search_logs_with_text_search() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        // Index logs with specific text
        let log = LogEntry {
            timestamp: Utc::now(),
            level: "ERROR".to_string(),
            message: "UniqueSearchableError12345".to_string(),
            kind: LogType::Stderr,
            job_id: Some("text-search-job".to_string()),
            image_id: None,
            node_id: None,
            meta: None,
        };
        store.index_log(&log).await.unwrap();

        // Wait for ES to refresh
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Search for the unique text
        let query = LogSearchQuery {
            search: Some("UniqueSearchableError12345".to_string()),
            ..Default::default()
        };

        let result = store.search_log(query).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.data.iter().any(|l| l.message.contains("UniqueSearchableError12345")));
    }

    #[tokio::test]
    async fn test_search_logs_pagination() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        // Index 15 logs
        let logs: Vec<LogEntry> = (0..15)
            .map(|i| create_test_log(
                "pagination-job",
                &format!("Pagination test message {}", i),
                LogType::Stdout,
            ))
            .collect();
        store.index_log_bulk(&logs).await.unwrap();

        // Wait for ES to refresh
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Get page 1 with limit 5
        let query = LogSearchQuery {
            job_id: Some("pagination-job".to_string()),
            page: 1,
            limit: 5,
            ..Default::default()
        };

        let result = store.search_log(query).await.unwrap();
        assert_eq!(result.data.len(), 5);
        assert_eq!(result.pagination.page, 1);
        assert_eq!(result.pagination.limit, 5);
        assert!(result.pagination.total >= 15);
    }

    #[tokio::test]
    async fn test_search_logs_filter_by_source() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        // Index stdout and stderr logs
        let stdout_log = create_test_log("source-filter-job", "stdout message", LogType::Stdout);
        let stderr_log = create_test_log("source-filter-job", "stderr message", LogType::Stderr);
        store.index_log(&stdout_log).await.unwrap();
        store.index_log(&stderr_log).await.unwrap();

        // Wait for ES to refresh
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Filter by stderr only
        let query = LogSearchQuery {
            job_id: Some("source-filter-job".to_string()),
            source: Some("stderr".to_string()),
            ..Default::default()
        };

        let result = store.search_log(query).await.unwrap();
        assert!(result.data.iter().all(|l| matches!(l.kind, LogType::Stderr)));
    }

    #[tokio::test]
    async fn test_get_logs_by_job() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        let job_id = "specific-job-12345";
        let log = create_test_log(job_id, "Job specific log", LogType::Stdout);
        store.index_log(&log).await.unwrap();

        // Wait for ES to refresh
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let result = store.get_logs_by_job(job_id).await;
        assert!(result.is_ok());

        let logs = result.unwrap();
        assert!(logs.iter().all(|l| l.job_id.as_deref() == Some(job_id)));
    }

    #[tokio::test]
    async fn test_get_logs_by_node() {
        let Some(store) = get_test_store() else {
            eprintln!("Skipping test: ELASTICSEARCH_URL not set");
            return;
        };

        store.ensure_index().await.unwrap();

        let node_id = "specific-node-67890";
        let log = LogEntry {
            timestamp: Utc::now(),
            level: "INFO".to_string(),
            message: "Node specific log".to_string(),
            kind: LogType::Stdout,
            job_id: Some("any-job".to_string()),
            image_id: None,
            node_id: Some(node_id.to_string()),
            meta: None,
        };
        store.index_log(&log).await.unwrap();

        // Wait for ES to refresh
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let result = store.get_logs_by_node(node_id, 100).await;
        assert!(result.is_ok());

        let logs = result.unwrap();
        assert!(logs.iter().all(|l| l.node_id.as_deref() == Some(node_id)));
    }
}
