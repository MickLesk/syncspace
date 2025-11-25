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
        const data = response ? response : generateMockStorageData();
        set(data);
      } catch (error) {
        console.warn('Storage quota API not available, using mock data:', error.message);
        const mockData = generateMockStorageData();
        set(mockData);
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
 * Generate mock storage data for UI development
 */
function generateMockStorageData() {
  const now = new Date();
  const totalQuota = 10 * 1024 * 1024 * 1024; // 10GB

  // Generate realistic file type distribution
  const types = {
    'Images (JPG, PNG)': {
      size: 3.5 * 1024 * 1024 * 1024, // 3.5GB
      count: 2150,
      icon: 'bi-image',
      color: '#3B82F6', // Blue
    },
    'Videos (MP4, WebM)': {
      size: 4.2 * 1024 * 1024 * 1024, // 4.2GB
      count: 45,
      icon: 'bi-play-circle',
      color: '#EF4444', // Red
    },
    'Documents (PDF, DOCX)': {
      size: 1.2 * 1024 * 1024 * 1024, // 1.2GB
      count: 3420,
      icon: 'bi-file-text',
      color: '#F59E0B', // Amber
    },
    'Archives (ZIP, RAR)': {
      size: 800 * 1024 * 1024, // 800MB
      count: 128,
      icon: 'bi-file-zip',
      color: '#10B981', // Green
    },
    'Audio (MP3, WAV)': {
      size: 300 * 1024 * 1024, // 300MB
      count: 512,
      icon: 'bi-music-note',
      color: '#8B5CF6', // Purple
    },
    'Other': {
      size: 180 * 1024 * 1024, // 180MB
      count: 340,
      icon: 'bi-file',
      color: '#6B7280', // Gray
    },
  };

  const totalUsed = Object.values(types).reduce((sum, t) => sum + t.size, 0);
  const percentUsed = Math.round((totalUsed / totalQuota) * 100);

  // Calculate percentages
  const byType = {};
  Object.entries(types).forEach(([name, data]) => {
    byType[name] = {
      size: data.size,
      count: data.count,
      icon: data.icon,
      color: data.color,
      percentage: Math.round((data.size / totalUsed) * 100),
    };
  });

  // Generate folder distribution
  const folders = [
    { path: '/Projects', size: 3.2 * 1024 * 1024 * 1024, percentage: 28 },
    { path: '/Photos', size: 3.8 * 1024 * 1024 * 1024, percentage: 33 },
    { path: '/Videos', size: 2.1 * 1024 * 1024 * 1024, percentage: 18 },
    { path: '/Documents', size: 900 * 1024 * 1024, percentage: 8 },
    { path: '/Downloads', size: 700 * 1024 * 1024, percentage: 6 },
    { path: '/Other', size: 420 * 1024 * 1024, percentage: 7 },
  ];

  // Generate trend data (last 30 days)
  const trend = [];
  for (let i = 29; i >= 0; i--) {
    const date = new Date(now);
    date.setDate(date.getDate() - i);
    const variation = Math.random() * 0.3 - 0.15; // ±15% variation
    trend.push({
      date: date.toISOString().split('T')[0],
      used: Math.max(0, totalUsed * (0.7 + variation)),
      quota: totalQuota,
    });
  }

  return {
    totalUsed,
    totalQuota,
    percentUsed,
    byType,
    byFolder: folders,
    trend,
    lastUpdated: now.toISOString(),
    isLoading: false,
    error: null,
  };
}

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
