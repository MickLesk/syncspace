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
    // List all versions for a file
    async list(fileId) {
      const response = await fetch(`${API_BASE}/files/${fileId}/versions`, {
        headers: getHeaders()
      });
      return { data: await handleResponse(response) };
    },

    // Create a new version of a file
    async create(fileId, versionData) {
      const response = await fetch(`${API_BASE}/files/${fileId}/versions`, {
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
      const response = await fetch(`${API_BASE}/files/${versionId}/versions/restore`, {
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

/**
 * Setup wizard endpoints (first-time installation)
 */
const setup = {
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
};
