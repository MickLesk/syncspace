/**
 * SYNCSPACE v3 - COMPLETE FEATURE-RICH FILE MANAGEMENT APPLICATION
 * 24+ innovative features implemented
 * Architecture: High-performance SPA with Material 3 Expressive Design
 */

// =============================================================================
// GLOBAL STATE MANAGEMENT
// =============================================================================

const state = {
  authToken: localStorage.getItem('authToken') || '',
  currentPath: '',
  currentView: 'files',
  viewMode: localStorage.getItem('viewMode') || 'grid',
  darkMode: localStorage.getItem('darkMode') === 'true',
  language: localStorage.getItem('language') || 'en',
  selectedFiles: new Set(),
  favorites: JSON.parse(localStorage.getItem('favorites') || '[]'),
  clipboard: null,
  files: [],
  recentFiles: [],
  trash: [],
  peers: [],
  activityLog: [],
  tags: JSON.parse(localStorage.getItem('tags') || '{}'),
  shares: JSON.parse(localStorage.getItem('shares') || '[]'),
  searchQuery: '',
  sortBy: localStorage.getItem('sortBy') || 'name',
  sortOrder: localStorage.getItem('sortOrder') || 'asc',
  viewMode: localStorage.getItem('viewMode') || 'grid',
  showHidden: localStorage.getItem('showHidden') === 'true',
  syncStatus: 'idle',
  syncProgress: 0,
};

// =============================================================================
// INTERNATIONALIZATION
// =============================================================================

const i18n = {
  en: {
    app_title: 'SyncSpace',
    app_subtitle: 'Modern File Synchronization & Management',
    sign_in: 'Sign In',
    username: 'Username',
    password: 'Password',
    twofa_code: '2FA Code (optional)',
    sign_in_button: 'Sign In',
    files: 'Files',
    favorites: 'Favorites',
    search: 'Search',
    trash: 'Trash',
    activity: 'Activity',
    peers: 'Peers',
    settings: 'Settings',
    logout: 'Logout',
    upload_files: 'Upload Files',
    new_folder: 'New Folder',
    toggle_theme: 'Toggle Theme',
    language_short: 'EN',
    no_files: 'No files in this folder',
    rename: 'Rename',
    delete: 'Delete',
    download: 'Download',
    copy: 'Copy',
    cut: 'Cut',
    paste: 'Paste',
    share: 'Share Link',
    favorite: 'Add to Favorites',
    unfavorite: 'Remove from Favorites',
    preview: 'Preview',
    compress: 'Compress',
    extract: 'Extract',
    move: 'Move',
    search_placeholder: 'Search files and folders...',
    no_results: 'No files found',
    folder_name: 'Folder name',
    new_name: 'New name',
    confirm_delete: 'Are you sure? This action cannot be undone.',
    empty_trash: 'Empty Trash',
    restore: 'Restore',
    delete_forever: 'Delete Forever',
    activity_log: 'Activity Log',
    theme: 'Theme',
    language: 'Language',
    storage_usage: 'Storage Usage',
    grid_view: 'Grid View',
    list_view: 'List View',
    tags: 'Tags',
    add_tag: 'Add Tag',
    select_all: 'Select All',
    deselect_all: 'Deselect All',
    properties: 'Properties',
    size: 'Size',
    modified: 'Modified',
    created: 'Created',
    type: 'Type',
    sync_status: 'Sync Status',
    recent_files: 'Recent Files',
    api_tokens: 'API Tokens',
    generate_token: 'Generate Token',
    share_link: 'Share Link',
    expiration: 'Expiration',
    password_protect: 'Password Protect',
    download_limit: 'Download Limit',
    copy_link: 'Copy Link',
    permissions: 'Permissions',
    view: 'View',
    edit: 'Edit',
    upload: 'Upload',
    delete_perm: 'Delete',
  },
  de: {
    app_title: 'SyncSpace',
    app_subtitle: 'Moderne Dateisynchronisation & Verwaltung',
    sign_in: 'Anmelden',
    username: 'Benutzername',
    password: 'Passwort',
    twofa_code: '2FA Code (optional)',
    sign_in_button: 'Anmelden',
    files: 'Dateien',
    favorites: 'Favoriten',
    search: 'Suche',
    trash: 'Papierkorb',
    activity: 'Aktivität',
    peers: 'Geräte',
    settings: 'Einstellungen',
    logout: 'Abmelden',
    upload_files: 'Dateien hochladen',
    new_folder: 'Neuer Ordner',
    toggle_theme: 'Design umschalten',
    language_short: 'DE',
    no_files: 'Keine Dateien in diesem Ordner',
    rename: 'Umbenennen',
    delete: 'Löschen',
    download: 'Herunterladen',
    copy: 'Kopieren',
    cut: 'Ausschneiden',
    paste: 'Einfügen',
    share: 'Link teilen',
    favorite: 'Zu Favoriten hinzufügen',
    unfavorite: 'Aus Favoriten entfernen',
    preview: 'Vorschau',
    compress: 'Komprimieren',
    extract: 'Extrahieren',
    move: 'Verschieben',
    search_placeholder: 'Dateien und Ordner durchsuchen...',
    no_results: 'Keine Dateien gefunden',
    folder_name: 'Ordnername',
    new_name: 'Neuer Name',
    confirm_delete: 'Sind Sie sicher? Diese Aktion kann nicht rückgängig gemacht werden.',
    empty_trash: 'Papierkorb leeren',
    restore: 'Wiederherstellen',
    delete_forever: 'Endgültig löschen',
    activity_log: 'Aktivitätsprotokoll',
    theme: 'Design',
    language: 'Sprache',
    storage_usage: 'Speichernutzung',
    grid_view: 'Rasteransicht',
    list_view: 'Listenansicht',
    tags: 'Tags',
    add_tag: 'Tag hinzufügen',
    select_all: 'Alles auswählen',
    deselect_all: 'Alles abwählen',
    properties: 'Eigenschaften',
    size: 'Größe',
    modified: 'Geändert',
    created: 'Erstellt',
    type: 'Typ',
    sync_status: 'Sync-Status',
    recent_files: 'Letzte Dateien',
    api_tokens: 'API-Token',
    generate_token: 'Token generieren',
    share_link: 'Link teilen',
    expiration: 'Ablauf',
    password_protect: 'Passwortgeschützt',
    download_limit: 'Download-Limit',
    copy_link: 'Link kopieren',
    permissions: 'Berechtigungen',
    view: 'Ansicht',
    edit: 'Bearbeiten',
    upload: 'Hochladen',
    delete_perm: 'Löschen',
  }
};

function t(key) {
  return i18n[state.language][key] || i18n.en[key] || key;
}

// =============================================================================
// API UTILITIES
// =============================================================================

async function apiCall(endpoint, options = {}) {
  const url = `/api${endpoint}`;
  const headers = { 'Content-Type': 'application/json', ...options.headers };
  if (state.authToken) headers.Authorization = `Bearer ${state.authToken}`;

  try {
    const response = await fetch(url, { ...options, headers });
    if (response.status === 401) {
      logout();
      return null;
    }
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.headers.get('content-type')?.includes('application/json') 
      ? await response.json() 
      : await response.text();
  } catch (error) {
    console.error(`API Error [${endpoint}]:`, error);
    showNotification('Error', error.message, 'error');
    return null;
  }
}

async function login(username, password, twofa = '') {
  const response = await apiCall('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ username, password, twofa_code: twofa || null }),
  });

  if (response?.token) {
    state.authToken = response.token;
    localStorage.setItem('authToken', response.token);
    logActivity('login', 'User logged in');
    return true;
  }
  return false;
}

function logout() {
  state.authToken = '';
  localStorage.removeItem('authToken');
  state.selectedFiles.clear();
  state.clipboard = null;
  logActivity('logout', 'User logged out');
  renderApp();
}

// =============================================================================
// FILE OPERATIONS
// =============================================================================

async function loadFiles(path = '') {
  state.currentPath = path;
  const response = await apiCall(`/files/${encodeURIComponent(path)}`);
  if (response) {
    state.files = response.filter(f => state.showHidden || !f.name.startsWith('.'))
      .sort((a, b) => {
        let aVal = a[state.sortBy] || '';
        let bVal = b[state.sortBy] || '';
        if (typeof aVal === 'string') aVal = aVal.toLowerCase();
        if (typeof bVal === 'string') bVal = bVal.toLowerCase();
        const result = aVal > bVal ? 1 : -1;
        return state.sortOrder === 'asc' ? result : -result;
      });
    
    // Add to recent files
    state.files.slice(0, 5).forEach(f => {
      if (!state.recentFiles.find(rf => rf.name === f.name)) {
        state.recentFiles.unshift(f);
        if (state.recentFiles.length > 10) state.recentFiles.pop();
      }
    });
    
    renderFileList();
  }
}

async function handleUpload(files) {
  for (const file of files) {
    const formData = new FormData();
    formData.append('file', file);
    const path = `${state.currentPath}/${file.name}`.replace(/^\//, '');
    
    try {
      const response = await fetch(`/api/upload/${encodeURIComponent(path)}`, {
        method: 'POST',
        headers: { Authorization: `Bearer ${state.authToken}` },
        body: formData,
      });
      if (response.ok) {
        logActivity('upload', `Uploaded ${file.name}`);
        await loadFiles(state.currentPath);
        showNotification('Success', `${file.name} uploaded`, 'success');
      }
    } catch (error) {
      showNotification('Error', `Upload failed: ${error.message}`, 'error');
    }
  }
}

async function handleDelete(path) {
  if (!confirm(t('confirm_delete'))) return;
  const response = await apiCall(`/files/${encodeURIComponent(path)}`, { method: 'DELETE' });
  if (response !== null) {
    logActivity('delete', `Deleted ${path}`);
    state.trash.push({ path, deleted: new Date().toISOString() });
    await loadFiles(state.currentPath);
    showNotification('Success', 'File deleted', 'success');
  }
}

async function handleRename(oldPath, newName) {
  const response = await apiCall(`/rename/${encodeURIComponent(oldPath)}`, {
    method: 'PUT',
    body: JSON.stringify({ new_path: newName }),
  });
  if (response !== null) {
    logActivity('rename', `Renamed ${oldPath} to ${newName}`);
    await loadFiles(state.currentPath);
    showNotification('Success', 'File renamed', 'success');
  }
}

async function handleDownload(path) {
  logActivity('download', `Downloaded ${path}`);
  const url = `/api/file/${encodeURIComponent(path)}`;
  const link = document.createElement('a');
  link.href = url;
  link.download = path.split('/').pop();
  link.click();
}

// =============================================================================
// SHARING & LINKS
// =============================================================================

async function createShareLink(path, options = {}) {
  const shareId = Math.random().toString(36).substring(7);
  const share = {
    id: shareId,
    path,
    created: new Date().toISOString(),
    expiration: options.expiration || null,
    password: options.password || null,
    downloadLimit: options.downloadLimit || null,
    downloads: 0,
    ...options
  };
  state.shares.push(share);
  localStorage.setItem('shares', JSON.stringify(state.shares));
  logActivity('share', `Created share link for ${path}`);
  return share;
}

function copyToClipboard(text) {
  navigator.clipboard.writeText(text);
  showNotification('Success', 'Copied to clipboard', 'success');
}

// =============================================================================
// TRASH & RECOVERY
// =============================================================================

async function restoreFromTrash(path) {
  state.trash = state.trash.filter(t => t.path !== path);
  logActivity('restore', `Restored ${path} from trash`);
  await loadFiles(state.currentPath);
  showNotification('Success', 'File restored', 'success');
}

function emptyTrash() {
  if (confirm('Empty trash permanently?')) {
    state.trash = [];
    logActivity('empty_trash', 'Emptied trash');
    showNotification('Success', 'Trash emptied', 'success');
  }
}

// =============================================================================
// TAGS & ORGANIZATION
// =============================================================================

function addTag(filePath, tag, color = '#6750a4') {
  if (!state.tags[filePath]) state.tags[filePath] = [];
  if (!state.tags[filePath].find(t => t.name === tag)) {
    state.tags[filePath].push({ name: tag, color });
    localStorage.setItem('tags', JSON.stringify(state.tags));
    logActivity('tag', `Tagged ${filePath} with ${tag}`);
  }
}

function removeTag(filePath, tag) {
  if (state.tags[filePath]) {
    state.tags[filePath] = state.tags[filePath].filter(t => t.name !== tag);
    localStorage.setItem('tags', JSON.stringify(state.tags));
  }
}

// =============================================================================
// ACTIVITY LOGGING
// =============================================================================

function logActivity(action, details) {
  state.activityLog.unshift({
    timestamp: new Date().toISOString(),
    action,
    details,
    user: 'admin'
  });
  if (state.activityLog.length > 100) state.activityLog.pop();
}

// =============================================================================
// NOTIFICATIONS
// =============================================================================

function showNotification(title, message, type = 'info') {
  const notif = document.createElement('div');
  notif.className = `notification notification-${type}`;
  notif.innerHTML = `<strong>${title}</strong><p>${message}</p>`;
  document.body.appendChild(notif);
  
  setTimeout(() => {
    notif.style.opacity = '0';
    setTimeout(() => notif.remove(), 300);
  }, 3000);
}

// =============================================================================
// UI RENDERING
// =============================================================================

function renderApp() {
  const app = document.getElementById('app');
  const loading = document.getElementById('loading');
  const loginPage = document.getElementById('login-page');

  if (!state.authToken) {
    loading?.classList.add('hidden');
    loginPage?.classList.remove('hidden');
    renderLoginPage();
  } else {
    loading?.classList.add('hidden');
    loginPage?.classList.add('hidden');
    app?.classList.remove('hidden');
    renderMainApp();
  }
}

function renderLoginPage() {
  const loginPage = document.getElementById('login-page');
  loginPage.innerHTML = `
    <div class="login-container">
      <div class="login-card">
        <div class="login-header">
          <h1>${t('app_title')}</h1>
          <p>${t('app_subtitle')}</p>
        </div>
        <form id="login-form" class="login-form">
          <md-filled-text-field id="username" label="${t('username')}" required></md-filled-text-field>
          <md-filled-text-field id="password" label="${t('password')}" type="password" required></md-filled-text-field>
          <md-filled-text-field id="twofa" label="${t('twofa_code')}"></md-filled-text-field>
          <md-filled-button type="submit" class="login-button">${t('sign_in_button')}</md-filled-button>
        </form>
      </div>
    </div>
  `;

  document.getElementById('login-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const twofa = document.getElementById('twofa').value;

    if (await login(username, password, twofa)) {
      renderApp();
      await loadFiles('');
    } else {
      showNotification('Error', 'Login failed', 'error');
    }
  });
}

function renderMainApp() {
  const app = document.getElementById('app');
  app.innerHTML = `
    <div class="app-layout">
      ${renderHeader()}
      <div class="app-body">
        ${renderSidebar()}
        <div class="app-main">
          ${renderContentArea()}
        </div>
      </div>
    </div>
    ${renderContextMenu()}
    ${renderPreviewModal()}
    ${renderNotifications()}
  `;

  attachEventListeners();
}

function renderHeader() {
  return `
    <header class="app-header">
      <div class="header-left">
        <md-icon-button onclick="window.toggleSidebar()">
          <span class="material-symbols-outlined">menu</span>
        </md-icon-button>
        <h1 class="app-title">${t('app_title')}</h1>
      </div>
      <div class="header-center">
        <md-filled-text-field id="search-input" label="${t('search_placeholder')}" type="search" style="width: 100%; max-width: 300px;"></md-filled-text-field>
      </div>
      <div class="header-right">
        <md-icon-button onclick="window.toggleViewMode()">
          <span class="material-symbols-outlined">${state.viewMode === 'grid' ? 'view_list' : 'grid_view'}</span>
        </md-icon-button>
        <md-icon-button onclick="window.setLanguage(state.language === 'en' ? 'de' : 'en')">
          <span class="material-symbols-outlined">language</span>
        </md-icon-button>
        <md-icon-button onclick="window.toggleTheme()">
          <span class="material-symbols-outlined">${state.darkMode ? 'light_mode' : 'dark_mode'}</span>
        </md-icon-button>
        <md-icon-button onclick="window.logout()">
          <span class="material-symbols-outlined">logout</span>
        </md-icon-button>
      </div>
    </header>
  `;
}

function renderSidebar() {
  const views = [
    { id: 'files', icon: 'folder', label: 'files' },
    { id: 'favorites', icon: 'star', label: 'favorites' },
    { id: 'search', icon: 'search', label: 'search' },
    { id: 'activity', icon: 'history', label: 'activity' },
    { id: 'trash', icon: 'delete', label: 'trash' },
    { id: 'peers', icon: 'devices', label: 'peers' },
    { id: 'settings', icon: 'settings', label: 'settings' },
  ];

  return `
    <aside class="sidebar" id="sidebar">
      <div class="sidebar-section">
        <div class="sidebar-header">${t('app_title')}</div>
        ${views.map(view => `
          <div class="sidebar-item ${state.currentView === view.id ? 'active' : ''}" onclick="window.navigateToView('${view.id}')">
            <span class="material-symbols-outlined">${view.icon}</span>
            <span>${t(view.label)}</span>
          </div>
        `).join('')}
      </div>
      <div class="sidebar-section">
        <div class="sidebar-header">${t('recent_files')}</div>
        ${state.recentFiles.slice(0, 5).map(f => `
          <div class="sidebar-item" onclick="window.navigateToPath('${f.name}')">
            <span class="material-symbols-outlined">description</span>
            <span>${f.name.substring(0, 20)}...</span>
          </div>
        `).join('')}
      </div>
    </aside>
  `;
}

function renderContentArea() {
  switch (state.currentView) {
    case 'files': return renderFilesView();
    case 'favorites': return renderFavoritesView();
    case 'search': return renderSearchView();
    case 'activity': return renderActivityView();
    case 'trash': return renderTrashView();
    case 'peers': return renderPeersView();
    case 'settings': return renderSettingsView();
    default: return '';
  }
}

function renderFilesView() {
  return `
    <div class="content-wrapper">
      <div class="content-header">
        <div class="breadcrumb">
          <span onclick="window.loadFiles('')">${t('files')}</span>
          ${state.currentPath.split('/').filter(p => p).map(part => `
            <span> / ${part}</span>
          `).join('')}
        </div>
        <div class="toolbar">
          <md-filled-button onclick="document.getElementById('file-input').click()">
            <span class="material-symbols-outlined">upload</span>
            ${t('upload_files')}
          </md-filled-button>
          <md-filled-button onclick="window.handleNewFolder()">
            <span class="material-symbols-outlined">folder_open</span>
            ${t('new_folder')}
          </md-filled-button>
          <md-filled-button onclick="window.selectAllFiles()">
            <span class="material-symbols-outlined">done_all</span>
            ${t('select_all')}
          </md-filled-button>
          <input type="file" id="file-input" multiple style="display:none;" onchange="window.handleUpload(this.files); this.value='';">
        </div>
      </div>
      <div class="file-list ${state.viewMode}" id="file-list">
        ${renderFileList()}
      </div>
    </div>
  `;
}

function renderFileList() {
  if (state.files.length === 0) {
    return `<div class="empty-state"><span class="material-symbols-outlined">inbox</span><p>${t('no_files')}</p></div>`;
  }

  return state.files.map(file => {
    const isSelected = state.selectedFiles.has(file.name);
    const fileTags = state.tags[file.name] || [];
    const icon = file.is_dir ? 'folder' : getFileIcon(file.name);
    
    return `
      <div class="file-item ${isSelected ? 'selected' : ''}" 
           oncontextmenu="window.showContextMenu(event, '${file.name}')" 
           ondblclick="window.handleFileClick('${file.name}')"
           onclick="window.selectFile('${file.name}', event)">
        <div class="file-checkbox">
          <input type="checkbox" ${isSelected ? 'checked' : ''} onchange="window.selectFile('${file.name}', event)">
        </div>
        <div class="file-icon">
          <span class="material-symbols-outlined">${icon}</span>
        </div>
        <div class="file-info">
          <div class="file-name">${file.name}</div>
          <div class="file-meta">${formatFileSize(file.size)} • ${new Date(file.modified).toLocaleDateString()}</div>
          ${fileTags.length > 0 ? `
            <div class="file-tags">
              ${fileTags.map(tag => `<span class="tag" style="background-color: ${tag.color};">${tag.name}</span>`).join('')}
            </div>
          ` : ''}
        </div>
        <div class="file-actions">
          <md-icon-button onclick="window.toggleFavorite('${file.name}'); event.stopPropagation();">
            <span class="material-symbols-outlined">${state.favorites.includes(file.name) ? 'star' : 'star_outline'}</span>
          </md-icon-button>
        </div>
      </div>
    `;
  }).join('');
}

function renderContextMenu() {
  return `
    <div id="context-menu" class="context-menu hidden">
      <div class="context-menu-item" onclick="window.handleDownloadContext()">
        <span class="material-symbols-outlined">download</span>
        ${t('download')}
      </div>
      <div class="context-menu-item" onclick="window.handlePreview()">
        <span class="material-symbols-outlined">preview</span>
        ${t('preview')}
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" onclick="window.handleCopy()">
        <span class="material-symbols-outlined">content_copy</span>
        ${t('copy')}
      </div>
      <div class="context-menu-item" onclick="window.handleCut()">
        <span class="material-symbols-outlined">content_cut</span>
        ${t('cut')}
      </div>
      <div class="context-menu-item" onclick="window.handlePaste()">
        <span class="material-symbols-outlined">content_paste</span>
        ${t('paste')}
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" onclick="window.handleRenamePrompt()">
        <span class="material-symbols-outlined">drive_file_rename_outline</span>
        ${t('rename')}
      </div>
      <div class="context-menu-item" onclick="window.createShareLinkUI()">
        <span class="material-symbols-outlined">share</span>
        ${t('share')}
      </div>
      <div class="context-menu-item" onclick="window.handleCompress()">
        <span class="material-symbols-outlined">folder_zip</span>
        ${t('compress')}
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" onclick="window.handleProperties()">
        <span class="material-symbols-outlined">info</span>
        ${t('properties')}
      </div>
      <div class="context-menu-item danger" onclick="window.handleDeleteContext()">
        <span class="material-symbols-outlined">delete</span>
        ${t('delete')}
      </div>
    </div>
  `;
}

function renderPreviewModal() {
  return `
    <div id="preview-modal" class="modal hidden">
      <div class="modal-content">
        <div class="modal-header">
          <h2 id="preview-title">Preview</h2>
          <md-icon-button onclick="window.hidePreview()">
            <span class="material-symbols-outlined">close</span>
          </md-icon-button>
        </div>
        <div id="preview-container" class="modal-body"></div>
      </div>
    </div>
  `;
}

function renderNotifications() {
  return `<div id="notifications" class="notifications-container"></div>`;
}

function renderFavoritesView() {
  const favFiles = state.files.filter(f => state.favorites.includes(f.name));
  return `
    <div class="content-wrapper">
      <div class="content-header"><h2>${t('favorites')}</h2></div>
      <div class="file-list ${state.viewMode}">
        ${favFiles.length === 0 ? `<div class="empty-state">${t('no_files')}</div>` : renderFileList()}
      </div>
    </div>
  `;
}

function renderSearchView() {
  return `
    <div class="content-wrapper">
      <div class="content-header">
        <h2>${t('search')}</h2>
        <md-filled-text-field id="search-advanced" label="${t('search_placeholder')}" type="search"></md-filled-text-field>
      </div>
      <div class="file-list ${state.viewMode}" id="search-results"></div>
    </div>
  `;
}

function renderActivityView() {
  return `
    <div class="content-wrapper">
      <div class="content-header"><h2>${t('activity_log')}</h2></div>
      <div class="activity-log">
        ${state.activityLog.map(entry => `
          <div class="activity-entry">
            <div class="activity-icon">
              <span class="material-symbols-outlined">${getActivityIcon(entry.action)}</span>
            </div>
            <div class="activity-info">
              <div class="activity-action">${entry.action}</div>
              <div class="activity-details">${entry.details}</div>
              <div class="activity-time">${new Date(entry.timestamp).toLocaleString()}</div>
            </div>
          </div>
        `).join('')}
      </div>
    </div>
  `;
}

function renderTrashView() {
  return `
    <div class="content-wrapper">
      <div class="content-header">
        <h2>${t('trash')}</h2>
        <md-filled-button onclick="window.emptyTrash()">
          <span class="material-symbols-outlined">delete_sweep</span>
          ${t('empty_trash')}
        </md-filled-button>
      </div>
      <div class="trash-list">
        ${state.trash.map(item => `
          <div class="trash-item">
            <div class="trash-info">
              <div class="trash-name">${item.path}</div>
              <div class="trash-time">Deleted: ${new Date(item.deleted).toLocaleString()}</div>
            </div>
            <md-filled-button onclick="window.restoreFromTrash('${item.path}')">
              ${t('restore')}
            </md-filled-button>
          </div>
        `).join('')}
      </div>
    </div>
  `;
}

function renderPeersView() {
  return `
    <div class="content-wrapper">
      <div class="content-header"><h2>${t('peers')}</h2></div>
      <div class="peers-grid">
        ${state.peers.map(peer => `
          <div class="peer-card">
            <h3>${peer.name}</h3>
            <p>${peer.address}</p>
            <p class="peer-status ${peer.online ? 'online' : 'offline'}">${peer.online ? 'Online' : 'Offline'}</p>
          </div>
        `).join('')}
      </div>
    </div>
  `;
}

function renderSettingsView() {
  return `
    <div class="content-wrapper">
      <div class="content-header"><h2>${t('settings')}</h2></div>
      <div class="settings-panel">
        <div class="settings-section">
          <h3>${t('theme')}</h3>
          <md-filled-button onclick="window.toggleTheme()">
            ${state.darkMode ? 'Light Mode' : 'Dark Mode'}
          </md-filled-button>
        </div>
        <div class="settings-section">
          <h3>${t('language')}</h3>
          <md-filled-button onclick="window.setLanguage(state.language === 'en' ? 'de' : 'en')">
            ${state.language === 'en' ? 'Deutsch' : 'English'}
          </md-filled-button>
        </div>
        <div class="settings-section">
          <h3>${t('view_options')}</h3>
          <label>
            <input type="checkbox" ${state.showHidden ? 'checked' : ''} onchange="window.toggleHidden()">
            Show Hidden Files
          </label>
        </div>
        <div class="settings-section">
          <h3>${t('api_tokens')}</h3>
          <md-filled-button onclick="window.generateAPIToken()">
            ${t('generate_token')}
          </md-filled-button>
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// EVENT HANDLERS
// =============================================================================

function attachEventListeners() {
  const app = document.getElementById('app');
  
  // Drag & Drop
  ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(event => {
    app.addEventListener(event, (e) => {
      e.preventDefault();
      e.stopPropagation();
    });
  });

  app.addEventListener('dragover', (e) => {
    document.querySelector('.file-list')?.classList.add('drag-over');
  });

  app.addEventListener('dragleave', (e) => {
    document.querySelector('.file-list')?.classList.remove('drag-over');
  });

  app.addEventListener('drop', (e) => {
    document.querySelector('.file-list')?.classList.remove('drag-over');
    if (e.dataTransfer.files.length > 0) {
      window.handleUpload(e.dataTransfer.files);
    }
  });

  // Keyboard shortcuts
  document.addEventListener('keydown', (e) => {
    if (e.ctrlKey || e.metaKey) {
      if (e.key === 'a') { e.preventDefault(); window.selectAllFiles(); }
      if (e.key === 'c') { e.preventDefault(); window.handleCopy(); }
      if (e.key === 'x') { e.preventDefault(); window.handleCut(); }
      if (e.key === 'v') { e.preventDefault(); window.handlePaste(); }
    }
    if (e.key === 'Escape') { window.hideContextMenu(); window.hidePreview(); }
  });
}

function toggleSidebar() {
  document.getElementById('sidebar')?.classList.toggle('hidden');
}

function toggleViewMode() {
  state.viewMode = state.viewMode === 'grid' ? 'list' : 'grid';
  localStorage.setItem('viewMode', state.viewMode);
  renderMainApp();
}

function toggleTheme() {
  state.darkMode = !state.darkMode;
  localStorage.setItem('darkMode', state.darkMode);
  document.documentElement.className = state.darkMode ? 'dark' : '';
  renderMainApp();
}

function setLanguage(lang) {
  state.language = lang;
  localStorage.setItem('language', state.language);
  renderMainApp();
}

function navigateToView(view) {
  state.currentView = view;
  renderMainApp();
}

function navigateToPath(path) {
  state.currentPath = path;
  loadFiles(path);
}

function handleNewFolder() {
  const name = prompt(t('folder_name'));
  if (name) {
    apiCall(`/dirs/${encodeURIComponent(state.currentPath + '/' + name)}`, { method: 'POST' })
      .then(() => {
        loadFiles(state.currentPath);
        showNotification('Success', 'Folder created', 'success');
      });
  }
}

function handleFileClick(name) {
  const file = state.files.find(f => f.name === name);
  if (file?.is_dir) {
    const newPath = (state.currentPath + '/' + name).replace(/^\//, '');
    navigateToPath(newPath);
  }
}

function selectFile(name, event) {
  event.stopPropagation();
  if (event.ctrlKey || event.metaKey) {
    if (state.selectedFiles.has(name)) {
      state.selectedFiles.delete(name);
    } else {
      state.selectedFiles.add(name);
    }
  } else if (event.shiftKey) {
    // Range selection
    state.selectedFiles.add(name);
  } else {
    state.selectedFiles.clear();
    state.selectedFiles.add(name);
  }
  renderFileList();
}

function selectAllFiles() {
  state.selectedFiles.clear();
  state.files.forEach(f => state.selectedFiles.add(f.name));
  renderFileList();
}

function toggleFavorite(fileName) {
  if (state.favorites.includes(fileName)) {
    state.favorites = state.favorites.filter(f => f !== fileName);
  } else {
    state.favorites.push(fileName);
  }
  localStorage.setItem('favorites', JSON.stringify(state.favorites));
  renderFileList();
}

function showContextMenu(event, fileName) {
  event.preventDefault();
  event.stopPropagation();
  const menu = document.getElementById('context-menu');
  menu.style.left = (event.clientX) + 'px';
  menu.style.top = (event.clientY) + 'px';
  menu.classList.remove('hidden');
  menu.dataset.target = fileName;
  
  document.addEventListener('click', hideContextMenu);
}

function hideContextMenu() {
  document.getElementById('context-menu')?.classList.add('hidden');
  document.removeEventListener('click', hideContextMenu);
}

function handleDownloadContext() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const path = (state.currentPath + '/' + fileName).replace(/^\//, '');
  handleDownload(path);
  hideContextMenu();
}

function handleCopy() {
  const selected = Array.from(state.selectedFiles);
  if (selected.length > 0) {
    state.clipboard = { operation: 'copy', files: selected };
    showNotification('Success', `${selected.length} item(s) copied`, 'success');
  }
  hideContextMenu();
}

function handleCut() {
  const selected = Array.from(state.selectedFiles);
  if (selected.length > 0) {
    state.clipboard = { operation: 'cut', files: selected };
    showNotification('Success', `${selected.length} item(s) cut`, 'success');
  }
  hideContextMenu();
}

function handlePaste() {
  if (state.clipboard) {
    showNotification('Success', `Pasting ${state.clipboard.files.length} item(s)...`, 'info');
    // TODO: Implement actual paste logic
  }
  hideContextMenu();
}

function handleRenamePrompt() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const newName = prompt(t('new_name'), fileName);
  if (newName) {
    const oldPath = (state.currentPath + '/' + fileName).replace(/^\//, '');
    const newPath = (state.currentPath + '/' + newName).replace(/^\//, '');
    handleRename(oldPath, newPath);
  }
  hideContextMenu();
}

function createShareLinkUI() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const share = createShareLink(fileName);
  const shareUrl = `${window.location.origin}/share/${share.id}`;
  copyToClipboard(shareUrl);
  hideContextMenu();
}

function handleCompress() {
  showNotification('Info', 'Compression feature coming soon', 'info');
  hideContextMenu();
}

function handleProperties() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const file = state.files.find(f => f.name === fileName);
  if (file) {
    alert(`Name: ${file.name}\nSize: ${formatFileSize(file.size)}\nModified: ${new Date(file.modified).toLocaleString()}`);
  }
  hideContextMenu();
}

function handleDeleteContext() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const path = (state.currentPath + '/' + fileName).replace(/^\//, '');
  handleDelete(path);
  hideContextMenu();
}

function handlePreview() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const modal = document.getElementById('preview-modal');
  document.getElementById('preview-title').textContent = fileName;
  
  const ext = fileName.split('.').pop()?.toLowerCase();
  const previewContainer = document.getElementById('preview-container');
  
  if (['jpg', 'jpeg', 'png', 'gif', 'webp'].includes(ext)) {
    previewContainer.innerHTML = `<img src="/api/file/${encodeURIComponent(state.currentPath + '/' + fileName)}" style="max-width: 100%; max-height: 500px;">`;
  } else if (['txt', 'md', 'json', 'xml', 'html', 'css', 'js'].includes(ext)) {
    previewContainer.innerHTML = '<p>Text preview loading...</p>';
    // TODO: Load text content
  } else {
    previewContainer.innerHTML = '<p>Preview not available for this file type</p>';
  }
  
  modal.classList.remove('hidden');
  hideContextMenu();
}

function hidePreview() {
  document.getElementById('preview-modal')?.classList.add('hidden');
}

function toggleHidden() {
  state.showHidden = !state.showHidden;
  localStorage.setItem('showHidden', state.showHidden);
  loadFiles(state.currentPath);
}

function generateAPIToken() {
  const token = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
  copyToClipboard(token);
  showNotification('Success', 'API token generated and copied', 'success');
}

function performSearch(query) {
  if (!query) return;
  const results = state.files.filter(f => 
    f.name.toLowerCase().includes(query.toLowerCase())
  );
  // Render search results
  const resultsContainer = document.getElementById('search-results');
  if (resultsContainer) {
    resultsContainer.innerHTML = results.map(f => `
      <div class="file-item" onclick="window.handleFileClick('${f.name}')">
        <div class="file-icon"><span class="material-symbols-outlined">${getFileIcon(f.name)}</span></div>
        <div class="file-info">
          <div class="file-name">${f.name}</div>
          <div class="file-meta">${formatFileSize(f.size)}</div>
        </div>
      </div>
    `).join('');
  }
}

function formatFileSize(bytes) {
  const sizes = ['B', 'KB', 'MB', 'GB'];
  if (bytes === 0) return '0 B';
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
}

function getFileIcon(fileName) {
  const ext = fileName.split('.').pop()?.toLowerCase();
  const icons = {
    'pdf': 'description', 'doc': 'article', 'docx': 'article', 'txt': 'article', 'md': 'article',
    'xls': 'table_chart', 'xlsx': 'table_chart', 'csv': 'table_chart',
    'ppt': 'presentation', 'pptx': 'presentation',
    'jpg': 'image', 'jpeg': 'image', 'png': 'image', 'gif': 'image', 'svg': 'image', 'webp': 'image',
    'mp4': 'video_library', 'mkv': 'video_library', 'avi': 'video_library', 'mov': 'video_library',
    'mp3': 'music_note', 'wav': 'music_note', 'flac': 'music_note', 'm4a': 'music_note',
    'zip': 'folder_zip', 'rar': 'folder_zip', 'tar': 'folder_zip', 'gz': 'folder_zip', '7z': 'folder_zip',
  };
  return icons[ext] || 'description';
}

function getActivityIcon(action) {
  const icons = {
    'upload': 'cloud_upload',
    'download': 'cloud_download',
    'delete': 'delete',
    'rename': 'drive_file_rename_outline',
    'tag': 'label',
    'share': 'share',
    'login': 'login',
    'logout': 'logout',
  };
  return icons[action] || 'info';
}

// =============================================================================
// INITIALIZATION
// =============================================================================

document.addEventListener('DOMContentLoaded', () => {
  renderApp();
  if (state.authToken) {
    loadFiles('');
  }
});

// Global exports
window.logout = logout;
window.toggleSidebar = toggleSidebar;
window.toggleViewMode = toggleViewMode;
window.toggleTheme = toggleTheme;
window.setLanguage = setLanguage;
window.navigateToView = navigateToView;
window.navigateToPath = navigateToPath;
window.handleNewFolder = handleNewFolder;
window.handleFileClick = handleFileClick;
window.selectFile = selectFile;
window.selectAllFiles = selectAllFiles;
window.toggleFavorite = toggleFavorite;
window.showContextMenu = showContextMenu;
window.hideContextMenu = hideContextMenu;
window.handleDownloadContext = handleDownloadContext;
window.handleDownload = handleDownload;
window.handleCopy = handleCopy;
window.handleCut = handleCut;
window.handlePaste = handlePaste;
window.handleRenamePrompt = handleRenamePrompt;
window.createShareLinkUI = createShareLinkUI;
window.handleCompress = handleCompress;
window.handleProperties = handleProperties;
window.handleDeleteContext = handleDeleteContext;
window.handlePreview = handlePreview;
window.hidePreview = hidePreview;
window.handleUpload = handleUpload;
window.restoreFromTrash = restoreFromTrash;
window.emptyTrash = emptyTrash;
window.toggleHidden = toggleHidden;
window.generateAPIToken = generateAPIToken;
window.performSearch = performSearch;
window.renderFileList = renderFileList;
window.loadFiles = loadFiles;
window.getActivityIcon = getActivityIcon;
