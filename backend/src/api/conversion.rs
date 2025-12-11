use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use crate::{auth::UserInfo, AppState};

// ============================================================================
// Types & Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversionJob {
    pub id: String,
    pub user_id: String,
    pub source_file_path: String,
    pub source_format: String,
    pub target_format: String,
    pub status: String, // pending, processing, completed, failed, cancelled
    pub output_file_path: Option<String>,
    pub error_message: Option<String>,
    pub conversion_options: Option<String>, // JSON
    pub progress: i64,
    pub file_size_bytes: Option<i64>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversionPreset {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub source_format: String,
    pub target_format: String,
    pub options: String, // JSON
    pub is_system: bool,
    pub created_by: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateConversionRequest {
    pub source_file_path: String,
    pub target_format: String,
    pub preset_id: Option<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePresetRequest {
    pub name: String,
    pub description: Option<String>,
    pub source_format: String,
    pub target_format: String,
    pub options: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ListJobsQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/conversion/jobs", axum::routing::post(create_conversion))
        .route("/conversion/jobs", axum::routing::get(list_jobs))
        .route("/conversion/jobs/:id", axum::routing::get(get_job))
        .route("/conversion/jobs/:id/cancel", axum::routing::post(cancel_job))
        .route("/conversion/jobs/:id", axum::routing::delete(delete_job))
        .route("/conversion/formats", axum::routing::get(list_formats))
        .route("/conversion/presets", axum::routing::get(list_presets))
        .route("/conversion/presets", axum::routing::post(create_preset))
        .route("/conversion/presets/:id", axum::routing::delete(delete_preset))
}

// ============================================================================
// Handlers
// ============================================================================

/// POST /api/conversion/jobs - Start a new conversion
async fn create_conversion(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateConversionRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Detect source format from file extension
    let source_format = std::path::Path::new(&req.source_file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown")
        .to_lowercase();

    // Get options from preset or use provided options
    let options = if let Some(preset_id) = req.preset_id {
        // Load preset options
        let preset: Option<ConversionPreset> = sqlx::query_as(
            "SELECT * FROM conversion_presets WHERE id = ?"
        )
        .bind(&preset_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch preset: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        preset.map(|p| p.options).unwrap_or_else(|| "{}".to_string())
    } else {
        req.options.map(|o| o.to_string()).unwrap_or_else(|| "{}".to_string())
    };

    // Create conversion job
    let job_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO conversion_jobs 
        (id, user_id, source_file_path, source_format, target_format, conversion_options, status, progress)
        VALUES (?, ?, ?, ?, ?, ?, 'pending', 0)"
    )
    .bind(&job_id)
    .bind(&user.user_id())
    .bind(&req.source_file_path)
    .bind(&source_format)
    .bind(&req.target_format)
    .bind(&options)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create conversion job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Fetch the created job
    let job: ConversionJob = sqlx::query_as(
        "SELECT * FROM conversion_jobs WHERE id = ?"
    )
    .bind(&job_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch created job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "success": true,
        "job": job
    })))
}

/// GET /api/conversion/jobs - List user's conversion jobs
async fn list_jobs(
    State(state): State<AppState>,
    user: UserInfo,
    Query(query): Query<ListJobsQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let limit = query.limit.unwrap_or(50).min(200);

    let jobs: Vec<ConversionJob> = if let Some(status) = query.status {
        sqlx::query_as(
            "SELECT * FROM conversion_jobs 
            WHERE user_id = ? AND status = ?
            ORDER BY created_at DESC
            LIMIT ?"
        )
        .bind(&user.user_id())
        .bind(&status)
        .bind(limit)
        .fetch_all(&state.db_pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT * FROM conversion_jobs 
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ?"
        )
        .bind(&user.user_id())
        .bind(limit)
        .fetch_all(&state.db_pool)
        .await
    }
    .map_err(|e| {
        eprintln!("Failed to list conversion jobs: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "success": true,
        "jobs": jobs
    })))
}

/// GET /api/conversion/jobs/:id - Get single job status
async fn get_job(
    State(state): State<AppState>,
    user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let job: Option<ConversionJob> = sqlx::query_as(
        "SELECT * FROM conversion_jobs WHERE id = ? AND user_id = ?"
    )
    .bind(&job_id)
    .bind(&user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch conversion job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match job {
        Some(job) => Ok(Json(json!({
            "success": true,
            "job": job
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /api/conversion/jobs/:id/cancel - Cancel a running job
async fn cancel_job(
    State(state): State<AppState>,
    user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        "UPDATE conversion_jobs 
        SET status = 'cancelled'
        WHERE id = ? AND user_id = ? AND status IN ('pending', 'processing')"
    )
    .bind(&job_id)
    .bind(&user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to cancel conversion job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "success": true,
        "message": "Conversion job cancelled"
    })))
}

/// DELETE /api/conversion/jobs/:id - Delete a job
async fn delete_job(
    State(state): State<AppState>,
    user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Also delete output file from disk
    let result = sqlx::query(
        "DELETE FROM conversion_jobs WHERE id = ? AND user_id = ?"
    )
    .bind(&job_id)
    .bind(&user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to delete conversion job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "success": true,
        "message": "Conversion job deleted"
    })))
}

/// GET /api/conversion/formats - List supported format conversions
async fn list_formats() -> Result<impl IntoResponse, StatusCode> {
    // TODO: Detect installed tools and return only supported formats
    Ok(Json(json!({
        "success": true,
        "formats": {
            "images": {
                "png": ["jpg", "webp", "gif", "bmp"],
                "jpg": ["png", "webp", "gif"],
                "webp": ["png", "jpg"],
                "gif": ["png", "jpg", "mp4"],
                "heic": ["jpg", "png"]
            },
            "documents": {
                "pdf": ["docx", "txt"],
                "docx": ["pdf", "txt"],
                "txt": ["pdf"],
                "md": ["pdf", "html"],
                "odt": ["pdf", "docx"]
            },
            "videos": {
                "mp4": ["webm", "gif", "avi"],
                "webm": ["mp4"],
                "avi": ["mp4", "webm"],
                "mov": ["mp4", "webm"],
                "mkv": ["mp4"]
            },
            "audio": {
                "wav": ["mp3", "ogg"],
                "mp3": ["ogg", "wav"],
                "flac": ["mp3", "wav"],
                "m4a": ["mp3"],
                "ogg": ["mp3", "wav"]
            }
        }
    })))
}

/// GET /api/conversion/presets - List conversion presets
async fn list_presets(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let presets: Vec<ConversionPreset> = sqlx::query_as(
        "SELECT * FROM conversion_presets 
        WHERE is_system = 1 OR created_by = ?
        ORDER BY is_system DESC, name ASC"
    )
    .bind(&user.user_id())
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to list conversion presets: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "success": true,
        "presets": presets
    })))
}

/// POST /api/conversion/presets - Create custom preset
async fn create_preset(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreatePresetRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let preset_id = uuid::Uuid::new_v4().to_string();
    let options_str = req.options.to_string();

    sqlx::query(
        "INSERT INTO conversion_presets 
        (id, name, description, source_format, target_format, options, is_system, created_by)
        VALUES (?, ?, ?, ?, ?, ?, 0, ?)"
    )
    .bind(&preset_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.source_format)
    .bind(&req.target_format)
    .bind(&options_str)
    .bind(&user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create conversion preset: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let preset: ConversionPreset = sqlx::query_as(
        "SELECT * FROM conversion_presets WHERE id = ?"
    )
    .bind(&preset_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch created preset: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "success": true,
        "preset": preset
    })))
}

/// DELETE /api/conversion/presets/:id - Delete custom preset
async fn delete_preset(
    State(state): State<AppState>,
    user: UserInfo,
    Path(preset_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM conversion_presets 
        WHERE id = ? AND created_by = ? AND is_system = 0"
    )
    .bind(&preset_id)
    .bind(&user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to delete conversion preset: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "success": true,
        "message": "Preset deleted"
    })))
}
