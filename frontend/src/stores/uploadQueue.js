/**
 * Enhanced Upload Store with Pause/Resume Support
 * Manages upload queue with granular control
 */

import { writable, derived } from 'svelte/store';

export const uploadQueue = writable([]);
export const isPaused = writable(false);
export const uploadStats = writable({
  totalUploaded: 0,
  totalSize: 0,
  startTime: null,
  uploadSpeed: 0
});

// Active XMLHttpRequest objects for cancellation
const activeRequests = new Map();

/**
 * Add files to upload queue
 */
export function addUploads(files, batchName = null) {
  const batchId = batchName ? Date.now() + Math.random() : null;
  const newUploads = Array.from(files).map((file, index) => ({
    id: `upload-${Date.now()}-${index}`,
    file,
    name: file.relativePath || file.webkitRelativePath || file.name,
    size: file.size,
    progress: 0,
    status: 'queued', // queued, uploading, paused, complete, error
    error: null,
    retries: 0,
    batchId,
    batchName,
    batchTotal: files.length,
    uploadedBytes: 0,
    startTime: null,
    endTime: null
  }));

  uploadQueue.update(queue => [...queue, ...newUploads]);
  
  // Start uploading if not paused
  setTimeout(() => processQueue(), 100);
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

  xhr.upload.addEventListener('progress', (e) => {
    if (e.lengthComputable) {
      const progress = Math.round((e.loaded / e.total) * 100);
      updateUploadProgress(upload.id, progress, e.loaded);
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

  xhr.open('POST', `http://localhost:8080/api/upload/${encodeURIComponent(uploadPath)}`);
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
function updateUploadProgress(uploadId, progress, uploadedBytes) {
  uploadQueue.update(queue => {
    const upload = queue.find(u => u.id === uploadId);
    if (upload) {
      upload.progress = progress;
      upload.uploadedBytes = uploadedBytes;
      
      // Calculate speed
      if (upload.startTime) {
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
