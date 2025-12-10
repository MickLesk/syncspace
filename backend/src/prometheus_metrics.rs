/// Prometheus metrics endpoint and collector
/// 
/// This module provides:
/// - MetricsCollector for tracking application metrics
/// - Prometheus-format endpoint for scraping
/// - JSON snapshot for internal use
/// - Request/response timing middleware

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::AppState;

/// Metrics collector for tracking application performance
#[derive(Clone)]
pub struct MetricsCollector {
    // HTTP metrics
    pub http_requests_total: Arc<AtomicU64>,
    pub http_requests_success: Arc<AtomicU64>,
    pub http_requests_errors: Arc<AtomicU64>,
    pub http_request_duration_ms_sum: Arc<AtomicU64>,
    
    // File operation metrics
    pub file_uploads_total: Arc<AtomicU64>,
    pub file_downloads_total: Arc<AtomicU64>,
    pub file_deletes_total: Arc<AtomicU64>,
    pub bytes_uploaded_total: Arc<AtomicU64>,
    pub bytes_downloaded_total: Arc<AtomicU64>,
    
    // User metrics
    pub active_users: Arc<AtomicU64>,
    pub total_users: Arc<AtomicU64>,
    pub logins_total: Arc<AtomicU64>,
    pub login_failures_total: Arc<AtomicU64>,
    
    // WebSocket metrics
    pub websocket_connections: Arc<AtomicU64>,
    pub websocket_messages_sent: Arc<AtomicU64>,
    pub websocket_messages_received: Arc<AtomicU64>,
    
    // Cache metrics
    pub cache_hits: Arc<AtomicU64>,
    pub cache_misses: Arc<AtomicU64>,
    
    // Database metrics
    pub database_queries_total: Arc<AtomicU64>,
    pub database_query_errors: Arc<AtomicU64>,
    pub database_query_duration_ms_sum: Arc<AtomicU64>,
    
    // Search metrics
    pub search_queries_total: Arc<AtomicU64>,
    pub search_results_total: Arc<AtomicU64>,
    
    // Storage metrics
    pub storage_used_bytes: Arc<AtomicU64>,
    pub storage_quota_bytes: Arc<AtomicU64>,
    pub files_count: Arc<AtomicU64>,
    pub folders_count: Arc<AtomicU64>,
    
    // Background job metrics
    pub jobs_queued: Arc<AtomicU64>,
    pub jobs_completed: Arc<AtomicU64>,
    pub jobs_failed: Arc<AtomicU64>,
    
    // Start time for uptime calculation
    start_time: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            http_requests_total: Arc::new(AtomicU64::new(0)),
            http_requests_success: Arc::new(AtomicU64::new(0)),
            http_requests_errors: Arc::new(AtomicU64::new(0)),
            http_request_duration_ms_sum: Arc::new(AtomicU64::new(0)),
            
            file_uploads_total: Arc::new(AtomicU64::new(0)),
            file_downloads_total: Arc::new(AtomicU64::new(0)),
            file_deletes_total: Arc::new(AtomicU64::new(0)),
            bytes_uploaded_total: Arc::new(AtomicU64::new(0)),
            bytes_downloaded_total: Arc::new(AtomicU64::new(0)),
            
            active_users: Arc::new(AtomicU64::new(0)),
            total_users: Arc::new(AtomicU64::new(0)),
            logins_total: Arc::new(AtomicU64::new(0)),
            login_failures_total: Arc::new(AtomicU64::new(0)),
            
            websocket_connections: Arc::new(AtomicU64::new(0)),
            websocket_messages_sent: Arc::new(AtomicU64::new(0)),
            websocket_messages_received: Arc::new(AtomicU64::new(0)),
            
            cache_hits: Arc::new(AtomicU64::new(0)),
            cache_misses: Arc::new(AtomicU64::new(0)),
            
            database_queries_total: Arc::new(AtomicU64::new(0)),
            database_query_errors: Arc::new(AtomicU64::new(0)),
            database_query_duration_ms_sum: Arc::new(AtomicU64::new(0)),
            
            search_queries_total: Arc::new(AtomicU64::new(0)),
            search_results_total: Arc::new(AtomicU64::new(0)),
            
            storage_used_bytes: Arc::new(AtomicU64::new(0)),
            storage_quota_bytes: Arc::new(AtomicU64::new(0)),
            files_count: Arc::new(AtomicU64::new(0)),
            folders_count: Arc::new(AtomicU64::new(0)),
            
            jobs_queued: Arc::new(AtomicU64::new(0)),
            jobs_completed: Arc::new(AtomicU64::new(0)),
            jobs_failed: Arc::new(AtomicU64::new(0)),
            
            start_time: Instant::now(),
        }
    }
    
    // HTTP metrics
    pub fn inc_http_requests(&self) {
        self.http_requests_total.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_http_success(&self) {
        self.http_requests_success.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_http_errors(&self) {
        self.http_requests_errors.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn add_http_duration(&self, duration_ms: u64) {
        self.http_request_duration_ms_sum.fetch_add(duration_ms, Ordering::Relaxed);
    }
    
    // File metrics
    pub fn inc_file_uploads(&self) {
        self.file_uploads_total.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_file_downloads(&self) {
        self.file_downloads_total.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_file_deletes(&self) {
        self.file_deletes_total.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn add_bytes_uploaded(&self, bytes: u64) {
        self.bytes_uploaded_total.fetch_add(bytes, Ordering::Relaxed);
    }
    
    pub fn add_bytes_downloaded(&self, bytes: u64) {
        self.bytes_downloaded_total.fetch_add(bytes, Ordering::Relaxed);
    }
    
    // User metrics
    pub fn set_active_users(&self, count: u64) {
        self.active_users.store(count, Ordering::Relaxed);
    }
    
    pub fn set_total_users(&self, count: u64) {
        self.total_users.store(count, Ordering::Relaxed);
    }
    
    pub fn inc_logins(&self) {
        self.logins_total.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_login_failures(&self) {
        self.login_failures_total.fetch_add(1, Ordering::Relaxed);
    }
    
    // WebSocket metrics
    pub fn inc_websocket_connections(&self) {
        self.websocket_connections.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn dec_websocket_connections(&self) {
        let current = self.websocket_connections.load(Ordering::Relaxed);
        if current > 0 {
            self.websocket_connections.fetch_sub(1, Ordering::Relaxed);
        }
    }
    
    pub fn inc_websocket_messages_sent(&self) {
        self.websocket_messages_sent.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_websocket_messages_received(&self) {
        self.websocket_messages_received.fetch_add(1, Ordering::Relaxed);
    }
    
    // Cache metrics
    pub fn inc_cache_hits(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_cache_misses(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }
    
    // Database metrics
    pub fn inc_database_queries(&self) {
        self.database_queries_total.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_database_errors(&self) {
        self.database_query_errors.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn add_database_duration(&self, duration_ms: u64) {
        self.database_query_duration_ms_sum.fetch_add(duration_ms, Ordering::Relaxed);
    }
    
    // Search metrics
    pub fn inc_search_queries(&self) {
        self.search_queries_total.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn add_search_results(&self, count: u64) {
        self.search_results_total.fetch_add(count, Ordering::Relaxed);
    }
    
    // Storage metrics
    pub fn set_storage_used(&self, bytes: u64) {
        self.storage_used_bytes.store(bytes, Ordering::Relaxed);
    }
    
    pub fn set_storage_quota(&self, bytes: u64) {
        self.storage_quota_bytes.store(bytes, Ordering::Relaxed);
    }
    
    pub fn set_files_count(&self, count: u64) {
        self.files_count.store(count, Ordering::Relaxed);
    }
    
    pub fn set_folders_count(&self, count: u64) {
        self.folders_count.store(count, Ordering::Relaxed);
    }
    
    // Job metrics
    pub fn inc_jobs_queued(&self) {
        self.jobs_queued.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_jobs_completed(&self) {
        self.jobs_completed.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn inc_jobs_failed(&self) {
        self.jobs_failed.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
    
    /// Export metrics in Prometheus format
    pub fn export_prometheus(&self) -> String {
        let uptime = self.uptime_seconds();
        let http_total = self.http_requests_total.load(Ordering::Relaxed);
        let http_duration = self.http_request_duration_ms_sum.load(Ordering::Relaxed);
        let avg_http_duration = if http_total > 0 { http_duration / http_total } else { 0 };
        
        let cache_hits = self.cache_hits.load(Ordering::Relaxed);
        let cache_misses = self.cache_misses.load(Ordering::Relaxed);
        let cache_total = cache_hits + cache_misses;
        let cache_hit_rate = if cache_total > 0 { 
            (cache_hits as f64 / cache_total as f64 * 100.0) as u64 
        } else { 0 };
        
        format!(
r#"# HELP syncspace_uptime_seconds Time since server started
# TYPE syncspace_uptime_seconds gauge
syncspace_uptime_seconds {uptime}

# HELP http_requests_total Total number of HTTP requests
# TYPE http_requests_total counter
http_requests_total {http_total}

# HELP http_requests_success Total number of successful HTTP requests
# TYPE http_requests_success counter
http_requests_success {}

# HELP http_requests_errors Total number of HTTP errors
# TYPE http_requests_errors counter
http_requests_errors {}

# HELP http_request_duration_ms_avg Average HTTP request duration in milliseconds
# TYPE http_request_duration_ms_avg gauge
http_request_duration_ms_avg {avg_http_duration}

# HELP file_uploads_total Total number of file uploads
# TYPE file_uploads_total counter
file_uploads_total {}

# HELP file_downloads_total Total number of file downloads
# TYPE file_downloads_total counter
file_downloads_total {}

# HELP file_deletes_total Total number of file deletions
# TYPE file_deletes_total counter
file_deletes_total {}

# HELP bytes_uploaded_total Total bytes uploaded
# TYPE bytes_uploaded_total counter
bytes_uploaded_total {}

# HELP bytes_downloaded_total Total bytes downloaded
# TYPE bytes_downloaded_total counter
bytes_downloaded_total {}

# HELP active_users Current number of active users
# TYPE active_users gauge
active_users {}

# HELP total_users Total number of users
# TYPE total_users gauge
total_users {}

# HELP logins_total Total number of successful logins
# TYPE logins_total counter
logins_total {}

# HELP login_failures_total Total number of failed login attempts
# TYPE login_failures_total counter
login_failures_total {}

# HELP websocket_connections Current number of WebSocket connections
# TYPE websocket_connections gauge
websocket_connections {}

# HELP websocket_messages_sent Total WebSocket messages sent
# TYPE websocket_messages_sent counter
websocket_messages_sent {}

# HELP websocket_messages_received Total WebSocket messages received
# TYPE websocket_messages_received counter
websocket_messages_received {}

# HELP cache_hits_total Total cache hits
# TYPE cache_hits_total counter
cache_hits_total {cache_hits}

# HELP cache_misses_total Total cache misses
# TYPE cache_misses_total counter
cache_misses_total {cache_misses}

# HELP cache_hit_rate_percent Cache hit rate percentage
# TYPE cache_hit_rate_percent gauge
cache_hit_rate_percent {cache_hit_rate}

# HELP database_queries_total Total database queries
# TYPE database_queries_total counter
database_queries_total {}

# HELP database_query_errors Total database query errors
# TYPE database_query_errors counter
database_query_errors {}

# HELP search_queries_total Total search queries
# TYPE search_queries_total counter
search_queries_total {}

# HELP storage_used_bytes Storage used in bytes
# TYPE storage_used_bytes gauge
storage_used_bytes {}

# HELP storage_quota_bytes Storage quota in bytes
# TYPE storage_quota_bytes gauge
storage_quota_bytes {}

# HELP files_count Total number of files
# TYPE files_count gauge
files_count {}

# HELP folders_count Total number of folders
# TYPE folders_count gauge
folders_count {}

# HELP jobs_queued Total jobs queued
# TYPE jobs_queued counter
jobs_queued {}

# HELP jobs_completed Total jobs completed
# TYPE jobs_completed counter
jobs_completed {}

# HELP jobs_failed Total jobs failed
# TYPE jobs_failed counter
jobs_failed {}
"#,
            self.http_requests_success.load(Ordering::Relaxed),
            self.http_requests_errors.load(Ordering::Relaxed),
            self.file_uploads_total.load(Ordering::Relaxed),
            self.file_downloads_total.load(Ordering::Relaxed),
            self.file_deletes_total.load(Ordering::Relaxed),
            self.bytes_uploaded_total.load(Ordering::Relaxed),
            self.bytes_downloaded_total.load(Ordering::Relaxed),
            self.active_users.load(Ordering::Relaxed),
            self.total_users.load(Ordering::Relaxed),
            self.logins_total.load(Ordering::Relaxed),
            self.login_failures_total.load(Ordering::Relaxed),
            self.websocket_connections.load(Ordering::Relaxed),
            self.websocket_messages_sent.load(Ordering::Relaxed),
            self.websocket_messages_received.load(Ordering::Relaxed),
            self.database_queries_total.load(Ordering::Relaxed),
            self.database_query_errors.load(Ordering::Relaxed),
            self.search_queries_total.load(Ordering::Relaxed),
            self.storage_used_bytes.load(Ordering::Relaxed),
            self.storage_quota_bytes.load(Ordering::Relaxed),
            self.files_count.load(Ordering::Relaxed),
            self.folders_count.load(Ordering::Relaxed),
            self.jobs_queued.load(Ordering::Relaxed),
            self.jobs_completed.load(Ordering::Relaxed),
            self.jobs_failed.load(Ordering::Relaxed),
        )
    }
    
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
        
        let http_total = self.http_requests_total.load(Ordering::Relaxed);
        let http_duration = self.http_request_duration_ms_sum.load(Ordering::Relaxed);
        let avg_request_duration_ms = if http_total > 0 { 
            http_duration as f64 / http_total as f64 
        } else { 0.0 };
        
        MetricsSnapshot {
            uptime_seconds: self.uptime_seconds(),
            
            http_requests_total: http_total,
            http_requests_success: self.http_requests_success.load(Ordering::Relaxed),
            http_requests_errors: self.http_requests_errors.load(Ordering::Relaxed),
            avg_request_duration_ms,
            
            file_uploads_total: self.file_uploads_total.load(Ordering::Relaxed),
            file_downloads_total: self.file_downloads_total.load(Ordering::Relaxed),
            file_deletes_total: self.file_deletes_total.load(Ordering::Relaxed),
            bytes_uploaded_total: self.bytes_uploaded_total.load(Ordering::Relaxed),
            bytes_downloaded_total: self.bytes_downloaded_total.load(Ordering::Relaxed),
            
            active_users: self.active_users.load(Ordering::Relaxed),
            total_users: self.total_users.load(Ordering::Relaxed),
            logins_total: self.logins_total.load(Ordering::Relaxed),
            login_failures_total: self.login_failures_total.load(Ordering::Relaxed),
            
            websocket_connections: self.websocket_connections.load(Ordering::Relaxed),
            websocket_messages_sent: self.websocket_messages_sent.load(Ordering::Relaxed),
            websocket_messages_received: self.websocket_messages_received.load(Ordering::Relaxed),
            
            cache_hits,
            cache_misses,
            cache_hit_rate,
            
            database_queries_total: self.database_queries_total.load(Ordering::Relaxed),
            database_query_errors: self.database_query_errors.load(Ordering::Relaxed),
            
            search_queries_total: self.search_queries_total.load(Ordering::Relaxed),
            search_results_total: self.search_results_total.load(Ordering::Relaxed),
            
            storage_used_bytes: self.storage_used_bytes.load(Ordering::Relaxed),
            storage_quota_bytes: self.storage_quota_bytes.load(Ordering::Relaxed),
            files_count: self.files_count.load(Ordering::Relaxed),
            folders_count: self.folders_count.load(Ordering::Relaxed),
            
            jobs_queued: self.jobs_queued.load(Ordering::Relaxed),
            jobs_completed: self.jobs_completed.load(Ordering::Relaxed),
            jobs_failed: self.jobs_failed.load(Ordering::Relaxed),
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of all metrics for JSON export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub uptime_seconds: u64,
    
    // HTTP
    pub http_requests_total: u64,
    pub http_requests_success: u64,
    pub http_requests_errors: u64,
    pub avg_request_duration_ms: f64,
    
    // Files
    pub file_uploads_total: u64,
    pub file_downloads_total: u64,
    pub file_deletes_total: u64,
    pub bytes_uploaded_total: u64,
    pub bytes_downloaded_total: u64,
    
    // Users
    pub active_users: u64,
    pub total_users: u64,
    pub logins_total: u64,
    pub login_failures_total: u64,
    
    // WebSocket
    pub websocket_connections: u64,
    pub websocket_messages_sent: u64,
    pub websocket_messages_received: u64,
    
    // Cache
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_hit_rate: f64,
    
    // Database
    pub database_queries_total: u64,
    pub database_query_errors: u64,
    
    // Search
    pub search_queries_total: u64,
    pub search_results_total: u64,
    
    // Storage
    pub storage_used_bytes: u64,
    pub storage_quota_bytes: u64,
    pub files_count: u64,
    pub folders_count: u64,
    
    // Jobs
    pub jobs_queued: u64,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
}

// ==================== API ENDPOINTS ====================

/// Build router for metrics endpoints
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/metrics", get(get_metrics_prometheus))
        .route("/metrics/json", get(get_metrics_json))
}

/// GET /metrics - Prometheus format metrics
async fn get_metrics_prometheus(State(state): State<AppState>) -> Response {
    // Update storage metrics from database
    if let Ok(counts) = get_storage_counts(&state.db_pool).await {
        state.performance_monitor.metrics().set_files_count(counts.files);
        state.performance_monitor.metrics().set_folders_count(counts.folders);
        state.performance_monitor.metrics().set_storage_used(counts.storage_bytes);
    }
    
    // Update user metrics
    if let Ok(user_count) = get_user_count(&state.db_pool).await {
        state.performance_monitor.metrics().set_total_users(user_count);
    }
    
    let metrics = state.performance_monitor.metrics().export_prometheus();
    
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        metrics,
    ).into_response()
}

/// GET /metrics/json - JSON format metrics
async fn get_metrics_json(State(state): State<AppState>) -> impl IntoResponse {
    // Update storage metrics from database
    if let Ok(counts) = get_storage_counts(&state.db_pool).await {
        state.performance_monitor.metrics().set_files_count(counts.files);
        state.performance_monitor.metrics().set_folders_count(counts.folders);
        state.performance_monitor.metrics().set_storage_used(counts.storage_bytes);
    }
    
    // Update user metrics
    if let Ok(user_count) = get_user_count(&state.db_pool).await {
        state.performance_monitor.metrics().set_total_users(user_count);
    }
    
    let snapshot = state.performance_monitor.metrics().snapshot();
    
    (StatusCode::OK, axum::Json(snapshot))
}

// Helper structs for database queries
struct StorageCounts {
    files: u64,
    folders: u64,
    storage_bytes: u64,
}

async fn get_storage_counts(pool: &sqlx::SqlitePool) -> Result<StorageCounts, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64, i64, i64)>(
        r#"
        SELECT 
            (SELECT COUNT(*) FROM files WHERE is_deleted = 0 AND is_directory = 0) as files,
            (SELECT COUNT(*) FROM files WHERE is_deleted = 0 AND is_directory = 1) as folders,
            (SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE is_deleted = 0) as storage
        "#
    )
    .fetch_one(pool)
    .await?;
    
    Ok(StorageCounts {
        files: row.0 as u64,
        folders: row.1 as u64,
        storage_bytes: row.2 as u64,
    })
}

async fn get_user_count(pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    Ok(count.0 as u64)
}
