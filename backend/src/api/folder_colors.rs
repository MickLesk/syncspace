//! Folder Colors API Routes
//! Allows users to customize folder appearance with colors

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SetFolderColorRequest {
    pub file_path: String,
    pub color: String, // Hex color like #FF5733
}

#[derive(Debug, Deserialize)]
pub struct GetFolderColorRequest {
    pub file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct RemoveFolderColorRequest {
    pub file_path: String,
}

#[derive(Debug, Serialize)]
pub struct FolderColorResponse {
    pub file_path: String,
    pub color: Option<String>,
}

/// Set folder color
async fn set_folder_color(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<SetFolderColorRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate color format
    if !req.color.starts_with('#') || (req.color.len() != 7 && req.color.len() != 9) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let file_path = req.file_path.trim_start_matches('/').trim_end_matches('/');

    // Update folder color in database
    let result = sqlx::query(
        r#"
        UPDATE folders
        SET color = ?
        WHERE path = ? AND owner_id = ? AND is_deleted = 0
        "#,
    )
    .bind(&req.color)
    .bind(file_path)
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

    // Log activity
    let folder_name = file_path.split('/').last().unwrap_or(file_path);
    let _ = crate::services::activity::log(
        &state,
        &user_info.id,
        crate::services::activity::actions::FOLDER_COLOR,
        file_path,
        folder_name,
        None,
        None,
        "success",
        None,
        Some(serde_json::json!({ "color": &req.color })),
    )
    .await;

    Ok(Json(serde_json::json!({
        "message": "Folder color updated",
        "file_path": file_path,
        "color": req.color
    })))
}

/// Get folder color
async fn get_folder_color(
    State(state): State<AppState>,
    Json(req): Json<GetFolderColorRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let file_path = req.file_path.trim_start_matches('/').trim_end_matches('/');

    let row = sqlx::query(
        r#"
        SELECT path, color
        FROM folders
        WHERE path = ? AND is_deleted = 0
        "#,
    )
    .bind(file_path)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get folder color: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(row) = row {
        let color: Option<String> = row.get("color");
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
    user_info: UserInfo,
    Json(req): Json<RemoveFolderColorRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let file_path = req.file_path.trim_start_matches('/').trim_end_matches('/');

    let result = sqlx::query(
        r#"
        UPDATE folders
        SET color = NULL
        WHERE path = ? AND owner_id = ? AND is_deleted = 0
        "#,
    )
    .bind(file_path)
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
        .route("/folders/color", put(set_folder_color))
        .route("/folders/color/get", post(get_folder_color))
        .route("/folders/color/remove", post(remove_folder_color))
}
