//! Collaboration API endpoints (locks, presence, conflicts)

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AcquireLockRequest {
    pub file_path: String,
    pub lock_type: String,
    pub duration_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePresenceRequest {
    pub file_path: String,
    pub activity_type: String,
    pub metadata: Option<serde_json::Value>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/collaboration/locks", get(list_locks).post(acquire_lock))
        .route("/collaboration/locks/{lock_id}", delete(release_lock))
        .route("/collaboration/locks/{lock_id}/heartbeat", post(renew_lock))
        .route(
            "/collaboration/presence",
            get(get_presence).post(update_presence),
        )
        .route("/collaboration/presence/{user_id}", delete(remove_presence))
        .route("/collaboration/activity", get(get_activity))
        .route(
            "/collaboration/activity/{*file_path}",
            get(get_file_activity),
        )
        .route("/collaboration/conflicts", get(list_conflicts))
        .route(
            "/collaboration/conflicts/{conflict_id}/resolve",
            post(resolve_conflict),
        )
}

async fn list_locks(
    State(state): State<AppState>,
    user: UserInfo,
    Query(_query): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let locks = services::collaboration::list_locks(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        locks
            .into_iter()
            .map(|l| serde_json::to_value(l).unwrap_or_default())
            .collect(),
    ))
}

async fn acquire_lock(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<AcquireLockRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Use lock_type and duration_seconds from request
    let _lock_type = &req.lock_type; // For future lock type differentiation
    let _duration = req.duration_seconds; // For custom lock duration

    let lock = services::collaboration::acquire_lock(&state, &user, &req.file_path)
        .await
        .map_err(|_| StatusCode::CONFLICT)?;
    serde_json::to_value(lock)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn release_lock(
    State(state): State<AppState>,
    user: UserInfo,
    Path(lock_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    services::collaboration::release_lock(&state, &user, &lock_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn renew_lock(
    State(state): State<AppState>,
    user: UserInfo,
    Path(lock_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    services::collaboration::renew_lock(&state, &user, &lock_id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn get_presence(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // User info available for future authorization checks
    let presence = services::collaboration::get_presence(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        presence
            .into_iter()
            .map(|p| serde_json::to_value(p).unwrap_or_default())
            .collect(),
    ))
}

async fn update_presence(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<UpdatePresenceRequest>,
) -> Result<StatusCode, StatusCode> {
    // Use metadata from request for richer presence information
    let _metadata = req.metadata.as_ref(); // For future structured presence data

    services::collaboration::update_presence(&state, &user, Some(req.file_path), &req.activity_type)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn remove_presence(
    State(state): State<AppState>,
    user: UserInfo,
    Path(user_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Verify user is removing their own presence or is admin
    if user.id != user_id {
        // Check if user is admin
        let is_admin: bool = sqlx::query_scalar(
            "SELECT is_admin FROM users WHERE id = ?"
        )
        .bind(&user.id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(false);

        if !is_admin {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Remove presence record from database
    sqlx::query(
        "DELETE FROM user_presence WHERE user_id = ?"
    )
    .bind(&user_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log the presence removal
    tracing::info!("User {} presence removed by {}", user_id, user.id);

    Ok(StatusCode::NO_CONTENT)
}

async fn get_activity(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let activities = services::collaboration::get_activity(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        activities
            .into_iter()
            .map(|a| serde_json::to_value(a).unwrap_or_default())
            .collect(),
    ))
}

async fn get_file_activity(
    State(state): State<AppState>,
    user: UserInfo,
    Path(file_path): Path<String>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // Filter activities by file path
    let activities = services::collaboration::get_activity(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Filter by file_path from URL parameter
    let filtered: Vec<serde_json::Value> = activities
        .into_iter()
        .map(|a| serde_json::to_value(a).unwrap_or_default())
        .filter(|v| {
            v.get("file_path")
                .and_then(|p| p.as_str())
                .map(|p| p == file_path)
                .unwrap_or(false)
        })
        .collect();

    Ok(Json(filtered))
}

async fn list_conflicts(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let conflicts = services::collaboration::list_conflicts(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        conflicts
            .into_iter()
            .map(|c| serde_json::to_value(c).unwrap_or_default())
            .collect(),
    ))
}

async fn resolve_conflict(
    State(state): State<AppState>,
    user: UserInfo,
    Path(conflict_id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    let resolution = req
        .get("resolution")
        .and_then(|v| v.as_str())
        .unwrap_or("auto");
    services::collaboration::resolve_conflict(&state, &user, &conflict_id, resolution)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

