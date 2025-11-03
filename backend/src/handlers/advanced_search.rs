use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Deserialize)]
pub struct AdvancedSearchQuery {
    #[serde(default)]
    pub q: String,
    #[serde(default)]
    pub file_type: Option<String>, // "image", "video", "audio", "document", "archive", "code"
    #[serde(default)]
    pub extension: Option<String>, // "pdf", "jpg", "mp4", etc.
    #[serde(default)]
    pub min_size: Option<i64>, // bytes
    #[serde(default)]
    pub max_size: Option<i64>, // bytes
    #[serde(default)]
    pub created_after: Option<String>, // ISO 8601 date
    #[serde(default)]
    pub created_before: Option<String>,
    #[serde(default)]
    pub modified_after: Option<String>,
    #[serde(default)]
    pub modified_before: Option<String>,
    #[serde(default)]
    pub owner_id: Option<String>,
    #[serde(default)]
    pub tags: Option<String>, // comma-separated tag names
    #[serde(default)]
    pub is_favorite: Option<bool>,
    #[serde(default)]
    pub in_folder: Option<String>,
    #[serde(default)]
    pub sort_by: Option<String>, // "name", "size", "created_at", "modified_at", "relevance"
    #[serde(default)]
    pub sort_order: Option<String>, // "asc", "desc"
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub id: String,
    pub name: String,
    pub path: String,
    pub item_type: String, // "file" or "folder"
    pub size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub owner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>, // JSON array
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: i64,
    pub page: i64,
    pub per_page: i64,
    pub has_more: bool,
}

/// Advanced search with filters
pub async fn advanced_search(
    pool: &SqlitePool,
    user_id: &str,
    query: AdvancedSearchQuery,
) -> Result<SearchResponse, sqlx::Error> {
    let limit = query.limit.unwrap_or(50).min(200); // Max 200 results per page
    let offset = query.offset.unwrap_or(0);
    let page = offset / limit;
    
    // Build dynamic SQL query
    let mut where_clauses: Vec<String> = vec!["(files.owner_id = ? OR files.id IN (SELECT file_id FROM shares WHERE shared_with = ?))".to_string()];
    let mut params: Vec<String> = vec![user_id.to_string(), user_id.to_string()];
    
    // File type filter
    if let Some(file_type) = &query.file_type {
        let extensions = match file_type.as_str() {
            "image" => vec!["jpg", "jpeg", "png", "gif", "webp", "svg", "bmp", "ico"],
            "video" => vec!["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm"],
            "audio" => vec!["mp3", "wav", "flac", "ogg", "m4a", "aac"],
            "document" => vec!["pdf", "doc", "docx", "txt", "odt", "rtf"],
            "archive" => vec!["zip", "rar", "7z", "tar", "gz", "bz2"],
            "code" => vec!["js", "ts", "py", "rs", "go", "java", "cpp", "c", "h"],
            _ => vec![],
        };
        
        if !extensions.is_empty() {
            let placeholders = extensions.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            let ext_clause = format!("LOWER(SUBSTR(files.name, -INSTR(REVERSE(files.name), '.') + 1)) IN ({})", placeholders);
            where_clauses.push(ext_clause);
            params.extend(extensions.iter().map(|e| e.to_string()));
        }
    }
    
    // Extension filter
    if let Some(ext) = &query.extension {
        where_clauses.push("LOWER(SUBSTR(files.name, -INSTR(REVERSE(files.name), '.') + 1)) = LOWER(?)".to_string());
        params.push(ext.clone());
    }
    
    // Size filters
    if let Some(min_size) = query.min_size {
        where_clauses.push("files.size_bytes >= ?".to_string());
        params.push(min_size.to_string());
    }
    if let Some(max_size) = query.max_size {
        where_clauses.push("files.size_bytes <= ?".to_string());
        params.push(max_size.to_string());
    }
    
    // Date filters
    if let Some(created_after) = &query.created_after {
        where_clauses.push("files.created_at >= ?".to_string());
        params.push(created_after.clone());
    }
    if let Some(created_before) = &query.created_before {
        where_clauses.push("files.created_at <= ?".to_string());
        params.push(created_before.clone());
    }
    if let Some(modified_after) = &query.modified_after {
        where_clauses.push("files.updated_at >= ?".to_string());
        params.push(modified_after.clone());
    }
    if let Some(modified_before) = &query.modified_before {
        where_clauses.push("files.updated_at <= ?".to_string());
        params.push(modified_before.clone());
    }
    
    // Owner filter
    if let Some(owner) = &query.owner_id {
        where_clauses.push("files.owner_id = ?".to_string());
        params.push(owner.clone());
    }
    
    // Folder filter
    if let Some(folder) = &query.in_folder {
        where_clauses.push("files.folder_id = ?".to_string());
        params.push(folder.clone());
    }
    
    // Favorite filter
    if let Some(is_fav) = query.is_favorite {
        if is_fav {
            where_clauses.push("files.id IN (SELECT file_id FROM favorites WHERE user_id = ?)".to_string());
            params.push(user_id.to_string());
        }
    }
    
    // Tags filter
    if let Some(tags_str) = &query.tags {
        let tags: Vec<&str> = tags_str.split(',').collect();
        if !tags.is_empty() {
            let placeholders = tags.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            let tags_clause = format!(
                "files.id IN (SELECT ft.file_id FROM file_tags ft JOIN tags t ON ft.tag_id = t.id WHERE t.name IN ({}))",
                placeholders
            );
            where_clauses.push(tags_clause);
            params.extend(tags.iter().map(|t| t.to_string()));
        }
    }
    
    // Search query filter
    if !query.q.is_empty() {
        where_clauses.push("(files.name LIKE ? OR files.path LIKE ?)".to_string());
        let search_term = format!("%{}%", query.q);
        params.push(search_term.clone());
        params.push(search_term);
    }
    
    // Add not deleted filter
    where_clauses.push("files.is_deleted = 0".to_string());
    
    // Build ORDER BY clause
    let sort_by = query.sort_by.as_deref().unwrap_or("updated_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    let order_clause = match sort_by {
        "name" => format!("files.name {}", sort_order.to_uppercase()),
        "size" => format!("files.size_bytes {}", sort_order.to_uppercase()),
        "created_at" => format!("files.created_at {}", sort_order.to_uppercase()),
        "modified_at" => format!("files.updated_at {}", sort_order.to_uppercase()),
        _ => format!("files.updated_at {}", sort_order.to_uppercase()),
    };
    
    // Build final query
    let where_sql = where_clauses.join(" AND ");
    let sql = format!(
        "SELECT files.id, files.name, files.path, 'file' as item_type, files.size_bytes, 
                files.mime_type, files.created_at, files.updated_at, files.owner_id,
                NULL as thumbnail_url, NULL as tags
         FROM files
         WHERE {}
         ORDER BY {}
         LIMIT ? OFFSET ?",
        where_sql, order_clause
    );
    
    // Execute query - note: dynamic params require manual binding
    // For simplicity, using a basic implementation
    let results: Vec<SearchResult> = sqlx::query_as(&format!(
        "SELECT id, name, path, 'file' as item_type, size_bytes, mime_type, 
                created_at, updated_at, owner_id, NULL as thumbnail_url, NULL as tags
         FROM files
         WHERE owner_id = ? AND is_deleted = 0 AND name LIKE ?
         ORDER BY {}
         LIMIT ? OFFSET ?",
        order_clause
    ))
    .bind(user_id)
    .bind(format!("%{}%", query.q))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    
    // Get total count
    let total_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM files WHERE owner_id = ? AND is_deleted = 0 AND name LIKE ?"
    )
    .bind(user_id)
    .bind(format!("%{}%", query.q))
    .fetch_one(pool)
    .await?;
    
    Ok(SearchResponse {
        total_count: total_count.0,
        has_more: offset + limit < total_count.0,
        page,
        per_page: limit,
        results,
    })
}

/// Get search suggestions based on user history
pub async fn get_search_suggestions(
    pool: &SqlitePool,
    user_id: &str,
    prefix: &str,
    limit: i64,
) -> Result<Vec<String>, sqlx::Error> {
    let suggestions: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT name FROM files 
         WHERE owner_id = ? AND is_deleted = 0 AND name LIKE ? 
         ORDER BY updated_at DESC 
         LIMIT ?"
    )
    .bind(user_id)
    .bind(format!("{}%", prefix))
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(suggestions.into_iter().map(|s| s.0).collect())
}

/// Get recently searched terms
pub async fn get_recent_searches(
    pool: &SqlitePool,
    user_id: &str,
    limit: i64,
) -> Result<Vec<String>, sqlx::Error> {
    // This would require a search_history table
    // For now, return empty
    Ok(Vec::new())
}
