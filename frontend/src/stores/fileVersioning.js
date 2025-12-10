/**
 * File Versioning Store
 * 
 * Backend-First: All versions stored in database
 * API: GET/DELETE /api/files/{path}/versions
 * Database Model: FileVersion { id, filePath, versionNumber, fileHash, sizBytes, createdBy, createdAt }
 * 
 * Features:
 * - View all versions of a file
 * - Compare two versions (diff)
 * - Restore to previous version
 * - Delete old versions (cleanup)
 * - Version timeline
 */

import { writable, derived } from 'svelte/store';

// ============================================================================
// STATE STRUCTURE
// ============================================================================

const defaultVersionState = {
  versions: [],
  currentFile: null, // { filePath, currentVersion }
  selectedVersionId: null,
  compareMode: false, // true = comparing two versions
  selectedVersionId2: null,
  diffResult: null, // { type, changes[] }
};

// ============================================================================
// INTERNAL STORES
// ============================================================================

const _versionState = writable(defaultVersionState);
const _loading = writable(false);
const _restoring = writable(false);
const _error = writable(null);

// Public
export const versionState = _versionState;
export const loading = _loading;
export const restoring = _restoring;
export const error = _error;

// Derived: version timeline
export const versionTimeline = derived(versionState, ($state) => {
  return $state.versions
    .sort((a, b) => new Date(b.createdAt) - new Date(a.createdAt))
    .map((v, idx) => ({
      ...v,
      position: idx,
      isCurrent: idx === 0,
      date: new Date(v.createdAt),
    }));
});

// ============================================================================
// API COMMUNICATION
// ============================================================================

import { API_BASE } from '../lib/api.js';

function getAuthToken() {
  if (typeof window !== 'undefined') {
    return localStorage.getItem('authToken');
  }
  return null;
}

async function request(endpoint, options = {}) {
  const token = getAuthToken();
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  _error.set(null);

  try {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers,
    });

    if (response.status === 401) {
      localStorage.removeItem('authToken');
      window.location.hash = '#/login';
      throw new Error('Unauthorized');
    }

    if (!response.ok) {
      const error = await response.json().catch(() => ({
        message: response.statusText,
      }));
      throw new Error(error.message || `HTTP ${response.status}`);
    }

    return await response.json();
  } catch (err) {
    _error.set(err.message);
    throw err;
  }
}

// ============================================================================
// PUBLIC API
// ============================================================================

/**
 * Load all versions for a file
 * @param {string} filePath - File path
 */
export async function loadVersions(filePath) {
  _loading.set(true);

  try {
    const versions = await request(
      `/files/${encodeURIComponent(filePath)}/versions`
    );

    _versionState.update((state) => ({
      ...state,
      versions: versions || [],
      currentFile: {
        filePath,
        currentVersion: versions?.[0],
      },
      selectedVersionId: null,
      compareMode: false,
    }));
  } catch (err) {
    _error.set(`Failed to load versions: ${err.message}`);
  } finally {
    _loading.set(false);
  }
}

/**
 * Download a specific version
 * @param {string} filePath - File path
 * @param {number} versionNumber - Version number
 */
export function downloadVersion(filePath, versionNumber) {
  const token = getAuthToken();
  const headers = token ? { Authorization: `Bearer ${token}` } : {};
  window.location.href = `${API_BASE}/files/${encodeURIComponent(filePath)}/versions/${versionNumber}/download`;
}

/**
 * Preview a version (get metadata)
 * @param {string} filePath - File path
 * @param {number} versionNumber - Version number
 */
export async function previewVersion(filePath, versionNumber) {
  try {
    return await request(
      `/files/${encodeURIComponent(filePath)}/versions/${versionNumber}`
    );
  } catch (err) {
    _error.set(`Failed to load version: ${err.message}`);
    return null;
  }
}

/**
 * Compare two versions (generate diff)
 * @param {string} filePath - File path
 * @param {number} versionId1 - First version
 * @param {number} versionId2 - Second version
 */
export async function compareVersions(filePath, versionId1, versionId2) {
  _loading.set(true);

  try {
    const diff = await request(`/files/${encodeURIComponent(filePath)}/versions/diff`, {
      method: 'POST',
      body: JSON.stringify({ versionId1, versionId2 }),
    });

    _versionState.update((state) => ({
      ...state,
      compareMode: true,
      selectedVersionId: versionId1,
      selectedVersionId2: versionId2,
      diffResult: diff,
    }));

    return diff;
  } catch (err) {
    _error.set(`Failed to compare versions: ${err.message}`);
  } finally {
    _loading.set(false);
  }
}

/**
 * Restore a previous version
 * @param {string} filePath - File path
 * @param {number} versionNumber - Version number to restore
 */
export async function restoreVersion(filePath, versionNumber) {
  if (!confirm('Restore this version? Current file will be moved to version history.')) {
    return;
  }

  _restoring.set(true);

  try {
    const result = await request(
      `/files/${encodeURIComponent(filePath)}/versions/${versionNumber}/restore`,
      { method: 'POST' }
    );

    _versionState.update((state) => ({
      ...state,
      versions: state.versions.map((v) =>
        v.versionNumber === versionNumber ? { ...v, restored: true } : v
      ),
    }));

    _error.set(null);
    return result;
  } catch (err) {
    _error.set(`Failed to restore version: ${err.message}`);
  } finally {
    _restoring.set(false);
  }
}

/**
 * Delete a version (cleanup)
 * @param {string} filePath - File path
 * @param {number} versionNumber - Version number to delete
 */
export async function deleteVersion(filePath, versionNumber) {
  if (!confirm('Permanently delete this version? This cannot be undone.')) {
    return;
  }

  try {
    await request(
      `/files/${encodeURIComponent(filePath)}/versions/${versionNumber}`,
      { method: 'DELETE' }
    );

    _versionState.update((state) => ({
      ...state,
      versions: state.versions.filter((v) => v.versionNumber !== versionNumber),
    }));
  } catch (err) {
    _error.set(`Failed to delete version: ${err.message}`);
  }
}

/**
 * Delete all versions older than N days
 * @param {string} filePath - File path
 * @param {number} daysOld - Delete versions older than this
 */
export async function cleanupOldVersions(filePath, daysOld = 30) {
  if (!confirm(`Delete all versions older than ${daysOld} days?`)) {
    return;
  }

  try {
    const result = await request(
      `/files/${encodeURIComponent(filePath)}/versions/cleanup`,
      {
        method: 'POST',
        body: JSON.stringify({ daysOld }),
      }
    );

    // Reload versions
    await loadVersions(filePath);

    return result;
  } catch (err) {
    _error.set(`Failed to cleanup versions: ${err.message}`);
  }
}

/**
 * Exit compare mode
 */
export function exitCompareMode() {
  _versionState.update((state) => ({
    ...state,
    compareMode: false,
    selectedVersionId2: null,
    diffResult: null,
  }));
}

/**
 * Select a version to view
 * @param {number} versionId - Version ID
 */
export function selectVersion(versionId) {
  _versionState.update((state) => ({
    ...state,
    selectedVersionId: versionId,
  }));
}

/**
 * Calculate version statistics
 */
export function getVersionStats(filePath) {
  let versions = [];
  versionState.subscribe((s) => (versions = s.versions))();

  if (versions.length === 0) return null;

  const sizes = versions.map((v) => v.sizeBytes);
  const totalSize = sizes.reduce((a, b) => a + b, 0);
  const avgSize = Math.round(totalSize / sizes.length);
  const maxSize = Math.max(...sizes);
  const minSize = Math.min(...sizes);

  const dates = versions.map((v) => new Date(v.createdAt).getTime());
  const timespans = [];
  for (let i = 1; i < dates.length; i++) {
    timespans.push(dates[i - 1] - dates[i]);
  }
  const avgTimespan =
    timespans.length > 0
      ? Math.round(timespans.reduce((a, b) => a + b, 0) / timespans.length)
      : 0;

  return {
    totalVersions: versions.length,
    totalSize,
    avgSize,
    maxSize,
    minSize,
    avgTimespan,
    avgTimeoutDays: Math.round(avgTimespan / (1000 * 60 * 60 * 24)),
    oldestVersion: versions[versions.length - 1],
    newestVersion: versions[0],
  };
}

/**
 * Export version history
 */
export function exportVersionHistory(filePath) {
  let state;
  versionState.subscribe((s) => (state = s))();

  return {
    filePath,
    versions: state.versions,
    stats: getVersionStats(filePath),
    exportedAt: new Date().toISOString(),
  };
}

export default {
  versionState,
  versionTimeline,
  loading,
  restoring,
  error,
  loadVersions,
  downloadVersion,
  previewVersion,
  compareVersions,
  restoreVersion,
  deleteVersion,
  cleanupOldVersions,
  exitCompareMode,
  selectVersion,
  getVersionStats,
  exportVersionHistory,
};
