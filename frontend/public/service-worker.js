// service-worker.js - SyncSpace Service Worker
// Provides offline functionality, caching, and background sync

const CACHE_NAME = 'syncspace-v1';
const API_CACHE = 'syncspace-api-v1';
const IMAGE_CACHE = 'syncspace-images-v1';
const OFFLINE_URL = '/offline.html';

// URLs to cache on install
const PRECACHE_URLS = [
  '/',
  '/index.html',
  '/manifest.json',
  '/favicon.ico',
  '/offline.html',
];

// API endpoints to cache
const API_CACHE_PATTERNS = [
  /\/api\/files\/list/,
  /\/api\/files\/\d+/,
  /\/api\/search/,
  /\/api\/users\/me/,
];

/**
 * Install event - cache static assets
 */
self.addEventListener('install', (event) => {
  console.log('[ServiceWorker] Installing...');

  event.waitUntil(
    caches
      .open(CACHE_NAME)
      .then((cache) => {
        console.log('[ServiceWorker] Caching app shell');
        return cache.addAll(PRECACHE_URLS);
      })
      .then(() => self.skipWaiting())
  );
});

/**
 * Activate event - clean up old caches
 */
self.addEventListener('activate', (event) => {
  console.log('[ServiceWorker] Activating...');

  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (cacheName !== CACHE_NAME && cacheName !== API_CACHE && cacheName !== IMAGE_CACHE) {
            console.log('[ServiceWorker] Deleting cache:', cacheName);
            return caches.delete(cacheName);
          }
        })
      );
    })
  );

  return self.clients.claim();
});

/**
 * Fetch event - network first with offline fallback
 */
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }

  // Skip cross-origin requests
  if (url.origin !== location.origin) {
    return;
  }

  // API requests - Network first, fallback to cache
  if (url.pathname.startsWith('/api/')) {
    event.respondWith(networkFirstStrategy(request, API_CACHE));
    return;
  }

  // Image requests - Cache first, fallback to network
  if (request.destination === 'image') {
    event.respondWith(cacheFirstStrategy(request, IMAGE_CACHE));
    return;
  }

  // Other requests - Cache first
  event.respondWith(cacheFirstStrategy(request, CACHE_NAME));
});

/**
 * Network first strategy - try network, fallback to cache
 */
async function networkFirstStrategy(request, cacheName) {
  try {
    const response = await fetch(request);

    // Cache successful responses
    if (response.status === 200) {
      const cache = await caches.open(cacheName);
      cache.put(request, response.clone());
    }

    return response;
  } catch (error) {
    console.log('[ServiceWorker] Network request failed, trying cache');

    const cached = await caches.match(request);
    if (cached) {
      return cached;
    }

    // Return offline page for document requests
    if (request.destination === 'document') {
      return caches.match(OFFLINE_URL);
    }

    // Return error response
    return new Response('Offline - Resource not available', {
      status: 503,
      statusText: 'Service Unavailable',
      headers: new Headers({
        'Content-Type': 'text/plain',
      }),
    });
  }
}

/**
 * Cache first strategy - use cache, fallback to network
 */
async function cacheFirstStrategy(request, cacheName) {
  const cached = await caches.match(request);

  if (cached) {
    return cached;
  }

  try {
    const response = await fetch(request);

    // Cache successful responses
    if (response.status === 200) {
      const cache = await caches.open(cacheName);
      cache.put(request, response.clone());
    }

    return response;
  } catch (error) {
    console.log('[ServiceWorker] Network request failed for:', request.url);

    // Return offline page for document requests
    if (request.destination === 'document') {
      return caches.match(OFFLINE_URL);
    }

    // Return error response
    return new Response('Offline - Resource not available', {
      status: 503,
      statusText: 'Service Unavailable',
      headers: new Headers({
        'Content-Type': 'text/plain',
      }),
    });
  }
}

/**
 * Background sync event - sync when online
 */
self.addEventListener('sync', (event) => {
  console.log('[ServiceWorker] Background sync:', event.tag);

  if (event.tag === 'syncFiles') {
    event.waitUntil(syncFiles());
  } else if (event.tag === 'syncUploads') {
    event.waitUntil(syncUploads());
  }
});

/**
 * Sync file operations when coming back online
 */
async function syncFiles() {
  try {
    const db = await openDB();
    const pendingOperations = await getPendingOperations(db);

    for (const operation of pendingOperations) {
      try {
        const response = await fetch(operation.url, {
          method: operation.method,
          headers: operation.headers,
          body: operation.body ? JSON.stringify(operation.body) : undefined,
        });

        if (response.ok) {
          await markOperationComplete(db, operation.id);
        } else {
          console.error('[ServiceWorker] Operation failed:', operation.id, response.status);
        }
      } catch (error) {
        console.error('[ServiceWorker] Sync error:', error);
      }
    }

    // Notify clients that sync is complete
    self.clients.matchAll().then((clients) => {
      clients.forEach((client) => {
        client.postMessage({
          type: 'SYNC_COMPLETE',
          data: { operationsCount: pendingOperations.length },
        });
      });
    });
  } catch (error) {
    console.error('[ServiceWorker] Sync failed:', error);
  }
}

/**
 * Sync pending uploads
 */
async function syncUploads() {
  try {
    const db = await openDB();
    const pendingUploads = await getPendingUploads(db);

    for (const upload of pendingUploads) {
      try {
        const formData = new FormData();
        formData.append('file', upload.file);

        const response = await fetch(`/api/upload/${upload.path}`, {
          method: 'POST',
          body: formData,
        });

        if (response.ok) {
          await markUploadComplete(db, upload.id);
        }
      } catch (error) {
        console.error('[ServiceWorker] Upload sync error:', error);
      }
    }
  } catch (error) {
    console.error('[ServiceWorker] Upload sync failed:', error);
  }
}

/**
 * Push notification event
 */
self.addEventListener('push', (event) => {
  const data = event.data ? event.data.json() : {};

  const options = {
    body: data.body || 'New notification',
    icon: '/icon-192x192.png',
    badge: '/badge-72x72.png',
    tag: data.tag || 'notification',
    requireInteraction: data.requireInteraction || false,
    actions: data.actions || [],
  };

  event.waitUntil(self.registration.showNotification(data.title || 'SyncSpace', options));
});

/**
 * Notification click event
 */
self.addEventListener('notificationclick', (event) => {
  event.notification.close();

  const urlToOpen = event.notification.data?.url || '/';

  event.waitUntil(
    clients.matchAll({ type: 'window', includeUncontrolled: true }).then((windowClients) => {
      // Check if app is already open
      for (let i = 0; i < windowClients.length; i++) {
        const client = windowClients[i];
        if (client.url === urlToOpen && 'focus' in client) {
          return client.focus();
        }
      }
      // If not open, open new window
      if (clients.openWindow) {
        return clients.openWindow(urlToOpen);
      }
    })
  );
});

// Helper functions (mock - use actual IDB in production)

async function openDB() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open('SyncSpace', 1);

    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(request.result);

    request.onupgradeneeded = (event) => {
      const db = event.target.result;
      if (!db.objectStoreNames.contains('pendingOperations')) {
        db.createObjectStore('pendingOperations', { keyPath: 'id' });
      }
      if (!db.objectStoreNames.contains('pendingUploads')) {
        db.createObjectStore('pendingUploads', { keyPath: 'id' });
      }
    };
  });
}

async function getPendingOperations(db) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction('pendingOperations', 'readonly');
    const store = tx.objectStore('pendingOperations');
    const request = store.getAll();

    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(request.result);
  });
}

async function getPendingUploads(db) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction('pendingUploads', 'readonly');
    const store = tx.objectStore('pendingUploads');
    const request = store.getAll();

    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(request.result);
  });
}

async function markOperationComplete(db, operationId) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction('pendingOperations', 'readwrite');
    const store = tx.objectStore('pendingOperations');
    const request = store.delete(operationId);

    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve();
  });
}

async function markUploadComplete(db, uploadId) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction('pendingUploads', 'readwrite');
    const store = tx.objectStore('pendingUploads');
    const request = store.delete(uploadId);

    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve();
  });
}
