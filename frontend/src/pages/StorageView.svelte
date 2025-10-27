<script>
  import { onMount } from "svelte";
  import { error as errorToast } from "../stores/toast";
  import api from "../lib/api";
  import Chart from "../components/ui/Chart.svelte";
  import PageWrapper from "../components/PageWrapper.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import Loading from "../components/Loading.svelte";

  let loading = true;
  let loadingDisk = true;
  let stats = {
    totalFiles: 0,
    totalSize: 0,
    byType: {},
    largestFiles: [],
  };

  // System disk usage stats
  let diskStats = {
    total_bytes: 0,
    available_bytes: 0,
    used_bytes: 0,
    usage_percent: 0,
    mount_point: "",
    filesystem: "",
  };

  $: totalSizeFormatted = formatSize(stats.totalSize);
  $: diskTotalFormatted = formatSize(diskStats.total_bytes);
  $: diskUsedFormatted = formatSize(diskStats.used_bytes);
  $: diskAvailableFormatted = formatSize(diskStats.available_bytes);

  // Chart data for disk usage
  $: diskChartData = [
    {
      label: "Used",
      value: diskStats.used_bytes,
      color: "hsl(var(--er))",
    },
    {
      label: "Available",
      value: diskStats.available_bytes,
      color: "hsl(var(--su))",
    },
  ];

  // Chart data for file types
  $: fileTypeChartData = Object.entries(stats.byType || {})
    .map(([type, data]) => {
      const category = typeCategories[type] || typeCategories.other;
      return {
        label: category.label,
        value: data.totalSize || 0,
        color: getCategoryColor(category.color),
      };
    })
    .filter((item) => item.value > 0)
    .sort((a, b) => b.value - a.value);

  // File type categories
  const typeCategories = {
    images: {
      label: "Images",
      icon: "image",
      color: "primary",
      extensions: [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg"],
    },
    documents: {
      label: "Documents",
      icon: "file-text",
      color: "info",
      extensions: [
        ".pdf",
        ".doc",
        ".docx",
        ".txt",
        ".md",
        ".rtf",
        ".xls",
        ".xlsx",
        ".ppt",
        ".pptx",
      ],
    },
    videos: {
      label: "Videos",
      icon: "film",
      color: "secondary",
      extensions: [".mp4", ".avi", ".mov", ".mkv", ".webm", ".flv"],
    },
    audio: {
      label: "Audio",
      icon: "music-note-beamed",
      color: "accent",
      extensions: [".mp3", ".wav", ".ogg", ".m4a", ".flac", ".aac"],
    },
    archives: {
      label: "Archives",
      icon: "file-zip",
      color: "warning",
      extensions: [".zip", ".rar", ".7z", ".tar", ".gz", ".bz2"],
    },
    other: {
      label: "Other",
      icon: "file-earmark",
      color: "neutral",
      extensions: [],
    },
  };

  onMount(async () => {
    await Promise.all([loadStats(), loadDiskStats()]);
  });

  async function loadDiskStats() {
    loadingDisk = true;
    try {
      diskStats = await api.system.getStorageInfo();
    } catch (err) {
      errorToast(err.message || "Failed to load disk statistics");
    } finally {
      loadingDisk = false;
    }
  }

  async function loadStats() {
    loading = true;
    try {
      // Use backend stats endpoint
      const basicStats = await api.system.getStats();
      const files = await api.files.list("");

      stats = {
        totalFiles: basicStats.file_count || 0,
        totalSize: basicStats.total_size || 0,
        byType: computeTypeStats(files),
        largestFiles: getLargestFiles(files, 10),
      };
    } catch (err) {
      console.error("Failed to load storage statistics:", err);
      errorToast(err.message || "Failed to load storage statistics");
    } finally {
      loading = false;
    }
  }

  function computeTypeStats(files) {
    const typeStats = {};

    // Initialize all categories
    for (const key of Object.keys(typeCategories)) {
      typeStats[key] = { count: 0, size: 0 };
    }

    for (const file of files) {
      if (file.is_dir) continue;

      const ext = file.name.toLowerCase().slice(file.name.lastIndexOf("."));
      let category = "other";

      for (const [cat, config] of Object.entries(typeCategories)) {
        if (config.extensions.includes(ext)) {
          category = cat;
          break;
        }
      }

      typeStats[category].count++;
      typeStats[category].size += file.size || 0;
    }

    return typeStats;
  }

  function getLargestFiles(files, n) {
    return files
      .filter((f) => !f.is_dir)
      .sort((a, b) => (b.size || 0) - (a.size || 0))
      .slice(0, n);
  }

  function formatSize(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function formatPercent(value, total) {
    if (total === 0) return 0;
    return Math.round((value / total) * 100);
  }

  function getCategoryColor(colorName) {
    const colorMap = {
      primary: "hsl(var(--p))",
      secondary: "hsl(var(--s))",
      accent: "hsl(var(--a))",
      info: "hsl(var(--in))",
      success: "hsl(var(--su))",
      warning: "hsl(var(--wa))",
      error: "hsl(var(--er))",
      neutral: "hsl(var(--n))",
    };
    return colorMap[colorName] || "hsl(var(--p))";
  }
</script>

<PageWrapper gradient>
  <!-- Animated Background Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  {#if loading || loadingDisk}
    <Loading />
  {:else}
    <!-- Page Header -->
    <div class="mb-8 relative z-10">
      <h1 class="text-4xl font-bold gradient-text-primary mb-2 flex items-center gap-3">
        <i class="bi bi-pie-chart-fill"></i>
        Storage Analytics
      </h1>
      <p class="text-base-content/70">Complete overview of your disk usage and file distribution</p>
    </div>

    <!-- System Disk Usage -->
    <ModernCard variant="glass" title="System Disk Usage" icon="device-hdd-fill" class="mb-6">
      {#snippet children()}
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          <div class="glass-card-light p-6 text-center">
            <div class="text-primary mb-2">
              <i class="bi bi-pie-chart-fill text-4xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">Disk Capacity</div>
            <div class="text-3xl font-bold text-primary mb-2">{diskTotalFormatted}</div>
            <div class="text-xs text-base-content/50">
              {diskStats.filesystem} @ {diskStats.mount_point}
            </div>
          </div>

          <div class="glass-card-light p-6 text-center">
            <div class="text-error mb-2">
              <i class="bi bi-exclamation-triangle-fill text-4xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">Used Space</div>
            <div class="text-3xl font-bold text-error mb-2">{diskUsedFormatted}</div>
            <div class="text-xs text-base-content/50">
              {diskStats.usage_percent.toFixed(1)}% of total
            </div>
          </div>

          <div class="glass-card-light p-6 text-center">
            <div class="text-success mb-2">
              <i class="bi bi-check-circle-fill text-4xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">Available</div>
            <div class="text-3xl font-bold text-success mb-2">{diskAvailableFormatted}</div>
            <div class="text-xs text-base-content/50">Free for new files</div>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <!-- Disk Usage Chart -->
          <div class="glass-card-light p-6">
            <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
              <i class="bi bi-pie-chart-fill text-primary"></i>
              Disk Distribution
            </h3>
            <Chart data={diskChartData} type="doughnut" size="md" title="Total" />
          </div>

          <!-- Capacity Progress -->
          <div class="glass-card-light p-6">
            <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
              <i class="bi bi-speedometer2 text-primary"></i>
              Capacity Overview
            </h3>
            <div class="space-y-4">
              <div class="flex justify-between">
                <span class="text-sm font-semibold">Disk Usage</span>
                <span class="text-sm font-semibold">{diskStats.usage_percent.toFixed(1)}%</span>
              </div>
              <progress
                class="progress {diskStats.usage_percent > 90
                  ? 'progress-error'
                  : diskStats.usage_percent > 70
                    ? 'progress-warning'
                    : 'progress-success'} h-4"
                value={diskStats.usage_percent}
                max="100"
              ></progress>

              <div class="grid grid-cols-2 gap-4 mt-6">
                <div class="text-center">
                  <div class="text-xs text-base-content/60 mb-1">Used</div>
                  <div class="text-xl font-bold text-error">{diskUsedFormatted}</div>
                </div>
                <div class="text-center">
                  <div class="text-xs text-base-content/60 mb-1">Free</div>
                  <div class="text-xl font-bold text-success">{diskAvailableFormatted}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      {/snippet}
    </ModernCard>

    <!-- Quick Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center">
            <div class="text-primary mb-3">
              <i class="bi bi-files text-5xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">Total Files</div>
            <div class="text-4xl font-bold mb-2">{stats.totalFiles.toLocaleString()}</div>
            <div class="text-xs text-base-content/50">Across all folders</div>
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center">
            <div class="text-secondary mb-3">
              <i class="bi bi-hdd-fill text-5xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">Storage Used</div>
            <div class="text-4xl font-bold mb-2">{totalSizeFormatted}</div>
            <div class="text-xs text-base-content/50">Total disk usage</div>
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center">
            <div class="text-accent mb-3">
              <i class="bi bi-pie-chart-fill text-5xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">File Types</div>
            <div class="text-4xl font-bold mb-2">
              {Object.values(stats.byType).filter((t) => t.count > 0).length}
            </div>
            <div class="text-xs text-base-content/50">Different categories</div>
          </div>
        {/snippet}
      </ModernCard>
    </div>

    <!-- File Type Charts -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <ModernCard variant="glass" title="File Type Distribution" icon="pie-chart-fill">
        {#snippet children()}
          <div class="mt-4">
            {#if fileTypeChartData.length > 0}
              <Chart data={fileTypeChartData} type="doughnut" size="md" title="Files" />
            {:else}
              <div class="text-center py-12 text-base-content/50">
                <i class="bi bi-inbox text-5xl mb-3"></i>
                <p>No files yet</p>
              </div>
            {/if}
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="glass" title="Storage by Category" icon="bar-chart-fill">
        {#snippet children()}
          <div class="mt-4">
            {#if fileTypeChartData.length > 0}
              <Chart data={fileTypeChartData} type="bar" size="md" />
            {:else}
              <div class="text-center py-12 text-base-content/50">
                <i class="bi bi-inbox text-5xl mb-3"></i>
                <p>No files yet</p>
              </div>
            {/if}
          </div>
        {/snippet}
      </ModernCard>
    </div>

    <!-- Detailed Breakdown -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <ModernCard variant="glass" title="Detailed Breakdown" icon="list-ul">
        {#snippet children()}
          <div class="space-y-4 mt-4">
            {#each Object.entries(stats.byType) as [type, data]}
              {@const config = typeCategories[type]}
              {@const percentage = formatPercent(data.size, stats.totalSize)}
              {#if data.count > 0}
                <div class="space-y-2 animate-slide-up">
                  <div class="flex justify-between items-center">
                    <div class="flex items-center gap-2">
                      <i class="bi bi-{config.icon} text-{config.color} text-xl"></i>
                      <span class="font-semibold">{config.label}</span>
                    </div>
                    <div class="text-right">
                      <div class="font-bold">{formatSize(data.size)}</div>
                      <div class="text-xs opacity-70">{data.count} files</div>
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    <progress
                      class="progress progress-{config.color} flex-1"
                      value={percentage}
                      max="100"
                    ></progress>
                    <span class="text-sm font-semibold min-w-[3rem] text-right">{percentage}%</span>
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="glass" title="Storage Distribution" icon="graph-up">
        {#snippet children()}
          <div class="grid grid-cols-3 gap-4 mt-4">
            {#each Object.entries(stats.byType).slice(0, 6) as [type, data]}
              {@const config = typeCategories[type]}
              {@const percentage = formatPercent(data.size, stats.totalSize)}
              {#if data.count > 0}
                <div class="flex flex-col items-center gap-2 animate-fade-in">
                  <div
                    class="radial-progress text-{config.color}"
                    style="--value:{percentage}; --size:5rem; --thickness: 0.4rem;"
                    role="progressbar"
                  >
                    {percentage}%
                  </div>
                  <div class="text-center">
                    <div class="text-xs font-semibold">{config.label}</div>
                    <div class="text-xs opacity-70">{formatSize(data.size)}</div>
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        {/snippet}
      </ModernCard>
    </div>

    <!-- Largest Files -->
    {#if stats.largestFiles.length > 0}
      <ModernCard variant="glass" title="Largest Files" icon="sort-down-alt">
        {#snippet children()}
          <div class="overflow-x-auto mt-4">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>#</th>
                  <th>Name</th>
                  <th>Size</th>
                  <th>% of Total</th>
                </tr>
              </thead>
              <tbody>
                {#each stats.largestFiles as file, i}
                  <tr class="animate-slide-up" style="animation-delay: {i * 50}ms;">
                    <td>{i + 1}</td>
                    <td>
                      <div class="flex items-center gap-2">
                        <i class="bi bi-file-earmark-fill text-primary"></i>
                        <span class="font-medium">{file.name}</span>
                      </div>
                    </td>
                    <td>
                      <span class="badge badge-glass-info font-semibold">{formatSize(file.size)}</span>
                    </td>
                    <td>
                      <div class="flex items-center gap-2">
                        <progress
                          class="progress progress-primary w-20"
                          value={formatPercent(file.size, stats.totalSize)}
                          max="100"
                        ></progress>
                        <span class="text-sm">{formatPercent(file.size, stats.totalSize)}%</span>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/snippet}
      </ModernCard>
    {/if}
  {/if}
</PageWrapper>

<style>
  /* Radial progress color overrides */
  :global(.radial-progress.text-primary) {
    --value: 0;
    --size: 5rem;
    --thickness: 0.4rem;
    color: oklch(var(--p));
  }

  :global(.radial-progress.text-secondary) {
    color: oklch(var(--s));
  }

  :global(.radial-progress.text-accent) {
    color: oklch(var(--a));
  }

  :global(.radial-progress.text-info) {
    color: oklch(var(--in));
  }

  :global(.radial-progress.text-success) {
    color: oklch(var(--su));
  }

  :global(.radial-progress.text-warning) {
    color: oklch(var(--wa));
  }

  :global(.radial-progress.text-neutral) {
    color: oklch(var(--n));
  }
</style>
