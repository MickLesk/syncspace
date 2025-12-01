<script>
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import CleanupManagementPanel from "../../components/admin/CleanupManagementPanel.svelte";

  const tr = (key, ...args) => t($currentLang, key, ...args);

  // Dashboard state
  let loading = $state(true);
  let stats = $state(null);
  let storageData = $state(null);
  let activityData = $state(null);
  let usersData = $state(null);
  let jobsData = $state(null);
  let healthData = $state(null);
  let realtimeData = $state(null);

  // Time range filter
  let timeRange = $state("7d"); // 24h, 7d, 30d, 90d

  // Auto-refresh interval
  let refreshInterval = null;
  let lastUpdated = $state(null);

  // Chart refs
  let storageChartEl = $state(null);
  let activityChartEl = $state(null);

  onMount(async () => {
    await loadAllData();
    // Auto-refresh every 30 seconds
    refreshInterval = setInterval(loadRealtimeData, 30000);
  });

  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });

  async function loadAllData() {
    loading = true;
    try {
      await Promise.all([
        loadDashboardStats(),
        loadStorageData(),
        loadActivityData(),
        loadUsersData(),
        loadJobsData(),
        loadHealthData(),
        loadRealtimeData(),
      ]);
      lastUpdated = new Date();
    } catch (err) {
      console.error("Failed to load dashboard data:", err);
      errorToast(tr("dashboard.loadError"));
    } finally {
      loading = false;
    }
  }

  async function loadDashboardStats() {
    try {
      const response = await fetch(
        `http://localhost:8080/api/dashboard/stats?range=${timeRange}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );
      if (response.ok) {
        stats = await response.json();
      }
    } catch (err) {
      console.error("Failed to load stats:", err);
    }
  }

  async function loadStorageData() {
    try {
      const response = await fetch(
        `http://localhost:8080/api/dashboard/storage`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );
      if (response.ok) {
        storageData = await response.json();
      }
    } catch (err) {
      console.error("Failed to load storage data:", err);
    }
  }

  async function loadActivityData() {
    try {
      const response = await fetch(
        `http://localhost:8080/api/dashboard/activity`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );
      if (response.ok) {
        activityData = await response.json();
      }
    } catch (err) {
      console.error("Failed to load activity data:", err);
    }
  }

  async function loadUsersData() {
    try {
      const response = await fetch(
        `http://localhost:8080/api/dashboard/users`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );
      if (response.ok) {
        usersData = await response.json();
      }
    } catch (err) {
      console.error("Failed to load users data:", err);
    }
  }

  async function loadJobsData() {
    try {
      const response = await fetch(`http://localhost:8080/api/dashboard/jobs`, {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });
      if (response.ok) {
        jobsData = await response.json();
      }
    } catch (err) {
      console.error("Failed to load jobs data:", err);
    }
  }

  async function loadHealthData() {
    try {
      const response = await fetch(
        `http://localhost:8080/api/dashboard/health`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );
      if (response.ok) {
        healthData = await response.json();
      }
    } catch (err) {
      console.error("Failed to load health data:", err);
    }
  }

  async function loadRealtimeData() {
    try {
      const response = await fetch(
        `http://localhost:8080/api/dashboard/realtime`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );
      if (response.ok) {
        realtimeData = await response.json();
      }
    } catch (err) {
      console.error("Failed to load realtime data:", err);
    }
  }

  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function formatNumber(num) {
    if (num >= 1000000) return (num / 1000000).toFixed(1) + "M";
    if (num >= 1000) return (num / 1000).toFixed(1) + "K";
    return num?.toLocaleString() || "0";
  }

  function getHealthColor(status) {
    switch (status) {
      case "healthy":
        return "text-green-500";
      case "degraded":
        return "text-yellow-500";
      case "unhealthy":
        return "text-red-500";
      default:
        return "text-gray-500";
    }
  }

  function getHealthBg(status) {
    switch (status) {
      case "healthy":
        return "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800";
      case "degraded":
        return "bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800";
      case "unhealthy":
        return "bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800";
      default:
        return "bg-gray-50 dark:bg-gray-800 border-gray-200 dark:border-gray-700";
    }
  }

  function formatTimeAgo(date) {
    if (!date) return "";
    const seconds = Math.floor((new Date() - new Date(date)) / 1000);
    if (seconds < 60) return tr("justNow");
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${tr("ago")}`;
    if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ${tr("ago")}`;
    return `${Math.floor(seconds / 86400)}d ${tr("ago")}`;
  }

  // Watch for time range changes
  $effect(() => {
    if (timeRange) {
      loadDashboardStats();
    }
  });
</script>

<div
  class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-blue-50 dark:from-gray-900 dark:via-gray-900 dark:to-gray-800 p-6"
>
  <!-- Header -->
  <div class="flex flex-col md:flex-row md:items-center justify-between mb-8">
    <div>
      <h1
        class="text-3xl font-bold text-gray-900 dark:text-white flex items-center gap-3"
      >
        <i class="bi bi-speedometer2 text-indigo-500"></i>
        {tr("dashboard.title")}
      </h1>
      <p class="text-gray-500 dark:text-gray-400 mt-1">
        {tr("dashboard.description")}
      </p>
    </div>
    <div class="flex items-center gap-4 mt-4 md:mt-0">
      <!-- Time Range Selector -->
      <div
        class="flex bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-1"
      >
        {#each ["24h", "7d", "30d", "90d"] as range}
          <button
            class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {timeRange ===
            range
              ? 'bg-indigo-500 text-white'
              : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'}"
            onclick={() => (timeRange = range)}
          >
            {range}
          </button>
        {/each}
      </div>
      <!-- Refresh Button -->
      <button
        class="flex items-center gap-2 px-4 py-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
        onclick={() => loadAllData()}
        disabled={loading}
      >
        <i class="bi bi-arrow-clockwise {loading ? 'animate-spin' : ''}"></i>
        {tr("refresh")}
      </button>
    </div>
  </div>

  {#if loading && !stats}
    <div class="flex items-center justify-center h-64">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-500"
      ></div>
    </div>
  {:else}
    <!-- System Health Status Bar -->
    {#if healthData}
      <div class="mb-6 p-4 rounded-xl border {getHealthBg(healthData.status)}">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div
              class="w-3 h-3 rounded-full {healthData.status === 'healthy'
                ? 'bg-green-500'
                : healthData.status === 'degraded'
                  ? 'bg-yellow-500'
                  : 'bg-red-500'} animate-pulse"
            ></div>
            <span class="font-medium text-gray-900 dark:text-white">
              {tr("dashboard.systemStatus")}:
              <span class="{getHealthColor(healthData.status)} capitalize"
                >{healthData.status}</span
              >
            </span>
          </div>
          <div class="flex items-center gap-6 text-sm">
            <div class="flex items-center gap-2">
              <i
                class="bi bi-database {getHealthColor(
                  healthData.database?.status
                )}"
              ></i>
              <span class="text-gray-600 dark:text-gray-400"
                >{tr("dashboard.database")}: {healthData.database
                  ?.latency_ms}ms</span
              >
            </div>
            <div class="flex items-center gap-2">
              <i class="bi bi-hdd {getHealthColor(healthData.storage?.status)}"
              ></i>
              <span class="text-gray-600 dark:text-gray-400"
                >{tr("dashboard.storage")}</span
              >
            </div>
            <div class="flex items-center gap-2">
              <i
                class="bi bi-search {getHealthColor(healthData.search?.status)}"
              ></i>
              <span class="text-gray-600 dark:text-gray-400"
                >{tr("dashboard.search")}</span
              >
            </div>
            <div class="flex items-center gap-2">
              <i
                class="bi bi-broadcast {getHealthColor(
                  healthData.websocket?.status
                )}"
              ></i>
              <span class="text-gray-600 dark:text-gray-400"
                >{tr("dashboard.websocket")}</span
              >
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Overview Stats Cards -->
    {#if stats?.overview}
      <div class="grid grid-cols-2 md:grid-cols-4 xl:grid-cols-8 gap-4 mb-8">
        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-people text-2xl text-blue-500"></i>
            <span class="text-xs text-green-500 font-medium"
              >+{stats.overview.active_users_today}
              {tr("dashboard.today")}</span
            >
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {formatNumber(stats.overview.total_users)}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.totalUsers")}
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-folder text-2xl text-indigo-500"></i>
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {formatNumber(stats.overview.total_files)}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.totalFiles")}
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-hdd text-2xl text-purple-500"></i>
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {formatBytes(stats.overview.total_storage_bytes)}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.totalStorage")}
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-share text-2xl text-green-500"></i>
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {formatNumber(stats.overview.total_shares)}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.totalShares")}
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-clock-history text-2xl text-amber-500"></i>
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {stats.overview.pending_jobs}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.pendingJobs")}
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-person-check text-2xl text-teal-500"></i>
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {stats.overview.active_sessions}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.activeSessions")}
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-cloud-check text-2xl text-cyan-500"></i>
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {stats.overview.backup_count}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.backups")}
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="flex items-center justify-between mb-2">
            <i class="bi bi-activity text-2xl text-rose-500"></i>
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">
            {activityData?.total_actions_today || 0}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-400">
            {tr("dashboard.actionsToday")}
          </div>
        </div>
      </div>
    {/if}

    <!-- Main Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
      <!-- Storage Overview -->
      {#if storageData}
        <div
          class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 shadow-sm"
        >
          <h3
            class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
          >
            <i class="bi bi-pie-chart text-indigo-500"></i>
            {tr("dashboard.storageOverview")}
          </h3>

          <!-- Usage Bar -->
          <div class="mb-6">
            <div class="flex justify-between text-sm mb-2">
              <span class="text-gray-600 dark:text-gray-400"
                >{formatBytes(storageData.used_storage_bytes)}
                {tr("dashboard.used")}</span
              >
              <span class="text-gray-600 dark:text-gray-400"
                >{formatBytes(storageData.total_storage_bytes)}
                {tr("dashboard.total")}</span
              >
            </div>
            <div
              class="h-3 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden"
            >
              <div
                class="h-full bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full transition-all duration-500"
                style="width: {Math.min(storageData.usage_percentage, 100)}%"
              ></div>
            </div>
            <div
              class="text-right text-xs text-gray-500 dark:text-gray-400 mt-1"
            >
              {storageData.usage_percentage.toFixed(1)}% {tr("dashboard.used")}
            </div>
          </div>

          <!-- Storage by Type -->
          <div class="space-y-3">
            {#each storageData.by_type || [] as type}
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <div
                    class="w-3 h-3 rounded-full"
                    style="background-color: {type.color}"
                  ></div>
                  <span
                    class="text-sm text-gray-700 dark:text-gray-300 capitalize"
                    >{type.file_type}</span
                  >
                </div>
                <div class="text-right">
                  <span
                    class="text-sm font-medium text-gray-900 dark:text-white"
                    >{formatBytes(type.size_bytes)}</span
                  >
                  <span class="text-xs text-gray-500 dark:text-gray-400 ml-2"
                    >({type.percentage.toFixed(1)}%)</span
                  >
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Activity Overview -->
      {#if activityData}
        <div
          class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 shadow-sm"
        >
          <h3
            class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
          >
            <i class="bi bi-activity text-green-500"></i>
            {tr("dashboard.activityOverview")}
          </h3>

          <!-- Activity Stats -->
          <div class="grid grid-cols-2 gap-4 mb-6">
            <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
              <div class="text-2xl font-bold text-gray-900 dark:text-white">
                {activityData.total_actions_today}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400">
                {tr("dashboard.today")}
              </div>
            </div>
            <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
              <div class="text-2xl font-bold text-gray-900 dark:text-white">
                {activityData.total_actions_week}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400">
                {tr("dashboard.thisWeek")}
              </div>
            </div>
          </div>

          <!-- Activity by Action -->
          <div class="space-y-2">
            {#each activityData.by_action?.slice(0, 5) || [] as action}
              <div
                class="flex items-center justify-between p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
              >
                <div class="flex items-center gap-2">
                  <i class="bi bi-{action.icon}" style="color: {action.color}"
                  ></i>
                  <span
                    class="text-sm text-gray-700 dark:text-gray-300 capitalize"
                    >{action.action}</span
                  >
                </div>
                <span class="text-sm font-medium text-gray-900 dark:text-white"
                  >{action.count}</span
                >
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Users Overview -->
      {#if usersData}
        <div
          class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 shadow-sm"
        >
          <h3
            class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
          >
            <i class="bi bi-people text-blue-500"></i>
            {tr("dashboard.usersOverview")}
          </h3>

          <!-- User Stats -->
          <div class="grid grid-cols-2 gap-4 mb-6">
            <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
              <div class="text-2xl font-bold text-gray-900 dark:text-white">
                {usersData.active_today}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400">
                {tr("dashboard.activeToday")}
              </div>
            </div>
            <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
              <div class="text-2xl font-bold text-gray-900 dark:text-white">
                {usersData.new_this_month}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400">
                {tr("dashboard.newThisMonth")}
              </div>
            </div>
          </div>

          <!-- Users by Role -->
          <div class="space-y-2">
            {#each usersData.by_role || [] as role}
              <div
                class="flex items-center justify-between p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
              >
                <div class="flex items-center gap-2">
                  <i
                    class="bi bi-{role.role === 'admin'
                      ? 'shield-check'
                      : 'person'} text-gray-500"
                  ></i>
                  <span
                    class="text-sm text-gray-700 dark:text-gray-300 capitalize"
                    >{role.role}</span
                  >
                </div>
                <span class="text-sm font-medium text-gray-900 dark:text-white"
                  >{role.count}</span
                >
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Jobs Overview -->
      {#if jobsData}
        <div
          class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 shadow-sm"
        >
          <h3
            class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
          >
            <i class="bi bi-gear text-amber-500"></i>
            {tr("dashboard.jobsOverview")}
          </h3>

          <!-- Job Stats -->
          <div class="grid grid-cols-4 gap-2 mb-6">
            <div
              class="bg-yellow-50 dark:bg-yellow-900/20 rounded-lg p-3 text-center"
            >
              <div
                class="text-xl font-bold text-yellow-600 dark:text-yellow-400"
              >
                {jobsData.pending}
              </div>
              <div class="text-xs text-yellow-600 dark:text-yellow-400">
                {tr("dashboard.pending")}
              </div>
            </div>
            <div
              class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-3 text-center"
            >
              <div class="text-xl font-bold text-blue-600 dark:text-blue-400">
                {jobsData.running}
              </div>
              <div class="text-xs text-blue-600 dark:text-blue-400">
                {tr("dashboard.running")}
              </div>
            </div>
            <div
              class="bg-green-50 dark:bg-green-900/20 rounded-lg p-3 text-center"
            >
              <div class="text-xl font-bold text-green-600 dark:text-green-400">
                {jobsData.completed_today}
              </div>
              <div class="text-xs text-green-600 dark:text-green-400">
                {tr("dashboard.completed")}
              </div>
            </div>
            <div
              class="bg-red-50 dark:bg-red-900/20 rounded-lg p-3 text-center"
            >
              <div class="text-xl font-bold text-red-600 dark:text-red-400">
                {jobsData.failed_today}
              </div>
              <div class="text-xs text-red-600 dark:text-red-400">
                {tr("dashboard.failed")}
              </div>
            </div>
          </div>

          <!-- Recent Jobs -->
          <div class="space-y-2 max-h-48 overflow-y-auto">
            {#each jobsData.recent_jobs || [] as job}
              <div
                class="flex items-center justify-between p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
              >
                <div class="flex items-center gap-2">
                  <span
                    class="w-2 h-2 rounded-full {job.status === 'completed'
                      ? 'bg-green-500'
                      : job.status === 'running'
                        ? 'bg-blue-500 animate-pulse'
                        : job.status === 'failed'
                          ? 'bg-red-500'
                          : 'bg-yellow-500'}"
                  ></span>
                  <span class="text-sm text-gray-700 dark:text-gray-300"
                    >{job.job_type}</span
                  >
                </div>
                <span class="text-xs text-gray-500 dark:text-gray-400"
                  >{formatTimeAgo(job.created_at)}</span
                >
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Top Uploaders -->
      {#if stats?.top_items?.top_uploaders?.length > 0}
        <div
          class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 shadow-sm"
        >
          <h3
            class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
          >
            <i class="bi bi-trophy text-yellow-500"></i>
            {tr("dashboard.topUploaders")}
          </h3>
          <div class="space-y-3">
            {#each stats.top_items.top_uploaders as user, i}
              <div class="flex items-center gap-3">
                <div
                  class="w-8 h-8 rounded-full bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center text-white font-bold text-sm"
                >
                  {i + 1}
                </div>
                <div class="flex-1">
                  <div
                    class="text-sm font-medium text-gray-900 dark:text-white"
                  >
                    {user.display_name || user.username}
                  </div>
                  <div class="text-xs text-gray-500 dark:text-gray-400">
                    {user.count}
                    {tr("dashboard.files")} • {formatBytes(
                      user.size_bytes || 0
                    )}
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Largest Files -->
      {#if stats?.top_items?.largest_files?.length > 0}
        <div
          class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 shadow-sm"
        >
          <h3
            class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
          >
            <i class="bi bi-file-earmark-arrow-up text-purple-500"></i>
            {tr("dashboard.largestFiles")}
          </h3>
          <div class="space-y-3">
            {#each stats.top_items.largest_files as file}
              <div
                class="flex items-center justify-between p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
              >
                <div class="flex items-center gap-2 min-w-0">
                  <i class="bi bi-file-earmark text-gray-400"></i>
                  <span
                    class="text-sm text-gray-700 dark:text-gray-300 truncate"
                    >{file.filename}</span
                  >
                </div>
                <span
                  class="text-sm font-medium text-gray-900 dark:text-white whitespace-nowrap ml-2"
                  >{formatBytes(file.size_bytes)}</span
                >
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <!-- Cleanup Management Panel -->
    <div class="mt-8">
      <CleanupManagementPanel />
    </div>

    <!-- Recent Activity Stream -->
    {#if activityData?.recent_activity?.length > 0}
      <div
        class="mt-6 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 shadow-sm"
      >
        <h3
          class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
        >
          <i class="bi bi-clock-history text-indigo-500"></i>
          {tr("dashboard.recentActivity")}
        </h3>
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
          {#each activityData.recent_activity.slice(0, 8) as activity}
            <div
              class="flex items-center gap-3 p-3 rounded-lg bg-gray-50 dark:bg-gray-700/50"
            >
              <div
                class="w-10 h-10 rounded-full bg-gradient-to-br from-gray-200 to-gray-300 dark:from-gray-600 dark:to-gray-700 flex items-center justify-center"
              >
                <i class="bi bi-person text-gray-500 dark:text-gray-400"></i>
              </div>
              <div class="min-w-0 flex-1">
                <div
                  class="text-sm font-medium text-gray-900 dark:text-white truncate"
                >
                  {activity.username}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400 truncate">
                  <span class="capitalize">{activity.action}</span>
                  {#if activity.resource}
                    <span class="text-gray-400">•</span>
                    {activity.resource.split("/").pop()}
                  {/if}
                </div>
              </div>
              <div class="text-xs text-gray-400">
                {formatTimeAgo(activity.created_at)}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Last Updated -->
    {#if lastUpdated}
      <div class="mt-6 text-center text-xs text-gray-400">
        {tr("dashboard.lastUpdated")}: {lastUpdated.toLocaleTimeString()}
      </div>
    {/if}
  {/if}
</div>
