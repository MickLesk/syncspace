<script>
  import { onMount } from "svelte";
  import { error as errorToast } from "../stores/toast";
  import api from "../lib/api";
  import Chart from "../components/ui/Chart.svelte";

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
      color: "hsl(var(--er))"
    },
    {
      label: "Available",
      value: diskStats.available_bytes,
      color: "hsl(var(--su))"
    }
  ];

  // Chart data for file types
  $: fileTypeChartData = Object.entries(stats.byType || {}).map(([type, data]) => {
    const category = typeCategories[type] || typeCategories.other;
    return {
      label: category.label,
      value: data.totalSize || 0,
      color: getCategoryColor(category.color)
    };
  }).filter(item => item.value > 0).sort((a, b) => b.value - a.value);

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
      neutral: "hsl(var(--n))"
    };
    return colorMap[colorName] || "hsl(var(--p))";
  }
</script>

<div class="storage-view">
  {#if loading || loadingDisk}
    <div class="flex justify-center items-center h-64">
      <span class="loading loading-spinner loading-lg text-primary"></span>
    </div>
  {:else}
    <!-- System Disk Usage - NEW! -->
    <div
      class="card bg-gradient-to-br from-primary/10 to-secondary/10 border border-primary/20 shadow-lg mb-6"
    >
      <div class="card-body">
        <h2 class="card-title text-2xl mb-4">
          <i class="bi bi-device-hdd-fill mr-2"></i>
          System Disk Usage
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="stat bg-base-100/50 rounded-box p-4">
            <div class="stat-figure text-primary">
              <i class="bi bi-pie-chart-fill text-3xl"></i>
            </div>
            <div class="stat-title">Disk Capacity</div>
            <div class="stat-value text-primary text-2xl">
              {diskTotalFormatted}
            </div>
            <div class="stat-desc">
              {diskStats.filesystem} @ {diskStats.mount_point}
            </div>
          </div>

          <div class="stat bg-base-100/50 rounded-box p-4">
            <div class="stat-figure text-error">
              <i class="bi bi-exclamation-triangle-fill text-3xl"></i>
            </div>
            <div class="stat-title">Used Space</div>
            <div class="stat-value text-error text-2xl">
              {diskUsedFormatted}
            </div>
            <div class="stat-desc">
              {diskStats.usage_percent.toFixed(1)}% of total capacity
            </div>
          </div>

          <div class="stat bg-base-100/50 rounded-box p-4">
            <div class="stat-figure text-success">
              <i class="bi bi-check-circle-fill text-3xl"></i>
            </div>
            <div class="stat-title">Available Space</div>
            <div class="stat-value text-success text-2xl">
              {diskAvailableFormatted}
            </div>
            <div class="stat-desc">Free for new files</div>
          </div>
        </div>

        <!-- Visual Charts Row -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
          <!-- Disk Usage Doughnut Chart -->
          <div class="bg-base-100/50 rounded-box p-6">
            <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
              <i class="bi bi-pie-chart-fill text-primary"></i>
              Disk Usage Distribution
            </h3>
            <Chart data={diskChartData} type="doughnut" size="md" title="Total" />
          </div>

          <!-- Disk Usage Progress Bar (Moved here) -->
          <div class="bg-base-100/50 rounded-box p-6">
            <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
              <i class="bi bi-speedometer2 text-primary"></i>
              Capacity Overview
            </h3>
            <div class="space-y-4">
              <div class="flex justify-between">
                <span class="text-sm font-semibold">Disk Usage</span>
                <span class="text-sm font-semibold"
                  >{diskStats.usage_percent.toFixed(1)}%</span
                >
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
                <div class="stat-compact">
                  <div class="stat-label">Used</div>
                  <div class="stat-value-sm text-error">{diskUsedFormatted}</div>
                </div>
                <div class="stat-compact">
                  <div class="stat-label">Free</div>
                  <div class="stat-value-sm text-success">{diskAvailableFormatted}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Top Stats -->
    <div class="stats stats-vertical lg:stats-horizontal shadow mb-6 w-full">
      <div class="stat">
        <div class="stat-figure text-primary">
          <i class="bi bi-files text-4xl"></i>
        </div>
        <div class="stat-title">Total Files</div>
        <div class="stat-value text-primary">
          {stats.totalFiles.toLocaleString()}
        </div>
        <div class="stat-desc">Across all folders</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-secondary">
          <i class="bi bi-hdd-fill text-4xl"></i>
        </div>
        <div class="stat-title">Storage Used</div>
        <div class="stat-value text-secondary">{totalSizeFormatted}</div>
        <div class="stat-desc">Total disk usage</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-accent">
          <i class="bi bi-pie-chart-fill text-4xl"></i>
        </div>
        <div class="stat-title">File Types</div>
        <div class="stat-value text-accent">
          {Object.values(stats.byType).filter((t) => t.count > 0).length}
        </div>
        <div class="stat-desc">Different categories</div>
      </div>
    </div>

    <!-- Storage Breakdown -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <!-- Visual File Type Chart -->
      <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">
            <i class="bi bi-pie-chart-fill mr-2"></i>
            File Type Distribution
          </h2>
          <div class="mt-4">
            {#if fileTypeChartData.length > 0}
              <Chart data={fileTypeChartData} type="doughnut" size="md" title="Files" />
            {:else}
              <div class="text-center py-8 text-base-content/50">
                <i class="bi bi-inbox text-4xl"></i>
                <p class="mt-2">No files yet</p>
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Bar Chart for File Types -->
      <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">
            <i class="bi bi-bar-chart-fill mr-2"></i>
            Storage by Category
          </h2>
          <div class="mt-4">
            {#if fileTypeChartData.length > 0}
              <Chart data={fileTypeChartData} type="bar" size="md" />
            {:else}
              <div class="text-center py-8 text-base-content/50">
                <i class="bi bi-inbox text-4xl"></i>
                <p class="mt-2">No files yet</p>
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>

    <!-- Detailed Storage Breakdown -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <!-- Type Distribution Cards -->
      <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">
            <i class="bi bi-list-ul mr-2"></i>
            Detailed Breakdown
          </h2>
          <div class="space-y-4 mt-4">
            {#each Object.entries(stats.byType) as [type, data]}
              {@const config = typeCategories[type]}
              {@const percentage = formatPercent(data.size, stats.totalSize)}
              {#if data.count > 0}
                <div class="space-y-2">
                  <div class="flex justify-between items-center">
                    <div class="flex items-center gap-2">
                      <i class="bi bi-{config.icon} text-{config.color} text-xl"
                      ></i>
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
                    <span class="text-sm font-semibold min-w-[3rem] text-right"
                      >{percentage}%</span
                    >
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>

      <!-- Radial Progress Visualization -->
      <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">
            <i class="bi bi-graph-up mr-2"></i>
            Storage Distribution
          </h2>
          <div class="grid grid-cols-3 gap-4 mt-4">
            {#each Object.entries(stats.byType).slice(0, 6) as [type, data]}
              {@const config = typeCategories[type]}
              {@const percentage = formatPercent(data.size, stats.totalSize)}
              {#if data.count > 0}
                <div class="flex flex-col items-center gap-2">
                  <div
                    class="radial-progress text-{config.color}"
                    style="--value:{percentage}; --size:5rem; --thickness: 0.4rem;"
                    role="progressbar"
                  >
                    {percentage}%
                  </div>
                  <div class="text-center">
                    <div class="text-xs font-semibold">{config.label}</div>
                    <div class="text-xs opacity-70">
                      {formatSize(data.size)}
                    </div>
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>
    </div>

    <!-- Largest Files -->
    {#if stats.largestFiles.length > 0}
      <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">
            <i class="bi bi-sort-down-alt mr-2"></i>
            Largest Files
          </h2>
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
                  <tr>
                    <td>{i + 1}</td>
                    <td>
                      <div class="flex items-center gap-2">
                        <i class="bi bi-file-earmark-fill text-primary"></i>
                        <span class="font-medium">{file.name}</span>
                      </div>
                    </td>
                    <td>
                      <span class="badge badge-ghost font-semibold"
                        >{formatSize(file.size)}</span
                      >
                    </td>
                    <td>
                      <div class="flex items-center gap-2">
                        <progress
                          class="progress progress-primary w-20"
                          value={formatPercent(file.size, stats.totalSize)}
                          max="100"
                        ></progress>
                        <span class="text-sm"
                          >{formatPercent(file.size, stats.totalSize)}%</span
                        >
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .storage-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }

  /* Compact stat styles */
  .stat-compact {
    text-align: center;
    padding: 0.75rem;
    background: hsl(var(--b2) / 0.5);
    border-radius: 0.5rem;
  }

  .stat-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: hsl(var(--bc) / 0.6);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .stat-value-sm {
    font-size: 1.25rem;
    font-weight: 700;
    margin-top: 0.25rem;
  }

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
