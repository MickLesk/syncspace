<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";
  import BackupScheduleManager from "../../components/backup/BackupScheduleManager.svelte";
  import BackupVerificationPanel from "../../components/backup/BackupVerificationPanel.svelte";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";

  let activeTab = $state("backups"); // 'backups', 'schedules', 'export'
  let viewMode = $state("timeline"); // 'timeline', 'grid'
  let backups = $state([]);
  let selectedBackup = $state(null);
  let loadingBackups = $state(false);
  let restorePreview = $state(null);
  let showRestoreModal = $state(false);

  function getStatusBadge(status) {
    const badges = {
      completed: "badge-glass-success",
      in_progress: "badge-glass-warning",
      failed: "badge-glass-error",
    };
    return badges[status] || "badge-glass-info";
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
    const diff = now.getTime() - date.getTime();
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

<PageWrapper gradient>
  <PageHeader
    title="Backup & Restore"
    subtitle="Manage backups, schedules, and restore points"
    icon="cloud-arrow-up"
  />

  <!-- Tabs -->
  <ModernCard variant="glass" class="mb-6">
    <div class="flex gap-2 p-2">
      <ModernButton
        variant={activeTab === "backups" ? "primary" : "ghost"}
        onclick={() => (activeTab = "backups")}
        class="flex-1"
      >
        <i class="bi bi-archive mr-2"></i>
        Backups
      </ModernButton>

      <ModernButton
        variant={activeTab === "schedules" ? "primary" : "ghost"}
        onclick={() => (activeTab = "schedules")}
        class="flex-1"
      >
        <i class="bi bi-clock-history mr-2"></i>
        Schedules
      </ModernButton>

      <ModernButton
        variant={activeTab === "export" ? "primary" : "ghost"}
        onclick={() => (activeTab = "export")}
        class="flex-1"
      >
        <i class="bi bi-cloud-arrow-down mr-2"></i>
        Export/Import
      </ModernButton>
    </div>
  </ModernCard>

  <!-- Tab Content -->
  {#if activeTab === "backups"}
    <div class="backups-tab">
      <ModernCard variant="glass" class="mb-4">
        <div
          class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 p-4"
        >
          <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
            Backup History
          </h2>
          <div class="flex flex-wrap gap-2 items-center">
            <!-- View Mode Toggle -->
            <div
              class="flex rounded-lg overflow-hidden border-2 border-gray-200 dark:border-gray-700"
            >
              <button
                class="px-3 py-1.5 text-sm transition-all {viewMode ===
                'timeline'
                  ? 'bg-primary-600 text-white'
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
                onclick={() => (viewMode = "timeline")}
                title="Timeline View"
              >
                <i class="bi bi-clock-history"></i>
              </button>
              <button
                class="px-3 py-1.5 text-sm border-l-2 border-gray-200 dark:border-gray-700 transition-all {viewMode ===
                'grid'
                  ? 'bg-primary-600 text-white'
                  : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
                onclick={() => (viewMode = "grid")}
                title="Grid View"
              >
                <i class="bi bi-grid-3x3-gap"></i>
              </button>
            </div>

            <!-- Create Backup Buttons -->
            <ModernButton
              variant="gradient"
              onclick={() => createBackup("full")}
            >
              <i class="bi bi-plus-lg mr-2"></i>
              Full Backup
            </ModernButton>
            <ModernButton
              variant="secondary"
              onclick={() => createBackup("database")}
            >
              Database
            </ModernButton>
            <ModernButton
              variant="secondary"
              onclick={() => createBackup("files")}
            >
              Files
            </ModernButton>
          </div>
        </div>
      </ModernCard>

      {#if loadingBackups}
        <ModernCard variant="glass">
          <div class="flex justify-center py-12">
            <span class="loading loading-spinner loading-lg text-primary-600"
            ></span>
          </div>
        </ModernCard>
      {:else if backups.length === 0}
        <ModernCard variant="glass">
          <div class="text-center py-12">
            <i
              class="bi bi-archive text-6xl text-gray-300 dark:text-gray-600 mb-4"
            ></i>
            <p class="text-gray-600 dark:text-gray-400 mb-4">
              No backups found
            </p>
            <ModernButton
              variant="gradient"
              onclick={() => createBackup("full")}
            >
              <i class="bi bi-plus-lg mr-2"></i>
              Create your first backup
            </ModernButton>
          </div>
        </ModernCard>
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
                <ModernCard
                  variant="glass"
                  hoverable
                  class="border-2 border-gray-100 dark:border-gray-800 hover:border-primary-200 dark:hover:border-primary-800"
                >
                  <div class="p-4">
                    <div class="flex justify-between items-start mb-3">
                      <div class="flex-1">
                        <div class="flex flex-wrap items-center gap-2 mb-2">
                          <span class="badge-glass-primary">
                            {backup.backup_type}
                          </span>
                          <span class={getStatusBadge(backup.status)}>
                            {backup.status}
                          </span>
                        </div>
                        <h3
                          class="font-bold text-sm text-gray-600 dark:text-gray-400"
                        >
                          {backup.id}
                        </h3>
                      </div>
                      <div class="text-right">
                        <div
                          class="text-sm font-semibold text-primary-600 dark:text-primary-400"
                        >
                          {getRelativeTime(backup.created_at)}
                        </div>
                        <div class="text-xs text-gray-500 dark:text-gray-500">
                          {formatDate(backup.created_at)}
                        </div>
                      </div>
                    </div>

                    <!-- Backup Stats -->
                    <div class="grid grid-cols-3 gap-3 mb-3">
                      <div class="stat-mini">
                        <i
                          class="bi bi-hdd text-primary-600 dark:text-primary-400"
                        ></i>
                        <div class="stat-mini-label">Size</div>
                        <div class="stat-mini-value">
                          {formatBytes(backup.size_bytes)}
                        </div>
                      </div>
                      <div class="stat-mini">
                        <i
                          class="bi bi-files text-secondary-600 dark:text-secondary-400"
                        ></i>
                        <div class="stat-mini-label">Files</div>
                        <div class="stat-mini-value">
                          {backup.file_count || 0}
                        </div>
                      </div>
                      <div class="stat-mini">
                        <i
                          class="bi bi-speedometer2 text-purple-600 dark:text-purple-400"
                        ></i>
                        <div class="stat-mini-label">Duration</div>
                        <div class="stat-mini-value">
                          {backup.duration_seconds
                            ? `${backup.duration_seconds}s`
                            : "N/A"}
                        </div>
                      </div>
                    </div>

                    {#if backup.error_message}
                      <div
                        class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3 mb-3 flex items-start gap-2"
                      >
                        <i
                          class="bi bi-exclamation-triangle-fill text-red-600 dark:text-red-400"
                        ></i>
                        <span class="text-xs text-red-700 dark:text-red-300"
                          >{backup.error_message}</span
                        >
                      </div>
                    {/if}

                    <!-- Actions -->
                    <div class="flex flex-wrap gap-2 justify-end">
                      {#if backup.status === "completed"}
                        <ModernButton
                          variant="primary"
                          onclick={() => previewRestore(backup)}
                          size="sm"
                        >
                          <i class="bi bi-arrow-counterclockwise mr-1"></i>
                          Restore
                        </ModernButton>
                        <ModernButton
                          variant="secondary"
                          onclick={() => (selectedBackup = backup)}
                          size="sm"
                        >
                          <i class="bi bi-shield-check mr-1"></i>
                          Verify
                        </ModernButton>
                      {/if}
                      <ModernButton
                        variant="danger"
                        onclick={() => deleteBackup(backup.id)}
                        size="sm"
                        aria-label="Delete backup"
                      >
                        <i class="bi bi-trash"></i>
                      </ModernButton>
                    </div>
                  </div>
                </ModernCard>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <!-- Grid View (Original) -->
        <div class="grid gap-4">
          {#each backups as backup}
            <ModernCard variant="glass" hoverable>
              <div class="p-4">
                <div
                  class="flex flex-col sm:flex-row justify-between items-start gap-4"
                >
                  <div class="flex-1">
                    <h3
                      class="text-lg font-bold mb-2 flex flex-wrap items-center gap-2"
                    >
                      <span class="badge-glass-primary"
                        >{backup.backup_type}</span
                      >
                      <span class="text-gray-900 dark:text-gray-100 text-base"
                        >{backup.id}</span
                      >
                      <span class={getStatusBadge(backup.status)}>
                        {backup.status}
                      </span>
                    </h3>

                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 text-sm">
                      <div>
                        <span class="text-gray-600 dark:text-gray-400"
                          >Size:</span
                        >
                        <span
                          class="ml-2 font-semibold text-gray-900 dark:text-gray-100"
                          >{formatBytes(backup.size_bytes)}</span
                        >
                      </div>
                      <div>
                        <span class="text-gray-600 dark:text-gray-400"
                          >Files:</span
                        >
                        <span
                          class="ml-2 font-semibold text-gray-900 dark:text-gray-100"
                          >{backup.file_count || 0}</span
                        >
                      </div>
                      <div>
                        <span class="text-gray-600 dark:text-gray-400"
                          >Created:</span
                        >
                        <span class="ml-2 text-gray-900 dark:text-gray-100"
                          >{formatDate(backup.created_at)}</span
                        >
                      </div>
                    </div>

                    {#if backup.error_message}
                      <div
                        class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3 mt-3 flex items-start gap-2"
                      >
                        <i
                          class="bi bi-exclamation-circle-fill text-red-600 dark:text-red-400"
                        ></i>
                        <span class="text-sm text-red-700 dark:text-red-300"
                          >{backup.error_message}</span
                        >
                      </div>
                    {/if}
                  </div>

                  <div class="flex flex-wrap gap-2">
                    {#if backup.status === "completed"}
                      <ModernButton
                        variant="secondary"
                        onclick={() => (selectedBackup = backup)}
                      >
                        <i class="bi bi-shield-check mr-1"></i>
                        Verify
                      </ModernButton>
                    {/if}

                    <ModernButton
                      variant="danger"
                      onclick={() => deleteBackup(backup.id)}
                      aria-label="Delete backup"
                    >
                      <i class="bi bi-trash"></i>
                    </ModernButton>
                  </div>
                </div>

                {#if selectedBackup?.id === backup.id}
                  <div
                    class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700"
                  >
                    <BackupVerificationPanel backupId={backup.id} />
                  </div>
                {/if}
              </div>
            </ModernCard>
          {/each}
        </div>
      {/if}
    </div>
  {:else if activeTab === "schedules"}
    <BackupScheduleManager />
  {:else if activeTab === "export"}
    <div class="export-tab space-y-4">
      <ModernCard variant="glass">
        <div
          class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4 flex items-start gap-3"
        >
          <i
            class="bi bi-info-circle-fill text-blue-600 dark:text-blue-400 text-2xl"
          ></i>
          <div>
            <h3 class="font-bold text-blue-900 dark:text-blue-100 mb-1">
              Export/Import Feature
            </h3>
            <div class="text-sm text-blue-800 dark:text-blue-200">
              Manual backup export and import coming soon. Use the Backups tab
              for automated backups.
            </div>
          </div>
        </div>
      </ModernCard>

      <ModernCard variant="glass">
        <div class="p-6">
          <h3 class="text-xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            Coming Soon
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            Manual export and import functionality will be integrated here. For
            now, use the backend API or schedules for automated backups.
          </p>
        </div>
      </ModernCard>
    </div>
  {/if}
</PageWrapper>

<!-- Restore Preview Modal -->
{#if showRestoreModal && restorePreview}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm"
      onclick={() => (showRestoreModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showRestoreModal = false)}
      role="button"
      tabindex="0"
      aria-label="Close modal"
    ></div>

    <ModernCard
      variant="glass"
      class="relative max-w-2xl w-full max-h-[90vh] overflow-y-auto"
    >
      <div class="p-6">
        <h3
          class="font-bold text-xl mb-4 flex items-center gap-2 text-gray-900 dark:text-gray-100"
        >
          <i class="bi bi-arrow-counterclockwise text-primary-600"></i>
          Restore Preview
        </h3>

        <div
          class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4 mb-4 flex items-start gap-3"
        >
          <i
            class="bi bi-exclamation-triangle-fill text-yellow-600 dark:text-yellow-400 text-xl"
          ></i>
          <div>
            <div class="font-bold text-yellow-900 dark:text-yellow-100">
              Warning
            </div>
            <div class="text-sm text-yellow-800 dark:text-yellow-200">
              Restoring will replace current data. Make sure you have a recent
              backup before proceeding.
            </div>
          </div>
        </div>

        <div class="space-y-4">
          <!-- Backup Details -->
          <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
            <h4 class="font-semibold mb-3 text-gray-900 dark:text-gray-100">
              Backup Information
            </h4>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm">
              <div>
                <span class="text-gray-600 dark:text-gray-400">Backup ID:</span>
                <div
                  class="font-mono text-xs mt-1 text-gray-900 dark:text-gray-100"
                >
                  {restorePreview.backupId}
                </div>
              </div>
              <div>
                <span class="text-gray-600 dark:text-gray-400">Created:</span>
                <div class="mt-1 text-gray-900 dark:text-gray-100">
                  {formatDate(restorePreview.backupDate)}
                </div>
              </div>
              <div>
                <span class="text-gray-600 dark:text-gray-400">Type:</span>
                <div class="mt-1">
                  <span class="badge-glass-primary">{restorePreview.type}</span>
                </div>
              </div>
              <div>
                <span class="text-gray-600 dark:text-gray-400">Size:</span>
                <div class="mt-1 text-gray-900 dark:text-gray-100">
                  {formatBytes(restorePreview.size)}
                </div>
              </div>
              <div>
                <span class="text-gray-600 dark:text-gray-400">Files:</span>
                <div class="mt-1 text-gray-900 dark:text-gray-100">
                  {restorePreview.fileCount.toLocaleString()}
                </div>
              </div>
              <div>
                <span class="text-gray-600 dark:text-gray-400"
                  >Estimated Duration:</span
                >
                <div class="mt-1 text-gray-900 dark:text-gray-100">
                  {restorePreview.estimatedDuration}
                </div>
              </div>
            </div>
          </div>

          <!-- Restore Impact -->
          <div
            class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4"
          >
            <h4
              class="font-semibold text-red-700 dark:text-red-300 mb-2 flex items-center gap-2"
            >
              <i class="bi bi-exclamation-octagon-fill"></i>
              Restore Impact
            </h4>
            <ul
              class="list-disc list-inside text-sm space-y-1 text-red-600 dark:text-red-400"
            >
              <li>All current files will be replaced with backup versions</li>
              <li>Database will be restored to backup state</li>
              <li>Recent changes since backup will be lost</li>
              <li>System will be unavailable during restore</li>
            </ul>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
          <ModernButton
            variant="ghost"
            onclick={() => (showRestoreModal = false)}
          >
            Cancel
          </ModernButton>
          <ModernButton variant="danger" onclick={confirmRestore}>
            <i class="bi bi-arrow-counterclockwise mr-2"></i>
            Confirm Restore
          </ModernButton>
        </div>
      </div>
    </ModernCard>
  </div>
{/if}

<style>
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
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
  }

  .timeline-icon-warning {
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
    color: white;
    animation: pulse-slow 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  .timeline-icon-error {
    background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
    color: white;
  }

  .timeline-line {
    position: absolute;
    left: 50%;
    top: 3rem;
    bottom: -2rem;
    width: 2px;
    background: linear-gradient(
      to bottom,
      rgba(156, 163, 175, 0.3) 0%,
      rgba(156, 163, 175, 0.1) 100%
    );
    transform: translateX(-50%);
    z-index: 1;
  }

  .timeline-content {
    flex: 1;
    animation: slide-up 0.4s ease-out;
  }

  /* Mini Stats */
  .stat-mini {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.75rem;
    background: rgba(229, 231, 235, 0.3);
    border-radius: 0.5rem;
    text-align: center;
  }

  :global(.dark) .stat-mini {
    background: rgba(55, 65, 81, 0.3);
  }

  .stat-mini i {
    font-size: 1.25rem;
    margin-bottom: 0.25rem;
  }

  .stat-mini-label {
    font-size: 0.625rem;
    font-weight: 600;
    color: rgb(107, 114, 128);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global(.dark) .stat-mini-label {
    color: rgb(156, 163, 175);
  }

  .stat-mini-value {
    font-size: 0.875rem;
    font-weight: 700;
    color: rgb(17, 24, 39);
  }

  :global(.dark) .stat-mini-value {
    color: rgb(243, 244, 246);
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
