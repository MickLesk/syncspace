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
  } from "../stores/performance.js";

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
          <div class="info-item">
            <span class="info-label">CPU Cores:</span>
            <span class="info-value">{$systemInfo.cpu_cores}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Version:</span>
            <span class="info-value">{$systemInfo.version}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Rust Version:</span>
            <span class="info-value">{$systemInfo.rust_version}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Memory Total:</span>
            <span class="info-value">{$systemInfo.memory_total}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Disk Space:</span>
            <span class="info-value">{$systemInfo.disk_space}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Uptime:</span>
            <span class="info-value">{$systemInfo.uptime}</span>
          </div>
        </div>

        {#if $systemInfo.features}
          <div class="features-section">
            <h5>Features</h5>
            <div class="features-grid">
              {#each Object.entries($systemInfo.features) as [feature, enabled]}
                <div class="feature-item">
                  <span class="feature-name">{feature}</span>
                  <span
                    class="feature-status"
                    class:enabled
                    class:disabled={!enabled}
                  >
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

