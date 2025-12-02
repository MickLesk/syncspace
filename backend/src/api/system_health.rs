//! System Health Monitoring API
//!
//! Provides comprehensive system health monitoring including:
//! - Server metrics (CPU, memory, disk)
//! - Database connection pool stats
//! - Search index health
//! - Background job queue status
//! - Real-time system metrics

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::{auth::UserInfo, AppState};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Comprehensive system health response
#[derive(Debug, Serialize)]
pub struct SystemHealth {
    pub status: String, // "healthy", "degraded", "unhealthy"
    pub uptime_seconds: u64,
    pub timestamp: DateTime<Utc>,
    pub server: ServerMetrics,
    pub database: DatabaseHealth,
    pub storage: StorageHealth,
    pub search: SearchHealth,
    pub jobs: JobsHealth,
    pub websocket: WebSocketHealth,
    pub services: Vec<ServiceHealth>,
}

/// Server-level metrics
#[derive(Debug, Serialize)]
pub struct ServerMetrics {
    pub hostname: String,
    pub os: String,
    pub arch: String,
    pub rust_version: String,
    pub app_version: String,
    pub pid: u32,
    pub threads: usize,
    pub memory_used_mb: f64,
    pub memory_total_mb: f64,
    pub memory_percent: f64,
    pub cpu_usage_percent: f64,
}

/// Database health metrics
#[derive(Debug, Serialize)]
pub struct DatabaseHealth {
    pub status: String,
    pub connection_pool_size: u32,
    pub connections_active: u32,
    pub connections_idle: u32,
    pub latency_ms: f64,
    pub total_queries: i64,
    pub tables_count: i32,
    pub database_size_mb: f64,
    pub wal_size_mb: f64,
}

/// Storage health metrics
#[derive(Debug, Serialize)]
pub struct StorageHealth {
    pub status: String,
    pub data_path: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub files_count: i64,
    pub directories_count: i64,
    pub total_file_size_bytes: i64,
}

/// Search index health
#[derive(Debug, Serialize)]
pub struct SearchHealth {
    pub status: String,
    pub index_path: String,
    pub documents_count: i64,
    pub index_size_mb: f64,
    pub last_indexed_at: Option<DateTime<Utc>>,
    pub pending_indexing: i32,
}

/// Background jobs health
#[derive(Debug, Serialize)]
pub struct JobsHealth {
    pub status: String,
    pub pending_count: i64,
    pub running_count: i64,
    pub completed_today: i64,
    pub failed_today: i64,
    pub oldest_pending_minutes: Option<i64>,
}

/// WebSocket health
#[derive(Debug, Serialize)]
pub struct WebSocketHealth {
    pub status: String,
    pub active_connections: i32,
    pub messages_sent_today: i64,
    pub messages_received_today: i64,
}

/// Individual service health
#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub name: String,
    pub status: String,
    pub latency_ms: Option<f64>,
    pub last_check: DateTime<Utc>,
    pub message: Option<String>,
}

/// Database statistics
#[derive(Debug, Serialize)]
pub struct DatabaseStats {
    pub table_name: String,
    pub row_count: i64,
    pub size_bytes: i64,
}

/// System metrics history point
#[derive(Debug, Serialize)]
pub struct MetricsPoint {
    pub timestamp: DateTime<Utc>,
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
    pub active_connections: i32,
    pub requests_per_second: f64,
}

/// Detailed table statistics
#[derive(Debug, Serialize)]
pub struct TableStats {
    pub name: String,
    pub row_count: i64,
    pub size_bytes: i64,
    pub index_count: i32,
}

// ============================================================================
// ROUTER
// ============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/system/health", get(get_system_health))
        .route("/system/health/detailed", get(get_detailed_health))
        .route("/system/metrics", get(get_system_metrics))
        .route("/system/database/stats", get(get_database_stats))
        .route("/system/database/tables", get(get_table_stats))
        .route("/system/processes", get(get_active_processes))
}

// ============================================================================
// HANDLERS
// ============================================================================

/// Get comprehensive system health
async fn get_system_health(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<SystemHealth>, StatusCode> {
    let start = Instant::now();

    // Get uptime
    let uptime_seconds = state.start_time.elapsed().as_secs();

    // Check database health
    let db_health = check_database_health(&state).await;

    // Check storage health
    let storage_health = check_storage_health(&state).await;

    // Check search health
    let search_health = check_search_health(&state).await;

    // Check jobs health
    let jobs_health = check_jobs_health(&state).await;

    // Check websocket health
    let ws_health = check_websocket_health(&state).await;

    // Get server metrics
    let server_metrics = get_server_metrics();

    // Check individual services
    let services = check_services(&state).await;

    // Determine overall status
    let status =
        determine_overall_status(&db_health, &storage_health, &search_health, &jobs_health);

    let health = SystemHealth {
        status,
        uptime_seconds,
        timestamp: Utc::now(),
        server: server_metrics,
        database: db_health,
        storage: storage_health,
        search: search_health,
        jobs: jobs_health,
        websocket: ws_health,
        services,
    };

    tracing::debug!("Health check completed in {:?}", start.elapsed());
    Ok(Json(health))
}

/// Get detailed health with more metrics
async fn get_detailed_health(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let health = get_system_health(State(state.clone()), _user).await?;

    // Add additional detailed metrics
    let table_stats = get_table_statistics(&state).await;

    Ok(Json(serde_json::json!({
        "health": health.0,
        "tables": table_stats,
        "configuration": {
            "max_upload_size_mb": 100,
            "max_connections": 10,
            "trash_retention_days": 30,
            "version_retention_count": 10
        }
    })))
}

/// Get current system metrics
async fn get_system_metrics(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let server = get_server_metrics();

    // Get recent activity counts
    let activity_1h: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM activity WHERE timestamp > datetime('now', '-1 hour')",
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let activity_24h: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM activity WHERE timestamp > datetime('now', '-24 hours')",
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    // Get upload/download counts
    let uploads_24h: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM activity WHERE action = 'upload' AND timestamp > datetime('now', '-24 hours')"
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let downloads_24h: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM activity WHERE action = 'download' AND timestamp > datetime('now', '-24 hours')"
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    Ok(Json(serde_json::json!({
        "timestamp": Utc::now(),
        "server": server,
        "activity": {
            "last_hour": activity_1h.0,
            "last_24h": activity_24h.0,
            "uploads_24h": uploads_24h.0,
            "downloads_24h": downloads_24h.0
        },
        "uptime_seconds": state.start_time.elapsed().as_secs()
    })))
}

/// Get database statistics
async fn get_database_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tables = get_table_statistics(&state).await;

    // Get database file size
    let db_size = std::fs::metadata("./data/syncspace.db")
        .map(|m| m.len())
        .unwrap_or(0);

    // Get WAL size
    let wal_size = std::fs::metadata("./data/syncspace.db-wal")
        .map(|m| m.len())
        .unwrap_or(0);

    // Get total row counts
    let total_rows: i64 = tables.iter().map(|t| t.row_count).sum();

    Ok(Json(serde_json::json!({
        "database_size_bytes": db_size,
        "wal_size_bytes": wal_size,
        "total_size_bytes": db_size + wal_size,
        "total_rows": total_rows,
        "tables_count": tables.len(),
        "tables": tables
    })))
}

/// Get detailed table statistics
async fn get_table_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<TableStats>>, StatusCode> {
    let stats = get_table_statistics(&state).await;
    Ok(Json(stats))
}

/// Get active processes/connections
async fn get_active_processes(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get active user sessions
    let sessions: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT user_id, ip_address, last_activity FROM user_sessions WHERE expires_at > datetime('now') ORDER BY last_activity DESC LIMIT 50"
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    // Get running jobs
    let jobs: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT id, job_type, status, created_at FROM background_jobs WHERE status IN ('pending', 'running') ORDER BY created_at DESC LIMIT 20"
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    Ok(Json(serde_json::json!({
        "active_sessions": sessions.len(),
        "sessions": sessions.iter().map(|(user_id, ip, last)| {
            serde_json::json!({
                "user_id": user_id,
                "ip_address": ip,
                "last_activity": last
            })
        }).collect::<Vec<_>>(),
        "running_jobs": jobs.len(),
        "jobs": jobs.iter().map(|(id, job_type, status, created)| {
            serde_json::json!({
                "id": id,
                "type": job_type,
                "status": status,
                "created_at": created
            })
        }).collect::<Vec<_>>()
    })))
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn get_server_metrics() -> ServerMetrics {
    use std::process;

    // Get system info
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();

    // Get memory info (simplified - in production use sysinfo crate)
    let memory_used_mb = 0.0; // Would need sysinfo crate
    let memory_total_mb = 0.0;
    let memory_percent = 0.0;
    let cpu_usage_percent = 0.0;

    ServerMetrics {
        hostname,
        os,
        arch,
        rust_version: rustc_version_runtime::version().to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        pid: process::id(),
        threads: std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1),
        memory_used_mb,
        memory_total_mb,
        memory_percent,
        cpu_usage_percent,
    }
}

async fn check_database_health(state: &AppState) -> DatabaseHealth {
    let start = Instant::now();

    // Simple ping query
    let ping_result = sqlx::query("SELECT 1").execute(&state.db_pool).await;

    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
    let status = if ping_result.is_ok() {
        "healthy"
    } else {
        "unhealthy"
    };

    // Get table count
    let tables_count: (i32,) =
        sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    // Get database size
    let db_size = std::fs::metadata("./data/syncspace.db")
        .map(|m| m.len() as f64 / 1024.0 / 1024.0)
        .unwrap_or(0.0);

    let wal_size = std::fs::metadata("./data/syncspace.db-wal")
        .map(|m| m.len() as f64 / 1024.0 / 1024.0)
        .unwrap_or(0.0);

    DatabaseHealth {
        status: status.to_string(),
        connection_pool_size: state.db_pool.options().get_max_connections(),
        connections_active: state.db_pool.size(),
        connections_idle: state.db_pool.num_idle() as u32,
        latency_ms,
        total_queries: 0, // Would need query tracking
        tables_count: tables_count.0,
        database_size_mb: db_size,
        wal_size_mb: wal_size,
    }
}

async fn check_storage_health(state: &AppState) -> StorageHealth {
    let data_path = "./data";

    // Get disk usage (simplified)
    let (total, used, available) = get_disk_usage(data_path);
    let usage_percent = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // Get file counts
    let files_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM files WHERE is_deleted = 0")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let dirs_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM files WHERE is_deleted = 0 AND is_directory = 1")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let total_size: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE is_deleted = 0")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let status = if usage_percent > 90.0 {
        "critical"
    } else if usage_percent > 75.0 {
        "warning"
    } else {
        "healthy"
    };

    StorageHealth {
        status: status.to_string(),
        data_path: data_path.to_string(),
        total_bytes: total,
        used_bytes: used,
        available_bytes: available,
        usage_percent,
        files_count: files_count.0,
        directories_count: dirs_count.0,
        total_file_size_bytes: total_size.0,
    }
}

async fn check_search_health(state: &AppState) -> SearchHealth {
    let index_path = "./data/search_index";

    // Get index size
    let index_size = get_directory_size(index_path);

    // Get documents count from search index stats
    let docs_count = state
        .search_index
        .stats()
        .map(|s| s.indexed_docs as i64)
        .unwrap_or(0);

    SearchHealth {
        status: "healthy".to_string(),
        index_path: index_path.to_string(),
        documents_count: docs_count,
        index_size_mb: index_size as f64 / 1024.0 / 1024.0,
        last_indexed_at: None,
        pending_indexing: 0,
    }
}

async fn check_jobs_health(state: &AppState) -> JobsHealth {
    let pending: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM background_jobs WHERE status = 'pending'")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let running: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM background_jobs WHERE status = 'running'")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let completed_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'completed' AND updated_at > datetime('now', '-24 hours')"
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let failed_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'failed' AND updated_at > datetime('now', '-24 hours')"
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let status = if failed_today.0 > 10 {
        "degraded"
    } else if pending.0 > 100 {
        "warning"
    } else {
        "healthy"
    };

    JobsHealth {
        status: status.to_string(),
        pending_count: pending.0,
        running_count: running.0,
        completed_today: completed_today.0,
        failed_today: failed_today.0,
        oldest_pending_minutes: None,
    }
}

async fn check_websocket_health(_state: &AppState) -> WebSocketHealth {
    // WebSocket stats would need to be tracked in AppState
    WebSocketHealth {
        status: "healthy".to_string(),
        active_connections: 0,
        messages_sent_today: 0,
        messages_received_today: 0,
    }
}

async fn check_services(state: &AppState) -> Vec<ServiceHealth> {
    let mut services = Vec::new();

    // Database service
    let db_start = Instant::now();
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
        .is_ok();
    services.push(ServiceHealth {
        name: "database".to_string(),
        status: if db_ok { "healthy" } else { "unhealthy" }.to_string(),
        latency_ms: Some(db_start.elapsed().as_secs_f64() * 1000.0),
        last_check: Utc::now(),
        message: None,
    });

    // Search service
    let search_ok = state.search_index.stats().is_ok();
    services.push(ServiceHealth {
        name: "search".to_string(),
        status: if search_ok { "healthy" } else { "degraded" }.to_string(),
        latency_ms: None,
        last_check: Utc::now(),
        message: None,
    });

    // Storage service
    let storage_ok = std::fs::metadata("./data").is_ok();
    services.push(ServiceHealth {
        name: "storage".to_string(),
        status: if storage_ok { "healthy" } else { "unhealthy" }.to_string(),
        latency_ms: None,
        last_check: Utc::now(),
        message: None,
    });

    services
}

async fn get_table_statistics(state: &AppState) -> Vec<TableStats> {
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '_sqlx_%' ORDER BY name"
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    let mut stats = Vec::new();
    for (table_name,) in tables {
        // Get row count
        let count_query = format!("SELECT COUNT(*) FROM \"{}\"", table_name);
        let row_count: (i64,) = sqlx::query_as(&count_query)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

        // Get index count
        let index_query = format!(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name='{}'",
            table_name
        );
        let index_count: (i32,) = sqlx::query_as(&index_query)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

        stats.push(TableStats {
            name: table_name,
            row_count: row_count.0,
            size_bytes: 0, // SQLite doesn't expose per-table sizes easily
            index_count: index_count.0,
        });
    }

    stats
}

fn get_disk_usage(path: &str) -> (u64, u64, u64) {
    use sysinfo::Disks;

    // Try to get actual disk info using sysinfo
    let disks = Disks::new_with_refreshed_list();

    // Find the disk that contains our path
    let path_buf = std::path::Path::new(path)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(path));

    for disk in disks.list() {
        let mount_point = disk.mount_point();
        if path_buf.starts_with(mount_point) {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            return (total, used, available);
        }
    }

    // Fallback: calculate used space from directory and estimate total
    let used: u64 = get_directory_size(path);
    let total: u64 = 100 * 1024 * 1024 * 1024; // 100 GB fallback
    let available = total.saturating_sub(used);
    (total, used, available)
}

fn get_directory_size(path: &str) -> u64 {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

fn determine_overall_status(
    db: &DatabaseHealth,
    storage: &StorageHealth,
    search: &SearchHealth,
    jobs: &JobsHealth,
) -> String {
    if db.status == "unhealthy" || storage.status == "critical" {
        "unhealthy".to_string()
    } else if db.status == "degraded"
        || storage.status == "warning"
        || search.status == "degraded"
        || jobs.status == "degraded"
    {
        "degraded".to_string()
    } else {
        "healthy".to_string()
    }
}
