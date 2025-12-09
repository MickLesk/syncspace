<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { t } from "../../i18n.js";

  let schedules = $state([]);
  let backups = $state([]);
  let stats = $state(null);
  let loading = $state(false);
  let error = $state(null);

  let showCreateModal = $state(false);
  let showRestoreModal = $state(false);
  let selectedBackup = $state(null);

  let ws = null;

  // Create Schedule Form
  let scheduleForm = $state({
    name: "",
    description: "",
    frequency: "daily",
    schedule_time: "02:00",
    day_of_week: 0,
    day_of_month: 1,
    backup_type: "full",
    include_paths: ["/"],
    exclude_patterns: [],
    enable_encryption: false,
    compression_level: 6,
    backup_path: "./data/backups",
    max_backups: 10,
    retention_days: 30,
  });

  // Restore Form
  let restoreForm = $state({
    restore_type: "full",
    source_paths: [],
    destination_path: "./data/restore",
    overwrite_existing: false,
  });

  onMount(() => {
    loadData();
  });

  async function loadData() {
    await Promise.all([loadSchedules(), loadBackups(), loadStats()]);
  }

  async function loadSchedules() {
    try {
      const response = await api.backup.listSchedules();
      schedules = response;
    } catch (err) {
      console.error("Failed to load schedules:", err);
    }
  }

  async function loadBackups() {
    try {
      const response = await api.backup.listJobs();
      backups = response;
    } catch (err) {
      console.error("Failed to load backups:", err);
    }
  }

  async function loadStats() {
    try {
      const response = await api.backup.getStats();
      stats = response;
    } catch (err) {
      console.error("Failed to load stats:", err);
    }
  }

  async function createSchedule() {
    try {
      loading = true;
      error = null;

      await api.backup.createSchedule(scheduleForm);

      showCreateModal = false;
      resetScheduleForm();
      await loadSchedules();
    } catch (err) {
      error = err.message || "Failed to create schedule";
    } finally {
      loading = false;
    }
  }

  async function deleteSchedule(scheduleId) {
    if (!confirm($t("backup.confirmDeleteSchedule"))) return;

    try {
      await api.backup.deleteSchedule(scheduleId);
      await loadSchedules();
    } catch (err) {
      alert("Failed to delete schedule: " + err.message);
    }
  }

  async function toggleSchedule(scheduleId, currentActive) {
    try {
      if (currentActive) {
        await api.backup.disableSchedule(scheduleId);
      } else {
        await api.backup.enableSchedule(scheduleId);
      }
      await loadSchedules();
    } catch (err) {
      alert("Failed to toggle schedule: " + err.message);
    }
  }

  async function triggerManualBackup() {
    try {
      loading = true;
      error = null;

      const response = await api.backup.triggerBackup({
        backup_type: "full",
        include_paths: ["/"],
      });

      alert($t("backup.backupStarted") + ": " + response.job_id);
      await loadBackups();
    } catch (err) {
      error = err.message || "Failed to start backup";
    } finally {
      loading = false;
    }
  }

  async function startRestore() {
    if (!selectedBackup) return;

    try {
      loading = true;
      error = null;

      const response = await api.backup.restore({
        backup_job_id: selectedBackup.id,
        ...restoreForm,
      });

      showRestoreModal = false;
      alert($t("backup.restoreStarted") + ": " + response.restore_id);
      resetRestoreForm();
    } catch (err) {
      error = err.message || "Failed to start restore";
    } finally {
      loading = false;
    }
  }

  async function cancelBackup(jobId) {
    try {
      await api.backup.cancelJob(jobId);
      await loadBackups();
    } catch (err) {
      alert("Failed to cancel backup: " + err.message);
    }
  }

  async function cleanupOldBackups() {
    if (!confirm($t("backup.confirmCleanup"))) return;

    try {
      loading = true;
      const response = await api.backup.cleanup();
      alert(response.message);
      await loadBackups();
    } catch (err) {
      alert("Failed to cleanup: " + err.message);
    } finally {
      loading = false;
    }
  }

  function updateBackupProgress(data) {
    const index = backups.findIndex((b) => b.id === data.job_id);
    if (index !== -1) {
      backups[index] = { ...backups[index], ...data };
    }
  }

  function openRestoreModal(backup) {
    selectedBackup = backup;
    showRestoreModal = true;
  }

  function resetScheduleForm() {
    scheduleForm = {
      name: "",
      description: "",
      frequency: "daily",
      schedule_time: "02:00",
      day_of_week: 0,
      day_of_month: 1,
      backup_type: "full",
      include_paths: ["/"],
      exclude_patterns: [],
      enable_encryption: false,
      compression_level: 6,
      backup_path: "./data/backups",
      max_backups: 10,
      retention_days: 30,
    };
  }

  function resetRestoreForm() {
    restoreForm = {
      restore_type: "full",
      source_paths: [],
      destination_path: "./data/restore",
      overwrite_existing: false,
    };
    selectedBackup = null;
  }

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    const date = new Date(dateStr);
    return date.toLocaleString();
  }

  function getStatusClass(status) {
    const statusMap = {
      completed: "badge-success",
      running: "badge-info",
      failed: "badge-error",
      pending: "badge-warning",
      cancelled: "badge-ghost",
    };
    return statusMap[status] || "badge-ghost";
  }

  function getProgressPercent(backup) {
    if (!backup.total_files || backup.total_files === 0) return 0;
    return Math.round((backup.processed_files / backup.total_files) * 100);
  }
</script>

<div class="container mx-auto p-6 space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold">{$t("backup.title")}</h1>
      <p class="text-base-content/70 mt-1">{$t("backup.subtitle")}</p>
    </div>
    <div class="flex gap-2">
      <button class="btn btn-outline" onclick={() => cleanupOldBackups()}>
        <i class="bi bi-trash" aria-hidden="true"></i>
        {$t("backup.cleanup")}
      </button>
      <button class="btn btn-primary" onclick={() => (showCreateModal = true)}>
        <i class="bi bi-plus-lg" aria-hidden="true"></i>
        {$t("backup.createSchedule")}
      </button>
      <button
        class="btn btn-success"
        onclick={triggerManualBackup}
        disabled={loading}
      >
        <i class="bi bi-play-fill" aria-hidden="true"></i>
        {$t("backup.runNow")}
      </button>
    </div>
  </div>

  <!-- Error Alert -->
  {#if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
      <span>{error}</span>
      <button
        class="btn btn-sm btn-ghost"
        onclick={() => (error = null)}
        aria-label="Close"
      >
        <i class="bi bi-x-lg" aria-hidden="true"></i>
      </button>
    </div>
  {/if}

  <!-- Stats Cards -->
  {#if stats}
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-primary">
            <i class="bi bi-calendar3 text-3xl" aria-hidden="true"></i>
          </div>
          <div class="stat-title">{$t("backup.totalSchedules")}</div>
          <div class="stat-value text-primary">{stats.total_schedules}</div>
          <div class="stat-desc">
            {stats.active_schedules}
            {$t("backup.active")}
          </div>
        </div>
      </div>

      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-success">
            <i class="bi bi-check-circle text-3xl" aria-hidden="true"></i>
          </div>
          <div class="stat-title">{$t("backup.successfulBackups")}</div>
          <div class="stat-value text-success">{stats.successful_backups}</div>
          <div class="stat-desc">
            {$t("backup.of")}
            {stats.total_backups}
            {$t("backup.total")}
          </div>
        </div>
      </div>

      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-error">
            <i class="bi bi-x-circle text-3xl" aria-hidden="true"></i>
          </div>
          <div class="stat-title">{$t("backup.failedBackups")}</div>
          <div class="stat-value text-error">{stats.failed_backups}</div>
        </div>
      </div>

      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-info">
            <i class="bi bi-hdd text-3xl" aria-hidden="true"></i>
          </div>
          <div class="stat-title">{$t("backup.totalSize")}</div>
          <div class="stat-value text-info text-2xl">
            {formatBytes(stats.total_backup_size)}
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Schedules Section -->
  <div class="card bg-base-100 shadow-lg">
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-calendar-check" aria-hidden="true"></i>
        {$t("backup.schedules")}
      </h2>

      {#if schedules.length === 0}
        <div class="text-center py-12">
          <i
            class="bi bi-calendar-x text-6xl text-base-content/20"
            aria-hidden="true"
          ></i>
          <p class="mt-4 text-base-content/60">{$t("backup.noSchedules")}</p>
        </div>
      {:else}
        <div class="overflow-x-auto">
          <table class="table table-zebra">
            <thead>
              <tr>
                <th>{$t("backup.name")}</th>
                <th>{$t("backup.frequency")}</th>
                <th>{$t("backup.type")}</th>
                <th>{$t("backup.nextRun")}</th>
                <th>{$t("backup.lastRun")}</th>
                <th>{$t("backup.status")}</th>
                <th>{$t("backup.actions")}</th>
              </tr>
            </thead>
            <tbody>
              {#each schedules as schedule}
                <tr>
                  <td>
                    <div class="font-semibold">{schedule.name}</div>
                    {#if schedule.description}
                      <div class="text-sm text-base-content/60">
                        {schedule.description}
                      </div>
                    {/if}
                  </td>
                  <td>
                    <span class="badge badge-outline">{schedule.frequency}</span
                    >
                    {#if schedule.schedule_time}
                      <div class="text-xs text-base-content/60">
                        {schedule.schedule_time}
                      </div>
                    {/if}
                  </td>
                  <td>
                    <span
                      class="badge {schedule.backup_type === 'full'
                        ? 'badge-primary'
                        : 'badge-secondary'}"
                    >
                      {schedule.backup_type}
                    </span>
                  </td>
                  <td class="text-sm">{formatDate(schedule.next_run_at)}</td>
                  <td class="text-sm">{formatDate(schedule.last_run_at)}</td>
                  <td>
                    {#if schedule.last_status}
                      <span
                        class="badge {getStatusClass(schedule.last_status)}"
                      >
                        {schedule.last_status}
                      </span>
                    {:else}
                      <span class="badge badge-ghost">-</span>
                    {/if}
                  </td>
                  <td>
                    <div class="flex gap-1">
                      <button
                        class="btn btn-sm {schedule.is_active
                          ? 'btn-warning'
                          : 'btn-success'}"
                        onclick={() =>
                          toggleSchedule(schedule.id, schedule.is_active)}
                        title={schedule.is_active
                          ? $t("backup.disable")
                          : $t("backup.enable")}
                      >
                        <i
                          class="bi {schedule.is_active
                            ? 'bi-pause-fill'
                            : 'bi-play-fill'}"
                        ></i>
                      </button>
                      <button
                        class="btn btn-sm btn-error btn-outline"
                        onclick={() => deleteSchedule(schedule.id)}
                        title={$t("backup.delete")}
                        aria-label="Delete"
                      >
                        <i class="bi bi-trash" aria-hidden="true"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>

  <!-- Backup History Section -->
  <div class="card bg-base-100 shadow-lg">
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-clock-history" aria-hidden="true"></i>
        {$t("backup.history")}
      </h2>

      {#if backups.length === 0}
        <div class="text-center py-12">
          <i
            class="bi bi-inbox text-6xl text-base-content/20"
            aria-hidden="true"
          ></i>
          <p class="mt-4 text-base-content/60">{$t("backup.noBackups")}</p>
        </div>
      {:else}
        <div class="overflow-x-auto">
          <table class="table table-zebra">
            <thead>
              <tr>
                <th>{$t("backup.id")}</th>
                <th>{$t("backup.type")}</th>
                <th>{$t("backup.files")}</th>
                <th>{$t("backup.size")}</th>
                <th>{$t("backup.progress")}</th>
                <th>{$t("backup.started")}</th>
                <th>{$t("backup.duration")}</th>
                <th>{$t("backup.status")}</th>
                <th>{$t("backup.actions")}</th>
              </tr>
            </thead>
            <tbody>
              {#each backups as backup}
                <tr>
                  <td class="font-mono text-xs">{backup.id.substring(0, 8)}</td>
                  <td>
                    <span class="badge badge-outline">{backup.job_type}</span>
                  </td>
                  <td>
                    {backup.processed_files} / {backup.total_files}
                  </td>
                  <td>
                    {#if backup.backup_size}
                      <div>{formatBytes(backup.backup_size)}</div>
                      {#if backup.compressed_bytes}
                        <div class="text-xs text-success">
                          {Math.round(
                            (1 - backup.compressed_bytes / backup.total_bytes) *
                              100
                          )}% {$t("backup.compressed")}
                        </div>
                      {/if}
                    {:else}
                      {formatBytes(backup.processed_bytes)}
                    {/if}
                  </td>
                  <td>
                    {#if backup.status === "running"}
                      <div class="flex items-center gap-2">
                        <progress
                          class="progress progress-primary w-20"
                          value={getProgressPercent(backup)}
                          max="100"
                        ></progress>
                        <span class="text-sm"
                          >{getProgressPercent(backup)}%</span
                        >
                      </div>
                    {:else}
                      <span class="text-base-content/60">-</span>
                    {/if}
                  </td>
                  <td class="text-sm">{formatDate(backup.started_at)}</td>
                  <td class="text-sm">
                    {#if backup.duration_seconds}
                      {Math.floor(backup.duration_seconds / 60)}m {backup.duration_seconds %
                        60}s
                    {:else if backup.status === "running"}
                      <span class="loading loading-spinner loading-xs"></span>
                    {:else}
                      -
                    {/if}
                  </td>
                  <td>
                    <span class="badge {getStatusClass(backup.status)}">
                      {backup.status}
                    </span>
                    {#if backup.error_message}
                      <div class="tooltip" data-tip={backup.error_message}>
                        <i
                          class="bi bi-exclamation-circle text-error"
                          aria-hidden="true"
                        ></i>
                      </div>
                    {/if}
                  </td>
                  <td>
                    <div class="flex gap-1">
                      {#if backup.status === "completed"}
                        <button
                          class="btn btn-sm btn-primary"
                          onclick={() => openRestoreModal(backup)}
                          title={$t("backup.restore")}
                        >
                          <i
                            class="bi bi-arrow-counterclockwise"
                            aria-hidden="true"
                          ></i>
                        </button>
                      {/if}
                      {#if backup.status === "running"}
                        <button
                          class="btn btn-sm btn-warning"
                          onclick={() => cancelBackup(backup.id)}
                          title={$t("backup.cancel")}
                          aria-label="Cancel"
                        >
                          <i class="bi bi-x-lg" aria-hidden="true"></i>
                        </button>
                      {/if}
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>
</div>

<!-- Create Schedule Modal -->
{#if showCreateModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-calendar-plus" aria-hidden="true"></i>
        {$t("backup.createSchedule")}
      </h3>

      <form
        onsubmit={(e) => {
          e.preventDefault();
          createSchedule();
        }}
        class="space-y-4"
      >
        <div class="form-control">
          <div class="label"><span>{$t("backup.scheduleName")}</span></div>
          <input
            type="text"
            class="input input-bordered"
            bind:value={scheduleForm.name}
            required
          />
        </div>

        <div class="form-control">
          <div class="label"><span>{$t("backup.description")}</span></div>
          <textarea
            class="textarea textarea-bordered"
            bind:value={scheduleForm.description}
            rows="2"
          ></textarea>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="form-control">
            <div class="label"><span>{$t("backup.frequency")}</span></div>
            <select
              class="select select-bordered"
              bind:value={scheduleForm.frequency}
            >
              <option value="manual">{$t("backup.manual")}</option>
              <option value="hourly">{$t("backup.hourly")}</option>
              <option value="daily">{$t("backup.daily")}</option>
              <option value="weekly">{$t("backup.weekly")}</option>
              <option value="monthly">{$t("backup.monthly")}</option>
            </select>
          </div>

          <div class="form-control">
            <div class="label"><span>{$t("backup.type")}</span></div>
            <select
              class="select select-bordered"
              bind:value={scheduleForm.backup_type}
            >
              <option value="full">{$t("backup.full")}</option>
              <option value="incremental">{$t("backup.incremental")}</option>
              <option value="differential">{$t("backup.differential")}</option>
            </select>
          </div>
        </div>

        {#if scheduleForm.frequency === "daily"}
          <div class="form-control">
            <div class="label"><span>{$t("backup.time")}</span></div>
            <input
              type="time"
              class="input input-bordered"
              bind:value={scheduleForm.schedule_time}
            />
          </div>
        {/if}

        {#if scheduleForm.frequency === "weekly"}
          <div class="form-control">
            <div class="label"><span>{$t("backup.dayOfWeek")}</span></div>
            <select
              class="select select-bordered"
              bind:value={scheduleForm.day_of_week}
            >
              <option value="0">{$t("backup.sunday")}</option>
              <option value="1">{$t("backup.monday")}</option>
              <option value="2">{$t("backup.tuesday")}</option>
              <option value="3">{$t("backup.wednesday")}</option>
              <option value="4">{$t("backup.thursday")}</option>
              <option value="5">{$t("backup.friday")}</option>
              <option value="6">{$t("backup.saturday")}</option>
            </select>
          </div>
        {/if}

        <div class="grid grid-cols-3 gap-4">
          <div class="form-control">
            <div class="label">
              <span>{$t("backup.compressionLevel")}</span>
            </div>
            <input
              type="number"
              min="0"
              max="9"
              class="input input-bordered"
              bind:value={scheduleForm.compression_level}
            />
          </div>

          <div class="form-control">
            <div class="label"><span>{$t("backup.maxBackups")}</span></div>
            <input
              type="number"
              min="1"
              class="input input-bordered"
              bind:value={scheduleForm.max_backups}
            />
          </div>

          <div class="form-control">
            <div class="label"><span>{$t("backup.retentionDays")}</span></div>
            <input
              type="number"
              min="1"
              class="input input-bordered"
              bind:value={scheduleForm.retention_days}
            />
          </div>
        </div>

        <div class="form-control">
          <label class="label cursor-pointer">
            <span class="label-text">{$t("backup.enableEncryption")}</span>
            <input
              type="checkbox"
              class="checkbox"
              bind:checked={scheduleForm.enable_encryption}
            />
          </label>
        </div>

        <div class="modal-action">
          <button
            type="button"
            class="btn"
            onclick={() => {
              showCreateModal = false;
              resetScheduleForm();
            }}
          >
            {$t("common.cancel")}
          </button>
          <button type="submit" class="btn btn-primary" disabled={loading}>
            {#if loading}
              <span class="loading loading-spinner loading-sm"></span>
            {/if}
            {$t("common.create")}
          </button>
        </div>
      </form>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button
        onclick={() => {
          showCreateModal = false;
          resetScheduleForm();
        }}>close</button
      >
    </form>
  </div>
{/if}

<!-- Restore Modal -->
{#if showRestoreModal && selectedBackup}
  <div class="modal modal-open">
    <div class="modal-box max-w-xl">
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-arrow-counterclockwise" aria-hidden="true"></i>
        {$t("backup.restoreBackup")}
      </h3>

      <div class="mb-4 p-4 bg-base-200 rounded-lg">
        <div class="text-sm space-y-1">
          <div>
            <strong>{$t("backup.backupId")}:</strong>
            {selectedBackup.id.substring(0, 8)}
          </div>
          <div>
            <strong>{$t("backup.created")}:</strong>
            {formatDate(selectedBackup.created_at)}
          </div>
          <div>
            <strong>{$t("backup.size")}:</strong>
            {formatBytes(selectedBackup.backup_size)}
          </div>
          <div>
            <strong>{$t("backup.files")}:</strong>
            {selectedBackup.total_files}
          </div>
        </div>
      </div>

      <form
        onsubmit={(e) => {
          e.preventDefault();
          startRestore();
        }}
        class="space-y-4"
      >
        <div class="form-control">
          <div class="label"><span>{$t("backup.restoreType")}</span></div>
          <select
            class="select select-bordered"
            bind:value={restoreForm.restore_type}
          >
            <option value="full">{$t("backup.fullRestore")}</option>
            <option value="selective">{$t("backup.selectiveRestore")}</option>
          </select>
        </div>

        <div class="form-control">
          <div class="label"><span>{$t("backup.destinationPath")}</span></div>
          <input
            type="text"
            class="input input-bordered"
            bind:value={restoreForm.destination_path}
            required
          />
        </div>

        <div class="form-control">
          <label class="label cursor-pointer">
            <span class="label-text">{$t("backup.overwriteExisting")}</span>
            <input
              type="checkbox"
              class="checkbox"
              bind:checked={restoreForm.overwrite_existing}
            />
          </label>
        </div>

        <div class="alert alert-warning">
          <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
          <span>{$t("backup.restoreWarning")}</span>
        </div>

        <div class="modal-action">
          <button
            type="button"
            class="btn"
            onclick={() => {
              showRestoreModal = false;
              resetRestoreForm();
            }}
          >
            {$t("common.cancel")}
          </button>
          <button type="submit" class="btn btn-primary" disabled={loading}>
            {#if loading}
              <span class="loading loading-spinner loading-sm"></span>
            {/if}
            {$t("backup.startRestore")}
          </button>
        </div>
      </form>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button
        onclick={() => {
          showRestoreModal = false;
          resetRestoreForm();
        }}>close</button
      >
    </form>
  </div>
{/if}
