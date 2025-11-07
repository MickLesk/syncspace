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
mod jobs;
mod middleware;
mod models;
mod search;
mod security;
mod services;
mod status;
mod websocket;
mod workers;

use std::net::SocketAddr;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    middleware as axum_middleware,
    routing::get,
    Router,
};
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::{Any, CorsLayer};
use sqlx::Row;

use auth::RateLimiter; // UserDB removed - using SQLite directly
use search::SearchIndex;
use websocket::FileChangeEvent;

// Performance and caching imports
mod performance;
use performance::{CacheManager, JobProcessor, PerformanceMonitor};

const DATA_DIR: &str = "./data";

// ==================== LOGGING SETUP ====================

/// Initialize structured logging with environment-based configuration
/// 
/// Set RUST_LOG environment variable to control log levels:
/// - RUST_LOG=debug    - Show all debug messages
/// - RUST_LOG=info     - Show info and above (default)
/// - RUST_LOG=warn     - Show warnings and errors only
/// - RUST_LOG=error    - Show errors only
/// - RUST_LOG=syncbackend=debug,tower_http=debug - Module-specific levels
fn init_tracing() {
    use tracing_subscriber::{
        fmt,
        layer::SubscriberExt,
        util::SubscriberInitExt,
        EnvFilter,
    };

    // Default log level: INFO for our app, WARN for dependencies
    let default_filter = "syncbackend=info,tower_http=info,axum=info,sqlx=warn";
    
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default_filter))
        .expect("Failed to create env filter");

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_level(true)
                .compact() // Compact format for better readability
        )
        .init();
}

// ==================== SHARED STATE ====================

#[derive(Clone)]
pub struct AppState {
    config: Arc<Mutex<Config>>,
    fs_tx: broadcast::Sender<FileChangeEvent>,
    // user_db removed - now using SQLite db_pool directly
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
    // Initialize structured logging with tracing
    init_tracing();

    tracing::info!("🚀 SyncSpace Backend Starting...");
    tracing::info!("📊 Build: {} @ {}", 
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        option_env!("BUILD_TIME").unwrap_or("unknown")
    );

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

    // Initialize rate limiter
    let rate_limiter = Arc::new(RateLimiter::new());

    // Check if admin user exists in SQLite database
    let admin_user_id_string = match auth::get_user_by_username(&db_pool, "admin").await {
        Ok(Some(user)) => {
            println!("✅ Admin user already exists");
            user.id
        }
        Ok(None) => {
            println!("⚠️  No admin user found in database");
            println!("✅ Setup completed - skipping default admin creation");
            // Use first user or a placeholder
            sqlx::query_scalar::<_, String>("SELECT id FROM users LIMIT 1")
                .fetch_optional(&db_pool)
                .await
                .unwrap_or(None)
                .unwrap_or_else(|| "00000000-0000-0000-0000-000000000000".to_string())
        }
        Err(e) => {
            eprintln!("❌ Failed to check for admin user: {}", e);
            "00000000-0000-0000-0000-000000000000".to_string()
        }
    };

    // Sync filesystem to database with admin user ownership
    match services::sync_service::sync_filesystem_to_db(&db_pool, &admin_user_id_string).await {
        Ok(count) => {
            if count > 0 {
                println!("✅ Synced {} existing files to database", count);
            }
        },
        Err(e) => eprintln!("⚠️  Filesystem sync failed: {}", e),
    }

    // Initialize search index
    let search_index = Arc::new(
        SearchIndex::new()
            .expect("Failed to initialize search index"),
    );

    // Background reindex of all files on startup
    {
        let index = search_index.clone();
        let pool = db_pool.clone();
        tokio::spawn(async move {
            println!("🔍 Starting background search index rebuild...");
            
            match sqlx::query("SELECT id, name, path, size_bytes, updated_at FROM files WHERE is_deleted = 0")
                .fetch_all(&pool)
                .await 
            {
                Ok(files) => {
                    let mut count = 0;
                    for row in files {
                        if let (Ok(file_id), Ok(filename), Ok(path), Ok(size_bytes), Ok(updated_at)) = (
                            row.try_get::<String, _>("id"),
                            row.try_get::<String, _>("name"),
                            row.try_get::<String, _>("path"),
                            row.try_get::<i64, _>("size_bytes"),
                            row.try_get::<String, _>("updated_at"),
                        ) {
                            let modified = chrono::DateTime::parse_from_rfc3339(&updated_at)
                                .unwrap_or_else(|_| chrono::Utc::now().into())
                                .with_timezone(&chrono::Utc);
                            
                            let file_path = std::path::Path::new("./data").join(&path);
                            let content = if file_path.exists() {
                                crate::search::extract_content(&file_path).await
                            } else {
                                None
                            };
                            
                            if index.index_file(&file_id, &filename, &path, content, modified, size_bytes as u64).await.is_ok() {
                                count += 1;
                            }
                        }
                    }
                    println!("✅ Background reindex complete: {} files indexed", count);
                }
                Err(e) => eprintln!("❌ Background reindex failed: {:?}", e),
            }
        });
    }

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
        // user_db removed - now using SQLite db_pool directly
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

    // Start background job worker pool
    let worker_pool = workers::WorkerPool::new(pool.clone(), 4);
    let worker_handle = tokio::spawn(async move {
        worker_pool.start().await;
    });

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
    
    // Wait for workers to finish (won't reach here normally)
    let _ = worker_handle.await;
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
        
        // Apply middleware (order matters!)
        .layer(axum_middleware::from_fn(security::security_headers_middleware))
        .layer(cors)
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100 MB
        
        .with_state(state)
}
