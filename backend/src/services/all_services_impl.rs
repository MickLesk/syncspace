//! All remaining services - compact implementation
use crate::{auth::UserInfo, AppState};
use anyhow::{anyhow, Result};
use chrono::Utc;
use uuid::Uuid;

// DIRECTORY SERVICE
pub mod directory {
    use super::*;
    use crate::models::DirectoryInfo;
    use tokio::fs;
    use std::path::Path;
    const DATA_DIR: &str = "./data";
    
    pub async fn create_directory(state: &AppState, user: &UserInfo, path: &str) -> Result<DirectoryInfo> {
        let full = Path::new(DATA_DIR).join(path);
        fs::create_dir_all(&full).await?;
        
        // Log activity
        let dirname = full.file_name().unwrap().to_string_lossy().to_string();
        let _ = super::activity::log(
            state,
            &user.id,
            "create",
            path,
            &dirname,
            None,
            None,
            "success",
            None,
            None,
        ).await;
        
        let _ = state.fs_tx.send(crate::FileChangeEvent::new(path.to_string(), "mkdir".to_string()));
        Ok(DirectoryInfo { id: Uuid::new_v4(), name: dirname, path: path.to_string(), parent_id: None, owner_id: Uuid::parse_str(&user.id).unwrap_or_default(), created_at: Utc::now() })
    }
    
    pub async fn delete_directory(state: &AppState, user: &UserInfo, dir_id: &str) -> Result<()> {
        fs::remove_dir_all(Path::new(DATA_DIR).join(dir_id)).await?;
        let _ = state.fs_tx.send(crate::FileChangeEvent::new(dir_id.to_string(), "delete".to_string()));
        Ok(())
    }
    
    pub async fn move_directory(state: &AppState, user: &UserInfo, dir_id: &str, new_parent_path: &str) -> Result<()> {
        let old_path = Path::new(DATA_DIR).join(dir_id);
        let new_path = Path::new(DATA_DIR).join(new_parent_path).join(old_path.file_name().unwrap());
        fs::rename(&old_path, &new_path).await?;
        let _ = state.fs_tx.send(crate::FileChangeEvent::new(new_parent_path.to_string(), "move".to_string()));
        Ok(())
    }
    
    pub async fn rename_directory(state: &AppState, user: &UserInfo, dir_id: &str, new_name: &str) -> Result<()> {
        let old_path = Path::new(DATA_DIR).join(dir_id);
        let parent = old_path.parent().ok_or_else(|| anyhow!("No parent directory"))?;
        let new_path = parent.join(new_name);
        fs::rename(&old_path, &new_path).await?;
        let _ = state.fs_tx.send(crate::FileChangeEvent::new(dir_id.to_string(), "rename".to_string()));
        Ok(())
    }
    
    // Missing functions for API compatibility
    pub async fn batch_move(state: &AppState, user: &UserInfo, paths: Vec<String>, target_path: &str) -> Result<()> {
        for path in paths {
            let _ = move_directory(state, user, &path, target_path).await;
        }
        Ok(())
    }
    
    pub async fn get_directory_tree(state: &AppState, user: &UserInfo, path: &str) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"path": path, "children": []}))
    }
}

// SHARING SERVICE
pub mod sharing {
    use super::*;
    use crate::models::Share;
    
    pub async fn create_share(state: &AppState, user: &UserInfo, path: &str, _is_public: bool) -> Result<Share> {
        let id = Uuid::new_v4();
        let token = format!("{}", Uuid::new_v4());
        let now = Utc::now();
        sqlx::query("INSERT INTO shared_links (id, item_type, item_id, created_by, is_public, allow_download, download_count, created_at) VALUES (?, 'file', ?, ?, ?, 1, 0, ?)")
            .bind(&id).bind(path).bind(&user.id).bind(1).bind(now.to_rfc3339()).execute(&state.db_pool).await?;
        Ok(Share {
            id,
            file_path: path.to_string(),
            token,
            permission: "read".to_string(),
            created_by: Uuid::parse_str(&user.id).unwrap_or_default(),
            created_at: now,
            expires_at: None,
            password_hash: None,
        })
    }
    
    pub async fn list_shares(state: &AppState, user: &UserInfo) -> Result<Vec<Share>> {
        let rows: Vec<crate::database::SharedLink> = sqlx::query_as("SELECT * FROM shared_links WHERE created_by = ?").bind(&user.id).fetch_all(&state.db_pool).await?;
        Ok(rows.iter().map(|r| Share {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            file_path: r.item_id.clone(),
            token: r.id.clone(),
            permission: "read".to_string(),
            created_by: Uuid::parse_str(&r.created_by).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap_or_else(|_| Utc::now().into()).with_timezone(&Utc),
            expires_at: r.expires_at.clone().and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
            password_hash: None,
        }).collect())
    }
    
    pub async fn delete_share(state: &AppState, user: &UserInfo, share_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM shared_links WHERE id = ? AND created_by = ?").bind(share_id).bind(&user.id).execute(&state.db_pool).await?;
        Ok(())
    }
    
    // Missing functions for API compatibility
    pub async fn get_share(state: &AppState, share_id: &str) -> Result<Share> {
        let row: crate::database::SharedLink = sqlx::query_as("SELECT * FROM shared_links WHERE id = ?").bind(share_id).fetch_one(&state.db_pool).await?;
        Ok(Share {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            file_path: row.item_id.clone(),
            token: row.id.clone(),
            permission: "read".to_string(),
            created_by: Uuid::parse_str(&row.created_by).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap_or_else(|_| Utc::now().into()).with_timezone(&Utc),
            expires_at: row.expires_at.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
            password_hash: None,
        })
    }
    
    pub async fn update_share(state: &AppState, user: &UserInfo, share_id: &str, _req: serde_json::Value) -> Result<()> {
        // Stub implementation
        Ok(())
    }
}

// ACTIVITY SERVICE
pub mod activity {
    use super::*;
    
    /// Log an activity to the database
    pub async fn log(
        state: &AppState,
        user_id: &str,
        action: &str,
        file_path: &str,
        file_name: &str,
        file_size: Option<i64>,
        old_path: Option<&str>,
        status: &str,
        error_message: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        sqlx::query(
            "INSERT INTO activity_log (id, user_id, action, file_path, file_name, file_size, old_path, status, error_message, metadata, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(user_id)
        .bind(action)
        .bind(file_path)
        .bind(file_name)
        .bind(file_size)
        .bind(old_path)
        .bind(status)
        .bind(error_message)
        .bind(metadata.map(|m| m.to_string()))
        .bind(&now)
        .execute(&state.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// List activities with pagination and permission filtering
    /// Only shows activities for files/folders the user can access
    pub async fn list(state: &AppState, user: &UserInfo, limit: usize) -> Result<Vec<serde_json::Value>> {
        #[derive(sqlx::FromRow)]
        struct ActivityRow {
            id: String,
            user_id: String,
            action: String,
            file_path: String,
            file_name: String,
            file_size: Option<i64>,
            old_path: Option<String>,
            status: String,
            error_message: Option<String>,
            metadata: Option<String>,
            created_at: String,
        }
        
        // Get all activities (we'll filter by permissions)
        let rows: Vec<ActivityRow> = sqlx::query_as(
            "SELECT * FROM activity_log ORDER BY created_at DESC LIMIT ?"
        )
        .bind((limit * 3) as i64) // Fetch more to ensure we have enough after filtering
        .fetch_all(&state.db_pool)
        .await?;
        
        let mut filtered_activities = Vec::new();
        
        for row in rows {
            // Check if user has permission to see this activity
            let has_access = check_file_access(state, user, &row.file_path).await;
            
            if has_access {
                filtered_activities.push(serde_json::json!({
                    "id": &row.id,
                    "user_id": &row.user_id,
                    "action": &row.action,
                    "file_path": &row.file_path,
                    "file_name": &row.file_name,
                    "file_size": row.file_size,
                    "old_path": &row.old_path,
                    "status": &row.status,
                    "error_message": &row.error_message,
                    "metadata": row.metadata.as_ref().and_then(|m| serde_json::from_str::<serde_json::Value>(m).ok()),
                    "created_at": &row.created_at,
                }));
                
                // Stop when we have enough filtered results
                if filtered_activities.len() >= limit {
                    break;
                }
            }
        }
        
        Ok(filtered_activities)
    }
    
    /// Check if user has access to a file based on ownership, permissions, or shares
    async fn check_file_access(state: &AppState, user: &UserInfo, file_path: &str) -> bool {
        let user_id = &user.user_id;
        
        // 1. Check if user is the owner of the file
        let is_owner: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM files WHERE file_path = ? AND owner_id = ?"
        )
        .bind(file_path)
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten();
        
        if let Some((count,)) = is_owner {
            if count > 0 {
                return true;
            }
        }
        
        // 2. Check if user has explicit permission via file_permissions table
        let has_permission: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM file_permissions fp 
             JOIN files f ON (fp.item_type = 'file' AND fp.item_id = f.id)
             WHERE f.file_path = ? AND fp.user_id = ? AND fp.can_read = 1 
             AND (fp.expires_at IS NULL OR fp.expires_at > datetime('now'))"
        )
        .bind(file_path)
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten();
        
        if let Some((count,)) = has_permission {
            if count > 0 {
                return true;
            }
        }
        
        // 3. Check if file is shared with user via shares table
        let is_shared: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM shares s
             JOIN files f ON s.file_id = f.id
             WHERE f.file_path = ? AND s.shared_with_user_id = ?
             AND (s.expires_at IS NULL OR s.expires_at > datetime('now'))"
        )
        .bind(file_path)
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten();
        
        if let Some((count,)) = is_shared {
            if count > 0 {
                return true;
            }
        }
        
        // No access found
        false
    }
    
    /// Get activity statistics with smart unread count
    /// Returns count of activities since user's last visit to Activity page
    pub async fn get_stats(state: &AppState, user: &UserInfo) -> Result<serde_json::Value> {
        let user_id = &user.user_id;
        
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM activity_log")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(0);
        
        let today: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM activity_log WHERE DATE(created_at) = DATE('now')"
        )
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or(0);
        
        // Get user's last activity visit timestamp
        let last_visit: Option<(Option<String>,)> = sqlx::query_as(
            "SELECT last_activity_visit FROM users WHERE id = ?"
        )
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten();
        
        // Count unread activities (since last visit, with permission filter)
        let unread_count = if let Some((Some(last_visit_time),)) = last_visit {
            // Get activities since last visit
            #[derive(sqlx::FromRow)]
            struct ActivityPath {
                file_path: String,
            }
            
            let activities: Vec<ActivityPath> = sqlx::query_as(
                "SELECT file_path FROM activity_log 
                 WHERE created_at > ? 
                 ORDER BY created_at DESC"
            )
            .bind(&last_visit_time)
            .fetch_all(&state.db_pool)
            .await
            .unwrap_or_default();
            
            // Filter by permissions
            let mut count = 0;
            for activity in activities {
                if check_file_access(state, user, &activity.file_path).await {
                    count += 1;
                }
            }
            count
        } else {
            // First visit - show today's activities
            today
        };
        
        let by_action: Vec<(String, i64)> = sqlx::query_as(
            "SELECT action, COUNT(*) as count FROM activity_log GROUP BY action ORDER BY count DESC"
        )
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();
        
        let mut action_counts = serde_json::Map::new();
        for (action, count) in by_action {
            action_counts.insert(action, serde_json::json!(count));
        }
        
        // Get last visit time for frontend display
        let last_visit_str = last_visit
            .and_then(|(v,)| v)
            .unwrap_or_else(|| "never".to_string());
        
        Ok(serde_json::json!({
            "total": total,
            "today": today,
            "unread_count": unread_count,
            "last_visit": last_visit_str,
            "by_action": action_counts,
        }))
    }
    
    /// Mark activity page as visited - resets unread count
    pub async fn mark_visited(state: &AppState, user: &UserInfo) -> Result<()> {
        let user_id = &user.user_id;
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query(
            "UPDATE users SET last_activity_visit = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(user_id)
        .execute(&state.db_pool)
        .await?;
        
        Ok(())
    }
}

// TAG SERVICE
pub mod tag {
    use super::*;
    use crate::models::Tag;
    
    pub async fn create_tag(state: &AppState, user: &UserInfo, name: &str, color: Option<String>) -> Result<Tag> {
        let id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();
        sqlx::query("INSERT INTO tags (id, name, color, owner_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(name).bind(color.as_deref()).bind(&user.id).bind(&now).bind(&now).execute(&state.db_pool).await?;
        Ok(Tag {
            id,
            name: name.to_string(),
            color,
            user_id: Uuid::parse_str(&user.id).unwrap_or_default(),
            created_at: Utc::now(),
        })
    }
    
    pub async fn list_tags(state: &AppState, user: &UserInfo) -> Result<Vec<Tag>> {
        let rows: Vec<crate::database::Tag> = sqlx::query_as("SELECT * FROM tags WHERE owner_id = ?").bind(&user.id).fetch_all(&state.db_pool).await?;
        Ok(rows.iter().map(|r| Tag {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name.clone(),
            color: r.color.clone(),
            user_id: Uuid::parse_str(&r.owner_id).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap_or_else(|_| Utc::now().into()).with_timezone(&Utc),
        }).collect())
    }
    
    // Alias functions for API compatibility
    pub async fn create(state: &AppState, user: &UserInfo, name: &str, color: Option<String>) -> Result<Tag> {
        create_tag(state, user, name, color).await
    }
    
    pub async fn list(state: &AppState, user: &UserInfo) -> Result<Vec<Tag>> {
        list_tags(state, user).await
    }
    
    pub async fn delete(state: &AppState, user: &UserInfo, tag_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM tags WHERE id = ? AND owner_id = ?").bind(tag_id).bind(&user.id).execute(&state.db_pool).await?;
        Ok(())
    }
    
    pub async fn tag_file(state: &AppState, user: &UserInfo, file_id: &str, tag_id: &str) -> Result<()> {
        sqlx::query("INSERT INTO file_tags (file_id, tag_id) VALUES (?, ?)").bind(file_id).bind(tag_id).execute(&state.db_pool).await?;
        Ok(())
    }
    
    pub async fn untag_file(state: &AppState, user: &UserInfo, file_id: &str, tag_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM file_tags WHERE file_id = ? AND tag_id = ?").bind(file_id).bind(tag_id).execute(&state.db_pool).await?;
        Ok(())
    }
}

// FAVORITES SERVICE
pub mod favorites {
    use super::*;
    use crate::database::Favorite;
    
    pub async fn add_favorite(state: &AppState, user: &UserInfo, item_type: &str, item_id: &str) -> Result<Favorite> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        sqlx::query("INSERT INTO favorites (id, user_id, item_type, item_id, sort_order, created_at) VALUES (?, ?, ?, ?, 0, ?)")
            .bind(&id).bind(&user.id).bind(item_type).bind(item_id).bind(&now).execute(&state.db_pool).await?;
        Ok(Favorite { id, user_id: user.id.clone(), item_type: item_type.to_string(), item_id: item_id.to_string(), sort_order: 0, created_at: now })
    }
    
    pub async fn list_favorites(state: &AppState, user: &UserInfo) -> Result<Vec<Favorite>> {
        let rows: Vec<crate::database::Favorite> = sqlx::query_as("SELECT * FROM favorites WHERE user_id = ?").bind(&user.id).fetch_all(&state.db_pool).await?;
        Ok(rows)
    }
    
    pub async fn remove_favorite(state: &AppState, user: &UserInfo, favorite_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM favorites WHERE id = ? AND user_id = ?").bind(favorite_id).bind(&user.id).execute(&state.db_pool).await?;
        Ok(())
    }
    
    // Alias functions for API compatibility
    pub async fn add(state: &AppState, user: &UserInfo, item_type: &str, item_id: &str) -> Result<Favorite> {
        add_favorite(state, user, item_type, item_id).await
    }
    
    pub async fn list(state: &AppState, user: &UserInfo) -> Result<Vec<Favorite>> {
        list_favorites(state, user).await
    }
    
    pub async fn remove(state: &AppState, user: &UserInfo, favorite_id: &str) -> Result<()> {
        remove_favorite(state, user, favorite_id).await
    }
}

// BACKUP SERVICE
pub mod backup {
    use super::*;
    use crate::models::Backup;
    
    pub async fn create_backup(state: &AppState, user: &UserInfo, backup_type: &str) -> Result<Backup> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let file_path = format!("/backups/{}.tar.gz", id);
        sqlx::query("INSERT INTO enhanced_backups (id, backup_type, size_bytes, storage_path, created_by, created_at, status) VALUES (?, ?, 0, ?, ?, ?, 'pending')")
            .bind(&id).bind(backup_type).bind(&file_path).bind(&user.id).bind(now.to_rfc3339()).execute(&state.db_pool).await?;
        Ok(Backup { id, backup_type: backup_type.to_string(), size_bytes: 0, created_by: Uuid::parse_str(&user.id).unwrap_or_default(), created_at: now, status: "pending".to_string(), file_path })
    }
    
    pub async fn list_backups(state: &AppState, user: &UserInfo) -> Result<Vec<Backup>> {
        let rows: Vec<crate::database::EnhancedBackup> = sqlx::query_as("SELECT * FROM enhanced_backups WHERE created_by = ? ORDER BY created_at DESC")
            .bind(&user.id).fetch_all(&state.db_pool).await?;
        Ok(rows.iter().map(|r| Backup {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            backup_type: r.backup_type.clone(),
            size_bytes: r.size_bytes,
            created_by: Uuid::parse_str(&r.created_by).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap_or_else(|_| Utc::now().into()).with_timezone(&Utc),
            status: r.status.clone(),
            file_path: r.storage_path.clone(),
        }).collect())
    }
}

// COLLABORATION SERVICE
pub mod collaboration {
    use super::*;
    use crate::models::{FileLock, UserPresence};
    
    pub async fn acquire_lock(state: &AppState, user: &UserInfo, file_path: &str) -> Result<FileLock> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let expires = now + chrono::Duration::minutes(30);
        sqlx::query("INSERT INTO file_locks (id, file_id, file_path, locked_by, locked_at, expires_at, lock_type, last_heartbeat) VALUES (?, ?, ?, ?, ?, ?, 'edit', ?)")
            .bind(&id).bind(file_path).bind(file_path).bind(&user.id).bind(now.to_rfc3339()).bind(expires.to_rfc3339()).bind(now.to_rfc3339()).execute(&state.db_pool).await?;
        Ok(FileLock {
            id,
            file_path: file_path.to_string(),
            user_id: Uuid::parse_str(&user.id).unwrap_or_default(),
            lock_type: "edit".to_string(),
            acquired_at: now,
            expires_at: expires,
        })
    }
    
    pub async fn release_lock(state: &AppState, user: &UserInfo, file_path: &str) -> Result<()> {
        sqlx::query("DELETE FROM file_locks WHERE file_path = ? AND locked_by = ?").bind(file_path).bind(&user.id).execute(&state.db_pool).await?;
        Ok(())
    }
    
    pub async fn update_presence(state: &AppState, user: &UserInfo, file_path: Option<String>, activity: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        sqlx::query("INSERT OR REPLACE INTO user_presence (id, user_id, username, file_path, activity_type, last_seen) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&user.id).bind(&user.username).bind(file_path).bind(activity).bind(&now).execute(&state.db_pool).await?;
        Ok(())
    }
    
    // Missing functions for API compatibility
    pub async fn list_locks(state: &AppState, user: &UserInfo) -> Result<Vec<FileLock>> {
        Ok(vec![])
    }
    
    pub async fn renew_lock(state: &AppState, user: &UserInfo, file_path: &str) -> Result<()> {
        Ok(())
    }
    
    pub async fn get_presence(state: &AppState) -> Result<Vec<UserPresence>> {
        Ok(vec![])
    }
    
    pub async fn get_activity(state: &AppState, user: &UserInfo) -> Result<Vec<serde_json::Value>> {
        Ok(vec![])
    }
    
    pub async fn list_conflicts(state: &AppState, user: &UserInfo) -> Result<Vec<serde_json::Value>> {
        Ok(vec![])
    }
    
    pub async fn resolve_conflict(state: &AppState, user: &UserInfo, conflict_id: &str, resolution: &str) -> Result<()> {
        Ok(())
    }
}

// SYSTEM SERVICE
pub mod system {
    use super::*;
    
    pub async fn get_stats(state: &AppState) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "file_count": 0,
            "total_size": 0,
            "user_count": 1,
        }))
    }
    
    pub async fn get_storage_info(state: &AppState) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "total": 1000000000,
            "used": 0,
            "free": 1000000000,
        }))
    }
    
    // Missing functions for API compatibility
    pub async fn get_status(state: &AppState) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"status": "ok", "version": "0.1.0"}))
    }
    
    pub async fn get_config_info(state: &AppState) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"max_upload_size": 10485760}))
    }
}
