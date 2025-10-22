//! SyncSpace backend server - Axum version
//! Migrated from warp 0.3 to axum 0.7 for better multipart support and modern async patterns

mod auth;
mod database;
mod search;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, DefaultBodyLimit, Multipart, Path as AxumPath, Query, State},
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post, put},
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
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;
use walkdir::WalkDir;

use auth::{
    AuthResponse, ChangePasswordRequest, Enable2FARequest, LoginRequest, RateLimiter,
    RegisterRequest, Setup2FAResponse, UserDB, UserInfo,
};

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
}

// ==================== DATA STRUCTURES ====================

#[derive(Serialize)]
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
    kind: String,
    timestamp: DateTime<Utc>,
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
    // Auth routes (public)
    let auth_routes = Router::new()
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/2fa/setup", post(setup_2fa_handler))
        .route("/api/auth/2fa/enable", post(enable_2fa_handler))
        .route("/api/auth/2fa/disable", post(disable_2fa_handler))
        .route("/api/auth/change-password", put(change_password_handler))
        .route("/api/auth/me", get(me_handler));
    
    // File routes (protected)
    let file_routes = Router::new()
        .route("/api/files/*path", get(list_files_handler))
        .route("/api/file/*path", get(download_file_handler))
        .route("/api/upload/*path", post(upload_file_handler))
        .route("/api/upload-multipart", post(upload_multipart_handler))
        .route("/api/files/*path", delete(delete_file_handler))
        .route("/api/dirs/*path", post(create_dir_handler))
        .route("/api/rename/*path", put(rename_file_handler));
    
    // Trash routes (protected)
    let trash_routes = Router::new()
        .route("/api/trash", get(list_trash_handler))
        .route("/api/trash/restore/*path", post(restore_trash_handler))
        .route("/api/trash/permanent/*path", delete(permanent_delete_handler))
        .route("/api/trash/cleanup", delete(cleanup_trash_handler))
        .route("/api/trash/empty", delete(empty_trash_handler));
    
    // Utility routes (protected)
    let utility_routes = Router::new()
        .route("/api/search", get(search_handler))
        .route("/api/stats", get(stats_handler))
        .route("/api/config", get(get_config_handler).put(put_config_handler))
        .route("/api/peers", get(list_peers_handler).post(add_peer_handler));
    
    // WebSocket route
    let ws_route = Router::new()
        .route("/api/ws", get(ws_handler));
    
    // Combine all routes
    Router::new()
        .merge(auth_routes)
        .merge(file_routes)
        .merge(trash_routes)
        .merge(utility_routes)
        .merge(ws_route)
        .nest_service("/", ServeFile::new("../frontend/index.html"))
        .nest_service("/static", ServeDir::new("../frontend"))
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
            match auth::generate_token(&user) {
                Ok(token) => {
                    let response = AuthResponse {
                        token,
                        user: UserInfo {
                            id: user.id.to_string(),
                            username: user.username,
                            totp_enabled: user.totp_enabled,
                        },
                        requires_2fa: false,
                    };
                    Ok(Json(response))
                }
                Err(e) => Err((
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

            match auth::generate_token(&user) {
                Ok(token) => Ok(Json(AuthResponse {
                    token,
                    user: UserInfo {
                        id: user.id.to_string(),
                        username: user.username,
                        totp_enabled: user.totp_enabled,
                    },
                    requires_2fa: false,
                })),
                Err(e) => Err((
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
    State(state): State<AppState>,
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

// ==================== FILE HANDLERS ====================

async fn list_files_handler(
    State(_state): State<AppState>,
    _user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<Json<Vec<EntryInfo>>, (StatusCode, Json<serde_json::Value>)> {
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
            Ok(Json(entries))
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Directory not found"})),
        )),
    }
}

async fn download_file_handler(
    State(_state): State<AppState>,
    _user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<Bytes, StatusCode> {
    let file_path = Path::new(DATA_DIR).join(&path);
    match fs::read(&file_path).await {
        Ok(bytes) => Ok(Bytes::from(bytes)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn upload_file_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(path): AxumPath<String>,
    body: Bytes,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let file_path = PathBuf::from(&path);
    
    if let Some(parent) = Path::new(DATA_DIR).join(&path).parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    fs::write(Path::new(DATA_DIR).join(&path), &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Broadcast event
    let _ = state.fs_tx.send(FileChangeEvent {
        path: path.clone(),
        kind: "create".to_string(),
        timestamp: Utc::now(),
    });
    
    // Background indexing
    let index = state.search_index.clone();
    let full_path = Path::new(DATA_DIR).join(&path);
    tokio::spawn(async move {
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
    _user: auth::User,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut uploaded_count = 0;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        if let Some(filename) = field.file_name().map(|s| s.to_string()) {
            let data = field.bytes().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let file_path = Path::new(DATA_DIR).join(&filename);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            }
            
            fs::write(&file_path, &data).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            // Broadcast event
            let _ = state.fs_tx.send(FileChangeEvent {
                path: filename.clone(),
                kind: "create".to_string(),
                timestamp: Utc::now(),
            });
            
            uploaded_count += 1;
        }
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
    let cleanup_days: i64 = sqlx::query_as::<_, (String,)>(
        "SELECT value FROM settings WHERE key = 'auto_trash_cleanup_days'"
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .and_then(|r| r.0.parse().ok())
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
            let folder_id: Option<String> = sqlx::query_as(
                "SELECT id FROM folders WHERE path = ? AND owner_id = ?"
            )
            .bind(&path)
            .bind(&user.id)
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
                .bind(&user.id)
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
                .bind(&user.id)
                .bind(&user.id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                id
            }
        } else {
            // Find or create file in database
            let file_id: Option<String> = sqlx::query_as(
                "SELECT id FROM files WHERE path = ? AND owner_id = ?"
            )
            .bind(&path)
            .bind(&user.id)
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
                .bind(&user.id)
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
                .bind(&user.id)
                .bind(size)
                .bind(storage_path)
                .bind(&user.id)
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
        
        sqlx::query(
            "INSERT INTO trash (id, item_type, item_id, original_path, original_name, deleted_by, deleted_at, auto_delete_at, size_bytes)
             VALUES (?, ?, ?, ?, ?, ?, datetime('now'), ?, ?)"
        )
        .bind(&trash_id)
        .bind(item_type)
        .bind(&item_id)
        .bind(&path)
        .bind(full_path.file_name().and_then(|n| n.to_str()).unwrap_or(""))
        .bind(&user.id)
        .bind(auto_delete_str)
        .bind(size)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Broadcast delete event
        let _ = state.fs_tx.send(FileChangeEvent {
            path: path.clone(),
            kind: "delete".to_string(),
            timestamp: Utc::now(),
        });
        
        Ok((StatusCode::OK, "moved to trash"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create_dir_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let full_path = Path::new(DATA_DIR).join(&path);
    fs::create_dir_all(&full_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let _ = state.fs_tx.send(FileChangeEvent {
        path,
        kind: "mkdir".to_string(),
        timestamp: Utc::now(),
    });
    
    Ok((StatusCode::CREATED, "created"))
}

async fn rename_file_handler(
    State(state): State<AppState>,
    _user: auth::User,
    AxumPath(old_path): AxumPath<String>,
    Json(req): Json<RenameRequest>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let old_full = Path::new(DATA_DIR).join(&old_path);
    let new_full = Path::new(DATA_DIR).join(&req.new_path);
    
    if let Some(parent) = new_full.parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    fs::rename(old_full, new_full).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let _ = state.fs_tx.send(FileChangeEvent {
        path: req.new_path,
        kind: "rename".to_string(),
        timestamp: Utc::now(),
    });
    
    Ok((StatusCode::OK, "renamed"))
}

// ==================== TRASH HANDLERS ====================

#[derive(Serialize, sqlx::FromRow)]
struct TrashItem {
    id: String,
    item_type: String,
    original_path: String,
    original_name: String,
    deleted_at: String,
    deleted_by_username: String,
    size_bytes: i64,
    auto_delete_at: Option<String>,
}

async fn list_trash_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<TrashItem>>, StatusCode> {
    let pool = &state.db_pool;
    
    let items = sqlx::query_as::<_, TrashItem>(
        r#"
        SELECT 
            t.id,
            t.item_type,
            t.original_path,
            t.original_name,
            t.deleted_at,
            u.username as deleted_by_username,
            t.size_bytes,
            t.auto_delete_at
        FROM trash t
        JOIN users u ON t.deleted_by = u.id
        WHERE t.deleted_by = ?
        ORDER BY t.deleted_at DESC
        "#
    )
    .bind(&user.id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch trash: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(items))
}

async fn restore_trash_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    
    // Find trash item by original path
    let trash_item: Option<(String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, item_type, item_id, original_parent_id FROM trash 
         WHERE original_path = ? AND deleted_by = ?"
    )
    .bind(&path)
    .bind(&user.id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some((trash_id, item_type, item_id, _)) = trash_item {
        // Update the item to unmark deletion
        match item_type.as_str() {
            "file" => {
                sqlx::query(
                    "UPDATE files SET is_deleted = 0, deleted_at = NULL, deleted_by = NULL 
                     WHERE id = ?"
                )
                .bind(&item_id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            }
            "folder" => {
                sqlx::query(
                    "UPDATE folders SET is_deleted = 0, deleted_at = NULL, deleted_by = NULL 
                     WHERE id = ?"
                )
                .bind(&item_id)
                .execute(pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            }
            _ => return Err(StatusCode::BAD_REQUEST),
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Broadcast restore event
        let _ = state.fs_tx.send(FileChangeEvent {
            path: path.clone(),
            kind: "restore".to_string(),
            timestamp: Utc::now(),
        });
        
        Ok((StatusCode::OK, "restored"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn permanent_delete_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    
    // Find trash item
    let trash_item: Option<(String, String, String, String)> = sqlx::query_as(
        "SELECT id, item_type, item_id, original_path FROM trash 
         WHERE original_path = ? AND deleted_by = ?"
    )
    .bind(&path)
    .bind(&user.id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some((trash_id, item_type, item_id, original_path)) = trash_item {
        // Get storage path before deletion
        let storage_path: Option<String> = if item_type == "file" {
            sqlx::query_as("SELECT storage_path FROM files WHERE id = ?")
                .bind(&item_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten()
        } else {
            None
        };
        
        // Delete from database
        match item_type.as_str() {
            "file" => {
                sqlx::query("DELETE FROM files WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                // Delete physical file
                if let Some(storage_path) = storage_path {
                    let full_path = Path::new(&storage_path);
                    let _ = fs::remove_file(full_path).await;
                }
            }
            "folder" => {
                sqlx::query("DELETE FROM folders WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                // Delete physical folder
                let full_path = Path::new(DATA_DIR).join(&original_path);
                let _ = fs::remove_dir_all(full_path).await;
            }
            _ => return Err(StatusCode::BAD_REQUEST),
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok((StatusCode::OK, "permanently deleted"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn cleanup_trash_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = &state.db_pool;
    
    // Get auto_trash_cleanup_days setting
    let cleanup_days: i64 = sqlx::query_as(
        "SELECT value FROM settings WHERE key = 'auto_trash_cleanup_days'"
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .and_then(|(value,): (String,)| value.parse().ok())
    .unwrap_or(30);
    
    if cleanup_days == 0 {
        return Ok(Json(serde_json::json!({
            "deleted_count": 0,
            "message": "Auto-cleanup disabled"
        })));
    }
    
    // Find expired trash items
    let expired_items: Vec<(String, String, String)> = sqlx::query_as(
        r#"
        SELECT id, item_type, item_id 
        FROM trash 
        WHERE auto_delete_at IS NOT NULL 
        AND datetime(auto_delete_at) <= datetime('now')
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut deleted_count = 0;
    
    for (trash_id, item_type, item_id) in expired_items {
        // Delete from database
        match item_type.as_str() {
            "file" => {
                // Get storage path
                let storage_path: Option<String> = sqlx::query_as(
                    "SELECT storage_path FROM files WHERE id = ?"
                )
                .bind(&item_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
                
                sqlx::query("DELETE FROM files WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
                
                if let Some(storage_path) = storage_path {
                    let _ = fs::remove_file(Path::new(&storage_path)).await;
                }
            }
            "folder" => {
                sqlx::query("DELETE FROM folders WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
            }
            _ => continue,
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .ok();
        
        deleted_count += 1;
    }
    
    Ok(Json(serde_json::json!({
        "deleted_count": deleted_count,
        "message": format!("Cleaned up {} expired items", deleted_count)
    })))
}

async fn empty_trash_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = &state.db_pool;
    
    // Get all trash items for user
    let trash_items: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT id, item_type, item_id FROM trash WHERE deleted_by = ?"
    )
    .bind(&user.id)
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut deleted_count = 0;
    
    for (trash_id, item_type, item_id) in trash_items {
        // Delete from database
        match item_type.as_str() {
            "file" => {
                let storage_path: Option<String> = sqlx::query_as(
                    "SELECT storage_path FROM files WHERE id = ?"
                )
                .bind(&item_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
                
                sqlx::query("DELETE FROM files WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
                
                if let Some(storage_path) = storage_path {
                    let _ = fs::remove_file(Path::new(&storage_path)).await;
                }
            }
            "folder" => {
                sqlx::query("DELETE FROM folders WHERE id = ?")
                    .bind(&item_id)
                    .execute(pool)
                    .await
                    .ok();
            }
            _ => continue,
        }
        
        // Remove from trash
        sqlx::query("DELETE FROM trash WHERE id = ?")
            .bind(&trash_id)
            .execute(pool)
            .await
            .ok();
        
        deleted_count += 1;
    }
    
    Ok(Json(serde_json::json!({
        "deleted_count": deleted_count,
        "message": "Trash emptied"
    })))
}

// ==================== UTILITY HANDLERS ====================

async fn search_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let query = params.get("q").cloned().unwrap_or_default();
    let limit = params.get("limit").and_then(|s| s.parse().ok()).unwrap_or(50);
    let fuzzy = params.get("fuzzy").and_then(|s| s.parse().ok()).unwrap_or(true);
    
    match state.search_index.search(&query, limit, fuzzy) {
        Ok(results) => Json(serde_json::json!({
            "results": results,
            "total": results.len(),
            "query": query,
        })),
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
                if sender.send(Message::Text(msg)).await.is_err() {
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
            
            let _ = tx.send(FileChangeEvent {
                path: relative,
                kind,
                timestamp: Utc::now(),
            });
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
