/**
 * Debounce utility to limit function execution frequency
 * @param {Function} func - Function to debounce
 * @param {number} wait - Delay in milliseconds
 * @returns {Function} Debounced function
 */
export function debounce(func, wait) {
  let timeout;
  
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    
    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}

/**
 * Throttle utility to ensure function is called at most once per interval
 * @param {Function} func - Function to throttle
 * @param {number} limit - Minimum time between calls in milliseconds
 * @returns {Function} Throttled function
 */
export function throttle(func, limit) {
  let inThrottle;
  
  return function(...args) {
    if (!inThrottle) {
      func.apply(this, args);
      inThrottle = true;
      setTimeout(() => inThrottle = false, limit);
    }
  };
}

/**
 * Creates a request queue to prevent parallel execution
 * Useful for upload/download operations
 */
export class RequestQueue {
  constructor(concurrency = 1) {
    this.concurrency = concurrency;
    this.running = 0;
    this.queue = [];
  }

  /**
   * Add a request to the queue
   * @param {Function} requestFn - Async function that returns a promise
   * @returns {Promise} Promise that resolves when request completes
   */
  async add(requestFn) {
    while (this.running >= this.concurrency) {
      await new Promise(resolve => {
        const checkSlot = () => {
          if (this.running < this.concurrency) {
            resolve();
          } else {
            setTimeout(checkSlot, 50);
          }
        };
        checkSlot();
      });
    }

    this.running++;
    
    try {
      const result = await requestFn();
      return result;
    } finally {
      this.running--;
    }
  }

  /**
   * Get current queue status
   */
  getStatus() {
    return {
      running: this.running,
      waiting: this.queue.length,
      total: this.running + this.queue.length
    };
  }

  /**
   * Clear all pending requests
   */
  clear() {
    this.queue = [];
  }
}

/**
 * Exponential backoff retry logic
 * @param {Function} fn - Async function to retry
 * @param {Object} options - Retry options
 * @returns {Promise} Result of successful execution
 */
export async function retryWithBackoff(fn, options = {}) {
  const {
    maxRetries = 3,
    initialDelay = 1000,
    maxDelay = 10000,
    backoffFactor = 2,
    shouldRetry = (error) => true
  } = options;

  let lastError;
  
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await fn();
    } catch (error) {
      lastError = error;
      
      // Don't retry if we've exhausted attempts or if error shouldn't be retried
      if (attempt >= maxRetries || !shouldRetry(error)) {
        throw error;
      }

      // Calculate delay with exponential backoff
      const delay = Math.min(
        initialDelay * Math.pow(backoffFactor, attempt),
        maxDelay
      );

      console.warn(`Retry attempt ${attempt + 1}/${maxRetries} after ${delay}ms`, error);
      
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }

  throw lastError;
}

/**
 * AbortController wrapper for cancellable requests
 */
export class CancellableRequest {
  constructor() {
    this.controller = new AbortController();
  }

  /**
   * Get the abort signal for fetch requests
   */
  get signal() {
    return this.controller.signal;
  }

  /**
   * Cancel the request
   */
  cancel() {
    this.controller.abort();
  }

  /**
   * Check if request was cancelled
   */
  get isCancelled() {
    return this.controller.signal.aborted;
  }

  /**
   * Execute a fetch request with cancellation support
   * @param {string} url - Request URL
   * @param {Object} options - Fetch options
   * @returns {Promise} Fetch promise
   */
  async fetch(url, options = {}) {
    return fetch(url, {
      ...options,
      signal: this.signal
    });
  }
}
