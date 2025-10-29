/**
 * Batch Operations Store
 * Manages batch file operations with progress tracking and cancellation
 */

import { writable, derived, get } from 'svelte/store';
import api from '../lib/api.js';

// Active batch operations: { jobId: { status, progress, ... } }
const activeOperations = writable({});

// Current operation being displayed in progress dialog
const currentJobId = writable(null);

/**
 * Start a batch copy operation
 * @param {string[]} sources - Array of file paths to copy
 * @param {string} destination - Destination folder path
 * @returns {Promise<string>} jobId
 */
export async function startBatchCopy(sources, destination) {
  try {
    const response = await api.batch.copy({ sources, destination });
    const jobId = response.job_id;
    
    // Add to active operations
    activeOperations.update(ops => ({
      ...ops,
      [jobId]: {
        type: 'copy',
        status: response.status,
        sources,
        destination,
        startedAt: new Date(),
      }
    }));
    
    // Set as current operation
    currentJobId.set(jobId);
    
    return jobId;
  } catch (error) {
    console.error('Failed to start batch copy:', error);
    throw error;
  }
}

/**
 * Start a batch compress operation
 * @param {string[]} files - Array of file paths to compress
 * @param {string} archiveName - Name of archive file
 * @param {string} format - Archive format (zip, tar.gz, etc.)
 * @returns {Promise<string>} jobId
 */
export async function startBatchCompress(files, archiveName, format = 'zip') {
  try {
    const response = await api.batch.compress({ files, archive_name: archiveName, format });
    const jobId = response.job_id;
    
    // Add to active operations
    activeOperations.update(ops => ({
      ...ops,
      [jobId]: {
        type: 'compress',
        status: response.status,
        files,
        archiveName,
        format,
        startedAt: new Date(),
      }
    }));
    
    // Set as current operation
    currentJobId.set(jobId);
    
    return jobId;
  } catch (error) {
    console.error('Failed to start batch compress:', error);
    throw error;
  }
}

/**
 * Get status of a batch operation
 * @param {string} jobId - Operation job ID
 * @returns {Promise<Object>} Status object
 */
export async function getOperationStatus(jobId) {
  try {
    const status = await api.batch.getStatus(jobId);
    
    // Update operation in store
    activeOperations.update(ops => {
      if (ops[jobId]) {
        return {
          ...ops,
          [jobId]: {
            ...ops[jobId],
            ...status,
          }
        };
      }
      return ops;
    });
    
    return status;
  } catch (error) {
    console.error('Failed to get operation status:', error);
    throw error;
  }
}

/**
 * Cancel a batch operation
 * @param {string} jobId - Operation job ID
 * @returns {Promise<void>}
 */
export async function cancelOperation(jobId) {
  try {
    await api.batch.cancel(jobId);
    
    // Update operation status
    activeOperations.update(ops => {
      if (ops[jobId]) {
        return {
          ...ops,
          [jobId]: {
            ...ops[jobId],
            status: 'cancelled',
            cancelledAt: new Date(),
          }
        };
      }
      return ops;
    });
  } catch (error) {
    console.error('Failed to cancel operation:', error);
    throw error;
  }
}

/**
 * Remove completed operation from store
 * @param {string} jobId - Operation job ID
 */
export function removeOperation(jobId) {
  activeOperations.update(ops => {
    // Create new object without the removed job
    const newOps = {};
    for (const [id, op] of Object.entries(ops)) {
      if (id !== jobId) {
        newOps[id] = op;
      }
    }
    return newOps;
  });
  
  // Clear current if it was this job
  if (get(currentJobId) === jobId) {
    currentJobId.set(null);
  }
}

/**
 * Clear all completed/failed/cancelled operations
 */
export function clearCompletedOperations() {
  activeOperations.update(ops => {
    const filtered = {};
    for (const [jobId, op] of Object.entries(ops)) {
      if (op.status === 'running' || op.status === 'pending') {
        filtered[jobId] = op;
      }
    }
    return filtered;
  });
}

// Derived store: list of all operations
export const operationsList = derived(activeOperations, $ops => 
  Object.entries($ops).map(([jobId, op]) => ({ jobId, ...op }))
);

// Derived store: count of active operations
export const activeOperationsCount = derived(activeOperations, $ops => 
  Object.values($ops).filter(op => 
    op.status === 'running' || op.status === 'pending'
  ).length
);

// Derived store: has any active operations
export const hasActiveOperations = derived(activeOperationsCount, count => count > 0);

// Export stores
export const batchOperations = {
  subscribe: activeOperations.subscribe,
  startCopy: startBatchCopy,
  startCompress: startBatchCompress,
  getStatus: getOperationStatus,
  cancel: cancelOperation,
  remove: removeOperation,
  clearCompleted: clearCompletedOperations,
};

export { currentJobId };
