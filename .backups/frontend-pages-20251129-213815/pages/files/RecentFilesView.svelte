<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t, formatRelativeTime, formatFileSize } from "../../i18n.js";

  let recentFiles = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let limit = $state(20);

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  onMount(async () => {
    await loadRecentFiles();
  });

  async function loadRecentFiles() {
    try {
      loading = true;
      error = null;
      console.log("[RecentFiles] Loading recent files, limit:", limit);

      // Use the new recent files API
      const data = await api.recent.list(limit);
      console.log("[RecentFiles] API response:", data);

      // Transform the response to match expected structure
      recentFiles = Array.isArray(data)
        ? data.map((file) => ({
            ...file,
            filename: file.name || file.filename,
            mime_type: file.mime_type || "application/octet-stream",
            last_accessed_at: file.accessed_at || file.last_accessed_at,
            access_count: file.access_count || 1,
            access_type: file.action || file.access_type || "view",
          }))
        : [];

      console.log("[RecentFiles] Transformed files:", recentFiles);
    } catch (err) {
      console.error("[RecentFiles] Failed to load recent files:", err);
      error = `${tr("recentLoadFailed")}: ${err.message}`;
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
      view: { icon: "eye", labelKey: "recentBadgeViewed", color: "info" },
      edit: { icon: "pencil", labelKey: "recentBadgeEdited", color: "warning" },
      download: {
        icon: "download",
        labelKey: "recentBadgeDownloaded",
        color: "success",
      },
      upload: {
        icon: "upload",
        labelKey: "recentBadgeUploaded",
        color: "primary",
      },
    };
    const badge = badges[accessType] || {
      icon: "file-earmark",
      labelKey: "recentBadgeAccessed",
      color: "info",
    };

    return { ...badge, label: tr(badge.labelKey) };
  }

  function formatLastAccessed(timestamp) {
    return formatRelativeTime($currentLang, timestamp);
  }

  async function openFile(file) {
    // Navigate to file location or open in preview
    try {
      // Emit event to parent to show file preview
      dispatchFilePreview(file);
    } catch (error) {
      console.error("Failed to open file:", error);
    }
  }

  function dispatchFilePreview(file) {
    // Store file in preview store and navigate/show modal
    console.log("Preview file:", file);
    // This would integrate with a file preview modal store
  }
</script>

<PageWrapper gradient>
  <PageHeader
    title={tr("recentFiles")}
    subtitle={tr("recentFilesSubtitle")}
    icon="clock-history"
  >
    {#snippet actions()}
      <select
        class="glass-input px-3 py-1.5 rounded-lg text-sm"
        bind:value={limit}
        onchange={loadRecentFiles}
      >
        <option value={10}>{tr("recentLastCount", 10)}</option>
        <option value={20}>{tr("recentLastCount", 20)}</option>
        <option value={50}>{tr("recentLastCount", 50)}</option>
        <option value={100}>{tr("recentLastCount", 100)}</option>
      </select>

      <ModernButton
        variant="secondary"
        icon="arrow-clockwise"
        onclick={loadRecentFiles}
        disabled={loading}
      >
        {tr("refresh")}
      </ModernButton>
    {/snippet}
  </PageHeader>

  <div class="space-y-6">
    {#if loading}
      <!-- Skeleton Loading State -->
      <div class="space-y-4">
        {#each Array(6) as _}
          <div class="skeleton h-24 w-full rounded-xl"></div>
        {/each}
      </div>
    {:else if error}
      <ModernCard variant="glass" padding="large">
        <div class="text-center animate-fade-in">
          <div class="mb-6">
            <i class="bi bi-exclamation-triangle text-6xl text-red-500/30"></i>
          </div>
          <h3 class="text-2xl font-bold text-red-600 dark:text-red-400 mb-3">
            {error}
          </h3>
          <ModernButton
            variant="gradient"
            icon="arrow-clockwise"
            onclick={loadRecentFiles}
          >
            {tr("tryAgain")}
          </ModernButton>
        </div>
      </ModernCard>
    {:else if recentFiles.length === 0}
      <ModernCard variant="glass" padding="large">
        <div class="text-center animate-fade-in">
          <div class="mb-6">
            <i class="bi bi-clock-history text-8xl opacity-20"></i>
          </div>
          <h3 class="text-2xl font-bold mb-3 text-gray-900 dark:text-gray-100">
            {tr("recentNoFilesTitle")}
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            {tr("recentNoFilesDescription")}
          </p>
        </div>
      </ModernCard>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        {#each recentFiles as file, i}
          <ModernCard
            variant="glass"
            hoverable
            clickable
            onclick={() => openFile(file)}
            class="animate-slide-up"
            style="animation-delay: {i * 30}ms"
          >
            <div class="flex gap-4">
              <div class="text-5xl flex-shrink-0">
                {getFileIcon(file.mime_type)}
              </div>

              <div class="flex-1 min-w-0">
                <h3
                  class="font-bold text-lg mb-2 truncate text-gray-900 dark:text-gray-100"
                  title={file.filename}
                >
                  {file.filename}
                </h3>

                <div
                  class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 mb-3"
                >
                  <span>{formatFileSize(file.size_bytes, $currentLang)}</span>
                  <span>•</span>
                  <span>{formatLastAccessed(file.last_accessed_at)}</span>
                </div>

                <div class="flex gap-2 flex-wrap items-center">
                  <span class="badge-glass-info text-xs">
                    <i class="bi bi-eye"></i>
                    {tr(
                      file.access_count === 1
                        ? "recentAccessCountSingular"
                        : "recentAccessCountPlural",
                      file.access_count
                    )}
                  </span>

                  {#if file.access_type}
                    {@const badge = getAccessTypeBadge(file.access_type)}
                    <span class="badge-glass-{badge.color} text-xs">
                      <i class="bi bi-{badge.icon}"></i>
                      {badge.label}
                    </span>
                  {/if}
                </div>
              </div>

              <div class="flex-shrink-0">
                <ModernButton
                  variant="ghost"
                  size="sm"
                  icon="eye"
                  onclick={(e) => {
                    e.stopPropagation();
                    console.log("Quick preview");
                  }}
                >
                  {tr("view")}
                </ModernButton>
              </div>
            </div>
          </ModernCard>
        {/each}
      </div>
    {/if}
  </div>
</PageWrapper>
