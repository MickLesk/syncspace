import { LitElement, html, css } from 'https://cdn.jsdelivr.net/gh/lit/dist@3/core/lit-all.min.js';

/**
 * Main component for the SyncSpace web UI. This component provides a
 * Material‐inspired interface to interact with the backend API: it lists
 * synchronised files and directories, allows uploads, renames, deletions,
 * directory creation, searching, peer management and displays basic
 * statistics. The design draws inspiration from Material 3 Expressive
 * guidelines with card layouts, vibrant colours and rounded corners.
 */
class MyApp extends LitElement {
  static properties = {
    files: { state: true },
    peers: { state: true },
    stats: { state: true },
    newDir: { state: true },
    newPeer: { state: true },
    currentPath: { state: true },
    searchQuery: { state: true },
    searchResults: { state: true },
  };

  constructor() {
    super();
    this.files = [];
    this.peers = [];
    this.stats = { file_count: 0, total_size: 0 };
    this.newDir = '';
    this.newPeer = '';
    this.currentPath = '';
    this.searchQuery = '';
    this.searchResults = [];
    this.ws = null;
  }

  /**
   * Lifecycle hook: invoked when the element is added to the DOM. Initialise
   * data and connect the WebSocket.
   */
  connectedCallback() {
    super.connectedCallback();
    this.loadFiles();
    this.loadPeers();
    this.loadStats();
    this.initWebSocket();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    if (this.ws) this.ws.close();
  }

  /** Fetch the list of entries for the current directory. */
  async loadFiles(sub = '') {
    try {
      const path = sub || this.currentPath || '';
      const res = await fetch(`http://localhost:8080/api/files/${encodeURIComponent(path)}`);
      if (!res.ok) throw new Error('Failed to load files');
      const data = await res.json();
      this.files = Array.isArray(data) ? data : [];
    } catch (err) {
      console.error(err);
    }
  }

  /** Fetch the list of peers. */
  async loadPeers() {
    try {
      const res = await fetch('http://localhost:8080/api/peers');
      if (!res.ok) throw new Error('Failed to load peers');
      const data = await res.json();
      this.peers = Array.isArray(data) ? data : [];
    } catch (err) {
      console.error(err);
    }
  }

  /** Fetch basic statistics about the sync directory. */
  async loadStats() {
    try {
      const res = await fetch('http://localhost:8080/api/stats');
      if (!res.ok) throw new Error('Failed to load stats');
      const data = await res.json();
      this.stats = data;
    } catch (err) {
      console.error(err);
    }
  }

  /** Set up a WebSocket connection for live file system updates. */
  initWebSocket() {
    try {
      this.ws = new WebSocket('ws://localhost:8080/api/ws');
      this.ws.onmessage = () => {
        // When a file change event is received, reload files and stats
        this.loadFiles();
        this.loadStats();
      };
    } catch (err) {
      console.error('WebSocket init error', err);
    }
  }

  /** Upload the selected file to the backend. */
  async uploadFile() {
    const input = this.renderRoot?.getElementById('fileInput');
    const file = input?.files?.[0];
    if (!file) return;
    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);
      const path = this.currentPath ? `${this.currentPath}/${file.name}` : file.name;
      const url = `http://localhost:8080/api/upload/${encodeURIComponent(path)}`;
      const res = await fetch(url, { method: 'POST', body: bytes });
      if (!res.ok) throw new Error('Upload failed');
      await this.loadFiles();
      await this.loadStats();
      input.value = '';
    } catch (err) {
      console.error(err);
    }
  }

  /** Delete a file or directory by name (relative to currentPath). */
  async deleteEntry(name) {
    try {
      const path = this.currentPath ? `${this.currentPath}/${name}` : name;
      const url = `http://localhost:8080/api/files/${encodeURIComponent(path)}`;
      const res = await fetch(url, { method: 'DELETE' });
      if (!res.ok) throw new Error('Delete failed');
      await this.loadFiles();
      await this.loadStats();
    } catch (err) {
      console.error(err);
    }
  }

  /** Download a file by opening a new window to the file endpoint. */
  downloadEntry(name) {
    const path = this.currentPath ? `${this.currentPath}/${name}` : name;
    const url = `http://localhost:8080/api/file/${encodeURIComponent(path)}`;
    window.open(url, '_blank');
  }

  /** Create a new directory relative to currentPath. */
  async createDir() {
    if (!this.newDir) return;
    try {
      const path = this.currentPath ? `${this.currentPath}/${this.newDir}` : this.newDir;
      const url = `http://localhost:8080/api/dirs/${encodeURIComponent(path)}`;
      const res = await fetch(url, { method: 'POST' });
      if (!res.ok) throw new Error('Failed to create directory');
      this.newDir = '';
      await this.loadFiles();
    } catch (err) {
      console.error(err);
    }
  }

  /** Add a new peer address. */
  async addPeer() {
    if (!this.newPeer) return;
    try {
      const peer = { id: crypto.randomUUID(), address: this.newPeer, last_seen: null };
      const res = await fetch('http://localhost:8080/api/peers', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(peer),
      });
      if (!res.ok) throw new Error('Failed to add peer');
      this.newPeer = '';
      await this.loadPeers();
    } catch (err) {
      console.error(err);
    }
  }

  /** Navigate into a directory. */
  navigate(dir) {
    this.currentPath = this.currentPath ? `${this.currentPath}/${dir}` : dir;
    this.searchQuery = '';
    this.searchResults = [];
    this.loadFiles();
  }

  /** Navigate using breadcrumb index. Pass -1 to go to root. */
  navigateToIndex(index) {
    if (index < 0) {
      this.currentPath = '';
    } else {
      const parts = this.currentPath.split('/').filter(Boolean);
      this.currentPath = parts.slice(0, index + 1).join('/');
    }
    this.searchQuery = '';
    this.searchResults = [];
    this.loadFiles();
  }

  /** Rename a file or directory. */
  async renameEntry(name) {
    const newName = prompt('Enter new name', name);
    if (!newName || newName === name) return;
    const oldPath = this.currentPath ? `${this.currentPath}/${name}` : name;
    const newPath = this.currentPath ? `${this.currentPath}/${newName}` : newName;
    try {
      const url = `http://localhost:8080/api/rename/${encodeURIComponent(oldPath)}`;
      const res = await fetch(url, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ new_path: newPath }),
      });
      if (!res.ok) throw new Error('Rename failed');
      await this.loadFiles();
    } catch (err) {
      console.error(err);
    }
  }

  /** Perform a search across all files/directories. */
  async search() {
    if (!this.searchQuery) {
      this.searchResults = [];
      return;
    }
    try {
      const url = `http://localhost:8080/api/search?q=${encodeURIComponent(this.searchQuery)}`;
      const res = await fetch(url);
      if (!res.ok) throw new Error('Search failed');
      const data = await res.json();
      this.searchResults = Array.isArray(data) ? data : [];
    } catch (err) {
      console.error(err);
    }
  }

  /** Format bytes into a human readable string. */
  formatSize(bytes) {
    const thresh = 1024;
    if (Math.abs(bytes) < thresh) return `${bytes} B`;
    const units = ['KB', 'MB', 'GB', 'TB'];
    let u = -1;
    let size = bytes;
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
  `;

  /** Render the component UI. */
  render() {
    const breadcrumb = this.currentPath
      ? this.currentPath.split('/').filter(Boolean)
      : [];
    return html`
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
          <button class="button" @click=${this.createDir}>Create</button>
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