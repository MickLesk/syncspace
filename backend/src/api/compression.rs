//! Compression API Routes
//! Provides endpoints for compressing/decompressing files and managing compression settings

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::fs;
use uuid::Uuid;

use crate::AppState;
use crate::auth::UserInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CompressionAlgorithm {
    Gzip,
    Bzip2,
    Lz4,
    Zstd,
    Lzma,
}

impl CompressionAlgorithm {
    pub fn extension(&self) -> &str {
        match self {
            CompressionAlgorithm::Gzip => "gz",
            CompressionAlgorithm::Bzip2 => "bz2",
            CompressionAlgorithm::Lz4 => "lz4",
            CompressionAlgorithm::Zstd => "zst",
            CompressionAlgorithm::Lzma => "xz",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressRequest {
    pub files: Vec<String>,
    pub algorithm: CompressionAlgorithm,
    #[serde(default = "default_compression_level")]
    pub level: u32, // 1-9 typically
    pub destination: Option<String>,
    #[serde(default)]
    pub delete_originals: bool,
}

fn default_compression_level() -> u32 {
    6
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecompressRequest {
    pub files: Vec<String>,
    pub destination: Option<String>,
    #[serde(default)]
    pub delete_originals: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: u64,
    pub compressed_size: u64,
    pub ratio: f64,
    pub algorithm: String,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionJobResponse {
    pub job_id: String,
    pub status: String,
    pub files_count: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeQuery {
    pub path: String,
}

/// Compress files
async fn compress_files(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<CompressRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    if req.files.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Validate compression level
    let level = req.level.min(9).max(1);
    
    // Create background job
    let payload = serde_json::json!({
        "files": req.files,
        "algorithm": req.algorithm,
        "level": level,
        "destination": req.destination,
        "delete_originals": req.delete_originals,
        "user_id": user_info.id,
    }).to_string();
    
    sqlx::query(
        "INSERT INTO background_jobs (id, job_type, status, payload, scheduled_at, created_at, updated_at)
         VALUES (?, 'compress_files', 'pending', ?, ?, ?, ?)"
    )
    .bind(&job_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create compression job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(CompressionJobResponse {
        job_id,
        status: "pending".to_string(),
        files_count: req.files.len(),
        message: format!("Compression job queued for {} files", req.files.len()),
    }))
}

/// Decompress files
async fn decompress_files(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<DecompressRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    if req.files.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Create background job
    let payload = serde_json::json!({
        "files": req.files,
        "destination": req.destination,
        "delete_originals": req.delete_originals,
        "user_id": user_info.id,
    }).to_string();
    
    sqlx::query(
        "INSERT INTO background_jobs (id, job_type, status, payload, scheduled_at, created_at, updated_at)
         VALUES (?, 'decompress_files', 'pending', ?, ?, ?, ?)"
    )
    .bind(&job_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create decompression job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(CompressionJobResponse {
        job_id,
        status: "pending".to_string(),
        files_count: req.files.len(),
        message: format!("Decompression job queued for {} files", req.files.len()),
    }))
}

/// Analyze compression potential of a file/directory
async fn analyze_compression(
    State(_state): State<AppState>,
    _user_info: UserInfo,
    Query(query): Query<AnalyzeQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let data_dir = std::env::current_dir()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .join("data");
    
    let target_path = data_dir.join(&query.path);
    
    if !target_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let metadata = fs::metadata(&target_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Analyze file type and estimate compression ratio
    let extension = target_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // Estimate compression ratio based on file type
    let (estimated_ratio, recommendation) = match extension.as_str() {
        // Already compressed formats
        "zip" | "gz" | "bz2" | "xz" | "7z" | "rar" | "lz4" | "zst" => {
            (1.0, "Already compressed - no benefit expected")
        }
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "mp3" | "mp4" | "mkv" | "avi" | "mov" => {
            (0.98, "Media file - minimal compression benefit")
        }
        // Highly compressible formats
        "txt" | "log" | "csv" | "json" | "xml" | "html" | "css" | "js" | "md" => {
            (0.3, "Text file - high compression potential")
        }
        "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => {
            (0.9, "Office document - already compressed internally")
        }
        // Moderately compressible
        "pdf" => (0.85, "PDF - some compression possible"),
        "svg" => (0.4, "SVG - good compression potential"),
        _ => (0.7, "Unknown type - moderate compression expected"),
    };
    
    let original_size = metadata.len();
    let estimated_compressed = (original_size as f64 * estimated_ratio) as u64;
    let savings = original_size.saturating_sub(estimated_compressed);
    
    Ok(Json(serde_json::json!({
        "path": query.path,
        "original_size": original_size,
        "estimated_compressed_size": estimated_compressed,
        "estimated_ratio": estimated_ratio,
        "estimated_savings_bytes": savings,
        "estimated_savings_percent": (1.0 - estimated_ratio) * 100.0,
        "recommendation": recommendation,
        "suggested_algorithm": if estimated_ratio < 0.5 { "zstd" } else { "gzip" },
        "is_already_compressed": estimated_ratio >= 0.95,
    })))
}

/// Get available compression algorithms
async fn get_algorithms(
    _user_info: UserInfo,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "algorithms": [
            {
                "id": "gzip",
                "name": "Gzip",
                "extension": ".gz",
                "speed": "fast",
                "ratio": "good",
                "levels": [1, 2, 3, 4, 5, 6, 7, 8, 9],
                "default_level": 6,
                "description": "Standard compression, widely compatible"
            },
            {
                "id": "bzip2",
                "name": "Bzip2",
                "extension": ".bz2",
                "speed": "slow",
                "ratio": "better",
                "levels": [1, 2, 3, 4, 5, 6, 7, 8, 9],
                "default_level": 6,
                "description": "Better compression ratio, slower speed"
            },
            {
                "id": "lz4",
                "name": "LZ4",
                "extension": ".lz4",
                "speed": "very_fast",
                "ratio": "moderate",
                "levels": [1, 2, 3, 4, 5, 6, 7, 8, 9],
                "default_level": 1,
                "description": "Extremely fast, good for large files"
            },
            {
                "id": "zstd",
                "name": "Zstandard",
                "extension": ".zst",
                "speed": "fast",
                "ratio": "excellent",
                "levels": [1, 2, 3, 4, 5, 6, 7, 8, 9],
                "default_level": 3,
                "description": "Modern algorithm, best balance of speed and ratio"
            },
            {
                "id": "lzma",
                "name": "LZMA/XZ",
                "extension": ".xz",
                "speed": "very_slow",
                "ratio": "best",
                "levels": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                "default_level": 6,
                "description": "Best compression ratio, very slow"
            }
        ]
    }))
}

/// Get compression statistics for user
async fn get_compression_stats(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Get completed compression jobs for this user
    let jobs: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT job_type, status, result FROM background_jobs 
         WHERE job_type IN ('compress_files', 'decompress_files')
         AND payload LIKE ?
         ORDER BY completed_at DESC
         LIMIT 100"
    )
    .bind(format!("%\"user_id\":\"{}\"%%", user_info.id))
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();
    
    let completed_count = jobs.iter().filter(|(_, s, _)| s == "completed").count();
    let failed_count = jobs.iter().filter(|(_, s, _)| s == "failed").count();
    
    Ok(Json(serde_json::json!({
        "total_jobs": jobs.len(),
        "completed": completed_count,
        "failed": failed_count,
        "total_bytes_saved": 0, // Would aggregate from job results
        "recent_jobs": jobs.iter().take(10).map(|(t, s, _)| {
            serde_json::json!({
                "type": t,
                "status": s,
            })
        }).collect::<Vec<_>>(),
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/compress", post(compress_files))
        .route("/decompress", post(decompress_files))
        .route("/analyze", get(analyze_compression))
        .route("/algorithms", get(get_algorithms))
        .route("/stats", get(get_compression_stats))
}
