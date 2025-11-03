//! Trash handlers for soft-delete and restore operations

use crate::auth;
use crate::{AppState, FileChangeEvent, DATA_DIR};
use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use std::path::Path;
use tokio::fs;

#[derive(Serialize, sqlx::FromRow)]
pub struct TrashItem {
    pub id: String,
    pub item_type: String,
    pub original_path: String,
    pub original_name: String,
    pub deleted_at: String,
    pub deleted_by_username: String,
    pub size_bytes: i64,
    pub auto_delete_at: Option<String>,
}

pub async fn list_trash_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<TrashItem>>, StatusCode> {
    let pool = &state.db_pool;
    
    let items = sqlx::query_as::<_, TrashItem>(
        r#"
        SELECT 
            t.id,
            t.item_type,
            t.original_path,
            t.original_name,
            t.deleted_at,
            u.username as deleted_by_username,
            t.size_bytes,
            t.auto_delete_at
        FROM trash t
        JOIN users u ON t.deleted_by = u.id
        WHERE t.deleted_by = ?
        ORDER BY t.deleted_at DESC
        "#
    )
    .bind(&user.id.to_string())
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch trash: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(items))
}

pub async fn restore_trash_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    
    // Find trash item by original path
    let trash_item: Option<(String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, item_type, item_id, original_parent_id FROM trash 
         WHERE original_path = ? AND deleted_by = ?"
    )
    .bind(&path)
    .bind(&user.id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some((trash_id, item_type, item_id, _)) = trash_item {
        // Update the item to unmark deletion
        match item_type.as_str() {
            "file" => {
                sqlx::query(
                    "UPDATE files SET is_deleted = 0, deleted_at = NULL, deleted_by = NULL 
                     WHERE id = ?"
                )
                .bind(&item_id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            }
            "folder" => {
                sqlx::query(
                    "UPDATE folders SET is_deleted = 0, deleted_at = NULL, deleted_by = NULL 
                     WHERE id = ?"
                )
                .bind(&item_id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            }
            _ => return Err(StatusCode::BAD_REQUEST),
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Broadcast restore event
        let _ = state.fs_tx.send(
            FileChangeEvent::new(path.clone(), "restore".to_string())
                .with_user(user.id.to_string())
        );
        
        Ok((StatusCode::OK, "restored"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn permanent_delete_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    
    // Find trash item
    let trash_item: Option<(String, String, String, String)> = sqlx::query_as(
        "SELECT id, item_type, item_id, original_path FROM trash 
         WHERE original_path = ? AND deleted_by = ?"
    )
    .bind(&path)
    .bind(&user.id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some((trash_id, item_type, item_id, original_path)) = trash_item {
        // Get storage path before deletion
        let storage_path: Option<String> = if item_type == "file" {
            sqlx::query_scalar("SELECT storage_path FROM files WHERE id = ?")
                .bind(&item_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten()
        } else {
            None
        };
        
        // Delete from database
        match item_type.as_str() {
            "file" => {
                // REINDEX: Remove from search index before deleting
                let _ = state.search_index.delete_from_index(&item_id).await;
                
                sqlx::query("DELETE FROM files WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                // Delete physical file
                if let Some(storage_path) = storage_path {
                    let full_path = Path::new(&storage_path);
                    let _ = fs::remove_file(full_path).await;
                }
            }
            "folder" => {
                sqlx::query("DELETE FROM folders WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                // Delete physical folder
                let full_path = Path::new(DATA_DIR).join(&original_path);
                let _ = fs::remove_dir_all(full_path).await;
            }
            _ => return Err(StatusCode::BAD_REQUEST),
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok((StatusCode::OK, "permanently deleted"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn cleanup_trash_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = &state.db_pool;
    
    // Get auto_trash_cleanup_days setting
    let cleanup_days: i64 = sqlx::query_scalar::<_, String>(
        "SELECT value FROM settings WHERE key = 'auto_trash_cleanup_days'"
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .and_then(|s| s.parse().ok())
    .unwrap_or(30);
    
    if cleanup_days == 0 {
        return Ok(Json(serde_json::json!({
            "deleted_count": 0,
            "message": "Auto-cleanup disabled"
        })));
    }
    
    // Find expired trash items
    let expired_items: Vec<(String, String, String)> = sqlx::query_as(
        r#"
        SELECT id, item_type, item_id 
        FROM trash 
        WHERE auto_delete_at IS NOT NULL 
        AND datetime(auto_delete_at) <= datetime('now')
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut deleted_count = 0;
    
    for (trash_id, item_type, item_id) in expired_items {
        // Delete from database
        match item_type.as_str() {
            "file" => {
                // REINDEX: Remove from search index before deleting
                let _ = state.search_index.delete_from_index(&item_id).await;
                
                // Get storage path
                let storage_path: Option<String> = sqlx::query_scalar(
                    "SELECT storage_path FROM files WHERE id = ?"
                )
                .bind(&item_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
                
                sqlx::query("DELETE FROM files WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
                
                if let Some(storage_path) = storage_path {
                    let _ = fs::remove_file(Path::new(&storage_path)).await;
                }
            }
            "folder" => {
                sqlx::query("DELETE FROM folders WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
            }
            _ => continue,
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .ok();
        
        deleted_count += 1;
    }
    
    Ok(Json(serde_json::json!({
        "deleted_count": deleted_count,
        "message": format!("Cleaned up {} expired items", deleted_count)
    })))
}

pub async fn empty_trash_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = &state.db_pool;
    
    // Get all trash items for user
    let trash_items: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT id, item_type, item_id FROM trash WHERE deleted_by = ?"
    )
    .bind(&user.id)
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut deleted_count = 0;
    
    for (trash_id, item_type, item_id) in trash_items {
        // Delete from database
        match item_type.as_str() {
            "file" => {
                // REINDEX: Remove from search index before deleting
                let _ = state.search_index.delete_from_index(&item_id).await;
                
                let storage_path: Option<String> = sqlx::query_scalar(
                    "SELECT storage_path FROM files WHERE id = ?"
                )
                .bind(&item_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
                
                sqlx::query("DELETE FROM files WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
                
                if let Some(storage_path) = storage_path {
                    let _ = fs::remove_file(Path::new(&storage_path)).await;
                }
            }
            "folder" => {
                sqlx::query("DELETE FROM folders WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
            }
            _ => continue,
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .ok();
        
        deleted_count += 1;
    }
    
    Ok(Json(serde_json::json!({
        "deleted_count": deleted_count,
        "message": "Trash emptied"
    })))
}
