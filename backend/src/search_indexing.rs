/// Advanced full-text search indexing with metadata extraction
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    pub file_id: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub indexed_at: String,
}

/// Extract text from any supported file using textractor
/// Supports: PDF, DOCX, XLSX, ODF, HTML, MD, TXT, ZIP and more
pub async fn extract_text_from_file(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let path = path.to_string();
    let result = tokio::task::spawn_blocking(move || {
        textractor::extract(&path)
    }).await??;
    
    Ok(result)
}

/// Extract text from PDF (using textractor)
pub async fn extract_text_from_pdf(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    extract_text_from_file(path).await
}

/// Extract text from Word document (using textractor)
pub async fn extract_text_from_docx(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    extract_text_from_file(path).await
}

/// OCR for images (using tesseract - requires external binary)
pub async fn extract_text_from_image(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let output = tokio::process::Command::new("tesseract")
        .args(&[path, "stdout"])
        .output()
        .await?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Index file content in search database
pub async fn index_file_content(
    pool: &SqlitePool,
    file_id: &str,
    file_path: &str,
    mime_type: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content = match mime_type {
        "application/pdf" => extract_text_from_pdf(file_path).await?,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => extract_text_from_docx(file_path).await?,
        m if m.starts_with("image/") => extract_text_from_image(file_path).await.unwrap_or_default(),
        m if m.starts_with("text/") => tokio::fs::read_to_string(file_path).await?,
        _ => String::new(),
    };
    
    if !content.is_empty() {
        sqlx::query(
            "INSERT OR REPLACE INTO search_index (file_id, content, indexed_at)
             VALUES (?, ?, datetime('now'))"
        )
        .bind(file_id)
        .bind(&content)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

/// Fuzzy search in indexed content
pub async fn fuzzy_search(
    pool: &SqlitePool,
    query: &str,
    limit: i64,
) -> Result<Vec<String>, sqlx::Error> {
    // Use SQLite FTS5 if available, otherwise LIKE
    let file_ids: Vec<(String,)> = sqlx::query_as(
        "SELECT file_id FROM search_index WHERE content LIKE ? LIMIT ?"
    )
    .bind(format!("%{}%", query))
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(file_ids.into_iter().map(|(id,)| id).collect())
}
