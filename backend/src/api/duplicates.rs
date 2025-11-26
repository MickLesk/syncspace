//! Duplicate files detection API endpoints
//! Implements duplicate detection using services layer for hash-based file comparison

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::auth::User;
use crate::AppState;

// Models for duplicate detection requests and responses
#[derive(serde::Deserialize)]
pub struct FindDuplicatesQuery {
    #[allow(dead_code)]
    pub min_size_bytes: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct ResolveDuplicatesRequest {
    #[allow(dead_code)]
    pub keep_file_id: String,
    #[allow(dead_code)]
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
    user: User,
    State(state): State<AppState>,
    Query(query): Query<FindDuplicatesQuery>,
) -> Result<Json<Vec<DuplicateGroup>>, StatusCode> {
    let min_size = query.min_size_bytes.unwrap_or(0);

    tracing::debug!(
        "Finding duplicates for user {} with min size {}",
        user.id,
        min_size
    );

    // Query file hashes that have duplicates (checksum_sha256 appears more than once)
    let duplicate_hashes: Vec<String> = sqlx::query_scalar(
        "SELECT checksum_sha256 FROM files 
         WHERE owner_id = ? AND size_bytes >= ? AND checksum_sha256 IS NOT NULL AND is_deleted = 0
         GROUP BY checksum_sha256 
         HAVING COUNT(*) > 1",
    )
    .bind(&user.id)
    .bind(min_size)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to query duplicate hashes: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // For each hash, get all files and calculate total size
    let mut groups = Vec::new();

    for hash in duplicate_hashes {
        // Get all files with this hash
        let files: Vec<(String, i64)> = sqlx::query_as(
            "SELECT path, size_bytes FROM files 
             WHERE checksum_sha256 = ? AND owner_id = ? AND is_deleted = 0",
        )
        .bind(&hash)
        .bind(&user.id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let total_size = files.iter().map(|(_, size)| size).sum();
        let file_paths = files.into_iter().map(|(path, _)| path).collect();

        groups.push(DuplicateGroup {
            hash: hash.clone(),
            files: file_paths,
            total_size,
        });
    }

    Ok(Json(groups))
}

/// Get duplicate statistics
/// GET /api/duplicates/stats
async fn stats(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::debug!("Getting duplicate statistics for user {}", user.id);

    // Count total duplicate files
    let total_duplicates: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM files 
         WHERE owner_id = ? AND is_deleted = 0 AND checksum_sha256 IN (
             SELECT checksum_sha256 FROM files 
             WHERE owner_id = ? AND is_deleted = 0 AND checksum_sha256 IS NOT NULL
             GROUP BY checksum_sha256 HAVING COUNT(*) > 1
         )",
    )
    .bind(&user.id)
    .bind(&user.id)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(0);

    // Count duplicate groups
    let num_groups: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT checksum_sha256) FROM files 
         WHERE owner_id = ? AND is_deleted = 0 AND checksum_sha256 IN (
             SELECT checksum_sha256 FROM files 
             WHERE owner_id = ? AND is_deleted = 0 AND checksum_sha256 IS NOT NULL
             GROUP BY checksum_sha256 HAVING COUNT(*) > 1
         )",
    )
    .bind(&user.id)
    .bind(&user.id)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(0);

    // Calculate wasted space (sum of size_bytes for duplicate files, minus one copy per group)
    let wasted_space: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(size_bytes * (cnt - 1)), 0) FROM (
             SELECT checksum_sha256, size_bytes, COUNT(*) as cnt 
             FROM files 
             WHERE owner_id = ? AND is_deleted = 0 AND checksum_sha256 IS NOT NULL
             GROUP BY checksum_sha256 
             HAVING COUNT(*) > 1
         )",
    )
    .bind(&user.id)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "total_duplicates": total_duplicates,
        "wasted_space": wasted_space,
        "num_groups": num_groups,
        "status": "active"
    })))
}

/// Resolve duplicates by keeping one and deleting others
/// POST /api/duplicates/resolve
async fn resolve_duplicates(
    user: User,
    State(state): State<AppState>,
    Json(req): Json<ResolveDuplicatesRequest>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!(
        "Resolving duplicates for user {}: keeping {}, deleting {} files",
        user.id,
        req.keep_file_id,
        req.delete_file_ids.len()
    );

    // Soft delete all files to be removed (set is_deleted = 1)
    let now = chrono::Utc::now().to_rfc3339();

    for file_id in &req.delete_file_ids {
        // Verify file belongs to user before deleting
        let result = sqlx::query(
            "UPDATE files SET is_deleted = 1, deleted_at = ?, deleted_by = ? 
             WHERE id = ? AND owner_id = ?",
        )
        .bind(&now)
        .bind(&user.id)
        .bind(file_id)
        .bind(&user.id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete duplicate file {}: {}", file_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if result.rows_affected() == 0 {
            tracing::warn!(
                "File {} not found or not owned by user {}",
                file_id,
                user.id
            );
            return Err(StatusCode::FORBIDDEN);
        }
    }

    tracing::info!(
        "Successfully deleted {} duplicate files for user {}",
        req.delete_file_ids.len(),
        user.id
    );
    Ok(StatusCode::OK)
}
