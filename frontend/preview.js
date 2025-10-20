// ========================
// File Preview System
// ========================

class FilePreview {
  constructor() {
    this.overlay = null;
    this.container = null;
  }

  show(file) {
    this.createPreview(file);
  }

  createPreview(file) {
    // Create overlay
    this.overlay = document.createElement('div');
    this.overlay.className = 'preview-overlay';
    this.overlay.onclick = () => this.close();

    // Create container
    this.container = document.createElement('div');
    this.container.className = 'preview-container';
    this.container.onclick = (e) => e.stopPropagation();

    // Header
    const header = document.createElement('div');
    header.className = 'preview-header';
    header.innerHTML = `
      <div class="preview-title">
        <span class="preview-icon">${this.getFileIcon(file.name)}</span>
        <span class="preview-filename">${file.name}</span>
      </div>
      <div class="preview-actions">
        <button class="preview-btn" onclick="window.filePreview.download()">
          📥 ${t('files.download')}
        </button>
        <button class="preview-btn" onclick="window.filePreview.close()">
          ✕ ${t('common.close')}
        </button>
      </div>
    `;

    // Content area
    const content = document.createElement('div');
    content.className = 'preview-content';
    content.innerHTML = '<div class="preview-loader">⏳ Loading...</div>';

    this.container.appendChild(header);
    this.container.appendChild(content);
    this.overlay.appendChild(this.container);
    document.body.appendChild(this.overlay);

    // Animate in
    setTimeout(() => {
      this.overlay.classList.add('active');
      this.container.classList.add('active');
    }, 10);

    // Load content
    this.currentFile = file;
    this.loadContent(file, content);
  }

  async loadContent(file, contentElement) {
    const ext = file.name.split('.').pop().toLowerCase();
    const filePath = `${currentPath}/${file.name}`.replace(/^\/+/, '');

    try {
      if (this.isImage(ext)) {
        await this.showImage(filePath, contentElement);
      } else if (this.isText(ext)) {
        await this.showText(filePath, contentElement);
      } else if (ext === 'pdf') {
        this.showPDF(filePath, contentElement);
      } else if (this.isVideo(ext)) {
        this.showVideo(filePath, contentElement);
      } else if (this.isAudio(ext)) {
        this.showAudio(filePath, contentElement);
      } else {
        this.showUnsupported(file, contentElement);
      }
    } catch (error) {
      contentElement.innerHTML = `
        <div class="preview-error">
          <p>❌ ${t('notify.error.load')}</p>
          <p class="preview-error-detail">${error.message}</p>
        </div>
      `;
    }
  }

  async showImage(filePath, contentElement) {
    const url = `${API_URL}/file/${encodeURIComponent(filePath)}`;
    contentElement.innerHTML = `
      <div class="preview-image-container">
        <img src="${url}" alt="Preview" class="preview-image">
      </div>
    `;
  }

  async showText(filePath, contentElement) {
    const url = `${API_URL}/file/${encodeURIComponent(filePath)}`;
    const response = await fetch(url, {
      headers: { 'Authorization': `Bearer ${authToken}` }
    });
    const text = await response.text();
    
    contentElement.innerHTML = `
      <pre class="preview-text">${this.escapeHtml(text)}</pre>
    `;
  }

  showPDF(filePath, contentElement) {
    const url = `${API_URL}/file/${encodeURIComponent(filePath)}`;
    contentElement.innerHTML = `
      <iframe src="${url}" class="preview-pdf"></iframe>
    `;
  }

  showVideo(filePath, contentElement) {
    const url = `${API_URL}/file/${encodeURIComponent(filePath)}`;
    contentElement.innerHTML = `
      <video controls class="preview-video">
        <source src="${url}">
        Your browser does not support video playback.
      </video>
    `;
  }

  showAudio(filePath, contentElement) {
    const url = `${API_URL}/file/${encodeURIComponent(filePath)}`;
    contentElement.innerHTML = `
      <div class="preview-audio-container">
        <audio controls class="preview-audio">
          <source src="${url}">
          Your browser does not support audio playback.
        </audio>
      </div>
    `;
  }

  showUnsupported(file, contentElement) {
    contentElement.innerHTML = `
      <div class="preview-unsupported">
        <div class="preview-unsupported-icon">${this.getFileIcon(file.name)}</div>
        <h3>${t('files.preview.unsupported')}</h3>
        <p>${file.name}</p>
        <p class="preview-size">${formatSize(file.size)}</p>
        <button class="preview-download-btn" onclick="window.filePreview.download()">
          📥 ${t('files.download')}
        </button>
      </div>
    `;
  }

  download() {
    if (!this.currentFile) return;
    const filePath = `${currentPath}/${this.currentFile.name}`.replace(/^\/+/, '');
    const url = `${API_URL}/file/${encodeURIComponent(filePath)}`;
    
    const a = document.createElement('a');
    a.href = url;
    a.download = this.currentFile.name;
    a.click();
  }

  close() {
    if (this.overlay) {
      this.overlay.classList.remove('active');
      this.container.classList.remove('active');
      
      setTimeout(() => {
        if (this.overlay && this.overlay.parentNode) {
          this.overlay.parentNode.removeChild(this.overlay);
        }
        this.overlay = null;
        this.container = null;
        this.currentFile = null;
      }, 300);
    }
  }

  isImage(ext) {
    return ['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp', 'bmp', 'ico'].includes(ext);
  }

  isText(ext) {
    return ['txt', 'md', 'json', 'xml', 'html', 'css', 'js', 'ts', 'py', 'java', 
            'c', 'cpp', 'h', 'rs', 'go', 'rb', 'php', 'sh', 'yaml', 'yml', 'toml',
            'ini', 'log', 'csv', 'sql'].includes(ext);
  }

  isVideo(ext) {
    return ['mp4', 'webm', 'ogg', 'mov', 'avi', 'mkv'].includes(ext);
  }

  isAudio(ext) {
    return ['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac'].includes(ext);
  }

  getFileIcon(filename) {
    const ext = filename.split('.').pop().toLowerCase();
    if (this.isImage(ext)) return '🖼️';
    if (this.isText(ext)) return '📄';
    if (ext === 'pdf') return '📕';
    if (this.isVideo(ext)) return '🎬';
    if (this.isAudio(ext)) return '🎵';
    if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) return '📦';
    return '📎';
  }

  escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }
}

// Global instance
window.filePreview = new FilePreview();
