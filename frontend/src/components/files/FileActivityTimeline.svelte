<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { format, formatDistanceToNow, parseISO } from "date-fns";
  import { de, enUS } from "date-fns/locale";
  import api from "../../lib/api.js";

  const tr = (key, ...args) => t($currentLang, key, ...args);

  // Props
  let { filePath = "" } = $props();

  // State
  let activities = $state([]);
  let loading = $state(true);
  let timeRange = $state("all"); // all, today, week, month
  let groupBy = $state("none"); // none, hour, day
  let selectedActivity = $state(null);

  const dateLocale = $currentLang === "de" ? de : enUS;

  onMount(async () => {
    await loadActivities();
  });

  async function loadActivities() {
    loading = true;
    try {
      const response = await fetch(
        `http://localhost:8080/api/files/${encodeURIComponent(filePath)}/activity?limit=100`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (response.ok) {
        const data = await response.json();
        activities = Array.isArray(data) ? data : data.activities || [];
        // Sort by date descending
        activities.sort(
          (a, b) => new Date(b.created_at) - new Date(a.created_at)
        );
      }
    } catch (err) {
      console.error("Failed to load activities:", err);
    } finally {
      loading = false;
    }
  }

  // Filter activities by time range
  const filteredActivities = $derived.by(() => {
    const now = new Date();
    let startDate = null;

    switch (timeRange) {
      case "today":
        startDate = new Date(now.getFullYear(), now.getMonth(), now.getDate());
        break;
      case "week":
        startDate = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
        break;
      case "month":
        startDate = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);
        break;
      default:
        return activities;
    }

    return activities.filter((a) => new Date(a.created_at) >= startDate);
  });

  // Group activities
  const groupedActivities = $derived.by(() => {
    if (groupBy === "none") {
      return [{ label: null, items: filteredActivities }];
    }

    const groups = new Map();
    filteredActivities.forEach((activity) => {
      const date = parseISO(activity.created_at);
      let label = "";

      if (groupBy === "hour") {
        label = format(date, "PPp", { locale: dateLocale });
      } else if (groupBy === "day") {
        label = format(date, "PPP", { locale: dateLocale });
      }

      if (!groups.has(label)) {
        groups.set(label, []);
      }
      groups.get(label).push(activity);
    });

    return Array.from(groups, ([label, items]) => ({ label, items }));
  });

  function getActivityIcon(action) {
    const iconMap = {
      create: "bi-plus-circle",
      upload: "bi-cloud-upload",
      download: "bi-cloud-download",
      view: "bi-eye",
      edit: "bi-pencil",
      delete: "bi-trash",
      move: "bi-arrow-right",
      copy: "bi-files",
      rename: "bi-type",
      share: "bi-share",
      comment: "bi-chat-left",
      tag: "bi-tags",
      restore: "bi-arrow-counterclockwise",
      lock: "bi-lock",
      unlock: "bi-unlock",
    };
    return iconMap[action] || "bi-file";
  }

  function getActivityColor(action) {
    const colorMap = {
      create: "text-green-500",
      upload: "text-green-500",
      download: "text-green-500",
      view: "text-gray-500",
      edit: "text-orange-500",
      delete: "text-red-500",
      move: "text-purple-500",
      copy: "text-indigo-500",
      rename: "text-yellow-500",
      share: "text-cyan-500",
      comment: "text-pink-500",
      tag: "text-violet-500",
      restore: "text-green-500",
      lock: "text-red-500",
      unlock: "text-green-500",
    };
    return colorMap[action] || "text-gray-500";
  }

  function getActionLabel(action) {
    const labels = {
      create: tr("activity.created"),
      upload: tr("activity.uploaded"),
      download: tr("activity.downloaded"),
      view: tr("activity.viewed"),
      edit: tr("activity.edited"),
      delete: tr("activity.deleted"),
      move: tr("activity.moved"),
      copy: tr("activity.copied"),
      rename: tr("activity.renamed"),
      share: tr("activity.shared"),
      comment: tr("activity.commented"),
      tag: tr("activity.tagged"),
      restore: tr("activity.restored"),
      lock: tr("activity.locked"),
      unlock: tr("activity.unlocked"),
    };
    return labels[action] || action;
  }
</script>

<div class="flex flex-col h-full bg-white dark:bg-gray-800">
  <!-- Header -->
  <div
    class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700"
  >
    <h2
      class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2"
    >
      <i class="bi bi-clock-history text-indigo-500" aria-hidden="true"></i>
      {tr("activity.timeline")}
    </h2>
    <button
      onclick={() => loadActivities()}
      class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition"
      title={tr("refresh")}
    >
      <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
    </button>
  </div>

  <!-- Filters -->
  <div
    class="p-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700/50 space-y-3"
  >
    <div class="grid grid-cols-2 gap-3">
      <!-- Time Range Filter -->
      <div>
        <div
          class="text-xs font-medium text-gray-700 dark:text-gray-300 block mb-2"
        >
          {tr("activity.timeRange")}
        </div>
        <select
          bind:value={timeRange}
          class="w-full px-2 py-1.5 text-sm bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded text-gray-900 dark:text-white"
        >
          <option value="all">{tr("activity.allTime")}</option>
          <option value="today">{tr("activity.today")}</option>
          <option value="week">{tr("activity.thisWeek")}</option>
          <option value="month">{tr("activity.thisMonth")}</option>
        </select>
      </div>

      <!-- Group By Filter -->
      <div>
        <div
          class="text-xs font-medium text-gray-700 dark:text-gray-300 block mb-2"
        >
          {tr("activity.groupBy")}
        </div>
        <select
          bind:value={groupBy}
          class="w-full px-2 py-1.5 text-sm bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded text-gray-900 dark:text-white"
        >
          <option value="none">{tr("activity.noGrouping")}</option>
          <option value="hour">{tr("activity.byHour")}</option>
          <option value="day">{tr("activity.byDay")}</option>
        </select>
      </div>
    </div>

    <div class="text-xs text-gray-500 dark:text-gray-400">
      {filteredActivities.length}
      {tr("activity.eventsFound")}
    </div>
  </div>

  <!-- Activity Timeline -->
  <div class="flex-1 overflow-y-auto p-4 space-y-6">
    {#if loading}
      <div class="flex items-center justify-center h-32">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-500"
        ></div>
      </div>
    {:else if filteredActivities.length === 0}
      <div class="text-center py-12">
        <i
          class="bi bi-inbox text-4xl text-gray-300 dark:text-gray-600 mb-3 block"
        ></i>
        <p class="text-gray-500 dark:text-gray-400">
          {tr("activity.noActivities")}
        </p>
      </div>
    {:else}
      {#each groupedActivities as group}
        {#if group.label}
          <div
            class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider px-2 py-2 sticky top-0 bg-gray-50 dark:bg-gray-700/50"
          >
            {group.label}
          </div>
        {/if}

        <div class="space-y-3 relative">
          {#each group.items as activity, idx}
            <div class="flex gap-4">
              <!-- Timeline dot and line -->
              <div class="flex flex-col items-center">
                <div
                  class="w-10 h-10 rounded-full bg-white dark:bg-gray-700 border-2 border-indigo-500 flex items-center justify-center flex-shrink-0 relative z-10"
                >
                  <i
                    class="bi {getActivityIcon(
                      activity.action
                    )} {getActivityColor(activity.action)} text-sm"
                  ></i>
                </div>
                {#if idx < group.items.length - 1}
                  <div
                    class="w-0.5 h-12 bg-gradient-to-b from-indigo-500 to-indigo-200 dark:to-indigo-800 my-1"
                  ></div>
                {/if}
              </div>

              <!-- Activity Card -->
              <div class="flex-1 pb-3">
                <button
                  onclick={() =>
                    (selectedActivity =
                      selectedActivity?.id === activity.id ? null : activity)}
                  class="w-full text-left p-3 bg-gray-50 dark:bg-gray-700/50 border border-gray-200 dark:border-gray-600 rounded-lg hover:border-indigo-300 dark:hover:border-indigo-700 hover:bg-gray-100 dark:hover:bg-gray-700 transition"
                >
                  <div class="flex items-start justify-between gap-2">
                    <div class="flex-1 min-w-0">
                      <div
                        class="font-medium text-sm text-gray-900 dark:text-white"
                      >
                        {getActionLabel(activity.action)}
                      </div>
                      <div
                        class="text-xs text-gray-500 dark:text-gray-400 mt-1 flex items-center gap-2"
                      >
                        <span>{activity.user_name || "System"}</span>
                        <span>•</span>
                        <span
                          title={format(parseISO(activity.created_at), "PPpp", {
                            locale: dateLocale,
                          })}
                        >
                          {formatDistanceToNow(parseISO(activity.created_at), {
                            addSuffix: true,
                            locale: dateLocale,
                          })}
                        </span>
                      </div>
                    </div>

                    <i
                      class="bi {selectedActivity?.id === activity.id
                        ? 'bi-chevron-up'
                        : 'bi-chevron-down'} text-gray-400 flex-shrink-0"
                    ></i>
                  </div>
                </button>

                <!-- Expanded Details -->
                {#if selectedActivity?.id === activity.id}
                  <div
                    class="mt-2 p-3 bg-indigo-50 dark:bg-indigo-900/20 border border-indigo-200 dark:border-indigo-800 rounded-lg text-sm space-y-2"
                  >
                    <div class="grid grid-cols-2 gap-3">
                      <div>
                        <div
                          class="text-xs font-medium text-indigo-700 dark:text-indigo-300"
                        >
                          {tr("activity.timestamp")}
                        </div>
                        <div
                          class="text-sm text-gray-700 dark:text-gray-300 font-mono"
                        >
                          {format(parseISO(activity.created_at), "PPpp", {
                            locale: dateLocale,
                          })}
                        </div>
                      </div>

                      <div>
                        <div
                          class="text-xs font-medium text-indigo-700 dark:text-indigo-300"
                        >
                          {tr("activity.user")}
                        </div>
                        <div class="text-sm text-gray-700 dark:text-gray-300">
                          {activity.user_name || "System"}
                        </div>
                      </div>

                      {#if activity.metadata}
                        <div class="col-span-2">
                          <div
                            class="text-xs font-medium text-indigo-700 dark:text-indigo-300 mb-1"
                          >
                            {tr("activity.details")}
                          </div>
                          <div
                            class="text-xs text-gray-700 dark:text-gray-300 font-mono bg-white dark:bg-gray-800 p-2 rounded max-h-32 overflow-y-auto"
                          >
                            {JSON.stringify(activity.metadata, null, 2)}
                          </div>
                        </div>
                      {/if}

                      {#if activity.ip_address}
                        <div>
                          <div
                            class="text-xs font-medium text-indigo-700 dark:text-indigo-300"
                          >
                            {tr("activity.ip")}
                          </div>
                          <div
                            class="text-sm text-gray-700 dark:text-gray-300 font-mono break-all"
                          >
                            {activity.ip_address}
                          </div>
                        </div>
                      {/if}
                    </div>
                  </div>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/each}
    {/if}
  </div>
</div>
