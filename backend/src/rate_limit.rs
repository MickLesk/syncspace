/// Bandwidth and rate limiting system
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RateLimitEntry {
    pub requests: u32,
    pub bytes_transferred: u64,
    pub window_start: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub max_requests_per_minute: u32,
    pub max_bytes_per_minute: u64,
    pub max_concurrent_uploads: u32,
    pub max_concurrent_downloads: u32,
}

pub struct BandwidthLimiter {
    entries: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    config: RateLimitConfig,
}

impl BandwidthLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }
    
    /// Check if user is rate limited
    pub async fn check_limit(&self, user_id: &str, bytes: u64) -> Result<(), String> {
        let mut entries = self.entries.lock().await;
        let now = Utc::now();
        
        let entry = entries.entry(user_id.to_string()).or_insert(RateLimitEntry {
            requests: 0,
            bytes_transferred: 0,
            window_start: now,
        });
        
        // Reset if window expired (1 minute)
        if now.signed_duration_since(entry.window_start) > Duration::minutes(1) {
            entry.requests = 0;
            entry.bytes_transferred = 0;
            entry.window_start = now;
        }
        
        // Check limits
        if entry.requests >= self.config.max_requests_per_minute {
            return Err("Rate limit exceeded: too many requests".to_string());
        }
        
        if entry.bytes_transferred + bytes > self.config.max_bytes_per_minute {
            return Err("Bandwidth limit exceeded".to_string());
        }
        
        // Update counters
        entry.requests += 1;
        entry.bytes_transferred += bytes;
        
        Ok(())
    }
    
    /// Clean up old entries
    pub async fn cleanup(&self) {
        let mut entries = self.entries.lock().await;
        let now = Utc::now();
        entries.retain(|_, entry| {
            now.signed_duration_since(entry.window_start) <= Duration::minutes(5)
        });
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 60,
            max_bytes_per_minute: 100 * 1024 * 1024, // 100 MB/min
            max_concurrent_uploads: 5,
            max_concurrent_downloads: 10,
        }
    }
}
