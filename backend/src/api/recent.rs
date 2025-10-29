//! Recent Files API endpoints

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

use crate::{auth::UserInfo, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/recent", get(list_recent_files))
        .route("/recent/accessed", get(list_recently_accessed))
        .route("/recent/uploaded", get(list_recently_uploaded))
}

#[derive(Debug, Serialize, Deserialize)]
struct RecentFilesQuery {
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 {
    50
}

#[derive(Debug, Serialize, FromRow)]
struct RecentFile {
    file_path: String,
    name: String,
    size_bytes: i64,
    accessed_at: DateTime<Utc>,
    action: String, // "view", "upload", "download", "edit"
}

async fn list_recent_files(
    State(state): State<AppState>,
    user: UserInfo,
    Query(params): Query<RecentFilesQuery>,
) -> Result<Json<Vec<RecentFile>>, StatusCode> {
    let recent = sqlx::query_as::<_, RecentFile>(
        r#"
        SELECT 
            path as file_path,
            name,
            size_bytes,
            created_at as accessed_at,
            'upload' as action
        FROM files
        WHERE owner_id = ? AND is_deleted = 0
        ORDER BY created_at DESC
        LIMIT ?
        "#,
    )
    .bind(&user.id)
    .bind(params.limit)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch recent files: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(recent))
}

async fn list_recently_accessed(
    State(state): State<AppState>,
    user: UserInfo,
    Query(params): Query<RecentFilesQuery>,
) -> Result<Json<Vec<RecentFile>>, StatusCode> {
    // For now, return uploaded files as recently accessed
    // In a full implementation, we'd track file access in an activity log
    list_recent_files(State(state), user, Query(params)).await
}

async fn list_recently_uploaded(
    State(state): State<AppState>,
    user: UserInfo,
    Query(params): Query<RecentFilesQuery>,
) -> Result<Json<Vec<RecentFile>>, StatusCode> {
    list_recent_files(State(state), user, Query(params)).await
}
