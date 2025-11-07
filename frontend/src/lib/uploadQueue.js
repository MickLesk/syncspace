/**
 * Advanced Upload Queue Manager
 * 
 * Features:
 * - Chunked uploads for large files (>100MB)
 * - Upload queue with priorities (high/normal/low)
 * - Pause/Resume functionality
 * - Persistent upload state (localStorage)
 * - Network status detection (online/offline)
 * - Automatic retry with exponential backoff
 * - Upload progress tracking
 * - Concurrent upload limits
 * 
 * Usage:
 * ```javascript
 * import { uploadQueueManager } from './lib/uploadQueue';
 * 
 * // Add files to queue
 * uploadQueueManager.addFile(file, '/uploads', 'high');
 * 
 * // Start uploads
 * uploadQueueManager.start();
 * 
 * // Subscribe to progress
 * uploadQueueManager.onProgress((status) => {
 *   console.log('Upload progress:', status);
 * });
 * ```
 */

import { writable, derived, get } from 'svelte/store';
import { api } from './api';
import { success, warning, error as toastError } from '../stores/toast';

// Upload priorities
export const PRIORITY = {
  HIGH: 3,
  NORMAL: 2,
  LOW: 1
};

// Upload status
export const UPLOAD_STATUS = {
  QUEUED: 'queued',
  UPLOADING: 'uploading',
  PAUSED: 'paused',
  COMPLETED: 'completed',
  FAILED: 'failed',
  CANCELLED: 'cancelled'
};

// Upload queue store
export const uploadQueue = writable([]);
export const isOnline = writable(navigator.onLine);
export const uploadStats = writable({
  totalFiles: 0,
  completedFiles: 0,
  failedFiles: 0,
  totalBytes: 0,
  uploadedBytes: 0
});

// Derived stores
export const activeUploads = derived(uploadQueue, $queue =>
  $queue.filter(u => u.status === UPLOAD_STATUS.UPLOADING)
);

export const queuedUploads = derived(uploadQueue, $queue =>
  $queue.filter(u => u.status === UPLOAD_STATUS.QUEUED)
);

export const completedUploads = derived(uploadQueue, $queue =>
  $queue.filter(u => u.status === UPLOAD_STATUS.COMPLETED)
);

export const failedUploads = derived(uploadQueue, $queue =>
  $queue.filter(u => u.status === UPLOAD_STATUS.FAILED)
);

// Constants
const CHUNK_SIZE = 5 * 1024 * 1024; // 5MB chunks
const LARGE_FILE_THRESHOLD = 100 * 1024 * 1024; // 100MB
const MAX_CONCURRENT_UPLOADS = 3;
const MAX_RETRIES = 5;
const RETRY_BASE_DELAY = 1000; // 1 second
const RETRY_MAX_DELAY = 30000; // 30 seconds
const STORAGE_KEY = 'syncspace_upload_queue';

class UploadQueueManager {
  constructor() {
    this.queue = [];
    this.activeUploads = new Map();
    this.maxConcurrent = MAX_CONCURRENT_UPLOADS;
    this.paused = false;
    this.progressHandlers = new Set();
    
    // Load persisted queue from localStorage
    this.loadQueue();
    
    // Setup network status listeners
    this.setupNetworkListeners();
    
    // Auto-save queue on changes
    uploadQueue.subscribe(queue => {
      this.queue = queue;
      this.saveQueue();
    });
  }

  // Network status monitoring
  setupNetworkListeners() {
    window.addEventListener('online', () => {
      console.log('🌐 Network online, resuming uploads');
      isOnline.set(true);
      warning('Connection restored, resuming uploads...', 3000);
      this.resumeAll();
    });

    window.addEventListener('offline', () => {
      console.log('🌐 Network offline, pausing uploads');
      isOnline.set(false);
      warning('Connection lost, uploads paused', 5000);
      this.pauseAll();
    });
  }

  // Persistence methods
  saveQueue() {
    try {
      const persistData = this.queue
        .filter(upload => upload.status !== UPLOAD_STATUS.COMPLETED)
        .map(upload => ({
          id: upload.id,
          fileName: upload.fileName,
          path: upload.path,
          size: upload.size,
          priority: upload.priority,
          status: upload.status,
          uploadedBytes: upload.uploadedBytes,
          uploadedChunks: upload.uploadedChunks,
          retries: upload.retries
        }));

      localStorage.setItem(STORAGE_KEY, JSON.stringify(persistData));
    } catch (err) {
      console.error('Failed to save upload queue:', err);
    }
  }

  loadQueue() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (!stored) return;

      const persistData = JSON.parse(stored);
      
      // Only load queued/paused uploads (not completed/failed)
      const restoredUploads = persistData
        .filter(u => [UPLOAD_STATUS.QUEUED, UPLOAD_STATUS.PAUSED].includes(u.status))
        .map(u => ({
          ...u,
          file: null, // File objects can't be persisted
          progress: 0,
          error: null,
          chunks: [],
          currentChunk: 0
        }));

      if (restoredUploads.length > 0) {
        console.log(`📦 Restored ${restoredUploads.length} uploads from storage`);
        uploadQueue.set(restoredUploads);
      }
    } catch (err) {
      console.error('Failed to load upload queue:', err);
      localStorage.removeItem(STORAGE_KEY);
    }
  }

  clearPersistedQueue() {
    localStorage.removeItem(STORAGE_KEY);
  }

  // Add file to queue
  addFile(file, targetPath, priority = PRIORITY.NORMAL) {
    const uploadId = `upload_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    
    const upload = {
      id: uploadId,
      file: file,
      fileName: file.name,
      path: targetPath,
      size: file.size,
      priority: priority,
      status: UPLOAD_STATUS.QUEUED,
      progress: 0,
      uploadedBytes: 0,
      retries: 0,
      error: null,
      createdAt: Date.now(),
      startedAt: null,
      completedAt: null,
      
      // Chunked upload state
      isChunked: file.size > LARGE_FILE_THRESHOLD,
      chunks: [],
      totalChunks: 0,
      currentChunk: 0,
      uploadedChunks: []
    };

    // Calculate chunks if file is large
    if (upload.isChunked) {
      upload.totalChunks = Math.ceil(file.size / CHUNK_SIZE);
      console.log(`📦 File ${file.name} will be uploaded in ${upload.totalChunks} chunks`);
    }

    uploadQueue.update(queue => [...queue, upload]);
    
    // Update stats
    uploadStats.update(stats => ({
      ...stats,
      totalFiles: stats.totalFiles + 1,
      totalBytes: stats.totalBytes + file.size
    }));

    console.log(`➕ Added ${file.name} to upload queue (priority: ${priority})`);
    
    // Auto-start if not paused
    if (!this.paused && get(activeUploads).length < this.maxConcurrent) {
      this.processQueue();
    }

    return uploadId;
  }

  // Add multiple files
  addFiles(files, targetPath, priority = PRIORITY.NORMAL) {
    const uploadIds = [];
    for (const file of files) {
      uploadIds.push(this.addFile(file, targetPath, priority));
    }
    return uploadIds;
  }

  // Process upload queue
  async processQueue() {
    if (this.paused || !navigator.onLine) {
      console.log('⏸️ Upload queue paused or offline');
      return;
    }

    const queue = get(uploadQueue);
    const active = queue.filter(u => u.status === UPLOAD_STATUS.UPLOADING);

    // Start new uploads if below concurrent limit
    const available = this.maxConcurrent - active.length;
    if (available <= 0) return;

    // Get next queued uploads (sorted by priority)
    const queued = queue
      .filter(u => u.status === UPLOAD_STATUS.QUEUED)
      .sort((a, b) => b.priority - a.priority)
      .slice(0, available);

    for (const upload of queued) {
      this.startUpload(upload.id);
    }
  }

  // Start individual upload
  async startUpload(uploadId) {
    const upload = this.getUpload(uploadId);
    if (!upload || !upload.file) {
      console.error(`Upload ${uploadId} not found or file missing`);
      return;
    }

    // Mark as uploading
    this.updateUpload(uploadId, {
      status: UPLOAD_STATUS.UPLOADING,
      startedAt: Date.now()
    });

    try {
      if (upload.isChunked) {
        await this.uploadChunked(upload);
      } else {
        await this.uploadStandard(upload);
      }

      // Mark as completed
      this.updateUpload(uploadId, {
        status: UPLOAD_STATUS.COMPLETED,
        progress: 100,
        completedAt: Date.now()
      });

      uploadStats.update(stats => ({
        ...stats,
        completedFiles: stats.completedFiles + 1
      }));

      success(`Uploaded ${upload.fileName}`, 3000);

      // Process next in queue
      this.processQueue();

    } catch (err) {
      console.error(`Upload failed for ${upload.fileName}:`, err);
      
      // Retry logic
      if (upload.retries < MAX_RETRIES && this.isRetryableError(err)) {
        await this.retryUpload(uploadId);
      } else {
        this.updateUpload(uploadId, {
          status: UPLOAD_STATUS.FAILED,
          error: err.message || 'Upload failed'
        });

        uploadStats.update(stats => ({
          ...stats,
          failedFiles: stats.failedFiles + 1
        }));

        toastError(`Failed to upload ${upload.fileName}: ${err.message}`, 5000);
        
        // Process next in queue
        this.processQueue();
      }
    }
  }

  // Standard upload (for files <100MB)
  async uploadStandard(upload) {
    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest();
      const formData = new FormData();
      formData.append('file', upload.file);

      // Progress tracking
      xhr.upload.addEventListener('progress', (e) => {
        if (e.lengthComputable) {
          const progress = Math.round((e.loaded / e.total) * 100);
          this.updateUpload(upload.id, {
            progress,
            uploadedBytes: e.loaded
          });

          uploadStats.update(stats => ({
            ...stats,
            uploadedBytes: stats.uploadedBytes + (e.loaded - upload.uploadedBytes)
          }));

          this.notifyProgress(upload.id, progress);
        }
      });

      xhr.addEventListener('load', () => {
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve();
        } else {
          reject(new Error(`HTTP ${xhr.status}: ${xhr.statusText}`));
        }
      });

      xhr.addEventListener('error', () => {
        reject(new Error('Network error'));
      });

      xhr.addEventListener('abort', () => {
        reject(new Error('Upload cancelled'));
      });

      // Get auth token
      const token = localStorage.getItem('authToken');
      xhr.open('POST', `http://localhost:8080/api/upload${upload.path}`);
      if (token) {
        xhr.setRequestHeader('Authorization', `Bearer ${token}`);
      }

      xhr.send(formData);
      
      // Store xhr for pause/cancel functionality
      this.activeUploads.set(upload.id, xhr);
    });
  }

  // Chunked upload (for files >100MB)
  async uploadChunked(upload) {
    console.log(`📦 Starting chunked upload for ${upload.fileName} (${upload.totalChunks} chunks)`);

    for (let i = upload.currentChunk; i < upload.totalChunks; i++) {
      // Check if paused or offline
      if (this.paused || !navigator.onLine) {
        console.log(`⏸️ Pausing chunked upload at chunk ${i}/${upload.totalChunks}`);
        this.updateUpload(upload.id, {
          status: UPLOAD_STATUS.PAUSED,
          currentChunk: i
        });
        throw new Error('Upload paused');
      }

      const start = i * CHUNK_SIZE;
      const end = Math.min(start + CHUNK_SIZE, upload.size);
      const chunk = upload.file.slice(start, end);

      await this.uploadChunk(upload, i, chunk);

      // Update progress
      const progress = Math.round(((i + 1) / upload.totalChunks) * 100);
      this.updateUpload(upload.id, {
        progress,
        currentChunk: i + 1,
        uploadedChunks: [...upload.uploadedChunks, i]
      });

      this.notifyProgress(upload.id, progress);
    }

    console.log(`✅ Chunked upload complete for ${upload.fileName}`);
  }

  // Upload single chunk
  async uploadChunk(upload, chunkIndex, chunkBlob) {
    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest();
      const formData = new FormData();
      formData.append('chunk', chunkBlob);
      formData.append('chunkIndex', chunkIndex);
      formData.append('totalChunks', upload.totalChunks);
      formData.append('fileName', upload.fileName);
      formData.append('fileSize', upload.size);

      xhr.addEventListener('load', () => {
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve();
        } else {
          reject(new Error(`Chunk ${chunkIndex} failed: HTTP ${xhr.status}`));
        }
      });

      xhr.addEventListener('error', () => {
        reject(new Error(`Chunk ${chunkIndex} network error`));
      });

      xhr.addEventListener('abort', () => {
        reject(new Error(`Chunk ${chunkIndex} cancelled`));
      });

      const token = localStorage.getItem('authToken');
      xhr.open('POST', `http://localhost:8080/api/upload-chunk${upload.path}`);
      if (token) {
        xhr.setRequestHeader('Authorization', `Bearer ${token}`);
      }

      xhr.send(formData);
    });
  }

  // Retry upload
  async retryUpload(uploadId) {
    const upload = this.getUpload(uploadId);
    if (!upload) return;

    const retryDelay = Math.min(
      RETRY_BASE_DELAY * Math.pow(2, upload.retries),
      RETRY_MAX_DELAY
    );

    console.log(`🔄 Retrying upload ${upload.fileName} (attempt ${upload.retries + 1}/${MAX_RETRIES}) in ${retryDelay}ms`);

    this.updateUpload(uploadId, {
      status: UPLOAD_STATUS.QUEUED,
      retries: upload.retries + 1,
      error: null
    });

    await new Promise(resolve => setTimeout(resolve, retryDelay));
    
    this.processQueue();
  }

  // Check if error is retryable
  isRetryableError(error) {
    const message = error.message?.toLowerCase() || '';
    return (
      message.includes('network') ||
      message.includes('timeout') ||
      message.includes('fetch') ||
      message.includes('503') ||
      message.includes('504')
    );
  }

  // Pause individual upload
  pause(uploadId) {
    const xhr = this.activeUploads.get(uploadId);
    if (xhr) {
      xhr.abort();
      this.activeUploads.delete(uploadId);
    }

    this.updateUpload(uploadId, {
      status: UPLOAD_STATUS.PAUSED
    });

    console.log(`⏸️ Paused upload ${uploadId}`);
  }

  // Resume individual upload
  resume(uploadId) {
    this.updateUpload(uploadId, {
      status: UPLOAD_STATUS.QUEUED
    });

    console.log(`▶️ Resumed upload ${uploadId}`);
    this.processQueue();
  }

  // Pause all uploads
  pauseAll() {
    this.paused = true;

    // Abort all active XHR requests
    this.activeUploads.forEach((xhr, id) => {
      xhr.abort();
      this.updateUpload(id, {
        status: UPLOAD_STATUS.PAUSED
      });
    });

    this.activeUploads.clear();
    console.log('⏸️ All uploads paused');
  }

  // Resume all uploads
  resumeAll() {
    this.paused = false;

    // Resume all paused uploads
    uploadQueue.update(queue => {
      return queue.map(upload => {
        if (upload.status === UPLOAD_STATUS.PAUSED) {
          return { ...upload, status: UPLOAD_STATUS.QUEUED };
        }
        return upload;
      });
    });

    console.log('▶️ Resuming all uploads');
    this.processQueue();
  }

  // Cancel individual upload
  cancel(uploadId) {
    const xhr = this.activeUploads.get(uploadId);
    if (xhr) {
      xhr.abort();
      this.activeUploads.delete(uploadId);
    }

    this.updateUpload(uploadId, {
      status: UPLOAD_STATUS.CANCELLED
    });

    console.log(`❌ Cancelled upload ${uploadId}`);
    
    // Remove from queue after delay
    setTimeout(() => {
      uploadQueue.update(queue => queue.filter(u => u.id !== uploadId));
    }, 3000);

    this.processQueue();
  }

  // Clear completed uploads
  clearCompleted() {
    uploadQueue.update(queue =>
      queue.filter(u => u.status !== UPLOAD_STATUS.COMPLETED)
    );
  }

  // Clear failed uploads
  clearFailed() {
    uploadQueue.update(queue =>
      queue.filter(u => u.status !== UPLOAD_STATUS.FAILED)
    );
  }

  // Start queue processing
  start() {
    this.paused = false;
    console.log('▶️ Upload queue started');
    this.processQueue();
  }

  // Stop queue processing
  stop() {
    this.pauseAll();
  }

  // Helper methods
  getUpload(uploadId) {
    const queue = get(uploadQueue);
    return queue.find(u => u.id === uploadId);
  }

  updateUpload(uploadId, updates) {
    uploadQueue.update(queue =>
      queue.map(upload =>
        upload.id === uploadId ? { ...upload, ...updates } : upload
      )
    );
  }

  // Progress notifications
  onProgress(handler) {
    this.progressHandlers.add(handler);
    return () => this.progressHandlers.delete(handler);
  }

  notifyProgress(uploadId, progress) {
    const upload = this.getUpload(uploadId);
    this.progressHandlers.forEach(handler => {
      try {
        handler({ uploadId, progress, upload });
      } catch (err) {
        console.error('Progress handler error:', err);
      }
    });
  }
}

// Create global upload queue manager
export const uploadQueueManager = new UploadQueueManager();

// Convenience functions
export function addFileToQueue(file, targetPath, priority = PRIORITY.NORMAL) {
  return uploadQueueManager.addFile(file, targetPath, priority);
}

export function addFilesToQueue(files, targetPath, priority = PRIORITY.NORMAL) {
  return uploadQueueManager.addFiles(files, targetPath, priority);
}

export function pauseUpload(uploadId) {
  uploadQueueManager.pause(uploadId);
}

export function resumeUpload(uploadId) {
  uploadQueueManager.resume(uploadId);
}

export function cancelUpload(uploadId) {
  uploadQueueManager.cancel(uploadId);
}

export function startUploads() {
  uploadQueueManager.start();
}

export function stopUploads() {
  uploadQueueManager.stop();
}
