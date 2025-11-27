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
mod cron;
mod database;
mod database_monitor;
mod db_monitor;
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
    extract::{State as AxumState, WebSocketUpgrade},
    http::Method,
    middleware as axum_middleware,
    response::Response,
    routing::get,
    Router,
};
use sqlx::Row;
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::{Any, CorsLayer};

use auth::RateLimiter;
use database_monitor::DatabaseMonitor;
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
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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
                .compact(), // Compact format for better readability
        )
        .init();
}

// ==================== SHARED STATE ====================

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub db_pool: sqlx::SqlitePool,
    pub fs_tx: broadcast::Sender<FileChangeEvent>,
    pub cache_manager: Arc<CacheManager>,
    pub rate_limiter: Arc<RateLimiter>,
    pub config: Arc<Mutex<Config>>,
    pub job_processor: JobProcessor,
    pub performance_monitor: Arc<PerformanceMonitor>,
    pub db_health_monitor: Arc<DatabaseMonitor>,
    pub search_index: Arc<SearchIndex>,
    pub start_time: u64,
    pub ws_connections: Arc<AtomicUsize>,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("db_pool", &"SqlitePool")
            .field("fs_tx", &"broadcast::Sender<FileChangeEvent>")
            .field("cache_manager", &"Arc<CacheManager>")
            .field("rate_limiter", &"Arc<RateLimiter>")
            .field("config", &"Arc<Mutex<Config>>")
            .field("job_processor", &"JobProcessor")
            .field("performance_monitor", &"Arc<PerformanceMonitor>")
            .field("db_health_monitor", &"Arc<DatabaseMonitor>")
            .field("search_index", &"Arc<SearchIndex>")
            .field("start_time", &self.start_time)
            .field("ws_connections", &"Arc<AtomicUsize>")
            .finish()
    }
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
    tracing::info!(
        "📊 Build: {} @ {}",
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

    // Initialize version storage directory
    services::version_storage_service::init_version_storage()
        .expect("Failed to initialize version storage");

    println!("✅ Version storage initialized");

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
        }
        Err(e) => eprintln!("⚠️  Filesystem sync failed: {}", e),
    }

    // Initialize search index
    let search_index = Arc::new(SearchIndex::new().expect("Failed to initialize search index"));

    // Background reindex of all files on startup
    {
        let index = search_index.clone();
        let pool = db_pool.clone();
        tokio::spawn(async move {
            println!("🔍 Starting background search index rebuild...");

            match sqlx::query(
                "SELECT id, name, path, size_bytes, updated_at FROM files WHERE is_deleted = 0",
            )
            .fetch_all(&pool)
            .await
            {
                Ok(files) => {
                    let mut count = 0;
                    for row in files {
                        if let (
                            Ok(file_id),
                            Ok(filename),
                            Ok(path),
                            Ok(size_bytes),
                            Ok(updated_at),
                        ) = (
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

                            if index
                                .index_file(
                                    &file_id,
                                    &filename,
                                    &path,
                                    content,
                                    modified,
                                    size_bytes as u64,
                                )
                                .await
                                .is_ok()
                            {
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
    let cache_manager_inner = performance::CacheManager::new(cache_config.clone())
        .await
        .expect("Failed to initialize cache manager");
    let cache_manager = Arc::new(cache_manager_inner);
    let job_processor = performance::JobProcessor::new(
        (*cache_manager).clone(),
        cache_config.background_job_workers,
    );
    let performance_monitor = Arc::new(performance::PerformanceMonitor::new(
        (*cache_manager).clone(),
    ));

    // Create WebSocket broadcast channel
    let (fs_tx, _) = broadcast::channel::<FileChangeEvent>(100);

    // Get start time for uptime calculation
    let _start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Initialize database monitor for connection pool monitoring
    let _db_monitor = Arc::new(db_monitor::DatabaseMonitor::new(20, 2));
    tracing::info!("📊 Database monitor initialized");

    // Initialize advanced database health monitor
    let db_health_monitor = Arc::new(database_monitor::DatabaseMonitor::new());
    tracing::info!("🏥 Database health monitor initialized");

    // Start background monitoring task
    let monitor_pool_clone = db_pool.clone();
    let monitor_health_clone = db_health_monitor.clone();
    tokio::spawn(async move {
        database_monitor::monitor_background(monitor_pool_clone, monitor_health_clone).await;
    });

    // Start background job worker for bulk operations
    services::job_worker::spawn_worker(db_pool.clone());
    println!("✅ Background job worker started");

    // Build application state
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let app_state = AppState {
        config,
        fs_tx,
        rate_limiter,
        db_pool: db_pool.clone(),
        cache_manager,
        job_processor,
        performance_monitor,
        db_health_monitor: db_health_monitor.clone(),
        search_index: search_index.clone(),
        start_time,
        ws_connections: Arc::new(AtomicUsize::new(0)),
    };

    // Optional: Start pool monitoring task (commented out - requires db_monitor)
    // let monitor_pool = db_pool.clone();
    // let monitor_db_monitor = db_monitor.clone();
    // tokio::spawn(async move {
    //     loop {
    //         tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    //         // ... monitoring code
    //     }
    // });

    // Build router
    let app = build_router(app_state.clone());

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🌐 Listening on http://{}", addr);
    println!("📡 WebSocket available at ws://{}/api/ws", addr);
    println!("✨ Ready to accept connections!");

    // Start background job worker pool
    let worker_pool =
        workers::WorkerPool::new(app_state.db_pool.clone(), app_state.fs_tx.clone(), 4);
    let worker_handle = tokio::spawn(async move {
        worker_pool.start().await;
    });

    // Start new job system workers and store shutdown token for graceful shutdown
    let job_worker_shutdown =
        jobs::worker::start_job_workers(Arc::new(app_state.db_pool.clone()), 2).await;

    // Start new job scheduler and store shutdown token for graceful shutdown
    let job_scheduler_shutdown =
        jobs::scheduler::start_job_scheduler(Arc::new(app_state.db_pool.clone())).await;

    // Store shutdown tokens for cleanup on exit
    let _shutdown_tokens = (job_worker_shutdown, job_scheduler_shutdown);

    // Start cron scheduler (currently disabled)
    let cron_scheduler = cron::CronScheduler::new(app_state.db_pool.clone());
    let _cron_handle = tokio::spawn(async move {
        cron_scheduler.start().await;
    });

    // Start periodic cleanup tasks for auth security
    let cleanup_pool = app_state.db_pool.clone();
    let _cleanup_handle = tokio::spawn(async move {
        use tokio::time::{interval, Duration};
        let mut interval = interval(Duration::from_secs(3600)); // Run every hour

        loop {
            interval.tick().await;

            // Cleanup expired sessions
            match services::auth_security_service::cleanup_expired_sessions(&cleanup_pool).await {
                Ok(count) => {
                    if count > 0 {
                        tracing::info!("🧹 Cleaned up {} expired sessions", count);
                    }
                }
                Err(e) => tracing::error!("Failed to cleanup expired sessions: {}", e),
            }

            // Cleanup old login attempts (keep last 90 days)
            match services::auth_security_service::cleanup_old_login_attempts(&cleanup_pool).await {
                Ok(count) => {
                    if count > 0 {
                        tracing::info!("🧹 Cleaned up {} old login attempts", count);
                    }
                }
                Err(e) => tracing::error!("Failed to cleanup old login attempts: {}", e),
            }

            // Cleanup expired refresh tokens
            match auth::cleanup_expired_tokens(&cleanup_pool).await {
                Ok(count) => {
                    if count > 0 {
                        tracing::info!("🧹 Cleaned up {} expired refresh tokens", count);
                    }
                }
                Err(e) => tracing::error!("Failed to cleanup expired tokens: {}", e),
            }

            // Cleanup old file versions (keep max 50, delete older than 90 days)
            match services::version_storage_service::cleanup_all_old_versions(&cleanup_pool).await {
                Ok(count) => {
                    if count > 0 {
                        tracing::info!("🧹 Cleaned up {} old file versions", count);
                    }
                }
                Err(e) => tracing::error!("Failed to cleanup old file versions: {}", e),
            }
        }
    });

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Server error");

    // Wait for workers to finish (won't reach here normally)
    let _ = worker_handle.await;
}

// ==================== ROUTER BUILDER ====================

/// WebSocket handler for main app (with AppState)
async fn ws_handler(ws: WebSocketUpgrade, AxumState(state): AxumState<AppState>) -> Response {
    let fs_tx = state.fs_tx.clone();
    ws.on_upgrade(|socket| async move { websocket::handle_socket(socket, fs_tx).await })
}

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
        .route("/api/ws", get(ws_handler))
        // Public status endpoints (no auth required)
        .route("/status", get(status::get_status_html))
        .route("/status/json", get(status::get_status_json))
        .route("/api/status", get(status::get_status_json)) // Public API status
        .route("/health", get(status::health_check))
        // API routes (delegated to api module)
        .nest("/api", api::build_api_router(state.clone()))
        // Apply middleware (order matters!)
        .layer(axum_middleware::from_fn(
            security::security_headers_middleware,
        ))
        .layer(cors)
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100 MB
        .with_state(state)
}
