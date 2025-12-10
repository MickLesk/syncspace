/// API endpoints for metrics and monitoring
/// 
/// Provides:
/// - Real-time metrics snapshot
/// - Historical metrics (time-series)
/// - Alerts and thresholds
/// - Export functionality

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{auth::UserInfo, AppState};

// ==================== TYPES ====================

#[derive(Debug, Serialize)]
pub struct MetricsOverview {
    pub uptime: UptimeInfo,
    pub http: HttpMetrics,
    pub files: FileMetrics,
    pub users: UserMetrics,
    pub websocket: WebSocketMetrics,
    pub cache: CacheMetrics,
    pub database: DatabaseMetrics,
    pub search: SearchMetrics,
    pub storage: StorageMetrics,
    pub jobs: JobMetrics,
}

#[derive(Debug, Serialize)]
pub struct UptimeInfo {
    pub seconds: u64,
    pub formatted: String,
}

#[derive(Debug, Serialize)]
pub struct HttpMetrics {
    pub total_requests: u64,
    pub successful: u64,
    pub errors: u64,
    pub error_rate: f64,
    pub avg_response_time_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct FileMetrics {
    pub uploads: u64,
    pub downloads: u64,
    pub deletes: u64,
    pub bytes_uploaded: u64,
    pub bytes_downloaded: u64,
    pub bytes_uploaded_formatted: String,
    pub bytes_downloaded_formatted: String,
}

#[derive(Debug, Serialize)]
pub struct UserMetrics {
    pub total: u64,
    pub active: u64,
    pub logins: u64,
    pub login_failures: u64,
    pub login_failure_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct WebSocketMetrics {
    pub active_connections: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
}

#[derive(Debug, Serialize)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub total_operations: u64,
}

#[derive(Debug, Serialize)]
pub struct DatabaseMetrics {
    pub total_queries: u64,
    pub errors: u64,
    pub error_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct SearchMetrics {
    pub total_queries: u64,
    pub total_results: u64,
    pub avg_results_per_query: f64,
}

#[derive(Debug, Serialize)]
pub struct StorageMetrics {
    pub used_bytes: u64,
    pub quota_bytes: u64,
    pub used_formatted: String,
    pub quota_formatted: String,
    pub usage_percent: f64,
    pub files_count: u64,
    pub folders_count: u64,
}

#[derive(Debug, Serialize)]
pub struct JobMetrics {
    pub queued: u64,
    pub completed: u64,
    pub failed: u64,
    pub success_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct TimeRangeQuery {
    pub range: Option<String>, // 1h, 6h, 24h, 7d, 30d
}

#[derive(Debug, Serialize)]
pub struct MetricsTimePoint {
    pub timestamp: String,
    pub http_requests: u64,
    pub file_operations: u64,
    pub active_users: u64,
    pub storage_bytes: u64,
}

#[derive(Debug, Serialize)]
pub struct MetricsHistory {
    pub range: String,
    pub points: Vec<MetricsTimePoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricAlert {
    pub id: String,
    pub name: String,
    pub metric: String,
    pub condition: String, // gt, lt, eq
    pub threshold: f64,
    pub enabled: bool,
    pub triggered: bool,
    pub last_triggered_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlertRequest {
    pub name: String,
    pub metric: String,
    pub condition: String,
    pub threshold: f64,
}

#[derive(Debug, Serialize)]
pub struct MetricsExport {
    pub format: String,
    pub timestamp: String,
    pub data: serde_json::Value,
}

// ==================== ROUTER ====================

pub fn router() -> Router<AppState> {
    Router::new()
        // Overview
        .route("/", get(get_metrics_overview))
        .route("/snapshot", get(get_metrics_snapshot))
        
        // History
        .route("/history", get(get_metrics_history))
        
        // Alerts
        .route("/alerts", get(list_alerts))
        .route("/alerts", post(create_alert))
        .route("/alerts/{id}", delete(delete_alert))
        .route("/alerts/{id}/toggle", post(toggle_alert))
        
        // Export
        .route("/export/json", get(export_json))
        .route("/export/prometheus", get(export_prometheus))
        .route("/export/csv", get(export_csv))
        
        // Real-time
        .route("/live", get(get_live_metrics))
}

// ==================== HANDLERS ====================

/// GET /api/metrics - Get formatted metrics overview
async fn get_metrics_overview(
    State(state): State<AppState>,
    _user: UserInfo,
) -> impl IntoResponse {
    let snapshot = state.performance_monitor.metrics().snapshot();
    
    // Calculate rates
    let http_error_rate = if snapshot.http_requests_total > 0 {
        snapshot.http_requests_errors as f64 / snapshot.http_requests_total as f64 * 100.0
    } else {
        0.0
    };
    
    let login_failure_rate = if snapshot.logins_total > 0 {
        snapshot.login_failures_total as f64 / (snapshot.logins_total + snapshot.login_failures_total) as f64 * 100.0
    } else {
        0.0
    };
    
    let db_error_rate = if snapshot.database_queries_total > 0 {
        snapshot.database_query_errors as f64 / snapshot.database_queries_total as f64 * 100.0
    } else {
        0.0
    };
    
    let avg_results = if snapshot.search_queries_total > 0 {
        snapshot.search_results_total as f64 / snapshot.search_queries_total as f64
    } else {
        0.0
    };
    
    let storage_usage = if snapshot.storage_quota_bytes > 0 {
        snapshot.storage_used_bytes as f64 / snapshot.storage_quota_bytes as f64 * 100.0
    } else {
        0.0
    };
    
    let job_success_rate = {
        let total = snapshot.jobs_completed + snapshot.jobs_failed;
        if total > 0 {
            snapshot.jobs_completed as f64 / total as f64 * 100.0
        } else {
            100.0
        }
    };
    
    let overview = MetricsOverview {
        uptime: UptimeInfo {
            seconds: snapshot.uptime_seconds,
            formatted: format_duration(snapshot.uptime_seconds),
        },
        http: HttpMetrics {
            total_requests: snapshot.http_requests_total,
            successful: snapshot.http_requests_success,
            errors: snapshot.http_requests_errors,
            error_rate: http_error_rate,
            avg_response_time_ms: snapshot.avg_request_duration_ms,
        },
        files: FileMetrics {
            uploads: snapshot.file_uploads_total,
            downloads: snapshot.file_downloads_total,
            deletes: snapshot.file_deletes_total,
            bytes_uploaded: snapshot.bytes_uploaded_total,
            bytes_downloaded: snapshot.bytes_downloaded_total,
            bytes_uploaded_formatted: format_bytes(snapshot.bytes_uploaded_total),
            bytes_downloaded_formatted: format_bytes(snapshot.bytes_downloaded_total),
        },
        users: UserMetrics {
            total: snapshot.total_users,
            active: snapshot.active_users,
            logins: snapshot.logins_total,
            login_failures: snapshot.login_failures_total,
            login_failure_rate,
        },
        websocket: WebSocketMetrics {
            active_connections: snapshot.websocket_connections,
            messages_sent: snapshot.websocket_messages_sent,
            messages_received: snapshot.websocket_messages_received,
        },
        cache: CacheMetrics {
            hits: snapshot.cache_hits,
            misses: snapshot.cache_misses,
            hit_rate: snapshot.cache_hit_rate * 100.0,
            total_operations: snapshot.cache_hits + snapshot.cache_misses,
        },
        database: DatabaseMetrics {
            total_queries: snapshot.database_queries_total,
            errors: snapshot.database_query_errors,
            error_rate: db_error_rate,
        },
        search: SearchMetrics {
            total_queries: snapshot.search_queries_total,
            total_results: snapshot.search_results_total,
            avg_results_per_query: avg_results,
        },
        storage: StorageMetrics {
            used_bytes: snapshot.storage_used_bytes,
            quota_bytes: snapshot.storage_quota_bytes,
            used_formatted: format_bytes(snapshot.storage_used_bytes),
            quota_formatted: format_bytes(snapshot.storage_quota_bytes),
            usage_percent: storage_usage,
            files_count: snapshot.files_count,
            folders_count: snapshot.folders_count,
        },
        jobs: JobMetrics {
            queued: snapshot.jobs_queued,
            completed: snapshot.jobs_completed,
            failed: snapshot.jobs_failed,
            success_rate: job_success_rate,
        },
    };
    
    (StatusCode::OK, Json(overview))
}

/// GET /api/metrics/snapshot - Get raw metrics snapshot
async fn get_metrics_snapshot(
    State(state): State<AppState>,
    _user: UserInfo,
) -> impl IntoResponse {
    let snapshot = state.performance_monitor.metrics().snapshot();
    (StatusCode::OK, Json(snapshot))
}

/// GET /api/metrics/history - Get historical metrics
async fn get_metrics_history(
    State(_state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<TimeRangeQuery>,
) -> impl IntoResponse {
    let range = query.range.unwrap_or_else(|| "1h".to_string());
    
    // Generate simulated history points (in production would come from database)
    let points = generate_history_points(&range);
    
    let history = MetricsHistory {
        range: range.clone(),
        points,
    };
    
    (StatusCode::OK, Json(history))
}

/// GET /api/metrics/alerts - List all alerts
async fn list_alerts(
    State(state): State<AppState>,
    _user: UserInfo,
) -> impl IntoResponse {
    let alerts: Vec<MetricAlert> = sqlx::query_as::<_, (String, String, String, String, f64, bool, bool, Option<String>, String)>(
        r#"
        SELECT id, name, metric, condition, threshold, enabled, triggered, last_triggered_at, created_at
        FROM metric_alerts
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|row| MetricAlert {
        id: row.0,
        name: row.1,
        metric: row.2,
        condition: row.3,
        threshold: row.4,
        enabled: row.5,
        triggered: row.6,
        last_triggered_at: row.7,
        created_at: row.8,
    })
    .collect();
    
    (StatusCode::OK, Json(alerts))
}

/// POST /api/metrics/alerts - Create new alert
async fn create_alert(
    State(state): State<AppState>,
    _user: UserInfo,
    Json(req): Json<CreateAlertRequest>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    
    let result = sqlx::query(
        r#"
        INSERT INTO metric_alerts (id, name, metric, condition, threshold, enabled, triggered, created_at)
        VALUES (?, ?, ?, ?, ?, 1, 0, datetime('now'))
        "#
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.metric)
    .bind(&req.condition)
    .bind(req.threshold)
    .execute(&state.db_pool)
    .await;
    
    match result {
        Ok(_) => {
            let alert = MetricAlert {
                id,
                name: req.name,
                metric: req.metric,
                condition: req.condition,
                threshold: req.threshold,
                enabled: true,
                triggered: false,
                last_triggered_at: None,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            (StatusCode::CREATED, Json(serde_json::json!({ "alert": alert })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ 
                "error": format!("Failed to create alert: {}", e) 
            })))
        }
    }
}

/// DELETE /api/metrics/alerts/{id} - Delete alert
async fn delete_alert(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM metric_alerts WHERE id = ?")
        .bind(&id)
        .execute(&state.db_pool)
        .await;
    
    match result {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// POST /api/metrics/alerts/{id}/toggle - Toggle alert enabled state
async fn toggle_alert(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "UPDATE metric_alerts SET enabled = NOT enabled WHERE id = ?"
    )
    .bind(&id)
    .execute(&state.db_pool)
    .await;
    
    match result {
        Ok(r) if r.rows_affected() > 0 => {
            (StatusCode::OK, Json(serde_json::json!({ "success": true })))
        }
        Ok(_) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Alert not found" })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ 
                "error": format!("Failed to toggle alert: {}", e) 
            })))
        }
    }
}

/// GET /api/metrics/export/json - Export metrics as JSON
async fn export_json(
    State(state): State<AppState>,
    _user: UserInfo,
) -> impl IntoResponse {
    let snapshot = state.performance_monitor.metrics().snapshot();
    
    let export = MetricsExport {
        format: "json".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        data: serde_json::to_value(&snapshot).unwrap_or_default(),
    };
    
    (StatusCode::OK, Json(export))
}

/// GET /api/metrics/export/prometheus - Export metrics in Prometheus format
async fn export_prometheus(
    State(state): State<AppState>,
    _user: UserInfo,
) -> impl IntoResponse {
    let prometheus_output = state.performance_monitor.metrics().export_prometheus();
    
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        prometheus_output,
    )
}

/// GET /api/metrics/export/csv - Export metrics as CSV
async fn export_csv(
    State(state): State<AppState>,
    _user: UserInfo,
) -> impl IntoResponse {
    let snapshot = state.performance_monitor.metrics().snapshot();
    
    let csv = format!(
        "metric,value\n\
        uptime_seconds,{}\n\
        http_requests_total,{}\n\
        http_requests_success,{}\n\
        http_requests_errors,{}\n\
        file_uploads_total,{}\n\
        file_downloads_total,{}\n\
        file_deletes_total,{}\n\
        bytes_uploaded_total,{}\n\
        bytes_downloaded_total,{}\n\
        active_users,{}\n\
        total_users,{}\n\
        logins_total,{}\n\
        login_failures_total,{}\n\
        websocket_connections,{}\n\
        cache_hits,{}\n\
        cache_misses,{}\n\
        database_queries_total,{}\n\
        storage_used_bytes,{}\n\
        files_count,{}\n\
        folders_count,{}\n\
        jobs_completed,{}\n\
        jobs_failed,{}\n",
        snapshot.uptime_seconds,
        snapshot.http_requests_total,
        snapshot.http_requests_success,
        snapshot.http_requests_errors,
        snapshot.file_uploads_total,
        snapshot.file_downloads_total,
        snapshot.file_deletes_total,
        snapshot.bytes_uploaded_total,
        snapshot.bytes_downloaded_total,
        snapshot.active_users,
        snapshot.total_users,
        snapshot.logins_total,
        snapshot.login_failures_total,
        snapshot.websocket_connections,
        snapshot.cache_hits,
        snapshot.cache_misses,
        snapshot.database_queries_total,
        snapshot.storage_used_bytes,
        snapshot.files_count,
        snapshot.folders_count,
        snapshot.jobs_completed,
        snapshot.jobs_failed,
    );
    
    (
        StatusCode::OK,
        [
            ("content-type", "text/csv; charset=utf-8"),
            ("content-disposition", "attachment; filename=\"metrics.csv\""),
        ],
        csv,
    )
}

/// GET /api/metrics/live - Get lightweight live metrics for real-time updates
async fn get_live_metrics(
    State(state): State<AppState>,
    _user: UserInfo,
) -> impl IntoResponse {
    let snapshot = state.performance_monitor.metrics().snapshot();
    
    let live = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime_seconds": snapshot.uptime_seconds,
        "http_requests": snapshot.http_requests_total,
        "http_errors": snapshot.http_requests_errors,
        "active_users": snapshot.active_users,
        "websocket_connections": snapshot.websocket_connections,
        "storage_used": snapshot.storage_used_bytes,
        "cache_hit_rate": snapshot.cache_hit_rate,
    });
    
    (StatusCode::OK, Json(live))
}

// ==================== HELPERS ====================

fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, mins, secs)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    
    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn generate_history_points(range: &str) -> Vec<MetricsTimePoint> {
    use chrono::{Duration, Utc};
    
    let (duration, interval_mins) = match range {
        "1h" => (Duration::hours(1), 5),
        "6h" => (Duration::hours(6), 30),
        "24h" => (Duration::hours(24), 60),
        "7d" => (Duration::days(7), 360),
        "30d" => (Duration::days(30), 1440),
        _ => (Duration::hours(1), 5),
    };
    
    let now = Utc::now();
    let start = now - duration;
    let mut points = Vec::new();
    let mut current = start;
    
    // Generate sample data points
    let mut http = 1000u64;
    let mut files = 50u64;
    let mut users = 5u64;
    let mut storage = 1024 * 1024 * 100u64; // 100MB
    
    while current <= now {
        // Simulate some variation
        http += (current.timestamp() % 100) as u64;
        files += (current.timestamp() % 10) as u64;
        users = 5 + (current.timestamp() % 20) as u64;
        storage += (current.timestamp() % 10000) as u64;
        
        points.push(MetricsTimePoint {
            timestamp: current.to_rfc3339(),
            http_requests: http,
            file_operations: files,
            active_users: users,
            storage_bytes: storage,
        });
        
        current = current + Duration::minutes(interval_mins);
    }
    
    points
}
