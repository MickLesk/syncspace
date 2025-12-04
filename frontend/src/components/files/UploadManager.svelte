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
  <div class="upload-manager" class:minimized={isMinimized}>
    <!-- Header -->
    <div class="upload-header">
      <div class="header-left">
        <i
          class="bi bi-{isBatchUpload() ? 'folder-fill' : 'cloud-upload-fill'}"
          aria-hidden="true"
        ></i>
        <span class="upload-title">
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
            <span class="progress-badge"
              >{batchInfo().completed}/{batchInfo().total}</span
            >
            <span class="progress-badge">{overallProgress()}%</span>
          {:else}
            <span class="progress-badge">{overallProgress()}%</span>
          {/if}
        {/if}
      </div>
      <div class="header-actions">
        <button
          class="btn-icon"
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
            class="btn-icon"
            aria-label="Close"
            onclick={close}
            title="Close"><i class="bi bi-x" aria-hidden="true"></i></button
          >
        {/if}
      </div>
    </div>

    {#if !isMinimized}
      <!-- Overall Progress Bar -->
      {#if activeUploads().length > 0}
        <div class="overall-progress">
          <div class="progress-bar">
            <div
              class="progress-fill"
              style="width: {overallProgress()}%"
            ></div>
          </div>
        </div>
      {/if}

      <!-- Upload List -->
      <div class="upload-list">
        {#each uploads as upload (upload.id)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="upload-item"
            class:completed={upload.status === "complete"}
            class:error={upload.status === "error"}
            class:clickable={upload.status === "complete"}
            onclick={() => handleFileClick(upload)}
            role="button"
            tabindex="0"
          >
            <!-- Icon -->
            <div class="upload-icon">
              {#if upload.status === "complete"}
                <i
                  class="bi bi-check-circle-fill text-success"
                  aria-hidden="true"
                ></i>
              {:else if upload.status === "error"}
                <i class="bi bi-x-circle-fill text-error" aria-hidden="true"
                ></i>
              {:else if upload.status === "retrying"}
                <i
                  class="bi bi-arrow-clockwise spinning text-warning"
                  aria-hidden="true"
                ></i>
              {:else if upload.status === "uploading"}
                <i class="bi bi-cloud-arrow-up text-primary" aria-hidden="true"
                ></i>
              {:else}
                <i class="bi bi-dash-circle text-muted" aria-hidden="true"></i>
              {/if}
            </div>

            <!-- Details -->
            <div class="upload-details">
              <div class="file-name" title={upload.name}>
                {upload.name}
              </div>
              <div class="upload-meta">
                {#if upload.status === "uploading"}
                  <span
                    >{upload.progress}% • {formatBytes(
                      (upload.progress / 100) * upload.size
                    )} / {formatBytes(upload.size)}</span
                  >
                {:else if upload.status === "retrying"}
                  <span class="text-warning"
                    >Retrying... (Attempt {upload.retries}/{3})</span
                  >
                {:else if upload.status === "complete"}
                  <span class="text-success"
                    >{formatBytes(upload.size)} • Complete</span
                  >
                {:else if upload.status === "error"}
                  <span class="text-error"
                    >{upload.error || "Upload failed"}</span
                  >
                {/if}
              </div>

              <!-- Progress Bar for active uploads -->
              {#if upload.status === "uploading"}
                <div class="file-progress">
                  <div
                    class="file-progress-fill"
                    style="width: {upload.progress}%"
                  ></div>
                </div>
              {/if}
            </div>

            <!-- Actions -->
            <div class="upload-actions">
              {#if upload.status === "uploading"}
                <button
                  class="btn-icon-sm"
                  onclick={() => onCancel?.(upload.id)}
                  title="Cancel"
                  aria-label="Cancel"
                >
                  <i class="bi bi-x-lg" aria-hidden="true"></i>
                </button>
              {:else if upload.status === "error"}
                <button
                  class="btn-icon-sm"
                  onclick={() => onRetry?.(upload.id)}
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
        <div class="upload-footer">
          <span class="footer-stats">
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
            <button class="btn-text" onclick={clearAll}>Clear all</button>
          {/if}
        </div>
      {/if}
    {/if}
  </div>
{/if}

<style>
  .upload-manager {
    position: fixed;
    bottom: 1.5rem;
    right: 1.5rem;
    width: 420px;
    max-height: 600px;
    background-color: white;
    border: 2px solid rgba(0, 0, 0, 0.1);
    border-radius: 1rem;
    box-shadow:
      0 20px 60px -15px rgba(0, 0, 0, 0.2),
      0 8px 16px -4px rgba(0, 0, 0, 0.1);
    z-index: 1000;
    display: flex;
    flex-direction: column;
    animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  :global(.dark) .upload-manager {
    background-color: rgb(30, 41, 59);
    border-color: rgba(148, 163, 184, 0.3);
    box-shadow:
      0 20px 60px -15px rgba(0, 0, 0, 0.5),
      0 8px 16px -4px rgba(0, 0, 0, 0.3);
  }

  .upload-manager.minimized {
    max-height: 60px;
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .upload-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
    border-radius: 1rem 1rem 0 0;
  }

  :global(.dark) .upload-header {
    background: linear-gradient(
      135deg,
      rgb(51, 65, 85) 0%,
      rgb(30, 41, 59) 100%
    );
    border-bottom-color: rgba(148, 163, 184, 0.2);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-weight: 600;
    font-size: 0.95rem;
  }

  .header-left i {
    font-size: 1.25rem;
    color: #3b82f6;
  }

  .upload-title {
    color: #1e293b;
  }

  :global(.dark) .upload-title {
    color: #f1f5f9;
  }

  .progress-badge {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
    padding: 0.125rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .header-actions {
    display: flex;
    gap: 0.25rem;
  }

  .btn-icon {
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: #64748b;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-icon:hover {
    background: rgba(0, 0, 0, 0.05);
    color: #1e293b;
  }

  :global(.dark) .btn-icon {
    color: #94a3b8;
  }

  :global(.dark) .btn-icon:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #f1f5f9;
  }

  /* Overall Progress */
  .overall-progress {
    padding: 1rem 1.25rem 0.5rem;
  }

  .progress-bar {
    height: 0.375rem;
    background: rgba(0, 0, 0, 0.1);
    border-radius: 1rem;
    overflow: hidden;
  }

  :global(.dark) .progress-bar {
    background: rgba(255, 255, 255, 0.1);
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    border-radius: 1rem;
    transition: width 0.3s ease;
  }

  /* Upload List */
  .upload-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    max-height: 440px;
    background-color: white;
  }

  :global(.dark) .upload-list {
    background-color: rgb(30, 41, 59);
  }

  .upload-item {
    display: flex;
    gap: 0.875rem;
    padding: 0.875rem;
    border-radius: 0.75rem;
    margin-bottom: 0.375rem;
    transition: all 0.15s;
    background-color: #f8fafc;
  }

  :global(.dark) .upload-item {
    background-color: rgb(30, 41, 59);
  }

  .upload-item:hover {
    background-color: #f1f5f9;
  }

  :global(.dark) .upload-item:hover {
    background-color: rgb(51, 65, 85);
  }

  .upload-item.completed {
    opacity: 0.7;
  }

  .upload-item.clickable {
    cursor: pointer;
  }

  .upload-item.clickable:hover {
    background-color: #e0f2fe;
    opacity: 1;
  }

  :global(.dark) .upload-item.clickable:hover {
    background-color: rgb(71, 85, 105);
  }

  .upload-item.error {
    background: rgba(239, 68, 68, 0.1);
  }

  .upload-icon {
    font-size: 1.5rem;
    display: flex;
    align-items: flex-start;
    padding-top: 0.125rem;
  }

  .upload-details {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-weight: 500;
    font-size: 0.9rem;
    color: #1e293b;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 0.375rem;
  }

  :global(.dark) .file-name {
    color: #f1f5f9;
  }

  .upload-meta {
    font-size: 0.8rem;
    color: #64748b;
    margin-bottom: 0.5rem;
  }

  :global(.dark) .upload-meta {
    color: #94a3b8;
  }

  .file-progress {
    height: 0.25rem;
    background: rgba(0, 0, 0, 0.1);
    border-radius: 1rem;
    overflow: hidden;
  }

  :global(.dark) .file-progress {
    background: rgba(255, 255, 255, 0.1);
  }

  .file-progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    border-radius: 1rem;
    transition: width 0.2s ease;
  }

  .upload-actions {
    display: flex;
    align-items: flex-start;
    padding-top: 0.125rem;
  }

  .btn-icon-sm {
    width: 1.75rem;
    height: 1.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: #64748b;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.15s;
    font-size: 0.875rem;
  }

  :global(.dark) .btn-icon-sm {
    color: #94a3b8;
  }

  .btn-icon-sm:hover {
    background: rgba(0, 0, 0, 0.1);
    color: #1e293b;
  }

  :global(.dark) .btn-icon-sm:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #f1f5f9;
  }

  /* Footer */
  .upload-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.25rem;
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    background: #f8fafc;
    border-radius: 0 0 1rem 1rem;
  }

  :global(.dark) .upload-footer {
    background-color: rgb(51, 65, 85);
    border-top-color: rgba(148, 163, 184, 0.2);
  }

  .footer-stats {
    font-size: 0.8rem;
    color: #64748b;
  }

  :global(.dark) .footer-stats {
    color: #94a3b8;
  }

  .btn-text {
    background: none;
    border: none;
    color: #3b82f6;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    transition: all 0.15s;
  }

  .btn-text:hover {
    background: rgba(59, 130, 246, 0.1);
  }

  /* Status Colors */
  .text-success {
    color: #22c55e;
  }
  .text-error {
    color: #ef4444;
  }
  .text-warning {
    color: #f59e0b;
  }
  .text-primary {
    color: #3b82f6;
  }
  .text-muted {
    color: #94a3b8;
  }

  /* Animations */
  .spinning {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  /* Scrollbar */
  .upload-list::-webkit-scrollbar {
    width: 0.375rem;
  }

  .upload-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .upload-list::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.15);
    border-radius: 1rem;
  }

  :global(.dark) .upload-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
  }

  .upload-list::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.25);
  }

  :global(.dark) .upload-list::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }

  /* Responsive */
  @media (max-width: 640px) {
    .upload-manager {
      width: calc(100vw - 2rem);
      right: 1rem;
      bottom: 1rem;
    }
  }
</style>
