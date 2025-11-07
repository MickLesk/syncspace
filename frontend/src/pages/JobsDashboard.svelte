<script>
  import { onMount, onDestroy } from "svelte";
  import * as api from "../lib/api.js";

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

  // Create cron modal
  let showCreateCronModal = $state(false);
  let newCronJob = $state({
    name: "",
    job_type: "FileCleanup",
    cron_expression: "0 0 * * *", // Daily at midnight
    payload: {},
  });

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
      const cronRes = await fetch("/api/cron", {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });
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

  async function createCronJob() {
    try {
      const res = await fetch("/api/cron", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
        body: JSON.stringify(newCronJob),
      });

      if (res.ok) {
        showCreateCronModal = false;
        newCronJob = {
          name: "",
          job_type: "FileCleanup",
          cron_expression: "0 0 * * *",
          payload: {},
        };
        await loadData();
      } else {
        alert("Failed to create cron job");
      }
    } catch (err) {
      console.error("Failed to create cron job:", err);
      alert("Failed to create cron job");
    }
  }

  async function toggleCronJob(cronJob) {
    try {
      const endpoint = cronJob.enabled ? "disable" : "enable";
      const res = await fetch(`/api/cron/${cronJob.id}/${endpoint}`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });

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
      const res = await fetch(`/api/cron/${cronJobId}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });

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

<div class="p-6 max-w-7xl mx-auto">
  <!-- Header -->
  <div class="mb-6">
    <h1 class="text-3xl font-bold mb-2">Background Jobs Dashboard</h1>
    <p class="text-base-content/70">
      Monitor and manage background jobs, cron schedules, and system tasks
    </p>
  </div>

  <!-- Error Alert -->
  {#if error}
    <div class="alert alert-error mb-6">
      <i class="bi bi-exclamation-triangle"></i>
      <span>Error: {error}</span>
    </div>
  {/if}

  <!-- Stats Cards -->
  {#if !loading}
    <div class="grid grid-cols-1 md:grid-cols-5 gap-4 mb-6">
      <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Pending</div>
        <div class="stat-value text-info">{stats.pending}</div>
      </div>

      <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Running</div>
        <div class="stat-value text-warning">{stats.running}</div>
      </div>

      <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Completed</div>
        <div class="stat-value text-success">{stats.completed}</div>
      </div>

      <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Failed</div>
        <div class="stat-value text-error">{stats.failed}</div>
      </div>

      <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Total</div>
        <div class="stat-value">{stats.total}</div>
      </div>
    </div>
  {/if}

  <!-- Tabs -->
  <div class="tabs tabs-boxed mb-6">
    <button
      class="tab {selectedTab === 'overview' ? 'tab-active' : ''}"
      onclick={() => (selectedTab = "overview")}
    >
      <i class="bi bi-speedometer2 mr-2"></i> Overview
    </button>
    <button
      class="tab {selectedTab === 'active' ? 'tab-active' : ''}"
      onclick={() => (selectedTab = "active")}
    >
      <i class="bi bi-play-circle mr-2"></i> Active Jobs ({activeJobs.length})
    </button>
    <button
      class="tab {selectedTab === 'history' ? 'tab-active' : ''}"
      onclick={() => (selectedTab = "history")}
    >
      <i class="bi bi-clock-history mr-2"></i> History
    </button>
    <button
      class="tab {selectedTab === 'cron' ? 'tab-active' : ''}"
      onclick={() => (selectedTab = "cron")}
    >
      <i class="bi bi-calendar-event mr-2"></i> Cron Jobs ({cronJobs.length})
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
            <h2 class="card-title">System Status</h2>
            <div class="stats stats-vertical lg:stats-horizontal shadow">
              <div class="stat">
                <div class="stat-figure text-success">
                  <i class="bi bi-check-circle text-4xl"></i>
                </div>
                <div class="stat-title">Worker Pool</div>
                <div class="stat-value text-success">Active</div>
                <div class="stat-desc">4 workers running</div>
              </div>

              <div class="stat">
                <div class="stat-figure text-info">
                  <i class="bi bi-clock text-4xl"></i>
                </div>
                <div class="stat-title">Cron Scheduler</div>
                <div class="stat-value text-info">Active</div>
                <div class="stat-desc">
                  {cronJobs.filter((j) => j.enabled).length} enabled jobs
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
            <i class="bi bi-play-circle"></i>
            View Active Jobs
          </button>
          <button
            class="btn btn-secondary"
            onclick={() => (selectedTab = "cron")}
          >
            <i class="bi bi-calendar-event"></i>
            Manage Cron Jobs
          </button>
          <button class="btn btn-outline" onclick={cleanupOldJobs}>
            <i class="bi bi-trash"></i>
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
              <i class="bi bi-inbox text-6xl mb-4"></i>
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
                        <div class="text-sm text-base-content/50">{job.id}</div>
                      </td>
                      <td>
                        <span class="badge {getStatusBadgeClass(job.status)}"
                          >{job.status}</span
                        >
                      </td>
                      <td>
                        <span class="badge badge-outline">{job.priority}</span>
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
              <i class="bi bi-inbox text-6xl mb-4"></i>
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
                        <div class="text-sm text-base-content/50">{job.id}</div>
                      </td>
                      <td>
                        <span class="badge {getStatusBadgeClass(job.status)}"
                          >{job.status}</span
                        >
                      </td>
                      <td>
                        <span class="badge badge-outline">{job.priority}</span>
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
            <button
              class="btn btn-primary btn-sm"
              onclick={() => (showCreateCronModal = true)}
            >
              <i class="bi bi-plus-circle"></i>
              Create Cron Job
            </button>
          </div>

          {#if cronJobs.length === 0}
            <div class="text-center py-8 text-base-content/50">
              <i class="bi bi-calendar-x text-6xl mb-4"></i>
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
                      <td class="text-sm">{formatDate(cronJob.last_run_at)}</td>
                      <td class="text-sm">{formatDate(cronJob.next_run_at)}</td>
                      <td>
                        <button
                          class="btn btn-sm btn-error"
                          onclick={() => deleteCronJob(cronJob.id)}
                        >
                          <i class="bi bi-trash"></i>
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

<!-- Create Cron Job Modal -->
{#if showCreateCronModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">Create Cron Job</h3>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Name</span>
        </label>
        <input
          type="text"
          class="input input-bordered"
          bind:value={newCronJob.name}
          placeholder="Daily Cleanup"
        />
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Job Type</span>
        </label>
        <select class="select select-bordered" bind:value={newCronJob.job_type}>
          <option value="SearchIndexing">Search Indexing</option>
          <option value="ThumbnailGeneration">Thumbnail Generation</option>
          <option value="FileCleanup">File Cleanup</option>
          <option value="BackupTask">Backup Task</option>
          <option value="EmailNotification">Email Notification</option>
          <option value="WebhookDelivery">Webhook Delivery</option>
          <option value="FileCompression">File Compression</option>
          <option value="VirusScan">Virus Scan</option>
        </select>
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Cron Expression</span>
        </label>
        <input
          type="text"
          class="input input-bordered"
          bind:value={newCronJob.cron_expression}
          placeholder="0 0 * * *"
        />
        <label class="label">
          <span class="label-text-alt"
            >Format: minute hour day month weekday</span
          >
        </label>
      </div>

      <div class="modal-action">
        <button class="btn" onclick={() => (showCreateCronModal = false)}
          >Cancel</button
        >
        <button class="btn btn-primary" onclick={createCronJob}>Create</button>
      </div>
    </div>
  </div>
{/if}
