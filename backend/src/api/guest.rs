//! Guest/External User Management API
//! 
//! Provides endpoints for creating and managing temporary guest accounts,
//! guest access links, and tracking guest activity.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

// ============================================================================
// Data Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GuestUser {
    pub id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub expires_at: String,
    pub max_accesses: Option<i64>,
    pub access_count: i64,
    pub is_active: bool,
    pub last_accessed_at: Option<String>,
    pub notes: Option<String>,
    pub can_view: bool,
    pub can_download: bool,
    pub can_upload: bool,
    pub can_comment: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GuestAccessLink {
    pub id: String,
    pub guest_id: Option<String>,
    pub token: String,
    pub file_path: String,
    pub access_type: String,
    pub created_by: String,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub is_active: bool,
    pub access_count: i64,
    pub max_accesses: Option<i64>,
    pub last_accessed_at: Option<String>,
    pub can_view: bool,
    pub can_download: bool,
    pub can_upload: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GuestAccessLog {
    pub id: i64,
    pub guest_id: Option<String>,
    pub link_id: Option<String>,
    pub action: String,
    pub file_path: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub accessed_at: String,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GuestInvitation {
    pub id: String,
    pub email: String,
    pub invited_by: String,
    pub created_at: String,
    pub expires_at: String,
    pub token: String,
    pub message: Option<String>,
    pub is_accepted: bool,
    pub accepted_at: Option<String>,
    pub guest_id: Option<String>,
    pub can_view: bool,
    pub can_download: bool,
    pub can_upload: bool,
    pub can_comment: bool,
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateGuestRequest {
    pub display_name: String,
    pub email: Option<String>,
    pub expires_in_days: i32,
    pub max_accesses: Option<i64>,
    pub notes: Option<String>,
    pub can_view: Option<bool>,
    pub can_download: Option<bool>,
    pub can_upload: Option<bool>,
    pub can_comment: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGuestRequest {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub expires_at: Option<String>,
    pub max_accesses: Option<i64>,
    pub notes: Option<String>,
    pub is_active: Option<bool>,
    pub can_view: Option<bool>,
    pub can_download: Option<bool>,
    pub can_upload: Option<bool>,
    pub can_comment: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLinkRequest {
    pub file_path: String,
    pub access_type: Option<String>,
    pub guest_id: Option<String>,
    pub expires_in_days: Option<i32>,
    pub password: Option<String>,
    pub max_accesses: Option<i64>,
    pub can_view: Option<bool>,
    pub can_download: Option<bool>,
    pub can_upload: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvitationRequest {
    pub email: String,
    pub message: Option<String>,
    pub expires_in_days: i32,
    pub can_view: Option<bool>,
    pub can_download: Option<bool>,
    pub can_upload: Option<bool>,
    pub can_comment: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AccessLinkRequest {
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub include_expired: Option<bool>,
    pub include_inactive: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GuestResponse {
    pub guest: GuestUser,
}

#[derive(Debug, Serialize)]
pub struct GuestsListResponse {
    pub guests: Vec<GuestUser>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub link: GuestAccessLink,
    pub access_url: String,
}

#[derive(Debug, Serialize)]
pub struct LinksListResponse {
    pub links: Vec<GuestAccessLink>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct InvitationResponse {
    pub invitation: GuestInvitation,
}

#[derive(Debug, Serialize)]
pub struct InvitationsListResponse {
    pub invitations: Vec<GuestInvitation>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct ActivityLogResponse {
    pub logs: Vec<GuestAccessLog>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct GuestStatsResponse {
    pub total_guests: i64,
    pub active_guests: i64,
    pub total_links: i64,
    pub active_links: i64,
    pub total_accesses: i64,
    pub pending_invitations: i64,
}

#[derive(Debug, Serialize)]
pub struct OperationResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LinkAccessResponse {
    pub valid: bool,
    pub file_path: Option<String>,
    pub access_type: Option<String>,
    pub can_view: bool,
    pub can_download: bool,
    pub can_upload: bool,
    pub guest_name: Option<String>,
}

// ============================================================================
// Router
// ============================================================================

/// Protected routes - require authentication
pub fn router() -> Router<AppState> {
    Router::new()
        // Guest Users
        .route("/guests", get(list_guests).post(create_guest))
        .route("/guests/stats", get(get_guest_stats))
        .route("/guests/{guest_id}", get(get_guest).put(update_guest).delete(delete_guest))
        .route("/guests/{guest_id}/activity", get(get_guest_activity))
        .route("/guests/{guest_id}/convert", post(convert_guest_to_user))
        // Guest Access Links
        .route("/guest-links", get(list_links).post(create_link))
        .route("/guest-links/{link_id}", get(get_link).put(update_link).delete(delete_link))
        .route("/guest-links/{link_id}/toggle", post(toggle_link))
        // Guest Invitations
        .route("/guest-invitations", get(list_invitations).post(create_invitation))
        .route("/guest-invitations/{invitation_id}", delete(delete_invitation))
        .route("/guest-invitations/{invitation_id}/resend", post(resend_invitation))
}

/// Public routes - no authentication required (token-based access)
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/guest-access/{token}", get(access_link).post(access_link_with_password))
}

// ============================================================================
// Guest User Handlers
// ============================================================================

/// List all guest users created by the current user
async fn list_guests(
    State(state): State<AppState>,
    user: UserInfo,
    Query(query): Query<ListQuery>,
) -> Result<Json<GuestsListResponse>, StatusCode> {
    let include_expired = query.include_expired.unwrap_or(false);
    let include_inactive = query.include_inactive.unwrap_or(false);
    
    let mut sql = String::from(
        "SELECT * FROM guest_users WHERE created_by = ?"
    );
    
    if !include_expired {
        sql.push_str(" AND expires_at > datetime('now')");
    }
    if !include_inactive {
        sql.push_str(" AND is_active = 1");
    }
    sql.push_str(" ORDER BY created_at DESC");
    
    let guests: Vec<GuestUser> = sqlx::query_as(&sql)
        .bind(user.user_id())
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list guests: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let total = guests.len() as i64;
    
    Ok(Json(GuestsListResponse { guests, total }))
}

/// Create a new guest user
async fn create_guest(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateGuestRequest>,
) -> Result<Json<GuestResponse>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(req.expires_in_days as i64))
        .ok_or(StatusCode::BAD_REQUEST)?
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    
    sqlx::query(
        "INSERT INTO guest_users (
            id, display_name, email, created_by, expires_at, max_accesses, 
            notes, can_view, can_download, can_upload, can_comment
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&req.display_name)
    .bind(&req.email)
    .bind(user.user_id())
    .bind(&expires_at)
    .bind(&req.max_accesses)
    .bind(&req.notes)
    .bind(req.can_view.unwrap_or(true))
    .bind(req.can_download.unwrap_or(true))
    .bind(req.can_upload.unwrap_or(false))
    .bind(req.can_comment.unwrap_or(false))
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create guest: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let guest: GuestUser = sqlx::query_as("SELECT * FROM guest_users WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(GuestResponse { guest }))
}

/// Get a specific guest user
async fn get_guest(
    State(state): State<AppState>,
    user: UserInfo,
    Path(guest_id): Path<String>,
) -> Result<Json<GuestResponse>, StatusCode> {
    let guest: GuestUser = sqlx::query_as(
        "SELECT * FROM guest_users WHERE id = ? AND created_by = ?"
    )
    .bind(&guest_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(GuestResponse { guest }))
}

/// Update a guest user
async fn update_guest(
    State(state): State<AppState>,
    user: UserInfo,
    Path(guest_id): Path<String>,
    Json(req): Json<UpdateGuestRequest>,
) -> Result<Json<GuestResponse>, StatusCode> {
    // Verify ownership
    let _: (String,) = sqlx::query_as(
        "SELECT id FROM guest_users WHERE id = ? AND created_by = ?"
    )
    .bind(&guest_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();
    
    if let Some(name) = &req.display_name {
        updates.push("display_name = ?");
        values.push(name.clone());
    }
    if let Some(email) = &req.email {
        updates.push("email = ?");
        values.push(email.clone());
    }
    if let Some(expires) = &req.expires_at {
        updates.push("expires_at = ?");
        values.push(expires.clone());
    }
    if let Some(notes) = &req.notes {
        updates.push("notes = ?");
        values.push(notes.clone());
    }
    if let Some(active) = req.is_active {
        updates.push("is_active = ?");
        values.push(if active { "1".to_string() } else { "0".to_string() });
    }
    if let Some(view) = req.can_view {
        updates.push("can_view = ?");
        values.push(if view { "1".to_string() } else { "0".to_string() });
    }
    if let Some(download) = req.can_download {
        updates.push("can_download = ?");
        values.push(if download { "1".to_string() } else { "0".to_string() });
    }
    if let Some(upload) = req.can_upload {
        updates.push("can_upload = ?");
        values.push(if upload { "1".to_string() } else { "0".to_string() });
    }
    if let Some(comment) = req.can_comment {
        updates.push("can_comment = ?");
        values.push(if comment { "1".to_string() } else { "0".to_string() });
    }
    
    if updates.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let sql = format!(
        "UPDATE guest_users SET {} WHERE id = ?",
        updates.join(", ")
    );
    
    let mut query = sqlx::query(&sql);
    for value in values {
        query = query.bind(value);
    }
    query = query.bind(&guest_id);
    
    query.execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update guest: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let guest: GuestUser = sqlx::query_as("SELECT * FROM guest_users WHERE id = ?")
        .bind(&guest_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(GuestResponse { guest }))
}

/// Delete a guest user
async fn delete_guest(
    State(state): State<AppState>,
    user: UserInfo,
    Path(guest_id): Path<String>,
) -> Result<Json<OperationResponse>, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM guest_users WHERE id = ? AND created_by = ?"
    )
    .bind(&guest_id)
    .bind(user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    
    Ok(Json(OperationResponse {
        success: true,
        message: "Guest deleted successfully".to_string(),
    }))
}

/// Get guest activity log
async fn get_guest_activity(
    State(state): State<AppState>,
    user: UserInfo,
    Path(guest_id): Path<String>,
) -> Result<Json<ActivityLogResponse>, StatusCode> {
    // Verify ownership
    let _: (String,) = sqlx::query_as(
        "SELECT id FROM guest_users WHERE id = ? AND created_by = ?"
    )
    .bind(&guest_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    let logs: Vec<GuestAccessLog> = sqlx::query_as(
        "SELECT * FROM guest_access_log WHERE guest_id = ? ORDER BY accessed_at DESC LIMIT 100"
    )
    .bind(&guest_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let total = logs.len() as i64;
    
    Ok(Json(ActivityLogResponse { logs, total }))
}

/// Convert guest to full user
async fn convert_guest_to_user(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(_guest_id): Path<String>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // TODO: Implement guest to user conversion
    Ok(Json(OperationResponse {
        success: false,
        message: "Feature not yet implemented".to_string(),
    }))
}

/// Get guest statistics
async fn get_guest_stats(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<GuestStatsResponse>, StatusCode> {
    let total_guests: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM guest_users WHERE created_by = ?"
    )
    .bind(user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));
    
    let active_guests: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM guest_users WHERE created_by = ? AND is_active = 1 AND expires_at > datetime('now')"
    )
    .bind(user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));
    
    let total_links: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM guest_access_links WHERE created_by = ?"
    )
    .bind(user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));
    
    let active_links: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM guest_access_links WHERE created_by = ? AND is_active = 1 AND (expires_at IS NULL OR expires_at > datetime('now'))"
    )
    .bind(user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));
    
    let total_accesses: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(access_count), 0) FROM guest_access_links WHERE created_by = ?"
    )
    .bind(user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));
    
    let pending_invitations: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM guest_invitations WHERE invited_by = ? AND is_accepted = 0 AND expires_at > datetime('now')"
    )
    .bind(user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));
    
    Ok(Json(GuestStatsResponse {
        total_guests: total_guests.0,
        active_guests: active_guests.0,
        total_links: total_links.0,
        active_links: active_links.0,
        total_accesses: total_accesses.0,
        pending_invitations: pending_invitations.0,
    }))
}

// ============================================================================
// Guest Access Link Handlers
// ============================================================================

/// List all access links
async fn list_links(
    State(state): State<AppState>,
    user: UserInfo,
    Query(query): Query<ListQuery>,
) -> Result<Json<LinksListResponse>, StatusCode> {
    let include_expired = query.include_expired.unwrap_or(false);
    let include_inactive = query.include_inactive.unwrap_or(false);
    
    let mut sql = String::from(
        "SELECT * FROM guest_access_links WHERE created_by = ?"
    );
    
    if !include_expired {
        sql.push_str(" AND (expires_at IS NULL OR expires_at > datetime('now'))");
    }
    if !include_inactive {
        sql.push_str(" AND is_active = 1");
    }
    sql.push_str(" ORDER BY created_at DESC");
    
    let links: Vec<GuestAccessLink> = sqlx::query_as(&sql)
        .bind(user.user_id())
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let total = links.len() as i64;
    
    Ok(Json(LinksListResponse { links, total }))
}

/// Create a new access link
async fn create_link(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateLinkRequest>,
) -> Result<Json<LinkResponse>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let token = generate_secure_token();
    let access_type = req.access_type.unwrap_or_else(|| "file".to_string());
    
    let expires_at = req.expires_in_days.map(|days| {
        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(days as i64))
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }).flatten();
    
    let password_hash = req.password.as_ref().map(|p| {
        // Hash password with SHA256
        let mut hasher = Sha256::new();
        hasher.update(p.as_bytes());
        format!("{:x}", hasher.finalize())
    });
    
    sqlx::query(
        "INSERT INTO guest_access_links (
            id, guest_id, token, file_path, access_type, created_by,
            expires_at, password_hash, max_accesses, can_view, can_download, can_upload
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&req.guest_id)
    .bind(&token)
    .bind(&req.file_path)
    .bind(&access_type)
    .bind(user.user_id())
    .bind(&expires_at)
    .bind(&password_hash)
    .bind(&req.max_accesses)
    .bind(req.can_view.unwrap_or(true))
    .bind(req.can_download.unwrap_or(true))
    .bind(req.can_upload.unwrap_or(false))
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create link: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let link: GuestAccessLink = sqlx::query_as("SELECT * FROM guest_access_links WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let access_url = format!("/guest/{}", token);
    
    Ok(Json(LinkResponse { link, access_url }))
}

/// Get a specific link
async fn get_link(
    State(state): State<AppState>,
    user: UserInfo,
    Path(link_id): Path<String>,
) -> Result<Json<LinkResponse>, StatusCode> {
    let link: GuestAccessLink = sqlx::query_as(
        "SELECT * FROM guest_access_links WHERE id = ? AND created_by = ?"
    )
    .bind(&link_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    let access_url = format!("/guest/{}", link.token);
    
    Ok(Json(LinkResponse { link, access_url }))
}

/// Update a link
async fn update_link(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(_link_id): Path<String>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // TODO: Implement link update
    Ok(Json(OperationResponse {
        success: false,
        message: "Feature not yet implemented".to_string(),
    }))
}

/// Delete a link
async fn delete_link(
    State(state): State<AppState>,
    user: UserInfo,
    Path(link_id): Path<String>,
) -> Result<Json<OperationResponse>, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM guest_access_links WHERE id = ? AND created_by = ?"
    )
    .bind(&link_id)
    .bind(user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    
    Ok(Json(OperationResponse {
        success: true,
        message: "Link deleted successfully".to_string(),
    }))
}

/// Toggle link active status
async fn toggle_link(
    State(state): State<AppState>,
    user: UserInfo,
    Path(link_id): Path<String>,
) -> Result<Json<LinkResponse>, StatusCode> {
    // Toggle is_active
    sqlx::query(
        "UPDATE guest_access_links SET is_active = NOT is_active WHERE id = ? AND created_by = ?"
    )
    .bind(&link_id)
    .bind(user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let link: GuestAccessLink = sqlx::query_as(
        "SELECT * FROM guest_access_links WHERE id = ? AND created_by = ?"
    )
    .bind(&link_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    let access_url = format!("/guest/{}", link.token);
    
    Ok(Json(LinkResponse { link, access_url }))
}

// ============================================================================
// Guest Invitation Handlers
// ============================================================================

/// List invitations
async fn list_invitations(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<InvitationsListResponse>, StatusCode> {
    let invitations: Vec<GuestInvitation> = sqlx::query_as(
        "SELECT * FROM guest_invitations WHERE invited_by = ? ORDER BY created_at DESC"
    )
    .bind(user.user_id())
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let total = invitations.len() as i64;
    
    Ok(Json(InvitationsListResponse { invitations, total }))
}

/// Create invitation
async fn create_invitation(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateInvitationRequest>,
) -> Result<Json<InvitationResponse>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let token = generate_secure_token();
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(req.expires_in_days as i64))
        .ok_or(StatusCode::BAD_REQUEST)?
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    
    sqlx::query(
        "INSERT INTO guest_invitations (
            id, email, invited_by, expires_at, token, message,
            can_view, can_download, can_upload, can_comment
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&req.email)
    .bind(user.user_id())
    .bind(&expires_at)
    .bind(&token)
    .bind(&req.message)
    .bind(req.can_view.unwrap_or(true))
    .bind(req.can_download.unwrap_or(true))
    .bind(req.can_upload.unwrap_or(false))
    .bind(req.can_comment.unwrap_or(false))
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create invitation: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let invitation: GuestInvitation = sqlx::query_as(
        "SELECT * FROM guest_invitations WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(InvitationResponse { invitation }))
}

/// Delete invitation
async fn delete_invitation(
    State(state): State<AppState>,
    user: UserInfo,
    Path(invitation_id): Path<String>,
) -> Result<Json<OperationResponse>, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM guest_invitations WHERE id = ? AND invited_by = ?"
    )
    .bind(&invitation_id)
    .bind(user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    
    Ok(Json(OperationResponse {
        success: true,
        message: "Invitation deleted".to_string(),
    }))
}

/// Resend invitation email
async fn resend_invitation(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(_invitation_id): Path<String>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // TODO: Implement email sending
    Ok(Json(OperationResponse {
        success: false,
        message: "Email sending not yet implemented".to_string(),
    }))
}

// ============================================================================
// Public Access Handlers (No Auth Required)
// ============================================================================

/// Access a guest link (GET - for viewing)
async fn access_link(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<Json<LinkAccessResponse>, StatusCode> {
    let link: Option<GuestAccessLink> = sqlx::query_as(
        "SELECT * FROM guest_access_links WHERE token = ? AND is_active = 1 
         AND (expires_at IS NULL OR expires_at > datetime('now'))
         AND (max_accesses IS NULL OR access_count < max_accesses)"
    )
    .bind(&token)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match link {
        Some(l) => {
            // Check if password protected
            let needs_password: Option<(String,)> = sqlx::query_as(
                "SELECT password_hash FROM guest_access_links WHERE token = ? AND password_hash IS NOT NULL"
            )
            .bind(&token)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten();
            
            if needs_password.is_some() {
                return Ok(Json(LinkAccessResponse {
                    valid: true,
                    file_path: None,
                    access_type: Some(l.access_type),
                    can_view: false,
                    can_download: false,
                    can_upload: false,
                    guest_name: None,
                }));
            }
            
            // Increment access count
            sqlx::query(
                "UPDATE guest_access_links SET access_count = access_count + 1, last_accessed_at = datetime('now') WHERE token = ?"
            )
            .bind(&token)
            .execute(&state.db_pool)
            .await
            .ok();
            
            // Log access
            log_guest_access(&state.db_pool, l.guest_id.as_deref(), Some(&l.id), "link_access", Some(&l.file_path)).await;
            
            Ok(Json(LinkAccessResponse {
                valid: true,
                file_path: Some(l.file_path),
                access_type: Some(l.access_type),
                can_view: l.can_view,
                can_download: l.can_download,
                can_upload: l.can_upload,
                guest_name: None,
            }))
        }
        None => Ok(Json(LinkAccessResponse {
            valid: false,
            file_path: None,
            access_type: None,
            can_view: false,
            can_download: false,
            can_upload: false,
            guest_name: None,
        })),
    }
}

/// Access with password
async fn access_link_with_password(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(req): Json<AccessLinkRequest>,
) -> Result<Json<LinkAccessResponse>, StatusCode> {
    let link: Option<GuestAccessLink> = sqlx::query_as(
        "SELECT * FROM guest_access_links WHERE token = ? AND is_active = 1 
         AND (expires_at IS NULL OR expires_at > datetime('now'))
         AND (max_accesses IS NULL OR access_count < max_accesses)"
    )
    .bind(&token)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match link {
        Some(l) => {
            // Verify password if required
            let stored_hash: Option<(Option<String>,)> = sqlx::query_as(
                "SELECT password_hash FROM guest_access_links WHERE token = ?"
            )
            .bind(&token)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten();
            
            if let Some((Some(hash),)) = stored_hash {
                let provided_hash = req.password
                    .as_ref()
                    .map(|p| {
                        let mut hasher = Sha256::new();
                        hasher.update(p.as_bytes());
                        format!("{:x}", hasher.finalize())
                    })
                    .unwrap_or_default();
                
                if hash != provided_hash {
                    return Ok(Json(LinkAccessResponse {
                        valid: false,
                        file_path: None,
                        access_type: None,
                        can_view: false,
                        can_download: false,
                        can_upload: false,
                        guest_name: None,
                    }));
                }
            }
            
            // Increment access count
            sqlx::query(
                "UPDATE guest_access_links SET access_count = access_count + 1, last_accessed_at = datetime('now') WHERE token = ?"
            )
            .bind(&token)
            .execute(&state.db_pool)
            .await
            .ok();
            
            // Log access
            log_guest_access(&state.db_pool, l.guest_id.as_deref(), Some(&l.id), "link_access", Some(&l.file_path)).await;
            
            Ok(Json(LinkAccessResponse {
                valid: true,
                file_path: Some(l.file_path),
                access_type: Some(l.access_type),
                can_view: l.can_view,
                can_download: l.can_download,
                can_upload: l.can_upload,
                guest_name: None,
            }))
        }
        None => Ok(Json(LinkAccessResponse {
            valid: false,
            file_path: None,
            access_type: None,
            can_view: false,
            can_download: false,
            can_upload: false,
            guest_name: None,
        })),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate a secure random token
fn generate_secure_token() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect();
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, &bytes)
}

/// Log guest access activity
async fn log_guest_access(
    pool: &sqlx::SqlitePool,
    guest_id: Option<&str>,
    link_id: Option<&str>,
    action: &str,
    file_path: Option<&str>,
) {
    let _ = sqlx::query(
        "INSERT INTO guest_access_log (guest_id, link_id, action, file_path) VALUES (?, ?, ?, ?)"
    )
    .bind(guest_id)
    .bind(link_id)
    .bind(action)
    .bind(file_path)
    .execute(pool)
    .await;
}
