import { LitElement, html, css } from 'https://cdn.jsdelivr.net/gh/lit/dist@3/core/lit-all.min.js';

/**
 * Hauptkomponente der Weboberfläche. Sie zeigt die aktuelle Liste der
 * synchronisierten Dateien an und ermöglicht das Hochladen neuer Dateien.
 */
class MyApp extends LitElement {
  static properties = {
    files: { state: true },
    peers: { state: true },
    newDir: { state: true },
    newPeer: { state: true },
  };

  static styles = css`
    :host {
      font-family: sans-serif;
      padding: 1rem;
      display: block;
      max-width: 700px;
    }
    h1 {
      margin-top: 0;
    }
    ul {
      list-style: none;
      padding-left: 0;
    }
    li {
      padding: 0.25rem 0;
      border-bottom: 1px solid #eee;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    .actions button {
      margin-left: 0.5rem;
    }
    form {
      margin: 1rem 0;
    }
  `;

  constructor() {
    super();
    this.files = [];
    this.peers = [];
    this.newDir = '';
    this.newPeer = '';
    this.ws = null;
  }

  connectedCallback() {
    super.connectedCallback();
    this.loadFiles();
    this.loadPeers();
    this.initWebSocket();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    if (this.ws) {
      this.ws.close();
    }
  }

  /**
   * Fetch the current list of entries from the backend.
   */
  async loadFiles(sub = '') {
    try {
      const res = await fetch(`http://localhost:8080/api/files/${sub}`);
      if (!res.ok) throw new Error('Failed to load files');
      const data = await res.json();
      this.files = Array.isArray(data) ? data : [];
    } catch (err) {
      console.error(err);
    }
  }

  /**
   * Fetch the list of peers from the backend.
   */
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

  /**
   * Initialize a WebSocket connection to receive file system events.
   */
  initWebSocket() {
    try {
      this.ws = new WebSocket('ws://localhost:8080/api/ws');
      this.ws.onmessage = () => {
        // Whenever an event is received, reload the file list
        this.loadFiles();
      };
    } catch (err) {
      console.error('WebSocket init error', err);
    }
  }

  /**
   * Upload a selected file to the backend.
   */
  async uploadFile() {
    const input = this.renderRoot?.getElementById('fileInput');
    const file = input?.files?.[0];
    if (!file) return;
    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);
      const url = `http://localhost:8080/api/upload/${encodeURIComponent(file.name)}`;
      const res = await fetch(url, {
        method: 'POST',
        body: bytes,
      });
      if (!res.ok) {
        throw new Error('Upload failed');
      }
      await this.loadFiles();
      input.value = '';
    } catch (err) {
      console.error(err);
    }
  }

  /**
   * Delete a file or directory via the backend.
   */
  async deleteEntry(name) {
    try {
      const url = `http://localhost:8080/api/files/${encodeURIComponent(name)}`;
      const res = await fetch(url, { method: 'DELETE' });
      if (!res.ok) throw new Error('Delete failed');
      await this.loadFiles();
    } catch (err) {
      console.error(err);
    }
  }

  /**
   * Download a file by navigating the browser to the file URL. This will
   * trigger a download.
   */
  downloadEntry(name) {
    const url = `http://localhost:8080/api/file/${encodeURIComponent(name)}`;
    window.open(url, '_blank');
  }

  /**
   * Create a new directory via the backend.
   */
  async createDir() {
    if (!this.newDir) return;
    try {
      const url = `http://localhost:8080/api/dirs/${encodeURIComponent(this.newDir)}`;
      const res = await fetch(url, { method: 'POST' });
      if (!res.ok) throw new Error('Failed to create directory');
      this.newDir = '';
      await this.loadFiles();
    } catch (err) {
      console.error(err);
    }
  }

  /**
   * Add a new peer via the backend.
   */
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

  render() {
    return html`
      <h1>SyncSpace</h1>
      <section>
        <h2>Upload</h2>
        <input id="fileInput" type="file" />
        <button @click=${this.uploadFile}>Upload</button>
      </section>
      <section>
        <h2>Create Directory</h2>
        <input type="text" .value=${this.newDir} @input=${(e) => (this.newDir = e.target.value)} placeholder="Directory name" />
        <button @click=${this.createDir}>Create</button>
      </section>
      <section>
        <h2>Files</h2>
        ${this.files.length === 0
          ? html`<p>No entries in the synchronisation folder.</p>`
          : html`<ul>
              ${this.files.map(
                (f) => html`<li>
                  <span>${f.name} ${f.is_dir ? '(dir)' : `(${f.size} bytes)`}</span>
                  <span class="actions">
                    ${!f.is_dir
                      ? html`<button @click=${() => this.downloadEntry(f.name)}>Download</button>`
                      : ''}
                    <button @click=${() => this.deleteEntry(f.name)}>Delete</button>
                  </span>
                </li>`
              )}
            </ul>`}
      </section>
      <section>
        <h2>Peers</h2>
        ${this.peers.length === 0
          ? html`<p>No peers configured.</p>`
          : html`<ul>
              ${this.peers.map((p) => html`<li>${p.address}</li>`)}
            </ul>`}
        <input type="text" .value=${this.newPeer} @input=${(e) => (this.newPeer = e.target.value)} placeholder="Peer address" />
        <button @click=${this.addPeer}>Add Peer</button>
      </section>
    `;
  }
}

customElements.define('my-app', MyApp);
