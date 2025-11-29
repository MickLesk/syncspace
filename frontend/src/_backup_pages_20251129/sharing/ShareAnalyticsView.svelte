<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import { error as errorToast } from "../../stores/toast.js";
  import api from "../../lib/api.js";

  const tr = (key, ...args) => t($currentLang, key, ...args);

  let { shareId } = $props();

  let loading = $state(true);
  let analytics = $state(null);
  let accessLog = $state([]);
  let timeRange = $state("7d"); // 7d, 30d, 90d, all

  const filteredAccessLog = $derived(() => {
    if (!accessLog.length) return [];

    const now = new Date();
    const cutoff = new Date();

    switch (timeRange) {
      case "7d":
        cutoff.setDate(now.getDate() - 7);
        break;
      case "30d":
        cutoff.setDate(now.getDate() - 30);
        break;
      case "90d":
        cutoff.setDate(now.getDate() - 90);
        break;
      default:
        return accessLog;
    }

    return accessLog.filter((log) => new Date(log.accessed_at) >= cutoff);
  });

  const chartData = $derived(() => {
    const logs = filteredAccessLog();
    const dailyViews = {};

    logs.forEach((log) => {
      const date = new Date(log.accessed_at).toISOString().split("T")[0];
      dailyViews[date] = (dailyViews[date] || 0) + 1;
    });

    return Object.entries(dailyViews)
      .sort((a, b) => a[0].localeCompare(b[0]))
      .map(([date, views]) => ({ date, views }));
  });

  const topUsers = $derived(() => {
    const logs = filteredAccessLog();
    const userViews = {};

    logs.forEach((log) => {
      const userId = log.user_id || "Anonymous";
      userViews[userId] = (userViews[userId] || 0) + 1;
    });

    return Object.entries(userViews)
      .map(([userId, count]) => ({ userId, count }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 10);
  });

  const actionBreakdown = $derived(() => {
    const logs = filteredAccessLog();
    const actions = {};

    logs.forEach((log) => {
      const action = log.action || "view";
      actions[action] = (actions[action] || 0) + 1;
    });

    return Object.entries(actions).map(([action, count]) => ({
      action,
      count,
    }));
  });

  onMount(() => {
    loadAnalytics();
  });

  async function loadAnalytics() {
    loading = true;
    try {
      const [analyticsData, logData] = await Promise.all([
        api.sharing.getShareAnalytics(shareId),
        api.sharing.getShareAccessLog(shareId),
      ]);

      analytics = analyticsData;
      accessLog = logData || [];
    } catch (err) {
      console.error("Failed to load analytics:", err);
      errorToast(tr("failedToLoadAnalytics"));
    } finally {
      loading = false;
    }
  }

  function formatDate(dateString) {
    if (!dateString) return tr("never");
    return new Date(dateString).toLocaleString($currentLang);
  }

  function getActionIcon(action) {
    switch (action) {
      case "view":
        return "bi-eye";
      case "download":
        return "bi-download";
      case "upload":
        return "bi-upload";
      default:
        return "bi-activity";
    }
  }

  function getActionColor(action) {
    switch (action) {
      case "view":
        return "text-blue-500";
      case "download":
        return "text-green-500";
      case "upload":
        return "text-purple-500";
      default:
        return "text-gray-500";
    }
  }
</script>

<div class="max-w-7xl mx-auto p-6 space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
        {tr("shareAnalytics")}
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mt-1">
        {tr("trackShareActivity")}
      </p>
    </div>

    <!-- Time Range Selector -->
    <select
      bind:value={timeRange}
      class="px-4 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg"
    >
      <option value="7d">{tr("last7Days")}</option>
      <option value="30d">{tr("last30Days")}</option>
      <option value="90d">{tr("last90Days")}</option>
      <option value="all">{tr("allTime")}</option>
    </select>
  </div>

  {#if loading}
    <div class="flex justify-center py-12">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"
      ></div>
    </div>
  {:else if analytics}
    <!-- Stats Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {tr("totalViews")}
            </p>
            <p class="text-3xl font-bold text-gray-900 dark:text-white mt-2">
              {analytics.total_accesses || 0}
            </p>
          </div>
          <div
            class="w-12 h-12 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center"
          >
            <i class="bi bi-eye text-2xl text-blue-500"></i>
          </div>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {tr("uniqueVisitors")}
            </p>
            <p class="text-3xl font-bold text-gray-900 dark:text-white mt-2">
              {topUsers().length}
            </p>
          </div>
          <div
            class="w-12 h-12 bg-green-100 dark:bg-green-900 rounded-lg flex items-center justify-center"
          >
            <i class="bi bi-people text-2xl text-green-500"></i>
          </div>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {tr("lastAccessed")}
            </p>
            <p class="text-sm font-medium text-gray-900 dark:text-white mt-2">
              {formatDate(analytics.last_accessed)}
            </p>
          </div>
          <div
            class="w-12 h-12 bg-purple-100 dark:bg-purple-900 rounded-lg flex items-center justify-center"
          >
            <i class="bi bi-clock-history text-2xl text-purple-500"></i>
          </div>
        </div>
      </div>
    </div>

    <!-- Views Chart -->
    <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
        {tr("viewsOverTime")}
      </h2>

      {#if chartData().length > 0}
        <div class="space-y-2">
          {#each chartData() as day}
            <div class="flex items-center gap-3">
              <span class="text-sm text-gray-600 dark:text-gray-400 w-24">
                {new Date(day.date).toLocaleDateString($currentLang)}
              </span>
              <div
                class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-8 relative overflow-hidden"
              >
                <div
                  class="bg-gradient-to-r from-blue-500 to-blue-600 h-full rounded-full flex items-center justify-end px-3"
                  style="width: {Math.min(
                    100,
                    (day.views / Math.max(...chartData().map((d) => d.views))) *
                      100
                  )}%"
                >
                  <span class="text-white text-sm font-medium">{day.views}</span
                  >
                </div>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <p class="text-center text-gray-500 dark:text-gray-400 py-8">
          {tr("noDataForPeriod")}
        </p>
      {/if}
    </div>

    <!-- Top Users -->
    <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
        {tr("topViewers")}
      </h2>

      {#if topUsers().length > 0}
        <div class="space-y-3">
          {#each topUsers() as user, index}
            <div class="flex items-center gap-3">
              <div
                class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-purple-500 flex items-center justify-center text-white font-semibold text-sm"
              >
                #{index + 1}
              </div>
              <div class="flex-1">
                <p class="font-medium text-gray-900 dark:text-white">
                  {user.userId === "Anonymous" ? tr("anonymous") : user.userId}
                </p>
              </div>
              <span
                class="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 rounded-full text-sm font-medium"
              >
                {user.count}
                {tr("views")}
              </span>
            </div>
          {/each}
        </div>
      {:else}
        <p class="text-center text-gray-500 dark:text-gray-400 py-8">
          {tr("noViewersYet")}
        </p>
      {/if}
    </div>

    <!-- Action Breakdown -->
    <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
        {tr("activityBreakdown")}
      </h2>

      {#if actionBreakdown().length > 0}
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          {#each actionBreakdown() as item}
            <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
              <i
                class="bi {getActionIcon(item.action)} text-3xl {getActionColor(
                  item.action
                )} mb-2"
              ></i>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">
                {item.count}
              </p>
              <p class="text-sm text-gray-600 dark:text-gray-400 capitalize">
                {tr(item.action)}
              </p>
            </div>
          {/each}
        </div>
      {:else}
        <p class="text-center text-gray-500 dark:text-gray-400 py-8">
          {tr("noActivityYet")}
        </p>
      {/if}
    </div>

    <!-- Access Log -->
    <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
        {tr("recentActivity")}
      </h2>

      {#if filteredAccessLog().length > 0}
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-700">
                <th
                  class="text-left py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-400"
                  >{tr("time")}</th
                >
                <th
                  class="text-left py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-400"
                  >{tr("action")}</th
                >
                <th
                  class="text-left py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-400"
                  >{tr("user")}</th
                >
                <th
                  class="text-left py-3 px-4 text-sm font-semibold text-gray-600 dark:text-gray-400"
                  >{tr("ipAddress")}</th
                >
              </tr>
            </thead>
            <tbody>
              {#each filteredAccessLog().slice(0, 50) as log}
                <tr
                  class="border-b border-gray-100 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700"
                >
                  <td class="py-3 px-4 text-sm text-gray-900 dark:text-white">
                    {formatDate(log.accessed_at)}
                  </td>
                  <td class="py-3 px-4">
                    <span
                      class="inline-flex items-center gap-2 px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded-lg text-sm"
                    >
                      <i
                        class="bi {getActionIcon(log.action)} {getActionColor(
                          log.action
                        )}"
                      ></i>
                      <span class="capitalize text-gray-900 dark:text-white"
                        >{log.action}</span
                      >
                    </span>
                  </td>
                  <td class="py-3 px-4 text-sm text-gray-900 dark:text-white">
                    {log.user_id || tr("anonymous")}
                  </td>
                  <td
                    class="py-3 px-4 text-sm text-gray-600 dark:text-gray-400 font-mono"
                  >
                    {log.ip_address || "—"}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {:else}
        <p class="text-center text-gray-500 dark:text-gray-400 py-8">
          {tr("noAccessLogYet")}
        </p>
      {/if}
    </div>
  {:else}
    <div class="text-center py-12">
      <i class="bi bi-graph-up text-6xl text-gray-400 mb-4"></i>
      <p class="text-gray-500 dark:text-gray-400">
        {tr("noAnalyticsAvailable")}
      </p>
    </div>
  {/if}
</div>
