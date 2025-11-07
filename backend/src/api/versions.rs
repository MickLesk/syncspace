///! File Versioning API Routes with Differential Storage and Compression//! File Versioning API Routes



use axum::{use axum::{

    body::Bytes,    extract::{Path, State},

    extract::{Path, State},    http::StatusCode,

    http::{header, StatusCode},    response::{IntoResponse, Json},

    response::{IntoResponse, Json, Response},    routing::{get, post},

    routing::{delete, get, post},    Router,

    Router,};

};use serde::{Deserialize, Serialize};

use serde::{Deserialize, Serialize};use sqlx::FromRow;

use sqlx::FromRow;use uuid::Uuid;

use uuid::Uuid;use chrono::Utc;

use chrono::Utc;

use std::path::PathBuf;use crate::AppState;

use crate::auth::UserInfo;

use crate::AppState;

use crate::auth::UserInfo;#[derive(Debug, Serialize, Deserialize, FromRow)]

use crate::services::version_storage_service;pub struct FileVersion {

    pub id: String,

#[derive(Debug, Serialize, Deserialize, FromRow)]    pub file_id: String,

pub struct FileVersion {    pub version_number: i32,

    pub id: String,    pub file_path: String,

    pub file_id: String,    pub file_size: i64,

    pub version_number: i32,    pub created_by: String,

    pub storage_path: Option<String>,    pub created_at: String,

    pub original_size: i64,    pub comment: Option<String>,

    pub compressed_size: i64,}

    pub is_compressed: bool,

    pub is_differential: bool,#[derive(Debug, Deserialize)]

    pub base_version_id: Option<String>,pub struct CreateVersionRequest {

    pub checksum: String,    pub comment: Option<String>,

    pub created_by: String,}

    pub created_at: String,

    pub comment: Option<String>,#[derive(Debug, Deserialize)]

}pub struct UpdateVersionMetadataRequest {

    pub comment: Option<String>,

#[derive(Debug, Deserialize)]}

pub struct CreateVersionRequest {

    pub file_path: String,/// List all versions of a file

    pub comment: Option<String>,async fn list_versions(

}    State(state): State<AppState>,

    Path(file_id): Path<String>,

#[derive(Debug, Serialize)]) -> Result<impl IntoResponse, StatusCode> {

pub struct VersionStats {    let versions = sqlx::query_as::<_, FileVersion>(

    pub version_count: i32,        r#"

    pub total_original_size: i64,        SELECT id, file_id, version_number, file_path, file_size,

    pub total_compressed_size: i64,               created_by, created_at, comment

    pub compression_ratio: f64,        FROM file_versions

    pub differential_count: i32,        WHERE file_id = ?

}        ORDER BY version_number DESC

        "#,

#[derive(Debug, Serialize)]    )

pub struct RestorePreview {    .bind(&file_id)

    pub version_id: String,    .fetch_all(&state.db)

    pub version_number: i32,    .await

    pub original_size: i64,    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    pub created_at: String,    

    pub created_by: String,    Ok(Json(versions))

    pub comment: Option<String>,}

    pub is_differential: bool,

    pub base_version_id: Option<String>,/// Create a new version

    pub checksum: String,async fn create_version(

}    State(state): State<AppState>,

    Path(file_id): Path<String>,

/// List all versions of a file    user_info: UserInfo,

async fn list_versions(    Json(req): Json<CreateVersionRequest>,

    State(state): State<AppState>,) -> Result<impl IntoResponse, StatusCode> {

    Path(file_id): Path<String>,    // Get current max version number

    user: UserInfo,    let max_version: i32 = sqlx::query_scalar(

) -> Result<impl IntoResponse, StatusCode> {        "SELECT COALESCE(MAX(version_number), 0) FROM file_versions WHERE file_id = ?"

    let versions = sqlx::query_as::<_, FileVersion>(    )

        r#"    .bind(&file_id)

        SELECT id, file_id, version_number, storage_path, original_size, compressed_size,    .fetch_one(&state.db)

               is_compressed, is_differential, base_version_id, checksum,     .await

               created_by, created_at, comment    .unwrap_or(0);

        FROM file_versions    

        WHERE file_id = ?    let version_id = Uuid::new_v4().to_string();

        ORDER BY version_number DESC    let new_version_number = max_version + 1;

        "#,    let now = Utc::now().to_rfc3339();

    )    

    .bind(&file_id)    // TODO: Copy actual file to versioned storage

    .fetch_all(&state.db_pool)    

    .await    sqlx::query(

    .map_err(|e| {        r#"

        eprintln!("Database error: {}", e);        INSERT INTO file_versions 

        StatusCode::INTERNAL_SERVER_ERROR        (id, file_id, version_number, file_path, file_size, created_by, created_at, comment)

    })?;        VALUES (?, ?, ?, '', 0, ?, ?, ?)

        "#,

    Ok(Json(versions))    )

}    .bind(&version_id)

    .bind(&file_id)

/// Create a new version with differential storage and compression    .bind(new_version_number)

async fn create_version(    .bind(&user_info.id)

    State(state): State<AppState>,    .bind(&now)

    Path(file_id): Path<String>,    .bind(&req.comment)

    user: UserInfo,    .execute(&state.db)

    Json(req): Json<CreateVersionRequest>,    .await

) -> Result<impl IntoResponse, StatusCode> {    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let file_path = PathBuf::from(&req.file_path);    

        Ok((StatusCode::CREATED, Json(serde_json::json!({

    if !file_path.exists() {        "id": version_id,

        return Err(StatusCode::NOT_FOUND);        "version_number": new_version_number

    }    }))))

    }

    match version_storage_service::create_version(

        &state.db_pool,/// Get version info

        &file_id,async fn get_version(

        &file_path,    State(state): State<AppState>,

        &user.id,    Path(version_id): Path<String>,

        req.comment.as_deref(),) -> Result<impl IntoResponse, StatusCode> {

    )    let version = sqlx::query_as::<_, FileVersion>(

    .await        r#"

    {        SELECT id, file_id, version_number, file_path, file_size,

        Ok(version_metadata) => {               created_by, created_at, comment

            // Convert to FileVersion response        FROM file_versions

            let version = FileVersion {        WHERE id = ?

                id: version_metadata.id,        "#,

                file_id: version_metadata.file_id,    )

                version_number: version_metadata.version_number,    .bind(&version_id)

                storage_path: Some(version_metadata.storage_path),    .fetch_optional(&state.db)

                original_size: version_metadata.original_size,    .await

                compressed_size: version_metadata.compressed_size,    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?

                is_compressed: version_metadata.is_compressed,    .ok_or(StatusCode::NOT_FOUND)?;

                is_differential: version_metadata.is_differential,    

                base_version_id: version_metadata.base_version_id,    Ok(Json(version))

                checksum: version_metadata.checksum,}

                created_by: version_metadata.created_by,

                created_at: version_metadata.created_at,/// Delete a version

                comment: version_metadata.comment,async fn delete_version(

            };    State(state): State<AppState>,

                Path(version_id): Path<String>,

            Ok((StatusCode::CREATED, Json(version)))    user_info: UserInfo,

        }) -> Result<impl IntoResponse, StatusCode> {

        Err(e) => {    let result = sqlx::query(

            eprintln!("Error creating version: {}", e);        "DELETE FROM file_versions WHERE id = ? AND created_by = ?"

            Err(StatusCode::INTERNAL_SERVER_ERROR)    )

        }    .bind(&version_id)

    }    .bind(&user_info.id)

}    .execute(&state.db)

    .await

/// Get a specific version    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

async fn get_version(    

    State(state): State<AppState>,    if result.rows_affected() == 0 {

    Path((file_id, version_id)): Path<(String, String)>,        return Err(StatusCode::NOT_FOUND);

    user: UserInfo,    }

) -> Result<impl IntoResponse, StatusCode> {    

    let version = sqlx::query_as::<_, FileVersion>(    Ok(StatusCode::NO_CONTENT)

        r#"}

        SELECT id, file_id, version_number, storage_path, original_size, compressed_size,

               is_compressed, is_differential, base_version_id, checksum,/// Restore a version (make it current)

               created_by, created_at, commentasync fn restore_version(

        FROM file_versions    State(state): State<AppState>,

        WHERE id = ? AND file_id = ?    Path(version_id): Path<String>,

        "#,    user_info: UserInfo,

    )) -> Result<impl IntoResponse, StatusCode> {

    .bind(&version_id)    // TODO: Implement actual file restoration

    .bind(&file_id)    Ok(Json(serde_json::json!({

    .fetch_optional(&state.db_pool)        "message": "Version restored successfully"

    .await    })))

    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;}



    match version {/// Get diff between two versions

        Some(v) => Ok(Json(v)),async fn get_version_diff(

        None => Err(StatusCode::NOT_FOUND),    State(state): State<AppState>,

    }    Path((from_version_id, to_version_id)): Path<(String, String)>,

}) -> Result<impl IntoResponse, StatusCode> {

    // TODO: Implement diff logic

/// Delete a version    Ok(Json(serde_json::json!({

async fn delete_version(        "from_version": from_version_id,

    State(state): State<AppState>,        "to_version": to_version_id,

    Path((file_id, version_id)): Path<(String, String)>,        "diff": "Not implemented"

    user: UserInfo,    })))

) -> Result<impl IntoResponse, StatusCode> {}

    // Get version info first

    let version: Option<(String,)> = sqlx::query_as(/// Download a specific version

        "SELECT storage_path FROM file_versions WHERE id = ? AND file_id = ?"async fn download_version(

    )    State(state): State<AppState>,

    .bind(&version_id)    Path(version_id): Path<String>,

    .bind(&file_id)) -> Result<impl IntoResponse, StatusCode> {

    .fetch_optional(&state.db_pool)    // TODO: Implement file download

    .await    Err::<Json<serde_json::Value>, _>(StatusCode::NOT_IMPLEMENTED)

    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;}

    

    if let Some((storage_path,)) = version {/// Get version metadata

        // Delete file from storageasync fn get_version_metadata(

        let _ = tokio::fs::remove_file(&storage_path).await;    State(state): State<AppState>,

            Path(version_id): Path<String>,

        // Delete from database) -> Result<impl IntoResponse, StatusCode> {

        let result = sqlx::query("DELETE FROM file_versions WHERE id = ?")    let version = get_version(State(state), Path(version_id)).await?;

            .bind(&version_id)    Ok(version)

            .execute(&state.db_pool)}

            .await

            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;/// Update version metadata

async fn update_version_metadata(

        if result.rows_affected() > 0 {    State(state): State<AppState>,

            Ok(StatusCode::NO_CONTENT)    Path(version_id): Path<String>,

        } else {    Json(req): Json<UpdateVersionMetadataRequest>,

            Err(StatusCode::NOT_FOUND)) -> Result<impl IntoResponse, StatusCode> {

        }    sqlx::query(

    } else {        "UPDATE file_versions SET comment = ? WHERE id = ?"

        Err(StatusCode::NOT_FOUND)    )

    }    .bind(&req.comment)

}    .bind(&version_id)

    .execute(&state.db)

/// Restore a version (returns file content)    .await

async fn restore_version(    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    State(state): State<AppState>,    

    Path((file_id, version_id)): Path<(String, String)>,    Ok(Json(serde_json::json!({

    user: UserInfo,        "message": "Metadata updated successfully"

) -> Result<Response, StatusCode> {    })))

    match version_storage_service::restore_version(&state.db_pool, &version_id).await {}

        Ok(content) => {

            let body = Bytes::from(content);/// Build versions router

            pub fn router() -> Router<AppState> {

            Ok(Response::builder()    Router::new()

                .status(StatusCode::OK)        .route("/versions/file/{file_id}", get(list_versions))

                .header(header::CONTENT_TYPE, "application/octet-stream")        .route("/versions/file/{file_id}/create", post(create_version))

                .header(        .route("/versions/version/{version_id}", get(get_version).delete(delete_version))

                    header::CONTENT_DISPOSITION,        .route("/versions/version/{version_id}/restore", post(restore_version))

                    format!("attachment; filename=\"restored_v{}.dat\"", version_id),        .route("/versions/diff/{from_version_id}/{to_version_id}", get(get_version_diff))

                )        .route("/versions/version/{version_id}/download", get(download_version))

                .body(body.into())        .route("/versions/version/{version_id}/metadata", get(get_version_metadata).put(update_version_metadata))

                .unwrap())}

        }
        Err(e) => {
            eprintln!("Error restoring version: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get restore preview (metadata before restoring)
async fn get_restore_preview(
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let version = sqlx::query_as::<_, FileVersion>(
        r#"
        SELECT id, file_id, version_number, storage_path, original_size, compressed_size,
               is_compressed, is_differential, base_version_id, checksum,
               created_by, created_at, comment
        FROM file_versions
        WHERE id = ? AND file_id = ?
        "#,
    )
    .bind(&version_id)
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match version {
        Some(v) => {
            let preview = RestorePreview {
                version_id: v.id,
                version_number: v.version_number,
                original_size: v.original_size,
                created_at: v.created_at,
                created_by: v.created_by,
                comment: v.comment,
                is_differential: v.is_differential,
                base_version_id: v.base_version_id,
                checksum: v.checksum,
            };
            Ok(Json(preview))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get diff between two versions
async fn get_version_diff(
    State(state): State<AppState>,
    Path((file_id, from_version, to_version)): Path<(String, String, String)>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    match version_storage_service::get_version_diff(&state.db_pool, &from_version, &to_version)
        .await
    {
        Ok(diff) => Ok(Json(diff)),
        Err(e) => {
            eprintln!("Error getting diff: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Download version content (without restoring)
async fn download_version(
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
    user: UserInfo,
) -> Result<Response, StatusCode> {
    // Same as restore but with different filename
    match version_storage_service::restore_version(&state.db_pool, &version_id).await {
        Ok(content) => {
            let body = Bytes::from(content);
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"version_{}.dat\"", version_id),
                )
                .body(body.into())
                .unwrap())
        }
        Err(e) => {
            eprintln!("Error downloading version: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get version statistics
async fn get_version_stats(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let stats: (i32, i64, i64, i32) = sqlx::query_as(
        r#"
        SELECT 
            COUNT(*) as version_count,
            SUM(original_size) as total_original_size,
            SUM(compressed_size) as total_compressed_size,
            SUM(CASE WHEN is_differential = 1 THEN 1 ELSE 0 END) as differential_count
        FROM file_versions
        WHERE file_id = ?
        "#,
    )
    .bind(&file_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let compression_ratio = if stats.1 > 0 {
        (stats.2 as f64 / stats.1 as f64) * 100.0
    } else {
        100.0
    };

    let version_stats = VersionStats {
        version_count: stats.0,
        total_original_size: stats.1,
        total_compressed_size: stats.2,
        compression_ratio,
        differential_count: stats.3,
    };

    Ok(Json(version_stats))
}

/// Cleanup old versions for a file
async fn cleanup_versions(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    match version_storage_service::cleanup_old_versions(&state.db_pool, &file_id).await {
        Ok(deleted_count) => Ok(Json(serde_json::json!({
            "deleted_count": deleted_count,
            "message": format!("Deleted {} old versions", deleted_count)
        }))),
        Err(e) => {
            eprintln!("Error cleaning up versions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Background cleanup job - cleanup all old versions
async fn cleanup_all_versions(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    match version_storage_service::cleanup_all_old_versions(&state.db_pool).await {
        Ok(deleted_count) => Ok(Json(serde_json::json!({
            "deleted_count": deleted_count,
            "message": format!("Deleted {} old versions across all files", deleted_count)
        }))),
        Err(e) => {
            eprintln!("Error cleaning up all versions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/files/:file_id/versions", get(list_versions).post(create_version))
        .route("/files/:file_id/versions/:version_id", get(get_version).delete(delete_version))
        .route("/files/:file_id/versions/:version_id/restore", post(restore_version))
        .route("/files/:file_id/versions/:version_id/preview", get(get_restore_preview))
        .route("/files/:file_id/versions/:from_version/diff/:to_version", get(get_version_diff))
        .route("/files/:file_id/versions/:version_id/download", get(download_version))
        .route("/files/:file_id/versions/stats", get(get_version_stats))
        .route("/files/:file_id/versions/cleanup", post(cleanup_versions))
        .route("/versions/cleanup-all", post(cleanup_all_versions))
}
