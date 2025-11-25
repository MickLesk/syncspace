//! File sharing API endpoints

use crate::auth::UserInfo;

use crate::{services, AppState};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;

#[derive(Debug, Deserialize)]
pub struct CreateShareRequest {
    pub file_path: String,
    #[allow(dead_code)]
    pub permission: String,
    #[allow(dead_code)]
    pub expires_at: Option<String>,
    #[allow(dead_code)]
    pub password: Option<String>,
    #[allow(dead_code)]
    pub user_ids: Option<Vec<String>>, // NEW: List of user IDs to share with
    #[allow(dead_code)]
    pub permissions: Option<Vec<String>>, // NEW: Permissions for each user (parallel array)
    #[allow(dead_code)]
    pub allow_external: Option<bool>, // NEW: Toggle external/public sharing
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
        // NEW: Share user management
        .route(
            "/shares/{share_id}/users",
            get(get_share_users).post(add_share_users),
        )
        .route(
            "/shares/{share_id}/users/{user_id}",
            delete(remove_share_user).put(update_share_user_permission),
        )
}

/// Public sharing routes - NO AUTH REQUIRED
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/sharing/public/{share_token}", get(get_public_share))
        .route(
            "/sharing/public/{share_token}/download",
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
    // Query files shared with current user from sharing table
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
    if let Some(ref password_hash) = share.password_hash {
        let provided_password = params.password.as_ref().ok_or(StatusCode::FORBIDDEN)?;
        
        // Verify password using Argon2
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Argon2::default()
            .verify_password(provided_password.as_bytes(), &parsed_hash)
            .map_err(|_| StatusCode::FORBIDDEN)?;
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
    if let Some(ref password_hash) = share.password_hash {
        let provided_password = params.password.as_ref().ok_or(StatusCode::FORBIDDEN)?;
        
        // Verify password using Argon2
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Argon2::default()
            .verify_password(provided_password.as_bytes(), &parsed_hash)
            .map_err(|_| StatusCode::FORBIDDEN)?;
    }

    // Increment download counter
    sqlx::query("UPDATE shared_links SET download_count = download_count + 1 WHERE id = ?")
        .bind(&share_token)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log access
    let _ = services::sharing::log_access(&state, &share.id, None, "download", None).await;

    // Stream file from storage
    let file_path = std::path::Path::new("./data").join(&share.item_id);
    
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let file = tokio::fs::File::open(&file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    // Get filename for content-disposition
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download");

    // Detect mime type
    let mime_type = mime_guess::from_path(&file_path)
        .first_or_octet_stream()
        .to_string();

    Ok((
        [
            ("content-type", mime_type),
            (
                "content-disposition",
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        body,
    ))
}

// ============================================================================
// SHARE USER MANAGEMENT ENDPOINTS
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct AddShareUsersRequest {
    pub user_ids: Vec<String>,
    pub permissions: Vec<String>, // Parallel array: permissions[i] for user_ids[i]
}

#[derive(Debug, Deserialize)]
pub struct UpdateShareUserPermissionRequest {
    pub permission: String, // 'read', 'write', 'admin'
}

/// GET /shares/{share_id}/users - Get all users for a share
async fn get_share_users(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // Verify ownership or membership
    let _share: crate::database::SharedLink =
        sqlx::query_as("SELECT * FROM shared_links WHERE id = ? AND created_by = ?")
            .bind(&share_id)
            .bind(&user.id)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|_| StatusCode::FORBIDDEN)?;

    let users = services::sharing::get_share_users(&state, &share_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        users
            .into_iter()
            .map(|u| serde_json::to_value(u).unwrap_or_default())
            .collect(),
    ))
}

/// POST /shares/{share_id}/users - Add users to a share
async fn add_share_users(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
    Json(req): Json<AddShareUsersRequest>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    if req.user_ids.len() != req.permissions.len() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let share_users =
        services::sharing::add_share_users(&state, &user, &share_id, req.user_ids, req.permissions)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(
        share_users
            .into_iter()
            .map(|u| serde_json::to_value(u).unwrap_or_default())
            .collect(),
    ))
}

/// DELETE /shares/{share_id}/users/{user_id} - Remove user from share
async fn remove_share_user(
    State(state): State<AppState>,
    user: UserInfo,
    Path((share_id, user_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    services::sharing::remove_share_user(&state, &user, &share_id, &user_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

/// PUT /shares/{share_id}/users/{user_id} - Update user permission
async fn update_share_user_permission(
    State(state): State<AppState>,
    user: UserInfo,
    Path((share_id, user_id)): Path<(String, String)>,
    Json(req): Json<UpdateShareUserPermissionRequest>,
) -> Result<StatusCode, StatusCode> {
    services::sharing::update_share_user_permission(
        &state,
        &user,
        &share_id,
        &user_id,
        &req.permission,
    )
    .await
    .map(|_| StatusCode::OK)
    .map_err(|_| StatusCode::BAD_REQUEST)
}
