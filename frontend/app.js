/**
 * SYNCSPACE - ADVANCED FILE MANAGEMENT APPLICATION
 * Features: 20+ innovative file management capabilities
 * Architecture: SPA with Material 3 Design, i18n, JWT Auth
 */

// =============================================================================
// GLOBAL STATE & CONFIGURATION
// =============================================================================

const API_BASE = '/api';
let authToken = localStorage.getItem('authToken') || '';
let currentPath = '';
let currentView = 'files'; // files, search, peers, settings, trash, favorites
let viewMode = localStorage.getItem('viewMode') || 'grid'; // grid or list
let darkMode = localStorage.getItem('darkMode') === 'true';
let currentLanguage = localStorage.getItem('language') || 'en';
let selectedFiles = new Set();
let multiSelectMode = false;
let favorites = JSON.parse(localStorage.getItem('favorites') || '[]');
let clipboard = null; // {operation: 'cut'|'copy', files: []}

// =============================================================================
// INTERNATIONALIZATION (i18n)
// =============================================================================

const translations = {
  en: {
    app_title: 'SyncSpace',
    loading: 'Loading...',
    login: 'Sign In',
    username: 'Username',
    password: 'Password',
    twofa_code: '2FA Code (optional)',
    signin: 'Sign In',
    upload: 'Upload',
    new_folder: 'New Folder',
    files: 'Files',
    search: 'Search',
    peers: 'Peers',
    settings: 'Settings',
    trash: 'Trash',
    favorites: 'Favorites',
    logout: 'Logout',
    home: 'Home',
    toggle_theme: 'Toggle Theme',
    toggle_menu: 'Toggle Menu',
    toggle_view: 'Toggle View',
    multiselect: 'Multi-select',
    drag_drop_here: 'Drag & drop files or folders here',
    or_click_upload: 'or click to select files',
    no_files: 'No files in this folder',
    rename: 'Rename',
    delete: 'Delete',
    download: 'Download',
    copy: 'Copy',
    cut: 'Cut',
    paste: 'Paste',
    share: 'Share',
    favorite: 'Add to Favorites',
    unfavorite: 'Remove from Favorites',
    properties: 'Properties',
    preview: 'Preview',
    compress: 'Compress',
    extract: 'Extract',
    new_share_link: 'Create Share Link',
    search_placeholder: 'Search files...',
    no_results: 'No results found',
    folder_name: 'Folder name',
    new_name: 'New name',
    confirm_delete: 'Are you sure you want to delete',
    items_selected: 'items selected',
    theme: 'Theme',
    language: 'Language',
    enable_2fa: 'Enable 2FA',
    change_password: 'Change Password',
    old_password: 'Old Password',
    new_password: 'New Password',
    save: 'Save',
    cancel: 'Cancel',
    close: 'Close',
    empty_trash: 'Empty Trash',
    restore: 'Restore',
    delete_forever: 'Delete Forever',
    show_hidden: 'Show Hidden Files',
    keyboard_shortcuts: 'Keyboard Shortcuts',
    activity_log: 'Activity Log',
    storage_usage: 'Storage Usage',
    file_versions: 'File Versions',
  },
  de: {
    app_title: 'SyncSpace',
    loading: 'Laden...',
    login: 'Anmelden',
    username: 'Benutzername',
    password: 'Passwort',
    twofa_code: '2FA Code (optional)',
    signin: 'Anmelden',
    upload: 'Hochladen',
    new_folder: 'Neuer Ordner',
    files: 'Dateien',
    search: 'Suchen',
    peers: 'Peers',
    settings: 'Einstellungen',
    trash: 'Papierkorb',
    favorites: 'Favoriten',
    logout: 'Abmelden',
    home: 'Start',
    toggle_theme: 'Theme wechseln',
    toggle_menu: 'Menü umschalten',
    toggle_view: 'Ansicht wechseln',
    multiselect: 'Mehrfachauswahl',
    drag_drop_here: 'Dateien oder Ordner hierher ziehen',
    or_click_upload: 'oder klicken zum Auswählen',
    no_files: 'Keine Dateien in diesem Ordner',
    rename: 'Umbenennen',
    delete: 'Löschen',
    download: 'Herunterladen',
    copy: 'Kopieren',
    cut: 'Ausschneiden',
    paste: 'Einfügen',
    share: 'Teilen',
    favorite: 'Zu Favoriten',
    unfavorite: 'Aus Favoriten',
    properties: 'Eigenschaften',
    preview: 'Vorschau',
    compress: 'Komprimieren',
    extract: 'Extrahieren',
    new_share_link: 'Freigabelink erstellen',
    search_placeholder: 'Dateien suchen...',
    no_results: 'Keine Ergebnisse',
    folder_name: 'Ordnername',
    new_name: 'Neuer Name',
    confirm_delete: 'Möchten Sie wirklich löschen',
    items_selected: 'Elemente ausgewählt',
    theme: 'Aussehen',
    language: 'Sprache',
    enable_2fa: '2FA aktivieren',
    change_password: 'Passwort ändern',
    old_password: 'Altes Passwort',
    new_password: 'Neues Passwort',
    save: 'Speichern',
    cancel: 'Abbrechen',
    close: 'Schließen',
    empty_trash: 'Papierkorb leeren',
    restore: 'Wiederherstellen',
    delete_forever: 'Endgültig löschen',
    show_hidden: 'Versteckte Dateien',
    keyboard_shortcuts: 'Tastenkürzel',
    activity_log: 'Aktivitätsprotokoll',
    storage_usage: 'Speichernutzung',
    file_versions: 'Dateiversionen',
  }
};

function t(key) {
  return translations[currentLanguage][key] || key;
}

function setLanguage(lang) {
  currentLanguage = lang;
  localStorage.setItem('language', lang);
  renderCurrentView();
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

function formatSize(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

function formatDate(dateString) {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat(currentLanguage, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(date);
}

function getFileIcon(filename, isDir) {
  if (isDir) return 'folder';
  
  const ext = filename.split('.').pop().toLowerCase();
  const iconMap = {
    // Images
    jpg: 'image', jpeg: 'image', png: 'image', gif: 'image', svg: 'image', webp: 'image',
    // Videos
    mp4: 'movie', avi: 'movie', mkv: 'movie', mov: 'movie', webm: 'movie',
    // Audio
    mp3: 'audio_file', wav: 'audio_file', flac: 'audio_file', ogg: 'audio_file',
    // Documents
    pdf: 'picture_as_pdf', doc: 'description', docx: 'description', txt: 'description',
    md: 'description', rtf: 'description',
    // Spreadsheets
    xls: 'table_chart', xlsx: 'table_chart', csv: 'table_chart',
    // Presentations
    ppt: 'slideshow', pptx: 'slideshow',
    // Archives
    zip: 'folder_zip', rar: 'folder_zip', '7z': 'folder_zip', tar: 'folder_zip', gz: 'folder_zip',
    // Code
    js: 'code', ts: 'code', html: 'code', css: 'code', json: 'code', xml: 'code',
    py: 'code', java: 'code', cpp: 'code', rs: 'code', go: 'code', php: 'code',
  };
  
  return iconMap[ext] || 'insert_drive_file';
}

function canPreview(filename) {
  const ext = filename.split('.').pop().toLowerCase();
  const previewable = ['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp', 'mp4', 'webm', 'mp3', 'pdf', 'txt', 'md', 'json', 'html', 'css', 'js'];
  return previewable.includes(ext);
}

function isArchive(filename) {
  const ext = filename.split('.').pop().toLowerCase();
  return ['zip', 'rar', '7z', 'tar', 'gz'].includes(ext);
}

// =============================================================================
// API CALLS
// =============================================================================

async function apiCall(endpoint, options = {}) {
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers
  };
  
  if (authToken) {
    headers['Authorization'] = `Bearer ${authToken}`;
  }
  
  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers
  });
  
  if (response.status === 401) {
    localStorage.removeItem('authToken');
    location.reload();
  }
  
  return response;
}

async function login(username, password, twofa_code = '') {
  const response = await fetch(`${API_BASE}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password, twofa_code })
  });
  
  if (response.ok) {
    const data = await response.json();
    authToken = data.token;
    localStorage.setItem('authToken', authToken);
    return true;
  }
  return false;
}

async function loadFiles(path = '') {
  const response = await apiCall(`/files/${encodeURIComponent(path)}`);
  if (response.ok) {
    return await response.json();
  }
  return [];
}

async function uploadFile(file, path = '') {
  const formData = new FormData();
  formData.append('file', file);
  
  const response = await fetch(`${API_BASE}/upload/${encodeURIComponent(path)}${file.name}`, {
    method: 'POST',
    headers: { 'Authorization': `Bearer ${authToken}` },
    body: await file.arrayBuffer()
  });
  
  return response.ok;
}

async function deleteFile(path) {
  const response = await apiCall(`/files/${encodeURIComponent(path)}`, { method: 'DELETE' });
  return response.ok;
}

async function renameFile(oldPath, newPath) {
  const response = await apiCall(`/rename/${encodeURIComponent(oldPath)}`, {
    method: 'PUT',
    body: JSON.stringify({ new_path: newPath })
  });
  return response.ok;
}

async function createFolder(path, folderName) {
  const fullPath = path ? `${path}/${folderName}` : folderName;
  const response = await apiCall(`/dirs/${encodeURIComponent(fullPath)}`, { method: 'POST' });
  return response.ok;
}

async function searchFiles(query) {
  const response = await apiCall(`/search?q=${encodeURIComponent(query)}`);
  if (response.ok) {
    return await response.json();
  }
  return [];
}

async function getStats() {
  const response = await apiCall('/stats');
  if (response.ok) {
    return await response.json();
  }
  return { file_count: 0, total_size: 0 };
}

// =============================================================================
// KEYBOARD SHORTCUTS
// =============================================================================

document.addEventListener('keydown', (e) => {
  // Ctrl+U: Upload
  if (e.ctrlKey && e.key === 'u') {
    e.preventDefault();
    document.getElementById('file-input')?.click();
  }
  
  // Ctrl+N: New Folder
  if (e.ctrlKey && e.key === 'n') {
    e.preventDefault();
    handleNewFolder();
  }
  
  // F2: Rename
  if (e.key === 'F2' && selectedFiles.size === 1) {
    e.preventDefault();
    const file = Array.from(selectedFiles)[0];
    handleRename(file);
  }
  
  // Delete: Delete selected
  if (e.key === 'Delete' && selectedFiles.size > 0) {
    e.preventDefault();
    handleBulkDelete();
  }
  
  // Ctrl+A: Select all
  if (e.ctrlKey && e.key === 'a') {
    e.preventDefault();
    selectAllFiles();
  }
  
  // Escape: Cancel selection / Close modals
  if (e.key === 'Escape') {
    selectedFiles.clear();
    multiSelectMode = false;
    hideContextMenu();
    hidePreview();
    renderCurrentView();
  }
  
  // Ctrl+C: Copy
  if (e.ctrlKey && e.key === 'c' && selectedFiles.size > 0) {
    e.preventDefault();
    clipboard = { operation: 'copy', files: Array.from(selectedFiles) };
  }
  
  // Ctrl+X: Cut
  if (e.ctrlKey && e.key === 'x' && selectedFiles.size > 0) {
    e.preventDefault();
    clipboard = { operation: 'cut', files: Array.from(selectedFiles) };
  }
  
  // Ctrl+V: Paste
  if (e.ctrlKey && e.key === 'v' && clipboard) {
    e.preventDefault();
    handlePaste();
  }
  
  // Ctrl+F: Search
  if (e.ctrlKey && e.key === 'f') {
    e.preventDefault();
    navigateToView('search');
  }
});

// =============================================================================
// FILE OPERATIONS
// =============================================================================

async function handleUpload(files) {
  const promises = Array.from(files).map(file => uploadFile(file, currentPath));
  await Promise.all(promises);
  await refreshCurrentView();
}

async function handleNewFolder() {
  const name = prompt(t('folder_name'));
  if (name) {
    await createFolder(currentPath, name);
    await refreshCurrentView();
  }
}

async function handleRename(filename) {
  const newName = prompt(t('new_name'), filename);
  if (newName && newName !== filename) {
    const oldPath = currentPath ? `${currentPath}/${filename}` : filename;
    const newPath = currentPath ? `${currentPath}/${newName}` : newName;
    await renameFile(oldPath, newPath);
    selectedFiles.delete(filename);
    await refreshCurrentView();
  }
}

async function handleDelete(filename) {
  if (confirm(`${t('confirm_delete')} "${filename}"?`)) {
    const path = currentPath ? `${currentPath}/${filename}` : filename;
    await deleteFile(path);
    selectedFiles.delete(filename);
    await refreshCurrentView();
  }
}

async function handleBulkDelete() {
  if (confirm(`${t('confirm_delete')} ${selectedFiles.size} ${t('items_selected')}?`)) {
    const promises = Array.from(selectedFiles).map(filename => {
      const path = currentPath ? `${currentPath}/${filename}` : filename;
      return deleteFile(path);
    });
    await Promise.all(promises);
    selectedFiles.clear();
    multiSelectMode = false;
    await refreshCurrentView();
  }
}

async function handleDownload(filename) {
  const path = currentPath ? `${currentPath}/${filename}` : filename;
  window.open(`${API_BASE}/file/${encodeURIComponent(path)}`, '_blank');
}

function handlePaste() {
  // TODO: Implement copy/paste functionality
  console.log('Paste operation:', clipboard);
}

function toggleFavorite(path) {
  const index = favorites.indexOf(path);
  if (index > -1) {
    favorites.splice(index, 1);
  } else {
    favorites.push(path);
  }
  localStorage.setItem('favorites', JSON.stringify(favorites));
  renderCurrentView();
}

function selectAllFiles() {
  const fileElements = document.querySelectorAll('.file-card, .file-list-item');
  fileElements.forEach(el => {
    const filename = el.dataset.filename;
    if (filename) selectedFiles.add(filename);
  });
  multiSelectMode = true;
  renderCurrentView();
}

// =============================================================================
// CONTEXT MENU
// =============================================================================

function showContextMenu(e, filename, isDir) {
  e.preventDefault();
  const menu = document.getElementById('context-menu');
  menu.innerHTML = `
    <div class="context-menu-item" onclick="handlePreview('${filename}')">
      <span class="material-symbols-outlined">visibility</span>
      ${t('preview')}
    </div>
    <div class="context-menu-item" onclick="handleDownload('${filename}')">
      <span class="material-symbols-outlined">download</span>
      ${t('download')}
    </div>
    <div class="context-menu-item" onclick="handleRename('${filename}')">
      <span class="material-symbols-outlined">edit</span>
      ${t('rename')}
    </div>
    <div class="context-menu-divider"></div>
    <div class="context-menu-item" onclick="clipboard={operation:'copy',files:['${filename}']};hideContextMenu()">
      <span class="material-symbols-outlined">content_copy</span>
      ${t('copy')}
    </div>
    <div class="context-menu-item" onclick="clipboard={operation:'cut',files:['${filename}']};hideContextMenu()">
      <span class="material-symbols-outlined">content_cut</span>
      ${t('cut')}
    </div>
    ${favorites.includes(currentPath + '/' + filename) ? `
    <div class="context-menu-item" onclick="toggleFavorite('${currentPath}/${filename}')">
      <span class="material-symbols-outlined">star</span>
      ${t('unfavorite')}
    </div>
    ` : `
    <div class="context-menu-item" onclick="toggleFavorite('${currentPath}/${filename}')">
      <span class="material-symbols-outlined">star_border</span>
      ${t('favorite')}
    </div>
    `}
    <div class="context-menu-divider"></div>
    <div class="context-menu-item danger" onclick="handleDelete('${filename}')">
      <span class="material-symbols-outlined">delete</span>
      ${t('delete')}
    </div>
  `;
  
  menu.style.left = e.pageX + 'px';
  menu.style.top = e.pageY + 'px';
  menu.classList.remove('hidden');
}

function hideContextMenu() {
  document.getElementById('context-menu').classList.add('hidden');
}

document.addEventListener('click', hideContextMenu);

// =============================================================================
// FILE PREVIEW
// =============================================================================

function handlePreview(filename) {
  const path = currentPath ? `${currentPath}/${filename}` : filename;
  const ext = filename.split('.').pop().toLowerCase();
  
  const modal = document.getElementById('preview-modal');
  let content = '';
  
  if (['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp'].includes(ext)) {
    content = `<img src="${API_BASE}/file/${encodeURIComponent(path)}" alt="${filename}">`;
  } else if (['mp4', 'webm'].includes(ext)) {
    content = `<video controls src="${API_BASE}/file/${encodeURIComponent(path)}"></video>`;
  } else if (['mp3', 'wav', 'ogg'].includes(ext)) {
    content = `<audio controls src="${API_BASE}/file/${encodeURIComponent(path)}"></audio>`;
  } else if (ext === 'pdf') {
    content = `<iframe src="${API_BASE}/file/${encodeURIComponent(path)}" style="width:100%;height:80vh;border:none;"></iframe>`;
  } else if (['txt', 'md', 'json', 'html', 'css', 'js'].includes(ext)) {
    fetch(`${API_BASE}/file/${encodeURIComponent(path)}`)
      .then(r => r.text())
      .then(text => {
        document.querySelector('.preview-content').innerHTML = `<pre style="white-space:pre-wrap;font-family:monospace;">${escapeHtml(text)}</pre>`;
      });
    content = '<div>Loading...</div>';
  } else {
    content = '<div>Preview not available for this file type</div>';
  }
  
  modal.innerHTML = `
    <div class="preview-container">
      <div class="preview-header">
        <h3>${filename}</h3>
        <md-icon-button onclick="hidePreview()">
          <span class="material-symbols-outlined">close</span>
        </md-icon-button>
      </div>
      <div class="preview-content">${content}</div>
    </div>
  `;
  
  modal.classList.remove('hidden');
  hideContextMenu();
}

function hidePreview() {
  document.getElementById('preview-modal').classList.add('hidden');
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// =============================================================================
// UI RENDERING
// =============================================================================

function showLogin() {
  document.getElementById('loading').classList.add('hidden');
  const loginPage = document.getElementById('login-page');
  loginPage.classList.remove('hidden');
  loginPage.innerHTML = `
    <div class="login-card">
      <h1>${t('app_title')}</h1>
      <p>Advanced File Management</p>
      <form id="login-form" style="display:flex;flex-direction:column;gap:16px;">
        <md-filled-text-field id="username" label="${t('username')}" required></md-filled-text-field>
        <md-filled-text-field id="password" label="${t('password')}" type="password" required></md-filled-text-field>
        <md-filled-text-field id="twofa" label="${t('twofa_code')}"></md-filled-text-field>
        <md-filled-button type="submit">${t('signin')}</md-filled-button>
      </form>
    </div>
  `;
  
  document.getElementById('login-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const twofa = document.getElementById('twofa').value;
    
    if (await login(username, password, twofa)) {
      loginPage.classList.add('hidden');
      showApp();
    } else {
      alert('Login failed!');
    }
  });
}

function showApp() {
  document.getElementById('app').classList.remove('hidden');
  if (darkMode) document.body.classList.add('dark-theme');
  renderApp();
}

function renderApp() {
  document.getElementById('app').innerHTML = `
    <div class="app-bar">
      <md-icon-button onclick="toggleDrawer()">
        <span class="material-symbols-outlined">menu</span>
      </md-icon-button>
      <h1>${t('app_title')}</h1>
      <md-icon-button onclick="setLanguage(currentLanguage === 'en' ? 'de' : 'en')">
        <span class="material-symbols-outlined">language</span>
      </md-icon-button>
      <md-icon-button onclick="toggleTheme()">
        <span class="material-symbols-outlined">${darkMode ? 'light_mode' : 'dark_mode'}</span>
      </md-icon-button>
    </div>
    <div class="app-content">
      <div class="nav-drawer" id="nav-drawer">
        <div class="nav-item ${currentView === 'files' ? 'active' : ''}" onclick="navigateToView('files')">
          <span class="material-symbols-outlined">folder</span>
          ${t('files')}
        </div>
        <div class="nav-item ${currentView === 'favorites' ? 'active' : ''}" onclick="navigateToView('favorites')">
          <span class="material-symbols-outlined">star</span>
          ${t('favorites')}
        </div>
        <div class="nav-item ${currentView === 'search' ? 'active' : ''}" onclick="navigateToView('search')">
          <span class="material-symbols-outlined">search</span>
          ${t('search')}
        </div>
        <div class="nav-item ${currentView === 'trash' ? 'active' : ''}" onclick="navigateToView('trash')">
          <span class="material-symbols-outlined">delete</span>
          ${t('trash')}
        </div>
        <div class="nav-item ${currentView === 'peers' ? 'active' : ''}" onclick="navigateToView('peers')">
          <span class="material-symbols-outlined">devices</span>
          ${t('peers')}
        </div>
        <div class="nav-item ${currentView === 'settings' ? 'active' : ''}" onclick="navigateToView('settings')">
          <span class="material-symbols-outlined">settings</span>
          ${t('settings')}
        </div>
        <div style="flex:1"></div>
        <div class="nav-item" onclick="handleLogout()">
          <span class="material-symbols-outlined">logout</span>
          ${t('logout')}
        </div>
      </div>
      <div class="content-area" id="content-area"></div>
    </div>
    ${multiSelectMode ? `
    <div class="multi-select-bar">
      <span>${selectedFiles.size} ${t('items_selected')}</span>
      <div style="display:flex;gap:8px;">
        <md-filled-button onclick="handleBulkDelete()">
          <span class="material-symbols-outlined" slot="icon">delete</span>
          ${t('delete')}
        </md-filled-button>
        <md-text-button onclick="selectedFiles.clear();multiSelectMode=false;renderCurrentView()">
          ${t('cancel')}
        </md-text-button>
      </div>
    </div>
    ` : ''}
    <div class="fab-container">
      <md-fab label="${t('upload')}" onclick="document.getElementById('file-input').click()">
        <span class="material-symbols-outlined" slot="icon">upload</span>
      </md-fab>
    </div>
    <input type="file" id="file-input" multiple hidden onchange="handleUpload(this.files);this.value=''">
  `;
  
  renderCurrentView();
}

async function renderCurrentView() {
  const container = document.getElementById('content-area');
  if (!container) return;
  
  switch(currentView) {
    case 'files':
      await renderFilesView(container);
      break;
    case 'search':
      renderSearchView(container);
      break;
    case 'favorites':
      renderFavoritesView(container);
      break;
    case 'trash':
      renderTrashView(container);
      break;
    case 'peers':
      renderPeersView(container);
      break;
    case 'settings':
      renderSettingsView(container);
      break;
  }
}

async function renderFilesView(container) {
  const files = await loadFiles(currentPath);
  
  // Breadcrumb
  const pathParts = currentPath ? currentPath.split('/') : [];
  let breadcrumb = `
    <div class="breadcrumb">
      <div class="breadcrumb-item" onclick="navigateToPath('')">
        <span class="material-symbols-outlined">home</span>
        ${t('home')}
      </div>
  `;
  
  let buildPath = '';
  pathParts.forEach((part, i) => {
    buildPath += (buildPath ? '/' : '') + part;
    const path = buildPath;
    breadcrumb += `
      <span class="breadcrumb-separator">/</span>
      <div class="breadcrumb-item" onclick="navigateToPath('${path}')">${part}</div>
    `;
  });
  breadcrumb += '</div>';
  
  // View Controls
  const controls = `
    <div class="view-controls">
      <md-filled-button onclick="handleNewFolder()">
        <span class="material-symbols-outlined" slot="icon">create_new_folder</span>
        ${t('new_folder')}
      </md-filled-button>
      <div style="display:flex;gap:8px;">
        <md-icon-button onclick="multiSelectMode=!multiSelectMode;renderCurrentView()" title="${t('multiselect')}">
          <span class="material-symbols-outlined">checklist</span>
        </md-icon-button>
        <md-icon-button onclick="viewMode='${viewMode === 'grid' ? 'list' : 'grid'}';localStorage.setItem('viewMode',viewMode);renderCurrentView()" title="${t('toggle_view')}">
          <span class="material-symbols-outlined">${viewMode === 'grid' ? 'view_list' : 'grid_view'}</span>
        </md-icon-button>
      </div>
    </div>
  `;
  
  // Drag & Drop Zone
  const dropZone = `
    <div class="drop-zone" id="drop-zone">
      <span class="material-symbols-outlined">cloud_upload</span>
      <p style="font-weight:500;margin-bottom:4px;">${t('drag_drop_here')}</p>
      <p style="color:var(--md-sys-color-on-surface-variant);font-size:14px;">${t('or_click_upload')}</p>
    </div>
  `;
  
  // File List
  let fileList = files.length === 0 ? 
    `<div style="text-align:center;color:var(--md-sys-color-on-surface-variant);padding:48px;">${t('no_files')}</div>` : '';
  
  if (files.length > 0) {
    if (viewMode === 'grid') {
      fileList = '<div class="file-grid">';
      files.forEach(file => {
        const isSelected = selectedFiles.has(file.name);
        fileList += `
          <div class="file-card ${isSelected ? 'selected' : ''}" 
               data-filename="${file.name}"
               onclick="handleFileClick('${file.name}', ${file.is_dir})"
               oncontextmenu="showContextMenu(event, '${file.name}', ${file.is_dir})">
            ${multiSelectMode ? `<input type="checkbox" ${isSelected ? 'checked' : ''} style="position:absolute;top:8px;right:8px;">` : ''}
            <div class="file-icon">
              <span class="material-symbols-outlined">${getFileIcon(file.name, file.is_dir)}</span>
            </div>
            <div class="file-name">${file.name}</div>
            <div class="file-meta">${file.is_dir ? 'Folder' : formatSize(file.size)}</div>
          </div>
        `;
      });
      fileList += '</div>';
    } else {
      fileList = '<div class="file-list">';
      files.forEach(file => {
        const isSelected = selectedFiles.has(file.name);
        fileList += `
          <div class="file-list-item ${isSelected ? 'selected' : ''}"
               data-filename="${file.name}"
               onclick="handleFileClick('${file.name}', ${file.is_dir})"
               oncontextmenu="showContextMenu(event, '${file.name}', ${file.is_dir})">
            ${multiSelectMode ? `<input type="checkbox" ${isSelected ? 'checked' : ''}>` : ''}
            <span class="material-symbols-outlined">${getFileIcon(file.name, file.is_dir)}</span>
            <div style="flex:1;">${file.name}</div>
            <div style="color:var(--md-sys-color-on-surface-variant);">${file.is_dir ? 'Folder' : formatSize(file.size)}</div>
          </div>
        `;
      });
      fileList += '</div>';
    }
  }
  
  container.innerHTML = breadcrumb + controls + dropZone + fileList;
  
  // Setup drag & drop
  const dropZoneEl = document.getElementById('drop-zone');
  ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
    dropZoneEl.addEventListener(eventName, e => {
      e.preventDefault();
      e.stopPropagation();
    });
  });
  
  ['dragenter', 'dragover'].forEach(eventName => {
    dropZoneEl.addEventListener(eventName, () => dropZoneEl.classList.add('drag-over'));
  });
  
  ['dragleave', 'drop'].forEach(eventName => {
    dropZoneEl.addEventListener(eventName, () => dropZoneEl.classList.remove('drag-over'));
  });
  
  dropZoneEl.addEventListener('drop', e => {
    const files = e.dataTransfer.files;
    handleUpload(files);
  });
}

function renderSearchView(container) {
  container.innerHTML = `
    <h2>${t('search')}</h2>
    <md-filled-text-field 
      id="search-input" 
      label="${t('search_placeholder')}" 
      style="width:100%;margin-bottom:24px;"
      oninput="performSearch(this.value)">
      <span class="material-symbols-outlined" slot="leadingicon">search</span>
    </md-filled-text-field>
    <div id="search-results" class="search-results"></div>
  `;
}

async function performSearch(query) {
  if (query.length < 2) {
    document.getElementById('search-results').innerHTML = '';
    return;
  }
  
  const results = await searchFiles(query);
  const container = document.getElementById('search-results');
  
  if (results.length === 0) {
    container.innerHTML = `<p style="color:var(--md-sys-color-on-surface-variant);">${t('no_results')}</p>`;
    return;
  }
  
  container.innerHTML = results.map(file => `
    <div class="search-result-item" onclick="navigateToPath('${file.path.split('/').slice(0,-1).join('/')}')">
      <span class="material-symbols-outlined">${getFileIcon(file.path, file.is_dir)}</span>
      <div style="flex:1;">
        <div style="font-weight:500;">${file.path.split('/').pop()}</div>
        <div style="font-size:12px;color:var(--md-sys-color-on-surface-variant);">${file.path}</div>
      </div>
      <div style="color:var(--md-sys-color-on-surface-variant);">${formatSize(file.size)}</div>
    </div>
  `).join('');
}

function renderFavoritesView(container) {
  if (favorites.length === 0) {
    container.innerHTML = `
      <div style="text-align:center;padding:48px;">
        <span class="material-symbols-outlined" style="font-size:64px;color:var(--md-sys-color-on-surface-variant);">star_border</span>
        <p style="color:var(--md-sys-color-on-surface-variant);margin-top:16px;">No favorites yet</p>
      </div>
    `;
    return;
  }
  
  container.innerHTML = `
    <h2>${t('favorites')}</h2>
    <div class="search-results">
      ${favorites.map(path => `
        <div class="search-result-item" onclick="navigateToPath('${path.split('/').slice(0,-1).join('/')}')">
          <span class="material-symbols-outlined">star</span>
          <div style="flex:1;">${path.split('/').pop()}</div>
          <md-icon-button onclick="event.stopPropagation();toggleFavorite('${path}')">
            <span class="material-symbols-outlined">close</span>
          </md-icon-button>
        </div>
      `).join('')}
    </div>
  `;
}

function renderTrashView(container) {
  container.innerHTML = `
    <h2>${t('trash')}</h2>
    <p style="color:var(--md-sys-color-on-surface-variant);">Trash feature coming soon...</p>
  `;
}

function renderPeersView(container) {
  container.innerHTML = `
    <h2>${t('peers')}</h2>
    <p style="color:var(--md-sys-color-on-surface-variant);">Peer synchronization coming soon...</p>
  `;
}

async function renderSettingsView(container) {
  const stats = await getStats();
  
  container.innerHTML = `
    <h2>${t('settings')}</h2>
    
    <div class="settings-section">
      <h3>${t('theme')}</h3>
      <div class="setting-item">
        <span>Dark Mode</span>
        <md-switch ${darkMode ? 'selected' : ''} onchange="toggleTheme()"></md-switch>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>${t('language')}</h3>
      <div class="setting-item">
        <span>English</span>
        <md-radio name="lang" ${currentLanguage === 'en' ? 'checked' : ''} onchange="setLanguage('en')"></md-radio>
      </div>
      <div class="setting-item">
        <span>Deutsch</span>
        <md-radio name="lang" ${currentLanguage === 'de' ? 'checked' : ''} onchange="setLanguage('de')"></md-radio>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>${t('storage_usage')}</h3>
      <div class="setting-item">
        <span>Files</span>
        <span>${stats.file_count}</span>
      </div>
      <div class="setting-item">
        <span>Total Size</span>
        <span>${formatSize(stats.total_size)}</span>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>Security</h3>
      <md-filled-button onclick="alert('2FA setup coming soon')">
        ${t('enable_2fa')}
      </md-filled-button>
      <md-filled-button onclick="alert('Password change coming soon')" style="margin-left:8px;">
        ${t('change_password')}
      </md-filled-button>
    </div>
  `;
}

// =============================================================================
// NAVIGATION & INTERACTION
// =============================================================================

function navigateToView(view) {
  currentView = view;
  selectedFiles.clear();
  multiSelectMode = false;
  renderApp();
}

function navigateToPath(path) {
  currentPath = path;
  currentView = 'files';
  selectedFiles.clear();
  multiSelectMode = false;
  renderCurrentView();
}

function handleFileClick(filename, isDir) {
  if (multiSelectMode) {
    if (selectedFiles.has(filename)) {
      selectedFiles.delete(filename);
    } else {
      selectedFiles.add(filename);
    }
    renderCurrentView();
  } else if (isDir) {
    navigateToPath(currentPath ? `${currentPath}/${filename}` : filename);
  } else {
    if (canPreview(filename)) {
      handlePreview(filename);
    } else {
      handleDownload(filename);
    }
  }
}

function toggleDrawer() {
  document.getElementById('nav-drawer').classList.toggle('closed');
}

function toggleTheme() {
  darkMode = !darkMode;
  localStorage.setItem('darkMode', darkMode);
  if (darkMode) {
    document.body.classList.add('dark-theme');
  } else {
    document.body.classList.remove('dark-theme');
  }
  renderApp();
}

function handleLogout() {
  localStorage.removeItem('authToken');
  location.reload();
}

async function refreshCurrentView() {
  await renderCurrentView();
}

// =============================================================================
// INITIALIZATION
// =============================================================================

window.addEventListener('DOMContentLoaded', () => {
  if (authToken) {
    showApp();
  } else {
    showLogin();
  }
});

// Make functions globally accessible
window.handleUpload = handleUpload;
window.handleNewFolder = handleNewFolder;
window.handleRename = handleRename;
window.handleDelete = handleDelete;
window.handleDownload = handleDownload;
window.handleBulkDelete = handleBulkDelete;
window.handlePreview = handlePreview;
window.hidePreview = hidePreview;
window.showContextMenu = showContextMenu;
window.hideContextMenu = hideContextMenu;
window.handleFileClick = handleFileClick;
window.navigateToView = navigateToView;
window.navigateToPath = navigateToPath;
window.toggleDrawer = toggleDrawer;
window.toggleTheme = toggleTheme;
window.toggleFavorite = toggleFavorite;
window.setLanguage = setLanguage;
window.performSearch = performSearch;
window.handleLogout = handleLogout;
window.handlePaste = handlePaste;
window.selectAllFiles = selectAllFiles;
