<script>
  import {
    uploadQueue,
    uploadQueueManager,
    UPLOAD_STATUS,
    PRIORITY,
    uploadStats,
  } from "../../lib/uploadQueue";
  import { fade, fly, slide } from "svelte/transition";
  import { t } from "../../i18n";
  import { currentLang } from "../../stores/ui";

  let { compact = false, showCompleted = false } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Filter uploads based on showCompleted
  const displayQueue = $derived(
    showCompleted
      ? $uploadQueue
      : $uploadQueue.filter((u) => u.status !== UPLOAD_STATUS.COMPLETED)
  );

  function formatBytes(bytes) {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatTime(timestamp) {
    if (!timestamp) return "";
    const date = new Date(timestamp);
    return date.toLocaleTimeString();
  }

  function getStatusIcon(status) {
    switch (status) {
      case UPLOAD_STATUS.QUEUED:
        return "bi-hourglass";
      case UPLOAD_STATUS.UPLOADING:
        return "bi-arrow-up-circle";
      case UPLOAD_STATUS.PAUSED:
        return "bi-pause-circle";
      case UPLOAD_STATUS.COMPLETED:
        return "bi-check-circle-fill";
      case UPLOAD_STATUS.FAILED:
        return "bi-x-circle-fill";
      case UPLOAD_STATUS.CANCELLED:
        return "bi-dash-circle-fill";
      default:
        return "bi-question-circle";
    }
  }

  function getStatusColor(status) {
    switch (status) {
      case UPLOAD_STATUS.QUEUED:
        return "text-gray-500";
      case UPLOAD_STATUS.UPLOADING:
        return "text-green-500";
      case UPLOAD_STATUS.PAUSED:
        return "text-orange-500";
      case UPLOAD_STATUS.COMPLETED:
        return "text-green-500";
      case UPLOAD_STATUS.FAILED:
        return "text-red-500";
      case UPLOAD_STATUS.CANCELLED:
        return "text-gray-400";
      default:
        return "text-gray-500";
    }
  }

  function getPriorityLabel(priority) {
    switch (priority) {
      case PRIORITY.HIGH:
        return "High";
      case PRIORITY.NORMAL:
        return "Normal";
      case PRIORITY.LOW:
        return "Low";
      default:
        return "Unknown";
    }
  }

  function handlePauseResume(upload) {
    if (upload.status === UPLOAD_STATUS.UPLOADING) {
      uploadQueueManager.pause(upload.id);
    } else if (
      upload.status === UPLOAD_STATUS.PAUSED ||
      upload.status === UPLOAD_STATUS.QUEUED
    ) {
      uploadQueueManager.resume(upload.id);
    }
  }

  function handleCancel(uploadId) {
    if (confirm("Cancel this upload?")) {
      uploadQueueManager.cancel(uploadId);
    }
  }

  function handleClearCompleted() {
    uploadQueueManager.clearCompleted();
  }

  function handleClearFailed() {
    uploadQueueManager.clearFailed();
  }

  function handleRetry(uploadId) {
    uploadQueueManager.resume(uploadId);
  }
</script>

{#if displayQueue.length > 0}
  <div
    class="fixed bottom-5 right-5 w-[420px] max-h-[500px] bg-white dark:bg-gray-800 rounded-2xl shadow-xl overflow-hidden z-[1000] border-2 border-black/5 dark:border-white/10 {compact
      ? 'w-[340px] max-h-[400px]'
      : ''} max-sm:w-[calc(100vw-40px)] max-sm:max-h-[300px] max-sm:bottom-2.5 max-sm:right-2.5"
    transition:fly={{ y: 20, duration: 300 }}
  >
    <!-- Header -->
    <div
      class="px-5 py-4 border-b-2 border-black/5 dark:border-white/10 flex justify-between items-center bg-gradient-to-br from-indigo-500/5 to-purple-500/5 dark:from-indigo-500/10 dark:to-purple-500/10"
    >
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-xl bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center text-white text-lg"
        >
          <i class="bi bi-cloud-upload" aria-hidden="true"></i>
        </div>
        <div>
          <h4
            class="text-sm font-bold text-gray-900 dark:text-gray-100 m-0 max-sm:text-xs"
          >
            Upload Queue
          </h4>
          <p
            class="text-[11px] text-gray-500 dark:text-gray-400 mt-0.5 max-sm:text-[10px]"
          >
            {$uploadStats.completedFiles}/{$uploadStats.totalFiles} completed • {formatBytes(
              $uploadStats.uploadedBytes
            )}/{formatBytes($uploadStats.totalBytes)}
          </p>
        </div>
      </div>

      <div class="flex gap-2">
        <button
          class="w-8 h-8 rounded-lg border-none bg-black/5 dark:bg-white/5 text-gray-500 dark:text-gray-400 cursor-pointer flex items-center justify-center transition-all duration-200 hover:enabled:bg-black/10 dark:hover:enabled:bg-white/10 hover:enabled:text-gray-900 dark:hover:enabled:text-gray-100 disabled:opacity-30 disabled:cursor-not-allowed"
          onclick={handleClearCompleted}
          disabled={$uploadQueue.filter(
            (u) => u.status === UPLOAD_STATUS.COMPLETED
          ).length === 0}
          title="Clear completed"
          aria-label="Clear completed uploads"
        >
          <i class="bi bi-check-circle" aria-hidden="true"></i>
        </button>
        <button
          class="w-8 h-8 rounded-lg border-none bg-black/5 dark:bg-white/5 text-gray-500 dark:text-gray-400 cursor-pointer flex items-center justify-center transition-all duration-200 hover:enabled:bg-black/10 dark:hover:enabled:bg-white/10 hover:enabled:text-gray-900 dark:hover:enabled:text-gray-100 disabled:opacity-30 disabled:cursor-not-allowed"
          onclick={handleClearFailed}
          disabled={$uploadQueue.filter(
            (u) => u.status === UPLOAD_STATUS.FAILED
          ).length === 0}
          title="Clear failed"
          aria-label="Clear failed uploads"
        >
          <i class="bi bi-x-circle" aria-hidden="true"></i>
        </button>
      </div>
    </div>

    <!-- Upload items -->
    <div class="max-h-[400px] overflow-y-auto p-3 scrollbar-thin">
      {#each displayQueue as upload (upload.id)}
        <div
          class="flex gap-3 p-3 rounded-xl bg-black/[0.02] dark:bg-white/[0.03] mb-2 transition-all duration-200 {upload.status ===
          UPLOAD_STATUS.UPLOADING
            ? 'bg-blue-500/5 border border-blue-500/15'
            : upload.status === UPLOAD_STATUS.COMPLETED
              ? 'bg-green-500/5 border border-green-500/15'
              : upload.status === UPLOAD_STATUS.FAILED
                ? 'bg-red-500/5 border border-red-500/15'
                : ''}"
          transition:slide={{ duration: 200 }}
        >
          <!-- Icon -->
          <div
            class="text-2xl flex items-center justify-center {getStatusColor(
              upload.status
            )}"
          >
            <i class="bi {getStatusIcon(upload.status)}"></i>
          </div>

          <!-- Details -->
          <div class="flex-1 min-w-0">
            <div
              class="text-[13px] font-semibold text-gray-900 dark:text-gray-100 mb-1 overflow-hidden text-ellipsis whitespace-nowrap"
            >
              {upload.fileName}
            </div>
            <div class="flex flex-wrap gap-2 mb-1.5">
              <span
                class="text-[10px] text-gray-500 dark:text-gray-400 flex items-center gap-1"
              >
                <i class="bi bi-hdd" aria-hidden="true"></i>
                {formatBytes(upload.size)}
              </span>
              {#if upload.isChunked}
                <span
                  class="text-[10px] text-gray-500 dark:text-gray-400 flex items-center gap-1"
                >
                  <i class="bi bi-puzzle" aria-hidden="true"></i>
                  {upload.currentChunk}/{upload.totalChunks} chunks
                </span>
              {/if}
              {#if upload.retries > 0}
                <span
                  class="text-[10px] text-orange-500 flex items-center gap-1"
                >
                  <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
                  Retry {upload.retries}
                </span>
              {/if}
              {#if upload.error}
                <span
                  class="text-[10px] text-red-500 flex items-center gap-1"
                  title={upload.error}
                >
                  <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
                  {upload.error}
                </span>
              {/if}
            </div>

            <!-- Progress bar -->
            {#if upload.status === UPLOAD_STATUS.UPLOADING || upload.status === UPLOAD_STATUS.PAUSED}
              <div
                class="relative h-1.5 bg-black/5 dark:bg-white/10 rounded-sm overflow-hidden"
              >
                <div
                  class="h-full rounded-sm transition-[width] duration-300 ease-out {upload.status ===
                  UPLOAD_STATUS.PAUSED
                    ? 'bg-gradient-to-r from-amber-500 to-orange-500'
                    : 'bg-gradient-to-r from-indigo-500 to-purple-500'}"
                  style="width: {upload.progress}%"
                ></div>
                <span
                  class="absolute right-1 -top-[18px] text-[9px] font-semibold text-gray-500"
                  >{upload.progress}%</span
                >
              </div>
            {/if}
          </div>

          <!-- Actions -->
          <div class="flex gap-1.5 items-center">
            {#if upload.status === UPLOAD_STATUS.UPLOADING || upload.status === UPLOAD_STATUS.PAUSED}
              <button
                class="w-7 h-7 rounded-lg border-none bg-black/5 dark:bg-white/5 text-gray-500 dark:text-gray-400 cursor-pointer flex items-center justify-center transition-all duration-200 text-[13px] hover:bg-black/10 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-gray-100"
                onclick={() => handlePauseResume(upload)}
                title={upload.status === UPLOAD_STATUS.UPLOADING
                  ? "Pause"
                  : "Resume"}
                aria-label={upload.status === UPLOAD_STATUS.UPLOADING
                  ? "Pause upload"
                  : "Resume upload"}
              >
                <i
                  class="bi {upload.status === UPLOAD_STATUS.UPLOADING
                    ? 'bi-pause'
                    : 'bi-play'}"
                ></i>
              </button>
            {/if}

            {#if upload.status === UPLOAD_STATUS.FAILED}
              <button
                class="w-7 h-7 rounded-lg border-none bg-black/5 dark:bg-white/5 text-gray-500 dark:text-gray-400 cursor-pointer flex items-center justify-center transition-all duration-200 text-[13px] hover:bg-black/10 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-gray-100"
                onclick={() => handleRetry(upload.id)}
                title="Retry"
                aria-label="Retry upload"
              >
                <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
              </button>
            {/if}

            {#if upload.status !== UPLOAD_STATUS.COMPLETED}
              <button
                class="w-7 h-7 rounded-lg border-none bg-black/5 dark:bg-white/5 text-gray-500 dark:text-gray-400 cursor-pointer flex items-center justify-center transition-all duration-200 text-[13px] hover:bg-red-500/10 hover:text-red-500 dark:hover:bg-red-500/15"
                onclick={() => handleCancel(upload.id)}
                title="Cancel"
                aria-label="Cancel upload"
              >
                <i class="bi bi-x" aria-hidden="true"></i>
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  /* Custom scrollbar styling */
  .scrollbar-thin::-webkit-scrollbar {
    width: 6px;
  }

  .scrollbar-thin::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 3px;
  }

  .scrollbar-thin::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  :global(.dark) .scrollbar-thin::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
  }

  :global(.dark) .scrollbar-thin::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
  }
</style>
