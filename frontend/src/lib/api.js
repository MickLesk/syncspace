/**
 * API Service for SyncSpace
 * Handles all HTTP requests to the backend
 */

// Central API configuration - supports dynamic server selection
function getApiBase() {
  return localStorage.getItem("syncspace_api_base") ||
    import.meta.env.VITE_API_URL ||
    "http://localhost:8080/api";
}

function getApiHost() {
  return localStorage.getItem("syncspace_api_host") ||
    import.meta.env.VITE_API_HOST ||
    "http://localhost:8080";
}

// Dynamic getters that always return fresh values
// Note: API_BASE/API_HOST are evaluated once at module load for backwards compatibility
// After login, page reloads and they get re-evaluated with the new server URL
export const API_BASE = getApiBase();
export const API_HOST = getApiHost();

// Helper functions to get current values (always fresh)
export function getCurrentApiBase() {
  return getApiBase();
}

export function getCurrentApiHost() {
  return getApiHost();
}

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

  async register(username, password, email = null) {
    const body = { username, password };
    if (email) body.email = email;
    const response = await fetch(`${API_BASE}/auth/register`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
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
      body: JSON.stringify({ preferences: data }),
    });
    return handleResponse(response);
  },

  /**
   * List all users (for sharing dialogs)
   */
  async listAll(roleFilter = null, statusFilter = null) {
    const params = new URLSearchParams();
    if (roleFilter) params.append('role', roleFilter);
    if (statusFilter) params.append('status', statusFilter);
    const queryString = params.toString() ? `?${params.toString()}` : '';

    const response = await fetch(`${API_BASE}/users/list${queryString}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // ========== Admin User Management ==========

  /**
   * Get all users (admin)
   */
  async getAll() {
    const response = await fetch(`${API_BASE}/admin/users`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new user (admin)
   */
  async create(userData) {
    const response = await fetch(`${API_BASE}/admin/users`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(userData),
    });
    return handleResponse(response);
  },

  /**
   * Update a user (admin)
   */
  async update(userId, userData) {
    const response = await fetch(`${API_BASE}/admin/users/${userId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(userData),
    });
    return handleResponse(response);
  },

  /**
   * Delete a user (admin)
   */
  async delete(userId) {
    const response = await fetch(`${API_BASE}/admin/users/${userId}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Reset user password (admin)
   */
  async resetPassword(userId, newPassword) {
    const response = await fetch(`${API_BASE}/admin/users/${userId}/reset-password`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ password: newPassword }),
    });
    return handleResponse(response);
  },

  /**
   * Force password change on next login (admin)
   */
  async forcePasswordChange(userId) {
    const response = await fetch(`${API_BASE}/admin/users/${userId}/force-password-change`, {
      method: "POST",
      headers: getHeaders(),
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
    // Ensure path is a string
    const pathStr = String(path || "");

    // Clean and encode path
    const cleanPath = pathStr.replace(/^\/+|\/+$/g, ''); // Remove leading/trailing slashes

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
   * Automatically uses chunked upload for files >100MB
   */
  async uploadWithProgress(path, file, onProgress, onPause = null) {
    const CHUNK_SIZE = 10 * 1024 * 1024; // 10MB chunks
    const LARGE_FILE_THRESHOLD = 100 * 1024 * 1024; // 100MB

    // Use chunked upload for large files
    if (file.size > LARGE_FILE_THRESHOLD) {
      return this.uploadChunked(path, file, onProgress, onPause);
    }

    // Standard upload for smaller files
    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest();

      // Store XHR for pause/cancel functionality
      if (onPause) {
        onPause(() => {
          xhr.abort();
          reject(new Error('Upload paused by user'));
        });
      }

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
        reject(new Error('Network error during upload'));
      });

      xhr.addEventListener('abort', () => {
        reject(new Error('Upload cancelled'));
      });

      xhr.addEventListener('timeout', () => {
        reject(new Error('Upload timeout'));
      });

      // Prepare FormData
      const formData = new FormData();
      formData.append('file', file);
      formData.append('path', path);

      // Open and send request
      xhr.open('POST', `${API_BASE}/upload-multipart`);
      xhr.timeout = 300000; // 5 minute timeout

      // Add auth header
      const token = getToken();
      if (token) {
        xhr.setRequestHeader('Authorization', `Bearer ${token}`);
      }

      xhr.send(formData);
    });
  },

  /**
   * Chunked upload for large files (>100MB)
   * Supports pause/resume functionality
   */
  async uploadChunked(path, file, onProgress, onPause = null) {
    const CHUNK_SIZE = 10 * 1024 * 1024; // 10MB chunks
    const totalChunks = Math.ceil(file.size / CHUNK_SIZE);
    const uploadId = `${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    let isPaused = false;
    let currentChunk = 0;

    // Restore from localStorage if resuming
    const savedProgress = this.getUploadProgress(file.name);
    if (savedProgress && savedProgress.uploadId === uploadId) {
      currentChunk = savedProgress.currentChunk;
      console.log(`📦 Resuming chunked upload from chunk ${currentChunk}/${totalChunks}`);
    }

    // Setup pause handler
    if (onPause) {
      onPause(() => {
        isPaused = true;
        this.saveUploadProgress(file.name, {
          uploadId,
          currentChunk,
          totalChunks,
          path,
          timestamp: Date.now()
        });
        throw new Error('Upload paused');
      });
    }

    try {
      // Upload chunks sequentially
      while (currentChunk < totalChunks) {
        if (isPaused) {
          throw new Error('Upload paused by user');
        }

        const start = currentChunk * CHUNK_SIZE;
        const end = Math.min(start + CHUNK_SIZE, file.size);
        const chunk = file.slice(start, end);

        // Upload chunk
        await this.uploadChunk(path, file.name, chunk, currentChunk, totalChunks, uploadId);

        currentChunk++;

        // Update progress
        if (onProgress) {
          const percentComplete = (currentChunk / totalChunks) * 100;
          const bytesUploaded = Math.min(currentChunk * CHUNK_SIZE, file.size);
          onProgress(percentComplete, bytesUploaded, file.size);
        }

        // Save progress to localStorage
        this.saveUploadProgress(file.name, {
          uploadId,
          currentChunk,
          totalChunks,
          path,
          timestamp: Date.now()
        });
      }

      // Finalize upload on backend
      await this.finalizeChunkedUpload(path, file.name, uploadId, totalChunks);

      // Clear saved progress
      this.clearUploadProgress(file.name);

      return { success: true, message: 'Chunked upload complete' };

    } catch (error) {
      if (error.message !== 'Upload paused' && error.message !== 'Upload paused by user') {
        // Clear progress on error (but not on pause)
        this.clearUploadProgress(file.name);
      }
      throw error;
    }
  },

  /**
   * Upload a single chunk
   */
  async uploadChunk(path, fileName, chunk, chunkIndex, totalChunks, uploadId) {
    const formData = new FormData();
    formData.append('chunk', chunk);
    formData.append('path', path);
    formData.append('fileName', fileName);
    formData.append('chunkIndex', chunkIndex.toString());
    formData.append('totalChunks', totalChunks.toString());
    formData.append('uploadId', uploadId);

    const response = await fetch(`${API_BASE}/upload-chunk`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${getToken()}`
      },
      body: formData
    });

    if (!response.ok) {
      throw new Error(`Chunk upload failed: ${response.status}`);
    }

    return response.json();
  },

  /**
   * Finalize chunked upload (merge chunks on backend)
   */
  async finalizeChunkedUpload(path, fileName, uploadId, totalChunks) {
    const response = await fetch(`${API_BASE}/upload-chunk-finalize`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({
        path,
        fileName,
        uploadId,
        totalChunks
      })
    });

    return handleResponse(response);
  },

  /**
   * Save upload progress to localStorage
   */
  saveUploadProgress(fileName, progress) {
    try {
      const key = `upload_progress_${fileName}`;
      localStorage.setItem(key, JSON.stringify(progress));
    } catch (error) {
      console.error('Failed to save upload progress:', error);
    }
  },

  /**
   * Get upload progress from localStorage
   */
  getUploadProgress(fileName) {
    try {
      const key = `upload_progress_${fileName}`;
      const saved = localStorage.getItem(key);
      if (saved) {
        const progress = JSON.parse(saved);
        // Only use if less than 24 hours old
        if (Date.now() - progress.timestamp < 24 * 60 * 60 * 1000) {
          return progress;
        }
        // Clear old progress
        this.clearUploadProgress(fileName);
      }
    } catch (error) {
      console.error('Failed to get upload progress:', error);
    }
    return null;
  },

  /**
   * Clear upload progress from localStorage
   */
  clearUploadProgress(fileName) {
    try {
      const key = `upload_progress_${fileName}`;
      localStorage.removeItem(key);
    } catch (error) {
      console.error('Failed to clear upload progress:', error);
    }
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
   * Get version history for a file
   */
  async getVersions(filePath) {
    const cleanPath = filePath.replace(/^\/+|\/+$/g, '');
    const encodedPath = cleanPath
      .split('/')
      .map(segment => encodeURIComponent(segment))
      .join('/');
    const response = await fetch(`${API_BASE}/files/${encodedPath}/versions`, {
      headers: getHeaders(),
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
// DIRECTORIES ENDPOINT
// ============================================

export const directories = {
  /**
   * Create a new directory
   */
  async create(data) {
    const response = await fetch(`${API_BASE}/dirs/create`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get full directory tree recursively
   */
  async getTree() {
    const response = await fetch(`${API_BASE}/dirs/tree`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * List all directories recursively (alias for getTree)
   */
  async listAll() {
    return this.getTree();
  },

  /**
   * Get directory info
   */
  async getInfo(path) {
    const encodedPath = encodeURIComponent(path);
    const response = await fetch(`${API_BASE}/dirs/${encodedPath}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
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

  /**
   * Get search suggestions for autocomplete
   */
  async suggest(prefix, limit = 10) {
    const params = new URLSearchParams({
      q: prefix,
      limit: limit.toString(),
    });
    const response = await fetch(`${API_BASE}/search/suggest?${params}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get search facets (aggregations by file type, size, date)
   */
  async facets(query = '') {
    const params = new URLSearchParams({
      q: query,
    });
    const response = await fetch(`${API_BASE}/search/facets?${params}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Reindex all files (admin operation)
   */
  async reindex() {
    const response = await fetch(`${API_BASE}/search/reindex`, {
      method: 'POST',
      headers: getHeaders(),
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

  /**
   * Mark activity as visited
   */
  async markVisited() {
    const response = await fetch(`${API_BASE}/activity/mark-visited`, {
      method: "PUT",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================
// COMMENTS & TAGS
// ============================================

export const comments = {
  /**
   * List all comments for a file
   */
  async list(file_path) {
    const params = new URLSearchParams({ file_path });
    const response = await fetch(`${API_BASE}/comments?${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new comment
   */
  async create(file_path, content) {
    const response = await fetch(`${API_BASE}/comments`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({ file_path, content }),
    });
    return handleResponse(response);
  },

  /**
   * Update an existing comment
   */
  async update(commentId, file_path, content) {
    const response = await fetch(`${API_BASE}/comments/${encodeURIComponent(commentId)}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify({ file_path, content }),
    });
    return handleResponse(response);
  },

  /**
   * Delete a comment
   */
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

  async getFiles(tagId) {
    const response = await fetch(`${API_BASE}/tags/${encodeURIComponent(tagId)}/files`, {
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
// FOLDER COLORS ENDPOINTS
// ============================================

export const folderColors = {
  /**
   * Get folder color
   */
  async get(folderPath) {
    const response = await fetch(
      `${API_BASE}/folders/color/get`,
      {
        method: "POST",
        headers: getHeaders(),
        body: JSON.stringify({ file_path: folderPath })
      }
    );
    return handleResponse(response);
  },

  /**
   * Set folder color
   */
  async set(folderPath, color) {
    const response = await fetch(
      `${API_BASE}/folders/color`,
      {
        method: "PUT",
        headers: getHeaders(),
        body: JSON.stringify({ file_path: folderPath, color }),
      }
    );
    return handleResponse(response);
  },

  /**
   * Remove folder color
   */
  async remove(folderPath) {
    const response = await fetch(
      `${API_BASE}/folders/color/remove`,
      {
        method: "POST",
        headers: getHeaders(),
        body: JSON.stringify({ file_path: folderPath }),
      }
    );
    return handleResponse(response);
  },
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
// BULK OPERATIONS
// ============================================

export const bulk = {
  /**
   * Create bulk operation job
   */
  async createJob(jobType, filePaths, operation, destination = null, priority = 5) {
    const response = await fetch(`${API_BASE}/bulk/jobs`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({
        job_type: jobType,
        file_paths: filePaths,
        operation: operation,
        destination: destination,
        priority: priority
      }),
    });
    return handleResponse(response);
  },

  /**
   * List all jobs for current user
   */
  async listJobs() {
    const response = await fetch(`${API_BASE}/bulk/jobs`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get specific job status
   */
  async getJobStatus(jobId) {
    const response = await fetch(`${API_BASE}/bulk/jobs/${jobId}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Cancel a job
   */
  async cancelJob(jobId) {
    const response = await fetch(`${API_BASE}/bulk/jobs/${jobId}/cancel`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ reason: "User cancelled" }),
    });
    return handleResponse(response);
  },

  /**
   * Delete a completed job
   */
  async deleteJob(jobId) {
    const response = await fetch(`${API_BASE}/bulk/jobs/${jobId}`, {
      method: "DELETE",
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },
};

// ============================================
// TEMPLATE ENDPOINTS
// ============================================

export const templates = {
  /**
   * List templates with optional filters
   */
  async listTemplates(category = null, isPublic = null, search = null) {
    const params = new URLSearchParams();
    if (category) params.append('category', category);
    if (isPublic !== null) params.append('is_public', isPublic);
    if (search) params.append('search', search);

    const url = `${API_BASE}/templates${params.toString() ? '?' + params.toString() : ''}`;
    const response = await fetch(url, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get a specific template
   */
  async getTemplate(templateId) {
    const response = await fetch(`${API_BASE}/templates/${templateId}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Create a new template
   */
  async createTemplate(data) {
    const response = await fetch(`${API_BASE}/templates`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update a template
   */
  async updateTemplate(templateId, data) {
    const response = await fetch(`${API_BASE}/templates/${templateId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a template
   */
  async deleteTemplate(templateId) {
    const response = await fetch(`${API_BASE}/templates/${templateId}`, {
      method: "DELETE",
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Use a template to create a file
   */
  async useTemplate(templateId, data) {
    const response = await fetch(`${API_BASE}/templates/${templateId}/use`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Toggle favorite status for a template
   */
  async toggleFavorite(templateId) {
    const response = await fetch(`${API_BASE}/templates/${templateId}/favorite`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * List template categories
   */
  async listCategories() {
    const response = await fetch(`${API_BASE}/template-categories`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },
};

// ============================================
// WORKFLOW AUTOMATION ENDPOINTS
// ============================================

export const workflow = {
  /**
   * Create a new workflow rule
   */
  async createRule(data) {
    const response = await fetch(`${API_BASE}/workflows`, {
      method: 'POST',
      headers: getHeaders(true),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * List workflow rules with optional filters
   */
  async listRules(triggerType, actionType, isActive, includeStats = false) {
    const params = new URLSearchParams();
    if (triggerType) params.append('trigger_type', triggerType);
    if (actionType) params.append('action_type', actionType);
    if (isActive !== null && isActive !== undefined) params.append('is_active', isActive);
    if (includeStats) params.append('include_stats', 'true');

    const url = `${API_BASE}/workflows${params.toString() ? '?' + params.toString() : ''}`;
    const response = await fetch(url, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get a specific workflow rule
   */
  async getRule(ruleId) {
    const response = await fetch(`${API_BASE}/workflows/${ruleId}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Update a workflow rule
   */
  async updateRule(ruleId, data) {
    const response = await fetch(`${API_BASE}/workflows/${ruleId}`, {
      method: 'PUT',
      headers: getHeaders(true),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a workflow rule
   */
  async deleteRule(ruleId) {
    const response = await fetch(`${API_BASE}/workflows/${ruleId}`, {
      method: 'DELETE',
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Toggle workflow rule active status
   */
  async toggleRule(ruleId) {
    const response = await fetch(`${API_BASE}/workflows/${ruleId}/toggle`, {
      method: 'POST',
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Manually execute a workflow rule
   */
  async executeRule(ruleId, filePath, triggerContext) {
    const response = await fetch(`${API_BASE}/workflows/${ruleId}/execute`, {
      method: 'POST',
      headers: getHeaders(true),
      body: JSON.stringify({ file_path: filePath, trigger_context: triggerContext }),
    });
    return handleResponse(response);
  },

  /**
   * Get execution history for a specific rule
   */
  async getExecutionHistory(ruleId, limit, offset, status) {
    const params = new URLSearchParams();
    if (limit) params.append('limit', limit);
    if (offset) params.append('offset', offset);
    if (status) params.append('status', status);

    const url = `${API_BASE}/workflows/${ruleId}/executions${params.toString() ? '?' + params.toString() : ''}`;
    const response = await fetch(url, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get recent workflow executions across all rules
   */
  async getRecentExecutions(limit, offset, status) {
    const params = new URLSearchParams();
    if (limit) params.append('limit', limit);
    if (offset) params.append('offset', offset);
    if (status) params.append('status', status);

    const url = `${API_BASE}/workflows/executions/recent${params.toString() ? '?' + params.toString() : ''}`;
    const response = await fetch(url, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * List available trigger types
   */
  async listTriggerTypes() {
    const response = await fetch(`${API_BASE}/workflows/trigger-types`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * List available action types
   */
  async listActionTypes() {
    const response = await fetch(`${API_BASE}/workflows/action-types`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },
};

// ============================================
// RBAC ENDPOINTS
// ============================================

export const rbac = {
  /**
   * List all roles
   */
  async listRoles(includeSystem = true) {
    const params = new URLSearchParams();
    if (includeSystem !== null) params.append('include_system', includeSystem);

    const url = `${API_BASE}/roles${params.toString() ? '?' + params.toString() : ''}`;
    const response = await fetch(url, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get a specific role
   */
  async getRole(roleId) {
    const response = await fetch(`${API_BASE}/roles/${roleId}`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Create a new role
   */
  async createRole(data) {
    const response = await fetch(`${API_BASE}/roles`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update a role
   */
  async updateRole(roleId, data) {
    const response = await fetch(`${API_BASE}/roles/${roleId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a role
   */
  async deleteRole(roleId) {
    const response = await fetch(`${API_BASE}/roles/${roleId}`, {
      method: "DELETE",
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get role permissions
   */
  async getRolePermissions(roleId) {
    const response = await fetch(`${API_BASE}/roles/${roleId}/permissions`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get user's roles
   */
  async getUserRoles(userId) {
    const response = await fetch(`${API_BASE}/users/${userId}/roles`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Assign role to user
   */
  async assignUserRole(userId, data) {
    const response = await fetch(`${API_BASE}/users/${userId}/roles`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Revoke role from user
   */
  async revokeUserRole(userId, roleId) {
    const response = await fetch(`${API_BASE}/users/${userId}/roles/${roleId}`, {
      method: "DELETE",
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get user's effective permissions
   */
  async getUserPermissions(userId) {
    const response = await fetch(`${API_BASE}/users/${userId}/permissions`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * List available permissions
   */
  async listAvailablePermissions() {
    const response = await fetch(`${API_BASE}/permissions/available`, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },

  /**
   * Get permission audit log
   */
  async getPermissionAudit(userId = null, action = null, limit = 100) {
    const params = new URLSearchParams();
    if (userId) params.append('user_id', userId);
    if (action) params.append('action', action);
    if (limit) params.append('limit', limit);

    const url = `${API_BASE}/permissions/audit${params.toString() ? '?' + params.toString() : ''}`;
    const response = await fetch(url, {
      headers: getHeaders(false),
    });
    return handleResponse(response);
  },
};

// ============================================
// WEBSOCKET
// ============================================

// WebSocket base URL (ws:// or wss://)
const WS_BASE = API_HOST.replace(/^http/, 'ws') + '/api/ws';

export function createWebSocket() {
  const ws = new WebSocket(WS_BASE);
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

// Recent files endpoints
export const recent = {
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
};

export const sharing = {
  // Create a share for a file or folder - ENHANCED
  async create(requestData) {
    // Support both old and new API format
    const body = requestData.file_path ? requestData : {
      file_id: requestData.fileId || requestData.file_id,
      folder_id: requestData.folderId || requestData.folder_id,
      shared_with: requestData.sharedWith || null,
      expires_in_days: requestData.expiresInDays || null,
      can_read: requestData.canRead !== false,
      can_write: requestData.canWrite || false,
      can_delete: requestData.canDelete || false,
      can_share: requestData.canShare || false
    };

    const response = await fetch(`${API_BASE}/shares`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(body)
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
  },

  // NEW: Get share users
  async getUsers(shareId) {
    const response = await fetch(`${API_BASE}/shares/${shareId}/users`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // NEW: Add users to share
  async addUsers(shareId, userIds, permissions) {
    const response = await fetch(`${API_BASE}/shares/${shareId}/users`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({
        user_ids: userIds,
        permissions: permissions
      })
    });
    return handleResponse(response);
  },

  // NEW: Remove user from share
  async removeUser(shareId, userId) {
    const response = await fetch(`${API_BASE}/shares/${shareId}/users/${userId}`, {
      method: 'DELETE',
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // NEW: Update user permission on share
  async updateUserPermission(shareId, userId, permission) {
    const response = await fetch(`${API_BASE}/shares/${shareId}/users/${userId}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify({ permission })
    });
    return handleResponse(response);
  },

  // Get share analytics summary
  async getShareAnalytics(shareId) {
    const response = await fetch(`${API_BASE}/shares/${shareId}/analytics`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  },

  // Get detailed access log for share
  async getShareAccessLog(shareId) {
    const response = await fetch(`${API_BASE}/shares/${shareId}/access-log`, {
      headers: getHeaders()
    });
    return handleResponse(response);
  }
};

// File versioning endpoints
export const versions = {
  // List all versions for a file (with tags)
  async list(fileId, includeTags = true) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}?include_tags=${includeTags}`, {
      headers: getHeaders()
    });
    return { data: await handleResponse(response) };
  },

  // Get version timeline with stats
  async getTimeline(fileId) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}/timeline`, {
      headers: getHeaders()
    });
    return { data: await handleResponse(response) };
  },

  // Create a new version of a file
  async create(fileId, versionData) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(versionData)
    });
    return { data: await handleResponse(response) };
  },

  // Update version (comment, pinned, starred)
  async update(fileId, versionId, updateData) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}/${versionId}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(updateData)
    });
    return { data: await handleResponse(response) };
  },

  // Delete a version (not current)
  async delete(fileId, versionId) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}/${versionId}`, {
      method: 'DELETE',
      headers: getHeaders()
    });
    return { data: await handleResponse(response) };
  },

  // Restore a version (creates new current version)
  async restore(fileId, versionId, options = {}) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}/${versionId}/restore`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(options)
    });
    return { data: await handleResponse(response) };
  },

  // Get diff between two versions
  async getDiff(fileId, fromVersionId, toVersionId) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}/${fromVersionId}/diff/${toVersionId}`, {
      headers: getHeaders()
    });
    return { data: await handleResponse(response) };
  },

  // Add tag to version
  async addTag(fileId, versionId, tagData) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}/${versionId}/tags`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(tagData)
    });
    return { data: await handleResponse(response) };
  },

  // Remove tag from version
  async removeTag(fileId, versionId, tagId) {
    const response = await fetch(`${API_BASE}/versions/${encodeURIComponent(fileId)}/${versionId}/tags/${tagId}`, {
      method: 'DELETE',
      headers: getHeaders()
    });
    return { data: await handleResponse(response) };
  },

  // Get tag templates
  async getTagTemplates() {
    const response = await fetch(`${API_BASE}/version-tags/templates`, {
      headers: getHeaders()
    });
    return { data: await handleResponse(response) };
  },

  // Get storage stats for all files
  async getStorageStats() {
    const response = await fetch(`${API_BASE}/version-storage-stats`, {
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
  }
};

// Duplicate files detection endpoints
export const duplicates = {
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
};

// Collaboration endpoints
export const collaboration = {
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
};

// Trash endpoints
export const trash = {
  list: () => fetch(`${API_BASE}/trash`, { headers: getHeaders() }),
  restore: (path) => fetch(`${API_BASE}/trash/restore/${encodeURIComponent(path)}`, {
    method: 'POST',
    headers: getHeaders()
  }),
  permanentDelete: (path) => fetch(`${API_BASE}/trash/permanent/${encodeURIComponent(path)}`, {
    method: 'DELETE',
    headers: getHeaders()
  }),
  cleanup: () => fetch(`${API_BASE}/trash/cleanup`, {
    method: 'DELETE',
    headers: getHeaders()
  }),
  empty: () => fetch(`${API_BASE}/trash/empty`, {
    method: 'DELETE',
    headers: getHeaders()
  })
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
    return { data: await handleResponse(response) };
  },

  /**
   * List backup schedules
   */
  async listSchedules() {
    const response = await fetch(`${API_BASE}/backup/schedules`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create backup schedule
   */
  async createSchedule(scheduleData) {
    const response = await fetch(`${API_BASE}/backup/schedules`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(scheduleData),
    });
    return handleResponse(response);
  },

  /**
   * Delete backup schedule
   */
  async deleteSchedule(scheduleId) {
    const response = await fetch(`${API_BASE}/backup/schedules/${scheduleId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Enable backup schedule
   */
  async enableSchedule(scheduleId) {
    const response = await fetch(`${API_BASE}/backup/schedules/${scheduleId}/enable`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Disable backup schedule
   */
  async disableSchedule(scheduleId) {
    const response = await fetch(`${API_BASE}/backup/schedules/${scheduleId}/disable`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * List backup jobs
   */
  async listJobs() {
    const response = await fetch(`${API_BASE}/backup/jobs`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Trigger manual backup
   */
  async triggerBackup(options = {}) {
    const response = await fetch(`${API_BASE}/backup/trigger`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(options),
    });
    return handleResponse(response);
  },

  /**
   * Cancel backup job
   */
  async cancelJob(jobId) {
    const response = await fetch(`${API_BASE}/backup/jobs/${jobId}/cancel`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get backup statistics
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/backup/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Restore from backup
   */
  async restore(restoreData) {
    const response = await fetch(`${API_BASE}/backup/restore`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(restoreData),
    });
    return handleResponse(response);
  },

  /**
   * Cleanup old backups
   */
  async cleanup() {
    const response = await fetch(`${API_BASE}/backup/cleanup`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new backup
   * @param {Object} options - Backup options
   */
  async create(options = {}) {
    const response = await fetch(`${API_BASE}/backups/create`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({
        backup_type: options.backup_type || 'full',
        include_versions: options.include_versions !== false,
        include_database: options.include_database !== false,
        description: options.description || null,
        encrypt: options.encrypt || false,
        destination_id: options.destination_id || null,
      }),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Get backup details
   * @param {string} backupId - Backup ID
   */
  async get(backupId) {
    const response = await fetch(`${API_BASE}/backups/${backupId}`, {
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
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
    return { data: await handleResponse(response) };
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
    return { data: await handleResponse(response) };
  },

  /**
   * List verifications for a backup
   * @param {string} backupId - Backup ID
   */
  async listVerifications(backupId) {
    const response = await fetch(`${API_BASE}/backups/${backupId}/verifications`, {
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Clean up old backups based on retention policies
   */
  async cleanup() {
    const response = await fetch(`${API_BASE}/backups/cleanup`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Restore a backup
   * @param {string} backupId - Backup ID
   * @param {Object} options - Restore options
   */
  async restore(backupId, options = {}) {
    const response = await fetch(`${API_BASE}/backups/${backupId}/restore`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({
        restore_type: options.restore_type || 'full',
        restore_path: options.restore_path || null,
      }),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Get backup statistics
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/backups/stats`, {
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * List restore history
   */
  async listRestores() {
    const response = await fetch(`${API_BASE}/backups/restores`, {
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  // === Schedule Methods ===

  /**
   * List all backup schedules
   */
  async listSchedules() {
    const response = await fetch(`${API_BASE}/backups/schedules`, {
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Create a new backup schedule
   * @param {Object} schedule - Schedule configuration
   */
  async createSchedule(schedule) {
    const response = await fetch(`${API_BASE}/backups/schedules`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(schedule),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Get schedule details
   * @param {string} scheduleId - Schedule ID
   */
  async getSchedule(scheduleId) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}`, {
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Update a backup schedule
   * @param {string} scheduleId - Schedule ID
   * @param {Object} updates - Schedule updates
   */
  async updateSchedule(scheduleId, updates) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(updates),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Delete a backup schedule
   * @param {string} scheduleId - Schedule ID
   */
  async deleteSchedule(scheduleId) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Toggle a schedule on/off
   * @param {string} scheduleId - Schedule ID
   */
  async toggleSchedule(scheduleId) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}/toggle`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Trigger a schedule manually
   * @param {string} scheduleId - Schedule ID
   */
  async triggerSchedule(scheduleId) {
    const response = await fetch(`${API_BASE}/backups/schedules/${scheduleId}/trigger`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  // === Remote Destination Methods ===

  /**
   * List all remote destinations
   */
  async listDestinations() {
    const response = await fetch(`${API_BASE}/backups/remote-destinations`, {
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Create a new remote destination
   * @param {Object} destination - Destination configuration
   */
  async createDestination(destination) {
    const response = await fetch(`${API_BASE}/backups/remote-destinations`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(destination),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Delete a remote destination
   * @param {string} destinationId - Destination ID
   */
  async deleteDestination(destinationId) {
    const response = await fetch(`${API_BASE}/backups/remote-destinations/${destinationId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
  },

  /**
   * Test remote destination connection
   * @param {string} destinationId - Destination ID
   */
  async testDestination(destinationId) {
    const response = await fetch(`${API_BASE}/backups/remote-destinations/${destinationId}/test`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return { data: await handleResponse(response) };
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
    const response = await fetch(`${API_HOST}/status/json`, {
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

// ============================================================================
// Jobs API
// ============================================================================

export const jobs = {
  /**
   * Get job statistics
   */
  stats: () => fetch(`${API_BASE}/jobs/stats`, { headers: getHeaders() }),

  /**
   * List jobs with filters
   */
  list: (params = {}) => {
    const query = new URLSearchParams();
    if (params.status) query.set("status", params.status);
    if (params.job_type) query.set("job_type", params.job_type);
    if (params.limit) query.set("limit", params.limit);
    if (params.offset) query.set("offset", params.offset);

    const queryString = query.toString();
    const url = queryString ? `${API_BASE}/jobs?${queryString}` : `${API_BASE}/jobs`;

    return fetch(url, { headers: getHeaders() });
  },

  /**
   * Get job by ID
   */
  get: (jobId) => fetch(`${API_BASE}/jobs/${jobId}`, { headers: getHeaders() }),

  /**
   * Enqueue new job
   */
  enqueue: (jobType, payload, priority = 5, scheduledAt = null) =>
    fetch(`${API_BASE}/jobs`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ job_type: jobType, payload, priority, scheduled_at: scheduledAt }),
    }),

  /**
   * Cancel job
   */
  cancel: (jobId) =>
    fetch(`${API_BASE}/jobs/${jobId}/cancel`, {
      method: "POST",
      headers: getHeaders(),
    }),

  /**
   * Cleanup old jobs
   */
  cleanup: () =>
    fetch(`${API_BASE}/jobs/cleanup`, {
      method: "POST",
      headers: getHeaders(),
    }),
};

// ============================================================================
// Cron API
// ============================================================================

export const cron = {
  /**
   * List all cron jobs
   */
  list: () => fetch(`${API_BASE}/cron`, { headers: getHeaders() }),

  /**
   * Get cron job by ID
   */
  get: (cronId) => fetch(`${API_BASE}/cron/${cronId}`, { headers: getHeaders() }),

  /**
   * Create cron job
   */
  create: (name, jobType, cronExpression, payload) =>
    fetch(`${API_BASE}/cron`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ name, job_type: jobType, cron_expression: cronExpression, payload }),
    }),

  /**
   * Update cron job
   */
  update: (cronId, cronExpression, payload) =>
    fetch(`${API_BASE}/cron/${cronId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify({ cron_expression: cronExpression, payload }),
    }),

  /**
   * Delete cron job
   */
  delete: (cronId) =>
    fetch(`${API_BASE}/cron/${cronId}`, {
      method: "DELETE",
      headers: getHeaders(),
    }),

  /**
   * Enable cron job
   */
  enable: (cronId) =>
    fetch(`${API_BASE}/cron/${cronId}/enable`, {
      method: "POST",
      headers: getHeaders(),
    }),

  /**
   * Disable cron job
   */
  disable: (cronId) =>
    fetch(`${API_BASE}/cron/${cronId}/disable`, {
      method: "POST",
      headers: getHeaders(),
    }),
};

// ============================================================================
// SETUP WIZARD API
// ============================================================================

// ============================================================================
// SETUP WIZARD API
// ============================================================================

/**
 * Setup wizard endpoints (first-time installation)
 */
export const setup = {
  /**
   * Check if setup is required
   */
  isRequired: () =>
    fetch(`${API_BASE}/setup/status`, {
      method: "GET",
      headers: getHeaders(),
    }).then(handleResponse),

  /**
   * Get setup status
   */
  status: () =>
    fetch(`${API_BASE}/setup/status`, {
      method: "GET",
      headers: getHeaders(),
    }).then(handleResponse),

  /**
   * Complete initial setup
   */
  complete: (formData) =>
    fetch(`${API_BASE}/setup/complete`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(formData),
    }).then(handleResponse),
};

// ============================================
// CLOUD STORAGE ENDPOINTS
// ============================================

export const cloudStorage = {
  /**
   * List all storage backends
   */
  async listBackends() {
    const response = await fetch(`${API_BASE}/storage/backends`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get single storage backend
   */
  async getBackend(id) {
    const response = await fetch(`${API_BASE}/storage/backends/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create new storage backend
   */
  async createBackend(data) {
    const response = await fetch(`${API_BASE}/storage/backends`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update storage backend
   */
  async updateBackend(id, data) {
    const response = await fetch(`${API_BASE}/storage/backends/${id}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete storage backend
   */
  async deleteBackend(id) {
    const response = await fetch(`${API_BASE}/storage/backends/${id}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    if (!response.ok) {
      if (response.status === 409) {
        throw new Error("Cannot delete backend with files");
      }
      throw new Error("Failed to delete backend");
    }
    return true;
  },

  /**
   * Check backend health
   */
  async checkHealth(id) {
    const response = await fetch(`${API_BASE}/storage/backends/${id}/health`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Test backend connection
   */
  async testConnection(id) {
    const response = await fetch(`${API_BASE}/storage/backends/${id}/test`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Set default backend
   */
  async setDefault(id) {
    const response = await fetch(`${API_BASE}/storage/backends/${id}/set-default`, {
      method: "POST",
      headers: getHeaders(),
    });
    if (!response.ok) throw new Error("Failed to set default backend");
    return true;
  },

  /**
   * Get storage statistics
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/storage/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // Migration endpoints
  /**
   * List migration jobs
   */
  async listMigrations() {
    const response = await fetch(`${API_BASE}/storage/migration/jobs`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get migration job details
   */
  async getMigration(id) {
    const response = await fetch(`${API_BASE}/storage/migration/jobs/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create migration job
   */
  async createMigration(data) {
    const response = await fetch(`${API_BASE}/storage/migration`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Cancel migration job
   */
  async cancelMigration(id) {
    const response = await fetch(`${API_BASE}/storage/migration/jobs/${id}/cancel`, {
      method: "POST",
      headers: getHeaders(),
    });
    if (!response.ok) throw new Error("Failed to cancel migration");
    return true;
  },
};

// ============================================================================
// FILE METADATA EXTRACTION
// ============================================================================

export const metadata = {
  /**
   * Get metadata for a file (EXIF, ID3, PDF info, etc.)
   */
  async get(path) {
    const response = await fetch(`${API_BASE}/metadata/${encodeURIComponent(path)}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get supported metadata file types
   */
  async getSupportedTypes() {
    const response = await fetch(`${API_BASE}/metadata-types`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================================================
// AUDIT & COMPLIANCE
// ============================================================================

export const audit = {
  // --- Audit Logs ---
  /**
   * List audit logs with filtering and pagination
   */
  async listLogs(params = {}) {
    const queryParams = new URLSearchParams();
    if (params.page) queryParams.append('page', params.page);
    if (params.limit) queryParams.append('limit', params.limit);
    if (params.user_id) queryParams.append('user_id', params.user_id);
    if (params.category) queryParams.append('category', params.category);
    if (params.severity) queryParams.append('severity', params.severity);
    if (params.action) queryParams.append('action', params.action);
    if (params.from_date) queryParams.append('from_date', params.from_date);
    if (params.to_date) queryParams.append('to_date', params.to_date);
    if (params.search) queryParams.append('search', params.search);

    const response = await fetch(`${API_BASE}/audit/logs?${queryParams}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get audit log statistics
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/audit/logs/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Export audit logs
   */
  async exportLogs(params = {}) {
    const queryParams = new URLSearchParams();
    if (params.format) queryParams.append('format', params.format);
    if (params.from_date) queryParams.append('from_date', params.from_date);
    if (params.to_date) queryParams.append('to_date', params.to_date);
    if (params.user_id) queryParams.append('user_id', params.user_id);
    if (params.category) queryParams.append('category', params.category);
    if (params.severity) queryParams.append('severity', params.severity);

    const response = await fetch(`${API_BASE}/audit/logs/export?${queryParams}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Compliance Reports ---
  /**
   * List compliance reports
   */
  async listReports() {
    const response = await fetch(`${API_BASE}/audit/compliance/reports`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Generate a new compliance report
   */
  async generateReport(data) {
    const response = await fetch(`${API_BASE}/audit/compliance/reports`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get a specific compliance report
   */
  async getReport(id) {
    const response = await fetch(`${API_BASE}/audit/compliance/reports/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Retention Policies ---
  /**
   * List retention policies
   */
  async listPolicies() {
    const response = await fetch(`${API_BASE}/audit/retention-policies`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a retention policy
   */
  async createPolicy(data) {
    const response = await fetch(`${API_BASE}/audit/retention-policies`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update a retention policy
   */
  async updatePolicy(id, data) {
    const response = await fetch(`${API_BASE}/audit/retention-policies/${id}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a retention policy
   */
  async deletePolicy(id) {
    const response = await fetch(`${API_BASE}/audit/retention-policies/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Apply retention policies now
   */
  async applyPolicies() {
    const response = await fetch(`${API_BASE}/audit/retention-policies/apply`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Alert Rules ---
  /**
   * List alert rules
   */
  async listAlertRules() {
    const response = await fetch(`${API_BASE}/audit/alert-rules`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create an alert rule
   */
  async createAlertRule(data) {
    const response = await fetch(`${API_BASE}/audit/alert-rules`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update an alert rule
   */
  async updateAlertRule(id, data) {
    const response = await fetch(`${API_BASE}/audit/alert-rules/${id}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete an alert rule
   */
  async deleteAlertRule(id) {
    const response = await fetch(`${API_BASE}/audit/alert-rules/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Alerts ---
  /**
   * List triggered alerts
   */
  async listAlerts(params = {}) {
    const queryParams = new URLSearchParams();
    if (params.acknowledged !== undefined) queryParams.append('acknowledged', params.acknowledged);
    if (params.severity) queryParams.append('severity', params.severity);
    if (params.limit) queryParams.append('limit', params.limit);

    const response = await fetch(`${API_BASE}/audit/alerts?${queryParams}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Acknowledge an alert
   */
  async acknowledgeAlert(id) {
    const response = await fetch(`${API_BASE}/audit/alerts/${id}/acknowledge`, {
      method: 'PUT',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Sessions ---
  /**
   * List active sessions
   */
  async listSessions() {
    const response = await fetch(`${API_BASE}/audit/sessions`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Terminate a session
   */
  async terminateSession(id) {
    const response = await fetch(`${API_BASE}/audit/sessions/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Templates ---
  /**
   * List report templates
   */
  async listTemplates() {
    const response = await fetch(`${API_BASE}/audit/templates`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Download a compliance report
   */
  async downloadReport(id) {
    const response = await fetch(`${API_BASE}/audit/compliance/reports/${id}/download`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Apply a specific retention policy
   */
  async applyPolicy(id) {
    const response = await fetch(`${API_BASE}/audit/retention-policies/${id}/apply`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Apply all active retention policies
   */
  async applyAllPolicies() {
    const response = await fetch(`${API_BASE}/audit/retention-policies/apply-all`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Archives ---
  /**
   * List audit archives
   */
  async listArchives() {
    const response = await fetch(`${API_BASE}/audit/archives`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create an audit archive
   */
  async createArchive() {
    const response = await fetch(`${API_BASE}/audit/archives`, {
      method: 'POST',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Delete an archive
   */
  async deleteArchive(id) {
    const response = await fetch(`${API_BASE}/audit/archives/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================================================
// ADMIN DASHBOARD
// ============================================================================

export const dashboard = {
  /**
   * Get comprehensive dashboard statistics
   */
  async getStats(range = '7d') {
    const response = await fetch(`${API_BASE}/dashboard/stats?range=${range}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get storage overview
   */
  async getStorage() {
    const response = await fetch(`${API_BASE}/dashboard/storage`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get activity overview
   */
  async getActivity() {
    const response = await fetch(`${API_BASE}/dashboard/activity`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get users overview
   */
  async getUsers() {
    const response = await fetch(`${API_BASE}/dashboard/users`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get jobs overview
   */
  async getJobs() {
    const response = await fetch(`${API_BASE}/dashboard/jobs`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get system health status
   */
  async getHealth() {
    const response = await fetch(`${API_BASE}/dashboard/health`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get real-time statistics
   */
  async getRealtime() {
    const response = await fetch(`${API_BASE}/dashboard/realtime`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================================================
// WEBHOOKS
// ============================================================================

export const webhooks = {
  /**
   * List all webhooks
   */
  async list() {
    const response = await fetch(`${API_BASE}/webhooks`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get available webhook events
   */
  async getEvents() {
    const response = await fetch(`${API_BASE}/webhooks/events`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get a specific webhook
   */
  async get(id) {
    const response = await fetch(`${API_BASE}/webhooks/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new webhook
   */
  async create(data) {
    const response = await fetch(`${API_BASE}/webhooks`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update a webhook
   */
  async update(id, data) {
    const response = await fetch(`${API_BASE}/webhooks/${id}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a webhook
   */
  async delete(id) {
    const response = await fetch(`${API_BASE}/webhooks/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    return true;
  },

  /**
   * Test a webhook
   */
  async test(id, eventType = 'test.ping') {
    const response = await fetch(`${API_BASE}/webhooks/${id}/test`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({ event_type: eventType }),
    });
    return handleResponse(response);
  },

  /**
   * Get webhook delivery history
   */
  async getDeliveries(id) {
    const response = await fetch(`${API_BASE}/webhooks/${id}/deliveries`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Reset failure count
   */
  async resetFailures(id) {
    const response = await fetch(`${API_BASE}/webhooks/${id}/reset-failures`, {
      method: 'POST',
      headers: getHeaders(),
    });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    return true;
  },
};

// ============================================================================
// SYSTEM HEALTH - System Health Monitoring
// ============================================================================

export const systemHealth = {
  /**
   * Get overall system health status
   */
  async getHealth() {
    const response = await fetch(`${API_BASE}/system/health`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get CPU metrics
   */
  async getCpu() {
    const response = await fetch(`${API_BASE}/system/health/cpu`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get memory metrics
   */
  async getMemory() {
    const response = await fetch(`${API_BASE}/system/health/memory`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get disk metrics
   */
  async getDisk() {
    const response = await fetch(`${API_BASE}/system/health/disk`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get database health
   */
  async getDatabase() {
    const response = await fetch(`${API_BASE}/system/health/database`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get connection pool statistics
   */
  async getConnections() {
    const response = await fetch(`${API_BASE}/system/health/connections`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get search index health
   */
  async getSearch() {
    const response = await fetch(`${API_BASE}/system/health/search`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get WebSocket health
   */
  async getWebsocket() {
    const response = await fetch(`${API_BASE}/system/health/websocket`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get application metrics
   */
  async getMetrics() {
    const response = await fetch(`${API_BASE}/system/health/metrics`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get historical metrics
   * @param {string} range - 1h, 6h, 24h, 7d
   */
  async getHistory(range = '1h') {
    const response = await fetch(`${API_BASE}/system/health/history?range=${range}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * List health alerts
   */
  async listAlerts() {
    const response = await fetch(`${API_BASE}/system/health/alerts`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a health alert
   */
  async createAlert(alert) {
    const response = await fetch(`${API_BASE}/system/health/alerts`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(alert),
    });
    return handleResponse(response);
  },

  /**
   * Delete a health alert
   */
  async deleteAlert(id) {
    const response = await fetch(`${API_BASE}/system/health/alerts/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    return true;
  },
};

// ============================================================================
// API TOKENS (Personal Access Tokens)
// ============================================================================

export const apiTokens = {
  /**
   * List all API tokens for the current user
   */
  async list() {
    const response = await fetch(`${API_BASE}/tokens`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new API token
   * @param {Object} data - { name, scopes, expires_in_days }
   * @returns {Object} - Token details including plain text token (only shown once)
   */
  async create(data) {
    const response = await fetch(`${API_BASE}/tokens`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get details of a specific token
   * @param {string} id - Token ID
   */
  async get(id) {
    const response = await fetch(`${API_BASE}/tokens/${encodeURIComponent(id)}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update a token's name or scopes
   * @param {string} id - Token ID
   * @param {Object} data - { name, scopes }
   */
  async update(id, data) {
    const response = await fetch(`${API_BASE}/tokens/${encodeURIComponent(id)}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a token permanently
   * @param {string} id - Token ID
   */
  async delete(id) {
    const response = await fetch(`${API_BASE}/tokens/${encodeURIComponent(id)}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Revoke a token (soft delete - keeps record but marks as revoked)
   * @param {string} id - Token ID
   */
  async revoke(id) {
    const response = await fetch(`${API_BASE}/tokens/${encodeURIComponent(id)}/revoke`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================================================
// DEFAULT EXPORT - All API modules
// ============================================================================

export default {
  auth,
  users,
  files,
  directories,
  search,
  activity,
  comments,
  tags,
  folderColors,
  config,
  peers,
  batch,
  favorites,
  versions,
  backup,
  backupSchedules,
  system,
  sharing,
  shares,
  jobs,
  cron,
  recent,
  duplicates,
  collaboration,
  trash,
  setup,
  performance,
  cloudStorage,
  metadata,
  audit,
  dashboard,
  webhooks,
  systemHealth,
  apiTokens,
  storageAnalytics: {
    /**
     * Get storage overview statistics
     */
    async getOverview() {
      const response = await fetch(`${API_BASE}/storage/analytics/overview`, {
        method: "GET",
        headers: getHeaders(),
      });
      return handleResponse(response);
    },

    /**
     * Get storage statistics by user
     */
    async getByUser() {
      const response = await fetch(`${API_BASE}/storage/analytics/by-user`, {
        method: "GET",
        headers: getHeaders(),
      });
      return handleResponse(response);
    },

    /**
     * Get storage statistics by folder
     */
    async getByFolder() {
      const response = await fetch(`${API_BASE}/storage/analytics/by-folder`, {
        method: "GET",
        headers: getHeaders(),
      });
      return handleResponse(response);
    },

    /**
     * Get top largest files
     */
    async getTopFiles(limit = 100) {
      const response = await fetch(
        `${API_BASE}/storage/analytics/top-files?limit=${limit}`,
        {
          method: "GET",
          headers: getHeaders(),
        }
      );
      return handleResponse(response);
    },

    /**
     * Get storage growth over time
     */
    async getGrowth(days = 30) {
      const response = await fetch(
        `${API_BASE}/storage/analytics/growth?days=${days}`,
        {
          method: "GET",
          headers: getHeaders(),
        }
      );
      return handleResponse(response);
    },

    /**
     * Get duplicate file statistics
     */
    async getDuplicateWaste() {
      const response = await fetch(
        `${API_BASE}/storage/analytics/duplicates`,
        {
          method: "GET",
          headers: getHeaders(),
        }
      );
      return handleResponse(response);
    },
  },
  smartFolders: {
    /**
     * List all smart folders for current user
     */
    async list() {
      const response = await fetch(`${API_BASE}/smart-folders`, {
        method: "GET",
        headers: getHeaders(),
      });
      return handleResponse(response);
    },

    /**
     * Create a new smart folder
     */
    async create(payload) {
      const response = await fetch(`${API_BASE}/smart-folders`, {
        method: "POST",
        headers: getHeaders(),
        body: JSON.stringify(payload),
      });
      return handleResponse(response);
    },

    /**
     * Update an existing smart folder
     */
    async update(id, payload) {
      const response = await fetch(`${API_BASE}/smart-folders/${id}`, {
        method: "PUT",
        headers: getHeaders(),
        body: JSON.stringify(payload),
      });
      return handleResponse(response);
    },

    /**
     * Delete a smart folder
     */
    async delete(id) {
      const response = await fetch(`${API_BASE}/smart-folders/${id}`, {
        method: "DELETE",
        headers: getHeaders(),
      });
      return handleResponse(response);
    },

    /**
     * Preview files matching a smart folder's rules
     */
    async preview(id) {
      const response = await fetch(`${API_BASE}/smart-folders/${id}/preview`, {
        method: "POST",
        headers: getHeaders(),
      });
      return handleResponse(response);
    },
  },
};

// Aggregated API export for convenience
export const api = {
  auth,
  users,
  files,
  directories,
  search,
  activity,
  comments,
  tags,
  folderColors,
  config,
  peers,
  batch,
  bulk,
  templates,
  workflow,
  rbac,
  favorites,
  recent,
  sharing,
  versions,
  duplicates,
  collaboration,
  trash,
  backup,
  backupSchedules,
  cron,
  cloudStorage,
  metadata,
  audit,
  webhooks,
  systemHealth,
  apiTokens,
};

// ============================================================================
// FILE ENCRYPTION AT REST
// ============================================================================

export const encryption = {
  // --- Key Management ---

  /**
   * List all encryption keys for the current user
   */
  async listKeys() {
    const response = await fetch(`${API_BASE}/encryption/keys`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new encryption key
   * @param {string} name - Key name/description
   * @param {string} password - Password to protect the key
   */
  async createKey(name, password) {
    const response = await fetch(`${API_BASE}/encryption/keys`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ name, password }),
    });
    return handleResponse(response);
  },

  /**
   * Delete (deactivate) an encryption key
   * @param {string} keyId - Key ID to delete
   */
  async deleteKey(keyId) {
    const response = await fetch(`${API_BASE}/encryption/keys/${keyId}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Rotate an encryption key's password
   * @param {string} keyId - Key ID to rotate
   * @param {string} oldPassword - Current password
   * @param {string} newPassword - New password
   */
  async rotateKey(keyId, oldPassword, newPassword) {
    const response = await fetch(`${API_BASE}/encryption/keys/${keyId}/rotate`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ old_password: oldPassword, new_password: newPassword }),
    });
    return handleResponse(response);
  },

  // --- File Operations ---

  /**
   * Get encryption status of a file
   * @param {string} fileId - File ID to check
   */
  async getFileStatus(fileId) {
    const response = await fetch(`${API_BASE}/encryption/files/${fileId}/status`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Encrypt a file
   * @param {string} fileId - File ID to encrypt
   * @param {string} keyId - Encryption key to use
   * @param {string} password - Password to decrypt the key
   */
  async encryptFile(fileId, keyId, password) {
    const response = await fetch(`${API_BASE}/encryption/files/${fileId}/encrypt`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ key_id: keyId, password }),
    });
    return handleResponse(response);
  },

  /**
   * Decrypt a file
   * @param {string} fileId - File ID to decrypt
   * @param {string} password - Password to decrypt the key
   */
  async decryptFile(fileId, password) {
    const response = await fetch(`${API_BASE}/encryption/files/${fileId}/decrypt`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ password }),
    });
    return handleResponse(response);
  },

  /**
   * Encrypt all files in a folder
   * @param {string} folderPath - Folder path to encrypt
   * @param {string} keyId - Encryption key to use
   * @param {string} password - Password to decrypt the key
   * @param {boolean} includeSubfolders - Whether to include subfolders
   */
  async encryptFolder(folderPath, keyId, password, includeSubfolders = true) {
    const response = await fetch(`${API_BASE}/encryption/folders/encrypt`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({
        folder_path: folderPath,
        key_id: keyId,
        password,
        include_subfolders: includeSubfolders
      }),
    });
    return handleResponse(response);
  },

  // --- Settings ---

  /**
   * Get encryption settings
   */
  async getSettings() {
    const response = await fetch(`${API_BASE}/encryption/settings`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update encryption settings
   * @param {Object} settings - Encryption settings
   */
  async updateSettings(settings) {
    const response = await fetch(`${API_BASE}/encryption/settings`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(settings),
    });
    return handleResponse(response);
  },
};

// ============================================================================
// STORAGE QUOTA MANAGEMENT
// ============================================================================

export const quota = {
  /**
   * Get current user's quota info
   */
  async getMyQuota() {
    const response = await fetch(`${API_BASE}/quota`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get all users' quota info (admin only)
   */
  async listAll() {
    const response = await fetch(`${API_BASE}/quota/all`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get quota info for a specific user (admin only)
   * @param {string} userId - User ID
   */
  async getUserQuota(userId) {
    const response = await fetch(`${API_BASE}/quota/${userId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Set quota for a user (admin only)
   * @param {string} userId - User ID
   * @param {number} quotaBytes - Quota in bytes
   */
  async setUserQuota(userId, quotaBytes) {
    const response = await fetch(`${API_BASE}/quota/${userId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify({ quota_bytes: quotaBytes }),
    });
    return handleResponse(response);
  },
};

// USER GROUPS API
export const groups = {
  /**
   * List all user groups
   */
  async list() {
    const response = await fetch(`${API_BASE}/groups`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new group
   * @param {string} name - Group name
   * @param {string} description - Group description (optional)
   */
  async create(name, description = null) {
    const response = await fetch(`${API_BASE}/groups`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ name, description }),
    });
    return handleResponse(response);
  },

  /**
   * Get group details with members
   * @param {string} groupId - Group ID
   */
  async get(groupId) {
    const response = await fetch(`${API_BASE}/groups/${groupId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Delete a group
   * @param {string} groupId - Group ID
   */
  async delete(groupId) {
    const response = await fetch(`${API_BASE}/groups/${groupId}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Add a member to a group
   * @param {string} groupId - Group ID
   * @param {string} userId - User ID to add
   */
  async addMember(groupId, userId) {
    const response = await fetch(`${API_BASE}/groups/${groupId}/members`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ user_id: userId }),
    });
    return handleResponse(response);
  },

  /**
   * Remove a member from a group
   * @param {string} groupId - Group ID
   * @param {string} userId - User ID to remove
   */
  async removeMember(groupId, userId) {
    const response = await fetch(`${API_BASE}/groups/${groupId}/members/${userId}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get user roles
   * @param {string} userId - User ID
   */
  async getUserRoles(userId) {
    const response = await fetch(`${API_BASE}/users/${userId}/roles`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Assign a role to a user
   * @param {string} userId - User ID
   * @param {string} role - Role to assign (admin, moderator, user, guest)
   */
  async assignRole(userId, role) {
    const response = await fetch(`${API_BASE}/users/${userId}/roles`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ role }),
    });
    return handleResponse(response);
  },

  /**
   * Suspend a user
   * @param {string} userId - User ID
   * @param {string} reason - Suspension reason
   * @param {number} durationDays - Suspension duration in days (optional)
   */
  async suspendUser(userId, reason, durationDays = null) {
    const response = await fetch(`${API_BASE}/users/${userId}/suspend`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ reason, duration_days: durationDays }),
    });
    return handleResponse(response);
  },

  /**
   * Unsuspend a user
   * @param {string} userId - User ID
   */
  async unsuspendUser(userId) {
    const response = await fetch(`${API_BASE}/users/${userId}/unsuspend`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================================================
// Guest/External User API
// ============================================================================

export const guests = {
  // --- Guest Users ---

  /**
   * List all guest users
   * @param {Object} options - Query options
   * @param {boolean} options.includeExpired - Include expired guests
   * @param {boolean} options.includeInactive - Include inactive guests
   */
  async list(options = {}) {
    const params = new URLSearchParams();
    if (options.includeExpired) params.append("include_expired", "true");
    if (options.includeInactive) params.append("include_inactive", "true");
    const response = await fetch(`${API_BASE}/guests?${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new guest user
   * @param {Object} data - Guest data
   * @param {string} data.display_name - Display name
   * @param {string} data.email - Email address
   * @param {number} data.expires_in_days - Days until expiration
   * @param {number} data.max_accesses - Maximum access count
   * @param {string} data.notes - Notes
   * @param {boolean} data.can_view - Can view files
   * @param {boolean} data.can_download - Can download files
   * @param {boolean} data.can_upload - Can upload files
   * @param {boolean} data.can_comment - Can add comments
   */
  async create(data) {
    const response = await fetch(`${API_BASE}/guests`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get a single guest user
   * @param {string} guestId - Guest ID
   */
  async get(guestId) {
    const response = await fetch(`${API_BASE}/guests/${guestId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update a guest user
   * @param {string} guestId - Guest ID
   * @param {Object} data - Updated data
   */
  async update(guestId, data) {
    const response = await fetch(`${API_BASE}/guests/${guestId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a guest user
   * @param {string} guestId - Guest ID
   */
  async delete(guestId) {
    const response = await fetch(`${API_BASE}/guests/${guestId}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get guest statistics
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/guests/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get guest activity log
   * @param {string} guestId - Guest ID
   * @param {Object} options - Query options
   * @param {number} options.limit - Limit results
   * @param {number} options.offset - Offset for pagination
   */
  async getActivity(guestId, options = {}) {
    const params = new URLSearchParams();
    if (options.limit) params.append("limit", options.limit);
    if (options.offset) params.append("offset", options.offset);
    const response = await fetch(`${API_BASE}/guests/${guestId}/activity?${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Convert guest to full user
   * @param {string} guestId - Guest ID
   * @param {Object} data - User data
   * @param {string} data.username - New username
   * @param {string} data.password - New password
   * @param {string} data.email - Email address
   */
  async convertToUser(guestId, data) {
    const response = await fetch(`${API_BASE}/guests/${guestId}/convert`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  // --- Guest Access Links ---

  /**
   * List all guest access links
   * @param {Object} options - Query options
   * @param {boolean} options.includeExpired - Include expired links
   * @param {boolean} options.includeInactive - Include inactive links
   */
  async listLinks(options = {}) {
    const params = new URLSearchParams();
    if (options.includeExpired) params.append("include_expired", "true");
    if (options.includeInactive) params.append("include_inactive", "true");
    const response = await fetch(`${API_BASE}/guest-links?${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new guest access link
   * @param {Object} data - Link data
   * @param {string} data.file_path - Path to file/folder
   * @param {string} data.guest_id - Optional guest to associate
   * @param {string} data.access_type - "file" or "folder"
   * @param {number} data.expires_in_days - Days until expiration
   * @param {string} data.password - Optional password protection
   * @param {number} data.max_accesses - Maximum access count
   * @param {boolean} data.can_view - Can view
   * @param {boolean} data.can_download - Can download
   * @param {boolean} data.can_upload - Can upload
   */
  async createLink(data) {
    const response = await fetch(`${API_BASE}/guest-links`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get a single link
   * @param {string} linkId - Link ID
   */
  async getLink(linkId) {
    const response = await fetch(`${API_BASE}/guest-links/${linkId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update a link
   * @param {string} linkId - Link ID
   * @param {Object} data - Updated data
   */
  async updateLink(linkId, data) {
    const response = await fetch(`${API_BASE}/guest-links/${linkId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a link
   * @param {string} linkId - Link ID
   */
  async deleteLink(linkId) {
    const response = await fetch(`${API_BASE}/guest-links/${linkId}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Toggle link active status
   * @param {string} linkId - Link ID
   */
  async toggleLink(linkId) {
    const response = await fetch(`${API_BASE}/guest-links/${linkId}/toggle`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Guest Invitations ---

  /**
   * List all invitations
   * @param {Object} options - Query options
   * @param {boolean} options.includeExpired - Include expired
   * @param {boolean} options.includeAccepted - Include accepted
   */
  async listInvitations(options = {}) {
    const params = new URLSearchParams();
    if (options.includeExpired) params.append("include_expired", "true");
    if (options.includeAccepted) params.append("include_accepted", "true");
    const response = await fetch(`${API_BASE}/guest-invitations?${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Send a new invitation
   * @param {Object} data - Invitation data
   * @param {string} data.email - Email to invite
   * @param {number} data.expires_in_days - Days until expiration
   * @param {string} data.message - Personal message
   * @param {boolean} data.can_view - Can view files
   * @param {boolean} data.can_download - Can download files
   * @param {boolean} data.can_upload - Can upload files
   * @param {boolean} data.can_comment - Can add comments
   */
  async createInvitation(data) {
    const response = await fetch(`${API_BASE}/guest-invitations`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete/revoke an invitation
   * @param {string} invitationId - Invitation ID
   */
  async deleteInvitation(invitationId) {
    const response = await fetch(`${API_BASE}/guest-invitations/${invitationId}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Resend an invitation email
   * @param {string} invitationId - Invitation ID
   */
  async resendInvitation(invitationId) {
    const response = await fetch(`${API_BASE}/guest-invitations/${invitationId}/resend`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Public Access (no auth) ---

  /**
   * Access a guest link (public, no auth required)
   * @param {string} token - Access token
   */
  async accessLink(token) {
    const response = await fetch(`${API_BASE}/guest-access/${token}`);
    return handleResponse(response);
  },

  /**
   * Access a password-protected link (public, no auth required)
   * @param {string} token - Access token
   * @param {string} password - Password
   */
  async accessLinkWithPassword(token, password) {
    const response = await fetch(`${API_BASE}/guest-access/${token}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ password }),
    });
    return handleResponse(response);
  },
};

// ============================================
// RATE LIMITING & QUOTAS
// ============================================

export const rateLimiting = {
  // --- Storage Quotas ---

  /**
   * List all storage quotas
   * @param {Object} params - { page, limit }
   */
  async listStorageQuotas(params = {}) {
    const searchParams = new URLSearchParams();
    if (params.page) searchParams.set("page", params.page);
    if (params.limit) searchParams.set("limit", params.limit);
    const queryString = searchParams.toString();
    const url = queryString ? `${API_BASE}/quotas/storage?${queryString}` : `${API_BASE}/quotas/storage`;
    const response = await fetch(url, { headers: getHeaders() });
    return handleResponse(response);
  },

  /**
   * Get storage quota for a specific user
   * @param {string} userId - User ID
   */
  async getStorageQuota(userId) {
    const response = await fetch(`${API_BASE}/quotas/storage/${userId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update storage quota for a user
   * @param {string} userId - User ID
   * @param {Object} data - { storage_limit_bytes, warning_threshold_percent, is_unlimited }
   */
  async updateStorageQuota(userId, data) {
    const response = await fetch(`${API_BASE}/quotas/storage/${userId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get storage usage for a user
   * @param {string} userId - User ID
   */
  async getStorageUsage(userId) {
    const response = await fetch(`${API_BASE}/quotas/storage/${userId}/usage`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Bandwidth Quotas ---

  /**
   * List all bandwidth quotas
   * @param {Object} params - { page, limit }
   */
  async listBandwidthQuotas(params = {}) {
    const searchParams = new URLSearchParams();
    if (params.page) searchParams.set("page", params.page);
    if (params.limit) searchParams.set("limit", params.limit);
    const queryString = searchParams.toString();
    const url = queryString ? `${API_BASE}/quotas/bandwidth?${queryString}` : `${API_BASE}/quotas/bandwidth`;
    const response = await fetch(url, { headers: getHeaders() });
    return handleResponse(response);
  },

  /**
   * Get bandwidth quota for a specific user
   * @param {string} userId - User ID
   */
  async getBandwidthQuota(userId) {
    const response = await fetch(`${API_BASE}/quotas/bandwidth/${userId}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update bandwidth quota for a user
   * @param {string} userId - User ID
   * @param {Object} data - { daily_upload_limit_bytes, daily_download_limit_bytes, monthly_upload_limit_bytes, monthly_download_limit_bytes, is_unlimited }
   */
  async updateBandwidthQuota(userId, data) {
    const response = await fetch(`${API_BASE}/quotas/bandwidth/${userId}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get bandwidth usage for a user
   * @param {string} userId - User ID
   * @param {Object} params - { start_date, end_date }
   */
  async getBandwidthUsage(userId, params = {}) {
    const searchParams = new URLSearchParams();
    if (params.start_date) searchParams.set("start_date", params.start_date);
    if (params.end_date) searchParams.set("end_date", params.end_date);
    const queryString = searchParams.toString();
    const url = queryString ? `${API_BASE}/quotas/bandwidth/${userId}/usage?${queryString}` : `${API_BASE}/quotas/bandwidth/${userId}/usage`;
    const response = await fetch(url, { headers: getHeaders() });
    return handleResponse(response);
  },

  // --- Rate Limits ---

  /**
   * List all rate limits
   * @param {Object} params - { page, limit }
   */
  async listRateLimits(params = {}) {
    const searchParams = new URLSearchParams();
    if (params.page) searchParams.set("page", params.page);
    if (params.limit) searchParams.set("limit", params.limit);
    const queryString = searchParams.toString();
    const url = queryString ? `${API_BASE}/rate-limits?${queryString}` : `${API_BASE}/rate-limits`;
    const response = await fetch(url, { headers: getHeaders() });
    return handleResponse(response);
  },

  /**
   * Create a new rate limit rule
   * @param {Object} data - { user_id, role_name, endpoint_pattern, requests_per_minute, requests_per_hour, requests_per_day, burst_limit }
   */
  async createRateLimit(data) {
    const response = await fetch(`${API_BASE}/rate-limits`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get a specific rate limit rule
   * @param {string} id - Rate limit ID
   */
  async getRateLimit(id) {
    const response = await fetch(`${API_BASE}/rate-limits/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update a rate limit rule
   * @param {string} id - Rate limit ID
   * @param {Object} data - { endpoint_pattern, requests_per_minute, requests_per_hour, requests_per_day, burst_limit, is_enabled }
   */
  async updateRateLimit(id, data) {
    const response = await fetch(`${API_BASE}/rate-limits/${id}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete a rate limit rule
   * @param {string} id - Rate limit ID
   */
  async deleteRateLimit(id) {
    const response = await fetch(`${API_BASE}/rate-limits/${id}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Alerts ---

  /**
   * List quota alerts for current user
   */
  async listAlerts() {
    const response = await fetch(`${API_BASE}/quotas/alerts`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Acknowledge a quota alert
   * @param {string} alertId - Alert ID
   */
  async acknowledgeAlert(alertId) {
    const response = await fetch(`${API_BASE}/quotas/alerts/${alertId}/acknowledge`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  // --- Statistics ---

  /**
   * Get quota statistics overview
   */
  async getQuotaStats() {
    const response = await fetch(`${API_BASE}/quotas/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get rate limit statistics
   */
  async getRateLimitStats() {
    const response = await fetch(`${API_BASE}/rate-limits/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ============================================================================
// ADMIN API ENDPOINTS
// ============================================================================

/**
 * Admin API for security policies, notifications, etc.
 */
export const admin = {
  /**
   * Get security policy settings
   */
  async getSecurityPolicy() {
    const response = await fetch(`${API_BASE}/admin/security-policy`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update security policy settings
   */
  async updateSecurityPolicy(data) {
    const response = await fetch(`${API_BASE}/admin/security-policy`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get notification settings
   */
  async getNotificationSettings() {
    const response = await fetch(`${API_BASE}/admin/notification-settings`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Update notification settings
   */
  async updateNotificationSettings(data) {
    const response = await fetch(`${API_BASE}/admin/notification-settings`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Test notification service
   */
  async testNotificationService(service, data = {}) {
    const response = await fetch(`${API_BASE}/admin/notification-settings/test/${service}`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Get storage stats
   */
  async getStorageStats() {
    const response = await fetch(`${API_BASE}/storage/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ==================== FTP SYNC ====================
const ftp = {
  /**
   * List all FTP connections
   */
  async list() {
    const response = await fetch(`${API_BASE}/ftp/connections`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get single FTP connection
   */
  async get(id) {
    const response = await fetch(`${API_BASE}/ftp/connections/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create FTP connection
   */
  async create(data) {
    const response = await fetch(`${API_BASE}/ftp/connections`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update FTP connection
   */
  async update(id, data) {
    const response = await fetch(`${API_BASE}/ftp/connections/${id}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete FTP connection
   */
  async delete(id) {
    const response = await fetch(`${API_BASE}/ftp/connections/${id}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Test FTP connection
   */
  async test(id) {
    const response = await fetch(`${API_BASE}/ftp/connections/${id}/test`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * List remote files
   */
  async listRemoteFiles(id) {
    const response = await fetch(`${API_BASE}/ftp/connections/${id}/files`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Trigger sync
   */
  async sync(id) {
    const response = await fetch(`${API_BASE}/ftp/connections/${id}/sync`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ==================== EMAIL INTEGRATION ====================
const email = {
  /**
   * List all email accounts
   */
  async list() {
    const response = await fetch(`${API_BASE}/email/accounts`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get single email account
   */
  async get(id) {
    const response = await fetch(`${API_BASE}/email/accounts/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create email account
   */
  async create(data) {
    const response = await fetch(`${API_BASE}/email/accounts`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update email account
   */
  async update(id, data) {
    const response = await fetch(`${API_BASE}/email/accounts/${id}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete email account
   */
  async delete(id) {
    const response = await fetch(`${API_BASE}/email/accounts/${id}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Test email connection
   */
  async test(id) {
    const response = await fetch(`${API_BASE}/email/accounts/${id}/test`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Fetch emails
   */
  async fetch(id) {
    const response = await fetch(`${API_BASE}/email/accounts/${id}/fetch`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get messages for account
   */
  async getMessages(id, limit = 50, offset = 0) {
    const response = await fetch(`${API_BASE}/email/accounts/${id}/messages?limit=${limit}&offset=${offset}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ==================== OAUTH ====================
const oauth = {
  /**
   * List available OAuth providers
   */
  async listProviders() {
    const response = await fetch(`${API_BASE}/oauth/providers`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get linked OAuth accounts
   */
  async getLinkedAccounts() {
    const response = await fetch(`${API_BASE}/oauth/linked`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Unlink OAuth account
   */
  async unlink(provider) {
    const response = await fetch(`${API_BASE}/oauth/${provider}/unlink`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Configure OAuth provider (admin)
   */
  async configureProvider(data) {
    const response = await fetch(`${API_BASE}/oauth/providers/config`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete OAuth provider (admin)
   */
  async deleteProvider(provider) {
    const response = await fetch(`${API_BASE}/oauth/providers/config/${provider}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get authorization URL
   */
  getAuthUrl(provider, redirectUrl = null) {
    let url = `${API_BASE}/oauth/${provider}/authorize`;
    if (redirectUrl) {
      url += `?redirect_url=${encodeURIComponent(redirectUrl)}`;
    }
    return url;
  },
};

// ==================== LDAP ====================
const ldap = {
  /**
   * List LDAP configurations
   */
  async listConfigs() {
    const response = await fetch(`${API_BASE}/ldap/configs`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get single LDAP config
   */
  async getConfig(id) {
    const response = await fetch(`${API_BASE}/ldap/configs/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create LDAP config
   */
  async createConfig(data) {
    const response = await fetch(`${API_BASE}/ldap/configs`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Update LDAP config
   */
  async updateConfig(id, data) {
    const response = await fetch(`${API_BASE}/ldap/configs/${id}`, {
      method: "PUT",
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse(response);
  },

  /**
   * Delete LDAP config
   */
  async deleteConfig(id) {
    const response = await fetch(`${API_BASE}/ldap/configs/${id}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Test LDAP connection
   */
  async testConnection(id) {
    const response = await fetch(`${API_BASE}/ldap/configs/${id}/test`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Sync all LDAP users
   */
  async syncAll() {
    const response = await fetch(`${API_BASE}/ldap/sync`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Sync single LDAP config
   */
  async sync(id) {
    const response = await fetch(`${API_BASE}/ldap/sync/${id}`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ==================== ARCHIVES ====================
const archives = {
  /**
   * List archives in a directory
   */
  async list(path = "") {
    const params = path ? `?path=${encodeURIComponent(path)}` : "";
    const response = await fetch(`${API_BASE}/archives${params}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Create a new archive
   */
  async create(files, archiveName, format = "zip", options = {}) {
    const response = await fetch(`${API_BASE}/archives/create`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({
        files,
        archive_name: archiveName,
        format,
        compression_level: options.compressionLevel,
        password: options.password,
        destination: options.destination,
      }),
    });
    return handleResponse(response);
  },

  /**
   * Extract an archive
   */
  async extract(archivePath, options = {}) {
    const response = await fetch(`${API_BASE}/archives/extract`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({
        archive_path: archivePath,
        destination: options.destination,
        password: options.password,
        flatten: options.flatten || false,
      }),
    });
    return handleResponse(response);
  },

  /**
   * Get archive contents/info
   */
  async getInfo(path) {
    const response = await fetch(`${API_BASE}/archives/${encodeURIComponent(path)}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get supported archive formats
   */
  async getFormats() {
    const response = await fetch(`${API_BASE}/archives/formats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Delete an archive
   */
  async delete(path) {
    const response = await fetch(`${API_BASE}/archives/${encodeURIComponent(path)}`, {
      method: "DELETE",
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// ==================== COMPRESSION ====================
const compression = {
  /**
   * Compress files
   */
  async compress(files, algorithm = "gzip", options = {}) {
    const response = await fetch(`${API_BASE}/compression/compress`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({
        files,
        algorithm,
        level: options.level || 6,
        destination: options.destination,
        delete_originals: options.deleteOriginals || false,
      }),
    });
    return handleResponse(response);
  },

  /**
   * Decompress files
   */
  async decompress(files, options = {}) {
    const response = await fetch(`${API_BASE}/compression/decompress`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({
        files,
        destination: options.destination,
        delete_originals: options.deleteOriginals || false,
      }),
    });
    return handleResponse(response);
  },

  /**
   * Analyze file compression potential
   */
  async analyze(path) {
    const response = await fetch(`${API_BASE}/compression/analyze?path=${encodeURIComponent(path)}`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get available compression algorithms
   */
  async getAlgorithms() {
    const response = await fetch(`${API_BASE}/compression/algorithms`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },

  /**
   * Get compression statistics
   */
  async getStats() {
    const response = await fetch(`${API_BASE}/compression/stats`, {
      headers: getHeaders(),
    });
    return handleResponse(response);
  },
};

// Add late-defined exports to api object
Object.assign(api, { encryption, quota, groups, guests, rateLimiting, admin, ftp, email, oauth, ldap, archives, compression });
