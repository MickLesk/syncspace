<script>
  import { onMount } from "svelte";
  import { favorites } from "../stores/favorites";
  import { currentPath, currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import { success, error as errorToast } from "../stores/toast";
  import Icon from "../components/ui/Icon.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import Spinner from "../components/ui/Spinner.svelte";
  import EmptyState from "../components/ui/EmptyState.svelte";
  import { getFileIcon } from "../utils/fileIcons";
  import api from "../lib/api.js";

  let favoriteFiles = $state([]);
  let loading = $state(false);
  let error = $state(null);

  function getFileIconEmoji(mimeType) {
    if (!mimeType) return "📄";
    if (mimeType.startsWith("image/")) return "🖼️";
    if (mimeType.startsWith("video/")) return "🎬";
    if (mimeType.startsWith("audio/")) return "🎵";
    if (mimeType.includes("pdf")) return "📕";
    if (mimeType.includes("zip") || mimeType.includes("archive")) return "📦";
    if (mimeType.includes("text")) return "📝";
    if (mimeType.includes("word")) return "📘";
    if (mimeType.includes("excel") || mimeType.includes("spreadsheet"))
      return "📊";
    if (mimeType.includes("powerpoint") || mimeType.includes("presentation"))
      return "📽️";
    return "📄";
  }

  onMount(async () => {
    await loadFavorites();
  });

  // Reload when favorites store changes
  favorites.subscribe(() => {
    loadFavorites();
  });

  async function loadFavorites() {
    loading = true;
    error = null;

    try {
      // Load from backend API using proper API method
      const favs = await api.favorites.list();
      console.log("[FavoritesView] Loaded favorites from API:", favs);

      // Convert favorites objects to display format
      favoriteFiles = (favs || []).map((fav) => ({
        id: fav.id,
        itemId: fav.item_id,
        itemType: fav.item_type,
        name: fav.item_id.split("/").pop() || fav.item_id,
        fullPath: fav.item_id,
        createdAt: fav.created_at,
        // Add metadata if available
        size: fav.size_bytes,
        mimeType: fav.mime_type,
      }));

      console.log("[FavoritesView] Display files:", favoriteFiles);
    } catch (err) {
      console.error("Failed to load favorites:", err);
      error = "Failed to load favorites";
      errorToast("Failed to load favorites: " + err.message);
    } finally {
      loading = false;
    }
  }

  async function removeFavorite(fav) {
    try {
      await favorites.remove(fav.itemId);
      success(`${fav.name} aus Favoriten entfernt`);
      await loadFavorites();
    } catch (err) {
      console.error("Failed to remove favorite:", err);
      errorToast("Failed to remove favorite");
    }
  }

  function navigateToFile(filePath) {
    const parts = filePath.split("/");
    const fileName = parts.pop();
    const dirPath = "/" + (parts.length > 0 ? parts.join("/") + "/" : "");
    currentPath.set(dirPath);
  }

  function formatPath(path) {
    return path || "/";
  }

  function formatFileSize(bytes) {
    if (!bytes) return "";
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
    if (bytes < 1024 * 1024 * 1024)
      return (bytes / (1024 * 1024)).toFixed(1) + " MB";
    return (bytes / (1024 * 1024 * 1024)).toFixed(1) + " GB";
  }

  function formatDate(dateString) {
    if (!dateString) return "";
    const date = new Date(dateString);
    const now = new Date();
    const diff = now.getTime() - date.getTime();

    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    if (diff < 604800000) return `${Math.floor(diff / 86400000)}d ago`;
    return date.toLocaleDateString();
  }
</script>

<div class="view-container">
  <div class="view-header">
    <div class="header-content">
      <div class="title-section">
        <h1 class="view-title">
          <span class="icon">⭐</span>
          {t($currentLang, "favorites")}
        </h1>
        <p class="view-subtitle">
          {favoriteFiles.length}
          {favoriteFiles.length === 1 ? "item" : "items"}
        </p>
      </div>

      <div class="header-actions">
        <button
          class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={loadFavorites}
          disabled={loading}
        >
          <i class="bi bi-arrow-clockwise"></i>
          Refresh
        </button>
      </div>
    </div>
  </div>

  {#if loading}
    <div class="loading-state">
      <Spinner size="large" />
      <p>Loading favorites...</p>
    </div>
  {:else if error}
    <div class="error-state">
      <i
        class="bi bi-exclamation-triangle text-6xl text-red-600 dark:text-red-400"
      ></i>
      <p class="text-red-600 dark:text-red-400">{error}</p>
      <button
        class="px-3 py-1.5 text-sm bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors"
        onclick={loadFavorites}
      >
        Try Again
      </button>
    </div>
  {:else if favoriteFiles.length === 0}
    <EmptyState
      icon="⭐"
      title={t($currentLang, "noFavorites")}
      description={t($currentLang, "markFilesAsFavorite")}
    />
  {:else}
    <div class="favorites-grid">
      {#each favoriteFiles as file}
        <div class="favorite-card">
          <button
            class="card-content"
            onclick={() => navigateToFile(file.fullPath)}
            type="button"
          >
            <div class="file-icon-large">
              {getFileIconEmoji(file.mimeType)}
            </div>

            <div class="file-info">
              <div class="file-name" title={file.name}>{file.name}</div>
              <div class="file-meta">
                {#if file.size}
                  <span class="meta-item">
                    <i class="bi bi-file-earmark"></i>
                    {formatFileSize(file.size)}
                  </span>
                {/if}
                {#if file.createdAt}
                  <span class="meta-item">
                    <i class="bi bi-clock"></i>
                    {formatDate(file.createdAt)}
                  </span>
                {/if}
              </div>
              <div class="file-path">{formatPath(file.fullPath)}</div>
            </div>
          </button>

          <div
            class="flex items-center justify-center p-2 border-t border-gray-200 dark:border-gray-700"
          >
            <button
              class="px-2 py-1 text-sm hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded transition-colors"
              onclick={(e) => {
                e.stopPropagation();
                removeFavorite(file);
              }}
              title="Remove from favorites"
            >
              <i class="bi bi-star-fill text-amber-500 dark:text-amber-400"></i>
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .view-container {
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
    max-width: 1400px;
    margin: 0 auto;
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

  .loading-state,
  .error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    gap: 1rem;
    padding: 2rem;
  }

  .favorites-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1.5rem;
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
    width: 100%;
  }

  .favorite-card {
    display: flex;
    flex-direction: column;
    background: hsl(var(--b2));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: 12px;
    overflow: hidden;
    transition: all 0.2s;
    height: 100%;
  }

  .favorite-card:hover {
    background: hsl(var(--b3));
    border-color: hsl(var(--p) / 0.3);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px hsl(var(--bc) / 0.1);
  }

  .card-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 1.5rem;
    background: transparent;
    border: none;
    cursor: pointer;
    text-align: center;
    width: 100%;
  }

  .file-icon-large {
    font-size: 4rem;
    margin-bottom: 0.5rem;
  }

  .file-info {
    width: 100%;
  }

  .file-name {
    font-weight: 600;
    font-size: 0.95rem;
    margin-bottom: 0.5rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: hsl(var(--bc));
  }

  .file-meta {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
    margin-bottom: 0.5rem;
  }

  .meta-item {
    font-size: 0.75rem;
    color: hsl(var(--bc) / 0.6);
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .file-path {
    font-size: 0.7rem;
    color: hsl(var(--bc) / 0.5);
    font-family: "Roboto Mono", monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .card-actions {
    display: flex;
    justify-content: center;
    padding: 0.75rem;
    border-top: 1px solid hsl(var(--bc) / 0.1);
    background: hsl(var(--b1));
  }
</style>
