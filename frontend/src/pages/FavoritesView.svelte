<script>
  import { onMount } from "svelte";
  import { favorites } from "../stores/favorites";
  import { currentPath, currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import { success, error as errorToast } from "../stores/toast";
  import Icon from "../components/ui/Icon.svelte";
  import { getFileIcon } from "../utils/fileIcons";

  let favoriteFiles = [];
  let loading = false;

  onMount(async () => {
    await loadFavorites();
  });

  $: $favorites, loadFavorites(); // Reactively reload when store changes

  async function loadFavorites() {
    loading = true;

    try {
      const favs = favorites.getAll();
      console.log("[FavoritesView] Loaded favorites:", favs);

      // Convert favorites objects to display format
      favoriteFiles = favs.map((fav) => ({
        id: fav.id,
        itemId: fav.item_id,
        itemType: fav.item_type,
        name: fav.item_id.split("/").pop() || fav.item_id,
        fullPath: fav.item_id,
      }));

      console.log("[FavoritesView] Display files:", favoriteFiles);
    } catch (err) {
      console.error("Failed to load favorites:", err);
      errorToast("Failed to load favorites: " + err.message);
    }

    loading = false;
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
</script>

<div class="view-container">
  <div class="page-header">
    <h2>⭐ {t($currentLang, "favorites")}</h2>
    {#if favoriteFiles.length > 0}
      <p class="subtitle">
        {favoriteFiles.length}
        {favoriteFiles.length === 1 ? "Datei" : "Dateien"}
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
              <Icon
                name={getFileIcon(file.name, file.itemType === "folder")}
                size={32}
              />
            </div>
            <div class="file-details">
              <div class="file-name">{file.name}</div>
              <div class="file-path">{formatPath(file.fullPath)}</div>
            </div>
          </div>
          <button
            class="btn-remove"
            on:click|stopPropagation={() => removeFavorite(file)}
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
