//! Full-text search module using Tantivy
//!
//! Provides fast, typo-tolerant search across files and folders.
//! Supports fuzzy search, faceting, and content extraction.

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::{FuzzyTermQuery, QueryParser};
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, Term};
use tokio::sync::Mutex;

const SEARCH_INDEX_DIR: &str = "./data/search_index";
const INDEX_WRITER_MEMORY: usize = 50_000_000; // 50MB

/// Search result with highlighting and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_id: String,
    pub filename: String,
    pub path: String,
    pub snippet: Option<String>,
    pub score: f32,
    pub size: u64,
    pub modified: String,
    pub file_type: String,
}

/// Search index manager
pub struct SearchIndex {
    index: Index,
    reader: IndexReader,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
    // Field handles for fast access
    file_id_field: Field,
    filename_field: Field,
    path_field: Field,
    content_field: Field,
    modified_field: Field,
    size_field: Field,
    file_type_field: Field,
}

impl SearchIndex {
    /// Create or open search index
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Create index directory if not exists
        std::fs::create_dir_all(SEARCH_INDEX_DIR)?;
        
        // Define schema
        let mut schema_builder = Schema::builder();
        
        // File metadata (stored for retrieval)
        let file_id_field = schema_builder.add_text_field("file_id", STRING | STORED);
        let filename_field = schema_builder.add_text_field("filename", TEXT | STORED);
        let path_field = schema_builder.add_text_field("path", TEXT | STORED);
        let file_type_field = schema_builder.add_text_field("file_type", STRING | STORED);
        
        // Searchable content (not stored to save space)
        let content_field = schema_builder.add_text_field("content", TEXT);
        
        // Metadata for filtering and sorting
        let modified_field = schema_builder.add_date_field("modified", STORED | FAST);
        let size_field = schema_builder.add_u64_field("size", STORED | FAST);
        
        let schema = schema_builder.build();
        
        // Clean up stale lock file if exists
        let lock_path = Path::new(SEARCH_INDEX_DIR).join(".tantivy-writer.lock");
        if lock_path.exists() {
            println!("🧹 Cleaning up stale search index lock");
            let _ = std::fs::remove_file(&lock_path);
        }
        
        // Open or create index
        let index = if Path::new(SEARCH_INDEX_DIR).join("meta.json").exists() {
            println!("🔍 Opening existing search index");
            Index::open_in_dir(SEARCH_INDEX_DIR)?
        } else {
            println!("📦 Creating new search index");
            Index::create_in_dir(SEARCH_INDEX_DIR, schema.clone())?
        };
        
        // Create reader and writer
        let reader = index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        
        let writer = index.writer(INDEX_WRITER_MEMORY)?;
        
        println!("✅ Search index initialized");
        
        Ok(Self {
            index,
            reader,
            writer: Arc::new(Mutex::new(writer)),
            schema,
            file_id_field,
            filename_field,
            path_field,
            content_field,
            modified_field,
            size_field,
            file_type_field,
        })
    }
    
    /// Index a file with optional content
    pub async fn index_file(
        &self,
        file_id: &str,
        filename: &str,
        path: &str,
        content: Option<String>,
        modified: chrono::DateTime<chrono::Utc>,
        size: u64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let file_type = Self::detect_file_type(filename);
        
        // Create document
        let doc = doc!(
            self.file_id_field => file_id,
            self.filename_field => filename,
            self.path_field => path,
            self.content_field => content.unwrap_or_default(),
            self.modified_field => tantivy::DateTime::from_timestamp_secs(modified.timestamp()),
            self.size_field => size,
            self.file_type_field => file_type,
        );
        
        // Delete old document if exists (update case)
        let term = Term::from_field_text(self.file_id_field, file_id);
        let mut writer = self.writer.lock().await;
        writer.delete_term(term);
        
        // Add new document
        writer.add_document(doc)?;
        
        // Commit (in production, batch commits for performance)
        writer.commit()?;
        
        Ok(())
    }
    
    /// Delete document from index
    pub async fn delete_from_index(&self, file_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let term = Term::from_field_text(self.file_id_field, file_id);
        let mut writer = self.writer.lock().await;
        writer.delete_term(term);
        writer.commit()?;
        Ok(())
    }
    
    /// Search with query string
    pub fn search(
        &self,
        query_str: &str,
        limit: usize,
        fuzzy: bool,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error + Send + Sync>> {
        let searcher = self.reader.searcher();
        
        // Parse query
        let query = if fuzzy {
            // Fuzzy search - allows typos
            let term = Term::from_field_text(self.filename_field, query_str);
            Box::new(FuzzyTermQuery::new(term, 2, true)) // max 2 edits
        } else {
            // Standard query parser for filename and content
            let query_parser = QueryParser::for_index(
                &self.index,
                vec![self.filename_field, self.path_field, self.content_field],
            );
            query_parser.parse_query(query_str)?
        };
        
        // Search
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
        
        // Convert to results
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc::<tantivy::TantivyDocument>(doc_address)?;
            
            let file_id = retrieved_doc
                .get_first(self.file_id_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let filename = retrieved_doc
                .get_first(self.filename_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let path = retrieved_doc
                .get_first(self.path_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let file_type = retrieved_doc
                .get_first(self.file_type_field)
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            
            let size = retrieved_doc
                .get_first(self.size_field)
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            
            let modified = retrieved_doc
                .get_first(self.modified_field)
                .and_then(|v| v.as_datetime())
                .map(|dt| {
                    chrono::DateTime::from_timestamp(dt.into_timestamp_secs(), 0)
                        .unwrap_or_default()
                        .to_rfc3339()
                })
                .unwrap_or_default();
            
            results.push(SearchResult {
                file_id,
                filename,
                path,
                snippet: None, // TODO: Add snippet extraction
                score,
                size,
                modified,
                file_type,
            });
        }
        
        Ok(results)
    }
    
    /// Detect file type from extension
    fn detect_file_type(filename: &str) -> String {
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let file_type = match ext.to_lowercase().as_str() {
            // Documents
            "pdf" => "document",
            "doc" | "docx" => "document",
            "xls" | "xlsx" => "spreadsheet",
            "ppt" | "pptx" => "presentation",
            
            // Text
            "txt" | "md" | "markdown" => "text",
            "json" | "xml" | "yaml" | "yml" => "data",
            
            // Code
            "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "go" => "code",
            "html" | "css" | "scss" => "web",
            
            // Images
            "jpg" | "jpeg" | "png" | "gif" | "svg" | "webp" => "image",
            
            // Video
            "mp4" | "avi" | "mkv" | "mov" | "webm" => "video",
            
            // Audio
            "mp3" | "wav" | "flac" | "ogg" | "m4a" => "audio",
            
            // Archive
            "zip" | "rar" | "7z" | "tar" | "gz" => "archive",
            
            _ => "unknown",
        };
        
        file_type.to_string()
    }
    
    /// Get index statistics
    pub fn stats(&self) -> Result<IndexStats, Box<dyn std::error::Error + Send + Sync>> {
        let searcher = self.reader.searcher();
        let num_docs = searcher.num_docs();
        
        // Get index size on disk
        let index_size = std::fs::read_dir(SEARCH_INDEX_DIR)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| entry.metadata().ok())
            .map(|metadata| metadata.len())
            .sum::<u64>();
        
        Ok(IndexStats {
            num_documents: num_docs,
            index_size_bytes: index_size,
        })
    }
}

/// Index statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStats {
    pub num_documents: u64,
    pub index_size_bytes: u64,
}

/// Extract text content from file based on type
pub async fn extract_content(file_path: &Path) -> Option<String> {
    let ext = file_path.extension()?.to_str()?.to_lowercase();
    
    match ext.as_str() {
        // Plain text files
        "txt" | "md" | "markdown" | "json" | "xml" | "yaml" | "yml" => {
            tokio::fs::read_to_string(file_path).await.ok()
        }
        
        // Code files
        "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "go" | "html" | "css" => {
            tokio::fs::read_to_string(file_path).await.ok()
        }
        
        // PDF extraction with lopdf
        "pdf" => extract_pdf_content(file_path).await,
        
        // TODO: DOCX extraction with docx-rs
        // "docx" => extract_docx_content(file_path).await,
        
        _ => None,
    }
}

/// Extract text from PDF file
async fn extract_pdf_content(file_path: &Path) -> Option<String> {
    use lopdf::Document;
    
    // Load PDF in blocking task (lopdf is sync)
    let path = file_path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        let doc = Document::load(&path).ok()?;
        let mut text = String::new();
        
        // Extract text from each page
        let pages = doc.get_pages();
        for (page_num, _) in pages.iter().take(100) { // Limit to first 100 pages
            if let Ok(content) = doc.extract_text(&[*page_num]) {
                text.push_str(&content);
                text.push('\n');
            }
        }
        
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    })
    .await
    .ok()
    .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_index_and_search() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let index = SearchIndex::new()?;
        
        // Index a test file
        index.index_file(
            "test-1",
            "example.txt",
            "/documents/example.txt",
            Some("This is a test document with searchable content".to_string()),
            chrono::Utc::now(),
            1024,
        ).await?;
        
        // Search
        let results = index.search("searchable", 10, false)?;
        assert!(!results.is_empty());
        assert_eq!(results[0].filename, "example.txt");
        
        Ok(())
    }
    
    #[test]
    fn test_file_type_detection() {
        assert_eq!(SearchIndex::detect_file_type("test.pdf"), "document");
        assert_eq!(SearchIndex::detect_file_type("code.rs"), "code");
        assert_eq!(SearchIndex::detect_file_type("image.png"), "image");
        assert_eq!(SearchIndex::detect_file_type("data.json"), "data");
    }
}
