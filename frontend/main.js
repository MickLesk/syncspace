// ========================import { LitElement, html, css } from 'https://cdn.jsdelivr.net/gh/lit/dist@3/core/lit-all.min.js';

// SyncSpace Main Entry

// Material 3 Expressive with Lit/**

// ======================== * Main component for the SyncSpace web UI. This component provides a

 * Material‐inspired interface to interact with the backend API: it lists

import { LitElement, html, css } from 'https://cdn.jsdelivr.net/gh/lit/dist@3/core/lit-core.min.js'; * synchronised files and directories, allows uploads, renames, deletions,

 * directory creation, searching, peer management and displays basic

// ======================== * statistics. The design draws inspiration from Material 3 Expressive

// App Shell Component * guidelines with card layouts, vibrant colours and rounded corners.

// ======================== */

class MyApp extends LitElement {

class AppShell extends LitElement {  static properties = {

  static properties = {    files: { state: true },

    isLoggedIn: { type: Boolean },    peers: { state: true },

    currentView: { type: String },    stats: { state: true },

    darkMode: { type: Boolean },    newDir: { state: true },

    drawerOpen: { type: Boolean }    newPeer: { state: true },

  };    currentPath: { state: true },

    searchQuery: { state: true },

  static styles = css`    searchResults: { state: true },

    :host {    isDragging: { state: true },

      display: block;    isLoading: { state: true },

      width: 100vw;    notification: { state: true },

      height: 100vh;  };

      overflow: hidden;

    }  constructor() {

        super();

    .app-container {    this.files = [];

      display: flex;    this.peers = [];

      flex-direction: column;    this.stats = { file_count: 0, total_size: 0 };

      height: 100%;    this.newDir = '';

    }    this.newPeer = '';

        this.currentPath = '';

    /* Top App Bar - Material 3 Expressive */    this.searchQuery = '';

    .top-app-bar {    this.searchResults = [];

      display: flex;    this.ws = null;

      align-items: center;    this.isDragging = false;

      justify-content: space-between;    this.isLoading = false;

      height: 64px;    this.notification = '';

      padding: 0 16px;  }

      background: linear-gradient(135deg, var(--md-sys-color-primary) 0%, var(--md-sys-color-tertiary) 100%);

      color: var(--md-sys-color-on-primary);  /**

      box-shadow: var(--md-sys-elevation-level2);   * Lifecycle hook: invoked when the element is added to the DOM. Initialise

      position: relative;   * data and connect the WebSocket.

      z-index: 100;   */

    }  connectedCallback() {

        super.connectedCallback();

    .app-bar-left {    this.loadFiles();

      display: flex;    this.loadPeers();

      align-items: center;    this.loadStats();

      gap: 16px;    this.initWebSocket();

    }  }

    

    .menu-button {  disconnectedCallback() {

      background: none;    super.disconnectedCallback();

      border: none;    if (this.ws) this.ws.close();

      color: inherit;  }

      font-size: 24px;

      cursor: pointer;  /** Fetch the list of entries for the current directory. */

      padding: 8px;  async loadFiles(sub = '') {

      border-radius: 50%;    try {

      transition: background 0.2s;      const path = sub || this.currentPath || '';

    }      const res = await fetch(`http://localhost:8080/api/files/${encodeURIComponent(path)}`);

          if (!res.ok) throw new Error('Failed to load files');

    .menu-button:hover {      const data = await res.json();

      background: rgba(255, 255, 255, 0.1);      this.files = Array.isArray(data) ? data : [];

    }    } catch (err) {

          console.error(err);

    .app-title {    }

      font-size: 22px;  }

      font-weight: 500;

      letter-spacing: 0.5px;  /** Fetch the list of peers. */

    }  async loadPeers() {

        try {

    .app-bar-actions {      const res = await fetch('http://localhost:8080/api/peers');

      display: flex;      if (!res.ok) throw new Error('Failed to load peers');

      gap: 8px;      const data = await res.json();

    }      this.peers = Array.isArray(data) ? data : [];

        } catch (err) {

    /* Main Content Area */      console.error(err);

    .main-content {    }

      display: flex;  }

      flex: 1;

      overflow: hidden;  /** Fetch basic statistics about the sync directory. */

    }  async loadStats() {

        try {

    /* Navigation Drawer */      const res = await fetch('http://localhost:8080/api/stats');

    .nav-drawer {      if (!res.ok) throw new Error('Failed to load stats');

      width: 280px;      const data = await res.json();

      background: var(--md-sys-color-surface);      this.stats = data;

      border-right: 1px solid var(--md-sys-color-outline-variant);    } catch (err) {

      padding: 16px 0;      console.error(err);

      overflow-y: auto;    }

      transition: transform 0.3s cubic-bezier(0.4, 0.0, 0.2, 1);  }

    }

      /** Set up a WebSocket connection for live file system updates. */

    .nav-drawer.closed {  initWebSocket() {

      transform: translateX(-100%);    try {

    }      this.ws = new WebSocket('ws://localhost:8080/api/ws');

          this.ws.onmessage = () => {

    .nav-item {        // When a file change event is received, reload files and stats

      display: flex;        this.loadFiles();

      align-items: center;        this.loadStats();

      gap: 12px;      };

      padding: 12px 24px;    } catch (err) {

      margin: 4px 12px;      console.error('WebSocket init error', err);

      border-radius: 24px;    }

      cursor: pointer;  }

      transition: all 0.2s;

      color: var(--md-sys-color-on-surface);  /** Upload the selected file to the backend. */

    }  async uploadFile(file = null) {

        this.isLoading = true;

    .nav-item:hover {    try {

      background: var(--md-sys-color-secondary-container);      let fileToUpload = file;

    }      if (!fileToUpload) {

            const input = this.renderRoot?.getElementById('fileInput');

    .nav-item.active {        fileToUpload = input?.files?.[0];

      background: var(--md-sys-color-secondary-container);        if (!fileToUpload) return;

      color: var(--md-sys-color-on-secondary-container);      }

    }      

          const arrayBuffer = await fileToUpload.arrayBuffer();

    .nav-icon {      const bytes = new Uint8Array(arrayBuffer);

      font-size: 24px;      const path = this.currentPath ? `${this.currentPath}/${fileToUpload.name}` : fileToUpload.name;

    }      const url = `http://localhost:8080/api/upload/${encodeURIComponent(path)}`;

          const res = await fetch(url, { method: 'POST', body: bytes });

    /* Content Area */      if (!res.ok) throw new Error('Upload failed');

    .content-area {      

      flex: 1;      this.showNotification(`✓ Uploaded: ${fileToUpload.name}`, 'success');

      overflow-y: auto;      await this.loadFiles();

      padding: 24px;      await this.loadStats();

      background: var(--md-sys-color-background);      

    }      const input = this.renderRoot?.getElementById('fileInput');

          if (input) input.value = '';

    /* FAB - Floating Action Button */    } catch (err) {

    .fab {      console.error(err);

      position: fixed;      this.showNotification('✗ Upload failed', 'error');

      bottom: 24px;    } finally {

      right: 24px;      this.isLoading = false;

      width: 56px;    }

      height: 56px;  }

      border-radius: 16px;

      background: var(--md-sys-color-primary-container);  /** Handle drag and drop events. */

      color: var(--md-sys-color-on-primary-container);  handleDragOver(e) {

      border: none;    e.preventDefault();

      box-shadow: var(--md-sys-elevation-level3);    this.isDragging = true;

      cursor: pointer;  }

      font-size: 24px;

      display: flex;  handleDragLeave(e) {

      align-items: center;    e.preventDefault();

      justify-content: center;    this.isDragging = false;

      transition: all 0.3s cubic-bezier(0.4, 0.0, 0.2, 1);  }

    }

      async handleDrop(e) {

    .fab:hover {    e.preventDefault();

      box-shadow: var(--md-sys-elevation-level4);    this.isDragging = false;

      transform: scale(1.05);    

    }    const files = Array.from(e.dataTransfer.files);

        if (files.length === 0) return;

    .fab:active {    

      transform: scale(0.95);    // Upload all dropped files

    }    for (const file of files) {

  `;      await this.uploadFile(file);

    }

  constructor() {  }

    super();

    this.isLoggedIn = !!localStorage.getItem('authToken');  /** Show a notification message. */

    this.currentView = 'files';  showNotification(message, type = 'info') {

    this.darkMode = localStorage.getItem('theme') === 'dark';    this.notification = { message, type };

    this.drawerOpen = true;    setTimeout(() => {

          this.notification = '';

    // Hide loading screen    }, 3000);

    setTimeout(() => {  }

      const loading = document.getElementById('app-loading');

      if (loading) loading.style.display = 'none';  /** Delete a file or directory by name (relative to currentPath). */

    }, 100);  async deleteEntry(name) {

        if (!confirm(`Delete "${name}"?`)) return;

    // Apply theme    

    this.applyTheme();    this.isLoading = true;

  }    try {

      const path = this.currentPath ? `${this.currentPath}/${name}` : name;

  applyTheme() {      const url = `http://localhost:8080/api/files/${encodeURIComponent(path)}`;

    if (this.darkMode) {      const res = await fetch(url, { method: 'DELETE' });

      document.body.classList.add('dark-theme');      if (!res.ok) throw new Error('Delete failed');

    } else {      

      document.body.classList.remove('dark-theme');      this.showNotification(`✓ Deleted: ${name}`, 'success');

    }      await this.loadFiles();

  }      await this.loadStats();

    } catch (err) {

  toggleTheme() {      console.error(err);

    this.darkMode = !this.darkMode;      this.showNotification('✗ Delete failed', 'error');

    localStorage.setItem('theme', this.darkMode ? 'dark' : 'light');    } finally {

    this.applyTheme();      this.isLoading = false;

  }    }

  }

  toggleDrawer() {

    this.drawerOpen = !this.drawerOpen;  /** Download a file by opening a new window to the file endpoint. */

  }  downloadEntry(name) {

    const path = this.currentPath ? `${this.currentPath}/${name}` : name;

  navigate(view) {    const url = `http://localhost:8080/api/file/${encodeURIComponent(path)}`;

    this.currentView = view;    window.open(url, '_blank');

  }  }



  logout() {  /** Create a new directory relative to currentPath. */

    localStorage.removeItem('authToken');  async createDir() {

    this.isLoggedIn = false;    if (!this.newDir) return;

  }    

    this.isLoading = true;

  render() {    try {

    if (!this.isLoggedIn) {      const path = this.currentPath ? `${this.currentPath}/${this.newDir}` : this.newDir;

      return html`<login-view @login=${() => this.isLoggedIn = true}></login-view>`;      const url = `http://localhost:8080/api/dirs/${encodeURIComponent(path)}`;

    }      const res = await fetch(url, { method: 'POST' });

      if (!res.ok) throw new Error('Failed to create directory');

    return html`      

      <div class="app-container">      this.showNotification(`✓ Created: ${this.newDir}`, 'success');

        <!-- Top App Bar -->      this.newDir = '';

        <div class="top-app-bar">      await this.loadFiles();

          <div class="app-bar-left">    } catch (err) {

            <button class="menu-button" @click=${this.toggleDrawer}>      console.error(err);

              <span class="material-symbols-outlined">menu</span>      this.showNotification('✗ Failed to create directory', 'error');

            </button>    } finally {

            <span class="app-title">SyncSpace</span>      this.isLoading = false;

          </div>    }

          <div class="app-bar-actions">  }

            <button class="menu-button" @click=${this.toggleTheme}>

              <span class="material-symbols-outlined">  /** Add a new peer address. */

                ${this.darkMode ? 'light_mode' : 'dark_mode'}  async addPeer() {

              </span>    if (!this.newPeer) return;

            </button>    try {

            <button class="menu-button" @click=${this.logout}>      const peer = { id: crypto.randomUUID(), address: this.newPeer, last_seen: null };

              <span class="material-symbols-outlined">logout</span>      const res = await fetch('http://localhost:8080/api/peers', {

            </button>        method: 'POST',

          </div>        headers: { 'Content-Type': 'application/json' },

        </div>        body: JSON.stringify(peer),

      });

        <!-- Main Content -->      if (!res.ok) throw new Error('Failed to add peer');

        <div class="main-content">      this.newPeer = '';

          <!-- Navigation Drawer -->      await this.loadPeers();

          <nav class="nav-drawer ${this.drawerOpen ? '' : 'closed'}">    } catch (err) {

            <div class="nav-item ${this.currentView === 'files' ? 'active' : ''}"      console.error(err);

                 @click=${() => this.navigate('files')}>    }

              <span class="material-symbols-outlined nav-icon">folder</span>  }

              <span>Files</span>

            </div>  /** Navigate into a directory. */

            <div class="nav-item ${this.currentView === 'search' ? 'active' : ''}"  navigate(dir) {

                 @click=${() => this.navigate('search')}>    this.currentPath = this.currentPath ? `${this.currentPath}/${dir}` : dir;

              <span class="material-symbols-outlined nav-icon">search</span>    this.searchQuery = '';

              <span>Search</span>    this.searchResults = [];

            </div>    this.loadFiles();

            <div class="nav-item ${this.currentView === 'peers' ? 'active' : ''}"  }

                 @click=${() => this.navigate('peers')}>

              <span class="material-symbols-outlined nav-icon">devices</span>  /** Navigate using breadcrumb index. Pass -1 to go to root. */

              <span>Peers</span>  navigateToIndex(index) {

            </div>    if (index < 0) {

            <div class="nav-item ${this.currentView === 'settings' ? 'active' : ''}"      this.currentPath = '';

                 @click=${() => this.navigate('settings')}>    } else {

              <span class="material-symbols-outlined nav-icon">settings</span>      const parts = this.currentPath.split('/').filter(Boolean);

              <span>Settings</span>      this.currentPath = parts.slice(0, index + 1).join('/');

            </div>    }

          </nav>    this.searchQuery = '';

    this.searchResults = [];

          <!-- Content Area -->    this.loadFiles();

          <main class="content-area">  }

            ${this.renderView()}

          </main>  /** Rename a file or directory. */

        </div>  async renameEntry(name) {

    const newName = prompt('Enter new name', name);

        <!-- FAB -->    if (!newName || newName === name) return;

        ${this.currentView === 'files' ? html`    

          <button class="fab" @click=${this.handleFabClick}>    this.isLoading = true;

            <span class="material-symbols-outlined">add</span>    const oldPath = this.currentPath ? `${this.currentPath}/${name}` : name;

          </button>    const newPath = this.currentPath ? `${this.currentPath}/${newName}` : newName;

        ` : ''}    try {

      </div>      const url = `http://localhost:8080/api/rename/${encodeURIComponent(oldPath)}`;

    `;      const res = await fetch(url, {

  }        method: 'PUT',

        headers: { 'Content-Type': 'application/json' },

  renderView() {        body: JSON.stringify({ new_path: newPath }),

    switch (this.currentView) {      });

      case 'files':      if (!res.ok) throw new Error('Rename failed');

        return html`<files-view></files-view>`;      

      case 'search':      this.showNotification(`✓ Renamed to: ${newName}`, 'success');

        return html`<search-view></search-view>`;      await this.loadFiles();

      case 'peers':    } catch (err) {

        return html`<peers-view></peers-view>`;      console.error(err);

      case 'settings':      this.showNotification('✗ Rename failed', 'error');

        return html`<settings-view></settings-view>`;    } finally {

      default:      this.isLoading = false;

        return html`<p>Unknown view</p>`;    }

    }  }

  }

  /** Perform a search across all files/directories. */

  handleFabClick() {  async search() {

    // Show upload dialog or menu    if (!this.searchQuery) {

    const input = document.createElement('input');      this.searchResults = [];

    input.type = 'file';      return;

    input.multiple = true;    }

    input.onchange = (e) => {    try {

      const filesView = this.shadowRoot.querySelector('files-view');      const url = `http://localhost:8080/api/search?q=${encodeURIComponent(this.searchQuery)}`;

      if (filesView) {      const res = await fetch(url);

        filesView.uploadFiles(Array.from(e.target.files));      if (!res.ok) throw new Error('Search failed');

      }      const data = await res.json();

    };      this.searchResults = Array.isArray(data) ? data : [];

    input.click();    } catch (err) {

  }      console.error(err);

}    }

  }

customElements.define('app-shell', AppShell);

  /** Format bytes into a human readable string. */

// Import all components  formatSize(bytes) {

import './components/login-view.js';    const thresh = 1024;

import './components/files-view.js';    if (Math.abs(bytes) < thresh) return `${bytes} B`;

import './components/search-view.js';    const units = ['KB', 'MB', 'GB', 'TB'];

import './components/peers-view.js';    let u = -1;

import './components/settings-view.js';    let size = bytes;

    do {
      size /= thresh;
      ++u;
    } while (Math.abs(size) >= thresh && u < units.length - 1);
    return `${size.toFixed(1)} ${units[u]}`;
  }

  /** Define component styles using CSS custom properties to emulate Material 3 look. */
  static styles = css`
    :host {
      --primary-color: #6750a4;
      --secondary-color: #4f378b;
      --background-color: #f5f5f5;
      --surface-color: #ffffff;
      --on-primary: #ffffff;
      --on-surface: #1c1b1f;
      display: block;
      font-family: 'Roboto', sans-serif;
      background: var(--background-color);
      color: var(--on-surface);
      padding: 16px;
      box-sizing: border-box;
      max-width: 900px;
      margin: 0 auto;
    }
    .header {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
      padding: 16px;
      border-radius: 24px;
      margin-bottom: 24px;
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
    }
    h1 {
      margin: 0;
      font-size: 1.8rem;
    }
    nav {
      margin-bottom: 16px;
      font-size: 0.9rem;
    }
    nav span {
      cursor: pointer;
      color: var(--primary-color);
    }
    nav span:hover {
      text-decoration: underline;
    }
    .card {
      background: var(--surface-color);
      border-radius: 24px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
      margin-bottom: 24px;
      padding: 16px;
    }
    .card h2 {
      margin-top: 0;
      color: var(--primary-color);
      font-size: 1.4rem;
    }
    ul {
      list-style: none;
      padding: 0;
      margin: 0;
    }
    li {
      padding: 12px 8px;
      border-bottom: 1px solid #e0e0e0;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    li:last-child {
      border-bottom: none;
    }
    .actions button {
      margin-left: 8px;
    }
    .button {
      background: var(--primary-color);
      color: var(--on-primary);
      border: none;
      border-radius: 20px;
      padding: 8px 16px;
      cursor: pointer;
      font-size: 0.8rem;
    }
    .button.secondary {
      background: var(--secondary-color);
      color: var(--on-primary);
    }
    .button:disabled {
      opacity: 0.6;
      cursor: not-allowed;
    }
    input[type='text'],
    input[type='file'] {
      padding: 8px;
      border-radius: 12px;
      border: 1px solid #ccc;
      font-size: 0.9rem;
      margin-right: 8px;
    }
    input[type='text']:focus,
    input[type='file']:focus {
      outline: none;
      border-color: var(--primary-color);
      box-shadow: 0 0 0 2px rgba(103, 80, 164, 0.2);
    }
    .drop-zone {
      border: 2px dashed #ccc;
      border-radius: 16px;
      padding: 32px;
      text-align: center;
      transition: all 0.3s ease;
      margin-bottom: 16px;
      background: #fafafa;
    }
    .drop-zone.dragging {
      border-color: var(--primary-color);
      background: rgba(103, 80, 164, 0.1);
      transform: scale(1.02);
    }
    .notification {
      position: fixed;
      top: 24px;
      right: 24px;
      padding: 16px 24px;
      border-radius: 12px;
      background: white;
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
      z-index: 1000;
      animation: slideIn 0.3s ease;
      max-width: 300px;
    }
    .notification.success {
      border-left: 4px solid #4caf50;
    }
    .notification.error {
      border-left: 4px solid #f44336;
    }
    @keyframes slideIn {
      from {
        transform: translateX(400px);
        opacity: 0;
      }
      to {
        transform: translateX(0);
        opacity: 1;
      }
    }
    .spinner {
      display: inline-block;
      width: 16px;
      height: 16px;
      border: 2px solid rgba(255, 255, 255, 0.3);
      border-top-color: white;
      border-radius: 50%;
      animation: spin 0.8s linear infinite;
    }
    @keyframes spin {
      to { transform: rotate(360deg); }
    }
    .loading-overlay {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0, 0, 0, 0.3);
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 999;
    }
    .loading-spinner {
      width: 48px;
      height: 48px;
      border: 4px solid rgba(103, 80, 164, 0.3);
      border-top-color: var(--primary-color);
      border-radius: 50%;
      animation: spin 0.8s linear infinite;
    }
  `;

  /** Render the component UI. */
  render() {
    const breadcrumb = this.currentPath
      ? this.currentPath.split('/').filter(Boolean)
      : [];
    return html`
      ${this.isLoading ? html`
        <div class="loading-overlay">
          <div class="loading-spinner"></div>
        </div>
      ` : ''}
      
      ${this.notification ? html`
        <div class="notification ${this.notification.type}">
          ${this.notification.message}
        </div>
      ` : ''}
      
      <div class="header">
        <h1>SyncSpace</h1>
      </div>
      <nav>
        <span @click=${() => this.navigateToIndex(-1)}>root</span>
        ${breadcrumb.map(
          (seg, idx) => html` /
            <span @click=${() => this.navigateToIndex(idx)}>${seg}</span>`
        )}
      </nav>

      <div class="card">
        <h2>Search</h2>
        <input
          type="text"
          .value=${this.searchQuery}
          @input=${(e) => {
            this.searchQuery = e.target.value;
          }}
          placeholder="Search files"
        />
        <button class="button secondary" @click=${this.search}>Search</button>
        ${this.searchQuery && this.searchResults.length > 0
          ? html`<ul>
              ${this.searchResults.map(
                (res) => html`<li>
                  <span>${res.path} ${res.is_dir
                      ? '(dir)'
                      : `(${this.formatSize(res.size)})`}</span>
                </li>`
              )}
            </ul>`
          : this.searchQuery
          ? html`<p>No matches found.</p>`
          : ''}
      </div>

      <div class="card">
        <h2>Stats</h2>
        <p>
          ${this.stats.file_count} files,
          total size ${this.formatSize(this.stats.total_size)}
        </p>
        <button class="button secondary" @click=${this.loadStats}>Refresh</button>
      </div>

      <div class="card">
        <h2>Upload & Directory</h2>
        
        <div 
          class="drop-zone ${this.isDragging ? 'dragging' : ''}"
          @dragover=${this.handleDragOver}
          @dragleave=${this.handleDragLeave}
          @drop=${this.handleDrop}
        >
          <p>📁 Drag & Drop files here or use the button below</p>
        </div>
        
        <div style="margin-bottom: 12px;">
          <input id="fileInput" type="file" />
          <button class="button" @click=${this.uploadFile}>Upload</button>
        </div>
        <div>
          <input
            type="text"
            .value=${this.newDir}
            @input=${(e) => (this.newDir = e.target.value)}
            placeholder="Directory name"
          />
          <button class="button" @click=${this.createDir}>Create Directory</button>
        </div>
      </div>

      <div class="card">
        <h2>Entries</h2>
        ${this.files.length === 0
          ? html`<p>No entries in this folder.</p>`
          : html`<ul>
              ${this.files.map(
                (f) => html`<li>
                    <span
                      style="cursor: ${f.is_dir ? 'pointer' : 'default'}; color: ${
                        f.is_dir ? 'var(--primary-color)' : 'inherit'
                      }"
                      @click=${() => {
                        if (f.is_dir) this.navigate(f.name);
                      }}
                      >${f.name}
                      ${f.is_dir ? '(dir)' : `(${this.formatSize(f.size)})`}</span
                    >
                    <span class="actions">
                      ${!f.is_dir
                        ? html`<button
                            class="button secondary"
                            @click=${() => this.downloadEntry(f.name)}
                          >Download</button>`
                        : ''}
                      <button
                        class="button secondary"
                        @click=${() => this.renameEntry(f.name)}
                      >Rename</button>
                      <button
                        class="button secondary"
                        @click=${() => this.deleteEntry(f.name)}
                      >Delete</button>
                    </span>
                  </li>`
              )}
            </ul>`}
      </div>

      <div class="card">
        <h2>Peers</h2>
        ${this.peers.length === 0
          ? html`<p>No peers configured.</p>`
          : html`<ul>
              ${this.peers.map((p) => html`<li>${p.address}</li>`)}
            </ul>`}
        <div style="margin-top: 12px;">
          <input
            type="text"
            .value=${this.newPeer}
            @input=${(e) => (this.newPeer = e.target.value)}
            placeholder="Peer address"
          />
          <button class="button" @click=${this.addPeer}>Add Peer</button>
        </div>
      </div>
    `;
  }
}

customElements.define('my-app', MyApp);