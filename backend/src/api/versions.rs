//! File Versioning API Routes with Differential Storage, Compression, and Tags

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::PathBuf;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::services::version_storage_service;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileVersion {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub storage_path: Option<String>,
    pub original_size: i64,
    pub compressed_size: i64,
    pub is_compressed: bool,
    pub is_differential: bool,
    pub base_version_id: Option<String>,
    pub checksum: String,
    pub created_by: String,
    pub created_at: String,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileVersionEnhanced {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub storage_path: Option<String>,
    pub original_size: i64,
    pub compressed_size: i64,
    pub is_compressed: bool,
    pub is_differential: bool,
    pub base_version_id: Option<String>,
    pub checksum: String,
    pub created_by: String,
    pub created_at: String,
    pub comment: Option<String>,
    pub is_pinned: bool,
    pub is_starred: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VersionTag {
    pub id: String,
    pub version_id: String,
    pub tag_name: String,
    pub tag_color: String,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VersionTagTemplate {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub is_system: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionWithTags {
    #[serde(flatten)]
    pub version: FileVersionEnhanced,
    pub tags: Vec<VersionTag>,
    pub created_by_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionStorageStats {
    pub file_id: String,
    pub filename: String,
    pub file_path: String,
    pub version_count: i64,
    pub total_original_size: i64,
    pub total_compressed_size: i64,
    pub storage_saved: i64,
    pub compression_ratio: f64,
    pub last_version_at: Option<String>,
    pub first_version_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionTimeline {
    pub versions: Vec<VersionWithTags>,
    pub stats: VersionStorageStats,
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub file_path: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub tag_name: String,
    pub tag_color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVersionRequest {
    pub comment: Option<String>,
    pub is_pinned: Option<bool>,
    pub is_starred: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct VersionListQuery {
    pub include_tags: Option<bool>,
}

/// List all versions of a file with optional tags
async fn list_versions(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    Query(query): Query<VersionListQuery>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let include_tags = query.include_tags.unwrap_or(true);

    let versions = sqlx::query_as::<_, FileVersionEnhanced>(
        r#"
        SELECT id, file_id, version_number, storage_path, original_size, compressed_size,
               is_compressed, is_differential, base_version_id, checksum, 
               created_by, created_at, comment,
               COALESCE(is_pinned, 0) as is_pinned,
               COALESCE(is_starred, 0) as is_starred
        FROM file_versions
        WHERE file_id = ?
        ORDER BY version_number DESC
        "#,
    )
    .bind(&file_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if include_tags {
        let mut versions_with_tags = Vec::new();

        for version in versions {
            let tags = sqlx::query_as::<_, VersionTag>(
                "SELECT id, version_id, tag_name, tag_color, description, created_by, created_at 
                 FROM version_tags WHERE version_id = ?",
            )
            .bind(&version.id)
            .fetch_all(&state.db_pool)
            .await
            .unwrap_or_default();

            // Get creator name
            let created_by_name: Option<(String,)> =
                sqlx::query_as("SELECT display_name FROM users WHERE id = ?")
                    .bind(&version.created_by)
                    .fetch_optional(&state.db_pool)
                    .await
                    .ok()
                    .flatten();

            versions_with_tags.push(VersionWithTags {
                version,
                tags,
                created_by_name: created_by_name.map(|n| n.0),
            });
        }

        Ok(Json(versions_with_tags))
    } else {
        // Return simple versions without tags
        let simple_versions: Vec<VersionWithTags> = versions
            .into_iter()
            .map(|v| VersionWithTags {
                version: v,
                tags: vec![],
                created_by_name: None,
            })
            .collect();
        Ok(Json(simple_versions))
    }
}

/// Create a new version with differential storage and compression
async fn create_version(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    user: UserInfo,
    Json(req): Json<CreateVersionRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let file_path = PathBuf::from(&req.file_path);

    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    match version_storage_service::create_version(
        &state.db_pool,
        &file_id,
        &file_path,
        &user.id,
        req.comment.as_deref(),
    )
    .await
    {
        Ok(version_metadata) => {
            let version = FileVersion {
                id: version_metadata.id,
                file_id: version_metadata.file_id,
                version_number: version_metadata.version_number,
                storage_path: Some(version_metadata.storage_path),
                original_size: version_metadata.original_size,
                compressed_size: version_metadata.compressed_size,
                is_compressed: version_metadata.is_compressed,
                is_differential: version_metadata.is_differential,
                base_version_id: version_metadata.base_version_id,
                checksum: version_metadata.checksum,
                created_by: version_metadata.created_by,
                created_at: version_metadata.created_at,
                comment: version_metadata.comment,
            };

            Ok((StatusCode::CREATED, Json(version)))
        }
        Err(e) => {
            eprintln!("Error creating version: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a version
async fn delete_version(
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let version: Option<(String,)> =
        sqlx::query_as("SELECT storage_path FROM file_versions WHERE id = ? AND file_id = ?")
            .bind(&version_id)
            .bind(&file_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((storage_path,)) = version {
        let _ = tokio::fs::remove_file(&storage_path).await;

        let result = sqlx::query("DELETE FROM file_versions WHERE id = ?")
            .bind(&version_id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if result.rows_affected() > 0 {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Restore a version (returns file content)
async fn restore_version(
    State(state): State<AppState>,
    Path((_file_id, version_id)): Path<(String, String)>,
    _user: UserInfo,
) -> Result<Response, StatusCode> {
    match version_storage_service::restore_version(&state.db_pool, &version_id).await {
        Ok(content) => {
            let body = Bytes::from(content);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"restored_v{}.dat\"", version_id),
                )
                .body(body.into())
                .unwrap())
        }
        Err(e) => {
            eprintln!("Error restoring version: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get version timeline with stats
async fn get_version_timeline(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Get versions with tags
    let versions = sqlx::query_as::<_, FileVersionEnhanced>(
        r#"
        SELECT id, file_id, version_number, storage_path, original_size, compressed_size,
               is_compressed, is_differential, base_version_id, checksum, 
               created_by, created_at, comment,
               COALESCE(is_pinned, 0) as is_pinned,
               COALESCE(is_starred, 0) as is_starred
        FROM file_versions
        WHERE file_id = ?
        ORDER BY version_number DESC
        "#,
    )
    .bind(&file_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut versions_with_tags = Vec::new();
    let mut total_original_size: i64 = 0;
    let mut total_compressed_size: i64 = 0;

    for version in &versions {
        total_original_size += version.original_size;
        total_compressed_size += version.compressed_size;

        let tags = sqlx::query_as::<_, VersionTag>(
            "SELECT id, version_id, tag_name, tag_color, description, created_by, created_at 
             FROM version_tags WHERE version_id = ?",
        )
        .bind(&version.id)
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();

        let created_by_name: Option<(String,)> =
            sqlx::query_as("SELECT display_name FROM users WHERE id = ?")
                .bind(&version.created_by)
                .fetch_optional(&state.db_pool)
                .await
                .ok()
                .flatten();

        versions_with_tags.push(VersionWithTags {
            version: version.clone(),
            tags,
            created_by_name: created_by_name.map(|n| n.0),
        });
    }

    // Get file info
    let file_info: Option<(String, String)> =
        sqlx::query_as("SELECT filename, file_path FROM files WHERE id = ?")
            .bind(&file_id)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten();

    let (filename, file_path) = file_info.unwrap_or(("Unknown".to_string(), "".to_string()));

    let storage_saved = total_original_size - total_compressed_size;
    let compression_ratio = if total_original_size > 0 {
        (total_compressed_size as f64 / total_original_size as f64) * 100.0
    } else {
        100.0
    };

    let stats = VersionStorageStats {
        file_id: file_id.clone(),
        filename,
        file_path,
        version_count: versions.len() as i64,
        total_original_size,
        total_compressed_size,
        storage_saved,
        compression_ratio,
        last_version_at: versions.first().map(|v| v.created_at.clone()),
        first_version_at: versions.last().map(|v| v.created_at.clone()),
    };

    Ok(Json(VersionTimeline {
        versions: versions_with_tags,
        stats,
    }))
}

/// Update version (comment, pinned, starred)
async fn update_version(
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
    _user: UserInfo,
    Json(req): Json<UpdateVersionRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(comment) = req.comment {
        updates.push("comment = ?");
        values.push(comment);
    }
    if let Some(is_pinned) = req.is_pinned {
        updates.push("is_pinned = ?");
        values.push(if is_pinned {
            "1".to_string()
        } else {
            "0".to_string()
        });
    }
    if let Some(is_starred) = req.is_starred {
        updates.push("is_starred = ?");
        values.push(if is_starred {
            "1".to_string()
        } else {
            "0".to_string()
        });
    }

    if updates.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let query = format!(
        "UPDATE file_versions SET {} WHERE id = ? AND file_id = ?",
        updates.join(", ")
    );

    let mut db_query = sqlx::query(&query);
    for value in values {
        db_query = db_query.bind(value);
    }
    db_query = db_query.bind(&version_id).bind(&file_id);

    let result = db_query
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Add tag to version
async fn add_version_tag(
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
    user: UserInfo,
    Json(req): Json<CreateTagRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verify version exists
    let version_exists: Option<(i32,)> =
        sqlx::query_as("SELECT 1 FROM file_versions WHERE id = ? AND file_id = ?")
            .bind(&version_id)
            .bind(&file_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if version_exists.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let tag_id = Uuid::new_v4().to_string();
    let tag_color = req.tag_color.unwrap_or_else(|| "#3b82f6".to_string());

    let result = sqlx::query(
        r#"
        INSERT INTO version_tags (id, version_id, tag_name, tag_color, description, created_by)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&tag_id)
    .bind(&version_id)
    .bind(&req.tag_name)
    .bind(&tag_color)
    .bind(&req.description)
    .bind(&user.id)
    .execute(&state.db_pool)
    .await;

    match result {
        Ok(_) => {
            let tag = VersionTag {
                id: tag_id,
                version_id,
                tag_name: req.tag_name,
                tag_color,
                description: req.description,
                created_by: user.id,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            Ok((StatusCode::CREATED, Json(tag)))
        }
        Err(e) => {
            if e.to_string().contains("UNIQUE") {
                Err(StatusCode::CONFLICT) // Tag already exists
            } else {
                eprintln!("Database error: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

/// Remove tag from version
async fn remove_version_tag(
    State(state): State<AppState>,
    Path((_file_id, _version_id, tag_id)): Path<(String, String, String)>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query("DELETE FROM version_tags WHERE id = ?")
        .bind(&tag_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get tag templates
async fn get_tag_templates(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let templates = sqlx::query_as::<_, VersionTagTemplate>(
        "SELECT id, name, color, description, is_system FROM version_tag_templates ORDER BY is_system DESC, name"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(templates))
}

/// Get diff between two versions
async fn get_version_diff(
    State(state): State<AppState>,
    Path((_file_id, from_version_id, to_version_id)): Path<(String, String, String)>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if diff is cached
    let cached_diff: Option<(String, String, i32, i32)> = sqlx::query_as(
        "SELECT diff_type, diff_content, added_lines, removed_lines FROM version_diffs 
         WHERE from_version_id = ? AND to_version_id = ?",
    )
    .bind(&from_version_id)
    .bind(&to_version_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((diff_type, diff_content, added_lines, removed_lines)) = cached_diff {
        return Ok(Json(serde_json::json!({
            "diff_type": diff_type,
            "diff_content": diff_content,
            "added_lines": added_lines,
            "removed_lines": removed_lines,
            "from_version_id": from_version_id,
            "to_version_id": to_version_id,
            "cached": true
        })));
    }

    // Get version contents
    let from_content = version_storage_service::restore_version(&state.db_pool, &from_version_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let to_content = version_storage_service::restore_version(&state.db_pool, &to_version_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Check if binary
    let is_text = |content: &[u8]| -> bool { content.iter().take(8000).all(|&b| b != 0) };

    if !is_text(&from_content) || !is_text(&to_content) {
        return Ok(Json(serde_json::json!({
            "diff_type": "binary",
            "diff_content": null,
            "added_lines": 0,
            "removed_lines": 0,
            "from_version_id": from_version_id,
            "to_version_id": to_version_id,
            "message": "Binary files cannot be compared"
        })));
    }

    let from_text = String::from_utf8_lossy(&from_content);
    let to_text = String::from_utf8_lossy(&to_content);

    // Simple line-by-line diff
    let from_lines: Vec<&str> = from_text.lines().collect();
    let to_lines: Vec<&str> = to_text.lines().collect();

    let mut diff_lines = Vec::new();
    let mut added = 0;
    let mut removed = 0;

    // Simple diff algorithm (for now)
    let max_lines = std::cmp::max(from_lines.len(), to_lines.len());
    for i in 0..max_lines {
        let from_line = from_lines.get(i).copied();
        let to_line = to_lines.get(i).copied();

        match (from_line, to_line) {
            (Some(f), Some(t)) if f == t => {
                diff_lines.push(format!("  {}", f));
            }
            (Some(f), Some(t)) => {
                diff_lines.push(format!("- {}", f));
                diff_lines.push(format!("+ {}", t));
                removed += 1;
                added += 1;
            }
            (Some(f), None) => {
                diff_lines.push(format!("- {}", f));
                removed += 1;
            }
            (None, Some(t)) => {
                diff_lines.push(format!("+ {}", t));
                added += 1;
            }
            (None, None) => {}
        }
    }

    let diff_content = diff_lines.join("\n");

    // Cache the diff
    let diff_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT OR REPLACE INTO version_diffs (id, from_version_id, to_version_id, diff_type, diff_content, added_lines, removed_lines)
         VALUES (?, ?, ?, 'text', ?, ?, ?)"
    )
    .bind(&diff_id)
    .bind(&from_version_id)
    .bind(&to_version_id)
    .bind(&diff_content)
    .bind(added)
    .bind(removed)
    .execute(&state.db_pool)
    .await;

    Ok(Json(serde_json::json!({
        "diff_type": "text",
        "diff_content": diff_content,
        "added_lines": added,
        "removed_lines": removed,
        "from_version_id": from_version_id,
        "to_version_id": to_version_id,
        "cached": false
    })))
}

/// Get storage stats for all files
async fn get_storage_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let stats = sqlx::query_as::<
        _,
        (
            String,
            String,
            String,
            i64,
            i64,
            i64,
            Option<String>,
            Option<String>,
        ),
    >(
        r#"
        SELECT 
            f.id,
            f.filename,
            f.file_path,
            COUNT(fv.id) as version_count,
            COALESCE(SUM(fv.original_size), 0) as total_original_size,
            COALESCE(SUM(fv.compressed_size), 0) as total_compressed_size,
            MAX(fv.created_at) as last_version_at,
            MIN(fv.created_at) as first_version_at
        FROM files f
        LEFT JOIN file_versions fv ON f.id = fv.file_id
        GROUP BY f.id, f.filename, f.file_path
        HAVING version_count > 0
        ORDER BY total_original_size DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result: Vec<VersionStorageStats> = stats
        .into_iter()
        .map(
            |(
                file_id,
                filename,
                file_path,
                version_count,
                total_original_size,
                total_compressed_size,
                last_version_at,
                first_version_at,
            )| {
                let storage_saved = total_original_size - total_compressed_size;
                let compression_ratio = if total_original_size > 0 {
                    (total_compressed_size as f64 / total_original_size as f64) * 100.0
                } else {
                    100.0
                };

                VersionStorageStats {
                    file_id,
                    filename,
                    file_path,
                    version_count,
                    total_original_size,
                    total_compressed_size,
                    storage_saved,
                    compression_ratio,
                    last_version_at,
                    first_version_at,
                }
            },
        )
        .collect();

    Ok(Json(result))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/versions/{file_id}",
            get(list_versions).post(create_version),
        )
        .route("/versions/{file_id}/timeline", get(get_version_timeline))
        .route(
            "/versions/{file_id}/{version_id}",
            delete(delete_version).put(update_version),
        )
        .route(
            "/versions/{file_id}/{version_id}/restore",
            post(restore_version),
        )
        .route(
            "/versions/{file_id}/{version_id}/tags",
            post(add_version_tag),
        )
        .route(
            "/versions/{file_id}/{version_id}/tags/{tag_id}",
            delete(remove_version_tag),
        )
        .route(
            "/versions/{file_id}/{from_version_id}/diff/{to_version_id}",
            get(get_version_diff),
        )
        .route("/version-tags/templates", get(get_tag_templates))
        .route("/version-storage-stats", get(get_storage_stats))
}
