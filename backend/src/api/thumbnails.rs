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

use crate::{auth::UserInfo, thumbnails::ThumbnailSize, AppState};

#[derive(Debug, Deserialize)]
pub struct ThumbnailQuery {
    /// Size: small (64), medium (256), large (512)
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

/// Get thumbnail for a file
async fn get_thumbnail(
    State(state): State<AppState>,
    Path(file_path): Path<String>,
    Query(query): Query<ThumbnailQuery>,
    _user: UserInfo,
) -> Result<Response, StatusCode> {
    let size = match query.size.as_str() {
        "small" => ThumbnailSize::Small,
        "large" => ThumbnailSize::Large,
        _ => ThumbnailSize::Medium,
    };

    // Resolve full path
    let full_path = std::path::Path::new("./data").join(&file_path);
    
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Generate thumbnail
    let thumbnail_result = crate::thumbnails::generate_thumbnail(&full_path, size)
        .await
        .map_err(|e| {
            tracing::error!("Thumbnail generation failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Check if thumbnail was generated
    let thumbnail_path = thumbnail_result.path;
    
    if !std::path::Path::new(&thumbnail_path).exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Read thumbnail file
    let thumbnail_data = fs::read(&thumbnail_path).await.map_err(|e| {
        tracing::error!("Failed to read thumbnail: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Determine content type
    let content_type = if thumbnail_path.ends_with(".webp") {
        "image/webp"
    } else if thumbnail_path.ends_with(".png") {
        "image/png"
    } else {
        "image/jpeg"
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=86400") // Cache for 24h
        .body(Body::from(thumbnail_data))
        .unwrap())
}

/// Get thumbnail info without generating
async fn get_thumbnail_info(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    _user: UserInfo,
) -> Result<Json<ThumbnailInfo>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&file_path);
    
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Check for existing thumbnail
    let thumb_dir = std::path::Path::new("./data/.thumbnails");
    let file_hash = format!("{:x}", md5::compute(file_path.as_bytes()));
    let thumb_path = thumb_dir.join(format!("{}_medium.webp", file_hash));

    if thumb_path.exists() {
        Ok(Json(ThumbnailInfo {
            path: file_path,
            size: "medium".to_string(),
            width: 256,
            height: 256,
            format: "webp".to_string(),
            cached: true,
            generated_at: chrono::Utc::now().to_rfc3339(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Delete thumbnail cache for a file
async fn delete_thumbnail(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let thumb_dir = std::path::Path::new("./data/.thumbnails");
    let file_hash = format!("{:x}", md5::compute(file_path.as_bytes()));
    
    // Delete all sizes
    for size in ["small", "medium", "large"] {
        let thumb_path = thumb_dir.join(format!("{}_{}.webp", file_hash, size));
        if thumb_path.exists() {
            let _ = fs::remove_file(&thumb_path).await;
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Clear entire thumbnail cache (admin only)
async fn clear_thumbnail_cache(
    State(_state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Admin check
    if user.role != "admin" {
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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/thumbnails/{*path}", get(get_thumbnail).delete(delete_thumbnail))
        .route("/thumbnails/{*path}/info", get(get_thumbnail_info))
        .route("/thumbnails/cache/clear", axum::routing::post(clear_thumbnail_cache))
}
