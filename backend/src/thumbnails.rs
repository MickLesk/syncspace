/// Thumbnail generation for images and videos
/// 
/// Provides automatic thumbnail generation:
/// - Image thumbnails (jpg, png, gif, webp, bmp)
/// - Video thumbnails (mp4, webm, mov via ffmpeg)
/// - PDF first page preview
/// - Caching in filesystem

use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;
use serde::{Deserialize, Serialize};

const THUMBNAIL_DIR: &str = "./data/.thumbnails";
const THUMBNAIL_SIZE_SMALL: u32 = 128;
const THUMBNAIL_SIZE_MEDIUM: u32 = 256;
const THUMBNAIL_SIZE_LARGE: u32 = 512;

/// Supported image formats
const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "tiff", "tif"];

/// Supported video formats
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "webm", "mov", "avi", "mkv", "wmv", "flv"];

/// Thumbnail size variants
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThumbnailSize {
    Small,  // 128x128
    Medium, // 256x256
    Large,  // 512x512
}

impl ThumbnailSize {
    pub fn pixels(&self) -> u32 {
        match self {
            ThumbnailSize::Small => THUMBNAIL_SIZE_SMALL,
            ThumbnailSize::Medium => THUMBNAIL_SIZE_MEDIUM,
            ThumbnailSize::Large => THUMBNAIL_SIZE_LARGE,
        }
    }
    
    pub fn suffix(&self) -> &'static str {
        match self {
            ThumbnailSize::Small => "sm",
            ThumbnailSize::Medium => "md",
            ThumbnailSize::Large => "lg",
        }
    }
}

/// Thumbnail metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailInfo {
    pub file_id: String,
    pub size: ThumbnailSize,
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Check if file is an image based on extension
pub fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Check if file is a video based on extension
pub fn is_video(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Check if file supports thumbnail generation
pub fn supports_thumbnail(path: &Path) -> bool {
    is_image(path) || is_video(path) || is_pdf(path)
}

/// Check if file is a PDF
pub fn is_pdf(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase() == "pdf")
        .unwrap_or(false)
}

/// Generate thumbnail path from file ID and size
pub fn get_thumbnail_path(file_id: &str, size: ThumbnailSize) -> PathBuf {
    PathBuf::from(THUMBNAIL_DIR).join(format!("{}_{}.webp", file_id, size.suffix()))
}

/// Check if thumbnail exists
pub async fn thumbnail_exists(file_id: &str, size: ThumbnailSize) -> bool {
    let path = get_thumbnail_path(file_id, size);
    path.exists()
}

/// Initialize thumbnail directory
pub async fn init_thumbnail_dir() -> Result<(), std::io::Error> {
    fs::create_dir_all(THUMBNAIL_DIR).await
}

/// Generate thumbnail for a file
pub async fn generate_thumbnail(
    source_path: &Path,
    file_id: &str,
    size: ThumbnailSize,
) -> Result<PathBuf, ThumbnailError> {
    // Ensure thumbnail directory exists
    init_thumbnail_dir().await
        .map_err(|e| ThumbnailError::IoError(e.to_string()))?;
    
    let thumb_path = get_thumbnail_path(file_id, size);
    
    // Check if already exists
    if thumb_path.exists() {
        return Ok(thumb_path);
    }
    
    if is_image(source_path) {
        generate_image_thumbnail(source_path, &thumb_path, size).await
    } else if is_video(source_path) {
        generate_video_thumbnail(source_path, &thumb_path, size).await
    } else if is_pdf(source_path) {
        generate_pdf_thumbnail(source_path, &thumb_path, size).await
    } else {
        Err(ThumbnailError::UnsupportedFormat)
    }
}

/// Generate thumbnail for image using image crate (pure Rust)
async fn generate_image_thumbnail(
    source_path: &Path,
    thumb_path: &Path,
    size: ThumbnailSize,
) -> Result<PathBuf, ThumbnailError> {
    let source = source_path.to_path_buf();
    let dest = thumb_path.to_path_buf();
    let max_size = size.pixels();
    
    // Run in blocking task to avoid blocking async runtime
    tokio::task::spawn_blocking(move || {
        let img = image::open(&source)
            .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
        
        // Resize maintaining aspect ratio
        let thumbnail = img.thumbnail(max_size, max_size);
        
        // Save as WebP
        thumbnail.save(&dest)
            .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
        
        Ok(dest)
    })
    .await
    .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?
}

/// Generate thumbnail for video using ffmpeg
async fn generate_video_thumbnail(
    source_path: &Path,
    thumb_path: &Path,
    size: ThumbnailSize,
) -> Result<PathBuf, ThumbnailError> {
    // Check if ffmpeg is available
    if !is_ffmpeg_available() {
        return Err(ThumbnailError::ToolNotInstalled("ffmpeg".to_string()));
    }
    
    let max_size = size.pixels();
    
    // Extract frame at 5 seconds or 10% of video
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", &source_path.to_string_lossy(),
            "-ss", "00:00:05",
            "-vframes", "1",
            "-vf", &format!("scale='min({},iw)':'-1'", max_size),
            "-y",
            &thumb_path.to_string_lossy(),
        ])
        .output()
        .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
    
    if !output.status.success() {
        // Try again at 0 seconds (for very short videos)
        let output = Command::new("ffmpeg")
            .args(&[
                "-i", &source_path.to_string_lossy(),
                "-ss", "00:00:00",
                "-vframes", "1",
                "-vf", &format!("scale='min({},iw)':'-1'", max_size),
                "-y",
                &thumb_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
        
        if !output.status.success() {
            return Err(ThumbnailError::ProcessingError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
    }
    
    Ok(thumb_path.to_path_buf())
}

/// Generate thumbnail for PDF using pdftoppm or ImageMagick
async fn generate_pdf_thumbnail(
    source_path: &Path,
    thumb_path: &Path,
    size: ThumbnailSize,
) -> Result<PathBuf, ThumbnailError> {
    let max_size = size.pixels();
    
    // Try pdftoppm first (from poppler-utils)
    if is_pdftoppm_available() {
        let temp_output = thumb_path.with_extension(""); // pdftoppm adds extension
        
        let output = Command::new("pdftoppm")
            .args(&[
                "-singlefile",
                "-png",
                "-scale-to", &max_size.to_string(),
                &source_path.to_string_lossy(),
                &temp_output.to_string_lossy(),
            ])
            .output()
            .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
        
        if output.status.success() {
            let png_path = temp_output.with_extension("png");
            if png_path.exists() {
                // Convert to WebP using image crate
                let img = image::open(&png_path)
                    .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
                img.save(thumb_path)
                    .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
                let _ = std::fs::remove_file(png_path);
                return Ok(thumb_path.to_path_buf());
            }
        }
    }
    
    // Fall back to ImageMagick
    if is_imagemagick_available() {
        let output = Command::new("convert")
            .args(&[
                "-density", "150",
                &format!("{}[0]", source_path.to_string_lossy()), // First page only
                "-thumbnail", &format!("{}x{}>", max_size, max_size),
                "-quality", "85",
                &thumb_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| ThumbnailError::ProcessingError(e.to_string()))?;
        
        if output.status.success() {
            return Ok(thumb_path.to_path_buf());
        }
    }
    
    Err(ThumbnailError::ToolNotInstalled("pdftoppm or ImageMagick".to_string()))
}

/// Generate all size variants of a thumbnail
pub async fn generate_all_thumbnails(
    source_path: &Path,
    file_id: &str,
) -> Result<Vec<PathBuf>, ThumbnailError> {
    let mut results = Vec::new();
    
    for size in [ThumbnailSize::Small, ThumbnailSize::Medium, ThumbnailSize::Large] {
        match generate_thumbnail(source_path, file_id, size).await {
            Ok(path) => results.push(path),
            Err(e) => {
                tracing::warn!("Failed to generate {:?} thumbnail: {}", size, e);
            }
        }
    }
    
    if results.is_empty() {
        Err(ThumbnailError::ProcessingError("All thumbnail generations failed".to_string()))
    } else {
        Ok(results)
    }
}

/// Delete all thumbnails for a file
pub async fn delete_thumbnails(file_id: &str) -> Result<u32, std::io::Error> {
    let mut deleted = 0;
    
    for size in [ThumbnailSize::Small, ThumbnailSize::Medium, ThumbnailSize::Large] {
        let path = get_thumbnail_path(file_id, size);
        if path.exists() {
            fs::remove_file(path).await?;
            deleted += 1;
        }
    }
    
    Ok(deleted)
}

/// Background thumbnail generation (fire and forget)
pub fn generate_thumbnail_background(source_path: PathBuf, file_id: String) {
    tokio::spawn(async move {
        if supports_thumbnail(&source_path) {
            match generate_thumbnail(&source_path, &file_id, ThumbnailSize::Medium).await {
                Ok(_) => tracing::debug!("Generated thumbnail for {}", file_id),
                Err(e) => tracing::warn!("Thumbnail generation failed for {}: {}", file_id, e),
            }
        }
    });
}

// ==================== TOOL DETECTION ====================

fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg").arg("-version").output().is_ok()
}

fn is_imagemagick_available() -> bool {
    Command::new("convert").arg("--version").output().is_ok()
}

fn is_pdftoppm_available() -> bool {
    Command::new("pdftoppm").arg("-v").output().is_ok()
}

/// Check which tools are available
pub fn check_available_tools() -> AvailableTools {
    AvailableTools {
        ffmpeg: is_ffmpeg_available(),
        imagemagick: is_imagemagick_available(),
        pdftoppm: is_pdftoppm_available(),
        image_crate: true, // Always available (pure Rust)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailableTools {
    pub ffmpeg: bool,
    pub imagemagick: bool,
    pub pdftoppm: bool,
    pub image_crate: bool,
}

// ==================== ERRORS ====================

#[derive(Debug, Clone)]
pub enum ThumbnailError {
    UnsupportedFormat,
    ToolNotInstalled(String),
    ProcessingError(String),
    IoError(String),
    FileNotFound,
}

impl std::fmt::Display for ThumbnailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThumbnailError::UnsupportedFormat => write!(f, "Unsupported file format for thumbnail"),
            ThumbnailError::ToolNotInstalled(tool) => write!(f, "Required tool not installed: {}", tool),
            ThumbnailError::ProcessingError(e) => write!(f, "Thumbnail processing error: {}", e),
            ThumbnailError::IoError(e) => write!(f, "IO error: {}", e),
            ThumbnailError::FileNotFound => write!(f, "Source file not found"),
        }
    }
}

impl std::error::Error for ThumbnailError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_image() {
        assert!(is_image(Path::new("test.jpg")));
        assert!(is_image(Path::new("test.PNG")));
        assert!(is_image(Path::new("test.webp")));
        assert!(!is_image(Path::new("test.pdf")));
        assert!(!is_image(Path::new("test.txt")));
    }
    
    #[test]
    fn test_is_video() {
        assert!(is_video(Path::new("video.mp4")));
        assert!(is_video(Path::new("video.MKV")));
        assert!(!is_video(Path::new("image.jpg")));
    }
    
    #[test]
    fn test_thumbnail_path() {
        let path = get_thumbnail_path("abc123", ThumbnailSize::Medium);
        assert!(path.to_string_lossy().contains("abc123_md.webp"));
    }
}
