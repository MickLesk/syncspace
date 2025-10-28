//! Collaboration handlers for real-time collaborative editing
//! Handles file locks, user presence, and collaboration activity

use crate::auth;
use crate::database;
use crate::{AppState, FileChangeEvent};
use axum::{
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AcquireLockRequest {
    pub file_path: String,
    pub lock_type: Option<String>, // 'exclusive' or 'shared'
    pub duration_seconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePresenceRequest {
    pub file_path: Option<String>,
    pub activity_type: String, // 'viewing', 'editing', 'idle'
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ResolveConflictRequest {
    pub resolution_strategy: String, // 'auto_merge', 'manual', 'keep_latest', 'keep_version'
    pub details: Option<serde_json::Value>,
}

/// List all active file locks
pub async fn list_file_locks_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let file_path = params.get("file_path");
    
    let locks = if let Some(path) = file_path {
        sqlx::query_as::<_, database::FileLock>(
            "SELECT id, file_id, file_path, locked_by, locked_at, expires_at, lock_type, last_heartbeat 
             FROM file_locks 
             WHERE file_path = ? AND datetime(expires_at) > datetime('now')
             ORDER BY locked_at DESC"
        )
        .bind(path)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_as::<_, database::FileLock>(
            "SELECT id, file_id, file_path, locked_by, locked_at, expires_at, lock_type, last_heartbeat 
             FROM file_locks 
             WHERE datetime(expires_at) > datetime('now')
             ORDER BY locked_at DESC"
        )
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };
    
    Ok(Json(serde_json::json!({ "locks": locks })))
}

/// Acquire a file lock
pub async fn acquire_file_lock_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(request): Json<AcquireLockRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let lock_type = request.lock_type.unwrap_or_else(|| "exclusive".to_string());
    let duration = request.duration_seconds.unwrap_or(300); // Default 5 minutes
    
    let now = Utc::now();
    let expires_at = now + chrono::Duration::seconds(duration);
    
    // Check for existing locks
    let existing_locks: Vec<database::FileLock> = sqlx::query_as(
        "SELECT id, file_id, file_path, locked_by, locked_at, expires_at, lock_type, last_heartbeat 
         FROM file_locks 
         WHERE file_path = ? AND datetime(expires_at) > datetime('now')"
    )
    .bind(&request.file_path)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Check if lock can be acquired
    if lock_type == "exclusive" && !existing_locks.is_empty() {
        return Err(StatusCode::CONFLICT);
    }
    
    if !existing_locks.is_empty() {
        let has_exclusive = existing_locks.iter().any(|l| l.lock_type == "exclusive");
        if has_exclusive {
            return Err(StatusCode::CONFLICT);
        }
    }
    
    let lock_id = Uuid::new_v4().to_string();
    let file_id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO file_locks (id, file_id, file_path, locked_by, locked_at, expires_at, lock_type, last_heartbeat)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&lock_id)
    .bind(&file_id)
    .bind(&request.file_path)
    .bind(&user.username)
    .bind(now.to_rfc3339())
    .bind(expires_at.to_rfc3339())
    .bind(&lock_type)
    .bind(now.to_rfc3339())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Log activity
    sqlx::query(
        "INSERT INTO collaboration_activity (id, user_id, username, file_path, activity_type, created_at)
         VALUES (?, ?, ?, ?, 'lock_acquired', ?)"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.username) // Using username as user_id for now
    .bind(&user.username)
    .bind(&request.file_path)
    .bind(now.to_rfc3339())
    .execute(&state.db_pool)
    .await
    .ok();
    
    // Broadcast lock event via WebSocket
    let event = FileChangeEvent {
        path: request.file_path.clone(),
        kind: "lock_acquired".to_string(),
        timestamp: now.to_rfc3339(),
        user_id: Some(user.username.clone()),
        metadata: Some(serde_json::json!({
            "lock_id": lock_id,
            "lock_type": lock_type
        })),
    };
    let _ = state.fs_tx.send(event);
    
    Ok(Json(serde_json::json!({
        "lock_id": lock_id,
        "file_path": request.file_path,
        "locked_by": user.username,
        "expires_at": expires_at.to_rfc3339(),
        "lock_type": lock_type
    })))
}

/// Release a file lock
pub async fn release_file_lock_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(lock_id): AxumPath<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get lock info before deleting
    let lock: Option<database::FileLock> = sqlx::query_as(
        "SELECT id, file_id, file_path, locked_by, locked_at, expires_at, lock_type, last_heartbeat 
         FROM file_locks WHERE id = ?"
    )
    .bind(&lock_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(lock) = lock {
        // Verify ownership
        if lock.locked_by != user.username {
            return Err(StatusCode::FORBIDDEN);
        }
        
        // Delete lock
        sqlx::query("DELETE FROM file_locks WHERE id = ?")
            .bind(&lock_id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Log activity
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO collaboration_activity (id, user_id, username, file_path, activity_type, created_at)
             VALUES (?, ?, ?, ?, 'lock_released', ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&user.username)
        .bind(&user.username)
        .bind(&lock.file_path)
        .bind(now.to_rfc3339())
        .execute(&state.db_pool)
        .await
        .ok();
        
        // Broadcast unlock event
        let event = FileChangeEvent {
            path: lock.file_path,
            kind: "lock_released".to_string(),
            timestamp: now.to_rfc3339(),
            user_id: Some(user.username.clone()),
            metadata: None,
        };
        let _ = state.fs_tx.send(event);
        
        Ok(Json(serde_json::json!({"status": "Lock released"})))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Update lock heartbeat to keep it alive
pub async fn lock_heartbeat_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(lock_id): AxumPath<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let now = Utc::now();
    
    // Update heartbeat and extend expiration
    let result = sqlx::query(
        "UPDATE file_locks 
         SET last_heartbeat = ?, expires_at = datetime(expires_at, '+5 minutes')
         WHERE id = ? AND locked_by = ?"
    )
    .bind(now.to_rfc3339())
    .bind(&lock_id)
    .bind(&user.username)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() > 0 {
        Ok(Json(serde_json::json!({"status": "Heartbeat updated"})))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get all active user presence
pub async fn get_user_presence_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let file_path = params.get("file_path");
    
    // Clean up stale presence (older than 2 minutes)
    sqlx::query("DELETE FROM user_presence WHERE datetime(last_seen, '+2 minutes') < datetime('now')")
        .execute(&state.db_pool)
        .await
        .ok();
    
    let presence = if let Some(path) = file_path {
        sqlx::query_as::<_, database::UserPresence>(
            "SELECT id, user_id, username, file_path, activity_type, last_seen, metadata 
             FROM user_presence 
             WHERE file_path = ?
             ORDER BY last_seen DESC"
        )
        .bind(path)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_as::<_, database::UserPresence>(
            "SELECT id, user_id, username, file_path, activity_type, last_seen, metadata 
             FROM user_presence 
             ORDER BY last_seen DESC LIMIT 100"
        )
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };
    
    Ok(Json(serde_json::json!({ "presence": presence })))
}

/// Update user presence
pub async fn update_user_presence_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(request): Json<UpdatePresenceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let now = Utc::now();
    let presence_id = Uuid::new_v4().to_string();
    let metadata_json = request.metadata.map(|m| m.to_string());
    
    // Delete old presence for this user
    sqlx::query("DELETE FROM user_presence WHERE username = ?")
        .bind(&user.username)
        .execute(&state.db_pool)
        .await
        .ok();
    
    // Insert new presence
    sqlx::query(
        "INSERT INTO user_presence (id, user_id, username, file_path, activity_type, last_seen, metadata)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&presence_id)
    .bind(&user.username)
    .bind(&user.username)
    .bind(&request.file_path)
    .bind(&request.activity_type)
    .bind(now.to_rfc3339())
    .bind(metadata_json)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Broadcast presence update
    if let Some(path) = &request.file_path {
        let event = FileChangeEvent {
            path: path.clone(),
            kind: "presence_updated".to_string(),
            timestamp: now.to_rfc3339(),
            user_id: Some(user.username.clone()),
            metadata: Some(serde_json::json!({
                "activity_type": request.activity_type
            })),
        };
        let _ = state.fs_tx.send(event);
    }
    
    Ok(Json(serde_json::json!({
        "presence_id": presence_id,
        "status": "Presence updated"
    })))
}

/// Remove user presence
pub async fn remove_user_presence_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(user_id): AxumPath<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Only allow users to remove their own presence
    if user_id != user.username {
        return Err(StatusCode::FORBIDDEN);
    }
    
    sqlx::query("DELETE FROM user_presence WHERE username = ?")
        .bind(&user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({"status": "Presence removed"})))
}

/// Get collaboration activity log
pub async fn get_collaboration_activity_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let limit: i64 = params.get("limit").and_then(|l| l.parse().ok()).unwrap_or(50);
    
    let activities = sqlx::query_as::<_, database::CollaborationActivity>(
        "SELECT id, user_id, username, file_path, activity_type, details, created_at 
         FROM collaboration_activity 
         ORDER BY created_at DESC 
         LIMIT ?"
    )
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({ "activities": activities })))
}

/// Get activity for specific file
pub async fn get_file_activity_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(file_path): AxumPath<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let activities = sqlx::query_as::<_, database::CollaborationActivity>(
        "SELECT id, user_id, username, file_path, activity_type, details, created_at 
         FROM collaboration_activity 
         WHERE file_path = ?
         ORDER BY created_at DESC 
         LIMIT 50"
    )
    .bind(&file_path)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({ "activities": activities })))
}

/// List edit conflicts
pub async fn list_edit_conflicts_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let status = params.get("status").map(|s| s.as_str()).unwrap_or("pending");
    
    let conflicts = sqlx::query_as::<_, database::EditConflict>(
        "SELECT id, file_id, file_path, conflict_type, user1_id, user2_id, detected_at, 
                resolved_at, resolution_strategy, status, details 
         FROM edit_conflicts 
         WHERE status = ?
         ORDER BY detected_at DESC 
         LIMIT 100"
    )
    .bind(status)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({ "conflicts": conflicts })))
}

/// Resolve an edit conflict
pub async fn resolve_edit_conflict_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(conflict_id): AxumPath<String>,
    Json(request): Json<ResolveConflictRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let now = Utc::now();
    let details_json = request.details.map(|d| d.to_string());
    
    let result = sqlx::query(
        "UPDATE edit_conflicts 
         SET status = 'resolved', resolved_at = ?, resolution_strategy = ?, details = ?
         WHERE id = ?"
    )
    .bind(now.to_rfc3339())
    .bind(&request.resolution_strategy)
    .bind(details_json)
    .bind(&conflict_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() > 0 {
        Ok(Json(serde_json::json!({"status": "Conflict resolved"})))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
