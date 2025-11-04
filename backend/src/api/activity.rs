//! Activity log and audit trail API

use crate::auth::UserInfo;

use axum::{extract::{Query, State}, http::StatusCode, routing::{get, put}, Json, Router};
use serde::Deserialize;
use crate::{services, AppState};

#[derive(Debug, Deserialize)]
pub struct ActivityQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
    pub action: Option<String>,
}

fn default_limit() -> usize { 100 }

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/activity", get(list_activity))
        .route("/activity/stats", get(get_stats))
        .route("/activity/mark-visited", put(mark_visited))
}

async fn list_activity(State(state): State<AppState>, user: UserInfo, Query(query): Query<ActivityQuery>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let activities = services::activity::list(&state, &user, query.limit).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(activities.into_iter().map(|a| serde_json::to_value(a).unwrap_or_default()).collect()))
}

async fn get_stats(State(state): State<AppState>, user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    services::activity::get_stats(&state, &user).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn mark_visited(State(state): State<AppState>, user: UserInfo) -> Result<StatusCode, StatusCode> {
    services::activity::mark_visited(&state, &user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}
