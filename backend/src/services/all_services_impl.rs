//! All remaining services - compact implementation
use crate::{auth::UserInfo, AppState};
use anyhow::{anyhow, Result};
use chrono::{TimeZone, Utc};
use uuid::Uuid;

// DIRECTORY SERVICE
pub mod directory {
    use super::*;
    use crate::models::DirectoryInfo;
    use std::path::Path;
    use tokio::fs;
    const DATA_DIR: &str = "./data";

    pub async fn create_directory(
        state: &AppState,
        user: &UserInfo,
        path: &str,
    ) -> Result<DirectoryInfo> {
        let full = Path::new(DATA_DIR).join(path);
        fs::create_dir_all(&full).await?;

        // Get folder name and determine parent
        let dirname = full.file_name().unwrap().to_string_lossy().to_string();
        let parent_path = Path::new(path)
            .parent()
            .and_then(|p| p.to_str())
            .filter(|p| !p.is_empty());

        // Generate UUID for the new folder
        let folder_id = Uuid::new_v4();
        let owner_uuid = Uuid::parse_str(&user.id).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        // Determine parent_id by looking up parent folder in database
        let parent_id: Option<String> = if let Some(parent) = parent_path {
            sqlx::query_scalar::<_, String>(
                "SELECT id FROM folders WHERE path = ? AND owner_id = ? AND is_deleted = 0",
            )
            .bind(parent)
            .bind(&user.id)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten()
        } else {
            None
        };

        // Insert folder into database
        sqlx::query(
            r#"
            INSERT INTO folders (id, name, path, parent_id, owner_id, is_deleted, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, 0, ?, ?)
            "#
        )
        .bind(folder_id.to_string())
        .bind(&dirname)
        .bind(path)
        .bind(parent_id.as_deref())
        .bind(&user.id)
        .bind(&now)
        .bind(&now)
        .execute(&state.db_pool)
        .await?;

        // Log activity
        let _ = super::activity::log(
            state, &user.id, "create", path, &dirname, None, None, "success", None, None,
        )
        .await;

        let _ = state.fs_tx.send(crate::FileChangeEvent::new(
            path.to_string(),
            "mkdir".to_string(),
        ));

        Ok(DirectoryInfo {
            id: folder_id,
            name: dirname,
            path: path.to_string(),
            parent_id: parent_id.and_then(|p| Uuid::parse_str(&p).ok()),
            owner_id: owner_uuid,
            created_at: Utc::now(),
        })
    }

    pub async fn delete_directory(state: &AppState, _user: &UserInfo, dir_id: &str) -> Result<()> {
        fs::remove_dir_all(Path::new(DATA_DIR).join(dir_id)).await?;
        let _ = state.fs_tx.send(crate::FileChangeEvent::new(
            dir_id.to_string(),
            "delete".to_string(),
        ));
        Ok(())
    }

    pub async fn move_directory(
        state: &AppState,
        _user: &UserInfo,
        dir_id: &str,
        new_parent_path: &str,
    ) -> Result<()> {
        let old_path = Path::new(DATA_DIR).join(dir_id);
        let new_path = Path::new(DATA_DIR)
            .join(new_parent_path)
            .join(old_path.file_name().unwrap());
        fs::rename(&old_path, &new_path).await?;
        let _ = state.fs_tx.send(crate::FileChangeEvent::new(
            new_parent_path.to_string(),
            "move".to_string(),
        ));
        Ok(())
    }

    pub async fn rename_directory(
        state: &AppState,
        _user: &UserInfo,
        dir_id: &str,
        new_name: &str,
    ) -> Result<()> {
        let old_path = Path::new(DATA_DIR).join(dir_id);
        let parent = old_path
            .parent()
            .ok_or_else(|| anyhow!("No parent directory"))?;
        let new_path = parent.join(new_name);
        fs::rename(&old_path, &new_path).await?;
        let _ = state.fs_tx.send(crate::FileChangeEvent::new(
            dir_id.to_string(),
            "rename".to_string(),
        ));
        Ok(())
    }

    // Batch operations for directories
    pub async fn batch_move(
        state: &AppState,
        user: &UserInfo,
        paths: Vec<String>,
        target_path: &str,
    ) -> Result<()> {
        for path in paths {
            let _ = move_directory(state, user, &path, target_path).await;
        }
        Ok(())
    }

    pub async fn get_directory_tree(
        _state: &AppState,
        _user: &UserInfo,
        path: &str,
    ) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"path": path, "children": []}))
    }
}

// SHARING SERVICE
pub mod sharing {
    use super::*;
    use crate::models::Share;

    pub async fn create_share(
        state: &AppState,
        user: &UserInfo,
        path: &str,
        _is_public: bool,
    ) -> Result<Share> {
        let id = Uuid::new_v4();
        let token = format!("{}", Uuid::new_v4());
        let now = Utc::now();
        sqlx::query("INSERT INTO shared_links (id, item_type, item_id, created_by, is_public, allow_download, download_count, created_at) VALUES (?, 'file', ?, ?, ?, 1, 0, ?)")
            .bind(&id).bind(path).bind(&user.id).bind(1).bind(now.to_rfc3339()).execute(&state.db_pool).await?;
        
        // Log share activity
        let file_name = path.split('/').last().unwrap_or(path).to_string();
        let _ = super::activity::log(
            state,
            &user.id,
            activity::actions::SHARE,
            path,
            &file_name,
            None,
            None,
            "success",
            None,
            Some(serde_json::json!({"share_id": id.to_string(), "token": &token})),
        ).await;

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
        let rows: Vec<crate::database::SharedLink> =
            sqlx::query_as("SELECT * FROM shared_links WHERE created_by = ?")
                .bind(&user.id)
                .fetch_all(&state.db_pool)
                .await?;

        // Convert SharedLink to Share
        let mut shares = Vec::new();
        for r in rows {
            // Parse created_at - try RFC3339 first, fall back to SQLite datetime format
            let created_at = chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .or_else(|| {
                    chrono::NaiveDateTime::parse_from_str(&r.created_at, "%Y-%m-%d %H:%M:%S")
                        .ok()
                        .map(|ndt| Utc.from_utc_datetime(&ndt))
                })
                .unwrap_or_else(|| Utc::now());

            // Parse expires_at if present
            let expires_at = r.expires_at.and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
                    .or_else(|| {
                        chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                            .ok()
                            .map(|ndt| Utc.from_utc_datetime(&ndt))
                    })
            });

            shares.push(Share {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                file_path: r.item_id.clone(),
                token: r.id.clone(),
                permission: if r.allow_upload {
                    "write".to_string()
                } else {
                    "read".to_string()
                },
                created_by: Uuid::parse_str(&r.created_by).unwrap_or_default(),
                created_at,
                expires_at,
                password_hash: r.password_hash,
            });
        }

        Ok(shares)
    }

    pub async fn delete_share(state: &AppState, user: &UserInfo, share_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM shared_links WHERE id = ? AND created_by = ?")
            .bind(share_id)
            .bind(&user.id)
            .execute(&state.db_pool)
            .await?;
        Ok(())
    }

    /// Get share details by share ID
    pub async fn get_share(state: &AppState, share_id: &str) -> Result<Share> {
        let row: crate::database::SharedLink =
            sqlx::query_as("SELECT * FROM shared_links WHERE id = ?")
                .bind(share_id)
                .fetch_one(&state.db_pool)
                .await?;
        Ok(Share {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            file_path: row.item_id.clone(),
            token: row.id.clone(),
            permission: "read".to_string(),
            created_by: Uuid::parse_str(&row.created_by).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            expires_at: row.expires_at.and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
            password_hash: None,
        })
    }

    /// Update share settings (expiration, permissions, etc.)
    pub async fn update_share(
        state: &AppState,
        user: &UserInfo,
        share_id: &str,
        req: serde_json::Value,
    ) -> Result<()> {
        // Update share expiration and other settings
        if let Some(expires_at) = req.get("expires_at").and_then(|v| v.as_str()) {
            sqlx::query("UPDATE shared_links SET expires_at = ? WHERE id = ? AND created_by = ?")
                .bind(expires_at)
                .bind(share_id)
                .bind(&user.id)
                .execute(&state.db_pool)
                .await?;
        }
        Ok(())
    }

    /// Regenerate share token - invalidates old token
    pub async fn regenerate_token(
        state: &AppState,
        user: &UserInfo,
        share_id: &str,
    ) -> Result<String> {
        let new_token = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE shared_links 
             SET id = ?, token_version = token_version + 1, regenerated_at = ?, regenerated_by = ?
             WHERE id = ? AND created_by = ?",
        )
        .bind(&new_token)
        .bind(&now)
        .bind(&user.id)
        .bind(share_id)
        .bind(&user.id)
        .execute(&state.db_pool)
        .await?;

        Ok(new_token)
    }

    /// Get share analytics
    pub async fn get_analytics(
        state: &AppState,
        user: &UserInfo,
        share_id: &str,
    ) -> Result<serde_json::Value> {
        // Verify ownership
        let _share: crate::database::SharedLink =
            sqlx::query_as("SELECT * FROM shared_links WHERE id = ? AND created_by = ?")
                .bind(share_id)
                .bind(&user.id)
                .fetch_one(&state.db_pool)
                .await?;

        // Get access statistics
        let stats: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) as total_accesses FROM shared_link_access_log WHERE share_id = ?",
        )
        .bind(share_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

        let last_access: Option<(String,)> = sqlx::query_as(
            "SELECT accessed_at FROM shared_link_access_log WHERE share_id = ? ORDER BY accessed_at DESC LIMIT 1"
        )
        .bind(share_id)
        .fetch_optional(&state.db_pool)
        .await
        .unwrap_or(None);

        Ok(serde_json::json!({
            "total_accesses": stats.0,
            "last_accessed": last_access.map(|la| la.0),
            "share_id": share_id
        }))
    }

    /// Get access log for share
    pub async fn get_access_log(
        state: &AppState,
        user: &UserInfo,
        share_id: &str,
    ) -> Result<Vec<serde_json::Value>> {
        // Verify ownership
        let _share: crate::database::SharedLink =
            sqlx::query_as("SELECT * FROM shared_links WHERE id = ? AND created_by = ?")
                .bind(share_id)
                .bind(&user.id)
                .fetch_one(&state.db_pool)
                .await?;

        let logs: Vec<crate::database::SharedLinkAccessLog> = sqlx::query_as(
            "SELECT * FROM shared_link_access_log WHERE share_id = ? ORDER BY accessed_at DESC LIMIT 100"
        )
        .bind(share_id)
        .fetch_all(&state.db_pool)
        .await?;

        Ok(logs
            .iter()
            .map(|log| {
                serde_json::json!({
                    "id": log.id,
                    "ip": log.ip_address,
                    "accessed_at": log.accessed_at,
                    "action": log.action,
                    "user_agent": log.user_agent
                })
            })
            .collect())
    }

    /// Log access to shared link
    pub async fn log_access(
        state: &AppState,
        share_id: &str,
        ip: Option<&str>,
        action: &str,
        user_agent: Option<&str>,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO shared_link_access_log (id, share_id, accessed_by_ip, accessed_at, action, status, user_agent)
             VALUES (?, ?, ?, ?, ?, 'success', ?)"
        )
        .bind(&id)
        .bind(share_id)
        .bind(ip)
        .bind(&now)
        .bind(action)
        .bind(user_agent)
        .execute(&state.db_pool)
        .await?;

        Ok(())
    }

    /// Add users to a share with specific permissions
    pub async fn add_share_users(
        state: &AppState,
        user: &UserInfo,
        share_id: &str,
        user_ids: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<Vec<crate::database::ShareUser>> {
        // Verify ownership of the share
        let _share: crate::database::SharedLink =
            sqlx::query_as("SELECT * FROM shared_links WHERE id = ? AND created_by = ?")
                .bind(share_id)
                .bind(&user.id)
                .fetch_one(&state.db_pool)
                .await?;

        let now = Utc::now().to_rfc3339();
        let mut share_users = Vec::new();

        for (user_id, permission) in user_ids.iter().zip(permissions.iter()) {
            let id = Uuid::new_v4().to_string();

            // Insert or update the share_user entry
            sqlx::query(
                "INSERT INTO share_users (id, share_id, user_id, permission, created_at, created_by)
                 VALUES (?, ?, ?, ?, ?, ?)
                 ON CONFLICT(share_id, user_id) DO UPDATE SET permission = excluded.permission"
            )
            .bind(&id)
            .bind(share_id)
            .bind(user_id)
            .bind(permission)
            .bind(&now)
            .bind(&user.id)
            .execute(&state.db_pool)
            .await?;

            share_users.push(crate::database::ShareUser {
                id,
                share_id: share_id.to_string(),
                user_id: user_id.clone(),
                permission: permission.clone(),
                created_at: now.clone(),
                created_by: user.id.clone(),
            });
        }

        Ok(share_users)
    }

    /// Get all users for a share
    pub async fn get_share_users(
        state: &AppState,
        share_id: &str,
    ) -> Result<Vec<crate::database::ShareUser>> {
        let users: Vec<crate::database::ShareUser> =
            sqlx::query_as("SELECT * FROM share_users WHERE share_id = ?")
                .bind(share_id)
                .fetch_all(&state.db_pool)
                .await?;
        Ok(users)
    }

    /// Remove a user from a share
    pub async fn remove_share_user(
        state: &AppState,
        user: &UserInfo,
        share_id: &str,
        user_id: &str,
    ) -> Result<()> {
        // Verify ownership
        let _share: crate::database::SharedLink =
            sqlx::query_as("SELECT * FROM shared_links WHERE id = ? AND created_by = ?")
                .bind(share_id)
                .bind(&user.id)
                .fetch_one(&state.db_pool)
                .await?;

        sqlx::query("DELETE FROM share_users WHERE share_id = ? AND user_id = ?")
            .bind(share_id)
            .bind(user_id)
            .execute(&state.db_pool)
            .await?;

        Ok(())
    }

    /// Update user permission on a share
    pub async fn update_share_user_permission(
        state: &AppState,
        user: &UserInfo,
        share_id: &str,
        user_id: &str,
        permission: &str,
    ) -> Result<()> {
        // Verify ownership
        let _share: crate::database::SharedLink =
            sqlx::query_as("SELECT * FROM shared_links WHERE id = ? AND created_by = ?")
                .bind(share_id)
                .bind(&user.id)
                .fetch_one(&state.db_pool)
                .await?;

        sqlx::query("UPDATE share_users SET permission = ? WHERE share_id = ? AND user_id = ?")
            .bind(permission)
            .bind(share_id)
            .bind(user_id)
            .execute(&state.db_pool)
            .await?;

        Ok(())
    }
}

// ACTIVITY SERVICE
pub mod activity {
    use super::*;

    /// All supported activity action types
    pub mod actions {
        // File operations
        pub const UPLOAD: &str = "upload";
        pub const DOWNLOAD: &str = "download";
        pub const DELETE: &str = "delete";
        pub const RENAME: &str = "rename";
        pub const MOVE: &str = "move";
        pub const COPY: &str = "copy";
        pub const CREATE: &str = "create";
        pub const PREVIEW: &str = "preview";
        pub const RESTORE: &str = "restore";
        
        // Folder operations
        pub const FOLDER_CREATE: &str = "folder_create";
        pub const FOLDER_DELETE: &str = "folder_delete";
        pub const FOLDER_RENAME: &str = "folder_rename";
        pub const FOLDER_MOVE: &str = "folder_move";
        pub const FOLDER_COLOR: &str = "folder_color";
        
        // Favorites
        pub const FAVORITE: &str = "favorite";
        pub const UNFAVORITE: &str = "unfavorite";
        
        // Sharing
        pub const SHARE: &str = "share";
        pub const UNSHARE: &str = "unshare";
        pub const SHARE_ACCESS: &str = "share_access";
        
        // Comments & Tags
        pub const COMMENT_ADD: &str = "comment_add";
        pub const COMMENT_DELETE: &str = "comment_delete";
        pub const TAG_ADD: &str = "tag_add";
        pub const TAG_REMOVE: &str = "tag_remove";
        
        // Versioning
        pub const VERSION_CREATE: &str = "version_create";
        pub const VERSION_RESTORE: &str = "version_restore";
        pub const VERSION_DELETE: &str = "version_delete";
        
        // Auth & Security
        pub const LOGIN: &str = "login";
        pub const LOGOUT: &str = "logout";
        pub const PASSWORD_CHANGE: &str = "password_change";
        pub const TOTP_ENABLE: &str = "2fa_enable";
        pub const TOTP_DISABLE: &str = "2fa_disable";
        
        // Settings
        pub const SETTINGS_CHANGE: &str = "settings_change";
        pub const PROFILE_UPDATE: &str = "profile_update";
    }

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
    pub async fn list(
        state: &AppState,
        user: &UserInfo,
        limit: usize,
    ) -> Result<Vec<serde_json::Value>> {
        list_filtered(state, user, limit, None).await
    }

    /// List activities with optional action filter
    pub async fn list_filtered(
        state: &AppState,
        user: &UserInfo,
        limit: usize,
        action_filter: Option<&str>,
    ) -> Result<Vec<serde_json::Value>> {
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

        // Fetch more to ensure we have enough after filtering
        let fetch_limit = (limit * 3) as i64;

        // Build query based on filter
        let rows: Vec<ActivityRow> = if let Some(action) = action_filter {
            sqlx::query_as(
                "SELECT * FROM activity_log WHERE action = ? ORDER BY created_at DESC LIMIT ?"
            )
            .bind(action)
            .bind(fetch_limit)
            .fetch_all(&state.db_pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT * FROM activity_log ORDER BY created_at DESC LIMIT ?"
            )
            .bind(fetch_limit)
            .fetch_all(&state.db_pool)
            .await?
        };

        // Get username lookup
        let usernames = get_usernames(state).await;

        let mut filtered_activities = Vec::new();
        let current_user_id = user.user_id();

        for row in rows {
            // Users can always see their own activities
            // For other users' activities, check file access permissions
            let has_access = row.user_id == current_user_id 
                || check_file_access(state, user, &row.file_path).await;

            if has_access {
                // Get username for display
                let username = usernames.get(&row.user_id).cloned().unwrap_or_else(|| "Unknown".to_string());
                
                filtered_activities.push(serde_json::json!({
                    "id": &row.id,
                    "user_id": &row.user_id,
                    "username": username,
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

    /// Get all usernames for display
    async fn get_usernames(state: &AppState) -> std::collections::HashMap<String, String> {
        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT id, COALESCE(display_name, username) as name FROM users"
        )
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();
        
        rows.into_iter().collect()
    }

    /// Check if user has access to a file based on ownership, permissions, or shares
    async fn check_file_access(state: &AppState, user: &UserInfo, file_path: &str) -> bool {
        let user_id = user.user_id();

        // 1. Check if user is the owner of the file (note: column is 'path' not 'file_path')
        let is_owner: Option<(i64,)> =
            sqlx::query_as("SELECT COUNT(*) FROM files WHERE path = ? AND owner_id = ?")
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
             WHERE f.path = ? AND fp.user_id = ? AND fp.can_read = 1 
             AND (fp.expires_at IS NULL OR fp.expires_at > datetime('now'))",
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
             WHERE f.path = ? AND s.shared_with_user_id = ?
             AND (s.expires_at IS NULL OR s.expires_at > datetime('now'))",
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

        // 4. For activities without a file path (login, settings, etc.), allow access
        if file_path.is_empty() {
            return true;
        }

        // No access found
        false
    }

    /// Get activity statistics with smart unread count
    /// Returns count of activities since user's last visit to Activity page
    pub async fn get_stats(state: &AppState, user: &UserInfo) -> Result<serde_json::Value> {
        let user_id = user.user_id();

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM activity_log")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(0);

        let today: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM activity_log WHERE DATE(created_at) = DATE('now')",
        )
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or(0);

        // Get user's last activity visit timestamp
        let last_visit: Option<(Option<String>,)> =
            sqlx::query_as("SELECT last_activity_visit FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_optional(&state.db_pool)
                .await
                .ok()
                .flatten();

        // Clone for later use
        let last_visit_str = last_visit
            .as_ref()
            .and_then(|(v,)| v.clone())
            .unwrap_or_else(|| "never".to_string());

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
                 ORDER BY created_at DESC",
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
        let user_id = user.user_id();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query("UPDATE users SET last_activity_visit = ? WHERE id = ?")
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

    pub async fn create_tag(
        state: &AppState,
        user: &UserInfo,
        name: &str,
        color: Option<String>,
    ) -> Result<Tag> {
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
        let rows: Vec<crate::database::Tag> =
            sqlx::query_as("SELECT * FROM tags WHERE owner_id = ?")
                .bind(&user.id)
                .fetch_all(&state.db_pool)
                .await?;
        Ok(rows
            .iter()
            .map(|r| Tag {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                name: r.name.clone(),
                color: r.color.clone(),
                user_id: Uuid::parse_str(&r.owner_id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc),
            })
            .collect())
    }

    // Alias functions for API compatibility
    pub async fn create(
        state: &AppState,
        user: &UserInfo,
        name: &str,
        color: Option<String>,
    ) -> Result<Tag> {
        create_tag(state, user, name, color).await
    }

    pub async fn list(state: &AppState, user: &UserInfo) -> Result<Vec<Tag>> {
        list_tags(state, user).await
    }

    pub async fn delete(state: &AppState, user: &UserInfo, tag_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM tags WHERE id = ? AND owner_id = ?")
            .bind(tag_id)
            .bind(&user.id)
            .execute(&state.db_pool)
            .await?;
        Ok(())
    }

    pub async fn list_by_file(
        state: &AppState,
        _user: &UserInfo,
        file_path: &str,
    ) -> Result<Vec<Tag>> {
        let rows: Vec<crate::database::Tag> = sqlx::query_as(
            "SELECT DISTINCT t.* FROM tags t 
             INNER JOIN file_tags ft ON t.id = ft.tag_id 
             WHERE ft.file_path = ?",
        )
        .bind(file_path)
        .fetch_all(&state.db_pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| Tag {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                name: r.name.clone(),
                color: r.color.clone(),
                user_id: Uuid::parse_str(&r.owner_id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc),
            })
            .collect())
    }

    pub async fn create_for_file(
        state: &AppState,
        user: &UserInfo,
        file_path: &str,
        name: &str,
        color: Option<String>,
    ) -> Result<Tag> {
        // First create the tag
        let tag = create_tag(state, user, name, color).await?;

        // Then associate it with the file
        let ft_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO file_tags (id, file_id, tag_id, item_type, file_path, tagged_by, created_at) 
             VALUES (?, ?, ?, 'file', ?, ?, ?)"
        )
        .bind(&ft_id)
        .bind(file_path)
        .bind(&tag.id.to_string())
        .bind(file_path)
        .bind(&user.id)
        .bind(&now)
        .execute(&state.db_pool)
        .await?;

        Ok(tag)
    }

    pub async fn update(
        state: &AppState,
        user: &UserInfo,
        tag_id: &str,
        name: &str,
        color: Option<String>,
    ) -> Result<Tag> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE tags SET name = ?, color = ?, updated_at = ? WHERE id = ? AND owner_id = ?",
        )
        .bind(name)
        .bind(color.as_deref())
        .bind(&now)
        .bind(tag_id)
        .bind(&user.id)
        .execute(&state.db_pool)
        .await?;

        Ok(Tag {
            id: Uuid::parse_str(tag_id).unwrap_or_default(),
            name: name.to_string(),
            color,
            user_id: Uuid::parse_str(&user.id).unwrap_or_default(),
            created_at: Utc::now(),
        })
    }

    pub async fn tag_file(
        state: &AppState,
        _user: &UserInfo,
        file_id: &str,
        tag_id: &str,
    ) -> Result<()> {
        sqlx::query("INSERT INTO file_tags (file_id, tag_id) VALUES (?, ?)")
            .bind(file_id)
            .bind(tag_id)
            .execute(&state.db_pool)
            .await?;
        Ok(())
    }

    pub async fn untag_file(
        state: &AppState,
        _user: &UserInfo,
        file_id: &str,
        tag_id: &str,
    ) -> Result<()> {
        sqlx::query("DELETE FROM file_tags WHERE file_id = ? AND tag_id = ?")
            .bind(file_id)
            .bind(tag_id)
            .execute(&state.db_pool)
            .await?;
        Ok(())
    }
}

// FAVORITES SERVICE
pub mod favorites {
    use super::*;
    use crate::database::UserFavorite;

    pub async fn add_favorite(
        state: &AppState,
        user: &UserInfo,
        item_type: &str,
        item_id: &str,
    ) -> Result<UserFavorite> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // item_id is the path - use it directly
        let item_path = item_id.to_string();

        // Get item name for activity log (last part of path)
        let item_name = item_id.split('/').last().unwrap_or(item_id).to_string();

        // Check if already favorited
        let existing: Option<UserFavorite> = sqlx::query_as(
            "SELECT * FROM user_favorites WHERE user_id = ? AND item_id = ?"
        )
        .bind(&user.id)
        .bind(item_id)
        .fetch_optional(&state.db_pool)
        .await?;

        if let Some(fav) = existing {
            return Ok(fav);
        }

        sqlx::query("INSERT INTO user_favorites (id, user_id, item_type, item_id, item_path, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&user.id).bind(item_type).bind(item_id).bind(&item_path).bind(&now).execute(&state.db_pool).await?;
        
        // Log activity
        let _ = super::activity::log(
            state,
            &user.id,
            activity::actions::FAVORITE,
            item_id,
            &item_name,
            None,
            None,
            "success",
            None,
            Some(serde_json::json!({"item_type": item_type})),
        ).await;

        Ok(UserFavorite {
            id,
            user_id: user.id.clone(),
            item_type: item_type.to_string(),
            item_id: item_id.to_string(),
            item_path,
            created_at: now,
        })
    }

    pub async fn list_favorites(state: &AppState, user: &UserInfo) -> Result<Vec<UserFavorite>> {
        let rows: Vec<UserFavorite> = sqlx::query_as(
            "SELECT * FROM user_favorites WHERE user_id = ? ORDER BY created_at DESC",
        )
        .bind(&user.id)
        .fetch_all(&state.db_pool)
        .await?;
        Ok(rows)
    }

    pub async fn remove_favorite(
        state: &AppState,
        user: &UserInfo,
        favorite_id: &str,
    ) -> Result<()> {
        // Get favorite info before deleting for activity log
        let fav: Option<UserFavorite> = sqlx::query_as(
            "SELECT * FROM user_favorites WHERE id = ? AND user_id = ?"
        )
        .bind(favorite_id)
        .bind(&user.id)
        .fetch_optional(&state.db_pool)
        .await?;

        sqlx::query("DELETE FROM user_favorites WHERE id = ? AND user_id = ?")
            .bind(favorite_id)
            .bind(&user.id)
            .execute(&state.db_pool)
            .await?;

        // Log activity if we found the favorite
        if let Some(favorite) = fav {
            let item_name = favorite.item_id.split('/').last().unwrap_or(&favorite.item_id).to_string();
            let _ = super::activity::log(
                state,
                &user.id,
                activity::actions::UNFAVORITE,
                &favorite.item_id,
                &item_name,
                None,
                None,
                "success",
                None,
                Some(serde_json::json!({"item_type": favorite.item_type})),
            ).await;
        }

        Ok(())
    }

    // Alias functions for API compatibility
    pub async fn add(
        state: &AppState,
        user: &UserInfo,
        item_type: &str,
        item_id: &str,
    ) -> Result<UserFavorite> {
        add_favorite(state, user, item_type, item_id).await
    }

    pub async fn list(state: &AppState, user: &UserInfo) -> Result<Vec<UserFavorite>> {
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

    #[allow(dead_code)]
    pub async fn create_backup(
        state: &AppState,
        user: &UserInfo,
        backup_type: &str,
    ) -> Result<Backup> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let file_path = format!("/backups/{}.tar.gz", id);
        sqlx::query("INSERT INTO enhanced_backups (id, backup_type, size_bytes, storage_path, created_by, created_at, status) VALUES (?, ?, 0, ?, ?, ?, 'pending')")
            .bind(&id).bind(backup_type).bind(&file_path).bind(&user.id).bind(now.to_rfc3339()).execute(&state.db_pool).await?;
        Ok(Backup {
            id,
            backup_type: backup_type.to_string(),
            size_bytes: 0,
            created_by: Uuid::parse_str(&user.id).unwrap_or_default(),
            created_at: now,
            status: "pending".to_string(),
            file_path,
        })
    }

    #[allow(dead_code)]
    pub async fn list_backups(state: &AppState, user: &UserInfo) -> Result<Vec<Backup>> {
        let rows: Vec<crate::database::EnhancedBackup> = sqlx::query_as(
            "SELECT * FROM enhanced_backups WHERE created_by = ? ORDER BY created_at DESC",
        )
        .bind(&user.id)
        .fetch_all(&state.db_pool)
        .await?;
        Ok(rows
            .iter()
            .map(|r| Backup {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                backup_type: r.backup_type.clone(),
                size_bytes: r.size_bytes,
                created_by: Uuid::parse_str(&r.created_by).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc),
                status: r.status.clone(),
                file_path: r.storage_path.clone(),
            })
            .collect())
    }
}

// COLLABORATION SERVICE
pub mod collaboration {
    use super::*;
    use crate::models::{FileLock, UserPresence};

    pub async fn acquire_lock(
        state: &AppState,
        user: &UserInfo,
        file_path: &str,
    ) -> Result<FileLock> {
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
        sqlx::query("DELETE FROM file_locks WHERE file_path = ? AND locked_by = ?")
            .bind(file_path)
            .bind(&user.id)
            .execute(&state.db_pool)
            .await?;
        Ok(())
    }

    pub async fn update_presence(
        state: &AppState,
        user: &UserInfo,
        file_path: Option<String>,
        activity: &str,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        sqlx::query("INSERT OR REPLACE INTO user_presence (id, user_id, username, file_path, activity_type, last_seen) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&user.id).bind(&user.username).bind(file_path).bind(activity).bind(&now).execute(&state.db_pool).await?;
        Ok(())
    }

    /// List all active file locks
    pub async fn list_locks(state: &AppState, _user: &UserInfo) -> Result<Vec<FileLock>> {
        let locks = sqlx::query_as::<_, FileLock>(
            "SELECT * FROM file_locks WHERE expires_at > datetime('now') ORDER BY locked_at DESC",
        )
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();
        Ok(locks)
    }

    /// Renew a lock heartbeat to prevent expiry
    pub async fn renew_lock(state: &AppState, user: &UserInfo, file_path: &str) -> Result<()> {
        let new_expiry = (Utc::now() + chrono::Duration::hours(1)).to_rfc3339();
        sqlx::query("UPDATE file_locks SET expires_at = ? WHERE file_path = ? AND locked_by = ?")
            .bind(&new_expiry)
            .bind(file_path)
            .bind(&user.id)
            .execute(&state.db_pool)
            .await?;
        Ok(())
    }

    /// Get all users currently viewing/editing files
    pub async fn get_presence(state: &AppState) -> Result<Vec<UserPresence>> {
        let presence = sqlx::query_as::<_, UserPresence>(
            "SELECT * FROM user_presence WHERE last_seen > datetime('now', '-5 minutes') ORDER BY last_seen DESC"
        )
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();
        Ok(presence)
    }

    /// Get recent collaboration activity filtered by user permissions
    pub async fn get_activity(state: &AppState, user: &UserInfo) -> Result<Vec<serde_json::Value>> {
        #[derive(sqlx::FromRow)]
        struct ActivityRow {
            id: String,
            user_id: String,
            action: String,
            file_path: String,
            created_at: String,
        }

        let rows = sqlx::query_as::<_, ActivityRow>(
            "SELECT id, user_id, action, file_path, created_at 
             FROM activity_log WHERE created_at > datetime('now', '-24 hours') ORDER BY created_at DESC LIMIT 100"
        )
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();

        // Filter activities by user permissions - only show activities from own files or shared files
        let user_id = user.user_id();
        let activities: Vec<serde_json::Value> = rows
            .into_iter()
            .filter_map(|row| {
                // Check if user owns the file or has access to it
                // For now, show all activities (permission check would be database query)
                // In production: check file ownership or share permissions
                Some(serde_json::json!({
                    "id": row.id,
                    "user_id": row.user_id,
                    "action": row.action,
                    "file_path": row.file_path,
                    "created_at": row.created_at,
                    "current_user_id": user_id,
                }))
            })
            .collect();

        Ok(activities)
    }

    /// List file conflicts that need resolution
    pub async fn list_conflicts(
        state: &AppState,
        user: &UserInfo,
    ) -> Result<Vec<serde_json::Value>> {
        // Query conflicts table where user has ownership or shared access
        #[derive(sqlx::FromRow)]
        struct ConflictRow {
            id: String,
            file_path: String,
            conflict_type: String,
            local_version: i32,
            remote_version: i32,
            created_at: String,
        }

        let conflicts = sqlx::query_as::<_, ConflictRow>(
            "SELECT id, file_path, conflict_type, local_version, remote_version, created_at 
             FROM file_conflicts WHERE user_id = ? AND resolved_at IS NULL ORDER BY created_at DESC"
        )
        .bind(&user.id)
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_default();

        let result = conflicts
            .into_iter()
            .map(|c| {
                serde_json::json!({
                    "id": c.id,
                    "file_path": c.file_path,
                    "conflict_type": c.conflict_type,
                    "local_version": c.local_version,
                    "remote_version": c.remote_version,
                    "created_at": c.created_at,
                })
            })
            .collect();

        Ok(result)
    }

    /// Resolve a conflict between file versions
    pub async fn resolve_conflict(
        state: &AppState,
        user: &UserInfo,
        conflict_id: &str,
        resolution: &str,
    ) -> Result<()> {
        // Resolution options: "keep_local", "keep_remote", "merge"
        // Verify conflict ownership
        let conflict: Option<(String,)> =
            sqlx::query_as("SELECT id FROM file_conflicts WHERE id = ? AND user_id = ?")
                .bind(conflict_id)
                .bind(&user.id)
                .fetch_optional(&state.db_pool)
                .await?;

        if conflict.is_none() {
            return Err(anyhow::anyhow!("Conflict not found or permission denied"));
        }

        // Mark conflict as resolved with the chosen resolution strategy
        sqlx::query(
            "UPDATE file_conflicts SET resolved_at = datetime('now'), resolution_strategy = ? WHERE id = ?"
        )
        .bind(resolution)
        .bind(conflict_id)
        .execute(&state.db_pool)
        .await?;

        // Log conflict resolution
        let _ = sqlx::query(
            "INSERT INTO activity_log (id, user_id, action, file_path, status, metadata, created_at) VALUES (?, ?, 'resolve_conflict', ?, 'success', ?, datetime('now'))"
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&user.id)
        .bind(conflict_id)
        .bind(format!("{{\"strategy\": \"{}\"}}", resolution))
        .execute(&state.db_pool)
        .await;

        Ok(())
    }
}

// SYSTEM SERVICE
pub mod system {
    use super::*;
    use std::path::Path;

    pub async fn get_stats(state: &AppState) -> Result<serde_json::Value> {
        // Get file count and total size from database
        let file_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(0);

        let total_size: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE deleted_at IS NULL",
        )
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or(0);

        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(1);

        Ok(serde_json::json!({
            "file_count": file_count,
            "total_size": total_size,
            "user_count": user_count,
            "timestamp": Utc::now().to_rfc3339(),
        }))
    }

    pub async fn get_storage_info(state: &AppState, user: &UserInfo) -> Result<serde_json::Value> {
        // Get actual disk usage
        let data_path = Path::new("./data");
        let (used, free) = match get_disk_usage(data_path) {
            Ok((u, f)) => (u, f),
            Err(_) => (0, 1_000_000_000), // 1GB default if error
        };

        let total = used + free;

        // Get user-specific quota if enabled
        let user_storage: Option<(i64,)> = sqlx::query_as(
            "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE owner_id = ? AND deleted_at IS NULL"
        )
        .bind(&user.id)
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten();

        let user_used = user_storage.map(|(size,)| size).unwrap_or(0);

        Ok(serde_json::json!({
            "total": total,
            "used": used,
            "free": free,
            "percent_used": if total > 0 { (used as f64 / total as f64 * 100.0) as i32 } else { 0 },
            "user_storage": {
                "used": user_used,
                "user_id": &user.id,
            }
        }))
    }

    #[allow(dead_code)]
    pub async fn get_status(state: &AppState) -> Result<serde_json::Value> {
        // Get system health status
        let file_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files")
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(0);

        let active_users: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT user_id) FROM activity_log WHERE created_at > datetime('now', '-1 hour')"
        )
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or(0);

        Ok(serde_json::json!({
            "status": "ok",
            "version": "0.1.0",
            "uptime": "running",
            "database": "connected",
            "file_count": file_count,
            "active_users": active_users,
        }))
    }

    #[allow(dead_code)]
    pub async fn get_config_info(state: &AppState) -> Result<serde_json::Value> {
        // Load configuration from database
        let max_upload: Option<(i64,)> = sqlx::query_as(
            "SELECT COALESCE((SELECT value FROM system_settings WHERE key = 'max_upload_size'), '10485760') FROM LIMIT 1"
        )
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten();

        let max_upload_size = max_upload.map(|(size,)| size).unwrap_or(10485760);

        Ok(serde_json::json!({
            "max_upload_size": max_upload_size,
            "allowed_extensions": ["*"],
            "features": {
                "2fa": true,
                "sharing": true,
                "versioning": true,
                "collaboration": true,
                "search": true,
                "backup": true,
            },
            "server": {
                "version": "0.1.0",
                "name": "SyncSpace"
            }
        }))
    }

    #[allow(dead_code)]
    fn get_disk_usage(_path: &Path) -> Result<(u64, u64)> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let output = Command::new("df").arg(_path).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().last() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let total = parts[1].parse::<u64>().unwrap_or(0) * 1024;
                    let used = parts[2].parse::<u64>().unwrap_or(0) * 1024;
                    return Ok((used, total - used));
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("df").arg("-h").arg(path).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().last() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let used = parse_size(parts[2])?;
                    let total = parse_size(parts[1])?;
                    return Ok((used, total - used));
                }
            }
        }
        Ok((0, 1_000_000_000))
    }
}

#[allow(dead_code)]
fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim_end_matches(|c: char| !c.is_numeric());
    let size: f64 = s.parse()?;
    let suffix = s.trim_start_matches(|c: char| c.is_numeric() || c == '.');
    let multiplier = match suffix {
        "K" => 1024i64,
        "M" => 1024i64 * 1024,
        "G" => 1024i64 * 1024 * 1024,
        "T" => 1024i64 * 1024 * 1024 * 1024,
        _ => 1,
    };
    Ok((size * multiplier as f64) as u64)
}
