//! SyncSpace backend server.
//!
//! This backend implements a set of HTTP and WebSocket APIs to manage a
//! synchronisation folder. It exposes endpoints to list files, download and
//! upload them, delete entries, create subdirectories, manage peers and
//! configuration, and broadcast file system change events to connected
//! clients via WebSockets. The implementation uses [`warp`] as the web
//! framework and [`notify`] to watch the file system.

use std::collections::HashMap;
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use notify::{Error as NotifyError, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::broadcast::{self, Sender};
use uuid::Uuid;
use warp::filters::BoxedFilter;
use futures_util::StreamExt;
use warp::http::StatusCode;
use warp::ws::{Message, WebSocket};
use warp::Filter;

/// Directory where all synchronised files are stored. This can be extended
/// later to support multiple directories based on configuration.
const DATA_DIR: &str = "./data";
/// Path to the configuration file.
const CONFIG_FILE: &str = "./config.json";

/// Information returned for each entry in the folder listing.
#[derive(Serialize)]
struct EntryInfo {
    name: String,
    is_dir: bool,
    size: u64,
}

/// Configuration structure persisted in [`CONFIG_FILE`].
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    /// Paths (relative to working directory) that should be synchronised.
    sync_dirs: Vec<String>,
    /// Known remote peers.
    peers: Vec<Peer>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sync_dirs: vec![DATA_DIR.to_string()],
            peers: Vec::new(),
        }
    }
}

/// Representation of a remote peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Peer {
    id: Uuid,
    address: String,
    last_seen: Option<DateTime<Utc>>,
}

/// Event describing a file system change. These events are broadcast to all
/// connected WebSocket clients.
#[derive(Debug, Clone, Serialize)]
struct FileChangeEvent {
    path: String,
    kind: String,
    timestamp: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    // Ensure data directory exists
    if let Err(e) = fs::create_dir_all(DATA_DIR).await {
        eprintln!("Failed to create data directory {}: {}", DATA_DIR, e);
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
    let api = routes(config.clone(), tx.clone());
    println!("SyncSpace backend listening on http://localhost:8080");
    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}

/// Construct all API routes as a single boxed filter.
fn routes(config: Arc<Mutex<Config>>, fs_tx: Sender<FileChangeEvent>) -> BoxedFilter<(impl warp::Reply,)> {
    // List entries in data directory or a subdirectory
    let list = warp::path("api")
        .and(warp::path("files"))
        .and(warp::path::tail())
        .and(warp::get())
        .and_then(list_entries);

    // Download file
    let download = warp::path("api")
        .and(warp::path("file"))
        .and(warp::path::tail())
        .and(warp::get())
        .and_then(download_file);

    // Upload file
    let upload = warp::path("api")
        .and(warp::path("upload"))
        .and(warp::path::tail())
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(move |tail: warp::path::Tail, bytes: bytes::Bytes| {
            let tx = fs_tx.clone();
            async move {
                let path = Path::new(tail.as_str());
                match upload_file(path, bytes).await {
                    Ok(_) => {
                        // Broadcast event
                        let _ = tx.send(FileChangeEvent {
                            path: tail.as_str().to_string(),
                            kind: "create".into(),
                            timestamp: Utc::now(),
                        });
                        Ok(warp::reply::with_status("uploaded", StatusCode::CREATED))
                    }
                    Err(_) => Ok(warp::reply::with_status("error", StatusCode::INTERNAL_SERVER_ERROR)),
                }
            }
        });

    // Delete file or directory
    let delete = warp::path("api")
        .and(warp::path("files"))
        .and(warp::path::tail())
        .and(warp::delete())
        .and_then(move |tail: warp::path::Tail| {
            let tx = fs_tx.clone();
            async move {
                let path = Path::new(tail.as_str());
                match delete_entry(path).await {
                    Ok(_) => {
                        let _ = tx.send(FileChangeEvent {
                            path: tail.as_str().to_string(),
                            kind: "delete".into(),
                            timestamp: Utc::now(),
                        });
                        Ok(warp::reply::with_status("deleted", StatusCode::OK))
                    }
                    Err(_) => Ok(warp::reply::with_status("not found", StatusCode::NOT_FOUND)),
                }
            }
        });

    // Create directory
    let mkdir = warp::path("api")
        .and(warp::path("dirs"))
        .and(warp::path::tail())
        .and(warp::post())
        .and_then(move |tail: warp::path::Tail| {
            let tx = fs_tx.clone();
            async move {
                let path = Path::new(tail.as_str());
                match create_dir(path).await {
                    Ok(_) => {
                        let _ = tx.send(FileChangeEvent {
                            path: tail.as_str().to_string(),
                            kind: "mkdir".into(),
                            timestamp: Utc::now(),
                        });
                        Ok(warp::reply::with_status("created", StatusCode::CREATED))
                    }
                    Err(_) => Ok(warp::reply::with_status("error", StatusCode::INTERNAL_SERVER_ERROR)),
                }
            }
        });

    // Configuration: get
    let get_config = warp::path("api")
        .and(warp::path("config"))
        .and(warp::get())
        .and_then(move || {
            let config = config.clone();
            async move {
                let cfg = config.lock().unwrap().clone();
                Ok::<_, Infallible>(warp::reply::json(&cfg))
            }
        });
    // Configuration: update
    let put_config = warp::path("api")
        .and(warp::path("config"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(move |new_cfg: Config| {
            let config = config.clone();
            async move {
                {
                    let mut cfg = config.lock().unwrap();
                    *cfg = new_cfg.clone();
                }
                // Persist configuration
                if let Err(e) = save_config(&new_cfg).await {
                    eprintln!("Failed to save config: {}", e);
                }
                Ok::<_, Infallible>(warp::reply::json(&new_cfg))
            }
        });

    // Peer registration (add a new peer)
    let add_peer = warp::path("api")
        .and(warp::path("peers"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |peer: Peer| {
            let config = config.clone();
            async move {
                {
                    let mut cfg = config.lock().unwrap();
                    cfg.peers.push(peer.clone());
                }
                if let Err(e) = save_config(&config.lock().unwrap()).await {
                    eprintln!("Failed to save config: {}", e);
                }
                Ok::<_, Infallible>(warp::reply::json(&peer))
            }
        });

    // List peers
    let list_peers = warp::path("api")
        .and(warp::path("peers"))
        .and(warp::get())
        .and_then(move || {
            let config = config.clone();
            async move {
                let peers = config.lock().unwrap().peers.clone();
                Ok::<_, Infallible>(warp::reply::json(&peers))
            }
        });

    // WebSocket endpoint for file events
    let ws_route = warp::path("api")
        .and(warp::path("ws"))
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = fs_tx.clone();
            ws.on_upgrade(move |socket| handle_ws_connection(socket, tx))
        });

    // Combine all routes and enable CORS
    list
        .or(download)
        .or(upload)
        .or(delete)
        .or(mkdir)
        .or(get_config)
        .or(put_config)
        .or(add_peer)
        .or(list_peers)
        .or(ws_route)
        .with(warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allow_headers(vec!["Content-Type"]))
        .boxed()
}

/// Read configuration from disk.
async fn load_config() -> Option<Config> {
    match fs::read(CONFIG_FILE).await {
        Ok(bytes) => serde_json::from_slice(&bytes).ok(),
        Err(_) => None,
    }
}

/// Persist configuration to disk.
async fn save_config(config: &Config) -> Result<(), std::io::Error> {
    let json = serde_json::to_vec_pretty(config).unwrap();
    fs::write(CONFIG_FILE, json).await
}

/// Watch the data directory for changes and send events on the provided channel.
async fn watch_data_dir(tx: Sender<FileChangeEvent>) -> Result<(), NotifyError> {
    // Channel to forward events from the notify callback into the async world
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<Event>(16);
    // Create the blocking watcher
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
        match res {
            Ok(event) => {
                // Ignore if the send fails
                let _ = event_tx.blocking_send(event);
            }
            Err(e) => eprintln!("notify error: {}", e),
        }
    })?;
    watcher.watch(Path::new(DATA_DIR), RecursiveMode::Recursive)?;
    // Process events and broadcast
    while let Some(event) = event_rx.recv().await {
        // Use the first path for simplicity
        if let Some(path) = event.paths.first() {
            let kind = format!("{:?}", event.kind);
            let relative = path.strip_prefix(DATA_DIR).unwrap_or(path).to_string_lossy().to_string();
            let _ = tx.send(FileChangeEvent {
                path: relative,
                kind,
                timestamp: Utc::now(),
            });
        }
    }
    Ok(())
}

/// List entries within the given subpath of the data directory. An empty tail
/// refers to the root of [`DATA_DIR`].
async fn list_entries(tail: warp::path::Tail) -> Result<impl warp::Reply, Infallible> {
    let sub = tail.as_str();
    let target_dir = Path::new(DATA_DIR).join(sub);
    let mut entries = Vec::new();
    match fs::read_dir(&target_dir).await {
        Ok(mut dir) => {
            while let Ok(Some(e)) = dir.next_entry().await {
                if let Ok(meta) = e.metadata().await {
                    entries.push(EntryInfo {
                        name: e.file_name().to_string_lossy().to_string(),
                        is_dir: meta.is_dir(),
                        size: meta.len(),
                    });
                }
            }
            Ok(warp::reply::json(&entries))
        }
        Err(_) => Ok(warp::reply::with_status(
            "directory not found",
            StatusCode::NOT_FOUND,
        )),
    }
}

/// Download a file and return its contents. If the path refers to a
/// directory, a 400 status is returned.
async fn download_file(tail: warp::path::Tail) -> Result<impl warp::Reply, Infallible> {
    let sub = tail.as_str();
    let file_path = Path::new(DATA_DIR).join(sub);
    match fs::metadata(&file_path).await {
        Ok(meta) if meta.is_file() => match fs::read(&file_path).await {
            Ok(bytes) => Ok(warp::reply::with_status(bytes, StatusCode::OK)),
            Err(_) => Ok(warp::reply::with_status(
                "failed to read file", StatusCode::INTERNAL_SERVER_ERROR,
            )),
        },
        Ok(_) => Ok(warp::reply::with_status(
            "not a file", StatusCode::BAD_REQUEST,
        )),
        Err(_) => Ok(warp::reply::with_status(
            "file not found", StatusCode::NOT_FOUND,
        )),
    }
}

/// Write the provided bytes to a file within the data directory. Any missing
/// parent directories are created automatically.
async fn upload_file(path: &Path, bytes: bytes::Bytes) -> Result<(), std::io::Error> {
    let file_path = Path::new(DATA_DIR).join(path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(file_path, bytes).await
}

/// Delete the specified file or directory. Directories are removed
/// recursively. The function returns an error if the entry does not exist.
async fn delete_entry(path: &Path) -> Result<(), std::io::Error> {
    let full = Path::new(DATA_DIR).join(path);
    let meta = fs::metadata(&full).await?;
    if meta.is_dir() {
        fs::remove_dir_all(&full).await
    } else {
        fs::remove_file(&full).await
    }
}

/// Create a new directory (and parents) under the data directory.
async fn create_dir(path: &Path) -> Result<(), std::io::Error> {
    let full = Path::new(DATA_DIR).join(path);
    fs::create_dir_all(full).await
}

/// Handle a WebSocket connection. Subscribes to the broadcast channel and
/// forwards incoming file change events to the client. Ignores any
/// messages received from the client.
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
    // Drain incoming messages and ignore them
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = ws_rx.next().await {
            // ignore any incoming messages from client
        }
    });
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
    println!("WebSocket client disconnected");
}