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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,
}

/// Search suggestion for autocomplete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    pub text: String,
    pub file_type: Option<String>,
    pub score: f32,
}

/// Faceted search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacets {
    pub file_types: std::collections::HashMap<String, usize>,
    pub size_ranges: std::collections::HashMap<String, usize>,
    pub date_ranges: std::collections::HashMap<String, usize>,
}

/// Advanced search options
#[derive(Debug, Clone, Deserialize)]
pub struct SearchOptions {
    pub fuzzy: bool,
    pub fuzzy_distance: Option<u8>, // Max edit distance (1-2)
    pub file_type_filter: Option<Vec<String>>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
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
    // Batch operation tracking
    pending_operations: Arc<Mutex<usize>>,
    last_commit: Arc<Mutex<std::time::Instant>>,
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

        let search_index = Self {
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
            pending_operations: Arc::new(Mutex::new(0)),
            last_commit: Arc::new(Mutex::new(std::time::Instant::now())),
        };

        // Start background commit task
        search_index.start_auto_commit();

        Ok(search_index)
    }

    /// Start background task that commits every 30 seconds or when 100 operations pending
    fn start_auto_commit(&self) {
        let writer = self.writer.clone();
        let pending = self.pending_operations.clone();
        let last_commit = self.last_commit.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                let pending_count = *pending.lock().await;
                let time_since_last = last_commit.lock().await.elapsed();

                // Commit if: 100+ pending operations OR 30+ seconds since last commit
                if pending_count > 0 && (pending_count >= 100 || time_since_last.as_secs() >= 30) {
                    println!(
                        "🔍 Auto-committing search index ({} pending operations)",
                        pending_count
                    );

                    let mut w = writer.lock().await;
                    if let Err(e) = w.commit() {
                        eprintln!("❌ Search index commit failed: {}", e);
                    } else {
                        *pending.lock().await = 0;
                        *last_commit.lock().await = std::time::Instant::now();
                        println!("✅ Search index committed");
                    }
                }
            }
        });
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

        // Increment pending operations counter
        *self.pending_operations.lock().await += 1;

        // Don't commit immediately - let auto-commit handle it
        // Only force commit if many operations pending (>100)
        let pending_count = *self.pending_operations.lock().await;
        if pending_count >= 100 {
            println!(
                "🔍 Force committing search index ({} operations)",
                pending_count
            );
            writer.commit()?;
            *self.pending_operations.lock().await = 0;
            *self.last_commit.lock().await = std::time::Instant::now();
        }

        Ok(())
    }

    /// Delete document from index
    pub async fn delete_from_index(
        &self,
        file_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let term = Term::from_field_text(self.file_id_field, file_id);
        let mut writer = self.writer.lock().await;
        writer.delete_term(term);

        // Increment pending operations counter
        *self.pending_operations.lock().await += 1;

        // Don't commit immediately - let auto-commit handle it
        let pending_count = *self.pending_operations.lock().await;
        if pending_count >= 100 {
            println!(
                "🔍 Force committing search index ({} operations)",
                pending_count
            );
            writer.commit()?;
            *self.pending_operations.lock().await = 0;
            *self.last_commit.lock().await = std::time::Instant::now();
        }

        Ok(())
    }

    /// Batch index multiple files (more efficient than individual calls)
    pub async fn batch_index_files(
        &self,
        files: Vec<(
            String,
            String,
            String,
            Option<String>,
            chrono::DateTime<chrono::Utc>,
            u64,
        )>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("🔍 Batch indexing {} files", files.len());

        let mut writer = self.writer.lock().await;

        for (file_id, filename, path, content, modified, size) in files {
            let file_type = Self::detect_file_type(&filename);

            // Create document
            let doc = doc!(
                self.file_id_field => file_id.as_str(),
                self.filename_field => filename.as_str(),
                self.path_field => path.as_str(),
                self.content_field => content.unwrap_or_default(),
                self.modified_field => tantivy::DateTime::from_timestamp_secs(modified.timestamp()),
                self.size_field => size,
                self.file_type_field => file_type,
            );

            // Delete old document if exists
            let term = Term::from_field_text(self.file_id_field, &file_id);
            writer.delete_term(term);

            // Add new document
            writer.add_document(doc)?;
        }

        // Commit batch
        writer.commit()?;
        *self.last_commit.lock().await = std::time::Instant::now();
        println!("✅ Batch index committed");

        Ok(())
    }

    /// Force commit pending operations immediately
    pub async fn force_commit(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pending_count = *self.pending_operations.lock().await;
        if pending_count > 0 {
            println!("🔍 Force committing {} pending operations", pending_count);
            let mut writer = self.writer.lock().await;
            writer.commit()?;
            *self.pending_operations.lock().await = 0;
            *self.last_commit.lock().await = std::time::Instant::now();
            println!("✅ Force commit completed");
        }
        Ok(())
    }

    /// Search with query string
    pub fn search(
        &self,
        query_str: &str,
        limit: usize,
        fuzzy: bool,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error + Send + Sync>> {
        self.search_advanced(
            query_str,
            limit,
            SearchOptions {
                fuzzy,
                fuzzy_distance: Some(2),
                file_type_filter: None,
                min_size: None,
                max_size: None,
                date_from: None,
                date_to: None,
            },
        )
    }

    /// Advanced search with filters and facets
    pub fn search_advanced(
        &self,
        query_str: &str,
        limit: usize,
        options: SearchOptions,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error + Send + Sync>> {
        let searcher = self.reader.searcher();

        // Build query using QueryParser for tokenized fields
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.filename_field, self.path_field, self.content_field],
        );

        // For non-fuzzy search, use AllQuery to get all documents (we filter in post-processing)
        // For fuzzy search, use standard query parser
        let mut query: Box<dyn tantivy::query::Query> = if options.fuzzy {
            query_parser.parse_query(query_str)?
        } else {
            // Use AllQuery to retrieve all documents, then filter by substring
            Box::new(tantivy::query::AllQuery)
        };

        // Apply filters if specified
        if let Some(ref file_types) = options.file_type_filter {
            // Add file type filter
            use tantivy::query::{BooleanQuery, Occur};

            let file_type_queries: Vec<Box<dyn tantivy::query::Query>> = file_types
                .iter()
                .map(|ft| {
                    let term = Term::from_field_text(self.file_type_field, ft);
                    Box::new(tantivy::query::TermQuery::new(
                        term,
                        tantivy::schema::IndexRecordOption::Basic,
                    )) as Box<dyn tantivy::query::Query>
                })
                .collect();

            if !file_type_queries.is_empty() {
                let file_type_filter = BooleanQuery::new(
                    file_type_queries
                        .into_iter()
                        .map(|q| (Occur::Should, q))
                        .collect(),
                );

                query = Box::new(BooleanQuery::new(vec![
                    (Occur::Must, query),
                    (Occur::Must, Box::new(file_type_filter)),
                ]));
            }
        }

        // Search - get more results than needed for post-filtering
        let search_limit = if !options.fuzzy { limit * 10 } else { limit };
        let top_docs = searcher.search(&query, &TopDocs::with_limit(search_limit))?;

        // Convert to results with post-processing substring filter
        let query_lower = query_str.to_lowercase();
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

            // POST-PROCESSING FILTER: For non-fuzzy search, only include if filename contains query substring
            if !options.fuzzy {
                let filename_lower = filename.to_lowercase();
                let path_lower = path.to_lowercase();

                // Check if query is substring of filename or path
                if !filename_lower.contains(&query_lower) && !path_lower.contains(&query_lower) {
                    continue; // Skip this result
                }
            }

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

            // Apply post-search filters (size, date)
            if let Some(min_size) = options.min_size {
                if size < min_size {
                    continue;
                }
            }
            if let Some(max_size) = options.max_size {
                if size > max_size {
                    continue;
                }
            }

            if let Some(date_from) = options.date_from {
                if let Ok(file_date) = chrono::DateTime::parse_from_rfc3339(&modified) {
                    if file_date.with_timezone(&chrono::Utc) < date_from {
                        continue;
                    }
                }
            }

            if let Some(date_to) = options.date_to {
                if let Ok(file_date) = chrono::DateTime::parse_from_rfc3339(&modified) {
                    if file_date.with_timezone(&chrono::Utc) > date_to {
                        continue;
                    }
                }
            }

            // Generate highlights for filename and path
            let highlights = Self::generate_highlights(query_str, &filename, &path);

            // Try to extract snippet from file content
            let file_path = std::path::Path::new("./data").join(&path);
            let snippet = if file_path.exists() {
                // Read first 4KB of text files for snippet extraction
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    let truncated = if content.len() > 4096 {
                        &content[..4096]
                    } else {
                        &content
                    };
                    Self::extract_snippet(query_str, truncated, 150)
                } else {
                    None
                }
            } else {
                None
            };

            results.push(SearchResult {
                file_id,
                filename,
                path,
                snippet,
                score,
                size,
                modified,
                file_type,
                highlights: Some(highlights),
            });

            // Limit results to requested amount after filtering
            if results.len() >= limit {
                break;
            }
        }

        Ok(results)
    }

    /// Generate search suggestions for autocomplete
    pub fn suggest(
        &self,
        prefix: &str,
        limit: usize,
    ) -> Result<Vec<SearchSuggestion>, Box<dyn std::error::Error + Send + Sync>> {
        let searcher = self.reader.searcher();

        // Create prefix query on filename field
        let query_str = format!("{}*", prefix); // Wildcard search
        let query_parser = QueryParser::for_index(&self.index, vec![self.filename_field]);

        let query = query_parser.parse_query(&query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit * 2))?; // Get more to deduplicate

        // Collect unique suggestions
        let mut suggestions = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for (score, doc_address) in top_docs {
            if suggestions.len() >= limit {
                break;
            }

            let retrieved_doc = searcher.doc::<tantivy::TantivyDocument>(doc_address)?;

            let filename = retrieved_doc
                .get_first(self.filename_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let file_type = retrieved_doc
                .get_first(self.file_type_field)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // Deduplicate by filename
            if !seen.contains(&filename) {
                seen.insert(filename.clone());
                suggestions.push(SearchSuggestion {
                    text: filename,
                    file_type,
                    score,
                });
            }
        }

        Ok(suggestions)
    }

    /// Get search facets (aggregations by file type, size, date)
    pub fn facets(
        &self,
        query_str: Option<&str>,
    ) -> Result<SearchFacets, Box<dyn std::error::Error + Send + Sync>> {
        let searcher = self.reader.searcher();

        // Get all or filtered documents
        let query: Box<dyn tantivy::query::Query> = if let Some(q) = query_str {
            let query_parser = QueryParser::for_index(
                &self.index,
                vec![self.filename_field, self.path_field, self.content_field],
            );
            query_parser.parse_query(q)?
        } else {
            Box::new(tantivy::query::AllQuery)
        };

        // Search all matching documents
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10000))?; // Limit to prevent OOM

        let mut file_types = std::collections::HashMap::new();
        let mut size_ranges = std::collections::HashMap::new();
        let mut date_ranges = std::collections::HashMap::new();

        for (_, doc_address) in top_docs {
            let retrieved_doc = searcher.doc::<tantivy::TantivyDocument>(doc_address)?;

            // Count by file type
            if let Some(file_type) = retrieved_doc
                .get_first(self.file_type_field)
                .and_then(|v| v.as_str())
            {
                *file_types.entry(file_type.to_string()).or_insert(0) += 1;
            }

            // Count by size range
            if let Some(size) = retrieved_doc
                .get_first(self.size_field)
                .and_then(|v| v.as_u64())
            {
                let range = match size {
                    0..=1024 => "< 1 KB",
                    1025..=10240 => "1-10 KB",
                    10241..=102400 => "10-100 KB",
                    102401..=1048576 => "100 KB - 1 MB",
                    1048577..=10485760 => "1-10 MB",
                    10485761..=104857600 => "10-100 MB",
                    _ => "> 100 MB",
                };
                *size_ranges.entry(range.to_string()).or_insert(0) += 1;
            }

            // Count by date range (last 7 days, 30 days, 90 days, older)
            if let Some(modified) = retrieved_doc
                .get_first(self.modified_field)
                .and_then(|v| v.as_datetime())
            {
                let file_date = chrono::DateTime::from_timestamp(modified.into_timestamp_secs(), 0)
                    .unwrap_or_default();
                let now = chrono::Utc::now();
                let days_ago = (now - file_date).num_days();

                let range = match days_ago {
                    0..=7 => "Last 7 days",
                    8..=30 => "Last 30 days",
                    31..=90 => "Last 90 days",
                    _ => "Older",
                };
                *date_ranges.entry(range.to_string()).or_insert(0) += 1;
            }
        }

        Ok(SearchFacets {
            file_types,
            size_ranges,
            date_ranges,
        })
    }

    /// Generate highlights by matching query terms in text
    fn generate_highlights(query: &str, filename: &str, path: &str) -> Vec<String> {
        let mut highlights = Vec::new();
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

        // Check filename
        let filename_lower = filename.to_lowercase();
        for term in &query_terms {
            if filename_lower.contains(term) {
                highlights.push(format!("Filename: {}", filename));
                break;
            }
        }

        // Check path
        let path_lower = path.to_lowercase();
        for term in &query_terms {
            if path_lower.contains(term) {
                highlights.push(format!("Path: {}", path));
                break;
            }
        }

        highlights
    }

    /// Extract a snippet around the first occurrence of query terms in content
    fn extract_snippet(query: &str, content: &str, max_length: usize) -> Option<String> {
        if content.is_empty() {
            return None;
        }

        let content_lower = content.to_lowercase();
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

        // Find the first matching term position
        let mut best_pos: Option<usize> = None;
        for term in &query_terms {
            if let Some(pos) = content_lower.find(term) {
                match best_pos {
                    None => best_pos = Some(pos),
                    Some(current) if pos < current => best_pos = Some(pos),
                    _ => {}
                }
            }
        }

        // If we found a match, extract snippet around it
        if let Some(pos) = best_pos {
            let half_len = max_length / 2;
            let start = pos.saturating_sub(half_len);
            let end = (pos + half_len).min(content.len());

            // Find word boundaries
            let actual_start = if start == 0 {
                0
            } else {
                content[start..]
                    .find(char::is_whitespace)
                    .map(|p| start + p + 1)
                    .unwrap_or(start)
            };

            let actual_end = if end >= content.len() {
                content.len()
            } else {
                content[..end]
                    .rfind(char::is_whitespace)
                    .unwrap_or(end)
            };

            let mut snippet = content[actual_start..actual_end].to_string();

            // Add ellipsis if truncated
            if actual_start > 0 {
                snippet = format!("...{}", snippet);
            }
            if actual_end < content.len() {
                snippet = format!("{}...", snippet);
            }

            return Some(snippet);
        }

        // No match found, return beginning of content
        let end = max_length.min(content.len());
        let actual_end = content[..end]
            .rfind(char::is_whitespace)
            .unwrap_or(end);
        
        let snippet = if actual_end < content.len() {
            format!("{}...", &content[..actual_end])
        } else {
            content[..actual_end].to_string()
        };

        Some(snippet)
    }

    /// Search with query string (legacy method - kept for backwards compatibility)
    pub fn search_legacy(
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

            // Try to extract snippet from file content
            let file_path = std::path::Path::new("./data").join(&path);
            let snippet = if file_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    let truncated = if content.len() > 4096 {
                        &content[..4096]
                    } else {
                        &content
                    };
                    Self::extract_snippet(query_str, truncated, 150)
                } else {
                    None
                }
            } else {
                None
            };

            results.push(SearchResult {
                file_id,
                filename,
                path,
                snippet,
                score,
                size,
                modified,
                file_type,
                highlights: None,
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

        // DOCX extraction - parse XML from zip archive
        "docx" => extract_docx_content(file_path).await,
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
        for (page_num, _) in pages.iter().take(100) {
            // Limit to first 100 pages
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

/// Extract text from DOCX file (Word document)
/// DOCX files are ZIP archives containing XML files
async fn extract_docx_content(file_path: &Path) -> Option<String> {
    use std::io::Read;
    
    let path = file_path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        // Open DOCX file as ZIP
        let file = std::fs::File::open(&path).ok()?;
        let mut archive = zip::ZipArchive::new(file).ok()?;
        let mut text = String::new();

        // Extract text from document.xml
        if let Ok(mut doc_xml) = archive.by_name("word/document.xml") {
            let mut content = String::new();
            doc_xml.read_to_string(&mut content).ok()?;
            
            // Simple XML tag stripping (remove <...> tags)
            let text_only: String = content
                .chars()
                .fold((String::new(), false), |(mut acc, in_tag), c| {
                    if c == '<' {
                        (acc, true)
                    } else if c == '>' {
                        (acc, false)
                    } else if !in_tag {
                        acc.push(c);
                        (acc, false)
                    } else {
                        (acc, true)
                    }
                })
                .0;
            
            text.push_str(&text_only);
        }

        // Extract from headers and footers if present
        if let Ok(mut header) = archive.by_name("word/header1.xml") {
            let mut content = String::new();
            header.read_to_string(&mut content).ok()?;
            text.push_str("\n");
            text.push_str(&content);
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_index_and_search() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let index = SearchIndex::new()?;

        // Index a test file
        index
            .index_file(
                "test-1",
                "example.txt",
                "/documents/example.txt",
                Some("This is a test document with searchable content".to_string()),
                chrono::Utc::now(),
                1024,
            )
            .await?;

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
