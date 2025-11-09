/**
 * Request Caching Service
 * Implements Stale-While-Revalidate, request deduplication, and cache invalidation
 * Works with existing API layer for transparent caching
 */

const cache = new Map();
const pendingRequests = new Map();

/**
 * Cache configuration per endpoint
 */
const CACHE_CONFIG = {
  // Files
  'GET /files/*': { ttl: 30 * 1000, swr: 5 * 60 * 1000 },      // 30s TTL, 5m SWR
  'POST /files/*': { ttl: 0, invalidates: ['GET /files/*'] },
  'PUT /files/*': { ttl: 0, invalidates: ['GET /files/*'] },
  'DELETE /files/*': { ttl: 0, invalidates: ['GET /files/*'] },

  // Users
  'GET /users/*': { ttl: 2 * 60 * 1000, swr: 10 * 60 * 1000 }, // 2m TTL, 10m SWR
  'PUT /users/*': { ttl: 0, invalidates: ['GET /users/*'] },

  // Search
  'GET /search/*': { ttl: 1 * 60 * 1000, swr: 5 * 60 * 1000 },  // 1m TTL, 5m SWR

  // Activity
  'GET /activity/*': { ttl: 15 * 1000, swr: 2 * 60 * 1000 },   // 15s TTL, 2m SWR

  // Share
  'GET /share/*': { ttl: 5 * 60 * 1000, swr: 30 * 60 * 1000 },  // 5m TTL, 30m SWR
};

/**
 * Generate cache key from method and URL
 */
function getCacheKey(method, url, params = {}) {
  const queryString = new URLSearchParams(params).toString();
  return `${method} ${url}${queryString ? '?' + queryString : ''}`;
}

/**
 * Find matching config pattern for URL
 */
function findCacheConfig(method, url) {
  for (const [pattern, config] of Object.entries(CACHE_CONFIG)) {
    const [patternMethod, patternPath] = pattern.split(' ');
    if (method !== patternMethod) continue;

    // Simple wildcard matching
    const regex = new RegExp('^' + patternPath.replace(/\*/g, '.*') + '$');
    if (regex.test(url)) {
      return config;
    }
  }
  return null;
}

/**
 * Get cache entry
 */
function getCacheEntry(key) {
  const entry = cache.get(key);
  if (!entry) return null;

  const now = Date.now();
  const age = now - entry.timestamp;

  // Still fresh
  if (age < entry.config.ttl) {
    return { data: entry.data, fresh: true };
  }

  // Stale but within SWR window
  if (age < entry.config.swr) {
    return { data: entry.data, fresh: false };
  }

  // Expired
  cache.delete(key);
  return null;
}

/**
 * Set cache entry
 */
function setCacheEntry(key, data, config) {
  cache.set(key, {
    data,
    config,
    timestamp: Date.now()
  });
}

/**
 * Invalidate cache patterns
 */
function invalidateCache(patterns) {
  if (typeof patterns === 'string') {
    patterns = [patterns];
  }

  for (const key of cache.keys()) {
    for (const pattern of patterns) {
      const regex = new RegExp('^' + pattern.replace(/\*/g, '.*'));
      if (regex.test(key)) {
        cache.delete(key);
      }
    }
  }
}

/**
 * Wrap fetch with caching
 */
export function createCachedFetch(originalFetch) {
  return async function cachedFetch(url, options = {}) {
    const method = (options.method || 'GET').toUpperCase();
    const cacheConfig = findCacheConfig(method, url);

    // Not cacheable
    if (!cacheConfig || cacheConfig.ttl === 0) {
      return originalFetch(url, options);
    }

    const cacheKey = getCacheKey(method, url, options.params);

    // Check for stale-while-revalidate
    const cached = getCacheEntry(cacheKey);
    if (cached) {
      // If fresh, return immediately
      if (cached.fresh) {
        return { data: cached.data };
      }

      // If stale, return stale data but revalidate in background
      if (!pendingRequests.has(cacheKey)) {
        // Revalidate in background
        originalFetch(url, options)
          .then(response => {
            if (response.ok) {
              setCacheEntry(cacheKey, response.data, cacheConfig);
              
              // Dispatch event for UI update
              window.dispatchEvent(new CustomEvent('cache-updated', {
                detail: { key: cacheKey, data: response.data }
              }));
            }
          })
          .catch(error => console.error('Background revalidation failed:', error))
          .finally(() => {
            pendingRequests.delete(cacheKey);
          });
      }

      return { data: cached.data, stale: true };
    }

    // Check if request is already pending (deduplication)
    if (pendingRequests.has(cacheKey)) {
      return pendingRequests.get(cacheKey);
    }

    // New request
    const requestPromise = (async () => {
      try {
        const response = await originalFetch(url, options);
        if (response.ok || response.data) {
          setCacheEntry(cacheKey, response.data, cacheConfig);
        }
        return response;
      } finally {
        pendingRequests.delete(cacheKey);
      }
    })();

    pendingRequests.set(cacheKey, requestPromise);
    return requestPromise;
  };
}

/**
 * Clear all cache
 */
export function clearCache() {
  cache.clear();
}

/**
 * Get cache statistics
 */
export function getCacheStats() {
  return {
    size: cache.size,
    entries: Array.from(cache.entries()).map(([key, entry]) => ({
      key,
      age: Date.now() - entry.timestamp,
      ttl: entry.config.ttl,
      swr: entry.config.swr,
      fresh: Date.now() - entry.timestamp < entry.config.ttl
    }))
  };
}

/**
 * Handle cache invalidation on mutations
 */
export function setupCacheInvalidation(apiClient) {
  const originalRequest = apiClient.request;

  apiClient.request = async function(...args) {
    const response = await originalRequest.apply(this, args);

    // Get method and URL from first argument (could be different formats)
    const [method, url] = typeof args[0] === 'string' 
      ? args[0].split(' ') 
      : [args[1]?.method || 'GET', args[0]];

    // Check if mutation should invalidate cache
    const cacheConfig = findCacheConfig(method, url);
    if (cacheConfig?.invalidates) {
      invalidateCache(cacheConfig.invalidates);
    }

    return response;
  };

  return apiClient;
}

/**
 * Manual cache invalidation helper
 */
export function invalidateCacheFor(method, urlPattern) {
  const pattern = `${method} ${urlPattern}`;
  invalidateCache([pattern]);
}

/**
 * Subscribe to cache updates
 */
export function onCacheUpdate(callback) {
  window.addEventListener('cache-updated', (e) => {
    callback(e.detail);
  });

  return () => {
    window.removeEventListener('cache-updated', callback);
  };
}
