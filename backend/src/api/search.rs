//! Search API endpoints

use crate::auth::UserInfo;

use axum::{extract::{Query, State}, http::StatusCode, routing::get, Json, Router};
use serde::Deserialize;
use crate::{services, AppState};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default = "default_fuzzy")]
    pub fuzzy: bool,
}

fn default_limit() -> usize { 50 }
fn default_fuzzy() -> bool { true }

pub fn router() -> Router<AppState> {
    Router::new().route("/search", get(search_handler))
}

async fn search_handler(State(state): State<AppState>, user: UserInfo, Query(query): Query<SearchQuery>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // SECURITY: Validate search query to prevent SQL injection
    let safe_query = crate::security::validate_search_query(&query.q)?;
    
    services::search(&state, &user, &safe_query, query.limit, query.fuzzy)
        .await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
