<script>
  import { onMount, onDestroy } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import * as api from "../lib/api.js";
  import { modals, modalEvents } from "../stores/modals.js";

  // Import createWebSocket from api
  const { createWebSocket } = api;

  // ========================================================================
  // State
  // ========================================================================

  let stats = $state({
    pending: 0,
    running: 0,
    completed: 0,
    failed: 0,
    total: 0,
  });

  let activeJobs = $state([]);
  let jobHistory = $state([]);
  let cronJobs = $state([]);

  let loading = $state(true);
  let error = $state(null);
  let selectedTab = $state("overview"); // overview, active, history, cron

  // Pagination
  let currentPage = $state(1);
  let pageSize = $state(20);

  // WebSocket
  let ws = null;
  let unsubscribe;

  // ========================================================================
  // Job Type Display Names
  // ========================================================================

  const JOB_TYPE_NAMES = {
    SearchIndexing: "Search Indexing",
    ThumbnailGeneration: "Thumbnail Generation",
    FileCleanup: "File Cleanup",
    BackupTask: "Backup Task",
    EmailNotification: "Email Notification",
    WebhookDelivery: "Webhook Delivery",
    FileCompression: "File Compression",
    VirusScan: "Virus Scan",
  };

  // ========================================================================
  // Lifecycle
  // ========================================================================

  onMount(async () => {
    await loadData();

    // Setup WebSocket for real-time updates
    ws = createWebSocket();
    if (ws) {
      ws.addEventListener("message", handleWebSocketMessage);
    }

    // Listen for modal save events
    unsubscribe = modalEvents.on("cronJobSaved", async () => {
      await loadData();
    });

    // Refresh every 10 seconds
    const interval = setInterval(loadData, 10000);

    return () => {
      clearInterval(interval);
      if (ws) {
        ws.removeEventListener("message", handleWebSocketMessage);
      }
    };
  });

  onDestroy(() => {
    if (ws) {
      ws.removeEventListener("message", handleWebSocketMessage);
    }
    if (unsubscribe) unsubscribe();
  });

  // ========================================================================
  // Data Loading
  // ========================================================================

  async function loadData() {
    try {
      loading = true;
      error = null;

      // Load stats
      const statsRes = await api.jobs.stats();
      if (statsRes.ok) {
        stats = await statsRes.json();
      }

      // Load active jobs
      const activeRes = await api.jobs.list({ status: "Running", limit: 50 });
      if (activeRes.ok) {
        const data = await activeRes.json();
        activeJobs = data.jobs || [];
      }

      // Load job history
      const historyRes = await api.jobs.list({
        limit: pageSize,
        offset: (currentPage - 1) * pageSize,
      });
      if (historyRes.ok) {
        const data = await historyRes.json();
        jobHistory = data.jobs || [];
      }

      // Load cron jobs
      const cronRes = await api.cron.list();
      if (cronRes.ok) {
        cronJobs = await cronRes.json();
      }
    } catch (err) {
      console.error("Failed to load dashboard data:", err);
      error = err.message;
    } finally {
      loading = false;
    }
  }

  // ========================================================================
  // WebSocket Handler
  // ========================================================================

  function handleWebSocketMessage(event) {
    try {
      const data = JSON.parse(event.data);

      // Reload data on job changes
      if (data.kind === "JobUpdate" || data.kind === "JobComplete") {
        loadData();
      }
    } catch (err) {
      console.error("WebSocket message error:", err);
    }
  }

  // ========================================================================
  // Actions
  // ========================================================================

  async function cancelJob(jobId) {
    if (!confirm("Cancel this job?")) return;

    try {
      const res = await api.jobs.cancel(jobId);
      if (res.ok) {
        await loadData();
      } else {
        alert("Failed to cancel job");
      }
    } catch (err) {
      console.error("Failed to cancel job:", err);
      alert("Failed to cancel job");
    }
  }

  async function cleanupOldJobs() {
    if (!confirm("Remove old completed/failed jobs (older than 30 days)?"))
      return;

    try {
      const res = await api.jobs.cleanup();
      if (res.ok) {
        await loadData();
        alert("Cleanup completed");
      } else {
        alert("Failed to cleanup jobs");
      }
    } catch (err) {
      console.error("Failed to cleanup jobs:", err);
      alert("Failed to cleanup jobs");
    }
  }

  function openCreateCronJob() {
    modals.openCronJobEditor(null);
  }

  async function toggleCronJob(cronJob) {
    try {
      const res = cronJob.enabled
        ? await api.cron.disable(cronJob.id)
        : await api.cron.enable(cronJob.id);

      if (res.ok) {
        await loadData();
      } else {
        alert("Failed to toggle cron job");
      }
    } catch (err) {
      console.error("Failed to toggle cron job:", err);
      alert("Failed to toggle cron job");
    }
  }

  async function deleteCronJob(cronJobId) {
    if (!confirm("Delete this cron job?")) return;

    try {
      const res = await api.cron.delete(cronJobId);

      if (res.ok) {
        await loadData();
      } else {
        alert("Failed to delete cron job");
      }
    } catch (err) {
      console.error("Failed to delete cron job:", err);
      alert("Failed to delete cron job");
    }
  }

  // ========================================================================
  // Utilities
  // ========================================================================

  function formatDate(dateStr) {
    if (!dateStr) return "N/A";
    return new Date(dateStr).toLocaleString();
  }

  function formatDuration(ms) {
    if (!ms) return "N/A";
    if (ms < 1000) return `${ms}ms`;
    return `${(ms / 1000).toFixed(2)}s`;
  }

  function getStatusBadgeClass(status) {
    switch (status) {
      case "Pending":
        return "badge-info";
      case "Running":
        return "badge-warning";
      case "Completed":
        return "badge-success";
      case "Failed":
        return "badge-error";
      case "Cancelled":
        return "badge-ghost";
      default:
        return "badge-neutral";
    }
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-base-100 to-base-200 p-6">
  <div class="max-w-7xl mx-auto">
    <!-- Modern Header with Glass Effect -->
    <div
      class="mb-8 bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl rounded-2xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all p-8"
    >
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-4 bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl shadow-lg"
        >
          <i
            class="bi bi-gear-wide-connected text-3xl text-white"
            aria-hidden="true"
          ></i>
        </div>
        <div>
          <h1
            class="text-4xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent"
          >
            Background Jobs Dashboard
          </h1>
          <p class="text-base-content/60 mt-1 text-lg">
            Monitor and manage background jobs, cron schedules, and system tasks
          </p>
        </div>
      </div>
    </div>

    <!-- Error Alert -->
    {#if error}
      <div class="alert alert-error mb-6">
        <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
        <span>Error: {error}</span>
      </div>
    {/if}

    <!-- Stats Cards with Modern Design -->
    {#if !loading}
      <div class="grid grid-cols-1 md:grid-cols-5 gap-6 mb-8">
        <!-- Pending Card -->
        <div
          class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-cyan-500/10 to-cyan-600/10 border border-cyan-500/20 p-6 hover:scale-105 transition-transform duration-200 shadow-xl"
        >
          <div class="flex items-center justify-between mb-2">
            <i
              class="bi bi-clock-history text-3xl text-cyan-500"
              aria-hidden="true"
            ></i>
            <span
              class="text-xs font-semibold text-cyan-600 dark:text-cyan-400 bg-cyan-500/20 px-3 py-1 rounded-full"
              >{tr("pending")}</span
            >
          </div>
          <div class="text-4xl font-bold text-cyan-600 dark:text-cyan-400 mb-1">
            {stats.pending}
          </div>
          <div class="text-sm text-base-content/60">
            {tr("jobs.waitingToStart")}
          </div>
        </div>

        <!-- Running Card -->
        <div
          class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-orange-500/10 to-orange-600/10 border border-orange-500/20 p-6 hover:scale-105 transition-transform duration-200 shadow-xl"
        >
          <div class="flex items-center justify-between mb-2">
            <i
              class="bi bi-arrow-repeat text-3xl text-orange-500 animate-spin"
              aria-hidden="true"
            ></i>
            <span
              class="text-xs font-semibold text-orange-600 dark:text-orange-400 bg-orange-500/20 px-3 py-1 rounded-full"
              >{tr("jobs.running")}</span
            >
          </div>
          <div
            class="text-4xl font-bold text-orange-600 dark:text-orange-400 mb-1"
          >
            {stats.running}
          </div>
          <div class="text-sm text-base-content/60">
            {tr("jobs.currentlyActive")}
          </div>
        </div>

        <!-- Completed Card -->
        <div
          class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-green-500/10 to-green-600/10 border border-green-500/20 p-6 hover:scale-105 transition-transform duration-200 shadow-xl"
        >
          <div class="flex items-center justify-between mb-2">
            <i
              class="bi bi-check-circle-fill text-3xl text-green-500"
              aria-hidden="true"
            ></i>
            <span
              class="text-xs font-semibold text-green-600 dark:text-green-400 bg-green-500/20 px-3 py-1 rounded-full"
              >{tr("success")}</span
            >
          </div>
          <div
            class="text-4xl font-bold text-green-600 dark:text-green-400 mb-1"
          >
            {stats.completed}
          </div>
          <div class="text-sm text-base-content/60">
            {tr("jobs.finishedSuccessfully")}
          </div>
        </div>

        <!-- Failed Card -->
        <div
          class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-red-500/10 to-red-600/10 border border-red-500/20 p-6 hover:scale-105 transition-transform duration-200 shadow-xl"
        >
          <div class="flex items-center justify-between mb-2">
            <i
              class="bi bi-exclamation-triangle-fill text-3xl text-red-500"
              aria-hidden="true"
            ></i>
            <span
              class="text-xs font-semibold text-red-600 dark:text-red-400 bg-red-500/20 px-3 py-1 rounded-full"
              >{tr("failed")}</span
            >
          </div>
          <div class="text-4xl font-bold text-red-600 dark:text-red-400 mb-1">
            {stats.failed}
          </div>
          <div class="text-sm text-base-content/60">
            {tr("jobs.errorsOccurred")}
          </div>
        </div>

        <!-- Total Card -->
        <div
          class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-purple-500/10 to-purple-600/10 border border-purple-500/20 p-6 hover:scale-105 transition-transform duration-200 shadow-xl"
        >
          <div class="flex items-center justify-between mb-2">
            <i
              class="bi bi-layers-fill text-3xl text-purple-500"
              aria-hidden="true"
            ></i>
            <span
              class="text-xs font-semibold text-purple-600 dark:text-purple-400 bg-purple-500/20 px-3 py-1 rounded-full"
              >{tr("jobs.total")}</span
            >
          </div>
          <div
            class="text-4xl font-bold text-purple-600 dark:text-purple-400 mb-1"
          >
            {stats.total}
          </div>
          <div class="text-sm text-base-content/60">
            {tr("jobs.allJobsTracked")}
          </div>
        </div>
      </div>
    {/if}

    <!-- Modern Tabs with Glass Effect -->
    <div
      class="flex flex-wrap gap-3 mb-8 p-2 bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl rounded-2xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
    >
      <button
        class="flex items-center gap-2 px-6 py-3 rounded-xl font-semibold transition-all duration-200 {selectedTab ===
        'overview'
          ? 'bg-gradient-to-r from-purple-500 to-pink-500 text-white shadow-lg scale-105'
          : 'bg-base-200/50 hover:bg-base-300 text-base-content'}"
        onclick={() => (selectedTab = "overview")}
      >
        <i class="bi bi-speedometer2" aria-hidden="true"></i>
        <span>{tr("jobs.overview")}</span>
      </button>
      <button
        class="flex items-center gap-2 px-6 py-3 rounded-xl font-semibold transition-all duration-200 {selectedTab ===
        'active'
          ? 'bg-gradient-to-r from-purple-500 to-pink-500 text-white shadow-lg scale-105'
          : 'bg-base-200/50 hover:bg-base-300 text-base-content'}"
        onclick={() => (selectedTab = "active")}
      >
        <i class="bi bi-play-circle" aria-hidden="true"></i>
        <span>{tr("jobs.activeJobs")}</span>
        {#if activeJobs.length > 0}
          <span class="badge badge-sm bg-orange-500 text-white border-0"
            >{activeJobs.length}</span
          >
        {/if}
      </button>
      <button
        class="flex items-center gap-2 px-6 py-3 rounded-xl font-semibold transition-all duration-200 {selectedTab ===
        'history'
          ? 'bg-gradient-to-r from-purple-500 to-pink-500 text-white shadow-lg scale-105'
          : 'bg-base-200/50 hover:bg-base-300 text-base-content'}"
        onclick={() => (selectedTab = "history")}
      >
        <i class="bi bi-clock-history" aria-hidden="true"></i>
        <span>{tr("jobs.history")}</span>
      </button>
      <button
        class="flex items-center gap-2 px-6 py-3 rounded-xl font-semibold transition-all duration-200 {selectedTab ===
        'cron'
          ? 'bg-gradient-to-r from-purple-500 to-pink-500 text-white shadow-lg scale-105'
          : 'bg-base-200/50 hover:bg-base-300 text-base-content'}"
        onclick={() => (selectedTab = "cron")}
      >
        <i class="bi bi-calendar-event" aria-hidden="true"></i>
        <span>{tr("jobs.cronJobs")}</span>
        {#if cronJobs.length > 0}
          <span class="badge badge-sm bg-purple-500 text-white border-0"
            >{cronJobs.length}</span
          >
        {/if}
      </button>
    </div>

    <!-- Tab Content -->
    {#if loading}
      <div class="flex justify-center items-center py-12">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
    {:else}
      <!-- Overview Tab -->
      {#if selectedTab === "overview"}
        <div class="space-y-6">
          <div class="card bg-base-200">
            <div class="card-body">
              <h2 class="card-title">{tr("jobs.systemStatus")}</h2>
              <div class="stats stats-vertical lg:stats-horizontal shadow">
                <div class="stat">
                  <div class="stat-figure text-success">
                    <i class="bi bi-check-circle text-4xl" aria-hidden="true"
                    ></i>
                  </div>
                  <div class="stat-title">{tr("jobs.workerPool")}</div>
                  <div class="stat-value text-success">{tr("active")}</div>
                  <div class="stat-desc">4 {tr("jobs.workersActive")}</div>
                </div>

                <div class="stat">
                  <div class="stat-figure text-info">
                    <i class="bi bi-clock text-4xl" aria-hidden="true"></i>
                  </div>
                  <div class="stat-title">{tr("jobs.cronScheduler")}</div>
                  <div class="stat-value text-info">{tr("active")}</div>
                  <div class="stat-desc">
                    {cronJobs.filter((j) => j.enabled).length}
                    {tr("jobs.jobsScheduled")}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="flex gap-4">
            <button
              class="btn btn-primary"
              onclick={() => (selectedTab = "active")}
            >
              <i class="bi bi-play-circle" aria-hidden="true"></i>
              View Active Jobs
            </button>
            <button
              class="btn btn-secondary"
              onclick={() => (selectedTab = "cron")}
            >
              <i class="bi bi-calendar-event" aria-hidden="true"></i>
              Manage Cron Jobs
            </button>
            <button class="btn btn-outline" onclick={cleanupOldJobs}>
              <i class="bi bi-trash" aria-hidden="true"></i>
              Cleanup Old Jobs
            </button>
          </div>
        </div>
      {/if}

      <!-- Active Jobs Tab -->
      {#if selectedTab === "active"}
        <div class="card bg-base-200">
          <div class="card-body">
            <h2 class="card-title mb-4">Active Jobs</h2>

            {#if activeJobs.length === 0}
              <div class="text-center py-8 text-base-content/50">
                <i class="bi bi-inbox text-6xl mb-4" aria-hidden="true"></i>
                <p>No active jobs</p>
              </div>
            {:else}
              <div class="overflow-x-auto">
                <table class="table table-zebra w-full">
                  <thead>
                    <tr>
                      <th>Job Type</th>
                      <th>Status</th>
                      <th>Priority</th>
                      <th>Started</th>
                      <th>Attempts</th>
                      <th>Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each activeJobs as job}
                      <tr>
                        <td>
                          <div class="font-semibold">
                            {JOB_TYPE_NAMES[job.job_type] || job.job_type}
                          </div>
                          <div class="text-sm text-base-content/50">
                            {job.id}
                          </div>
                        </td>
                        <td>
                          <span class="badge {getStatusBadgeClass(job.status)}"
                            >{job.status}</span
                          >
                        </td>
                        <td>
                          <span class="badge badge-outline">{job.priority}</span
                          >
                        </td>
                        <td class="text-sm">{formatDate(job.started_at)}</td>
                        <td>{job.attempts} / {job.max_attempts}</td>
                        <td>
                          <button
                            class="btn btn-sm btn-error"
                            onclick={() => cancelJob(job.id)}
                          >
                            Cancel
                          </button>
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- History Tab -->
      {#if selectedTab === "history"}
        <div class="card bg-base-200">
          <div class="card-body">
            <h2 class="card-title mb-4">Job History</h2>

            {#if jobHistory.length === 0}
              <div class="text-center py-8 text-base-content/50">
                <i class="bi bi-inbox text-6xl mb-4" aria-hidden="true"></i>
                <p>No job history</p>
              </div>
            {:else}
              <div class="overflow-x-auto">
                <table class="table table-zebra w-full">
                  <thead>
                    <tr>
                      <th>Job Type</th>
                      <th>Status</th>
                      <th>Priority</th>
                      <th>Created</th>
                      <th>Completed</th>
                      <th>Attempts</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each jobHistory as job}
                      <tr>
                        <td>
                          <div class="font-semibold">
                            {JOB_TYPE_NAMES[job.job_type] || job.job_type}
                          </div>
                          <div class="text-sm text-base-content/50">
                            {job.id}
                          </div>
                        </td>
                        <td>
                          <span class="badge {getStatusBadgeClass(job.status)}"
                            >{job.status}</span
                          >
                        </td>
                        <td>
                          <span class="badge badge-outline">{job.priority}</span
                          >
                        </td>
                        <td class="text-sm">{formatDate(job.created_at)}</td>
                        <td class="text-sm">{formatDate(job.completed_at)}</td>
                        <td>{job.attempts}</td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- Cron Jobs Tab -->
      {#if selectedTab === "cron"}
        <div class="card bg-base-200">
          <div class="card-body">
            <div class="flex justify-between items-center mb-4">
              <h2 class="card-title">Cron Jobs</h2>
              <button class="btn btn-primary gap-2" onclick={openCreateCronJob}>
                <i class="bi bi-plus-circle" aria-hidden="true"></i>
                Create Cron Job
              </button>
            </div>

            {#if cronJobs.length === 0}
              <div class="text-center py-8 text-base-content/50">
                <i class="bi bi-calendar-x text-6xl mb-4" aria-hidden="true"
                ></i>
                <p>No cron jobs configured</p>
              </div>
            {:else}
              <div class="overflow-x-auto">
                <table class="table table-zebra w-full">
                  <thead>
                    <tr>
                      <th>Name</th>
                      <th>Job Type</th>
                      <th>Schedule</th>
                      <th>Enabled</th>
                      <th>Last Run</th>
                      <th>Next Run</th>
                      <th>Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each cronJobs as cronJob}
                      <tr>
                        <td class="font-semibold">{cronJob.name}</td>
                        <td
                          >{JOB_TYPE_NAMES[cronJob.job_type] ||
                            cronJob.job_type}</td
                        >
                        <td>
                          <code class="text-sm">{cronJob.cron_expression}</code>
                        </td>
                        <td>
                          <input
                            type="checkbox"
                            class="toggle toggle-success"
                            checked={cronJob.enabled}
                            onchange={() => toggleCronJob(cronJob)}
                          />
                        </td>
                        <td class="text-sm"
                          >{formatDate(cronJob.last_run_at)}</td
                        >
                        <td class="text-sm"
                          >{formatDate(cronJob.next_run_at)}</td
                        >
                        <td>
                          <button
                            class="btn btn-sm btn-error"
                            onclick={() => deleteCronJob(cronJob.id)}
                            aria-label="Delete"
                          >
                            <i class="bi bi-trash" aria-hidden="true"></i>
                          </button>
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>
        </div>
      {/if}
    {/if}
  </div>
</div>
