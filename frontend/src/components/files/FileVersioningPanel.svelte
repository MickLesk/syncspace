<script>
  /**
   * File Versioning Timeline Component
   * Shows version history with diff viewer and restore functionality
   */
  import { onMount } from "svelte";
  import {
    versionState,
    versionTimeline,
    loading,
    restoring,
    error,
    loadVersions,
    downloadVersion,
    compareVersions,
    restoreVersion,
    deleteVersion,
    selectVersion,
    exitCompareMode,
    getVersionStats,
  } from "../stores/fileVersioning.js";

  let { filePath = "" } = $props();
  let { readOnly = false } = $props();

  let compareMode = false;
  let selectedV1 = null;
  let selectedV2 = null;
  let showStats = false;
  let showDiff = false;

  onMount(async () => {
    if (filePath) {
      await loadVersions(filePath);
    }
  });

  $: if (filePath) {
    loadVersions(filePath);
    compareMode = false;
  }

  async function handleCompare() {
    if (!selectedV1 || !selectedV2) return;
    showDiff = true;
    await compareVersions(filePath, selectedV1, selectedV2);
  }

  function formatDate(date) {
    return new Date(date).toLocaleDateString("de-DE", {
      month: "short",
      day: "numeric",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function formatSize(bytes) {
    if (!bytes) return "0 B";
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round((bytes / Math.pow(1024, i)) * 100) / 100 + " " + sizes[i];
  }

  function getStats() {
    return getVersionStats(filePath);
  }
</script>

<div
  class="flex flex-col h-full bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700"
>
  <!-- Header with Tabs -->
  <div
    class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700"
  >
    <div class="flex gap-2">
      <button
        class={!showStats
          ? "px-4 py-2 rounded text-sm font-medium transition-colors bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200"
          : "px-4 py-2 rounded text-sm font-medium transition-colors text-slate-600 dark:text-slate-400"}
        on:click={() => (showStats = false)}
      >
        <i class="bi bi-clock-history mr-2" />
        Timeline
      </button>

      <button
        class={showStats
          ? "px-4 py-2 rounded text-sm font-medium transition-colors bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200"
          : "px-4 py-2 rounded text-sm font-medium transition-colors text-slate-600 dark:text-slate-400"}
        on:click={() => (showStats = true)}
      >
        <i class="bi bi-pie-chart mr-2" />
        Stats
      </button>
    </div>

    {#if $versionState.versions.length > 0}
      <div class="text-sm text-slate-600 dark:text-slate-400">
        {$versionState.versions.length} version{$versionState.versions
          .length !== 1
          ? "s"
          : ""}
      </div>
    {/if}
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto p-4">
    {#if $loading}
      <div class="flex items-center justify-center h-full">
        <div class="animate-spin">
          <i class="bi bi-hourglass text-2xl text-blue-500" />
        </div>
      </div>
    {:else if $error}
      <div
        class="bg-red-50 dark:bg-red-900/20 p-3 rounded text-red-700 dark:text-red-200 text-sm"
      >
        <i class="bi bi-exclamation-circle mr-2" />
        {$error}
      </div>
    {:else if showStats}
      <!-- STATS VIEW -->
      {#if true}
        {@const stats = getStats()}
        <div class="space-y-4">
          <!-- Stats Grid -->
          <div class="grid grid-cols-2 gap-4">
            <div class="bg-blue-50 dark:bg-blue-900/20 p-4 rounded">
              <p class="text-xs text-blue-600 dark:text-blue-300 font-medium">
                Total Versions
              </p>
              <p
                class="text-3xl font-bold text-blue-700 dark:text-blue-200 mt-1"
              >
                {stats.totalVersions}
              </p>
            </div>

            <div class="bg-green-50 dark:bg-green-900/20 p-4 rounded">
              <p class="text-xs text-green-600 dark:text-green-300 font-medium">
                Total Size
              </p>
              <p
                class="text-2xl font-bold text-green-700 dark:text-green-200 mt-1"
              >
                {formatSize(stats.totalSize)}
              </p>
            </div>

            <div class="bg-purple-50 dark:bg-purple-900/20 p-4 rounded">
              <p
                class="text-xs text-purple-600 dark:text-purple-300 font-medium"
              >
                Avg Size
              </p>
              <p
                class="text-2xl font-bold text-purple-700 dark:text-purple-200 mt-1"
              >
                {formatSize(stats.avgSize)}
              </p>
            </div>

            <div class="bg-orange-50 dark:bg-orange-900/20 p-4 rounded">
              <p
                class="text-xs text-orange-600 dark:text-orange-300 font-medium"
              >
                Avg Timespan
              </p>
              <p
                class="text-2xl font-bold text-orange-700 dark:text-orange-200 mt-1"
              >
                {stats.avgTimeoutDays}d
              </p>
            </div>
          </div>

          <!-- Size Range -->
          <div class="bg-slate-50 dark:bg-slate-700/50 p-4 rounded">
            <p class="text-sm font-medium text-slate-900 dark:text-white mb-3">
              Size Range
            </p>
            <div class="space-y-2">
              <div
                class="flex justify-between text-xs text-slate-600 dark:text-slate-400"
              >
                <span>Smallest:</span>
                <span>{formatSize(stats.minSize)}</span>
              </div>
              <div
                class="w-full bg-slate-200 dark:bg-slate-600 rounded-full h-2"
              >
                <div
                  class="bg-gradient-to-r from-blue-400 to-blue-600 h-2 rounded-full"
                  style="width: {Math.min(
                    100,
                    Math.round(
                      ((stats.avgSize - stats.minSize) /
                        (stats.maxSize - stats.minSize)) *
                        100
                    )
                  )}%"
                />
              </div>
              <div
                class="flex justify-between text-xs text-slate-600 dark:text-slate-400"
              >
                <span>Largest:</span>
                <span>{formatSize(stats.maxSize)}</span>
              </div>
            </div>
          </div>
        </div>
      {/if}
    {:else}
      <!-- TIMELINE VIEW -->
      {#if $versionState.versions.length === 0}
        <p class="text-center text-slate-500 dark:text-slate-400 text-sm py-8">
          No versions available
        </p>
      {:else}
        <div class="space-y-4">
          <!-- Version List -->
          <div class="space-y-2">
            {#each $versionTimeline as version, idx (version.id)}
              <button
                type="button"
                class={version.isCurrent
                  ? "flex items-start gap-4 p-3 border rounded cursor-pointer transition-colors hover:bg-slate-50 dark:hover:bg-slate-700/50 border-blue-400 bg-blue-50 dark:bg-blue-900/20 w-full text-left"
                  : "flex items-start gap-4 p-3 border rounded cursor-pointer transition-colors hover:bg-slate-50 dark:hover:bg-slate-700/50 border-slate-200 dark:border-slate-700 w-full text-left"}
                onclick={() => selectVersion(version.id)}
              >
                <!-- Timeline Marker -->
                <div class="flex flex-col items-center">
                  <div
                    class={version.isCurrent
                      ? "w-4 h-4 rounded-full border-2 bg-blue-500 border-blue-500"
                      : "w-4 h-4 rounded-full border-2 border-slate-300 dark:border-slate-600"}
                  />
                  {#if idx < $versionTimeline.length - 1}
                    <div
                      class="w-0.5 h-12 bg-slate-200 dark:bg-slate-700 mt-1"
                    />
                  {/if}
                </div>

                <!-- Version Info -->
                <div class="flex-1 min-w-0">
                  <div class="flex items-center justify-between gap-2">
                    <h4 class="font-medium text-slate-900 dark:text-white">
                      v{version.versionNumber}
                      {#if version.isCurrent}
                        <span
                          class="ml-2 px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 text-xs font-semibold rounded"
                        >
                          Current
                        </span>
                      {/if}
                      {#if version.restored}
                        <span
                          class="ml-1 px-2 py-0.5 bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200 text-xs font-semibold rounded"
                        >
                          Restored
                        </span>
                      {/if}
                    </h4>
                  </div>

                  <p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
                    {formatDate(version.createdAt)}
                  </p>

                  <p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                    {formatSize(version.sizeBytes)} • By {version.createdBy ||
                      "System"}
                  </p>
                </div>

                <!-- Actions -->
                {#if !version.isCurrent}
                  <div class="flex gap-1">
                    <button
                      on:click|stopPropagation={() =>
                        downloadVersion(filePath, version.versionNumber)}
                      class="p-2 text-slate-600 hover:text-blue-600 dark:text-slate-400 dark:hover:text-blue-400 transition-colors"
                      title="Download"
                    >
                      <i class="bi bi-download" />
                    </button>

                    {#if !readOnly}
                      <button
                        on:click|stopPropagation={() =>
                          restoreVersion(filePath, version.versionNumber)}
                        disabled={$restoring}
                        class="p-2 text-slate-600 hover:text-green-600 dark:text-slate-400 dark:hover:text-green-400 disabled:opacity-50 transition-colors"
                        title="Restore"
                      >
                        <i class="bi bi-arrow-counterclockwise" />
                      </button>

                      <button
                        type="button"
                        on:click|stopPropagation={() =>
                          deleteVersion(filePath, version.versionNumber)}
                        class="p-2 text-slate-600 hover:text-red-600 dark:text-slate-400 dark:hover:text-red-400 transition-colors"
                        title="Delete"
                      >
                        <i class="bi bi-trash" />
                      </button>
                    {/if}
                  </div>
                {/if}
              </button>
            {/each}
          </div>

          <!-- Compare Section -->
          {#if !readOnly && $versionState.versions.length > 1}
            <div
              class="mt-6 pt-4 border-t border-slate-200 dark:border-slate-700"
            >
              <h4 class="font-medium text-slate-900 dark:text-white mb-3">
                Compare Versions
              </h4>

              <div class="space-y-3">
                <div>
                  <label
                    for="version-1-select"
                    class="text-sm text-slate-700 dark:text-slate-300 block mb-1"
                  >
                    Version 1
                  </label>
                  <select
                    id="version-1-select"
                    bind:value={selectedV1}
                    class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-800 text-slate-900 dark:text-white text-sm"
                  >
                    <option value={null}>Select version...</option>
                    {#each $versionState.versions as v}
                      <option value={v.id}>v{v.versionNumber}</option>
                    {/each}
                  </select>
                </div>

                <div>
                  <label
                    for="version-2-select"
                    class="text-sm text-slate-700 dark:text-slate-300 block mb-1"
                  >
                    Version 2
                  </label>
                  <select
                    id="version-2-select"
                    bind:value={selectedV2}
                    class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-800 text-slate-900 dark:text-white text-sm"
                  >
                    <option value={null}>Select version...</option>
                    {#each $versionState.versions as v}
                      <option value={v.id}>v{v.versionNumber}</option>
                    {/each}
                  </select>
                </div>

                <button
                  on:click={handleCompare}
                  disabled={!selectedV1 || !selectedV2 || $loading}
                  class="w-full px-4 py-2 bg-blue-500 hover:bg-blue-600 disabled:bg-slate-300 text-white rounded text-sm font-medium transition-colors"
                >
                  <i class="bi bi-diagram-2 mr-2" />
                  Compare Versions
                </button>
              </div>
            </div>
          {/if}

          <!-- Diff Result -->
          {#if showDiff && $versionState.diffResult}
            <div
              class="mt-4 pt-4 border-t border-slate-200 dark:border-slate-700"
            >
              <div class="flex items-center justify-between mb-3">
                <h4 class="font-medium text-slate-900 dark:text-white">
                  Diff Result
                </h4>
                <button
                  on:click={() => {
                    showDiff = false;
                    exitCompareMode();
                  }}
                  class="text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white"
                >
                  <i class="bi bi-x" />
                </button>
              </div>

              <div
                class="bg-slate-50 dark:bg-slate-700/50 rounded p-3 max-h-64 overflow-y-auto text-xs font-mono text-slate-600 dark:text-slate-400 space-y-1"
              >
                {#if $versionState.diffResult.changes && $versionState.diffResult.changes.length > 0}
                  {#each $versionState.diffResult.changes as change}
                    <div class="flex gap-2">
                      <span class="w-8"
                        >{change.type === "added"
                          ? "+"
                          : change.type === "removed"
                            ? "−"
                            : "~"}</span
                      >
                      <span>{change.line}</span>
                    </div>
                  {/each}
                {:else}
                  <p>No changes found</p>
                {/if}
              </div>
            </div>
          {/if}
        </div>
      {/if}
    {/if}
  </div>
</div>
