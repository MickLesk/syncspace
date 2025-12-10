/// Prometheus metrics endpoint
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Metrics collector for Prometheus
#[derive(Clone)]
pub struct MetricsCollector {
    pub http_requests_total: Arc<AtomicU64>,
    pub http_requests_errors: Arc<AtomicU64>,
    pub file_uploads_total: Arc<AtomicU64>,
    pub file_downloads_total: Arc<AtomicU64>,
    pub file_deletes_total: Arc<AtomicU64>,
    pub bytes_uploaded_total: Arc<AtomicU64>,
    pub bytes_downloaded_total: Arc<AtomicU64>,
    pub active_users: Arc<AtomicU64>,
    pub websocket_connections: Arc<AtomicU64>,
    pub cache_hits: Arc<AtomicU64>,
    pub cache_misses: Arc<AtomicU64>,
    pub database_queries_total: Arc<AtomicU64>,
    pub database_query_errors: Arc<AtomicU64>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            http_requests_total: Arc::new(AtomicU64::new(0)),
            http_requests_errors: Arc::new(AtomicU64::new(0)),
            file_uploads_total: Arc::new(AtomicU64::new(0)),
            file_downloads_total: Arc::new(AtomicU64::new(0)),
            file_deletes_total: Arc::new(AtomicU64::new(0)),
            bytes_uploaded_total: Arc::new(AtomicU64::new(0)),
            bytes_downloaded_total: Arc::new(AtomicU64::new(0)),
            active_users: Arc::new(AtomicU64::new(0)),
            websocket_connections: Arc::new(AtomicU64::new(0)),
            cache_hits: Arc::new(AtomicU64::new(0)),
            cache_misses: Arc::new(AtomicU64::new(0)),
            database_queries_total: Arc::new(AtomicU64::new(0)),
            database_query_errors: Arc::new(AtomicU64::new(0)),
        }
    }
    
    /// Increment HTTP request counter
    pub fn inc_http_requests(&self) {
        self.http_requests_total.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Increment HTTP error counter
    pub fn inc_http_errors(&self) {
        self.http_requests_errors.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Increment file upload counter
    pub fn inc_file_uploads(&self) {
        self.file_uploads_total.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Increment file download counter
    pub fn inc_file_downloads(&self) {
        self.file_downloads_total.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Increment file delete counter
    pub fn inc_file_deletes(&self) {
        self.file_deletes_total.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Add uploaded bytes
    pub fn add_bytes_uploaded(&self, bytes: u64) {
        self.bytes_uploaded_total.fetch_add(bytes, Ordering::Relaxed);
    }
    
    /// Add downloaded bytes
    pub fn add_bytes_downloaded(&self, bytes: u64) {
        self.bytes_downloaded_total.fetch_add(bytes, Ordering::Relaxed);
    }
    
    /// Set active users count
    pub fn set_active_users(&self, count: u64) {
        self.active_users.store(count, Ordering::Relaxed);
    }
    
    /// Increment WebSocket connections
    pub fn inc_websocket_connections(&self) {
        self.websocket_connections.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Decrement WebSocket connections
    pub fn dec_websocket_connections(&self) {
        self.websocket_connections.fetch_sub(1, Ordering::Relaxed);
    }
    
    /// Increment cache hits
    pub fn inc_cache_hits(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Increment cache misses
    pub fn inc_cache_misses(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Increment database queries
    pub fn inc_database_queries(&self) {
        self.database_queries_total.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Increment database query errors
    pub fn inc_database_errors(&self) {
        self.database_query_errors.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Export metrics in Prometheus format
    pub fn export_prometheus(&self) -> String {
        format!(
            "# HELP http_requests_total Total number of HTTP requests\n\
             # TYPE http_requests_total counter\n\
             http_requests_total {}\n\
             \n\
             # HELP http_requests_errors Total number of HTTP errors\n\
             # TYPE http_requests_errors counter\n\
             http_requests_errors {}\n\
             \n\
             # HELP file_uploads_total Total number of file uploads\n\
             # TYPE file_uploads_total counter\n\
             file_uploads_total {}\n\
             \n\
             # HELP file_downloads_total Total number of file downloads\n\
             # TYPE file_downloads_total counter\n\
             file_downloads_total {}\n\
             \n\
             # HELP file_deletes_total Total number of file deletions\n\
             # TYPE file_deletes_total counter\n\
             file_deletes_total {}\n\
             \n\
             # HELP bytes_uploaded_total Total bytes uploaded\n\
             # TYPE bytes_uploaded_total counter\n\
             bytes_uploaded_total {}\n\
             \n\
             # HELP bytes_downloaded_total Total bytes downloaded\n\
             # TYPE bytes_downloaded_total counter\n\
             bytes_downloaded_total {}\n\
             \n\
             # HELP active_users Current number of active users\n\
             # TYPE active_users gauge\n\
             active_users {}\n\
             \n\
             # HELP websocket_connections Current number of WebSocket connections\n\
             # TYPE websocket_connections gauge\n\
             websocket_connections {}\n\
             \n\
             # HELP cache_hits_total Total cache hits\n\
             # TYPE cache_hits_total counter\n\
             cache_hits_total {}\n\
             \n\
             # HELP cache_misses_total Total cache misses\n\
             # TYPE cache_misses_total counter\n\
             cache_misses_total {}\n\
             \n\
             # HELP database_queries_total Total database queries\n\
             # TYPE database_queries_total counter\n\
             database_queries_total {}\n\
             \n\
             # HELP database_query_errors Total database query errors\n\
             # TYPE database_query_errors counter\n\
             database_query_errors {}\n",
            self.http_requests_total.load(Ordering::Relaxed),
            self.http_requests_errors.load(Ordering::Relaxed),
            self.file_uploads_total.load(Ordering::Relaxed),
            self.file_downloads_total.load(Ordering::Relaxed),
            self.file_deletes_total.load(Ordering::Relaxed),
            self.bytes_uploaded_total.load(Ordering::Relaxed),
            self.bytes_downloaded_total.load(Ordering::Relaxed),
            self.active_users.load(Ordering::Relaxed),
            self.websocket_connections.load(Ordering::Relaxed),
            self.cache_hits.load(Ordering::Relaxed),
            self.cache_misses.load(Ordering::Relaxed),
            self.database_queries_total.load(Ordering::Relaxed),
            self.database_query_errors.load(Ordering::Relaxed),
        )
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Example middleware for tracking HTTP requests (for Axum)
/*
Usage in main.rs:

use axum::{
    middleware::{self, Next},
    response::Response,
};
use http::Request;

async fn metrics_middleware<B>(
    metrics: Arc<MetricsCollector>,
    req: Request<B>,
    next: Next<B>,
) -> Response {
    metrics.inc_http_requests();
    
    let response = next.run(req).await;
    
    if !response.status().is_success() {
        metrics.inc_http_errors();
    }
    
    response
}

// In router:
let app = Router::new()
    .route("/metrics", get(metrics_handler))
    .layer(middleware::from_fn(metrics_middleware))
    .with_state(app_state);
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub http_requests_total: u64,
    pub http_requests_errors: u64,
    pub file_uploads_total: u64,
    pub file_downloads_total: u64,
    pub file_deletes_total: u64,
    pub bytes_uploaded_total: u64,
    pub bytes_downloaded_total: u64,
    pub active_users: u64,
    pub websocket_connections: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub database_queries_total: u64,
    pub database_query_errors: u64,
    pub cache_hit_rate: f64,
}

impl MetricsCollector {
    /// Get a snapshot of current metrics as JSON
    pub fn snapshot(&self) -> MetricsSnapshot {
        let cache_hits = self.cache_hits.load(Ordering::Relaxed);
        let cache_misses = self.cache_misses.load(Ordering::Relaxed);
        let cache_total = cache_hits + cache_misses;
        let cache_hit_rate = if cache_total > 0 {
            cache_hits as f64 / cache_total as f64
        } else {
            0.0
        };
        
        MetricsSnapshot {
            http_requests_total: self.http_requests_total.load(Ordering::Relaxed),
            http_requests_errors: self.http_requests_errors.load(Ordering::Relaxed),
            file_uploads_total: self.file_uploads_total.load(Ordering::Relaxed),
            file_downloads_total: self.file_downloads_total.load(Ordering::Relaxed),
            file_deletes_total: self.file_deletes_total.load(Ordering::Relaxed),
            bytes_uploaded_total: self.bytes_uploaded_total.load(Ordering::Relaxed),
            bytes_downloaded_total: self.bytes_downloaded_total.load(Ordering::Relaxed),
            active_users: self.active_users.load(Ordering::Relaxed),
            websocket_connections: self.websocket_connections.load(Ordering::Relaxed),
            cache_hits,
            cache_misses,
            database_queries_total: self.database_queries_total.load(Ordering::Relaxed),
            database_query_errors: self.database_query_errors.load(Ordering::Relaxed),
            cache_hit_rate,
        }
    }
}
