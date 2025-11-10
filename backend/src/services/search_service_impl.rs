//! Search service - Full implementation
use crate::{auth::UserInfo, AppState};
use anyhow::Result;

pub async fn search(
    state: &AppState,
    user: &UserInfo,
    query: &str,
    limit: usize,
    fuzzy: bool,
) -> Result<Vec<serde_json::Value>> {
    let cache_key = format!("search_{}_{}_{}_{}", query, limit, fuzzy, user.id);
    if let Ok(Some(cached)) = state.cache_manager.get_search_results(&cache_key).await {
        return Ok(cached
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "file_path": r.file_path,
                    "relevance_score": r.relevance_score,
                    "snippet": r.snippet,
                })
            })
            .collect());
    }

    match state.search_index.search(query, limit, fuzzy) {
        Ok(results) => {
            // Map SearchResult to proper JSON with all fields
            let json_results = results
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.file_id,
                        "file_id": r.file_id,
                        "name": r.filename,
                        "filename": r.filename,
                        "path": r.path,
                        "file_path": r.path,
                        "snippet": r.snippet,
                        "score": r.score,
                        "size_bytes": r.size,
                        "size": r.size,
                        "modified": r.modified,
                        "file_type": r.file_type,
                        "highlights": r.highlights,
                        "is_dir": false, // Files only in search index currently
                    })
                })
                .collect();
            Ok(json_results)
        }
        Err(e) => Err(anyhow::anyhow!("Search failed: {}", e)),
    }
}
