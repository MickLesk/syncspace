//! Rate Limiting & Quotas API
//! User storage quotas, bandwidth limits, and API rate limiting management

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::{auth::UserInfo, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        // Storage Quotas
        .route("/quotas/storage", get(list_storage_quotas))
        .route("/quotas/storage/:user_id", get(get_user_storage_quota))
        .route("/quotas/storage/:user_id", put(update_storage_quota))
        .route("/quotas/storage/:user_id/usage", get(get_storage_usage))
        
        // Bandwidth Quotas
        .route("/quotas/bandwidth", get(list_bandwidth_quotas))
        .route("/quotas/bandwidth/:user_id", get(get_user_bandwidth_quota))
        .route("/quotas/bandwidth/:user_id", put(update_bandwidth_quota))
        .route("/quotas/bandwidth/:user_id/usage", get(get_bandwidth_usage))
        
        // Rate Limits
        .route("/rate-limits", get(list_rate_limits))
        .route("/rate-limits", post(create_rate_limit))
        .route("/rate-limits/:id", get(get_rate_limit))
        .route("/rate-limits/:id", put(update_rate_limit))
        .route("/rate-limits/:id", delete(delete_rate_limit))
        
        // Alerts
        .route("/quotas/alerts", get(list_quota_alerts))
        .route("/quotas/alerts/:id/acknowledge", post(acknowledge_alert))
        
        // Analytics
        .route("/quotas/stats", get(get_quota_stats))
        .route("/rate-limits/stats", get(get_rate_limit_stats))
}

// ==================== Models ====================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StorageQuota {
    pub id: String,
    pub user_id: String,
    pub storage_limit_bytes: i64,
    pub storage_used_bytes: i64,
    pub warning_threshold_percent: i32,
    pub is_unlimited: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BandwidthQuota {
    pub id: String,
    pub user_id: String,
    pub daily_upload_limit_bytes: Option<i64>,
    pub daily_download_limit_bytes: Option<i64>,
    pub monthly_upload_limit_bytes: Option<i64>,
    pub monthly_download_limit_bytes: Option<i64>,
    pub is_unlimited: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BandwidthUsage {
    pub id: String,
    pub user_id: String,
    pub date: String,
    pub upload_bytes: i64,
    pub download_bytes: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ApiRateLimit {
    pub id: String,
    pub user_id: Option<String>,
    pub role_name: Option<String>,
    pub endpoint_pattern: String,
    pub requests_per_minute: i32,
    pub requests_per_hour: i32,
    pub requests_per_day: i32,
    pub burst_limit: i32,
    pub is_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuotaAlert {
    pub id: String,
    pub user_id: String,
    pub alert_type: String,
    pub threshold_percent: Option<i32>,
    pub message: String,
    pub is_acknowledged: bool,
    pub created_at: String,
}

// ==================== Request/Response Types ====================

#[derive(Debug, Deserialize)]
pub struct UpdateStorageQuotaRequest {
    pub storage_limit_bytes: Option<i64>,
    pub warning_threshold_percent: Option<i32>,
    pub is_unlimited: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBandwidthQuotaRequest {
    pub daily_upload_limit_bytes: Option<i64>,
    pub daily_download_limit_bytes: Option<i64>,
    pub monthly_upload_limit_bytes: Option<i64>,
    pub monthly_download_limit_bytes: Option<i64>,
    pub is_unlimited: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRateLimitRequest {
    pub user_id: Option<String>,
    pub role_name: Option<String>,
    pub endpoint_pattern: String,
    pub requests_per_minute: i32,
    pub requests_per_hour: i32,
    pub requests_per_day: i32,
    pub burst_limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRateLimitRequest {
    pub endpoint_pattern: Option<String>,
    pub requests_per_minute: Option<i32>,
    pub requests_per_hour: Option<i32>,
    pub requests_per_day: Option<i32>,
    pub burst_limit: Option<i32>,
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuotasQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BandwidthUsageQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StorageUsageResponse {
    pub user_id: String,
    pub storage_limit_bytes: i64,
    pub storage_used_bytes: i64,
    pub storage_available_bytes: i64,
    pub usage_percent: f64,
    pub is_unlimited: bool,
    pub file_count: i64,
}

#[derive(Debug, Serialize)]
pub struct BandwidthUsageResponse {
    pub user_id: String,
    pub daily_upload_used: i64,
    pub daily_download_used: i64,
    pub daily_upload_limit: Option<i64>,
    pub daily_download_limit: Option<i64>,
    pub monthly_upload_used: i64,
    pub monthly_download_used: i64,
    pub monthly_upload_limit: Option<i64>,
    pub monthly_download_limit: Option<i64>,
    pub is_unlimited: bool,
}

#[derive(Debug, Serialize)]
pub struct QuotaStatsResponse {
    pub total_users: i64,
    pub users_with_quotas: i64,
    pub total_storage_allocated: i64,
    pub total_storage_used: i64,
    pub users_near_limit: i64,
    pub users_over_limit: i64,
}

#[derive(Debug, Serialize)]
pub struct RateLimitStatsResponse {
    pub total_rules: i64,
    pub active_rules: i64,
    pub requests_today: i64,
    pub requests_rate_limited_today: i64,
    pub top_endpoints: Vec<EndpointStats>,
}

#[derive(Debug, Serialize)]
pub struct EndpointStats {
    pub endpoint: String,
    pub request_count: i64,
    pub rate_limited_count: i64,
}

// ==================== Storage Quota Handlers ====================

async fn list_storage_quotas(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Query(params): Query<ListQuotasQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = (params.page.unwrap_or(1) - 1) * limit;

    let quotas = sqlx::query_as::<_, StorageQuota>(
        r#"
        SELECT id, user_id, storage_limit_bytes, storage_used_bytes, 
               warning_threshold_percent, is_unlimited, created_at, updated_at
        FROM user_storage_quotas
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list storage quotas: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_storage_quotas")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    Ok(Json(serde_json::json!({
        "quotas": quotas,
        "total": total.0,
        "page": params.page.unwrap_or(1),
        "limit": limit
    })))
}

async fn get_user_storage_quota(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<StorageQuota>, StatusCode> {
    // Create default quota if not exists
    let _ = sqlx::query(
        r#"
        INSERT OR IGNORE INTO user_storage_quotas (id, user_id)
        VALUES (lower(hex(randomblob(16))), ?)
        "#
    )
    .bind(&user_id)
    .execute(&state.db_pool)
    .await;

    let quota = sqlx::query_as::<_, StorageQuota>(
        r#"
        SELECT id, user_id, storage_limit_bytes, storage_used_bytes,
               warning_threshold_percent, is_unlimited, created_at, updated_at
        FROM user_storage_quotas
        WHERE user_id = ?
        "#
    )
    .bind(&user_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(quota))
}

async fn update_storage_quota(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<UpdateStorageQuotaRequest>,
) -> Result<Json<StorageQuota>, StatusCode> {
    // Create default quota if not exists
    let _ = sqlx::query(
        r#"
        INSERT OR IGNORE INTO user_storage_quotas (id, user_id)
        VALUES (lower(hex(randomblob(16))), ?)
        "#
    )
    .bind(&user_id)
    .execute(&state.db_pool)
    .await;

    // Update fields
    if let Some(limit) = req.storage_limit_bytes {
        sqlx::query("UPDATE user_storage_quotas SET storage_limit_bytes = ?, updated_at = datetime('now') WHERE user_id = ?")
            .bind(limit)
            .bind(&user_id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update storage limit: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    if let Some(threshold) = req.warning_threshold_percent {
        sqlx::query("UPDATE user_storage_quotas SET warning_threshold_percent = ?, updated_at = datetime('now') WHERE user_id = ?")
            .bind(threshold)
            .bind(&user_id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update warning threshold: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    if let Some(unlimited) = req.is_unlimited {
        sqlx::query("UPDATE user_storage_quotas SET is_unlimited = ?, updated_at = datetime('now') WHERE user_id = ?")
            .bind(unlimited)
            .bind(&user_id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update unlimited status: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    // Return updated quota
    get_user_storage_quota(_user_info, State(state), Path(user_id)).await
}

async fn get_storage_usage(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<StorageUsageResponse>, StatusCode> {
    // Get quota
    let quota = sqlx::query_as::<_, (i64, i64, bool)>(
        r#"
        SELECT storage_limit_bytes, storage_used_bytes, is_unlimited
        FROM user_storage_quotas
        WHERE user_id = ?
        "#
    )
    .bind(&user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get storage quota: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .unwrap_or((10737418240, 0, false)); // Default 10GB

    // Get file count
    let file_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM files WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(&user_id)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    // Calculate actual storage used from files
    let actual_used: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(&user_id)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let storage_limit = quota.0;
    let storage_used = actual_used.0;
    let storage_available = if quota.2 { i64::MAX } else { storage_limit - storage_used };
    let usage_percent = if quota.2 || storage_limit == 0 { 
        0.0 
    } else { 
        (storage_used as f64 / storage_limit as f64) * 100.0 
    };

    Ok(Json(StorageUsageResponse {
        user_id,
        storage_limit_bytes: storage_limit,
        storage_used_bytes: storage_used,
        storage_available_bytes: storage_available.max(0),
        usage_percent,
        is_unlimited: quota.2,
        file_count: file_count.0,
    }))
}

// ==================== Bandwidth Quota Handlers ====================

async fn list_bandwidth_quotas(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Query(params): Query<ListQuotasQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = (params.page.unwrap_or(1) - 1) * limit;

    let quotas = sqlx::query_as::<_, BandwidthQuota>(
        r#"
        SELECT id, user_id, daily_upload_limit_bytes, daily_download_limit_bytes,
               monthly_upload_limit_bytes, monthly_download_limit_bytes,
               is_unlimited, created_at, updated_at
        FROM user_bandwidth_quotas
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list bandwidth quotas: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_bandwidth_quotas")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    Ok(Json(serde_json::json!({
        "quotas": quotas,
        "total": total.0,
        "page": params.page.unwrap_or(1),
        "limit": limit
    })))
}

async fn get_user_bandwidth_quota(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<BandwidthQuota>, StatusCode> {
    // Create default quota if not exists
    let _ = sqlx::query(
        r#"
        INSERT OR IGNORE INTO user_bandwidth_quotas (id, user_id)
        VALUES (lower(hex(randomblob(16))), ?)
        "#
    )
    .bind(&user_id)
    .execute(&state.db_pool)
    .await;

    let quota = sqlx::query_as::<_, BandwidthQuota>(
        r#"
        SELECT id, user_id, daily_upload_limit_bytes, daily_download_limit_bytes,
               monthly_upload_limit_bytes, monthly_download_limit_bytes,
               is_unlimited, created_at, updated_at
        FROM user_bandwidth_quotas
        WHERE user_id = ?
        "#
    )
    .bind(&user_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(quota))
}

async fn update_bandwidth_quota(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<UpdateBandwidthQuotaRequest>,
) -> Result<Json<BandwidthQuota>, StatusCode> {
    // Create default quota if not exists
    let _ = sqlx::query(
        r#"
        INSERT OR IGNORE INTO user_bandwidth_quotas (id, user_id)
        VALUES (lower(hex(randomblob(16))), ?)
        "#
    )
    .bind(&user_id)
    .execute(&state.db_pool)
    .await;

    // Update fields
    sqlx::query(
        r#"
        UPDATE user_bandwidth_quotas 
        SET daily_upload_limit_bytes = COALESCE(?, daily_upload_limit_bytes),
            daily_download_limit_bytes = COALESCE(?, daily_download_limit_bytes),
            monthly_upload_limit_bytes = COALESCE(?, monthly_upload_limit_bytes),
            monthly_download_limit_bytes = COALESCE(?, monthly_download_limit_bytes),
            is_unlimited = COALESCE(?, is_unlimited),
            updated_at = datetime('now')
        WHERE user_id = ?
        "#
    )
    .bind(req.daily_upload_limit_bytes)
    .bind(req.daily_download_limit_bytes)
    .bind(req.monthly_upload_limit_bytes)
    .bind(req.monthly_download_limit_bytes)
    .bind(req.is_unlimited)
    .bind(&user_id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update bandwidth quota: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_user_bandwidth_quota(_user_info, State(state), Path(user_id)).await
}

async fn get_bandwidth_usage(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Query(params): Query<BandwidthUsageQuery>,
) -> Result<Json<BandwidthUsageResponse>, StatusCode> {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let month_start = chrono::Utc::now().format("%Y-%m-01").to_string();

    // Get daily usage
    let daily: (i64, i64) = sqlx::query_as(
        "SELECT COALESCE(upload_bytes, 0), COALESCE(download_bytes, 0) FROM bandwidth_usage WHERE user_id = ? AND date = ?"
    )
    .bind(&user_id)
    .bind(&today)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0, 0));

    // Get monthly usage
    let monthly: (i64, i64) = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM(upload_bytes), 0), COALESCE(SUM(download_bytes), 0) 
        FROM bandwidth_usage 
        WHERE user_id = ? AND date >= ?
        "#
    )
    .bind(&user_id)
    .bind(&month_start)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0, 0));

    // Get quota limits
    let quota = sqlx::query_as::<_, (Option<i64>, Option<i64>, Option<i64>, Option<i64>, bool)>(
        r#"
        SELECT daily_upload_limit_bytes, daily_download_limit_bytes,
               monthly_upload_limit_bytes, monthly_download_limit_bytes, is_unlimited
        FROM user_bandwidth_quotas
        WHERE user_id = ?
        "#
    )
    .bind(&user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get bandwidth quota: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .unwrap_or((None, None, None, None, true));

    Ok(Json(BandwidthUsageResponse {
        user_id,
        daily_upload_used: daily.0,
        daily_download_used: daily.1,
        daily_upload_limit: quota.0,
        daily_download_limit: quota.1,
        monthly_upload_used: monthly.0,
        monthly_download_used: monthly.1,
        monthly_upload_limit: quota.2,
        monthly_download_limit: quota.3,
        is_unlimited: quota.4,
    }))
}

// ==================== Rate Limit Handlers ====================

async fn list_rate_limits(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Query(params): Query<ListQuotasQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = (params.page.unwrap_or(1) - 1) * limit;

    let limits = sqlx::query_as::<_, ApiRateLimit>(
        r#"
        SELECT id, user_id, role_name, endpoint_pattern, requests_per_minute,
               requests_per_hour, requests_per_day, burst_limit, is_enabled,
               created_at, updated_at
        FROM api_rate_limits
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list rate limits: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM api_rate_limits")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    Ok(Json(serde_json::json!({
        "rateLimits": limits,
        "total": total.0,
        "page": params.page.unwrap_or(1),
        "limit": limit
    })))
}

async fn create_rate_limit(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Json(req): Json<CreateRateLimitRequest>,
) -> Result<Json<ApiRateLimit>, StatusCode> {
    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO api_rate_limits (id, user_id, role_name, endpoint_pattern, 
                                     requests_per_minute, requests_per_hour, 
                                     requests_per_day, burst_limit)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&req.user_id)
    .bind(&req.role_name)
    .bind(&req.endpoint_pattern)
    .bind(req.requests_per_minute)
    .bind(req.requests_per_hour)
    .bind(req.requests_per_day)
    .bind(req.burst_limit.unwrap_or(10))
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create rate limit: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_rate_limit(_user_info, State(state), Path(id)).await
}

async fn get_rate_limit(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiRateLimit>, StatusCode> {
    let limit = sqlx::query_as::<_, ApiRateLimit>(
        r#"
        SELECT id, user_id, role_name, endpoint_pattern, requests_per_minute,
               requests_per_hour, requests_per_day, burst_limit, is_enabled,
               created_at, updated_at
        FROM api_rate_limits
        WHERE id = ?
        "#
    )
    .bind(&id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(limit))
}

async fn update_rate_limit(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRateLimitRequest>,
) -> Result<Json<ApiRateLimit>, StatusCode> {
    sqlx::query(
        r#"
        UPDATE api_rate_limits 
        SET endpoint_pattern = COALESCE(?, endpoint_pattern),
            requests_per_minute = COALESCE(?, requests_per_minute),
            requests_per_hour = COALESCE(?, requests_per_hour),
            requests_per_day = COALESCE(?, requests_per_day),
            burst_limit = COALESCE(?, burst_limit),
            is_enabled = COALESCE(?, is_enabled),
            updated_at = datetime('now')
        WHERE id = ?
        "#
    )
    .bind(&req.endpoint_pattern)
    .bind(req.requests_per_minute)
    .bind(req.requests_per_hour)
    .bind(req.requests_per_day)
    .bind(req.burst_limit)
    .bind(req.is_enabled)
    .bind(&id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update rate limit: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    get_rate_limit(_user_info, State(state), Path(id)).await
}

async fn delete_rate_limit(
    _user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM api_rate_limits WHERE id = ?")
        .bind(&id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete rate limit: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

// ==================== Alert Handlers ====================

async fn list_quota_alerts(
    user_info: UserInfo,
    State(state): State<AppState>,
) -> Result<Json<Vec<QuotaAlert>>, StatusCode> {
    let alerts = sqlx::query_as::<_, QuotaAlert>(
        r#"
        SELECT id, user_id, alert_type, threshold_percent, message, is_acknowledged, created_at
        FROM quota_alerts
        WHERE user_id = ? AND is_acknowledged = 0
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .bind(&user_info.id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list quota alerts: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(alerts))
}

async fn acknowledge_alert(
    user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("UPDATE quota_alerts SET is_acknowledged = 1 WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to acknowledge alert: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

// ==================== Stats Handlers ====================

async fn get_quota_stats(
    _user_info: UserInfo,
    State(state): State<AppState>,
) -> Result<Json<QuotaStatsResponse>, StatusCode> {
    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let users_with_quotas: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_storage_quotas")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let storage_stats: (i64, i64) = sqlx::query_as(
        "SELECT COALESCE(SUM(storage_limit_bytes), 0), COALESCE(SUM(storage_used_bytes), 0) FROM user_storage_quotas WHERE is_unlimited = 0"
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0, 0));

    let users_near_limit: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM user_storage_quotas 
        WHERE is_unlimited = 0 
        AND (storage_used_bytes * 100.0 / storage_limit_bytes) >= warning_threshold_percent
        AND storage_used_bytes < storage_limit_bytes
        "#
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let users_over_limit: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_storage_quotas WHERE is_unlimited = 0 AND storage_used_bytes >= storage_limit_bytes"
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    Ok(Json(QuotaStatsResponse {
        total_users: total_users.0,
        users_with_quotas: users_with_quotas.0,
        total_storage_allocated: storage_stats.0,
        total_storage_used: storage_stats.1,
        users_near_limit: users_near_limit.0,
        users_over_limit: users_over_limit.0,
    }))
}

async fn get_rate_limit_stats(
    _user_info: UserInfo,
    State(state): State<AppState>,
) -> Result<Json<RateLimitStatsResponse>, StatusCode> {
    let total_rules: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM api_rate_limits")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let active_rules: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM api_rate_limits WHERE is_enabled = 1")
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    let requests_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM api_request_log WHERE date(created_at) = ?"
    )
    .bind(&today)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let rate_limited_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM api_request_log WHERE date(created_at) = ? AND is_rate_limited = 1"
    )
    .bind(&today)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let top_endpoints = sqlx::query_as::<_, (String, i64, i64)>(
        r#"
        SELECT endpoint, COUNT(*) as request_count, SUM(is_rate_limited) as rate_limited_count
        FROM api_request_log
        WHERE date(created_at) = ?
        GROUP BY endpoint
        ORDER BY request_count DESC
        LIMIT 10
        "#
    )
    .bind(&today)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|(endpoint, request_count, rate_limited_count)| EndpointStats {
        endpoint,
        request_count,
        rate_limited_count,
    })
    .collect();

    Ok(Json(RateLimitStatsResponse {
        total_rules: total_rules.0,
        active_rules: active_rules.0,
        requests_today: requests_today.0,
        requests_rate_limited_today: rate_limited_today.0,
        top_endpoints,
    }))
}
