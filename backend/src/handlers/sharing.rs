//! File sharing handlers
//! 
//! Implements secure file sharing with permissions and expiration.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::User;
use crate::AppState;

// ==================== MODELS ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Share {
    pub id: String,
    pub file_id: Option<String>,
    pub folder_id: Option<String>,
    pub shared_by: String,
    pub shared_with: Option<String>, // NULL = public link
    pub share_link: String,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SharePermission {
    pub id: String,
    pub share_id: String,
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
    pub can_share: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShareWithPermissions {
    #[serde(flatten)]
    pub share: Share,
    pub permissions: SharePermission,
    pub file_name: Option<String>,
    pub folder_name: Option<String>,
}

// ==================== REQUESTS ====================

#[derive(Debug, Deserialize)]
pub struct CreateShareRequest {
    pub file_id: Option<String>,
    pub folder_id: Option<String>,
    pub shared_with: Option<String>, // user_id or email, None = public link
    pub expires_in_days: Option<i64>, // None = never expires
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
    pub can_share: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionsRequest {
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
    pub can_share: bool,
}

// ==================== HANDLERS ====================

/// Create a new share
/// POST /api/shares
pub async fn create_share(
    user: User,
    State(state): State<AppState>,
    Json(req): Json<CreateShareRequest>,
) -> Result<Json<ShareWithPermissions>, StatusCode> {
    // Validate: must share either file or folder
    if req.file_id.is_none() && req.folder_id.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if req.file_id.is_some() && req.folder_id.is_some() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let share_id = Uuid::new_v4().to_string();
    let share_link = Uuid::new_v4().to_string(); // Random UUID as share link
    let now = Utc::now();
    
    let expires_at = req.expires_in_days.map(|days| {
        (now + Duration::days(days)).to_rfc3339()
    });

    // Verify ownership
    if let Some(file_id) = &req.file_id {
        let owner: Option<(String,)> = sqlx::query_as(
            "SELECT owner_id FROM files WHERE id = ? AND is_deleted = 0"
        )
        .bind(file_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if owner.map(|o| o.0) != Some(user.id.to_string()) {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    if let Some(folder_id) = &req.folder_id {
        let owner: Option<(String,)> = sqlx::query_as(
            "SELECT owner_id FROM folders WHERE id = ? AND is_deleted = 0"
        )
        .bind(folder_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if owner.map(|o| o.0) != Some(user.id.to_string()) {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Create share
    sqlx::query(
        r#"
        INSERT INTO shares 
        (id, file_id, folder_id, shared_by, shared_with, share_link, expires_at, created_at, is_active)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1)
        "#
    )
    .bind(&share_id)
    .bind(&req.file_id)
    .bind(&req.folder_id)
    .bind(&user.id.to_string())
    .bind(&req.shared_with)
    .bind(&share_link)
    .bind(&expires_at)
    .bind(now.to_rfc3339())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create permissions
    let perm_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO share_permissions
        (id, share_id, can_read, can_write, can_delete, can_share)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&perm_id)
    .bind(&share_id)
    .bind(req.can_read)
    .bind(req.can_write)
    .bind(req.can_delete)
    .bind(req.can_share)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO activity_log (id, user_id, action, resource_type, resource_id, timestamp)
        VALUES (?, ?, 'share_created', ?, ?, ?)
        "#
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.id.to_string())
    .bind(if req.file_id.is_some() { "file" } else { "folder" })
    .bind(req.file_id.as_ref().or(req.folder_id.as_ref()).unwrap())
    .bind(now.to_rfc3339())
    .execute(&state.db_pool)
    .await;

    // Fetch created share with permissions
    let share: Share = sqlx::query_as("SELECT * FROM shares WHERE id = ?")
        .bind(&share_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let permissions: SharePermission = sqlx::query_as(
        "SELECT * FROM share_permissions WHERE share_id = ?"
    )
    .bind(&share_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ShareWithPermissions {
        share,
        permissions,
        file_name: None,
        folder_name: None,
    }))
}

/// List all shares created by the current user
/// GET /api/shares
pub async fn list_shares(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<Vec<ShareWithPermissions>>, StatusCode> {
    let shares: Vec<Share> = sqlx::query_as(
        r#"
        SELECT * FROM shares 
        WHERE shared_by = ? AND is_active = 1
        ORDER BY created_at DESC
        "#
    )
    .bind(&user.id.to_string())
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut result = Vec::new();
    for share in shares {
        let permissions: SharePermission = sqlx::query_as(
            "SELECT * FROM share_permissions WHERE share_id = ?"
        )
        .bind(&share.id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let file_name: Option<(String,)> = if share.file_id.is_some() {
            sqlx::query_as("SELECT name FROM files WHERE id = ?")
                .bind(share.file_id.as_ref().unwrap())
                .fetch_optional(&state.db_pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        } else {
            None
        };

        let folder_name: Option<(String,)> = if share.folder_id.is_some() {
            sqlx::query_as("SELECT name FROM folders WHERE id = ?")
                .bind(share.folder_id.as_ref().unwrap())
                .fetch_optional(&state.db_pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        } else {
            None
        };

        result.push(ShareWithPermissions {
            share,
            permissions,
            file_name: file_name.map(|n| n.0),
            folder_name: folder_name.map(|n| n.0),
        });
    }

    Ok(Json(result))
}

/// List files/folders shared with me
/// GET /api/shared-with-me
pub async fn list_shared_with_me(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<Vec<ShareWithPermissions>>, StatusCode> {
    let shares: Vec<Share> = sqlx::query_as(
        r#"
        SELECT * FROM shares 
        WHERE shared_with = ? AND is_active = 1
        AND (expires_at IS NULL OR expires_at > ?)
        ORDER BY created_at DESC
        "#
    )
    .bind(&user.id.to_string())
    .bind(Utc::now().to_rfc3339())
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut result = Vec::new();
    for share in shares {
        let permissions: SharePermission = sqlx::query_as(
            "SELECT * FROM share_permissions WHERE share_id = ?"
        )
        .bind(&share.id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let file_name: Option<(String,)> = if share.file_id.is_some() {
            sqlx::query_as("SELECT name FROM files WHERE id = ?")
                .bind(share.file_id.as_ref().unwrap())
                .fetch_optional(&state.db_pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        } else {
            None
        };

        let folder_name: Option<(String,)> = if share.folder_id.is_some() {
            sqlx::query_as("SELECT name FROM folders WHERE id = ?")
                .bind(share.folder_id.as_ref().unwrap())
                .fetch_optional(&state.db_pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        } else {
            None
        };

        result.push(ShareWithPermissions {
            share,
            permissions,
            file_name: file_name.map(|n| n.0),
            folder_name: folder_name.map(|n| n.0),
        });
    }

    Ok(Json(result))
}

/// Delete a share (revoke access)
/// DELETE /api/shares/{id}
pub async fn delete_share(
    user: User,
    State(state): State<AppState>,
    Path(share_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Verify ownership
    let share: Option<Share> = sqlx::query_as(
        "SELECT * FROM shares WHERE id = ? AND shared_by = ?"
    )
    .bind(&share_id)
    .bind(&user.id.to_string())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if share.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Soft delete
    sqlx::query("UPDATE shares SET is_active = 0 WHERE id = ?")
        .bind(&share_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO activity_log (id, user_id, action, resource_type, resource_id, timestamp)
        VALUES (?, ?, 'share_revoked', 'share', ?, ?)
        "#
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.id.to_string())
    .bind(&share_id)
    .bind(Utc::now().to_rfc3339())
    .execute(&state.db_pool)
    .await;

    Ok(StatusCode::NO_CONTENT)
}

/// Update share permissions
/// PUT /api/shares/{id}/permissions
pub async fn update_permissions(
    user: User,
    State(state): State<AppState>,
    Path(share_id): Path<String>,
    Json(req): Json<UpdatePermissionsRequest>,
) -> Result<Json<SharePermission>, StatusCode> {
    // Verify ownership
    let share: Option<Share> = sqlx::query_as(
        "SELECT * FROM shares WHERE id = ? AND shared_by = ?"
    )
    .bind(&share_id)
    .bind(&user.id.to_string())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if share.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Update permissions
    sqlx::query(
        r#"
        UPDATE share_permissions 
        SET can_read = ?, can_write = ?, can_delete = ?, can_share = ?
        WHERE share_id = ?
        "#
    )
    .bind(req.can_read)
    .bind(req.can_write)
    .bind(req.can_delete)
    .bind(req.can_share)
    .bind(&share_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch updated permissions
    let permissions: SharePermission = sqlx::query_as(
        "SELECT * FROM share_permissions WHERE share_id = ?"
    )
    .bind(&share_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(permissions))
}
