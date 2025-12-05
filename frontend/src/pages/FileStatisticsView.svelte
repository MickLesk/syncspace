<script>
  import { onMount } from "svelte";
  import { fileStatistics } from "$stores/fileStatistics.js";
  import { t } from "$lib/i18n.js";

  let selectedMetric = "byType";

  onMount(async () => {
    await fileStatistics.loadStatistics();
  });

  function handleMetricChange(metric) {
    selectedMetric = metric;
    fileStatistics.selectMetric(metric);
  }

  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function getHeatmapColor(value, max) {
    const intensity = value / max;
    if (intensity > 0.75) return "bg-red-500";
    if (intensity > 0.5) return "bg-yellow-500";
    if (intensity > 0.25) return "bg-green-400";
    return "bg-gray-300 dark:bg-gray-600";
  }
</script>

<div
  class="p-6 bg-white dark:bg-gray-900 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700"
>
  <!-- Header -->
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
      <i class="bi bi-graph-up mr-2" aria-hidden="true"></i>
      {t("statistics.dashboard_title")}
    </h2>
    <p class="text-sm text-gray-600 dark:text-gray-400">
      {t("statistics.dashboard_description")}
    </p>
  </div>

  <!-- Metric Selector -->
  <div class="mb-8 flex gap-3 flex-wrap">
    <button
      on:click={() => handleMetricChange("byType")}
      class="px-4 py-2 rounded-lg font-medium transition-all"
      class:bg-green-600={selectedMetric === "byType"}
      class:text-white={selectedMetric === "byType"}
      class:bg-gray-200={selectedMetric !== "byType"}
      class:text-gray-700={selectedMetric !== "byType"}
      class:dark:bg-gray-700={selectedMetric !== "byType"}
      class:dark:text-gray-300={selectedMetric !== "byType"}
    >
      <i class="bi bi-pie-chart mr-2" aria-hidden="true"></i>
      {t("statistics.by_type")}
    </button>

    <button
      on:click={() => handleMetricChange("bySize")}
      class="px-4 py-2 rounded-lg font-medium transition-all"
      class:bg-green-600={selectedMetric === "bySize"}
      class:text-white={selectedMetric === "bySize"}
      class:bg-gray-200={selectedMetric !== "bySize"}
      class:text-gray-700={selectedMetric !== "bySize"}
      class:dark:bg-gray-700={selectedMetric !== "bySize"}
      class:dark:text-gray-300={selectedMetric !== "bySize"}
    >
      <i class="bi bi-bar-chart mr-2" aria-hidden="true"></i>
      {t("statistics.by_size")}
    </button>

    <button
      on:click={() => handleMetricChange("byAccess")}
      class="px-4 py-2 rounded-lg font-medium transition-all"
      class:bg-green-600={selectedMetric === "byAccess"}
      class:text-white={selectedMetric === "byAccess"}
      class:bg-gray-200={selectedMetric !== "byAccess"}
      class:text-gray-700={selectedMetric !== "byAccess"}
      class:dark:bg-gray-700={selectedMetric !== "byAccess"}
      class:dark:text-gray-300={selectedMetric !== "byAccess"}
    >
      <i class="bi bi-fire mr-2" aria-hidden="true"></i>
      {t("statistics.access_patterns")}
    </button>
  </div>

  {#if $fileStatistics.isLoading}
    <div class="flex items-center justify-center h-64">
      <div class="text-center">
        <div class="animate-spin mb-4">
          <i class="bi bi-hourglass text-3xl text-gray-400" aria-hidden="true"
          ></i>
        </div>
        <p class="text-gray-600 dark:text-gray-400">{t("common.loading")}</p>
      </div>
    </div>
  {:else if selectedMetric === "byType"}
    <!-- File Type Distribution -->
    <div class="space-y-8">
      <div>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          {t("statistics.file_type_distribution")}
        </h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <!-- Bar Chart -->
          <div>
            <div class="space-y-3">
              {#each Object.entries($fileStatistics.fileTypeDistribution) as [type, data]}
                <div>
                  <div class="flex justify-between items-center mb-1">
                    <p
                      class="text-sm font-medium text-gray-700 dark:text-gray-300"
                    >
                      {type}
                    </p>
                    <p
                      class="text-sm font-semibold text-gray-900 dark:text-white"
                    >
                      {data.percentage}%
                    </p>
                  </div>
                  <div
                    class="w-full bg-gray-300 dark:bg-gray-600 rounded-full h-2 overflow-hidden"
                  >
                    <div
                      class="h-full transition-all duration-500"
                      style="width: {data.percentage}%; background-color: {data.color};"
                    ></div>
                  </div>
                  <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {data.count.toLocaleString()} files · {formatBytes(
                      data.size
                    )}
                  </p>
                </div>
              {/each}
            </div>
          </div>

          <!-- Stats Cards -->
          <div class="grid grid-cols-1 gap-3">
            {#each Object.entries($fileStatistics.fileTypeDistribution) as [type, data]}
              <div
                class="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border-l-4"
                style="border-color: {data.color}"
              >
                <p class="text-sm font-semibold text-gray-900 dark:text-white">
                  {type}
                </p>
                <div class="flex justify-between mt-1">
                  <p class="text-xs text-gray-600 dark:text-gray-400">
                    {data.count} files
                  </p>
                  <p class="text-xs font-medium text-gray-900 dark:text-white">
                    {data.percentage}%
                  </p>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Most Accessed Files -->
      <div class="pt-8 border-t border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          {t("statistics.most_accessed")}
        </h3>
        <div class="space-y-2">
          {#each $fileStatistics.mostAccessedFiles as file, index}
            <div
              class="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg flex items-center justify-between"
            >
              <div class="flex items-center gap-3">
                <div
                  class="flex-shrink-0 w-8 h-8 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center"
                >
                  <p
                    class="text-sm font-bold text-green-600 dark:text-green-400"
                  >
                    {index + 1}
                  </p>
                </div>
                <div>
                  <p class="text-sm font-medium text-gray-900 dark:text-white">
                    {file.name}
                  </p>
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    {file.lastAccess}
                  </p>
                </div>
              </div>
              <p
                class="text-sm font-semibold text-green-600 dark:text-green-400"
              >
                {file.accesses.toLocaleString()} accesses
              </p>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else if selectedMetric === "bySize"}
    <!-- Size Distribution -->
    <div class="space-y-8">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
        {t("statistics.size_distribution")}
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <!-- Size Buckets -->
        <div class="space-y-4">
          {#each $fileStatistics.sizeDistribution as bucket}
            <div>
              <div class="flex justify-between items-center mb-2">
                <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
                  {bucket.bucket}
                </p>
                <p class="text-sm font-semibold text-gray-900 dark:text-white">
                  {bucket.percentage}%
                </p>
              </div>
              <div
                class="w-full bg-gray-300 dark:bg-gray-600 rounded-full h-3 overflow-hidden"
              >
                <div
                  class="h-full bg-gradient-to-r from-green-500 to-green-600 transition-all duration-500"
                  style="width: {bucket.percentage}%"
                ></div>
              </div>
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {bucket.files} files
              </p>
            </div>
          {/each}
        </div>

        <!-- Summary Stats -->
        <div
          class="bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-900/10 rounded-lg p-6 border border-green-200 dark:border-green-700"
        >
          <h4
            class="text-sm font-semibold text-green-900 dark:text-green-100 mb-4"
          >
            {t("statistics.summary")}
          </h4>
          <div class="space-y-3">
            {#each $fileStatistics.sizeDistribution as bucket}
              <div class="flex justify-between text-sm">
                <span class="text-green-700 dark:text-green-300"
                  >{bucket.bucket}</span
                >
                <span class="font-medium text-green-900 dark:text-green-100"
                  >{bucket.files} ({bucket.percentage}%)</span
                >
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>
  {:else if selectedMetric === "byAccess"}
    <!-- Access Patterns & Heatmap -->
    <div class="space-y-8">
      <!-- Hourly Access Pattern -->
      <div>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          {t("statistics.hourly_access_pattern")}
        </h3>
        <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4 overflow-x-auto">
          <div class="flex items-end gap-1" style="height: 200px;">
            {#each $fileStatistics.fileAccessPatterns as pattern}
              <div class="flex-1 flex flex-col items-center">
                <div
                  class="w-full bg-green-500 rounded-t transition-all hover:bg-green-600"
                  style="height: {(pattern.accesses / 200) *
                    100}%; min-height: 4px;"
                  title="{pattern.hour}: {pattern.accesses} accesses"
                ></div>
                <p
                  class="text-xs text-gray-500 dark:text-gray-400 mt-2 rotate-45 whitespace-nowrap"
                >
                  {pattern.hour}
                </p>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Weekly Heatmap -->
      <div>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          {t("statistics.access_heatmap")}
        </h3>
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr>
                <th
                  class="text-xs font-semibold text-gray-600 dark:text-gray-400 text-left pb-2 pr-2"
                  >Day</th
                >
                {#each Array.from({ length: 24 }, (_, i) => i) as hour}
                  <th
                    class="text-xs font-semibold text-gray-600 dark:text-gray-400 pb-2 px-0.5 text-center"
                  >
                    {String(hour).padStart(2, "0")}
                  </th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each Object.entries($fileStatistics.accessHeatmap) as [day, hours]}
                <tr>
                  <td
                    class="text-xs font-medium text-gray-700 dark:text-gray-300 pr-2 py-2"
                    >{day}</td
                  >
                  {#each Array.from({ length: 24 }, (_, i) => i) as hour}
                    {#if hours[hour] !== undefined}
                      <td class="p-0.5">
                        <div
                          class="w-4 h-4 rounded {getHeatmapColor(
                            hours[hour],
                            100
                          )}"
                          title="{day} {String(hour).padStart(
                            2,
                            '0'
                          )}:00 - {hours[hour]} accesses"
                        ></div>
                      </td>
                    {/if}
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

        <!-- Heatmap Legend -->
        <div
          class="flex items-center gap-3 mt-4 text-xs text-gray-600 dark:text-gray-400"
        >
          <span>{t("statistics.less")}</span>
          <div class="flex gap-1">
            <div class="w-4 h-4 rounded bg-gray-300 dark:bg-gray-600"></div>
            <div class="w-4 h-4 rounded bg-green-400"></div>
            <div class="w-4 h-4 rounded bg-yellow-500"></div>
            <div class="w-4 h-4 rounded bg-red-500"></div>
          </div>
          <span>{t("statistics.more")}</span>
        </div>
      </div>
    </div>
  {/if}
</div>
