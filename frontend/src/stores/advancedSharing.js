/**
 * Advanced File Sharing Store
 * 
 * Backend-First: All shares stored in database
 * API: GET/POST/PUT/DELETE /api/sharing, /api/files/{path}/shares
 * Database Model: Share { id, filePath, shareToken, permission, expiresAt, password, downloadLimit, createdBy, createdAt }
 * 
 * Features:
 * - Create share links with custom settings
 * - Expiry dates (never, 1 week, 1 month, etc)
 * - Password protection
 * - Download limits
 * - Permission levels (read, write)
 * - View share analytics (downloads, access logs)
 */

import { writable, derived } from 'svelte/store';

// ============================================================================
// STATE STRUCTURE
// ============================================================================

const defaultShareState = {
  shares: [],
  activeShare: null,
  analytics: {
    totalDownloads: 0,
    lastAccessed: null,
    accessLog: [],
  },
};

// ============================================================================
// INTERNAL STORES
// ============================================================================

const _shareState = writable(defaultShareState);
const _loading = writable(false);
const _creatingShare = writable(false);
const _error = writable(null);

// Public
export const shareState = _shareState;
export const loading = _loading;
export const creatingShare = _creatingShare;
export const error = _error;

// Derived: active shares
export const activeShares = derived(shareState, ($state) => {
  return $state.shares.filter(
    (s) => !s.expiresAt || new Date(s.expiresAt) > new Date()
  );
});

// Derived: expired shares
export const expiredShares = derived(shareState, ($state) => {
  return $state.shares.filter(
    (s) => s.expiresAt && new Date(s.expiresAt) <= new Date()
  );
});

// ============================================================================
// API COMMUNICATION
// ============================================================================

const API_BASE = 'http://localhost:8080/api';

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
 * Load all shares for a file
 * @param {string} filePath - File path
 */
export async function loadShares(filePath) {
  _loading.set(true);

  try {
    const shares = await request(`/files/${encodeURIComponent(filePath)}/shares`);

    _shareState.update((state) => ({
      ...state,
      shares: shares || [],
    }));
  } catch (err) {
    _error.set(`Failed to load shares: ${err.message}`);
  } finally {
    _loading.set(false);
  }
}

/**
 * Create a new share
 * @param {string} filePath - File path
 * @param {object} options - Share options
 * @param {string} options.permission - 'read' | 'write'
 * @param {string} options.expiresIn - 'never' | '1week' | '1month' | '90days'
 * @param {string} options.password - Optional password
 * @param {number} options.downloadLimit - Optional download limit
 */
export async function createShare(filePath, options = {}) {
  _creatingShare.set(true);

  try {
    // Calculate expiry date
    const expiresAt = calculateExpiryDate(options.expiresIn || 'never');

    const shareData = {
      filePath,
      permission: options.permission || 'read',
      expiresAt,
      ...(options.password && { password: options.password }),
      ...(options.downloadLimit && { downloadLimit: options.downloadLimit }),
    };

    const created = await request('/sharing', {
      method: 'POST',
      body: JSON.stringify(shareData),
    });

    _shareState.update((state) => ({
      ...state,
      shares: [...state.shares, created],
    }));

    return created;
  } catch (err) {
    _error.set(`Failed to create share: ${err.message}`);
  } finally {
    _creatingShare.set(false);
  }
}

/**
 * Update a share
 * @param {string} shareId - Share ID
 * @param {object} updates - Fields to update
 */
export async function updateShare(shareId, updates) {
  try {
    const updated = await request(`/sharing/${shareId}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });

    _shareState.update((state) => ({
      ...state,
      shares: state.shares.map((s) =>
        s.id === shareId ? updated : s
      ),
    }));

    return updated;
  } catch (err) {
    _error.set(`Failed to update share: ${err.message}`);
  }
}

/**
 * Delete a share
 * @param {string} shareId - Share ID
 */
export async function deleteShare(shareId) {
  if (!confirm('Delete this share link?')) return;

  try {
    await request(`/sharing/${shareId}`, { method: 'DELETE' });

    _shareState.update((state) => ({
      ...state,
      shares: state.shares.filter((s) => s.id !== shareId),
    }));
  } catch (err) {
    _error.set(`Failed to delete share: ${err.message}`);
  }
}

/**
 * Get share analytics
 * @param {string} shareId - Share ID
 */
export async function getShareAnalytics(shareId) {
  try {
    const analytics = await request(`/sharing/${shareId}/analytics`);

    _shareState.update((state) => ({
      ...state,
      analytics,
    }));

    return analytics;
  } catch (err) {
    _error.set(`Failed to load analytics: ${err.message}`);
    return null;
  }
}

/**
 * Get share access log
 * @param {string} shareId - Share ID
 */
export async function getShareAccessLog(shareId) {
  try {
    return await request(`/sharing/${shareId}/access-log`);
  } catch (err) {
    _error.set(`Failed to load access log: ${err.message}`);
    return [];
  }
}

/**
 * Regenerate share token
 * @param {string} shareId - Share ID
 */
export async function regenerateShareToken(shareId) {
  if (!confirm('Generate a new share link? Old link will stop working.')) return;

  try {
    const updated = await request(`/sharing/${shareId}/regenerate-token`, {
      method: 'POST',
    });

    _shareState.update((state) => ({
      ...state,
      shares: state.shares.map((s) =>
        s.id === shareId ? updated : s
      ),
    }));

    return updated;
  } catch (err) {
    _error.set(`Failed to regenerate token: ${err.message}`);
  }
}

/**
 * Copy share link to clipboard
 * @param {string} shareToken - Share token
 */
export function copyShareLink(shareToken) {
  const link = `${window.location.origin}/share/${shareToken}`;

  if (navigator.clipboard) {
    navigator.clipboard.writeText(link);
  } else {
    // Fallback for older browsers
    const textarea = document.createElement('textarea');
    textarea.value = link;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand('copy');
    document.body.removeChild(textarea);
  }

  return link;
}

/**
 * Get public share info (no auth needed)
 * @param {string} shareToken - Share token
 * @param {string} password - Password if protected
 */
export async function getPublicShare(shareToken, password = null) {
  try {
    const response = await fetch(`${API_BASE}/sharing/public/${shareToken}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        ...(password && { 'X-Share-Password': password }),
      },
    });

    if (!response.ok) {
      throw new Error('Share not found or access denied');
    }

    return await response.json();
  } catch (err) {
    throw err;
  }
}

/**
 * Download from public share
 * @param {string} shareToken - Share token
 */
export function downloadPublicShare(shareToken) {
  window.location.href = `${API_BASE}/sharing/public/${shareToken}/download`;
}

// ============================================================================
// UTILITIES
// ============================================================================

function calculateExpiryDate(expiresIn) {
  if (expiresIn === 'never') return null;

  const now = new Date();
  const durations = {
    '1week': 7 * 24 * 60 * 60 * 1000,
    '1month': 30 * 24 * 60 * 60 * 1000,
    '90days': 90 * 24 * 60 * 60 * 1000,
    '1year': 365 * 24 * 60 * 60 * 1000,
  };

  const ms = durations[expiresIn] || 0;
  return new Date(now.getTime() + ms).toISOString();
}

export function getExpiryLabel(expiresAt) {
  if (!expiresAt) return 'Never';

  const now = new Date();
  const expiry = new Date(expiresAt);
  const diff = expiry.getTime() - now.getTime();

  if (diff < 0) return 'Expired';
  if (diff < 24 * 60 * 60 * 1000) return 'Today';
  if (diff < 7 * 24 * 60 * 60 * 1000) {
    const days = Math.floor(diff / (24 * 60 * 60 * 1000));
    return `${days}d left`;
  }
  if (diff < 30 * 24 * 60 * 60 * 1000) {
    const weeks = Math.floor(diff / (7 * 24 * 60 * 60 * 1000));
    return `${weeks}w left`;
  }

  const months = Math.floor(diff / (30 * 24 * 60 * 60 * 1000));
  return `${months}m left`;
}

export function isShareExpired(expiresAt) {
  if (!expiresAt) return false;
  return new Date(expiresAt) <= new Date();
}

export function isDownloadLimitReached(share) {
  if (!share.downloadLimit) return false;
  return share.downloads >= share.downloadLimit;
}

export default {
  shareState,
  activeShares,
  expiredShares,
  loading,
  creatingShare,
  error,
  loadShares,
  createShare,
  updateShare,
  deleteShare,
  getShareAnalytics,
  getShareAccessLog,
  regenerateShareToken,
  copyShareLink,
  getPublicShare,
  downloadPublicShare,
  getExpiryLabel,
  isShareExpired,
  isDownloadLimitReached,
};
