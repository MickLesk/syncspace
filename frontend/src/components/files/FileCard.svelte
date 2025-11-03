<script>
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons.js";
  import api from "../../lib/api.js";
  import { onMount } from "svelte";

  let {
    file,
    selected = false,
    viewMode = "grid",
    onSelect,
    onOpen,
    onContextMenu,
  } = $props();

  let isFavorite = $state(false);
  let favoriteId = $state(null);
  let favoriteLoading = $state(false);

  // Check if file is favorited
  async function checkFavoriteStatus() {
    try {
      const data = await api.favorites.list();
      const favoritesList = Array.isArray(data) ? data : data.value || [];
      const favorite = favoritesList.find((fav) => fav.item_id === file.id);
      if (favorite) {
        isFavorite = true;
        favoriteId = favorite.id;
      }
    } catch (err) {
      console.error("[FileCard] Error checking favorite status:", err);
    }
  }

  // Toggle favorite status
  async function toggleFavorite(e) {
    e.stopPropagation(); // Prevent opening file
    favoriteLoading = true;

    try {
      if (isFavorite && favoriteId) {
        // Remove from favorites
        await api.favorites.remove(favoriteId);
        isFavorite = false;
        favoriteId = null;
      } else {
        // Add to favorites
        const data = await api.favorites.add(
          file.is_directory ? "folder" : "file",
          file.id
        );
        isFavorite = true;
        favoriteId = data.id;
      }
    } catch (err) {
      console.error("[FileCard] Error toggling favorite:", err);
    } finally {
      favoriteLoading = false;
    }
  }

  onMount(() => {
    checkFavoriteStatus();
  });

  // getFileIcon returns just icon name like "folder-fill", need to add "bi bi-"
  function getIconClass(file) {
    const iconName = getFileIcon(file.name, file.is_directory);
    return `bi bi-${iconName}`;
  }

  function formatFileSize(bytes) {
    if (!bytes) return "0 B";
    const units = ["B", "KB", "MB", "GB"];
    let size = bytes;
    let unitIndex = 0;
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  }

  function formatDate(dateString) {
    if (!dateString) return "Unknown";
    const date = new Date(dateString);
    return date.toLocaleDateString() + " " + date.toLocaleTimeString();
  }
</script>

{#if viewMode === "grid"}
  <div class="file-card-wrapper relative">
    <!-- Favorite Star Button -->
    <button
      type="button"
      class="favorite-btn absolute top-2 right-2 p-1.5 rounded-full transition-all z-10"
      class:opacity-0={!isFavorite}
      class:opacity-100={isFavorite}
      class:hover:opacity-100={true}
      onclick={toggleFavorite}
      disabled={favoriteLoading}
      title={isFavorite ? "Remove from favorites" : "Add to favorites"}
    >
      <i
        class="bi text-xl transition-colors"
        class:bi-star-fill={isFavorite}
        class:bi-star={!isFavorite}
        class:text-yellow-500={isFavorite}
        class:text-gray-400={!isFavorite}
        class:hover:text-yellow-500={!isFavorite}
      ></i>
    </button>

    <button
      type="button"
      class="file-card-grid p-4 bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-lg dark:shadow-gray-900/50 text-left w-full transition-all border-2"
      class:border-blue-500={selected}
      class:dark:border-blue-400={selected}
      class:border-transparent={!selected}
      class:hover:border-blue-300={!selected}
      class:dark:hover:border-blue-600={!selected}
      onclick={(e) => {
        if (e.shiftKey || e.ctrlKey) {
          onSelect?.(file);
        } else {
          onOpen?.(file);
        }
      }}
      oncontextmenu={(e) => {
        e.preventDefault();
        onContextMenu?.(file, e);
      }}
    >
      <div class="flex flex-col items-center gap-3 text-center">
        <i class="{getIconClass(file)} text-5xl {getFileIconColor(file.name)}"
        ></i>
        <div class="w-full min-w-0">
          <p
            class="font-medium truncate text-gray-900 dark:text-gray-100"
            title={file.name}
          >
            {file.name}
          </p>
          {#if !file.is_directory}
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {formatFileSize(file.size_bytes)}
            </p>
          {:else}
            <p class="text-sm text-gray-500 dark:text-gray-400">Folder</p>
          {/if}
          {#if file.modified_at}
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">
              {formatDate(file.modified_at)}
            </p>
          {/if}
        </div>
      </div>
    </button>
  </div>
{:else}
  <div class="file-card-wrapper relative">
    <button
      type="button"
      class="file-card-list p-3 bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-md text-left w-full transition-all border-2 flex items-center gap-4"
      class:border-blue-500={selected}
      class:dark:border-blue-400={selected}
      class:border-transparent={!selected}
      class:hover:border-blue-300={!selected}
      class:dark:hover:border-blue-600={!selected}
      onclick={(e) => {
        if (e.shiftKey || e.ctrlKey) {
          onSelect?.(file);
        } else {
          onOpen?.(file);
        }
      }}
      oncontextmenu={(e) => {
        e.preventDefault();
        onContextMenu?.(file, e);
      }}
    >
      <i
        class="{getIconClass(file)} text-3xl flex-shrink-0 {getFileIconColor(
          file.name
        )}"
      ></i>
      <div class="flex-1 min-w-0">
        <p class="font-medium truncate text-gray-900 dark:text-gray-100">
          {file.name}
        </p>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          {#if !file.is_directory}
            {formatFileSize(file.size_bytes)} • {formatDate(file.modified_at)}
          {:else}
            Folder • {formatDate(file.modified_at)}
          {/if}
        </p>
      </div>

      {#if selected}
        <i
          class="bi bi-check-circle-fill text-2xl text-blue-500 dark:text-blue-400"
        ></i>
      {/if}
    </button>

    <!-- Favorite Star Button -->
    <button
      type="button"
      class="favorite-btn-list absolute right-12 top-1/2 -translate-y-1/2 p-2 rounded-full transition-all z-10"
      onclick={toggleFavorite}
      disabled={favoriteLoading}
      title={isFavorite ? "Remove from favorites" : "Add to favorites"}
    >
      <i
        class="bi text-xl transition-colors"
        class:bi-star-fill={isFavorite}
        class:bi-star={!isFavorite}
        class:text-yellow-500={isFavorite}
        class:text-gray-400={!isFavorite}
        class:hover:text-yellow-500={!isFavorite}
      ></i>
    </button>
  </div>
{/if}

<style>
  .file-card-grid:hover,
  .file-card-list:hover {
    transform: translateY(-2px);
  }

  .file-card-grid,
  .file-card-list {
    cursor: pointer;
    user-select: none;
    position: relative;
  }

  .favorite-btn,
  .favorite-btn-list {
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(4px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    z-index: 10;
  }

  .favorite-btn:hover,
  .favorite-btn-list:hover {
    background: rgba(255, 255, 255, 1);
    border-color: rgba(0, 0, 0, 0.2);
    transform: scale(1.1);
  }

  .favorite-btn:disabled,
  .favorite-btn-list:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Show favorite button on hover for grid view */
  .file-card-grid:hover .favorite-btn:not(.opacity-100) {
    opacity: 1 !important;
  }

  :global(.dark) .favorite-btn,
  :global(.dark) .favorite-btn-list {
    background: rgba(31, 41, 55, 0.9);
    border-color: rgba(255, 255, 255, 0.1);
  }

  :global(.dark) .favorite-btn:hover,
  :global(.dark) .favorite-btn-list:hover {
    background: rgba(31, 41, 55, 1);
    border-color: rgba(255, 255, 255, 0.2);
  }
</style>
