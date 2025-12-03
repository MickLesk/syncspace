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
        return "text-blue-500";
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
    class="upload-queue-container"
    class:compact
    transition:fly={{ y: 20, duration: 300 }}
  >
    <!-- Header -->
    <div class="upload-queue-header">
      <div class="flex items-center gap-3">
        <div class="header-icon">
          <i class="bi bi-cloud-upload" aria-hidden="true"></i>
        </div>
        <div>
          <h4 class="header-title">Upload Queue</h4>
          <p class="header-subtitle">
            {$uploadStats.completedFiles}/{$uploadStats.totalFiles} completed • {formatBytes(
              $uploadStats.uploadedBytes
            )}/{formatBytes($uploadStats.totalBytes)}
          </p>
        </div>
      </div>

      <div class="header-actions">
        <button
          class="action-btn"
          onclick={handleClearCompleted}
          disabled={$uploadQueue.filter(
            (u) => u.status === UPLOAD_STATUS.COMPLETED
          ).length === 0}
          title="Clear completed"
        >
          <i class="bi bi-check-circle" aria-hidden="true"></i>
        </button>
        <button
          class="action-btn"
          onclick={handleClearFailed}
          disabled={$uploadQueue.filter(
            (u) => u.status === UPLOAD_STATUS.FAILED
          ).length === 0}
          title="Clear failed"
        >
          <i class="bi bi-x-circle" aria-hidden="true"></i>
        </button>
      </div>
    </div>

    <!-- Upload items -->
    <div class="upload-queue-list">
      {#each displayQueue as upload (upload.id)}
        <div
          class="upload-item"
          class:uploading={upload.status === UPLOAD_STATUS.UPLOADING}
          class:completed={upload.status === UPLOAD_STATUS.COMPLETED}
          class:failed={upload.status === UPLOAD_STATUS.FAILED}
          transition:slide={{ duration: 200 }}
        >
          <!-- Icon -->
          <div class="upload-icon {getStatusColor(upload.status)}">
            <i class="bi {getStatusIcon(upload.status)}"></i>
          </div>

          <!-- Details -->
          <div class="upload-details">
            <div class="upload-name">{upload.fileName}</div>
            <div class="upload-info">
              <span class="info-item">
                <i class="bi bi-hdd" aria-hidden="true"></i>
                {formatBytes(upload.size)}
              </span>
              {#if upload.isChunked}
                <span class="info-item">
                  <i class="bi bi-puzzle" aria-hidden="true"></i>
                  {upload.currentChunk}/{upload.totalChunks} chunks
                </span>
              {/if}
              {#if upload.retries > 0}
                <span class="info-item text-orange-500">
                  <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
                  Retry {upload.retries}
                </span>
              {/if}
              {#if upload.error}
                <span class="info-item text-red-500" title={upload.error}>
                  <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
                  {upload.error}
                </span>
              {/if}
            </div>

            <!-- Progress bar -->
            {#if upload.status === UPLOAD_STATUS.UPLOADING || upload.status === UPLOAD_STATUS.PAUSED}
              <div class="progress-bar-container">
                <div
                  class="progress-bar {upload.status === UPLOAD_STATUS.PAUSED
                    ? 'paused'
                    : ''}"
                  style="width: {upload.progress}%"
                ></div>
                <span class="progress-text">{upload.progress}%</span>
              </div>
            {/if}
          </div>

          <!-- Actions -->
          <div class="upload-actions">
            {#if upload.status === UPLOAD_STATUS.UPLOADING || upload.status === UPLOAD_STATUS.PAUSED}
              <button
                class="icon-btn"
                onclick={() => handlePauseResume(upload)}
                title={upload.status === UPLOAD_STATUS.UPLOADING
                  ? "Pause"
                  : "Resume"}
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
                class="icon-btn"
                onclick={() => handleRetry(upload.id)}
                title="Retry"
              >
                <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
              </button>
            {/if}

            {#if upload.status !== UPLOAD_STATUS.COMPLETED}
              <button
                class="icon-btn delete"
                onclick={() => handleCancel(upload.id)}
                title="Cancel"
               aria-label="Cancel">
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
  .upload-queue-container {
    position: fixed;
    bottom: 20px;
    right: 20px;
    width: 420px;
    max-height: 500px;
    background: white;
    border-radius: 16px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
    overflow: hidden;
    z-index: 1000;
    border: 2px solid rgba(0, 0, 0, 0.05);
  }

  :global(.dark) .upload-queue-container {
    background: rgb(31 41 55);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .upload-queue-container.compact {
    width: 340px;
    max-height: 400px;
  }

  .upload-queue-header {
    padding: 16px 20px;
    border-bottom: 2px solid rgba(0, 0, 0, 0.08);
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.05),
      rgba(139, 92, 246, 0.05)
    );
  }

  :global(.dark) .upload-queue-header {
    border-bottom-color: rgba(255, 255, 255, 0.08);
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.1),
      rgba(139, 92, 246, 0.1)
    );
  }

  .header-icon {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 18px;
  }

  .header-title {
    font-size: 15px;
    font-weight: 700;
    color: rgb(17 24 39);
    margin: 0;
  }

  :global(.dark) .header-title {
    color: rgb(243 244 246);
  }

  .header-subtitle {
    font-size: 11px;
    color: rgb(107 114 128);
    margin: 2px 0 0 0;
  }

  :global(.dark) .header-subtitle {
    color: rgb(156 163 175);
  }

  .header-actions {
    display: flex;
    gap: 8px;
  }

  .action-btn {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    border: none;
    background: rgba(0, 0, 0, 0.05);
    color: rgb(107 114 128);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .action-btn:hover:not(:disabled) {
    background: rgba(0, 0, 0, 0.1);
    color: rgb(17 24 39);
  }

  .action-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  :global(.dark) .action-btn {
    background: rgba(255, 255, 255, 0.05);
    color: rgb(156 163 175);
  }

  :global(.dark) .action-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
    color: rgb(243 244 246);
  }

  .upload-queue-list {
    max-height: 400px;
    overflow-y: auto;
    padding: 12px;
  }

  .upload-queue-list::-webkit-scrollbar {
    width: 6px;
  }

  .upload-queue-list::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 3px;
  }

  .upload-queue-list::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  :global(.dark) .upload-queue-list::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
  }

  :global(.dark) .upload-queue-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
  }

  .upload-item {
    display: flex;
    gap: 12px;
    padding: 12px;
    border-radius: 12px;
    background: rgba(0, 0, 0, 0.02);
    margin-bottom: 8px;
    transition: all 0.2s;
  }

  :global(.dark) .upload-item {
    background: rgba(255, 255, 255, 0.03);
  }

  .upload-item.uploading {
    background: rgba(59, 130, 246, 0.05);
    border: 1px solid rgba(59, 130, 246, 0.15);
  }

  .upload-item.completed {
    background: rgba(34, 197, 94, 0.05);
    border: 1px solid rgba(34, 197, 94, 0.15);
  }

  .upload-item.failed {
    background: rgba(239, 68, 68, 0.05);
    border: 1px solid rgba(239, 68, 68, 0.15);
  }

  .upload-icon {
    font-size: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .upload-details {
    flex: 1;
    min-width: 0;
  }

  .upload-name {
    font-size: 13px;
    font-weight: 600;
    color: rgb(17 24 39);
    margin-bottom: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark) .upload-name {
    color: rgb(243 244 246);
  }

  .upload-info {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 6px;
  }

  .info-item {
    font-size: 10px;
    color: rgb(107 114 128);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  :global(.dark) .info-item {
    color: rgb(156 163 175);
  }

  .progress-bar-container {
    position: relative;
    height: 6px;
    background: rgba(0, 0, 0, 0.08);
    border-radius: 3px;
    overflow: hidden;
  }

  :global(.dark) .progress-bar-container {
    background: rgba(255, 255, 255, 0.08);
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #6366f1, #8b5cf6);
    border-radius: 3px;
    transition: width 0.3s ease;
    position: relative;
  }

  .progress-bar.paused {
    background: linear-gradient(90deg, #f59e0b, #f97316);
  }

  .progress-text {
    position: absolute;
    right: 4px;
    top: -18px;
    font-size: 9px;
    font-weight: 600;
    color: rgb(107 114 128);
  }

  .upload-actions {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .icon-btn {
    width: 28px;
    height: 28px;
    border-radius: 7px;
    border: none;
    background: rgba(0, 0, 0, 0.06);
    color: rgb(107 114 128);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    font-size: 13px;
  }

  .icon-btn:hover {
    background: rgba(0, 0, 0, 0.12);
    color: rgb(17 24 39);
  }

  .icon-btn.delete:hover {
    background: rgba(239, 68, 68, 0.1);
    color: rgb(239, 68, 68);
  }

  :global(.dark) .icon-btn {
    background: rgba(255, 255, 255, 0.06);
    color: rgb(156 163 175);
  }

  :global(.dark) .icon-btn:hover {
    background: rgba(255, 255, 255, 0.12);
    color: rgb(243 244 246);
  }

  :global(.dark) .icon-btn.delete:hover {
    background: rgba(239, 68, 68, 0.15);
    color: rgb(239, 68, 68);
  }

  /* Mobile responsive */
  @media (max-width: 768px) {
    .upload-queue-container {
      width: calc(100vw - 40px);
      max-height: 300px;
      bottom: 10px;
      right: 10px;
    }

    .header-title {
      font-size: 13px;
    }

    .header-subtitle {
      font-size: 10px;
    }
  }
</style>
