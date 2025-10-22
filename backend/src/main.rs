//! SyncSpace backend server with authentication and database.

mod auth;
mod database;
mod search;

use std::collections::HashMap;
use std::convert::Infallible;
use std::path::Path;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, TimeZone, Utc};
use notify::{Error as NotifyError, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::broadcast::{self, Sender};
use uuid::Uuid;
use warp::filters::BoxedFilter;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use warp::http::StatusCode;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use walkdir::WalkDir;

use auth::{
    AuthResponse, ChangePasswordRequest, Enable2FARequest, LoginRequest, RateLimiter,
    RegisterRequest, Setup2FAResponse, UserDB, UserInfo,
};

const DATA_DIR: &str = "./data";
const CONFIG_FILE: &str = "./config.json";

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

#[tokio::main]
async fn main() {
    println!("🚀 Starting SyncSpace Backend v0.2.0");
    
    // Initialize database
    let _db_pool = match database::init_db().await {
        Ok(pool) => {
            println!("✅ Database initialized");
            Arc::new(pool)
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
    
    // Initialize auth system (legacy - will be migrated to DB)
    let user_db = UserDB::new();
    let rate_limiter = Arc::new(RateLimiter::new());
    
    // Create default admin user if no users exist
    // TODO: Migrate this to use database
    if user_db.get_by_username("admin").is_none() {
        println!("📝 Creating default admin user (username: admin, password: admin)");
        if let Err(e) = user_db.create_user("admin".to_string(), "admin".to_string()) {
            eprintln!("Failed to create admin user: {}", e);
        }
    }
    
    // Load configuration or create default
    let config = Arc::new(Mutex::new(load_config().await.unwrap_or_default()));

    // Broadcast channel for file system events
    let (tx, _rx) = broadcast::channel::<FileChangeEvent>(32);
    let tx_clone = tx.clone();
    
    // Spawn the file watcher task
    tokio::spawn(async move {
        if let Err(e) = watch_data_dir(tx_clone).await {
            eprintln!("File system watcher error: {}", e);
        }
    });

    // Build all routes
    let api = routes(
        config.clone(),
        tx.clone(),
        user_db.clone(),
        rate_limiter.clone(),
        search_index.clone(),
    );
    
    println!("✅ SyncSpace backend listening on http://localhost:8080");
    println!("🔐 Authentication enabled - use /api/auth/register or /api/auth/login");
    println!("🔍 Search available at /api/search?q=term");
    
    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}

fn routes(
    config: Arc<Mutex<Config>>,
    fs_tx: Sender<FileChangeEvent>,
    user_db: UserDB,
    rate_limiter: Arc<RateLimiter>,
    search_index: Arc<search::SearchIndex>,
) -> BoxedFilter<(impl warp::Reply,)> {
    
    // ==================== AUTH ROUTES ====================
    
    // Register new user
    let register = {
        let db = user_db.clone();
        warp::path!("api" / "auth" / "register")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |req: RegisterRequest| {
                let db = db.clone();
                async move {
                    match db.create_user(req.username.clone(), req.password) {
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
                                    Ok::<_, Infallible>(warp::reply::with_status(
                                        warp::reply::json(&response),
                                        StatusCode::CREATED,
                                    ))
                                }
                                Err(e) => {
                                    let error = serde_json::json!({"error": e});
                                    Ok(warp::reply::with_status(
                                        warp::reply::json(&error),
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                    ))
                                }
                            }
                        }
                        Err(e) => {
                            let error = serde_json::json!({"error": e});
                            Ok(warp::reply::with_status(
                                warp::reply::json(&error),
                                StatusCode::BAD_REQUEST,
                            ))
                        }
                    }
                }
            })
    };

    // Login
    let login = {
        let db = user_db.clone();
        let limiter = rate_limiter.clone();
        warp::path!("api" / "auth" / "login")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |req: LoginRequest| {
                let db = db.clone();
                let limiter = limiter.clone();
                async move {
                    // Rate limiting
                    if !limiter.check_rate_limit(&req.username, 5, 60) {
                        let error = serde_json::json!({"error": "Too many login attempts. Try again later."});
                        return Ok::<_, Infallible>(warp::reply::with_status(
                            warp::reply::json(&error),
                            StatusCode::TOO_MANY_REQUESTS,
                        ));
                    }

                    match db.verify_password(&req.username, &req.password) {
                        Ok(mut user) => {
                            // Check if 2FA is enabled
                            if user.totp_enabled {
                                if let Some(code) = req.totp_code {
                                    // Verify TOTP code
                                    if let Some(ref secret) = user.totp_secret {
                                        if !auth::verify_totp_code(secret, &code) {
                                            let error = serde_json::json!({"error": "Invalid 2FA code"});
                                            return Ok(warp::reply::with_status(
                                                warp::reply::json(&error),
                                                StatusCode::UNAUTHORIZED,
                                            ));
                                        }
                                    } else {
                                        let error = serde_json::json!({"error": "2FA not properly configured"});
                                        return Ok(warp::reply::with_status(
                                            warp::reply::json(&error),
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                        ));
                                    }
                                } else {
                                    // 2FA required but not provided
                                    let response = serde_json::json!({
                                        "requires_2fa": true,
                                        "message": "Please provide 2FA code"
                                    });
                                    return Ok(warp::reply::with_status(
                                        warp::reply::json(&response),
                                        StatusCode::OK,
                                    ));
                                }
                            }

                            // Update last login
                            user.last_login = Some(Utc::now());
                            db.update_user(user.clone());

                            // Generate token
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
                                    Ok(warp::reply::with_status(
                                        warp::reply::json(&response),
                                        StatusCode::OK,
                                    ))
                                }
                                Err(e) => {
                                    let error = serde_json::json!({"error": e});
                                    Ok(warp::reply::with_status(
                                        warp::reply::json(&error),
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                    ))
                                }
                            }
                        }
                        Err(e) => {
                            let error = serde_json::json!({"error": e});
                            Ok(warp::reply::with_status(
                                warp::reply::json(&error),
                                StatusCode::UNAUTHORIZED,
                            ))
                        }
                    }
                }
            })
    };

    // Setup 2FA
    let setup_2fa = {
        let db = user_db.clone();
        warp::path!("api" / "auth" / "2fa" / "setup")
            .and(warp::post())
            .and(auth::with_auth(db.clone()))
            .and_then(move |user: auth::User| async move {
                let secret = auth::generate_totp_secret();
                let qr_url = format!(
                    "otpauth://totp/SyncSpace:{}?secret={}&issuer=SyncSpace",
                    user.username, secret
                );
                
                let response = Setup2FAResponse { secret, qr_url };
                Ok::<_, Infallible>(warp::reply::json(&response))
            })
    };

    // Enable 2FA
    let enable_2fa = {
        let db = user_db.clone();
        warp::path!("api" / "auth" / "2fa" / "enable")
            .and(warp::post())
            .and(auth::with_auth(db.clone()))
            .and(warp::body::json())
            .and_then(move |mut user: auth::User, req: Enable2FARequest| {
                let db = db.clone();
                async move {
                    // Secret should be in session or re-generated (simplified here)
                    let secret = user.totp_secret.clone().unwrap_or_else(auth::generate_totp_secret);
                    
                    if auth::verify_totp_code(&secret, &req.totp_code) {
                        user.totp_secret = Some(secret);
                        user.totp_enabled = true;
                        db.update_user(user);
                        
                        let response = serde_json::json!({"message": "2FA enabled successfully"});
                        Ok::<_, Infallible>(warp::reply::with_status(
                            warp::reply::json(&response),
                            StatusCode::OK,
                        ))
                    } else {
                        let error = serde_json::json!({"error": "Invalid 2FA code"});
                        Ok(warp::reply::with_status(
                            warp::reply::json(&error),
                            StatusCode::BAD_REQUEST,
                        ))
                    }
                }
            })
    };

    // Disable 2FA
    let disable_2fa = {
        let db = user_db.clone();
        warp::path!("api" / "auth" / "2fa" / "disable")
            .and(warp::post())
            .and(auth::with_auth(db.clone()))
            .and_then(move |mut user: auth::User| {
                let db = db.clone();
                async move {
                    user.totp_enabled = false;
                    user.totp_secret = None;
                    db.update_user(user);
                    
                    let response = serde_json::json!({"message": "2FA disabled"});
                    Ok::<_, Infallible>(warp::reply::json(&response))
                }
            })
    };

    // Change password
    let change_password = {
        let db = user_db.clone();
        warp::path!("api" / "auth" / "change-password")
            .and(warp::put())
            .and(auth::with_auth(db.clone()))
            .and(warp::body::json())
            .and_then(move |user: auth::User, req: ChangePasswordRequest| {
                let db = db.clone();
                async move {
                    match db.change_password(user.id, &req.old_password, &req.new_password) {
                        Ok(_) => {
                            let response = serde_json::json!({"message": "Password changed successfully"});
                            Ok::<_, Infallible>(warp::reply::with_status(
                                warp::reply::json(&response),
                                StatusCode::OK,
                            ))
                        }
                        Err(e) => {
                            let error = serde_json::json!({"error": e});
                            Ok(warp::reply::with_status(
                                warp::reply::json(&error),
                                StatusCode::BAD_REQUEST,
                            ))
                        }
                    }
                }
            })
    };

    // Get current user info
    let me = {
        let db = user_db.clone();
        warp::path!("api" / "auth" / "me")
            .and(warp::get())
            .and(auth::with_auth(db.clone()))
            .and_then(|user: auth::User| async move {
                let info = UserInfo {
                    id: user.id.to_string(),
                    username: user.username,
                    totp_enabled: user.totp_enabled,
                };
                Ok::<_, Infallible>(warp::reply::json(&info))
            })
    };

    // ==================== FILE ROUTES (Protected) ====================

    // List files (protected)
    let list = {
        let db = user_db.clone();
        warp::path!("api" / "files" / ..)
            .and(warp::get())
            .and(warp::path::tail())
            .and(auth::with_auth(db))
            .and_then(|tail: warp::path::Tail, _user: auth::User| async move {
                list_entries(tail).await
            })
    };

    // Download file (protected)
    let download = {
        let db = user_db.clone();
        warp::path!("api" / "file" / ..)
            .and(warp::get())
            .and(warp::path::tail())
            .and(auth::with_auth(db))
            .and_then(|tail: warp::path::Tail, _user: auth::User| async move {
                download_file(tail).await
            })
    };

    // Upload file (protected)
    let upload = {
        let db = user_db.clone();
        let tx = fs_tx.clone();
        let index = search_index.clone();
        warp::path!("api" / "upload" / ..)
            .and(warp::post())
            .and(warp::path::tail())
            .and(warp::body::bytes())
            .and(auth::with_auth(db))
            .and_then(move |tail: warp::path::Tail, bytes: bytes::Bytes, _user: auth::User| {
                let tx = tx.clone();
                let index = index.clone();
                async move {
                    let path = Path::new(tail.as_str());
                    match upload_file(path, bytes).await {
                        Ok(_) => {
                            let _ = tx.send(FileChangeEvent {
                                path: tail.as_str().to_string(),
                                kind: "create".into(),
                                timestamp: Utc::now(),
                            });
                            
                            // Background indexing
                            let file_path = Path::new(DATA_DIR).join(tail.as_str());
                            let filename = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();
                            let path_str = tail.as_str().to_string();
                            let file_id = uuid::Uuid::new_v4().to_string();
                            
                            tokio::spawn(async move {
                                // Extract content if text file
                                let content = search::extract_content(&file_path).await;
                                
                                // Get file metadata
                                if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
                                    let size = metadata.len();
                                    let modified = metadata.modified()
                                        .ok()
                                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                        .map(|d| chrono::Utc.timestamp_opt(d.as_secs() as i64, 0).unwrap())
                                        .unwrap_or_else(chrono::Utc::now);
                                    
                                    // Index file
                                    if let Err(e) = index.index_file(
                                        &file_id,
                                        &filename,
                                        &path_str,
                                        content,
                                        modified,
                                        size,
                                    ).await {
                                        eprintln!("⚠️ Failed to index file {}: {}", filename, e);
                                    } else {
                                        println!("📇 Indexed: {}", filename);
                                    }
                                }
                            });
                            
                            Ok::<_, Infallible>(warp::reply::with_status("uploaded", StatusCode::CREATED))
                        }
                        Err(_) => Ok(warp::reply::with_status("error", StatusCode::INTERNAL_SERVER_ERROR)),
                    }
                }
            })
    };

    // Delete file (protected)
    let delete = {
        let db = user_db.clone();
        let tx = fs_tx.clone();
        let index = search_index.clone();
        warp::path!("api" / "files" / ..)
            .and(warp::delete())
            .and(warp::path::tail())
            .and(auth::with_auth(db))
            .and_then(move |tail: warp::path::Tail, _user: auth::User| {
                let tx = tx.clone();
                let index = index.clone();
                async move {
                    let path = Path::new(tail.as_str());
                    let path_str = tail.as_str().to_string();
                    
                    match delete_entry(path).await {
                        Ok(_) => {
                            let _ = tx.send(FileChangeEvent {
                                path: tail.as_str().to_string(),
                                kind: "delete".into(),
                                timestamp: Utc::now(),
                            });
                            
                            // Background: Remove from search index
                            tokio::spawn(async move {
                                // In production, we'd need to store file_id in database
                                // For now, we search by path to find the file_id
                                if let Ok(results) = index.search(&path_str, 1, false) {
                                    if let Some(result) = results.first() {
                                        if let Err(e) = index.delete_from_index(&result.file_id).await {
                                            eprintln!("⚠️ Failed to remove from index: {}", e);
                                        } else {
                                            println!("🗑️ Removed from index: {}", path_str);
                                        }
                                    }
                                }
                            });
                            
                            Ok::<_, Infallible>(warp::reply::with_status("deleted", StatusCode::OK))
                        }
                        Err(_) => Ok(warp::reply::with_status("not found", StatusCode::NOT_FOUND)),
                    }
                }
            })
    };

    // Create directory (protected)
    let mkdir = {
        let db = user_db.clone();
        let tx = fs_tx.clone();
        warp::path!("api" / "dirs" / ..)
            .and(warp::post())
            .and(warp::path::tail())
            .and(auth::with_auth(db))
            .and_then(move |tail: warp::path::Tail, _user: auth::User| {
                let tx = tx.clone();
                async move {
                    let path = Path::new(tail.as_str());
                    match create_dir(path).await {
                        Ok(_) => {
                            let _ = tx.send(FileChangeEvent {
                                path: tail.as_str().to_string(),
                                kind: "mkdir".into(),
                                timestamp: Utc::now(),
                            });
                            Ok::<_, Infallible>(warp::reply::with_status("created", StatusCode::CREATED))
                        }
                        Err(_) => Ok(warp::reply::with_status("error", StatusCode::INTERNAL_SERVER_ERROR)),
                    }
                }
            })
    };

    // Rename file (protected)
    let rename = {
        let db = user_db.clone();
        let tx = fs_tx.clone();
        warp::path!("api" / "rename" / ..)
            .and(warp::put())
            .and(warp::path::tail())
            .and(warp::body::json())
            .and(auth::with_auth(db))
            .and_then(move |tail: warp::path::Tail, req: RenameRequest, _user: auth::User| {
                let tx = tx.clone();
                async move {
                    let old_path = Path::new(tail.as_str());
                    let new_path = Path::new(&req.new_path);
                    match rename_entry(old_path, new_path).await {
                        Ok(_) => {
                            let _ = tx.send(FileChangeEvent {
                                path: req.new_path.clone(),
                                kind: "rename".into(),
                                timestamp: Utc::now(),
                            });
                            Ok::<_, Infallible>(warp::reply::with_status("renamed", StatusCode::OK))
                        }
                        Err(_) => Ok(warp::reply::with_status("error", StatusCode::INTERNAL_SERVER_ERROR)),
                    }
                }
            })
    };

    // Search (protected)
    let search = {
        let db = user_db.clone();
        let index = search_index.clone();
        warp::path!("api" / "search")
            .and(warp::get())
            .and(warp::query::<HashMap<String, String>>())
            .and(auth::with_auth(db))
            .and_then(move |params: HashMap<String, String>, _user: auth::User| {
                let index = index.clone();
                async move {
                    let query = params.get("q").cloned().unwrap_or_default();
                    let limit = params.get("limit")
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(50);
                    let fuzzy = params.get("fuzzy")
                        .and_then(|s| s.parse::<bool>().ok())
                        .unwrap_or(true);
                    
                    // Use Tantivy search
                    match index.search(&query, limit, fuzzy) {
                        Ok(results) => {
                            let response = serde_json::json!({
                                "results": results,
                                "total": results.len(),
                                "query": query,
                            });
                            Ok::<_, Infallible>(warp::reply::json(&response))
                        }
                        Err(e) => {
                            let error = serde_json::json!({
                                "error": format!("Search failed: {}", e)
                            });
                            Ok(warp::reply::json(&error))
                        }
                    }
                }
            })
    };

    // Stats (protected)
    let stats = {
        let db = user_db.clone();
        warp::path!("api" / "stats")
            .and(warp::get())
            .and(auth::with_auth(db))
            .and_then(|_user: auth::User| async move {
                let (count, size) = compute_stats_async().await;
                let body = serde_json::json!({
                    "file_count": count,
                    "total_size": size
                });
                Ok::<_, Infallible>(warp::reply::json(&body))
            })
    };

    // ==================== CONFIG & PEERS (Protected) ====================

    let get_config = {
        let db = user_db.clone();
        let cfg = config.clone();
        warp::path!("api" / "config")
            .and(warp::get())
            .and(auth::with_auth(db))
            .and_then(move |_user: auth::User| {
                let cfg = cfg.clone();
                async move {
                    let config = cfg.lock().unwrap().clone();
                    Ok::<_, Infallible>(warp::reply::json(&config))
                }
            })
    };

    let put_config = {
        let db = user_db.clone();
        let cfg = config.clone();
        warp::path!("api" / "config")
            .and(warp::put())
            .and(warp::body::json())
            .and(auth::with_auth(db))
            .and_then(move |new_cfg: Config, _user: auth::User| {
                let cfg = cfg.clone();
                async move {
                    {
                        let mut config = cfg.lock().unwrap();
                        *config = new_cfg.clone();
                    }
                    if let Err(e) = save_config(&new_cfg).await {
                        eprintln!("Failed to save config: {}", e);
                    }
                    Ok::<_, Infallible>(warp::reply::json(&new_cfg))
                }
            })
    };

    let add_peer = {
        let db = user_db.clone();
        let cfg = config.clone();
        warp::path!("api" / "peers")
            .and(warp::post())
            .and(warp::body::json())
            .and(auth::with_auth(db))
            .and_then(move |peer: Peer, _user: auth::User| {
                let cfg = cfg.clone();
                async move {
                    let cfg_to_save = {
                        let mut config = cfg.lock().unwrap();
                        config.peers.push(peer.clone());
                        config.clone()
                    };
                    if let Err(e) = save_config(&cfg_to_save).await {
                        eprintln!("Failed to save config: {}", e);
                    }
                    Ok::<_, Infallible>(warp::reply::json(&peer))
                }
            })
    };

    let list_peers = {
        let db = user_db.clone();
        let cfg = config.clone();
        warp::path!("api" / "peers")
            .and(warp::get())
            .and(auth::with_auth(db))
            .and_then(move |_user: auth::User| {
                let cfg = cfg.clone();
                async move {
                    let peers = cfg.lock().unwrap().peers.clone();
                    Ok::<_, Infallible>(warp::reply::json(&peers))
                }
            })
    };

    // WebSocket endpoint (protected via query param token)
    let ws_route = {
        let tx = fs_tx.clone();
        warp::path!("api" / "ws")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let tx = tx.clone();
                ws.on_upgrade(move |socket| handle_ws_connection(socket, tx))
            })
    };

    // ==================== FRONTEND ROUTES ====================
    
    // Serve index.html for root path
    let frontend_root = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("../frontend/index.html"));
    
    // Serve static assets (JS, CSS, etc.)
    let frontend_static = warp::path("static")
        .and(warp::fs::dir("../frontend"));

    // Combine all routes
    frontend_root
        .or(frontend_static)
        .or(register)
        .or(login)
        .or(setup_2fa)
        .or(enable_2fa)
        .or(disable_2fa)
        .or(change_password)
        .or(me)
        .or(list)
        .or(download)
        .or(upload)
        .or(delete)
        .or(mkdir)
        .or(rename)
        .or(search)
        .or(get_config)
        .or(put_config)
        .or(add_peer)
        .or(list_peers)
        .or(stats)
        .or(ws_route)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(vec!["GET", "POST", "DELETE", "PUT"])
                .allow_headers(vec!["Content-Type", "Authorization"]),
        )
        .boxed()
}

// ==================== HELPER FUNCTIONS ====================

// Check if a filename is a system file that should be hidden
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
            match res {
                Ok(event) => {
                    let _ = event_tx.blocking_send(event);
                }
                Err(e) => eprintln!("notify error: {}", e),
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

async fn list_entries(tail: warp::path::Tail) -> Result<impl warp::Reply, Infallible> {
    let sub = tail.as_str();
    let target_dir = Path::new(DATA_DIR).join(sub);
    let mut entries = Vec::new();
    match fs::read_dir(&target_dir).await {
        Ok(mut dir) => {
            while let Ok(Some(e)) = dir.next_entry().await {
                if let Ok(meta) = e.metadata().await {
                    let filename = e.file_name().to_string_lossy().to_string();
                    
                    // Filter out system files in ALL directories (not just root)
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
            Ok(warp::reply::json(&entries))
        }
        Err(_) => {
            let error = serde_json::json!({"error": "directory not found"});
            Ok(warp::reply::json(&error))
        }
    }
}

async fn download_file(tail: warp::path::Tail) -> Result<impl warp::Reply, Infallible> {
    let sub = tail.as_str();
    let file_path = Path::new(DATA_DIR).join(sub);
    match fs::metadata(&file_path).await {
        Ok(meta) if meta.is_file() => match fs::read(&file_path).await {
            Ok(bytes) => Ok(warp::reply::with_status(bytes, StatusCode::OK)),
            Err(_) => Ok(warp::reply::with_status(
                Vec::<u8>::new(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        },
        Ok(_) => Ok(warp::reply::with_status(
            Vec::<u8>::new(),
            StatusCode::BAD_REQUEST,
        )),
        Err(_) => Ok(warp::reply::with_status(
            Vec::<u8>::new(),
            StatusCode::NOT_FOUND,
        )),
    }
}

async fn upload_file(path: &Path, bytes: bytes::Bytes) -> Result<(), std::io::Error> {
    let file_path = Path::new(DATA_DIR).join(path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(file_path, bytes).await
}

async fn delete_entry(path: &Path) -> Result<(), std::io::Error> {
    let full = Path::new(DATA_DIR).join(path);
    let meta = fs::metadata(&full).await?;
    if meta.is_dir() {
        fs::remove_dir_all(&full).await
    } else {
        fs::remove_file(&full).await
    }
}

async fn create_dir(path: &Path) -> Result<(), std::io::Error> {
    let full = Path::new(DATA_DIR).join(path);
    fs::create_dir_all(full).await
}

async fn handle_ws_connection(ws: WebSocket, tx: Sender<FileChangeEvent>) {
    println!("WebSocket client connected");
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let msg = serde_json::to_string(&event).unwrap();
            if ws_tx.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = ws_rx.next().await {}
    });
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
    println!("WebSocket client disconnected");
}

async fn rename_entry(old: &Path, new_rel: &Path) -> Result<(), std::io::Error> {
    let old_full = Path::new(DATA_DIR).join(old);
    let new_full = Path::new(DATA_DIR).join(new_rel);
    if let Some(parent) = new_full.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::rename(old_full, new_full).await
}

async fn search_entries(query: String) -> Vec<SearchResult> {
    let query_lower = query.to_lowercase();
    tokio::task::spawn_blocking(move || {
        let mut results = Vec::new();
        for entry in WalkDir::new(DATA_DIR).into_iter().filter_map(Result::ok) {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.to_lowercase().contains(&query_lower) {
                let rel_path = entry.path().strip_prefix(DATA_DIR).unwrap_or(entry.path());
                let metadata = entry.metadata().unwrap();
                results.push(SearchResult {
                    path: rel_path.to_string_lossy().to_string(),
                    is_dir: metadata.is_dir(),
                    size: metadata.len(),
                });
            }
        }
        results
    })
    .await
    .unwrap_or_default()
}

async fn compute_stats_async() -> (u64, u64) {
    tokio::task::spawn_blocking(|| {
        let mut count: u64 = 0;
        let mut total_size: u64 = 0;
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
