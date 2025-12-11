//! Thumbnail API endpoints
//! Generates and serves thumbnails for images, videos, and documents

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{auth::UserInfo, thumbnails::{self, ThumbnailSize}, AppState};

#[derive(Debug, Deserialize)]
pub struct ThumbnailQuery {
    /// Size: small (128), medium (256), large (512)
    #[serde(default = "default_size")]
    pub size: String,
    /// Force regeneration
    #[serde(default)]
    pub force: bool,
}

fn default_size() -> String {
    "medium".to_string()
}

#[derive(Debug, Serialize)]
pub struct ThumbnailInfo {
    pub path: String,
    pub size: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub cached: bool,
    pub generated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ThumbnailStatus {
    pub available_tools: thumbnails::AvailableTools,
    pub thumbnail_dir: String,
    pub supported_images: Vec<&'static str>,
    pub supported_videos: Vec<&'static str>,
}

/// Generate file ID from path (hash-based)
fn generate_file_id(path: &str) -> String {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Parse size string to ThumbnailSize enum
fn parse_size(size: &str) -> ThumbnailSize {
    match size.to_lowercase().as_str() {
        "small" | "sm" | "s" => ThumbnailSize::Small,
        "large" | "lg" | "l" => ThumbnailSize::Large,
        _ => ThumbnailSize::Medium,
    }
}

/// GET /api/thumbnails/{path}
/// Get thumbnail for a file
async fn get_thumbnail(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    Query(query): Query<ThumbnailQuery>,
    _user: UserInfo,
) -> Result<Response, StatusCode> {
    let size = parse_size(&query.size);

    // Resolve full path
    let full_path = std::path::Path::new("./data").join(&file_path);
    
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    // Check if file type supports thumbnails
    if !thumbnails::supports_thumbnail(&full_path) {
        return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    // Generate file ID from path
    let file_id = generate_file_id(&file_path);
    
    // Check if thumbnail exists (unless force regeneration)
    let thumb_path = thumbnails::get_thumbnail_path(&file_id, size);
    if !query.force && thumb_path.exists() {
        return serve_thumbnail(&thumb_path).await;
    }

    // Generate thumbnail
    match thumbnails::generate_thumbnail(&full_path, &file_id, size).await {
        Ok(thumb_path) => serve_thumbnail(&thumb_path).await,
        Err(e) => {
            tracing::error!("Thumbnail generation failed for {}: {}", file_path, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Serve thumbnail from path
async fn serve_thumbnail(thumb_path: &std::path::Path) -> Result<Response, StatusCode> {
    if !thumb_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Read thumbnail file
    let thumbnail_data = fs::read(thumb_path).await.map_err(|e| {
        tracing::error!("Failed to read thumbnail: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Determine content type from extension
    let content_type = match thumb_path.extension().and_then(|e| e.to_str()) {
        Some("webp") => "image/webp",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "image/webp",
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=86400") // Cache for 24h
        .header("X-Thumbnail-Cached", "true")
        .body(Body::from(thumbnail_data))
        .unwrap())
}

/// GET /api/thumbnails/{path}/info
/// Get thumbnail info without generating
async fn get_thumbnail_info(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    Query(query): Query<ThumbnailQuery>,
    _user: UserInfo,
) -> Result<Json<ThumbnailInfo>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&file_path);
    
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let size = parse_size(&query.size);
    let file_id = generate_file_id(&file_path);
    let thumb_path = thumbnails::get_thumbnail_path(&file_id, size);

    if thumb_path.exists() {
        let metadata = fs::metadata(&thumb_path).await.ok();
        let generated_at = metadata
            .and_then(|m| m.modified().ok())
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
        
        Ok(Json(ThumbnailInfo {
            path: file_path,
            size: query.size,
            width: size.pixels(),
            height: size.pixels(),
            format: "webp".to_string(),
            cached: true,
            generated_at,
        }))
    } else {
        // Thumbnail doesn't exist yet
        Ok(Json(ThumbnailInfo {
            path: file_path,
            size: query.size,
            width: 0,
            height: 0,
            format: "webp".to_string(),
            cached: false,
            generated_at: String::new(),
        }))
    }
}

/// DELETE /api/thumbnails/{path}
/// Delete thumbnail cache for a file
async fn delete_thumbnail(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let file_id = generate_file_id(&file_path);
    
    // Delete all size variants
    for size in [ThumbnailSize::Small, ThumbnailSize::Medium, ThumbnailSize::Large] {
        let thumb_path = thumbnails::get_thumbnail_path(&file_id, size);
        if thumb_path.exists() {
            let _ = fs::remove_file(&thumb_path).await;
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/thumbnails/cache/clear
/// Clear entire thumbnail cache (admin only)
async fn clear_thumbnail_cache(
    State(_state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Admin check
    if user.role.as_deref() != Some("admin") {
        return Err(StatusCode::FORBIDDEN);
    }

    let thumb_dir = std::path::Path::new("./data/.thumbnails");
    
    if thumb_dir.exists() {
        let mut count = 0;
        let mut entries = fs::read_dir(thumb_dir).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.path().is_file() {
                let _ = fs::remove_file(entry.path()).await;
                count += 1;
            }
        }
        
        Ok(Json(serde_json::json!({
            "success": true,
            "deleted": count,
            "message": format!("Cleared {} cached thumbnails", count)
        })))
    } else {
        Ok(Json(serde_json::json!({
            "success": true,
            "deleted": 0,
            "message": "Thumbnail cache was already empty"
        })))
    }
}

/// GET /api/thumbnails/status
/// Get thumbnail system status
async fn get_thumbnail_status(
    _user: UserInfo,
) -> Json<ThumbnailStatus> {
    Json(ThumbnailStatus {
        available_tools: thumbnails::check_available_tools(),
        thumbnail_dir: "./data/.thumbnails".to_string(),
        supported_images: vec!["jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "tiff"],
        supported_videos: vec!["mp4", "webm", "mov", "avi", "mkv"],
    })
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/thumbnails/status", get(get_thumbnail_status))
        .route("/thumbnails/cache/clear", axum::routing::post(clear_thumbnail_cache))
        .route("/thumbnails/info/{*path}", get(get_thumbnail_info))
        .route("/thumbnails/{*path}", get(get_thumbnail).delete(delete_thumbnail))
}
