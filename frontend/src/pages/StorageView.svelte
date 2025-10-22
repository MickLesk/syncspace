<script>
  import { onMount } from "svelte";
  import Icon from "../components/ui/Icon.svelte";
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
                family: "Roboto",
                size: 14,
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

<div class="storage-dashboard">
  <div class="dashboard-header">
    <h2><Icon name="pie-chart" size={24} /> Storage Usage</h2>
  </div>

  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Loading statistics...</p>
    </div>
  {:else}
    <div class="dashboard-content">
      <!-- Overview Cards -->
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon" style="background: #4CAF50;">
            <Icon name="files" size={32} color="#fff" />
          </div>
          <div class="stat-content">
            <div class="stat-value">{stats.totalFiles}</div>
            <div class="stat-label">Total Files</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon" style="background: #2196F3;">
            <Icon name="hdd" size={32} color="#fff" />
          </div>
          <div class="stat-content">
            <div class="stat-value">{formatSize(stats.totalSize)}</div>
            <div class="stat-label">Total Size</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon" style="background: #FF9800;">
            <Icon name="file-earmark" size={32} color="#fff" />
          </div>
          <div class="stat-content">
            <div class="stat-value">
              {Object.values(stats.byType).reduce((sum, t) => sum + t.count, 0)}
            </div>
            <div class="stat-label">File Types</div>
          </div>
        </div>
      </div>

      <!-- Chart & Breakdown -->
      <div class="chart-section">
        <div class="chart-container">
          <h3>Storage by Type</h3>
          <canvas bind:this={chartCanvas}></canvas>
        </div>

        <div class="type-breakdown">
          <h3>Type Breakdown</h3>
          <div class="breakdown-list">
            {#each Object.entries(stats.byType) as [type, data]}
              {#if data.size > 0}
                <div class="breakdown-item">
                  <div class="breakdown-info">
                    <span class="breakdown-type"
                      >{type.charAt(0).toUpperCase() + type.slice(1)}</span
                    >
                    <span class="breakdown-count">{data.count} files</span>
                  </div>
                  <div class="breakdown-bar">
                    <div
                      class="breakdown-fill"
                      style="width: {formatPercent(data.size, stats.totalSize)}"
                    ></div>
                  </div>
                  <span class="breakdown-size">{formatSize(data.size)}</span>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>

      <!-- Largest Files -->
      {#if stats.largestFiles.length > 0}
        <div class="largest-files">
          <h3><Icon name="sort-down-alt" size={20} /> Largest Files</h3>
          <div class="file-list">
            {#each stats.largestFiles as file}
              <div class="file-item">
                <Icon name="file-earmark" size={20} />
                <span class="file-name">{file.name}</span>
                <span class="file-size">{formatSize(file.size)}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .storage-dashboard {
    padding: 32px;
    max-width: 1400px;
    margin: 0 auto;
  }

  .dashboard-header {
    margin-bottom: 32px;
  }

  .dashboard-header h2 {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 64px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .spinner {
    width: 48px;
    height: 48px;
    border: 4px solid var(--md-sys-color-surface-variant);
    border-top-color: var(--md-sys-color-primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 24px;
    margin-bottom: 32px;
  }

  .stat-card {
    background: var(--md-sys-color-surface);
    border-radius: 20px;
    padding: 24px;
    box-shadow: var(--md-elevation-1);
    display: flex;
    align-items: center;
    gap: 20px;
    transition: all 0.3s ease;
  }

  .stat-card:hover {
    box-shadow: var(--md-elevation-3);
    transform: translateY(-4px);
  }

  .stat-icon {
    width: 64px;
    height: 64px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .stat-content {
    flex: 1;
  }

  .stat-value {
    font-size: 32px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    line-height: 1;
    margin-bottom: 8px;
  }

  .stat-label {
    font-size: 14px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  .chart-section {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 32px;
    margin-bottom: 32px;
  }

  @media (max-width: 768px) {
    .chart-section {
      grid-template-columns: 1fr;
    }
  }

  .chart-container,
  .type-breakdown {
    background: var(--md-sys-color-surface);
    border-radius: 20px;
    padding: 24px;
    box-shadow: var(--md-elevation-1);
  }

  .chart-container h3,
  .type-breakdown h3,
  .largest-files h3 {
    font-size: 18px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 24px 0;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .chart-container canvas {
    max-height: 300px;
  }

  .breakdown-list {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .breakdown-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .breakdown-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .breakdown-type {
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
  }

  .breakdown-count {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .breakdown-bar {
    height: 8px;
    background: var(--md-sys-color-surface-variant);
    border-radius: 4px;
    overflow: hidden;
  }

  .breakdown-fill {
    height: 100%;
    background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
    transition: width 0.5s ease;
  }

  .breakdown-size {
    font-size: 14px;
    font-weight: 600;
    color: var(--md-sys-color-primary);
    text-align: right;
  }

  .largest-files {
    background: var(--md-sys-color-surface);
    border-radius: 20px;
    padding: 24px;
    box-shadow: var(--md-elevation-1);
  }

  .file-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    border-radius: 12px;
    background: var(--md-sys-color-surface-variant);
    transition: all 0.2s;
  }

  .file-item:hover {
    background: var(--md-sys-color-secondary-container);
  }

  .file-name {
    flex: 1;
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-size {
    font-size: 14px;
    font-weight: 600;
    color: var(--md-sys-color-primary);
  }
</style>
