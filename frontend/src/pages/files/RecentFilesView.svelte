<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

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
      error = "Failed to load recent files: " + err.message;
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
      view: { icon: "eye", label: "Viewed", color: "info" },
      edit: { icon: "pencil", label: "Edited", color: "warning" },
      download: { icon: "download", label: "Downloaded", color: "success" },
      upload: { icon: "upload", label: "Uploaded", color: "primary" },
    };
    return (
      badges[accessType] || {
        icon: "file-earmark",
        label: "Accessed",
        color: "info",
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

<PageWrapper gradient>
  <PageHeader
    title="Recent Files"
    subtitle="Files you've recently accessed"
    icon="clock-history"
  >
    {#snippet actions()}
      <select
        class="glass-input px-3 py-1.5 rounded-lg text-sm"
        bind:value={limit}
        onchange={loadRecentFiles}
      >
        <option value={10}>Last 10</option>
        <option value={20}>Last 20</option>
        <option value={50}>Last 50</option>
        <option value={100}>Last 100</option>
      </select>

      <ModernButton
        variant="secondary"
        icon="arrow-clockwise"
        onclick={loadRecentFiles}
        disabled={loading}
      >
        Refresh
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
            Try Again
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
            No recent files
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            Files you access will appear here
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
                  <span>{formatFileSize(file.size_bytes)}</span>
                  <span>•</span>
                  <span>{formatLastAccessed(file.last_accessed_at)}</span>
                </div>

                <div class="flex gap-2 flex-wrap items-center">
                  <span class="badge-glass-info text-xs">
                    <i class="bi bi-eye"></i>
                    {file.access_count}
                    {file.access_count === 1 ? "time" : "times"}
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
                  View
                </ModernButton>
              </div>
            </div>
          </ModernCard>
        {/each}
      </div>
    {/if}
  </div>
</PageWrapper>
