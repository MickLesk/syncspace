<script>
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons.js";
  import api from "../../lib/api.js";
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    file,
    selected = false,
    selectionMode = false,
    viewMode = "grid",
    onSelect,
    onOpen,
    onContextMenu,
    onDragStart,
    onDragEnd,
    onDrop,
  } = $props();

  let isFavorite = $state(false);
  let favoriteId = $state(null);
  let favoriteLoading = $state(false);
  let folderColor = $state(null);

  // Drag & Drop state
  let isDragging = $state(false);
  let isDropTarget = $state(false);

  // Get folder color from backend
  async function loadFolderColor() {
    if (!file.is_directory) return;

    // If color already in file object (from backend), use it
    if (file.folder_color) {
      folderColor = file.folder_color;
      return;
    }

    // Skip system folders
    const folderName = file.name || "";
    const systemFolders = ["versions", "search_index", "syncspace.db", ".git"];
    if (
      systemFolders.some(
        (sys) => folderName.toLowerCase() === sys.toLowerCase()
      )
    ) {
      return; // Don't load colors for system folders
    }

    // Otherwise fetch from API
    try {
      const response = await api.folderColors.get(file.file_path || file.name);
      folderColor = response.color;
      console.log("[FileCard] Loaded folder color from backend:", folderColor);
    } catch (err) {
      // 404 is ok - folder has no color set
      if (err.message && !err.message.includes("404")) {
        console.error("[FileCard] Error loading folder color:", err);
      }
    }
  }

  // Check if file is favorited
  async function checkFavoriteStatus() {
    try {
      const data = await api.favorites.list();
      const favoritesList = Array.isArray(data) ? data : data.value || [];
      // Use path as the consistent identifier for both files and folders
      const itemId = file.path || file.file_path || file.name;
      const favorite = favoritesList.find((fav) => fav.item_id === itemId);
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
      // Use path as the consistent identifier for both files and folders
      const itemId = file.path || file.file_path || file.name;

      if (isFavorite && favoriteId) {
        // Remove from favorites
        await api.favorites.remove(favoriteId);
        isFavorite = false;
        favoriteId = null;
        console.log("[FileCard] Removed from favorites:", itemId);
      } else {
        // Add to favorites
        const data = await api.favorites.add(
          itemId, // Use path/file_path/name as itemId
          file.is_directory ? "folder" : "file" // itemType second
        );
        isFavorite = true;
        favoriteId = data.id;
        console.log("[FileCard] Added to favorites:", itemId);
      }
    } catch (err) {
      console.error("[FileCard] Error toggling favorite:", err);
    } finally {
      favoriteLoading = false;
    }
  }

  onMount(() => {
    checkFavoriteStatus();
    loadFolderColor();
  });

  // Check if file is an image that can be thumbnailed
  function isImageFile(fileName) {
    const ext = (fileName || "").split(".").pop()?.toLowerCase();
    return ["jpg", "jpeg", "png", "gif", "webp", "bmp", "svg"].includes(ext);
  }

  // Generate thumbnail URL for images
  function getThumbnailUrl(file) {
    if (!file || file.is_directory || !isImageFile(file.name)) return null;
    const filePath = file.file_path || file.path || file.name;
    const token = localStorage.getItem("authToken");
    // Use download endpoint with token
    return `http://localhost:8080/api/download/${encodeURIComponent(filePath)}?token=${token}`;
  }

  // Track thumbnail loading state
  let thumbnailLoaded = $state(false);
  let thumbnailError = $state(false);

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
    if (!dateString) return tr("unknown");
    const date = new Date(dateString);
    return date.toLocaleDateString() + " " + date.toLocaleTimeString();
  }
</script>

{#if viewMode === "grid"}
  <div class="file-card-wrapper relative hover-lift">
    <!-- Favorite Star Button -->
    <button
      type="button"
      class="favorite-btn absolute top-2 right-2 p-1.5 rounded-full transition-all z-10"
      class:opacity-0={!isFavorite}
      class:opacity-100={isFavorite}
      class:hover:opacity-100={true}
      onclick={toggleFavorite}
      disabled={favoriteLoading}
      title={isFavorite ? tr("removeFromFavorites") : tr("addToFavorites")}
    >
      <i
        class="bi text-xl transition-colors"
        class:bi-star-fill={isFavorite}
        class:bi-star={!isFavorite}
        class:text-amber-400={isFavorite}
        class:text-gray-400={!isFavorite}
        class:hover:text-amber-400={!isFavorite}
      ></i>
    </button>

    <button
      type="button"
      draggable="true"
      data-file-name={file.name}
      class="file-card-grid p-5 bg-white dark:bg-gray-800 rounded-2xl shadow-lg hover:shadow-xl dark:shadow-gray-900/50 text-left w-full transition-all border-2 {selected
        ? 'border-green-500 dark:border-green-500 bg-green-50 dark:bg-green-900/20'
        : isDropTarget
          ? 'border-green-400 bg-green-50'
          : 'border-gray-100 dark:border-gray-700 hover:border-green-300 dark:hover:border-green-600'} {isDragging
        ? 'opacity-50'
        : ''}"
      ondragstart={(e) => {
        isDragging = true;
        e.dataTransfer.effectAllowed = "move";
        e.dataTransfer.setData("text/plain", JSON.stringify(file));
        onDragStart?.(file, e);
      }}
      ondragend={(e) => {
        isDragging = false;
        onDragEnd?.(file, e);
      }}
      ondragover={(e) => {
        // Only allow drop on folders
        if (file.is_directory) {
          e.preventDefault();
          e.dataTransfer.dropEffect = "move";
          isDropTarget = true;
        }
      }}
      ondragleave={(e) => {
        isDropTarget = false;
      }}
      ondrop={(e) => {
        e.preventDefault();
        isDropTarget = false;

        // Only allow drop on folders
        if (!file.is_directory) return;

        try {
          const draggedFileData = e.dataTransfer.getData("text/plain");
          const draggedFile = JSON.parse(draggedFileData);

          // Don't allow dropping a folder onto itself
          if (draggedFile.path === file.path) return;

          onDrop?.(draggedFile, file, e);
        } catch (err) {
          console.error("[FileCard] Error handling drop:", err);
        }
      }}
      onclick={(e) => {
        if (selectionMode || e.shiftKey || e.ctrlKey) {
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
      <!-- Selection Checkbox when in selection mode -->
      {#if selectionMode}
        <div class="absolute top-3 left-3 z-10">
          <div
            class="w-6 h-6 rounded-lg border-2 flex items-center justify-center transition-all {selected
              ? 'bg-green-500 border-green-500'
              : 'bg-white/80 dark:bg-gray-800/80 border-gray-300 dark:border-gray-600 hover:border-green-400'}"
          >
            {#if selected}
              <i
                class="bi bi-check text-white text-sm font-bold"
                aria-hidden="true"
              ></i>
            {/if}
          </div>
        </div>
      {/if}
      <div class="flex flex-col items-center gap-3 text-center">
        <div class="relative">
          {#if isImageFile(file.name) && !file.is_directory && !thumbnailError}
            <!-- Image Thumbnail -->
            <div
              class="w-16 h-16 rounded-lg overflow-hidden bg-gray-100 dark:bg-gray-700 flex items-center justify-center"
            >
              {#if !thumbnailLoaded}
                <div
                  class="animate-pulse w-full h-full bg-gray-200 dark:bg-gray-600"
                ></div>
              {/if}
              <img
                src={getThumbnailUrl(file)}
                alt={file.name}
                class="w-full h-full object-cover {thumbnailLoaded
                  ? ''
                  : 'hidden'}"
                onload={() => (thumbnailLoaded = true)}
                onerror={() => (thumbnailError = true)}
              />
            </div>
          {:else}
            <!-- Default Icon -->
            <i
              class="{getIconClass(file)} text-5xl {file.is_directory &&
              folderColor
                ? ''
                : getFileIconColor(file.name)}"
              style={file.is_directory && folderColor
                ? `color: ${folderColor};`
                : ""}
            ></i>
          {/if}
          {#if file.is_directory && folderColor}
            <div
              class="absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-white dark:border-gray-800"
              style="background-color: {folderColor};"
            ></div>
          {/if}
        </div>
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
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("folder")}
            </p>
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
  <div class="file-card-wrapper relative hover-lift">
    <button
      type="button"
      draggable="true"
      class="file-card-list p-4 bg-white dark:bg-gray-800 rounded-xl shadow-md hover:shadow-lg text-left w-full transition-all border-2 flex items-center gap-4 {selected
        ? 'border-green-500 dark:border-green-500 bg-green-50 dark:bg-green-900/20'
        : isDropTarget
          ? 'border-green-400 bg-green-50'
          : 'border-gray-100 dark:border-gray-700 hover:border-green-300 dark:hover:border-green-600'} {isDragging
        ? 'opacity-50'
        : ''}"
      ondragstart={(e) => {
        isDragging = true;
        e.dataTransfer.effectAllowed = "move";
        e.dataTransfer.setData("text/plain", JSON.stringify(file));
        onDragStart?.(file, e);
      }}
      ondragend={(e) => {
        isDragging = false;
        onDragEnd?.(file, e);
      }}
      ondragover={(e) => {
        if (file.is_directory) {
          e.preventDefault();
          e.dataTransfer.dropEffect = "move";
          isDropTarget = true;
        }
      }}
      ondragleave={(e) => {
        isDropTarget = false;
      }}
      ondrop={(e) => {
        e.preventDefault();
        isDropTarget = false;

        if (!file.is_directory) return;

        try {
          const draggedFileData = e.dataTransfer.getData("text/plain");
          const draggedFile = JSON.parse(draggedFileData);

          if (draggedFile.path === file.path) return;

          onDrop?.(draggedFile, file, e);
        } catch (err) {
          console.error("[FileCard] Error handling drop:", err);
        }
      }}
      onclick={(e) => {
        if (selectionMode || e.shiftKey || e.ctrlKey) {
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
      <!-- Selection Checkbox when in selection mode -->
      {#if selectionMode}
        <div class="flex-shrink-0">
          <div
            class="w-6 h-6 rounded-lg border-2 flex items-center justify-center transition-all {selected
              ? 'bg-green-500 border-green-500'
              : 'bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 hover:border-green-400'}"
          >
            {#if selected}
              <i
                class="bi bi-check text-white text-sm font-bold"
                aria-hidden="true"
              ></i>
            {/if}
          </div>
        </div>
      {/if}
      <div class="relative flex-shrink-0">
        {#if isImageFile(file.name) && !file.is_directory && !thumbnailError}
          <!-- Image Thumbnail for List View -->
          <div
            class="w-10 h-10 rounded-lg overflow-hidden bg-gray-100 dark:bg-gray-700 flex items-center justify-center"
          >
            {#if !thumbnailLoaded}
              <div
                class="animate-pulse w-full h-full bg-gray-200 dark:bg-gray-600"
              ></div>
            {/if}
            <img
              src={getThumbnailUrl(file)}
              alt={file.name}
              class="w-full h-full object-cover {thumbnailLoaded
                ? ''
                : 'hidden'}"
              onload={() => (thumbnailLoaded = true)}
              onerror={() => (thumbnailError = true)}
            />
          </div>
        {:else}
          <!-- Default Icon for List View -->
          <i
            class="{getIconClass(file)} text-3xl {file.is_directory &&
            folderColor
              ? ''
              : getFileIconColor(file.name)}"
            style={file.is_directory && folderColor
              ? `color: ${folderColor};`
              : ""}
          ></i>
        {/if}
        {#if file.is_directory && folderColor}
          <div
            class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-white dark:border-gray-800"
            style="background-color: {folderColor};"
          ></div>
        {/if}
      </div>
      <div class="flex-1 min-w-0">
        <p class="font-medium truncate text-gray-900 dark:text-gray-100">
          {file.name}
        </p>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          {#if !file.is_directory}
            {formatFileSize(file.size_bytes)} • {formatDate(file.modified_at)}
          {:else}
            {tr("folder")} • {formatDate(file.modified_at)}
          {/if}
        </p>
      </div>

      {#if selected}
        <i
          class="bi bi-check-circle-fill text-2xl text-green-500 dark:text-green-400"
        ></i>
      {/if}
    </button>

    <!-- Favorite Star Button -->
    <button
      type="button"
      class="favorite-btn-list absolute right-12 top-1/2 -translate-y-1/2 p-2 rounded-full transition-all z-10"
      onclick={toggleFavorite}
      disabled={favoriteLoading}
      title={isFavorite ? tr("removeFromFavorites") : tr("addToFavorites")}
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
  .hover-lift {
    transition:
      transform 0.2s ease,
      box-shadow 0.2s ease;
  }

  .hover-lift:hover {
    transform: translateY(-4px);
  }

  .file-card-grid,
  .file-card-list {
    cursor: pointer;
    user-select: none;
    position: relative;
    transition: all 0.2s ease;
  }

  .favorite-btn,
  .favorite-btn-list {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(8px);
    border: 1px solid rgba(0, 0, 0, 0.08);
    z-index: 10;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .favorite-btn:hover,
  .favorite-btn-list:hover {
    background: rgba(255, 255, 255, 1);
    border-color: rgba(0, 0, 0, 0.15);
    transform: scale(1.15);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
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
    background: rgba(31, 41, 55, 0.95);
    border-color: rgba(255, 255, 255, 0.1);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  :global(.dark) .favorite-btn:hover,
  :global(.dark) .favorite-btn-list:hover {
    background: rgba(31, 41, 55, 1);
    border-color: rgba(255, 255, 255, 0.2);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }
</style>
