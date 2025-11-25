//! Database monitoring and diagnostics module
//! 
//! Provides health checks, performance monitoring, and connection leak detection.

use sqlx::{SqlitePool, Row};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, warn, error};
use serde::{Serialize, Deserialize};

/// Slow query threshold - queries taking longer than this will be logged
const SLOW_QUERY_THRESHOLD_MS: u64 = 100;

/// Connection leak detection threshold - connections held longer than this are suspicious
const CONNECTION_LEAK_THRESHOLD_SECS: u64 = 60;

/// Database health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseHealth {
    pub status: HealthStatus,
    pub pool_size: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub total_queries: u64,
    pub slow_queries: u64,
    pub average_query_time_ms: f64,
    pub database_size_mb: f64,
    pub wal_size_mb: f64,
    pub last_checkpoint: Option<String>,
    pub uptime_seconds: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Query execution statistics
#[derive(Debug, Clone)]
pub struct QueryStats {
    pub query: String,
    pub duration: Duration,
    pub timestamp: Instant,
}

/// Database monitor for tracking performance and health
pub struct DatabaseMonitor {
    start_time: Instant,
    total_queries: std::sync::atomic::AtomicU64,
    slow_queries: std::sync::atomic::AtomicU64,
    total_query_time: std::sync::Arc<std::sync::Mutex<Duration>>,
}

impl DatabaseMonitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_queries: std::sync::atomic::AtomicU64::new(0),
            slow_queries: std::sync::atomic::AtomicU64::new(0),
            total_query_time: std::sync::Arc::new(std::sync::Mutex::new(Duration::ZERO)),
        }
    }

    /// Record a query execution
    pub fn record_query(&self, duration: Duration) {
        self.total_queries.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        // Check for slow query
        if duration.as_millis() as u64 > SLOW_QUERY_THRESHOLD_MS {
            self.slow_queries.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            warn!("⚠️ Slow query detected: {:?}", duration);
        }
        
        // Update total query time
        if let Ok(mut total) = self.total_query_time.lock() {
            *total += duration;
        }
    }

    /// Get current statistics
    pub fn get_stats(&self) -> (u64, u64, f64) {
        let total = self.total_queries.load(std::sync::atomic::Ordering::Relaxed);
        let slow = self.slow_queries.load(std::sync::atomic::Ordering::Relaxed);
        
        let avg = if total > 0 {
            if let Ok(total_time) = self.total_query_time.lock() {
                total_time.as_millis() as f64 / total as f64
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        (total, slow, avg)
    }

    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

impl Default for DatabaseMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute a query with timing and monitoring
#[macro_export]
macro_rules! monitored_query {
    ($pool:expr, $query:expr, $monitor:expr) => {{
        let start = std::time::Instant::now();
        let result = $query;
        let duration = start.elapsed();
        $monitor.record_query(duration);
        result
    }};
}

/// Check database health
pub async fn check_health(pool: &SqlitePool, monitor: &DatabaseMonitor) -> Result<DatabaseHealth, sqlx::Error> {
    let mut errors = Vec::new();
    let mut status = HealthStatus::Healthy;

    // Get pool statistics
    let pool_size = pool.size();
    let idle_count = pool.num_idle() as u32; // Convert usize to u32
    let active_connections = idle_count;
    let idle_connections = pool_size.saturating_sub(idle_count);

    // Check if pool is over-utilized
    if active_connections as f32 / pool_size as f32 > 0.9 {
        errors.push("Connection pool is over 90% utilized".to_string());
        status = HealthStatus::Degraded;
    }

    // Get query statistics
    let (total_queries, slow_queries, avg_query_time) = monitor.get_stats();

    // Check for too many slow queries
    if total_queries > 100 && (slow_queries as f32 / total_queries as f32) > 0.1 {
        errors.push(format!("High slow query rate: {:.1}%", 
            (slow_queries as f32 / total_queries as f32) * 100.0));
        status = HealthStatus::Degraded;
    }

    // Get database file sizes
    let db_size_result = sqlx::query("SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()")
        .fetch_one(pool)
        .await;
    
    let database_size_mb = match db_size_result {
        Ok(row) => {
            let size_bytes: i64 = row.get(0);
            size_bytes as f64 / (1024.0 * 1024.0)
        }
        Err(e) => {
            errors.push(format!("Failed to get database size: {}", e));
            0.0
        }
    };

    // Get WAL size
    let wal_info = sqlx::query("PRAGMA wal_checkpoint(PASSIVE)")
        .fetch_optional(pool)
        .await;
    
    let wal_size_mb = match wal_info {
        Ok(Some(_)) => {
            // WAL size estimation via file metadata
            use std::path::Path;
            if let Ok(metadata) = std::fs::metadata("./data/syncspace.db-wal") {
                metadata.len() as f64 / 1024.0 / 1024.0 // Convert to MB
            } else {
                0.0
            }
        }
        Ok(None) => 0.0,
        Err(e) => {
            errors.push(format!("Failed to get WAL info: {}", e));
            0.0
        }
    };

    // Check last checkpoint time
    let last_checkpoint = None; // Would need to track this separately

    // Get uptime
    let uptime_seconds = monitor.uptime_seconds();

    // Test a simple query
    match sqlx::query("SELECT 1").fetch_one(pool).await {
        Ok(_) => {}
        Err(e) => {
            errors.push(format!("Health check query failed: {}", e));
            status = HealthStatus::Unhealthy;
        }
    }

    Ok(DatabaseHealth {
        status,
        pool_size,
        active_connections,
        idle_connections,
        total_queries,
        slow_queries,
        average_query_time_ms: avg_query_time,
        database_size_mb,
        wal_size_mb,
        last_checkpoint,
        uptime_seconds,
        errors,
    })
}

/// Perform a WAL checkpoint
pub async fn checkpoint_wal(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("🔄 Starting WAL checkpoint...");
    
    let start = Instant::now();
    
    // RESTART mode: checkpoint and reset WAL
    sqlx::query("PRAGMA wal_checkpoint(RESTART)")
        .execute(pool)
        .await?;
    
    let duration = start.elapsed();
    info!("✅ WAL checkpoint completed in {:?}", duration);
    
    Ok(())
}

/// Analyze database and update statistics
pub async fn analyze_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("📊 Analyzing database...");
    
    let start = Instant::now();
    
    sqlx::query("ANALYZE")
        .execute(pool)
        .await?;
    
    let duration = start.elapsed();
    info!("✅ Database analysis completed in {:?}", duration);
    
    Ok(())
}

/// Vacuum database to reclaim space
pub async fn vacuum_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("🧹 Vacuuming database...");
    
    let start = Instant::now();
    
    sqlx::query("VACUUM")
        .execute(pool)
        .await?;
    
    let duration = start.elapsed();
    info!("✅ Database vacuumed in {:?}", duration);
    
    Ok(())
}

/// Detect and warn about connection leaks
pub async fn detect_connection_leaks(pool: &SqlitePool) {
    let pool_size = pool.size();
    let idle = pool.num_idle() as u32;
    let active = pool_size.saturating_sub(idle);
    
    // If most connections are active for extended period, might be a leak
    if active > 0 && active as f32 / pool_size as f32 > 0.8 {
        warn!("⚠️ Possible connection leak detected: {}/{} connections active", 
              active, pool_size);
        warn!("⚠️ Consider checking for missing connection releases");
    }
    
    info!("🔍 Connection status: {} active, {} idle, {} total", 
          active, idle, pool_size);
}

/// Background task to monitor database health
pub async fn monitor_background(pool: SqlitePool, monitor: std::sync::Arc<DatabaseMonitor>) {
    info!("🔍 Starting database monitor background task");
    
    loop {
        sleep(Duration::from_secs(60)).await; // Check every minute
        
        // Check for connection leaks
        detect_connection_leaks(&pool).await;
        
        // Log statistics
        let (total, slow, avg) = monitor.get_stats();
        info!("📊 Database stats: {} total queries, {} slow (>{} ms), {:.2} ms avg", 
              total, slow, SLOW_QUERY_THRESHOLD_MS, avg);
        
        // Periodic WAL checkpoint (every 5 minutes)
        if monitor.uptime_seconds() % 300 == 0 {
            if let Err(e) = checkpoint_wal(&pool).await {
                error!("❌ WAL checkpoint failed: {}", e);
            }
        }
        
        // Periodic database analysis (every 30 minutes)
        if monitor.uptime_seconds() % 1800 == 0 {
            if let Err(e) = analyze_database(&pool).await {
                error!("❌ Database analysis failed: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        let monitor = DatabaseMonitor::new();
        assert_eq!(monitor.total_queries.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(monitor.slow_queries.load(std::sync::atomic::Ordering::Relaxed), 0);
    }

    #[test]
    fn test_query_recording() {
        let monitor = DatabaseMonitor::new();
        
        // Record normal query
        monitor.record_query(Duration::from_millis(50));
        
        // Record slow query
        monitor.record_query(Duration::from_millis(150));
        
        let (total, slow, _) = monitor.get_stats();
        assert_eq!(total, 2);
        assert_eq!(slow, 1);
    }
}
