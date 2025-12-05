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

/// Extract text from any supported file based on extension
/// Uses pdf-extract for PDFs, docx-rust for DOCX, calamine for Excel
pub async fn extract_text_from_file(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let path_buf = Path::new(path).to_path_buf();
    let ext = path_buf.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    
    match ext.as_str() {
        "pdf" => extract_text_from_pdf(path).await,
        "docx" => extract_text_from_docx(path).await,
        "xlsx" | "xls" | "ods" => extract_text_from_spreadsheet(path).await,
        "txt" | "md" | "json" | "xml" | "yaml" | "yml" | "html" | "htm" | "css" | "js" | "ts" | "rs" | "py" => {
            Ok(tokio::fs::read_to_string(path).await?)
        }
        _ => Ok(String::new()),
    }
}

/// Extract text from PDF using pdf-extract
pub async fn extract_text_from_pdf(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let path = path.to_string();
    let result = tokio::task::spawn_blocking(move || {
        pdf_extract::extract_text(&path)
    }).await??;
    
    Ok(result)
}

/// Extract text from Word document using docx-rust
pub async fn extract_text_from_docx(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let path = path.to_string();
    let result = tokio::task::spawn_blocking(move || -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let docx = docx_rust::Docx::parse(std::fs::File::open(&path)?)?;
        let mut text = String::new();
        
        // Extract text from document body
        if let Some(body) = &docx.document.body {
            for content in &body.content {
                if let docx_rust::document::BodyContent::Paragraph(para) = content {
                    for run in &para.content {
                        if let docx_rust::document::ParagraphContent::Run(r) = run {
                            for content in &r.content {
                                if let docx_rust::document::RunContent::Text(t) = content {
                                    text.push_str(&t.text);
                                }
                            }
                        }
                    }
                    text.push('\n');
                }
            }
        }
        
        Ok(text)
    }).await??;
    
    Ok(result)
}

/// Extract text from spreadsheet using calamine (xlsx, xls, ods)
pub async fn extract_text_from_spreadsheet(path: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let path = path.to_string();
    let result = tokio::task::spawn_blocking(move || -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        use calamine::{Reader, open_workbook_auto};
        
        let mut workbook = open_workbook_auto(&path)?;
        let mut text = String::new();
        
        for sheet_name in workbook.sheet_names().to_vec() {
            if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                text.push_str(&format!("=== {} ===\n", sheet_name));
                for row in range.rows() {
                    let row_text: Vec<String> = row.iter().map(|cell| {
                        cell.to_string()
                    }).collect();
                    text.push_str(&row_text.join("\t"));
                    text.push('\n');
                }
                text.push('\n');
            }
        }
        
        Ok(text)
    }).await??;
    
    Ok(result)
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
