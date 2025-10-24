/// Smart folder organization with auto-tagging and rules
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SmartFolder {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub rules: String, // JSON array of rule objects
    pub auto_update: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderRule {
    pub field: String, // "extension", "size", "created_date", "tag", "content"
    pub operator: String, // "equals", "contains", "greater_than", "less_than"
    pub value: String,
}

/// Create smart folder
pub async fn create_smart_folder(
    pool: &SqlitePool,
    user_id: &str,
    name: &str,
    rules: Vec<FolderRule>,
) -> Result<SmartFolder, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let rules_json = serde_json::to_string(&rules).unwrap_or_else(|_| "[]".to_string());
    
    sqlx::query(
        "INSERT INTO smart_folders (id, user_id, name, rules, auto_update, created_at)
         VALUES (?, ?, ?, ?, 1, datetime('now'))"
    )
    .bind(&id)
    .bind(user_id)
    .bind(name)
    .bind(&rules_json)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM smart_folders WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Auto-tag file based on content/extension
pub async fn auto_tag_file(
    pool: &SqlitePool,
    file_id: &str,
    file_name: &str,
    mime_type: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let mut tags = Vec::new();
    
    // Tag by file type
    if mime_type.starts_with("image/") {
        tags.push("image".to_string());
    } else if mime_type.starts_with("video/") {
        tags.push("video".to_string());
    } else if mime_type.starts_with("audio/") {
        tags.push("audio".to_string());
    } else if mime_type.contains("pdf") {
        tags.push("document".to_string());
        tags.push("pdf".to_string());
    }
    
    // Tag by extension
    if let Some(ext) = file_name.split('.').last() {
        match ext.to_lowercase().as_str() {
            "rs" => tags.push("rust".to_string()),
            "js" | "ts" => tags.push("javascript".to_string()),
            "py" => tags.push("python".to_string()),
            "go" => tags.push("golang".to_string()),
            _ => {}
        }
    }
    
    // Apply tags
    for tag_name in &tags {
        // Get or create tag
        let tag_id: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM tags WHERE name = ?"
        )
        .bind(tag_name)
        .fetch_optional(pool)
        .await?;
        
        let tag_id = if let Some((id,)) = tag_id {
            id
        } else {
            let new_id = Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO tags (id, name, created_at) VALUES (?, ?, datetime('now'))")
                .bind(&new_id)
                .bind(tag_name)
                .execute(pool)
                .await?;
            new_id
        };
        
        // Link tag to file
        sqlx::query(
            "INSERT OR IGNORE INTO file_tags (file_id, tag_id) VALUES (?, ?)"
        )
        .bind(file_id)
        .bind(&tag_id)
        .execute(pool)
        .await?;
    }
    
    Ok(tags)
}
