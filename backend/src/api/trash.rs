#![allow(dead_code)]

//! Trash/Recycle Bin API Routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TrashItem {
    pub id: Uuid,
    pub original_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub deleted_at: DateTime<Utc>,
    pub deleted_by: String,
}

#[derive(Debug, Deserialize)]
pub struct RestoreRequest {
    pub destination_path: Option<String>,
}

/// List trash items
async fn list_trash(
    State(state): State<AppState>,
    _user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Query files marked as deleted from the files table
    #[derive(sqlx::FromRow)]
    struct TrashRow {
        id: String,
        original_path: String,
        file_name: String,
        file_size: i64,
        deleted_at: String,
        deleted_by: String,
    }

    let items: Vec<TrashRow> = sqlx::query_as(
        "SELECT id, path as original_path, name as file_name, size_bytes as file_size, updated_at as deleted_at, owner_id as deleted_by
         FROM files
         WHERE is_deleted = 1
         ORDER BY updated_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let trash_items: Vec<TrashItem> = items
        .iter()
        .map(|row| TrashItem {
            id: uuid::Uuid::parse_str(&row.id).unwrap_or_default(),
            original_path: row.original_path.clone(),
            file_name: row.file_name.clone(),
            file_size: row.file_size,
            deleted_at: chrono::DateTime::parse_from_rfc3339(&row.deleted_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            deleted_by: row.deleted_by.clone(),
        })
        .collect();

    Ok(Json(trash_items))
}

/// Restore item from trash
async fn restore_from_trash(
    State(state): State<AppState>,
    Path(path): Path<String>,
    user_info: UserInfo,
    Json(req): Json<RestoreRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Restore file by setting is_deleted = 0
    let now = chrono::Utc::now().to_rfc3339();

    // Use destination_path if provided, otherwise restore to original location
    let restore_path = req.destination_path.unwrap_or_else(|| path.clone());

    let result = sqlx::query(
        "UPDATE files SET is_deleted = 0, path = ?, updated_at = ? WHERE path = ? AND is_deleted = 1",
    )
    .bind(&restore_path)
    .bind(&now)
    .bind(&path)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Log the restoration
    let log_id = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, user_id, action, file_path, status, created_at) VALUES (?, ?, 'restored', ?, 'success', datetime('now'))")
        .bind(&log_id).bind(&user_info.id).bind(&path).execute(&state.db).await;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Item restored successfully"
        })),
    ))
}

/// Permanently delete item
async fn permanent_delete(
    State(state): State<AppState>,
    Path(path): Path<String>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM trash_items
        WHERE original_path = ? AND deleted_by = ?
        "#,
    )
    .bind(&path)
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // TODO: Delete actual file from filesystem

    Ok(StatusCode::NO_CONTENT)
}

/// Cleanup old trash items (older than 30 days)
async fn cleanup_trash(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
    let thirty_days_ago_str = thirty_days_ago.to_rfc3339();

    let result = sqlx::query(
        r#"
        DELETE FROM trash_items
        WHERE deleted_by = ? AND deleted_at < ?
        "#,
    )
    .bind(&user_info.id)
    .bind(&thirty_days_ago_str)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "deleted_count": result.rows_affected()
    })))
}

/// Empty entire trash
async fn empty_trash(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM trash_items
        WHERE deleted_by = ?
        "#,
    )
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "deleted_count": result.rows_affected()
    })))
}

/// Build trash router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/trash", get(list_trash))
        .route("/trash/restore/{*path}", post(restore_from_trash))
        .route("/trash/permanent/{*path}", delete(permanent_delete))
        .route("/trash/cleanup", delete(cleanup_trash))
        .route("/trash/empty", delete(empty_trash))
}
