<script>
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";

  /** @type {string | null} jobId - UUID of batch operation */
  let { jobId = $bindable(null), onClose = () => {} } = $props();

  /** @type {Object | null} operation status */
  let status = $state(null);
  /** @type {string | null} error message */
  let error = $state(null);
  /** @type {boolean} loading state */
  let loading = $state(false);
  /** @type {number | null} polling interval ID */
  let pollInterval = null;

  // Computed values
  const isComplete = $derived(status?.status === "completed");
  const isFailed = $derived(status?.status === "failed");
  const isCancelled = $derived(status?.status === "cancelled");
  const isRunning = $derived(
    status?.status === "running" || status?.status === "pending"
  );
  const canCancel = $derived(isRunning && jobId);

  // Format file size
  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${sizes[i]}`;
  }

  // Format duration
  function formatDuration(start, end) {
    if (!start) return "--";
    const startTime = new Date(start).getTime();
    const endTime = end ? new Date(end).getTime() : Date.now();
    const duration = endTime - startTime;
    const seconds = Math.floor(duration / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);

    if (hours > 0) return `${hours}h ${minutes % 60}m`;
    if (minutes > 0) return `${minutes}m ${seconds % 60}s`;
    return `${seconds}s`;
  }

  // Fetch operation status
  async function fetchStatus() {
    if (!jobId) return;

    try {
      loading = true;
      error = null;
      const data = await api.batch.getStatus(jobId);
      status = data;

      // Stop polling if operation is done
      if (isComplete || isFailed || isCancelled) {
        stopPolling();
      }
    } catch (err) {
      error = err.message || "Failed to fetch operation status";
      stopPolling();
    } finally {
      loading = false;
    }
  }

  // Start polling for status updates
  function startPolling() {
    if (pollInterval) return;

    fetchStatus(); // Initial fetch
    pollInterval = setInterval(fetchStatus, 1000); // Poll every second
  }

  // Stop polling
  function stopPolling() {
    if (pollInterval) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
  }

  // Cancel operation
  async function handleCancel() {
    if (!jobId || !canCancel) return;

    try {
      loading = true;
      await api.batch.cancel(jobId);
      await fetchStatus(); // Refresh status
    } catch (err) {
      error = err.message || "Failed to cancel operation";
    } finally {
      loading = false;
    }
  }

  // Close dialog
  function handleClose() {
    stopPolling();
    onClose();
  }

  // Start polling on mount
  onMount(() => {
    if (jobId) {
      startPolling();
    }
  });

  // Cleanup on destroy
  onDestroy(() => {
    stopPolling();
  });
</script>

{#if jobId}
  <dialog class="modal modal-open" aria-labelledby="batch-progress-title">
    <div class="modal-box max-w-2xl">
      <h3
        id="batch-progress-title"
        class="text-lg font-bold mb-4 flex items-center gap-2"
      >
        <svg
          class="w-5 h-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
          />
        </svg>
        Batch Operation Progress
      </h3>

      {#if error}
        <div role="alert" class="alert alert-error mb-4">
          <svg
            class="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <span>{error}</span>
        </div>
      {/if}

      {#if status}
        <div class="space-y-4">
          <!-- Status Badge -->
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <span class="text-sm text-base-content/70">Status:</span>
              {#if isComplete}
                <span class="badge badge-success gap-1">
                  <svg
                    class="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M5 13l4 4L19 7"
                    />
                  </svg>
                  Completed
                </span>
              {:else if isFailed}
                <span class="badge badge-error gap-1">
                  <svg
                    class="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                  Failed
                </span>
              {:else if isCancelled}
                <span class="badge badge-warning gap-1">
                  <svg
                    class="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                  Cancelled
                </span>
              {:else}
                <span class="badge badge-info gap-1">
                  <svg
                    class="w-4 h-4 animate-spin"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                    />
                  </svg>
                  {status.status === "pending" ? "Pending" : "Running"}
                </span>
              {/if}
            </div>

            <span class="text-sm text-base-content/70">
              Duration: {formatDuration(status.started_at, status.completed_at)}
            </span>
          </div>

          <!-- Progress Bar -->
          <div class="space-y-2">
            <div class="flex justify-between text-sm">
              <span class="text-base-content/70">Progress</span>
              <span class="font-medium"
                >{status.progress?.toFixed(1) || 0}%</span
              >
            </div>
            <progress
              class="progress {isComplete
                ? 'progress-success'
                : isFailed
                  ? 'progress-error'
                  : 'progress-primary'} w-full"
              value={status.progress || 0}
              max="100"
              aria-label="Operation progress"
            ></progress>
          </div>

          <!-- Items Progress -->
          <div class="grid grid-cols-2 gap-4 p-4 bg-base-200 rounded-lg">
            <div>
              <div class="text-xs text-base-content/70 mb-1">
                Processed Items
              </div>
              <div class="text-2xl font-bold">
                {status.processed_items || 0}
              </div>
            </div>
            <div>
              <div class="text-xs text-base-content/70 mb-1">Total Items</div>
              <div class="text-2xl font-bold">{status.total_items || 0}</div>
            </div>
          </div>

          <!-- Error Message (if failed) -->
          {#if status.error}
            <div role="alert" class="alert alert-error">
              <svg
                class="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <div>
                <div class="font-bold">Operation Error</div>
                <div class="text-sm">{status.error}</div>
              </div>
            </div>
          {/if}

          <!-- Timestamps -->
          <div class="text-xs text-base-content/70 space-y-1">
            <div>Started: {new Date(status.started_at).toLocaleString()}</div>
            {#if status.completed_at}
              <div>
                Completed: {new Date(status.completed_at).toLocaleString()}
              </div>
            {/if}
          </div>
        </div>
      {:else if loading}
        <div class="flex flex-col items-center justify-center py-8 gap-4">
          <span class="loading loading-spinner loading-lg"></span>
          <p class="text-base-content/70">Loading operation status...</p>
        </div>
      {/if}

      <!-- Actions -->
      <div class="modal-action">
        {#if canCancel}
          <button
            class="btn btn-error btn-sm"
            onclick={handleCancel}
            disabled={loading}
            aria-label="Cancel operation"
          >
            <svg
              class="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
            Cancel Operation
          </button>
        {/if}

        <button
          class="btn btn-sm"
          onclick={handleClose}
          disabled={isRunning}
          aria-label="Close dialog"
        >
          {isRunning ? "Running..." : "Close"}
        </button>
      </div>
    </div>

    <!-- Backdrop -->
    <button
      class="modal-backdrop bg-black/50"
      onclick={handleClose}
      onkeydown={(e) => e.key === "Escape" && handleClose()}
      aria-label="Close progress dialog"
      tabindex="-1"
    ></button>
  </dialog>
{/if}

<style>
  .modal-box {
    animation: slideUp 0.3s ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
