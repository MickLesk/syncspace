<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let overview = $state(null);
  let userStats = $state([]);
  let folderStats = $state([]);
  let topFiles = $state([]);
  let growth = $state([]);
  let duplicateWaste = $state(null);
  let loading = $state(true);
  let error = $state("");
  let activeTab = $state("overview"); // 'overview' | 'users' | 'folders' | 'top-files' | 'growth' | 'duplicates'

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  onMount(async () => {
    await loadAllData();
  });

  async function loadAllData() {
    loading = true;
    error = "";

    try {
      const [
        overviewData,
        usersData,
        foldersData,
        filesData,
        growthData,
        dupData,
      ] = await Promise.all([
        api.storageAnalytics.getOverview(),
        api.storageAnalytics.getByUser(),
        api.storageAnalytics.getByFolder(),
        api.storageAnalytics.getTopFiles(100),
        api.storageAnalytics.getGrowth(30),
        api.storageAnalytics.getDuplicateWaste(),
      ]);

      overview = overviewData;
      userStats = usersData;
      folderStats = foldersData;
      topFiles = filesData;
      growth = growthData;
      duplicateWaste = dupData;
    } catch (err) {
      console.error("Failed to load storage analytics:", err);
      error = tr("failedToLoadAnalytics");
    } finally {
      loading = false;
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    const date = new Date(dateStr);
    return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }
</script>

<div class="container mx-auto p-6 space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1
        class="text-3xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-3"
      >
        <i class="bi bi-bar-chart-line" aria-hidden="true"></i>
        {tr("storageAnalytics")}
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mt-1">
        {tr("storageAnalyticsDescription")}
      </p>
    </div>
    <button
      onclick={loadAllData}
      class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition flex items-center gap-2"
      disabled={loading}
    >
      {#if loading}
        <i class="bi bi-arrow-clockwise animate-spin" aria-hidden="true"></i>
      {:else}
        <i class="bi bi-arrow-clockwise" aria-hidden="true"></i>
      {/if}
      {tr("refresh")}
    </button>
  </div>

  {#if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
      <span>{error}</span>
    </div>
  {/if}

  {#if loading}
    <div class="flex items-center justify-center py-20">
      <div class="flex flex-col items-center gap-3">
        <i class="bi bi-arrow-clockwise animate-spin text-4xl text-primary-500"
         aria-hidden="true"></i>
        <span class="text-gray-600 dark:text-gray-400"
          >{tr("loadingAnalytics")}</span
        >
      </div>
    </div>
  {:else}
    <!-- Tabs -->
    <div class="tabs tabs-boxed bg-gray-100 dark:bg-gray-800 p-1">
      <button
        class="tab {activeTab === 'overview' ? 'tab-active' : ''}"
        onclick={() => (activeTab = "overview")}
      >
        <i class="bi bi-pie-chart mr-2" aria-hidden="true"></i>
        {tr("overview")}
      </button>
      <button
        class="tab {activeTab === 'users' ? 'tab-active' : ''}"
        onclick={() => (activeTab = "users")}
      >
        <i class="bi bi-people mr-2" aria-hidden="true"></i>
        {tr("byUser")}
      </button>
      <button
        class="tab {activeTab === 'folders' ? 'tab-active' : ''}"
        onclick={() => (activeTab = "folders")}
      >
        <i class="bi bi-folder mr-2" aria-hidden="true"></i>
        {tr("byFolder")}
      </button>
      <button
        class="tab {activeTab === 'top-files' ? 'tab-active' : ''}"
        onclick={() => (activeTab = "top-files")}
      >
        <i class="bi bi-file-earmark-fill mr-2" aria-hidden="true"></i>
        {tr("topFiles")}
      </button>
      <button
        class="tab {activeTab === 'growth' ? 'tab-active' : ''}"
        onclick={() => (activeTab = "growth")}
      >
        <i class="bi bi-graph-up mr-2" aria-hidden="true"></i>
        {tr("growth")}
      </button>
      <button
        class="tab {activeTab === 'duplicates' ? 'tab-active' : ''}"
        onclick={() => (activeTab = "duplicates")}
      >
        <i class="bi bi-files mr-2" aria-hidden="true"></i>
        {tr("duplicates")}
      </button>
    </div>

    <!-- Overview Tab -->
    {#if activeTab === "overview" && overview}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- Total Files Card -->
        <div
          class="card bg-gradient-to-br from-blue-500 to-blue-600 text-white shadow-lg"
        >
          <div class="card-body">
            <i class="bi bi-files text-4xl opacity-80" aria-hidden="true"></i>
            <h2 class="card-title text-5xl font-bold">
              {overview.total_files.toLocaleString()}
            </h2>
            <p class="opacity-90">{tr("totalFiles")}</p>
          </div>
        </div>

        <!-- Total Size Card -->
        <div
          class="card bg-gradient-to-br from-green-500 to-green-600 text-white shadow-lg"
        >
          <div class="card-body">
            <i class="bi bi-hdd text-4xl opacity-80" aria-hidden="true"></i>
            <h2 class="card-title text-3xl font-bold">
              {overview.total_size_formatted}
            </h2>
            <p class="opacity-90">{tr("totalStorage")}</p>
          </div>
        </div>

        <!-- Active Users Card -->
        <div
          class="card bg-gradient-to-br from-purple-500 to-purple-600 text-white shadow-lg"
        >
          <div class="card-body">
            <i class="bi bi-people text-4xl opacity-80" aria-hidden="true"></i>
            <h2 class="card-title text-5xl font-bold">
              {overview.active_users.toLocaleString()}
            </h2>
            <p class="opacity-90">{tr("activeUsers")}</p>
          </div>
        </div>

        <!-- Avg File Size Card -->
        <div
          class="card bg-gradient-to-br from-orange-500 to-orange-600 text-white shadow-lg"
        >
          <div class="card-body">
            <i class="bi bi-file-earmark-bar-graph text-4xl opacity-80" aria-hidden="true"></i>
            <h2 class="card-title text-2xl font-bold">
              {(overview.avg_file_size_bytes / 1024 / 1024).toFixed(2)} MB
            </h2>
            <p class="opacity-90">{tr("avgFileSize")}</p>
          </div>
        </div>
      </div>

      <!-- Additional Stats -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div
          class="card bg-base-100 border border-gray-200 dark:border-gray-700 shadow"
        >
          <div class="card-body">
            <h3 class="card-title text-gray-900 dark:text-gray-100">
              <i class="bi bi-trophy text-yellow-500" aria-hidden="true"></i>
              {tr("largestFile")}
            </h3>
            <p class="text-3xl font-bold text-gray-700 dark:text-gray-300">
              {(overview.largest_file_bytes / 1024 / 1024).toFixed(2)} MB
            </p>
          </div>
        </div>

        {#if overview.storage_limit_bytes}
          <div
            class="card bg-base-100 border border-gray-200 dark:border-gray-700 shadow"
          >
            <div class="card-body">
              <h3 class="card-title text-gray-900 dark:text-gray-100">
                <i class="bi bi-speedometer2 text-red-500" aria-hidden="true"></i>
                {tr("storageUsage")}
              </h3>
              <div class="flex items-center gap-3">
                <progress
                  class="progress progress-primary w-full"
                  value={overview.usage_percentage}
                  max="100"
                ></progress>
                <span
                  class="text-xl font-bold text-gray-700 dark:text-gray-300"
                >
                  {overview.usage_percentage.toFixed(1)}%
                </span>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Users Tab -->
    {#if activeTab === "users"}
      <div
        class="card bg-base-100 border border-gray-200 dark:border-gray-700 shadow"
      >
        <div class="card-body">
          <h2 class="card-title text-gray-900 dark:text-gray-100">
            <i class="bi bi-people" aria-hidden="true"></i>
            {tr("storageByUser")}
          </h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{tr("username")}</th>
                  <th>{tr("files")}</th>
                  <th>{tr("storage")}</th>
                  <th>{tr("lastUpload")}</th>
                </tr>
              </thead>
              <tbody>
                {#each userStats as user}
                  <tr>
                    <td class="font-medium text-gray-900 dark:text-gray-100"
                      >{user.username}</td
                    >
                    <td>{user.total_files.toLocaleString()}</td>
                    <td class="font-mono">{user.total_size_formatted}</td>
                    <td class="text-sm text-gray-600 dark:text-gray-400"
                      >{formatDate(user.last_upload)}</td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}

    <!-- Folders Tab -->
    {#if activeTab === "folders"}
      <div
        class="card bg-base-100 border border-gray-200 dark:border-gray-700 shadow"
      >
        <div class="card-body">
          <h2 class="card-title text-gray-900 dark:text-gray-100">
            <i class="bi bi-folder" aria-hidden="true"></i>
            {tr("storageByFolder")}
          </h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{tr("folder")}</th>
                  <th>{tr("files")}</th>
                  <th>{tr("storage")}</th>
                </tr>
              </thead>
              <tbody>
                {#each folderStats as folder}
                  <tr>
                    <td class="font-medium text-gray-900 dark:text-gray-100">
                      <i class="bi bi-folder text-yellow-500 mr-2" aria-hidden="true"></i>
                      {folder.folder_path || "(Root)"}
                    </td>
                    <td>{folder.file_count.toLocaleString()}</td>
                    <td class="font-mono">{folder.total_size_formatted}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}

    <!-- Top Files Tab -->
    {#if activeTab === "top-files"}
      <div
        class="card bg-base-100 border border-gray-200 dark:border-gray-700 shadow"
      >
        <div class="card-body">
          <h2 class="card-title text-gray-900 dark:text-gray-100">
            <i class="bi bi-file-earmark-fill" aria-hidden="true"></i>
            {tr("topLargestFiles")}
          </h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>#</th>
                  <th>{tr("fileName")}</th>
                  <th>{tr("size")}</th>
                  <th>{tr("type")}</th>
                  <th>{tr("owner")}</th>
                  <th>{tr("created")}</th>
                </tr>
              </thead>
              <tbody>
                {#each topFiles as file, index}
                  <tr>
                    <td class="text-gray-500 dark:text-gray-400">{index + 1}</td
                    >
                    <td
                      class="font-medium text-gray-900 dark:text-gray-100 max-w-xs truncate"
                      title={file.file_path}
                    >
                      {file.file_name}
                    </td>
                    <td
                      class="font-mono font-bold text-red-600 dark:text-red-400"
                      >{file.size_formatted}</td
                    >
                    <td class="text-sm text-gray-600 dark:text-gray-400"
                      >{file.mime_type || "-"}</td
                    >
                    <td class="text-sm text-gray-600 dark:text-gray-400"
                      >{file.owner_username}</td
                    >
                    <td class="text-sm text-gray-600 dark:text-gray-400"
                      >{formatDate(file.created_at)}</td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}

    <!-- Growth Tab -->
    {#if activeTab === "growth"}
      <div
        class="card bg-base-100 border border-gray-200 dark:border-gray-700 shadow"
      >
        <div class="card-body">
          <h2 class="card-title text-gray-900 dark:text-gray-100">
            <i class="bi bi-graph-up" aria-hidden="true"></i>
            {tr("storageGrowth")}
          </h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{tr("date")}</th>
                  <th>{tr("newFiles")}</th>
                  <th>{tr("totalFiles")}</th>
                  <th>{tr("totalStorage")}</th>
                </tr>
              </thead>
              <tbody>
                {#each growth as point}
                  <tr>
                    <td class="font-medium text-gray-900 dark:text-gray-100"
                      >{point.period}</td
                    >
                    <td class="text-green-600 dark:text-green-400"
                      >+{point.new_files}</td
                    >
                    <td>{point.file_count.toLocaleString()}</td>
                    <td class="font-mono"
                      >{(point.total_size_bytes / 1024 / 1024).toFixed(2)} MB</td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}

    <!-- Duplicates Tab -->
    {#if activeTab === "duplicates" && duplicateWaste}
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div
          class="card bg-gradient-to-br from-red-500 to-red-600 text-white shadow-lg"
        >
          <div class="card-body">
            <i class="bi bi-files text-4xl opacity-80" aria-hidden="true"></i>
            <h2 class="card-title text-4xl font-bold">
              {duplicateWaste.total_duplicates.toLocaleString()}
            </h2>
            <p class="opacity-90">{tr("totalDuplicates")}</p>
          </div>
        </div>

        <div
          class="card bg-gradient-to-br from-orange-500 to-orange-600 text-white shadow-lg"
        >
          <div class="card-body">
            <i class="bi bi-exclamation-triangle text-4xl opacity-80" aria-hidden="true"></i>
            <h2 class="card-title text-3xl font-bold">
              {duplicateWaste.wasted_formatted}
            </h2>
            <p class="opacity-90">{tr("wastedSpace")}</p>
          </div>
        </div>

        <div
          class="card bg-gradient-to-br from-green-500 to-green-600 text-white shadow-lg"
        >
          <div class="card-body">
            <i class="bi bi-piggy-bank text-4xl opacity-80" aria-hidden="true"></i>
            <h2 class="card-title text-3xl font-bold">
              {duplicateWaste.savings_potential_formatted}
            </h2>
            <p class="opacity-90">{tr("savingsPotential")}</p>
          </div>
        </div>
      </div>

      <div class="alert alert-info">
        <i class="bi bi-info-circle" aria-hidden="true"></i>
        <div>
          <p class="font-medium">{tr("duplicateInfo")}</p>
          <p class="text-sm opacity-80">{tr("duplicateInfoDescription")}</p>
        </div>
      </div>
    {/if}
  {/if}
</div>
