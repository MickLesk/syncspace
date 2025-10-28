<script>
  import { onMount } from "svelte";
  import api from "../lib/api.js";

  let recentFiles = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let limit = $state(20);

  function formatFileSize(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatDate(dateString) {
    if (!dateString) return "";
    const date = new Date(dateString);
    return date.toLocaleDateString();
  }

  onMount(async () => {
    await loadRecentFiles();
  });

  async function loadRecentFiles() {
    try {
      loading = true;
      error = null;
      // Use generic fetch with auth token since this endpoint doesn't exist in api.files
      const token = localStorage.getItem('authToken');
      const response = await fetch(`http://localhost:8080/api/files/recent?limit=${limit}`, {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });
      
      if (response.ok) {
        const data = await response.json();
        recentFiles = data || [];
      } else {
        throw new Error(`API Error ${response.status}`);
      }
    } catch (err) {
      console.error("Failed to load recent files:", err);
      error = "Failed to load recent files";
      recentFiles = [];
    } finally {
      loading = false;
    }
  }

  function getFileIcon(mimeType) {
    if (!mimeType) return "📄";
    if (mimeType.startsWith("image/")) return "🖼️";
    if (mimeType.startsWith("video/")) return "🎬";
    if (mimeType.startsWith("audio/")) return "🎵";
    if (mimeType.includes("pdf")) return "📕";
    if (mimeType.includes("zip") || mimeType.includes("archive")) return "📦";
    if (mimeType.includes("text")) return "📝";
    return "📄";
  }

  function getAccessTypeBadge(accessType) {
    const badges = {
      view: { icon: "👁️", label: "Viewed", color: "badge-info" },
      edit: { icon: "✏️", label: "Edited", color: "badge-warning" },
      download: { icon: "⬇️", label: "Downloaded", color: "badge-success" },
      upload: { icon: "⬆️", label: "Uploaded", color: "badge-primary" },
    };
    return (
      badges[accessType] || {
        icon: "📁",
        label: "Accessed",
        color: "badge-ghost",
      }
    );
  }

  function formatLastAccessed(timestamp) {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();

    if (diff < 3600000) return `${Math.floor(diff / 60000)} minutes ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)} hours ago`;
    if (diff < 604800000) return `${Math.floor(diff / 86400000)} days ago`;
    return date.toLocaleDateString();
  }

  async function openFile(file) {
    // Navigate to file location or open preview
    console.log("Opening file:", file);
    // TODO: Implement file preview/navigation
  }
</script>

<div class="recent-files-view">
  <div class="view-header">
    <div class="header-content">
      <div class="title-section">
        <h1 class="view-title">
          <span class="icon">🕒</span>
          Recent Files
        </h1>
        <p class="view-subtitle">Files you've recently accessed</p>
      </div>

      <div class="header-actions">
        <select
          class="select select-bordered select-sm"
          bind:value={limit}
          onchange={loadRecentFiles}
        >
          <option value={10}>Last 10</option>
          <option value={20}>Last 20</option>
          <option value={50}>Last 50</option>
          <option value={100}>Last 100</option>
        </select>

        <button
          class="btn btn-sm btn-outline"
          onclick={loadRecentFiles}
          disabled={loading}
        >
          <i class="bi bi-arrow-clockwise"></i>
          Refresh
        </button>
      </div>
    </div>
  </div>

  <div class="view-content">
    {#if loading}
      <div class="loading-state">
        <span class="loading loading-spinner loading-lg"></span>
        <p>Loading recent files...</p>
      </div>
    {:else if error}
      <div class="error-state">
        <i class="bi bi-exclamation-triangle text-6xl text-error"></i>
        <p class="text-error">{error}</p>
        <button class="btn btn-sm btn-primary" onclick={loadRecentFiles}>
          Try Again
        </button>
      </div>
    {:else if recentFiles.length === 0}
      <div class="empty-state">
        <i class="bi bi-clock-history text-8xl opacity-20"></i>
        <h3>No recent files</h3>
        <p>Files you access will appear here</p>
      </div>
    {:else}
      <div class="files-grid">
        {#each recentFiles as file}
          <div
            class="file-card"
            onclick={() => openFile(file)}
            role="button"
            tabindex="0"
          >
            <div class="file-icon">
              {getFileIcon(file.mime_type)}
            </div>

            <div class="file-info">
              <div class="file-name" title={file.filename}>
                {file.filename}
              </div>

              <div class="file-meta">
                <span class="file-size">{formatFileSize(file.size_bytes)}</span>
                <span class="separator">•</span>
                <span class="file-date">
                  {formatLastAccessed(file.last_accessed_at)}
                </span>
              </div>

              <div class="file-stats">
                <span class="stat">
                  <i class="bi bi-eye"></i>
                  {file.access_count}
                  {file.access_count === 1 ? "time" : "times"}
                </span>

                {#if file.access_type}
                  {@const badge = getAccessTypeBadge(file.access_type)}
                  <span class="badge badge-sm {badge.color}">
                    {badge.icon}
                    {badge.label}
                  </span>
                {/if}
              </div>
            </div>

            <div class="file-actions">
              <button
                class="btn btn-ghost btn-xs"
                onclick={(e) => {
                  e.stopPropagation();
                  console.log("Quick preview");
                }}
              >
                <i class="bi bi-eye"></i>
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .recent-files-view {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: hsl(var(--b1));
  }

  .view-header {
    padding: 2rem;
    background: hsl(var(--b1));
    border-bottom: 1px solid hsl(var(--bc) / 0.1);
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 2rem;
  }

  .title-section {
    flex: 1;
  }

  .view-title {
    font-size: 2rem;
    font-weight: 700;
    margin: 0 0 0.5rem 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .view-title .icon {
    font-size: 2.5rem;
  }

  .view-subtitle {
    color: hsl(var(--bc) / 0.6);
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .view-content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }

  .files-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .file-card {
    display: flex;
    gap: 1rem;
    padding: 1.25rem;
    background: hsl(var(--b2));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .file-card:hover {
    background: hsl(var(--b3));
    border-color: hsl(var(--p) / 0.3);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px hsl(var(--bc) / 0.1);
  }

  .file-icon {
    font-size: 2.5rem;
    flex-shrink: 0;
  }

  .file-info {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-weight: 600;
    margin-bottom: 0.5rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-meta {
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.6);
    margin-bottom: 0.5rem;
  }

  .separator {
    margin: 0 0.5rem;
  }

  .file-stats {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .stat {
    font-size: 0.75rem;
    color: hsl(var(--bc) / 0.5);
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .file-actions {
    flex-shrink: 0;
  }

  .loading-state,
  .error-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    gap: 1rem;
  }

  .empty-state h3 {
    margin: 0.5rem 0 0 0;
    font-size: 1.5rem;
    font-weight: 600;
  }

  .empty-state p {
    color: hsl(var(--bc) / 0.5);
    margin: 0;
  }
</style>
