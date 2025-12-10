<script>
  /**
   * Advanced Upload UI Component
   * Displays upload queue with progress, speed, time remaining
   * Supports pause, resume, cancel, retry actions
   *
   * Usage:
   * <AdvancedUploadUI />
   */

  import {
    uploadManager,
    formatBytes,
    formatTime,
  } from "../../stores/uploadManager.js";

  let jobs = $state([]);
  let stats = $state({});
  let isExpanded = $state(true);

  $effect(() => {
    const unsubscribe = uploadManager.jobs.subscribe((j) => (jobs = j));
    return unsubscribe;
  });

  $effect(() => {
    const unsubscribe = uploadManager.uploadStats.subscribe((s) => (stats = s));
    return unsubscribe;
  });

  function getStatusIcon(state) {
    switch (state) {
      case "uploading":
        return "bi-arrow-up-circle-fill";
      case "completed":
        return "bi-check-circle-fill";
      case "failed":
        return "bi-x-circle-fill";
      case "paused":
        return "bi-pause-circle-fill";
      case "cancelled":
        return "bi-x-circle";
      default:
        return "bi-hourglass-split";
    }
  }

  function getStatusColor(state) {
    switch (state) {
      case "uploading":
        return "text-green-500";
      case "completed":
        return "text-green-500";
      case "failed":
        return "text-red-500";
      case "paused":
        return "text-yellow-500";
      default:
        return "text-gray-500";
    }
  }
</script>

{#if stats.total > 0}
  <div
    class="fixed bottom-0 right-0 m-4 w-96 bg-white dark:bg-gray-800 rounded-lg shadow-2xl overflow-hidden z-40"
  >
    <!-- Header -->
    <div
      class="bg-gradient-to-r from-primary-500 to-primary-600 text-white p-4 cursor-pointer flex justify-between items-center"
      onclick={() => (isExpanded = !isExpanded)}
      onkeydown={(e) =>
        (e.key === "Enter" || e.key === " ") && (isExpanded = !isExpanded)}
      role="button"
      tabindex="0"
    >
      <div class="flex items-center gap-2">
        <i class="bi bi-cloud-upload" aria-hidden="true"></i>
        <span class="font-semibold">Uploads</span>
        {#if stats.active > 0}
          <span class="bg-white/20 px-2 py-1 rounded-full text-xs"
            >{stats.active} active</span
          >
        {/if}
      </div>
      <button
        onclick={(e) => {
          e.stopPropagation();
          isExpanded = !isExpanded;
        }}
        class="text-xl"
      >
        {isExpanded ? "▼" : "▲"}
      </button>
    </div>

    {#if isExpanded}
      <!-- Progress Summary -->
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <div class="flex justify-between mb-2 text-sm">
          <span class="text-gray-600 dark:text-gray-400">Overall Progress</span>
          <span class="font-semibold text-gray-900 dark:text-white"
            >{stats.percentage}%</span
          >
        </div>
        <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2 mb-4">
          <div
            class="bg-gradient-to-r from-primary-500 to-primary-600 h-2 rounded-full transition-all duration-300"
            style="width: {stats.percentage}%"
          ></div>
        </div>

        <div class="grid grid-cols-3 gap-2 text-xs">
          <div>
            <p class="text-gray-600 dark:text-gray-400">Speed</p>
            <p class="font-semibold text-gray-900 dark:text-white">
              {formatBytes(stats.averageSpeed)}/s
            </p>
          </div>
          <div>
            <p class="text-gray-600 dark:text-gray-400">Uploaded</p>
            <p class="font-semibold text-gray-900 dark:text-white">
              {formatBytes(stats.sizeUploaded)} / {formatBytes(stats.sizeTotal)}
            </p>
          </div>
          <div>
            <p class="text-gray-600 dark:text-gray-400">Files</p>
            <p class="font-semibold text-gray-900 dark:text-white">
              {stats.completed} / {stats.total}
            </p>
          </div>
        </div>
      </div>

      <!-- Upload Jobs List -->
      <div
        class="divide-y divide-gray-200 dark:divide-gray-700 max-h-96 overflow-y-auto"
      >
        {#each jobs as job (job.id)}
          <div
            class="p-4 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
          >
            <div class="flex items-center gap-3 mb-2">
              <i
                class={`bi ${getStatusIcon(job.state)} ${getStatusColor(job.state)}`}
              ></i>
              <div class="flex-1 min-w-0">
                <p class="font-medium text-gray-900 dark:text-white truncate">
                  {job.filename}
                </p>
                <p class="text-xs text-gray-500 dark:text-gray-400 truncate">
                  {job.destination}
                </p>
              </div>
              <span
                class="text-xs font-semibold text-gray-600 dark:text-gray-400 whitespace-nowrap"
              >
                {formatBytes(job.size)}
              </span>
            </div>

            {#if job.state === "uploading" || job.state === "paused"}
              <div class="mb-2">
                <div
                  class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1.5 mb-1"
                >
                  <div
                    class="bg-primary-500 h-1.5 rounded-full transition-all duration-300"
                    style="width: {job.progress}%"
                  ></div>
                </div>
                <div
                  class="flex justify-between text-xs text-gray-600 dark:text-gray-400"
                >
                  <span>{job.progress}%</span>
                  {#if job.state === "uploading"}
                    <span>
                      {formatBytes(job.speedBytesPerSecond)}/s · ETA: {formatTime(
                        job.estimatedTimeRemaining
                      )}
                    </span>
                  {/if}
                </div>
              </div>
            {/if}

            {#if job.error}
              <p class="text-xs text-red-600 dark:text-red-400 mb-2">
                {job.error}
              </p>
            {/if}

            <!-- Action Buttons -->
            <div class="flex gap-2 text-xs">
              {#if job.state === "uploading"}
                <button
                  onclick={() => uploadManager.pauseUpload(job.id)}
                  class="px-2 py-1 bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-400 rounded hover:bg-yellow-200 dark:hover:bg-yellow-900/50 transition-colors"
                >
                  <i class="bi bi-pause-fill" aria-hidden="true"></i> Pause
                </button>
              {:else if job.state === "paused"}
                <button
                  onclick={() => uploadManager.resumeUpload(job.id)}
                  class="px-2 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 rounded hover:bg-green-200 dark:hover:bg-green-900/50 transition-colors"
                >
                  <i class="bi bi-play-fill" aria-hidden="true"></i> Resume
                </button>
              {/if}

              {#if job.state === "uploading" || job.state === "paused"}
                <button
                  onclick={() => uploadManager.cancelUpload(job.id)}
                  class="px-2 py-1 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 rounded hover:bg-red-200 dark:hover:bg-red-900/50 transition-colors"
                >
                  <i class="bi bi-x-lg" aria-hidden="true"></i> Cancel
                </button>
              {/if}

              {#if job.state === "failed"}
                <button
                  onclick={() => uploadManager.retryUpload(job.id)}
                  class="px-2 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 rounded hover:bg-green-200 dark:hover:bg-green-900/50 transition-colors"
                >
                  <i class="bi bi-arrow-repeat" aria-hidden="true"></i> Retry
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>

      <!-- Footer Actions -->
      {#if stats.completed > 0 || stats.failed > 0}
        <div
          class="p-3 border-t border-gray-200 dark:border-gray-700 flex gap-2"
        >
          <button
            onclick={() => uploadManager.clearCompleted()}
            class="flex-1 px-3 py-2 text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
          >
            Clear Completed
          </button>
        </div>
      {/if}
    {/if}
  </div>
{/if}

<style>
  ::-webkit-scrollbar {
    width: 6px;
  }

  ::-webkit-scrollbar-track {
    background: transparent;
  }

  ::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  ::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
  }
</style>
