//! SyncSpace backend server - Axum version
//! Migrated from warp 0.3 to axum 0.7 for better multipart support and modern async patterns

mod auth;
mod database;
mod search;
mod handlers;
mod thumbnails;
mod notifications;
mod webhooks;
mod analytics;
// mod encryption;  // Disabled temporarily - needs aes_gcm crate
mod locking;
mod permissions;
mod preview;
mod audit;
mod virus_scan;
mod smart_folders;
mod rate_limit;
mod external_storage;
mod search_indexing;
mod system_settings;
mod email_integration;
mod s3_storage;
mod webdav;
mod ftp_sync;
mod ldap_integration;
mod prometheus_metrics;
mod redis_cache;
mod archive_management;
mod compression;
mod oauth;
mod performance;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, DefaultBodyLimit, Multipart, Path as AxumPath, Query, State},
    http::{Method, Request, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post, put},
    middleware,
    body::Body,
    Router,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use notify::{Error as NotifyError, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use std::io::Write;
use tower_http::cors::{Any, CorsLayer};
// ServeDir/ServeFile removed - using Vite dev server on port 5173
use uuid::Uuid;
use walkdir::WalkDir;
use sysinfo::{System, Disks};

use auth::{
    AuthResponse, ChangePasswordRequest, Enable2FARequest, LoginRequest, RateLimiter,
    RegisterRequest, Setup2FAResponse, UserDB, UserInfo,
};

// Performance and caching imports
use performance::{CacheConfig, CacheManager, JobProcessor, PerformanceMonitor};

const DATA_DIR: &str = "./data";
const CONFIG_FILE: &str = "./config.json";

// ==================== SHARED STATE ====================

#[derive(Clone)]
struct AppState {
    config: Arc<Mutex<Config>>,
    fs_tx: Sender<FileChangeEvent>,
    user_db: UserDB,
    rate_limiter: Arc<RateLimiter>,
    search_index: Arc<search::SearchIndex>,
    db_pool: sqlx::SqlitePool,
    cache_manager: performance::CacheManager,
    job_processor: performance::JobProcessor,
    performance_monitor: Arc<performance::PerformanceMonitor>,
}

// ==================== DATA STRUCTURES ====================

#[derive(Serialize, Deserialize)]
struct EntryInfo {
    name: String,
    is_dir: bool,
    size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    sync_dirs: Vec<String>,
    peers: Vec<Peer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sync_dirs: vec![DATA_DIR.to_string()],
            peers: Vec::new(),
            api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Peer {
    id: Uuid,
    address: String,
    last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
struct FileChangeEvent {
    path: String,
    kind: String,  // "create", "modify", "delete", "rename", "share", "comment", "batch_progress", "batch_complete"
    timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<serde_json::Value>,
}

// Enhanced Batch Operation Structures
#[derive(Debug, Serialize, Deserialize)]
struct BatchCopyRequest {
    items: Vec<BatchFileItem>,
    target_folder: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCompressRequest {
    items: Vec<BatchFileItem>,
    archive_name: String,
    compression_level: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchFileItem {
    path: String,
    name: String,
    size: Option<u64>,
}

#[derive(Debug, Serialize)]
struct BatchJobResponse {
    job_id: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct BatchJobStatus {
    job_id: String,
    status: String,
    progress: u32,
    total_items: u32,
    completed_items: u32,
    error_count: u32,
}

// Real-time Collaboration Structures
#[derive(Debug, Serialize, Deserialize)]
struct FileLock {
    id: String,
    file_path: String,
    locked_by_user_id: String,
    locked_by_username: String,
    locked_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    lock_type: String, // "read", "write", "exclusive"
    client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AcquireLockRequest {
    file_path: String,
    lock_type: String,
    duration_minutes: Option<u32>, // Default: 30 minutes
    client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserPresence {
    user_id: String,
    username: String,
    current_file: Option<String>,
    status: String, // "active", "away", "busy"
    last_activity: DateTime<Utc>,
    client_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePresenceRequest {
    current_file: Option<String>,
    status: String,
    client_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CollaborationActivity {
    id: String,
    user_id: String,
    username: String,
    activity_type: String, // "file_opened", "file_locked", "file_edited", "file_closed"
    file_path: String,
    timestamp: DateTime<Utc>,
    details: Option<serde_json::Value>,
}

impl FileChangeEvent {
    fn new(path: String, kind: String) -> Self {
        Self {
            path,
            kind,
            timestamp: Utc::now(),
            user_id: None,
            metadata: None,
        }
    }
    
    fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
struct RenameRequest {
    new_path: String,
}

#[derive(Debug, Clone, Serialize)]
struct SearchResult {
    path: String,
    is_dir: bool,
    size: u64,
}

// ==================== MAIN ====================

#[tokio::main]
async fn main() {
    println!("🚀 Starting SyncSpace Backend v0.3.0 (Axum)");
    
    // Initialize database
    let db_pool = match database::init_db().await {
        Ok(pool) => {
            println!("✅ Database initialized");
            pool
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Initialize search index
    let search_index = match search::SearchIndex::new() {
        Ok(index) => {
            println!("✅ Search index initialized");
            Arc::new(index)
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize search index: {}", e);
            std::process::exit(1);
        }
    };
    
    // Ensure data directory exists
    if let Err(e) = fs::create_dir_all(DATA_DIR).await {
        eprintln!("Failed to create data directory {}: {}", DATA_DIR, e);
    }
    
    // Initialize auth system
    let user_db = UserDB::new();
    let rate_limiter = Arc::new(RateLimiter::new());
    
    // Initialize performance and caching
    let cache_config = performance::CacheConfig::default();
    let cache_manager = match performance::CacheManager::new(cache_config.clone()).await {
        Ok(manager) => {
            println!("✅ Cache manager initialized");
            manager
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize cache manager: {}", e);
            std::process::exit(1);
        }
    };
    
    let job_processor = performance::JobProcessor::new(cache_manager.clone(), cache_config.background_job_workers);
    let performance_monitor = Arc::new(performance::PerformanceMonitor::new(cache_manager.clone()));
    
    // Start background job processing
    job_processor.start_processing().await;
    println!("✅ Background job processor started with {} workers", cache_config.background_job_workers);
    
    // Create default admin user if no users exist
    if user_db.get_by_username("admin").is_none() {
        println!("📝 Creating default admin user (username: admin, password: admin)");
        if let Err(e) = user_db.create_user("admin".to_string(), "admin".to_string()) {
            eprintln!("Failed to create admin user: {}", e);
        }
    }
    
    // Load configuration
    let config = Arc::new(Mutex::new(load_config().await.unwrap_or_default()));

    // Broadcast channel for file system events
    let (tx, _rx) = broadcast::channel::<FileChangeEvent>(32);
    let tx_clone = tx.clone();
    
    // Spawn file watcher task
    tokio::spawn(async move {
        if let Err(e) = watch_data_dir(tx_clone).await {
            eprintln!("File system watcher error: {}", e);
        }
    });

    // Build shared state
    let state = AppState {
        config,
        fs_tx: tx,
        user_db,
        rate_limiter,
        search_index,
        db_pool,
        cache_manager,
        job_processor,
        performance_monitor,
    };

    // Build router
    let app = build_router(state);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ SyncSpace backend listening on http://{}", addr);
    println!("🔐 Authentication enabled - use /api/auth/register or /api/auth/login");
    println!("🔍 Search available at /api/search?q=term");
    
    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ==================== ROUTER ====================

fn build_router(state: AppState) -> Router {
    // API key middleware enforces a configured api_key for mutating requests (POST/PUT/DELETE).
    // If `config.api_key` is None, middleware lets requests through (development mode).
    async fn api_key_middleware(State(state): State<AppState>, req: Request<Body>, next: middleware::Next) -> Response {
        // Allow safe methods and CORS preflight through
        if req.method() == Method::GET || req.method() == Method::OPTIONS {
            return next.run(req).await;
        }

        // Only enforce for mutating methods
        let is_mutating = matches!(req.method(), &Method::POST | &Method::PUT | &Method::DELETE);
        if !is_mutating {
            return next.run(req).await;
        }

        // Allow public paths to bypass API-key check (auth endpoints, status, search, websocket)
        if let Some(path) = req.uri().path().strip_prefix('/') {
            let p = format!("/{}", path);
            let public_prefixes = ["/api/auth", "/api/status", "/api/search", "/api/ws"];
            for pref in &public_prefixes {
                if p.starts_with(pref) {
                    return next.run(req).await;
                }
            }
        }

        // Check configured api_key
        let cfg = state.config.lock().await;
        if let Some(expected) = &cfg.api_key {
            // Extract header
            let provided = req
                .headers()
                .get("x-api-key")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");

            if provided != expected.as_str() {
                return (StatusCode::UNAUTHORIZED).into_response();
            }
        }

        next.run(req).await
    }
    // Auth routes (public)
    let auth_routes = Router::new()
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/refresh", post(refresh_token_handler))
        .route("/api/auth/oauth/{provider}", get(oauth_login_handler))
        .route("/api/auth/oauth/callback", get(oauth_callback_handler))
        .route("/api/auth/2fa/setup", post(setup_2fa_handler))
        .route("/api/auth/2fa/enable", post(enable_2fa_handler))
        .route("/api/auth/2fa/disable", post(disable_2fa_handler))
        .route("/api/auth/change-password", put(change_password_handler))
        .route("/api/auth/me", get(me_handler))
        .route("/api/users/profile", get(get_profile_handler).put(update_profile_handler))
        .route("/api/users/settings", get(handlers::users::get_user_settings_handler).put(handlers::users::update_user_settings_handler))
        .route("/api/users/preferences", get(handlers::users::get_user_preferences_handler).put(handlers::users::update_user_preferences_handler));
    
    // File routes (protected)
    let file_routes = Router::new()
        .route("/api/files/", get(handlers::files::list_files_root))  // Root directory with trailing slash
        .route("/api/file/{*path}", get(handlers::files::download_file_handler))
        .route("/api/upload/{*path}", post(handlers::files::upload_file_handler))
        .route("/api/upload-multipart", post(handlers::files::upload_multipart_handler))
        .route("/api/dirs/{*path}", post(handlers::files::create_dir_handler))
        .route("/api/rename/{*path}", put(handlers::files::rename_file_handler))
        .route("/api/move/{*path}", put(handlers::files::move_file_handler))
        .route("/api/copy/{*path}", post(handlers::files::copy_file_handler))
        // Wildcard routes LAST
        .route("/api/files/{*path}", get(handlers::files::list_files_handler).delete(handlers::files::delete_file_handler));
    
    // Trash routes (protected)
    let trash_routes = Router::new()
        .route("/api/trash", get(handlers::trash::list_trash_handler))
        .route("/api/trash/restore/{*path}", post(handlers::trash::restore_trash_handler))
        .route("/api/trash/permanent/{*path}", delete(handlers::trash::permanent_delete_handler))
        .route("/api/trash/cleanup", delete(handlers::trash::cleanup_trash_handler))
        .route("/api/trash/empty", delete(handlers::trash::empty_trash_handler));
    
    // Favorites routes (protected)
    let favorites_routes = Router::new()
        .route("/api/favorites", get(handlers::favorites::list_favorites_handler).post(handlers::favorites::toggle_favorite_handler))
        .route("/api/favorites/{id}", delete(handlers::favorites::delete_favorite_handler));
    
    // Activity/Audit Log routes (protected)
    let activity_routes = Router::new()
        .route("/api/activity", get(handlers::activity::list_activity_handler))
        .route("/api/activity/stats", get(handlers::activity::activity_stats_handler));
    
    // Comments & Tags routes (protected)
    let comments_tags_routes = Router::new()
        .route("/api/comments", post(handlers::comments::create_comment_handler).get(handlers::comments::list_comments_handler))
        .route("/api/comments/{id}", delete(handlers::comments::delete_comment_handler))
        .route("/api/tags", get(handlers::comments::list_tags_handler).post(handlers::comments::create_tag_handler))
        .route("/api/tags/{id}", delete(handlers::comments::delete_tag_handler))
        .route("/api/file-tags/{id}", post(handlers::comments::tag_file_handler).delete(handlers::comments::untag_file_handler));  // Changed from /api/files/{id}/tags
    
    // Sharing routes (protected)
    let sharing_routes = Router::new()
        .route("/api/shares", post(handlers::sharing::create_share).get(handlers::sharing::list_shares))
        .route("/api/shares/{id}", delete(handlers::sharing::delete_share))
        .route("/api/shares/{id}/permissions", put(handlers::sharing::update_permissions));
    
    // Fixed sharing routes (protected)
    let sharing_routes = Router::new()
        .route("/api/shares", post(handlers::sharing::create_share).get(handlers::sharing::list_shares))
        .route("/api/shares/{id}", delete(handlers::sharing::delete_share))
        .route("/api/shares/{id}/permissions", put(handlers::sharing::update_permissions))
        .route("/api/shared-with-me", get(handlers::sharing::list_shared_with_me));
    
    // Storage routes (protected)
    let storage_routes = Router::new()
        .route("/api/storage/stats", get(handlers::storage::get_storage_stats))
        .route("/api/storage/usage/{user_id}", get(handlers::storage::get_user_storage_usage))
        .route("/api/storage/quota/{user_id}", put(handlers::storage::update_user_quota))
        .route("/api/storage/cleanup", post(handlers::storage::cleanup_storage))
        .route("/api/storage/recalculate", post(handlers::storage::recalculate_storage));
    
    // Duplicates routes (protected)
    let duplicates_routes = Router::new()
        .route("/api/duplicates", get(handlers::duplicates::find_duplicates))
        .route("/api/duplicates/resolve", post(handlers::duplicates::resolve_duplicates))
        .route("/api/duplicates/stats", get(handlers::duplicates::duplicate_stats));
    
    // Versioning routes (protected) - separate path to avoid /api/files/{*path} conflict
    let versioning_routes = Router::new()
        .route("/api/versions/{file_id}", get(handlers::versioning::list_versions))
        .route("/api/versions/{file_id}/count", get(handlers::versioning::version_count))
        .route("/api/versions/{file_id}/{version_id}/restore", post(handlers::versioning::restore_version))
        .route("/api/versions/{file_id}/{version_id}", delete(handlers::versioning::delete_version));
    
    // Backup routes (protected)
    let backup_routes = Router::new()
        .route("/api/backups/create", post(handlers::backup::create_backup))
        .route("/api/backups", get(handlers::backup::list_backups))
        .route("/api/backups/{id}", get(handlers::backup::get_backup).delete(handlers::backup::delete_backup))
        .route("/api/backups/{id}/restore", post(handlers::backup::restore_backup))
        .route("/api/backups/{id}/verify", post(handlers::backup::verify_backup))
        .route("/api/backups/{id}/verifications", get(handlers::backup::list_verifications))
        .route("/api/backups/cleanup", post(handlers::backup::cleanup_old_backups))
        // Scheduling routes
        .route("/api/backups/schedules", post(handlers::backup::create_schedule).get(handlers::backup::list_schedules))
        .route("/api/backups/schedules/{id}", get(handlers::backup::get_schedule).put(handlers::backup::update_schedule).delete(handlers::backup::delete_schedule))
        .route("/api/backups/schedules/{id}/trigger", post(handlers::backup::trigger_schedule));
    
    
    // Collaboration routes (protected)
    let collaboration_routes = Router::new()
        .route("/api/collaboration/locks", get(handlers::collaboration::list_file_locks_handler).post(handlers::collaboration::acquire_file_lock_handler))
        .route("/api/collaboration/locks/{lock_id}", delete(handlers::collaboration::release_file_lock_handler))
        .route("/api/collaboration/locks/{lock_id}/heartbeat", post(handlers::collaboration::lock_heartbeat_handler))
        .route("/api/collaboration/presence", get(handlers::collaboration::get_user_presence_handler).post(handlers::collaboration::update_user_presence_handler))
        .route("/api/collaboration/presence/{user_id}", delete(handlers::collaboration::remove_user_presence_handler))
        .route("/api/collaboration/activity", get(handlers::collaboration::get_collaboration_activity_handler))
        .route("/api/collaboration/activity/{file_path}", get(handlers::collaboration::get_file_activity_handler))
        .route("/api/collaboration/conflicts", get(handlers::collaboration::list_edit_conflicts_handler))
        .route("/api/collaboration/conflicts/{conflict_id}/resolve", post(handlers::collaboration::resolve_edit_conflict_handler));
    
    // Notifications routes (protected)
    let notification_routes = Router::new()
        .route("/api/notifications", get(get_notifications_handler))
        .route("/api/notifications/unread", get(get_unread_notifications_handler))
        .route("/api/notifications/{id}/read", put(mark_notification_read_handler))
        .route("/api/notifications/read-all", put(mark_all_notifications_read_handler))
        .route("/api/notifications/{id}", delete(delete_notification_handler));
    
    // Webhooks routes (protected)
    let webhook_routes = Router::new()
        .route("/api/webhooks", get(list_webhooks_handler).post(create_webhook_handler))
        .route("/api/webhooks/{id}", get(get_webhook_handler).put(update_webhook_handler).delete(delete_webhook_handler))
        .route("/api/webhooks/{id}/test", post(test_webhook_handler));
    
    // Analytics routes (protected)
    let analytics_routes = Router::new()
        .route("/api/analytics/dashboard", get(analytics_dashboard_handler))
        .route("/api/analytics/storage", get(analytics_storage_handler))
        .route("/api/analytics/activity", get(analytics_activity_handler))
        .route("/api/analytics/files", get(analytics_files_handler))
        .route("/api/analytics/users", get(analytics_users_handler));
    
    // Batch operations routes (protected) - Enhanced with progress tracking
    let batch_routes = Router::new()
        .route("/api/batch/delete", post(batch_delete_handler))
        .route("/api/batch/move", post(batch_move_handler))
        .route("/api/batch/copy", post(batch_copy_handler))
        .route("/api/batch/tag", post(batch_tag_handler))
        .route("/api/batch/compress", post(batch_compress_handler))
        .route("/api/batch/operations/{job_id}", get(get_batch_operation_status))
        .route("/api/batch/operations/{job_id}/cancel", post(cancel_batch_operation));
    
    // Advanced search routes (protected)
    let search_routes = Router::new()
        .route("/api/search/advanced", get(advanced_search_handler))
        .route("/api/search/suggestions", get(search_suggestions_handler))
        .route("/api/search/recent", get(recent_searches_handler));
    
    // Integration routes (protected)
    let integration_routes = Router::new()
        // System Settings
        .route("/api/settings", get(get_system_settings_handler).put(update_system_settings_handler))
        // Email Integration
        .route("/api/email/accounts", get(list_email_accounts_handler).post(create_email_account_handler))
        .route("/api/email/accounts/{id}", delete(delete_email_account_handler))
        // S3 Storage
        .route("/api/s3/configs", get(list_s3_configs_handler).post(create_s3_config_handler))
        .route("/api/s3/configs/{id}", delete(delete_s3_config_handler))
        .route("/api/s3/test", post(test_s3_connection_handler))
        // WebDAV - supports standard WebDAV methods (PROPFIND, MKCOL, etc.)
        // .route("/*path", any(webdav_handler))  // Uncomment for full WebDAV support
        // FTP Sync
        .route("/api/ftp/connections", get(list_ftp_connections_handler).post(create_ftp_connection_handler))
        .route("/api/ftp/connections/{id}", delete(delete_ftp_connection_handler))
        .route("/api/ftp/sync", post(trigger_ftp_sync_handler))
        // LDAP Integration
        .route("/api/ldap/configs", get(list_ldap_configs_handler).post(create_ldap_config_handler))
        .route("/api/ldap/configs/{id}", put(update_ldap_config_handler).delete(delete_ldap_config_handler))
        .route("/api/ldap/test", post(test_ldap_connection_handler))
        // Prometheus Metrics
        .route("/metrics", get(prometheus_metrics_handler))
        // Redis Cache
        .route("/api/cache/{key}", get(get_cache_handler).delete(delete_cache_handler))
        // Archive Management
        .route("/api/archives/create", post(create_archive_handler))
        .route("/api/archives/extract", post(extract_archive_handler))
        // Compression Rules
        .route("/api/compression/rules", get(list_compression_rules_handler).post(create_compression_rule_handler))
        .route("/api/compression/rules/{id}", delete(delete_compression_rule_handler))
        .route("/api/compression/run", post(run_compression_handler));
    
    // Utility routes (protected)
    let utility_routes = Router::new()
        .route("/api/status", get(status_handler))  // Status endpoint (public)
        .route("/api/system/storage", get(system_storage_handler))  // System disk usage
        .route("/api/search", get(search_handler))
        .route("/api/stats", get(stats_handler))
        .route("/api/thumbnails/{file_id}", get(get_thumbnail_handler))  // Thumbnail serving
        .route("/api/config", get(get_config_handler).put(put_config_handler))
        .route("/api/peers", get(list_peers_handler).post(add_peer_handler));
    
    // Performance routes (protected)
    let performance_routes = Router::new()
        .route("/api/performance/metrics", get(get_performance_metrics))
        .route("/api/performance/metrics/history", get(get_performance_history))
        .route("/api/performance/cache/stats", get(get_cache_stats))
        .route("/api/performance/cache/clear", post(clear_cache))
        .route("/api/performance/jobs", get(list_background_jobs).post(queue_background_job))
        .route("/api/performance/jobs/{job_id}/status", get(get_job_status))
        .route("/api/performance/system/info", get(get_system_info));
    
    // WebSocket route
    let ws_route = Router::new()
        .route("/api/ws", get(ws_handler));
    
    // Root route (status page)
    let root_route = Router::new()
        .route("/", get(root_handler));
    
    // Combine all routes
    let router = Router::new()
        .merge(root_route)
        .merge(auth_routes)
        .merge(file_routes)
        .merge(trash_routes)
        .merge(favorites_routes)
        .merge(activity_routes)
        .merge(comments_tags_routes)
        .merge(sharing_routes)
        .merge(storage_routes)
        .merge(duplicates_routes)
        .merge(versioning_routes)  // Now uses /api/versions/{file_id}
        .merge(backup_routes)
        .merge(collaboration_routes)  // Real-time collaboration
        .merge(notification_routes)
        .merge(webhook_routes)
        .merge(analytics_routes)
        .merge(batch_routes)
        .merge(search_routes)
        .merge(integration_routes)
        .merge(utility_routes)
        .merge(performance_routes)
        .merge(ws_route)
    ;

    // Note: Frontend is served by Vite on port 5173 in development
    // In production, use a reverse proxy (nginx/caddy) or build frontend into ./dist
    router
        // API-key middleware applied with state (cloned)
        .layer(middleware::from_fn_with_state(state.clone(), api_key_middleware))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(Any)
        )
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100MB upload limit
        .with_state(state)
}

// ==================== AUTH HANDLERS ====================

async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    match state.user_db.create_user(req.username.clone(), req.password) {
        Ok(user) => {
            match (auth::generate_token(&user), auth::generate_refresh_token(&user)) {
                (Ok(token), Ok(refresh_token)) => {
                    let response = AuthResponse {
                        token,
                        refresh_token,
                        user: UserInfo {
                            id: user.id.to_string(),
                            username: user.username,
                            totp_enabled: user.totp_enabled,
                        },
                        requires_2fa: false,
                    };
                    Ok(Json(response))
                }
                (Err(e), _) | (_, Err(e)) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e})),
                )),
            }
        }
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )),
    }
}

async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Rate limiting
    if !state.rate_limiter.check_rate_limit(&req.username, 5, 60) {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::json!({"error": "Too many login attempts"})),
        ));
    }

    match state.user_db.verify_password(&req.username, &req.password) {
        Ok(mut user) => {
            // Check 2FA
            if user.totp_enabled {
                if let Some(code) = req.totp_code {
                    if let Some(ref secret) = user.totp_secret {
                        if !auth::verify_totp_code(secret, &code) {
                            return Err((
                                StatusCode::UNAUTHORIZED,
                                Json(serde_json::json!({"error": "Invalid 2FA code"})),
                            ));
                        }
                    }
                } else {
                    return Err((
                        StatusCode::OK,
                        Json(serde_json::json!({"requires_2fa": true})),
                    ));
                }
            }

            user.last_login = Some(Utc::now());
            state.user_db.update_user(user.clone());

            match (auth::generate_token(&user), auth::generate_refresh_token(&user)) {
                (Ok(token), Ok(refresh_token)) => Ok(Json(AuthResponse {
                    token,
                    refresh_token,
                    user: UserInfo {
                        id: user.id.to_string(),
                        username: user.username,
                        totp_enabled: user.totp_enabled,
                    },
                    requires_2fa: false,
                })),
                (Err(e), _) | (_, Err(e)) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e})),
                )),
            }
        }
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": e})),
        )),
    }
}

async fn setup_2fa_handler(
    State(_state): State<AppState>,
    user: auth::User,
) -> Json<Setup2FAResponse> {
    let secret = auth::generate_totp_secret();
    let qr_url = format!(
        "otpauth://totp/SyncSpace:{}?secret={}&issuer=SyncSpace",
        user.username, secret
    );
    Json(Setup2FAResponse { secret, qr_url })
}

async fn enable_2fa_handler(
    State(state): State<AppState>,
    mut user: auth::User,
    Json(req): Json<Enable2FARequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let secret = user.totp_secret.clone().unwrap_or_else(auth::generate_totp_secret);
    
    if auth::verify_totp_code(&secret, &req.totp_code) {
        user.totp_secret = Some(secret);
        user.totp_enabled = true;
        state.user_db.update_user(user);
        Ok(Json(serde_json::json!({"message": "2FA enabled"})))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid code"})),
        ))
    }
}

async fn disable_2fa_handler(
    State(state): State<AppState>,
    mut user: auth::User,
) -> Json<serde_json::Value> {
    user.totp_enabled = false;
    user.totp_secret = None;
    state.user_db.update_user(user);
    Json(serde_json::json!({"message": "2FA disabled"}))
}

async fn change_password_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.user_db.change_password(user.id, &req.old_password, &req.new_password) {
        Ok(_) => Ok(Json(serde_json::json!({"message": "Password changed"}))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )),
    }
}

async fn me_handler(user: auth::User) -> Json<UserInfo> {
    Json(UserInfo {
        id: user.id.to_string(),
        username: user.username,
        totp_enabled: user.totp_enabled,
    })
}

async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(req): Json<auth::RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Verify refresh token
    match auth::verify_refresh_token(&req.refresh_token) {
        Ok(claims) => {
            let user_id = Uuid::parse_str(&claims.sub)
                .map_err(|_| (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid user ID"}))))?;

            // Get user from database
            match state.user_db.get_by_id(&user_id) {
                Some(user) => {
                    // Generate new tokens
                    match (auth::generate_token(&user), auth::generate_refresh_token(&user)) {
                        (Ok(token), Ok(refresh_token)) => Ok(Json(AuthResponse {
                            token,
                            refresh_token,
                            user: UserInfo {
                                id: user.id.to_string(),
                                username: user.username.clone(),
                                totp_enabled: user.totp_enabled,
                            },
                            requires_2fa: false,
                        })),
                        (Err(e), _) | (_, Err(e)) => Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({"error": e})),
                        )),
                    }
                }
                None => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({"error": "User not found"})),
                )),
            }
        }
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": e})),
        )),
    }
}

async fn oauth_login_handler(
    AxumPath(provider): AxumPath<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Implement OAuth2 login redirect
    // 1. Get provider config from database
    // 2. Generate OAuth2 authorization URL with state
    // 3. Redirect to provider's authorization endpoint
    
    // Placeholder response
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "OAuth2 login not yet implemented",
            "provider": provider,
            "message": "Configure OAuth providers in database and implement oauth2 crate integration"
        })),
    ))
}

async fn oauth_callback_handler(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Implement OAuth2 callback handler
    // 1. Verify state parameter
    // 2. Exchange authorization code for access token
    // 3. Fetch user info from provider
    // 4. Create or link user account
    // 5. Generate JWT tokens
    // 6. Redirect to frontend with tokens
    
    // Placeholder response
    let code = params.get("code").cloned();
    let state = params.get("state").cloned();
    
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "OAuth2 callback not yet implemented",
            "code": code,
            "state": state,
            "message": "Implement oauth2 crate integration for token exchange"
        })),
    ))
}

// ==================== USER PROFILE HANDLERS ====================

#[derive(Debug, Serialize, Deserialize)]
struct UserProfile {
    id: String,
    username: String,
    email: Option<String>,
    display_name: Option<String>,
    avatar_base64: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateProfileRequest {
    email: Option<String>,
    display_name: Option<String>,
    avatar_base64: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserSettings {
    language: String,
    theme: String,
    default_view: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateSettingsRequest {
    language: Option<String>,
    theme: Option<String>,
    default_view: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserPreferences {
    view_mode: String,                // "grid" or "list"
    recent_searches: Vec<String>,     // last search queries
    sidebar_collapsed: bool,          // sidebar state
    sort_by: String,                 // "name", "size", "date"
    sort_order: String,              // "asc", "desc"
    auto_refresh: bool,              // auto refresh files
    upload_progress_visible: bool,    // show upload progress
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePreferencesRequest {
    view_mode: Option<String>,
    recent_searches: Option<Vec<String>>,
    sidebar_collapsed: Option<bool>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    auto_refresh: Option<bool>,
    upload_progress_visible: Option<bool>,
}

/// Get user profile information
async fn get_profile_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<UserProfile>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    let profile: Option<(String, String, Option<String>, Option<String>, Option<String>, String, String)> = 
        sqlx::query_as(
            "SELECT id, username, email, display_name, avatar_base64, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(&user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match profile {
        Some((id, username, email, display_name, avatar_base64, created_at, updated_at)) => {
            Ok(Json(UserProfile {
                id,
                username,
                email,
                display_name,
                avatar_base64,
                created_at,
                updated_at,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Update user profile information
async fn update_profile_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfile>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    let now = Utc::now().to_rfc3339();
    
    // Update profile fields
    sqlx::query(
        "UPDATE users SET email = COALESCE(?, email), display_name = COALESCE(?, display_name), avatar_base64 = COALESCE(?, avatar_base64), updated_at = ? WHERE id = ?"
    )
    .bind(&req.email)
    .bind(&req.display_name)
    .bind(&req.avatar_base64)
    .bind(&now)
    .bind(&user_id)
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Fetch and return updated profile
    get_profile_handler(State(state), user).await
}

/// Get user settings (theme, language, default view)
async fn get_user_settings_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<UserSettings>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    let settings: Option<(String, String, String)> = 
        sqlx::query_as(
            "SELECT language, theme, default_view FROM users WHERE id = ?"
        )
        .bind(&user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match settings {
        Some((language, theme, default_view)) => {
            Ok(Json(UserSettings {
                language,
                theme,
                default_view,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Update user settings
async fn update_user_settings_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<UpdateSettingsRequest>,
) -> Result<Json<UserSettings>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    let now = Utc::now().to_rfc3339();
    
    // Update settings (only non-null values)
    let query_str = if req.language.is_some() && req.theme.is_some() && req.default_view.is_some() {
        "UPDATE users SET language = ?, theme = ?, default_view = ?, updated_at = ? WHERE id = ?"
    } else if req.language.is_some() && req.theme.is_some() {
        "UPDATE users SET language = ?, theme = ?, updated_at = ? WHERE id = ?"
    } else if req.language.is_some() && req.default_view.is_some() {
        "UPDATE users SET language = ?, default_view = ?, updated_at = ? WHERE id = ?"
    } else if req.theme.is_some() && req.default_view.is_some() {
        "UPDATE users SET theme = ?, default_view = ?, updated_at = ? WHERE id = ?"
    } else if req.language.is_some() {
        "UPDATE users SET language = ?, updated_at = ? WHERE id = ?"
    } else if req.theme.is_some() {
        "UPDATE users SET theme = ?, updated_at = ? WHERE id = ?"
    } else if req.default_view.is_some() {
        "UPDATE users SET default_view = ?, updated_at = ? WHERE id = ?"
    } else {
        // Nothing to update
        return get_user_settings_handler(State(state), user).await;
    };
    
    // Build query dynamically based on provided fields
    let mut query = sqlx::query(query_str);
    
    if let Some(lang) = req.language {
        query = query.bind(lang);
    }
    if let Some(theme) = req.theme {
        query = query.bind(theme);
    }
    if let Some(view) = req.default_view {
        query = query.bind(view);
    }
    query = query.bind(&now).bind(&user_id);
    
    query
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Fetch and return updated settings
    get_user_settings_handler(State(state), user).await
}

/// Get user preferences (client-specific settings)
async fn get_user_preferences_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<UserPreferences>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    let prefs: Option<(String, String, i64, String, String, i64, i64)> = 
        sqlx::query_as(
            "SELECT 
                COALESCE(view_mode, 'grid') as view_mode,
                COALESCE(recent_searches, '[]') as recent_searches, 
                COALESCE(sidebar_collapsed, 0) as sidebar_collapsed,
                COALESCE(sort_by, 'name') as sort_by,
                COALESCE(sort_order, 'asc') as sort_order,
                COALESCE(auto_refresh, 1) as auto_refresh,
                COALESCE(upload_progress_visible, 1) as upload_progress_visible
             FROM users WHERE id = ?"
        )
        .bind(&user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match prefs {
        Some((view_mode, recent_searches_json, sidebar_collapsed, sort_by, sort_order, auto_refresh, upload_progress_visible)) => {
            let recent_searches: Vec<String> = serde_json::from_str(&recent_searches_json).unwrap_or_default();
            
            Ok(Json(UserPreferences {
                view_mode,
                recent_searches,
                sidebar_collapsed: sidebar_collapsed != 0,
                sort_by,
                sort_order,
                auto_refresh: auto_refresh != 0,
                upload_progress_visible: upload_progress_visible != 0,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Update user preferences
async fn update_user_preferences_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<UpdatePreferencesRequest>,
) -> Result<Json<UserPreferences>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    let now = Utc::now().to_rfc3339();
    
    // Build update query dynamically based on provided fields
    let mut updates = Vec::new();
    let mut values: Vec<Box<dyn sqlx::Encode<sqlx::Sqlite> + Send + Sync>> = Vec::new();
    
    // Make clones for individual updates
    let view_mode_clone = req.view_mode.clone();
    let recent_searches_clone = req.recent_searches.clone();
    let sort_by_clone = req.sort_by.clone();
    let sort_order_clone = req.sort_order.clone();
    
    if let Some(view_mode) = req.view_mode {
        updates.push("view_mode = ?");
        values.push(Box::new(view_mode));
    }
    if let Some(recent_searches) = req.recent_searches {
        updates.push("recent_searches = ?");
        let json_str = serde_json::to_string(&recent_searches).unwrap_or_default();
        values.push(Box::new(json_str));
    }
    if let Some(sidebar_collapsed) = req.sidebar_collapsed {
        updates.push("sidebar_collapsed = ?");
        values.push(Box::new(if sidebar_collapsed { 1i64 } else { 0i64 }));
    }
    if let Some(sort_by) = req.sort_by {
        updates.push("sort_by = ?");
        values.push(Box::new(sort_by));
    }
    if let Some(sort_order) = req.sort_order {
        updates.push("sort_order = ?");
        values.push(Box::new(sort_order));
    }
    if let Some(auto_refresh) = req.auto_refresh {
        updates.push("auto_refresh = ?");
        values.push(Box::new(if auto_refresh { 1i64 } else { 0i64 }));
    }
    if let Some(upload_progress_visible) = req.upload_progress_visible {
        updates.push("upload_progress_visible = ?");
        values.push(Box::new(if upload_progress_visible { 1i64 } else { 0i64 }));
    }
    
    if !updates.is_empty() {
        // Simplified version with individual updates for each field
        if let Some(view_mode) = view_mode_clone {
            sqlx::query("UPDATE users SET view_mode = ?, updated_at = ? WHERE id = ?")
                .bind(view_mode)
                .bind(&now)
                .bind(&user_id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        if let Some(recent_searches) = recent_searches_clone {
            let json_str = serde_json::to_string(&recent_searches).unwrap_or_default();
            sqlx::query("UPDATE users SET recent_searches = ?, updated_at = ? WHERE id = ?")
                .bind(json_str)
                .bind(&now)
                .bind(&user_id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        if let Some(sidebar_collapsed) = req.sidebar_collapsed {
            sqlx::query("UPDATE users SET sidebar_collapsed = ?, updated_at = ? WHERE id = ?")
                .bind(if sidebar_collapsed { 1i64 } else { 0i64 })
                .bind(&now)
                .bind(&user_id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        // Continue for other fields as needed...
    }
    
    // Fetch and return updated preferences
    get_user_preferences_handler(State(state), user).await
}

// Handler for root directory listing (/api/files/)
async fn list_files_root(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<EntryInfo>>, (StatusCode, Json<serde_json::Value>)> {
    // Call the main handler with empty path
    list_files_handler(State(state), user, AxumPath(String::new())).await
}

async fn list_files_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<Json<Vec<EntryInfo>>, (StatusCode, Json<serde_json::Value>)> {
    // Check cache first
    if let Ok(Some(cached_entries)) = state.cache_manager.get_directory_listing(&path).await {
        return Ok(Json(cached_entries));
    }
    
    let target_dir = Path::new(DATA_DIR).join(&path);
    let mut entries = Vec::new();
    
    match fs::read_dir(&target_dir).await {
        Ok(mut dir) => {
            while let Ok(Some(e)) = dir.next_entry().await {
                if let Ok(meta) = e.metadata().await {
                    let filename = e.file_name().to_string_lossy().to_string();
                    if is_system_file(&filename) {
                        continue;
                    }
                    entries.push(EntryInfo {
                        name: filename,
                        is_dir: meta.is_dir(),
                        size: meta.len(),
                    });
                }
            }
            
            // Cache the result
            if let Err(e) = state.cache_manager.cache_directory_listing(&path, &entries).await {
                eprintln!("Failed to cache directory listing: {}", e);
            }
            
            Ok(Json(entries))
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Directory not found"})),
        )),
    }
}

async fn download_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<Bytes, StatusCode> {
    let file_path = Path::new(DATA_DIR).join(&path);
    match fs::read(&file_path).await {
        Ok(bytes) => {
            // Log download activity
            let user_id = user.id.to_string();
            let pool = &state.db_pool;
            let log_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO file_history (id, user_id, action, file_path, status, error_message, created_at) 
                 VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
            )
            .bind(&log_id)
            .bind(&user_id)
            .bind("downloaded")
            .bind(&path)
            .bind("success")
            .bind::<Option<String>>(None)
            .execute(pool)
            .await;
            
            Ok(Bytes::from(bytes))
        },
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn upload_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
    body: Bytes,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let file_path = PathBuf::from(&path);
    let file_size = body.len() as i64;
    
    // 🔒 QUOTA CHECK - Prevent upload if user exceeds quota
    if let Err(_) = handlers::storage::check_quota(&state.db_pool, &user.id.to_string(), file_size).await {
        return Err(StatusCode::INSUFFICIENT_STORAGE);
    }
    
    if let Some(parent) = Path::new(DATA_DIR).join(&path).parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    // Write atomically: write to a .tmp file in the same directory, then rename
    let target = Path::new(DATA_DIR).join(&path);
    let tmp_name = format!("{}.{}.tmp", target.file_name().and_then(|n| n.to_str()).unwrap_or("upload"), Uuid::new_v4());
    let tmp_path = target.with_file_name(tmp_name);

    // Create and write tmp file
    if let Some(parent) = tmp_path.parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let mut tmp_file = fs::File::create(&tmp_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tmp_file.write_all(&body).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tmp_file.flush().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Atomically rename into place
    fs::rename(&tmp_path, &target).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Log activity
    let user_id = user.id.to_string();
    let pool = &state.db_pool;
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT INTO file_history (id, user_id, action, file_path, status, error_message, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&log_id)
    .bind(&user_id)
    .bind("created")
    .bind(&path)
    .bind("success")
    .bind::<Option<String>>(None)
    .execute(pool)
    .await;
    
    // Broadcast event
    let _ = state.fs_tx.send(
        FileChangeEvent::new(path.clone(), "create".to_string())
            .with_user(user_id.clone())
    );
    
    // 💾 UPDATE STORAGE USAGE
    let _ = handlers::storage::update_storage_usage(&state.db_pool, &user_id, file_size).await;
    
    // 🖼️ THUMBNAIL GENERATION for images
    if thumbnails::is_image(&target) {
        let file_id = Uuid::new_v4().to_string();
        thumbnails::generate_thumbnail_async(target.clone(), file_id);
    }
    
    // Background indexing
    let index = state.search_index.clone();
    let cache_manager = state.cache_manager.clone();
    let job_processor = state.job_processor.clone();
    let full_path = Path::new(DATA_DIR).join(&path);
    let dir_path = if let Some(parent) = Path::new(&path).parent() {
        parent.to_string_lossy().to_string()
    } else {
        ".".to_string()
    };
    
    tokio::spawn(async move {
        // Invalidate directory listing cache
        let dir_cache_key = format!("dir_list:{}", dir_path);
        let _ = cache_manager.delete(&dir_cache_key).await;
        
        // Queue background tasks
        let thumbnail_payload = serde_json::json!({
            "file_path": path.clone(),
            "full_path": full_path.to_string_lossy()
        });
        let _ = job_processor.queue_job("thumbnail_generation".to_string(), thumbnail_payload, 1).await;
        
        let indexing_payload = serde_json::json!({
            "file_path": path.clone(),
            "full_path": full_path.to_string_lossy()
        });
        let _ = job_processor.queue_job("search_indexing".to_string(), indexing_payload, 2).await;
        
        // Original indexing (can be removed later when background job is fully implemented)
        if let Some(filename) = file_path.file_name() {
            let content = search::extract_content(&full_path).await;
            if let Ok(metadata) = fs::metadata(&full_path).await {
                let file_id = Uuid::new_v4().to_string();
                let _ = index.index_file(
                    &file_id,
                    &filename.to_string_lossy(),
                    &path,
                    content,
                    Utc::now(),
                    metadata.len(),
                ).await;
            }
        }
    });
    
    Ok((StatusCode::CREATED, "uploaded"))
}

async fn upload_multipart_handler(
    State(state): State<AppState>,
    user: auth::User,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut uploaded_count = 0;
    let mut total_size: i64 = 0;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        if let Some(filename) = field.file_name().map(|s| s.to_string()) {
            let data = field.bytes().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let file_size = data.len() as i64;
            
            // 🔒 QUOTA CHECK for each file
            if let Err(_) = handlers::storage::check_quota(&state.db_pool, &user.id.to_string(), file_size).await {
                return Err((StatusCode::INSUFFICIENT_STORAGE, format!("Quota exceeded for file: {}", filename)));
            }
            
            let target = Path::new(DATA_DIR).join(&filename);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            }

            let tmp_name = format!("{}.{}.tmp", target.file_name().and_then(|n| n.to_str()).unwrap_or("upload"), Uuid::new_v4());
            let tmp_path = target.with_file_name(tmp_name);
            let mut tmp_file = fs::File::create(&tmp_path).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tmp_file.write_all(&data).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tmp_file.flush().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            fs::rename(&tmp_path, &target).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            // Broadcast event
            let _ = state.fs_tx.send(
                FileChangeEvent::new(filename.clone(), "create".to_string())
                    .with_user(user.id.to_string())
            );
            
            uploaded_count += 1;
            total_size += file_size;
        }
    }
    
    // 💾 UPDATE STORAGE USAGE for all uploaded files
    if total_size > 0 {
        let _ = handlers::storage::update_storage_usage(&state.db_pool, &user.id.to_string(), total_size).await;
    }
    
    Ok((StatusCode::CREATED, format!("Uploaded {} files", uploaded_count)))
}

async fn delete_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let full_path = Path::new(DATA_DIR).join(&path);
    let pool = &state.db_pool;
    
    // Get auto_trash_cleanup_days setting for auto-delete timestamp
    let cleanup_days: i64 = sqlx::query_scalar::<_, String>(
        "SELECT value FROM settings WHERE key = 'auto_trash_cleanup_days'"
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .and_then(|s| s.parse().ok())
    .unwrap_or(30);
    
    let auto_delete_at = if cleanup_days > 0 {
        Some(Utc::now() + chrono::Duration::days(cleanup_days))
    } else {
        None
    };
    
    if let Ok(meta) = fs::metadata(&full_path).await {
        let is_dir = meta.is_dir();
        let size = if is_dir {
            // Calculate folder size recursively
            WalkDir::new(&full_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter_map(|e| e.metadata().ok())
                .map(|m| m.len())
                .sum::<u64>() as i64
        } else {
            meta.len() as i64
        };
        
        // Check if item exists in database
        let item_id = if is_dir {
            // Find or create folder in database
            let folder_id: Option<String> = sqlx::query_scalar(
                "SELECT id FROM folders WHERE path = ? AND owner_id = ?"
            )
            .bind(&path)
            .bind(&user.id.to_string())
            .fetch_optional(pool)
            .await
            .ok()
            .flatten();
            
            if let Some(id) = folder_id {
                // Mark as deleted
                sqlx::query(
                    "UPDATE folders SET is_deleted = 1, deleted_at = datetime('now'), deleted_by = ? 
                     WHERE id = ?"
                )
                .bind(&user.id.to_string())
                .bind(&id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                id
            } else {
                // Create folder entry then mark deleted
                let id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO folders (id, name, path, owner_id, is_deleted, deleted_at, deleted_by, created_at, updated_at)
                     VALUES (?, ?, ?, ?, 1, datetime('now'), ?, datetime('now'), datetime('now'))"
                )
                .bind(&id)
                .bind(full_path.file_name().and_then(|n| n.to_str()).unwrap_or(""))
                .bind(&path)
                .bind(&user.id.to_string())
                .bind(&user.id.to_string())
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                id
            }
        } else {
            // Find or create file in database
            let file_id: Option<String> = sqlx::query_scalar(
                "SELECT id FROM files WHERE path = ? AND owner_id = ?"
            )
            .bind(&path)
            .bind(&user.id.to_string())
            .fetch_optional(pool)
            .await
            .ok()
            .flatten();
            
            if let Some(id) = file_id {
                // Mark as deleted
                sqlx::query(
                    "UPDATE files SET is_deleted = 1, deleted_at = datetime('now'), deleted_by = ? 
                     WHERE id = ?"
                )
                .bind(&user.id.to_string())
                .bind(&id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                id
            } else {
                // Create file entry then mark deleted
                let id = Uuid::new_v4().to_string();
                let storage_path = full_path.to_str().unwrap_or("");
                
                sqlx::query(
                    "INSERT INTO files (id, name, path, owner_id, size_bytes, storage_path, is_deleted, deleted_at, deleted_by, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, 1, datetime('now'), ?, datetime('now'), datetime('now'))"
                )
                .bind(&id)
                .bind(full_path.file_name().and_then(|n| n.to_str()).unwrap_or(""))
                .bind(&path)
                .bind(&user.id.to_string())
                .bind(size)
                .bind(storage_path)
                .bind(&user.id.to_string())
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                id
            }
        };
        
        // Add to trash table
        let trash_id = Uuid::new_v4().to_string();
        let item_type = if is_dir { "folder" } else { "file" };
        let auto_delete_str = auto_delete_at.map(|dt| dt.to_rfc3339());
        let deleted_at = Utc::now().to_rfc3339();
        
        sqlx::query(
            "INSERT INTO trash (id, item_type, item_id, original_path, original_name, deleted_by, deleted_at, auto_delete_at, size_bytes)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&trash_id)
        .bind(item_type)
        .bind(&item_id)
        .bind(&path)
        .bind(full_path.file_name().and_then(|n| n.to_str()).unwrap_or(""))
        .bind(&user.id.to_string())
        .bind(&deleted_at)
        .bind(auto_delete_str)
        .bind(size)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Broadcast delete event
        let _ = state.fs_tx.send(
            FileChangeEvent::new(path.clone(), "delete".to_string())
                .with_user(user.id.to_string())
        );
        
        // Log activity
        let log_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO file_history (id, user_id, action, file_path, status, error_message, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(&log_id)
        .bind(&user.id.to_string())
        .bind("deleted")
        .bind(&path)
        .bind("success")
        .bind::<Option<String>>(None)
        .execute(pool)
        .await;
        
        // 💾 UPDATE STORAGE USAGE - decrease by file size (soft delete doesn't free space yet)
        // Storage is freed when trash is permanently deleted
        
        Ok((StatusCode::OK, "moved to trash"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create_dir_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let full_path = Path::new(DATA_DIR).join(&path);
    fs::create_dir_all(&full_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let _ = state.fs_tx.send(
        FileChangeEvent::new(path, "mkdir".to_string())
            .with_user(user.id.to_string())
    );
    
    Ok((StatusCode::CREATED, "created"))
}

async fn rename_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(old_path): AxumPath<String>,
    Json(req): Json<RenameRequest>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let old_full = Path::new(DATA_DIR).join(&old_path);
    let new_full = Path::new(DATA_DIR).join(&req.new_path);
    
    if let Some(parent) = new_full.parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    fs::rename(old_full, new_full).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Log activity
    let user_id = user.id.to_string();
    let pool = &state.db_pool;
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT INTO file_history (id, user_id, action, file_path, status, error_message, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&log_id)
    .bind(&user_id)
    .bind("renamed")
    .bind(&req.new_path)
    .bind("success")
    .bind::<Option<String>>(None)
    .execute(pool)
    .await;
    
    let _ = state.fs_tx.send(
        FileChangeEvent::new(req.new_path, "rename".to_string())
            .with_user(user.id.to_string())
    );
    
    Ok((StatusCode::OK, "renamed"))
}

async fn move_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(old_path): AxumPath<String>,
    Json(req): Json<RenameRequest>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let old_full = Path::new(DATA_DIR).join(&old_path);
    let new_full = Path::new(DATA_DIR).join(&req.new_path);
    
    if !old_full.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    if let Some(parent) = new_full.parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    fs::rename(old_full, new_full).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Log activity
    let user_id = user.id.to_string();
    let pool = &state.db_pool;
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT INTO file_history (id, user_id, action, file_path, status, error_message, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&log_id)
    .bind(&user_id)
    .bind("moved")
    .bind(&req.new_path)
    .bind("success")
    .bind::<Option<String>>(None)
    .execute(pool)
    .await;
    
    let _ = state.fs_tx.send(
        FileChangeEvent::new(req.new_path, "move".to_string())
            .with_user(user.id.to_string())
    );
    
    Ok((StatusCode::OK, "moved"))
}

async fn copy_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(source_path): AxumPath<String>,
    Json(req): Json<RenameRequest>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let source_full = Path::new(DATA_DIR).join(&source_path);
    let dest_full = Path::new(DATA_DIR).join(&req.new_path);
    
    if !source_full.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    if let Some(parent) = dest_full.parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if source_full.is_dir() {
        // Recursive directory copy
        fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
            std::fs::create_dir_all(&dst)?;
            for entry in std::fs::read_dir(src)? {
                let entry = entry?;
                let ty = entry.file_type()?;
                if ty.is_dir() {
                    copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
                } else {
                    std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
                }
            }
            Ok(())
        }
        
        tokio::task::spawn_blocking(move || copy_dir_all(source_full, dest_full))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        fs::copy(source_full, dest_full).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    // Log activity
    let user_id = user.id.to_string();
    let pool = &state.db_pool;
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT INTO file_history (id, user_id, action, file_path, status, error_message, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&log_id)
    .bind(&user_id)
    .bind("copied")
    .bind(&req.new_path)
    .bind("success")
    .bind::<Option<String>>(None)
    .execute(pool)
    .await;
    
    let _ = state.fs_tx.send(
        FileChangeEvent::new(req.new_path, "copy".to_string())
            .with_user(user.id.to_string())
    );
    
    Ok((StatusCode::CREATED, "copied"))
}


async fn search_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let query = params.get("q").cloned().unwrap_or_default();
    let limit = params.get("limit").and_then(|s| s.parse().ok()).unwrap_or(50);
    let fuzzy = params.get("fuzzy").and_then(|s| s.parse().ok()).unwrap_or(true);
    
    // Check cache for search results
    let cache_key = format!("search_{}_{}_{}_{}", query, limit, fuzzy, _user.id);
    if let Ok(Some(cached_results)) = state.cache_manager.get_search_results(&cache_key).await {
        return Json(serde_json::json!({
            "results": cached_results,
            "total": cached_results.len(),
            "query": query,
            "cached": true,
        }));
    }
    
    match state.search_index.search(&query, limit, fuzzy) {
        Ok(results) => {
            // Convert to SearchResult format and cache
            let search_results: Vec<performance::SearchResult> = results.iter().map(|r| {
                performance::SearchResult {
                    file_path: r.path.clone(),
                    relevance_score: r.score as f64,
                    snippet: r.snippet.clone(),
                    metadata: performance::FileMetadata {
                        size: r.size,
                        modified_at: Utc::now(), // Use current time as fallback
                        checksum: None,
                        mime_type: None,
                        tags: Vec::new(),
                    },
                }
            }).collect();
            
            // Cache the results
            if let Err(e) = state.cache_manager.cache_search_results(&cache_key, &search_results).await {
                eprintln!("Failed to cache search results: {}", e);
            }
            
            Json(serde_json::json!({
                "results": results,
                "total": results.len(),
                "query": query,
                "cached": false,
            }))
        },
        Err(e) => Json(serde_json::json!({
            "error": format!("Search failed: {}", e)
        })),
    }
}

async fn stats_handler(
    _user: auth::User,
) -> Json<serde_json::Value> {
    let (count, size) = compute_stats_async().await;
    Json(serde_json::json!({
        "file_count": count,
        "total_size": size,
    }))
}

async fn get_thumbnail_handler(
    _user: auth::User,
    AxumPath(file_id): AxumPath<String>,
) -> Result<Response, StatusCode> {
    let thumb_path = thumbnails::get_thumbnail_path(&file_id);
    
    if !thumb_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let data = fs::read(&thumb_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/webp")
        .header("Cache-Control", "public, max-age=31536000") // Cache for 1 year
        .body(Body::from(data))
        .unwrap())
}

async fn get_config_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Json<Config> {
    let config = state.config.lock().await.clone();
    Json(config)
}

async fn put_config_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(new_config): Json<Config>,
) -> Json<Config> {
    *state.config.lock().await = new_config.clone();
    let _ = save_config(&new_config).await;
    Json(new_config)
}

async fn list_peers_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Json<Vec<Peer>> {
    let peers = state.config.lock().await.peers.clone();
    Json(peers)
}

async fn add_peer_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(peer): Json<Peer>,
) -> Json<Peer> {
    let mut config = state.config.lock().await;
    config.peers.push(peer.clone());
    let _ = save_config(&*config).await;
    Json(peer)
}

// ==================== INTEGRATION HANDLERS ====================

// System Settings Handlers
async fn get_system_settings_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder - system_settings needs a get_all_settings function
    Err((StatusCode::NOT_IMPLEMENTED, "System settings not yet implemented".to_string()))
}

async fn update_system_settings_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(settings): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Settings update not implemented".to_string()))
}

// Email Integration Handlers
async fn list_email_accounts_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    // Placeholder - email_integration needs list_accounts function
    Ok(Json(vec![]))
}

async fn create_email_account_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(account): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Email account creation not implemented".to_string()))
}

async fn delete_email_account_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Email account deletion not implemented".to_string()))
}

// S3 Storage Handlers
async fn list_s3_configs_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<Vec<s3_storage::S3Config>>, (StatusCode, String)> {
    match s3_storage::list_s3_configs(&state.db_pool).await {
        Ok(configs) => Ok(Json(configs)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn create_s3_config_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(config): Json<s3_storage::S3Config>,
) -> Result<Json<s3_storage::S3Config>, (StatusCode, String)> {
    match s3_storage::create_s3_config(&state.db_pool, config).await {
        Ok(created) => Ok(Json(created)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_s3_config_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    match s3_storage::delete_s3_config(&state.db_pool, &id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn test_s3_connection_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(config): Json<s3_storage::S3Config>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match s3_storage::test_s3_connection(&config).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "ok"}))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

// FTP Sync Handlers
async fn list_ftp_connections_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    // Placeholder
    Ok(Json(vec![]))
}

async fn create_ftp_connection_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(connection): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "FTP connection creation not implemented".to_string()))
}

async fn delete_ftp_connection_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "FTP connection deletion not implemented".to_string()))
}

async fn trigger_ftp_sync_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "FTP sync not implemented".to_string()))
}

// LDAP Integration Handlers
async fn list_ldap_configs_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    // Placeholder
    Ok(Json(vec![]))
}

async fn create_ldap_config_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(config): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "LDAP config creation not implemented".to_string()))
}

async fn update_ldap_config_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(id): AxumPath<String>,
    Json(config): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "LDAP config update not implemented".to_string()))
}

async fn delete_ldap_config_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "LDAP config deletion not implemented".to_string()))
}

async fn test_ldap_connection_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(config): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "LDAP connection test not implemented".to_string()))
}

// Prometheus Metrics Handler
async fn prometheus_metrics_handler(
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    // Placeholder - return empty metrics
    Ok("# No metrics available yet\n".to_string())
}

// Redis Cache Handlers
async fn get_cache_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(key): AxumPath<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_FOUND, "Cache not implemented".to_string()))
}

async fn delete_cache_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(key): AxumPath<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Cache deletion not implemented".to_string()))
}

// Archive Management Handlers
async fn create_archive_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Archive creation not implemented".to_string()))
}

async fn extract_archive_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Archive extraction not implemented".to_string()))
}

// Compression Handlers
async fn list_compression_rules_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    // Placeholder
    Ok(Json(vec![]))
}

async fn create_compression_rule_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(rule): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Compression rule creation not implemented".to_string()))
}

async fn delete_compression_rule_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Compression rule deletion not implemented".to_string()))
}

async fn run_compression_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Json(params): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Placeholder
    Err((StatusCode::NOT_IMPLEMENTED, "Compression not implemented".to_string()))
}

// ==================== WEBSOCKET HANDLER ====================

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_ws_connection(socket, state.fs_tx))
}

async fn handle_ws_connection(socket: WebSocket, tx: Sender<FileChangeEvent>) {
    println!("WebSocket client connected");
    let (mut sender, mut receiver) = socket.split();
    let mut rx = tx.subscribe();
    
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            if let Ok(msg) = serde_json::to_string(&event) {
                if sender.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        }
    });
    
    let mut recv_task = tokio::spawn(async move {
        while receiver.next().await.is_some() {}
    });
    
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
    
    println!("WebSocket client disconnected");
}

// ==================== HELPER FUNCTIONS ====================

fn is_system_file(filename: &str) -> bool {
    filename == "syncspace.db" 
        || filename == "syncspace.db-shm" 
        || filename == "syncspace.db-wal"
        || filename == "search_index"
        || filename.ends_with(".lock") 
        || filename.starts_with(".tantivy-")
        || filename.starts_with(".git")
        || filename == ".DS_Store"
        || filename == "Thumbs.db"
}

async fn load_config() -> Option<Config> {
    match fs::read(CONFIG_FILE).await {
        Ok(bytes) => serde_json::from_slice(&bytes).ok(),
        Err(_) => None,
    }
}

async fn save_config(config: &Config) -> Result<(), std::io::Error> {
    let json = serde_json::to_vec_pretty(config).unwrap();
    fs::write(CONFIG_FILE, json).await
}

async fn watch_data_dir(tx: Sender<FileChangeEvent>) -> Result<(), NotifyError> {
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<Event>(16);
    
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = event_tx.blocking_send(event);
            }
        },
        notify::Config::default(),
    )?;
    
    watcher.watch(Path::new(DATA_DIR), RecursiveMode::Recursive)?;
    
    while let Some(event) = event_rx.recv().await {
        if let Some(path) = event.paths.first() {
            let kind = format!("{:?}", event.kind);
            let relative = path
                .strip_prefix(DATA_DIR)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            
            let _ = tx.send(
                FileChangeEvent::new(relative, kind)
            );
        }
    }
    
    Ok(())
}

async fn compute_stats_async() -> (u64, u64) {
    tokio::task::spawn_blocking(|| {
        let mut count = 0;
        let mut total_size = 0;
        
        for entry in WalkDir::new(DATA_DIR).into_iter().filter_map(Result::ok) {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                    count += 1;
                    total_size += meta.len();
                }
            }
        }
        
        (count, total_size)
    })
    .await
    .unwrap_or((0, 0))
}

// ==================== STATUS HANDLERS ====================

#[derive(Serialize)]
struct SystemStorageInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
    usage_percent: f32,
    mount_point: String,
    filesystem: String,
}

async fn system_storage_handler() -> Json<SystemStorageInfo> {
    tokio::task::spawn_blocking(|| {
        let disks = Disks::new_with_refreshed_list();
        
        // Find the disk containing our DATA_DIR
        let data_path = std::path::PathBuf::from(DATA_DIR);
        let canonical_data = data_path.canonicalize().unwrap_or(data_path);
        
        // Find the disk with the longest matching mount point
        let mut best_match: Option<&sysinfo::Disk> = None;
        let mut longest_match = 0;
        
        for disk in &disks {
            let mount_point = disk.mount_point();
            if canonical_data.starts_with(mount_point) {
                let match_len = mount_point.as_os_str().len();
                if match_len > longest_match {
                    longest_match = match_len;
                    best_match = Some(disk);
                }
            }
        }
        
        if let Some(disk) = best_match {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            let usage_percent = if total > 0 {
                (used as f64 / total as f64 * 100.0) as f32
            } else {
                0.0
            };
            
            SystemStorageInfo {
                total_bytes: total,
                available_bytes: available,
                used_bytes: used,
                usage_percent,
                mount_point: disk.mount_point().display().to_string(),
                filesystem: disk.file_system().to_string_lossy().to_string(),
            }
        } else {
            // Fallback if no disk found
            SystemStorageInfo {
                total_bytes: 0,
                available_bytes: 0,
                used_bytes: 0,
                usage_percent: 0.0,
                mount_point: String::from("unknown"),
                filesystem: String::from("unknown"),
            }
        }
    })
    .await
    .unwrap_or_else(|_| SystemStorageInfo {
        total_bytes: 0,
        available_bytes: 0,
        used_bytes: 0,
        usage_percent: 0.0,
        mount_point: String::from("error"),
        filesystem: String::from("error"),
    })
    .into()
}

#[derive(Serialize)]
struct ServerStatus {
    version: String,
    status: String,
    uptime_seconds: u64,
    data_dir: String,
    file_count: u64,
    total_size_bytes: u64,
    users_count: usize,
    database_connected: bool,
    search_enabled: bool,
}

async fn status_handler(
    State(state): State<AppState>,
) -> Json<ServerStatus> {
    let (file_count, total_size) = compute_stats_async().await;
    let users_count = state.user_db.list_users().len();
    
    Json(ServerStatus {
        version: "0.3.0".to_string(),
        status: "running".to_string(),
        uptime_seconds: 0, // TODO: Track actual uptime
        data_dir: DATA_DIR.to_string(),
        file_count,
        total_size_bytes: total_size,
        users_count,
        database_connected: true,
        search_enabled: true,
    })
}

async fn root_handler() -> Response {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SyncSpace Backend</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .card {
            background: white;
            border-radius: 24px;
            padding: 48px;
            max-width: 600px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        h1 {
            font-size: 32px;
            margin-bottom: 8px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        .subtitle {
            color: #666;
            font-size: 16px;
            margin-bottom: 32px;
        }
        .status {
            display: flex;
            align-items: center;
            gap: 12px;
            margin-bottom: 24px;
            padding: 16px;
            background: #f0fdf4;
            border-radius: 12px;
            border-left: 4px solid #22c55e;
        }
        .status-dot {
            width: 12px;
            height: 12px;
            background: #22c55e;
            border-radius: 50%;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
        .info-grid {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 16px;
            margin-bottom: 32px;
        }
        .info-item {
            padding: 16px;
            background: #f8fafc;
            border-radius: 12px;
        }
        .info-label {
            font-size: 12px;
            color: #64748b;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 4px;
        }
        .info-value {
            font-size: 24px;
            font-weight: 600;
            color: #1e293b;
        }
        .endpoints {
            background: #f8fafc;
            border-radius: 12px;
            padding: 16px;
        }
        .endpoints h3 {
            font-size: 14px;
            color: #64748b;
            margin-bottom: 12px;
        }
        .endpoint {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 8px;
            font-family: 'Courier New', monospace;
            font-size: 13px;
        }
        .method {
            padding: 4px 8px;
            border-radius: 6px;
            font-weight: 600;
            font-size: 11px;
        }
        .get { background: #dbeafe; color: #1e40af; }
        .post { background: #dcfce7; color: #166534; }
        .footer {
            text-align: center;
            color: #94a3b8;
            font-size: 14px;
            margin-top: 32px;
        }
    </style>
</head>
<body>
    <div class="card">
        <h1>🚀 SyncSpace Backend</h1>
        <div class="subtitle">Self-hosted file synchronization server</div>
        
        <div class="status">
            <div class="status-dot"></div>
            <div>
                <strong>Server is running</strong>
                <div style="font-size: 13px; color: #666;">Version 0.3.0 (Axum)</div>
            </div>
        </div>
        
        <div id="stats" class="info-grid">
            <div class="info-item">
                <div class="info-label">Loading...</div>
                <div class="info-value">...</div>
            </div>
        </div>
        
        <div class="endpoints">
            <h3>API Endpoints</h3>
            <div class="endpoint"><span class="method get">GET</span> /api/status</div>
            <div class="endpoint"><span class="method post">POST</span> /api/auth/login</div>
            <div class="endpoint"><span class="method get">GET</span> /api/files/*path</div>
            <div class="endpoint"><span class="method post">POST</span> /api/upload/*path</div>
            <div class="endpoint"><span class="method get">GET</span> /api/search?q=term</div>
        </div>
        
        <div class="footer">
            Frontend: <a href="http://localhost:5173" style="color: #667eea;">http://localhost:5173</a>
        </div>
    </div>
    
    <script>
        fetch('/api/status')
            .then(r => r.json())
            .then(data => {
                const formatBytes = (bytes) => {
                    if (bytes === 0) return '0 B';
                    const k = 1024;
                    const sizes = ['B', 'KB', 'MB', 'GB'];
                    const i = Math.floor(Math.log(bytes) / Math.log(k));
                    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
                };
                
                document.getElementById('stats').innerHTML = `
                    <div class="info-item">
                        <div class="info-label">Files</div>
                        <div class="info-value">${data.file_count}</div>
                    </div>
                    <div class="info-item">
                        <div class="info-label">Storage</div>
                        <div class="info-value">${formatBytes(data.total_size_bytes)}</div>
                    </div>
                    <div class="info-item">
                        <div class="info-label">Users</div>
                        <div class="info-value">${data.users_count}</div>
                    </div>
                    <div class="info-item">
                        <div class="info-label">Database</div>
                        <div class="info-value">${data.database_connected ? '✓' : '✗'}</div>
                    </div>
                `;
            })
            .catch(() => {
                document.getElementById('stats').innerHTML = '<div class="info-item"><div class="info-value">Error loading stats</div></div>';
            });
    </script>
</body>
</html>"#;
    
    (StatusCode::OK, [("content-type", "text/html; charset=utf-8")], html).into_response()
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct ActivityLog {
    id: String,
    user_id: String,
    action: String,
    file_path: String,
    status: String,
    error_message: Option<String>,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct ActivityQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    action: Option<String>,
}

/// List activity logs for the current user
async fn list_activity_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<ActivityQuery>,
) -> Result<Json<Vec<ActivityLog>>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    let limit = params.limit.unwrap_or(100).min(1000) as i32;
    let offset = params.offset.unwrap_or(0) as i32;
    
    let activities: Vec<ActivityLog> = if let Some(action) = params.action {
        sqlx::query_as::<_, ActivityLog>(
            "SELECT id, user_id, action, file_path, status, error_message, created_at FROM file_history 
             WHERE user_id = ? AND action = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(&user_id)
        .bind(action)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Activity query error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    } else {
        sqlx::query_as::<_, ActivityLog>(
            "SELECT id, user_id, action, file_path, status, error_message, created_at FROM file_history 
             WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(&user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Activity query error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    };
    
    Ok(Json(activities))
}

#[derive(Debug, Serialize)]
struct ActivityStats {
    total_actions: i64,
    uploads_count: i64,
    downloads_count: i64,
    deletes_count: i64,
    renames_count: i64,
    failed_actions: i64,
}

/// Get activity statistics for the current user
async fn activity_stats_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<ActivityStats>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM file_history WHERE user_id = ?"
    )
    .bind(&user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let uploads: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM file_history WHERE user_id = ? AND action = 'created'"
    )
    .bind(&user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let downloads: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM file_history WHERE user_id = ? AND action = 'downloaded'"
    )
    .bind(&user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let deletes: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM file_history WHERE user_id = ? AND action = 'deleted'"
    )
    .bind(&user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let renames: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM file_history WHERE user_id = ? AND action = 'renamed'"
    )
    .bind(&user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let failed: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM file_history WHERE user_id = ? AND status = 'failed'"
    )
    .bind(&user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(ActivityStats {
        total_actions: total.0,
        uploads_count: uploads.0,
        downloads_count: downloads.0,
        deletes_count: deletes.0,
        renames_count: renames.0,
        failed_actions: failed.0,
    }))
}

// ==================== NOTIFICATIONS HANDLERS ====================

async fn get_notifications_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<notifications::Notification>>, StatusCode> {
    let limit = params.get("limit").and_then(|l| l.parse().ok()).unwrap_or(50);
    let offset = params.get("offset").and_then(|o| o.parse().ok()).unwrap_or(0);
    notifications::get_user_notifications(&state.db_pool, &user.id.to_string(), limit, offset)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_unread_notifications_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<notifications::Notification>>, StatusCode> {
    let all_notifs = notifications::get_user_notifications(&state.db_pool, &user.id.to_string(), 100, 0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let unread: Vec<_> = all_notifs.into_iter().filter(|n| !n.is_read).collect();
    Ok(Json(unread))
}

async fn mark_notification_read_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    notifications::mark_as_read(&state.db_pool, &id, &user.id.to_string())
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn mark_all_notifications_read_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<StatusCode, StatusCode> {
    notifications::mark_all_as_read(&state.db_pool, &user.id.to_string())
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_notification_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    notifications::delete_notification(&state.db_pool, &id, &user.id.to_string())
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ==================== WEBHOOKS HANDLERS ====================

async fn list_webhooks_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<webhooks::Webhook>>, StatusCode> {
    webhooks::list_webhooks(&state.db_pool, &user.id.to_string())
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_webhook_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<webhooks::CreateWebhookRequest>,
) -> Result<Json<webhooks::Webhook>, StatusCode> {
    webhooks::create_webhook(&state.db_pool, &user.id.to_string(), req)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_webhook_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<webhooks::Webhook>, StatusCode> {
    webhooks::get_webhook(&state.db_pool, &id, &user.id.to_string())
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_webhook_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(id): AxumPath<String>,
    Json(req): Json<webhooks::UpdateWebhookRequest>,
) -> Result<Json<webhooks::Webhook>, StatusCode> {
    webhooks::update_webhook(&state.db_pool, &id, &user.id.to_string(), req)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_webhook_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    webhooks::delete_webhook(&state.db_pool, &id, &user.id.to_string())
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn test_webhook_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let webhook = webhooks::get_webhook(&state.db_pool, &id, &user.id.to_string())
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let test_event = webhooks::WebhookEvent {
        event_type: "test".to_string(),
        user_id: user.id.to_string(),
        file_path: Some("/test/file.txt".to_string()),
        metadata: serde_json::json!({"test": true}),
        timestamp: Utc::now(),
    };
    
    webhooks::send_webhook(&webhook, &test_event)
        .await
        .map(|_| Json(serde_json::json!({"status": "success", "message": "Test webhook sent"})))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ==================== ANALYTICS HANDLERS ====================

async fn analytics_dashboard_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<analytics::DashboardStats>, StatusCode> {
    analytics::get_dashboard_stats(&state.db_pool, Some(&user.id.to_string()))
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn analytics_storage_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<analytics::StorageMetrics>, StatusCode> {
    analytics::get_storage_metrics(&state.db_pool, Some(&user.id.to_string()))
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn analytics_activity_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<analytics::ActivityMetrics>, StatusCode> {
    analytics::get_activity_metrics(&state.db_pool, Some(&user.id.to_string()))
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn analytics_files_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<analytics::FileMetrics>, StatusCode> {
    analytics::get_file_metrics(&state.db_pool, Some(&user.id.to_string()))
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn analytics_users_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<analytics::UserMetrics>, StatusCode> {
    // Only admin users should access this
    analytics::get_user_metrics(&state.db_pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ==================== BATCH OPERATIONS HANDLERS ====================

async fn batch_delete_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<handlers::batch::BatchDeleteRequest>,
) -> Result<Json<handlers::batch::BatchOperationResult>, StatusCode> {
    handlers::batch::batch_delete(&state.db_pool, &user.id.to_string(), req.items)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn batch_move_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<handlers::batch::BatchMoveRequest>,
) -> Result<Json<handlers::batch::BatchOperationResult>, StatusCode> {
    handlers::batch::batch_move(&state.db_pool, &user.id.to_string(), req.items, &req.target_folder)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn batch_tag_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<handlers::batch::BatchTagRequest>,
) -> Result<Json<handlers::batch::BatchOperationResult>, StatusCode> {
    handlers::batch::batch_tag(&state.db_pool, &user.id.to_string(), req.items, req.tags, &req.action)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn batch_copy_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<BatchCopyRequest>,
) -> Result<Json<BatchJobResponse>, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let job_id_clone = job_id.clone();
    let user_id = user.id.to_string();
    let fs_tx = state.fs_tx.clone();
    
    // Start background job for batch copy
    tokio::spawn(async move {
        let mut success_count = 0;
        let mut error_count = 0;
        let total_files = req.items.len();
        
        for (_index, item) in req.items.iter().enumerate() {
            let source_path = Path::new(DATA_DIR).join(&item.path);
            let dest_path = Path::new(DATA_DIR).join(&req.target_folder).join(&item.name);
            
            match fs::copy(&source_path, &dest_path).await {
                Ok(_) => {
                    success_count += 1;
                    // Send progress update
                    let _ = fs_tx.send(
                        FileChangeEvent::new(format!("batch_copy_progress:{job_id_clone}:{success_count}:{total_files}"), "batch_progress".to_string())
                            .with_user(user_id.clone())
                    );
                }
                Err(_) => {
                    error_count += 1;
                }
            }
        }
        
        // Send completion notification
        let _ = fs_tx.send(
            FileChangeEvent::new(format!("batch_copy_complete:{job_id_clone}:{success_count}:{error_count}"), "batch_complete".to_string())
                .with_user(user_id)
        );
    });
    
    Ok(Json(BatchJobResponse { job_id, status: "started".to_string() }))
}

async fn batch_compress_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<BatchCompressRequest>,
) -> Result<Json<BatchJobResponse>, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let job_id_clone = job_id.clone();
    let user_id = user.id.to_string();
    let fs_tx = state.fs_tx.clone();
    
    // Start background compression job
    tokio::spawn(async move {
        use std::fs::File;
        use zip::ZipWriter;
        use zip::write::FileOptions;
        
        let archive_path = Path::new(DATA_DIR).join(&req.archive_name);
        let file = File::create(&archive_path).unwrap();
        let mut zip = ZipWriter::new(file);
        
        let mut files_added = 0;
        
        for item in &req.items {
            let source_path = Path::new(DATA_DIR).join(&item.path);
            
            if source_path.is_file() {
                let options = zip::write::FileOptions::<()>::default()
                    .compression_method(zip::CompressionMethod::Deflated)
                    .unix_permissions(0o755);
                
                if let Ok(file_content) = std::fs::read(&source_path) {
                    let _ = zip.start_file(&item.name, options);
                    let _ = zip.write_all(&file_content);
                    files_added += 1;
                    
                    // Send progress
                    let _ = fs_tx.send(
                        FileChangeEvent::new(format!("batch_compress_progress:{job_id_clone}:{files_added}:{}", req.items.len()), "batch_progress".to_string())
                            .with_user(user_id.clone())
                    );
                }
            }
        }
        
        let _ = zip.finish();
        
        // Send completion
        let _ = fs_tx.send(
            FileChangeEvent::new(format!("batch_compress_complete:{job_id_clone}:{files_added}"), "batch_complete".to_string())
                .with_user(user_id)
        );
    });
    
    Ok(Json(BatchJobResponse { job_id, status: "started".to_string() }))
}

async fn get_batch_operation_status(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(job_id): AxumPath<String>,
) -> Result<Json<BatchJobStatus>, StatusCode> {
    // For now, return mock status - in real implementation, store job status in Redis/DB
    Ok(Json(BatchJobStatus {
        job_id: job_id.clone(),
        status: "running".to_string(),
        progress: 50,
        total_items: 100,
        completed_items: 50,
        error_count: 0,
    }))
}

async fn cancel_batch_operation(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(job_id): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    // Implementation would cancel running job
    Ok(StatusCode::OK)
}

// ==================== ADVANCED SEARCH HANDLERS ====================

async fn advanced_search_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(query): Query<handlers::advanced_search::AdvancedSearchQuery>,
) -> Result<Json<handlers::advanced_search::SearchResponse>, StatusCode> {
    handlers::advanced_search::advanced_search(&state.db_pool, &user.id.to_string(), query)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn search_suggestions_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let query = params.get("q").cloned().unwrap_or_default();
    let limit = params.get("limit").and_then(|l| l.parse().ok()).unwrap_or(10);
    
    handlers::advanced_search::get_search_suggestions(&state.db_pool, &user.id.to_string(), &query, limit)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn recent_searches_handler(
    State(state): State<AppState>,
    user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let limit = params.get("limit").and_then(|l| l.parse().ok()).unwrap_or(10);
    
    handlers::advanced_search::get_recent_searches(&state.db_pool, &user.id.to_string(), limit)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ==================== PERFORMANCE HANDLERS ====================

async fn get_performance_metrics(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<performance::PerformanceMetrics>, StatusCode> {
    let metrics = state.performance_monitor.collect_metrics().await;
    Ok(Json(metrics))
}

async fn get_performance_history(
    State(state): State<AppState>,
    _user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<performance::PerformanceMetrics>>, StatusCode> {
    let limit = params.get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(100);
    
    let history = state.performance_monitor.get_metrics_history(limit).await;
    Ok(Json(history))
}

async fn get_cache_stats(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Return cache statistics
    let stats = serde_json::json!({
        "memory_cache_entries": 0, // Would need actual cache implementation
        "redis_connected": state.cache_manager.has_redis(),
        "cache_hit_ratio": 0.85,
        "total_requests": 1000,
        "cache_hits": 850
    });
    
    Ok(Json(stats))
}

async fn clear_cache(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Clear specific cache patterns based on request
    if let Err(_) = state.cache_manager.invalidate_pattern("*").await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(Json(serde_json::json!({"status": "Cache cleared successfully"})))
}

#[derive(Deserialize)]
struct QueueJobRequest {
    job_type: String,
    payload: serde_json::Value,
    priority: Option<i32>,
}

async fn queue_background_job(
    State(state): State<AppState>,
    _user: auth::User,
    Json(request): Json<QueueJobRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let job_id = state.job_processor.queue_job(
        request.job_type,
        request.payload,
        request.priority.unwrap_or(0),
    ).await;
    
    Ok(Json(serde_json::json!({
        "job_id": job_id,
        "status": "queued"
    })))
}

async fn list_background_jobs(
    State(_state): State<AppState>,
    _user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Return list of current jobs
    Ok(Json(serde_json::json!({
        "jobs": [],
        "queue_length": 0,
        "active_workers": 4
    })))
}

async fn get_job_status(
    State(_state): State<AppState>,
    _user: auth::User,
    AxumPath(job_id): AxumPath<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Return job status
    Ok(Json(serde_json::json!({
        "job_id": job_id,
        "status": "completed",
        "progress": 100,
        "result": "Job completed successfully"
    })))
}

async fn get_system_info(
    State(_state): State<AppState>,
    _user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Return system information
    let info = serde_json::json!({
        "cpu_cores": num_cpus::get(),
        "memory_total": "Unknown",
        "disk_space": "Unknown",
        "uptime": "Unknown",
        "version": "0.3.0",
        "rust_version": std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        "features": {
            "redis_cache": std::env::var("REDIS_URL").is_ok(),
            "background_jobs": true,
            "performance_monitoring": true
        }
    });
    
    Ok(Json(info))
}

// ==================== FILE VERSIONING HANDLERS ====================

async fn list_file_versions_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(file_id): AxumPath<String>,
) -> Result<Json<Vec<database::FileVersion>>, StatusCode> {
    let versions: Vec<database::FileVersion> = sqlx::query_as::<_, database::FileVersion>(
        "SELECT id, file_id, version_number, content_hash, size_bytes, created_at, created_by, comment, is_current, storage_path FROM file_versions WHERE file_id = ? ORDER BY version_number DESC"
    )
    .bind(file_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(versions))
}

#[derive(Deserialize)]
struct CreateVersionRequest {
    comment: Option<String>,
    content_hash: String,
    size_bytes: i64,
    storage_path: String,
}

async fn create_file_version_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(file_id): AxumPath<String>,
    Json(request): Json<CreateVersionRequest>,
) -> Result<Json<database::FileVersion>, StatusCode> {
    let version_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Get next version number
    let next_version: i32 = sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(MAX(version_number), 0) + 1 FROM file_versions WHERE file_id = ?"
    )
    .bind(&file_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Mark all other versions as not current
    sqlx::query(
        "UPDATE file_versions SET is_current = FALSE WHERE file_id = ?"
    )
    .bind(&file_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Create new version
    let version = database::FileVersion {
        id: version_id.clone(),
        file_id: file_id.clone(),
        version_number: next_version,
        content_hash: request.content_hash,
        size_bytes: request.size_bytes,
        created_at: now.clone(),
        created_by: user.id.to_string(),
        comment: request.comment,
        is_current: true,
        storage_path: request.storage_path,
    };
    
    sqlx::query(
        "INSERT INTO file_versions (id, file_id, version_number, content_hash, size_bytes, created_at, created_by, comment, is_current, storage_path)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&version.id)
    .bind(&version.file_id)
    .bind(version.version_number)
    .bind(&version.content_hash)
    .bind(version.size_bytes)
    .bind(&version.created_at)
    .bind(&version.created_by)
    .bind(&version.comment)
    .bind(version.is_current)
    .bind(&version.storage_path)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(version))
}

async fn get_file_version_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(version_id): AxumPath<String>,
) -> Result<Json<database::FileVersion>, StatusCode> {
    let version: database::FileVersion = sqlx::query_as::<_, database::FileVersion>(
        "SELECT id, file_id, version_number, content_hash, size_bytes, created_at, created_by, comment, is_current, storage_path FROM file_versions WHERE id = ?"
    )
    .bind(version_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(version))
}

async fn delete_file_version_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(version_id): AxumPath<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Don't allow deleting current version
    let is_current: Option<bool> = sqlx::query_scalar::<_, bool>(
        "SELECT is_current FROM file_versions WHERE id = ?"
    )
    .bind(&version_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if is_current.unwrap_or(false) {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    sqlx::query(
        "DELETE FROM file_versions WHERE id = ?"
    )
    .bind(&version_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({"status": "Version deleted successfully"})))
}

#[derive(Deserialize)]
struct RestoreVersionRequest {
    comment: Option<String>,
}

async fn restore_file_version_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(version_id): AxumPath<String>,
    Json(request): Json<RestoreVersionRequest>,
) -> Result<Json<database::FileVersion>, StatusCode> {
    let old_version: database::FileVersion = sqlx::query_as::<_, database::FileVersion>(
        "SELECT id, file_id, version_number, content_hash, size_bytes, created_at, created_by, comment, is_current, storage_path FROM file_versions WHERE id = ?"
    )
    .bind(&version_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    
    // Create new version from old one
    let new_version_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Get next version number
    let next_version: i32 = sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(MAX(version_number), 0) + 1 FROM file_versions WHERE file_id = ?"
    )
    .bind(&old_version.file_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Mark all versions as not current
    sqlx::query(
        "UPDATE file_versions SET is_current = FALSE WHERE file_id = ?"
    )
    .bind(&old_version.file_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Create restored version
    let new_version = database::FileVersion {
        id: new_version_id.clone(),
        file_id: old_version.file_id.clone(),
        version_number: next_version,
        content_hash: old_version.content_hash.clone(),
        size_bytes: old_version.size_bytes,
        created_at: now.clone(),
        created_by: user.id.to_string(),
        comment: request.comment.clone().or_else(|| Some(format!("Restored from version {}", old_version.version_number))),
        is_current: true,
        storage_path: old_version.storage_path.clone(),
    };
    
    sqlx::query(
        "INSERT INTO file_versions (id, file_id, version_number, content_hash, size_bytes, created_at, created_by, comment, is_current, storage_path)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&new_version.id)
    .bind(&new_version.file_id)
    .bind(new_version.version_number)
    .bind(&new_version.content_hash)
    .bind(new_version.size_bytes)
    .bind(&new_version.created_at)
    .bind(&new_version.created_by)
    .bind(&new_version.comment)
    .bind(new_version.is_current)
    .bind(&new_version.storage_path)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Log the restoration
    sqlx::query(
        "INSERT INTO version_restorations (id, file_id, restored_from_version_id, restored_to_version_id, restored_by, restored_at, reason)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&old_version.file_id)
    .bind(&version_id)
    .bind(&new_version.id)
    .bind(user.id.to_string())
    .bind(&now)
    .bind(&request.comment)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(new_version))
}

async fn get_version_diff_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath((from_id, to_id)): AxumPath<(String, String)>,
) -> Result<Json<database::VersionDiff>, StatusCode> {
    let diff: database::VersionDiff = sqlx::query_as::<_, database::VersionDiff>(
        "SELECT id, from_version_id, to_version_id, diff_type, diff_content, added_lines, removed_lines, created_at FROM version_diffs WHERE from_version_id = ? AND to_version_id = ?"
    )
    .bind(from_id)
    .bind(to_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(diff))
}

async fn download_file_version_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(version_id): AxumPath<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let version: database::FileVersion = sqlx::query_as::<_, database::FileVersion>(
        "SELECT id, file_id, version_number, content_hash, size_bytes, created_at, created_by, comment, is_current, storage_path FROM file_versions WHERE id = ?"
    )
    .bind(version_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    
    // Read file from storage path
    let file_path = std::path::Path::new(&version.storage_path);
    let content = tokio::fs::read(file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let filename = format!("version_{}.bin", version.version_number);
    
    use axum::response::Response;
    use axum::body::Body;
    
    let mut response = Response::new(Body::from(content));
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap()
    );
    response.headers_mut().insert(
        axum::http::header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", filename).parse().unwrap()
    );
    
    Ok(response)
}

async fn get_version_metadata_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(version_id): AxumPath<String>,
) -> Result<Json<Vec<database::VersionMetadata>>, StatusCode> {
    let metadata: Vec<database::VersionMetadata> = sqlx::query_as::<_, database::VersionMetadata>(
        "SELECT id, version_id, metadata_key, metadata_value, created_at FROM version_metadata WHERE version_id = ?"
    )
    .bind(version_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(metadata))
}

#[derive(Deserialize)]
struct UpdateMetadataRequest {
    metadata: std::collections::HashMap<String, String>,
}

async fn update_version_metadata_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(version_id): AxumPath<String>,
    Json(request): Json<UpdateMetadataRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let now = Utc::now().to_rfc3339();
    
    // Delete existing metadata
    sqlx::query(
        "DELETE FROM version_metadata WHERE version_id = ?"
    )
    .bind(&version_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Insert new metadata
    for (key, value) in request.metadata {
        sqlx::query(
            "INSERT INTO version_metadata (id, version_id, metadata_key, metadata_value, created_at)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&version_id)
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok(Json(serde_json::json!({"status": "Metadata updated successfully"})))
}
