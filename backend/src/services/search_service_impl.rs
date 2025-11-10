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
        Ok(mut results) => {
            // ALSO search in folders table for folder matching
            let query_lower = query.to_lowercase();
            if let Ok(folders) = sqlx::query_as::<_, (String, String)>(
                "SELECT name, path FROM folders WHERE LOWER(name) LIKE ?",
            )
            .bind(format!("%{}%", query_lower))
            .fetch_all(&state.db_pool)
            .await
            {
                for (name, path) in folders {
                    // Add folder to results
                    results.push(crate::search::SearchResult {
                        file_id: path.clone(),
                        filename: name.clone(),
                        path: path.clone(),
                        snippet: None,
                        score: 10.0, // High score for exact folder matches
                        size: 0,
                        modified: chrono::Utc::now().to_rfc3339(),
                        file_type: "folder".to_string(),
                        highlights: None,
                    });
                }
            }

            // Map SearchResult to proper JSON with all fields
            let json_results = results
                .iter()
                .map(|r| {
                    let is_folder = r.file_type == "folder";
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
                        "is_dir": is_folder,
                        "type": if is_folder { "folder" } else { "file" },
                    })
                })
                .collect();
            Ok(json_results)
        }
        Err(e) => Err(anyhow::anyhow!("Search failed: {}", e)),
    }
}
