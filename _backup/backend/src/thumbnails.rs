/// Thumbnail generation for images and videos
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;

const THUMBNAIL_DIR: &str = "./data/.thumbnails";
const THUMBNAIL_SIZE: u32 = 256; // Max dimension for thumbnails

/// Supported image formats for thumbnail generation
const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp", "bmp", "ico"];

/// Check if file is an image based on extension
pub fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Generate thumbnail path from original file path
pub fn get_thumbnail_path(file_id: &str) -> PathBuf {
    PathBuf::from(THUMBNAIL_DIR).join(format!("{}.webp", file_id))
}

/// Generate thumbnail for an image file
/// Uses ImageMagick's `convert` command if available, otherwise returns error
pub async fn generate_thumbnail(
    source_path: &Path,
    file_id: &str,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    // Ensure thumbnail directory exists
    fs::create_dir_all(THUMBNAIL_DIR).await?;
    
    let thumb_path = get_thumbnail_path(file_id);
    
    // Check if ImageMagick is available
    let has_imagemagick = Command::new("convert")
        .arg("--version")
        .output()
        .is_ok();
    
    if !has_imagemagick {
        return Err("ImageMagick not installed - cannot generate thumbnails".into());
    }
    
    // Generate thumbnail using ImageMagick
    let output = Command::new("convert")
        .arg(source_path)
        .arg("-thumbnail")
        .arg(format!("{}x{}>", THUMBNAIL_SIZE, THUMBNAIL_SIZE)) // Resize maintaining aspect ratio
        .arg("-quality")
        .arg("85")
        .arg("-auto-orient") // Respect EXIF orientation
        .arg(&thumb_path)
        .output()?;
    
    if !output.status.success() {
        return Err(format!(
            "Thumbnail generation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    
    Ok(thumb_path)
}

/// Generate thumbnail in background (fire and forget)
pub fn generate_thumbnail_async(source_path: PathBuf, file_id: String) {
    tokio::spawn(async move {
        if let Err(e) = generate_thumbnail(&source_path, &file_id).await {
            eprintln!("⚠️ Thumbnail generation failed for {}: {}", source_path.display(), e);
        } else {
            println!("✅ Generated thumbnail for {}", source_path.display());
        }
    });
}

/// Delete thumbnail for a file
pub async fn delete_thumbnail(file_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let thumb_path = get_thumbnail_path(file_id);
    if thumb_path.exists() {
        fs::remove_file(thumb_path).await?;
    }
    Ok(())
}

/// Get thumbnail URL path for API
pub fn get_thumbnail_url(file_id: &str) -> String {
    format!("/api/thumbnails/{}", file_id)
}

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
    fn test_thumbnail_path() {
        let path = get_thumbnail_path("abc123");
        assert!(path.to_string_lossy().contains("abc123.webp"));
    }
}
