<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import { favorites } from "../../stores/favorites";
  import { currentPath } from "../../stores/ui.js";
  import { success, error } from "../../stores/toast";
  import { modals } from "../../stores/modals";
  import api from "../../lib/api.js";
  import { getContextMenuItems } from "../../lib/contextMenuActions.js";
  import ContextMenu from "../../components/ui/ContextMenu.svelte";
  import Loading from "../../components/ui/Loading.svelte";

  let favoriteFiles = $state([]);
  let loading = $state(false);
  let errorMsg = $state(null);

  // Context menu state
  let contextMenu = $state(null);
  let contextMenuVisible = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuItems = $state([]);

  function getFileIconEmoji(itemType, mimeType) {
    // Check if it's a folder first
    if (itemType === "folder") return "📁";

    // Then check mime type for files
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
    errorMsg = null;

    try {
      const favs = await api.favorites.list();
      console.log("[FavoritesView] Loaded favorites from API:", favs);

      favoriteFiles = (favs || []).map((fav) => ({
        id: fav.id,
        itemId: fav.item_id,
        itemType: fav.item_type,
        name: fav.item_id.split("/").pop() || fav.item_id,
        fullPath: fav.item_id,
        createdAt: fav.created_at,
        size: fav.size_bytes,
        mimeType: fav.mime_type,
      }));

      console.log("[FavoritesView] Display files:", favoriteFiles);
    } catch (err) {
      console.error("Failed to load favorites:", err);
      errorMsg = tr("failedToLoadFavorites");
      error(tr("failedToLoadFavorites") + ": " + err.message);
    } finally {
      loading = false;
    }
  }

  async function removeFavorite(fav, e) {
    e?.stopPropagation();
    try {
      await favorites.remove(fav.itemId);
      success(`${fav.name} ${tr("removedFromFavorites").toLowerCase()}`);
      await loadFavorites();
    } catch (err) {
      console.error("Failed to remove favorite:", err);
      error(tr("failedToRemoveFavorite"));
    }
  }

  async function handleItemClick(file) {
    if (file.itemType === "folder") {
      // Navigate to folder in Files view
      window.location.hash = "#/files";
      setTimeout(() => {
        currentPath.set(file.fullPath);
      }, 100);
    } else {
      // For files, show preview or download
      try {
        // Try to open preview modal if available
        if (modals.openPreview) {
          const fileObj = {
            name: file.name,
            path: file.fullPath,
            file_path: file.fullPath,
            mime_type: file.mimeType,
            size_bytes: file.size,
            is_directory: false,
          };
          modals.openPreview(fileObj);
        } else {
          // Fallback: download file
          await downloadFile(file);
        }
      } catch (err) {
        console.error("Failed to open file:", err);
        error(tr("failedToOpenFile"));
      }
    }
  }

  async function downloadFile(file) {
    try {
      const response = await fetch(
        `http://localhost:8080/api/files${file.fullPath}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (!response.ok) throw new Error(tr("downloadFailed"));

      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = file.name;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);

      success(`${file.name} ${tr("downloaded").toLowerCase()}`);
    } catch (err) {
      console.error("Download failed:", err);
      error(tr("downloadFailed"));
    }
  }

  function handleContextMenu(file, event) {
    event.preventDefault();
    event.stopPropagation();

    // Convert favorite to file object format for context menu
    const fileObj = {
      name: file.name,
      path: file.fullPath,
      file_path: file.fullPath,
      is_directory: file.itemType === "folder",
      mime_type: file.mimeType,
      size_bytes: file.size,
      type: file.itemType === "folder" ? "folder" : "file",
    };

    contextMenuItems = getContextMenuItems(fileObj, {
      canEdit: true,
      canDelete: true,
      canShare: true,
    });

    contextMenuX = event.clientX;
    contextMenuY = event.clientY;
    contextMenuVisible = true;
  }

  function closeContextMenu() {
    contextMenuVisible = false;
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

{#if loading}
  <!-- Skeleton Loading State -->
  <div
    class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
  >
    <div class="max-w-7xl mx-auto space-y-6">
      <!-- Header Skeleton -->
      <div class="skeleton h-24 w-full rounded-2xl"></div>

      <!-- Grid Skeleton -->
      <div
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
      >
        {#each Array(8) as _}
          <div class="skeleton h-64 w-full rounded-2xl"></div>
        {/each}
      </div>
    </div>
  </div>
{:else}
  <!-- Main Container -->
  <div
    class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
  >
    <!-- Animated Background Blobs -->
    <div class="blob blob-1"></div>
    <div class="blob blob-2"></div>
    <div class="blob blob-3"></div>

    <!-- Page Header -->
    <div class="max-w-7xl mx-auto mb-6">
      <div class="glass-card p-6 animate-slide-down">
        <div class="flex flex-wrap justify-between gap-4">
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {favoriteFiles.length}
              {favoriteFiles.length === 1 ? "Favorite" : "Favorites"}
            </p>
          </div>

          <button
            onclick={loadFavorites}
            disabled={loading}
            class="px-4 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 font-semibold rounded-xl hover:border-gray-300 dark:hover:border-gray-600 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            <i
              class="bi bi-arrow-clockwise {loading ? 'animate-spin' : ''}"
              aria-hidden="true"
            ></i>
            <span>Refresh</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Content -->
    <div class="max-w-7xl mx-auto">
      {#if errorMsg}
        <!-- Error State -->
        <div class="glass-card p-12 text-center animate-slide-up">
          <div class="mb-6">
            <i
              class="bi bi-exclamation-triangle text-6xl text-red-500/30"
              aria-hidden="true"
            ></i>
          </div>
          <h3 class="text-2xl font-bold text-red-600 dark:text-red-400 mb-3">
            {errorMsg}
          </h3>
          <button
            onclick={loadFavorites}
            class="px-6 py-3 bg-green-600 text-white font-semibold rounded-xl hover:bg-green-700 shadow-lg hover:shadow-xl transition-all flex items-center gap-2 mx-auto"
          >
            <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
            {tr("common.tryAgain")}
          </button>
        </div>
      {:else if favoriteFiles.length === 0}
        <!-- Empty State -->
        <div class="glass-card p-12 text-center animate-slide-up">
          <div class="mb-6">
            <i class="bi bi-star text-6xl text-yellow-500/30" aria-hidden="true"
            ></i>
          </div>
          <h3 class="text-2xl font-bold mb-3">{tr("noFavoritesYet")}</h3>
          <p class="text-gray-600 dark:text-gray-400">
            {tr("markFilesAsFavorites")}
          </p>
        </div>
      {:else}
        <!-- Favorites Grid -->
        <div
          class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
        >
          {#each favoriteFiles as file, i}
            <div
              class="glass-card p-6 cursor-pointer hover:scale-105 transition-all duration-200 animate-slide-up relative group"
              style="animation-delay: {i * 30}ms"
              role="button"
              tabindex="0"
              onclick={() => handleItemClick(file)}
              oncontextmenu={(e) => handleContextMenu(file, e)}
              onkeydown={(e) =>
                (e.key === "Enter" || e.key === " ") && handleItemClick(file)}
            >
              <!-- Quick Action Buttons (visible on hover) -->
              <div
                class="absolute top-2 right-2 flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity z-10"
              >
                {#if file.itemType !== "folder"}
                  <button
                    type="button"
                    onclick={(e) => {
                      e.stopPropagation();
                      downloadFile(file);
                    }}
                    class="p-2 bg-white dark:bg-gray-800 rounded-lg shadow-lg hover:bg-green-50 dark:hover:bg-green-900/30 transition-colors"
                    title="Download"
                  >
                    <i
                      class="bi bi-download text-green-600 dark:text-green-400"
                      aria-hidden="true"
                    ></i>
                  </button>
                {/if}
                <button
                  type="button"
                  onclick={(e) => removeFavorite(file, e)}
                  class="p-2 bg-white dark:bg-gray-800 rounded-lg shadow-lg hover:bg-red-50 dark:hover:bg-red-900/30 transition-colors"
                  title="Remove from Favorites"
                >
                  <i class="bi bi-star-fill text-yellow-500" aria-hidden="true"
                  ></i>
                </button>
              </div>
              <!-- File Icon -->
              <div class="text-center mb-4">
                <div class="text-6xl mb-3">
                  {getFileIconEmoji(file.itemType, file.mimeType)}
                </div>
                <h3 class="font-bold text-lg mb-2 truncate" title={file.name}>
                  {file.name}
                </h3>
              </div>

              <!-- File Metadata -->
              <div
                class="flex flex-wrap gap-3 justify-center text-xs text-gray-600 dark:text-gray-400 mb-3"
              >
                {#if file.itemType === "folder"}
                  <span
                    class="flex items-center gap-1 px-2 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 rounded-lg"
                  >
                    <i class="bi bi-folder" aria-hidden="true"></i>
                    Ordner
                  </span>
                {:else if file.size}
                  <span
                    class="flex items-center gap-1 px-2 py-1 bg-gray-100 dark:bg-gray-800 rounded-lg"
                  >
                    <i class="bi bi-file-earmark" aria-hidden="true"></i>
                    {formatFileSize(file.size)}
                  </span>
                {/if}
                {#if file.createdAt}
                  <span
                    class="flex items-center gap-1 px-2 py-1 bg-gray-100 dark:bg-gray-800 rounded-lg"
                  >
                    <i class="bi bi-clock" aria-hidden="true"></i>
                    {formatDate(file.createdAt)}
                  </span>
                {/if}
              </div>

              <!-- File Path -->
              <div
                class="text-xs font-mono text-gray-500 dark:text-gray-500 mb-4 truncate text-center px-2"
              >
                {formatPath(file.fullPath)}
              </div>

              <!-- Remove Button -->
              <button
                onclick={(e) => removeFavorite(file, e)}
                class="w-full px-4 py-2 bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-400 font-semibold rounded-xl hover:bg-yellow-200 dark:hover:bg-yellow-900/50 transition-all flex items-center justify-center gap-2"
              >
                <i class="bi bi-star-fill" aria-hidden="true"></i>
                <span>{tr("removeFromFavorites")}</span>
              </button>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}

<!-- Context Menu -->
{#if contextMenuVisible}
  <ContextMenu
    x={contextMenuX}
    y={contextMenuY}
    items={contextMenuItems}
    onClose={closeContextMenu}
  />
{/if}

<style>
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .animate-slide-up {
    animation: slideUp 0.3s ease-out both;
  }
</style>
