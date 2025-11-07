/**
 * Frontend Error Reporting to Backend
 * Sends client-side errors to backend for centralized logging
 */

const ERROR_REPORT_ENDPOINT = 'http://localhost:8080/api/errors/report';
const MAX_STACK_LENGTH = 2000; // Limit stack trace size
const REPORT_DEBOUNCE_MS = 1000; // Prevent spam

let pendingErrors = [];
let reportTimer = null;

/**
 * Report an error to the backend
 * @param {Error|string} error - The error to report
 * @param {Object} context - Additional context about the error
 */
export function reportError(error, context = {}) {
  // Don't report in development unless explicitly enabled
  if (import.meta.env.DEV && !import.meta.env.VITE_REPORT_ERRORS) {
    console.warn('🐛 [Dev] Error would be reported:', error, context);
    return;
  }

  const errorReport = {
    message: error?.message || String(error),
    stack: error?.stack?.substring(0, MAX_STACK_LENGTH),
    type: error?.name || 'Error',
    url: window.location.href,
    userAgent: navigator.userAgent,
    timestamp: new Date().toISOString(),
    context: {
      ...context,
      viewport: {
        width: window.innerWidth,
        height: window.innerHeight,
      },
      online: navigator.onLine,
    },
  };

  pendingErrors.push(errorReport);

  // Debounce reporting to batch multiple errors
  if (reportTimer) {
    clearTimeout(reportTimer);
  }

  reportTimer = setTimeout(() => {
    sendErrorBatch();
  }, REPORT_DEBOUNCE_MS);
}

/**
 * Send batched errors to backend
 */
async function sendErrorBatch() {
  if (pendingErrors.length === 0) return;

  const errors = [...pendingErrors];
  pendingErrors = [];

  try {
    const token = localStorage.getItem('authToken');
    const headers = {
      'Content-Type': 'application/json',
    };

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await fetch(ERROR_REPORT_ENDPOINT, {
      method: 'POST',
      headers,
      body: JSON.stringify({ errors }),
    });

    if (!response.ok) {
      console.warn('Failed to report errors to backend:', response.status);
    }
  } catch (err) {
    console.error('Error reporting failed:', err);
    // Don't retry to avoid infinite loops
  }
}

/**
 * Initialize global error handlers
 */
export function initErrorReporting() {
  // Catch unhandled errors
  window.addEventListener('error', (event) => {
    reportError(event.error || event.message, {
      type: 'unhandled_error',
      filename: event.filename,
      lineno: event.lineno,
      colno: event.colno,
    });
  });

  // Catch unhandled promise rejections
  window.addEventListener('unhandledrejection', (event) => {
    reportError(event.reason, {
      type: 'unhandled_rejection',
      promise: 'Promise rejection',
    });
  });

  // Report before page unload if there are pending errors
  window.addEventListener('beforeunload', () => {
    if (pendingErrors.length > 0) {
      // Use sendBeacon for reliable sending during page unload
      const token = localStorage.getItem('authToken');
      const data = JSON.stringify({ errors: pendingErrors });
      
      if (navigator.sendBeacon) {
        const blob = new Blob([data], { type: 'application/json' });
        navigator.sendBeacon(ERROR_REPORT_ENDPOINT, blob);
      }
    }
  });

  console.log('✅ Error reporting initialized');
}

/**
 * Manually report a caught error with context
 * @param {Error} error 
 * @param {Object} context 
 */
export function logError(error, context = {}) {
  console.error('🔴 Error:', error, context);
  reportError(error, { ...context, handled: true });
}

/**
 * Report a warning (non-critical error)
 * @param {string} message 
 * @param {Object} context 
 */
export function logWarning(message, context = {}) {
  console.warn('⚠️ Warning:', message, context);
  reportError(new Error(message), { ...context, severity: 'warning' });
}

/**
 * Report performance issues
 * @param {string} metric 
 * @param {number} value 
 * @param {Object} context 
 */
export function reportPerformance(metric, value, context = {}) {
  // Only report slow operations
  const thresholds = {
    'page_load': 3000,
    'api_call': 2000,
    'render': 100,
  };

  if (value > (thresholds[metric] || 1000)) {
    reportError(new Error(`Slow ${metric}: ${value}ms`), {
      ...context,
      type: 'performance',
      metric,
      value,
    });
  }
}
