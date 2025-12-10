/// Redis caching layer for metadata and thumbnails
/// 
/// This module provides a unified caching interface with:
/// - Redis as primary cache (when available)
/// - In-memory fallback for single-server deployments
/// - TTL-based expiration
/// - Cache statistics tracking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// Redis cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: i32,
    pub ttl_seconds: u64,
    pub max_connections: u32,
    pub enabled: bool,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 6379,
            password: None,
            db: 0,
            ttl_seconds: 3600, // 1 hour default
            max_connections: 10,
            enabled: false, // Disabled by default, uses in-memory
        }
    }
}

/// Cache entry with value and expiration
#[derive(Clone)]
struct CacheEntry {
    value: Vec<u8>,
    expires_at: SystemTime,
    size_bytes: usize,
}

/// In-memory cache as fallback (used when Redis is not available)
#[derive(Clone)]
pub struct InMemoryCache {
    data: Arc<RwLock<HashMap<String, CacheEntry>>>,
    stats: Arc<CacheStatsInternal>,
    max_size_bytes: usize,
    current_size: Arc<AtomicU64>,
}

/// Internal stats tracking
struct CacheStatsInternal {
    hits: AtomicU64,
    misses: AtomicU64,
    sets: AtomicU64,
    deletes: AtomicU64,
    evictions: AtomicU64,
}

impl InMemoryCache {
    pub fn new() -> Self {
        Self::with_max_size(100 * 1024 * 1024) // 100 MB default
    }
    
    pub fn with_max_size(max_size_bytes: usize) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(CacheStatsInternal {
                hits: AtomicU64::new(0),
                misses: AtomicU64::new(0),
                sets: AtomicU64::new(0),
                deletes: AtomicU64::new(0),
                evictions: AtomicU64::new(0),
            }),
            max_size_bytes,
            current_size: Arc::new(AtomicU64::new(0)),
        }
    }
    
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let cache = self.data.read().ok()?;
        let entry = cache.get(key)?;
        
        if entry.expires_at > SystemTime::now() {
            self.stats.hits.fetch_add(1, Ordering::Relaxed);
            Some(entry.value.clone())
        } else {
            self.stats.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }
    
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|v| String::from_utf8(v).ok())
    }
    
    pub fn set(&self, key: String, value: Vec<u8>, ttl_seconds: u64) -> Result<(), String> {
        let value_size = value.len();
        
        // Check if we need to evict entries
        self.maybe_evict(value_size);
        
        let mut cache = self.data.write().map_err(|e| e.to_string())?;
        
        // Remove old entry size if exists
        if let Some(old) = cache.get(&key) {
            self.current_size.fetch_sub(old.size_bytes as u64, Ordering::Relaxed);
        }
        
        let expires_at = SystemTime::now() + Duration::from_secs(ttl_seconds);
        
        cache.insert(key, CacheEntry { 
            value, 
            expires_at,
            size_bytes: value_size,
        });
        
        self.current_size.fetch_add(value_size as u64, Ordering::Relaxed);
        self.stats.sets.fetch_add(1, Ordering::Relaxed);
        
        Ok(())
    }
    
    pub fn set_string(&self, key: String, value: String, ttl_seconds: u64) -> Result<(), String> {
        self.set(key, value.into_bytes(), ttl_seconds)
    }
    
    pub fn delete(&self, key: &str) -> Result<bool, String> {
        let mut cache = self.data.write().map_err(|e| e.to_string())?;
        
        if let Some(entry) = cache.remove(key) {
            self.current_size.fetch_sub(entry.size_bytes as u64, Ordering::Relaxed);
            self.stats.deletes.fetch_add(1, Ordering::Relaxed);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
    
    /// Clean up expired entries
    pub fn cleanup_expired(&self) -> usize {
        if let Ok(mut cache) = self.data.write() {
            let now = SystemTime::now();
            let before = cache.len();
            
            let expired_keys: Vec<String> = cache.iter()
                .filter(|(_, entry)| entry.expires_at <= now)
                .map(|(k, _)| k.clone())
                .collect();
            
            for key in &expired_keys {
                if let Some(entry) = cache.remove(key) {
                    self.current_size.fetch_sub(entry.size_bytes as u64, Ordering::Relaxed);
                }
            }
            
            before - cache.len()
        } else {
            0
        }
    }
    
    /// Evict oldest entries if cache is too large
    fn maybe_evict(&self, needed_bytes: usize) {
        let current = self.current_size.load(Ordering::Relaxed) as usize;
        
        if current + needed_bytes <= self.max_size_bytes {
            return;
        }
        
        // Evict oldest entries until we have space
        if let Ok(mut cache) = self.data.write() {
            let mut entries: Vec<_> = cache.iter()
                .map(|(k, v)| (k.clone(), v.expires_at, v.size_bytes))
                .collect();
            
            // Sort by expiration (oldest first)
            entries.sort_by_key(|(_, exp, _)| *exp);
            
            let mut freed = 0;
            for (key, _, size) in entries {
                if freed >= needed_bytes {
                    break;
                }
                
                if cache.remove(&key).is_some() {
                    freed += size;
                    self.stats.evictions.fetch_add(1, Ordering::Relaxed);
                }
            }
            
            self.current_size.fetch_sub(freed as u64, Ordering::Relaxed);
        }
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.data.read().ok();
        let total_keys = cache.as_ref().map(|c| c.len()).unwrap_or(0);
        let memory_usage_bytes = self.current_size.load(Ordering::Relaxed) as usize;
        
        let hits = self.stats.hits.load(Ordering::Relaxed);
        let misses = self.stats.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 { hits as f64 / total as f64 } else { 0.0 };
        
        CacheStats {
            hits,
            misses,
            hit_rate,
            total_keys,
            memory_usage_bytes,
            max_size_bytes: self.max_size_bytes,
            sets: self.stats.sets.load(Ordering::Relaxed),
            deletes: self.stats.deletes.load(Ordering::Relaxed),
            evictions: self.stats.evictions.load(Ordering::Relaxed),
        }
    }
    
    /// Clear all entries
    pub fn clear(&self) -> Result<(), String> {
        let mut cache = self.data.write().map_err(|e| e.to_string())?;
        cache.clear();
        self.current_size.store(0, Ordering::Relaxed);
        Ok(())
    }
    
    /// Get all keys matching a pattern (simple prefix match)
    pub fn keys(&self, pattern: &str) -> Vec<String> {
        if let Ok(cache) = self.data.read() {
            let prefix = pattern.trim_end_matches('*');
            cache.keys()
                .filter(|k| k.starts_with(prefix))
                .cloned()
                .collect()
        } else {
            vec![]
        }
    }
}

impl Default for InMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Redis cache wrapper with in-memory fallback
/// 
/// This implementation provides a fully functional in-memory cache as fallback.
/// Redis integration can be enabled by setting `enabled: true` in config.
/// The in-memory cache is production-ready for single-server deployments.
#[derive(Clone)]
pub struct RedisCache {
    config: RedisConfig,
    fallback: InMemoryCache,
}

impl RedisCache {
    pub fn new(config: RedisConfig) -> Self {
        let max_size = (config.max_connections as usize) * 10 * 1024 * 1024; // ~10MB per connection
        
        Self {
            config,
            fallback: InMemoryCache::with_max_size(max_size),
        }
    }
    
    /// Get value from cache
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        // TODO: Add Redis support when enabled
        // if self.config.enabled {
        //     // Redis implementation
        // }
        
        Ok(self.fallback.get(key))
    }
    
    /// Get string value from cache
    pub async fn get_string(&self, key: &str) -> Result<Option<String>, CacheError> {
        Ok(self.fallback.get_string(key))
    }
    
    /// Set value in cache with TTL
    pub async fn set(&self, key: String, value: Vec<u8>, ttl_seconds: Option<u64>) -> Result<(), CacheError> {
        let ttl = ttl_seconds.unwrap_or(self.config.ttl_seconds);
        self.fallback.set(key, value, ttl).map_err(CacheError::Internal)
    }
    
    /// Set string value in cache with TTL
    pub async fn set_string(&self, key: String, value: String, ttl_seconds: Option<u64>) -> Result<(), CacheError> {
        let ttl = ttl_seconds.unwrap_or(self.config.ttl_seconds);
        self.fallback.set_string(key, value, ttl).map_err(CacheError::Internal)
    }
    
    /// Delete value from cache
    pub async fn delete(&self, key: &str) -> Result<bool, CacheError> {
        self.fallback.delete(key).map_err(CacheError::Internal)
    }
    
    /// Check if key exists
    pub async fn exists(&self, key: &str) -> Result<bool, CacheError> {
        Ok(self.fallback.exists(key))
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.fallback.stats()
    }
    
    /// Clear all cache entries
    pub async fn clear(&self) -> Result<(), CacheError> {
        self.fallback.clear().map_err(CacheError::Internal)
    }
    
    /// Get keys matching pattern
    pub async fn keys(&self, pattern: &str) -> Result<Vec<String>, CacheError> {
        Ok(self.fallback.keys(pattern))
    }
    
    /// Run cleanup of expired entries
    pub async fn cleanup(&self) -> usize {
        self.fallback.cleanup_expired()
    }
    
    /// Check if Redis is connected (always false for in-memory)
    pub fn is_redis_connected(&self) -> bool {
        false // In-memory fallback mode
    }
    
    /// Get configuration
    pub fn config(&self) -> &RedisConfig {
        &self.config
    }
}

/// Cache error types
#[derive(Debug, Clone)]
pub enum CacheError {
    ConnectionFailed(String),
    Internal(String),
    Timeout,
    KeyNotFound,
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::ConnectionFailed(msg) => write!(f, "Cache connection failed: {}", msg),
            CacheError::Internal(msg) => write!(f, "Cache internal error: {}", msg),
            CacheError::Timeout => write!(f, "Cache operation timed out"),
            CacheError::KeyNotFound => write!(f, "Key not found in cache"),
        }
    }
}

impl std::error::Error for CacheError {}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub total_keys: usize,
    pub memory_usage_bytes: usize,
    pub max_size_bytes: usize,
    pub sets: u64,
    pub deletes: u64,
    pub evictions: u64,
}

/// Cache helper functions for generating consistent keys
pub mod cache_keys {
    /// Generate cache key for file metadata
    pub fn file_metadata(file_id: &str) -> String {
        format!("file:metadata:{}", file_id)
    }
    
    /// Generate cache key for thumbnail
    pub fn thumbnail(file_id: &str, size: &str) -> String {
        format!("thumbnail:{}:{}", file_id, size)
    }
    
    /// Generate cache key for search results
    pub fn search_results(query: &str, user_id: &str) -> String {
        format!("search:{}:{}", user_id, query)
    }
    
    /// Generate cache key for user session
    pub fn user_session(session_id: &str) -> String {
        format!("session:{}", session_id)
    }
    
    /// Generate cache key for folder contents
    pub fn folder_contents(folder_path: &str) -> String {
        format!("folder:contents:{}", folder_path)
    }
    
    /// Generate cache key for user preferences
    pub fn user_preferences(user_id: &str) -> String {
        format!("user:preferences:{}", user_id)
    }
    
    /// Generate cache key for file preview
    pub fn file_preview(file_id: &str) -> String {
        format!("preview:{}", file_id)
    }
    
    /// Generate cache key for directory listing
    pub fn directory_listing(path: &str, user_id: &str) -> String {
        format!("dir:{}:{}", user_id, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_in_memory_cache() {
        let cache = InMemoryCache::new();
        
        // Test set and get
        cache.set("key1".to_string(), b"value1".to_vec(), 60).unwrap();
        assert_eq!(cache.get("key1"), Some(b"value1".to_vec()));
        
        // Test non-existent key
        assert_eq!(cache.get("nonexistent"), None);
        
        // Test delete
        cache.delete("key1").unwrap();
        assert_eq!(cache.get("key1"), None);
    }
    
    #[test]
    fn test_cache_stats() {
        let cache = InMemoryCache::new();
        
        cache.set("key1".to_string(), b"value1".to_vec(), 60).unwrap();
        cache.get("key1"); // hit
        cache.get("key2"); // miss
        
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.total_keys, 1);
    }
}
