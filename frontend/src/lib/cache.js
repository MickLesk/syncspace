/**
 * Simple in-memory cache for API responses
 * Reduces redundant API calls and improves performance
 */

class ApiCache {
  constructor() {
    this.cache = new Map();
    this.timestamps = new Map();
    this.defaultTTL = 5 * 60 * 1000; // 5 minutes
  }

  /**
   * Generate cache key from URL and params
   */
  generateKey(endpoint, params = {}) {
    const paramString = Object.keys(params)
      .sort()
      .map(key => `${key}=${JSON.stringify(params[key])}`)
      .join('&');
    return `${endpoint}?${paramString}`;
  }

  /**
   * Get cached value if not expired
   */
  get(key) {
    const timestamp = this.timestamps.get(key);
    if (!timestamp) return null;

    const now = Date.now();
    const age = now - timestamp;

    // Check if expired
    if (age > this.defaultTTL) {
      this.cache.delete(key);
      this.timestamps.delete(key);
      return null;
    }

    return this.cache.get(key);
  }

  /**
   * Set cached value with timestamp
   */
  set(key, value, ttl = this.defaultTTL) {
    this.cache.set(key, value);
    this.timestamps.set(key, Date.now());

    // Auto-cleanup after TTL
    setTimeout(() => {
      this.cache.delete(key);
      this.timestamps.delete(key);
    }, ttl);
  }

  /**
   * Check if key exists and is valid
   */
  has(key) {
    return this.get(key) !== null;
  }

  /**
   * Invalidate specific cache key
   */
  invalidate(key) {
    this.cache.delete(key);
    this.timestamps.delete(key);
  }

  /**
   * Invalidate cache entries matching pattern
   */
  invalidatePattern(pattern) {
    const regex = new RegExp(pattern);
    for (const key of this.cache.keys()) {
      if (regex.test(key)) {
        this.cache.delete(key);
        this.timestamps.delete(key);
      }
    }
  }

  /**
   * Clear all cache
   */
  clear() {
    this.cache.clear();
    this.timestamps.clear();
  }

  /**
   * Get cache statistics
   */
  getStats() {
    return {
      size: this.cache.size,
      keys: Array.from(this.cache.keys()),
      oldestEntry: Math.min(...Array.from(this.timestamps.values())),
      newestEntry: Math.max(...Array.from(this.timestamps.values())),
    };
  }
}

// Export singleton instance
export const apiCache = new ApiCache();

/**
 * Cache decorator for async functions
 * Usage: const cachedFn = withCache(originalFn, 'cache-key', 60000);
 */
export function withCache(fn, keyPrefix, ttl = 5 * 60 * 1000) {
  return async function(...args) {
    const key = apiCache.generateKey(keyPrefix, { args });
    
    // Try cache first
    const cached = apiCache.get(key);
    if (cached !== null) {
      console.log(`[Cache HIT] ${key}`);
      return cached;
    }

    // Cache miss - call original function
    console.log(`[Cache MISS] ${key}`);
    const result = await fn.apply(this, args);
    apiCache.set(key, result, ttl);
    return result;
  };
}

/**
 * Cache invalidation helpers
 */
export const cacheInvalidators = {
  // Invalidate file list caches when files change
  onFileChange(path) {
    apiCache.invalidatePattern(`/api/files.*${path}`);
    apiCache.invalidatePattern(`/api/search`);
  },

  // Invalidate user caches on profile update
  onUserUpdate(userId) {
    apiCache.invalidatePattern(`/api/users/${userId}`);
    apiCache.invalidatePattern(`/api/users/preferences`);
  },

  // Invalidate share caches
  onShareChange() {
    apiCache.invalidatePattern(`/api/shares`);
    apiCache.invalidatePattern(`/api/shared-with-me`);
  },

  // Invalidate collaboration caches
  onCollaborationChange() {
    apiCache.invalidatePattern(`/api/collaboration`);
  },
};
