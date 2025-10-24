//! Storage management and quota enforcement
//!
//! Handles storage statistics, quota checks, and cleanup.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

use crate::auth::User;
use crate::AppState;

// ==================== MODELS ====================

#[derive(Debug, Clone, Serialize)]
pub struct StorageStats {
    pub total_files: i64,
    pub total_folders: i64,
    pub total_size_bytes: i64,
    pub used_quota_bytes: i64,
    pub quota_limit_bytes: i64,
    pub quota_percentage: f64,
    pub trash_size_bytes: i64,
    pub versions_size_bytes: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserStorageUsage {
    pub user_id: String,
    pub username: String,
    pub storage_used_bytes: i64,
    pub storage_quota_bytes: i64,
    pub quota_percentage: f64,
    pub file_count: i64,
    pub folder_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuotaRequest {
    pub quota_bytes: i64,
}

// ==================== HELPERS ====================

/// Check if user has enough quota for upload
pub async fn check_quota(
    pool: &sqlx::SqlitePool,
    user_id: &str,
    additional_bytes: i64,
) -> Result<bool, sqlx::Error> {
    let (used, quota): (i64, i64) = sqlx::query_as(
        "SELECT storage_used_bytes, storage_quota_bytes FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(used + additional_bytes <= quota)
}

/// Update user's storage usage
pub async fn update_storage_usage(
    pool: &sqlx::SqlitePool,
    user_id: &str,
    delta_bytes: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users 
        SET storage_used_bytes = storage_used_bytes + ?,
            updated_at = ?
        WHERE id = ?
        "#
    )
    .bind(delta_bytes)
    .bind(Utc::now().to_rfc3339())
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Recalculate storage usage for user (use after cleanup)
pub async fn recalculate_storage_usage(
    pool: &sqlx::SqlitePool,
    user_id: &str,
) -> Result<i64, sqlx::Error> {
    let total: Option<(i64,)> = sqlx::query_as(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let total_bytes = total.map(|t| t.0).unwrap_or(0);

    sqlx::query("UPDATE users SET storage_used_bytes = ? WHERE id = ?")
        .bind(total_bytes)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(total_bytes)
}

// ==================== HANDLERS ====================

/// Get detailed storage statistics for current user
/// GET /api/storage/stats
pub async fn get_storage_stats(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<StorageStats>, StatusCode> {
    // Total files (not deleted)
    let total_files: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM files WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(&user.id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Total folders (not deleted)
    let total_folders: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM folders WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(&user.id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Total size (not deleted)
    let total_size: Option<(i64,)> = sqlx::query_as(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(&user.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Trash size
    let trash_size: Option<(i64,)> = sqlx::query_as(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE owner_id = ? AND is_deleted = 1"
    )
    .bind(&user.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Versions size
    let versions_size: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM(size_bytes), 0) FROM file_versions 
        WHERE file_id IN (SELECT id FROM files WHERE owner_id = ?)
        "#
    )
    .bind(&user.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // User quota
    let (used_quota, quota_limit): (i64, i64) = sqlx::query_as(
        "SELECT storage_used_bytes, storage_quota_bytes FROM users WHERE id = ?"
    )
    .bind(&user.id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let quota_percentage = if quota_limit > 0 {
        (used_quota as f64 / quota_limit as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(StorageStats {
        total_files: total_files.0,
        total_folders: total_folders.0,
        total_size_bytes: total_size.map(|s| s.0).unwrap_or(0),
        used_quota_bytes: used_quota,
        quota_limit_bytes: quota_limit,
        quota_percentage,
        trash_size_bytes: trash_size.map(|s| s.0).unwrap_or(0),
        versions_size_bytes: versions_size.map(|s| s.0).unwrap_or(0),
    }))
}

/// Get storage usage for specific user (admin only)
/// GET /api/storage/usage/{user_id}
pub async fn get_user_storage_usage(
    _user: User, // TODO: Check admin role
    State(state): State<AppState>,
    Path(target_user_id): Path<String>,
) -> Result<Json<UserStorageUsage>, StatusCode> {
    // Get user info
    let (username, used, quota): (String, i64, i64) = sqlx::query_as(
        "SELECT username, storage_used_bytes, storage_quota_bytes FROM users WHERE id = ?"
    )
    .bind(&target_user_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    // File count
    let file_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM files WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(&target_user_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Folder count
    let folder_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM folders WHERE owner_id = ? AND is_deleted = 0"
    )
    .bind(&target_user_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let quota_percentage = if quota > 0 {
        (used as f64 / quota as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(UserStorageUsage {
        user_id: target_user_id,
        username,
        storage_used_bytes: used,
        storage_quota_bytes: quota,
        quota_percentage,
        file_count: file_count.0,
        folder_count: folder_count.0,
    }))
}

/// Update user quota (admin only)
/// PUT /api/storage/quota/{user_id}
pub async fn update_user_quota(
    _user: User, // TODO: Check admin role
    State(state): State<AppState>,
    Path(target_user_id): Path<String>,
    Json(req): Json<UpdateQuotaRequest>,
) -> Result<StatusCode, StatusCode> {
    if req.quota_bytes < 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query("UPDATE users SET storage_quota_bytes = ? WHERE id = ?")
        .bind(req.quota_bytes)
        .bind(&target_user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Cleanup old file versions and recalculate storage
/// POST /api/storage/cleanup
pub async fn cleanup_storage(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<StorageStats>, StatusCode> {
    // Delete old trash items (30+ days)
    let thirty_days_ago = (Utc::now() - chrono::Duration::days(30)).to_rfc3339();
    
    sqlx::query(
        "DELETE FROM files WHERE owner_id = ? AND is_deleted = 1 AND deleted_at < ?"
    )
    .bind(&user.id)
    .bind(&thirty_days_ago)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Delete old file versions (keep only last 5 per file)
    sqlx::query(
        r#"
        DELETE FROM file_versions
        WHERE id NOT IN (
            SELECT id FROM file_versions v
            WHERE v.file_id IN (SELECT id FROM files WHERE owner_id = ?)
            ORDER BY v.created_at DESC
            LIMIT 5
        )
        "#
    )
    .bind(&user.id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Recalculate storage usage
    let new_usage = recalculate_storage_usage(&state.db_pool, &user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO activity_log (id, user_id, action, resource_type, resource_id, timestamp, metadata)
        VALUES (?, ?, 'storage_cleanup', 'storage', ?, ?, ?)
        "#
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.id)
    .bind(&user.id)
    .bind(Utc::now().to_rfc3339())
    .bind(format!("{{\"new_usage\": {}}}", new_usage))
    .execute(&state.db_pool)
    .await;

    // Return updated stats
    get_storage_stats(user, State(state)).await
}

/// Recalculate storage usage for current user
/// POST /api/storage/recalculate
pub async fn recalculate_storage(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<StorageStats>, StatusCode> {
    recalculate_storage_usage(&state.db_pool, &user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    get_storage_stats(user, State(state)).await
}
