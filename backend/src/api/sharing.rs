//! File sharing API endpoints

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateShareRequest {
    pub file_path: String,
    pub permission: String,
    pub expires_at: Option<String>,
    pub password: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/shares", get(list_shares).post(create_share))
        .route(
            "/shares/{share_id}",
            get(get_share).put(update_share).delete(delete_share),
        )
        .route(
            "/shares/{share_id}/permissions",
            put(update_share_permissions),
        )
        .route(
            "/shares/{share_id}/regenerate-token",
            post(regenerate_share_token),
        )
        .route("/shares/{share_id}/analytics", get(get_share_analytics))
        .route("/shares/{share_id}/access-log", get(get_access_log))
        .route("/shared-with-me", get(list_shared_with_me))
}

/// Public sharing routes - NO AUTH REQUIRED
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/sharing/public/:share_token", get(get_public_share))
        .route(
            "/sharing/public/:share_token/download",
            get(download_public_share),
        )
}

async fn list_shares(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let shares = services::sharing::list_shares(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        shares
            .into_iter()
            .map(|s| serde_json::to_value(s).unwrap_or_default())
            .collect(),
    ))
}

async fn create_share(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateShareRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let share = services::sharing::create_share(&state, &user, &req.file_path, false)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    serde_json::to_value(share)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_share(
    State(state): State<AppState>,
    Path(share_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let share = services::sharing::get_share(&state, &share_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    serde_json::to_value(share)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_share(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    services::sharing::update_share(&state, &user, &share_id, req)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn delete_share(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    services::sharing::delete_share(&state, &user, &share_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_share_permissions(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    // Reuse update_share for now
    services::sharing::update_share(&state, &user, &share_id, req)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn list_shared_with_me(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // TODO: Implement actual query for files shared with current user
    let shares = services::sharing::list_shares(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        shares
            .into_iter()
            .map(|s| serde_json::to_value(s).unwrap_or_default())
            .collect(),
    ))
}

// ============================================================================
// ADDITIONAL SHARING FEATURES
// ============================================================================

/// POST /shares/{share_id}/regenerate-token - Generate new share token
async fn regenerate_share_token(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let new_token = services::sharing::regenerate_token(&state, &user, &share_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "share_id": share_id,
        "share_token": new_token,
        "message": "Share token regenerated successfully"
    })))
}

/// GET /shares/{share_id}/analytics - Get share analytics
async fn get_share_analytics(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let analytics = services::sharing::get_analytics(&state, &user, &share_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(analytics))
}

/// GET /shares/{share_id}/access-log - Get access log for share
async fn get_access_log(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let logs = services::sharing::get_access_log(&state, &user, &share_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(logs))
}

// ============================================================================
// PUBLIC SHARING ENDPOINTS (NO AUTH)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct PublicShareQuery {
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicShareResponse {
    pub file_path: String,
    pub permission: String,
    pub expires_at: Option<String>,
    pub is_expired: bool,
    pub requires_password: bool,
}

/// GET /sharing/public/{share_token} - Get public share info (NO AUTH)
async fn get_public_share(
    State(state): State<AppState>,
    Path(share_token): Path<String>,
    Query(params): Query<PublicShareQuery>,
) -> Result<Json<PublicShareResponse>, StatusCode> {
    // Get share by token (the share token IS the share ID)
    let share: crate::database::SharedLink =
        sqlx::query_as("SELECT * FROM shared_links WHERE id = ? AND is_public = 1")
            .bind(&share_token)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

    // Check expiry
    if let Some(ref expires_at) = share.expires_at {
        if expires_at < &Utc::now().to_rfc3339() {
            return Err(StatusCode::GONE); // 410 Gone - Share expired
        }
    }

    // Check max downloads exceeded
    if let Some(max_dl) = share.max_downloads {
        if share.download_count >= max_dl {
            return Err(StatusCode::GONE); // 410 Gone - Download limit reached
        }
    }

    // Check password if required
    if let Some(ref _password_hash) = share.password_hash {
        if params.password.is_none() {
            return Err(StatusCode::FORBIDDEN); // 403 - Password required
        }
        // TODO: Verify password against hash
    }

    // Log access
    let _ = services::sharing::log_access(&state, &share.id, None, "view", None).await;

    Ok(Json(PublicShareResponse {
        file_path: share.item_id,
        permission: if share.allow_upload { "write" } else { "read" }.to_string(),
        expires_at: share.expires_at,
        is_expired: false,
        requires_password: share.password_hash.is_some(),
    }))
}

/// GET /sharing/public/{share_token}/download - Download via public share (NO AUTH)
async fn download_public_share(
    State(state): State<AppState>,
    Path(share_token): Path<String>,
    Query(params): Query<PublicShareQuery>,
) -> Result<impl axum::response::IntoResponse, StatusCode> {
    // Get share by token
    let share: crate::database::SharedLink = sqlx::query_as(
        "SELECT * FROM shared_links WHERE id = ? AND is_public = 1 AND allow_download = 1",
    )
    .bind(&share_token)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    // Check expiry
    if let Some(ref expires_at) = share.expires_at {
        if expires_at < &Utc::now().to_rfc3339() {
            return Err(StatusCode::GONE);
        }
    }

    // Check download limit
    if let Some(max_dl) = share.max_downloads {
        if share.download_count >= max_dl {
            return Err(StatusCode::GONE);
        }
    }

    // Check password
    if let Some(ref _password_hash) = share.password_hash {
        if params.password.is_none() {
            return Err(StatusCode::FORBIDDEN);
        }
        // TODO: Verify password
    }

    // Increment download counter
    sqlx::query("UPDATE shared_links SET download_count = download_count + 1 WHERE id = ?")
        .bind(&share_token)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log access
    let _ = services::sharing::log_access(&state, &share.id, None, "download", None).await;

    // TODO: Stream file from storage_path or file path
    // For now, return 501 Not Implemented
    Ok((
        StatusCode::NOT_IMPLEMENTED,
        "File streaming not yet implemented",
    ))
}
