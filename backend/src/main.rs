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
mod middleware;
mod models;
mod search;
mod services;
mod websocket;

use std::net::SocketAddr;
use std::sync::Arc;

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

    // Initialize database
    let db_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect("sqlite:./data/syncspace.db?mode=rwc")
        .await
        .expect("Failed to connect to database");

    // Run migrations
    // TODO: Fix duplicate migration version numbers (003, 012) before enabling
    // sqlx::migrate!("./migrations")
    //     .run(&db_pool)
    //     .await
    //     .expect("Failed to run migrations");

    println!("✅ Database connection established (migrations skipped)");

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

    // Build application state
    let app_state = AppState {
        config,
        fs_tx,
        user_db,
        rate_limiter,
        search_index,
        db_pool,
        cache_manager,
        job_processor,
        performance_monitor,
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
        // WebSocket endpoint
        .route("/api/ws", get(websocket::ws_handler))
        
        // API routes (delegated to api module)
        .nest("/api", api::build_api_router(state.clone()))
        
        // Apply middleware
        .layer(cors)
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100 MB
        
        .with_state(state)
}
