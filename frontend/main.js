import { LitElement, html, css } from 'https://cdn.jsdelivr.net/gh/lit/dist@3/core/lit-all.min.js';

/**
 * Hauptkomponente der Weboberfläche. Sie zeigt die aktuelle Liste der
 * synchronisierten Dateien an und ermöglicht das Hochladen neuer Dateien.
 */
class MyApp extends LitElement {
  static properties = {
    files: { state: true },
  };

  static styles = css`
    :host {
      font-family: sans-serif;
      padding: 1rem;
      display: block;
      max-width: 600px;
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
    }
  `;

  constructor() {
    super();
    this.files = [];
  }

  connectedCallback() {
    super.connectedCallback();
    this.loadFiles();
  }

  /**
   * Lädt die aktuelle Dateiliste vom Backend.
   */
  async loadFiles() {
    try {
      const res = await fetch('http://localhost:8080/api/files');
      if (!res.ok) throw new Error('Fehler beim Laden der Dateien');
      const data = await res.json();
      this.files = Array.isArray(data) ? data : [];
    } catch (err) {
      console.error(err);
    }
  }

  /**
   * Handler für den Upload‑Button. Liest die ausgewählte Datei, sendet sie
   * mittels POST an das Backend und lädt danach die Dateiliste neu.
   */
  async handleUpload() {
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
        headers: {
          // Kein content‑type setzen, damit der Browser einen passenden setzt
        },
      });
      if (!res.ok) {
        throw new Error('Upload fehlgeschlagen');
      }
      await this.loadFiles();
      // Leere Auswahl zurücksetzen
      input.value = '';
    } catch (err) {
      console.error(err);
    }
  }

  render() {
    return html`
      <h1>SyncSpace</h1>
      <div>
        <input id="fileInput" type="file" />
        <button @click=${this.handleUpload}>Hochladen</button>
      </div>
      <h2>Dateien</h2>
      ${this.files.length === 0
        ? html`<p>Keine Dateien im Synchronisationsordner.</p>`
        : html`<ul>
            ${this.files.map(
              (f) => html`<li><strong>${f.name}</strong> &mdash; ${f.is_dir ? 'Ordner' : `${f.size} Bytes`}</li>`
            )}
          </ul>`}
    `;
  }
}

customElements.define('my-app', MyApp);
