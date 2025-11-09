/**
 * Async Component Loader
 * Wraps dynamic imports for code splitting with loading and error states
 * 
 * Usage:
 * const FilesView = defineAsyncComponent(() => import('./pages/files/FilesView.svelte'));
 * 
 * In template:
 * <svelte:component this={FilesView} />
 */

/**
 * Create an async component that loads on demand
 * @param {() => Promise<{default: any}>} loader - Dynamic import function
 * @param {number} delay - Debounce delay for loading state (ms)
 * @returns {Promise<any>} Resolved component
 */
export async function defineAsyncComponent(loader, delay = 0) {
  try {
    const module = await loader();
    return module.default;
  } catch (error) {
    console.error('Failed to load async component:', error);
    throw error;
  }
}

/**
 * Create a component with loading and error states
 * @param {() => Promise<{default: any}>} loader - Dynamic import function
 * @param {any} LoadingComponent - Component to show while loading
 * @param {any} ErrorComponent - Component to show on error
 * @param {number} delay - Delay before showing loading component (ms)
 * @returns {{component, isLoading, error}} Reactive component state
 */
export function defineAsyncComponentWithFallback(
  loader,
  LoadingComponent = null,
  ErrorComponent = null,
  delay = 200
) {
  let component = $state(null);
  let isLoading = $state(true);
  let error = $state(null);
  let timeoutId = null;

  // Start loading
  const loadPromise = loader()
    .then(mod => {
      // Clear loading timeout
      if (timeoutId) clearTimeout(timeoutId);
      
      component = mod.default;
      isLoading = false;
      error = null;
    })
    .catch(err => {
      console.error('Failed to load component:', err);
      if (timeoutId) clearTimeout(timeoutId);
      
      error = err;
      component = ErrorComponent;
      isLoading = false;
    });

  // Show loading state after delay
  timeoutId = setTimeout(() => {
    if (component === null && !error) {
      component = LoadingComponent;
      isLoading = true;
    }
  }, delay);

  return {
    get component() { return component; },
    get isLoading() { return isLoading; },
    get error() { return error; },
  };
}

/**
 * Prefetch a component for faster loading
 * @param {() => Promise<{default: any}>} loader - Dynamic import function
 */
export function prefetchComponent(loader) {
  // Start loading but don't do anything with result
  loader().catch(err => {
    console.warn('Failed to prefetch component:', err);
  });
}

/**
 * Route-based component map for easy lazy loading
 * @param {Object<string, () => Promise<{default: any}>>} routes - Route to loader map
 * @returns {{loadRoute: (name: string) => Promise<any>, prefetchRoute: (name: string) => void}}
 */
export function createRouteLoader(routes) {
  const loaded = new Map();

  return {
    /**
     * Load component for route
     * @param {string} name - Route name
     * @returns {Promise<any>} Component
     */
    async loadRoute(name) {
      if (loaded.has(name)) {
        return loaded.get(name);
      }

      if (!routes[name]) {
        throw new Error(`Route not found: ${name}`);
      }

      try {
        const component = await defineAsyncComponent(routes[name]);
        loaded.set(name, component);
        return component;
      } catch (error) {
        console.error(`Failed to load route ${name}:`, error);
        throw error;
      }
    },

    /**
     * Prefetch component for route
     * @param {string} name - Route name
     */
    prefetchRoute(name) {
      if (loaded.has(name) || !routes[name]) return;
      
      prefetchComponent(routes[name]);
    },

    /**
     * Clear component cache
     * @param {string} name - Route name (optional, clears all if omitted)
     */
    clearCache(name) {
      if (name) {
        loaded.delete(name);
      } else {
        loaded.clear();
      }
    },
  };
}

/**
 * Monitor chunk load performance
 * @returns {{getMetrics: () => Object}} Performance metrics
 */
export function createChunkMonitor() {
  const metrics = new Map();

  if (typeof window !== 'undefined' && 'PerformanceObserver' in window) {
    try {
      const observer = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.name.includes('chunks/') || entry.name.includes('.js')) {
            metrics.set(entry.name, {
              duration: entry.duration,
              size: entry.transferSize,
              encoded: entry.encodedBodySize,
              decoded: entry.decodedBodySize,
              startTime: entry.startTime,
            });
          }
        }
      });

      observer.observe({ entryTypes: ['resource'] });
    } catch (err) {
      console.warn('PerformanceObserver not supported:', err);
    }
  }

  return {
    /**
     * Get all chunk metrics
     * @returns {Object} Metrics map
     */
    getMetrics() {
      return Object.fromEntries(metrics);
    },

    /**
     * Get specific chunk metric
     * @param {string} name - Chunk name
     * @returns {Object|null} Chunk metric
     */
    getMetric(name) {
      return metrics.get(name) || null;
    },

    /**
     * Log all metrics to console
     */
    logMetrics() {
      console.table(Object.fromEntries(metrics));
    },

    /**
     * Calculate total load time for all chunks
     * @returns {number} Total milliseconds
     */
    getTotalLoadTime() {
      return Array.from(metrics.values()).reduce((sum, m) => sum + m.duration, 0);
    },

    /**
     * Calculate total bytes transferred
     * @returns {number} Total bytes
     */
    getTotalBytes() {
      return Array.from(metrics.values()).reduce((sum, m) => sum + m.size, 0);
    },
  };
}

// Export default async component loader
export default defineAsyncComponent;
