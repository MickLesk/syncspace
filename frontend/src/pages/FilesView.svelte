<script>
  import { onMount } from 'svelte';
  import { files, currentPath } from '../stores/ui';
  import { auth } from '../stores/auth';
  
  let loading = true;
  
  onMount(() => {
    loadFiles();
  });
  
  async function loadFiles() {
    loading = true;
    try {
      const response = await fetch(`http://localhost:8080/api/files/${$currentPath}`, {
        headers: {
          'Authorization': `Bearer ${$auth.token}`
        }
      });
      
      if (response.ok) {
        const data = await response.json();
        files.set(data);
      }
    } catch (error) {
      console.error('Failed to load files:', error);
      files.set([]);
    }
    loading = false;
  }
  
  function getFileIcon(filename) {
    const ext = filename.split('.').pop().toLowerCase();
    const icons = {
      pdf: '📄', txt: '📝', doc: '📘', docx: '📘',
      jpg: '🖼️', jpeg: '🖼️', png: '🖼️', gif: '🖼️',
      mp4: '🎬', avi: '🎬', mov: '🎬',
      mp3: '🎵', wav: '🎵',
      zip: '📦', rar: '📦',
      js: '💻', html: '🌐', css: '🎨'
    };
    return icons[ext] || '📄';
  }
  
  function formatSize(bytes) {
    if (!bytes) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  }
</script>

<div class="files-view">
  <div class="page-header">
    <h2>📁 Files</h2>
    <div class="actions">
      <button class="btn btn-primary">⬆️ Upload</button>
      <button class="btn btn-secondary">➕ New Folder</button>
    </div>
  </div>
  
  <input type="text" class="search-box" placeholder="🔍 Search files..." />
  
  <div class="drop-zone">
    <p style="font-size: 48px; margin-bottom: 16px;">📤</p>
    <p style="font-size: 16px; font-weight: 600; margin-bottom: 8px;">Drag & Drop files here</p>
    <p style="font-size: 14px; color: var(--text-secondary);">or click Upload button above</p>
  </div>
  
  {#if loading}
    <div class="loading">⏳ Loading files...</div>
  {:else if $files.length === 0}
    <div class="empty-state">
      <p style="font-size: 64px; margin-bottom: 16px;">📂</p>
      <p style="font-size: 18px; font-weight: 600;">No files yet</p>
      <p style="color: var(--text-secondary);">Upload some files to get started!</p>
    </div>
  {:else}
    <div class="file-grid">
      {#each $files as file}
        <div class="file-card">
          <span class="file-icon">{getFileIcon(file.name)}</span>
          <div class="file-name">{file.name}</div>
          <div class="file-meta">{formatSize(file.size)}</div>
          <div class="file-actions">
            <button class="btn-icon">⬇️</button>
            <button class="btn-icon">🔗</button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .files-view {
    padding: 32px;
  }
  
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }
  
  h2 {
    font-size: 28px;
    font-weight: 600;
    color: var(--text);
    margin: 0;
  }
  
  .actions {
    display: flex;
    gap: 12px;
  }
  
  .btn {
    padding: 10px 16px;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.2s;
  }
  
  .btn-primary {
    background: var(--primary);
    color: white;
  }
  
  .btn-primary:hover {
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }
  
  .btn-secondary {
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
  }
  
  .btn-secondary:hover {
    background: var(--bg);
  }
  
  .search-box {
    width: 100%;
    max-width: 500px;
    padding: 12px 16px;
    border: 1px solid var(--border);
    border-radius: 24px;
    font-size: 14px;
    margin-bottom: 20px;
    background: var(--surface);
    color: var(--text);
  }
  
  .search-box:focus {
    outline: none;
    border-color: var(--primary);
  }
  
  .drop-zone {
    border: 2px dashed var(--primary);
    border-radius: 12px;
    padding: 40px;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s;
    background: rgba(103, 80, 164, 0.05);
    margin-bottom: 24px;
  }
  
  .drop-zone:hover {
    background: rgba(103, 80, 164, 0.1);
  }
  
  .loading, .empty-state {
    text-align: center;
    padding: 60px 20px;
    color: var(--text-secondary);
  }
  
  .file-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 20px;
  }
  
  .file-card {
    background: var(--surface);
    border-radius: 16px;
    padding: 20px;
    box-shadow: var(--shadow-sm);
    border: 1px solid var(--border);
    transition: all 0.2s;
    cursor: pointer;
  }
  
  .file-card:hover {
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
  }
  
  .file-icon {
    font-size: 48px;
    display: block;
    margin-bottom: 12px;
  }
  
  .file-name {
    font-weight: 500;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 4px;
  }
  
  .file-meta {
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 12px;
  }
  
  .file-actions {
    display: flex;
    gap: 8px;
  }
  
  .btn-icon {
    flex: 1;
    padding: 8px;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
    font-size: 16px;
    transition: all 0.2s;
  }
  
  .btn-icon:hover {
    background: var(--surface);
  }
</style>
