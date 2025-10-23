/**
 * Thumbnail generation and caching system
 * Generates image thumbnails for preview in grid view
 */

const THUMBNAIL_SIZE = 200; // Max width/height for thumbnails
const CACHE_NAME = 'syncspace-thumbnails';
const CACHE_EXPIRY_DAYS = 7;
const MAX_CACHE_SIZE_MB = 50; // Maximum cache size in megabytes
const MAX_CACHE_SIZE_BYTES = MAX_CACHE_SIZE_MB * 1024 * 1024;

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
        const store = db.createObjectStore('thumbnails', { keyPath: 'fileId' }); // Changed from 'path' to 'fileId'
        store.createIndex('timestamp', 'timestamp', { unique: false });
        store.createIndex('size', 'size', { unique: false }); // Index by size for cache management
      }
    };
  });
}

/**
 * Get thumbnail from cache
 * @param {string} fileId - Unique file ID (or path as fallback)
 * @param {string} fileModified - File modification timestamp
 */
async function getCachedThumbnail(fileId, fileModified) {
  try {
    const db = await openDB();
    const transaction = db.transaction(['thumbnails'], 'readonly');
    const store = transaction.objectStore('thumbnails');
    
    return new Promise((resolve, reject) => {
      const request = store.get(fileId);
      
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
 * Enforce cache size limit by deleting oldest entries
 * @param {IDBDatabase} db - IndexedDB database instance
 * @param {number} newEntrySize - Size of new entry to be added
 */
async function enforceCacheSizeLimit(db, newEntrySize) {
  try {
    const transaction = db.transaction(['thumbnails'], 'readonly');
    const store = transaction.objectStore('thumbnails');
    
    return new Promise((resolve, reject) => {
      const request = store.getAll();
      
      request.onsuccess = async () => {
        const allEntries = request.result;
        
        // Calculate total cache size
        const totalSize = allEntries.reduce((sum, entry) => sum + (entry.size || 0), 0);
        
        // If adding new entry would exceed limit, delete oldest entries
        if (totalSize + newEntrySize > MAX_CACHE_SIZE_BYTES) {
          // Sort by timestamp (oldest first)
          allEntries.sort((a, b) => a.timestamp - b.timestamp);
          
          let freedSpace = 0;
          const toDelete = [];
          
          // Delete oldest entries until we have enough space
          for (const entry of allEntries) {
            toDelete.push(entry.fileId);
            freedSpace += entry.size || 0;
            
            if (totalSize - freedSpace + newEntrySize <= MAX_CACHE_SIZE_BYTES) {
              break;
            }
          }
          
          // Delete entries
          if (toDelete.length > 0) {
            const deleteTransaction = db.transaction(['thumbnails'], 'readwrite');
            const deleteStore = deleteTransaction.objectStore('thumbnails');
            
            for (const fileId of toDelete) {
              deleteStore.delete(fileId);
            }
            
            await new Promise((res, rej) => {
              deleteTransaction.oncomplete = () => res();
              deleteTransaction.onerror = () => rej(deleteTransaction.error);
            });
            
            console.log(`Cleaned up ${toDelete.length} thumbnails to enforce ${MAX_CACHE_SIZE_MB}MB limit`);
          }
        }
        
        resolve();
      };
      
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    console.error('Failed to enforce cache size limit:', err);
  }
}

/**
 * Save thumbnail to cache with size management
 * @param {string} fileId - Unique file ID
 * @param {string} filePath - File path (for fetching)
 * @param {string} fileModified - File modification timestamp
 * @param {string} dataUrl - Thumbnail data URL
 */
async function cacheThumbnail(fileId, filePath, fileModified, dataUrl) {
  try {
    const db = await openDB();
    
    // Calculate data URL size in bytes
    const sizeBytes = new Blob([dataUrl]).size;
    
    // Check total cache size and cleanup if needed
    await enforceCacheSizeLimit(db, sizeBytes);
    
    const transaction = db.transaction(['thumbnails'], 'readwrite');
    const store = transaction.objectStore('thumbnails');
    
    store.put({
      fileId,
      filePath, // Keep for reference
      dataUrl,
      fileModified,
      timestamp: Date.now(),
      size: sizeBytes
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
 * @param {Object} file - File object with id, path, name, modified properties
 * @param {string} file.id - Unique file ID (preferred) or path as fallback
 * @param {string} file.path - File path for fetching
 * @param {string} file.name - File name
 * @param {string} file.modified - File modification timestamp
 * @returns {Promise<string|null>} - Data URL of thumbnail or null
 */
export async function getThumbnail(file) {
  const fileId = file.id || file.path || file.name; // Use ID if available, fallback to path
  const filePath = file.path ? `${file.path}/${file.name}` : file.name;
  const fileModified = file.modified;
  
  // Check cache first
  const cached = await getCachedThumbnail(fileId, fileModified);
  if (cached) {
    return cached;
  }
  
  // Generate new thumbnail
  try {
    const imageUrl = `/api/file/${encodeURIComponent(filePath)}`;
    const thumbnail = await generateThumbnail(imageUrl);
    
    // Cache for future use with file ID as key
    await cacheThumbnail(fileId, filePath, fileModified, thumbnail);
    
    return thumbnail;
  } catch (err) {
    console.error('Failed to generate thumbnail:', err);
    return null;
  }
}

/**
 * Invalidate thumbnail cache for specific file(s)
 * Call this after file operations (upload, delete, rename, move)
 * @param {string|string[]} fileIds - File ID(s) to invalidate
 */
export async function invalidateThumbnails(fileIds) {
  try {
    const db = await openDB();
    const transaction = db.transaction(['thumbnails'], 'readwrite');
    const store = transaction.objectStore('thumbnails');
    
    const ids = Array.isArray(fileIds) ? fileIds : [fileIds];
    
    for (const fileId of ids) {
      store.delete(fileId);
    }
    
    return new Promise((resolve, reject) => {
      transaction.oncomplete = () => {
        console.log(`Invalidated ${ids.length} thumbnail(s)`);
        resolve();
      };
      transaction.onerror = () => reject(transaction.error);
    });
  } catch (err) {
    console.error('Failed to invalidate thumbnails:', err);
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
