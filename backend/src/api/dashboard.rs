//! Admin Dashboard API
//! Comprehensive system analytics and monitoring endpoints

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::UserInfo;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/dashboard/stats", get(get_dashboard_stats))
        .route("/dashboard/storage", get(get_storage_overview))
        .route("/dashboard/activity", get(get_activity_overview))
        .route("/dashboard/users", get(get_users_overview))
        .route("/dashboard/jobs", get(get_jobs_overview))
        .route("/dashboard/health", get(get_system_health))
        .route("/dashboard/realtime", get(get_realtime_stats))
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct TimeRangeQuery {
    #[serde(default = "default_range")]
    pub range: String, // "24h", "7d", "30d", "90d"
}

fn default_range() -> String {
    "7d".to_string()
}

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub overview: OverviewStats,
    pub trends: TrendData,
    pub top_items: TopItems,
    pub system_info: SystemInfo,
}

#[derive(Debug, Serialize)]
pub struct OverviewStats {
    pub total_users: i64,
    pub active_users_today: i64,
    pub total_files: i64,
    pub total_storage_bytes: i64,
    pub total_shares: i64,
    pub pending_jobs: i64,
    pub active_sessions: i64,
    pub backup_count: i64,
}

#[derive(Debug, Serialize)]
pub struct TrendData {
    pub storage_trend: Vec<DataPoint>,
    pub upload_trend: Vec<DataPoint>,
    pub user_activity_trend: Vec<DataPoint>,
    pub share_trend: Vec<DataPoint>,
}

#[derive(Debug, Serialize)]
pub struct DataPoint {
    pub date: String,
    pub value: i64,
}

#[derive(Debug, Serialize)]
pub struct TopItems {
    pub top_uploaders: Vec<UserStat>,
    pub largest_files: Vec<FileStat>,
    pub most_shared: Vec<FileStat>,
    pub most_active_users: Vec<UserStat>,
}

#[derive(Debug, Serialize)]
pub struct UserStat {
    pub user_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub count: i64,
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct FileStat {
    pub file_path: String,
    pub filename: String,
    pub size_bytes: i64,
    pub owner: String,
    pub count: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub version: String,
    pub uptime_seconds: i64,
    pub database_size_bytes: i64,
    pub search_index_size_bytes: i64,
    pub cache_hit_rate: f64,
    pub last_backup: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StorageOverview {
    pub total_storage_bytes: i64,
    pub used_storage_bytes: i64,
    pub available_storage_bytes: i64,
    pub usage_percentage: f64,
    pub by_type: Vec<StorageByType>,
    pub by_user: Vec<StorageByUser>,
    pub trend: Vec<DataPoint>,
}

#[derive(Debug, Serialize)]
pub struct StorageByType {
    pub file_type: String,
    pub count: i64,
    pub size_bytes: i64,
    pub percentage: f64,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct StorageByUser {
    pub user_id: String,
    pub username: String,
    pub used_bytes: i64,
    pub quota_bytes: i64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct ActivityOverview {
    pub total_actions_today: i64,
    pub total_actions_week: i64,
    pub by_action: Vec<ActionCount>,
    pub by_hour: Vec<HourlyActivity>,
    pub recent_activity: Vec<RecentActivity>,
}

#[derive(Debug, Serialize)]
pub struct ActionCount {
    pub action: String,
    pub count: i64,
    pub icon: String,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct HourlyActivity {
    pub hour: i32,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct RecentActivity {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub action: String,
    pub resource: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct UsersOverview {
    pub total_users: i64,
    pub active_today: i64,
    pub active_this_week: i64,
    pub new_this_month: i64,
    pub by_role: Vec<RoleCount>,
    pub top_active: Vec<UserStat>,
    pub recent_signups: Vec<UserInfo2>,
}

#[derive(Debug, Serialize)]
pub struct RoleCount {
    pub role: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct UserInfo2 {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct JobsOverview {
    pub pending: i64,
    pub running: i64,
    pub completed_today: i64,
    pub failed_today: i64,
    pub by_type: Vec<JobTypeCount>,
    pub recent_jobs: Vec<JobInfo>,
    pub scheduled_jobs: Vec<ScheduledJob>,
}

#[derive(Debug, Serialize)]
pub struct JobTypeCount {
    pub job_type: String,
    pub count: i64,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct JobInfo {
    pub id: String,
    pub job_type: String,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ScheduledJob {
    pub id: String,
    pub name: String,
    pub cron_expression: String,
    pub next_run: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct SystemHealth {
    pub status: String, // "healthy", "degraded", "unhealthy"
    pub database: HealthCheck,
    pub storage: HealthCheck,
    pub search: HealthCheck,
    pub websocket: HealthCheck,
    pub memory_usage_mb: i64,
    pub cpu_usage_percent: f64,
    pub disk_usage_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub status: String,
    pub latency_ms: i64,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RealtimeStats {
    pub active_connections: i64,
    pub requests_per_minute: i64,
    pub active_uploads: i64,
    pub active_downloads: i64,
    pub websocket_connections: i64,
}

// ============================================================================
// HANDLERS
// ============================================================================

/// GET /dashboard/stats - Get comprehensive dashboard statistics
async fn get_dashboard_stats(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<TimeRangeQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let days = parse_range_to_days(&query.range);

    // Get overview stats
    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let today = Utc::now().format("%Y-%m-%d").to_string();
    let active_users_today: (i64,) =
        sqlx::query_as("SELECT COUNT(DISTINCT user_id) FROM activity WHERE DATE(created_at) = ?")
            .bind(&today)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let total_files: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM files")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let total_storage: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(size_bytes), 0) FROM files")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let total_shares: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM shared_links")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let pending_jobs: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status IN ('pending', 'running')",
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let active_sessions: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_sessions WHERE is_active = 1")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let backup_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM backups")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    // Get trends
    let storage_trend = get_trend(&state, "files", "size_bytes", days).await;
    let upload_trend = get_trend(&state, "activity", "upload", days).await;
    let user_activity_trend = get_trend(&state, "activity", "all", days).await;
    let share_trend = get_trend(&state, "shared_links", "count", days).await;

    // Get top items
    let top_uploaders = get_top_uploaders(&state, 5).await;
    let largest_files = get_largest_files(&state, 5).await;
    let most_shared = get_most_shared(&state, 5).await;
    let most_active_users = get_most_active_users(&state, 5).await;

    // Get system info
    let db_size = get_database_size(&state).await;

    Ok(Json(DashboardStats {
        overview: OverviewStats {
            total_users: total_users.0,
            active_users_today: active_users_today.0,
            total_files: total_files.0,
            total_storage_bytes: total_storage.0,
            total_shares: total_shares.0,
            pending_jobs: pending_jobs.0,
            active_sessions: active_sessions.0,
            backup_count: backup_count.0,
        },
        trends: TrendData {
            storage_trend,
            upload_trend,
            user_activity_trend,
            share_trend,
        },
        top_items: TopItems {
            top_uploaders,
            largest_files,
            most_shared,
            most_active_users,
        },
        system_info: SystemInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs().saturating_sub(state.start_time))
                .unwrap_or(0),
            database_size_bytes: db_size,
            search_index_size_bytes: 0,
            cache_hit_rate: 0.0,
            last_backup: None,
        },
    }))
}

/// GET /dashboard/storage - Get storage overview
async fn get_storage_overview(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let total_storage: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(size_bytes), 0) FROM files")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    // Get storage by file type
    let by_type_rows: Vec<(String, i64, i64)> = sqlx::query_as(
        "SELECT 
            CASE 
                WHEN mime_type LIKE 'image/%' THEN 'images'
                WHEN mime_type LIKE 'video/%' THEN 'videos'
                WHEN mime_type LIKE 'audio/%' THEN 'audio'
                WHEN mime_type LIKE 'application/pdf' THEN 'documents'
                WHEN mime_type LIKE 'application/msword%' OR mime_type LIKE 'application/vnd.openxmlformats%' THEN 'documents'
                WHEN mime_type LIKE 'application/zip%' OR mime_type LIKE 'application/x-rar%' OR mime_type LIKE 'application/x-7z%' THEN 'archives'
                ELSE 'other'
            END as file_type,
            COUNT(*) as count,
            COALESCE(SUM(size_bytes), 0) as size
         FROM files
         GROUP BY file_type
         ORDER BY size DESC"
    )
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();

    let total = total_storage.0.max(1) as f64;
    let by_type: Vec<StorageByType> = by_type_rows
        .into_iter()
        .map(|(file_type, count, size)| {
            let color = match file_type.as_str() {
                "images" => "#3B82F6",
                "videos" => "#8B5CF6",
                "audio" => "#EC4899",
                "documents" => "#10B981",
                "archives" => "#F59E0B",
                _ => "#6B7280",
            };
            StorageByType {
                file_type,
                count,
                size_bytes: size,
                percentage: (size as f64 / total) * 100.0,
                color: color.to_string(),
            }
        })
        .collect();

    // Get storage by user
    let by_user_rows: Vec<(String, String, i64, i64)> = sqlx::query_as(
        "SELECT u.id, u.username, COALESCE(SUM(f.size_bytes), 0) as used, u.storage_quota_bytes
         FROM users u
         LEFT JOIN files f ON u.id = f.owner_id
         GROUP BY u.id
         ORDER BY used DESC
         LIMIT 10",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    let by_user: Vec<StorageByUser> = by_user_rows
        .into_iter()
        .map(|(user_id, username, used, quota)| {
            let quota = if quota > 0 {
                quota
            } else {
                10 * 1024 * 1024 * 1024
            }; // 10GB default
            StorageByUser {
                user_id,
                username,
                used_bytes: used,
                quota_bytes: quota,
                percentage: (used as f64 / quota as f64) * 100.0,
            }
        })
        .collect();

    Ok(Json(StorageOverview {
        total_storage_bytes: 100 * 1024 * 1024 * 1024, // 100GB default capacity
        used_storage_bytes: total_storage.0,
        available_storage_bytes: 100 * 1024 * 1024 * 1024 - total_storage.0,
        usage_percentage: (total_storage.0 as f64 / (100.0 * 1024.0 * 1024.0 * 1024.0)) * 100.0,
        by_type,
        by_user,
        trend: vec![],
    }))
}

/// GET /dashboard/activity - Get activity overview
async fn get_activity_overview(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let week_ago = (Utc::now() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();

    let total_today: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM activity WHERE DATE(created_at) = ?")
            .bind(&today)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let total_week: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM activity WHERE DATE(created_at) >= ?")
            .bind(&week_ago)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    // Actions by type
    let by_action_rows: Vec<(String, i64)> = sqlx::query_as(
        "SELECT action, COUNT(*) as count FROM activity GROUP BY action ORDER BY count DESC LIMIT 10"
    )
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();

    let by_action: Vec<ActionCount> = by_action_rows
        .into_iter()
        .map(|(action, count)| {
            let (icon, color) = match action.as_str() {
                "upload" | "created" => ("cloud-upload", "#10B981"),
                "download" => ("cloud-download", "#3B82F6"),
                "delete" | "deleted" => ("trash", "#EF4444"),
                "share" | "shared" => ("share", "#8B5CF6"),
                "rename" | "renamed" => ("pencil", "#F59E0B"),
                "move" | "moved" => ("arrows-move", "#6366F1"),
                _ => ("activity", "#6B7280"),
            };
            ActionCount {
                action,
                count,
                icon: icon.to_string(),
                color: color.to_string(),
            }
        })
        .collect();

    // Hourly activity
    let by_hour_rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT CAST(strftime('%H', created_at) AS INTEGER) as hour, COUNT(*) as count 
         FROM activity 
         WHERE DATE(created_at) = ?
         GROUP BY hour 
         ORDER BY hour",
    )
    .bind(&today)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    let by_hour: Vec<HourlyActivity> = by_hour_rows
        .into_iter()
        .map(|(hour, count)| HourlyActivity { hour, count })
        .collect();

    // Recent activity
    let recent_rows: Vec<(String, String, String, String, String, String)> = sqlx::query_as(
        "SELECT a.id, a.user_id, COALESCE(u.username, 'Unknown') as username, a.action, 
                COALESCE(a.resource_id, '') as resource, a.created_at
         FROM activity a
         LEFT JOIN users u ON a.user_id = u.id
         ORDER BY a.created_at DESC
         LIMIT 20",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    let recent_activity: Vec<RecentActivity> = recent_rows
        .into_iter()
        .map(
            |(id, user_id, username, action, resource, created_at)| RecentActivity {
                id,
                user_id,
                username,
                action,
                resource,
                created_at,
            },
        )
        .collect();

    Ok(Json(ActivityOverview {
        total_actions_today: total_today.0,
        total_actions_week: total_week.0,
        by_action,
        by_hour,
        recent_activity,
    }))
}

/// GET /dashboard/users - Get users overview
async fn get_users_overview(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let week_ago = (Utc::now() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    let month_ago = (Utc::now() - Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();

    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let active_today: (i64,) =
        sqlx::query_as("SELECT COUNT(DISTINCT user_id) FROM activity WHERE DATE(created_at) = ?")
            .bind(&today)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let active_week: (i64,) =
        sqlx::query_as("SELECT COUNT(DISTINCT user_id) FROM activity WHERE DATE(created_at) >= ?")
            .bind(&week_ago)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let new_month: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE DATE(created_at) >= ?")
            .bind(&month_ago)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    // Users by role
    let by_role_rows: Vec<(String, i64)> =
        sqlx::query_as("SELECT COALESCE(role, 'user') as role, COUNT(*) FROM users GROUP BY role")
            .fetch_all(&state.db_pool)
            .await
            .unwrap_or_default();

    let by_role: Vec<RoleCount> = by_role_rows
        .into_iter()
        .map(|(role, count)| RoleCount { role, count })
        .collect();

    Ok(Json(UsersOverview {
        total_users: total_users.0,
        active_today: active_today.0,
        active_this_week: active_week.0,
        new_this_month: new_month.0,
        by_role,
        top_active: vec![],
        recent_signups: vec![],
    }))
}

/// GET /dashboard/jobs - Get jobs overview
async fn get_jobs_overview(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let today = Utc::now().format("%Y-%m-%d").to_string();

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
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'completed' AND DATE(updated_at) = ?",
    )
    .bind(&today)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let failed_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'failed' AND DATE(updated_at) = ?",
    )
    .bind(&today)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    // Recent jobs
    let recent_rows: Vec<(String, String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, job_type, status, created_at, updated_at 
         FROM background_jobs 
         ORDER BY created_at DESC 
         LIMIT 10",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    let recent_jobs: Vec<JobInfo> = recent_rows
        .into_iter()
        .map(|(id, job_type, status, created_at, completed_at)| JobInfo {
            id,
            job_type,
            status,
            created_at,
            completed_at,
        })
        .collect();

    // Scheduled jobs
    let scheduled_rows: Vec<(String, String, String, Option<String>, bool)> = sqlx::query_as(
        "SELECT id, name, cron_expression, next_run, is_active 
         FROM scheduled_jobs 
         ORDER BY next_run ASC 
         LIMIT 10",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    let scheduled_jobs: Vec<ScheduledJob> = scheduled_rows
        .into_iter()
        .map(
            |(id, name, cron_expression, next_run, is_active)| ScheduledJob {
                id,
                name,
                cron_expression,
                next_run,
                is_active,
            },
        )
        .collect();

    Ok(Json(JobsOverview {
        pending: pending.0,
        running: running.0,
        completed_today: completed_today.0,
        failed_today: failed_today.0,
        by_type: vec![],
        recent_jobs,
        scheduled_jobs,
    }))
}

/// GET /dashboard/health - Get system health status
async fn get_system_health(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Database health check
    let db_start = std::time::Instant::now();
    let db_ok = sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .is_ok();
    let db_latency = db_start.elapsed().as_millis() as i64;

    let database = HealthCheck {
        status: if db_ok {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        latency_ms: db_latency,
        message: if db_ok {
            None
        } else {
            Some("Database connection failed".to_string())
        },
    };

    // Storage health check (check if data directory is accessible)
    let storage_ok = std::path::Path::new("./data").exists();
    let storage = HealthCheck {
        status: if storage_ok {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        latency_ms: 0,
        message: if storage_ok {
            None
        } else {
            Some("Data directory not accessible".to_string())
        },
    };

    // Search health (Tantivy)
    let search = HealthCheck {
        status: "healthy".to_string(),
        latency_ms: 0,
        message: None,
    };

    // WebSocket health
    let websocket = HealthCheck {
        status: "healthy".to_string(),
        latency_ms: 0,
        message: None,
    };

    let overall_status = if db_ok && storage_ok {
        "healthy"
    } else if db_ok || storage_ok {
        "degraded"
    } else {
        "unhealthy"
    };

    Ok(Json(SystemHealth {
        status: overall_status.to_string(),
        database,
        storage,
        search,
        websocket,
        memory_usage_mb: 0,
        cpu_usage_percent: 0.0,
        disk_usage_percent: 0.0,
    }))
}

/// GET /dashboard/realtime - Get real-time statistics
async fn get_realtime_stats(
    State(_state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // These would typically come from in-memory counters
    Ok(Json(RealtimeStats {
        active_connections: 0,
        requests_per_minute: 0,
        active_uploads: 0,
        active_downloads: 0,
        websocket_connections: 0,
    }))
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn parse_range_to_days(range: &str) -> i64 {
    match range {
        "24h" => 1,
        "7d" => 7,
        "30d" => 30,
        "90d" => 90,
        _ => 7,
    }
}

async fn get_trend(state: &AppState, table: &str, metric: &str, days: i64) -> Vec<DataPoint> {
    let start_date = (Utc::now() - Duration::days(days))
        .format("%Y-%m-%d")
        .to_string();

    let query = match (table, metric) {
        ("files", "size_bytes") => {
            "SELECT DATE(created_at) as date, COALESCE(SUM(size_bytes), 0) as value 
             FROM files 
             WHERE DATE(created_at) >= ?
             GROUP BY date 
             ORDER BY date"
        }
        ("activity", "upload") => {
            "SELECT DATE(created_at) as date, COUNT(*) as value 
             FROM activity 
             WHERE action = 'upload' AND DATE(created_at) >= ?
             GROUP BY date 
             ORDER BY date"
        }
        ("activity", "all") => {
            "SELECT DATE(created_at) as date, COUNT(*) as value 
             FROM activity 
             WHERE DATE(created_at) >= ?
             GROUP BY date 
             ORDER BY date"
        }
        ("shared_links", "count") => {
            "SELECT DATE(created_at) as date, COUNT(*) as value 
             FROM shared_links 
             WHERE DATE(created_at) >= ?
             GROUP BY date 
             ORDER BY date"
        }
        _ => return vec![],
    };

    let rows: Vec<(String, i64)> = sqlx::query_as(query)
        .bind(&start_date)
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();

    rows.into_iter()
        .map(|(date, value)| DataPoint { date, value })
        .collect()
}

async fn get_top_uploaders(state: &AppState, limit: i32) -> Vec<UserStat> {
    let rows: Vec<(String, String, Option<String>, i64, i64)> = sqlx::query_as(
        "SELECT u.id, u.username, u.display_name, COUNT(*) as count, COALESCE(SUM(f.size_bytes), 0) as size
         FROM users u
         JOIN files f ON u.id = f.owner_id
         GROUP BY u.id
         ORDER BY count DESC
         LIMIT ?"
    )
        .bind(limit)
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();

    rows.into_iter()
        .map(|(user_id, username, display_name, count, size)| UserStat {
            user_id,
            username,
            display_name,
            count,
            size_bytes: Some(size),
        })
        .collect()
}

async fn get_largest_files(state: &AppState, limit: i32) -> Vec<FileStat> {
    let rows: Vec<(String, String, i64, String)> = sqlx::query_as(
        "SELECT f.file_path, f.filename, f.size_bytes, COALESCE(u.username, 'Unknown') as owner
         FROM files f
         LEFT JOIN users u ON f.owner_id = u.id
         ORDER BY f.size_bytes DESC
         LIMIT ?",
    )
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|(file_path, filename, size_bytes, owner)| FileStat {
            file_path,
            filename,
            size_bytes,
            owner,
            count: None,
        })
        .collect()
}

async fn get_most_shared(state: &AppState, limit: i32) -> Vec<FileStat> {
    let rows: Vec<(String, String, i64, String, i64)> = sqlx::query_as(
        "SELECT f.file_path, f.filename, f.size_bytes, COALESCE(u.username, 'Unknown') as owner, 
                COUNT(s.id) as share_count
         FROM files f
         LEFT JOIN users u ON f.owner_id = u.id
         LEFT JOIN shared_links s ON f.file_path = s.item_id
         GROUP BY f.id
         HAVING share_count > 0
         ORDER BY share_count DESC
         LIMIT ?",
    )
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|(file_path, filename, size_bytes, owner, count)| FileStat {
            file_path,
            filename,
            size_bytes,
            owner,
            count: Some(count),
        })
        .collect()
}

async fn get_most_active_users(state: &AppState, limit: i32) -> Vec<UserStat> {
    let rows: Vec<(String, String, Option<String>, i64)> = sqlx::query_as(
        "SELECT u.id, u.username, u.display_name, COUNT(*) as count
         FROM users u
         JOIN activity a ON u.id = a.user_id
         WHERE DATE(a.created_at) >= DATE('now', '-7 days')
         GROUP BY u.id
         ORDER BY count DESC
         LIMIT ?",
    )
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|(user_id, username, display_name, count)| UserStat {
            user_id,
            username,
            display_name,
            count,
            size_bytes: None,
        })
        .collect()
}

async fn get_database_size(state: &AppState) -> i64 {
    let result: Option<(i64,)> = sqlx::query_as(
        "SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()",
    )
    .fetch_optional(&state.db_pool)
    .await
    .unwrap_or(None);

    result.map(|(size,)| size).unwrap_or(0)
}
