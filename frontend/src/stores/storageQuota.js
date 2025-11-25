import { writable, derived } from 'svelte/store';
import api from '$lib/api.js';
import { toast } from './toast.js';
import { t } from '$lib/i18n.js';

/**
 * Storage Quota Store
 * Manages user storage usage, quotas, and trends
 * Real-time updates via WebSocket integration
 */

// Create storage quota store
function createStorageStore() {
  const { subscribe, set, update } = writable({
    totalUsed: 0,
    totalQuota: 10 * 1024 * 1024 * 1024, // 10GB default
    percentUsed: 0,
    byType: {}, // { 'image': { size, count, percentage }, ... }
    byFolder: [], // [{ path, size, percentage }, ...]
    trend: [], // Historical usage data
    lastUpdated: null,
    isLoading: false,
    error: null,
  });

  return {
    subscribe,
    
    // Fetch latest storage statistics
    async loadQuotaData() {
      update(state => ({ ...state, isLoading: true, error: null }));
      try {
        const response = await api.users?.getStorageQuota?.();
        if (!response) {
          throw new Error('Storage quota API not available');
        }
        set({
          ...response,
          isLoading: false,
          error: null,
          lastUpdated: new Date().toISOString()
        });
      } catch (error) {
        console.error('Failed to load storage quota:', error);
        update(state => ({ 
          ...state, 
          isLoading: false, 
          error: error.message || 'Failed to load storage data'
        }));
      }
    },

    // Get storage status with color coding
    getStatus(percentUsed) {
      if (percentUsed < 50) return 'success'; // Green
      if (percentUsed < 80) return 'warning'; // Yellow
      return 'danger'; // Red
    },

    // Format bytes to human readable
    formatSize(bytes) {
      if (bytes === 0) return '0 B';
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    },

    // Calculate remaining storage
    getRemainingBytes() {
      let remaining = 0;
      const state = get();
      return state.totalQuota - state.totalUsed;
    },

    // Clear data
    clear() {
      set({
        totalUsed: 0,
        totalQuota: 10 * 1024 * 1024 * 1024,
        percentUsed: 0,
        byType: {},
        byFolder: [],
        trend: [],
        lastUpdated: null,
        isLoading: false,
        error: null,
      });
    },
  };
}

// Main storage quota store
export const storageQuota = createStorageStore();

// Derived store for color status
export const storageStatus = derived(
  storageQuota,
  ($quota) => {
    if ($quota.percentUsed < 50) return 'success';
    if ($quota.percentUsed < 80) return 'warning';
    return 'danger';
  }
);

// Derived store for remaining bytes formatted
export const remainingStorage = derived(
  storageQuota,
  ($quota) => formatStorageSize($quota.totalQuota - $quota.totalUsed)
);

// Derived store for type breakdown as sorted array
export const typeBreakdown = derived(
  storageQuota,
  ($quota) => {
    return Object.entries($quota.byType || {})
      .map(([type, data]) => ({
        type,
        ...data,
      }))
      .sort((a, b) => b.size - a.size);
  }
);

/**
 * Format storage size to human readable format
 */
export function formatStorageSize(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}
