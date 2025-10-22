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
// FILE ENDPOINTS
// ============================================

export const files = {
  /**
   * List files in a directory
   */
  async list(path = "") {
    // Don't encode slashes - only encode individual path segments
    // Backend expects /api/files/ for root, /api/files/folder/ for folders
    const cleanPath = path.replace(/^\/+|\/+$/g, ''); // Remove leading/trailing slashes
    const encodedPath = cleanPath
      .split('/')
      .map(segment => encodeURIComponent(segment))
      .join('/');
    
    const response = await fetch(`${API_BASE}/files/${encodedPath}`, {
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
    // Don't manually encode - fetch API does it automatically
    const response = await fetch(`${API_BASE}/dirs/${cleanPath}`, {
      method: "POST",
      headers: getHeaders(false),
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
   * Get file stats
   */
  async stats() {
    const response = await fetch(`${API_BASE}/stats`, {
      headers: getHeaders(false),
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
// WEBSOCKET
// ============================================

export function createWebSocket() {
  const ws = new WebSocket("ws://localhost:8080/api/ws");
  return ws;
}

export default {
  auth,
  files,
  search,
  config,
  peers,
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

