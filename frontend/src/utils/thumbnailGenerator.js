/**
 * Thumbnail generation and caching system
 * Generates image thumbnails for preview in grid view
 */

const THUMBNAIL_SIZE = 200; // Max width/height for thumbnails
const CACHE_NAME = 'syncspace-thumbnails';
const CACHE_EXPIRY_DAYS = 7;

/**
 * Check if file type supports thumbnail generation
 */
export function canGenerateThumbnail(filename) {
  const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp', '.svg'];
  const ext = filename.toLowerCase().slice(filename.lastIndexOf('.'));
  return imageExtensions.includes(ext);
}

/**
 * Open IndexedDB for thumbnail cache
 */
function openDB() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open('SyncSpaceThumbnails', 1);
    
    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(request.result);
    
    request.onupgradeneeded = (event) => {
      const db = event.target.result;
      if (!db.objectStoreNames.contains('thumbnails')) {
        const store = db.createObjectStore('thumbnails', { keyPath: 'path' });
        store.createIndex('timestamp', 'timestamp', { unique: false });
      }
    };
  });
}

/**
 * Get thumbnail from cache
 */
async function getCachedThumbnail(filePath, fileModified) {
  try {
    const db = await openDB();
    const transaction = db.transaction(['thumbnails'], 'readonly');
    const store = transaction.objectStore('thumbnails');
    
    return new Promise((resolve, reject) => {
      const request = store.get(filePath);
      
      request.onsuccess = () => {
        const cached = request.result;
        
        if (!cached) {
          resolve(null);
          return;
        }
        
        // Check if cache is expired or file was modified
        const now = Date.now();
        const cacheAge = now - cached.timestamp;
        const maxAge = CACHE_EXPIRY_DAYS * 24 * 60 * 60 * 1000;
        
        if (cacheAge > maxAge || cached.fileModified !== fileModified) {
          resolve(null);
          return;
        }
        
        resolve(cached.dataUrl);
      };
      
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    console.error('Failed to get cached thumbnail:', err);
    return null;
  }
}

/**
 * Save thumbnail to cache
 */
async function cacheThumbnail(filePath, fileModified, dataUrl) {
  try {
    const db = await openDB();
    const transaction = db.transaction(['thumbnails'], 'readwrite');
    const store = transaction.objectStore('thumbnails');
    
    store.put({
      path: filePath,
      dataUrl,
      fileModified,
      timestamp: Date.now()
    });
    
    return new Promise((resolve, reject) => {
      transaction.oncomplete = () => resolve();
      transaction.onerror = () => reject(transaction.error);
    });
  } catch (err) {
    console.error('Failed to cache thumbnail:', err);
  }
}

/**
 * Generate thumbnail from image URL
 */
function generateThumbnail(imageUrl) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    
    img.onload = () => {
      try {
        // Calculate dimensions maintaining aspect ratio
        let width = img.width;
        let height = img.height;
        
        if (width > height) {
          if (width > THUMBNAIL_SIZE) {
            height = (height * THUMBNAIL_SIZE) / width;
            width = THUMBNAIL_SIZE;
          }
        } else {
          if (height > THUMBNAIL_SIZE) {
            width = (width * THUMBNAIL_SIZE) / height;
            height = THUMBNAIL_SIZE;
          }
        }
        
        // Create canvas and draw scaled image
        const canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        
        const ctx = canvas.getContext('2d');
        ctx.drawImage(img, 0, 0, width, height);
        
        // Convert to data URL
        const dataUrl = canvas.toDataURL('image/jpeg', 0.8);
        resolve(dataUrl);
      } catch (err) {
        reject(err);
      }
    };
    
    img.onerror = () => reject(new Error('Failed to load image'));
    img.src = imageUrl;
  });
}

/**
 * Get or generate thumbnail for file
 * @param {string} filePath - Path to file relative to current directory
 * @param {string} fileModified - File modification timestamp
 * @returns {Promise<string|null>} - Data URL of thumbnail or null
 */
export async function getThumbnail(filePath, fileModified) {
  // Check cache first
  const cached = await getCachedThumbnail(filePath, fileModified);
  if (cached) {
    return cached;
  }
  
  // Generate new thumbnail
  try {
    const imageUrl = `/api/file/${encodeURIComponent(filePath)}`;
    const thumbnail = await generateThumbnail(imageUrl);
    
    // Cache for future use
    await cacheThumbnail(filePath, fileModified, thumbnail);
    
    return thumbnail;
  } catch (err) {
    console.error('Failed to generate thumbnail:', err);
    return null;
  }
}

/**
 * Clear old thumbnails from cache (run on startup)
 */
export async function cleanupCache() {
  try {
    const db = await openDB();
    const transaction = db.transaction(['thumbnails'], 'readwrite');
    const store = transaction.objectStore('thumbnails');
    const index = store.index('timestamp');
    
    const now = Date.now();
    const maxAge = CACHE_EXPIRY_DAYS * 24 * 60 * 60 * 1000;
    const cutoff = now - maxAge;
    
    const request = index.openCursor();
    
    request.onsuccess = (event) => {
      const cursor = event.target.result;
      if (cursor) {
        if (cursor.value.timestamp < cutoff) {
          cursor.delete();
        }
        cursor.continue();
      }
    };
  } catch (err) {
    console.error('Failed to cleanup thumbnail cache:', err);
  }
}
