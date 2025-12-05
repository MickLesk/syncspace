<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

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
  <div class="loading-container"><div class="spinner"></div></div>
{:else}
  {#if success}
    <div class="toast success">
      <i class="bi bi-check-circle-fill"></i>
      {success}
    </div>
  {/if}

  <!-- Quick Stats -->
  <div class="stats-grid">
    <div class="stat-card">
      <div class="stat-icon blue"><i class="bi bi-hdd-stack-fill"></i></div>
      <div class="stat-content">
        <span class="stat-value">{formatSize(totalSpace)}</span>
        <span class="stat-label">{tr("totalStorage")}</span>
      </div>
    </div>
    <div class="stat-card">
      <div class="stat-icon purple"><i class="bi bi-pie-chart-fill"></i></div>
      <div class="stat-content">
        <span class="stat-value">{formatSize(usedSpace)}</span>
        <span class="stat-label">{tr("usedStorage")}</span>
      </div>
    </div>
    <div class="stat-card">
      <div class="stat-icon green"><i class="bi bi-check-circle-fill"></i></div>
      <div class="stat-content">
        <span class="stat-value">{formatSize(freeSpace)}</span>
        <span class="stat-label">{tr("availableStorage")}</span>
      </div>
    </div>
    <div class="stat-card">
      <div class="stat-icon amber"><i class="bi bi-files"></i></div>
      <div class="stat-content">
        <span class="stat-value">{fileCount.toLocaleString()}</span>
        <span class="stat-label">{tr("totalFiles")}</span>
      </div>
    </div>
  </div>

  <div class="storage-grid">
    <!-- Storage Bar Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon blue"><i class="bi bi-speedometer2"></i></div>
        <div class="card-title">
          <h3>{tr("storageUsage")}</h3>
          <p>{tr("currentStorageUsage")}</p>
        </div>
      </div>
      <div class="storage-visual">
        <div class="storage-bar-container">
          <div class="storage-bar" style="width: {usagePercent}%"></div>
        </div>
        <div class="storage-labels">
          <span>{formatSize(usedSpace)} {tr("used")}</span>
          <span class="percent">{usagePercent.toFixed(1)}%</span>
          <span>{formatSize(freeSpace)} {tr("free")}</span>
        </div>
      </div>
    </div>

    <!-- Configuration Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon green"><i class="bi bi-gear-fill"></i></div>
        <div class="card-title">
          <h3>{tr("storageConfiguration")}</h3>
          <p>{tr("manageStorageSettings")}</p>
        </div>
      </div>
      <div class="config-list">
        <div class="config-item">
          <div class="config-label">
            <i class="bi bi-folder-fill"></i>
            <span>{tr("storageLocation")}</span>
          </div>
          <code class="config-value">{storageLocation}</code>
        </div>
        <div class="config-item">
          <div class="config-label">
            <i class="bi bi-file-earmark-arrow-up"></i>
            <span>{tr("maxFileSize")}</span>
          </div>
          <div class="config-input">
            <input type="number" bind:value={maxFileSize} min="1" max="10000" />
            <span>MB</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Cache Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon amber"><i class="bi bi-archive-fill"></i></div>
        <div class="card-title">
          <h3>{tr("cache")}</h3>
          <p>{tr("manageApplicationCache")}</p>
        </div>
      </div>
      <div class="cache-content">
        <div class="cache-info">
          <div class="cache-size">
            <span class="value">{cacheSize}</span>
            <span class="unit">MB</span>
          </div>
          <span class="cache-label">{tr("currentCacheSize")}</span>
        </div>
        <button
          class="btn-danger"
          onclick={clearCache}
          disabled={clearing || cacheSize === 0}
        >
          <i class="bi {clearing ? 'bi-arrow-clockwise spinning' : 'bi-trash3'}"
          ></i>
          {clearing ? tr("clearing") : tr("clearCache")}
        </button>
      </div>
    </div>

    <!-- Cleanup Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon rose"><i class="bi bi-trash3-fill"></i></div>
        <div class="card-title">
          <h3>{tr("cleanup")}</h3>
          <p>{tr("freeUpSpace")}</p>
        </div>
      </div>
      <div class="cleanup-list">
        <div class="cleanup-item">
          <div class="cleanup-info">
            <i class="bi bi-clock-history"></i>
            <div class="cleanup-text">
              <span class="cleanup-title">{tr("oldVersions")}</span>
              <span class="cleanup-desc">{tr("removeOldFileVersions")}</span>
            </div>
          </div>
          <button class="btn-outline-sm">{tr("clean")}</button>
        </div>
        <div class="cleanup-item">
          <div class="cleanup-info">
            <i class="bi bi-files"></i>
            <div class="cleanup-text">
              <span class="cleanup-title">{tr("duplicates")}</span>
              <span class="cleanup-desc">{tr("findAndRemoveDuplicates")}</span>
            </div>
          </div>
          <button class="btn-outline-sm">{tr("scan")}</button>
        </div>
        <div class="cleanup-item">
          <div class="cleanup-info">
            <i class="bi bi-trash3"></i>
            <div class="cleanup-text">
              <span class="cleanup-title">{tr("emptyTrash")}</span>
              <span class="cleanup-desc"
                >{tr("permanentlyDeleteTrashItems")}</span
              >
            </div>
          </div>
          <button class="btn-outline-sm">{tr("empty")}</button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .loading-container {
    display: flex;
    justify-content: center;
    padding: 4rem;
  }
  .spinner {
    width: 36px;
    height: 36px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  .spinning {
    animation: spin 1s linear infinite;
  }

  /* Toast */
  .toast {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
    font-weight: 500;
  }
  .toast.success {
    background: #dcfce7;
    color: #166534;
  }
  :global(.dark) .toast.success {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  .stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
  }
  :global(.dark) .stat-card {
    background: #1f2937;
    border-color: #374151;
  }
  .stat-icon {
    width: 44px;
    height: 44px;
    border-radius: 0.625rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
  }
  .stat-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }
  .stat-icon.purple {
    background: #f3e8ff;
    color: #9333ea;
  }
  .stat-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }
  .stat-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }
  :global(.dark) .stat-icon.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }
  :global(.dark) .stat-icon.purple {
    background: rgba(147, 51, 234, 0.2);
    color: #c084fc;
  }
  :global(.dark) .stat-icon.green {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }
  :global(.dark) .stat-icon.amber {
    background: rgba(245, 158, 11, 0.2);
    color: #fbbf24;
  }
  .stat-content {
    display: flex;
    flex-direction: column;
  }
  .stat-value {
    font-size: 1.25rem;
    font-weight: 700;
    color: #111827;
  }
  .stat-label {
    font-size: 0.75rem;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  :global(.dark) .stat-value {
    color: #f9fafb;
  }
  :global(.dark) .stat-label {
    color: #9ca3af;
  }

  /* Grid */
  .storage-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  /* Card */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
  }
  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }
  .card-header {
    display: flex;
    align-items: flex-start;
    gap: 0.875rem;
    margin-bottom: 1.25rem;
  }
  .card-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }
  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }
  .card-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }
  .card-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }
  .card-icon.rose {
    background: #ffe4e6;
    color: #e11d48;
  }
  :global(.dark) .card-icon.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }
  :global(.dark) .card-icon.green {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }
  :global(.dark) .card-icon.amber {
    background: rgba(245, 158, 11, 0.2);
    color: #fbbf24;
  }
  :global(.dark) .card-icon.rose {
    background: rgba(225, 29, 72, 0.2);
    color: #fb7185;
  }
  .card-title h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }
  .card-title p {
    font-size: 0.8125rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .card-title h3 {
    color: #f9fafb;
  }
  :global(.dark) .card-title p {
    color: #9ca3af;
  }

  /* Storage Visual */
  .storage-visual {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .storage-bar-container {
    height: 12px;
    background: #e5e7eb;
    border-radius: 6px;
    overflow: hidden;
  }
  :global(.dark) .storage-bar-container {
    background: #374151;
  }
  .storage-bar {
    height: 100%;
    background: linear-gradient(90deg, #22c55e, #16a34a);
    border-radius: 6px;
    transition: width 0.5s ease;
  }
  .storage-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.8125rem;
    color: #6b7280;
  }
  .storage-labels .percent {
    font-weight: 600;
    color: #22c55e;
  }
  :global(.dark) .storage-labels {
    color: #9ca3af;
  }

  /* Config List */
  .config-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .config-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .config-item {
    background: #374151;
  }
  .config-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: #374151;
  }
  .config-label i {
    color: #6b7280;
  }
  :global(.dark) .config-label {
    color: #e5e7eb;
  }
  :global(.dark) .config-label i {
    color: #9ca3af;
  }
  .config-value {
    font-size: 0.8125rem;
    padding: 0.25rem 0.5rem;
    background: #e5e7eb;
    border-radius: 0.25rem;
    font-family: monospace;
  }
  :global(.dark) .config-value {
    background: #1f2937;
    color: #e5e7eb;
  }
  .config-input {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }
  .config-input input {
    width: 80px;
    padding: 0.375rem 0.5rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    text-align: right;
  }
  .config-input input:focus {
    outline: none;
    border-color: #22c55e;
  }
  :global(.dark) .config-input input {
    background: #1f2937;
    border-color: #4b5563;
    color: #f9fafb;
  }
  .config-input span {
    font-size: 0.8125rem;
    color: #6b7280;
  }

  /* Cache */
  .cache-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .cache-content {
    background: #374151;
  }
  .cache-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .cache-size {
    display: flex;
    align-items: baseline;
    gap: 0.25rem;
  }
  .cache-size .value {
    font-size: 1.5rem;
    font-weight: 700;
    color: #111827;
  }
  .cache-size .unit {
    font-size: 0.875rem;
    color: #6b7280;
  }
  .cache-label {
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .cache-size .value {
    color: #f9fafb;
  }
  :global(.dark) .cache-size .unit,
  :global(.dark) .cache-label {
    color: #9ca3af;
  }

  /* Cleanup */
  .cleanup-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .cleanup-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .cleanup-item {
    background: #374151;
  }
  .cleanup-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .cleanup-info > i {
    font-size: 1.125rem;
    color: #6b7280;
  }
  :global(.dark) .cleanup-info > i {
    color: #9ca3af;
  }
  .cleanup-text {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .cleanup-title {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }
  .cleanup-desc {
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .cleanup-title {
    color: #f9fafb;
  }
  :global(.dark) .cleanup-desc {
    color: #9ca3af;
  }

  /* Buttons */
  .btn-danger {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #dc2626;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 500;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-danger:hover:not(:disabled) {
    background: #b91c1c;
  }
  .btn-danger:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-outline-sm {
    padding: 0.375rem 0.75rem;
    background: transparent;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }
  .btn-outline-sm:hover {
    background: #f3f4f6;
    border-color: #d1d5db;
  }
  :global(.dark) .btn-outline-sm {
    color: #e5e7eb;
    border-color: #4b5563;
  }
  :global(.dark) .btn-outline-sm:hover {
    background: #4b5563;
  }

  @media (max-width: 768px) {
    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
    .storage-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
