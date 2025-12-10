//! Archive Management API Routes
//! Provides endpoints for creating, extracting, and managing archives (zip, tar.gz, etc.)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::Path as StdPath;
use tokio::fs;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::AppState;
use crate::auth::UserInfo;

// Archive types supported
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArchiveFormat {
    Zip,
    TarGz,
    Tar,
    #[serde(rename = "7z")]
    SevenZ,
}

impl ArchiveFormat {
    pub fn extension(&self) -> &str {
        match self {
            ArchiveFormat::Zip => "zip",
            ArchiveFormat::TarGz => "tar.gz",
            ArchiveFormat::Tar => "tar",
            ArchiveFormat::SevenZ => "7z",
        }
    }

    pub fn from_path(path: &str) -> Option<Self> {
        let lower = path.to_lowercase();
        if lower.ends_with(".zip") {
            Some(ArchiveFormat::Zip)
        } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
            Some(ArchiveFormat::TarGz)
        } else if lower.ends_with(".tar") {
            Some(ArchiveFormat::Tar)
        } else if lower.ends_with(".7z") {
            Some(ArchiveFormat::SevenZ)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArchiveRequest {
    pub files: Vec<String>,
    pub archive_name: String,
    pub format: ArchiveFormat,
    #[serde(default)]
    pub compression_level: Option<u32>, // 0-9 for most formats
    #[serde(default)]
    pub password: Option<String>,
    pub destination: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractArchiveRequest {
    pub archive_path: String,
    pub destination: Option<String>,
    pub password: Option<String>,
    #[serde(default)]
    pub flatten: bool, // Extract without directory structure
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveInfo {
    pub path: String,
    pub format: String,
    pub size_bytes: u64,
    pub compressed_size: u64,
    pub file_count: usize,
    pub entries: Vec<ArchiveEntry>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub compressed_size: u64,
    pub is_directory: bool,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveJobResponse {
    pub job_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ListArchivesQuery {
    pub path: Option<String>,
}

/// Create a new archive from selected files
async fn create_archive(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<CreateArchiveRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Validate input
    if req.files.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Determine destination
    let destination = req.destination.unwrap_or_else(|| {
        if let Some(first_file) = req.files.first() {
            StdPath::new(first_file)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| ".".to_string())
        } else {
            ".".to_string()
        }
    });
    
    // Build archive filename
    let archive_filename = if req.archive_name.contains('.') {
        req.archive_name.clone()
    } else {
        format!("{}.{}", req.archive_name, req.format.extension())
    };
    
    // Create background job for archive creation
    let payload = serde_json::json!({
        "files": req.files,
        "archive_name": archive_filename,
        "format": req.format,
        "destination": destination,
        "compression_level": req.compression_level,
        "password": req.password,
        "user_id": user_info.id,
    }).to_string();
    
    sqlx::query(
        "INSERT INTO background_jobs (id, job_type, status, payload, scheduled_at, created_at, updated_at)
         VALUES (?, 'create_archive', 'pending', ?, ?, ?, ?)"
    )
    .bind(&job_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create archive job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(ArchiveJobResponse {
        job_id,
        status: "pending".to_string(),
        message: "Archive creation queued".to_string(),
    }))
}

/// Extract an archive
async fn extract_archive(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<ExtractArchiveRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Determine destination
    let destination = req.destination.unwrap_or_else(|| {
        StdPath::new(&req.archive_path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string())
    });
    
    // Create background job for extraction
    let payload = serde_json::json!({
        "archive_path": req.archive_path,
        "destination": destination,
        "password": req.password,
        "flatten": req.flatten,
        "user_id": user_info.id,
    }).to_string();
    
    sqlx::query(
        "INSERT INTO background_jobs (id, job_type, status, payload, scheduled_at, created_at, updated_at)
         VALUES (?, 'extract_archive', 'pending', ?, ?, ?, ?)"
    )
    .bind(&job_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create extract job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(ArchiveJobResponse {
        job_id,
        status: "pending".to_string(),
        message: "Archive extraction queued".to_string(),
    }))
}

/// List contents of an archive without extracting
async fn list_archive_contents(
    State(_state): State<AppState>,
    _user_info: UserInfo,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Path is already URL-decoded by axum
    let decoded_path = path;
    
    let data_dir = std::env::current_dir()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .join("data");
    
    let full_path = data_dir.join(&decoded_path);
    
    // Validate path exists and is an archive
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let format = ArchiveFormat::from_path(&decoded_path)
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // Get file metadata
    let metadata = fs::metadata(&full_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // For now, return basic info without reading archive contents
    // Full implementation would use zip/tar crates to list contents
    let info = ArchiveInfo {
        path: decoded_path,
        format: format.extension().to_string(),
        size_bytes: metadata.len(),
        compressed_size: metadata.len(),
        file_count: 0, // Would be populated by archive reading
        entries: vec![], // Would be populated by archive reading
        created_at: metadata.created().ok().map(|t| DateTime::from(t)),
    };
    
    Ok(Json(info))
}

/// Get supported archive formats
async fn get_supported_formats(
    _user_info: UserInfo,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "formats": [
            {
                "id": "zip",
                "name": "ZIP Archive",
                "extension": ".zip",
                "supports_password": true,
                "supports_compression_level": true,
            },
            {
                "id": "tar.gz",
                "name": "Gzipped Tar Archive",
                "extension": ".tar.gz",
                "supports_password": false,
                "supports_compression_level": true,
            },
            {
                "id": "tar",
                "name": "Tar Archive",
                "extension": ".tar",
                "supports_password": false,
                "supports_compression_level": false,
            },
            {
                "id": "7z",
                "name": "7-Zip Archive",
                "extension": ".7z",
                "supports_password": true,
                "supports_compression_level": true,
            }
        ]
    }))
}

/// List archives in a directory
async fn list_archives(
    State(_state): State<AppState>,
    _user_info: UserInfo,
    Query(query): Query<ListArchivesQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let data_dir = std::env::current_dir()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .join("data");
    
    let search_path = if let Some(path) = &query.path {
        data_dir.join(path)
    } else {
        data_dir.clone()
    };
    
    if !search_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let mut archives = Vec::new();
    let mut read_dir = fs::read_dir(&search_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    while let Some(entry) = read_dir.next_entry().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();
        
        if let Some(format) = ArchiveFormat::from_path(&path_str) {
            if let Ok(metadata) = entry.metadata().await {
                archives.push(serde_json::json!({
                    "name": entry.file_name().to_string_lossy(),
                    "path": path.strip_prefix(&data_dir)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or(path_str),
                    "format": format.extension(),
                    "size": metadata.len(),
                    "modified": metadata.modified().ok().map(|t| DateTime::<Utc>::from(t)),
                }));
            }
        }
    }
    
    Ok(Json(serde_json::json!({
        "archives": archives,
        "total": archives.len(),
    })))
}

/// Delete an archive
async fn delete_archive(
    State(_state): State<AppState>,
    _user_info: UserInfo,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Path is already URL-decoded by axum
    let decoded_path = path;
    
    let data_dir = std::env::current_dir()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .join("data");
    
    let full_path = data_dir.join(&decoded_path);
    
    // Validate it's an archive
    if ArchiveFormat::from_path(&decoded_path).is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    fs::remove_file(&full_path)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete archive: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Archive deleted"
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_archives))
        .route("/create", post(create_archive))
        .route("/extract", post(extract_archive))
        .route("/formats", get(get_supported_formats))
        .route("/{path:.*}", get(list_archive_contents))
        .route("/{path:.*}", delete(delete_archive))
}
