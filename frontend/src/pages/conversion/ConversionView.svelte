<script>
  import { onMount, onDestroy } from "svelte";
  import { language } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { conversion } from "../../lib/api.js";
  import { websocketManager } from "../../stores/websocket.js";
  import ConversionCard from "./ConversionCard.svelte";
  import ConversionModal from "./ConversionModal.svelte";
  import ConversionPresetsModal from "./ConversionPresetsModal.svelte";

  let jobs = $state([]);
  let formats = $state({});
  let loading = $state(true);
  let error = $state(null);
  let showCreateModal = $state(false);
  let showPresetsModal = $state(false);
  let refreshInterval = null;
  let wsUnsubscribe = null;

  onMount(async () => {
    await loadData();

    // Auto-refresh jobs every 10 seconds (backup for WebSocket)
    refreshInterval = setInterval(loadJobs, 10000);

    // Connect to WebSocket for real-time updates
    websocketManager.connect();

    // Subscribe to conversion progress events
    wsUnsubscribe = websocketManager.on("conversion_progress", (event) => {
      handleConversionProgress(event);
    });

    // Also listen to all WebSocket messages
    wsUnsubscribe = websocketManager.on("message", (data) => {
      if (data.event_type === "conversion_progress") {
        handleConversionProgress(data);
      }
    });
  });

  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
    if (wsUnsubscribe) {
      wsUnsubscribe();
    }
  });

  function handleConversionProgress(event) {
    console.log("📊 Conversion progress update:", event);

    // Update job in the list
    const index = jobs.findIndex((j) => j.job_id === event.job_id);
    if (index !== -1) {
      jobs[index] = {
        ...jobs[index],
        status: event.status,
        progress: event.progress,
        error_message: event.error_message,
        output_path: event.output_path,
        completed_at:
          event.status === "completed" || event.status === "failed"
            ? new Date().toISOString()
            : jobs[index].completed_at,
      };
      jobs = [...jobs]; // Trigger reactivity
    } else {
      // Job not in list yet, refresh
      loadJobs();
    }
  }

  async function loadData() {
    loading = true;
    error = null;
    try {
      const [jobsData, formatsData] = await Promise.all([
        conversion.listJobs(),
        conversion.listFormats(),
      ]);
      jobs = jobsData;
      formats = formatsData;
    } catch (err) {
      error = err.message || "Failed to load conversion data";
      console.error("Load conversion data error:", err);
    } finally {
      loading = false;
    }
  }

  async function loadJobs() {
    try {
      jobs = await conversion.listJobs();
    } catch (err) {
      console.error("Refresh jobs error:", err);
    }
  }

  async function handleCancelJob(jobId) {
    try {
      await conversion.cancelJob(jobId);
      await loadJobs();
    } catch (err) {
      console.error("Cancel job error:", err);
    }
  }

  async function handleDeleteJob(jobId) {
    if (!confirm(t($language, "conversion.confirmDelete"))) return;
    try {
      await conversion.deleteJob(jobId);
      await loadJobs();
    } catch (err) {
      console.error("Delete job error:", err);
    }
  }

  function handleJobCreated() {
    showCreateModal = false;
    loadJobs();
  }

  // Separate jobs by status
  const activeJobs = $derived(
    jobs.filter((j) => j.status === "pending" || j.status === "processing")
  );
  const completedJobs = $derived(jobs.filter((j) => j.status === "completed"));
  const failedJobs = $derived(
    jobs.filter((j) => j.status === "failed" || j.status === "cancelled")
  );
</script>

<div class="flex h-full flex-col bg-base-100">
  <!-- Header -->
  <div class="border-b border-base-300 bg-base-200 px-6 py-4">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-base-content">
          <i class="bi bi-arrow-repeat mr-2"></i>
          {t($language, "conversion.title")}
        </h1>
        <p class="mt-1 text-sm text-base-content/60">
          {t($language, "conversion.subtitle")}
        </p>
      </div>
      <div class="flex gap-2">
        <button
          onclick={() => (showPresetsModal = true)}
          class="btn btn-ghost btn-sm"
          aria-label={t($language, "conversion.presets")}
        >
          <i class="bi bi-bookmark"></i>
          {t($language, "conversion.presets")}
        </button>
        <button
          onclick={loadData}
          class="btn btn-ghost btn-sm"
          aria-label={t($language, "conversion.refreshJobs")}
        >
          <i class="bi bi-arrow-clockwise"></i>
          {t($language, "conversion.refreshJobs")}
        </button>
        <button
          onclick={() => (showCreateModal = true)}
          class="btn btn-primary btn-sm"
          aria-label={t($language, "conversion.createJob")}
        >
          <i class="bi bi-plus-lg"></i>
          {t($language, "conversion.createJob")}
        </button>
      </div>
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto p-6">
    {#if loading}
      <div class="flex items-center justify-center py-12">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
    {:else if error}
      <div class="alert alert-error">
        <i class="bi bi-exclamation-triangle"></i>
        <span>{error}</span>
      </div>
    {:else if jobs.length === 0}
      <div class="flex flex-col items-center justify-center py-12 text-center">
        <i class="bi bi-arrow-repeat mb-4 text-6xl text-base-content/20"></i>
        <h3 class="mb-2 text-xl font-semibold text-base-content">
          {t($language, "conversion.noJobs")}
        </h3>
        <p class="mb-6 text-base-content/60">
          {t($language, "conversion.noJobsHint")}
        </p>
        <button
          onclick={() => (showCreateModal = true)}
          class="btn btn-primary"
        >
          <i class="bi bi-plus-lg"></i>
          {t($language, "conversion.createJob")}
        </button>
      </div>
    {:else}
      <!-- Active Jobs -->
      {#if activeJobs.length > 0}
        <div class="mb-8">
          <h2 class="mb-4 text-lg font-semibold text-base-content">
            <i class="bi bi-hourglass-split mr-2"></i>
            {t($language, "conversion.statusProcessing")}
            <span class="badge badge-primary ml-2">{activeJobs.length}</span>
          </h2>
          <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
            {#each activeJobs as job (job.job_id)}
              <ConversionCard
                {job}
                oncancel={() => handleCancelJob(job.job_id)}
                ondelete={() => handleDeleteJob(job.job_id)}
              />
            {/each}
          </div>
        </div>
      {/if}

      <!-- Completed Jobs -->
      {#if completedJobs.length > 0}
        <div class="mb-8">
          <h2 class="mb-4 text-lg font-semibold text-base-content">
            <i class="bi bi-check-circle mr-2"></i>
            {t($language, "conversion.statusCompleted")}
            <span class="badge badge-success ml-2">{completedJobs.length}</span>
          </h2>
          <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
            {#each completedJobs as job (job.job_id)}
              <ConversionCard
                {job}
                ondelete={() => handleDeleteJob(job.job_id)}
              />
            {/each}
          </div>
        </div>
      {/if}

      <!-- Failed Jobs -->
      {#if failedJobs.length > 0}
        <div>
          <h2 class="mb-4 text-lg font-semibold text-base-content">
            <i class="bi bi-x-circle mr-2"></i>
            {t($language, "conversion.statusFailed")}
            <span class="badge badge-error ml-2">{failedJobs.length}</span>
          </h2>
          <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
            {#each failedJobs as job (job.job_id)}
              <ConversionCard
                {job}
                ondelete={() => handleDeleteJob(job.job_id)}
              />
            {/each}
          </div>
        </div>
      {/if}
    {/if}
  </div>
</div>

<!-- Modals -->
{#if showCreateModal}
  <ConversionModal
    {formats}
    onclose={() => (showCreateModal = false)}
    oncreated={handleJobCreated}
  />
{/if}

{#if showPresetsModal}
  <ConversionPresetsModal
    {formats}
    onclose={() => (showPresetsModal = false)}
  />
{/if}
