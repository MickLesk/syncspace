<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../stores/toast";
  import api from "../lib/api";
  import BackupScheduleManager from "../components/BackupScheduleManager.svelte";
  import BackupVerificationPanel from "../components/BackupVerificationPanel.svelte";

  let activeTab = $state("backups"); // 'backups', 'schedules', 'export'
  let backups = $state([]);
  let selectedBackup = $state(null);
  let loadingBackups = $state(false);

  function getStatusBadge(status) {
    const badges = {
      completed: "badge-success",
      in_progress: "badge-warning",
      failed: "badge-error",
    };
    return badges[status] || "badge-ghost";
  }

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return (bytes / Math.pow(k, i)).toFixed(2) + " " + sizes[i];
  }

  function formatDate(dateString) {
    if (!dateString) return "Never";
    return new Date(dateString).toLocaleString();
  }

  async function createBackup(type = "full") {
    try {
      await api.backup.create(type, true);
      success("Backup started! Check back in a few moments.");
      setTimeout(() => loadBackups(), 2000);
    } catch (err) {
      errorToast("Failed to create backup: " + (err.message || err));
      console.error(err);
    }
  }

  async function deleteBackup(backupId) {
    if (!confirm("Are you sure you want to delete this backup?")) return;

    try {
      await api.backup.delete(backupId);
      success("Backup deleted successfully");
      selectedBackup = null;
      await loadBackups();
    } catch (err) {
      errorToast("Failed to delete backup: " + (err.message || err));
      console.error(err);
    }
  }

  async function loadBackups() {
    loadingBackups = true;
    try {
      backups = await api.backup.list();
    } catch (err) {
      errorToast("Failed to load backups: " + (err.message || err));
      console.error(err);
    } finally {
      loadingBackups = false;
    }
  }

  $effect(() => {
    if (activeTab === "backups" && backups.length === 0) {
      loadBackups();
    }
  });
</script>

<div class="backup-view p-6">
  <div class="mb-6">
    <h1 class="text-3xl font-bold mb-2">Backup & Restore</h1>
    <p class="text-base-content/70">
      Manage backups, schedules, and restore points
    </p>
  </div>

  <!-- Tabs -->
  <div class="tabs tabs-boxed mb-6">
    <button
      class="tab {activeTab === 'backups' ? 'tab-active' : ''}"
      onclick={() => (activeTab = "backups")}
    >
      <svg
        class="w-5 h-5 mr-2"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
        />
      </svg>
      Backups
    </button>

    <button
      class="tab {activeTab === 'schedules' ? 'tab-active' : ''}"
      onclick={() => (activeTab = "schedules")}
    >
      <svg
        class="w-5 h-5 mr-2"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      Schedules
    </button>

    <button
      class="tab {activeTab === 'export' ? 'tab-active' : ''}"
      onclick={() => (activeTab = "export")}
    >
      <svg
        class="w-5 h-5 mr-2"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
        />
      </svg>
      Export/Import
    </button>
  </div>

  <!-- Tab Content -->
  {#if activeTab === "backups"}
    <div class="backups-tab">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-bold">Backup History</h2>
        <div class="flex gap-2">
          <button
            class="btn btn-sm btn-primary"
            onclick={() => createBackup("full")}
          >
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
                d="M12 4v16m8-8H4"
              />
            </svg>
            Full Backup
          </button>
          <button
            class="btn btn-sm btn-outline"
            onclick={() => createBackup("database")}
          >
            Database
          </button>
          <button
            class="btn btn-sm btn-outline"
            onclick={() => createBackup("files")}
          >
            Files
          </button>
        </div>
      </div>

      {#if loadingBackups}
        <div class="flex justify-center py-12">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
      {:else if backups.length === 0}
        <div class="text-center py-12">
          <svg
            class="w-16 h-16 mx-auto mb-4 text-base-content/30"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
            />
          </svg>
          <p class="text-base-content/70 mb-4">No backups found</p>
          <button class="btn btn-primary" onclick={() => createBackup("full")}>
            Create your first backup
          </button>
        </div>
      {:else}
        <div class="grid gap-4">
          {#each backups as backup}
            <div class="card bg-base-200 shadow-md">
              <div class="card-body">
                <div class="flex justify-between items-start">
                  <div class="flex-1">
                    <h3 class="card-title text-lg mb-2">
                      <span class="badge badge-sm">{backup.backup_type}</span>
                      {backup.id}
                      <div
                        class="badge badge-sm {getStatusBadge(backup.status)}"
                      >
                        {backup.status}
                      </div>
                    </h3>

                    <div class="grid grid-cols-3 gap-3 text-sm">
                      <div>
                        <span class="text-base-content/70">Size:</span>
                        <span class="ml-2 font-semibold"
                          >{formatBytes(backup.size_bytes)}</span
                        >
                      </div>
                      <div>
                        <span class="text-base-content/70">Files:</span>
                        <span class="ml-2 font-semibold"
                          >{backup.file_count || 0}</span
                        >
                      </div>
                      <div>
                        <span class="text-base-content/70">Created:</span>
                        <span class="ml-2">{formatDate(backup.created_at)}</span
                        >
                      </div>
                    </div>

                    {#if backup.error_message}
                      <div class="alert alert-error mt-2">
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
                        <span>{backup.error_message}</span>
                      </div>
                    {/if}
                  </div>

                  <div class="flex gap-2">
                    {#if backup.status === "completed"}
                      <button
                        class="btn btn-sm btn-primary"
                        onclick={() => (selectedBackup = backup)}
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
                            d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                          />
                        </svg>
                        Verify
                      </button>
                    {/if}

                    <button
                      class="btn btn-sm btn-ghost text-error"
                      onclick={() => deleteBackup(backup.id)}
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
                          d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                        />
                      </svg>
                    </button>
                  </div>
                </div>

                {#if selectedBackup?.id === backup.id}
                  <div class="mt-4 pt-4 border-t border-base-300">
                    <BackupVerificationPanel backupId={backup.id} />
                  </div>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else if activeTab === "schedules"}
    <BackupScheduleManager />
  {:else if activeTab === "export"}
    <div class="export-tab">
      <div class="alert alert-info mb-6">
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
            d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        <div>
          <h3 class="font-bold">Export/Import Feature</h3>
          <div class="text-sm">
            Manual backup export and import coming soon. Use the Backups tab for
            automated backups.
          </div>
        </div>
      </div>

      <div class="card bg-base-200">
        <div class="card-body">
          <h3 class="card-title">Coming Soon</h3>
          <p>
            Manual export and import functionality will be integrated here. For
            now, use the backend API or schedules for automated backups.
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .backup-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }
</style>
