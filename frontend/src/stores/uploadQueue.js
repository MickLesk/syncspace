/**
 * Enhanced Upload Store with Pause/Resume Support
 * Manages upload queue with granular control
 */

import { writable, derived } from 'svelte/store';
import { API_BASE, API_HOST } from '../lib/api.js';

export const uploadQueue = writable([]);
export const isPaused = writable(false);
export const uploadStats = writable({
  totalUploaded: 0,
  totalSize: 0,
  startTime: null,
  uploadSpeed: 0
});

// Upload settings
export const uploadSettings = writable({
  speedLimit: null, // KB/s, null = unlimited
  skipDuplicates: false,
  duplicateAction: 'ask' // ask, skip, replace, keep-both
});

// Active XMLHttpRequest objects for cancellation
const activeRequests = new Map();

// Throttle state
let lastChunkTime = Date.now();
let lastChunkBytes = 0;

/**
 * Add files to upload queue with duplicate detection
 */
export async function addUploads(files, batchName = null) {
  const settings = await new Promise(resolve => {
    uploadSettings.subscribe(s => resolve(s))();
  });

  const batchId = batchName ? Date.now() + Math.random() : null;
  const filesToUpload = [];
  const duplicates = [];

  // Check for duplicates if enabled
  if (settings.skipDuplicates || settings.duplicateAction !== 'keep-both') {
    const currentPath = localStorage.getItem('currentPath') || '/';
    
    for (const file of files) {
      const fileName = file.relativePath || file.webkitRelativePath || file.name;
      const isDuplicate = await checkDuplicate(fileName, currentPath);
      
      if (isDuplicate) {
        if (settings.duplicateAction === 'skip') {
          continue; // Skip this file
        } else if (settings.duplicateAction === 'ask') {
          duplicates.push(file);
          continue; // Will be handled by UI
        }
        // 'replace' and 'keep-both' will proceed
      }
      
      filesToUpload.push(file);
    }
  } else {
    filesToUpload.push(...files);
  }

  const newUploads = filesToUpload.map((file, index) => ({
    id: `upload-${Date.now()}-${index}`,
    file,
    name: file.relativePath || file.webkitRelativePath || file.name,
    size: file.size,
    progress: 0,
    status: 'queued',
    error: null,
    retries: 0,
    batchId,
    batchName,
    batchTotal: filesToUpload.length,
    uploadedBytes: 0,
    startTime: null,
    endTime: null,
    speed: 0
  }));

  uploadQueue.update(queue => [...queue, ...newUploads]);
  
  // Start uploading if not paused
  setTimeout(() => processQueue(), 100);

  // Return duplicates for UI handling
  return { uploaded: newUploads.length, duplicates: duplicates.length, duplicateFiles: duplicates };
}

/**
 * Check if file already exists
 */
async function checkDuplicate(fileName, path) {
  try {
    const token = localStorage.getItem('authToken');
    const response = await fetch(`${API_BASE}/files${path}`, {
      headers: { 'Authorization': `Bearer ${token}` }
    });
    
    if (response.ok) {
      const data = await response.json();
      const files = data.files || [];
      return files.some(f => f.filename === fileName.split('/').pop());
    }
  } catch (e) {
    console.error('Duplicate check failed:', e);
  }
  return false;
}

/**
 * Process upload queue
 */
async function processQueue() {
  let paused = false;
  isPaused.subscribe(p => paused = p)();

  if (paused) return;

  uploadQueue.update(queue => {
    // Find next queued upload
    const nextUpload = queue.find(u => u.status === 'queued');
    if (nextUpload) {
      nextUpload.status = 'uploading';
      nextUpload.startTime = Date.now();
      uploadFile(nextUpload);
    }
    return queue;
  });
}

/**
 * Upload a single file with XHR for progress tracking
 */
async function uploadFile(upload) {
  const formData = new FormData();
  formData.append('file', upload.file);

  // Determine upload path
  let uploadPath = localStorage.getItem('currentPath') || '/';
  if (upload.file.relativePath) {
    const pathParts = upload.file.relativePath.split('/');
    if (pathParts.length > 1) {
      pathParts.pop(); // Remove filename
      uploadPath = `${uploadPath}/${pathParts.join('/')}`.replace(/\/+/g, '/');
    }
  }

  const xhr = new XMLHttpRequest();
  activeRequests.set(upload.id, xhr);

  // Get current settings for throttling
  let currentSettings = {};
  uploadSettings.subscribe(s => currentSettings = s)();

  let lastProgressTime = Date.now();
  let lastProgressBytes = 0;

  xhr.upload.addEventListener('progress', async (e) => {
    if (e.lengthComputable) {
      // Apply speed throttling if enabled
      if (currentSettings.speedLimit && currentSettings.speedLimit > 0) {
        const now = Date.now();
        const timeDiff = (now - lastProgressTime) / 1000; // seconds
        const bytesDiff = e.loaded - lastProgressBytes;
        const currentSpeed = bytesDiff / timeDiff / 1024; // KB/s
        const targetSpeed = currentSettings.speedLimit;

        if (currentSpeed > targetSpeed) {
          // Throttle by pausing briefly
          const delayMs = ((bytesDiff / 1024) / targetSpeed - timeDiff) * 1000;
          if (delayMs > 0) {
            await new Promise(resolve => setTimeout(resolve, Math.min(delayMs, 1000)));
          }
        }

        lastProgressTime = now;
        lastProgressBytes = e.loaded;
      }

      const progress = Math.round((e.loaded / e.total) * 100);
      const now = Date.now();
      const elapsed = (now - upload.startTime) / 1000; // seconds
      const speed = elapsed > 0 ? (e.loaded / elapsed / 1024) : 0; // KB/s
      
      updateUploadProgress(upload.id, progress, e.loaded, speed);
    }
  });

  xhr.addEventListener('load', () => {
    if (xhr.status >= 200 && xhr.status < 300) {
      updateUploadStatus(upload.id, 'complete');
      activeRequests.delete(upload.id);
      
      // Update stats
      uploadStats.update(stats => ({
        ...stats,
        totalUploaded: stats.totalUploaded + upload.size,
      }));
      
      // Process next in queue
      setTimeout(() => processQueue(), 100);
    } else {
      handleUploadError(upload);
    }
  });

  xhr.addEventListener('error', () => {
    handleUploadError(upload);
  });

  xhr.addEventListener('abort', () => {
    updateUploadStatus(upload.id, 'paused');
    activeRequests.delete(upload.id);
  });

  xhr.open('POST', `${API_HOST}/api/upload/${encodeURIComponent(uploadPath)}`);
  xhr.setRequestHeader('Authorization', `Bearer ${localStorage.getItem('authToken')}`);
  xhr.send(formData);
}

/**
 * Handle upload errors with retry logic
 */
function handleUploadError(upload) {
  const MAX_RETRIES = 3;
  
  uploadQueue.update(queue => {
    const item = queue.find(u => u.id === upload.id);
    if (item) {
      item.retries++;
      if (item.retries < MAX_RETRIES) {
        item.status = 'queued'; // Retry
        setTimeout(() => processQueue(), 1000 * item.retries);
      } else {
        item.status = 'error';
        item.error = 'Upload failed after multiple attempts';
        activeRequests.delete(upload.id);
        // Process next in queue
        setTimeout(() => processQueue(), 100);
      }
    }
    return queue;
  });
}

/**
 * Update upload progress
 */
function updateUploadProgress(uploadId, progress, uploadedBytes, speed = null) {
  uploadQueue.update(queue => {
    const upload = queue.find(u => u.id === uploadId);
    if (upload) {
      upload.progress = progress;
      upload.uploadedBytes = uploadedBytes;
      
      // Use provided speed or calculate
      if (speed !== null) {
        upload.speed = speed;
      } else if (upload.startTime) {
        const elapsed = (Date.now() - upload.startTime) / 1000;
        upload.speed = uploadedBytes / elapsed;
      }
    }
    return queue;
  });
}

/**
 * Update upload status
 */
function updateUploadStatus(uploadId, status) {
  uploadQueue.update(queue => {
    const upload = queue.find(u => u.id === uploadId);
    if (upload) {
      upload.status = status;
      if (status === 'complete') {
        upload.progress = 100;
        upload.endTime = Date.now();
      }
    }
    return queue;
  });
}

/**
 * Pause all uploads
 */
export function pauseAllUploads() {
  isPaused.set(true);
  
  // Abort all active XHR requests
  activeRequests.forEach(xhr => xhr.abort());
  
  // Update status
  uploadQueue.update(queue => {
    queue.forEach(upload => {
      if (upload.status === 'uploading') {
        upload.status = 'paused';
      }
    });
    return queue;
  });
}

/**
 * Resume all uploads
 */
export function resumeAllUploads() {
  isPaused.set(false);
  
  // Reset paused uploads to queued
  uploadQueue.update(queue => {
    queue.forEach(upload => {
      if (upload.status === 'paused') {
        upload.status = 'queued';
      }
    });
    return queue;
  });
  
  // Start processing
  setTimeout(() => processQueue(), 100);
}

/**
 * Pause specific upload
 */
export function pauseUpload(uploadId) {
  const xhr = activeRequests.get(uploadId);
  if (xhr) {
    xhr.abort();
  }
  
  updateUploadStatus(uploadId, 'paused');
}

/**
 * Resume specific upload
 */
export function resumeUpload(uploadId) {
  uploadQueue.update(queue => {
    const upload = queue.find(u => u.id === uploadId);
    if (upload && upload.status === 'paused') {
      upload.status = 'queued';
      upload.retries = 0; // Reset retries
    }
    return queue;
  });
  
  setTimeout(() => processQueue(), 100);
}

/**
 * Cancel upload
 */
export function cancelUpload(uploadId) {
  const xhr = activeRequests.get(uploadId);
  if (xhr) {
    xhr.abort();
  }
  activeRequests.delete(uploadId);
  
  uploadQueue.update(queue => queue.filter(u => u.id !== uploadId));
}

/**
 * Retry failed upload
 */
export function retryUpload(uploadId) {
  uploadQueue.update(queue => {
    const upload = queue.find(u => u.id === uploadId);
    if (upload && upload.status === 'error') {
      upload.status = 'queued';
      upload.retries = 0;
      upload.error = null;
      upload.progress = 0;
      upload.uploadedBytes = 0;
    }
    return queue;
  });
  
  setTimeout(() => processQueue(), 100);
}

/**
 * Clear completed uploads
 */
export function clearCompleted() {
  uploadQueue.update(queue => 
    queue.filter(u => u.status !== 'complete')
  );
}

/**
 * Clear all uploads
 */
export function clearAll() {
  // Cancel active uploads
  activeRequests.forEach(xhr => xhr.abort());
  activeRequests.clear();
  
  uploadQueue.set([]);
  uploadStats.set({
    totalUploaded: 0,
    totalSize: 0,
    startTime: null,
    uploadSpeed: 0
  });
}

/**
 * Get upload statistics
 */
export const uploadStatistics = derived(
  uploadQueue,
  $queue => {
    const total = $queue.length;
    const completed = $queue.filter(u => u.status === 'complete').length;
    const uploading = $queue.filter(u => u.status === 'uploading').length;
    const queued = $queue.filter(u => u.status === 'queued').length;
    const paused = $queue.filter(u => u.status === 'paused').length;
    const failed = $queue.filter(u => u.status === 'error').length;
    
    const totalSize = $queue.reduce((sum, u) => sum + u.size, 0);
    const uploadedSize = $queue.reduce((sum, u) => 
      u.status === 'complete' ? sum + u.size : sum + u.uploadedBytes, 0
    );
    
    const overallProgress = totalSize > 0 ? (uploadedSize / totalSize) * 100 : 0;
    
    // Calculate average speed
    const activeUploads = $queue.filter(u => u.status === 'uploading' && u.speed);
    const avgSpeed = activeUploads.length > 0
      ? activeUploads.reduce((sum, u) => sum + (u.speed || 0), 0) / activeUploads.length
      : 0;
    
    return {
      total,
      completed,
      uploading,
      queued,
      paused,
      failed,
      totalSize,
      uploadedSize,
      overallProgress,
      avgSpeed
    };
  }
);
