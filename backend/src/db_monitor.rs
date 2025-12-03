//! Database Connection Pool Monitoring and Health Checks
//! 
//! Provides real-time metrics, slow query logging, connection leak detection,
//! and dynamic pool sizing capabilities.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::VecDeque;

/// Pool metrics for monitoring and adaptive sizing
#[derive(Debug, Clone, Serialize)]
pub struct PoolMetrics {
    pub timestamp: DateTime<Utc>,
    pub connections_active: u32,
    pub connections_idle: u32,
    pub connections_total: u32,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeouts: u64,
    pub connection_errors: u64,
    pub slow_queries: u64,
    pub average_acquire_time_ms: f64,
    pub average_query_time_ms: f64,
    pub pool_utilization: f64, // 0.0 - 1.0
}

/// Slow query information for logging and analysis
#[derive(Debug, Clone, Serialize)]
pub struct SlowQuery {
    pub timestamp: DateTime<Utc>,
    pub query: String,
    pub duration_ms: u64,
    pub caller: Option<String>,
}

/// Connection leak detection entry
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ConnectionLease {
    acquired_at: Instant,
    caller: String,
}

/// Database monitor with real-time metrics tracking
#[allow(dead_code)]
pub struct DatabaseMonitor {
    metrics: Arc<RwLock<PoolMetrics>>,
    slow_queries: Arc<RwLock<VecDeque<SlowQuery>>>,
    connection_leases: Arc<RwLock<Vec<ConnectionLease>>>,
    slow_query_threshold_ms: u64,
    leak_detection_threshold_secs: u64,
    max_slow_query_history: usize,
}

#[allow(dead_code)]
impl DatabaseMonitor {
    /// Create new database monitor
    pub fn new(max_connections: u32, min_connections: u32) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PoolMetrics {
                timestamp: Utc::now(),
                connections_active: 0,
                connections_idle: 0,
                connections_total: 0,
                max_connections,
                min_connections,
                acquire_timeouts: 0,
                connection_errors: 0,
                slow_queries: 0,
                average_acquire_time_ms: 0.0,
                average_query_time_ms: 0.0,
                pool_utilization: 0.0,
            })),
            slow_queries: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            connection_leases: Arc::new(RwLock::new(Vec::new())),
            slow_query_threshold_ms: 100, // Log queries taking > 100ms
            leak_detection_threshold_secs: 300, // Warn if connection held > 5 minutes
            max_slow_query_history: 100,
        }
    }

    /// Get current pool metrics
    pub async fn get_metrics(&self) -> PoolMetrics {
        self.metrics.read().await.clone()
    }

    /// Update pool utilization metrics
    pub async fn update_pool_stats(&self, active: u32, idle: u32, total: u32) {
        let mut metrics = self.metrics.write().await;
        metrics.timestamp = Utc::now();
        metrics.connections_active = active;
        metrics.connections_idle = idle;
        metrics.connections_total = total;
        metrics.pool_utilization = if metrics.max_connections > 0 {
            total as f64 / metrics.max_connections as f64
        } else {
            0.0
        };
    }

    /// Record a connection acquire event
    pub async fn record_acquire(&self, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        let duration_ms = duration.as_millis() as f64;
        
        // Exponential moving average
        if metrics.average_acquire_time_ms == 0.0 {
            metrics.average_acquire_time_ms = duration_ms;
        } else {
            metrics.average_acquire_time_ms = 
                metrics.average_acquire_time_ms * 0.9 + duration_ms * 0.1;
        }
    }

    /// Record a query execution
    pub async fn record_query(&self, query: &str, duration: Duration, caller: Option<String>) {
        let duration_ms = duration.as_millis() as u64;
        
        // Update average query time
        {
            let mut metrics = self.metrics.write().await;
            if metrics.average_query_time_ms == 0.0 {
                metrics.average_query_time_ms = duration_ms as f64;
            } else {
                metrics.average_query_time_ms = 
                    metrics.average_query_time_ms * 0.9 + duration_ms as f64 * 0.1;
            }
        }

        // Log slow queries
        if duration_ms > self.slow_query_threshold_ms {
            let slow_query = SlowQuery {
                timestamp: Utc::now(),
                query: query.to_string(),
                duration_ms,
                caller,
            };

            let mut slow_queries = self.slow_queries.write().await;
            slow_queries.push_back(slow_query.clone());
            
            // Trim history
            while slow_queries.len() > self.max_slow_query_history {
                slow_queries.pop_front();
            }

            // Increment counter
            self.metrics.write().await.slow_queries += 1;

            // Log warning
            tracing::warn!(
                "Slow query detected: {}ms - {}",
                duration_ms,
                query.chars().take(100).collect::<String>()
            );
        }
    }

    /// Record connection timeout
    pub async fn record_timeout(&self) {
        self.metrics.write().await.acquire_timeouts += 1;
        tracing::warn!("Connection acquire timeout detected");
    }

    /// Record connection error
    pub async fn record_error(&self, error: &str) {
        self.metrics.write().await.connection_errors += 1;
        tracing::error!("Connection error: {}", error);
    }

    /// Get slow query history
    pub async fn get_slow_queries(&self) -> Vec<SlowQuery> {
        self.slow_queries.read().await.iter().cloned().collect()
    }

    /// Track connection acquisition (for leak detection)
    pub async fn track_connection(&self, caller: String) {
        let lease = ConnectionLease {
            acquired_at: Instant::now(),
            caller,
        };
        self.connection_leases.write().await.push(lease);
    }

    /// Release tracked connection
    pub async fn release_connection(&self) {
        let mut leases = self.connection_leases.write().await;
        if !leases.is_empty() {
            leases.remove(0);
        }
    }

    /// Check for connection leaks
    pub async fn detect_leaks(&self) -> Vec<String> {
        let leases = self.connection_leases.read().await;
        let threshold = Duration::from_secs(self.leak_detection_threshold_secs);
        let now = Instant::now();

        leases
            .iter()
            .filter(|lease| now.duration_since(lease.acquired_at) > threshold)
            .map(|lease| {
                let duration = now.duration_since(lease.acquired_at).as_secs();
                format!(
                    "Connection leak detected: {} (held for {}s)",
                    lease.caller, duration
                )
            })
            .collect()
    }

    /// Calculate recommended pool size based on metrics
    pub async fn recommend_pool_size(&self) -> (u32, u32) {
        let metrics = self.metrics.read().await;
        
        // If utilization is high (>80%), recommend increase
        if metrics.pool_utilization > 0.8 && metrics.max_connections < 50 {
            let new_max = (metrics.max_connections + 5).min(50);
            let new_min = (metrics.min_connections + 1).min(10);
            return (new_max, new_min);
        }
        
        // If utilization is low (<30%) and we have many connections, recommend decrease
        if metrics.pool_utilization < 0.3 && metrics.max_connections > 10 {
            let new_max = (metrics.max_connections - 5).max(10);
            let new_min = metrics.min_connections;
            return (new_max, new_min);
        }

        // No change needed
        (metrics.max_connections, metrics.min_connections)
    }

    /// Get health status
    pub async fn health_status(&self) -> HealthStatus {
        let metrics = self.metrics.read().await;
        let slow_queries = self.slow_queries.read().await;
        let leaks = self.detect_leaks().await;

        let status = if metrics.pool_utilization > 0.95 {
            "critical"
        } else if metrics.pool_utilization > 0.8 || !leaks.is_empty() {
            "warning"
        } else if metrics.acquire_timeouts > 10 || metrics.connection_errors > 5 {
            "degraded"
        } else {
            "healthy"
        };

        HealthStatus {
            status: status.to_string(),
            timestamp: Utc::now(),
            pool_utilization: metrics.pool_utilization,
            active_connections: metrics.connections_active,
            total_connections: metrics.connections_total,
            max_connections: metrics.max_connections,
            acquire_timeouts: metrics.acquire_timeouts,
            connection_errors: metrics.connection_errors,
            slow_query_count: slow_queries.len(),
            connection_leaks: leaks.len(),
            average_query_time_ms: metrics.average_query_time_ms,
            average_acquire_time_ms: metrics.average_acquire_time_ms,
        }
    }
}

/// Health status response
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct HealthStatus {
    pub status: String, // "healthy", "degraded", "warning", "critical"
    pub timestamp: DateTime<Utc>,
    pub pool_utilization: f64,
    pub active_connections: u32,
    pub total_connections: u32,
    pub max_connections: u32,
    pub acquire_timeouts: u64,
    pub connection_errors: u64,
    pub slow_query_count: usize,
    pub connection_leaks: usize,
    pub average_query_time_ms: f64,
    pub average_acquire_time_ms: f64,
}

/// Wrapper for monitored query execution
#[allow(dead_code)]
pub async fn execute_monitored<F, R>(
    monitor: &DatabaseMonitor,
    query_name: &str,
    f: F,
) -> R
where
    F: std::future::Future<Output = R>,
{
    let start = Instant::now();
    let caller = format!("query:{}", query_name);
    
    monitor.track_connection(caller.clone()).await;
    
    let result = f.await;
    
    let duration = start.elapsed();
    monitor.record_query(query_name, duration, Some(caller)).await;
    monitor.release_connection().await;
    
    result
}

/// Verify database health (connectivity check)
pub async fn verify_db_health(pool: &SqlitePool) -> Result<(), String> {
    match sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Database health check failed: {}", e)),
    }
}

/// Get SQLite performance statistics
#[allow(dead_code)]
pub async fn get_sqlite_stats(pool: &SqlitePool) -> Result<SqliteStats, sqlx::Error> {
    // Get cache hit rate
    let cache_stats: (i64, i64) = sqlx::query_as("SELECT * FROM pragma_cache_size(), pragma_page_count()")
        .fetch_one(pool)
        .await
        .unwrap_or((0, 0));

    // Get WAL checkpoint info
    let wal_info: (i64, i64, i64) = sqlx::query_as("PRAGMA wal_checkpoint(PASSIVE)")
        .fetch_one(pool)
        .await
        .unwrap_or((0, 0, 0));

    Ok(SqliteStats {
        cache_size_pages: cache_stats.0,
        total_pages: cache_stats.1,
        wal_frames_checkpointed: wal_info.1,
        wal_frames_pending: wal_info.2,
    })
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct SqliteStats {
    pub cache_size_pages: i64,
    pub total_pages: i64,
    pub wal_frames_checkpointed: i64,
    pub wal_frames_pending: i64,
}
