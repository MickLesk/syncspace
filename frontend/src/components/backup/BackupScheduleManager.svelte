<script>
  import { onMount } from "svelte";
  import { backupSchedules } from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

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
  let cronPresets = $derived([
    { label: tr("dailyAt2am"), value: "0 2 * * *" },
    { label: tr("weeklySunday3am"), value: "0 3 * * 0" },
    { label: tr("monthlyFirstAt4am"), value: "0 4 1 * *" },
    { label: tr("every6hours"), value: "0 */6 * * *" },
    { label: tr("custom"), value: "" },
  ]);

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

<div class="max-w-[1200px] mx-auto p-6">
  <div class="header flex justify-between items-center mb-6">
    <div>
      <h2 class="text-2xl font-bold mb-2 text-gray-900 dark:text-white">
        Backup Schedules
      </h2>
      <p class="text-gray-500 dark:text-gray-400">
        Manage automated backup schedules
      </p>
    </div>
    <button
      class="px-4 py-2 text-sm font-medium text-white bg-green-600 dark:bg-green-500 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors flex items-center gap-2"
      onclick={() => (showCreateModal = true)}
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
          d="M12 4v16m8-8H4"
        />
      </svg>
      Create Schedule
    </button>
  </div>

  {#if error}
    <div
      class="rounded-lg p-4 mb-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 flex items-center gap-3"
    >
      <svg
        class="w-6 h-6 text-red-600 dark:text-red-400"
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
      <span class="text-red-900 dark:text-red-100">{error}</span>
    </div>
  {/if}

  {#if loading}
    <div class="flex justify-center items-center py-12">
      <div
        class="w-12 h-12 border-4 border-green-200 dark:border-green-900 border-t-green-600 dark:border-t-green-400 rounded-full animate-spin"
      ></div>
    </div>
  {:else if schedules.length === 0}
    <div class="text-center py-12">
      <svg
        class="w-16 h-16 mx-auto mb-4 text-gray-300 dark:text-gray-600"
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
      <p class="text-gray-500 dark:text-gray-400 mb-4">
        No backup schedules configured
      </p>
      <button
        class="px-4 py-2 text-sm font-medium text-white bg-green-600 dark:bg-green-500 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors"
        onclick={() => (showCreateModal = true)}
      >
        Create your first schedule
      </button>
    </div>
  {:else}
    <div class="grid gap-4">
      {#each schedules as schedule}
        <div
          class="bg-gray-50 dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700"
        >
          <div class="p-4">
            <div class="flex justify-between items-start">
              <div class="flex-1">
                <h3
                  class="text-lg font-bold mb-2 flex items-center gap-2 text-gray-900 dark:text-white"
                >
                  {schedule.name}
                  <div
                    class="px-2 py-0.5 text-xs font-medium rounded-full {schedule.enabled
                      ? 'bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200'
                      : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'}"
                  >
                    {schedule.enabled ? "Active" : "Disabled"}
                  </div>
                </h3>

                <div class="grid grid-cols-2 gap-3 text-sm">
                  <div>
                    <span class="text-gray-500 dark:text-gray-400"
                      >Schedule:</span
                    >
                    <span class="font-mono ml-2 text-gray-900 dark:text-white"
                      >{formatCronExpression(schedule.cron_expression)}</span
                    >
                  </div>
                  <div>
                    <span class="text-gray-500 dark:text-gray-400">Type:</span>
                    <span
                      class="px-1.5 py-0.5 text-xs font-medium bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200 rounded ml-2"
                      >{schedule.backup_type}</span
                    >
                  </div>
                  <div>
                    <span class="text-gray-500 dark:text-gray-400"
                      >Destination:</span
                    >
                    <span class="ml-2 text-gray-900 dark:text-white"
                      >{schedule.destination_type}</span
                    >
                  </div>
                  <div>
                    <span class="text-gray-500 dark:text-gray-400"
                      >Retention:</span
                    >
                    <span class="ml-2 text-gray-900 dark:text-white"
                      >{schedule.retention_days} days</span
                    >
                  </div>
                  <div class="col-span-2">
                    <span class="text-gray-500 dark:text-gray-400"
                      >Next run:</span
                    >
                    <span class="ml-2 text-gray-900 dark:text-white"
                      >{formatDate(schedule.next_run_at)}</span
                    >
                  </div>
                  {#if schedule.last_run_at}
                    <div class="col-span-2">
                      <span class="text-gray-500 dark:text-gray-400"
                        >Last run:</span
                      >
                      <span class="ml-2 text-gray-900 dark:text-white"
                        >{formatDate(schedule.last_run_at)}</span
                      >
                    </div>
                  {/if}
                </div>
              </div>

              <div class="flex gap-2">
                <button
                  class="px-2 py-1 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200 transition-colors"
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
                  class="px-2 py-1 text-sm rounded-lg bg-green-600 dark:bg-green-500 text-white hover:bg-green-700 dark:hover:bg-green-600 transition-colors"
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
                  class="px-2 py-1 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-red-600 dark:text-red-400 transition-colors"
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
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-lg shadow-xl max-w-2xl w-full border border-gray-200 dark:border-gray-700 max-h-[90vh] overflow-y-auto"
    >
      <div class="p-6">
        <h3 class="font-bold text-lg mb-4 text-gray-900 dark:text-white">
          Create Backup Schedule
        </h3>

        <form
          onsubmit={(e) => {
            e.preventDefault();
            createSchedule();
          }}
        >
          <div class="mb-4">
            <label class="block mb-2">
              <span class="text-sm font-medium text-gray-700 dark:text-gray-200"
                >Schedule Name</span
              >
              <input
                type="text"
                class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent"
                bind:value={formData.name}
                placeholder="e.g., Daily Full Backup"
                required
              />
            </label>
          </div>

          <div class="mb-4">
            <label class="block mb-2">
              <span class="text-sm font-medium text-gray-700 dark:text-gray-200"
                >Schedule</span
              >
              <select
                class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent"
                bind:value={formData.cron_expression}
              >
                {#each cronPresets as preset}
                  <option value={preset.value}>{preset.label}</option>
                {/each}
              </select>
            </label>
            {#if formData.cron_expression === ""}
              <input
                type="text"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent mt-2"
                bind:value={formData.cron_expression}
                placeholder="* * * * *"
                required
              />
            {/if}
            <div class="mt-1">
              <span class="text-xs text-gray-500 dark:text-gray-400"
                >Format: minute hour day month weekday</span
              >
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4 mb-4">
            <div>
              <label class="block mb-2">
                <span
                  class="text-sm font-medium text-gray-700 dark:text-gray-200"
                  >Backup Type</span
                >
                <select
                  class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent"
                  bind:value={formData.backup_type}
                >
                  <option value="full">Full Backup</option>
                  <option value="database">Database Only</option>
                  <option value="files">Files Only</option>
                </select>
              </label>
            </div>

            <div>
              <label class="block mb-2">
                <span
                  class="text-sm font-medium text-gray-700 dark:text-gray-200"
                  >Destination</span
                >
                <select
                  class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent"
                  bind:value={formData.destination_type}
                >
                  <option value="local">Local Storage</option>
                  <option value="s3">Amazon S3</option>
                  <option value="webdav">WebDAV</option>
                </select>
              </label>
            </div>
          </div>

          <div class="mb-4">
            <label class="block mb-2">
              <span class="text-sm font-medium text-gray-700 dark:text-gray-200"
                >Retention Period (days)</span
              >
              <input
                type="number"
                class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent"
                bind:value={formData.retention_days}
                min="1"
                max="365"
                required
              />
            </label>
            <div class="mt-1">
              <span class="text-xs text-gray-500 dark:text-gray-400"
                >Backups older than this will be automatically deleted</span
              >
            </div>
          </div>

          <div class="mb-6">
            <label class="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                class="w-4 h-4 text-green-600 bg-gray-100 border-gray-300 rounded focus:ring-green-500 dark:focus:ring-green-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                bind:checked={formData.enabled}
              />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-200"
                >Enable schedule immediately</span
              >
            </label>
          </div>

          <div class="flex justify-end gap-2">
            <button
              type="button"
              class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
              onclick={() => {
                showCreateModal = false;
                resetForm();
              }}
            >
              Cancel
            </button>
            <button
              type="submit"
              class="px-4 py-2 text-sm font-medium text-white bg-green-600 dark:bg-green-500 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors"
            >
              Create Schedule
            </button>
          </div>
        </form>
      </div>
    </div>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="fixed inset-0 -z-10"
      role="presentation"
      onclick={() => {
        showCreateModal = false;
        resetForm();
      }}
    ></div>
  </div>
{/if}
