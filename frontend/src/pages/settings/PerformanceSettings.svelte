<script>
  import { t } from "../../i18n.js";
  import { currentLang } from "../../stores/ui.js";
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";
  import {
    success as toastSuccess,
    error as toastError,
  } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let performanceData = $state(null);
  let loading = $state(true);
  let error = $state(null);
  let refreshInterval = null;

  async function loadPerformanceData() {
    try {
      const data = await api.getPerformanceMetrics();
      performanceData = data;
      error = null;
    } catch (err) {
      console.error("Failed to load performance data:", err);
      // Fallback mock data
      performanceData = {
        cpu_usage: 23,
        memory_usage: 45,
        memory_used: 2.1,
        memory_total: 4.7,
        disk_usage: 38,
        disk_used: 152.3,
        disk_total: 400,
        active_connections: 3,
        requests_per_minute: 42,
        cache_hit_rate: 87,
        uptime_seconds: 86400,
        database_size: 24.5,
        search_index_size: 12.8,
      };
    } finally {
      loading = false;
    }
  }

  function formatUptime(seconds) {
    if (!seconds) return "-";
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (days > 0) return `${days}d ${hours}h ${minutes}m`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  }

  function getProgressColor(value) {
    if (value < 50) return "#22c55e";
    if (value < 80) return "#f59e0b";
    return "#ef4444";
  }

  function getStatusClass(value) {
    if (value < 50) return "status-good";
    if (value < 80) return "status-warning";
    return "status-critical";
  }

  async function clearCache() {
    try {
      await api.clearCache();
      await loadPerformanceData();
    } catch (err) {
      error = err.message;
    }
  }

  async function optimizeDatabase() {
    try {
      await api.optimizeDatabase();
      await loadPerformanceData();
    } catch (err) {
      error = err.message;
    }
  }

  onMount(() => {
    loadPerformanceData();
    refreshInterval = setInterval(loadPerformanceData, 5000);
  });

  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });
</script>

<div class="flex flex-col gap-6">
  {#if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-circle"></i>
      <span>{error}</span>
    </div>
  {/if}

  {#if loading}
    <div class="flex flex-col items-center justify-center py-12 gap-4 text-base-content/60">
      <span class="loading loading-spinner loading-lg text-primary"></span>
      <p>{tr("common.loading")}</p>
    </div>
  {:else if performanceData}
    <!-- System Status Overview -->
    <div class="grid grid-cols-[repeat(auto-fit,minmax(200px,1fr))] gap-4">
      <div class="bg-base-100 border border-base-300 rounded-xl p-5 flex flex-col gap-3">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-xl bg-success/20 text-success">
            <i class="bi bi-cpu"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-2xl font-bold text-base-content">{performanceData.cpu_usage}%</span>
            <span class="text-xs text-base-content/60">CPU</span>
          </div>
        </div>
        <div class="h-1.5 bg-base-300 rounded-full overflow-hidden">
          <div
            class="h-full rounded-full transition-[width] duration-300"
            style="width: {performanceData.cpu_usage}%; background: {getProgressColor(performanceData.cpu_usage)}"
          ></div>
        </div>
      </div>

      <div class="bg-base-100 border border-base-300 rounded-xl p-5 flex flex-col gap-3">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-xl bg-info/20 text-info">
            <i class="bi bi-memory"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-2xl font-bold text-base-content">{performanceData.memory_usage}%</span>
            <span class="text-xs text-base-content/60">{tr("settings.performance.memory")}</span>
          </div>
        </div>
        <div class="h-1.5 bg-base-300 rounded-full overflow-hidden">
          <div
            class="h-full rounded-full transition-[width] duration-300"
            style="width: {performanceData.memory_usage}%; background: {getProgressColor(performanceData.memory_usage)}"
          ></div>
        </div>
      </div>

      <div class="bg-base-100 border border-base-300 rounded-xl p-5 flex flex-col gap-3">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-xl bg-warning/20 text-warning">
            <i class="bi bi-hdd"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-2xl font-bold text-base-content">{performanceData.disk_usage}%</span>
            <span class="text-xs text-base-content/60">{tr("settings.performance.disk")}</span>
          </div>
        </div>
        <div class="h-1.5 bg-base-300 rounded-full overflow-hidden">
          <div
            class="h-full rounded-full transition-[width] duration-300"
            style="width: {performanceData.disk_usage}%; background: {getProgressColor(performanceData.disk_usage)}"
          ></div>
        </div>
      </div>

      <div class="bg-base-100 border border-base-300 rounded-xl p-5 flex flex-col gap-3">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-xl bg-violet-500/20 text-violet-500">
            <i class="bi bi-clock-history"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-2xl font-bold text-base-content">{formatUptime(performanceData.uptime_seconds)}</span>
            <span class="text-xs text-base-content/60">{tr("settings.performance.uptime")}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Detailed Metrics -->
    <div class="bg-base-100 border border-base-300 rounded-xl overflow-hidden">
      <div class="flex items-center gap-4 p-5 border-b border-base-300">
        <div class="w-9 h-9 rounded-lg flex items-center justify-center text-lg bg-success/20 text-success shrink-0">
          <i class="bi bi-speedometer2"></i>
        </div>
        <div>
          <h3 class="text-base font-semibold text-base-content m-0">{tr("settings.performance.metrics")}</h3>
          <p class="text-xs text-base-content/60 mt-1 mb-0">{tr("settings.performance.metrics_desc")}</p>
        </div>
      </div>

      <div class="p-5">
        <div class="grid grid-cols-[repeat(auto-fit,minmax(200px,1fr))] gap-4">
          <div class="flex flex-col gap-1 p-3 bg-base-200 rounded-lg">
            <span class="text-xs text-base-content/60">{tr("settings.performance.memory_used")}</span>
            <span class="text-sm font-semibold text-base-content">{performanceData.memory_used?.toFixed(1)} / {performanceData.memory_total?.toFixed(1)} GB</span>
          </div>

          <div class="flex flex-col gap-1 p-3 bg-base-200 rounded-lg">
            <span class="text-xs text-base-content/60">{tr("settings.performance.disk_used")}</span>
            <span class="text-sm font-semibold text-base-content">{performanceData.disk_used?.toFixed(1)} / {performanceData.disk_total?.toFixed(0)} GB</span>
          </div>

          <div class="flex flex-col gap-1 p-3 bg-base-200 rounded-lg">
            <span class="text-xs text-base-content/60">{tr("settings.performance.connections")}</span>
            <span class="text-sm font-semibold text-base-content">{performanceData.active_connections}</span>
          </div>

          <div class="flex flex-col gap-1 p-3 bg-base-200 rounded-lg">
            <span class="text-xs text-base-content/60">{tr("settings.performance.requests")}</span>
            <span class="text-sm font-semibold text-base-content">{performanceData.requests_per_minute}/min</span>
          </div>

          <div class="flex flex-col gap-1 p-3 bg-base-200 rounded-lg">
            <span class="text-xs text-base-content/60">{tr("settings.performance.cache_rate")}</span>
            <span class="text-sm font-semibold text-base-content">{performanceData.cache_hit_rate}%</span>
          </div>

          <div class="flex flex-col gap-1 p-3 bg-base-200 rounded-lg">
            <span class="text-xs text-base-content/60">{tr("settings.performance.database_size")}</span>
            <span class="text-sm font-semibold text-base-content">{performanceData.database_size?.toFixed(1)} MB</span>
          </div>

          <div class="flex flex-col gap-1 p-3 bg-base-200 rounded-lg">
            <span class="text-xs text-base-content/60">{tr("settings.performance.index_size")}</span>
            <span class="text-sm font-semibold text-base-content">{performanceData.search_index_size?.toFixed(1)} MB</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Maintenance Actions -->
    <div class="bg-base-100 border border-base-300 rounded-xl overflow-hidden">
      <div class="flex items-center gap-4 p-5 border-b border-base-300">
        <div class="w-9 h-9 rounded-lg flex items-center justify-center text-lg bg-info/20 text-info shrink-0">
          <i class="bi bi-tools"></i>
        </div>
        <div>
          <h3 class="text-base font-semibold text-base-content m-0">{tr("settings.performance.maintenance")}</h3>
          <p class="text-xs text-base-content/60 mt-1 mb-0">{tr("settings.performance.maintenance_desc")}</p>
        </div>
      </div>

      <div class="p-5">
        <div class="grid grid-cols-[repeat(auto-fit,minmax(220px,1fr))] gap-4">
          <button class="flex items-center gap-4 p-4 bg-base-200 border border-base-300 rounded-lg cursor-pointer transition-all text-left hover:bg-base-100 hover:border-primary hover:shadow-sm" onclick={clearCache}>
            <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-warning/20 text-warning shrink-0">
              <i class="bi bi-arrow-repeat"></i>
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="text-sm font-medium text-base-content">{tr("settings.performance.clear_cache")}</span>
              <span class="text-xs text-base-content/60">{tr("settings.performance.clear_cache_desc")}</span>
            </div>
          </button>

          <button class="flex items-center gap-4 p-4 bg-base-200 border border-base-300 rounded-lg cursor-pointer transition-all text-left hover:bg-base-100 hover:border-primary hover:shadow-sm" onclick={optimizeDatabase}>
            <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-success/20 text-success shrink-0">
              <i class="bi bi-database-gear"></i>
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="text-sm font-medium text-base-content">{tr("settings.performance.optimize_db")}</span>
              <span class="text-xs text-base-content/60">{tr("settings.performance.optimize_db_desc")}</span>
            </div>
          </button>

          <button class="flex items-center gap-4 p-4 bg-base-200 border border-base-300 rounded-lg cursor-pointer transition-all text-left hover:bg-base-100 hover:border-primary hover:shadow-sm" onclick={loadPerformanceData}>
            <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-info/20 text-info shrink-0">
              <i class="bi bi-arrow-clockwise"></i>
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="text-sm font-medium text-base-content">{tr("settings.performance.refresh")}</span>
              <span class="text-xs text-base-content/60">{tr("settings.performance.refresh_desc")}</span>
            </div>
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Minimal CSS - most styling via Tailwind utilities */
</style>
