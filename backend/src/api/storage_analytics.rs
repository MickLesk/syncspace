use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::{auth::UserInfo, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/storage/analytics/overview", get(get_storage_overview))
        .route("/storage/analytics/by-user", get(get_storage_by_user))
        .route("/storage/analytics/by-folder", get(get_storage_by_folder))
        .route("/storage/analytics/top-files", get(get_top_files))
        .route("/storage/analytics/growth", get(get_storage_growth))
        .route("/storage/analytics/duplicates", get(get_duplicate_waste))
}

#[derive(Serialize)]
struct StorageOverview {
    total_files: i64,
    total_size_bytes: i64,
    total_size_formatted: String,
    active_users: i64,
    avg_file_size_bytes: i64,
    largest_file_bytes: i64,
    storage_limit_bytes: Option<i64>,
    usage_percentage: Option<f64>,
}

#[derive(Serialize)]
struct UserStorageStats {
    user_id: String,
    username: Option<String>,
    total_files: i64,
    total_size_bytes: i64,
    total_size_formatted: String,
    last_upload: Option<String>,
}

#[derive(Serialize)]
struct FolderStorageStats {
    folder: String,
    file_count: i64,
    total_size_bytes: i64,
    total_size_formatted: String,
}

#[derive(Serialize)]
struct TopFile {
    id: String,
    filename: String,
    file_path: String,
    size_bytes: i64,
    size_formatted: String,
    mime_type: Option<String>,
    owner_id: String,
    created_at: String,
}

#[derive(Serialize)]
struct StorageGrowthPoint {
    period: String,
    files_added: i64,
    size_added_bytes: i64,
    size_added_formatted: String,
}

#[derive(Deserialize)]
struct TopFilesQuery {
    limit: Option<i64>,
}

#[derive(Deserialize)]
struct GrowthQuery {
    days: Option<i64>,
}

#[derive(Serialize)]
struct DuplicateWaste {
    total_duplicates: i64,
    wasted_bytes: i64,
    wasted_formatted: String,
    duplicate_groups: i64,
    savings_potential_formatted: String,
}

/// GET /api/storage/analytics/overview
/// Returns overall storage statistics
async fn get_storage_overview(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let total_stats = sqlx::query(
        r#"
        SELECT 
            COUNT(*) as total_files,
            COALESCE(SUM(size_bytes), 0) as total_size,
            COALESCE(AVG(size_bytes), 0) as avg_size,
            COALESCE(MAX(size_bytes), 0) as max_size
        FROM files 
        WHERE is_deleted = 0
        "#,
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch storage overview: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let active_users = sqlx::query(
        r#"
        SELECT COUNT(DISTINCT owner_id) as count 
        FROM files 
        WHERE is_deleted = 0
        "#,
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_files: i64 = total_stats.try_get("total_files").unwrap_or(0);
    let total_size: i64 = total_stats.try_get("total_size").unwrap_or(0);
    let avg_size: i64 = total_stats.try_get("avg_size").unwrap_or(0);
    let max_size: i64 = total_stats.try_get("max_size").unwrap_or(0);
    let active_users_count: i64 = active_users.try_get("count").unwrap_or(0);

    let storage_limit: Option<i64> = None; // TODO: Implement storage limits per system
    let usage_percentage = storage_limit.map(|limit| (total_size as f64 / limit as f64) * 100.0);

    Ok(Json(StorageOverview {
        total_files,
        total_size_bytes: total_size,
        total_size_formatted: format_bytes(total_size),
        active_users: active_users_count,
        avg_file_size_bytes: avg_size,
        largest_file_bytes: max_size,
        storage_limit_bytes: storage_limit,
        usage_percentage,
    }))
}

/// GET /api/storage/analytics/by-user
/// Returns storage statistics per user
async fn get_storage_by_user(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let rows = sqlx::query(
        r#"
        SELECT 
            f.owner_id as user_id,
            u.username,
            COUNT(*) as total_files,
            COALESCE(SUM(f.size_bytes), 0) as total_size_bytes,
            MAX(f.created_at) as last_upload
        FROM files f
        LEFT JOIN users u ON f.owner_id = u.id
        WHERE f.is_deleted = 0
        GROUP BY f.owner_id, u.username
        ORDER BY total_size_bytes DESC
        "#,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch user storage stats: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let formatted_stats: Vec<UserStorageStats> = rows
        .into_iter()
        .map(|row| {
            let user_id: String = row.try_get("user_id").unwrap_or_default();
            let username: Option<String> = row.try_get("username").ok();
            let total_files: i64 = row.try_get("total_files").unwrap_or(0);
            let total_size_bytes: i64 = row.try_get("total_size_bytes").unwrap_or(0);
            let last_upload: Option<String> = row.try_get("last_upload").ok();

            UserStorageStats {
                user_id,
                username,
                total_files,
                total_size_bytes,
                total_size_formatted: format_bytes(total_size_bytes),
                last_upload,
            }
        })
        .collect();

    Ok(Json(formatted_stats))
}

/// GET /api/storage/analytics/by-folder
/// Returns storage statistics per folder
async fn get_storage_by_folder(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let rows = sqlx::query(
        r#"
        SELECT 
            CASE 
                WHEN path LIKE '%/%' THEN 
                    SUBSTR(path, 1, INSTR(path, '/'))
                ELSE 
                    '/'
            END as folder,
            COUNT(*) as file_count,
            COALESCE(SUM(size_bytes), 0) as total_size
        FROM files 
        WHERE is_deleted = 0
        GROUP BY folder
        ORDER BY total_size DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch folder storage stats: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let formatted_stats: Vec<FolderStorageStats> = rows
        .into_iter()
        .map(|row| {
            let folder: String = row.try_get("folder").unwrap_or_else(|_| "/".to_string());
            let file_count: i64 = row.try_get("file_count").unwrap_or(0);
            let total_size: i64 = row.try_get("total_size").unwrap_or(0);

            FolderStorageStats {
                folder,
                file_count,
                total_size_bytes: total_size,
                total_size_formatted: format_bytes(total_size),
            }
        })
        .collect();

    Ok(Json(formatted_stats))
}

/// GET /api/storage/analytics/top-files
/// Returns the largest files
async fn get_top_files(
    State(state): State<AppState>,
    Query(params): Query<TopFilesQuery>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let limit = params.limit.unwrap_or(100).min(500);

    let rows = sqlx::query(
        r#"
        SELECT 
            id,
            name as filename,
            path as file_path,
            size_bytes,
            mime_type,
            owner_id,
            created_at
        FROM files 
        WHERE is_deleted = 0
        ORDER BY size_bytes DESC
        LIMIT ?
        "#,
    )
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch top files: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let formatted_files: Vec<TopFile> = rows
        .into_iter()
        .map(|row| {
            let id: String = row.try_get("id").unwrap_or_default();
            let filename: String = row.try_get("filename").unwrap_or_default();
            let file_path: String = row.try_get("file_path").unwrap_or_default();
            let size_bytes: i64 = row.try_get("size_bytes").unwrap_or(0);
            let mime_type: Option<String> = row.try_get("mime_type").ok();
            let owner_id: String = row.try_get("owner_id").unwrap_or_default();
            let created_at: String = row.try_get("created_at").unwrap_or_default();

            TopFile {
                id,
                filename,
                file_path,
                size_bytes,
                size_formatted: format_bytes(size_bytes),
                mime_type,
                owner_id,
                created_at,
            }
        })
        .collect();

    Ok(Json(formatted_files))
}

/// GET /api/storage/analytics/growth
/// Returns storage growth over time
async fn get_storage_growth(
    State(state): State<AppState>,
    Query(params): Query<GrowthQuery>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let days = params.days.unwrap_or(30).min(365);

    let rows = sqlx::query(
        r#"
        SELECT 
            DATE(created_at) as period,
            COUNT(*) as files_added,
            COALESCE(SUM(size_bytes), 0) as size_added
        FROM files 
        WHERE is_deleted = 0 
            AND created_at >= DATE('now', '-' || ? || ' days')
        GROUP BY period
        ORDER BY period ASC
        "#,
    )
    .bind(days)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch storage growth: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let growth_data: Vec<StorageGrowthPoint> = rows
        .into_iter()
        .map(|row| {
            let period: String = row.try_get("period").unwrap_or_default();
            let files_added: i64 = row.try_get("files_added").unwrap_or(0);
            let size_added: i64 = row.try_get("size_added").unwrap_or(0);

            StorageGrowthPoint {
                period,
                files_added,
                size_added_bytes: size_added,
                size_added_formatted: format_bytes(size_added),
            }
        })
        .collect();

    Ok(Json(growth_data))
}

/// GET /api/storage/analytics/duplicates
/// Returns statistics about duplicate files
async fn get_duplicate_waste(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let row = sqlx::query(
        r#"
        SELECT 
            COUNT(*) as total_duplicates,
            COALESCE(SUM(size_bytes * (duplicate_count - 1)), 0) as wasted_bytes,
            COUNT(DISTINCT checksum_sha256) as duplicate_groups,
            COALESCE(SUM(size_bytes * (duplicate_count - 1)), 0) as savings_potential
        FROM (
            SELECT 
                checksum_sha256,
                size_bytes,
                COUNT(*) as duplicate_count
            FROM files
            WHERE is_deleted = 0 
                AND checksum_sha256 IS NOT NULL
            GROUP BY checksum_sha256
            HAVING COUNT(*) > 1
        )
        "#,
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch duplicate waste stats: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total_duplicates: i64 = row.try_get("total_duplicates").unwrap_or(0);
    let wasted_bytes: i64 = row.try_get("wasted_bytes").unwrap_or(0);
    let duplicate_groups: i64 = row.try_get("duplicate_groups").unwrap_or(0);
    let savings_potential: i64 = row.try_get("savings_potential").unwrap_or(0);

    Ok(Json(DuplicateWaste {
        total_duplicates,
        wasted_bytes,
        wasted_formatted: format_bytes(wasted_bytes),
        duplicate_groups,
        savings_potential_formatted: format_bytes(savings_potential),
    }))
}

// Helper function to format bytes into human-readable format
fn format_bytes(bytes: i64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes_f = bytes as f64;
    let index = (bytes_f.log10() / 1024_f64.log10()).floor() as usize;
    let index = index.min(UNITS.len() - 1);

    let value = bytes_f / 1024_f64.powi(index as i32);

    format!("{:.2} {}", value, UNITS[index])
}
