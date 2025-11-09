/**
 * Upload Manager Store
 * Manages parallel file uploads with progress tracking, pause/resume, error handling
 * 
 * Usage:
 * import { uploadManager } from './stores/uploadManager.js';
 * const job = uploadManager.addUpload(file, destination);
 * uploadManager.subscribe(jobs => console.log(jobs));
 */

import { writable, derived } from 'svelte/store';

// Upload job states
export const UPLOAD_STATE = {
  PENDING: 'pending',
  UPLOADING: 'uploading',
  PAUSED: 'paused',
  COMPLETED: 'completed',
  FAILED: 'failed',
  CANCELLED: 'cancelled',
};

// Create main store
const uploadJobs = writable([]);

// Configuration
const MAX_PARALLEL_UPLOADS = 3;
const CHUNK_SIZE = 5 * 1024 * 1024; // 5MB chunks

let activeUploads = 0;
let uploadQueue = [];

/**
 * Create upload job
 * @param {File} file - File to upload
 * @param {string} destination - Target path
 * @param {Object} options - Additional options
 * @returns {string} Job ID
 */
function createUploadJob(file, destination, options = {}) {
  const jobId = `upload-${Date.now()}-${Math.random()}`;

  const job = {
    id: jobId,
    filename: file.name,
    size: file.size,
    destination,
    state: UPLOAD_STATE.PENDING,
    progress: 0,
    speedBytesPerSecond: 0,
    estimatedTimeRemaining: 0,
    uploadedBytes: 0,
    error: null,
    startTime: null,
    endTime: null,
    retryCount: 0,
    maxRetries: options.maxRetries || 3,
    file,
    ...options,
  };

  uploadJobs.update((jobs) => [...jobs, job]);
  uploadQueue.push(jobId);
  processQueue();

  return jobId;
}

/**
 * Process upload queue
 */
async function processQueue() {
  while (uploadQueue.length > 0 && activeUploads < MAX_PARALLEL_UPLOADS) {
    const jobId = uploadQueue.shift();

    uploadJobs.update((jobs) =>
      jobs.map((job) =>
        job.id === jobId
          ? {
              ...job,
              state: UPLOAD_STATE.UPLOADING,
              startTime: Date.now(),
            }
          : job
      )
    );

    activeUploads++;

    try {
      await performUpload(jobId);
    } catch (error) {
      console.error(`Upload failed for ${jobId}:`, error);
    }

    activeUploads--;
    processQueue();
  }
}

/**
 * Perform actual upload
 * @param {string} jobId - Upload job ID
 */
async function performUpload(jobId) {
  const getJob = () => {
    let currentJob;
    uploadJobs.subscribe((jobs) => {
      currentJob = jobs.find((j) => j.id === jobId);
    })();
    return currentJob;
  };

  let job = getJob();
  if (!job) return;

  const file = job.file;
  const fileSize = file.size;
  const chunksNeeded = Math.ceil(fileSize / CHUNK_SIZE);

  try {
    for (let chunkIndex = 0; chunkIndex < chunksNeeded; chunkIndex++) {
      job = getJob();

      // Check for pause
      if (job.state === UPLOAD_STATE.PAUSED) {
        return;
      }

      // Check for cancel
      if (job.state === UPLOAD_STATE.CANCELLED) {
        return;
      }

      const start = chunkIndex * CHUNK_SIZE;
      const end = Math.min(start + CHUNK_SIZE, fileSize);
      const chunk = file.slice(start, end);

      const chunkStartTime = performance.now();

      // Upload chunk
      await uploadChunk(jobId, chunkIndex, chunksNeeded, chunk, job.destination);

      const chunkEndTime = performance.now();
      const chunkDuration = (chunkEndTime - chunkStartTime) / 1000; // seconds
      const chunkSpeed = chunk.size / chunkDuration;

      // Update progress
      const uploadedBytes = end;
      const progress = Math.round((uploadedBytes / fileSize) * 100);
      const timeElapsed = (Date.now() - job.startTime) / 1000;
      const uploadSpeed = uploadedBytes / timeElapsed;
      const timeRemaining = (fileSize - uploadedBytes) / uploadSpeed;

      uploadJobs.update((jobs) =>
        jobs.map((j) =>
          j.id === jobId
            ? {
                ...j,
                progress,
                uploadedBytes,
                speedBytesPerSecond: uploadSpeed,
                estimatedTimeRemaining: timeRemaining,
              }
            : j
        )
      );
    }

    // Upload completed
    uploadJobs.update((jobs) =>
      jobs.map((j) =>
        j.id === jobId
          ? {
              ...j,
              state: UPLOAD_STATE.COMPLETED,
              progress: 100,
              endTime: Date.now(),
            }
          : j
      )
    );
  } catch (error) {
    job = getJob();
    if (job.retryCount < job.maxRetries) {
      // Retry
      uploadJobs.update((jobs) =>
        jobs.map((j) =>
          j.id === jobId
            ? {
                ...j,
                retryCount: j.retryCount + 1,
                state: UPLOAD_STATE.PENDING,
              }
            : j
        )
      );
      uploadQueue.unshift(jobId); // Add back to front of queue
      processQueue();
    } else {
      // Final failure
      uploadJobs.update((jobs) =>
        jobs.map((j) =>
          j.id === jobId
            ? {
                ...j,
                state: UPLOAD_STATE.FAILED,
                error: error.message,
                endTime: Date.now(),
              }
            : j
        )
      );
    }
  }
}

/**
 * Upload single chunk
 * @param {string} jobId - Upload job ID
 * @param {number} chunkIndex - Chunk number
 * @param {number} chunksTotal - Total chunks
 * @param {Blob} chunk - Chunk data
 * @param {string} destination - Target path
 */
async function uploadChunk(jobId, chunkIndex, chunksTotal, chunk, destination) {
  const formData = new FormData();
  formData.append('chunk', chunk);
  formData.append('chunkIndex', chunkIndex);
  formData.append('chunksTotal', chunksTotal);
  formData.append('destination', destination);
  formData.append('jobId', jobId);

  const response = await fetch('/api/upload/chunk', {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${localStorage.getItem('authToken')}`,
    },
    body: formData,
  });

  if (!response.ok) {
    throw new Error(`Upload chunk failed: ${response.statusText}`);
  }
}

/**
 * Pause upload
 * @param {string} jobId - Upload job ID
 */
function pauseUpload(jobId) {
  uploadJobs.update((jobs) =>
    jobs.map((j) => (j.id === jobId ? { ...j, state: UPLOAD_STATE.PAUSED } : j))
  );
}

/**
 * Resume upload
 * @param {string} jobId - Upload job ID
 */
function resumeUpload(jobId) {
  uploadJobs.update((jobs) =>
    jobs.map((j) =>
      j.id === jobId ? { ...j, state: UPLOAD_STATE.UPLOADING } : j
    )
  );

  uploadQueue.push(jobId);
  processQueue();
}

/**
 * Cancel upload
 * @param {string} jobId - Upload job ID
 */
function cancelUpload(jobId) {
  uploadJobs.update((jobs) =>
    jobs.map((j) =>
      j.id === jobId
        ? { ...j, state: UPLOAD_STATE.CANCELLED, endTime: Date.now() }
        : j
    )
  );
}

/**
 * Retry failed upload
 * @param {string} jobId - Upload job ID
 */
function retryUpload(jobId) {
  uploadJobs.update((jobs) =>
    jobs.map((j) =>
      j.id === jobId
        ? {
            ...j,
            state: UPLOAD_STATE.PENDING,
            progress: 0,
            uploadedBytes: 0,
            error: null,
            retryCount: 0,
          }
        : j
    )
  );

  uploadQueue.push(jobId);
  processQueue();
}

/**
 * Clear completed uploads
 */
function clearCompleted() {
  uploadJobs.update((jobs) =>
    jobs.filter(
      (j) =>
        j.state !== UPLOAD_STATE.COMPLETED &&
        j.state !== UPLOAD_STATE.FAILED &&
        j.state !== UPLOAD_STATE.CANCELLED
    )
  );
}

/**
 * Format bytes to human-readable size
 * @param {number} bytes - Size in bytes
 * @returns {string} Formatted size
 */
export function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

/**
 * Format seconds to human-readable time
 * @param {number} seconds - Duration in seconds
 * @returns {string} Formatted time
 */
export function formatTime(seconds) {
  if (seconds < 60) {
    return Math.round(seconds) + 's';
  }

  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) {
    return minutes + 'm ' + Math.round(seconds % 60) + 's';
  }

  const hours = Math.floor(minutes / 60);
  const remainingMinutes = minutes % 60;
  return hours + 'h ' + remainingMinutes + 'm';
}

// Derived stores
export const activeUploads = derived(uploadJobs, (jobs) =>
  jobs.filter((j) => j.state === UPLOAD_STATE.UPLOADING)
);

export const completedUploads = derived(uploadJobs, (jobs) =>
  jobs.filter((j) => j.state === UPLOAD_STATE.COMPLETED)
);

export const failedUploads = derived(uploadJobs, (jobs) =>
  jobs.filter((j) => j.state === UPLOAD_STATE.FAILED)
);

export const totalProgress = derived(uploadJobs, (jobs) => {
  if (jobs.length === 0) return 0;

  const totalProgress = jobs.reduce((sum, j) => sum + j.progress, 0);
  return Math.round(totalProgress / jobs.length);
});

export const totalSizeUploaded = derived(uploadJobs, (jobs) =>
  jobs.reduce((sum, j) => sum + j.uploadedBytes, 0)
);

export const totalSizeToUpload = derived(uploadJobs, (jobs) =>
  jobs.reduce((sum, j) => sum + j.size, 0)
);

export const uploadStats = derived(
  [uploadJobs, totalSizeUploaded, totalSizeToUpload],
  ([$jobs, $uploaded, $total]) => {
    const completed = $jobs.filter((j) => j.state === UPLOAD_STATE.COMPLETED).length;
    const failed = $jobs.filter((j) => j.state === UPLOAD_STATE.FAILED).length;
    const active = $jobs.filter((j) => j.state === UPLOAD_STATE.UPLOADING).length;

    const avgSpeed =
      $jobs.length > 0
        ? $jobs.reduce((sum, j) => sum + j.speedBytesPerSecond, 0) / $jobs.length
        : 0;

    return {
      total: $jobs.length,
      active,
      completed,
      failed,
      sizeUploaded: $uploaded,
      sizeTotal: $total,
      averageSpeed: avgSpeed,
      percentage: $total > 0 ? Math.round(($uploaded / $total) * 100) : 0,
    };
  }
);

// Export manager
export const uploadManager = {
  addUpload: createUploadJob,
  pauseUpload,
  resumeUpload,
  cancelUpload,
  retryUpload,
  clearCompleted,
  formatBytes,
  formatTime,
  jobs: {
    subscribe: uploadJobs.subscribe,
  },
  activeUploads: {
    subscribe: activeUploads.subscribe,
  },
  completedUploads: {
    subscribe: completedUploads.subscribe,
  },
  failedUploads: {
    subscribe: failedUploads.subscribe,
  },
  uploadStats: {
    subscribe: uploadStats.subscribe,
  },
};

export default uploadManager;
