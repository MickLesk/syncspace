/**
 * Route Prefetching System
 * Intelligently prefetch routes when user hovers/focuses navigation items
 * 
 * Usage:
 * import { setupPrefetching } from './lib/prefetch';
 * setupPrefetching();
 * 
 * In components:
 * <button onmouseover={() => prefetchRoute('settings')}>Settings</button>
 */

import { prefetchComponent, createRouteLoader } from './asyncComponent.js';

// Define all routes for prefetching
const routeMap = {
  'files': () => import('../pages/files/FilesView.svelte'),
  'search': () => import('../pages/search/SearchView.svelte'),
  'shared': () => import('../pages/files/SharedView.svelte'),
  'recent': () => import('../pages/files/RecentFilesView.svelte'),
  'trash': () => import('../pages/trash/TrashView.svelte'),
  'settings': () => import('../pages/settings/SettingsView.svelte'),
  'profile': () => import('../pages/user/UserProfileView.svelte'),
  'user-settings': () => import('../pages/user/UserSettingsView.svelte'),
  'security': () => import('../pages/user/SecurityView.svelte'),
  'activity': () => import('../pages/system/ActivityView.svelte'),
  'notifications': () => import('../pages/system/NotificationsView.svelte'),
  'backup': () => import('../pages/system/BackupView.svelte'),
  'storage': () => import('../pages/system/StorageView.svelte'),
  'users': () => import('../pages/system/UsersView.svelte'),
  'jobs': () => import('../pages/JobsDashboard.svelte'),
  'duplicates': () => import('../pages/tools/DuplicatesView.svelte'),
};

// Create route loader instance
export const routeLoader = createRouteLoader(routeMap);

// Track prefetch requests to avoid duplicates
const prefetchQueue = new Set();
let prefetchTimeoutId = null;

/**
 * Prefetch a route (debounced)
 * @param {string} view - Route name
 * @param {number} delay - Debounce delay (ms)
 */
export function prefetchRoute(view, delay = 50) {
  if (!routeMap[view] || prefetchQueue.has(view)) {
    return;
  }

  prefetchQueue.add(view);

  // Debounce prefetch to avoid too many concurrent requests
  clearTimeout(prefetchTimeoutId);
  prefetchTimeoutId = setTimeout(() => {
    try {
      routeLoader.prefetchRoute(view);
      console.debug(`[Prefetch] Started loading ${view}`);
    } catch (error) {
      console.warn(`[Prefetch] Failed to prefetch ${view}:`, error);
    } finally {
      prefetchQueue.delete(view);
    }
  }, delay);
}

/**
 * Batch prefetch multiple routes
 * @param {string[]} views - Route names to prefetch
 */
export function prefetchRoutes(views) {
  views.forEach((view, index) => {
    // Stagger prefetch requests
    setTimeout(() => prefetchRoute(view), index * 100);
  });
}

/**
 * Prefetch nearby routes based on current view
 * @param {string} currentView - Current route name
 */
export function prefetchNearbyRoutes(currentView) {
  // Define route groups for smart prefetching
  const routeGroups = {
    'files': ['shared', 'recent', 'trash'],
    'search': ['files', 'activity'],
    'settings': ['profile', 'security'],
    'activity': ['notifications', 'backup'],
    'storage': ['backup', 'duplicates'],
  };

  const nearby = routeGroups[currentView] || [];
  prefetchRoutes(nearby);
}

/**
 * Setup automatic prefetching on navigation element hover
 * Call this once on app init
 */
export function setupPrefetching() {
  if (typeof window === 'undefined') return;

  // Add prefetch listeners to all nav items
  const navItems = document.querySelectorAll('[data-prefetch]');
  navItems.forEach(item => {
    const view = item.getAttribute('data-prefetch');
    
    // Prefetch on hover
    item.addEventListener('mouseenter', () => prefetchRoute(view));
    
    // Prefetch on focus for keyboard navigation
    item.addEventListener('focus', () => prefetchRoute(view));
    
    // Immediate prefetch on iOS (no hover support)
    if (/iPhone|iPad|iPod/.test(navigator.userAgent)) {
      item.addEventListener('touchstart', () => prefetchRoute(view), { passive: true });
    }
  });

  // Prefetch critical routes on idle
  if ('requestIdleCallback' in window) {
    requestIdleCallback(() => {
      prefetchRoutes(['files', 'search', 'settings']);
    }, { timeout: 2000 });
  } else {
    // Fallback for browsers without requestIdleCallback
    setTimeout(() => {
      prefetchRoutes(['files', 'search', 'settings']);
    }, 3000);
  }
}

/**
 * Get prefetch statistics
 * @returns {Object} Statistics
 */
export function getPrefetchStats() {
  return {
    queueSize: prefetchQueue.size,
    totalRoutes: Object.keys(routeMap).length,
    queuedRoutes: Array.from(prefetchQueue),
  };
}

/**
 * Clear prefetch queue
 */
export function clearPrefetchQueue() {
  prefetchQueue.clear();
  if (prefetchTimeoutId) {
    clearTimeout(prefetchTimeoutId);
  }
}

// Export prefetch component wrapper for use in templates
export function createPrefetchButton(view) {
  return {
    onmouseover: () => prefetchRoute(view),
    onfocus: () => prefetchRoute(view),
  };
}

export default {
  prefetchRoute,
  prefetchRoutes,
  prefetchNearbyRoutes,
  setupPrefetching,
  getPrefetchStats,
  clearPrefetchQueue,
  routeLoader,
};
