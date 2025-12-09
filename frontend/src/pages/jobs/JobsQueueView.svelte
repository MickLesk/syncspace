<script>
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";
  import { t } from "../../i18n.js";

  let jobs = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let refreshInterval = null;

  onMount(async () => {
    await loadJobs();
    // Auto-refresh every 3 seconds
    refreshInterval = setInterval(loadJobs, 3000);
  });

  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });

  async function loadJobs() {
    try {
      const response = await api.bulk.listJobs();
      jobs = response.jobs || [];
      error = null;
    } catch (err) {
      console.error("Failed to load jobs:", err);
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function cancelJob(jobId) {
    if (!confirm($t("jobs.confirmCancel"))) return;

    try {
      await api.bulk.cancelJob(jobId);
      await loadJobs();
    } catch (err) {
      alert($t("jobs.failedToCancel") + ": " + err.message);
    }
  }

  async function deleteJob(jobId) {
    if (!confirm($t("jobs.confirmDelete"))) return;

    try {
      await api.bulk.deleteJob(jobId);
      await loadJobs();
    } catch (err) {
      alert($t("jobs.failedToDelete") + ": " + err.message);
    }
  }

  function getStatusColor(status) {
    switch (status) {
      case "completed":
        return "text-green-600 bg-green-100";
      case "running":
        return "text-orange-600 bg-orange-100";
      case "pending":
        return "text-yellow-600 bg-yellow-100";
      case "failed":
        return "text-red-600 bg-red-100";
      case "cancelled":
        return "text-gray-600 bg-gray-100";
      default:
        return "text-gray-600 bg-gray-100";
    }
  }

  function getJobTypeLabel(jobType) {
    const labels = {
      bulk_delete: $t("jobs.bulkDelete"),
      bulk_move: $t("jobs.bulkMove"),
      bulk_copy: $t("jobs.bulkCopy"),
      bulk_compress: $t("jobs.bulkCompress"),
    };
    return labels[jobType] || jobType;
  }

  function formatDate(dateString) {
    if (!dateString) return "-";
    const date = new Date(dateString);
    return date.toLocaleString();
  }

  $effect(() => {
    // Auto-refresh when tab is visible
    const handleVisibilityChange = () => {
      if (!document.hidden && refreshInterval) {
        loadJobs();
      }
    };

    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  });
</script>

<div class="p-6">
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-lg bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center shadow-lg"
        >
          <i class="bi bi-list-task text-white text-lg" aria-hidden="true"></i>
        </div>
        <div>
          <h2 class="text-xl font-bold text-gray-900 dark:text-white">
            {$t("jobs.title")}
          </h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {$t("jobs.description")}
          </p>
        </div>
      </div>

      <button
        onclick={() => loadJobs()}
        class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors flex items-center gap-2"
      >
        <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
        {$t("refresh")}
      </button>
    </div>

    <!-- Loading State -->
    {#if loading && jobs.length === 0}
      <div class="text-center py-12">
        <div
          class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-green-600"
        ></div>
        <p class="mt-4 text-gray-500 dark:text-gray-400">{$t("loading")}</p>
      </div>
    {:else if error}
      <div class="bg-red-50 border border-red-200 rounded-lg p-4">
        <p class="text-red-800">{error}</p>
      </div>
    {:else if jobs.length === 0}
      <div class="text-center py-12">
        <i
          class="bi bi-inbox text-6xl text-gray-300 dark:text-gray-600"
          aria-hidden="true"
        ></i>
        <p class="mt-4 text-gray-500 dark:text-gray-400">{$t("jobs.noJobs")}</p>
      </div>
    {:else}
      <!-- Jobs Table -->
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50 dark:bg-gray-900">
            <tr>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
              >
                {$t("jobs.type")}
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
              >
                {$t("jobs.status")}
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
              >
                {$t("jobs.progress")}
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
              >
                {$t("jobs.created")}
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
              >
                {$t("jobs.completed")}
              </th>
              <th
                class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
              >
                {$t("actions")}
              </th>
            </tr>
          </thead>
          <tbody
            class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700"
          >
            {#each jobs as job (job.job_id)}
              <tr class="hover:bg-gray-50 dark:hover:bg-gray-750">
                <!-- Type -->
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex items-center">
                    <i class="bi bi-stack text-gray-400 mr-2" aria-hidden="true"
                    ></i>
                    <span
                      class="text-sm font-medium text-gray-900 dark:text-white"
                    >
                      {getJobTypeLabel(job.job_type)}
                    </span>
                  </div>
                </td>

                <!-- Status Badge -->
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full {getStatusColor(
                      job.status
                    )}"
                  >
                    {$t("jobs.status_" + job.status)}
                  </span>
                </td>

                <!-- Progress Bar -->
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex items-center gap-2">
                    <div
                      class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2 overflow-hidden"
                    >
                      <div
                        class="h-full transition-all duration-300 {job.status ===
                        'failed'
                          ? 'bg-red-500'
                          : 'bg-green-600'}"
                        style="width: {job.progress}%"
                      ></div>
                    </div>
                    <span
                      class="text-sm text-gray-600 dark:text-gray-400 w-12 text-right"
                    >
                      {job.progress}%
                    </span>
                  </div>
                </td>

                <!-- Created -->
                <td
                  class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400"
                >
                  {formatDate(job.created_at)}
                </td>

                <!-- Completed -->
                <td
                  class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400"
                >
                  {formatDate(job.completed_at)}
                </td>

                <!-- Actions -->
                <td
                  class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium"
                >
                  <div class="flex items-center justify-end gap-2">
                    {#if job.status === "running" || job.status === "pending"}
                      <button
                        onclick={() => cancelJob(job.job_id)}
                        class="text-red-600 hover:text-red-900 dark:hover:text-red-400"
                        title={$t("jobs.cancel")}
                        aria-label="Cancel"
                      >
                        <i class="bi bi-x-circle" aria-hidden="true"></i>
                      </button>
                    {/if}

                    {#if job.status === "completed" || job.status === "failed" || job.status === "cancelled"}
                      <button
                        onclick={() => deleteJob(job.job_id)}
                        class="text-gray-600 hover:text-gray-900 dark:hover:text-gray-400"
                        title={$t("jobs.delete")}
                        aria-label="Delete"
                      >
                        <i class="bi bi-trash" aria-hidden="true"></i>
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

<style>
  /* Custom scrollbar for table */
  .overflow-x-auto::-webkit-scrollbar {
    height: 8px;
  }

  .overflow-x-auto::-webkit-scrollbar-track {
    background: rgb(243 244 246);
  }

  .overflow-x-auto::-webkit-scrollbar-thumb {
    background: rgb(209 213 219);
    border-radius: 4px;
  }

  .overflow-x-auto::-webkit-scrollbar-thumb:hover {
    background: rgb(156 163 175);
  }

  :global(.dark) .overflow-x-auto::-webkit-scrollbar-track {
    background: rgb(31 41 55);
  }

  :global(.dark) .overflow-x-auto::-webkit-scrollbar-thumb {
    background: rgb(55 65 81);
  }

  :global(.dark) .overflow-x-auto::-webkit-scrollbar-thumb:hover {
    background: rgb(75 85 99);
  }
</style>
