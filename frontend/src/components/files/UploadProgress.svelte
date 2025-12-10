<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { uploads = [] } = $props();

  function formatBytes(bytes) {
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 ** 2) return (bytes / 1024).toFixed(1) + " KB";
    if (bytes < 1024 ** 3) return (bytes / 1024 ** 2).toFixed(1) + " MB";
    return (bytes / 1024 ** 3).toFixed(1) + " GB";
  }

  function getStatusClass(status) {
    return status === "complete"
      ? "success"
      : status === "error"
        ? "error"
        : status === "cancelled"
          ? "warning"
          : "primary";
  }
</script>

{#if uploads.length > 0}
  <div
    class="fixed bottom-4 right-4 w-[400px] max-h-[500px] bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-[0_10px_40px_rgba(0,0,0,0.15)] z-[100] animate-slideUp"
  >
    <div
      class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 rounded-t-xl"
    >
      <h3
        class="font-semibold flex items-center gap-2 m-0 text-gray-900 dark:text-gray-50"
      >
        <i class="bi bi-cloud-upload" aria-hidden="true"></i>
        {tr("uploads")} ({uploads.length})
      </h3>
      <button
        class="px-3 py-1 text-sm bg-transparent border border-gray-300 dark:border-gray-600 rounded-md text-gray-700 dark:text-gray-300 cursor-pointer transition-all hover:bg-gray-100 dark:hover:bg-gray-700"
        >{tr("clearAll")}</button
      >
    </div>

    <div class="max-h-[400px] overflow-y-auto p-2">
      {#each uploads as upload (upload.id)}
        <div
          class="flex gap-3 p-3 rounded-lg transition-colors hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          <div class="text-2xl flex items-center">
            {#if upload.status === "complete"}<i
                class="bi bi-check-circle-fill text-green-500 animate-scaleIn"
              ></i>
            {:else if upload.status === "error"}<i
                class="bi bi-x-circle-fill text-red-500 animate-shake"
              ></i>
            {:else if upload.status === "cancelled"}<i
                class="bi bi-dash-circle-fill text-amber-500"
              ></i>
            {:else if upload.status === "retrying"}<i
                class="bi bi-arrow-clockwise text-amber-500 animate-spin"
              ></i>
            {:else}<i
                class="bi bi-file-earmark-arrow-up text-green-500"
                aria-hidden="true"
              ></i>{/if}
          </div>

          <div class="flex-1 min-w-0">
            <div
              class="font-medium overflow-hidden text-ellipsis whitespace-nowrap text-gray-900 dark:text-gray-50"
            >
              {upload.name}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
              {#if upload.status === "uploading"}
                {formatBytes((upload.progress / 100) * upload.size)} / {formatBytes(
                  upload.size
                )} • {Math.round(upload.progress)}%
              {:else if upload.status === "retrying"}
                Retrying... (Attempt {upload.retries + 1}/{3}) • {formatBytes(
                  upload.size
                )}
              {:else if upload.status === "complete"}
                {formatBytes(upload.size)} • Complete
              {:else if upload.status === "error"}
                {upload.error || "Upload failed"}
                {#if upload.retries > 0}
                  • Failed after {upload.retries} retries
                {/if}
              {:else}
                {formatBytes(upload.size)} • Cancelled
              {/if}
            </div>
            {#if upload.status === "uploading" || upload.status === "retrying"}
              <progress
                class="progress progress-{getStatusClass(upload.status)} w-full"
                value={upload.progress}
                max="100"
              ></progress>
            {/if}
          </div>

          <div class="flex items-center">
            {#if upload.status === "uploading"}
              <button
                class="px-2 py-1 text-xs bg-transparent border-none text-gray-500 dark:text-gray-400 cursor-pointer rounded hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-gray-50 flex items-center gap-1"
                aria-label="Cancel upload"
                ><i class="bi bi-x-lg" aria-hidden="true"></i></button
              >
            {:else if upload.status === "error"}
              <button
                class="px-2 py-1 text-xs bg-transparent border-none text-gray-500 dark:text-gray-400 cursor-pointer rounded hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-gray-50 flex items-center gap-1"
                aria-label="Retry upload"
                ><i class="bi bi-arrow-clockwise" aria-hidden="true"></i> Retry</button
              >
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  @keyframes slideUp {
    from {
      transform: translateY(100px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
  .animate-slideUp {
    animation: slideUp 0.3s ease-out;
  }

  @keyframes scaleIn {
    from {
      transform: scale(0);
    }
    to {
      transform: scale(1);
    }
  }
  .animate-scaleIn {
    animation: scaleIn 0.3s ease-out;
  }

  @keyframes shake {
    0%,
    100% {
      transform: translateX(0);
    }
    10%,
    30%,
    50%,
    70%,
    90% {
      transform: translateX(-5px);
    }
    20%,
    40%,
    60%,
    80% {
      transform: translateX(5px);
    }
  }
  .animate-shake {
    animation: shake 0.5s;
  }
</style>
