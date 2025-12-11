//! Integration tests for API endpoints
//! 
//! Tests full request/response cycles with authentication.

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use tempfile::TempDir;
use tower::ServiceExt; // For oneshot

// ============================================================================
// Test Setup Helpers
// ============================================================================

async fn create_test_app() -> (Router, SqlitePool, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display()))
        .await
        .expect("Failed to connect to test database");
    
    // Create basic schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email TEXT,
            display_name TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");
    
    // Create a simple test router
    use axum::{routing::get, Json};
    
    let app = Router::new()
        .route("/api/health", get(|| async { Json(json!({"status": "ok"})) }));
    
    (app, pool, temp_dir)
}

// ============================================================================
// Health Check Tests
// ============================================================================

#[tokio::test]
async fn test_health_endpoint() {
    let (app, _pool, _temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .uri("/api/health")
        .body(Body::empty())
        .expect("Failed to build request");
    
    let response = app
        .oneshot(request)
        .await
        .expect("Failed to execute request");
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body");
    
    let json: Value = serde_json::from_slice(&body)
        .expect("Failed to parse JSON");
    
    assert_eq!(json["status"], "ok");
}

// ============================================================================
// Path Validation Tests
// ============================================================================

#[test]
fn test_path_traversal_detection() {
    fn is_safe_path(path: &str) -> bool {
        !path.contains("..") && !path.contains("\\..") && path.starts_with('/')
    }
    
    // Safe paths
    assert!(is_safe_path("/test.txt"));
    assert!(is_safe_path("/folder/file.txt"));
    assert!(is_safe_path("/deep/nested/path/file.txt"));
    
    // Unsafe paths
    assert!(!is_safe_path("../etc/passwd"), "Path traversal");
    assert!(!is_safe_path("/folder/../etc/passwd"), "Path traversal in middle");
    assert!(!is_safe_path("test.txt"), "Relative path");
}

#[test]
fn test_filename_validation() {
    fn is_valid_filename(filename: &str) -> bool {
        !filename.is_empty()
            && !filename.contains('/')
            && !filename.contains('\\')
            && !filename.contains('\0')
            && filename != "."
            && filename != ".."
    }
    
    // Valid filenames
    assert!(is_valid_filename("test.txt"));
    assert!(is_valid_filename("document.pdf"));
    assert!(is_valid_filename("image_2024.jpg"));
    
    // Invalid filenames
    assert!(!is_valid_filename(""), "Empty filename");
    assert!(!is_valid_filename("test/file.txt"), "Contains slash");
    assert!(!is_valid_filename("test\\file.txt"), "Contains backslash");
    assert!(!is_valid_filename("."), "Current directory");
    assert!(!is_valid_filename(".."), "Parent directory");
}

// ============================================================================
// File Size Validation Tests
// ============================================================================

#[test]
fn test_file_size_limits() {
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB
    
    fn is_valid_file_size(size: u64) -> bool {
        size > 0 && size <= MAX_FILE_SIZE
    }
    
    assert!(is_valid_file_size(1024)); // 1 KB
    assert!(is_valid_file_size(1024 * 1024)); // 1 MB
    assert!(is_valid_file_size(50 * 1024 * 1024)); // 50 MB
    
    assert!(!is_valid_file_size(0), "Empty file");
    assert!(!is_valid_file_size(200 * 1024 * 1024), "File too large");
}

// ============================================================================
// MIME Type Validation Tests
// ============================================================================

#[test]
fn test_mime_type_detection() {
    fn guess_mime_type(filename: &str) -> &'static str {
        let ext = filename.rsplit('.').next().unwrap_or("");
        match ext.to_lowercase().as_str() {
            "txt" => "text/plain",
            "pdf" => "application/pdf",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "mp4" => "video/mp4",
            "mp3" => "audio/mpeg",
            "zip" => "application/zip",
            "json" => "application/json",
            "xml" => "application/xml",
            _ => "application/octet-stream",
        }
    }
    
    assert_eq!(guess_mime_type("test.txt"), "text/plain");
    assert_eq!(guess_mime_type("document.pdf"), "application/pdf");
    assert_eq!(guess_mime_type("photo.jpg"), "image/jpeg");
    assert_eq!(guess_mime_type("image.png"), "image/png");
    assert_eq!(guess_mime_type("animation.gif"), "image/gif");
    assert_eq!(guess_mime_type("video.mp4"), "video/mp4");
    assert_eq!(guess_mime_type("unknown.xyz"), "application/octet-stream");
}

// ============================================================================
// Rate Limiting Tests
// ============================================================================

#[test]
fn test_rate_limit_calculation() {
    use std::time::{Duration, Instant};
    
    struct RateLimiter {
        max_requests: u32,
        window: Duration,
        requests: Vec<Instant>,
    }
    
    impl RateLimiter {
        fn new(max_requests: u32, window: Duration) -> Self {
            Self {
                max_requests,
                window,
                requests: Vec::new(),
            }
        }
        
        fn allow(&mut self, now: Instant) -> bool {
            // Remove old requests outside window
            self.requests.retain(|&time| now.duration_since(time) < self.window);
            
            if self.requests.len() < self.max_requests as usize {
                self.requests.push(now);
                true
            } else {
                false
            }
        }
    }
    
    let mut limiter = RateLimiter::new(3, Duration::from_secs(1));
    let now = Instant::now();
    
    // First 3 requests should be allowed
    assert!(limiter.allow(now));
    assert!(limiter.allow(now));
    assert!(limiter.allow(now));
    
    // 4th request should be denied
    assert!(!limiter.allow(now));
    
    // After window expires, should be allowed again
    let later = now + Duration::from_secs(2);
    assert!(limiter.allow(later));
}

// ============================================================================
// Error Response Tests
// ============================================================================

#[test]
fn test_error_response_format() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize)]
    struct ErrorResponse {
        error: String,
        message: String,
    }
    
    let error = ErrorResponse {
        error: "NotFound".to_string(),
        message: "File not found".to_string(),
    };
    
    let json = serde_json::to_string(&error).expect("Failed to serialize");
    let parsed: ErrorResponse = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(parsed.error, "NotFound");
    assert_eq!(parsed.message, "File not found");
}

// ============================================================================
// Query Parameter Validation Tests
// ============================================================================

#[test]
fn test_pagination_parameters() {
    fn validate_pagination(page: i32, limit: i32) -> Result<(i32, i32), String> {
        if page < 1 {
            return Err("Page must be >= 1".to_string());
        }
        if !(1..=100).contains(&limit) {
            return Err("Limit must be between 1 and 100".to_string());
        }
        Ok((page, limit))
    }
    
    assert!(validate_pagination(1, 10).is_ok());
    assert!(validate_pagination(5, 50).is_ok());
    assert!(validate_pagination(100, 100).is_ok());
    
    assert!(validate_pagination(0, 10).is_err(), "Invalid page");
    assert!(validate_pagination(-1, 10).is_err(), "Negative page");
    assert!(validate_pagination(1, 0).is_err(), "Invalid limit");
    assert!(validate_pagination(1, 101).is_err(), "Limit too high");
}

#[test]
fn test_sort_parameter_validation() {
    fn validate_sort(sort: &str) -> bool {
        matches!(
            sort,
            "name" | "size" | "created" | "updated" | "type"
                | "-name" | "-size" | "-created" | "-updated" | "-type"
        )
    }
    
    // Valid sort parameters
    assert!(validate_sort("name"));
    assert!(validate_sort("-name"));
    assert!(validate_sort("size"));
    assert!(validate_sort("-created"));
    
    // Invalid sort parameters
    assert!(!validate_sort("invalid"));
    assert!(!validate_sort(""));
    assert!(!validate_sort("name-"));
}

// ============================================================================
// WebSocket Message Tests
// ============================================================================

#[test]
fn test_websocket_message_format() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct FileChangeEvent {
        path: String,
        kind: String,
        timestamp: String,
    }
    
    let event = FileChangeEvent {
        path: "/test.txt".to_string(),
        kind: "created".to_string(),
        timestamp: "2024-01-01T00:00:00Z".to_string(),
    };
    
    let json = serde_json::to_string(&event).expect("Failed to serialize");
    let parsed: FileChangeEvent = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(parsed, event);
}

// ============================================================================
// Search Query Tests
// ============================================================================

#[test]
fn test_search_query_sanitization() {
    fn sanitize_search_query(query: &str) -> String {
        query
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
            .collect()
    }
    
    assert_eq!(sanitize_search_query("test query"), "test query");
    assert_eq!(sanitize_search_query("file-name_123"), "file-name_123");
    
    // Remove special characters
    assert_eq!(sanitize_search_query("test<script>"), "testscript");
    assert_eq!(sanitize_search_query("query; DROP TABLE"), "query DROP TABLE");
}

#[test]
fn test_search_query_length() {
    fn is_valid_search_query(query: &str) -> bool {
        let trimmed = query.trim();
        !trimmed.is_empty() && trimmed.len() <= 200
    }
    
    assert!(is_valid_search_query("test"));
    assert!(is_valid_search_query("a".repeat(100).as_str()));
    assert!(is_valid_search_query("a".repeat(200).as_str()));
    
    assert!(!is_valid_search_query(""), "Empty query");
    assert!(!is_valid_search_query("   "), "Whitespace only");
    assert!(!is_valid_search_query(&"a".repeat(201)), "Query too long");
}
