// API Handlers for Backend-First Architecture
// All data management happens here - frontend only displays data

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::{auth, database::*, AppState, UserPreferences};

// ==================== FOLDERS API ====================

#[derive(Debug, Deserialize)]
pub struct ListFoldersQuery {
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_id: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFolderRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

pub async fn list_folders(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<ListFoldersQuery>,
) -> Result<Json<Vec<Folder>>, StatusCode> {
    let folders: Vec<Folder> = sqlx::query_as(
        "SELECT * FROM folders 
         WHERE owner_id = ? AND parent_id = ? AND is_deleted = 0
         ORDER BY name ASC"
    )
        .bind(&user.id.to_string())
        .bind(&params.parent_id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(folders))
}

pub async fn create_folder(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<CreateFolderRequest>,
) -> Result<Json<Folder>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Calculate path based on parent
    let path = if let Some(ref parent_id) = req.parent_id {
        let parent: Option<Folder> = sqlx::query_as(
            "SELECT * FROM folders WHERE id = ? AND owner_id = ?"
        )
            .bind(parent_id)
            .bind(&user.id.to_string())
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match parent {
            Some(p) => format!("{}/{}", p.path, req.name),
            None => return Err(StatusCode::NOT_FOUND),
        }
    } else {
        format!("/{}", req.name)
    };

    sqlx::query(
        "INSERT INTO folders (id, name, path, parent_id, owner_id, color, icon, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(&id)
        .bind(&req.name)
        .bind(&path)
        .bind(&req.parent_id)
        .bind(&user.id.to_string())
        .bind(&req.color)
        .bind(&req.icon)
        .bind(&now)
        .bind(&now)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let folder: Folder = sqlx::query_as("SELECT * FROM folders WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(folder))
}

pub async fn update_folder(
    State(state): State<AppState>,
    user: auth::User,
    Path(id): Path<String>,
    Json(req): Json<UpdateFolderRequest>,
) -> Result<Json<Folder>, StatusCode> {
    let now = Utc::now().to_rfc3339();

    // Verify ownership
    let existing: Option<Folder> = sqlx::query_as(
        "SELECT * FROM folders WHERE id = ? AND owner_id = ?"
    )
    .bind(&id)
    .bind(&user.id.to_string())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut updates = Vec::new();
    let mut bindings: Vec<String> = Vec::new();

    if let Some(ref name) = req.name {
        updates.push("name = ?");
        bindings.push(name.clone());
        
        // Recalculate path if name changed
        // TODO: Also update child folders' paths
    }
    if let Some(ref color) = req.color {
        updates.push("color = ?");
        bindings.push(color.clone());
    }
    if let Some(ref icon) = req.icon {
        updates.push("icon = ?");
        bindings.push(icon.clone());
    }

    updates.push("updated_at = ?");
    bindings.push(now.clone());

    let sql = format!("UPDATE folders SET {} WHERE id = ?", updates.join(", "));
    let mut query = sqlx::query(&sql);
    
    for binding in bindings {
        query = query.bind(binding);
    }
    query = query.bind(&id);

    query.execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let folder: Folder = sqlx::query_as("SELECT * FROM folders WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(folder))
}

pub async fn delete_folder(
    State(state): State<AppState>,
    user: auth::User,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let now = Utc::now().to_rfc3339();

    // Soft delete
    sqlx::query(
        "UPDATE folders SET is_deleted = 1, deleted_at = ? WHERE id = ? AND owner_id = ?"
    )
    .bind(&now)
    .bind(&id)
    .bind(&user.id.to_string())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Also soft-delete all files in this folder and subfolders

    Ok(StatusCode::NO_CONTENT)
}

// ==================== NOTIFICATIONS API ====================

#[derive(Debug, Deserialize)]
pub struct ListNotificationsQuery {
    pub is_read: Option<bool>,
}

pub async fn list_notifications(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<ListNotificationsQuery>,
) -> Result<Json<Vec<Notification>>, StatusCode> {
    let notifications: Vec<Notification> = if let Some(is_read) = params.is_read {
        sqlx::query_as(
            "SELECT * FROM notifications 
             WHERE user_id = ? AND is_read = ?
             ORDER BY created_at DESC"
        )
            .bind(&user.id.to_string())
            .bind(is_read as i64)
            .fetch_all(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_as(
            "SELECT * FROM notifications 
             WHERE user_id = ?
             ORDER BY created_at DESC"
        )
            .bind(&user.id.to_string())
            .fetch_all(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(notifications))
}

pub async fn mark_notification_read(
    State(state): State<AppState>,
    user: auth::User,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "UPDATE notifications SET is_read = 1 WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user.id.to_string())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_notification(
    State(state): State<AppState>,
    user: auth::User,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "DELETE FROM notifications WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user.id.to_string())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// Helper function for backend to create notifications
pub async fn create_notification(
    pool: &SqlitePool,
    user_id: &str,
    notification_type: &str,
    title: &str,
    message: &str,
    action_url: Option<&str>,
    priority: &str,
) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO notifications 
         (id, user_id, notification_type, title, message, action_url, is_read, priority, created_at)
         VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(notification_type)
    .bind(title)
    .bind(message)
    .bind(action_url)
    .bind(priority)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(())
}

// ==================== USER PREFERENCES API ====================

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub view_mode: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub sidebar_collapsed: Option<bool>,
    pub notification_email: Option<bool>,
    pub notification_browser: Option<bool>,
    pub notification_sound: Option<bool>,
    pub activity_visible: Option<bool>,
}

pub async fn get_user_preferences(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<UserPreferences>, StatusCode> {
    // Query only the fields we need, matching the UserPreferences struct
    let prefs: Option<(String, String, i64, String, String)> = sqlx::query_as(
        "SELECT view_mode, COALESCE(recent_searches, '[]'), sidebar_collapsed, sort_by, sort_order 
         FROM user_preferences WHERE user_id = ?"
    )
    .bind(&user.id.to_string())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("❌ GET preferences error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    if let Some((view_mode, recent_searches_json, sidebar_collapsed, sort_by, sort_order)) = prefs {
        let recent_searches: Vec<String> = serde_json::from_str(&recent_searches_json).unwrap_or_default();
        
        Ok(Json(UserPreferences {
            view_mode,
            recent_searches,
            sidebar_collapsed: sidebar_collapsed != 0,
            sort_by,
            sort_order,
            auto_refresh: true,
            upload_progress_visible: true,
        }))
    } else {
        // Auto-create default preferences
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        sqlx::query(
            "INSERT INTO user_preferences 
             (id, user_id, view_mode, sort_by, sort_order, recent_searches, sidebar_collapsed,
              notification_email, notification_browser, notification_sound, 
              activity_visible, created_at, updated_at)
             VALUES (?, ?, 'grid', 'name', 'asc', '[]', 0, 1, 1, 0, 1, ?, ?)"
        )
        .bind(&id)
        .bind(&user.id.to_string())
        .bind(&now)
        .bind(&now)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("❌ INSERT preferences error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Json(UserPreferences {
            view_mode: "grid".to_string(),
            recent_searches: vec![],
            sidebar_collapsed: false,
            sort_by: "name".to_string(),
            sort_order: "asc".to_string(),
            auto_refresh: true,
            upload_progress_visible: true,
        }))
    }
}

pub async fn update_user_preferences(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<UpdatePreferencesRequest>,
) -> Result<Json<UserPreferences>, StatusCode> {
    let now = Utc::now().to_rfc3339();

    let mut updates = Vec::new();
    let mut bindings: Vec<String> = Vec::new();

    if let Some(ref view_mode) = req.view_mode {
        updates.push("view_mode = ?");
        bindings.push(view_mode.clone());
    }
    if let Some(ref sort_by) = req.sort_by {
        updates.push("sort_by = ?");
        bindings.push(sort_by.clone());
    }
    if let Some(ref sort_order) = req.sort_order {
        updates.push("sort_order = ?");
        bindings.push(sort_order.clone());
    }
    if let Some(sidebar_collapsed) = req.sidebar_collapsed {
        updates.push("sidebar_collapsed = ?");
        bindings.push((sidebar_collapsed as i64).to_string());
    }
    if let Some(notification_email) = req.notification_email {
        updates.push("notification_email = ?");
        bindings.push((notification_email as i64).to_string());
    }
    if let Some(notification_browser) = req.notification_browser {
        updates.push("notification_browser = ?");
        bindings.push((notification_browser as i64).to_string());
    }
    if let Some(notification_sound) = req.notification_sound {
        updates.push("notification_sound = ?");
        bindings.push((notification_sound as i64).to_string());
    }
    if let Some(activity_visible) = req.activity_visible {
        updates.push("activity_visible = ?");
        bindings.push((activity_visible as i64).to_string());
    }

    if updates.is_empty() {
        // No updates, just return current preferences
        return get_user_preferences(State(state), user).await;
    }

    updates.push("updated_at = ?");
    bindings.push(now.clone());

    let user_id_str = user.id.to_string();
    let sql = format!("UPDATE user_preferences SET {} WHERE user_id = ?", updates.join(", "));
    let mut query = sqlx::query(&sql);
    
    for binding in bindings {
        query = query.bind(binding);
    }
    query = query.bind(&user_id_str);

    query.execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    get_user_preferences(State(state), user).await
}

// ==================== FAVORITES API ====================

pub async fn list_favorites(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<Favorite>>, StatusCode> {
    let favorites: Vec<Favorite> = sqlx::query_as(
        "SELECT * FROM favorites 
         WHERE user_id = ?
         ORDER BY sort_order ASC, created_at DESC"
    )
    .bind(&user.id.to_string())
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(favorites))
}

#[derive(Debug, Deserialize)]
pub struct AddFavoriteRequest {
    pub item_type: String,
    pub item_id: String,
}

pub async fn add_favorite(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<AddFavoriteRequest>,
) -> Result<Json<Favorite>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO favorites (id, user_id, item_type, item_id, sort_order, created_at)
         VALUES (?, ?, ?, ?, 0, ?)"
    )
    .bind(&id)
    .bind(&user.id.to_string())
    .bind(&req.item_type)
    .bind(&req.item_id)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let favorite: Favorite = sqlx::query_as("SELECT * FROM favorites WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(favorite))
}

pub async fn remove_favorite(
    State(state): State<AppState>,
    user: auth::User,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "DELETE FROM favorites WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user.id.to_string())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// ==================== RECENT FILES API ====================

#[derive(Debug, Deserialize)]
pub struct ListRecentFilesQuery {
    pub limit: Option<i64>,
}

pub async fn list_recent_files(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<ListRecentFilesQuery>,
) -> Result<Json<Vec<RecentFile>>, StatusCode> {
    let limit = params.limit.unwrap_or(20);

    let recent_files: Vec<RecentFile> = sqlx::query_as(
        "SELECT * FROM recent_files 
         WHERE user_id = ?
         ORDER BY last_accessed_at DESC
         LIMIT ?"
    )
    .bind(&user.id.to_string())
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(recent_files))
}

// Helper function to track file access
pub async fn track_file_access(
    pool: &SqlitePool,
    user_id: &str,
    file_id: &str,
    access_type: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    // Check if record exists
    let existing: Option<RecentFile> = sqlx::query_as(
        "SELECT * FROM recent_files WHERE user_id = ? AND file_id = ?"
    )
    .bind(user_id)
    .bind(file_id)
    .fetch_optional(pool)
    .await?;

    if let Some(mut rec) = existing {
        // Update existing record
        sqlx::query(
            "UPDATE recent_files 
             SET access_count = ?, last_accessed_at = ?, access_type = ?
             WHERE user_id = ? AND file_id = ?"
        )
        .bind(rec.access_count + 1)
        .bind(&now)
        .bind(access_type)
        .bind(user_id)
        .bind(file_id)
        .execute(pool)
        .await?;
    } else {
        // Insert new record
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO recent_files (id, user_id, file_id, access_count, last_accessed_at, access_type)
             VALUES (?, ?, ?, 1, ?, ?)"
        )
        .bind(&id)
        .bind(user_id)
        .bind(file_id)
        .bind(&now)
        .bind(access_type)
        .execute(pool)
        .await?;
    }

    Ok(())
}
