<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { uploads = [], onClear, onRetry, onCancel, onFileClick } = $props();

  let isMinimized = $state(false);
  let isVisible = $state(true);

  // Auto-show when new uploads start
  $effect(() => {
    if (uploads.length > 0 && !isVisible) {
      isVisible = true;
      isMinimized = false;
    }
  });

  // Check if this is a batch/folder upload
  const isBatchUpload = $derived(() => {
    return uploads.length > 0 && uploads[0].batchId != null;
  });

  const batchInfo = $derived(() => {
    if (!isBatchUpload()) return null;
    const firstUpload = uploads[0];
    return {
      name: firstUpload.batchName,
      total: firstUpload.batchTotal,
      completed: completedUploads().length,
      active: activeUploads().length,
    };
  });

  // Calculate overall progress
  const overallProgress = $derived(() => {
    if (uploads.length === 0) return 0;
    const total = uploads.reduce((sum, u) => sum + u.progress, 0);
    return Math.round(total / uploads.length);
  });

  const activeUploads = $derived(() =>
    uploads.filter((u) => u.status === "uploading" || u.status === "retrying")
  );

  const completedUploads = $derived(() =>
    uploads.filter((u) => u.status === "complete")
  );

  const failedUploads = $derived(() =>
    uploads.filter((u) => u.status === "error")
  );

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 ** 2) return (bytes / 1024).toFixed(1) + " KB";
    if (bytes < 1024 ** 3) return (bytes / 1024 ** 2).toFixed(1) + " MB";
    return (bytes / 1024 ** 3).toFixed(1) + " GB";
  }

  function toggleMinimize() {
    isMinimized = !isMinimized;
  }

  function close() {
    isVisible = false;
    // Clear all completed uploads when closing
    if (onClear) {
      onClear("all");
    }
  }

  function clearCompleted() {
    if (onClear) {
      onClear("completed");
    }
  }

  function clearAll() {
    if (onClear) {
      onClear("all");
    }
  }

  function handleFileClick(upload) {
    if (upload.status === "complete" && onFileClick) {
      onFileClick(upload);
    }
  }
</script>

{#if isVisible && uploads.length > 0}
  <div
    class="upload-manager-responsive fixed bottom-6 right-6 w-[420px] max-h-[600px] bg-white dark:bg-slate-800 border-2 border-black/10 dark:border-slate-400/30 rounded-2xl shadow-2xl z-[1000] flex flex-col animate-slideIn {isMinimized
      ? 'max-h-[60px]'
      : ''}"
  >
    <!-- Header -->
    <div
      class="flex justify-between items-center px-5 py-4 border-b border-black/10 dark:border-slate-400/20 bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-700 dark:to-slate-800 rounded-t-2xl"
    >
      <div class="flex items-center gap-3 font-semibold text-[0.95rem]">
        <i
          class="bi bi-{isBatchUpload()
            ? 'folder-fill'
            : 'cloud-upload-fill'} text-xl text-blue-500"
          aria-hidden="true"
        ></i>
        <span class="text-slate-800 dark:text-slate-100">
          {#if isBatchUpload()}
            {#if activeUploads().length > 0}
              Uploading folder "{batchInfo().name}"
            {:else if completedUploads().length === batchInfo().total}
              Folder "{batchInfo().name}" uploaded
            {:else}
              Folder upload: {batchInfo().name}
            {/if}
          {:else if activeUploads().length > 0}
            Uploading {activeUploads().length}
            {activeUploads().length === 1 ? "file" : "files"}...
          {:else if completedUploads().length === uploads.length}
            Upload complete
          {:else}
            Uploads ({uploads.length})
          {/if}
        </span>
        {#if activeUploads().length > 0}
          {#if isBatchUpload()}
            <span
              class="bg-blue-500/15 text-blue-500 px-2 py-0.5 rounded-md text-xs font-semibold"
              >{batchInfo().completed}/{batchInfo().total}</span
            >
            <span
              class="bg-blue-500/15 text-blue-500 px-2 py-0.5 rounded-md text-xs font-semibold"
              >{overallProgress()}%</span
            >
          {:else}
            <span
              class="bg-blue-500/15 text-blue-500 px-2 py-0.5 rounded-md text-xs font-semibold"
              >{overallProgress()}%</span
            >
          {/if}
        {/if}
      </div>
      <div class="flex gap-1">
        <button
          class="w-8 h-8 flex items-center justify-center border-none bg-transparent text-slate-500 dark:text-slate-400 rounded-lg cursor-pointer transition-all hover:bg-black/5 dark:hover:bg-white/10 hover:text-slate-800 dark:hover:text-slate-100"
          onclick={toggleMinimize}
          title={isMinimized ? "Expand" : "Minimize"}
        >
          <i
            class="bi bi-{isMinimized ? 'chevron-up' : 'chevron-down'}"
            aria-hidden="true"
          ></i>
        </button>
        {#if activeUploads().length === 0}
          <button
            class="w-8 h-8 flex items-center justify-center border-none bg-transparent text-slate-500 dark:text-slate-400 rounded-lg cursor-pointer transition-all hover:bg-black/5 dark:hover:bg-white/10 hover:text-slate-800 dark:hover:text-slate-100"
            aria-label="Close"
            onclick={close}
            title="Close"
          >
            <i class="bi bi-x" aria-hidden="true"></i>
          </button>
        {/if}
      </div>
    </div>

    {#if !isMinimized}
      <!-- Overall Progress Bar -->
      {#if activeUploads().length > 0}
        <div class="px-5 pt-4 pb-2">
          <div
            class="h-1.5 bg-black/10 dark:bg-white/10 rounded-full overflow-hidden"
          >
            <div
              class="h-full bg-gradient-to-r from-blue-500 to-violet-500 rounded-full transition-all duration-300"
              style="width: {overallProgress()}%"
            ></div>
          </div>
        </div>
      {/if}

      <!-- Upload List -->
      <div
        class="flex-1 overflow-y-auto p-2 max-h-[440px] bg-white dark:bg-slate-800 scrollbar-modern"
      >
        {#each uploads as upload (upload.id)}
          <div
            class="flex gap-3.5 p-3.5 rounded-xl mb-1.5 transition-all bg-slate-50 dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 {upload.status ===
            'complete'
              ? 'opacity-70 cursor-pointer hover:bg-sky-100 dark:hover:bg-slate-600 hover:opacity-100'
              : ''} {upload.status === 'error' ? 'bg-red-500/10' : ''}"
            onclick={() => handleFileClick(upload)}
            onkeydown={(e) => e.key === "Enter" && handleFileClick(upload)}
            role="button"
            tabindex="0"
          >
            <!-- Icon -->
            <div class="text-2xl flex items-start pt-0.5">
              {#if upload.status === "complete"}
                <i
                  class="bi bi-check-circle-fill text-green-500"
                  aria-hidden="true"
                ></i>
              {:else if upload.status === "error"}
                <i class="bi bi-x-circle-fill text-red-500" aria-hidden="true"
                ></i>
              {:else if upload.status === "retrying"}
                <i
                  class="bi bi-arrow-clockwise animate-spin text-amber-500"
                  aria-hidden="true"
                ></i>
              {:else if upload.status === "uploading"}
                <i class="bi bi-cloud-arrow-up text-blue-500" aria-hidden="true"
                ></i>
              {:else}
                <i class="bi bi-dash-circle text-slate-400" aria-hidden="true"
                ></i>
              {/if}
            </div>

            <!-- Details -->
            <div class="flex-1 min-w-0">
              <div
                class="font-medium text-sm text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap mb-1.5"
                title={upload.name}
              >
                {upload.name}
              </div>
              <div class="text-xs text-slate-500 dark:text-slate-400 mb-2">
                {#if upload.status === "uploading"}
                  <span
                    >{upload.progress}% • {formatBytes(
                      (upload.progress / 100) * upload.size
                    )} / {formatBytes(upload.size)}</span
                  >
                {:else if upload.status === "retrying"}
                  <span class="text-amber-500"
                    >Retrying... (Attempt {upload.retries}/{3})</span
                  >
                {:else if upload.status === "complete"}
                  <span class="text-green-500"
                    >{formatBytes(upload.size)} • Complete</span
                  >
                {:else if upload.status === "error"}
                  <span class="text-red-500"
                    >{upload.error || "Upload failed"}</span
                  >
                {/if}
              </div>

              <!-- Progress Bar for active uploads -->
              {#if upload.status === "uploading"}
                <div
                  class="h-1 bg-black/10 dark:bg-white/10 rounded-full overflow-hidden"
                >
                  <div
                    class="h-full bg-gradient-to-r from-blue-500 to-violet-500 rounded-full transition-all duration-200"
                    style="width: {upload.progress}%"
                  ></div>
                </div>
              {/if}
            </div>

            <!-- Actions -->
            <div class="flex items-start pt-0.5">
              {#if upload.status === "uploading"}
                <button
                  class="w-7 h-7 flex items-center justify-center border-none bg-transparent text-slate-500 dark:text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-black/10 dark:hover:bg-white/10 hover:text-slate-800 dark:hover:text-slate-100"
                  onclick={(e) => {
                    e.stopPropagation();
                    onCancel?.(upload.id);
                  }}
                  title="Cancel"
                  aria-label="Cancel"
                >
                  <i class="bi bi-x-lg" aria-hidden="true"></i>
                </button>
              {:else if upload.status === "error"}
                <button
                  class="w-7 h-7 flex items-center justify-center border-none bg-transparent text-slate-500 dark:text-slate-400 rounded-md cursor-pointer transition-all text-sm hover:bg-black/10 dark:hover:bg-white/10 hover:text-slate-800 dark:hover:text-slate-100"
                  onclick={(e) => {
                    e.stopPropagation();
                    onRetry?.(upload.id);
                  }}
                  title="Retry"
                >
                  <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>

      <!-- Footer -->
      {#if uploads.length > 3}
        <div
          class="flex justify-between items-center px-5 py-3 border-t border-black/10 dark:border-slate-400/20 bg-slate-50 dark:bg-slate-700 rounded-b-2xl"
        >
          <span class="text-xs text-slate-500 dark:text-slate-400">
            {#if isBatchUpload()}
              {batchInfo().completed} of {batchInfo().total} files completed
              {#if failedUploads().length > 0}
                • {failedUploads().length} failed
              {/if}
            {:else}
              {completedUploads().length} completed
              {#if failedUploads().length > 0}
                • {failedUploads().length} failed
              {/if}
            {/if}
          </span>
          {#if activeUploads().length === 0}
            <button
              class="bg-transparent border-none text-blue-500 text-sm font-medium cursor-pointer px-2 py-1 rounded-md transition-all hover:bg-blue-500/10"
              onclick={clearAll}>Clear all</button
            >
          {/if}
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
