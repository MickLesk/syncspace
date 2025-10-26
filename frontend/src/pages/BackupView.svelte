<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../stores/toast";
  import api from "../lib/api";
  import BackupScheduleManager from "../components/BackupScheduleManager.svelte";
  import BackupVerificationPanel from "../components/BackupVerificationPanel.svelte";

  let activeTab = $state("backups"); // 'backups', 'schedules', 'export'
  let viewMode = $state("timeline"); // 'timeline', 'grid'
  let backups = $state([]);
  let selectedBackup = $state(null);
  let loadingBackups = $state(false);
  let restorePreview = $state(null);
  let showRestoreModal = $state(false);

  function getStatusBadge(status) {
    const badges = {
      completed: "badge-success",
      in_progress: "badge-warning",
      failed: "badge-error",
    };
    return badges[status] || "badge-ghost";
  }

  function getStatusIcon(status) {
    const icons = {
      completed: "check-circle-fill",
      in_progress: "hourglass-split",
      failed: "x-circle-fill",
    };
    return icons[status] || "question-circle";
  }

  function getStatusColor(status) {
    const colors = {
      completed: "success",
      in_progress: "warning",
      failed: "error",
    };
    return colors[status] || "neutral";
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

  async function previewRestore(backup) {
    try {
      // Mock restore preview - in real app this would fetch from API
      restorePreview = {
        backupId: backup.id,
        backupDate: backup.created_at,
        fileCount: backup.file_count || 0,
        size: backup.size_bytes || 0,
        type: backup.backup_type,
        estimatedDuration:
          Math.ceil((backup.file_count || 0) / 100) + " minutes",
      };
      showRestoreModal = true;
    } catch (err) {
      errorToast("Failed to load restore preview");
    }
  }

  async function confirmRestore() {
    if (!restorePreview) return;

    try {
      success("Restore started! This may take a while.");
      showRestoreModal = false;
      restorePreview = null;
      // Add actual restore API call here
    } catch (err) {
      errorToast("Failed to restore backup");
    }
  }

  function getRelativeTime(dateString) {
    const date = new Date(dateString);
    const now = new Date();
    const diff = now - date;
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 30) return formatDate(dateString);
    if (days > 0) return `${days} day${days > 1 ? "s" : ""} ago`;
    if (hours > 0) return `${hours} hour${hours > 1 ? "s" : ""} ago`;
    if (minutes > 0) return `${minutes} minute${minutes > 1 ? "s" : ""} ago`;
    return "Just now";
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
        <div class="flex gap-2 items-center">
          <!-- View Mode Toggle -->
          <div class="join">
            <button
              class="btn btn-sm join-item {viewMode === 'timeline'
                ? 'btn-active'
                : ''}"
              onclick={() => (viewMode = "timeline")}
              title="Timeline View"
            >
              <i class="bi bi-clock-history"></i>
            </button>
            <button
              class="btn btn-sm join-item {viewMode === 'grid'
                ? 'btn-active'
                : ''}"
              onclick={() => (viewMode = "grid")}
              title="Grid View"
            >
              <i class="bi bi-grid-3x3-gap"></i>
            </button>
          </div>

          <!-- Create Backup Buttons -->
          <button
            class="btn btn-sm btn-primary"
            onclick={() => createBackup("full")}
          >
            <i class="bi bi-plus-lg"></i>
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
          <i class="bi bi-archive text-6xl text-base-content/30 mb-4"></i>
          <p class="text-base-content/70 mb-4">No backups found</p>
          <button class="btn btn-primary" onclick={() => createBackup("full")}>
            Create your first backup
          </button>
        </div>
      {:else if viewMode === "timeline"}
        <!-- Timeline View -->
        <div class="backup-timeline">
          {#each backups as backup, index}
            <div class="timeline-item">
              <div class="timeline-marker">
                <div
                  class="timeline-icon timeline-icon-{getStatusColor(
                    backup.status
                  )}"
                >
                  <i class="bi bi-{getStatusIcon(backup.status)}"></i>
                </div>
                {#if index < backups.length - 1}
                  <div class="timeline-line"></div>
                {/if}
              </div>

              <div class="timeline-content">
                <div
                  class="card bg-base-100 shadow-md hover:shadow-lg transition-all"
                >
                  <div class="card-body p-4">
                    <div class="flex justify-between items-start mb-3">
                      <div class="flex-1">
                        <div class="flex items-center gap-2 mb-2">
                          <span
                            class="badge badge-outline badge-{getStatusColor(
                              backup.status
                            )} badge-lg"
                          >
                            {backup.backup_type}
                          </span>
                          <span class="badge {getStatusBadge(backup.status)}">
                            {backup.status}
                          </span>
                        </div>
                        <h3 class="font-bold text-sm text-base-content/70">
                          {backup.id}
                        </h3>
                      </div>
                      <div class="text-right">
                        <div class="text-sm font-semibold text-primary">
                          {getRelativeTime(backup.created_at)}
                        </div>
                        <div class="text-xs text-base-content/50">
                          {formatDate(backup.created_at)}
                        </div>
                      </div>
                    </div>

                    <!-- Backup Stats -->
                    <div class="grid grid-cols-3 gap-3 mb-3">
                      <div class="stat-mini">
                        <i class="bi bi-hdd text-primary"></i>
                        <div class="stat-mini-label">Size</div>
                        <div class="stat-mini-value">
                          {formatBytes(backup.size_bytes)}
                        </div>
                      </div>
                      <div class="stat-mini">
                        <i class="bi bi-files text-secondary"></i>
                        <div class="stat-mini-label">Files</div>
                        <div class="stat-mini-value">
                          {backup.file_count || 0}
                        </div>
                      </div>
                      <div class="stat-mini">
                        <i class="bi bi-speedometer2 text-accent"></i>
                        <div class="stat-mini-label">Duration</div>
                        <div class="stat-mini-value">
                          {backup.duration_seconds
                            ? `${backup.duration_seconds}s`
                            : "N/A"}
                        </div>
                      </div>
                    </div>

                    {#if backup.error_message}
                      <div class="alert alert-error alert-sm mb-3">
                        <i class="bi bi-exclamation-triangle-fill"></i>
                        <span class="text-xs">{backup.error_message}</span>
                      </div>
                    {/if}

                    <!-- Actions -->
                    <div class="flex gap-2 justify-end">
                      {#if backup.status === "completed"}
                        <button
                          class="btn btn-sm btn-primary"
                          onclick={() => previewRestore(backup)}
                        >
                          <i class="bi bi-arrow-counterclockwise"></i>
                          Restore
                        </button>
                        <button
                          class="btn btn-sm btn-outline"
                          onclick={() => (selectedBackup = backup)}
                        >
                          <i class="bi bi-shield-check"></i>
                          Verify
                        </button>
                      {/if}
                      <button
                        class="btn btn-sm btn-ghost text-error"
                        onclick={() => deleteBackup(backup.id)}
                      >
                        <i class="bi bi-trash"></i>
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <!-- Grid View (Original) -->
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

<!-- Restore Preview Modal -->
{#if showRestoreModal && restorePreview}
  <div class="modal modal-open">
    <div
      class="modal-backdrop-enhanced"
      onclick={() => (showRestoreModal = false)}
    ></div>
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-xl mb-4 flex items-center gap-2">
        <i class="bi bi-arrow-counterclockwise text-primary"></i>
        Restore Preview
      </h3>

      <div class="alert alert-warning mb-4">
        <i class="bi bi-exclamation-triangle-fill"></i>
        <div>
          <div class="font-bold">Warning</div>
          <div class="text-sm">
            Restoring will replace current data. Make sure you have a recent
            backup before proceeding.
          </div>
        </div>
      </div>

      <div class="space-y-4">
        <!-- Backup Details -->
        <div class="bg-base-200 rounded-lg p-4">
          <h4 class="font-semibold mb-3">Backup Information</h4>
          <div class="grid grid-cols-2 gap-3 text-sm">
            <div>
              <span class="text-base-content/70">Backup ID:</span>
              <div class="font-mono text-xs mt-1">
                {restorePreview.backupId}
              </div>
            </div>
            <div>
              <span class="text-base-content/70">Created:</span>
              <div class="mt-1">{formatDate(restorePreview.backupDate)}</div>
            </div>
            <div>
              <span class="text-base-content/70">Type:</span>
              <div class="mt-1">
                <span class="badge badge-outline">{restorePreview.type}</span>
              </div>
            </div>
            <div>
              <span class="text-base-content/70">Size:</span>
              <div class="mt-1">{formatBytes(restorePreview.size)}</div>
            </div>
            <div>
              <span class="text-base-content/70">Files:</span>
              <div class="mt-1">
                {restorePreview.fileCount.toLocaleString()}
              </div>
            </div>
            <div>
              <span class="text-base-content/70">Estimated Duration:</span>
              <div class="mt-1">{restorePreview.estimatedDuration}</div>
            </div>
          </div>
        </div>

        <!-- Restore Impact -->
        <div class="bg-error/10 border border-error/20 rounded-lg p-4">
          <h4 class="font-semibold text-error mb-2 flex items-center gap-2">
            <i class="bi bi-exclamation-octagon-fill"></i>
            Restore Impact
          </h4>
          <ul
            class="list-disc list-inside text-sm space-y-1 text-base-content/70"
          >
            <li>All current files will be replaced with backup versions</li>
            <li>Database will be restored to backup state</li>
            <li>Recent changes since backup will be lost</li>
            <li>System will be unavailable during restore</li>
          </ul>
        </div>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => (showRestoreModal = false)}
        >
          Cancel
        </button>
        <button class="btn btn-error" onclick={confirmRestore}>
          <i class="bi bi-arrow-counterclockwise"></i>
          Confirm Restore
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .backup-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }

  /* Timeline Styles */
  .backup-timeline {
    position: relative;
    padding-left: 2rem;
  }

  .timeline-item {
    position: relative;
    display: flex;
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .timeline-item:last-child {
    margin-bottom: 0;
  }

  .timeline-marker {
    position: relative;
    flex-shrink: 0;
  }

  .timeline-icon {
    width: 3rem;
    height: 3rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    position: relative;
    z-index: 2;
    transition: all 0.3s ease;
  }

  .timeline-icon-success {
    background: hsl(var(--su));
    color: hsl(var(--suc));
  }

  .timeline-icon-warning {
    background: hsl(var(--wa));
    color: hsl(var(--wac));
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  .timeline-icon-error {
    background: hsl(var(--er));
    color: hsl(var(--erc));
  }

  .timeline-line {
    position: absolute;
    left: 50%;
    top: 3rem;
    bottom: -2rem;
    width: 2px;
    background: linear-gradient(
      to bottom,
      hsl(var(--bc) / 0.2) 0%,
      hsl(var(--bc) / 0.1) 100%
    );
    transform: translateX(-50%);
    z-index: 1;
  }

  .timeline-content {
    flex: 1;
    animation: slideEnter 0.4s ease-out;
  }

  .timeline-content .card {
    border: 2px solid hsl(var(--bc) / 0.05);
  }

  .timeline-content .card:hover {
    border-color: hsl(var(--p) / 0.2);
  }

  /* Mini Stats */
  .stat-mini {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.75rem;
    background: hsl(var(--b2) / 0.5);
    border-radius: 0.5rem;
    text-align: center;
  }

  .stat-mini i {
    font-size: 1.25rem;
    margin-bottom: 0.25rem;
  }

  .stat-mini-label {
    font-size: 0.625rem;
    font-weight: 600;
    color: hsl(var(--bc) / 0.6);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .stat-mini-value {
    font-size: 0.875rem;
    font-weight: 700;
    color: hsl(var(--bc));
  }

  /* Modal Backdrop */
  .modal-backdrop-enhanced {
    position: fixed;
    inset: 0;
    background: hsl(var(--b1) / 0.7);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    animation: fadeIn 0.3s ease-out;
  }

  @media (max-width: 768px) {
    .backup-timeline {
      padding-left: 1rem;
    }

    .timeline-item {
      gap: 1rem;
    }

    .timeline-icon {
      width: 2.5rem;
      height: 2.5rem;
      font-size: 1rem;
    }

    .timeline-line {
      top: 2.5rem;
    }
  }
</style>
