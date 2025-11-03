//! Duplicate files detection API endpoints
//! TODO: Reimplement using services layer instead of handlers

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::auth::User;
use crate::AppState;

// TODO: Move these to models or create proper service
#[derive(serde::Deserialize)]
pub struct FindDuplicatesQuery {
    pub min_size_bytes: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct ResolveDuplicatesRequest {
    pub keep_file_id: String,
    pub delete_file_ids: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<String>,
    pub total_size: i64,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/duplicates", get(find_duplicates))
        .route("/duplicates/stats", get(stats))
        .route("/duplicates/resolve", post(resolve_duplicates))
}

/// Find all duplicate files
/// GET /api/duplicates?min_size_bytes=1048576
async fn find_duplicates(
    _user: User,
    State(_state): State<AppState>,
    Query(_query): Query<FindDuplicatesQuery>,
) -> Result<Json<Vec<DuplicateGroup>>, StatusCode> {
    // TODO: Implement duplicate detection using services layer
    Ok(Json(vec![]))
}

/// Get duplicate statistics
/// GET /api/duplicates/stats
async fn stats(
    _user: User,
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Implement stats using services layer
    Ok(Json(serde_json::json!({
        "total_duplicates": 0,
        "wasted_space": 0
    })))
}

/// Resolve duplicates by keeping one and deleting others
/// POST /api/duplicates/resolve
async fn resolve_duplicates(
    _user: User,
    State(_state): State<AppState>,
    Json(_req): Json<ResolveDuplicatesRequest>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement resolve using services layer
    Ok(StatusCode::NOT_IMPLEMENTED)
}
