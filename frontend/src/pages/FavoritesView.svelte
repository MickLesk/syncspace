<script>
  import { onMount } from "svelte";
  import { favorites } from "../stores/favorites";
  import { currentPath, currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import { success, error as errorToast } from "../stores/toast";
  import { getFileIcon } from "../utils/fileIcons";
  import api from "../lib/api.js";
  import PageWrapper from "../components/PageWrapper.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";
  import Loading from "../components/Loading.svelte";

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

<PageWrapper gradient>
  <!-- Animated Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Page Header -->
  <div class="flex justify-between items-start mb-8 relative z-10">
    <div>
      <h1
        class="text-4xl font-bold gradient-text-primary mb-2 flex items-center gap-3"
      >
        <i class="bi bi-star-fill text-amber-500"></i>
        {t($currentLang, "favorites")}
      </h1>
      <p class="text-base-content/70">
        {favoriteFiles.length}
        {favoriteFiles.length === 1 ? "item" : "items"}
      </p>
    </div>
    <ModernButton
      variant="ghost"
      icon="arrow-clockwise"
      disabled={loading}
      onclick={loadFavorites}
    >
      Refresh
    </ModernButton>
  </div>

  {#if loading}
    <Loading />
  {:else if error}
    <ModernCard variant="glass" class="text-center py-16">
      {#snippet children()}
        <div class="text-error/30 mb-6">
          <i class="bi bi-exclamation-triangle text-8xl"></i>
        </div>
        <h3 class="text-2xl font-bold text-error mb-3">{error}</h3>
        <ModernButton
          variant="primary"
          icon="arrow-clockwise"
          onclick={loadFavorites}
        >
          Try Again
        </ModernButton>
      {/snippet}
    </ModernCard>
  {:else if favoriteFiles.length === 0}
    <ModernCard variant="glass" class="text-center py-16">
      {#snippet children()}
        <div class="animate-fade-in">
          <div class="text-amber-500/30 mb-6">
            <i class="bi bi-star text-8xl"></i>
          </div>
          <h3 class="text-2xl font-bold mb-3">
            {t($currentLang, "noFavorites")}
          </h3>
          <p class="text-base-content/60">
            {t($currentLang, "markFilesAsFavorite")}
          </p>
        </div>
      {/snippet}
    </ModernCard>
  {:else}
    <div
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
    >
      {#each favoriteFiles as file, i}
        <ModernCard
          variant="glass"
          hoverable
          clickable
          onclick={() => navigateToFile(file.fullPath)}
          class="animate-slide-up"
          style="animation-delay: {i * 30}ms;"
        >
          {#snippet children()}
            <div class="text-center">
              <div class="text-6xl mb-4">
                {getFileIconEmoji(file.mimeType)}
              </div>

              <h3 class="font-bold text-lg mb-2 truncate" title={file.name}>
                {file.name}
              </h3>

              <div
                class="flex gap-3 justify-center text-xs text-base-content/60 mb-3"
              >
                {#if file.size}
                  <span class="flex items-center gap-1">
                    <i class="bi bi-file-earmark"></i>
                    {formatFileSize(file.size)}
                  </span>
                {/if}
                {#if file.createdAt}
                  <span class="flex items-center gap-1">
                    <i class="bi bi-clock"></i>
                    {formatDate(file.createdAt)}
                  </span>
                {/if}
              </div>

              <div class="text-xs font-mono text-base-content/50 mb-4 truncate">
                {formatPath(file.fullPath)}
              </div>

              <ModernButton
                variant="ghost"
                size="sm"
                icon="star-fill"
                onclick={(e) => {
                  e.stopPropagation();
                  removeFavorite(file);
                }}
                fullWidth
              >
                <span class="text-amber-500">Remove Favorite</span>
              </ModernButton>
            </div>
          {/snippet}
        </ModernCard>
      {/each}
    </div>
  {/if}
</PageWrapper>
