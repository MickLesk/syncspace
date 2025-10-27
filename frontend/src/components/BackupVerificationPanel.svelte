<script>
  import { onMount } from "svelte";

  let { backupId } = $props();

  let verifications = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let verifying = $state(false);

  onMount(() => {
    loadVerifications();
  });

  async function loadVerifications() {
    loading = true;
    error = null;
    try {
      const response = await fetch(`/api/backups/${backupId}/verifications`, {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });

      if (!response.ok) throw new Error("Failed to load verifications");

      verifications = await response.json();
    } catch (err) {
      error = err.message;
      console.error("Failed to load verifications:", err);
    } finally {
      loading = false;
    }
  }

  async function verifyBackup(verificationType) {
    verifying = true;
    error = null;

    try {
      const response = await fetch(`/api/backups/${backupId}/verify`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
        body: JSON.stringify({ verification_type: verificationType }),
      });

      if (!response.ok) throw new Error("Failed to start verification");

      // Poll for verification completion
      setTimeout(() => loadVerifications(), 2000);
    } catch (err) {
      error = err.message;
      console.error("Failed to verify backup:", err);
    } finally {
      verifying = false;
    }
  }

  function formatDate(dateString) {
    return new Date(dateString).toLocaleString();
  }

  function getStatusBadge(status) {
    const badges = {
      passed: "badge-success",
      failed: "badge-error",
      in_progress: "badge-warning",
    };
    return badges[status] || "badge-ghost";
  }

  function getVerificationIcon(type) {
    const icons = {
      checksum: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
      file_count:
        "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
      restore_test:
        "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15",
    };
    return icons[type] || "M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z";
  }
</script>

<div class="backup-verification-panel">
  <div class="header mb-4">
    <h3 class="text-lg font-bold mb-2">Backup Verifications</h3>
    <div class="flex gap-2">
      <button
        class="btn btn-sm btn-primary"
        onclick={() => verifyBackup("checksum")}
        disabled={verifying}
      >
        {#if verifying}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <svg
            class="w-4 h-4 mr-1"
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
        {/if}
        Checksum
      </button>

      <button
        class="btn btn-sm btn-primary"
        onclick={() => verifyBackup("file_count")}
        disabled={verifying}
      >
        {#if verifying}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <svg
            class="w-4 h-4 mr-1"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
        {/if}
        File Count
      </button>

      <button
        class="btn btn-sm btn-primary"
        onclick={() => verifyBackup("restore_test")}
        disabled={verifying}
      >
        {#if verifying}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <svg
            class="w-4 h-4 mr-1"
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
        {/if}
        Restore Test
      </button>
    </div>
  </div>

  {#if error}
    <div class="alert alert-error mb-4">
      <svg
        class="w-6 h-6"
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

  {#if loading}
    <div class="flex justify-center py-8">
      <span class="loading loading-spinner loading-md"></span>
    </div>
  {:else if verifications.length === 0}
    <div class="text-center py-8 text-base-content/70">
      <svg
        class="w-12 h-12 mx-auto mb-3 opacity-30"
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
      <p>No verifications yet</p>
      <p class="text-sm mt-1">Run a verification to ensure backup integrity</p>
    </div>
  {:else}
    <div class="space-y-3">
      {#each verifications as verification}
        <div class="card bg-slate-50 dark:bg-slate-800 shadow-sm">
          <div class="card-body p-4">
            <div class="flex items-start justify-between">
              <div class="flex items-start gap-3 flex-1">
                <svg
                  class="w-6 h-6 mt-1 text-primary"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d={getVerificationIcon(verification.verification_type)}
                  />
                </svg>

                <div class="flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="font-semibold capitalize">
                      {verification.verification_type.replace("_", " ")}
                    </span>
                    <div
                      class="badge badge-sm {getStatusBadge(
                        verification.status
                      )}"
                    >
                      {verification.status}
                    </div>
                  </div>

                  <div class="text-sm text-base-content/70">
                    {formatDate(verification.verified_at)}
                  </div>

                  {#if verification.details}
                    <div class="mt-2 text-sm">
                      {#if verification.verification_type === "checksum"}
                        <div class="font-mono text-xs bg-slate-100 dark:bg-slate-700 p-2 rounded">
                          {JSON.parse(verification.details).checksum ||
                            verification.details}
                        </div>
                      {:else if verification.verification_type === "file_count"}
                        <div>
                          Files: <span class="font-semibold"
                            >{JSON.parse(verification.details).file_count ||
                              "N/A"}</span
                          >
                        </div>
                      {:else}
                        <div class="bg-slate-100 dark:bg-slate-700 p-2 rounded text-xs">
                          {verification.details}
                        </div>
                      {/if}
                    </div>
                  {/if}
                </div>
              </div>

              {#if verification.status === "in_progress"}
                <span class="loading loading-spinner loading-sm"></span>
              {:else if verification.status === "passed"}
                <svg
                  class="w-6 h-6 text-success"
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
              {:else if verification.status === "failed"}
                <svg
                  class="w-6 h-6 text-error"
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
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .backup-verification-panel {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: 1.5rem;
  }
</style>
