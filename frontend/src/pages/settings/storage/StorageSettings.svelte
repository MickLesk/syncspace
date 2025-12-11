<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import api from "../../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const tabs = [
    { id: "general", icon: "hdd-fill", label: "Allgemein" },
    { id: "cloud", icon: "cloud-fill", label: "Cloudspeicher" },
    { id: "quotas", icon: "speedometer2", label: "Quotas & Rate-Limiting" },
    { id: "encryption", icon: "shield-lock-fill", label: "Verschlüsselung" },
  ];

  let activeTab = $state("general");
  let loading = $state(true);
  let clearing = $state(false);
  let storageLocation = $state("./data");
  let maxFileSize = $state(100);
  let totalSpace = $state(50);
  let usedSpace = $state(0);
  let fileCount = $state(0);
  let cacheSize = $state(0);
  let success = $state("");

  const usagePercent = $derived(
    totalSpace > 0 ? Math.min((usedSpace / totalSpace) * 100, 100) : 0
  );
  const freeSpace = $derived(Math.max(totalSpace - usedSpace, 0));

  onMount(async () => {
    await loadStorageData();
  });

  async function loadStorageData() {
    loading = true;
    try {
      const overview = await api.storageAnalytics.getOverview();
      if (overview) {
        usedSpace = (overview.total_storage_bytes || 0) / 1024 ** 3;
        fileCount = overview.total_files || 0;
        cacheSize = 245; // Mock cache size in MB
      }
    } catch (err) {
      console.error("Failed to load storage data:", err);
    } finally {
      loading = false;
    }
  }

  async function clearCache() {
    clearing = true;
    try {
      await new Promise((resolve) => setTimeout(resolve, 1000));
      cacheSize = 0;
      success = tr("cacheCleared");
      setTimeout(() => (success = ""), 3000);
    } catch (err) {
      console.error("Failed to clear cache:", err);
    } finally {
      clearing = false;
    }
  }

  function formatSize(gb) {
    if (gb < 1) return `${(gb * 1024).toFixed(0)} MB`;
    return `${gb.toFixed(2)} GB`;
  }
</script>

{#if loading}
  <div class="flex justify-center p-16">
    <span class="loading loading-spinner loading-lg text-success"></span>
  </div>
{:else}
  {#if success}
    <div
      class="flex items-center gap-2 px-4 py-3 rounded-lg mb-6 text-sm font-medium bg-green-100 text-green-800 dark:bg-green-500/20 dark:text-green-300"
    >
      <i class="bi bi-check-circle-fill"></i>
      {success}
    </div>
  {/if}

  <!-- Tab Navigation -->
  <div class="tabs-header">
    {#each tabs as tab}
      <button
        class="tab-button"
        class:active={activeTab === tab.id}
        onclick={() => (activeTab = tab.id)}
      >
        <i class="bi bi-{tab.icon}"></i>
        <span>{tab.label}</span>
      </button>
    {/each}
  </div>

  <!-- Tab Content -->
  <div class="tab-content">
    {#if activeTab === "general"}
      <!-- Quick Stats -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
        <div
          class="flex items-center gap-4 p-4 bg-gradient-to-br from-blue-500/5 to-transparent rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div
            class="w-11 h-11 rounded-lg flex items-center justify-center text-xl bg-blue-100 text-blue-600 dark:bg-blue-500/20 dark:text-blue-400"
          >
            <i class="bi bi-hdd-stack-fill"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-xl font-bold text-base-content"
              >{formatSize(totalSpace)}</span
            >
            <span class="text-xs text-base-content/60 uppercase tracking-wider"
              >{tr("totalStorage")}</span
            >
          </div>
        </div>
        <div
          class="flex items-center gap-4 p-4 bg-gradient-to-br from-purple-500/5 to-transparent rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div
            class="w-11 h-11 rounded-lg flex items-center justify-center text-xl bg-purple-100 text-purple-600 dark:bg-purple-500/20 dark:text-purple-400"
          >
            <i class="bi bi-pie-chart-fill"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-xl font-bold text-base-content"
              >{formatSize(usedSpace)}</span
            >
            <span class="text-xs text-base-content/60 uppercase tracking-wider"
              >{tr("usedStorage")}</span
            >
          </div>
        </div>
        <div
          class="flex items-center gap-4 p-4 bg-gradient-to-br from-emerald-500/5 to-transparent rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div
            class="w-11 h-11 rounded-lg flex items-center justify-center text-xl bg-green-100 text-green-600 dark:bg-green-500/20 dark:text-green-400"
          >
            <i class="bi bi-check-circle-fill"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-xl font-bold text-base-content"
              >{formatSize(freeSpace)}</span
            >
            <span class="text-xs text-base-content/60 uppercase tracking-wider"
              >{tr("availableStorage")}</span
            >
          </div>
        </div>
        <div
          class="flex items-center gap-4 p-4 bg-gradient-to-br from-amber-500/5 to-transparent rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div
            class="w-11 h-11 rounded-lg flex items-center justify-center text-xl bg-amber-100 text-amber-600 dark:bg-amber-500/20 dark:text-amber-400"
          >
            <i class="bi bi-files"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-xl font-bold text-base-content"
              >{fileCount.toLocaleString()}</span
            >
            <span class="text-xs text-base-content/60 uppercase tracking-wider"
              >{tr("totalFiles")}</span
            >
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Storage Bar Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl p-5 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div class="flex items-start gap-3.5 mb-5">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0 bg-blue-100 text-blue-600 dark:bg-blue-500/20 dark:text-blue-400"
            >
              <i class="bi bi-speedometer2"></i>
            </div>
            <div>
              <h3 class="text-base font-semibold text-base-content m-0 mb-1">
                {tr("storageUsage")}
              </h3>
              <p class="text-[0.8125rem] text-base-content/60 m-0">
                {tr("currentStorageUsage")}
              </p>
            </div>
          </div>
          <div class="flex flex-col gap-3">
            <div class="h-3 bg-base-300 rounded-md overflow-hidden">
              <div
                class="h-full bg-gradient-to-r from-success to-green-600 rounded-md transition-all duration-500"
                style="width: {usagePercent}%"
              ></div>
            </div>
            <div
              class="flex justify-between text-[0.8125rem] text-base-content/60"
            >
              <span>{formatSize(usedSpace)} {tr("used")}</span>
              <span class="font-semibold text-success"
                >{usagePercent.toFixed(1)}%</span
              >
              <span>{formatSize(freeSpace)} {tr("free")}</span>
            </div>
          </div>
        </div>

        <!-- Configuration Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl p-5 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div class="flex items-start gap-3.5 mb-5">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0 bg-green-100 text-green-600 dark:bg-green-500/20 dark:text-green-400"
            >
              <i class="bi bi-gear-fill"></i>
            </div>
            <div>
              <h3 class="text-base font-semibold text-base-content m-0 mb-1">
                {tr("storageConfiguration")}
              </h3>
              <p class="text-[0.8125rem] text-base-content/60 m-0">
                {tr("manageStorageSettings")}
              </p>
            </div>
          </div>
          <div class="flex flex-col gap-4">
            <div
              class="flex justify-between items-center gap-4 p-3 bg-base-200 rounded-lg"
            >
              <div class="flex items-center gap-2 text-sm text-base-content">
                <i class="bi bi-folder-fill text-base-content/60"></i>
                <span>{tr("storageLocation")}</span>
              </div>
              <code
                class="text-[0.8125rem] px-2 py-1 bg-base-300 rounded font-mono"
                >{storageLocation}</code
              >
            </div>
            <div
              class="flex justify-between items-center gap-4 p-3 bg-base-200 rounded-lg"
            >
              <div class="flex items-center gap-2 text-sm text-base-content">
                <i class="bi bi-file-earmark-arrow-up text-base-content/60"></i>
                <span>{tr("maxFileSize")}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <input
                  type="number"
                  class="input input-sm input-bordered w-20 text-right"
                  bind:value={maxFileSize}
                  min="1"
                  max="10000"
                />
                <span class="text-[0.8125rem] text-base-content/60">MB</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Cache Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl p-5 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div class="flex items-start gap-3.5 mb-5">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0 bg-amber-100 text-amber-600 dark:bg-amber-500/20 dark:text-amber-400"
            >
              <i class="bi bi-archive-fill"></i>
            </div>
            <div>
              <h3 class="text-base font-semibold text-base-content m-0 mb-1">
                {tr("cache")}
              </h3>
              <p class="text-[0.8125rem] text-base-content/60 m-0">
                {tr("manageApplicationCache")}
              </p>
            </div>
          </div>
          <div
            class="flex items-center justify-between gap-4 p-4 bg-base-200 rounded-lg"
          >
            <div class="flex flex-col gap-0.5">
              <div class="flex items-baseline gap-1">
                <span class="text-2xl font-bold text-base-content"
                  >{cacheSize}</span
                >
                <span class="text-sm text-base-content/60">MB</span>
              </div>
              <span class="text-xs text-base-content/60"
                >{tr("currentCacheSize")}</span
              >
            </div>
            <button
              class="btn btn-error btn-sm gap-2"
              onclick={clearCache}
              disabled={clearing || cacheSize === 0}
            >
              <i
                class="bi {clearing
                  ? 'bi-arrow-clockwise animate-spin'
                  : 'bi-trash3'}"
              ></i>
              {clearing ? tr("clearing") : tr("clearCache")}
            </button>
          </div>
        </div>

        <!-- Cleanup Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl p-5 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
        >
          <div class="flex items-start gap-3.5 mb-5">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0 bg-rose-100 text-rose-600 dark:bg-rose-500/20 dark:text-rose-400"
            >
              <i class="bi bi-trash3-fill"></i>
            </div>
            <div>
              <h3 class="text-base font-semibold text-base-content m-0 mb-1">
                {tr("cleanup")}
              </h3>
              <p class="text-[0.8125rem] text-base-content/60 m-0">
                {tr("freeUpSpace")}
              </p>
            </div>
          </div>
          <div class="flex flex-col gap-3">
            <div
              class="flex items-center justify-between gap-4 p-3 bg-base-200 rounded-lg"
            >
              <div class="flex items-center gap-3">
                <i class="bi bi-clock-history text-lg text-base-content/60"></i>
                <div class="flex flex-col gap-0.5">
                  <span class="text-sm font-medium text-base-content"
                    >{tr("oldVersions")}</span
                  >
                  <span class="text-xs text-base-content/60"
                    >{tr("removeOldFileVersions")}</span
                  >
                </div>
              </div>
              <button class="btn btn-ghost btn-sm">{tr("clean")}</button>
            </div>
            <div
              class="flex items-center justify-between gap-4 p-3 bg-base-200 rounded-lg"
            >
              <div class="flex items-center gap-3">
                <i class="bi bi-files text-lg text-base-content/60"></i>
                <div class="flex flex-col gap-0.5">
                  <span class="text-sm font-medium text-base-content"
                    >{tr("duplicates")}</span
                  >
                  <span class="text-xs text-base-content/60"
                    >{tr("findAndRemoveDuplicates")}</span
                  >
                </div>
              </div>
              <button class="btn btn-ghost btn-sm">{tr("scan")}</button>
            </div>
            <div
              class="flex items-center justify-between gap-4 p-3 bg-base-200 rounded-lg"
            >
              <div class="flex items-center gap-3">
                <i class="bi bi-trash3 text-lg text-base-content/60"></i>
                <div class="flex flex-col gap-0.5">
                  <span class="text-sm font-medium text-base-content"
                    >{tr("emptyTrash")}</span
                  >
                  <span class="text-xs text-base-content/60"
                    >{tr("permanentlyDeleteTrashItems")}</span
                  >
                </div>
              </div>
              <button class="btn btn-ghost btn-sm">{tr("empty")}</button>
            </div>
          </div>
        </div>
      </div>
    {:else if activeTab === "cloud"}
      {#await import("../admin/CloudStorageView.svelte") then module}
        <module.default />
      {/await}
    {:else if activeTab === "quotas"}
      {#await import("../system/QuotasView.svelte") then module}
        <module.default />
      {/await}
    {:else if activeTab === "encryption"}
      {#await import("../EncryptionView.svelte") then module}
        <module.default />
      {/await}
    {/if}
  </div>
{/if}

<style>
  /* Tab Navigation */
  .tabs-header {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 2px solid #e5e7eb;
    padding-bottom: 0;
  }
  :global(.dark) .tabs-header {
    border-bottom-color: #374151;
  }

  .tab-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    margin-bottom: -2px;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s;
  }
  .tab-button:hover {
    color: #111827;
    background: #f9fafb;
  }
  .tab-button.active {
    color: #22c55e;
    border-bottom-color: #22c55e;
  }
  .tab-button i {
    font-size: 1rem;
  }
  :global(.dark) .tab-button {
    color: #9ca3af;
  }
  :global(.dark) .tab-button:hover {
    color: #f9fafb;
    background: #374151;
  }
  :global(.dark) .tab-button.active {
    color: #22c55e;
  }

  /* Tab Content */
  .tab-content {
    animation: fadeIn 0.3s ease-in-out;
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
