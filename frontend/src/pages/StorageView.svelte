<script>
  import { onMount } from "svelte";
  import Icon from "../components/ui/Icon.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import StatCard from "../components/ui/StatCard.svelte";
  import ChartCard from "../components/ui/ChartCard.svelte";
  import Spinner from "../components/ui/Spinner.svelte";
  import api from "../lib/api";
  import { error as errorToast } from "../stores/toast";

  let loading = true;
  let stats = {
    totalFiles: 0,
    totalSize: 0,
    byType: {},
    largestFiles: [],
    folderSizes: [],
  };

  let chartCanvas;
  let chart = null;

  onMount(async () => {
    await loadStats();
    renderChart();
  });

  /**
   * Load storage statistics from backend
   */
  async function loadStats() {
    loading = true;
    try {
      // Get basic stats
      const basicStats = await api.files.getStats();

      // Get file list to compute advanced stats
      const files = await api.files.list("");

      stats = {
        totalFiles: basicStats.file_count || 0,
        totalSize: basicStats.total_size || 0,
        byType: computeTypeStats(files),
        largestFiles: getLargestFiles(files, 5),
        folderSizes: getFolderSizes(files, 5),
      };
    } catch (err) {
      console.error("Failed to load stats:", err);
      errorToast("Failed to load storage statistics");
    }
    loading = false;
  }

  /**
   * Compute statistics by file type
   */
  function computeTypeStats(files) {
    const typeStats = {
      images: { count: 0, size: 0 },
      documents: { count: 0, size: 0 },
      videos: { count: 0, size: 0 },
      audio: { count: 0, size: 0 },
      archives: { count: 0, size: 0 },
      other: { count: 0, size: 0 },
    };

    const categories = {
      images: [
        ".jpg",
        ".jpeg",
        ".png",
        ".gif",
        ".bmp",
        ".webp",
        ".svg",
        ".ico",
      ],
      documents: [
        ".pdf",
        ".doc",
        ".docx",
        ".txt",
        ".md",
        ".rtf",
        ".odt",
        ".xls",
        ".xlsx",
        ".ppt",
        ".pptx",
      ],
      videos: [".mp4", ".avi", ".mov", ".mkv", ".webm", ".flv", ".wmv"],
      audio: [".mp3", ".wav", ".ogg", ".m4a", ".flac", ".aac"],
      archives: [".zip", ".rar", ".7z", ".tar", ".gz", ".bz2"],
    };

    for (const file of files) {
      if (file.is_dir) continue;

      const ext = file.name.toLowerCase().slice(file.name.lastIndexOf("."));
      let category = "other";

      for (const [cat, extensions] of Object.entries(categories)) {
        if (extensions.includes(ext)) {
          category = cat;
          break;
        }
      }

      typeStats[category].count++;
      typeStats[category].size += file.size || 0;
    }

    return typeStats;
  }

  /**
   * Get N largest files
   */
  function getLargestFiles(files, n) {
    return files
      .filter((f) => !f.is_dir)
      .sort((a, b) => (b.size || 0) - (a.size || 0))
      .slice(0, n);
  }

  /**
   * Get N largest folders (placeholder - needs recursive calculation)
   */
  function getFolderSizes(files, n) {
    // Simplified: just list folders with file count
    return files
      .filter((f) => f.is_dir)
      .map((f) => ({
        name: f.name,
        size: 0, // Would need recursive calculation
        count: 0, // Would need recursive calculation
      }))
      .slice(0, n);
  }

  /**
   * Render pie chart using Chart.js (loaded from CDN)
   */
  function renderChart() {
    // @ts-ignore - Chart.js loaded from CDN
    if (!chartCanvas || !window.Chart) return;

    const ctx = chartCanvas.getContext("2d");

    const colors = {
      images: "#4CAF50",
      documents: "#2196F3",
      videos: "#9C27B0",
      audio: "#FF9800",
      archives: "#795548",
      other: "#9E9E9E",
    };

    const data = Object.entries(stats.byType)
      .map(([type, data]) => ({
        label: type.charAt(0).toUpperCase() + type.slice(1),
        value: data.size,
        count: data.count,
        color: colors[type],
      }))
      .filter((item) => item.value > 0);

    if (chart) chart.destroy();

    // @ts-ignore - Chart.js loaded from CDN
    chart = new window.Chart(ctx, {
      type: "doughnut",
      data: {
        labels: data.map((d) => d.label),
        datasets: [
          {
            data: data.map((d) => d.value),
            backgroundColor: data.map((d) => d.color),
            borderWidth: 0,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: "bottom",
            labels: {
              padding: 20,
              font: {
                family: "Inter",
                size: 14,
                weight: 500,
              },
            },
          },
          tooltip: {
            callbacks: {
              label: function (context) {
                const item = data[context.dataIndex];
                return `${item.label}: ${formatSize(item.value)} (${item.count} files)`;
              },
            },
          },
        },
      },
    });
  }

  /**
   * Format file size
   */
  function formatSize(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  /**
   * Format percentage
   */
  function formatPercent(value, total) {
    if (total === 0) return "0%";
    return ((value / total) * 100).toFixed(1) + "%";
  }
</script>

<svelte:head>
  <script
    src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.min.js"
  ></script>
</svelte:head>

<div class="storage-view">
  <PageHeader
    title="Storage Analytics"
    subtitle=""
    icon="hdd-fill"
    gradient="green"
  />

  <div class="page-content">
    {#if loading}
      <div class="loading">
        <Spinner size="large" />
        <p>Loading storage data...</p>
      </div>
    {:else}
      <!-- Stats Grid with StatCard -->
      <div class="stats-grid">
        <StatCard
          icon="bi-files"
          label="Total Files"
          value={stats.totalFiles}
          gradient="linear-gradient(135deg, #10b981, #34d399)"
        />

        <StatCard
          icon="bi-hdd-fill"
          label="Storage Used"
          value={formatSize(stats.totalSize)}
          gradient="linear-gradient(135deg, #3b82f6, #60a5fa)"
        />

        <StatCard
          icon="bi-file-earmark"
          label="File Types"
          value={Object.keys(stats.byType).filter(
            (k) => stats.byType[k].size > 0
          ).length}
          gradient="linear-gradient(135deg, #f59e0b, #fbbf24)"
        />
      </div>

      <!-- Chart & Breakdown -->
      <div class="content-grid">
        <!-- Chart Card -->
        <ChartCard title="Usage by Type" icon="bi-pie-chart-fill">
          <canvas bind:this={chartCanvas}></canvas>
        </ChartCard>

        <!-- Breakdown Card -->
        <div class="glass-card">
          <div class="card-header">
            <div
              class="head-icon"
              style="background: rgba(99, 102, 241, 0.12);"
            >
              <Icon name="list-ul" size={20} color="#6366f1" />
            </div>
            <h3>Type Breakdown</h3>
          </div>
          <div class="breakdown">
            {#each Object.entries(stats.byType) as [type, data]}
              {#if data.size > 0}
                <div class="item">
                  <div class="info">
                    <span class="type"
                      >{type.charAt(0).toUpperCase() + type.slice(1)}</span
                    >
                    <span class="count">{data.count} files</span>
                  </div>
                  <div class="bar">
                    <div
                      class="fill"
                      style="width: {formatPercent(data.size, stats.totalSize)}"
                    ></div>
                  </div>
                  <span class="size">{formatSize(data.size)}</span>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>

      <!-- Largest Files -->
      {#if stats.largestFiles.length > 0}
        <div class="glass-card">
          <div class="card-header">
            <div class="head-icon" style="background: rgba(239, 68, 68, 0.12);">
              <Icon name="sort-down-alt" size={20} color="#ef4444" />
            </div>
            <h3>Largest Files</h3>
          </div>
          <div class="files">
            {#each stats.largestFiles as file}
              <div class="file">
                <Icon name="file-earmark-fill" size={16} />
                <span class="name">{file.name}</span>
                <span class="size">{formatSize(file.size)}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .storage-view {
    min-height: 100vh;
    background: var(--md-sys-color-surface);
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
    margin-bottom: 32px;
  }

  /* Content Grid */
  .content-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 24px;
    margin-bottom: 24px;
  }

  /* Card Header */
  .card-header {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 24px;
  }

  .head-icon {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15);
  }

  .card-header h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin: 0;
    letter-spacing: -0.3px;
  }

  /* Canvas */
  canvas {
    max-height: 320px;
    margin: 0 auto;
    display: block;
  }

  /* Breakdown */
  .breakdown {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .breakdown .item {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .type {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
  }

  .count {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  .bar {
    height: 8px;
    background: rgba(15, 23, 42, 0.06);
    border-radius: 4px;
    overflow: hidden;
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .fill {
    height: 100%;
    background: linear-gradient(90deg, #6366f1 0%, #8b5cf6 100%);
    transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 0 8px rgba(99, 102, 241, 0.4);
  }

  .breakdown .size {
    font-size: 14px;
    font-weight: 700;
    color: #6366f1;
    text-align: right;
  }

  /* Files List */
  .files {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .file {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.4);
    border: 1px solid rgba(15, 23, 42, 0.06);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .file:hover {
    background: rgba(99, 102, 241, 0.08);
    border-color: rgba(99, 102, 241, 0.15);
    transform: translateX(4px);
  }

  .file .name {
    flex: 1;
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file .size {
    font-size: 13px;
    font-weight: 700;
    color: #6366f1;
  }

  /* Loading State */
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 64px 32px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .loading p {
    font-size: 14px;
    font-weight: 500;
  }

  @keyframes float {
    0%,
    100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-10px);
    }
  }

  /* Dark Mode */
  @media (prefers-color-scheme: dark) {
    .file {
      background: rgba(255, 255, 255, 0.04);
      border-color: rgba(255, 255, 255, 0.06);
    }

    .file:hover {
      background: rgba(99, 102, 241, 0.12);
      border-color: rgba(99, 102, 241, 0.2);
    }

    .bar {
      background: rgba(255, 255, 255, 0.08);
    }
  }

  /* Responsive */
  @media (max-width: 768px) {
    .stats-grid {
      grid-template-columns: 1fr;
      gap: 16px;
    }

    .content-grid {
      grid-template-columns: 1fr;
      gap: 20px;
    }

    .head-icon {
      width: 40px;
      height: 40px;
    }

    .card-header h3 {
      font-size: 16px;
    }

    canvas {
      max-height: 260px;
    }
  }
</style>
