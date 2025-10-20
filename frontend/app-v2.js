/**
 * SYNCSPACE v2 - MODERN FILE MANAGEMENT APPLICATION
 * Features: 24+ innovative file management capabilities
 * Architecture: Performance-optimized SPA with Material 3 Expressive Design
 */

// =============================================================================
// GLOBAL STATE & CONFIGURATION
// =============================================================================

const API_BASE = '/api';
let state = {
  authToken: localStorage.getItem('authToken') || '',
  currentPath: '',
  currentView: 'files', // files, search, peers, settings, trash, favorites, activity
  viewMode: localStorage.getItem('viewMode') || 'grid',
  darkMode: localStorage.getItem('darkMode') === 'true',
  language: localStorage.getItem('language') || 'en',
  selectedFiles: new Set(),
  favorites: JSON.parse(localStorage.getItem('favorites') || '[]'),
  clipboard: null,
  files: [],
  peers: [],
  searchQuery: '',
  sortBy: localStorage.getItem('sortBy') || 'name',
  sortOrder: localStorage.getItem('sortOrder') || 'asc',
};

// =============================================================================
// INTERNATIONALIZATION (i18n)
// =============================================================================

const translations = {
  en: {
    app_title: 'SyncSpace',
    app_subtitle: 'File Synchronization & Management',
    sign_in: 'Sign In',
    username: 'Username',
    password: 'Password',
    twofa_code: '2FA Code (optional)',
    sign_in_button: 'Sign In',
    no_account: "Don't have an account?",
    register: 'Register',
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
    toggle_view: 'Toggle View',
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
    properties: 'Properties',
    preview: 'Preview',
    compress: 'Compress',
    extract: 'Extract',
    open_with: 'Open With',
    move: 'Move',
    search_placeholder: 'Search files and folders...',
    no_results: 'No files found',
    folder_name: 'Folder name',
    new_name: 'New name',
    confirm_delete: 'Are you sure? This action cannot be undone.',
    items_selected: 'items selected',
    theme: 'Theme',
    language: 'Language',
    storage_usage: 'Storage Usage',
    empty_trash: 'Empty Trash',
    restore: 'Restore',
    delete_forever: 'Delete Forever',
    keyboard_shortcuts: 'Keyboard Shortcuts',
    activity_log: 'Activity Log',
    quick_access: 'Quick Access',
    recent_files: 'Recent Files',
    size: 'Size',
    modified: 'Modified',
    created: 'Created',
    type: 'Type',
    status: 'Status',
    view_options: 'View Options',
    sort_by: 'Sort by',
    ascending: 'Ascending',
    descending: 'Descending',
    grid_view: 'Grid View',
    list_view: 'List View',
  },
  de: {
    app_title: 'SyncSpace',
    app_subtitle: 'Dateisynchronisation & Verwaltung',
    sign_in: 'Anmelden',
    username: 'Benutzername',
    password: 'Passwort',
    twofa_code: '2FA Code (optional)',
    sign_in_button: 'Anmelden',
    no_account: 'Noch kein Konto?',
    register: 'Registrieren',
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
    toggle_view: 'Ansicht umschalten',
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
    properties: 'Eigenschaften',
    preview: 'Vorschau',
    compress: 'Komprimieren',
    extract: 'Extrahieren',
    open_with: 'Öffnen mit',
    move: 'Verschieben',
    search_placeholder: 'Dateien und Ordner durchsuchen...',
    no_results: 'Keine Dateien gefunden',
    folder_name: 'Ordnername',
    new_name: 'Neuer Name',
    confirm_delete: 'Sind Sie sicher? Diese Aktion kann nicht rückgängig gemacht werden.',
    items_selected: 'Elemente ausgewählt',
    theme: 'Design',
    language: 'Sprache',
    storage_usage: 'Speichernutzung',
    empty_trash: 'Papierkorb leeren',
    restore: 'Wiederherstellen',
    delete_forever: 'Endgültig löschen',
    keyboard_shortcuts: 'Tastenkombinationen',
    activity_log: 'Aktivitätsprotokoll',
    quick_access: 'Schnellzugriff',
    recent_files: 'Letzte Dateien',
    size: 'Größe',
    modified: 'Geändert',
    created: 'Erstellt',
    type: 'Typ',
    status: 'Status',
    view_options: 'Anzeigeoptionen',
    sort_by: 'Sortieren nach',
    ascending: 'Aufsteigend',
    descending: 'Absteigend',
    grid_view: 'Rasteransicht',
    list_view: 'Listenansicht',
  }
};

function t(key) {
  return translations[state.language][key] || translations.en[key] || key;
}

// =============================================================================
// API UTILITIES
// =============================================================================

async function apiCall(endpoint, options = {}) {
  const url = `${API_BASE}${endpoint}`;
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };
  if (state.authToken) headers.Authorization = `Bearer ${state.authToken}`;

  try {
    const response = await fetch(url, { ...options, headers });
    if (response.status === 401) {
      logout();
      return null;
    }
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return await response.json();
  } catch (error) {
    console.error(`API Error [${endpoint}]:`, error);
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
    return true;
  }
  return false;
}

function logout() {
  state.authToken = '';
  state.authToken = '';
  localStorage.removeItem('authToken');
  localStorage.removeItem('favorites');
  state.selectedFiles.clear();
  state.clipboard = null;
  renderApp();
}

// =============================================================================
// FILE OPERATIONS
// =============================================================================

async function loadFiles(path = '') {
  state.currentPath = path;
  const response = await apiCall(`/files/${encodeURIComponent(path)}`);
  if (response) {
    state.files = response.sort((a, b) => {
      let aVal = a[state.sortBy] || '';
      let bVal = b[state.sortBy] || '';
      if (typeof aVal === 'string') aVal = aVal.toLowerCase();
      if (typeof bVal === 'string') bVal = bVal.toLowerCase();
      const result = aVal > bVal ? 1 : -1;
      return state.sortOrder === 'asc' ? result : -result;
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
      const response = await fetch(`${API_BASE}/upload/${encodeURIComponent(path)}`, {
        method: 'POST',
        headers: { Authorization: `Bearer ${state.authToken}` },
        body: formData,
      });
      if (response.ok) {
        await loadFiles(state.currentPath);
      }
    } catch (error) {
      console.error('Upload failed:', error);
    }
  }
}

async function handleDelete(path) {
  if (!confirm(t('confirm_delete'))) return;
  const response = await apiCall(`/files/${encodeURIComponent(path)}`, { method: 'DELETE' });
  if (response) {
    await loadFiles(state.currentPath);
  }
}

async function handleRename(oldPath, newName) {
  const response = await apiCall(`/rename/${encodeURIComponent(oldPath)}`, {
    method: 'PUT',
    body: JSON.stringify({ new_path: newName }),
  });
  if (response) {
    await loadFiles(state.currentPath);
  }
}

async function handleDownload(path) {
  const url = `${API_BASE}/file/${encodeURIComponent(path)}`;
  window.open(url, '_blank');
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
      alert('Login failed!');
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
    ${renderActionMenu()}
  `;

  attachEventListeners();
}

function renderHeader() {
  return `
    <header class="app-header">
      <div class="header-left">
        <md-icon-button onclick="toggleSidebar()">
          <span class="material-symbols-outlined">menu</span>
        </md-icon-button>
        <h1 class="app-title">${t('app_title')}</h1>
      </div>
      <div class="header-right">
        <md-filled-text-field id="search-input" label="${t('search_placeholder')}" type="search" onkeyup="performSearch()"></md-filled-text-field>
        <md-icon-button onclick="toggleViewMode()">
          <span class="material-symbols-outlined">${state.viewMode === 'grid' ? 'view_list' : 'grid_view'}</span>
        </md-icon-button>
        <md-icon-button onclick="setLanguage(state.language === 'en' ? 'de' : 'en')">
          <span class="material-symbols-outlined">language</span>
        </md-icon-button>
        <md-icon-button onclick="toggleTheme()">
          <span class="material-symbols-outlined">${state.darkMode ? 'light_mode' : 'dark_mode'}</span>
        </md-icon-button>
        <md-icon-button onclick="logout()">
          <span class="material-symbols-outlined">logout</span>
        </md-icon-button>
      </div>
    </header>
  `;
}

function renderSidebar() {
  const views = ['files', 'favorites', 'search', 'activity', 'trash', 'peers', 'settings'];
  const icons = {
    files: 'folder',
    favorites: 'star',
    search: 'search',
    activity: 'history',
    trash: 'delete',
    peers: 'devices',
    settings: 'settings',
  };

  return `
    <aside class="sidebar">
      <div class="sidebar-section">
        <h3>${t('quick_access')}</h3>
        ${views.map(view => `
          <div class="sidebar-item ${state.currentView === view ? 'active' : ''}" onclick="navigateToView('${view}')">
            <span class="material-symbols-outlined">${icons[view]}</span>
            <span>${t(view)}</span>
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
    <div class="content-header">
      <h2>${state.currentPath || t('files')}</h2>
      <div class="toolbar">
        <md-filled-button onclick="document.getElementById('file-input').click()">
          <span class="material-symbols-outlined">upload</span>
          ${t('upload_files')}
        </md-filled-button>
        <md-filled-button onclick="handleNewFolder()">
          <span class="material-symbols-outlined">folder_open</span>
          ${t('new_folder')}
        </md-filled-button>
        <input type="file" id="file-input" multiple style="display:none" onchange="handleUpload(this.files)">
      </div>
    </div>
    <div class="file-list ${state.viewMode}">
      ${renderFileList()}
    </div>
  `;
}

function renderFileList() {
  if (state.files.length === 0) {
    return `<div class="empty-state">${t('no_files')}</div>`;
  }

  return state.files.map(file => {
    const icon = file.is_dir ? 'folder' : getFileIcon(file.name);
    return `
      <div class="file-item" oncontextmenu="showContextMenu(event, '${file.name}')" ondblclick="handleFileClick('${file.name}')">
        <div class="file-icon">
          <span class="material-symbols-outlined">${icon}</span>
        </div>
        <div class="file-info">
          <div class="file-name">${file.name}</div>
          <div class="file-meta">${file.size_str} • ${file.modified}</div>
        </div>
        <div class="file-actions">
          <md-icon-button onclick="toggleFavorite('${file.name}')">
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
      <div class="context-menu-item" onclick="handleDownload()">
        <span class="material-symbols-outlined">download</span>
        ${t('download')}
      </div>
      <div class="context-menu-item" onclick="handlePreview()">
        <span class="material-symbols-outlined">preview</span>
        ${t('preview')}
      </div>
      <div class="context-menu-item" onclick="handleCopy()">
        <span class="material-symbols-outlined">content_copy</span>
        ${t('copy')}
      </div>
      <div class="context-menu-item" onclick="handleCut()">
        <span class="material-symbols-outlined">content_cut</span>
        ${t('cut')}
      </div>
      <div class="context-menu-item" onclick="handlePaste()">
        <span class="material-symbols-outlined">content_paste</span>
        ${t('paste')}
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" onclick="handleRenamePrompt()">
        <span class="material-symbols-outlined">drive_file_rename_outline</span>
        ${t('rename')}
      </div>
      <div class="context-menu-item" onclick="handleShare()">
        <span class="material-symbols-outlined">share</span>
        ${t('share')}
      </div>
      <div class="context-menu-item danger" onclick="handleDelete()">
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
          <h2>Preview</h2>
          <md-icon-button onclick="hidePreview()">
            <span class="material-symbols-outlined">close</span>
          </md-icon-button>
        </div>
        <div id="preview-container" class="modal-body"></div>
      </div>
    </div>
  `;
}

function renderActionMenu() {
  return `<div id="action-menu" class="action-menu hidden"></div>`;
}

function renderFavoritesView() {
  return `<div class="content-header"><h2>${t('favorites')}</h2></div>`;
}

function renderSearchView() {
  return `<div class="content-header"><h2>${t('search')}</h2></div>`;
}

function renderActivityView() {
  return `<div class="content-header"><h2>${t('activity')}</h2></div>`;
}

function renderTrashView() {
  return `<div class="content-header"><h2>${t('trash')}</h2></div>`;
}

function renderPeersView() {
  return `<div class="content-header"><h2>${t('peers')}</h2></div>`;
}

function renderSettingsView() {
  return `<div class="content-header"><h2>${t('settings')}</h2></div>`;
}

// =============================================================================
// EVENT HANDLERS
// =============================================================================

function attachEventListeners() {
  // Drag & drop
  const app = document.getElementById('app');
  app.addEventListener('dragover', (e) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  });

  app.addEventListener('drop', (e) => {
    e.preventDefault();
    if (e.dataTransfer.files.length > 0) {
      handleUpload(e.dataTransfer.files);
    }
  });

  // Keyboard shortcuts
  document.addEventListener('keydown', (e) => {
    if (e.ctrlKey || e.metaKey) {
      if (e.key === 'a') { e.preventDefault(); selectAllFiles(); }
      if (e.key === 'c') { e.preventDefault(); handleCopy(); }
      if (e.key === 'x') { e.preventDefault(); handleCut(); }
      if (e.key === 'v') { e.preventDefault(); handlePaste(); }
    }
  });
}

function toggleSidebar() {
  const sidebar = document.querySelector('.sidebar');
  sidebar?.classList.toggle('hidden');
}

function toggleViewMode() {
  state.viewMode = state.viewMode === 'grid' ? 'list' : 'grid';
  localStorage.setItem('viewMode', state.viewMode);
  renderMainApp();
}

function toggleTheme() {
  state.darkMode = !state.darkMode;
  localStorage.setItem('darkMode', state.darkMode);
  document.documentElement.style.setProperty('--theme-mode', state.darkMode ? 'dark' : 'light');
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

function handleNewFolder() {
  const name = prompt(t('folder_name'));
  if (name) {
    apiCall(`/dirs/${encodeURIComponent(state.currentPath + '/' + name)}`, { method: 'POST' })
      .then(() => loadFiles(state.currentPath));
  }
}

function handleFileClick(name) {
  const file = state.files.find(f => f.name === name);
  if (file?.is_dir) {
    loadFiles((state.currentPath + '/' + name).replace(/^\//, ''));
  }
}

function toggleFavorite(fileName) {
  const fullPath = (state.currentPath + '/' + fileName).replace(/^\//, '');
  if (state.favorites.includes(fullPath)) {
    state.favorites = state.favorites.filter(f => f !== fullPath);
  } else {
    state.favorites.push(fullPath);
  }
  localStorage.setItem('favorites', JSON.stringify(state.favorites));
  renderFileList();
}

function showContextMenu(event, fileName) {
  event.preventDefault();
  const menu = document.getElementById('context-menu');
  menu.style.left = event.clientX + 'px';
  menu.style.top = event.clientY + 'px';
  menu.classList.remove('hidden');
  menu.dataset.target = fileName;
  document.addEventListener('click', hideContextMenu);
}

function hideContextMenu() {
  document.getElementById('context-menu')?.classList.add('hidden');
  document.removeEventListener('click', hideContextMenu);
}

function handleDownload() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const path = (state.currentPath + '/' + fileName).replace(/^\//, '');
  handleDownload(path);
  hideContextMenu();
}

function handleCopy() {
  const fileName = document.getElementById('context-menu').dataset.target;
  state.clipboard = { operation: 'copy', files: [fileName] };
  hideContextMenu();
}

function handleCut() {
  const fileName = document.getElementById('context-menu').dataset.target;
  state.clipboard = { operation: 'cut', files: [fileName] };
  hideContextMenu();
}

function handlePaste() {
  // TODO: Implement paste
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

function handleShare() {
  // TODO: Implement share link generation
  hideContextMenu();
}

function handleDelete() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const path = (state.currentPath + '/' + fileName).replace(/^\//, '');
  handleDelete(path);
  hideContextMenu();
}

function handlePreview() {
  const fileName = document.getElementById('context-menu').dataset.target;
  const modal = document.getElementById('preview-modal');
  modal.classList.remove('hidden');
  // TODO: Load preview content
  hideContextMenu();
}

function hidePreview() {
  document.getElementById('preview-modal')?.classList.add('hidden');
}

function selectAllFiles() {
  state.selectedFiles = new Set(state.files.map(f => f.name));
  renderFileList();
}

function performSearch() {
  const query = document.getElementById('search-input')?.value || '';
  state.searchQuery = query;
  // TODO: Implement search
}

function getFileIcon(fileName) {
  const ext = fileName.split('.').pop()?.toLowerCase();
  const icons = {
    'pdf': 'description',
    'doc': 'article', 'docx': 'article', 'txt': 'article',
    'xls': 'table_chart', 'xlsx': 'table_chart', 'csv': 'table_chart',
    'ppt': 'presentation', 'pptx': 'presentation',
    'jpg': 'image', 'jpeg': 'image', 'png': 'image', 'gif': 'image', 'svg': 'image',
    'mp4': 'video_library', 'mkv': 'video_library', 'avi': 'video_library',
    'mp3': 'music_note', 'wav': 'music_note', 'flac': 'music_note',
    'zip': 'folder_zip', 'rar': 'folder_zip', 'tar': 'folder_zip', 'gz': 'folder_zip',
  };
  return icons[ext] || 'description';
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

// Global exports for HTML onclick handlers
window.toggleSidebar = toggleSidebar;
window.toggleViewMode = toggleViewMode;
window.toggleTheme = toggleTheme;
window.setLanguage = setLanguage;
window.navigateToView = navigateToView;
window.handleNewFolder = handleNewFolder;
window.handleFileClick = handleFileClick;
window.toggleFavorite = toggleFavorite;
window.showContextMenu = showContextMenu;
window.hideContextMenu = hideContextMenu;
window.handleDownload = handleDownload;
window.handleCopy = handleCopy;
window.handleCut = handleCut;
window.handlePaste = handlePaste;
window.handleRenamePrompt = handleRenamePrompt;
window.handleShare = handleShare;
window.handleDelete = handleDelete;
window.handlePreview = handlePreview;
window.hidePreview = hidePreview;
window.selectAllFiles = selectAllFiles;
window.performSearch = performSearch;
window.logout = logout;
window.handleUpload = handleUpload;
window.getFileIcon = getFileIcon;
