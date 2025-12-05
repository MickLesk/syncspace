<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";

  let overview = $state(null);
  let userStats = $state([]);
  let folderStats = $state([]);
  let topFiles = $state([]);
  let growth = $state([]);
  let duplicateWaste = $state(null);
  let loading = $state(true);
  let error = $state("");
  let activeTab = $state("overview");

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const tabs = [
    { id: "overview", icon: "bi-pie-chart", label: "overview" },
    { id: "users", icon: "bi-people", label: "byUser" },
    { id: "folders", icon: "bi-folder", label: "byFolder" },
    { id: "top-files", icon: "bi-file-earmark-fill", label: "topFiles" },
    { id: "growth", icon: "bi-graph-up", label: "growth" },
    { id: "duplicates", icon: "bi-files", label: "duplicates" },
  ];

  onMount(async () => {
    await loadAllData();
  });

  async function loadAllData() {
    loading = true;
    error = "";
    try {
      const results = await Promise.allSettled([
        api.storageAnalytics.getOverview(),
        api.storageAnalytics.getByUser(),
        api.storageAnalytics.getByFolder(),
        api.storageAnalytics.getTopFiles(100),
        api.storageAnalytics.getGrowth(30),
        api.storageAnalytics.getDuplicateWaste(),
      ]);
      if (results[0].status === "fulfilled") overview = results[0].value;
      if (results[1].status === "fulfilled") userStats = results[1].value || [];
      if (results[2].status === "fulfilled")
        folderStats = results[2].value || [];
      if (results[3].status === "fulfilled") topFiles = results[3].value || [];
      if (results[4].status === "fulfilled") growth = results[4].value || [];
      if (results[5].status === "fulfilled") duplicateWaste = results[5].value;
      const allFailed = results.every((r) => r.status === "rejected");
      if (allFailed) error = tr("failedToLoadAnalytics");
    } catch (err) {
      console.error("Failed to load storage analytics:", err);
      error = tr("failedToLoadAnalytics");
    } finally {
      loading = false;
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    try {
      const date = new Date(dateStr);
      if (isNaN(date.getTime())) return "-";
      return date.toLocaleDateString(
        $currentLang === "de" ? "de-DE" : "en-US",
        {
          year: "numeric",
          month: "short",
          day: "numeric",
        }
      );
    } catch {
      return "-";
    }
  }

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${units[i]}`;
  }

  function getFileIcon(mimeType) {
    if (!mimeType) return "bi-file-earmark";
    if (mimeType.startsWith("image/")) return "bi-file-earmark-image";
    if (mimeType.startsWith("video/")) return "bi-file-earmark-play";
    if (mimeType.startsWith("audio/")) return "bi-file-earmark-music";
    if (mimeType.includes("pdf")) return "bi-file-earmark-pdf";
    if (mimeType.includes("zip") || mimeType.includes("archive"))
      return "bi-file-earmark-zip";
    return "bi-file-earmark";
  }
</script>

<PageWrapper title={tr("storageAnalytics")} showSidebar={true}>
  <div class="analytics">
    <div class="analytics-header">
      <h1 class="analytics-title">
        <i class="bi bi-bar-chart-line-fill"></i>
        {tr("storageAnalytics")}
      </h1>
      <button onclick={loadAllData} disabled={loading} class="btn-primary">
        <i class="bi bi-arrow-clockwise {loading ? 'spinning' : ''}"></i>
        {tr("refresh")}
      </button>
    </div>

    {#if error}
      <div class="error-banner">
        <i class="bi bi-exclamation-triangle"></i>
        <span>{error}</span>
      </div>
    {/if}

    {#if loading}
      <div class="loading-container">
        <div class="spinner"></div>
      </div>
    {:else}
      <div class="tabs-container">
        {#each tabs as tab}
          <button
            onclick={() => (activeTab = tab.id)}
            class="tab-btn"
            class:active={activeTab === tab.id}
          >
            <i class="bi {tab.icon}"></i>
            <span>{tr(tab.label)}</span>
          </button>
        {/each}
      </div>

      {#if activeTab === "overview"}
        {#if overview}
          <div class="card storage-card">
            <div class="card-header">
              <div class="card-icon storage-icon">
                <i class="bi bi-hdd-stack"></i>
              </div>
              <h2>{tr("storageUsage")}</h2>
              <span class="storage-total"
                >{overview.total_size_formatted || "0 B"}</span
              >
            </div>
            {#if overview.storage_limit_bytes}
              <div class="storage-bar-container">
                <div
                  class="storage-bar"
                  style="width: {Math.min(
                    overview.usage_percentage || 0,
                    100
                  )}%"
                ></div>
              </div>
              <div class="storage-info">
                <span>{overview.total_size_formatted} {tr("used")}</span>
                <span>{overview.usage_percentage?.toFixed(1) || 0}%</span>
              </div>
            {/if}
          </div>
          <div class="quick-stats">
            <div class="stat-card">
              <div class="stat-icon files-icon">
                <i class="bi bi-files"></i>
              </div>
              <div class="stat-text">
                <h3>{overview.total_files?.toLocaleString() || 0}</h3>
                <p>{tr("totalFiles")}</p>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon users-icon">
                <i class="bi bi-people"></i>
              </div>
              <div class="stat-text">
                <h3>{overview.active_users?.toLocaleString() || 0}</h3>
                <p>{tr("activeUsers")}</p>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon size-icon">
                <i class="bi bi-file-earmark-bar-graph"></i>
              </div>
              <div class="stat-text">
                <h3>{formatBytes(overview.avg_file_size_bytes || 0)}</h3>
                <p>{tr("avgFileSize")}</p>
              </div>
            </div>
            <div class="stat-card">
              <div class="stat-icon trophy-icon">
                <i class="bi bi-trophy"></i>
              </div>
              <div class="stat-text">
                <h3>{formatBytes(overview.largest_file_bytes || 0)}</h3>
                <p>{tr("largestFile")}</p>
              </div>
            </div>
          </div>
        {:else}
          <div class="card empty-state">
            <i class="bi bi-inbox"></i>
            <p>{tr("noDataAvailable")}</p>
          </div>
        {/if}
      {/if}

      {#if activeTab === "users"}
        <div class="card table-card">
          <div class="card-header-row">
            <h2><i class="bi bi-people"></i> {tr("storageByUser")}</h2>
          </div>
          {#if userStats.length > 0}
            <div class="table-container">
              <table>
                <thead
                  ><tr
                    ><th>{tr("username")}</th><th>{tr("files")}</th><th
                      >{tr("storage")}</th
                    ><th>{tr("lastUpload")}</th></tr
                  ></thead
                >
                <tbody>
                  {#each userStats as user}
                    <tr>
                      <td class="name-cell"
                        ><div class="user-avatar">
                          {(user.username || "?").charAt(0).toUpperCase()}
                        </div>
                        {user.username || tr("unknown")}</td
                      >
                      <td
                        ><span class="badge blue"
                          >{user.total_files?.toLocaleString() || 0}</span
                        ></td
                      >
                      <td class="mono">{user.total_size_formatted || "0 B"}</td>
                      <td class="muted">{formatDate(user.last_upload)}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <div class="empty-state">
              <i class="bi bi-people"></i>
              <p>{tr("noUsersFound")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "folders"}
        <div class="card table-card">
          <div class="card-header-row">
            <h2><i class="bi bi-folder"></i> {tr("storageByFolder")}</h2>
          </div>
          {#if folderStats.length > 0}
            <div class="table-container">
              <table>
                <thead
                  ><tr
                    ><th>{tr("folder")}</th><th>{tr("files")}</th><th
                      >{tr("storage")}</th
                    ><th>{tr("percentage")}</th></tr
                  ></thead
                >
                <tbody>
                  {#each folderStats as folder}
                    {@const totalSize = folderStats.reduce(
                      (sum, f) => sum + (f.total_size_bytes || 0),
                      0
                    )}
                    {@const percentage =
                      totalSize > 0
                        ? ((folder.total_size_bytes || 0) / totalSize) * 100
                        : 0}
                    <tr>
                      <td class="name-cell"
                        ><i class="bi bi-folder-fill folder-icon"
                        ></i>{folder.folder || "/"}</td
                      >
                      <td
                        ><span class="badge amber"
                          >{folder.file_count?.toLocaleString() || 0}</span
                        ></td
                      >
                      <td class="mono"
                        >{folder.total_size_formatted || "0 B"}</td
                      >
                      <td
                        ><div class="progress-cell">
                          <div class="progress-bar">
                            <div
                              class="progress-fill amber"
                              style="width: {percentage}%"
                            ></div>
                          </div>
                          <span class="muted">{percentage.toFixed(1)}%</span>
                        </div></td
                      >
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <div class="empty-state">
              <i class="bi bi-folder"></i>
              <p>{tr("noFoldersFound")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "top-files"}
        <div class="card table-card">
          <div class="card-header-row">
            <h2>
              <i class="bi bi-file-earmark-fill"></i>
              {tr("topLargestFiles")}
            </h2>
          </div>
          {#if topFiles.length > 0}
            <div class="table-container">
              <table>
                <thead
                  ><tr
                    ><th>#</th><th>{tr("fileName")}</th><th>{tr("size")}</th><th
                      >{tr("type")}</th
                    ><th>{tr("created")}</th></tr
                  ></thead
                >
                <tbody>
                  {#each topFiles.slice(0, 50) as file, index}
                    <tr>
                      <td
                        ><span class="rank" class:top={index < 3}
                          >{index + 1}</span
                        ></td
                      >
                      <td class="name-cell">
                        <i class="bi {getFileIcon(file.mime_type)} file-icon"
                        ></i>
                        <div class="file-info">
                          <span class="filename" title={file.filename}
                            >{file.filename}</span
                          >
                          <span class="filepath muted" title={file.file_path}
                            >{file.file_path}</span
                          >
                        </div>
                      </td>
                      <td class="mono highlight"
                        >{file.size_formatted ||
                          formatBytes(file.size_bytes)}</td
                      >
                      <td class="muted">{file.mime_type || "-"}</td>
                      <td class="muted">{formatDate(file.created_at)}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
            {#if topFiles.length > 50}
              <div class="table-footer">
                {tr("showingTop50of")}
                {topFiles.length}
                {tr("files")}
              </div>
            {/if}
          {:else}
            <div class="empty-state">
              <i class="bi bi-file-earmark"></i>
              <p>{tr("noFilesFound")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "growth"}
        <div class="card table-card">
          <div class="card-header-row">
            <h2>
              <i class="bi bi-graph-up"></i>
              {tr("storageGrowth")}
              <span class="subtitle">({tr("last30Days")})</span>
            </h2>
          </div>
          {#if growth.length > 0}
            <div class="chart-container">
              <div class="bar-chart">
                {#each growth as point}
                  {@const maxSize = Math.max(
                    ...growth.map((g) => g.size_added_bytes || 0),
                    1
                  )}
                  {@const height =
                    ((point.size_added_bytes || 0) / maxSize) * 100}
                  <div
                    class="bar"
                    style="height: {Math.max(height, 2)}%"
                    title="{point.period}: +{point.files_added} files"
                  >
                    <div class="tooltip">
                      {point.period}<br />+{point.files_added} files<br
                      />+{point.size_added_formatted}
                    </div>
                  </div>
                {/each}
              </div>
              <div class="chart-labels">
                <span>{growth[0]?.period || ""}</span><span
                  >{growth[growth.length - 1]?.period || ""}</span
                >
              </div>
            </div>
            <div class="table-container scrollable">
              <table>
                <thead
                  ><tr
                    ><th>{tr("date")}</th><th>{tr("newFiles")}</th><th
                      >{tr("sizeAdded")}</th
                    ></tr
                  ></thead
                >
                <tbody>
                  {#each growth.slice().reverse() as point}
                    <tr>
                      <td>{point.period}</td>
                      <td
                        ><span class="badge green"
                          >+{point.files_added?.toLocaleString() || 0}</span
                        ></td
                      >
                      <td class="mono">{point.size_added_formatted || "0 B"}</td
                      >
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <div class="empty-state">
              <i class="bi bi-graph-up"></i>
              <p>{tr("noGrowthData")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "duplicates"}
        {#if duplicateWaste}
          <div class="duplicate-stats">
            <div class="dup-card red">
              <div class="dup-icon"><i class="bi bi-files"></i></div>
              <div class="dup-value">
                {duplicateWaste.duplicate_groups?.toLocaleString() || 0}
              </div>
              <div class="dup-label">{tr("duplicateGroups")}</div>
            </div>
            <div class="dup-card amber">
              <div class="dup-icon">
                <i class="bi bi-exclamation-triangle"></i>
              </div>
              <div class="dup-value">
                {duplicateWaste.wasted_formatted || "0 B"}
              </div>
              <div class="dup-label">{tr("wastedSpace")}</div>
            </div>
            <div class="dup-card green">
              <div class="dup-icon"><i class="bi bi-piggy-bank"></i></div>
              <div class="dup-value">
                {duplicateWaste.savings_potential_formatted || "0 B"}
              </div>
              <div class="dup-label">{tr("savingsPotential")}</div>
            </div>
          </div>
          <div class="card info-card">
            <div class="info-icon"><i class="bi bi-info-circle"></i></div>
            <div class="info-content">
              <h3>{tr("duplicateInfo")}</h3>
              <p>{tr("duplicateInfoDescription")}</p>
              <button
                class="btn-outline"
                onclick={() => {
                  window.location.hash = "#/duplicates";
                }}
              >
                <i class="bi bi-search"></i>
                {tr("viewDuplicates")}
              </button>
            </div>
          </div>
        {:else}
          <div class="card empty-state">
            <i class="bi bi-files"></i>
            <p>{tr("noDuplicatesFound")}</p>
          </div>
        {/if}
      {/if}
    {/if}
  </div>
</PageWrapper>

<style>
  .analytics {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1.5rem;
  }
  .analytics-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  .analytics-title {
    font-size: 1.5rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #111827;
  }
  :global(.dark) .analytics-title {
    color: #f9fafb;
  }
  .error-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 0.75rem;
    color: #dc2626;
    margin-bottom: 1.5rem;
  }
  :global(.dark) .error-banner {
    background: rgba(220, 38, 38, 0.1);
    border-color: rgba(220, 38, 38, 0.3);
    color: #fca5a5;
  }
  .loading-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 400px;
  }
  .spinner {
    width: 40px;
    height: 40px;
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
  .btn-primary {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #22c55e;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }
  .btn-primary:hover {
    background: #16a34a;
  }
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .btn-outline {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: transparent;
    color: #3b82f6;
    border: 1px solid #3b82f6;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }
  .btn-outline:hover {
    background: #3b82f6;
    color: white;
  }
  .tabs-container {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.5rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    margin-bottom: 1.5rem;
  }
  :global(.dark) .tabs-container {
    background: #1f2937;
    border-color: #374151;
  }
  .tab-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: transparent;
    border: none;
    border-radius: 0.5rem;
    color: #6b7280;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }
  .tab-btn:hover {
    background: #f3f4f6;
  }
  :global(.dark) .tab-btn {
    color: #9ca3af;
  }
  :global(.dark) .tab-btn:hover {
    background: #374151;
  }
  .tab-btn.active {
    background: #22c55e;
    color: white;
  }
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    margin-bottom: 1.5rem;
  }
  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }
  .storage-card {
    padding: 1.25rem;
  }
  .card-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  .card-header h2 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    flex: 1;
  }
  :global(.dark) .card-header h2 {
    color: #f9fafb;
  }
  .card-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
  }
  .storage-icon {
    background: #dcfce7;
    color: #22c55e;
  }
  :global(.dark) .storage-icon {
    background: rgba(34, 197, 94, 0.2);
  }
  .storage-total {
    font-size: 0.875rem;
    color: #6b7280;
  }
  :global(.dark) .storage-total {
    color: #9ca3af;
  }
  .storage-bar-container {
    height: 8px;
    background: #e5e7eb;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.75rem;
  }
  :global(.dark) .storage-bar-container {
    background: #374151;
  }
  .storage-bar {
    height: 100%;
    background: linear-gradient(90deg, #22c55e, #16a34a);
    border-radius: 4px;
    transition: width 0.5s ease;
  }
  .storage-info {
    display: flex;
    justify-content: space-between;
    font-size: 0.875rem;
    color: #6b7280;
  }
  :global(.dark) .storage-info {
    color: #9ca3af;
  }
  .quick-stats {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  @media (max-width: 1024px) {
    .quick-stats {
      grid-template-columns: repeat(2, 1fr);
    }
  }
  @media (max-width: 640px) {
    .quick-stats {
      grid-template-columns: 1fr;
    }
  }
  .stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
  }
  :global(.dark) .stat-card {
    background: #1f2937;
    border-color: #374151;
  }
  .stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
  }
  .files-icon {
    background: #dbeafe;
    color: #3b82f6;
  }
  .users-icon {
    background: #dcfce7;
    color: #22c55e;
  }
  .size-icon {
    background: #fef3c7;
    color: #f59e0b;
  }
  .trophy-icon {
    background: #fce7f3;
    color: #ec4899;
  }
  :global(.dark) .files-icon {
    background: rgba(59, 130, 246, 0.2);
  }
  :global(.dark) .users-icon {
    background: rgba(34, 197, 94, 0.2);
  }
  :global(.dark) .size-icon {
    background: rgba(245, 158, 11, 0.2);
  }
  :global(.dark) .trophy-icon {
    background: rgba(236, 72, 153, 0.2);
  }
  .stat-text h3 {
    font-size: 1.5rem;
    font-weight: 700;
    color: #111827;
    margin-bottom: 0.25rem;
  }
  :global(.dark) .stat-text h3 {
    color: #f9fafb;
  }
  .stat-text p {
    font-size: 0.875rem;
    color: #6b7280;
  }
  :global(.dark) .stat-text p {
    color: #9ca3af;
  }
  .table-card {
    overflow: hidden;
  }
  .card-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }
  :global(.dark) .card-header-row {
    border-color: #374151;
  }
  .card-header-row h2 {
    font-size: 1rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #111827;
  }
  :global(.dark) .card-header-row h2 {
    color: #f9fafb;
  }
  .card-header-row .subtitle {
    font-weight: 400;
    color: #6b7280;
  }
  :global(.dark) .card-header-row .subtitle {
    color: #9ca3af;
  }
  .table-container {
    overflow-x: auto;
  }
  .table-container.scrollable {
    max-height: 320px;
    overflow-y: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
  }
  thead {
    background: #f9fafb;
  }
  :global(.dark) thead {
    background: #111827;
  }
  th {
    padding: 0.75rem 1rem;
    text-align: left;
    font-size: 0.75rem;
    font-weight: 600;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  :global(.dark) th {
    color: #9ca3af;
  }
  td {
    padding: 0.875rem 1rem;
    font-size: 0.875rem;
    color: #374151;
    border-top: 1px solid #e5e7eb;
  }
  :global(.dark) td {
    color: #d1d5db;
    border-color: #374151;
  }
  tr:hover td {
    background: #f9fafb;
  }
  :global(.dark) tr:hover td {
    background: #111827;
  }
  .name-cell {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-weight: 500;
    color: #111827;
  }
  :global(.dark) .name-cell {
    color: #f9fafb;
  }
  .user-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: linear-gradient(135deg, #22c55e, #16a34a);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 0.875rem;
  }
  .folder-icon {
    color: #f59e0b;
    font-size: 1.25rem;
  }
  .file-icon {
    font-size: 1.25rem;
    color: #6b7280;
  }
  .file-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .filename {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 200px;
  }
  .filepath {
    font-size: 0.75rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 200px;
  }
  .mono {
    font-family: ui-monospace, monospace;
    font-weight: 600;
  }
  .highlight {
    color: #dc2626;
  }
  :global(.dark) .highlight {
    color: #f87171;
  }
  .muted {
    color: #6b7280;
  }
  :global(.dark) .muted {
    color: #9ca3af;
  }
  .badge {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 500;
  }
  .badge.blue {
    background: #dbeafe;
    color: #1d4ed8;
  }
  .badge.green {
    background: #dcfce7;
    color: #16a34a;
  }
  .badge.amber {
    background: #fef3c7;
    color: #d97706;
  }
  :global(.dark) .badge.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #93c5fd;
  }
  :global(.dark) .badge.green {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }
  :global(.dark) .badge.amber {
    background: rgba(245, 158, 11, 0.2);
    color: #fcd34d;
  }
  .rank {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 0.375rem;
    font-weight: 600;
    font-size: 0.875rem;
    background: #f3f4f6;
    color: #6b7280;
  }
  :global(.dark) .rank {
    background: #374151;
    color: #9ca3af;
  }
  .rank.top {
    background: linear-gradient(135deg, #fbbf24, #f59e0b);
    color: white;
  }
  .progress-cell {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .progress-bar {
    flex: 1;
    height: 6px;
    background: #e5e7eb;
    border-radius: 3px;
    overflow: hidden;
    max-width: 100px;
  }
  :global(.dark) .progress-bar {
    background: #374151;
  }
  .progress-fill {
    height: 100%;
    border-radius: 3px;
  }
  .progress-fill.amber {
    background: linear-gradient(90deg, #fbbf24, #f59e0b);
  }
  .table-footer {
    padding: 0.75rem 1rem;
    text-align: center;
    font-size: 0.875rem;
    color: #6b7280;
    border-top: 1px solid #e5e7eb;
  }
  :global(.dark) .table-footer {
    color: #9ca3af;
    border-color: #374151;
  }
  .chart-container {
    padding: 1.5rem;
    border-bottom: 1px solid #e5e7eb;
  }
  :global(.dark) .chart-container {
    border-color: #374151;
  }
  .bar-chart {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 160px;
  }
  .bar {
    flex: 1;
    background: linear-gradient(to top, #22c55e, #4ade80);
    border-radius: 4px 4px 0 0;
    position: relative;
    cursor: pointer;
    transition: opacity 0.2s;
  }
  .bar:hover {
    opacity: 0.8;
  }
  .bar .tooltip {
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    margin-bottom: 8px;
    padding: 0.5rem;
    background: #1f2937;
    color: white;
    font-size: 0.75rem;
    border-radius: 0.375rem;
    white-space: nowrap;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s;
    z-index: 10;
  }
  .bar:hover .tooltip {
    opacity: 1;
  }
  .chart-labels {
    display: flex;
    justify-content: space-between;
    margin-top: 0.5rem;
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .chart-labels {
    color: #9ca3af;
  }
  .duplicate-stats {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  @media (max-width: 768px) {
    .duplicate-stats {
      grid-template-columns: 1fr;
    }
  }
  .dup-card {
    padding: 1.5rem;
    border-radius: 0.75rem;
    color: white;
    text-align: center;
  }
  .dup-card.red {
    background: linear-gradient(135deg, #f43f5e, #e11d48);
  }
  .dup-card.amber {
    background: linear-gradient(135deg, #f59e0b, #d97706);
  }
  .dup-card.green {
    background: linear-gradient(135deg, #22c55e, #16a34a);
  }
  .dup-icon {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    opacity: 0.9;
  }
  .dup-value {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }
  .dup-label {
    font-size: 0.875rem;
    opacity: 0.9;
  }
  .info-card {
    display: flex;
    gap: 1rem;
    padding: 1.5rem;
    background: #eff6ff;
    border-color: #bfdbfe;
  }
  :global(.dark) .info-card {
    background: rgba(59, 130, 246, 0.1);
    border-color: rgba(59, 130, 246, 0.3);
  }
  .info-icon {
    width: 48px;
    height: 48px;
    border-radius: 0.5rem;
    background: linear-gradient(135deg, #3b82f6, #2563eb);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    flex-shrink: 0;
  }
  .info-content h3 {
    font-weight: 600;
    color: #1e40af;
    margin-bottom: 0.25rem;
  }
  :global(.dark) .info-content h3 {
    color: #93c5fd;
  }
  .info-content p {
    font-size: 0.875rem;
    color: #1d4ed8;
    margin-bottom: 0.75rem;
  }
  :global(.dark) .info-content p {
    color: #bfdbfe;
  }
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    color: #6b7280;
  }
  :global(.dark) .empty-state {
    color: #9ca3af;
  }
  .empty-state i {
    font-size: 3rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }
</style>
