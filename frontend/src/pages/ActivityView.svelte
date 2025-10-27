<script>
  import { onMount } from "svelte";
  import { activity } from "../stores/activity";
  import { error as errorToast } from "../stores/toast";

  let groupedActivities = {};
  let selectedFilter = "all";
  let searchQuery = "";

  $: filteredActivities = filterActivities(
    $activity,
    selectedFilter,
    searchQuery
  );
  $: groupedActivities = groupByDate(filteredActivities);
  $: todayCount = activity.getToday().length;

  const activityTypes = [
    { value: "all", label: "All", icon: "list-ul" },
    { value: "upload", label: "Uploads", icon: "upload" },
    { value: "download", label: "Downloads", icon: "download" },
    { value: "delete", label: "Deletes", icon: "trash" },
    { value: "rename", label: "Renames", icon: "pencil" },
    { value: "move", label: "Moves", icon: "arrow-right" },
  ];

  const typeConfig = {
    upload: { label: "Uploaded", color: "success", icon: "upload" },
    download: { label: "Downloaded", color: "info", icon: "download" },
    delete: { label: "Deleted", color: "error", icon: "trash" },
    rename: { label: "Renamed", color: "warning", icon: "pencil" },
    create: { label: "Created", color: "primary", icon: "plus-circle" },
    move: { label: "Moved", color: "secondary", icon: "arrow-right" },
    share: { label: "Shared", color: "accent", icon: "share" },
    favorite: { label: "Favorited", color: "warning", icon: "star-fill" },
    unfavorite: { label: "Unfavorited", color: "neutral", icon: "star" },
  };

  onMount(async () => {
    await activity.load({ limit: 100 });
    const oldKey = "syncspace_activity";
    if (localStorage.getItem(oldKey)) {
      localStorage.removeItem(oldKey);
    }
  });

  function filterActivities(activities, filter, search) {
    let filtered = activities;

    if (filter !== "all") {
      filtered = filtered.filter((a) => a.type === filter);
    }

    if (search.trim()) {
      const query = search.toLowerCase();
      filtered = filtered.filter(
        (a) =>
          a.filename.toLowerCase().includes(query) ||
          a.path.toLowerCase().includes(query) ||
          (a.details && a.details.toLowerCase().includes(query))
      );
    }

    return filtered;
  }

  function groupByDate(activities) {
    const groups = {};
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    for (const act of activities) {
      const date = new Date(act.timestamp);
      const dateKey = new Date(
        date.getFullYear(),
        date.getMonth(),
        date.getDate()
      );

      let label;
      if (dateKey.getTime() === today.getTime()) {
        label = "Today";
      } else if (dateKey.getTime() === yesterday.getTime()) {
        label = "Yesterday";
      } else {
        label = formatDate(date);
      }

      if (!groups[label]) {
        groups[label] = [];
      }
      groups[label].push(act);
    }

    return groups;
  }

  function formatDate(date) {
    return date.toLocaleDateString("en-US", {
      weekday: "short",
      month: "short",
      day: "numeric",
      year:
        date.getFullYear() !== new Date().getFullYear() ? "numeric" : undefined,
    });
  }

  function formatTime(timestamp) {
    return new Date(timestamp).toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function getRelativeTime(timestamp) {
    const now = Date.now();
    const diff = now - timestamp;

    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return "Just now";
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    if (days < 7) return `${days}d ago`;
    return formatDate(new Date(timestamp));
  }

  function handleClearAll() {
    if (confirm("Clear all activity history?")) {
      activity.clear();
    }
  }
</script>

<div class="activity-view">
  <!-- Stats -->
  {#if $activity.length > 0}
    <div class="stats stats-vertical lg:stats-horizontal shadow mb-6 w-full">
      <div class="stat">
        <div class="stat-figure text-primary">
          <i class="bi bi-activity text-4xl"></i>
        </div>
        <div class="stat-title">Total Events</div>
        <div class="stat-value text-primary">{$activity.length}</div>
        <div class="stat-desc">All time</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-success">
          <i class="bi bi-calendar-check text-4xl"></i>
        </div>
        <div class="stat-title">Today</div>
        <div class="stat-value text-success">{todayCount}</div>
        <div class="stat-desc">Recent activity</div>
      </div>

      <div class="stat">
        <div class="stat-actions">
          <button
            class="px-3 py-1.5 text-sm bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
            on:click={handleClearAll}
            disabled={$activity.length === 0}
          >
            <i class="bi bi-trash-fill"></i>
            Clear All
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Filters & Search -->
  <div
    class="bg-white dark:bg-gray-900 rounded-lg border border-gray-300 dark:border-gray-600 shadow-sm mb-6"
  >
    <div class="p-4">
      <div class="flex flex-col md:flex-row gap-4">
        <!-- Filter Tabs -->
        <div
          role="tablist"
          class="flex flex-wrap gap-2 flex-1 bg-gray-100 dark:bg-gray-800 p-1 rounded-lg"
        >
          {#each activityTypes as type}
            <button
              role="tab"
              class="px-3 py-1.5 text-sm rounded-md transition-colors flex items-center gap-2 {selectedFilter ===
              type.value
                ? 'bg-white dark:bg-gray-700 text-blue-600 dark:text-blue-400 shadow-sm'
                : 'text-gray-700 dark:text-gray-300 hover:bg-white/50 dark:hover:bg-gray-700/50'}"
              on:click={() => (selectedFilter = type.value)}
            >
              <i class="bi bi-{type.icon}"></i>
              {type.label}
            </button>
          {/each}
        </div>

        <!-- Search -->
        <div class="flex items-center">
          <div
            class="flex rounded-lg border border-gray-300 dark:border-gray-600 overflow-hidden"
          >
            <input
              type="text"
              placeholder="Search activities..."
              class="px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 min-w-[200px]"
              bind:value={searchQuery}
            />
            <button
              class="px-3 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-200 transition-colors"
              aria-label="Search"
            >
              <i class="bi bi-search"></i>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Timeline -->
  {#if Object.keys(groupedActivities).length === 0}
    <div class="hero min-h-[400px]">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <i class="bi bi-clock-history text-7xl text-base-300 mb-4"></i>
          <h1 class="text-3xl font-bold">No Activity Yet</h1>
          <p class="py-6">File operations will appear here</p>
        </div>
      </div>
    </div>
  {:else}
    {#each Object.entries(groupedActivities) as [dateLabel, activities]}
      <div class="mb-8">
        <!-- Date Badge -->
        <div class="flex items-center gap-3 mb-4">
          <div
            class="px-4 py-2 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 rounded-lg font-bold text-sm"
          >
            {dateLabel}
          </div>
          <div class="flex-1 h-px bg-gray-200 dark:bg-gray-700"></div>
        </div>

        <!-- Custom Timeline -->
        <div
          class="relative pl-8 border-l-2 border-gray-200 dark:border-gray-700 space-y-6"
        >
          {#each activities as act, i}
            {@const config = typeConfig[act.type] || typeConfig.create}
            {@const colorMap = {
              success: "bg-green-500 dark:bg-green-600",
              info: "bg-blue-500 dark:bg-blue-600",
              error: "bg-red-500 dark:bg-red-600",
              warning: "bg-amber-500 dark:bg-amber-600",
              secondary: "bg-purple-500 dark:bg-purple-600",
              accent: "bg-pink-500 dark:bg-pink-600",
            }}
            {@const badgeColorMap = {
              success:
                "bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200",
              info: "bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200",
              error:
                "bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200",
              warning:
                "bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-200",
              secondary:
                "bg-purple-100 dark:bg-purple-900 text-purple-700 dark:text-purple-200",
              accent:
                "bg-pink-100 dark:bg-pink-900 text-pink-700 dark:text-pink-200",
            }}
            <div class="relative">
              <!-- Timeline Icon -->
              <div
                class="absolute -left-[2.25rem] top-2 w-10 h-10 rounded-full {colorMap[
                  config.color
                ] ||
                  'bg-gray-500'} flex items-center justify-center text-white shadow-md"
              >
                <i class="bi bi-{config.icon} text-lg"></i>
              </div>

              <!-- Time Stamp -->
              <div
                class="absolute -left-[10.5rem] top-2 text-xs text-gray-500 dark:text-gray-400 text-right w-32"
              >
                {formatTime(act.timestamp)}
              </div>

              <!-- Activity Card -->
              <div
                class="bg-white dark:bg-gray-900 rounded-lg border border-gray-300 dark:border-gray-600 shadow-sm hover:shadow-md transition-shadow ml-4"
              >
                <div class="p-4">
                  <div class="flex items-start justify-between gap-2">
                    <div class="flex-1">
                      <div
                        class="px-2 py-1 text-xs rounded flex items-center gap-1 w-fit mb-2 {badgeColorMap[
                          config.color
                        ] ||
                          'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300'}"
                      >
                        <i class="bi bi-{config.icon}"></i>
                        {config.label}
                      </div>
                      <h3
                        class="font-bold text-base text-gray-900 dark:text-white"
                      >
                        {act.filename}
                      </h3>
                      {#if act.path}
                        <p
                          class="text-xs font-mono text-gray-500 dark:text-gray-400 mt-1"
                        >
                          {act.path}
                        </p>
                      {/if}
                      {#if act.details}
                        <p
                          class="text-sm text-gray-600 dark:text-gray-300 mt-2"
                        >
                          {act.details}
                        </p>
                      {/if}
                      <div
                        class="text-xs text-gray-400 dark:text-gray-500 mt-2 italic"
                      >
                        {getRelativeTime(act.timestamp)}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .activity-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }
</style>
