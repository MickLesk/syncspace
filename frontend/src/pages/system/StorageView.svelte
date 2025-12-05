<script>
  import { onMount } from "svelte";
  import { error as errorToast, success } from "../../stores/toast";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api";
  import Chart from "../../components/ui/Chart.svelte";
  import Loading from "../../components/ui/Loading.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let loadingDisk = $state(true);
  let stats = $state({
    totalFiles: 0,
    totalSize: 0,
    byType: {},
    largestFiles: [],
  });

  // System disk usage stats
  let diskStats = $state({
    total_bytes: 0,
    available_bytes: 0,
    used_bytes: 0,
    usage_percent: 0,
    mount_point: "",
    filesystem: "",
  });

  const totalSizeFormatted = $derived(formatSize(stats.totalSize));
  const diskTotalFormatted = $derived(formatSize(diskStats.total_bytes));
  const diskUsedFormatted = $derived(formatSize(diskStats.used_bytes));
  const diskAvailableFormatted = $derived(
    formatSize(diskStats.available_bytes)
  );

  // Chart data for disk usage
  const diskChartData = $derived([
    {
      label: $t("used"),
      value: diskStats.used_bytes,
      color: "rgb(239, 68, 68)",
    },
    {
      label: $t("available"),
      value: diskStats.available_bytes,
      color: "rgb(34, 197, 94)",
    },
  ]);

  // Chart data for file types
  const fileTypeChartData = $derived(
    Object.entries(stats.byType || {})
      .map(([type, data]) => {
        const category = typeCategories[type] || typeCategories.other;
        return {
          label: category.label,
          value: data.totalSize || 0,
          color: getCategoryColor(category.color),
        };
      })
      .filter((item) => item.value > 0)
      .sort((a, b) => b.value - a.value)
  );

  // File type categories
  const getTypeCategories = () => ({
    images: {
      label: $t("images"),
      icon: "image",
      color: "purple",
      extensions: [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg"],
    },
    documents: {
      label: $t("documents"),
      icon: "file-text",
      color: "blue",
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
      label: $t("videos"),
      icon: "film",
      color: "red",
      extensions: [".mp4", ".avi", ".mov", ".mkv", ".webm", ".flv"],
    },
    audio: {
      label: $t("audio"),
      icon: "music-note-beamed",
      color: "green",
      extensions: [".mp3", ".wav", ".ogg", ".m4a", ".flac", ".aac"],
    },
    archives: {
      label: $t("archives"),
      icon: "file-zip",
      color: "yellow",
      extensions: [".zip", ".rar", ".7z", ".tar", ".gz", ".bz2"],
    },
    other: {
      label: $t("file"),
      icon: "file-earmark",
      color: "gray",
      extensions: [],
    },
  });

  const typeCategories = $derived(getTypeCategories());

  onMount(async () => {
    await Promise.all([loadStats(), loadDiskStats()]);
  });

  async function loadDiskStats() {
    loadingDisk = true;
    try {
      diskStats = await api.system.storage();
    } catch (err) {
      errorToast(err.message || $t("failedToLoadDiskStatistics"));
    } finally {
      loadingDisk = false;
    }
  }

  async function loadStats() {
    loading = true;
    try {
      // Use backend stats endpoint
      const basicStats = await api.system.stats();
      const files = await api.files.list("");

      stats = {
        totalFiles: basicStats.file_count || 0,
        totalSize: basicStats.total_size || 0,
        byType: computeTypeStats(files),
        largestFiles: getLargestFiles(files, 10),
      };
    } catch (err) {
      console.error("Failed to load storage statistics:", err);
      errorToast(err.message || $t("failedToLoadStorageStatistics"));
    } finally {
      loading = false;
    }
  }

  function computeTypeStats(files) {
    const typeStats = {};

    // Initialize all categories
    for (const key of Object.keys(typeCategories)) {
      typeStats[key] = { count: 0, size: 0, totalSize: 0 };
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
      typeStats[category].totalSize += file.size || 0;
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
      purple: "rgb(147, 51, 234)",
      blue: "rgb(59, 130, 246)",
      red: "rgb(239, 68, 68)",
      green: "rgb(34, 197, 94)",
      yellow: "rgb(234, 179, 8)",
      gray: "rgb(107, 114, 128)",
    };
    return colorMap[colorName] || "rgb(99, 102, 241)";
  }
</script>

<!-- Main Container -->
<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
>
  <!-- Animated Background Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  {#if loading || loadingDisk}
    <Loading />
  {:else}
    <!-- Content Container -->
    <div class="relative z-10 max-w-7xl mx-auto">
      <!-- Page Header -->
      <div class="mb-6">
        <h1
          class="text-4xl font-bold gradient-text mb-2 flex items-center gap-3"
        >
          <i class="bi bi-pie-chart-fill" aria-hidden="true"></i>
          Storage Analytics
        </h1>
        <p class="text-gray-600 dark:text-gray-400">
          Complete overview of your disk usage and file distribution
        </p>
      </div>

      <!-- System Disk Usage -->
      <div class="glass-card mb-6 p-6">
        <h2
          class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2"
        >
          <i class="bi bi-device-hdd-fill text-green-600" aria-hidden="true"></i>
          System Disk Usage
        </h2>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          <div class="glass-card p-6 text-center">
            <div class="text-green-600 dark:text-green-400 mb-2">
              <i class="bi bi-pie-chart-fill text-4xl" aria-hidden="true"></i>
            </div>
            <div
              class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
            >
              Disk Capacity
            </div>
            <div
              class="text-3xl font-bold text-green-600 dark:text-green-400 mb-2"
            >
              {diskTotalFormatted}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-500">
              {diskStats.filesystem} @ {diskStats.mount_point}
            </div>
          </div>

          <div class="glass-card p-6 text-center">
            <div class="text-red-600 dark:text-red-400 mb-2">
              <i class="bi bi-exclamation-triangle-fill text-4xl" aria-hidden="true"></i>
            </div>
            <div
              class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
            >
              Used Space
            </div>
            <div class="text-3xl font-bold text-red-600 dark:text-red-400 mb-2">
              {diskUsedFormatted}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-500">
              {diskStats.usage_percent.toFixed(1)}% of total
            </div>
          </div>

          <div class="glass-card p-6 text-center">
            <div class="text-green-600 dark:text-green-400 mb-2">
              <i class="bi bi-check-circle-fill text-4xl" aria-hidden="true"></i>
            </div>
            <div
              class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
            >
              Available
            </div>
            <div
              class="text-3xl font-bold text-green-600 dark:text-green-400 mb-2"
            >
              {diskAvailableFormatted}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-500">
              Free for new files
            </div>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <!-- Disk Usage Chart -->
          <div class="glass-card p-6">
            <h3
              class="text-lg font-bold mb-4 flex items-center gap-2 text-gray-900 dark:text-gray-100"
            >
              <i class="bi bi-pie-chart-fill text-green-600" aria-hidden="true"></i>
              Disk Distribution
            </h3>
            <Chart
              data={diskChartData}
              type="doughnut"
              size="md"
              title="Total"
            />
          </div>

          <!-- Capacity Progress -->
          <div class="glass-card p-6">
            <h3
              class="text-lg font-bold mb-4 flex items-center gap-2 text-gray-900 dark:text-gray-100"
            >
              <i class="bi bi-speedometer2 text-green-600" aria-hidden="true"></i>
              Capacity Overview
            </h3>
            <div class="space-y-4">
              <div class="flex justify-between">
                <span
                  class="text-sm font-semibold text-gray-700 dark:text-gray-300"
                  >Disk Usage</span
                >
                <span
                  class="text-sm font-semibold text-gray-700 dark:text-gray-300"
                >
                  {diskStats.usage_percent.toFixed(1)}%
                </span>
              </div>

              <div
                class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-4 overflow-hidden"
              >
                <div
                  class="h-full transition-all duration-500 {diskStats.usage_percent >
                  90
                    ? 'bg-gradient-to-r from-red-500 to-red-600'
                    : diskStats.usage_percent > 70
                      ? 'bg-gradient-to-r from-yellow-500 to-yellow-600'
                      : 'bg-gradient-to-r from-green-500 to-green-600'}"
                  style="width: {diskStats.usage_percent}%"
                ></div>
              </div>

              <div
                class="grid grid-cols-2 gap-4 mt-6 pt-4 border-t border-gray-200 dark:border-gray-700"
              >
                <div class="text-center">
                  <div class="text-xs text-gray-500 dark:text-gray-400 mb-1">
                    Used
                  </div>
                  <div class="text-xl font-bold text-red-600 dark:text-red-400">
                    {diskUsedFormatted}
                  </div>
                </div>
                <div class="text-center">
                  <div class="text-xs text-gray-500 dark:text-gray-400 mb-1">
                    Free
                  </div>
                  <div
                    class="text-xl font-bold text-green-600 dark:text-green-400"
                  >
                    {diskAvailableFormatted}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Quick Stats -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
        <div
          class="glass-card p-6 text-center hover:shadow-xl transition-all duration-300"
        >
          <div class="text-green-600 dark:text-green-400 mb-3">
            <i class="bi bi-files text-5xl" aria-hidden="true"></i>
          </div>
          <div
            class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
          >
            Total Files
          </div>
          <div class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            {stats.totalFiles.toLocaleString()}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            Across all folders
          </div>
        </div>

        <div
          class="glass-card p-6 text-center hover:shadow-xl transition-all duration-300"
        >
          <div class="text-purple-600 dark:text-purple-400 mb-3">
            <i class="bi bi-hdd-fill text-5xl" aria-hidden="true"></i>
          </div>
          <div
            class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
          >
            Storage Used
          </div>
          <div class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            {totalSizeFormatted}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            Total disk usage
          </div>
        </div>

        <div
          class="glass-card p-6 text-center hover:shadow-xl transition-all duration-300"
        >
          <div class="text-green-600 dark:text-green-400 mb-3">
            <i class="bi bi-pie-chart-fill text-5xl" aria-hidden="true"></i>
          </div>
          <div
            class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
          >
            File Types
          </div>
          <div class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            {Object.values(stats.byType).filter((t) => t.count > 0).length}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            Different categories
          </div>
        </div>
      </div>

      <!-- File Type Charts -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
        <div class="glass-card p-6">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-4 flex items-center gap-2"
          >
            <i class="bi bi-pie-chart-fill text-green-600" aria-hidden="true"></i>
            File Type Distribution
          </h2>
          <div class="mt-4">
            {#if fileTypeChartData.length > 0}
              <Chart
                data={fileTypeChartData}
                type="doughnut"
                size="md"
                title="Files"
              />
            {:else}
              <div class="text-center py-12 text-gray-500 dark:text-gray-400">
                <i class="bi bi-inbox text-5xl mb-3" aria-hidden="true"></i>
                <p>No files yet</p>
              </div>
            {/if}
          </div>
        </div>

        <div class="glass-card p-6">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-4 flex items-center gap-2"
          >
            <i class="bi bi-bar-chart-fill text-green-600" aria-hidden="true"></i>
            Storage by Category
          </h2>
          <div class="mt-4">
            {#if fileTypeChartData.length > 0}
              <Chart data={fileTypeChartData} type="bar" size="md" />
            {:else}
              <div class="text-center py-12 text-gray-500 dark:text-gray-400">
                <i class="bi bi-inbox text-5xl mb-3" aria-hidden="true"></i>
                <p>No files yet</p>
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Detailed Breakdown -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
        <div class="glass-card p-6">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2"
          >
            <i class="bi bi-list-ul text-green-600" aria-hidden="true"></i>
            Detailed Breakdown
          </h2>
          <div class="space-y-4">
            {#each Object.entries(stats.byType) as [type, data]}
              {@const config = typeCategories[type]}
              {@const percentage = formatPercent(data.size, stats.totalSize)}
              {#if data.count > 0}
                <div class="space-y-2">
                  <div class="flex justify-between items-center">
                    <div class="flex items-center gap-2">
                      <i
                        class="bi bi-{config.icon} text-{config.color}-600 dark:text-{config.color}-400 text-xl"
                      ></i>
                      <span
                        class="font-semibold text-gray-900 dark:text-gray-100"
                        >{config.label}</span
                      >
                    </div>
                    <div class="text-right">
                      <div class="font-bold text-gray-900 dark:text-gray-100">
                        {formatSize(data.size)}
                      </div>
                      <div class="text-xs text-gray-500 dark:text-gray-400">
                        {data.count} files
                      </div>
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    <div
                      class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2 overflow-hidden"
                    >
                      <div
                        class="h-full bg-gradient-to-r from-green-500 to-emerald-600 transition-all duration-500"
                        style="width: {percentage}%"
                      ></div>
                    </div>
                    <span
                      class="text-sm font-semibold min-w-[3rem] text-right text-gray-700 dark:text-gray-300"
                    >
                      {percentage}%
                    </span>
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        </div>

        <div class="glass-card p-6">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2"
          >
            <i class="bi bi-graph-up text-green-600" aria-hidden="true"></i>
            Storage Distribution
          </h2>
          <div class="grid grid-cols-3 gap-4">
            {#each Object.entries(stats.byType).slice(0, 6) as [type, data]}
              {@const config = typeCategories[type]}
              {@const percentage = formatPercent(data.size, stats.totalSize)}
              {#if data.count > 0}
                <div class="flex flex-col items-center gap-2">
                  <!-- Radial Progress -->
                  <div class="relative w-20 h-20">
                    <svg class="w-20 h-20 transform -rotate-90">
                      <circle
                        cx="40"
                        cy="40"
                        r="32"
                        stroke="currentColor"
                        stroke-width="8"
                        fill="transparent"
                        class="text-gray-200 dark:text-gray-700"
                      />
                      <circle
                        cx="40"
                        cy="40"
                        r="32"
                        stroke="currentColor"
                        stroke-width="8"
                        fill="transparent"
                        stroke-dasharray="{(percentage / 100) * 201} 201"
                        class="text-{config.color}-600 dark:text-{config.color}-400 transition-all duration-500"
                      />
                    </svg>
                    <div
                      class="absolute inset-0 flex items-center justify-center"
                    >
                      <span
                        class="text-sm font-bold text-gray-900 dark:text-gray-100"
                        >{percentage}%</span
                      >
                    </div>
                  </div>
                  <div class="text-center">
                    <div
                      class="text-xs font-semibold text-gray-900 dark:text-gray-100"
                    >
                      {config.label}
                    </div>
                    <div class="text-xs text-gray-500 dark:text-gray-400">
                      {formatSize(data.size)}
                    </div>
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>

      <!-- Largest Files -->
      {#if stats.largestFiles.length > 0}
        <div class="glass-card p-6">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-2"
          >
            <i class="bi bi-sort-down-alt text-green-600" aria-hidden="true"></i>
            Largest Files
          </h2>
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead>
                <tr class="border-b border-gray-200 dark:border-gray-700">
                  <th
                    class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                    >#</th
                  >
                  <th
                    class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                    >{$t("name")}</th
                  >
                  <th
                    class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                    >{$t("size")}</th
                  >
                  <th
                    class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                    >% {$t("total")}</th
                  >
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                {#each stats.largestFiles as file, i}
                  <tr
                    class="hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors"
                  >
                    <td class="px-6 py-4 text-gray-700 dark:text-gray-300"
                      >{i + 1}</td
                    >
                    <td class="px-6 py-4">
                      <div class="flex items-center gap-2">
                        <i class="bi bi-file-earmark-fill text-green-600" aria-hidden="true"></i>
                        <span
                          class="font-medium text-gray-900 dark:text-gray-100"
                          >{file.name}</span
                        >
                      </div>
                    </td>
                    <td class="px-6 py-4">
                      <span
                        class="px-2 py-1 text-xs font-semibold bg-green-100 dark:bg-green-900/20 text-green-700 dark:text-green-300 rounded-full"
                      >
                        {formatSize(file.size)}
                      </span>
                    </td>
                    <td class="px-6 py-4">
                      <div class="flex items-center gap-2">
                        <div
                          class="w-20 bg-gray-200 dark:bg-gray-700 rounded-full h-2 overflow-hidden"
                        >
                          <div
                            class="h-full bg-gradient-to-r from-green-500 to-emerald-600 transition-all duration-500"
                            style="width: {formatPercent(
                              file.size,
                              stats.totalSize
                            )}%"
                          ></div>
                        </div>
                        <span class="text-sm text-gray-700 dark:text-gray-300">
                          {formatPercent(file.size, stats.totalSize)}%
                        </span>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>
