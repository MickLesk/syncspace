<script>
  import {
    uploadQueue,
    uploadStatistics,
    isPaused,
    uploadSettings,
    pauseAllUploads,
    resumeAllUploads,
    pauseUpload,
    resumeUpload,
    cancelUpload,
    retryUpload,
    clearCompleted,
    clearAll,
  } from "../../stores/uploadQueue.js";
  import { t } from "../../i18n.js";

  let isMinimized = $state(false);
  let showSettings = $state(false);

  // Settings state
  let settingsForm = $state({
    speedLimit: $uploadSettings.speedLimit || 0,
    skipDuplicates: $uploadSettings.skipDuplicates || false,
    duplicateAction: $uploadSettings.duplicateAction || "ask",
  });

  const stats = $derived($uploadStatistics);
  const paused = $derived($isPaused);

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 ** 2) return (bytes / 1024).toFixed(1) + " KB";
    if (bytes < 1024 ** 3) return (bytes / 1024 ** 2).toFixed(1) + " MB";
    return (bytes / 1024 ** 3).toFixed(1) + " GB";
  }

  function formatSpeed(bytesPerSec) {
    return formatBytes(bytesPerSec) + "/s";
  }

  function formatTime(seconds) {
    if (seconds < 60) return Math.round(seconds) + "s";
    if (seconds < 3600)
      return Math.floor(seconds / 60) + "m " + Math.round(seconds % 60) + "s";
    return (
      Math.floor(seconds / 3600) +
      "h " +
      Math.floor((seconds % 3600) / 60) +
      "m"
    );
  }

  function estimatedTimeRemaining() {
    if (stats.avgSpeed === 0) return "—";
    const remaining = stats.totalSize - stats.uploadedSize;
    const seconds = remaining / stats.avgSpeed;
    return formatTime(seconds);
  }

  function togglePauseAll() {
    if (paused) {
      resumeAllUploads();
    } else {
      pauseAllUploads();
    }
  }

  function handlePauseResume(upload) {
    if (upload.status === "paused") {
      resumeUpload(upload.id);
    } else if (upload.status === "uploading") {
      pauseUpload(upload.id);
    }
  }

  function getStatusIcon(upload) {
    switch (upload.status) {
      case "complete":
        return "bi-check-circle-fill";
      case "error":
        return "bi-x-circle-fill";
      case "uploading":
        return "bi-cloud-arrow-up";
      case "paused":
        return "bi-pause-circle-fill";
      case "queued":
        return "bi-hourglass-split";
      default:
        return "bi-file-earmark";
    }
  }

  function getStatusClass(upload) {
    switch (upload.status) {
      case "complete":
        return "text-success";
      case "error":
        return "text-error";
      case "uploading":
        return "text-primary";
      case "paused":
        return "text-warning";
      case "queued":
        return "text-muted";
      default:
        return "text-muted";
    }
  }

  function saveSettings() {
    uploadSettings.set({
      speedLimit: settingsForm.speedLimit > 0 ? settingsForm.speedLimit : null,
      skipDuplicates: settingsForm.skipDuplicates,
      duplicateAction: settingsForm.duplicateAction,
    });
    showSettings = false;
  }
</script>

{#if $uploadQueue.length > 0}
  <div
    class="upload-manager-responsive fixed bottom-6 right-6 w-[450px] bg-slate-800 border-2 border-slate-400/30 rounded-xl shadow-2xl z-[1000] flex flex-col animate-slideIn {isMinimized
      ? 'max-h-[60px]'
      : 'max-h-[650px]'}"
  >
    <!-- Header -->
    <div
      class="flex justify-between items-center px-5 py-4 border-b border-slate-400/20 bg-slate-700 rounded-t-xl"
    >
      <div class="flex items-center gap-3 font-semibold text-[0.95rem]">
        <i
          class="bi bi-cloud-upload-fill text-blue-400 text-xl"
          aria-hidden="true"
        ></i>
        <span class="text-slate-200">
          {#if stats.uploading > 0}
            {$t("uploads.uploading")} {stats.uploading} {$t("files")}...
          {:else if stats.completed === stats.total}
            {$t("uploads.allComplete")}
          {:else}
            {$t("uploads.title")} ({stats.total})
          {/if}
        </span>

        {#if stats.uploading > 0}
          <span
            class="bg-blue-500/15 text-blue-400 px-2 py-0.5 rounded-md text-xs font-semibold"
            >{Math.round(stats.overallProgress)}%</span
          >
          <span
            class="bg-blue-500/15 text-blue-400 px-2 py-0.5 rounded-md text-xs font-semibold"
            >{formatSpeed(stats.avgSpeed)}</span
          >
        {/if}
      </div>

      <div class="flex gap-1">
        <!-- Settings -->
        <button
          class="w-8 h-8 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all hover:bg-slate-400/10 hover:text-slate-200"
          onclick={() => (showSettings = true)}
          title={$t("uploads.settings")}
        >
          <i class="bi bi-gear" aria-hidden="true"></i>
        </button>

        <!-- Pause/Resume All -->
        {#if stats.uploading > 0 || stats.paused > 0}
          <button
            class="w-8 h-8 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all hover:bg-slate-400/10 hover:text-slate-200"
            onclick={togglePauseAll}
            title={paused ? $t("uploads.resumeAll") : $t("uploads.pauseAll")}
          >
            <i
              class="bi bi-{paused ? 'play-fill' : 'pause-fill'}"
              aria-hidden="true"
            ></i>
          </button>
        {/if}

        <!-- Minimize/Expand -->
        <button
          aria-label="Toggle"
          class="w-8 h-8 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all hover:bg-slate-400/10 hover:text-slate-200"
          onclick={() => (isMinimized = !isMinimized)}
        >
          <i
            class="bi bi-{isMinimized ? 'chevron-up' : 'chevron-down'}"
            aria-hidden="true"
          ></i>
        </button>

        <!-- Close (only when no active uploads) -->
        {#if stats.uploading === 0 && stats.queued === 0}
          <button
            class="w-8 h-8 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all hover:bg-slate-400/10 hover:text-slate-200"
            aria-label="Close"
            onclick={clearAll}
          >
            <i class="bi bi-x" aria-hidden="true"></i>
          </button>
        {/if}
      </div>
    </div>

    {#if !isMinimized}
      <!-- Overall Progress Bar -->
      {#if stats.uploading > 0 || stats.queued > 0}
        <div class="px-5 pt-4 pb-3 bg-slate-800">
          <div class="flex justify-between mb-2 text-sm text-slate-300">
            <span
              >{formatBytes(stats.uploadedSize)} / {formatBytes(
                stats.totalSize
              )}</span
            >
            <span class="text-slate-400">
              {#if stats.avgSpeed > 0}
                ETA: {estimatedTimeRemaining()}
              {/if}
            </span>
          </div>
          <div class="h-2 bg-slate-400/20 rounded-full overflow-hidden">
            <div
              class="h-full bg-gradient-to-r from-blue-500 to-violet-500 rounded-full transition-all duration-300"
              style="width: {stats.overallProgress}%"
            ></div>
          </div>
        </div>
      {/if}

      <!-- Upload Settings Modal -->
      {#if showSettings}
        <div
          class="fixed inset-0 bg-black/50 flex items-center justify-center z-[10000]"
          onclick={() => (showSettings = false)}
          onkeydown={(e) => e.key === "Escape" && (showSettings = false)}
          role="button"
          tabindex="0"
        >
          <div
            class="bg-slate-800 rounded-xl w-[90%] max-w-[500px] shadow-2xl"
            onclick={(e) => e.stopPropagation()}
            onkeydown={(e) => e.stopPropagation()}
            role="dialog"
            tabindex="-1"
          >
            <div
              class="flex items-center justify-between p-6 border-b border-slate-700"
            >
              <h3 class="m-0 text-xl font-semibold">
                {$t("uploads.settings")}
              </h3>
              <button
                aria-label="Close"
                class="w-8 h-8 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all hover:bg-slate-400/10 hover:text-slate-200"
                onclick={() => (showSettings = false)}
              >
                <i class="bi bi-x-lg" aria-hidden="true"></i>
              </button>
            </div>

            <div class="p-6 flex flex-col gap-6">
              <!-- Speed Limit -->
              <div class="flex flex-col gap-2">
                <label
                  for="speed-limit-input"
                  class="font-medium text-slate-200 flex items-center gap-2"
                >
                  <i class="bi bi-speedometer2" aria-hidden="true"></i>
                  {$t("uploads.speedLimit")}
                </label>
                <div class="flex items-center gap-2">
                  <input
                    id="speed-limit-input"
                    type="number"
                    bind:value={settingsForm.speedLimit}
                    min="0"
                    step="100"
                    placeholder="0 = {$t('uploads.unlimited')}"
                    class="flex-1 bg-slate-900 border border-slate-700 rounded-md px-3 py-2 text-slate-200 focus:outline-none focus:border-blue-400 focus:ring-1 focus:ring-blue-400/20"
                  />
                  <span class="text-slate-400">KB/s</span>
                </div>
                <small class="text-slate-400 text-sm"
                  >{$t("uploads.speedLimitHint")}</small
                >
              </div>

              <!-- Duplicate Handling -->
              <div class="flex flex-col gap-2">
                <label
                  for="duplicate-handling-select"
                  class="font-medium text-slate-200 flex items-center gap-2"
                >
                  <i class="bi bi-files" aria-hidden="true"></i>
                  {$t("uploads.duplicateHandling")}
                </label>
                <select
                  id="duplicate-handling-select"
                  bind:value={settingsForm.duplicateAction}
                  class="bg-slate-900 border border-slate-700 rounded-md px-3 py-2 text-slate-200 focus:outline-none focus:border-blue-400 focus:ring-1 focus:ring-blue-400/20"
                >
                  <option value="ask">{$t("uploads.duplicateAsk")}</option>
                  <option value="skip">{$t("uploads.duplicateSkip")}</option>
                  <option value="replace"
                    >{$t("uploads.duplicateReplace")}</option
                  >
                  <option value="keep-both"
                    >{$t("uploads.duplicateKeepBoth")}</option
                  >
                </select>
                <small class="text-slate-400 text-sm"
                  >{$t("uploads.duplicateHandlingHint")}</small
                >
              </div>

              <!-- Skip Duplicates Checkbox -->
              <div class="flex flex-col gap-2">
                <label
                  class="flex items-center gap-2 cursor-pointer font-medium text-slate-200"
                >
                  <input
                    type="checkbox"
                    bind:checked={settingsForm.skipDuplicates}
                    class="w-5 h-5 cursor-pointer"
                  />
                  {$t("uploads.skipDuplicates")}
                </label>
                <small class="text-slate-400 text-sm"
                  >{$t("uploads.skipDuplicatesHint")}</small
                >
              </div>
            </div>

            <div class="flex justify-end gap-3 p-6 border-t border-slate-700">
              <button
                class="bg-transparent text-slate-400 border border-slate-700 px-4 py-2 rounded-md font-medium cursor-pointer transition-all hover:bg-slate-700 hover:text-slate-200"
                onclick={() => (showSettings = false)}
              >
                {$t("cancel")}
              </button>
              <button
                class="bg-blue-500 text-white border-none px-4 py-2 rounded-md font-medium cursor-pointer transition-colors hover:bg-blue-600"
                onclick={saveSettings}
              >
                {$t("save")}
              </button>
            </div>
          </div>
        </div>
      {/if}

      <!-- Upload List -->
      <div
        class="flex-1 overflow-y-auto p-2 max-h-[480px] bg-slate-800 scrollbar-modern"
      >
        {#each $uploadQueue as upload (upload.id)}
          <div
            class="flex gap-3.5 p-3.5 rounded-lg mb-1.5 bg-slate-700 transition-all hover:bg-slate-600 {upload.status ===
            'complete'
              ? 'opacity-70'
              : ''} {upload.status === 'error' ? 'bg-red-500/10' : ''}"
          >
            <!-- Icon -->
            <div class="text-2xl flex items-start pt-0.5">
              <i
                class="bi {getStatusIcon(upload)} {upload.status === 'complete'
                  ? 'text-green-500'
                  : upload.status === 'error'
                    ? 'text-red-500'
                    : upload.status === 'uploading'
                      ? 'text-blue-400'
                      : upload.status === 'paused'
                        ? 'text-amber-400'
                        : 'text-slate-400'}"
              ></i>
            </div>

            <!-- Details -->
            <div class="flex-1 min-w-0">
              <div
                class="font-medium text-sm text-slate-200 overflow-hidden text-ellipsis whitespace-nowrap mb-1.5"
                title={upload.name}
              >
                {upload.name}
              </div>

              <div class="text-xs text-slate-400 mb-2">
                {#if upload.status === "uploading"}
                  <span>
                    {upload.progress}% • {formatBytes(upload.uploadedBytes)} / {formatBytes(
                      upload.size
                    )}
                    {#if upload.speed}
                      • {formatSpeed(upload.speed)}
                    {/if}
                  </span>
                {:else if upload.status === "paused"}
                  <span class="text-amber-400">
                    {$t("uploads.paused")} • {upload.progress}% ({formatBytes(
                      upload.uploadedBytes
                    )})
                  </span>
                {:else if upload.status === "queued"}
                  <span class="text-slate-400">
                    {$t("uploads.queued")} • {formatBytes(upload.size)}
                  </span>
                {:else if upload.status === "complete"}
                  <span class="text-green-500">
                    {$t("uploads.complete")} • {formatBytes(upload.size)}
                  </span>
                {:else if upload.status === "error"}
                  <span class="text-red-500">
                    {upload.error || $t("uploads.failed")}
                  </span>
                {/if}
              </div>

              <!-- Progress Bar -->
              {#if upload.status === "uploading" || upload.status === "paused"}
                <div class="h-1 bg-slate-400/20 rounded-full overflow-hidden">
                  <div
                    class="h-full bg-gradient-to-r from-blue-500 to-violet-500 rounded-full transition-all duration-200"
                    style="width: {upload.progress}%"
                  ></div>
                </div>
              {/if}
            </div>

            <!-- Actions -->
            <div class="flex gap-1 items-start pt-0.5">
              {#if upload.status === "uploading"}
                <button
                  class="w-7 h-7 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-slate-400/10 hover:text-slate-200"
                  onclick={() => pauseUpload(upload.id)}
                  title={$t("uploads.pause")}
                >
                  <i class="bi bi-pause-fill" aria-hidden="true"></i>
                </button>
                <button
                  class="w-7 h-7 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-slate-400/10 hover:text-slate-200"
                  onclick={() => cancelUpload(upload.id)}
                  title={$t("cancel")}
                  aria-label="Cancel"
                >
                  <i class="bi bi-x-lg" aria-hidden="true"></i>
                </button>
              {:else if upload.status === "paused"}
                <button
                  class="w-7 h-7 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-slate-400/10 hover:text-slate-200"
                  onclick={() => resumeUpload(upload.id)}
                  title={$t("uploads.resume")}
                >
                  <i class="bi bi-play-fill" aria-hidden="true"></i>
                </button>
                <button
                  class="w-7 h-7 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-slate-400/10 hover:text-slate-200"
                  onclick={() => cancelUpload(upload.id)}
                  title={$t("cancel")}
                  aria-label="Cancel"
                >
                  <i class="bi bi-x-lg" aria-hidden="true"></i>
                </button>
              {:else if upload.status === "error"}
                <button
                  class="w-7 h-7 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-slate-400/10 hover:text-slate-200"
                  onclick={() => retryUpload(upload.id)}
                  title={$t("retry")}
                >
                  <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
                </button>
                <button
                  class="w-7 h-7 flex items-center justify-center bg-transparent text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-slate-400/10 hover:text-slate-200"
                  onclick={() => cancelUpload(upload.id)}
                  title={$t("remove")}
                  aria-label="Cancel"
                >
                  <i class="bi bi-trash" aria-hidden="true"></i>
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>

      <!-- Footer -->
      {#if $uploadQueue.length > 0}
        <div
          class="flex justify-between items-center px-5 py-3 border-t border-slate-400/20 bg-slate-700 rounded-b-xl"
        >
          <div class="text-sm text-slate-400 flex gap-2">
            <span
              >{stats.completed} / {stats.total}
              {$t("uploads.filesCompleted")}</span
            >
            {#if stats.failed > 0}
              <span class="text-red-500">• {stats.failed} {$t("failed")}</span>
            {/if}
            {#if stats.paused > 0}
              <span class="text-amber-400"
                >• {stats.paused} {$t("uploads.paused")}</span
              >
            {/if}
          </div>

          <div class="flex gap-2">
            {#if stats.completed > 0 && stats.uploading === 0}
              <button
                class="bg-transparent border-none text-blue-400 text-sm font-medium cursor-pointer px-2 py-1 rounded-md transition-all hover:bg-blue-500/10"
                onclick={clearCompleted}
              >
                {$t("uploads.clearCompleted")}
              </button>
            {/if}
          </div>
        </div>
      {/if}
    {/if}
  </div>
{/if}

<style>
  @media (max-width: 640px) {
    :global(.upload-manager-responsive) {
      width: calc(100vw - 2rem) !important;
      right: 1rem !important;
      bottom: 1rem !important;
    }
  }
</style>
