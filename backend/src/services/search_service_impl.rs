//! Search service - Full implementation
use crate::{auth::UserInfo, AppState};
use anyhow::Result;

pub async fn search(state: &AppState, user: &UserInfo, query: &str, limit: usize, fuzzy: bool) -> Result<Vec<serde_json::Value>> {
    let cache_key = format!("search_{}_{}_{}_{}", query, limit, fuzzy, user.id);
    if let Ok(Some(cached)) = state.cache_manager.get_search_results(&cache_key).await {
        return Ok(cached.into_iter().map(|r| serde_json::json!({
            "path": r.file_path, "score": r.relevance_score, "snippet": r.snippet,
        })).collect());
    }
    
    match state.search_index.search(query, limit, fuzzy) {
        Ok(results) => Ok(results.iter().map(|r| serde_json::json!({
            "path": r.path, "score": r.score, "snippet": r.snippet, "size": r.size,
        })).collect()),
        Err(e) => Err(anyhow::anyhow!("Search failed: {}", e)),
    }
}
