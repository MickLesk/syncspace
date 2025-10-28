//! Trash/Recycle Bin API Routes

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::AppState;
use crate::auth::UserInfo;

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
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let items = sqlx::query_as::<_, TrashItem>(
        r#"
        SELECT id, original_path, file_name, file_size, deleted_at, deleted_by
        FROM trash_items
        WHERE deleted_by = ?
        ORDER BY deleted_at DESC
        "#,
    )
    .bind(&user_info.id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(items))
}

/// Restore item from trash
async fn restore_from_trash(
    State(state): State<AppState>,
    Path(path): Path<String>,
    user_info: UserInfo,
    Json(req): Json<RestoreRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement actual file restoration logic
    // For now, just remove from trash_items table
    
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
    
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Item restored successfully"
    }))))
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
