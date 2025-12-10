/// File preview generation system
/// 
/// Supports multiple preview types:
/// - PDF: First page as image
/// - Video: Thumbnail at timestamp + metadata
/// - Documents: Convert to PDF then preview
/// - Code: Syntax highlighted HTML

use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

const PREVIEW_DIR: &str = "./data/.previews";

/// Preview metadata stored in database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PreviewMetadata {
    pub id: String,
    pub file_id: String,
    pub preview_type: String,
    pub preview_path: String,
    pub size_bytes: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<f64>,
    pub page_count: Option<i32>,
    pub generated_at: String,
}

/// Preview types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PreviewType {
    Thumbnail,
    PdfPage,
    VideoFrame,
    DocumentPdf,
    CodeHtml,
    TextPlain,
}

impl PreviewType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PreviewType::Thumbnail => "thumbnail",
            PreviewType::PdfPage => "pdf_page",
            PreviewType::VideoFrame => "video_frame",
            PreviewType::DocumentPdf => "document_pdf",
            PreviewType::CodeHtml => "code_html",
            PreviewType::TextPlain => "text_plain",
        }
    }
}

/// Video metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration_seconds: f64,
    pub width: i32,
    pub height: i32,
    pub codec: String,
    pub bitrate: Option<i64>,
    pub fps: Option<f64>,
}

/// PDF metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub page_count: i32,
    pub title: Option<String>,
    pub author: Option<String>,
    pub created_at: Option<String>,
}

// ==================== PREVIEW GENERATION ====================

/// Initialize preview directory
pub async fn init_preview_dir() -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all(PREVIEW_DIR).await
}

/// Get preview path
pub fn get_preview_path(file_id: &str, preview_type: PreviewType) -> PathBuf {
    let ext = match preview_type {
        PreviewType::Thumbnail | PreviewType::PdfPage | PreviewType::VideoFrame => "webp",
        PreviewType::DocumentPdf => "pdf",
        PreviewType::CodeHtml => "html",
        PreviewType::TextPlain => "txt",
    };
    PathBuf::from(PREVIEW_DIR).join(format!("{}_{}.{}", file_id, preview_type.as_str(), ext))
}

/// Generate PDF preview (first page as image)
pub async fn generate_pdf_preview(
    pdf_path: &Path,
    file_id: &str,
    page: u32,
) -> Result<PathBuf, PreviewError> {
    init_preview_dir().await
        .map_err(|e| PreviewError::IoError(e.to_string()))?;
    
    let output_path = get_preview_path(file_id, PreviewType::PdfPage);
    
    // Try pdftoppm first
    if is_tool_available("pdftoppm") {
        let temp_base = output_path.with_extension("");
        
        let output = Command::new("pdftoppm")
            .args(&[
                "-f", &page.to_string(),
                "-l", &page.to_string(),
                "-singlefile",
                "-png",
                "-scale-to", "1200",
                &pdf_path.to_string_lossy(),
                &temp_base.to_string_lossy(),
            ])
            .output()
            .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
        
        if output.status.success() {
            let png_path = temp_base.with_extension("png");
            if png_path.exists() {
                // Convert to WebP
                convert_to_webp(&png_path, &output_path)?;
                let _ = std::fs::remove_file(png_path);
                return Ok(output_path);
            }
        }
    }
    
    // Fallback to ImageMagick
    if is_tool_available("convert") {
        let output = Command::new("convert")
            .args(&[
                "-density", "200",
                &format!("{}[{}]", pdf_path.to_string_lossy(), page),
                "-resize", "1200x>",
                "-quality", "90",
                &output_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
        
        if output.status.success() {
            return Ok(output_path);
        }
    }
    
    Err(PreviewError::ToolNotInstalled("pdftoppm or ImageMagick".to_string()))
}

/// Generate video preview (frame at timestamp)
pub async fn generate_video_preview(
    video_path: &Path,
    file_id: &str,
    timestamp_secs: f64,
) -> Result<PathBuf, PreviewError> {
    init_preview_dir().await
        .map_err(|e| PreviewError::IoError(e.to_string()))?;
    
    if !is_tool_available("ffmpeg") {
        return Err(PreviewError::ToolNotInstalled("ffmpeg".to_string()));
    }
    
    let output_path = get_preview_path(file_id, PreviewType::VideoFrame);
    let timestamp = format_timestamp(timestamp_secs);
    
    let output = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-ss", &timestamp,
            "-i", &video_path.to_string_lossy(),
            "-vframes", "1",
            "-vf", "scale='min(1200,iw)':-1",
            "-q:v", "2",
            &output_path.to_string_lossy(),
        ])
        .output()
        .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
    
    if !output.status.success() {
        return Err(PreviewError::ProcessingError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    
    Ok(output_path)
}

/// Generate document preview (convert to PDF first)
pub async fn generate_document_preview(
    doc_path: &Path,
    file_id: &str,
) -> Result<PathBuf, PreviewError> {
    init_preview_dir().await
        .map_err(|e| PreviewError::IoError(e.to_string()))?;
    
    // Supported document formats
    let ext = doc_path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    
    if !["doc", "docx", "odt", "rtf", "xls", "xlsx", "ods", "ppt", "pptx", "odp"].contains(&ext.as_str()) {
        return Err(PreviewError::UnsupportedFormat);
    }
    
    if !is_tool_available("libreoffice") && !is_tool_available("soffice") {
        return Err(PreviewError::ToolNotInstalled("LibreOffice".to_string()));
    }
    
    let output_path = get_preview_path(file_id, PreviewType::DocumentPdf);
    let preview_dir = output_path.parent().unwrap_or(Path::new(PREVIEW_DIR));
    
    // Convert to PDF using LibreOffice
    let libreoffice = if is_tool_available("libreoffice") { "libreoffice" } else { "soffice" };
    
    let output = Command::new(libreoffice)
        .args(&[
            "--headless",
            "--convert-to", "pdf",
            "--outdir", &preview_dir.to_string_lossy(),
            &doc_path.to_string_lossy(),
        ])
        .output()
        .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
    
    if !output.status.success() {
        return Err(PreviewError::ProcessingError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    
    // LibreOffice outputs with same name but .pdf extension
    let expected_pdf = preview_dir.join(
        doc_path.file_stem().unwrap_or_default()
    ).with_extension("pdf");
    
    if expected_pdf.exists() {
        // Rename to our expected path
        std::fs::rename(&expected_pdf, &output_path)
            .map_err(|e| PreviewError::IoError(e.to_string()))?;
    }
    
    Ok(output_path)
}

/// Get video metadata using ffprobe
pub async fn get_video_metadata(video_path: &Path) -> Result<VideoMetadata, PreviewError> {
    if !is_tool_available("ffprobe") {
        return Err(PreviewError::ToolNotInstalled("ffprobe".to_string()));
    }
    
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            &video_path.to_string_lossy(),
        ])
        .output()
        .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
    
    if !output.status.success() {
        return Err(PreviewError::ProcessingError("ffprobe failed".to_string()));
    }
    
    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
    
    // Find video stream
    let video_stream = json["streams"].as_array()
        .and_then(|streams| streams.iter().find(|s| s["codec_type"] == "video"));
    
    let format = &json["format"];
    
    let duration = format["duration"].as_str()
        .and_then(|d| d.parse::<f64>().ok())
        .unwrap_or(0.0);
    
    let (width, height, codec, fps) = if let Some(stream) = video_stream {
        (
            stream["width"].as_i64().unwrap_or(0) as i32,
            stream["height"].as_i64().unwrap_or(0) as i32,
            stream["codec_name"].as_str().unwrap_or("unknown").to_string(),
            stream["r_frame_rate"].as_str()
                .and_then(|fps| {
                    let parts: Vec<&str> = fps.split('/').collect();
                    if parts.len() == 2 {
                        let num = parts[0].parse::<f64>().ok()?;
                        let den = parts[1].parse::<f64>().ok()?;
                        if den > 0.0 { Some(num / den) } else { None }
                    } else {
                        fps.parse::<f64>().ok()
                    }
                }),
        )
    } else {
        (0, 0, "unknown".to_string(), None)
    };
    
    let bitrate = format["bit_rate"].as_str()
        .and_then(|b| b.parse::<i64>().ok());
    
    Ok(VideoMetadata {
        duration_seconds: duration,
        width,
        height,
        codec,
        bitrate,
        fps,
    })
}

/// Get PDF metadata
pub async fn get_pdf_metadata(pdf_path: &Path) -> Result<PdfMetadata, PreviewError> {
    // Try pdfinfo first
    if is_tool_available("pdfinfo") {
        let output = Command::new("pdfinfo")
            .arg(&pdf_path.to_string_lossy().to_string())
            .output()
            .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            let page_count = stdout.lines()
                .find(|l| l.starts_with("Pages:"))
                .and_then(|l| l.split(':').last())
                .and_then(|s| s.trim().parse::<i32>().ok())
                .unwrap_or(1);
            
            let title = stdout.lines()
                .find(|l| l.starts_with("Title:"))
                .and_then(|l| l.split(':').last())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());
            
            let author = stdout.lines()
                .find(|l| l.starts_with("Author:"))
                .and_then(|l| l.split(':').last())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());
            
            return Ok(PdfMetadata {
                page_count,
                title,
                author,
                created_at: None,
            });
        }
    }
    
    // Fallback: just return page count 1
    Ok(PdfMetadata {
        page_count: 1,
        title: None,
        author: None,
        created_at: None,
    })
}

// ==================== DATABASE OPERATIONS ====================

/// Store preview metadata
pub async fn store_preview_metadata(
    pool: &SqlitePool,
    file_id: &str,
    preview_type: PreviewType,
    preview_path: &Path,
    width: Option<i32>,
    height: Option<i32>,
    duration: Option<f64>,
    page_count: Option<i32>,
) -> Result<PreviewMetadata, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    let size = tokio::fs::metadata(preview_path).await
        .map(|m| m.len() as i64)
        .unwrap_or(0);
    
    sqlx::query(
        "INSERT INTO preview_metadata 
         (id, file_id, preview_type, preview_path, size_bytes, width, height, duration_seconds, page_count, generated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(file_id)
    .bind(preview_type.as_str())
    .bind(&preview_path.to_string_lossy().to_string())
    .bind(size)
    .bind(width)
    .bind(height)
    .bind(duration)
    .bind(page_count)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM preview_metadata WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Get preview for a file
pub async fn get_preview(
    pool: &SqlitePool,
    file_id: &str,
    preview_type: PreviewType,
) -> Result<Option<PreviewMetadata>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM preview_metadata 
         WHERE file_id = ? AND preview_type = ? 
         ORDER BY generated_at DESC LIMIT 1"
    )
    .bind(file_id)
    .bind(preview_type.as_str())
    .fetch_optional(pool)
    .await
}

/// Delete previews for a file
pub async fn delete_previews(pool: &SqlitePool, file_id: &str) -> Result<u64, sqlx::Error> {
    // Get preview paths
    let previews: Vec<PreviewMetadata> = sqlx::query_as(
        "SELECT * FROM preview_metadata WHERE file_id = ?"
    )
    .bind(file_id)
    .fetch_all(pool)
    .await?;
    
    // Delete files
    for preview in &previews {
        let _ = tokio::fs::remove_file(&preview.preview_path).await;
    }
    
    // Delete from database
    let result = sqlx::query("DELETE FROM preview_metadata WHERE file_id = ?")
        .bind(file_id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected())
}

// ==================== HELPERS ====================

fn is_tool_available(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn format_timestamp(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

fn convert_to_webp(input: &Path, output: &Path) -> Result<(), PreviewError> {
    let img = image::open(input)
        .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
    img.save(output)
        .map_err(|e| PreviewError::ProcessingError(e.to_string()))?;
    Ok(())
}

// ==================== ERRORS ====================

#[derive(Debug, Clone)]
pub enum PreviewError {
    UnsupportedFormat,
    ToolNotInstalled(String),
    ProcessingError(String),
    IoError(String),
    FileNotFound,
}

impl std::fmt::Display for PreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreviewError::UnsupportedFormat => write!(f, "Unsupported file format"),
            PreviewError::ToolNotInstalled(tool) => write!(f, "Required tool not installed: {}", tool),
            PreviewError::ProcessingError(e) => write!(f, "Preview processing error: {}", e),
            PreviewError::IoError(e) => write!(f, "IO error: {}", e),
            PreviewError::FileNotFound => write!(f, "File not found"),
        }
    }
}

impl std::error::Error for PreviewError {}
