//! Duplicate files detection API endpoints

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::auth::User;
use crate::handlers::duplicates::{
    find_duplicates as find_duplicates_handler,
    resolve_duplicates as resolve_duplicates_handler,
    duplicate_stats as duplicate_stats_handler,
    FindDuplicatesQuery, ResolveDuplicatesRequest, DuplicateGroup,
};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/duplicates", get(find_duplicates))
        .route("/duplicates/stats", get(stats))
        .route("/duplicates/resolve", post(resolve_duplicates))
}

/// Find all duplicate files
/// GET /api/duplicates?min_size_bytes=1048576
async fn find_duplicates(
    user: User,
    State(state): State<AppState>,
    Query(query): Query<FindDuplicatesQuery>,
) -> Result<Json<Vec<DuplicateGroup>>, StatusCode> {
    find_duplicates_handler(user, State(state), Query(query)).await
}

/// Get duplicate statistics
/// GET /api/duplicates/stats
async fn stats(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    duplicate_stats_handler(user, State(state)).await
}

/// Resolve duplicates by keeping one and deleting others
/// POST /api/duplicates/resolve
async fn resolve_duplicates(
    user: User,
    State(state): State<AppState>,
    Json(req): Json<ResolveDuplicatesRequest>,
) -> Result<StatusCode, StatusCode> {
    resolve_duplicates_handler(user, State(state), Json(req)).await
}
