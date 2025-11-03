//! Search API endpoints

use crate::auth::UserInfo;

use axum::{extract::{Query, State}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::{services, AppState};

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

fn default_limit() -> usize { 50 }
fn default_fuzzy() -> bool { true }

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/search", get(search_handler))
        .route("/search/reindex", post(reindex_handler))
}

async fn search_handler(State(state): State<AppState>, user: UserInfo, Query(query): Query<SearchQuery>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // SECURITY: Validate search query to prevent SQL injection
    let safe_query = crate::security::validate_search_query(&query.q)?;
    
    services::search(&state, &user, &safe_query, query.limit, query.fuzzy)
        .await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// POST /api/search/reindex - Rebuild search index from all files in database
async fn reindex_handler(State(state): State<AppState>, _user: UserInfo) -> Result<Json<ReindexResponse>, StatusCode> {
    eprintln!("🔄 Starting search index rebuild...");
    
    // Get all non-deleted files from database
    let files = sqlx::query(
        "SELECT id, name, path, size_bytes, updated_at FROM files WHERE is_deleted = 0"
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
        let updated_at: String = row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now().to_rfc3339());
        
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
        if let Err(e) = state.search_index.index_file(
            &file_id,
            &filename,
            &path,
            content,
            modified,
            size_bytes as u64,
        ).await {
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

