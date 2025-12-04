#![allow(dead_code)]

//! Full-text search API endpoints

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default = "default_fuzzy")]
    pub fuzzy: bool,
}

#[derive(Debug, Serialize)]
pub struct ReindexResponse {
    pub message: String,
    pub files_indexed: usize,
}

fn default_limit() -> usize {
    50
}
fn default_fuzzy() -> bool {
    true
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/search", get(search_handler))
        .route("/search/suggest", get(suggest_handler))
        .route("/search/facets", get(facets_handler))
        .route("/search/reindex", post(reindex_handler))
}

async fn search_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Query(query): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // SECURITY: Validate search query to prevent SQL injection
    let safe_query = crate::security::validate_search_query(&query.q)?;

    let results = services::search(&state, &user, &safe_query, query.limit, query.fuzzy)
        .await
        .map_err(|e| {
            eprintln!("Search error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Wrap results in proper response format
    Ok(Json(serde_json::json!({
        "results": results,
        "query": query.q,
        "total": results.len(),
        "limit": query.limit,
        "fuzzy": query.fuzzy
    })))
}

/// POST /api/search/reindex - Rebuild search index from all files in database
async fn reindex_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<ReindexResponse>, StatusCode> {
    eprintln!("🔄 Starting search index rebuild...");

    // Get all non-deleted files from database
    let files = sqlx::query(
        "SELECT id, name, path, size_bytes, updated_at FROM files WHERE is_deleted = 0",
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch files for reindex: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut indexed_count = 0;

    for row in files {
        let file_id: String = row.try_get("id").unwrap_or_default();
        let filename: String = row.try_get("name").unwrap_or_default();
        let path: String = row.try_get("path").unwrap_or_default();
        let size_bytes: i64 = row.try_get("size_bytes").unwrap_or(0);
        let updated_at: String = row
            .try_get("updated_at")
            .unwrap_or_else(|_| chrono::Utc::now().to_rfc3339());

        // Parse datetime
        let modified = chrono::DateTime::parse_from_rfc3339(&updated_at)
            .unwrap_or_else(|_| chrono::Utc::now().into())
            .with_timezone(&chrono::Utc);

        // Extract content from file (if possible)
        let file_path = std::path::Path::new("./data").join(&path);
        let content = if file_path.exists() {
            crate::search::extract_content(&file_path).await
        } else {
            None
        };

        // Index the file
        if let Err(e) = state
            .search_index
            .index_file(
                &file_id,
                &filename,
                &path,
                content,
                modified,
                size_bytes as u64,
            )
            .await
        {
            eprintln!("Failed to index file {}: {:?}", filename, e);
        } else {
            indexed_count += 1;
        }
    }

    eprintln!("✅ Reindex complete: {} files indexed", indexed_count);

    Ok(Json(ReindexResponse {
        message: format!("Successfully reindexed {} files", indexed_count),
        files_indexed: indexed_count,
    }))
}

/// GET /api/search/suggest - Get search suggestions for autocomplete
/// Uses database LIKE query for reliable prefix matching
async fn suggest_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<crate::search::SearchSuggestion>>, StatusCode> {
    // Use database LIKE query for reliable autocomplete
    let search_pattern = format!("%{}%", query.q.to_lowercase());
    let prefix_pattern = format!("{}%", query.q.to_lowercase());
    
    // Simpler query without UNION - just search files and use mime_type to detect folders
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT name, 
               COALESCE(
                   CASE 
                       WHEN mime_type = 'inode/directory' THEN 'folder'
                       WHEN mime_type IS NULL OR mime_type = '' THEN 
                           CASE 
                               WHEN INSTR(name, '.') > 0 THEN LOWER(SUBSTR(name, INSTR(name, '.') + 1))
                               ELSE 'file'
                           END
                       ELSE SUBSTR(mime_type, INSTR(mime_type, '/') + 1)
                   END,
                   'file'
               ) as file_type,
               path,
               size_bytes,
               CASE WHEN LOWER(name) LIKE ?2 THEN 0 ELSE 1 END as priority
        FROM files 
        WHERE is_deleted = 0 
          AND (LOWER(name) LIKE ?1 OR LOWER(path) LIKE ?1)
        ORDER BY priority, name
        LIMIT ?3
        "#,
    )
    .bind(&search_pattern)
    .bind(&prefix_pattern)
    .bind(query.limit as i64)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get suggestions from DB: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let suggestions: Vec<crate::search::SearchSuggestion> = rows
        .iter()
        .map(|row| {
            use sqlx::Row;
            let path: String = row.try_get("path").unwrap_or_default();
            // Extract folder path (parent directory)
            let folder_path = if path.contains('/') {
                path.rsplit_once('/').map(|(parent, _)| parent.to_string())
            } else {
                None
            };
            crate::search::SearchSuggestion {
                text: row.try_get("name").unwrap_or_default(),
                file_type: row.try_get("file_type").ok(),
                score: 1.0,
                path: folder_path,
                size_bytes: row.try_get("size_bytes").ok(),
            }
        })
        .collect();

    Ok(Json(suggestions))
}

/// GET /api/search/facets - Get search facets (aggregations)
async fn facets_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<SearchQuery>,
) -> Result<Json<crate::search::SearchFacets>, StatusCode> {
    let query_str = if query.q.is_empty() {
        None
    } else {
        Some(query.q.as_str())
    };

    let facets = state.search_index.facets(query_str).map_err(|e| {
        eprintln!("Failed to get facets: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(facets))
}
