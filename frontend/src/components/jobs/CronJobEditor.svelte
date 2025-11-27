<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";
  import Modal from "../ui/Modal.svelte";

  let { cronJob = null, onSave, onCancel } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let formData = $state({
    name: cronJob?.name || "",
    job_type: cronJob?.job_type || "FileCleanup",
    cron_expression: cronJob?.cron_expression || "0 0 * * *",
    payload: cronJob?.payload || "{}",
  });

  let loading = $state(false);
  let error = $state("");

  const jobTypes = [
    { value: "SearchIndexing", label: "Search Indexing" },
    { value: "ThumbnailGeneration", label: "Thumbnail Generation" },
    { value: "FileCleanup", label: "File Cleanup" },
    { value: "BackupTask", label: "Backup Task" },
    { value: "EmailNotification", label: "Email Notification" },
    { value: "WebhookDelivery", label: "Webhook Delivery" },
    { value: "FileCompression", label: "File Compression" },
    { value: "VirusScan", label: "Virus Scan" },
  ];

  async function handleSubmit(e) {
    e?.preventDefault();

    if (!formData.name.trim()) {
      error = "Name is required";
      return;
    }

    loading = true;
    error = "";

    try {
      if (cronJob) {
        // Update existing
        await api.cron.update(cronJob.id, formData);
      } else {
        // Create new
        await api.cron.create(
          formData.name,
          formData.job_type,
          formData.cron_expression,
          formData.payload
        );
      }
      onSave?.();
    } catch (err) {
      console.error("Error saving cron job:", err);
      error = err.message || "Failed to save cron job";
    } finally {
      loading = false;
    }
  }
</script>

<Modal
  visible={true}
  title={cronJob ? tr("jobs.editCronJob") : tr("jobs.createCronJob")}
  icon="clock-history"
  size="md"
  onclose={onCancel}
>
  {#snippet children()}
    {#if error}
      <div
        class="bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-300 px-4 py-3 rounded-lg mb-4"
      >
        <i class="bi bi-exclamation-triangle mr-2"></i>
        <span>{error}</span>
      </div>
    {/if}

    <form onsubmit={handleSubmit} class="space-y-4">
      <!-- Name -->
      <div class="form-control">
        <label
          class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
          for="cron-name"
        >
          {tr("jobs.name")}
        </label>
        <input
          id="cron-name"
          type="text"
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          bind:value={formData.name}
          placeholder="Daily Cleanup"
          required
        />
      </div>

      <!-- Job Type -->
      <div class="form-control">
        <label
          class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
          for="cron-type"
        >
          {tr("jobs.jobType")}
        </label>
        <select
          id="cron-type"
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          bind:value={formData.job_type}
        >
          {#each jobTypes as jt}
            <option value={jt.value}>{jt.label}</option>
          {/each}
        </select>
      </div>

      <!-- Cron Expression -->
      <div class="form-control">
        <label
          class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
          for="cron-expr"
        >
          {tr("jobs.cronExpression")}
        </label>
        <input
          id="cron-expr"
          type="text"
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          bind:value={formData.cron_expression}
          placeholder="0 0 * * *"
        />
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          <i class="bi bi-info-circle mr-1"></i>
          {tr("jobs.cronExpressionHint")}
        </p>
      </div>
    </form>
  {/snippet}

  {#snippet actions()}
    <button
      type="button"
      onclick={onCancel}
      class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
      disabled={loading}
    >
      {tr("common.cancel")}
    </button>
    <button
      type="submit"
      class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50"
      disabled={loading}
      onclick={handleSubmit}
    >
      {#if loading}
        <span class="loading loading-spinner loading-sm mr-2"></span>
      {/if}
      {cronJob ? tr("common.save") : tr("common.create")}
    </button>
  {/snippet}
</Modal>
