//! Activity log and audit trail API

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ActivityQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
    pub action: Option<String>,
}

fn default_limit() -> usize {
    100
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/activity", get(list_activity))
        .route("/activity/stats", get(get_stats))
        .route("/activity/mark-visited", put(mark_visited))
        .route("/activity/actions", get(get_available_actions))
}

async fn list_activity(
    State(state): State<AppState>,
    user: UserInfo,
    Query(query): Query<ActivityQuery>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // Use filtered version if action is specified
    let activities = services::activity::list_filtered(
        &state, 
        &user, 
        query.limit,
        query.action.as_deref()
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(activities))
}

async fn get_stats(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::activity::get_stats(&state, &user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn mark_visited(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    services::activity::mark_visited(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

/// GET /activity/actions - Returns all available action types
async fn get_available_actions() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "file_operations": [
            {"id": "upload", "label": "Upload", "icon": "bi-upload"},
            {"id": "download", "label": "Download", "icon": "bi-download"},
            {"id": "delete", "label": "Delete", "icon": "bi-trash"},
            {"id": "rename", "label": "Rename", "icon": "bi-pencil"},
            {"id": "move", "label": "Move", "icon": "bi-arrow-right"},
            {"id": "copy", "label": "Copy", "icon": "bi-files"},
            {"id": "create", "label": "Create", "icon": "bi-plus-circle"},
            {"id": "preview", "label": "Preview", "icon": "bi-eye"},
            {"id": "restore", "label": "Restore", "icon": "bi-arrow-counterclockwise"}
        ],
        "folder_operations": [
            {"id": "folder_create", "label": "Create Folder", "icon": "bi-folder-plus"},
            {"id": "folder_delete", "label": "Delete Folder", "icon": "bi-folder-x"},
            {"id": "folder_rename", "label": "Rename Folder", "icon": "bi-pencil"},
            {"id": "folder_move", "label": "Move Folder", "icon": "bi-arrow-right"},
            {"id": "folder_color", "label": "Change Color", "icon": "bi-palette"}
        ],
        "favorites": [
            {"id": "favorite", "label": "Add Favorite", "icon": "bi-star-fill"},
            {"id": "unfavorite", "label": "Remove Favorite", "icon": "bi-star"}
        ],
        "sharing": [
            {"id": "share", "label": "Share", "icon": "bi-share"},
            {"id": "unshare", "label": "Unshare", "icon": "bi-share-fill"},
            {"id": "share_access", "label": "Access Share", "icon": "bi-link-45deg"}
        ],
        "collaboration": [
            {"id": "comment_add", "label": "Add Comment", "icon": "bi-chat-dots"},
            {"id": "comment_delete", "label": "Delete Comment", "icon": "bi-chat-dots-fill"},
            {"id": "tag_add", "label": "Add Tag", "icon": "bi-tag"},
            {"id": "tag_remove", "label": "Remove Tag", "icon": "bi-tag-fill"}
        ],
        "versioning": [
            {"id": "version_create", "label": "Create Version", "icon": "bi-clock-history"},
            {"id": "version_restore", "label": "Restore Version", "icon": "bi-arrow-counterclockwise"},
            {"id": "version_delete", "label": "Delete Version", "icon": "bi-trash"}
        ],
        "auth": [
            {"id": "login", "label": "Login", "icon": "bi-box-arrow-in-right"},
            {"id": "logout", "label": "Logout", "icon": "bi-box-arrow-right"},
            {"id": "password_change", "label": "Password Change", "icon": "bi-key"},
            {"id": "2fa_enable", "label": "Enable 2FA", "icon": "bi-shield-check"},
            {"id": "2fa_disable", "label": "Disable 2FA", "icon": "bi-shield"}
        ],
        "settings": [
            {"id": "settings_change", "label": "Change Settings", "icon": "bi-gear"},
            {"id": "profile_update", "label": "Update Profile", "icon": "bi-person"}
        ]
    }))
}
