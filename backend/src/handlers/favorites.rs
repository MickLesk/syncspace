//! Favorites handlers for managing user bookmarks

use crate::auth;
use crate::AppState;
use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteRequest {
    pub item_type: String,  // 'file' or 'folder'
    pub item_id: String,    // file/folder ID or path
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteResponse {
    pub id: String,
    pub user_id: String,
    pub item_type: String,
    pub item_id: String,
    pub created_at: String,
}

pub async fn list_favorites_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<FavoriteResponse>>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    let favorites = sqlx::query_as::<_, (String, String, String, String, String)>(
        "SELECT id, user_id, item_type, item_id, created_at FROM favorites WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(&user_id)
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let result = favorites.into_iter().map(|(id, user_id, item_type, item_id, created_at)| {
        FavoriteResponse { id, user_id, item_type, item_id, created_at }
    }).collect();
    
    Ok(Json(result))
}

pub async fn toggle_favorite_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<FavoriteRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    // Check if already favorited
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT id FROM favorites WHERE user_id = ? AND item_type = ? AND item_id = ?"
    )
    .bind(&user_id)
    .bind(&req.item_type)
    .bind(&req.item_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(fav_id) = existing {
        // Remove favorite
        sqlx::query("DELETE FROM favorites WHERE id = ?")
            .bind(&fav_id)
            .execute(pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(serde_json::json!({
            "status": "removed",
            "item_type": req.item_type,
            "item_id": req.item_id
        })))
    } else {
        // Add favorite
        let fav_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        sqlx::query(
            "INSERT INTO favorites (id, user_id, item_type, item_id, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&fav_id)
        .bind(&user_id)
        .bind(&req.item_type)
        .bind(&req.item_id)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(serde_json::json!({
            "status": "added",
            "item_type": req.item_type,
            "item_id": req.item_id
        })))
    }
}

pub async fn delete_favorite_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    // Verify ownership and delete
    sqlx::query("DELETE FROM favorites WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::OK, ""))
}
