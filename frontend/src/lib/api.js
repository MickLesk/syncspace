/**
 * API Service for SyncSpace
 * Handles all HTTP requests to the backend
 */

const API_BASE = "http://localhost:8080/api";

/**
 * Get auth token from localStorage
 */
function getToken() {
  // Try new format first
  const token = localStorage.getItem("authToken");
  if (token) return token;
  
  // Fallback to old format
  const authData = localStorage.getItem("auth");
  if (!authData) return null;
  try {
    const parsed = JSON.parse(authData);
    return parsed.token;
  } catch {
    return null;
  }
}

/**
 * Create headers with auth token
 */
function getHeaders(includeContentType = true) {
  const headers = new Headers();
  const token = getToken();
  if (token) {
    headers.set("Authorization", `Bearer ${token}`);
  }
  if (includeContentType) {
    headers.set("Content-Type", "application/json");
  }
  return headers;
}

/**
 * Handle API errors
 */
async function handleResponse(response) {
  console.log(`[api.js] Response status: ${response.status}, ok: ${response.ok}`);
  if (!response.ok) {
    // Handle 401 Unauthorized - redirect to login
    if (response.status === 401) {
      console.warn("[api.js] 401 Unauthorized - clearing auth and redirecting to login");
      localStorage.removeItem("authToken");
      localStorage.removeItem("auth");
      // Redirect to login page
      if (!window.location.hash.includes("#/login")) {
        window.location.hash = "#/login";
      }
    }
    const errorText = await response.text();
    console.error(`[api.js] API Error ${response.status}: ${errorText}`);
    throw new Error(`API Error ${response.status}: ${errorText}`);
  }
  const contentType = response.headers.get("content-type");
  console.log(`[api.js] Content-Type: ${contentType}`);
  if (contentType && contentType.includes("application/json")) {
    const data = await response.json();
    console.log(`[api.js] JSON data:`, data);
    console.log(`[api.js] Is Array?`, Array.isArray(data));
    console.log(`[api.js] Type:`, typeof data);
    return data;
  }
  console.log(`[api.js] Non-JSON response, returning raw response`);
  return response;
}

// ============================================
// AUTH ENDPOINTS
// ============================================

export const auth = {
  async login(username, password, totp_code = null) {
    const response = await fetch(`${API_BASE}/auth/login`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password, totp_code }),
    });
    return handleResponse(response);
  },

  async register(username, password) {
    const response = await fetch(`${API_BASE}/auth/register`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    });
    return handleResponse(response);
  },

  async me() {
    const response = await fetch(`${API_BASE}/auth/me`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  async changePassword(oldPassword, newPassword) {
    const response = await fetch(`${API_BASE}/auth/change-password`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ old_password: oldPassword, new_password: newPassword }),
    });
    return handleResponse(response);
  },

  async setup2FA() {
    const response = await fetch(`${API_BASE}/auth/2fa/setup`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  async enable2FA(totpCode) {
    const response = await fetch(`${API_BASE}/auth/2fa/enable`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ totp_code: totpCode }),
    });
    return handleResponse(response);
  },

  async disable2FA(password) {
    const response = await fetch(`${API_BASE}/auth/2fa/disable`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ password }),
    });
    return handleResponse(response);
  },
};

// ============================================
// USER ENDPOINTS
// ============================================

export const users = {
  /**
   * Get user profile
   */
  async getProfile() {
    const response = await fetch(`${API_BASE}/users/profile`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update user profile
   */
  async updateProfile(data) {
    const response = await fetch(`${API_BASE}/users/profile`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get user settings (theme, language, view preference)
   */
  async getSettings() {
    const response = await fetch(`${API_BASE}/users/settings`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update user settings
   */
  async updateSettings(data) {
    const response = await fetch(`${API_BASE}/users/settings`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get user preferences (client-specific settings)
   */
  async getPreferences() {
    const response = await fetch(`${API_BASE}/users/preferences`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update user preferences
   */
  async updatePreferences(data) {
    const response = await fetch(`${API_BASE}/users/preferences`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },
};

// ============================================
// FILE ENDPOINTS
// ============================================

export const files = {
  /**
   * List files in a directory
   */
  async list(path = "") {
    // Clean and encode path
    const cleanPath = path.replace(/^\/+|\/+$/g, ''); // Remove leading/trailing slashes
    
    // For root directory, use /api/files (no trailing slash)
    // For subdirectories, use /api/files/{path}
    const url = cleanPath 
      ? `${API_BASE}/files/${cleanPath.split('/').map(segment => encodeURIComponent(segment)).join('/')}`
      : `${API_BASE}/files`;
    
    const response = await fetch(url, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Download a file
   */
  async download(path) {
    const cleanPath = path.replace(/^\/+|\/+$/g, '');
    const encodedPath = cleanPath
      .split('/')
      .map(segment => encodeURIComponent(segment))
      .join('/');
    const response = await fetch(`${API_BASE}/file/${encodedPath}`, {
      headers: getHeaders(false),
    });
    if (!response.ok) {
      throw new Error(`Download failed: ${response.status}`);
    }
    return response.blob();
  },

  /**
   * Upload a file
   */
  async upload(path, file) {
    // Don't encode the entire path - split by /, encode segments, rejoin
    const cleanPath = path.replace(/^\/+/, ''); // Remove leading slashes
    const encodedPath = cleanPath
      .split('/')
      .map(segment => encodeURIComponent(segment))
      .join('/');
    
    const headers = getHeaders(false);
    
    const response = await fetch(`${API_BASE}/upload/${encodedPath}`, {
      method: "POST",
      headers,
      body: file,
    });
    return handleResponse(response);
  },

  /**
   * Upload a file with FormData and progress tracking
   */
  async uploadWithProgress(path, file, onProgress) {
    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest();
      
      // Progress tracking
      if (onProgress) {
        xhr.upload.addEventListener('progress', (e) => {
          if (e.lengthComputable) {
            const percentComplete = (e.loaded / e.total) * 100;
            onProgress(percentComplete, e.loaded, e.total);
          }
        });
      }
      
      // Response handling
      xhr.addEventListener('load', () => {
        if (xhr.status >= 200 && xhr.status < 300) {
          try {
            const data = JSON.parse(xhr.responseText);
            resolve(data);
          } catch {
            resolve({ success: true });
          }
        } else {
          reject(new Error(`Upload failed: ${xhr.status}`));
        }
      });
      
      xhr.addEventListener('error', () => {
        reject(new Error('Upload failed'));
      });
      
      xhr.addEventListener('abort', () => {
        reject(new Error('Upload cancelled'));
      });
      
      // Prepare FormData
      const formData = new FormData();
      formData.append('file', file);
      formData.append('path', path);
      
      // Open and send request
      xhr.open('POST', `${API_BASE}/upload-multipart`);
      
      // Add auth header
      const token = getToken();
      if (token) {
        xhr.setRequestHeader('Authorization', `Bearer ${token}`);
      }
      
      xhr.send(formData);
    });
  },

  /**
   * Delete a file or directory
   */
  async delete(path) {
    const cleanPath = path.replace(/^\/+|\/+$/g, '');
    // Don't manually encode - fetch API does it automatically
    const response = await fetch(`${API_BASE}/files/${cleanPath}`, {
      method: "DELETE",
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Create a directory
   */
  async createDir(path) {
    const cleanPath = path.replace(/^\/+|\/+$/g, '');
    // Changed to use /dirs/create with path in body instead of URL
    // Backend cannot support /dirs/{*path} catch-all due to Axum route conflicts
    const response = await fetch(`${API_BASE}/dirs/create`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ path: cleanPath }),
    });
    return handleResponse(response);
  },

  /**
   * Rename/move a file
   */
  async rename(oldPath, newPath) {
    const cleanOld = oldPath.replace(/^\/+|\/+$/g, '');
    // Don't manually encode - fetch API does it automatically
    const response = await fetch(`${API_BASE}/rename/${cleanOld}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify({ new_path: newPath }),
    });
    return handleResponse(response);
  },

  /**
   * Move a file to a new location
   */
  async move(oldPath, newPath) {
    const cleanOld = oldPath.replace(/^\/+|\/+$/g, '');
    const response = await fetch(`${API_BASE}/move/${cleanOld}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify({ new_path: newPath }),
    });
    return handleResponse(response);
  },

  /**
   * Copy a file to a new location
   */
  async copy(sourcePath, destPath) {
    const cleanSource = sourcePath.replace(/^\/+|\/+$/g, '');
    const response = await fetch(`${API_BASE}/copy/${cleanSource}`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ new_path: destPath }),
    });
    return handleResponse(response);
  },

  /**
   * Get thumbnail for a file
   */
  async getThumbnail(filePath) {
    // Generate a file ID from the path (simple hash)
    const fileId = btoa(filePath).replace(/[^a-zA-Z0-9]/g, '');
    const response = await fetch(`${API_BASE}/thumbnails/${fileId}`, {
      headers: getHeaders(false),
    });
    if (!response.ok) {
      throw new Error(`Thumbnail not available: ${response.status}`);
    }
    return response.blob();
  },

  /**
   * Get thumbnail URL for a file
   */
  getThumbnailUrl(filePath) {
    const fileId = btoa(filePath).replace(/[^a-zA-Z0-9]/g, '');
    return `${API_BASE}/thumbnails/${fileId}`;
  },
};

// ============================================
// SEARCH ENDPOINT
// ============================================

export const search = {
  /**
   * Search for files
   */
  async query(q, limit = 50, fuzzy = true) {
    const params = new URLSearchParams({
      q,
      limit: limit.toString(),
      fuzzy: fuzzy.toString(),
    });
    const response = await fetch(`${API_BASE}/search?${params}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },
};

// ============================================
// ACTIVITY LOG / AUDIT TRAIL
// ============================================

export const activity = {
  /**
   * List activity logs for current user
   */
  async list(limit = 100, offset = 0, actionFilter = null) {
    const params = new URLSearchParams({
      limit: limit.toString(),
      offset: offset.toString(),
    });
    if (actionFilter) {
      params.append('action', actionFilter);
    }
    
    const response = await fetch(`${API_BASE}/activity?${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get activity statistics
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/activity/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================
// COMMENTS & TAGS
// ============================================

export const comments = {
  async create(req) {
    const response = await fetch(`${API_BASE}/comments`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(req),
    });
    return handleResponse(response);
  },

  async list(file_path) {
    const params = new URLSearchParams({ file_path });
    const response = await fetch(`${API_BASE}/comments?${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  async delete(id) {
    const response = await fetch(`${API_BASE}/comments/${encodeURIComponent(id)}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  }
};

export const tags = {
  async list() {
    const response = await fetch(`${API_BASE}/tags`, { headers: getHeaders() });
    return handleResponse(response);
  },

  async create(req) {
    const response = await fetch(`${API_BASE}/tags`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(req),
    });
    return handleResponse(response);
  },

  async delete(id) {
    const response = await fetch(`${API_BASE}/tags/${encodeURIComponent(id)}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  async tagFile(req) {
    const response = await fetch(`${API_BASE}/file-tags`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(req),
    });
    return handleResponse(response);
  },

  async untagFile(file_tag_id) {
    const response = await fetch(`${API_BASE}/file-tags/${encodeURIComponent(file_tag_id)}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  }
};

// ============================================
// CONFIG & PEERS
// ============================================

export const config = {
  async get() {
    const response = await fetch(`${API_BASE}/config`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  async update(newConfig) {
    const response = await fetch(`${API_BASE}/config`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(newConfig),
    });
    return handleResponse(response);
  },
};

export const peers = {
  async list() {
    const response = await fetch(`${API_BASE}/peers`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  async add(peer) {
    const response = await fetch(`${API_BASE}/peers`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(peer),
    });
    return handleResponse(response);
  },
};

// ============================================
// BATCH OPERATIONS
// ============================================

export const batch = {
  /**
   * Batch copy files to target folder with progress tracking
   */
  async copy(items, targetFolder) {
    const response = await fetch(`${API_BASE}/batch/copy`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ 
        items: items.map(item => ({
          path: item.path,
          name: item.name,
          size: item.size
        })), 
        target_folder: targetFolder 
      }),
    });
    return handleResponse(response);
  },

  /**
   * Batch compress files into archive
   */
  async compress(items, archiveName, compressionLevel = 6) {
    const response = await fetch(`${API_BASE}/batch/compress`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ 
        items: items.map(item => ({
          path: item.path,
          name: item.name,
          size: item.size
        })), 
        archive_name: archiveName,
        compression_level: compressionLevel
      }),
    });
    return handleResponse(response);
  },

  /**
   * Get status of batch operation
   */
  async getOperationStatus(jobId) {
    const response = await fetch(`${API_BASE}/batch/operations/${jobId}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Cancel running batch operation
   */
  async cancelOperation(jobId) {
    const response = await fetch(`${API_BASE}/batch/operations/${jobId}/cancel`, {
      method: "POST",
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },
};

// ============================================
// WEBSOCKET
// ============================================

export function createWebSocket() {
  const ws = new WebSocket("ws://localhost:8080/api/ws");
  return ws;
}

const performance = {
  // Get current performance metrics
  getMetrics: async () => {
    const response = await fetch(`${API_BASE}/performance/metrics`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // Get performance history
  getHistory: async (limit = 50) => {
    const response = await fetch(`${API_BASE}/performance/metrics/history?limit=${limit}`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // Get cache statistics
  getCacheStats: async () => {
    const response = await fetch(`${API_BASE}/performance/cache/stats`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // Clear cache
  clearCache: async () => {
    const response = await fetch(`${API_BASE}/performance/cache/clear`, {
      method: 'POST',
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // Get background jobs
  getBackgroundJobs: async () => {
    const response = await fetch(`${API_BASE}/performance/jobs`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // Queue background job
  queueJob: async (jobData) => {
    const response = await fetch(`${API_BASE}/performance/jobs`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(jobData)
    });
    return handleResponse(response);
  },

  // Get job status
  getJobStatus: async (jobId) => {
    const response = await fetch(`${API_BASE}/performance/jobs/${jobId}/status`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // Get system info
  getSystemInfo: async () => {
    const response = await fetch(`${API_BASE}/performance/system/info`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  }
};

// ============================================
// FAVORITES ENDPOINTS
// ============================================

export const favorites = {
  /**
   * List all favorites for current user
   */
  async list() {
    const response = await fetch(`${API_BASE}/favorites/list`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Add file or folder to favorites
   * @param {string} itemId - File or folder path/ID
   * @param {string} itemType - 'file' or 'folder'
   */
  async add(itemId, itemType = 'file') {
    const response = await fetch(`${API_BASE}/favorites/add`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ item_id: itemId, item_type: itemType }),
    });
    return handleResponse(response);
  },

  /**
   * Remove item from favorites
   * @param {string} favoriteId - ID of the favorite record
   */
  async remove(favoriteId) {
    const response = await fetch(`${API_BASE}/favorites/${favoriteId}/remove`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Check if an item is favorited
   * @param {string} itemId - File or folder path/ID
   */
  async isFavorite(itemId) {
    try {
      const favorites = await this.list();
      return favorites.some(fav => fav.item_id === itemId);
    } catch (err) {
      console.error("Failed to check favorite status:", err);
      return false;
    }
  }
};

// ============================================
// GENERIC API METHODS (for new endpoints)
// ============================================

export default {
  auth,
  users,
  files,
  search,
  config,
  peers,
  comments,
  tags,
  batch,
  performance,
  favorites,  // Add favorites to default export
  
  // Recent files endpoints
  recent: {
    async list(limit = 50) {
      const response = await fetch(`${API_BASE}/recent?limit=${limit}`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    },
    
    async accessed(limit = 50) {
      const response = await fetch(`${API_BASE}/recent/accessed?limit=${limit}`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    },
    
    async uploaded(limit = 50) {
      const response = await fetch(`${API_BASE}/recent/uploaded?limit=${limit}`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    }
  },
  
  // System endpoints
  system: {
    async storage() {
      const response = await fetch(`${API_BASE}/system/storage`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    },
    
    async stats() {
      const response = await fetch(`${API_BASE}/stats`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    }
  },
  
  sharing: {
    // Create a share for a file or folder
    async create(fileId = null, folderId = null, options = {}) {
      const response = await fetch(`${API_BASE}/shares`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          file_id: fileId,
          folder_id: folderId,
          shared_with: options.sharedWith || null, // null = public link
          expires_in_days: options.expiresInDays || null,
          can_read: options.canRead !== false,
          can_write: options.canWrite || false,
          can_delete: options.canDelete || false,
          can_share: options.canShare || false
        })
      });
      return handleResponse(response);
    },

    // List shares created by current user
    async list() {
      const response = await fetch(`${API_BASE}/shares`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    },

    // List shares shared with current user
    async listSharedWithMe() {
      const response = await fetch(`${API_BASE}/shared-with-me`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    },

    // Delete a share
    async delete(shareId) {
      const response = await fetch(`${API_BASE}/shares/${shareId}`, {
        method: 'DELETE',
        headers: getHeaders()
      });
      return handleResponse(response);
    },

    // Update share permissions
    async updatePermissions(shareId, permissions) {
      const response = await fetch(`${API_BASE}/shares/${shareId}/permissions`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(permissions)
      });
      return handleResponse(response);
    }
  },
  
  // File versioning endpoints
  versions: {
    // List all versions for a file
    async list(fileId) {
      const response = await fetch(`${API_BASE}/versions/${fileId}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },

    // Create a new version of a file
    async create(fileId, versionData) {
      const response = await fetch(`${API_BASE}/versions/${fileId}/create`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(versionData)
      });
      return { data: await handleResponse(response) };
    },

    // Get specific version details
    async get(versionId) {
      const response = await fetch(`${API_BASE}/versions/${versionId}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },

    // Delete a version (not current)
    async delete(versionId) {
      const response = await fetch(`${API_BASE}/versions/${versionId}`, {
        method: 'DELETE',
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },

    // Restore a version (creates new current version)
    async restore(versionId, options = {}) {
      const response = await fetch(`${API_BASE}/versions/${versionId}/restore`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(options)
      });
      return { data: await handleResponse(response) };
    },

    // Get diff between two versions
    async getDiff(fromVersionId, toVersionId) {
      const response = await fetch(`${API_BASE}/versions/${fromVersionId}/diff/${toVersionId}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },

    // Download a specific version
    async download(versionId) {
      const response = await fetch(`${API_BASE}/versions/${versionId}/download`, {
        headers: getHeaders(false)
      });
      if (!response.ok) {
        throw new Error(`Download failed: ${response.statusText}`);
      }
      return { data: await response.blob() };
    },

    // Get version metadata
    async getMetadata(versionId) {
      const response = await fetch(`${API_BASE}/versions/${versionId}/metadata`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },

    // Update version metadata
    async updateMetadata(versionId, metadata) {
      const response = await fetch(`${API_BASE}/versions/${versionId}/metadata`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify({ metadata })
      });
      return { data: await handleResponse(response) };
    }
  },
  
  // Duplicate files detection endpoints
  duplicates: {
    // Find all duplicate files
    async find(minSizeBytes = 0) {
      const response = await fetch(`${API_BASE}/duplicates?min_size_bytes=${minSizeBytes}`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    },
    
    // Get duplicate statistics
    async stats() {
      const response = await fetch(`${API_BASE}/duplicates/stats`, {
        headers: getHeaders()
      });
      return handleResponse(response);
    },
    
    // Resolve duplicates by keeping one and deleting others
    async resolve(checksum, keepFileId, deleteFileIds) {
      const response = await fetch(`${API_BASE}/duplicates/resolve`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          checksum,
          keep_file_id: keepFileId,
          delete_file_ids: deleteFileIds
        })
      });
      return handleResponse(response);
    }
  },
  
  // Collaboration endpoints
  collaboration: {
    // File Locks
    async listLocks(filePath = null) {
      const params = filePath ? `?file_path=${encodeURIComponent(filePath)}` : '';
      const response = await fetch(`${API_BASE}/collaboration/locks${params}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    async acquireLock(filePath, lockType = 'exclusive', durationSeconds = 300) {
      const response = await fetch(`${API_BASE}/collaboration/locks`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          file_path: filePath,
          lock_type: lockType,
          duration_seconds: durationSeconds
        })
      });
      return { data: await handleResponse(response) };
    },
    
    async releaseLock(lockId) {
      const response = await fetch(`${API_BASE}/collaboration/locks/${lockId}`, {
        method: 'DELETE',
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    async renewLock(lockId) {
      const response = await fetch(`${API_BASE}/collaboration/locks/${lockId}/heartbeat`, {
        method: 'POST',
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    // User Presence
    async getPresence(filePath = null) {
      const params = filePath ? `?file_path=${encodeURIComponent(filePath)}` : '';
      const response = await fetch(`${API_BASE}/collaboration/presence${params}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    async updatePresence(filePath, activityType, metadata = null) {
      const response = await fetch(`${API_BASE}/collaboration/presence`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          file_path: filePath,
          activity_type: activityType,
          metadata
        })
      });
      return { data: await handleResponse(response) };
    },
    
    async removePresence(userId) {
      const response = await fetch(`${API_BASE}/collaboration/presence/${userId}`, {
        method: 'DELETE',
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    // Activity
    async getActivity(limit = 50) {
      const response = await fetch(`${API_BASE}/collaboration/activity?limit=${limit}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    async getFileActivity(filePath) {
      const response = await fetch(`${API_BASE}/collaboration/activity/${encodeURIComponent(filePath)}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    // Conflicts
    async listConflicts(status = 'pending') {
      const response = await fetch(`${API_BASE}/collaboration/conflicts?status=${status}`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },
    
    async resolveConflict(conflictId, resolutionStrategy, details = null) {
      const response = await fetch(`${API_BASE}/collaboration/conflicts/${conflictId}/resolve`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          resolution_strategy: resolutionStrategy,
          details
        })
      });
      return { data: await handleResponse(response) };
    }
  },
  
  createWebSocket,
  // Trash endpoints
  listTrash: () => fetch(`${API_BASE}/trash`, { headers: getHeaders() }),
  restoreTrash: (path) => fetch(`${API_BASE}/trash/restore/${encodeURIComponent(path)}`, { 
    method: 'POST', 
    headers: getHeaders() 
  }),
  permanentDeleteTrash: (path) => fetch(`${API_BASE}/trash/permanent/${encodeURIComponent(path)}`, { 
    method: 'DELETE', 
    headers: getHeaders() 
  }),
  cleanupTrash: () => fetch(`${API_BASE}/trash/cleanup`, { 
    method: 'DELETE', 
    headers: getHeaders() 
  }),
  emptyTrash: () => fetch(`${API_BASE}/trash/empty`, { 
    method: 'DELETE', 
    headers: getHeaders() 
  }),
};

// ============================================
// BACKUP ENDPOINTS
// ============================================

export const backup = {
  /**
   * List all backups
   */
  async list() {
    const response = await fetch(`${API_BASE}/backups`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new backup
   * @param {string} backupType - Type of backup: 'full', 'database', 'files'
   * @param {boolean} includeVersions - Include version history
   */
  async create(backupType = 'full', includeVersions = true) {
    const response = await fetch(`${API_BASE}/backups/create`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({ 
        backup_type: backupType,
        include_versions: includeVersions 
      }),
    });
    return handleResponse(response);
  },

  /**
   * Get backup details
   * @param {string} backupId - Backup ID
   */
  async get(backupId) {
    const response = await fetch(`${API_BASE}/backups/${backupId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Delete a backup
   * @param {string} backupId - Backup ID
   */
  async delete(backupId) {
    const response = await fetch(`${API_BASE}/backups/${backupId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Verify a backup
   * @param {string} backupId - Backup ID
   * @param {string} verificationType - 'checksum', 'file_count', 'restore_test'
   */
  async verify(backupId, verificationType = 'checksum') {
    const response = await fetch(`${API_BASE}/backups/${backupId}/verify`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({ verification_type: verificationType }),
    });
    return handleResponse(response);
  },

  /**
   * List verifications for a backup
   * @param {string} backupId - Backup ID
   */
  async listVerifications(backupId) {
    const response = await fetch(`${API_BASE}/backups/${backupId}/verifications`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Clean up old backups based on retention policies
   */
  async cleanup() {
    const response = await fetch(`${API_BASE}/backups/cleanup`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================
// BACKUP SCHEDULE ENDPOINTS
// ============================================

export const backupSchedules = {
  /**
   * List all backup schedules
   */
  async list() {
    const response = await fetch(`${API_BASE}/backups/schedules`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new backup schedule
   * @param {Object} schedule - Schedule configuration
   */
  async create(schedule) {
    const response = await fetch(`${API_BASE}/backups/schedules`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(schedule),
    });
    return handleResponse(response);
  },

  /**
   * Get schedule details
   * @param {string} scheduleId - Schedule ID
   */
  async get(scheduleId) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update a backup schedule
   * @param {string} scheduleId - Schedule ID
   * @param {Object} updates - Schedule updates
   */
  async update(scheduleId, updates) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(updates),
    });
    return handleResponse(response);
  },

  /**
   * Delete a backup schedule
   * @param {string} scheduleId - Schedule ID
   */
  async delete(scheduleId) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Trigger a schedule manually
   * @param {string} scheduleId - Schedule ID
   */
  async trigger(scheduleId) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}/trigger`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================
// SYSTEM ENDPOINTS
// ============================================

export const system = {
  /**
   * Get system storage information (disk usage)
   */
  async getStorageInfo() {
    const response = await fetch(`${API_BASE}/system/storage`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get file statistics (count, size, etc.)
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get server status
   */
  async getStatus() {
    const response = await fetch(`http://localhost:8080/status/json`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================
// SHARES ENDPOINTS
// ============================================

export const shares = {
  /**
   * List all shares
   */
  async list() {
    const response = await fetch(`${API_BASE}/shares`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new share
   * @param {Object} shareData - { file_path, permission, expires_at?, password? }
   */
  async create(shareData) {
    const response = await fetch(`${API_BASE}/shares`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(shareData),
    });
    return handleResponse(response);
  },

  /**
   * Get share details
   * @param {string} shareId - Share ID or token
   */
  async get(shareId) {
    const response = await fetch(`${API_BASE}/shares/${shareId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update a share
   * @param {string} shareId - Share ID
   * @param {Object} updates - { permission?, expires_at? }
   */
  async update(shareId, updates) {
    const response = await fetch(`${API_BASE}/shares/${shareId}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(updates),
    });
    return handleResponse(response);
  },

  /**
   * Delete a share
   * @param {string} shareId - Share ID
   */
  async delete(shareId) {
    const response = await fetch(`${API_BASE}/shares/${shareId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================
// GENERIC API METHODS (for new endpoints)
// ============================================

/**
 * Generic GET request
 */
export async function get(endpoint) {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    headers: getHeaders(),
  });
  return handleResponse(response);
}

/**
 * Generic POST request
 */
export async function post(endpoint, data) {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    method: 'POST',
    headers: getHeaders(),
    body: JSON.stringify(data),
  });
  return handleResponse(response);
}

/**
 * Generic PUT request
 */
export async function put(endpoint, data = null) {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    method: 'PUT',
    headers: getHeaders(),
    body: data ? JSON.stringify(data) : null,
  });
  return handleResponse(response);
}

/**
 * Generic DELETE request
 */
export async function deleteRequest(endpoint) {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    method: 'DELETE',
    headers: getHeaders(),
  });
  return handleResponse(response);
}
