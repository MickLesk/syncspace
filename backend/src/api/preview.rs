//! File Preview API endpoints
//! Generates previews for various file types (PDFs, videos, documents)

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

use crate::{auth::UserInfo, file_preview::PreviewType, AppState};

#[derive(Debug, Deserialize)]
pub struct PreviewQuery {
    /// Preview type: thumbnail, full, metadata
    #[serde(default = "default_type")]
    pub preview_type: String,
    /// Page number for PDFs
    #[serde(default = "default_page")]
    pub page: u32,
    /// Quality (1-100)
    #[serde(default = "default_quality")]
    pub quality: u32,
}

fn default_type() -> String {
    "thumbnail".to_string()
}
fn default_page() -> u32 {
    1
}
fn default_quality() -> u32 {
    80
}

#[derive(Debug, Serialize)]
pub struct PreviewMetadata {
    pub file_path: String,
    pub file_type: String,
    pub preview_available: bool,
    pub preview_type: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct VideoPreviewInfo {
    pub duration_seconds: f64,
    pub width: u32,
    pub height: u32,
    pub codec: String,
    pub bitrate: u64,
    pub fps: f32,
    pub thumbnail_url: String,
}

#[derive(Debug, Serialize)]
pub struct PdfPreviewInfo {
    pub page_count: u32,
    pub title: Option<String>,
    pub author: Option<String>,
    pub page_previews: Vec<String>,
}

/// Get preview for a file
async fn get_preview(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    Query(query): Query<PreviewQuery>,
    _user: UserInfo,
) -> Result<Response, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&file_path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Detect file type and generate preview
    let preview_type = match query.preview_type.as_str() {
        "full" => PreviewType::Full,
        "metadata" => PreviewType::Metadata,
        _ => PreviewType::Thumbnail,
    };

    let preview_result = crate::file_preview::generate_preview(&full_path, preview_type)
        .await
        .map_err(|e| {
            tracing::error!("Preview generation failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // If metadata requested, return JSON
    if matches!(preview_type, PreviewType::Metadata) {
        return Ok(Json(preview_result.metadata).into_response());
    }

    // Read preview file
    let preview_path = preview_result.path;
    if !std::path::Path::new(&preview_path).exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let preview_data = fs::read(&preview_path).await.map_err(|e| {
        tracing::error!("Failed to read preview: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Determine content type
    let content_type = match preview_result.format.as_str() {
        "webp" => "image/webp",
        "png" => "image/png",
        "pdf" => "application/pdf",
        _ => "image/jpeg",
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(Body::from(preview_data))
        .unwrap())
}

/// Get file metadata for preview
async fn get_preview_metadata(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    _user: UserInfo,
) -> Result<Json<PreviewMetadata>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&file_path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Get file extension
    let extension = full_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Determine preview availability and type
    let (preview_available, preview_type, metadata) = match extension.as_str() {
        // Images
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" => {
            (true, "image", serde_json::json!({"type": "image"}))
        }
        // Videos
        "mp4" | "webm" | "mov" | "avi" | "mkv" => {
            let video_meta = crate::file_preview::get_video_metadata(&full_path)
                .await
                .unwrap_or_default();
            (true, "video", serde_json::to_value(video_meta).unwrap_or_default())
        }
        // PDFs
        "pdf" => {
            let pdf_meta = crate::file_preview::get_pdf_metadata(&full_path)
                .await
                .unwrap_or_default();
            (true, "pdf", serde_json::to_value(pdf_meta).unwrap_or_default())
        }
        // Documents
        "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp" => {
            (true, "document", serde_json::json!({"type": "office_document"}))
        }
        // Text
        "txt" | "md" | "json" | "xml" | "csv" | "log" => {
            (true, "text", serde_json::json!({"type": "text"}))
        }
        // Code
        "rs" | "js" | "ts" | "py" | "java" | "c" | "cpp" | "go" | "rb" | "php" => {
            (true, "code", serde_json::json!({"type": "code", "language": extension}))
        }
        // No preview
        _ => (false, "none", serde_json::json!({})),
    };

    Ok(Json(PreviewMetadata {
        file_path,
        file_type: extension,
        preview_available,
        preview_type: preview_type.to_string(),
        metadata,
    }))
}

/// Get video preview with timeline thumbnails
async fn get_video_preview(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    _user: UserInfo,
) -> Result<Json<VideoPreviewInfo>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&file_path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let video_meta = crate::file_preview::get_video_metadata(&full_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(VideoPreviewInfo {
        duration_seconds: video_meta.duration_seconds,
        width: video_meta.width,
        height: video_meta.height,
        codec: video_meta.codec,
        bitrate: video_meta.bitrate,
        fps: video_meta.fps,
        thumbnail_url: format!("/api/thumbnails/{}", file_path),
    }))
}

/// Get PDF preview with page thumbnails
async fn get_pdf_preview(
    State(_state): State<AppState>,
    Path(file_path): Path<String>,
    Query(query): Query<PreviewQuery>,
    _user: UserInfo,
) -> Result<Json<PdfPreviewInfo>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&file_path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let pdf_meta = crate::file_preview::get_pdf_metadata(&full_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Generate page preview URLs
    let page_previews: Vec<String> = (1..=pdf_meta.page_count.min(10))
        .map(|p| format!("/api/preview/{}/page/{}?quality={}", file_path, p, query.quality))
        .collect();

    Ok(Json(PdfPreviewInfo {
        page_count: pdf_meta.page_count,
        title: pdf_meta.title,
        author: pdf_meta.author,
        page_previews,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/preview/{*path}", get(get_preview))
        .route("/preview/{*path}/metadata", get(get_preview_metadata))
        .route("/preview/{*path}/video", get(get_video_preview))
        .route("/preview/{*path}/pdf", get(get_pdf_preview))
}
