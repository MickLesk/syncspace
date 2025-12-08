<script>
  import { onMount, onDestroy } from "svelte";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";

  let { visible = $bindable(false) } = $props();

  let operations = $state([]);
  let pollInterval = null;

  onMount(() => {
    loadOperations();
    // Poll for updates every 2 seconds
    pollInterval = setInterval(loadOperations, 2000);
  });

  onDestroy(() => {
    if (pollInterval) clearInterval(pollInterval);
  });

  async function loadOperations() {
    try {
      // Try both API endpoints
      let jobs = [];
      try {
        jobs = (await api.bulk.listJobs()) || [];
      } catch {
        // Fallback to batch API
        try {
          const status = await api.batch.getStatus("current");
          if (status) jobs = [status];
        } catch {
          jobs = [];
        }
      }
      operations = jobs;

      // Show panel if there are active operations
      if (
        operations.some(
          (op) => op.status === "running" || op.status === "pending"
        )
      ) {
        visible = true;
      }
    } catch (err) {
      console.error("Failed to load batch operations:", err);
    }
  }
  async function cancelOperation(jobId) {
    try {
      // Try both APIs
      try {
        await api.bulk.cancelJob(jobId);
      } catch {
        await api.batch.cancelOperation(jobId);
      }
      success("Operation cancelled");
      loadOperations();
    } catch (err) {
      errorToast("Failed to cancel operation");
    }
  }

  function getStatusColor(status) {
    switch (status) {
      case "completed":
        return "bg-green-500";
      case "failed":
        return "bg-red-500";
      case "cancelled":
        return "bg-gray-500";
      case "running":
        return "bg-blue-500";
      default:
        return "bg-yellow-500";
    }
  }

  function getStatusIcon(status) {
    switch (status) {
      case "completed":
        return "check-circle-fill";
      case "failed":
        return "x-circle-fill";
      case "cancelled":
        return "slash-circle-fill";
      case "running":
        return "arrow-repeat";
      default:
        return "hourglass-split";
    }
  }

  function formatProgress(op) {
    if (!op.total || op.total === 0) return "0%";
    return `${Math.round((op.completed / op.total) * 100)}%`;
  }
</script>

{#if visible && operations.length > 0}
  <div class="batch-panel">
    <div class="batch-header">
      <div class="flex items-center gap-2">
        <i class="bi bi-layers text-lg text-primary-500" aria-hidden="true"></i>
        <span class="font-semibold">Batch Operations</span>
        <span class="badge badge-sm badge-primary">{operations.length}</span>
      </div>
      <button
        class="btn btn-ghost btn-xs"
        onclick={() => (visible = false)}
        aria-label="Close panel"
      >
        <i class="bi bi-x-lg" aria-hidden="true"></i>
      </button>
    </div>

    <div class="batch-list">
      {#each operations as op}
        <div class="batch-item">
          <div class="flex items-center gap-3 mb-2">
            <div class="status-icon {getStatusColor(op.status)}">
              <i
                class="bi bi-{getStatusIcon(op.status)} {op.status === 'running'
                  ? 'animate-spin'
                  : ''}"
                aria-hidden="true"
              ></i>
            </div>
            <div class="flex-1 min-w-0">
              <div class="font-medium text-sm truncate">
                {op.operation_type || "Operation"}
              </div>
              <div class="text-xs text-gray-500">
                {op.completed || 0} / {op.total || 0} items
              </div>
            </div>
            {#if op.status === "running" || op.status === "pending"}
              <button
                class="btn btn-ghost btn-xs text-error"
                onclick={() => cancelOperation(op.id)}
                title="Cancel"
              >
                <i class="bi bi-x-circle" aria-hidden="true"></i>
              </button>
            {/if}
          </div>

          <!-- Progress Bar -->
          <div class="progress-bar-bg">
            <div
              class="progress-bar-fill {getStatusColor(op.status)}"
              style="width: {formatProgress(op)}"
            ></div>
          </div>

          {#if op.current_file}
            <div class="text-xs text-gray-400 mt-1 truncate">
              {op.current_file}
            </div>
          {/if}

          {#if op.error}
            <div class="text-xs text-error mt-1">
              <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
              {op.error}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .batch-panel {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    width: 360px;
    max-width: calc(100vw - 2rem);
    background: var(--md-sys-color-surface);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 1rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    z-index: 100;
    overflow: hidden;
  }

  :global(.dark) .batch-panel {
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .batch-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
    background: var(--md-sys-color-surface-container-low);
  }

  .batch-list {
    max-height: 300px;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .batch-item {
    padding: 0.75rem;
    background: var(--md-sys-color-surface-container);
    border-radius: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .batch-item:last-child {
    margin-bottom: 0;
  }

  .status-icon {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 0.875rem;
  }

  .progress-bar-bg {
    height: 4px;
    background: var(--md-sys-color-surface-container-highest);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-bar-fill {
    height: 100%;
    border-radius: 2px;
    transition: width 0.3s ease;
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
