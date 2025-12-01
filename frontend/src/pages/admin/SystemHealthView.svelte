<script>
  import { onMount, onDestroy } from "svelte";
  import { t } from "../../i18n.js";
  import { systemHealth } from "../../lib/api.js";

  // Health data
  let health = $state(null);
  let cpuMetrics = $state(null);
  let memoryMetrics = $state(null);
  let diskMetrics = $state(null);
  let databaseHealth = $state(null);
  let connectionStats = $state(null);
  let searchHealth = $state(null);
  let websocketHealth = $state(null);
  let applicationMetrics = $state(null);
  let historicalData = $state(null);
  let alerts = $state([]);

  // UI state
  let loading = $state(true);
  let error = $state(null);
  let refreshing = $state(false);
  let autoRefresh = $state(true);
  let historyRange = $state("1h");
  let showAlertModal = $state(false);
  let editingAlert = $state(null);
  let lastUpdate = $state(null);

  // Alert form
  let alertForm = $state({
    name: "",
    metric: "cpu_usage",
    condition: "greater_than",
    threshold: 80,
    enabled: true,
  });

  // Auto-refresh interval
  let refreshInterval = null;

  const metricOptions = [
    { value: "cpu_usage", label: () => $t("systemHealth.cpuUsageAlert") },
    { value: "memory_usage", label: () => $t("systemHealth.memoryUsageAlert") },
    { value: "disk_usage", label: () => $t("systemHealth.diskUsageAlert") },
    { value: "error_rate", label: () => $t("systemHealth.errorRateAlert") },
    {
      value: "response_time",
      label: () => $t("systemHealth.responseTimeAlert"),
    },
    {
      value: "connection_count",
      label: () => $t("systemHealth.connectionCountAlert"),
    },
  ];

  const conditionOptions = [
    { value: "greater_than", label: () => $t("systemHealth.greaterThan") },
    { value: "less_than", label: () => $t("systemHealth.lessThan") },
    { value: "equals", label: () => $t("systemHealth.equals") },
  ];

  const historyRangeOptions = [
    { value: "1h", label: () => $t("systemHealth.last1Hour") },
    { value: "6h", label: () => $t("systemHealth.last6Hours") },
    { value: "24h", label: () => $t("systemHealth.last24Hours") },
    { value: "7d", label: () => $t("systemHealth.last7Days") },
  ];

  onMount(() => {
    loadAllData();
    if (autoRefresh) {
      startAutoRefresh();
    }
  });

  onDestroy(() => {
    stopAutoRefresh();
  });

  function startAutoRefresh() {
    stopAutoRefresh();
    refreshInterval = setInterval(loadAllData, 30000); // 30 seconds
  }

  function stopAutoRefresh() {
    if (refreshInterval) {
      clearInterval(refreshInterval);
      refreshInterval = null;
    }
  }

  function toggleAutoRefresh() {
    autoRefresh = !autoRefresh;
    if (autoRefresh) {
      startAutoRefresh();
    } else {
      stopAutoRefresh();
    }
  }

  async function loadAllData() {
    if (refreshing) return;

    try {
      refreshing = true;
      error = null;

      // Load all health data in parallel
      const [
        healthData,
        cpuData,
        memoryData,
        diskData,
        dbData,
        connData,
        searchData,
        wsData,
        metricsData,
        historyData,
        alertsData,
      ] = await Promise.all([
        systemHealth.getHealth().catch(() => null),
        systemHealth.getCpu().catch(() => null),
        systemHealth.getMemory().catch(() => null),
        systemHealth.getDisk().catch(() => null),
        systemHealth.getDatabase().catch(() => null),
        systemHealth.getConnections().catch(() => null),
        systemHealth.getSearch().catch(() => null),
        systemHealth.getWebsocket().catch(() => null),
        systemHealth.getMetrics().catch(() => null),
        systemHealth.getHistory(historyRange).catch(() => null),
        systemHealth.listAlerts().catch(() => []),
      ]);

      health = healthData;
      cpuMetrics = cpuData;
      memoryMetrics = memoryData;
      diskMetrics = diskData;
      databaseHealth = dbData;
      connectionStats = connData;
      searchHealth = searchData;
      websocketHealth = wsData;
      applicationMetrics = metricsData;
      historicalData = historyData;
      alerts = alertsData || [];
      lastUpdate = new Date();
    } catch (err) {
      console.error("Failed to load health data:", err);
      error = $t("systemHealth.loadError");
    } finally {
      loading = false;
      refreshing = false;
    }
  }

  async function loadHistory() {
    try {
      historicalData = await systemHealth.getHistory(historyRange);
    } catch (err) {
      console.error("Failed to load history:", err);
    }
  }

  function openAlertModal(alert = null) {
    if (alert) {
      editingAlert = alert;
      alertForm = { ...alert };
    } else {
      editingAlert = null;
      alertForm = {
        name: "",
        metric: "cpu_usage",
        condition: "greater_than",
        threshold: 80,
        enabled: true,
      };
    }
    showAlertModal = true;
  }

  function closeAlertModal() {
    showAlertModal = false;
    editingAlert = null;
  }

  async function saveAlert() {
    try {
      await systemHealth.createAlert(alertForm);
      alerts = await systemHealth.listAlerts();
      closeAlertModal();
    } catch (err) {
      console.error("Failed to save alert:", err);
    }
  }

  async function deleteAlert(id) {
    if (!confirm($t("common.confirmDelete"))) return;
    try {
      await systemHealth.deleteAlert(id);
      alerts = alerts.filter((a) => a.id !== id);
    } catch (err) {
      console.error("Failed to delete alert:", err);
    }
  }

  function getHealthStatusClass(status) {
    switch (status) {
      case "healthy":
        return "text-success bg-success/10";
      case "degraded":
        return "text-warning bg-warning/10";
      case "unhealthy":
        return "text-error bg-error/10";
      default:
        return "text-base-content/50 bg-base-300";
    }
  }

  function getHealthIcon(status) {
    switch (status) {
      case "healthy":
        return "bi-check-circle-fill";
      case "degraded":
        return "bi-exclamation-triangle-fill";
      case "unhealthy":
        return "bi-x-circle-fill";
      default:
        return "bi-question-circle";
    }
  }

  function formatBytes(bytes) {
    if (!bytes) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    let i = 0;
    while (bytes >= 1024 && i < units.length - 1) {
      bytes /= 1024;
      i++;
    }
    return `${bytes.toFixed(1)} ${units[i]}`;
  }

  function formatDuration(ms) {
    if (!ms) return "0ms";
    if (ms < 1000) return `${ms.toFixed(0)}ms`;
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
    return `${(ms / 60000).toFixed(1)}m`;
  }

  function formatUptime(seconds) {
    if (!seconds) return "-";
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${days}d ${hours}h ${minutes}m`;
  }

  function formatPercent(value) {
    if (value == null) return "-";
    return `${value.toFixed(1)}%`;
  }

  function formatNumber(num) {
    if (num == null) return "-";
    return num.toLocaleString();
  }
</script>

<div class="p-6 space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-bold">{$t("systemHealth.title")}</h1>
      <p class="text-base-content/60">{$t("systemHealth.description")}</p>
    </div>
    <div class="flex items-center gap-4">
      {#if lastUpdate}
        <span class="text-sm text-base-content/50">
          {$t("systemHealth.lastCheck")}: {lastUpdate.toLocaleTimeString()}
        </span>
      {/if}
      <label class="flex items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          class="toggle toggle-sm toggle-primary"
          checked={autoRefresh}
          onchange={toggleAutoRefresh}
        />
        <span class="text-sm">{$t("systemHealth.autoRefresh")}</span>
      </label>
      <button
        class="btn btn-primary btn-sm"
        onclick={loadAllData}
        disabled={refreshing}
      >
        {#if refreshing}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <i class="bi bi-arrow-clockwise"></i>
        {/if}
        {$t("common.refresh")}
      </button>
    </div>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-20">
      <span class="loading loading-spinner loading-lg text-primary"></span>
    </div>
  {:else if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-triangle"></i>
      <span>{error}</span>
      <button class="btn btn-sm btn-ghost" onclick={loadAllData}>
        {$t("systemHealth.retry")}
      </button>
    </div>
  {:else}
    <!-- Overall Health Status -->
    {#if health}
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center gap-4">
            <div
              class={`p-4 rounded-full ${getHealthStatusClass(health.status)}`}
            >
              <i class={`bi ${getHealthIcon(health.status)} text-3xl`}></i>
            </div>
            <div>
              <h2 class="text-xl font-semibold">
                {$t("systemHealth.overallHealth")}
              </h2>
              <p
                class={`text-lg font-medium ${health.status === "healthy" ? "text-success" : health.status === "degraded" ? "text-warning" : "text-error"}`}
              >
                {$t(`systemHealth.${health.status}`)}
              </p>
            </div>
            {#if health.uptime}
              <div class="ml-auto text-right">
                <p class="text-sm text-base-content/60">
                  {$t("systemHealth.uptime")}
                </p>
                <p class="font-mono text-lg">{formatUptime(health.uptime)}</p>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {/if}

    <!-- Resource Metrics Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
      <!-- CPU Card -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center gap-2 mb-4">
            <i class="bi bi-cpu text-lg text-primary"></i>
            <h3 class="font-semibold">{$t("systemHealth.cpu")}</h3>
          </div>
          {#if cpuMetrics}
            <div class="space-y-3">
              <div>
                <div class="flex justify-between text-sm mb-1">
                  <span>{$t("systemHealth.cpuUsage")}</span>
                  <span class="font-mono"
                    >{formatPercent(cpuMetrics.usage_percent)}</span
                  >
                </div>
                <progress
                  class="progress progress-primary w-full"
                  value={cpuMetrics.usage_percent || 0}
                  max="100"
                ></progress>
              </div>
              {#if cpuMetrics.cores}
                <div class="text-sm text-base-content/60">
                  <i class="bi bi-cpu-fill mr-1"></i>
                  {cpuMetrics.cores}
                  {$t("systemHealth.cpuCores")}
                </div>
              {/if}
              {#if cpuMetrics.load_average}
                <div class="text-xs text-base-content/50">
                  <span>{$t("systemHealth.loadAverage")}:</span>
                  <span class="font-mono ml-1">
                    {cpuMetrics.load_average[0]?.toFixed(2)} / {cpuMetrics.load_average[1]?.toFixed(
                      2
                    )} / {cpuMetrics.load_average[2]?.toFixed(2)}
                  </span>
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>

      <!-- Memory Card -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center gap-2 mb-4">
            <i class="bi bi-memory text-lg text-secondary"></i>
            <h3 class="font-semibold">{$t("systemHealth.memory")}</h3>
          </div>
          {#if memoryMetrics}
            <div class="space-y-3">
              <div>
                <div class="flex justify-between text-sm mb-1">
                  <span>{$t("systemHealth.memoryUsage")}</span>
                  <span class="font-mono"
                    >{formatPercent(memoryMetrics.usage_percent)}</span
                  >
                </div>
                <progress
                  class="progress progress-secondary w-full"
                  value={memoryMetrics.usage_percent || 0}
                  max="100"
                ></progress>
              </div>
              <div class="grid grid-cols-2 gap-2 text-sm">
                <div>
                  <span class="text-base-content/60"
                    >{$t("systemHealth.usedMemory")}</span
                  >
                  <p class="font-mono">
                    {formatBytes(memoryMetrics.used_bytes)}
                  </p>
                </div>
                <div>
                  <span class="text-base-content/60"
                    >{$t("systemHealth.totalMemory")}</span
                  >
                  <p class="font-mono">
                    {formatBytes(memoryMetrics.total_bytes)}
                  </p>
                </div>
              </div>
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>

      <!-- Disk Card -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center gap-2 mb-4">
            <i class="bi bi-hdd text-lg text-accent"></i>
            <h3 class="font-semibold">{$t("systemHealth.disk")}</h3>
          </div>
          {#if diskMetrics}
            <div class="space-y-3">
              <div>
                <div class="flex justify-between text-sm mb-1">
                  <span>{$t("systemHealth.diskUsage")}</span>
                  <span class="font-mono"
                    >{formatPercent(diskMetrics.usage_percent)}</span
                  >
                </div>
                <progress
                  class="progress progress-accent w-full"
                  value={diskMetrics.usage_percent || 0}
                  max="100"
                ></progress>
              </div>
              <div class="grid grid-cols-2 gap-2 text-sm">
                <div>
                  <span class="text-base-content/60"
                    >{$t("systemHealth.usedSpace")}</span
                  >
                  <p class="font-mono">{formatBytes(diskMetrics.used_bytes)}</p>
                </div>
                <div>
                  <span class="text-base-content/60"
                    >{$t("systemHealth.freeSpace")}</span
                  >
                  <p class="font-mono">{formatBytes(diskMetrics.free_bytes)}</p>
                </div>
              </div>
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>

      <!-- Process Card -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center gap-2 mb-4">
            <i class="bi bi-terminal text-lg text-info"></i>
            <h3 class="font-semibold">{$t("systemHealth.process")}</h3>
          </div>
          {#if applicationMetrics}
            <div class="space-y-2 text-sm">
              {#if applicationMetrics.process_memory}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.processMemory")}</span
                  >
                  <span class="font-mono"
                    >{formatBytes(applicationMetrics.process_memory)}</span
                  >
                </div>
              {/if}
              {#if applicationMetrics.threads}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.threads")}</span
                  >
                  <span class="font-mono">{applicationMetrics.threads}</span>
                </div>
              {/if}
              {#if applicationMetrics.open_files}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.openFiles")}</span
                  >
                  <span class="font-mono">{applicationMetrics.open_files}</span>
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Service Health Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
      <!-- Database Health -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-2">
              <i class="bi bi-database text-lg text-primary"></i>
              <h3 class="font-semibold">{$t("systemHealth.database")}</h3>
            </div>
            {#if databaseHealth}
              <span
                class={`badge ${databaseHealth.status === "healthy" ? "badge-success" : databaseHealth.status === "degraded" ? "badge-warning" : "badge-error"}`}
              >
                {$t(`systemHealth.${databaseHealth.status || "unknown"}`)}
              </span>
            {/if}
          </div>
          {#if databaseHealth}
            <div class="space-y-2 text-sm">
              {#if databaseHealth.size_bytes}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.databaseSize")}</span
                  >
                  <span class="font-mono"
                    >{formatBytes(databaseHealth.size_bytes)}</span
                  >
                </div>
              {/if}
              {#if databaseHealth.table_count}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.tableCount")}</span
                  >
                  <span class="font-mono">{databaseHealth.table_count}</span>
                </div>
              {/if}
              {#if databaseHealth.avg_query_time_ms}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.avgQueryTime")}</span
                  >
                  <span class="font-mono"
                    >{formatDuration(databaseHealth.avg_query_time_ms)}</span
                  >
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>

      <!-- Connection Pool -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-2">
              <i class="bi bi-diagram-3 text-lg text-secondary"></i>
              <h3 class="font-semibold">{$t("systemHealth.connectionPool")}</h3>
            </div>
          </div>
          {#if connectionStats}
            <div class="space-y-3">
              <div>
                <div class="flex justify-between text-sm mb-1">
                  <span>{$t("systemHealth.poolUtilization")}</span>
                  <span class="font-mono">
                    {connectionStats.active || 0}/{connectionStats.max || 10}
                  </span>
                </div>
                <progress
                  class="progress progress-secondary w-full"
                  value={(connectionStats.active /
                    (connectionStats.max || 10)) *
                    100}
                  max="100"
                ></progress>
              </div>
              <div class="grid grid-cols-2 gap-2 text-sm">
                <div>
                  <span class="text-base-content/60"
                    >{$t("systemHealth.activeConnections")}</span
                  >
                  <p class="font-mono text-lg">{connectionStats.active || 0}</p>
                </div>
                <div>
                  <span class="text-base-content/60"
                    >{$t("systemHealth.idleConnections")}</span
                  >
                  <p class="font-mono text-lg">{connectionStats.idle || 0}</p>
                </div>
              </div>
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>

      <!-- Search Index Health -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-2">
              <i class="bi bi-search text-lg text-accent"></i>
              <h3 class="font-semibold">{$t("systemHealth.search")}</h3>
            </div>
            {#if searchHealth}
              <span
                class={`badge ${searchHealth.status === "healthy" ? "badge-success" : searchHealth.status === "degraded" ? "badge-warning" : "badge-error"}`}
              >
                {$t(`systemHealth.${searchHealth.status || "unknown"}`)}
              </span>
            {/if}
          </div>
          {#if searchHealth}
            <div class="space-y-2 text-sm">
              {#if searchHealth.indexed_documents != null}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.indexedDocuments")}</span
                  >
                  <span class="font-mono"
                    >{formatNumber(searchHealth.indexed_documents)}</span
                  >
                </div>
              {/if}
              {#if searchHealth.index_size_bytes}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.indexSize")}</span
                  >
                  <span class="font-mono"
                    >{formatBytes(searchHealth.index_size_bytes)}</span
                  >
                </div>
              {/if}
              {#if searchHealth.pending_indexing != null}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.pendingIndexing")}</span
                  >
                  <span class="font-mono">{searchHealth.pending_indexing}</span>
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>

      <!-- WebSocket Health -->
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-2">
              <i class="bi bi-broadcast text-lg text-info"></i>
              <h3 class="font-semibold">{$t("systemHealth.websocket")}</h3>
            </div>
            {#if websocketHealth}
              <span
                class={`badge ${websocketHealth.status === "healthy" ? "badge-success" : websocketHealth.status === "degraded" ? "badge-warning" : "badge-error"}`}
              >
                {$t(`systemHealth.${websocketHealth.status || "unknown"}`)}
              </span>
            {/if}
          </div>
          {#if websocketHealth}
            <div class="space-y-2 text-sm">
              <div class="flex justify-between">
                <span class="text-base-content/60"
                  >{$t("systemHealth.connectedClients")}</span
                >
                <span class="font-mono text-lg"
                  >{websocketHealth.connected_clients || 0}</span
                >
              </div>
              {#if websocketHealth.messages_per_sec != null}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.messagesPerSec")}</span
                  >
                  <span class="font-mono"
                    >{websocketHealth.messages_per_sec.toFixed(1)}</span
                  >
                </div>
              {/if}
              {#if websocketHealth.connection_errors != null}
                <div class="flex justify-between">
                  <span class="text-base-content/60"
                    >{$t("systemHealth.connectionErrors")}</span
                  >
                  <span class="font-mono"
                    >{websocketHealth.connection_errors}</span
                  >
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-base-content/50 text-sm">
              {$t("systemHealth.unknown")}
            </div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Application Metrics -->
    {#if applicationMetrics}
      <div class="card bg-base-100 shadow-sm border border-base-300">
        <div class="card-body">
          <h3 class="font-semibold mb-4 flex items-center gap-2">
            <i class="bi bi-graph-up text-primary"></i>
            {$t("systemHealth.applicationMetrics")}
          </h3>
          <div class="grid grid-cols-2 md:grid-cols-4 xl:grid-cols-6 gap-4">
            <div class="stat bg-base-200 rounded-lg p-4">
              <div class="stat-title text-xs">
                {$t("systemHealth.requestsTotal")}
              </div>
              <div class="stat-value text-xl font-mono">
                {formatNumber(applicationMetrics.requests_total)}
              </div>
            </div>
            <div class="stat bg-base-200 rounded-lg p-4">
              <div class="stat-title text-xs">
                {$t("systemHealth.requestsPerMin")}
              </div>
              <div class="stat-value text-xl font-mono">
                {applicationMetrics.requests_per_min?.toFixed(1) || 0}
              </div>
            </div>
            <div class="stat bg-base-200 rounded-lg p-4">
              <div class="stat-title text-xs">
                {$t("systemHealth.avgResponseTime")}
              </div>
              <div class="stat-value text-xl font-mono">
                {formatDuration(applicationMetrics.avg_response_time_ms)}
              </div>
            </div>
            <div class="stat bg-base-200 rounded-lg p-4">
              <div class="stat-title text-xs">
                {$t("systemHealth.errorRate")}
              </div>
              <div class="stat-value text-xl font-mono">
                {formatPercent(applicationMetrics.error_rate)}
              </div>
            </div>
            <div class="stat bg-base-200 rounded-lg p-4">
              <div class="stat-title text-xs">
                {$t("systemHealth.uploadsTotal")}
              </div>
              <div class="stat-value text-xl font-mono">
                {formatNumber(applicationMetrics.uploads_total)}
              </div>
            </div>
            <div class="stat bg-base-200 rounded-lg p-4">
              <div class="stat-title text-xs">
                {$t("systemHealth.downloadsTotal")}
              </div>
              <div class="stat-value text-xl font-mono">
                {formatNumber(applicationMetrics.downloads_total)}
              </div>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Historical Metrics -->
    <div class="card bg-base-100 shadow-sm border border-base-300">
      <div class="card-body">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-semibold flex items-center gap-2">
            <i class="bi bi-clock-history text-primary"></i>
            {$t("systemHealth.historicalMetrics")}
          </h3>
          <select
            class="select select-bordered select-sm"
            bind:value={historyRange}
            onchange={loadHistory}
          >
            {#each historyRangeOptions as option}
              <option value={option.value}>{option.label()}</option>
            {/each}
          </select>
        </div>
        {#if historicalData && historicalData.data_points?.length > 0}
          <div class="overflow-x-auto">
            <table class="table table-sm">
              <thead>
                <tr>
                  <th>{$t("common.time")}</th>
                  <th>{$t("systemHealth.cpuUsage")}</th>
                  <th>{$t("systemHealth.memoryUsage")}</th>
                  <th>{$t("systemHealth.diskUsage")}</th>
                  <th>{$t("systemHealth.activeConnections")}</th>
                </tr>
              </thead>
              <tbody>
                {#each historicalData.data_points.slice(-10) as point}
                  <tr>
                    <td class="font-mono text-xs"
                      >{new Date(point.timestamp).toLocaleTimeString()}</td
                    >
                    <td class="font-mono">{formatPercent(point.cpu_usage)}</td>
                    <td class="font-mono"
                      >{formatPercent(point.memory_usage)}</td
                    >
                    <td class="font-mono">{formatPercent(point.disk_usage)}</td>
                    <td class="font-mono">{point.active_connections || 0}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {:else}
          <div class="text-center py-8 text-base-content/50">
            <i class="bi bi-clock text-4xl mb-2"></i>
            <p>{$t("systemHealth.noHistoricalData")}</p>
          </div>
        {/if}
      </div>
    </div>

    <!-- Health Alerts -->
    <div class="card bg-base-100 shadow-sm border border-base-300">
      <div class="card-body">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-semibold flex items-center gap-2">
            <i class="bi bi-bell text-warning"></i>
            {$t("systemHealth.alerts")}
          </h3>
          <button
            class="btn btn-primary btn-sm"
            onclick={() => openAlertModal()}
          >
            <i class="bi bi-plus-lg"></i>
            {$t("systemHealth.createAlert")}
          </button>
        </div>
        {#if alerts.length > 0}
          <div class="overflow-x-auto">
            <table class="table">
              <thead>
                <tr>
                  <th>{$t("systemHealth.alertName")}</th>
                  <th>{$t("systemHealth.alertMetric")}</th>
                  <th>{$t("systemHealth.alertCondition")}</th>
                  <th>{$t("systemHealth.alertThreshold")}</th>
                  <th class="text-center">{$t("common.status")}</th>
                  <th class="text-right">{$t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each alerts as alert}
                  <tr>
                    <td class="font-medium">{alert.name}</td>
                    <td>{alert.metric}</td>
                    <td>{alert.condition}</td>
                    <td class="font-mono">{alert.threshold}</td>
                    <td class="text-center">
                      <span
                        class={`badge ${alert.enabled ? "badge-success" : "badge-ghost"}`}
                      >
                        {alert.enabled
                          ? $t("systemHealth.alertEnabled")
                          : $t("common.disabled")}
                      </span>
                    </td>
                    <td class="text-right">
                      <button
                        class="btn btn-ghost btn-xs text-error"
                        onclick={() => deleteAlert(alert.id)}
                      >
                        <i class="bi bi-trash"></i>
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {:else}
          <div class="text-center py-8 text-base-content/50">
            <i class="bi bi-bell-slash text-4xl mb-2"></i>
            <p>{$t("systemHealth.noAlerts")}</p>
            <p class="text-sm">{$t("systemHealth.noAlertsDesc")}</p>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<!-- Alert Modal -->
{#if showAlertModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        {editingAlert
          ? $t("systemHealth.editAlert")
          : $t("systemHealth.createAlert")}
      </h3>
      <form
        onsubmit={(e) => {
          e.preventDefault();
          saveAlert();
        }}
      >
        <div class="form-control mb-4">
          <label class="label">
            <span class="label-text">{$t("systemHealth.alertName")}</span>
          </label>
          <input
            type="text"
            class="input input-bordered"
            bind:value={alertForm.name}
            required
          />
        </div>
        <div class="form-control mb-4">
          <label class="label">
            <span class="label-text">{$t("systemHealth.alertMetric")}</span>
          </label>
          <select class="select select-bordered" bind:value={alertForm.metric}>
            {#each metricOptions as option}
              <option value={option.value}>{option.label()}</option>
            {/each}
          </select>
        </div>
        <div class="grid grid-cols-2 gap-4 mb-4">
          <div class="form-control">
            <label class="label">
              <span class="label-text">{$t("systemHealth.alertCondition")}</span
              >
            </label>
            <select
              class="select select-bordered"
              bind:value={alertForm.condition}
            >
              {#each conditionOptions as option}
                <option value={option.value}>{option.label()}</option>
              {/each}
            </select>
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">{$t("systemHealth.alertThreshold")}</span
              >
            </label>
            <input
              type="number"
              class="input input-bordered"
              bind:value={alertForm.threshold}
              min="0"
              required
            />
          </div>
        </div>
        <div class="form-control mb-6">
          <label class="label cursor-pointer justify-start gap-3">
            <input
              type="checkbox"
              class="toggle toggle-primary"
              bind:checked={alertForm.enabled}
            />
            <span class="label-text">{$t("systemHealth.alertEnabled")}</span>
          </label>
        </div>
        <div class="modal-action">
          <button type="button" class="btn" onclick={closeAlertModal}>
            {$t("common.cancel")}
          </button>
          <button type="submit" class="btn btn-primary">
            {$t("common.save")}
          </button>
        </div>
      </form>
    </div>
    <div class="modal-backdrop" onclick={closeAlertModal}></div>
  </div>
{/if}
