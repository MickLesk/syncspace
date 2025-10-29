//! SyncSpace Backend - Refactored Architecture
//! 
//! Clean separation of concerns:
//! - `api/` - HTTP endpoints and routing
//! - `services/` - Business logic layer
//! - `models/` - Data structures and DB entities
//! - `middleware/` - Request/response processing
//! - `websocket/` - Real-time event broadcasting
//! - `db/` - Database operations
//! - `utils/` - Helper functions

mod api;
mod auth;
mod database;
mod handlers;
mod middleware;
mod models;
mod search;
mod services;
mod status;
mod websocket;

use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::{Any, CorsLayer};

use auth::{RateLimiter, UserDB};
use search::SearchIndex;
use websocket::FileChangeEvent;

// Performance and caching imports
mod performance;
use performance::{CacheManager, JobProcessor, PerformanceMonitor};

const DATA_DIR: &str = "./data";

// ==================== SHARED STATE ====================

#[derive(Clone)]
pub struct AppState {
    config: Arc<Mutex<Config>>,
    fs_tx: broadcast::Sender<FileChangeEvent>,
    user_db: UserDB,
    rate_limiter: Arc<RateLimiter>,
    search_index: Arc<SearchIndex>,
    db_pool: sqlx::SqlitePool,
    cache_manager: CacheManager,
    job_processor: JobProcessor,
    performance_monitor: Arc<PerformanceMonitor>,
    // Status page fields
    pub start_time: u64,
    pub active_ws_connections: Arc<AtomicUsize>,
    pub db: sqlx::SqlitePool, // Alias for status page
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub max_file_size: usize,
    pub allowed_origins: Vec<String>,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024, // 100 MB
            allowed_origins: vec!["http://localhost:5173".to_string()],
            database_url: "sqlite:./data/syncspace.db".to_string(),
        }
    }
}

// ==================== MAIN ====================

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 SyncSpace Backend Starting...");

    // Ensure data directory exists
    tokio::fs::create_dir_all(DATA_DIR)
        .await
        .expect("Failed to create data directory");

    // Load or create config
    let config = Arc::new(Mutex::new(Config::default()));

    // Initialize database with migrations
    let db_pool = database::init_db()
        .await
        .expect("Failed to initialize database");

    println!("✅ Database connection established");

    // Initialize auth system
    let user_db = UserDB::new();
    let rate_limiter = Arc::new(RateLimiter::new());

    // Create default admin user if no users exist
    if user_db.get_by_username("admin").is_none() {
        println!("📝 Creating default admin user (username: admin, password: admin)");
        println!("⚠️  WARNING: Change the default password after first login!");
        if let Err(e) = user_db.create_user("admin".to_string(), "admin".to_string()) {
            eprintln!("❌ Failed to create admin user: {}", e);
        } else {
            println!("✅ Default admin user created successfully");
        }
    } else {
        println!("✅ Admin user already exists");
    }

    // Initialize search index
    let search_index = Arc::new(
        SearchIndex::new()
            .expect("Failed to initialize search index"),
    );

    // Initialize performance monitoring
    let cache_config = performance::CacheConfig::default();
    let cache_manager = performance::CacheManager::new(cache_config.clone())
        .await
        .expect("Failed to initialize cache manager");
    let job_processor = performance::JobProcessor::new(cache_manager.clone(), cache_config.background_job_workers);
    let performance_monitor = Arc::new(performance::PerformanceMonitor::new(cache_manager.clone()));

    // Create WebSocket broadcast channel
    let (fs_tx, _) = broadcast::channel::<FileChangeEvent>(100);

    // Get start time for uptime calculation
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Build application state
    let app_state = AppState {
        config,
        fs_tx,
        user_db,
        rate_limiter,
        search_index,
        db_pool: db_pool.clone(),
        cache_manager,
        job_processor,
        performance_monitor,
        start_time,
        active_ws_connections: Arc::new(AtomicUsize::new(0)),
        db: db_pool, // Alias for status page compatibility
    };

    // Build router
    let app = build_router(app_state);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🌐 Listening on http://{}", addr);
    println!("📡 WebSocket available at ws://{}/api/ws", addr);
    println!("✨ Ready to accept connections!");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

// ==================== ROUTER BUILDER ====================

fn build_router(state: AppState) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    Router::new()
        // Root - Status page (direct access on http://localhost:8080)
        .route("/", get(status::get_status_html))
        
        // WebSocket endpoint
        .route("/api/ws", get(websocket::ws_handler))
        
        // Public status endpoints (no auth required)
        .route("/status", get(status::get_status_html))
        .route("/status/json", get(status::get_status_json))
        .route("/api/status", get(status::get_status_json))  // Public API status
        .route("/health", get(status::health_check))
        
        // API routes (delegated to api module)
        .nest("/api", api::build_api_router(state.clone()))
        
        // Apply middleware
        .layer(cors)
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100 MB
        
        .with_state(state)
}
