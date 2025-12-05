<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let triggering = $state(false);
  let eligibleCount = $state(0);
  let retentionDays = $state(30);
  let lastCleanup = $state(null);
  let error = $state(null);

  onMount(async () => {
    await loadCleanupStatus();
  });

  async function loadCleanupStatus() {
    loading = true;
    error = null;
    try {
      const response = await fetch("http://127.0.0.1:8080/api/cleanup/status", {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });
      if (response.ok) {
        const data = await response.json();
        eligibleCount = data.eligible_for_cleanup;
        retentionDays = data.retention_days;
        lastCleanup = data.last_cleanup;
      } else {
        error = tr("cleanup.loadError");
      }
    } catch (e) {
      console.error("Failed to load cleanup status:", e);
      error = tr("cleanup.loadError");
      errorToast(error);
    } finally {
      loading = false;
    }
  }

  async function triggerCleanup() {
    if (!confirm(tr("cleanup.confirmCleanup"))) return;

    triggering = true;
    try {
      const response = await fetch(
        "http://127.0.0.1:8080/api/cleanup/trigger",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
          body: JSON.stringify({
            dry_run: false,
            retention_days: retentionDays,
          }),
        }
      );

      if (response.ok) {
        const data = await response.json();
        success(
          `${tr("cleanup.success")}: ${data.stats.files_deleted} ${tr("cleanup.files")}`
        );
        await loadCleanupStatus();
      } else {
        throw new Error(tr("cleanup.triggerError"));
      }
    } catch (e) {
      console.error("Cleanup failed:", e);
      errorToast(e.message || tr("cleanup.triggerError"));
    } finally {
      triggering = false;
    }
  }

  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatDate(dateString) {
    if (!dateString) return "—";
    return new Date(dateString).toLocaleString();
  }
</script>

<div
  class="bg-white dark:bg-gray-800 rounded-2xl shadow-lg p-8 border border-gray-200 dark:border-gray-700"
>
  <div class="mb-8">
    <h2
      class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-3 mb-2"
    >
      <i class="bi bi-trash2 text-red-500" aria-hidden="true"></i>{tr(
        "cleanup.title"
      )}
    </h2>
    <p class="text-gray-600 dark:text-gray-400">{tr("cleanup.description")}</p>
  </div>

  {#if loading}
    <div class="flex justify-center py-12">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-green-500"
      ></div>
    </div>
  {:else if error}
    <div
      class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 text-red-700 dark:text-red-200 mb-6"
    >
      <i class="bi bi-exclamation-circle mr-2" aria-hidden="true"></i>{error}
    </div>
  {:else}
    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <!-- Eligible for Cleanup -->
      <div
        class="bg-gradient-to-br from-orange-50 to-red-50 dark:from-orange-900/20 dark:to-red-900/20 rounded-xl p-6 border border-orange-200 dark:border-orange-800"
      >
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-semibold text-gray-900 dark:text-white">
            {tr("cleanup.eligibleFiles")}
          </h3>
          <i
            class="bi bi-exclamation-triangle text-orange-500 text-xl"
            aria-hidden="true"
          ></i>
        </div>
        <p class="text-3xl font-bold text-orange-600 dark:text-orange-400">
          {eligibleCount}
        </p>
        <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
          {tr("cleanup.olderThan")}
          {retentionDays}
          {tr("cleanup.days")}
        </p>
      </div>

      <!-- Retention Policy -->
      <div
        class="bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 rounded-xl p-6 border border-green-200 dark:border-green-800"
      >
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-semibold text-gray-900 dark:text-white">
            {tr("cleanup.retentionPolicy")}
          </h3>
          <i class="bi bi-calendar text-green-500 text-xl" aria-hidden="true"
          ></i>
        </div>
        <p class="text-3xl font-bold text-green-600 dark:text-green-400">
          {retentionDays}
        </p>
        <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
          {tr("cleanup.days")}
        </p>
      </div>

      <!-- Last Cleanup -->
      <div
        class="bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 rounded-xl p-6 border border-green-200 dark:border-green-800"
      >
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-semibold text-gray-900 dark:text-white">
            {tr("cleanup.lastCleanup")}
          </h3>
          <i
            class="bi bi-check-circle text-green-500 text-xl"
            aria-hidden="true"
          ></i>
        </div>
        <p class="text-lg font-bold text-green-600 dark:text-green-400">
          {lastCleanup ? lastCleanup.files_deleted : "—"}
          {tr("cleanup.files")}
        </p>
        <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
          {lastCleanup
            ? formatDate(lastCleanup.last_cleanup_at)
            : tr("cleanup.never")}
        </p>
      </div>
    </div>

    <!-- Last Cleanup Details -->
    {#if lastCleanup}
      <div
        class="bg-gray-50 dark:bg-gray-900 rounded-xl p-6 mb-8 border border-gray-200 dark:border-gray-700"
      >
        <h3
          class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
        >
          <i class="bi bi-info-circle" aria-hidden="true"></i>{tr(
            "cleanup.lastCleanupDetails"
          )}
        </h3>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <p
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
            >
              {tr("cleanup.filesDeleted")}
            </p>
            <p class="text-lg font-bold text-gray-900 dark:text-white">
              {lastCleanup.files_deleted}
            </p>
          </div>
          <div>
            <p
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
            >
              {tr("cleanup.storageFreed")}
            </p>
            <p class="text-lg font-bold text-gray-900 dark:text-white">
              {formatBytes(lastCleanup.storage_freed_bytes)}
            </p>
          </div>
          <div>
            <p
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
            >
              {tr("cleanup.failed")}
            </p>
            <p class="text-lg font-bold text-gray-900 dark:text-white">
              {lastCleanup.failed_deletions}
            </p>
          </div>
          <div>
            <p
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
            >
              {tr("cleanup.duration")}
            </p>
            <p class="text-lg font-bold text-gray-900 dark:text-white">
              {lastCleanup.duration_ms}ms
            </p>
          </div>
        </div>
      </div>
    {/if}

    <!-- Action Buttons -->
    <div class="flex gap-3">
      <button
        onclick={triggerCleanup}
        disabled={triggering || eligibleCount === 0}
        class="px-6 py-3 bg-red-500 text-white rounded-lg font-medium hover:bg-red-600 transition-all duration-200 flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <i class="bi bi-play-circle" aria-hidden="true"></i>
        {triggering ? tr("cleanup.running") : tr("cleanup.trigger")}
      </button>

      <button
        onclick={loadCleanupStatus}
        disabled={loading}
        class="px-6 py-3 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-all duration-200 flex items-center gap-2 disabled:opacity-50"
      >
        <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>{tr(
          "cleanup.refresh"
        )}
      </button>
    </div>

    <!-- Info Box -->
    <div
      class="mt-8 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4 text-sm text-green-700 dark:text-green-200"
    >
      <i class="bi bi-info-circle mr-2" aria-hidden="true"></i>
      <span>{tr("cleanup.info1")}</span><br />
      <span class="ml-6">{tr("cleanup.info2")}</span><br />
      <span class="ml-6">{tr("cleanup.info3")}</span>
    </div>
  {/if}
</div>
