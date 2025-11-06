<script>
  import { onMount, onDestroy } from "svelte";
  import {
    performanceMetrics,
    performanceHistory,
    cacheStats,
    backgroundJobs,
    systemInfo,
    performanceScore,
    performanceStatus,
    performanceMonitor,
    performanceUtils,
  } from "../../stores/performance.js";

  export let compact = false;

  let isMonitoring = false;
  let updateInterval = 30000; // 30 seconds
  let showAdvanced = false;
  let selectedTab = "overview";

  // Chart data for performance history
  let chartData = [];

  onMount(async () => {
    // Load initial data
    await performanceMonitor.loadSystemInfo();
    await performanceMonitor.loadCacheStats();
    await performanceMonitor.loadBackgroundJobs();

    // Check if auto-start is enabled
    const settings = performanceMonitor.loadSettings();
    if (settings.autoStart) {
      startMonitoring();
    }
  });

  onDestroy(() => {
    if (isMonitoring) {
      performanceMonitor.stopMonitoring();
    }
  });

  function startMonitoring() {
    isMonitoring = true;
    performanceMonitor.startMonitoring();
  }

  function stopMonitoring() {
    isMonitoring = false;
    performanceMonitor.stopMonitoring();
  }

  async function clearCache() {
    try {
      await performanceMonitor.clearCache();
      // Show success message
    } catch (error) {
      console.error("Failed to clear cache:", error);
      // Show error message
    }
  }

  async function queueTestJob() {
    try {
      const jobId = await performanceMonitor.queueJob(
        "thumbnail_generation",
        {
          file_path: "test/sample.jpg",
        },
        1
      );
      console.log("Test job queued:", jobId);
    } catch (error) {
      console.error("Failed to queue test job:", error);
    }
  }

  function exportData() {
    const data = performanceMonitor.exportData();
    const blob = new Blob([JSON.stringify(data, null, 2)], {
      type: "application/json",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `syncspace-performance-${new Date().toISOString().split("T")[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  // Update chart data when performance history changes
  $: if ($performanceHistory.length > 0) {
    chartData = $performanceHistory.map((point, index) => ({
      x: point.timestamp,
      y: point.cpu_usage,
      index,
    }));
  }
</script>

<div class="performance-monitor" class:compact>
  <div class="monitor-header">
    <div class="header-left">
      <h3>Performance Monitor</h3>
      {#if $performanceMetrics.last_updated}
        <span class="status-indicator {$performanceStatus.color}">
          {$performanceStatus.level} ({$performanceScore}/100)
        </span>
      {/if}
    </div>

    <div class="header-controls">
      {#if !compact}
        <button
          class="px-3 py-1.5 text-sm rounded-lg font-medium transition-colors {isMonitoring
            ? 'bg-red-500 hover:bg-red-600 text-white'
            : 'bg-green-500 hover:bg-green-600 text-white'}"
          onclick={isMonitoring ? stopMonitoring : startMonitoring}
        >
          {isMonitoring ? "Stop" : "Start"} Monitoring
        </button>

        <div class="relative group">
          <button
            class="px-3 py-1.5 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            >⚙️</button
          >
          <div
            class="absolute right-0 mt-2 bg-white dark:bg-gray-900 rounded-lg z-10 w-52 p-2 shadow-xl border border-gray-200 dark:border-gray-700 hidden group-hover:block"
          >
            <button
              class="w-full px-3 py-2 text-sm text-left rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
              onclick={clearCache}
            >
              Clear Cache
            </button>
            <button
              class="w-full px-3 py-2 text-sm text-left rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
              onclick={queueTestJob}
            >
              Queue Test Job
            </button>
            <button
              class="w-full px-3 py-2 text-sm text-left rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
              onclick={exportData}
            >
              Export Data
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>

  {#if compact}
    <!-- Compact view for dashboard -->
    <div class="compact-metrics">
      <div class="metric">
        <span class="metric-label">CPU</span>
        <span class="metric-value"
          >{$performanceMetrics.cpu_usage.toFixed(1)}%</span
        >
      </div>
      <div class="metric">
        <span class="metric-label">Cache Hit</span>
        <span class="metric-value"
          >{performanceUtils.formatPercentage(
            $performanceMetrics.cache_hit_ratio
          )}</span
        >
      </div>
      <div class="metric">
        <span class="metric-label">Response</span>
        <span class="metric-value"
          >{$performanceMetrics.average_response_time}ms</span
        >
      </div>
    </div>
  {:else}
    <!-- Full view -->
    <div class="flex gap-2 bg-gray-100 dark:bg-gray-800 rounded-lg p-1 mb-4">
      <button
        class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {selectedTab ===
        'overview'
          ? 'bg-white dark:bg-gray-900 text-blue-600 dark:text-blue-400 shadow'
          : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
        onclick={() => (selectedTab = "overview")}
      >
        Overview
      </button>
      <button
        class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {selectedTab ===
        'cache'
          ? 'bg-white dark:bg-gray-900 text-blue-600 dark:text-blue-400 shadow'
          : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
        onclick={() => (selectedTab = "cache")}
      >
        Cache
      </button>
      <button
        class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {selectedTab ===
        'jobs'
          ? 'bg-white dark:bg-gray-900 text-blue-600 dark:text-blue-400 shadow'
          : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
        onclick={() => (selectedTab = "jobs")}
      >
        Jobs
      </button>
      <button
        class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {selectedTab ===
        'system'
          ? 'bg-white dark:bg-gray-900 text-blue-600 dark:text-blue-400 shadow'
          : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
        onclick={() => (selectedTab = "system")}
      >
        System
      </button>
    </div>

    {#if selectedTab === "overview"}
      <div class="metrics-grid">
        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">CPU Usage</span>
            <span class="metric-value"
              >{$performanceMetrics.cpu_usage.toFixed(1)}%</span
            >
          </div>
          <div class="metric-progress">
            <progress
              class="progress progress-info w-full"
              value={$performanceMetrics.cpu_usage}
              max="100"
            ></progress>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">Memory Usage</span>
            <span class="metric-value"
              >{performanceUtils.formatBytes(
                $performanceMetrics.memory_usage
              )}</span
            >
          </div>
          <div class="metric-progress">
            <progress
              class="progress progress-warning w-full"
              value={$performanceMetrics.memory_usage}
              max="1000000000"
            ></progress>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">Cache Hit Ratio</span>
            <span class="metric-value"
              >{performanceUtils.formatPercentage(
                $performanceMetrics.cache_hit_ratio
              )}</span
            >
          </div>
          <div class="metric-progress">
            <progress
              class="progress progress-success w-full"
              value={$performanceMetrics.cache_hit_ratio * 100}
              max="100"
            ></progress>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">Response Time</span>
            <span class="metric-value"
              >{$performanceMetrics.average_response_time}ms</span
            >
          </div>
          <div class="metric-progress">
            <progress
              class="progress progress-accent w-full"
              value={Math.min($performanceMetrics.average_response_time, 200)}
              max="200"
            ></progress>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">Active Connections</span>
            <span class="metric-value"
              >{$performanceMetrics.active_connections}</span
            >
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">Disk Usage</span>
            <span class="metric-value"
              >{performanceUtils.formatBytes(
                $performanceMetrics.disk_usage
              )}</span
            >
          </div>
        </div>
      </div>

      {#if $performanceHistory.length > 0}
        <div class="chart-container">
          <h4>Performance History</h4>
          <div class="simple-chart">
            {#each chartData as point, index}
              <div
                class="chart-bar"
                style="height: {point.y}%; left: {(index / chartData.length) *
                  100}%;"
                title="CPU: {point.y.toFixed(
                  1
                )}% at {point.x.toLocaleTimeString()}"
              ></div>
            {/each}
          </div>
        </div>
      {/if}
    {:else if selectedTab === "cache"}
      <div class="cache-stats">
        <div
          class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 shadow-sm"
        >
          <div
            class="bg-white dark:bg-gray-900 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
          >
            <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              Memory Cache Entries
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">
              {$cacheStats.memory_cache_entries}
            </div>
          </div>

          <div
            class="bg-white dark:bg-gray-900 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
          >
            <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              Redis Connected
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">
              {$cacheStats.redis_connected ? "✅" : "❌"}
            </div>
          </div>

          <div
            class="bg-white dark:bg-gray-900 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
          >
            <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              Cache Hit Ratio
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">
              {performanceUtils.formatPercentage($cacheStats.cache_hit_ratio)}
            </div>
          </div>

          <div
            class="bg-white dark:bg-gray-900 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
          >
            <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              Total Requests
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">
              {$cacheStats.total_requests.toLocaleString()}
            </div>
          </div>
        </div>

        <div class="cache-actions mt-4">
          <button
            class="px-4 py-2 bg-amber-500 hover:bg-amber-600 text-white rounded-lg font-medium transition-colors"
            onclick={clearCache}
          >
            Clear All Caches
          </button>
        </div>
      </div>
    {:else if selectedTab === "jobs"}
      <div class="jobs-section">
        <div class="jobs-stats">
          <div class="stat-item">
            <span class="stat-label">Queue Length:</span>
            <span class="stat-value">{$backgroundJobs.queue_length}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Active Workers:</span>
            <span class="stat-value">{$backgroundJobs.active_workers}</span>
          </div>
        </div>

        {#if $backgroundJobs.jobs.length > 0}
          <div class="jobs-list">
            <h5>Current Jobs</h5>
            {#each $backgroundJobs.jobs as job}
              <div class="job-item">
                <span class="job-type">{job.type}</span>
                <span class="job-status">{job.status}</span>
                <span class="job-time">{job.created_at}</span>
              </div>
            {/each}
          </div>
        {:else}
          <div class="no-jobs">
            <p>No background jobs currently running</p>
            <button
              class="px-3 py-1.5 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors"
              onclick={queueTestJob}
            >
              Queue Test Job
            </button>
          </div>
        {/if}
      </div>
    {:else if selectedTab === "system"}
      <div class="system-info">
        <div class="info-grid">
          <!-- OS Information -->
          <div class="info-section">
            <h5
              class="text-lg font-semibold text-gray-900 dark:text-white mb-3"
            >
              <i class="bi bi-laptop mr-2"></i>Operating System
            </h5>
            <div class="space-y-2">
              <div class="info-item">
                <span class="info-label">OS Type:</span>
                <span
                  class="info-value font-semibold text-blue-600 dark:text-blue-400"
                >
                  {$systemInfo.os_type || $systemInfo.os_name || "Unknown"}
                </span>
              </div>
              <div class="info-item">
                <span class="info-label">Version:</span>
                <span class="info-value"
                  >{$systemInfo.os_version || "Unknown"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Kernel:</span>
                <span class="info-value"
                  >{$systemInfo.kernel_version || "Unknown"}</span
                >
              </div>
            </div>
          </div>

          <!-- CPU Information -->
          <div class="info-section">
            <h5
              class="text-lg font-semibold text-gray-900 dark:text-white mb-3"
            >
              <i class="bi bi-cpu mr-2"></i>Processor
            </h5>
            <div class="space-y-2">
              <div class="info-item">
                <span class="info-label">Cores:</span>
                <span class="info-value font-semibold"
                  >{$systemInfo.cpu_cores || 0}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Brand:</span>
                <span class="info-value"
                  >{$systemInfo.cpu_brand || "Unknown"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Usage:</span>
                <span class="info-value">
                  {$systemInfo.cpu_usage
                    ? $systemInfo.cpu_usage.toFixed(1)
                    : 0}%
                </span>
              </div>
            </div>
          </div>

          <!-- Memory Information -->
          <div class="info-section">
            <h5
              class="text-lg font-semibold text-gray-900 dark:text-white mb-3"
            >
              <i class="bi bi-memory mr-2"></i>Memory
            </h5>
            <div class="space-y-2">
              <div class="info-item">
                <span class="info-label">Total:</span>
                <span class="info-value font-semibold"
                  >{$systemInfo.memory_total || "Unknown"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Used:</span>
                <span class="info-value"
                  >{$systemInfo.memory_used || "Unknown"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Available:</span>
                <span class="info-value text-green-600 dark:text-green-400">
                  {$systemInfo.memory_available || "Unknown"}
                </span>
              </div>
              {#if $systemInfo.memory_usage_percent}
                <div class="info-item">
                  <span class="info-label">Usage:</span>
                  <span class="info-value"
                    >{$systemInfo.memory_usage_percent.toFixed(1)}%</span
                  >
                </div>
                <div class="mt-2">
                  <progress
                    class="progress progress-warning w-full"
                    value={$systemInfo.memory_usage_percent}
                    max="100"
                  ></progress>
                </div>
              {/if}
            </div>
          </div>

          <!-- Storage Information -->
          <div class="info-section">
            <h5
              class="text-lg font-semibold text-gray-900 dark:text-white mb-3"
            >
              <i class="bi bi-hdd mr-2"></i>Storage
            </h5>
            <div class="space-y-2">
              <div class="info-item">
                <span class="info-label">Total Space:</span>
                <span class="info-value font-semibold"
                  >{$systemInfo.disk_space || "Unknown"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Used:</span>
                <span class="info-value"
                  >{$systemInfo.disk_used || "Unknown"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Available:</span>
                <span class="info-value text-green-600 dark:text-green-400">
                  {$systemInfo.disk_available || "Unknown"}
                </span>
              </div>
              {#if $systemInfo.disk_usage_percent}
                <div class="info-item">
                  <span class="info-label">Usage:</span>
                  <span class="info-value"
                    >{$systemInfo.disk_usage_percent.toFixed(1)}%</span
                  >
                </div>
                <div class="mt-2">
                  <progress
                    class="progress progress-info w-full"
                    value={$systemInfo.disk_usage_percent}
                    max="100"
                  ></progress>
                </div>
              {/if}
            </div>
          </div>

          <!-- Application Info -->
          <div class="info-section">
            <h5
              class="text-lg font-semibold text-gray-900 dark:text-white mb-3"
            >
              <i class="bi bi-info-circle mr-2"></i>Application
            </h5>
            <div class="space-y-2">
              <div class="info-item">
                <span class="info-label">Version:</span>
                <span class="info-value font-semibold"
                  >{$systemInfo.version || "0.3.0"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Rust Version:</span>
                <span class="info-value"
                  >{$systemInfo.rust_version || "Unknown"}</span
                >
              </div>
              <div class="info-item">
                <span class="info-label">Uptime:</span>
                <span class="info-value text-blue-600 dark:text-blue-400">
                  {$systemInfo.uptime || "Unknown"}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- Disk Details Section -->
        {#if $systemInfo.disks && $systemInfo.disks.length > 0}
          <div class="disk-details mt-6">
            <h5
              class="text-lg font-semibold text-gray-900 dark:text-white mb-3"
            >
              <i class="bi bi-hdd-stack mr-2"></i>Disk Details
            </h5>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              {#each $systemInfo.disks as disk}
                <div
                  class="disk-card bg-white dark:bg-gray-900 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
                >
                  <div class="flex items-start justify-between mb-3">
                    <div>
                      <div class="font-semibold text-gray-900 dark:text-white">
                        {disk.name}
                      </div>
                      <div class="text-sm text-gray-600 dark:text-gray-400">
                        {disk.mount_point}
                      </div>
                    </div>
                    <span
                      class="px-2 py-1 text-xs font-medium rounded-full {disk.type.includes(
                        'NVMe'
                      )
                        ? 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200'
                        : disk.type.includes('SSD')
                          ? 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200'
                          : disk.type.includes('Removable')
                            ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'
                            : 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200'}"
                    >
                      {disk.type}
                    </span>
                  </div>
                  <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                      <span class="text-gray-600 dark:text-gray-400"
                        >File System:</span
                      >
                      <span class="text-gray-900 dark:text-white"
                        >{disk.file_system}</span
                      >
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600 dark:text-gray-400"
                        >Total:</span
                      >
                      <span class="text-gray-900 dark:text-white">
                        {performanceUtils.formatBytes(disk.total_space)}
                      </span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600 dark:text-gray-400"
                        >Available:</span
                      >
                      <span class="text-green-600 dark:text-green-400">
                        {performanceUtils.formatBytes(disk.available_space)}
                      </span>
                    </div>
                    <div class="mt-2">
                      <progress
                        class="progress progress-info w-full h-2"
                        value={disk.total_space - disk.available_space}
                        max={disk.total_space}
                      ></progress>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Features Section -->
        {#if $systemInfo.features}
          <div class="features-section mt-6">
            <h5
              class="text-lg font-semibold text-gray-900 dark:text-white mb-3"
            >
              <i class="bi bi-toggles mr-2"></i>Features
            </h5>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
              {#each Object.entries($systemInfo.features) as [feature, enabled]}
                <div
                  class="feature-item bg-white dark:bg-gray-900 rounded-lg p-3 border border-gray-200 dark:border-gray-700 flex items-center justify-between"
                >
                  <span
                    class="text-sm text-gray-700 dark:text-gray-300 capitalize"
                  >
                    {feature.replace(/_/g, " ")}
                  </span>
                  <span class="text-lg">
                    {enabled ? "✅" : "❌"}
                  </span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .performance-monitor {
    background: white;
    dark: bg-gray-900;
    border-radius: 16px;
    padding: 24px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .performance-monitor.compact {
    padding: 16px;
  }

  .monitor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  .header-left h3 {
    font-size: 20px;
    font-weight: 600;
    margin: 0 0 8px 0;
    color: var(--md-sys-color-on-surface);
  }

  .status-indicator {
    display: inline-block;
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 13px;
    font-weight: 500;
  }

  .status-indicator.green {
    background: #10b981;
    color: white;
  }

  .status-indicator.yellow {
    background: #f59e0b;
    color: white;
  }

  .status-indicator.red {
    background: #ef4444;
    color: white;
  }

  .header-controls {
    display: flex;
    gap: 12px;
    align-items: center;
  }

  .compact-metrics {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
  }

  .metric {
    text-align: center;
  }

  .metric-label {
    display: block;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    margin-bottom: 4px;
  }

  .metric-value {
    display: block;
    font-size: 20px;
    font-weight: 600;
    color: var(--md-sys-color-primary);
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    margin-bottom: 24px;
  }

  .metric-card {
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
    padding: 16px;
    border: 1px solid var(--md-sys-color-outline-variant);
  }

  .metric-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .metric-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
  }

  .metric-card .metric-value {
    font-size: 24px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .metric-progress {
    margin-top: 8px;
  }

  .chart-container {
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
    padding: 20px;
    border: 1px solid var(--md-sys-color-outline-variant);
  }

  .chart-container h4 {
    font-size: 16px;
    font-weight: 600;
    margin: 0 0 16px 0;
    color: var(--md-sys-color-on-surface);
  }

  .simple-chart {
    position: relative;
    height: 150px;
    background: var(--md-sys-color-surface);
    border-radius: 8px;
    overflow: hidden;
  }

  .chart-bar {
    position: absolute;
    bottom: 0;
    width: 2px;
    background: var(--md-sys-color-primary);
    transition: height 0.3s ease;
  }

  .cache-stats,
  .jobs-section,
  .system-info {
    margin-top: 16px;
  }

  .jobs-stats {
    display: flex;
    gap: 24px;
    margin-bottom: 16px;
    padding: 16px;
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
  }

  .stat-item {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .stat-label {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .stat-value {
    font-size: 18px;
    font-weight: 600;
    color: var(--md-sys-color-primary);
  }

  .jobs-list h5,
  .features-section h5 {
    font-size: 14px;
    font-weight: 600;
    margin: 16px 0 12px 0;
    color: var(--md-sys-color-on-surface);
  }

  .job-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--md-sys-color-surface-container-low);
    border-radius: 8px;
    margin-bottom: 8px;
  }

  .job-type {
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
  }

  .job-status {
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .no-jobs {
    text-align: center;
    padding: 40px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .info-section {
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
    padding: 20px;
    border: 1px solid var(--md-sys-color-outline-variant);
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
    margin-bottom: 24px;
  }

  .info-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 0;
  }

  .info-label {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .info-value {
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
  }

  @media (max-width: 768px) {
    .compact-metrics {
      grid-template-columns: 1fr;
    }

    .metrics-grid {
      grid-template-columns: 1fr;
    }

    .info-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
