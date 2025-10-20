// ========================
// State Management
// ========================

const API_BASE = 'http://localhost:8080/api';
const WS_URL = 'ws://localhost:8080/api/ws';
let ws = null;
let currentPath = '';
let currentUser = { username: 'admin', has2FA: false };
let authToken = null;

// ========================
// Authentication
// ========================

function initAuth() {
  // Check for stored auth token
  authToken = localStorage.getItem('authToken');
  const theme = localStorage.getItem('theme') || 'light';
  const userStr = localStorage.getItem('user');
  
  if (authToken && userStr) {
    currentUser = JSON.parse(userStr);
    showDashboard();
  } else {
    showLogin();
  }
  
  // Apply saved theme
  document.documentElement.setAttribute('data-theme', theme);
  const darkModeToggle = document.getElementById('darkModeToggle');
  if (darkModeToggle) {
    darkModeToggle.checked = theme === 'dark';
  }
}

function showLogin() {
  document.getElementById('loginPage').classList.remove('hidden');
  document.getElementById('dashboardPage').classList.add('hidden');
}

function showDashboard() {
  document.getElementById('loginPage').classList.add('hidden');
  document.getElementById('dashboardPage').classList.remove('hidden');
  document.getElementById('currentUser').textContent = currentUser.username;
  
  // Initialize dashboard
  loadFiles();
  loadStats();
  loadPeers();
  initWebSocket();
  
  // Update 2FA status
  update2FAStatus();
}

async function handleLogin(e) {
  e.preventDefault();
  
  const username = document.getElementById('username').value;
  const password = document.getElementById('password').value;
  const totp = document.getElementById('totp').value;
  
  showLoading(true);
  
  try {
    // Real API call to backend
    const response = await fetch(`${API_BASE}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        username: username,
        password: password,
        totp_code: totp || null
      })
    });
    
    const data = await response.json();
    
    if (!response.ok) {
      showNotification(data.error || 'Login failed', 'error');
      showLoading(false);
      return;
    }
    
    // Check if 2FA is required
    if (data.requires_2fa && !totp) {
      document.getElementById('2faGroup').classList.remove('hidden');
      showNotification('Please enter your 2FA code', 'error');
      showLoading(false);
      return;
    }
    
    // Successful login
    authToken = data.token;
    currentUser = {
      username: data.user.username,
      id: data.user.id,
      has2FA: data.user.totp_enabled
    };
    
    // Store auth data
    localStorage.setItem('authToken', authToken);
    localStorage.setItem('user', JSON.stringify(currentUser));
    
    showNotification('✅ Login successful!', 'success');
    showDashboard();
  } catch (error) {
    showNotification('Login failed: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

function handleLogout() {
  if (!confirm('Are you sure you want to logout?')) return;
  
  // Clear auth data
  localStorage.removeItem('authToken');
  localStorage.removeItem('user');
  authToken = null;
  currentUser = { username: '', has2FA: false };
  
  // Close WebSocket
  if (ws) {
    ws.close();
    ws = null;
  }
  
  showNotification('Logged out successfully', 'success');
  showLogin();
}

// ========================
// Theme Management
// ========================

function toggleTheme() {
  const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
  const newTheme = currentTheme === 'light' ? 'dark' : 'light';
  
  document.documentElement.setAttribute('data-theme', newTheme);
  localStorage.setItem('theme', newTheme);
  
  // Update toggle if on settings page
  const darkModeToggle = document.getElementById('darkModeToggle');
  if (darkModeToggle) {
    darkModeToggle.checked = newTheme === 'dark';
  }
  
  showNotification(`${newTheme === 'dark' ? '🌙' : '☀️'} ${newTheme.charAt(0).toUpperCase() + newTheme.slice(1)} mode enabled`, 'success');
}

// ========================
// 2FA Management
// ========================

// Setup 2FA
function update2FAStatus() {
  const statusEl = document.getElementById('2faStatus');
  if (!statusEl) return;
  
  if (currentUser.has2FA) {
    statusEl.textContent = '✅ Enabled';
    statusEl.style.color = 'var(--success)';
  } else {
    statusEl.textContent = '⚠️ Not enabled';
    statusEl.style.color = 'var(--error)';
  }
}

function toggle2FASetup() {
  const setupDiv = document.getElementById('2faSetup');
  const isHidden = setupDiv.classList.contains('hidden');
  
  if (isHidden) {
    // Call backend to get secret
    fetch(`${API_BASE}/auth/2fa/setup`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${authToken}`,
        'Content-Type': 'application/json'
      }
    })
    .then(res => res.json())
    .then(data => {
      document.getElementById('2faSecret').textContent = `Secret: ${data.secret}`;
      generateQRCode(data.qr_url);
      // Store secret temporarily for verification
      window.temp2FASecret = data.secret;
      setupDiv.classList.remove('hidden');
    })
    .catch(err => {
      showNotification('Failed to setup 2FA: ' + err.message, 'error');
    });
  } else {
    setupDiv.classList.add('hidden');
  }
}

function generateTOTPSecret() {
  // Generate a random base32 secret (16 characters)
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
  let secret = '';
  for (let i = 0; i < 16; i++) {
    secret += chars[Math.floor(Math.random() * chars.length)];
  }
  return secret;
}

function generateQRCode(text) {
  const canvas = document.getElementById('qrCanvas');
  const ctx = canvas.getContext('2d');
  const size = 256;
  const qrSize = 29; // Standard QR code size for this amount of data
  const moduleSize = Math.floor(size / qrSize);
  
  // Clear canvas
  ctx.fillStyle = 'white';
  ctx.fillRect(0, 0, size, size);
  
  // Generate simple QR pattern (mock - use a real QR library in production)
  ctx.fillStyle = 'black';
  
  // Create a simple pattern (not a real QR code - use qrcode.js in production)
  for (let i = 0; i < qrSize; i++) {
    for (let j = 0; j < qrSize; j++) {
      // Random pattern for demo purposes
      if (Math.random() > 0.5) {
        ctx.fillRect(j * moduleSize, i * moduleSize, moduleSize, moduleSize);
      }
    }
  }
  
  // Add finder patterns (corners)
  const drawFinderPattern = (x, y) => {
    ctx.fillStyle = 'black';
    ctx.fillRect(x * moduleSize, y * moduleSize, 7 * moduleSize, 7 * moduleSize);
    ctx.fillStyle = 'white';
    ctx.fillRect((x + 1) * moduleSize, (y + 1) * moduleSize, 5 * moduleSize, 5 * moduleSize);
    ctx.fillStyle = 'black';
    ctx.fillRect((x + 2) * moduleSize, (y + 2) * moduleSize, 3 * moduleSize, 3 * moduleSize);
  };
  
  drawFinderPattern(0, 0);
  drawFinderPattern(qrSize - 7, 0);
  drawFinderPattern(0, qrSize - 7);
}

function verifyTOTPCode(secret, code) {
  // Mock verification - in production, use a TOTP library
  // For demo, accept any 6-digit code
  return /^\d{6}$/.test(code);
}

async function verify2FA() {
  const code = document.getElementById('verify2fa').value;
  
  if (!code || code.length !== 6) {
    showNotification('Please enter a valid 6-digit code', 'error');
    return;
  }
  
  showLoading(true);
  
  try {
    const response = await fetch(`${API_BASE}/auth/2fa/enable`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${authToken}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ totp_code: code })
    });
    
    const data = await response.json();
    
    if (!response.ok) {
      showNotification(data.error || 'Invalid code', 'error');
      showLoading(false);
      return;
    }
    
    // Update user state
    currentUser.has2FA = true;
    localStorage.setItem('user', JSON.stringify(currentUser));
    
    update2FAStatus();
    toggle2FASetup();
    
    showNotification('✅ 2FA enabled successfully!', 'success');
  } catch (error) {
    showNotification('Failed to enable 2FA: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

function showChangePassword() {
  const oldPass = prompt('Enter current password:');
  if (!oldPass) return;
  
  const newPass = prompt('Enter new password:');
  if (!newPass) return;
  
  const confirmPass = prompt('Confirm new password:');
  if (newPass !== confirmPass) {
    showNotification('Passwords do not match', 'error');
    return;
  }
  
  showLoading(true);
  
  // Call backend API
  fetch(`${API_BASE}/auth/change-password`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${authToken}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      old_password: oldPass,
      new_password: newPass
    })
  })
  .then(res => res.json())
  .then(data => {
    if (data.error) {
      showNotification(data.error, 'error');
    } else {
      showNotification('✅ Password changed successfully!', 'success');
    }
  })
  .catch(err => {
    showNotification('Failed to change password: ' + err.message, 'error');
  })
  .finally(() => {
    showLoading(false);
  });
}

// ========================
// Navigation
// ========================

function showPage(page) {
  // Hide all pages
  ['files', 'search', 'peers', 'settings'].forEach(p => {
    document.getElementById(p + 'Page').classList.add('hidden');
  });
  
  // Show selected page
  document.getElementById(page + 'Page').classList.remove('hidden');
  
  // Update active menu item
  document.querySelectorAll('.sidebar-menu a').forEach(a => {
    a.classList.remove('active');
  });
  event.target.classList.add('active');
  
  // Load page data
  if (page === 'files') loadFiles();
  if (page === 'peers') loadPeers();
}

// ========================
// File Management
// ========================

async function loadFiles() {
  showLoading(true);
  try {
    const response = await fetch(`${API_BASE}/files/${currentPath}`, {
      headers: {
        'Authorization': `Bearer ${authToken}`
      }
    });
    if (!response.ok) throw new Error('Failed to load files');
    
    const files = await response.json();
    displayFiles(files);
    updateBreadcrumb();
  } catch (error) {
    showNotification('Error loading files: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

function displayFiles(files) {
  const list = document.getElementById('fileList');
  
  if (files.length === 0) {
    list.innerHTML = '<li style="padding: 20px; text-align: center; color: var(--secondary);">No files in this directory</li>';
    return;
  }
  
  list.innerHTML = files.map(file => `
    <li class="file-item">
      <div class="file-name ${file.is_directory ? 'directory' : ''}" onclick="${file.is_directory ? `navigateTo('${file.name}')` : ''}">
        <span>${file.is_directory ? '📁' : '📄'}</span>
        <span>${file.name}</span>
        ${!file.is_directory ? `<span style="color: var(--secondary); font-size: 14px; margin-left: auto;">${formatSize(file.size)}</span>` : ''}
      </div>
      <div class="file-actions">
        ${!file.is_directory ? `<button class="btn btn-secondary btn-sm" onclick="downloadFile('${file.name}')">Download</button>` : ''}
        <button class="btn btn-secondary btn-sm" onclick="renameFile('${file.name}')">Rename</button>
        <button class="btn btn-secondary btn-sm" onclick="deleteFile('${file.name}', ${file.is_directory})">Delete</button>
      </div>
    </li>
  `).join('');
}

function updateBreadcrumb() {
  const breadcrumb = document.getElementById('breadcrumb');
  if (!currentPath) {
    breadcrumb.innerHTML = '';
    return;
  }
  
  const parts = currentPath.split('/').filter(p => p);
  breadcrumb.innerHTML = parts.map((part, index) => {
    const path = parts.slice(0, index + 1).join('/');
    return `<span>›</span><span onclick="navigateTo('${path}')">${part}</span>`;
  }).join('');
}

function navigateTo(path) {
  currentPath = path;
  loadFiles();
}

function navigateToRoot() {
  currentPath = '';
  loadFiles();
}

async function uploadFile(file) {
  showLoading(true);
  try {
    const path = currentPath ? `${currentPath}/${file.name}` : file.name;
    const response = await fetch(`${API_BASE}/upload/${path}`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${authToken}`
      },
      body: await file.arrayBuffer()
    });
    
    if (!response.ok) throw new Error('Upload failed');
    
    showNotification(`✅ Uploaded: ${file.name}`, 'success');
    loadFiles();
    loadStats();
  } catch (error) {
    showNotification('Upload error: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

async function downloadFile(filename) {
  showLoading(true);
  try {
    const path = currentPath ? `${currentPath}/${filename}` : filename;
    const response = await fetch(`${API_BASE}/file/${path}`, {
      headers: {
        'Authorization': `Bearer ${authToken}`
      }
    });
    if (!response.ok) throw new Error('Download failed');
    
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    a.remove();
    window.URL.revokeObjectURL(url);
    
    showNotification(`✅ Downloaded: ${filename}`, 'success');
  } catch (error) {
    showNotification('Download error: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

async function deleteFile(filename, isDirectory) {
  if (!confirm(`Delete ${isDirectory ? 'directory' : 'file'}: ${filename}?`)) return;
  
  showLoading(true);
  try {
    const path = currentPath ? `${currentPath}/${filename}` : filename;
    const response = await fetch(`${API_BASE}/files/${path}`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${authToken}`
      }
    });
    
    if (!response.ok) throw new Error('Delete failed');
    
    showNotification(`🗑️ Deleted: ${filename}`, 'success');
    loadFiles();
    loadStats();
  } catch (error) {
    showNotification('Delete error: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

async function renameFile(oldName) {
  const newName = prompt('New name:', oldName);
  if (!newName || newName === oldName) return;
  
  showLoading(true);
  try {
    const oldPath = currentPath ? `${currentPath}/${oldName}` : oldName;
    const newPath = currentPath ? `${currentPath}/${newName}` : newName;
    
    const response = await fetch(`${API_BASE}/rename/${encodeURIComponent(oldPath)}`, {
      method: 'PUT',
      headers: { 
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${authToken}`
      },
      body: JSON.stringify({ new_path: newPath })
    });
    
    if (!response.ok) throw new Error('Rename failed');
    
    showNotification(`✏️ Renamed: ${oldName} → ${newName}`, 'success');
    loadFiles();
  } catch (error) {
    showNotification('Rename error: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

async function createDir() {
  const name = document.getElementById('dirInput').value.trim();
  if (!name) {
    showNotification('Please enter a directory name', 'error');
    return;
  }
  
  showLoading(true);
  try {
    const path = currentPath ? `${currentPath}/${name}` : name;
    const response = await fetch(`${API_BASE}/dirs/${path}`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${authToken}`
      }
    });
    
    if (!response.ok) throw new Error('Create directory failed');
    
    showNotification(`📁 Created: ${name}`, 'success');
    document.getElementById('dirInput').value = '';
    loadFiles();
  } catch (error) {
    showNotification('Create directory error: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

// ========================
// Search
// ========================

async function searchFiles() {
  const query = document.getElementById('searchInput').value.trim();
  if (!query) {
    showNotification('Please enter a search term', 'error');
    return;
  }
  
  showLoading(true);
  try {
    const response = await fetch(`${API_BASE}/search?q=${encodeURIComponent(query)}`);
    if (!response.ok) throw new Error('Search failed');
    
    const results = await response.json();
    displaySearchResults(results);
  } catch (error) {
    showNotification('Search error: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

function displaySearchResults(results) {
  const container = document.getElementById('searchResults');
  
  if (results.length === 0) {
    container.innerHTML = '<p style="padding: 20px; text-align: center; color: var(--secondary);">No results found</p>';
    return;
  }
  
  container.innerHTML = `
    <p style="margin-bottom: 16px; color: var(--secondary);">Found ${results.length} result(s)</p>
    <ul class="file-list">
      ${results.map(r => `
        <li class="file-item">
          <div class="file-name">
            <span>${r.is_directory ? '📁' : '📄'}</span>
            <span>${r.path}</span>
            ${!r.is_directory ? `<span style="color: var(--secondary); font-size: 14px; margin-left: auto;">${formatSize(r.size)}</span>` : ''}
          </div>
        </li>
      `).join('')}
    </ul>
  `;
}

// ========================
// Statistics
// ========================

async function loadStats() {
  try {
    const response = await fetch(`${API_BASE}/stats`, {
      headers: {
        'Authorization': `Bearer ${authToken}`
      }
    });
    if (!response.ok) throw new Error('Failed to load stats');
    
    const stats = await response.json();
    document.getElementById('stats').textContent = 
      `📊 ${stats.file_count} files • 💾 ${formatSize(stats.total_size)} total`;
  } catch (error) {
    document.getElementById('stats').textContent = 'Error loading statistics';
  }
}

// ========================
// Peers
// ========================

async function loadPeers() {
  showLoading(true);
  try {
    const response = await fetch(`${API_BASE}/peers`, {
      headers: {
        'Authorization': `Bearer ${authToken}`
      }
    });
    if (!response.ok) throw new Error('Failed to load peers');
    
    const peers = await response.json();
    displayPeers(peers);
  } catch (error) {
    showNotification('Error loading peers: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

function displayPeers(peers) {
  const list = document.getElementById('peerList');
  
  if (peers.length === 0) {
    list.innerHTML = '<li style="padding: 20px; text-align: center; color: var(--secondary);">No peers connected</li>';
    return;
  }
  
  list.innerHTML = peers.map(peer => `
    <li class="file-item">
      <div class="file-name">
        <span>🖥️</span>
        <span>${peer.address}</span>
        <span style="color: var(--secondary); font-size: 14px; margin-left: 12px;">ID: ${peer.id.substring(0, 8)}...</span>
      </div>
      <button class="btn btn-secondary btn-sm" onclick="removePeer('${peer.id}')">Remove</button>
    </li>
  `).join('');
}

async function addPeer() {
  const address = document.getElementById('peerInput').value.trim();
  if (!address) {
    showNotification('Please enter a peer address', 'error');
    return;
  }
  
  showLoading(true);
  try {
    const response = await fetch(`${API_BASE}/peers`, {
      method: 'POST',
      headers: { 
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${authToken}`
      },
      body: JSON.stringify({
        id: crypto.randomUUID(),
        address: address
      })
    });
    
    if (!response.ok) throw new Error('Failed to add peer');
    
    showNotification(`✅ Added peer: ${address}`, 'success');
    document.getElementById('peerInput').value = '';
    loadPeers();
  } catch (error) {
    showNotification('Add peer error: ' + error.message, 'error');
  } finally {
    showLoading(false);
  }
}

async function removePeer(id) {
  if (!confirm('Remove this peer?')) return;
  
  // In production, implement DELETE /api/peers/:id endpoint
  showNotification('⚠️ Remove peer not yet implemented', 'error');
}

// ========================
// WebSocket
// ========================

function initWebSocket() {
  if (ws) return;
  
  ws = new WebSocket(WS_URL);
  
  ws.onopen = () => {
    console.log('WebSocket connected');
  };
  
  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      console.log('File change:', data);
      
      // Refresh files and stats
      loadFiles();
      loadStats();
      
      showNotification(`📁 ${data.kind}: ${data.path}`, 'success');
    } catch (error) {
      console.error('WebSocket message error:', error);
    }
  };
  
  ws.onerror = (error) => {
    console.error('WebSocket error:', error);
  };
  
  ws.onclose = () => {
    console.log('WebSocket disconnected');
    ws = null;
    
    // Reconnect after 5 seconds
    setTimeout(() => {
      if (authToken) initWebSocket();
    }, 5000);
  };
}

// ========================
// Drag & Drop
// ========================

function initDragDrop() {
  const dropZone = document.getElementById('dropZone');
  const fileInput = document.getElementById('fileInput');
  
  ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
    dropZone.addEventListener(eventName, preventDefaults, false);
  });
  
  function preventDefaults(e) {
    e.preventDefault();
    e.stopPropagation();
  }
  
  ['dragenter', 'dragover'].forEach(eventName => {
    dropZone.addEventListener(eventName, () => {
      dropZone.classList.add('dragging');
    }, false);
  });
  
  ['dragleave', 'drop'].forEach(eventName => {
    dropZone.addEventListener(eventName, () => {
      dropZone.classList.remove('dragging');
    }, false);
  });
  
  dropZone.addEventListener('drop', (e) => {
    const files = e.dataTransfer.files;
    handleFiles(files);
  }, false);
  
  dropZone.addEventListener('click', () => {
    fileInput.click();
  });
  
  fileInput.addEventListener('change', (e) => {
    const files = e.target.files;
    handleFiles(files);
  });
}

function handleFiles(files) {
  [...files].forEach(uploadFile);
}

// ========================
// UI Helpers
// ========================

function showLoading(show) {
  const overlay = document.getElementById('loadingOverlay');
  if (show) {
    overlay.classList.remove('hidden');
  } else {
    overlay.classList.add('hidden');
  }
}

function showNotification(message, type = 'success') {
  const container = document.getElementById('toastContainer');
  const toast = document.createElement('div');
  toast.className = `toast ${type}`;
  toast.textContent = message;
  
  container.appendChild(toast);
  
  setTimeout(() => {
    toast.remove();
  }, 4000);
}

function formatSize(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// ========================
// Initialization
// ========================

document.addEventListener('DOMContentLoaded', () => {
  initAuth();
  initDragDrop();
});
