<script>
  import { onMount } from "svelte";
  import { error as errorToast } from "../stores/toast";
  import api from "../lib/api";

  let loading = true;
  let stats = {
    totalFiles: 0,
    totalSize: 0,
    byType: {},
    largestFiles: [],
  };

  $: totalSizeFormatted = formatSize(stats.totalSize);

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
    await loadStats();
  });

  async function loadStats() {
    loading = true;
    try {
      const basicStats = await api.files.stats();
      const files = await api.files.list("");

      stats = {
        totalFiles: basicStats.file_count || 0,
        totalSize: basicStats.total_size || 0,
        byType: computeTypeStats(files),
        largestFiles: getLargestFiles(files, 10),
      };
    } catch (err) {
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
</script>

<div class="storage-view">
  {#if loading}
    <div class="flex justify-center items-center h-64">
      <span class="loading loading-spinner loading-lg text-primary"></span>
    </div>
  {:else}
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
      <!-- Type Distribution Cards -->
      <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">
            <i class="bi bi-pie-chart-fill mr-2"></i>
            Storage by Type
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
