//! Favorites API endpoints

use crate::auth::UserInfo;

use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Json, Router};
use serde::Deserialize;
use crate::{services, AppState};

#[derive(Debug, Deserialize)]
pub struct AddFavoriteRequest {
    pub item_id: String,
    pub item_type: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/favorites/list", get(list_favorites))
        .route("/favorites/add", post(add_favorite))
        .route("/favorites/:favorite_id/remove", delete(remove_favorite))
}

async fn list_favorites(State(state): State<AppState>, user: UserInfo) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let favorites = services::favorites::list(&state, &user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(favorites.into_iter().map(|f| serde_json::to_value(f).unwrap_or_default()).collect()))
}

async fn add_favorite(State(state): State<AppState>, user: UserInfo, Json(req): Json<AddFavoriteRequest>) -> Result<StatusCode, StatusCode> {
    services::favorites::add(&state, &user, &req.item_type, &req.item_id).await.map(|_| StatusCode::CREATED).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn remove_favorite(State(state): State<AppState>, user: UserInfo, Path(favorite_id): Path<String>) -> Result<StatusCode, StatusCode> {
    services::favorites::remove(&state, &user, &favorite_id).await.map(|_| StatusCode::NO_CONTENT).map_err(|_| StatusCode::NOT_FOUND)
}
