<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { success, error as errorToast } from "../../stores/toast.js";

  /** @type {{visible?: boolean}} */
  let { visible = $bindable(false) } = $props();

  /** @type {Array} edit conflicts */
  let conflicts = $state([]);
  /** @type {Object | null} selected conflict */
  let selectedConflict = $state(null);
  /** @type {string} resolution strategy */
  let resolutionStrategy = $state("keep_latest");
  /** @type {boolean} loading state */
  let loading = $state(false);

  // Load conflicts
  async function loadConflicts() {
    try {
      loading = true;
      const response = await api.collaboration.listConflicts("pending");
      conflicts = Array.isArray(response) ? response : response?.data || [];
    } catch (err) {
      errorToast("Failed to load conflicts");
    } finally {
      loading = false;
    }
  }

  // Select conflict
  function selectConflict(conflict) {
    selectedConflict = conflict;
    resolutionStrategy = "keep_latest";
  }

  // Resolve conflict
  async function resolveConflict() {
    if (!selectedConflict) return;

    try {
      loading = true;
      await api.collaboration.resolveConflict(
        selectedConflict.id,
        resolutionStrategy
      );

      success("Conflict resolved");

      // Remove from list
      conflicts = conflicts.filter((c) => c.id !== selectedConflict.id);
      selectedConflict = null;

      if (conflicts.length === 0) {
        visible = false;
      }
    } catch (err) {
      errorToast("Failed to resolve conflict");
    } finally {
      loading = false;
    }
  }

  // Format date
  function formatDate(dateStr) {
    if (!dateStr) return "";
    const date = new Date(dateStr);
    return date.toLocaleString();
  }

  // Load conflicts when modal opens
  $effect(() => {
    if (visible) {
      loadConflicts();
    }
  });
</script>

{#if visible}
  <dialog class="modal modal-open" aria-labelledby="conflicts-title">
    <div class="modal-box max-w-4xl">
      <h3
        id="conflicts-title"
        class="text-lg font-bold mb-4 flex items-center gap-2"
      >
        <svg
          class="w-5 h-5 text-warning"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          />
        </svg>
        Edit Conflicts ({conflicts.length})
      </h3>

      {#if loading && conflicts.length === 0}
        <div class="flex justify-center py-8">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
      {:else if conflicts.length === 0}
        <div class="text-center py-8 text-base-content/70">
          <svg
            class="w-16 h-16 mx-auto mb-4 opacity-50"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <p class="text-lg">No conflicts detected</p>
        </div>
      {:else}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <!-- Conflicts List -->
          <div class="space-y-2 max-h-96 overflow-y-auto">
            {#each conflicts as conflict (conflict.id)}
              <button
                class="card bg-base-200 hover:bg-base-300 w-full text-left transition-all {selectedConflict?.id ===
                conflict.id
                  ? 'ring-2 ring-primary'
                  : ''}"
                onclick={() => selectConflict(conflict)}
              >
                <div class="card-body p-3">
                  <div class="font-medium truncate">{conflict.file_path}</div>
                  <div class="text-sm text-base-content/70">
                    Type: {conflict.conflict_type}
                  </div>
                  <div class="text-xs text-base-content/60">
                    Detected: {formatDate(conflict.detected_at)}
                  </div>
                  {#if conflict.conflicting_users && conflict.conflicting_users.length > 0}
                    <div class="flex items-center gap-1 mt-2">
                      <svg
                        class="w-3 h-3"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
                        />
                      </svg>
                      <span class="text-xs text-base-content/60">
                        {conflict.conflicting_users.join(", ")}
                      </span>
                    </div>
                  {/if}
                </div>
              </button>
            {/each}
          </div>

          <!-- Resolution Panel -->
          {#if selectedConflict}
            <div class="card bg-base-200">
              <div class="card-body p-4">
                <h4 class="font-bold mb-3">Resolve Conflict</h4>

                <div class="space-y-3 mb-4">
                  <div>
                    <div class="text-sm font-medium mb-1">File:</div>
                    <div class="text-sm text-base-content/70 break-all">
                      {selectedConflict.file_path}
                    </div>
                  </div>

                  <div>
                    <div class="text-sm font-medium mb-1">Conflict Type:</div>
                    <div class="badge badge-warning">
                      {selectedConflict.conflict_type}
                    </div>
                  </div>

                  {#if selectedConflict.details}
                    <div>
                      <div class="text-sm font-medium mb-1">Details:</div>
                      <div class="text-sm text-base-content/70">
                        {selectedConflict.details}
                      </div>
                    </div>
                  {/if}
                </div>

                <div class="divider my-2"></div>

                <div class="form-control mb-4">
                  <label class="label" for="resolution-strategy">
                    <span class="label-text font-medium"
                      >Resolution Strategy</span
                    >
                  </label>
                  <select
                    id="resolution-strategy"
                    bind:value={resolutionStrategy}
                    class="select select-bordered w-full"
                  >
                    <option value="keep_latest">Keep Latest Version</option>
                    <option value="keep_version">Keep Specific Version</option>
                    <option value="auto_merge">Auto Merge (if possible)</option>
                    <option value="manual">Manual Resolution</option>
                  </select>
                </div>

                {#if resolutionStrategy === "keep_latest"}
                  <div role="alert" class="alert alert-info text-sm">
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
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                      />
                    </svg>
                    <span>The most recent version will be preserved</span>
                  </div>
                {:else if resolutionStrategy === "auto_merge"}
                  <div role="alert" class="alert alert-warning text-sm">
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
                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                      />
                    </svg>
                    <span
                      >System will attempt to merge changes automatically</span
                    >
                  </div>
                {:else if resolutionStrategy === "manual"}
                  <div role="alert" class="alert text-sm">
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
                        d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                      />
                    </svg>
                    <span>You will need to manually edit the file</span>
                  </div>
                {/if}

                <button
                  class="btn btn-primary w-full mt-4"
                  onclick={resolveConflict}
                  disabled={loading}
                >
                  {#if loading}
                    <span class="loading loading-spinner loading-sm"></span>
                  {:else}
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
                  {/if}
                  Resolve Conflict
                </button>
              </div>
            </div>
          {:else}
            <div class="card bg-base-200">
              <div class="card-body p-4 text-center text-base-content/60">
                <svg
                  class="w-12 h-12 mx-auto mb-2 opacity-50"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122"
                  />
                </svg>
                <p class="text-sm">Select a conflict to resolve</p>
              </div>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Actions -->
      <div class="modal-action">
        <button
          class="btn"
          onclick={() => (visible = false)}
          aria-label="Close conflicts dialog"
        >
          Close
        </button>
      </div>
    </div>

    <!-- Backdrop -->
    <button
      class="modal-backdrop bg-black/50"
      onclick={() => (visible = false)}
      onkeydown={(e) => e.key === "Escape" && (visible = false)}
      aria-label="Close modal"
      tabindex="-1"
    ></button>
  </dialog>
{/if}
