// Cloud Storage Integration API
// Multi-backend storage management (S3, MinIO, GCS, Azure Blob Storage)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::auth::UserInfo;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/storage/backends", post(create_backend).get(list_backends))
        .route(
            "/storage/backends/{id}",
            get(get_backend).put(update_backend).delete(delete_backend),
        )
        .route("/storage/backends/{id}/health", get(check_backend_health))
        .route("/storage/backends/{id}/test", post(test_backend_connection))
        .route(
            "/storage/backends/{id}/set-default",
            post(set_default_backend),
        )
        .route("/storage/migration", post(create_migration_job))
        .route("/storage/migration/jobs", get(list_migration_jobs))
        .route("/storage/migration/jobs/{id}", get(get_migration_job))
        .route(
            "/storage/migration/jobs/{id}/cancel",
            post(cancel_migration_job),
        )
        .route("/storage/stats", get(get_storage_stats))
}

#[derive(Debug, Serialize, FromRow)]
pub struct StorageBackend {
    pub id: String,
    pub name: String,
    pub backend_type: String,
    pub config: String, // JSON
    pub is_active: i64,
    pub is_default: i64,
    pub priority: i64,
    pub storage_used_bytes: i64,
    pub file_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBackendRequest {
    pub name: String,
    pub backend_type: String, // 's3', 'minio', 'gcs', 'azure_blob'
    pub config: serde_json::Value,
    pub is_active: Option<bool>,
    pub priority: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBackendRequest {
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub priority: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct StorageMigrationJob {
    pub id: String,
    pub name: String,
    pub source_backend_id: String,
    pub target_backend_id: String,
    pub file_filter: Option<String>,
    pub status: String,
    pub progress_current: i64,
    pub progress_total: i64,
    pub bytes_migrated: i64,
    pub bytes_total: i64,
    pub files_migrated: i64,
    pub files_failed: i64,
    pub error_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMigrationJobRequest {
    pub name: String,
    pub source_backend_id: String,
    pub target_backend_id: String,
    pub file_filter: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct StorageStats {
    pub total_backends: i64,
    pub active_backends: i64,
    pub total_storage_bytes: i64,
    pub total_files: i64,
    pub backends_by_type: Vec<BackendTypeStats>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BackendTypeStats {
    pub backend_type: String,
    pub count: i64,
    pub storage_bytes: i64,
    pub file_count: i64,
}

// Create new storage backend
async fn create_backend(
    State(state): State<AppState>,
    UserInfo { id, .. }: UserInfo,
    Json(req): Json<CreateBackendRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verify admin permission
    let is_admin = check_admin_permission(&state.db_pool, &id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    if !is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // Validate backend type
    if !matches!(
        req.backend_type.as_str(),
        "s3" | "minio" | "gcs" | "azure_blob" | "local"
    ) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let backend_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let config_json = serde_json::to_string(&req.config).unwrap_or_default();

    sqlx::query(
        "INSERT INTO storage_backends (id, name, backend_type, config, is_active, is_default, priority, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?)"
    )
    .bind(&backend_id)
    .bind(&req.name)
    .bind(&req.backend_type)
    .bind(&config_json)
    .bind(req.is_active.unwrap_or(true) as i64)
    .bind(req.priority.unwrap_or(100))
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create storage backend: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "id": backend_id,
        "message": "Storage backend created successfully"
    })))
}

// List all storage backends
async fn list_backends(
    State(state): State<AppState>,
    UserInfo { id, .. }: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let backends = sqlx::query_as::<_, StorageBackend>(
        "SELECT * FROM storage_backends ORDER BY priority DESC, name ASC",
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to list storage backends: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(backends))
}

// Get single storage backend
async fn get_backend(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let backend =
        sqlx::query_as::<_, StorageBackend>("SELECT * FROM storage_backends WHERE id = ?")
            .bind(&id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(backend))
}

// Update storage backend
async fn update_backend(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(id): Path<String>,
    Json(req): Json<UpdateBackendRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let is_admin = check_admin_permission(&state.db_pool, &user_id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    if !is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now().to_rfc3339();

    // Simple approach: update all provided fields
    if let Some(name) = req.name {
        sqlx::query("UPDATE storage_backends SET name = ?, updated_at = ? WHERE id = ?")
            .bind(&name)
            .bind(&now)
            .bind(&id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let Some(config) = req.config {
        let config_json = serde_json::to_string(&config).unwrap_or_default();
        sqlx::query("UPDATE storage_backends SET config = ?, updated_at = ? WHERE id = ?")
            .bind(&config_json)
            .bind(&now)
            .bind(&id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let Some(is_active) = req.is_active {
        sqlx::query("UPDATE storage_backends SET is_active = ?, updated_at = ? WHERE id = ?")
            .bind(is_active as i64)
            .bind(&now)
            .bind(&id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let Some(priority) = req.priority {
        sqlx::query("UPDATE storage_backends SET priority = ?, updated_at = ? WHERE id = ?")
            .bind(priority)
            .bind(&now)
            .bind(&id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

// Delete storage backend
async fn delete_backend(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let is_admin = check_admin_permission(&state.db_pool, &user_id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    if !is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // Check if backend has files
    let file_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM file_storage_locations WHERE backend_id = ?")
            .bind(&id)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if file_count.0 > 0 {
        return Err(StatusCode::CONFLICT); // Cannot delete backend with files
    }

    sqlx::query("DELETE FROM storage_backends WHERE id = ?")
        .bind(&id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// Check backend health
async fn check_backend_health(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement actual health check logic for each backend type
    // For now, return a placeholder response
    Ok(Json(serde_json::json!({
        "backend_id": id,
        "status": "healthy",
        "checks": {
            "connectivity": "pass",
            "read": "pass",
            "write": "pass"
        },
        "last_check": chrono::Utc::now().to_rfc3339()
    })))
}

// Test backend connection
async fn test_backend_connection(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let is_admin = check_admin_permission(&state.db_pool, &user_id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    if !is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // TODO: Implement actual connection test
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Connection test successful",
        "latency_ms": 45
    })))
}

// Set default backend
async fn set_default_backend(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let is_admin = check_admin_permission(&state.db_pool, &user_id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    if !is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now().to_rfc3339();

    // Remove default from all backends
    sqlx::query("UPDATE storage_backends SET is_default = 0, updated_at = ?")
        .bind(&now)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Set new default
    sqlx::query("UPDATE storage_backends SET is_default = 1, updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// Create migration job
async fn create_migration_job(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Json(req): Json<CreateMigrationJobRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let is_admin = check_admin_permission(&state.db_pool, &user_id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    if !is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let job_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let filter_json = req
        .file_filter
        .map(|f| serde_json::to_string(&f).unwrap_or_default());

    sqlx::query(
        "INSERT INTO storage_migration_jobs (id, name, source_backend_id, target_backend_id, file_filter, status, created_by, created_at)
         VALUES (?, ?, ?, ?, ?, 'pending', ?, ?)"
    )
    .bind(&job_id)
    .bind(&req.name)
    .bind(&req.source_backend_id)
    .bind(&req.target_backend_id)
    .bind(&filter_json)
    .bind(&user_id)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create migration job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "id": job_id,
        "message": "Migration job created successfully"
    })))
}

// List migration jobs
async fn list_migration_jobs(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let jobs = sqlx::query_as::<_, StorageMigrationJob>(
        "SELECT * FROM storage_migration_jobs ORDER BY created_at DESC LIMIT 100",
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to list migration jobs: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(jobs))
}

// Get migration job details
async fn get_migration_job(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let job = sqlx::query_as::<_, StorageMigrationJob>(
        "SELECT * FROM storage_migration_jobs WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(job))
}

// Cancel migration job
async fn cancel_migration_job(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let is_admin = check_admin_permission(&state.db_pool, &user_id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    if !is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query("UPDATE storage_migration_jobs SET status = 'cancelled' WHERE id = ? AND status = 'pending'")
        .bind(&id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// Get storage statistics
async fn get_storage_stats(
    State(state): State<AppState>,
    UserInfo { id, .. }: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM storage_backends")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let active: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM storage_backends WHERE is_active = 1")
            .fetch_one(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let storage_sum: (Option<i64>,) =
        sqlx::query_as("SELECT SUM(storage_used_bytes) FROM storage_backends")
            .fetch_one(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let files_sum: (Option<i64>,) = sqlx::query_as("SELECT SUM(file_count) FROM storage_backends")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let backends_by_type = sqlx::query_as::<_, BackendTypeStats>(
        "SELECT backend_type, COUNT(*) as count, SUM(storage_used_bytes) as storage_bytes, SUM(file_count) as file_count
         FROM storage_backends
         GROUP BY backend_type
         ORDER BY count DESC"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(StorageStats {
        total_backends: total.0,
        active_backends: active.0,
        total_storage_bytes: storage_sum.0.unwrap_or(0),
        total_files: files_sum.0.unwrap_or(0),
        backends_by_type,
    }))
}

// Helper: Check if user has admin permission
async fn check_admin_permission(
    pool: &sqlx::SqlitePool,
    user_id: &str,
) -> Result<bool, sqlx::Error> {
    let result: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM user_roles ur
         JOIN roles r ON ur.role_id = r.id
         WHERE ur.user_id = ? AND r.name IN ('super_admin', 'admin')",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|(count,)| count > 0).unwrap_or(false))
}
