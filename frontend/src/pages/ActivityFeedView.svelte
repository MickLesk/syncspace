<script>
  import { onMount } from "svelte";
  import { activityFeed, ACTIVITY_TYPES } from "$stores/activityFeed.js";
  import { t } from "$lib/i18n.js";

  let searchQuery = "";
  let selectedType = "all";
  let showFilters = false;
  let dateStart = "";
  let dateEnd = "";
  let expandedActivity = null;

  onMount(async () => {
    await activityFeed.loadActivities();
  });

  function handleTypeFilter(type) {
    selectedType = type;
    activityFeed.filterByType(type);
  }

  function handleSearch(e) {
    searchQuery = e.target.value;
    activityFeed.search(searchQuery);
  }

  function handleDateFilter() {
    const start = dateStart ? new Date(dateStart) : null;
    const end = dateEnd ? new Date(dateEnd) : null;
    activityFeed.filterByDateRange(start, end);
  }

  function clearFilters() {
    searchQuery = "";
    selectedType = "all";
    dateStart = "";
    dateEnd = "";
    activityFeed.clearFilters();
  }

  function toggleExpanded(id) {
    expandedActivity = expandedActivity === id ? null : id;
  }

  function getActivityIcon(type) {
    return ACTIVITY_TYPES[type]?.icon || "bi-info-circle";
  }

  function getActivityColor(type) {
    return ACTIVITY_TYPES[type]?.color || "#6B7280";
  }

  function getActivityLabel(type) {
    return t(ACTIVITY_TYPES[type]?.label || "activity.unknown");
  }
</script>

<div
  class="p-6 bg-white dark:bg-gray-900 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700"
>
  <!-- Header -->
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
      <i class="bi bi-clock-history mr-2" aria-hidden="true"></i>
      {t("activity.feed_title")}
    </h2>
    <p class="text-sm text-gray-600 dark:text-gray-400">
      {t("activity.feed_description")}
    </p>
  </div>

  <!-- Statistics Cards -->
  {#if !$activityFeed.isLoading}
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
      <!-- Total Activities -->
      <div
        class="bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-900/10 rounded-lg p-4 border border-green-200 dark:border-green-700"
      >
        <p class="text-sm font-medium text-green-700 dark:text-green-300 mb-1">
          {t("activity.total")}
        </p>
        <p class="text-3xl font-bold text-green-900 dark:text-green-100">
          {$activityFeed.stats.totalActivities}
        </p>
        <p class="text-xs text-green-600 dark:text-green-400 mt-1">
          {t("activity.all_time")}
        </p>
      </div>

      <!-- Today -->
      <div
        class="bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-900/10 rounded-lg p-4 border border-green-200 dark:border-green-700"
      >
        <p class="text-sm font-medium text-green-700 dark:text-green-300 mb-1">
          {t("activity.today")}
        </p>
        <p class="text-3xl font-bold text-green-900 dark:text-green-100">
          {$activityFeed.stats.today}
        </p>
        <p class="text-xs text-green-600 dark:text-green-400 mt-1">
          {t("activity.events_today")}
        </p>
      </div>

      <!-- This Week -->
      <div
        class="bg-gradient-to-br from-yellow-50 to-yellow-100 dark:from-yellow-900/20 dark:to-yellow-900/10 rounded-lg p-4 border border-yellow-200 dark:border-yellow-700"
      >
        <p
          class="text-sm font-medium text-yellow-700 dark:text-yellow-300 mb-1"
        >
          {t("activity.this_week")}
        </p>
        <p class="text-3xl font-bold text-yellow-900 dark:text-yellow-100">
          {$activityFeed.stats.thisWeek}
        </p>
        <p class="text-xs text-yellow-600 dark:text-yellow-400 mt-1">
          {t("activity.last_7_days")}
        </p>
      </div>

      <!-- This Month -->
      <div
        class="bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/20 dark:to-purple-900/10 rounded-lg p-4 border border-purple-200 dark:border-purple-700"
      >
        <p
          class="text-sm font-medium text-purple-700 dark:text-purple-300 mb-1"
        >
          {t("activity.this_month")}
        </p>
        <p class="text-3xl font-bold text-purple-900 dark:text-purple-100">
          {$activityFeed.stats.thisMonth}
        </p>
        <p class="text-xs text-purple-600 dark:text-purple-400 mt-1">
          {t("activity.last_30_days")}
        </p>
      </div>
    </div>
  {/if}

  <!-- Search and Filters -->
  <div class="mb-8 pb-8 border-b border-gray-200 dark:border-gray-700">
    <!-- Search Bar -->
    <div class="mb-4">
      <div class="relative">
        <i
          class="bi bi-search absolute left-3 top-3 text-gray-400"
          aria-hidden="true"
        ></i>
        <input
          type="text"
          placeholder={t("activity.search_placeholder")}
          value={searchQuery}
          on:input={handleSearch}
          class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-800 dark:text-white focus:ring-2 focus:ring-green-500 focus:border-transparent"
        />
      </div>
    </div>

    <!-- Filter Toggle -->
    <button
      on:click={() => (showFilters = !showFilters)}
      class="flex items-center text-sm font-medium text-gray-700 dark:text-gray-300 hover:text-green-600 dark:hover:text-green-400 transition-colors mb-4"
    >
      <i class="bi bi-funnel mr-2" aria-hidden="true"></i>
      {showFilters ? t("activity.hide_filters") : t("activity.show_filters")}
    </button>

    {#if showFilters}
      <div
        class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg"
      >
        <!-- Date Range -->
        <div>
          <div
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            {t("activity.from_date")}
          </div>
          <input
            type="date"
            bind:value={dateStart}
            on:change={handleDateFilter}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          />
        </div>

        <div>
          <div
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            {t("activity.to_date")}
          </div>
          <input
            type="date"
            bind:value={dateEnd}
            on:change={handleDateFilter}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          />
        </div>

        <div>
          <div
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            {t("activity.action")}
          </div>
          <select
            bind:value={selectedType}
            on:change={() => handleTypeFilter(selectedType)}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700 dark:text-white"
          >
            <option value="all">{t("activity.all_types")}</option>
            {#each Object.entries(ACTIVITY_TYPES) as [key, config]}
              <option value={key}>{getActivityLabel(key)}</option>
            {/each}
          </select>
        </div>
      </div>

      <button
        on:click={clearFilters}
        class="px-3 py-1 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white border border-gray-300 dark:border-gray-600 rounded transition-colors"
      >
        {t("activity.clear_filters")}
      </button>
    {/if}

    <!-- Type Filter Chips -->
    <div class="flex flex-wrap gap-2 mt-4">
      {#each ["all", ...Object.keys(ACTIVITY_TYPES)] as type}
        <button
          on:click={() => handleTypeFilter(type)}
          class="px-3 py-1.5 text-sm font-medium rounded-full transition-all"
          class:bg-green-600={selectedType === type}
          class:text-white={selectedType === type}
          class:bg-gray-200={selectedType !== type}
          class:text-gray-700={selectedType !== type}
          class:dark:bg-gray-700={selectedType !== type}
          class:dark:text-gray-300={selectedType !== type}
        >
          {#if type !== "all"}
            <i class="{getActivityIcon(type)} mr-1"></i>
          {/if}
          {type === "all" ? t("activity.all") : getActivityLabel(type)}
        </button>
      {/each}
    </div>
  </div>

  <!-- Activity Timeline -->
  {#if $activityFeed.isLoading}
    <div class="flex items-center justify-center h-64">
      <div class="text-center">
        <div class="animate-spin mb-4">
          <i class="bi bi-hourglass text-3xl text-gray-400" aria-hidden="true"
          ></i>
        </div>
        <p class="text-gray-600 dark:text-gray-400">{t("common.loading")}</p>
      </div>
    </div>
  {:else if $activityFeed.filtered.length === 0}
    <div class="text-center py-12">
      <i
        class="bi bi-inbox text-5xl text-gray-300 dark:text-gray-600 mb-4 block"
        aria-hidden="true"
      ></i>
      <p class="text-gray-600 dark:text-gray-400 font-medium mb-2">
        {t("activity.no_activities")}
      </p>
      <p class="text-sm text-gray-500 dark:text-gray-500">
        {t("activity.check_filters")}
      </p>
    </div>
  {:else}
    <div class="space-y-3">
      {#each $activityFeed.filtered as activity (activity.id)}
        <div
          class="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 hover:shadow-md transition-shadow cursor-pointer"
          on:click={() => toggleExpanded(activity.id)}
        >
          <div class="flex items-start gap-4">
            <!-- Icon -->
            <div
              class="flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center"
              style="background-color: {getActivityColor(activity.type)}20;"
            >
              <i
                class="{getActivityIcon(activity.type)} text-lg"
                style="color: {getActivityColor(activity.type)}"
              ></i>
            </div>

            <!-- Content -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between mb-1">
                <p class="font-semibold text-gray-900 dark:text-white">
                  {getActivityLabel(activity.type)}
                </p>
                <p class="text-sm text-gray-500 dark:text-gray-400">
                  {activityFeed.formatTime(activity.timestamp)}
                </p>
              </div>

              <!-- Filename and Path -->
              <p class="text-sm text-gray-700 dark:text-gray-300 truncate">
                <span class="font-medium">{activity.filename}</span>
              </p>
              <p class="text-xs text-gray-500 dark:text-gray-500 truncate">
                {activity.filePath}
              </p>

              <!-- User and Timestamp -->
              <div
                class="flex items-center gap-3 mt-2 text-xs text-gray-600 dark:text-gray-400"
              >
                <span class="flex items-center">
                  <i class="bi bi-person-circle mr-1" aria-hidden="true"></i>
                  {activity.user}
                </span>
                <span class="flex items-center">
                  <i class="bi bi-calendar mr-1" aria-hidden="true"></i>
                  {new Date(activity.timestamp).toLocaleDateString()}
                </span>
              </div>
            </div>

            <!-- Expand Icon -->
            <div class="flex-shrink-0">
              <i
                class="bi transition-transform"
                class:bi-chevron-down={expandedActivity !== activity.id}
                class:bi-chevron-up={expandedActivity === activity.id}
              ></i>
            </div>
          </div>

          <!-- Expanded Details -->
          {#if expandedActivity === activity.id}
            <div
              class="mt-4 pt-4 border-t border-gray-300 dark:border-gray-600 space-y-2"
            >
              {#if activity.metadata.size}
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600 dark:text-gray-400"
                    >{t("activity.size")}:</span
                  >
                  <span class="font-medium text-gray-900 dark:text-white">
                    {#if activity.metadata.size > 0}
                      {Math.round(activity.metadata.size / 1024 / 1024)} MB
                    {:else}
                      N/A
                    {/if}
                  </span>
                </div>
              {/if}
              <div class="flex justify-between text-sm">
                <span class="text-gray-600 dark:text-gray-400"
                  >{t("activity.timestamp")}:</span
                >
                <span class="font-medium text-gray-900 dark:text-white">
                  {new Date(activity.timestamp).toLocaleString()}
                </span>
              </div>
              {#if activity.metadata.description}
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600 dark:text-gray-400"
                    >{t("activity.description")}:</span
                  >
                  <span class="font-medium text-gray-900 dark:text-white">
                    {activity.metadata.description}
                  </span>
                </div>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  <!-- Export Button -->
  {#if $activityFeed.filtered.length > 0}
    <div
      class="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700 flex justify-end"
    >
      <button
        on:click={() => activityFeed.export()}
        class="flex items-center px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors"
      >
        <i class="bi bi-download mr-2" aria-hidden="true"></i>
        {t("activity.export")}
      </button>
    </div>
  {/if}
</div>
