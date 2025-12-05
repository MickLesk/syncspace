<script>
  import { t } from "../../i18n.js";
  import { currentLang } from "../../stores/ui.js";
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";
  import { success as toastSuccess, error as toastError } from "../../stores/toast.js";

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

<div class="performance-settings">
  {#if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-circle"></i>
      <span>{error}</span>
    </div>
  {/if}

  {#if loading}
    <div class="loading-container">
      <div class="loading-spinner"></div>
      <p>{tr("common.loading")}</p>
    </div>
  {:else if performanceData}
    <!-- System Status Overview -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon green">
          <i class="bi bi-cpu"></i>
        </div>
        <div class="stat-content">
          <span class="stat-value">{performanceData.cpu_usage}%</span>
          <span class="stat-label">CPU</span>
        </div>
        <div class="stat-bar">
          <div
            class="stat-bar-fill"
            style="width: {performanceData.cpu_usage}%; background: {getProgressColor(
              performanceData.cpu_usage
            )}"
          ></div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon blue">
          <i class="bi bi-memory"></i>
        </div>
        <div class="stat-content">
          <span class="stat-value">{performanceData.memory_usage}%</span>
          <span class="stat-label">{tr("settings.performance.memory")}</span>
        </div>
        <div class="stat-bar">
          <div
            class="stat-bar-fill"
            style="width: {performanceData.memory_usage}%; background: {getProgressColor(
              performanceData.memory_usage
            )}"
          ></div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon amber">
          <i class="bi bi-hdd"></i>
        </div>
        <div class="stat-content">
          <span class="stat-value">{performanceData.disk_usage}%</span>
          <span class="stat-label">{tr("settings.performance.disk")}</span>
        </div>
        <div class="stat-bar">
          <div
            class="stat-bar-fill"
            style="width: {performanceData.disk_usage}%; background: {getProgressColor(
              performanceData.disk_usage
            )}"
          ></div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon purple">
          <i class="bi bi-clock-history"></i>
        </div>
        <div class="stat-content">
          <span class="stat-value"
            >{formatUptime(performanceData.uptime_seconds)}</span
          >
          <span class="stat-label">{tr("settings.performance.uptime")}</span>
        </div>
      </div>
    </div>

    <!-- Detailed Metrics -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon green">
          <i class="bi bi-speedometer2"></i>
        </div>
        <div>
          <h3>{tr("settings.performance.metrics")}</h3>
          <p class="card-subtitle">{tr("settings.performance.metrics_desc")}</p>
        </div>
      </div>

      <div class="card-body">
        <div class="metrics-grid">
          <div class="metric-item">
            <span class="metric-label"
              >{tr("settings.performance.memory_used")}</span
            >
            <span class="metric-value"
              >{performanceData.memory_used?.toFixed(1)} / {performanceData.memory_total?.toFixed(
                1
              )} GB</span
            >
          </div>

          <div class="metric-item">
            <span class="metric-label"
              >{tr("settings.performance.disk_used")}</span
            >
            <span class="metric-value"
              >{performanceData.disk_used?.toFixed(1)} / {performanceData.disk_total?.toFixed(
                0
              )} GB</span
            >
          </div>

          <div class="metric-item">
            <span class="metric-label"
              >{tr("settings.performance.connections")}</span
            >
            <span class="metric-value"
              >{performanceData.active_connections}</span
            >
          </div>

          <div class="metric-item">
            <span class="metric-label"
              >{tr("settings.performance.requests")}</span
            >
            <span class="metric-value"
              >{performanceData.requests_per_minute}/min</span
            >
          </div>

          <div class="metric-item">
            <span class="metric-label"
              >{tr("settings.performance.cache_rate")}</span
            >
            <span class="metric-value">{performanceData.cache_hit_rate}%</span>
          </div>

          <div class="metric-item">
            <span class="metric-label"
              >{tr("settings.performance.database_size")}</span
            >
            <span class="metric-value"
              >{performanceData.database_size?.toFixed(1)} MB</span
            >
          </div>

          <div class="metric-item">
            <span class="metric-label"
              >{tr("settings.performance.index_size")}</span
            >
            <span class="metric-value"
              >{performanceData.search_index_size?.toFixed(1)} MB</span
            >
          </div>
        </div>
      </div>
    </div>

    <!-- Maintenance Actions -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon blue">
          <i class="bi bi-tools"></i>
        </div>
        <div>
          <h3>{tr("settings.performance.maintenance")}</h3>
          <p class="card-subtitle">
            {tr("settings.performance.maintenance_desc")}
          </p>
        </div>
      </div>

      <div class="card-body">
        <div class="actions-grid">
          <button class="action-card" onclick={clearCache}>
            <div class="action-icon amber">
              <i class="bi bi-arrow-repeat"></i>
            </div>
            <div class="action-content">
              <span class="action-title"
                >{tr("settings.performance.clear_cache")}</span
              >
              <span class="action-desc"
                >{tr("settings.performance.clear_cache_desc")}</span
              >
            </div>
          </button>

          <button class="action-card" onclick={optimizeDatabase}>
            <div class="action-icon green">
              <i class="bi bi-database-gear"></i>
            </div>
            <div class="action-content">
              <span class="action-title"
                >{tr("settings.performance.optimize_db")}</span
              >
              <span class="action-desc"
                >{tr("settings.performance.optimize_db_desc")}</span
              >
            </div>
          </button>

          <button class="action-card" onclick={loadPerformanceData}>
            <div class="action-icon blue">
              <i class="bi bi-arrow-clockwise"></i>
            </div>
            <div class="action-content">
              <span class="action-title"
                >{tr("settings.performance.refresh")}</span
              >
              <span class="action-desc"
                >{tr("settings.performance.refresh_desc")}</span
              >
            </div>
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .performance-settings {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* Alert */
  .alert {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
  }

  .alert-error {
    background: #fef2f2;
    color: #dc2626;
    border: 1px solid #fecaca;
  }

  :global([data-theme="dark"]) .alert-error {
    background: rgba(220, 38, 38, 0.1);
    border-color: rgba(220, 38, 38, 0.3);
  }

  /* Loading */
  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    gap: 1rem;
    color: #6b7280;
  }

  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .stat-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  :global([data-theme="dark"]) .stat-card {
    background: #1f2937;
    border-color: #374151;
  }

  .stat-card > div:first-child {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .stat-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
  }

  .stat-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }

  .stat-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }

  .stat-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }

  .stat-icon.purple {
    background: #f3e8ff;
    color: #9333ea;
  }

  :global([data-theme="dark"]) .stat-icon.green {
    background: rgba(22, 163, 74, 0.2);
  }

  :global([data-theme="dark"]) .stat-icon.blue {
    background: rgba(37, 99, 235, 0.2);
  }

  :global([data-theme="dark"]) .stat-icon.amber {
    background: rgba(217, 119, 6, 0.2);
  }

  :global([data-theme="dark"]) .stat-icon.purple {
    background: rgba(147, 51, 234, 0.2);
  }

  .stat-content {
    display: flex;
    flex-direction: column;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: #111827;
  }

  :global([data-theme="dark"]) .stat-value {
    color: #f9fafb;
  }

  .stat-label {
    font-size: 0.75rem;
    color: #6b7280;
  }

  .stat-bar {
    height: 6px;
    background: #e5e7eb;
    border-radius: 3px;
    overflow: hidden;
  }

  :global([data-theme="dark"]) .stat-bar {
    background: #374151;
  }

  .stat-bar-fill {
    height: 100%;
    border-radius: 3px;
    transition: width 0.3s ease;
  }

  /* Cards */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    overflow: hidden;
  }

  :global([data-theme="dark"]) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .card-header {
    border-bottom-color: #374151;
  }

  .card-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  :global([data-theme="dark"]) .card-header h3 {
    color: #f9fafb;
  }

  .card-subtitle {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0.25rem 0 0 0;
  }

  .card-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }

  .card-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }

  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }

  :global([data-theme="dark"]) .card-icon.green {
    background: rgba(22, 163, 74, 0.2);
  }

  :global([data-theme="dark"]) .card-icon.blue {
    background: rgba(37, 99, 235, 0.2);
  }

  .card-body {
    padding: 1.25rem;
  }

  /* Metrics Grid */
  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .metric-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }

  :global([data-theme="dark"]) .metric-item {
    background: #374151;
  }

  .metric-label {
    font-size: 0.75rem;
    color: #6b7280;
  }

  .metric-value {
    font-size: 0.9rem;
    font-weight: 600;
    color: #111827;
  }

  :global([data-theme="dark"]) .metric-value {
    color: #f9fafb;
  }

  /* Actions Grid */
  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1rem;
  }

  .action-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
  }

  .action-card:hover {
    background: white;
    border-color: #22c55e;
    box-shadow: 0 2px 8px rgba(34, 197, 94, 0.1);
  }

  :global([data-theme="dark"]) .action-card {
    background: #374151;
    border-color: #4b5563;
  }

  :global([data-theme="dark"]) .action-card:hover {
    background: #4b5563;
    border-color: #22c55e;
  }

  .action-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }

  .action-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }

  .action-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }

  .action-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }

  :global([data-theme="dark"]) .action-icon.amber {
    background: rgba(217, 119, 6, 0.2);
  }

  :global([data-theme="dark"]) .action-icon.green {
    background: rgba(22, 163, 74, 0.2);
  }

  :global([data-theme="dark"]) .action-icon.blue {
    background: rgba(37, 99, 235, 0.2);
  }

  .action-content {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .action-title {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }

  :global([data-theme="dark"]) .action-title {
    color: #f9fafb;
  }

  .action-desc {
    font-size: 0.75rem;
    color: #6b7280;
  }
</style>
