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
  <div class="upload-manager-enhanced" class:minimized={isMinimized}>
    <!-- Header -->
    <div class="upload-header">
      <div class="header-left">
        <i class="bi bi-cloud-upload-fill text-primary" aria-hidden="true"></i>
        <span class="upload-title">
          {#if stats.uploading > 0}
            {$t("uploads.uploading")} {stats.uploading} {$t("files")}...
          {:else if stats.completed === stats.total}
            {$t("uploads.allComplete")}
          {:else}
            {$t("uploads.title")} ({stats.total})
          {/if}
        </span>

        {#if stats.uploading > 0}
          <span class="progress-badge"
            >{Math.round(stats.overallProgress)}%</span
          >
          <span class="speed-badge">{formatSpeed(stats.avgSpeed)}</span>
        {/if}
      </div>

      <div class="header-actions">
        <!-- Settings -->
        <button
          class="btn-icon"
          onclick={() => (showSettings = true)}
          title={$t("uploads.settings")}
        >
          <i class="bi bi-gear" aria-hidden="true"></i>
        </button>

        <!-- Pause/Resume All -->
        {#if stats.uploading > 0 || stats.paused > 0}
          <button
            class="btn-icon"
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
          class="btn-icon"
          onclick={() => (isMinimized = !isMinimized)}
          ><i
            class="bi bi-{isMinimized ? 'chevron-up' : 'chevron-down'}"
            aria-hidden="true"
          ></i></button
        >

        <!-- Close (only when no active uploads) -->
        {#if stats.uploading === 0 && stats.queued === 0}
          <button class="btn-icon" aria-label="Close" onclick={clearAll}
            ><i class="bi bi-x" aria-hidden="true"></i></button
          >
        {/if}
      </div>
    </div>

    {#if !isMinimized}
      <!-- Overall Progress Bar -->
      {#if stats.uploading > 0 || stats.queued > 0}
        <div class="overall-progress">
          <div class="progress-info">
            <span
              >{formatBytes(stats.uploadedSize)} / {formatBytes(
                stats.totalSize
              )}</span
            >
            <span class="text-muted">
              {#if stats.avgSpeed > 0}
                ETA: {estimatedTimeRemaining()}
              {/if}
            </span>
          </div>
          <div class="progress-bar">
            <div
              class="progress-fill"
              style="width: {stats.overallProgress}%"
            ></div>
          </div>
        </div>
      {/if}

      <!-- Upload Settings Modal -->
      {#if showSettings}
        <div
          class="modal-overlay"
          onclick={() => (showSettings = false)}
          onkeydown={(e) => e.key === "Escape" && (showSettings = false)}
          role="button"
          tabindex="0"
        >
          <div
            class="modal-content"
            onclick={(e) => e.stopPropagation()}
            onkeydown={(e) => e.stopPropagation()}
            role="button"
            tabindex="0"
          >
            <div class="modal-header">
              <h3>{$t("uploads.settings")}</h3>
              <button
                aria-label="Close"
                class="btn-icon"
                onclick={() => (showSettings = false)}
              >
                <i class="bi bi-x-lg" aria-hidden="true"></i>
              </button>
            </div>

            <div class="modal-body">
              <!-- Speed Limit -->
              <div class="form-group">
                <label for="speed-limit-input">
                  <i class="bi bi-speedometer2" aria-hidden="true"></i>
                  {$t("uploads.speedLimit")}
                </label>
                <div class="input-group">
                  <input
                    id="speed-limit-input"
                    type="number"
                    bind:value={settingsForm.speedLimit}
                    min="0"
                    step="100"
                    placeholder="0 = {$t('uploads.unlimited')}"
                  />
                  <span class="input-suffix">KB/s</span>
                </div>
                <small class="form-hint">
                  {$t("uploads.speedLimitHint")}
                </small>
              </div>

              <!-- Duplicate Handling -->
              <div class="form-group">
                <label for="duplicate-handling-select">
                  <i class="bi bi-files" aria-hidden="true"></i>
                  {$t("uploads.duplicateHandling")}
                </label>
                <select
                  id="duplicate-handling-select"
                  bind:value={settingsForm.duplicateAction}
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
                <small class="form-hint">
                  {$t("uploads.duplicateHandlingHint")}
                </small>
              </div>

              <!-- Skip Duplicates Checkbox -->
              <div class="form-group">
                <label class="checkbox-label">
                  <input
                    type="checkbox"
                    bind:checked={settingsForm.skipDuplicates}
                  />
                  {$t("uploads.skipDuplicates")}
                </label>
                <small class="form-hint">
                  {$t("uploads.skipDuplicatesHint")}
                </small>
              </div>
            </div>

            <div class="modal-footer">
              <button
                class="btn-secondary"
                onclick={() => (showSettings = false)}
              >
                {$t("cancel")}
              </button>
              <button class="btn-primary" onclick={saveSettings}>
                {$t("save")}
              </button>
            </div>
          </div>
        </div>
      {/if}
      <!-- Upload List -->
      <div class="upload-list">
        {#each $uploadQueue as upload (upload.id)}
          <div
            class="upload-item"
            class:completed={upload.status === "complete"}
            class:error={upload.status === "error"}
          >
            <!-- Icon -->
            <div class="upload-icon">
              <i class="bi {getStatusIcon(upload)} {getStatusClass(upload)}"
              ></i>
            </div>

            <!-- Details -->
            <div class="upload-details">
              <div class="file-name" title={upload.name}>
                {upload.name}
              </div>

              <div class="upload-meta">
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
                  <span class="text-warning">
                    {$t("uploads.paused")} • {upload.progress}% ({formatBytes(
                      upload.uploadedBytes
                    )})
                  </span>
                {:else if upload.status === "queued"}
                  <span class="text-muted">
                    {$t("uploads.queued")} • {formatBytes(upload.size)}
                  </span>
                {:else if upload.status === "complete"}
                  <span class="text-success">
                    {$t("uploads.complete")} • {formatBytes(upload.size)}
                  </span>
                {:else if upload.status === "error"}
                  <span class="text-error">
                    {upload.error || $t("uploads.failed")}
                  </span>
                {/if}
              </div>

              <!-- Progress Bar -->
              {#if upload.status === "uploading" || upload.status === "paused"}
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
                  onclick={() => pauseUpload(upload.id)}
                  title={$t("uploads.pause")}
                >
                  <i class="bi bi-pause-fill" aria-hidden="true"></i>
                </button>
                <button
                  class="btn-icon-sm"
                  onclick={() => cancelUpload(upload.id)}
                  title={$t("cancel")}
                  aria-label="Cancel"
                >
                  <i class="bi bi-x-lg" aria-hidden="true"></i>
                </button>
              {:else if upload.status === "paused"}
                <button
                  class="btn-icon-sm"
                  onclick={() => resumeUpload(upload.id)}
                  title={$t("uploads.resume")}
                >
                  <i class="bi bi-play-fill" aria-hidden="true"></i>
                </button>
                <button
                  class="btn-icon-sm"
                  onclick={() => cancelUpload(upload.id)}
                  title={$t("cancel")}
                  aria-label="Cancel"
                >
                  <i class="bi bi-x-lg" aria-hidden="true"></i>
                </button>
              {:else if upload.status === "error"}
                <button
                  class="btn-icon-sm"
                  onclick={() => retryUpload(upload.id)}
                  title={$t("retry")}
                >
                  <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
                </button>
                <button
                  class="btn-icon-sm"
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
        <div class="upload-footer">
          <div class="footer-stats">
            <span>
              {stats.completed} / {stats.total}
              {$t("uploads.filesCompleted")}
            </span>
            {#if stats.failed > 0}
              <span class="text-error">• {stats.failed} {$t("failed")}</span>
            {/if}
            {#if stats.paused > 0}
              <span class="text-warning"
                >• {stats.paused} {$t("uploads.paused")}</span
              >
            {/if}
          </div>

          <div class="footer-actions">
            {#if stats.completed > 0 && stats.uploading === 0}
              <button class="btn-text" onclick={clearCompleted}>
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
  .upload-manager-enhanced {
    position: fixed;
    bottom: 1.5rem;
    right: 1.5rem;
    width: 450px;
    max-height: 650px;
    background-color: rgb(30, 41, 59);
    border: 2px solid rgba(148, 163, 184, 0.3);
    border-radius: 0.75rem;
    box-shadow:
      0 20px 60px -15px rgba(0, 0, 0, 0.5),
      0 8px 16px -4px rgba(0, 0, 0, 0.3);
    z-index: 1000;
    display: flex;
    flex-direction: column;
    animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .upload-manager-enhanced.minimized {
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
    border-bottom: 1px solid rgba(148, 163, 184, 0.2);
    background-color: rgb(51, 65, 85);
    border-radius: 0.75rem 0.75rem 0 0;
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
  }

  .upload-title {
    color: rgb(226, 232, 240);
  }

  .progress-badge,
  .speed-badge {
    background: rgba(59, 130, 246, 0.15);
    color: rgb(96, 165, 250);
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
    color: rgb(148, 163, 184);
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-icon:hover {
    background: rgba(148, 163, 184, 0.1);
    color: rgb(226, 232, 240);
  }

  .overall-progress {
    padding: 1rem 1.25rem 0.75rem;
    background-color: rgb(30, 41, 59);
  }

  .progress-info {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.5rem;
    font-size: 0.85rem;
    color: rgb(203, 213, 225);
  }

  .progress-bar {
    height: 0.5rem;
    background: rgba(148, 163, 184, 0.2);
    border-radius: 1rem;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, rgb(59, 130, 246), rgb(139, 92, 246));
    border-radius: 1rem;
    transition: width 0.3s ease;
  }

  .upload-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    max-height: 480px;
    background-color: rgb(30, 41, 59);
  }

  .upload-item {
    display: flex;
    gap: 0.875rem;
    padding: 0.875rem;
    border-radius: 0.5rem;
    margin-bottom: 0.375rem;
    background-color: rgb(51, 65, 85);
    transition: all 0.15s;
  }

  .upload-item:hover {
    background-color: rgb(71, 85, 105);
  }

  .upload-item.completed {
    opacity: 0.7;
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
    color: rgb(226, 232, 240);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 0.375rem;
  }

  .upload-meta {
    font-size: 0.8rem;
    color: rgb(148, 163, 184);
    margin-bottom: 0.5rem;
  }

  .file-progress {
    height: 0.25rem;
    background: rgba(148, 163, 184, 0.2);
    border-radius: 1rem;
    overflow: hidden;
  }

  .file-progress-fill {
    height: 100%;
    background: linear-gradient(90deg, rgb(59, 130, 246), rgb(139, 92, 246));
    border-radius: 1rem;
    transition: width 0.2s ease;
  }

  .upload-actions {
    display: flex;
    gap: 0.25rem;
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
    color: rgb(148, 163, 184);
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.15s;
    font-size: 0.875rem;
  }

  .btn-icon-sm:hover {
    background: rgba(148, 163, 184, 0.1);
    color: rgb(226, 232, 240);
  }

  .upload-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.25rem;
    border-top: 1px solid rgba(148, 163, 184, 0.2);
    background-color: rgb(51, 65, 85);
    border-radius: 0 0 0.75rem 0.75rem;
  }

  .footer-stats {
    font-size: 0.85rem;
    color: rgb(148, 163, 184);
    display: flex;
    gap: 0.5rem;
  }

  .footer-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-text {
    background: none;
    border: none;
    color: rgb(96, 165, 250);
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

  .text-success {
    color: rgb(34, 197, 94);
  }
  .text-error {
    color: rgb(239, 68, 68);
  }
  .text-warning {
    color: rgb(251, 191, 36);
  }
  .text-primary {
    color: rgb(96, 165, 250);
  }
  .text-muted {
    color: rgb(148, 163, 184);
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
  }

  .modal-content {
    background: rgb(30, 41, 59);
    border-radius: 0.75rem;
    width: 90%;
    max-width: 500px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
    border-bottom: 1px solid rgb(51, 65, 85);
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .modal-body {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1.5rem;
    border-top: 1px solid rgb(51, 65, 85);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-weight: 500;
    color: rgb(226, 232, 240);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .form-group input[type="number"],
  .form-group select {
    background: rgb(15, 23, 42);
    border: 1px solid rgb(51, 65, 85);
    border-radius: 0.375rem;
    padding: 0.5rem 0.75rem;
    color: rgb(226, 232, 240);
    font-size: 0.95rem;
  }

  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: rgb(96, 165, 250);
    box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.1);
  }

  .input-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .input-group input {
    flex: 1;
  }

  .input-suffix {
    color: rgb(148, 163, 184);
    font-size: 0.95rem;
  }

  .form-hint {
    color: rgb(148, 163, 184);
    font-size: 0.85rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .checkbox-label input[type="checkbox"] {
    width: 1.25rem;
    height: 1.25rem;
    cursor: pointer;
  }

  .btn-primary {
    background: rgb(59, 130, 246);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-primary:hover {
    background: rgb(37, 99, 235);
  }

  .btn-secondary {
    background: transparent;
    color: rgb(148, 163, 184);
    border: 1px solid rgb(51, 65, 85);
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-secondary:hover {
    background: rgb(51, 65, 85);
    color: rgb(226, 232, 240);
  }

  .upload-list::-webkit-scrollbar {
    width: 0.375rem;
  }

  .upload-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .upload-list::-webkit-scrollbar-thumb {
    background: rgba(148, 163, 184, 0.3);
    border-radius: 1rem;
  }

  @media (max-width: 640px) {
    .upload-manager-enhanced {
      width: calc(100vw - 2rem);
      right: 1rem;
      bottom: 1rem;
    }
  }
</style>
