/// File preview generation system
/// Supports PDF, images, videos, documents
use std::process::Command;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PreviewMetadata {
    pub id: String,
    pub file_id: String,
    pub preview_type: String, // "thumbnail", "pdf", "video", "document"
    pub preview_path: String,
    pub size_bytes: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f64>, // For videos
    pub page_count: Option<i32>, // For PDFs/documents
    pub generated_at: String,
}

/// Generate PDF preview (first page as image)
pub async fn generate_pdf_preview(
    pdf_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Use pdftoppm (from poppler-utils)
    let output = Command::new("pdftoppm")
        .args(&[
            "-singlefile",
            "-png",
            "-scale-to",
            "800",
            pdf_path,
            output_path.trim_end_matches(".png"),
        ])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("PDF preview generation failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(())
}

/// Generate video thumbnail using ffmpeg
pub async fn generate_video_thumbnail(
    video_path: &str,
    output_path: &str,
    timestamp: &str, // e.g., "00:00:05"
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            video_path,
            "-ss",
            timestamp,
            "-vframes",
            "1",
            "-vf",
            "scale=800:-1",
            "-y",
            output_path,
        ])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Video thumbnail generation failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(())
}

/// Generate document preview using LibreOffice headless
pub async fn generate_document_preview(
    doc_path: &str,
    output_dir: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("libreoffice")
        .args(&[
            "--headless",
            "--convert-to",
            "pdf",
            "--outdir",
            output_dir,
            doc_path,
        ])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Document preview generation failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    // Return PDF path
    let pdf_name = std::path::Path::new(doc_path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string() + ".pdf";
    Ok(format!("{}/{}", output_dir, pdf_name))
}

/// Generate code syntax highlight preview as HTML
pub async fn generate_code_preview(
    code_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Use pygmentize for syntax highlighting
    let output = Command::new("pygmentize")
        .args(&[
            "-f",
            "html",
            "-O",
            "full,style=monokai",
            "-o",
            output_path,
            code_path,
        ])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Code preview generation failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(())
}

/// Store preview metadata in database
pub async fn store_preview_metadata(
    pool: &SqlitePool,
    file_id: &str,
    preview_type: &str,
    preview_path: &str,
    width: Option<i32>,
    height: Option<i32>,
) -> Result<PreviewMetadata, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    // Get file size
    let size = tokio::fs::metadata(preview_path)
        .await
        .map(|m| m.len() as i64)
        .unwrap_or(0);
    
    sqlx::query(
        "INSERT INTO preview_metadata (id, file_id, preview_type, preview_path, size_bytes, width, height, generated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(file_id)
    .bind(preview_type)
    .bind(preview_path)
    .bind(size)
    .bind(width)
    .bind(height)
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
    preview_type: &str,
) -> Result<Option<PreviewMetadata>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM preview_metadata WHERE file_id = ? AND preview_type = ? ORDER BY generated_at DESC LIMIT 1"
    )
    .bind(file_id)
    .bind(preview_type)
    .fetch_optional(pool)
    .await
}
