//! Duplicate file detection and management
//!
//! Finds duplicate files based on SHA-256 checksum.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::Utc;
use uuid::Uuid;

use crate::auth::User;
use crate::AppState;

// ==================== MODELS ====================

#[derive(Debug, Clone, Serialize)]
pub struct DuplicateGroup {
    pub checksum: String,
    pub total_size_bytes: i64,
    pub file_count: i64,
    pub potential_savings_bytes: i64, // (file_count - 1) * size
    pub files: Vec<DuplicateFile>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DuplicateFile {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size_bytes: i64,
    pub created_at: String,
    pub owner_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FindDuplicatesQuery {
    pub min_size_bytes: Option<i64>, // Only find duplicates >= this size
}

#[derive(Debug, Deserialize)]
pub struct ResolveDuplicatesRequest {
    pub checksum: String,
    pub keep_file_id: String, // ID of file to keep
    pub delete_file_ids: Vec<String>, // IDs to delete
}

// ==================== HANDLERS ====================

/// Find all duplicate files for current user
/// GET /api/duplicates?min_size_bytes=1048576
pub async fn find_duplicates(
    user: User,
    State(state): State<AppState>,
    Query(query): Query<FindDuplicatesQuery>,
) -> Result<Json<Vec<DuplicateGroup>>, StatusCode> {
    let min_size = query.min_size_bytes.unwrap_or(0);

    // Find checksums with duplicates
    let checksums: Vec<(String, i64, i64)> = sqlx::query_as(
        r#"
        SELECT 
            checksum_sha256,
            COUNT(*) as file_count,
            MAX(size_bytes) as size_bytes
        FROM files
        WHERE owner_id = ? 
          AND is_deleted = 0 
          AND checksum_sha256 IS NOT NULL
          AND size_bytes >= ?
        GROUP BY checksum_sha256
        HAVING COUNT(*) > 1
        ORDER BY size_bytes DESC
        "#
    )
    .bind(&user.id.to_string())
    .bind(min_size)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut groups = Vec::new();

    for (checksum, count, size) in checksums {
        // Get all files with this checksum
        let files: Vec<DuplicateFile> = sqlx::query_as(
            r#"
            SELECT id, name, path, size_bytes, created_at, owner_id
            FROM files
            WHERE checksum_sha256 = ? AND owner_id = ? AND is_deleted = 0
            ORDER BY created_at ASC
            "#
        )
        .bind(&checksum)
        .bind(&user.id.to_string())
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let potential_savings = (count - 1) * size;

        groups.push(DuplicateGroup {
            checksum: checksum.clone(),
            total_size_bytes: size,
            file_count: count,
            potential_savings_bytes: potential_savings,
            files,
        });
    }

    Ok(Json(groups))
}

/// Resolve duplicates by keeping one and deleting others
/// POST /api/duplicates/resolve
pub async fn resolve_duplicates(
    user: User,
    State(state): State<AppState>,
    Json(req): Json<ResolveDuplicatesRequest>,
) -> Result<StatusCode, StatusCode> {
    // Verify all files belong to user and have the same checksum
    for file_id in std::iter::once(&req.keep_file_id).chain(req.delete_file_ids.iter()) {
        let file: Option<(String, String)> = sqlx::query_as(
            "SELECT owner_id, checksum_sha256 FROM files WHERE id = ? AND is_deleted = 0"
        )
        .bind(file_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match file {
            None => return Err(StatusCode::NOT_FOUND),
            Some((owner_id, checksum)) => {
                if owner_id != user.id.to_string() {
                    return Err(StatusCode::FORBIDDEN);
                }
                if checksum != req.checksum {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }

    let now = Utc::now().to_rfc3339();

    // Soft delete the duplicates
    for file_id in &req.delete_file_ids {
        sqlx::query(
            r#"
            UPDATE files 
            SET is_deleted = 1, deleted_at = ?, deleted_by = ?
            WHERE id = ?
            "#
        )
        .bind(&now)
        .bind(&user.id.to_string())
        .bind(file_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO activity_log (id, user_id, action, resource_type, resource_id, timestamp, metadata)
        VALUES (?, ?, 'duplicates_resolved', 'file', ?, ?, ?)
        "#
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.id.to_string())
    .bind(&req.keep_file_id)
    .bind(&now)
    .bind(format!("{{\"deleted_count\": {}, \"checksum\": \"{}\"}}", req.delete_file_ids.len(), req.checksum))
    .execute(&state.db_pool)
    .await;

    // Recalculate storage usage
    crate::handlers::storage::recalculate_storage_usage(&state.db_pool, &user.id.to_string())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get duplicate statistics
/// GET /api/duplicates/stats
pub async fn duplicate_stats(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Total duplicates
    let total_groups: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT checksum_sha256)
        FROM files
        WHERE owner_id = ? 
          AND is_deleted = 0 
          AND checksum_sha256 IS NOT NULL
          AND checksum_sha256 IN (
              SELECT checksum_sha256 
              FROM files 
              WHERE owner_id = ? AND is_deleted = 0
              GROUP BY checksum_sha256 
              HAVING COUNT(*) > 1
          )
        "#
    )
    .bind(&user.id.to_string())
    .bind(&user.id.to_string())
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Total duplicate files
    let total_files: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM files
        WHERE owner_id = ? 
          AND is_deleted = 0 
          AND checksum_sha256 IN (
              SELECT checksum_sha256 
              FROM files 
              WHERE owner_id = ? AND is_deleted = 0
              GROUP BY checksum_sha256 
              HAVING COUNT(*) > 1
          )
        "#
    )
    .bind(&user.id.to_string())
    .bind(&user.id.to_string())
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Potential savings
    let potential_savings: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM((cnt - 1) * size), 0)
        FROM (
            SELECT checksum_sha256, COUNT(*) as cnt, MAX(size_bytes) as size
            FROM files
            WHERE owner_id = ? AND is_deleted = 0 AND checksum_sha256 IS NOT NULL
            GROUP BY checksum_sha256
            HAVING COUNT(*) > 1
        )
        "#
    )
    .bind(&user.id.to_string())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "total_duplicate_groups": total_groups.0,
        "total_duplicate_files": total_files.0,
        "potential_savings_bytes": potential_savings.map(|s| s.0).unwrap_or(0),
    })))
}
