//! Integration tests for Bonfire web endpoints
//!
//! Run with: cargo test --test web_tests
//! For integration tests with ES: ELASTICSEARCH_URL=http://localhost:9200 cargo test --test web_tests
//! 
//! To run with a live server:
//! 1. Start Elasticsearch: docker compose -f docker/docker-compose.elasticsearch.yml up -d
//! 2. Start Bonfire: cargo run --bin bonsol-bonfire
//! 3. Run tests: BONFIRE_URL=http://localhost:8080 cargo test --test web_tests -- --ignored

use std::time::Duration;

/// Test configuration
struct TestConfig {
    base_url: String,
}

impl TestConfig {
    fn new() -> Option<Self> {
        std::env::var("BONFIRE_URL").ok().map(|url| Self { base_url: url })
    }
    
    fn default_url() -> String {
        "http://localhost:8080".to_string()
    }
}

#[cfg(test)]
mod web_endpoint_tests {
    use super::*;

    /// Helper to make HTTP GET requests
    async fn get_json(url: &str) -> Result<serde_json::Value, String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| format!("Failed to create client: {}", e))?;
            
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
            
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read body: {}", e))?;
            
        if status.is_success() {
            serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse JSON: {} - Body: {}", e, body))
        } else {
            Err(format!("HTTP {}: {}", status, body))
        }
    }

    #[tokio::test]
    #[ignore = "Requires running Bonfire server"]
    async fn test_health_endpoint() {
        let base_url = TestConfig::new()
            .map(|c| c.base_url)
            .unwrap_or_else(TestConfig::default_url);
            
        let url = format!("{}/health", base_url);
        let client = reqwest::Client::new();
        
        let response = client.get(&url).send().await;
        assert!(response.is_ok(), "Health check should succeed");
        
        let resp = response.unwrap();
        assert!(resp.status().is_success(), "Health check should return 2xx");
    }

    #[tokio::test]
    #[ignore = "Requires running Bonfire server with ES"]
    async fn test_logs_history_endpoint() {
        let base_url = TestConfig::new()
            .map(|c| c.base_url)
            .unwrap_or_else(TestConfig::default_url);
            
        let url = format!("{}/logs/history", base_url);
        let result = get_json(&url).await;
        
        // Either succeeds with ES or returns 503 without ES
        match result {
            Ok(json) => {
                assert_eq!(json["success"], true, "Should indicate success");
                assert!(json["data"].is_array(), "Should have data array");
                assert!(json["pagination"].is_object(), "Should have pagination");
            }
            Err(e) if e.contains("503") => {
                // ES not configured - acceptable
                println!("ES not configured: {}", e);
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[tokio::test]
    #[ignore = "Requires running Bonfire server with ES"]
    async fn test_logs_history_with_filters() {
        let base_url = TestConfig::new()
            .map(|c| c.base_url)
            .unwrap_or_else(TestConfig::default_url);
            
        let url = format!(
            "{}/logs/history?source=stdout&level=INFO&page=1&limit=10&order=desc",
            base_url
        );
        let result = get_json(&url).await;
        
        match result {
            Ok(json) => {
                assert_eq!(json["success"], true);
                assert!(json["pagination"]["limit"].as_u64().unwrap() <= 10);
            }
            Err(e) if e.contains("503") => {
                println!("ES not configured: {}", e);
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[tokio::test]
    #[ignore = "Requires running Bonfire server with ES"]
    async fn test_logs_by_job_endpoint() {
        let base_url = TestConfig::new()
            .map(|c| c.base_url)
            .unwrap_or_else(TestConfig::default_url);
            
        let url = format!("{}/logs/history/job/test-job-123", base_url);
        let result = get_json(&url).await;
        
        match result {
            Ok(json) => {
                assert_eq!(json["success"], true);
                assert_eq!(json["job_id"], "test-job-123");
                assert!(json["data"].is_array());
            }
            Err(e) if e.contains("503") => {
                println!("ES not configured: {}", e);
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[tokio::test]
    #[ignore = "Requires running Bonfire server with ES"]
    async fn test_logs_by_node_endpoint() {
        let base_url = TestConfig::new()
            .map(|c| c.base_url)
            .unwrap_or_else(TestConfig::default_url);
            
        let url = format!("{}/logs/history/node/test-node-abc?limit=50", base_url);
        let result = get_json(&url).await;
        
        match result {
            Ok(json) => {
                assert_eq!(json["success"], true);
                assert_eq!(json["node_id"], "test-node-abc");
                assert!(json["data"].is_array());
            }
            Err(e) if e.contains("503") => {
                println!("ES not configured: {}", e);
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[tokio::test]
    #[ignore = "Requires running Bonfire server with ES"]
    async fn test_logs_stats_endpoint() {
        let base_url = TestConfig::new()
            .map(|c| c.base_url)
            .unwrap_or_else(TestConfig::default_url);
            
        let url = format!("{}/logs/history/stats", base_url);
        let result = get_json(&url).await;
        
        match result {
            Ok(json) => {
                assert_eq!(json["success"], true);
                assert!(json["total_logs"].is_number());
                assert_eq!(json["elasticsearch_available"], true);
            }
            Err(e) if e.contains("503") => {
                println!("ES not configured: {}", e);
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}

#[cfg(test)]
mod query_parameter_tests {
    // Tests for query parameter parsing and validation

    use bonsol_elasticsearch::LogSearchQuery;

    #[test]
    fn test_query_with_filters() {
        let query = LogSearchQuery {
            source: Some("stderr".to_string()),
            job_id: Some("job-123".to_string()),
            level: Some("ERROR".to_string()),
            page: 2,
            limit: 25,
            order: "asc".to_string(),
            ..Default::default()
        };

        assert_eq!(query.source.as_deref(), Some("stderr"));
        assert_eq!(query.job_id.as_deref(), Some("job-123"));
        assert_eq!(query.level.as_deref(), Some("ERROR"));
        assert_eq!(query.page, 2);
        assert_eq!(query.limit, 25);
        assert_eq!(query.order, "asc");
    }

    #[test]
    fn test_query_time_range() {
        use chrono::{TimeZone, Utc};

        let from = Utc.with_ymd_and_hms(2025, 12, 1, 0, 0, 0).unwrap();
        let to = Utc.with_ymd_and_hms(2025, 12, 7, 23, 59, 59).unwrap();

        let query = LogSearchQuery {
            from: Some(from),
            to: Some(to),
            ..Default::default()
        };

        assert!(query.from.is_some());
        assert!(query.to.is_some());
        assert!(query.from.unwrap() < query.to.unwrap());
    }
}

// ============================================================================
// Elasticsearch Store Unit Tests
// ============================================================================

#[cfg(test)]
mod elasticsearch_unit_tests {
    use bonsol_elasticsearch::{BonsolStore, LogEntry, LogSearchQuery, LogType};
    use chrono::Utc;

    #[test]
    fn test_bonsolstore_creation_valid_urls() {
        // Various valid URL formats
        let urls = vec![
            "http://localhost:9200",
            "https://elasticsearch.example.com",
            "http://es:9200",
            "https://user:password@es.cloud.com:9243",
        ];

        for url in urls {
            let result = BonsolStore::new(url, "test_index");
            assert!(result.is_ok(), "Should accept valid URL: {}", url);
        }
    }

    #[test]
    fn test_bonsolstore_creation_invalid_urls() {
        // Only truly invalid URLs that can't be parsed
        let invalid_urls = vec![
            "",
            "://missing-scheme",
        ];

        for url in invalid_urls {
            let result = BonsolStore::new(url, "test_index");
            assert!(result.is_err(), "Should reject invalid URL: {}", url);
        }
    }

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: "INFO".to_string(),
            message: "Test message".to_string(),
            kind: LogType::Stdout,
            job_id: Some("job-123".to_string()),
            image_id: Some("image-456".to_string()),
            node_id: Some("node-789".to_string()),
            meta: None,
        };

        assert_eq!(entry.level, "INFO");
        assert!(matches!(entry.kind, LogType::Stdout));
    }

    #[test]
    fn test_log_search_query_limit_bounds() {
        // Default limit
        let query = LogSearchQuery::default();
        assert_eq!(query.limit, 50);

        // Custom limit
        let query = LogSearchQuery {
            limit: 100,
            ..Default::default()
        };
        assert_eq!(query.limit, 100);
    }

    #[test]
    fn test_log_search_query_order_values() {
        let asc = LogSearchQuery {
            order: "asc".to_string(),
            ..Default::default()
        };
        assert_eq!(asc.order, "asc");

        let desc = LogSearchQuery {
            order: "desc".to_string(),
            ..Default::default()
        };
        assert_eq!(desc.order, "desc");
    }
}
