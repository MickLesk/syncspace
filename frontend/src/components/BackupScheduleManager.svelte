<script>
  import { onMount } from "svelte";
  import { backupSchedules } from "../lib/api.js";

  let schedules = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let showCreateModal = $state(false);

  // Form state
  let formData = $state({
    name: "",
    cron_expression: "0 2 * * *", // Default: 2am daily
    backup_type: "full",
    destination_type: "local",
    retention_days: 30,
    enabled: true,
  });

  // Cron presets
  const cronPresets = [
    { label: "Daily at 2am", value: "0 2 * * *" },
    { label: "Weekly (Sunday 3am)", value: "0 3 * * 0" },
    { label: "Monthly (1st at 4am)", value: "0 4 1 * *" },
    { label: "Every 6 hours", value: "0 */6 * * *" },
    { label: "Custom", value: "" },
  ];

  onMount(() => {
    loadSchedules();
  });

  async function loadSchedules() {
    loading = true;
    error = null;
    try {
      const response = await fetch("/api/backups/schedules", {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });

      if (!response.ok) throw new Error("Failed to load schedules");

      schedules = await response.json();
    } catch (err) {
      error = err.message;
      console.error("Failed to load schedules:", err);
    } finally {
      loading = false;
    }
  }

  async function createSchedule() {
    try {
      const response = await fetch("/api/backups/schedules", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
        body: JSON.stringify(formData),
      });

      if (!response.ok) throw new Error("Failed to create schedule");

      showCreateModal = false;
      resetForm();
      await loadSchedules();
    } catch (err) {
      error = err.message;
      console.error("Failed to create schedule:", err);
    }
  }

  async function toggleSchedule(scheduleId, currentStatus) {
    try {
      const response = await fetch(`/api/backups/schedules/${scheduleId}`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
        body: JSON.stringify({ enabled: !currentStatus }),
      });

      if (!response.ok) throw new Error("Failed to update schedule");

      await loadSchedules();
    } catch (err) {
      error = err.message;
      console.error("Failed to toggle schedule:", err);
    }
  }

  async function deleteSchedule(scheduleId) {
    if (!confirm("Are you sure you want to delete this schedule?")) return;

    try {
      const response = await fetch(`/api/backups/schedules/${scheduleId}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });

      if (!response.ok) throw new Error("Failed to delete schedule");

      await loadSchedules();
    } catch (err) {
      error = err.message;
      console.error("Failed to delete schedule:", err);
    }
  }

  async function triggerSchedule(scheduleId) {
    try {
      const response = await fetch(
        `/api/backups/schedules/${scheduleId}/trigger`,
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (!response.ok) throw new Error("Failed to trigger backup");

      alert("Backup started successfully!");
    } catch (err) {
      error = err.message;
      console.error("Failed to trigger backup:", err);
    }
  }

  function resetForm() {
    formData = {
      name: "",
      cron_expression: "0 2 * * *",
      backup_type: "full",
      destination_type: "local",
      retention_days: 30,
      enabled: true,
    };
  }

  function formatCronExpression(cron) {
    const presets = {
      "0 2 * * *": "Daily at 2am",
      "0 3 * * 0": "Weekly (Sunday 3am)",
      "0 4 1 * *": "Monthly (1st at 4am)",
      "0 */6 * * *": "Every 6 hours",
    };
    return presets[cron] || cron;
  }

  function formatDate(dateString) {
    if (!dateString) return "Never";
    return new Date(dateString).toLocaleString();
  }
</script>

<div class="backup-schedule-manager p-6">
  <div class="header flex justify-between items-center mb-6">
    <div>
      <h2 class="text-2xl font-bold mb-2">Backup Schedules</h2>
      <p class="text-base-content/70">Manage automated backup schedules</p>
    </div>
    <button class="btn btn-primary" onclick={() => (showCreateModal = true)}>
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
          d="M12 4v16m8-8H4"
        />
      </svg>
      Create Schedule
    </button>
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
    <div class="flex justify-center items-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if schedules.length === 0}
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
          d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <p class="text-base-content/70 mb-4">No backup schedules configured</p>
      <button class="btn btn-primary" onclick={() => (showCreateModal = true)}>
        Create your first schedule
      </button>
    </div>
  {:else}
    <div class="grid gap-4">
      {#each schedules as schedule}
        <div class="card bg-base-200 shadow-md">
          <div class="card-body">
            <div class="flex justify-between items-start">
              <div class="flex-1">
                <h3 class="card-title text-lg mb-2">
                  {schedule.name}
                  <div
                    class="badge badge-sm {schedule.enabled
                      ? 'badge-success'
                      : 'badge-ghost'}"
                  >
                    {schedule.enabled ? "Active" : "Disabled"}
                  </div>
                </h3>

                <div class="grid grid-cols-2 gap-3 text-sm">
                  <div>
                    <span class="text-base-content/70">Schedule:</span>
                    <span class="font-mono ml-2"
                      >{formatCronExpression(schedule.cron_expression)}</span
                    >
                  </div>
                  <div>
                    <span class="text-base-content/70">Type:</span>
                    <span class="badge badge-sm ml-2"
                      >{schedule.backup_type}</span
                    >
                  </div>
                  <div>
                    <span class="text-base-content/70">Destination:</span>
                    <span class="ml-2">{schedule.destination_type}</span>
                  </div>
                  <div>
                    <span class="text-base-content/70">Retention:</span>
                    <span class="ml-2">{schedule.retention_days} days</span>
                  </div>
                  <div class="col-span-2">
                    <span class="text-base-content/70">Next run:</span>
                    <span class="ml-2">{formatDate(schedule.next_run_at)}</span>
                  </div>
                  {#if schedule.last_run_at}
                    <div class="col-span-2">
                      <span class="text-base-content/70">Last run:</span>
                      <span class="ml-2"
                        >{formatDate(schedule.last_run_at)}</span
                      >
                    </div>
                  {/if}
                </div>
              </div>

              <div class="flex gap-2">
                <button
                  class="btn btn-sm btn-ghost"
                  onclick={() => toggleSchedule(schedule.id, schedule.enabled)}
                  title={schedule.enabled ? "Disable" : "Enable"}
                >
                  {#if schedule.enabled}
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
                        d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z"
                      />
                    </svg>
                  {:else}
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
                        d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                      />
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                      />
                    </svg>
                  {/if}
                </button>

                <button
                  class="btn btn-sm btn-primary"
                  onclick={() => triggerSchedule(schedule.id)}
                  title="Run now"
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
                      d="M13 10V3L4 14h7v7l9-11h-7z"
                    />
                  </svg>
                </button>

                <button
                  class="btn btn-sm btn-ghost text-error"
                  onclick={() => deleteSchedule(schedule.id)}
                  title="Delete"
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
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Create Schedule Modal -->
{#if showCreateModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-lg mb-4">Create Backup Schedule</h3>

      <form
        onsubmit={(e) => {
          e.preventDefault();
          createSchedule();
        }}
      >
        <div class="form-control mb-4">
          <label class="label">
            <span class="label-text">Schedule Name</span>
          </label>
          <input
            type="text"
            class="input input-bordered"
            bind:value={formData.name}
            placeholder="e.g., Daily Full Backup"
            required
          />
        </div>

        <div class="form-control mb-4">
          <label class="label">
            <span class="label-text">Schedule</span>
          </label>
          <select
            class="select select-bordered"
            bind:value={formData.cron_expression}
          >
            {#each cronPresets as preset}
              <option value={preset.value}>{preset.label}</option>
            {/each}
          </select>
          {#if formData.cron_expression === ""}
            <input
              type="text"
              class="input input-bordered mt-2"
              bind:value={formData.cron_expression}
              placeholder="* * * * *"
              required
            />
          {/if}
          <label class="label">
            <span class="label-text-alt"
              >Format: minute hour day month weekday</span
            >
          </label>
        </div>

        <div class="grid grid-cols-2 gap-4 mb-4">
          <div class="form-control">
            <label class="label">
              <span class="label-text">Backup Type</span>
            </label>
            <select
              class="select select-bordered"
              bind:value={formData.backup_type}
            >
              <option value="full">Full Backup</option>
              <option value="database">Database Only</option>
              <option value="files">Files Only</option>
            </select>
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text">Destination</span>
            </label>
            <select
              class="select select-bordered"
              bind:value={formData.destination_type}
            >
              <option value="local">Local Storage</option>
              <option value="s3">Amazon S3</option>
              <option value="webdav">WebDAV</option>
            </select>
          </div>
        </div>

        <div class="form-control mb-4">
          <label class="label">
            <span class="label-text">Retention Period (days)</span>
          </label>
          <input
            type="number"
            class="input input-bordered"
            bind:value={formData.retention_days}
            min="1"
            max="365"
            required
          />
          <label class="label">
            <span class="label-text-alt"
              >Backups older than this will be automatically deleted</span
            >
          </label>
        </div>

        <div class="form-control mb-6">
          <label class="label cursor-pointer justify-start gap-4">
            <input
              type="checkbox"
              class="checkbox"
              bind:checked={formData.enabled}
            />
            <span class="label-text">Enable schedule immediately</span>
          </label>
        </div>

        <div class="modal-action">
          <button
            type="button"
            class="btn"
            onclick={() => {
              showCreateModal = false;
              resetForm();
            }}
          >
            Cancel
          </button>
          <button type="submit" class="btn btn-primary">
            Create Schedule
          </button>
        </div>
      </form>
    </div>
    <div
      class="modal-backdrop"
      onclick={() => {
        showCreateModal = false;
        resetForm();
      }}
    ></div>
  </div>
{/if}

<style>
  .backup-schedule-manager {
    max-width: 1200px;
    margin: 0 auto;
  }
</style>
