use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, put},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::auth::UserInfo;

#[derive(Debug, Serialize)]
pub struct QuotaInfo {
    pub user_id: String,
    pub username: String,
    pub storage_used_bytes: i64,
    pub storage_quota_bytes: i64,
    pub usage_percentage: f64,
    pub available_bytes: i64,
}

#[derive(Debug, Deserialize)]
pub struct SetQuotaRequest {
    pub quota_bytes: i64,
}

/// Get current user's quota info
async fn get_my_quota(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let quota_info: Option<(i64, i64)> = sqlx::query_as(
        "SELECT storage_used_bytes, storage_quota_bytes FROM users WHERE id = ?"
    )
    .bind(&user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (used, quota) = quota_info.ok_or(StatusCode::NOT_FOUND)?;

    let usage_percentage = if quota > 0 {
        (used as f64 / quota as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(QuotaInfo {
        user_id: user.user_id().to_string(),
        username: user.username.clone(),
        storage_used_bytes: used,
        storage_quota_bytes: quota,
        usage_percentage,
        available_bytes: quota - used,
    }))
}

/// Get quota info for specific user (admin only)
async fn get_user_quota(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    _admin: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let user_info: Option<(String, i64, i64)> = sqlx::query_as(
        "SELECT username, storage_used_bytes, storage_quota_bytes FROM users WHERE id = ?"
    )
    .bind(&user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (username, used, quota) = user_info.ok_or(StatusCode::NOT_FOUND)?;

    let usage_percentage = if quota > 0 {
        (used as f64 / quota as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(QuotaInfo {
        user_id,
        username,
        storage_used_bytes: used,
        storage_quota_bytes: quota,
        usage_percentage,
        available_bytes: quota - used,
    }))
}

/// Set quota for user (admin only)
async fn set_user_quota(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    _admin: UserInfo,
    Json(req): Json<SetQuotaRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if req.quota_bytes < 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE users SET storage_quota_bytes = ?, updated_at = ? WHERE id = ?"
    )
    .bind(req.quota_bytes)
    .bind(&now)
    .bind(&user_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Quota updated successfully",
        "quota_bytes": req.quota_bytes
    }))))
}

/// Update storage usage (internal helper, called when files are uploaded/deleted)
pub async fn update_storage_usage(
    pool: &sqlx::SqlitePool,
    user_id: &str,
) -> Result<(), StatusCode> {
    // Calculate total storage used by summing file sizes
    let total_used: Option<(i64,)> = sqlx::query_as(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = total_used.map(|(t,)| t).unwrap_or(0);

    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE users SET storage_used_bytes = ?, updated_at = ? WHERE id = ?"
    )
    .bind(total)
    .bind(&now)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

/// Check if user has enough quota for upload
pub async fn check_quota_available(
    pool: &sqlx::SqlitePool,
    user_id: &str,
    required_bytes: i64,
) -> Result<bool, StatusCode> {
    let quota_info: Option<(i64, i64)> = sqlx::query_as(
        "SELECT storage_used_bytes, storage_quota_bytes FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (used, quota) = quota_info.ok_or(StatusCode::NOT_FOUND)?;

    Ok(used + required_bytes <= quota)
}

/// Get all users' quota info (admin only)
async fn list_all_quotas(
    State(state): State<AppState>,
    _admin: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let users: Vec<(String, String, i64, i64)> = sqlx::query_as(
        "SELECT id, username, storage_used_bytes, storage_quota_bytes 
         FROM users 
         ORDER BY storage_used_bytes DESC"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let quota_infos: Vec<QuotaInfo> = users
        .into_iter()
        .map(|(id, username, used, quota)| {
            let usage_percentage = if quota > 0 {
                (used as f64 / quota as f64) * 100.0
            } else {
                0.0
            };

            QuotaInfo {
                user_id: id,
                username,
                storage_used_bytes: used,
                storage_quota_bytes: quota,
                usage_percentage,
                available_bytes: quota - used,
            }
        })
        .collect();

    Ok(Json(quota_infos))
}

/// Router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/quota", get(get_my_quota))
        .route("/api/quota/all", get(list_all_quotas))
        .route("/api/quota/{user_id}", get(get_user_quota).put(set_user_quota))
}
