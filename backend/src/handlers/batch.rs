use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Deserialize)]
pub struct BatchDeleteRequest {
    pub items: Vec<BatchItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchMoveRequest {
    pub items: Vec<BatchItem>,
    pub target_folder: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCopyRequest {
    pub items: Vec<BatchItem>,
    pub target_folder: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchTagRequest {
    pub items: Vec<BatchItem>,
    pub tags: Vec<String>,
    pub action: String, // "add" or "remove"
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchItem {
    pub id: String,
    pub item_type: String, // "file" or "folder"
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchOperationResult {
    pub success_count: usize,
    pub failure_count: usize,
    pub failed_items: Vec<FailedItem>,
    pub total_size: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FailedItem {
    pub id: String,
    pub error: String,
}

/// Batch delete files/folders
pub async fn batch_delete(
    pool: &SqlitePool,
    user_id: &str,
    items: Vec<BatchItem>,
) -> Result<BatchOperationResult, sqlx::Error> {
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failed_items = Vec::new();
    let mut total_size: i64 = 0;
    
    for item in items {
        match perform_delete(pool, user_id, &item).await {
            Ok(size) => {
                success_count += 1;
                total_size += size;
            }
            Err(e) => {
                failure_count += 1;
                failed_items.push(FailedItem {
                    id: item.id.clone(),
                    error: e.to_string(),
                });
            }
        }
    }
    
    Ok(BatchOperationResult {
        success_count,
        failure_count,
        failed_items,
        total_size,
    })
}

/// Batch move files/folders
pub async fn batch_move(
    pool: &SqlitePool,
    user_id: &str,
    items: Vec<BatchItem>,
    target_folder: &str,
) -> Result<BatchOperationResult, sqlx::Error> {
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failed_items = Vec::new();
    let total_size: i64 = 0;
    
    for item in items {
        match perform_move(pool, user_id, &item, target_folder).await {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                failure_count += 1;
                failed_items.push(FailedItem {
                    id: item.id.clone(),
                    error: e.to_string(),
                });
            }
        }
    }
    
    Ok(BatchOperationResult {
        success_count,
        failure_count,
        failed_items,
        total_size,
    })
}

/// Batch tag files/folders
pub async fn batch_tag(
    pool: &SqlitePool,
    user_id: &str,
    items: Vec<BatchItem>,
    tags: Vec<String>,
    action: &str,
) -> Result<BatchOperationResult, sqlx::Error> {
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failed_items = Vec::new();
    let total_size: i64 = 0;
    
    for item in items {
        let result = if action == "add" {
            add_tags_to_item(pool, user_id, &item, &tags).await
        } else {
            remove_tags_from_item(pool, user_id, &item, &tags).await
        };
        
        match result {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                failure_count += 1;
                failed_items.push(FailedItem {
                    id: item.id.clone(),
                    error: e.to_string(),
                });
            }
        }
    }
    
    Ok(BatchOperationResult {
        success_count,
        failure_count,
        failed_items,
        total_size,
    })
}

// Helper functions

async fn perform_delete(
    pool: &SqlitePool,
    user_id: &str,
    item: &BatchItem,
) -> Result<i64, sqlx::Error> {
    let size = if item.item_type == "file" {
        // Get file size
        let size: Option<(i64,)> = sqlx::query_as(
            "SELECT size_bytes FROM files WHERE id = ? AND owner_id = ?"
        )
        .bind(&item.id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        
        // Mark as deleted
        sqlx::query(
            "UPDATE files SET is_deleted = 1, deleted_at = datetime('now'), deleted_by = ? WHERE id = ?"
        )
        .bind(user_id)
        .bind(&item.id)
        .execute(pool)
        .await?;
        
        size.map(|s| s.0).unwrap_or(0)
    } else {
        // Calculate folder size and mark as deleted
        sqlx::query(
            "UPDATE folders SET is_deleted = 1, deleted_at = datetime('now'), deleted_by = ? WHERE id = ?"
        )
        .bind(user_id)
        .bind(&item.id)
        .execute(pool)
        .await?;
        
        0 // TODO: Calculate folder size
    };
    
    // Add to trash
    sqlx::query(
        "INSERT INTO trash (id, item_type, item_id, original_path, original_name, deleted_by, deleted_at, size_bytes)
         SELECT ?, ?, id, path, name, ?, datetime('now'), ?
         FROM files WHERE id = ? 
         UNION ALL
         SELECT ?, ?, id, path, name, ?, datetime('now'), 0
         FROM folders WHERE id = ?"
    )
    .bind(&uuid::Uuid::new_v4().to_string())
    .bind(&item.item_type)
    .bind(user_id)
    .bind(size)
    .bind(&item.id)
    .bind(&uuid::Uuid::new_v4().to_string())
    .bind(&item.item_type)
    .bind(user_id)
    .bind(&item.id)
    .execute(pool)
    .await?;
    
    Ok(size)
}

async fn perform_move(
    pool: &SqlitePool,
    user_id: &str,
    item: &BatchItem,
    target_folder: &str,
) -> Result<(), sqlx::Error> {
    if item.item_type == "file" {
        sqlx::query(
            "UPDATE files SET folder_id = ?, updated_at = datetime('now') 
             WHERE id = ? AND owner_id = ?"
        )
        .bind(target_folder)
        .bind(&item.id)
        .bind(user_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "UPDATE folders SET parent_folder_id = ?, updated_at = datetime('now') 
             WHERE id = ? AND owner_id = ?"
        )
        .bind(target_folder)
        .bind(&item.id)
        .bind(user_id)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

async fn add_tags_to_item(
    pool: &SqlitePool,
    _user_id: &str,
    item: &BatchItem,
    tags: &[String],
) -> Result<(), sqlx::Error> {
    for tag in tags {
        // Insert tag association
        sqlx::query(
            "INSERT OR IGNORE INTO file_tags (file_id, tag_id, created_at)
             SELECT ?, id, datetime('now') FROM tags WHERE name = ?"
        )
        .bind(&item.id)
        .bind(tag)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

async fn remove_tags_from_item(
    pool: &SqlitePool,
    _user_id: &str,
    item: &BatchItem,
    tags: &[String],
) -> Result<(), sqlx::Error> {
    for tag in tags {
        sqlx::query(
            "DELETE FROM file_tags WHERE file_id = ? AND tag_id IN (SELECT id FROM tags WHERE name = ?)"
        )
        .bind(&item.id)
        .bind(tag)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}
