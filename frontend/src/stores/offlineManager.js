import { writable, derived } from 'svelte/store';

/**
 * PWA Offline Support Store
 * Manages offline queue, sync status, and data availability
 */
function createOfflineManager() {
  const isOnline = writable(typeof navigator !== 'undefined' ? navigator.onLine : true);
  const isSyncing = writable(false);
  const offlineQueue = writable([]);
  const cacheStatus = writable({
    size: 0,
    items: 0,
    lastSync: null,
  });

  // Listen to online/offline events
  if (typeof window !== 'undefined') {
    window.addEventListener('online', () => {
      isOnline.set(true);
      console.log('[Offline] Network restored - syncing...');
      syncOfflineQueue();
    });

    window.addEventListener('offline', () => {
      isOnline.set(false);
      console.log('[Offline] Network lost - using cache');
    });
  }

  /**
   * Add operation to offline queue
   */
  function queueOperation(operation) {
    offlineQueue.update((queue) => [
      ...queue,
      {
        id: `op-${Date.now()}-${Math.random()}`,
        ...operation,
        timestamp: new Date().toISOString(),
      },
    ]);
  }

  /**
   * Sync offline queue when coming back online
   */
  async function syncOfflineQueue() {
    let $isOnline;
    isOnline.subscribe((v) => ($isOnline = v))();

    if (!$isOnline) {
      console.log('[Offline] Still offline, skipping sync');
      return;
    }

    isSyncing.set(true);

    try {
      let $queue;
      offlineQueue.subscribe((q) => ($queue = q))();

      const successfulOps = [];

      for (const operation of $queue) {
        try {
          console.log('[Offline] Syncing operation:', operation.id);

          const response = await fetch(operation.url, {
            method: operation.method || 'GET',
            headers: operation.headers || {},
            body: operation.body ? JSON.stringify(operation.body) : undefined,
          });

          if (response.ok) {
            successfulOps.push(operation.id);
          } else {
            console.error('[Offline] Sync failed:', operation.id, response.status);
          }
        } catch (error) {
          console.error('[Offline] Sync error:', operation.id, error);
        }
      }

      // Remove successful operations from queue
      offlineQueue.update((queue) =>
        queue.filter((op) => !successfulOps.includes(op.id))
      );

      cacheStatus.update((status) => ({
        ...status,
        lastSync: new Date().toISOString(),
      }));

      console.log('[Offline] Sync complete, successful:', successfulOps.length);
    } catch (error) {
      console.error('[Offline] Sync failed:', error);
    } finally {
      isSyncing.set(false);
    }
  }

  /**
   * Clear offline queue
   */
  function clearQueue() {
    offlineQueue.set([]);
  }

  /**
   * Register service worker
   */
  async function registerServiceWorker() {
    if (!('serviceWorker' in navigator)) {
      console.log('[PWA] Service Worker not supported');
      return;
    }

    try {
      const registration = await navigator.serviceWorker.register('/service-worker.js');
      console.log('[PWA] Service Worker registered:', registration);
      return registration;
    } catch (error) {
      console.error('[PWA] Service Worker registration failed:', error);
    }
  }

  /**
   * Request cache usage
   */
  async function getCacheSize() {
    if (!('storage' in navigator) || !('estimate' in navigator.storage)) {
      return { usage: 0, quota: 0 };
    }

    try {
      const estimate = await navigator.storage.estimate();
      return {
        usage: estimate.usage,
        quota: estimate.quota,
        percentage: Math.round((estimate.usage / estimate.quota) * 100),
      };
    } catch (error) {
      console.error('[PWA] Cache size error:', error);
      return { usage: 0, quota: 0 };
    }
  }

  /**
   * Request persistent storage
   */
  async function requestPersistentStorage() {
    if (!('storage' in navigator) || !('persist' in navigator.storage)) {
      console.log('[PWA] Persistent storage not supported');
      return false;
    }

    try {
      const persistent = await navigator.storage.persist();
      console.log('[PWA] Persistent storage:', persistent);
      return persistent;
    } catch (error) {
      console.error('[PWA] Persistent storage request failed:', error);
      return false;
    }
  }

  /**
   * Derived: Percentage of operations synced
   */
  const syncProgress = derived(offlineQueue, ($queue) => {
    if ($queue.length === 0) return 100;
    return 0; // Could track partial sync progress
  });

  /**
   * Derived: Is offline mode active
   */
  const isOfflineMode = derived(
    [isOnline, offlineQueue],
    ([$isOnline, $queue]) => !$isOnline || $queue.length > 0
  );

  return {
    isOnline,
    isSyncing,
    offlineQueue,
    cacheStatus,
    syncProgress,
    isOfflineMode,
    queueOperation,
    syncOfflineQueue,
    clearQueue,
    registerServiceWorker,
    getCacheSize,
    requestPersistentStorage,
  };
}

export const offlineManager = createOfflineManager();

/**
 * Helper: Make fetch request with offline fallback
 */
export async function fetchWithOfflineFallback(url, options = {}) {
  try {
    const response = await fetch(url, options);
    return response;
  } catch (error) {
    let $isOnline;
    offlineManager.isOnline.subscribe((v) => ($isOnline = v))();

    if (!$isOnline) {
      // If offline, queue for later sync
      offlineManager.queueOperation({
        url,
        ...options,
      });

      // Return cached response if available
      const cache = await caches.match(url);
      if (cache) {
        return cache;
      }

      throw new Error('Offline - No cached data available');
    }

    throw error;
  }
}

/**
 * Helper: Format cache size for display
 */
export function formatCacheSize(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}
