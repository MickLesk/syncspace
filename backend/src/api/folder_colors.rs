//! Folder Colors API Routes
//! Allows users to customize folder appearance with colors

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::AppState;
use crate::auth::UserInfo;

#[derive(Debug, Deserialize)]
pub struct SetFolderColorRequest {
    pub color: String, // Hex color like #FF5733
}

#[derive(Debug, Serialize)]
pub struct FolderColorResponse {
    pub file_path: String,
    pub color: Option<String>,
}

/// Set folder color
async fn set_folder_color(
    State(state): State<AppState>,
    Path(file_path): Path<String>,
    user_info: UserInfo,
    Json(req): Json<SetFolderColorRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate color format
    if !req.color.starts_with('#') || (req.color.len() != 7 && req.color.len() != 9) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Update folder color in database
    let result = sqlx::query(
        r#"
        UPDATE files
        SET folder_color = ?
        WHERE path = ? AND is_dir = 1 AND owner_id = ?
        "#,
    )
    .bind(&req.color)
    .bind(&file_path)
    .bind(&user_info.id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to set folder color: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(serde_json::json!({
        "message": "Folder color updated",
        "file_path": file_path,
        "color": req.color
    })))
}

/// Get folder color
async fn get_folder_color(
    State(state): State<AppState>,
    Path(file_path): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let row = sqlx::query(
        r#"
        SELECT path, folder_color
        FROM files
        WHERE path = ? AND is_dir = 1
        "#,
    )
    .bind(&file_path)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get folder color: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(row) = row {
        let color: Option<String> = row.get("folder_color");
        Ok(Json(FolderColorResponse {
            file_path: row.get("path"),
            color,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Remove folder color
async fn remove_folder_color(
    State(state): State<AppState>,
    Path(file_path): Path<String>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        r#"
        UPDATE files
        SET folder_color = NULL
        WHERE path = ? AND is_dir = 1 AND owner_id = ?
        "#,
    )
    .bind(&file_path)
    .bind(&user_info.id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to remove folder color: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Build folder colors router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/folders/{file_path}/color", 
            get(get_folder_color)
            .put(set_folder_color)
        )
        .route("/folders/{file_path}/color/remove", 
            put(remove_folder_color)
        )
}
