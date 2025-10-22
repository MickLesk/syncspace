<script>
  import { favorites } from "../stores/favorites";
  import { currentPath, currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import { success } from "../stores/toast";
  import Icon from "../components/ui/Icon.svelte";
  import { getFileIcon } from "../utils/fileIcons";
  import api from "../lib/api";

  let favoriteFiles = [];
  let loading = false;

  $: updateFavorites($favorites);

  async function updateFavorites(favSet) {
    if (favSet.size === 0) {
      favoriteFiles = [];
      return;
    }

    loading = true;
    const files = [];

    for (const path of favSet) {
      try {
        // Try to get file info (this would need backend support)
        // For now, just display the path
        files.push({
          path,
          name: path.split("/").pop() || path,
          fullPath: path,
        });
      } catch (e) {
        console.error("Failed to load favorite:", path, e);
      }
    }

    favoriteFiles = files;
    loading = false;
  }

  function removeFavorite(filePath) {
    favorites.remove(filePath);
    success(`Aus Favoriten entfernt`);
  }

  function navigateToFile(filePath) {
    const parts = filePath.split("/");
    const fileName = parts.pop();
    const dirPath = parts.join("/");
    currentPath.set(dirPath);
  }

  function formatPath(path) {
    return path || "/";
  }
</script>

<div class="view-container">
  <div class="page-header">
    <h2>⭐ {t($currentLang, "favorites")}</h2>
    {#if $favorites.size > 0}
      <p class="subtitle">
        {$favorites.size}
        {$favorites.size === 1 ? "Datei" : "Dateien"}
      </p>
    {/if}
  </div>

  {#if loading}
    <div class="loading-state">
      <Icon name="arrow-clockwise" size={32} />
      <p>Lade Favoriten...</p>
    </div>
  {:else if favoriteFiles.length === 0}
    <div class="empty-state">
      <div class="empty-icon">⭐</div>
      <h3>{t($currentLang, "noFavorites")}</h3>
      <p>{t($currentLang, "markFilesAsFavorite")}</p>
    </div>
  {:else}
    <div class="favorites-list">
      {#each favoriteFiles as file}
        <div class="favorite-item">
          <div class="file-info" on:click={() => navigateToFile(file.fullPath)}>
            <div class="file-icon">
              <Icon name={getFileIcon(file.name, false)} size={32} />
            </div>
            <div class="file-details">
              <div class="file-name">{file.name}</div>
              <div class="file-path">{formatPath(file.fullPath)}</div>
            </div>
          </div>
          <button
            class="btn-remove"
            on:click|stopPropagation={() => removeFavorite(file.fullPath)}
            title="Aus Favoriten entfernen"
          >
            <Icon name="x-lg" size={20} />
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .view-container {
    padding: 24px;
    max-width: 1400px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 32px;
  }

  h2 {
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 8px 0;
  }

  .subtitle {
    font-size: 14px;
    color: var(--md-sys-color-on-surface-variant);
    margin: 0;
  }

  .loading-state {
    text-align: center;
    padding: 80px 20px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .loading-state :global(svg) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .empty-state {
    text-align: center;
    padding: 80px 20px;
  }

  .empty-icon {
    font-size: 80px;
    margin-bottom: 24px;
    opacity: 0.5;
  }

  h3 {
    font-size: 24px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 8px;
  }

  p {
    font-size: 16px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .favorites-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .favorite-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    background: var(--md-sys-color-surface-container);
    border-radius: 16px;
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .favorite-item:hover {
    background: var(--md-sys-color-surface-container-high);
    transform: translateX(4px);
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 16px;
    flex: 1;
  }

  .file-icon {
    flex-shrink: 0;
  }

  .file-details {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-size: 16px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-path {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    font-family: "Roboto Mono", monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .btn-remove {
    flex-shrink: 0;
    padding: 8px;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: var(--md-sys-color-error);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-remove:hover {
    background: var(--md-sys-color-error-container);
  }
</style>
