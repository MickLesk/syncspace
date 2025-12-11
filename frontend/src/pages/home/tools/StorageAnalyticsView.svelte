<script>
  import { onMount } from "svelte";
  import api from "../../../lib/api.js";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import PageWrapper from "../../../components/PageWrapper.svelte";

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
    console.log("[StorageAnalytics] Loading data...");
    try {
      const results = await Promise.allSettled([
        api.storageAnalytics.getOverview(),
        api.storageAnalytics.getByUser(),
        api.storageAnalytics.getByFolder(),
        api.storageAnalytics.getTopFiles(100),
        api.storageAnalytics.getGrowth(30),
        api.storageAnalytics.getDuplicateWaste(),
      ]);

      console.log("[StorageAnalytics] Results:", results);

      if (results[0].status === "fulfilled") {
        overview = results[0].value;
        console.log("[StorageAnalytics] Overview:", overview);
      } else {
        console.error("[StorageAnalytics] Overview failed:", results[0].reason);
      }

      if (results[1].status === "fulfilled") userStats = results[1].value || [];
      if (results[2].status === "fulfilled")
        folderStats = results[2].value || [];
      if (results[3].status === "fulfilled") topFiles = results[3].value || [];
      if (results[4].status === "fulfilled") growth = results[4].value || [];
      if (results[5].status === "fulfilled") duplicateWaste = results[5].value;

      const allFailed = results.every((r) => r.status === "rejected");
      if (allFailed) {
        error = tr("failedToLoadAnalytics") || "Fehler beim Laden der Daten";
        console.error("[StorageAnalytics] All requests failed");
      }
    } catch (err) {
      console.error("Failed to load storage analytics:", err);
      error = tr("failedToLoadAnalytics") || "Fehler beim Laden der Daten";
    } finally {
      loading = false;
      console.log("[StorageAnalytics] Loading complete. Overview:", overview);
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

  // Export functions
  function exportToJson() {
    const exportData = {
      exported_at: new Date().toISOString(),
      overview,
      user_stats: userStats,
      folder_stats: folderStats,
      top_files: topFiles,
      growth_data: growth,
      duplicate_waste: duplicateWaste,
    };
    downloadFile(
      JSON.stringify(exportData, null, 2),
      `storage-analytics-${new Date().toISOString().split("T")[0]}.json`,
      "application/json"
    );
  }

  function exportToCsv() {
    // Export different sections based on active tab
    let csvContent = "";
    let filename = "";

    if (activeTab === "overview" && overview) {
      csvContent = "Metric,Value\n";
      csvContent += `Total Files,${overview.total_files || 0}\n`;
      csvContent += `Total Size,${overview.total_size_formatted || "0 B"}\n`;
      csvContent += `Usage Percentage,${overview.usage_percentage || 0}%\n`;
      csvContent += `Active Users,${overview.active_users || 0}\n`;
      csvContent += `Average File Size,${overview.avg_file_size_formatted || "0 B"}\n`;
      filename = "storage-overview.csv";
    } else if (activeTab === "users" && userStats.length > 0) {
      csvContent =
        "User ID,Username,File Count,Total Size (bytes),Total Size (formatted)\n";
      userStats.forEach((u) => {
        csvContent += `${u.user_id},"${u.username || "Unknown"}",${u.file_count},${u.total_size},${formatBytes(u.total_size)}\n`;
      });
      filename = "storage-by-user.csv";
    } else if (activeTab === "folders" && folderStats.length > 0) {
      csvContent =
        "Folder Path,File Count,Total Size (bytes),Total Size (formatted)\n";
      folderStats.forEach((f) => {
        csvContent += `"${f.folder_path || "/"}",${f.file_count},${f.total_size},${formatBytes(f.total_size)}\n`;
      });
      filename = "storage-by-folder.csv";
    } else if (activeTab === "top-files" && topFiles.length > 0) {
      csvContent =
        "Filename,Path,Size (bytes),Size (formatted),MIME Type,Owner\n";
      topFiles.forEach((f) => {
        csvContent += `"${f.filename}","${f.file_path}",${f.size_bytes},${formatBytes(f.size_bytes)},"${f.mime_type || ""}","${f.owner_name || ""}"\n`;
      });
      filename = "top-files.csv";
    } else if (activeTab === "growth" && growth.length > 0) {
      csvContent =
        "Date,Files Added,Size Added (bytes),Size Added (formatted)\n";
      growth.forEach((g) => {
        csvContent += `${g.date},${g.files_added},${g.size_added},${formatBytes(g.size_added)}\n`;
      });
      filename = "storage-growth.csv";
    } else if (activeTab === "duplicates" && duplicateWaste) {
      csvContent = "Metric,Value\n";
      csvContent += `Duplicate Groups,${duplicateWaste.duplicate_groups || 0}\n`;
      csvContent += `Total Duplicate Files,${duplicateWaste.total_duplicate_files || 0}\n`;
      csvContent += `Wasted Space,${duplicateWaste.wasted_space_formatted || "0 B"}\n`;
      filename = "duplicate-analysis.csv";
    } else {
      return; // No data to export
    }

    downloadFile(csvContent, filename, "text/csv");
  }

  function downloadFile(content, filename, mimeType) {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }
</script>

<PageWrapper title={tr("storageAnalytics")} showSidebar={true}>
  <div class="max-w-6xl mx-auto p-6">
    <div class="flex justify-between items-center mb-6">
      <h1
        class="text-2xl font-semibold flex items-center gap-2 text-base-content"
      >
        <i class="bi bi-bar-chart-line-fill"></i>
        {tr("storageAnalytics")}
      </h1>
      <div class="flex gap-2">
        <div class="dropdown dropdown-end">
          <button
            tabindex="0"
            class="btn btn-ghost bg-base-200 hover:bg-base-300"
            aria-label="Export data"
          >
            <i class="bi bi-download"></i>
            Export
          </button>
          <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
          <ul
            tabindex="0"
            class="dropdown-content z-10 menu p-2 shadow bg-base-200 rounded-box w-40"
          >
            <li>
              <button onclick={exportToJson}
                ><i class="bi bi-filetype-json"></i> JSON</button
              >
            </li>
            <li>
              <button onclick={exportToCsv}
                ><i class="bi bi-filetype-csv"></i> CSV</button
              >
            </li>
          </ul>
        </div>
        <button
          onclick={loadAllData}
          disabled={loading}
          class="btn btn-primary gap-2"
        >
          <i class="bi bi-arrow-clockwise {loading ? 'animate-spin' : ''}"></i>
          {tr("refresh")}
        </button>
      </div>
    </div>

    {#if error}
      <div class="alert alert-error mb-6">
        <i class="bi bi-exclamation-triangle"></i>
        <span>{error}</span>
        <button onclick={loadAllData} class="btn btn-ghost btn-sm gap-1">
          <i class="bi bi-arrow-clockwise"></i>
          {tr("retry")}
        </button>
      </div>
    {/if}

    {#if loading}
      <div
        class="flex flex-col justify-center items-center min-h-[400px] gap-4"
      >
        <span class="loading loading-spinner loading-lg text-primary"></span>
        <p class="text-base-content/60">{tr("loadingAnalytics")}</p>
      </div>
    {:else}
      <div
        class="flex flex-wrap gap-2 p-2 bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl rounded-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] mb-6"
      >
        {#each tabs as tab}
          <button
            onclick={() => (activeTab = tab.id)}
            class="flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-all {activeTab ===
            tab.id
              ? 'bg-primary text-primary-content'
              : 'text-base-content/60 hover:bg-base-200'}"
          >
            <i class="bi {tab.icon}"></i>
            <span>{tr(tab.label)}</span>
          </button>
        {/each}
      </div>

      {#if activeTab === "overview"}
        <!-- Storage Usage Card -->
        <div
          class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all mb-6"
        >
          <div class="card-body p-5">
            <div class="flex items-center gap-3 mb-4">
              <div
                class="w-9 h-9 rounded-lg bg-success/20 text-success flex items-center justify-center text-lg"
              >
                <i class="bi bi-hdd-stack"></i>
              </div>
              <h2 class="text-base font-semibold text-base-content flex-1">
                {tr("storageUsage")}
              </h2>
              <span class="text-sm text-base-content/60"
                >{overview?.total_size_formatted || "0 B"}</span
              >
            </div>
            <div class="h-2 bg-base-300 rounded overflow-hidden mb-3">
              <div
                class="h-full bg-gradient-to-r from-success to-green-600 rounded transition-all duration-500"
                style="width: {Math.min(overview?.usage_percentage || 0, 100)}%"
              ></div>
            </div>
            <div class="flex justify-between text-sm text-base-content/60">
              <span>{overview?.total_size_formatted || "0 B"} {tr("used")}</span
              >
              <span>{(overview?.usage_percentage || 0).toFixed(1)}%</span>
            </div>
          </div>
        </div>

        <!-- Quick Stats Grid -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
          <div
            class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
          >
            <div class="card-body p-5 flex-row items-center gap-4">
              <div
                class="w-12 h-12 rounded-lg bg-info/20 text-info flex items-center justify-center text-2xl"
              >
                <i class="bi bi-files"></i>
              </div>
              <div>
                <h3 class="text-2xl font-bold text-base-content">
                  {(overview?.total_files || 0).toLocaleString()}
                </h3>
                <p class="text-sm text-base-content/60">{tr("totalFiles")}</p>
              </div>
            </div>
          </div>
          <div
            class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
          >
            <div class="card-body p-5 flex-row items-center gap-4">
              <div
                class="w-12 h-12 rounded-lg bg-success/20 text-success flex items-center justify-center text-2xl"
              >
                <i class="bi bi-people"></i>
              </div>
              <div>
                <h3 class="text-2xl font-bold text-base-content">
                  {(overview?.active_users || 0).toLocaleString()}
                </h3>
                <p class="text-sm text-base-content/60">{tr("activeUsers")}</p>
              </div>
            </div>
          </div>
          <div
            class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
          >
            <div class="card-body p-5 flex-row items-center gap-4">
              <div
                class="w-12 h-12 rounded-lg bg-warning/20 text-warning flex items-center justify-center text-2xl"
              >
                <i class="bi bi-file-earmark-bar-graph"></i>
              </div>
              <div>
                <h3 class="text-2xl font-bold text-base-content">
                  {formatBytes(overview?.avg_file_size_bytes || 0)}
                </h3>
                <p class="text-sm text-base-content/60">{tr("avgFileSize")}</p>
              </div>
            </div>
          </div>
          <div
            class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
          >
            <div class="card-body p-5 flex-row items-center gap-4">
              <div
                class="w-12 h-12 rounded-lg bg-pink-500/20 text-pink-500 flex items-center justify-center text-2xl"
              >
                <i class="bi bi-trophy"></i>
              </div>
              <div>
                <h3 class="text-2xl font-bold text-base-content">
                  {formatBytes(overview?.largest_file_bytes || 0)}
                </h3>
                <p class="text-sm text-base-content/60">{tr("largestFile")}</p>
              </div>
            </div>
          </div>
        </div>
      {/if}

      {#if activeTab === "users"}
        <div
          class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden mb-6"
        >
          <div
            class="flex justify-between items-center p-4 px-5 border-b border-base-300"
          >
            <h2
              class="text-base font-semibold flex items-center gap-2 text-base-content"
            >
              <i class="bi bi-people"></i>
              {tr("storageByUser")}
            </h2>
          </div>
          {#if userStats.length > 0}
            <div class="overflow-x-auto">
              <table class="table">
                <thead class="bg-base-200">
                  <tr>
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("username")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("files")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("storage")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("lastUpload")}</th
                    >
                  </tr>
                </thead>
                <tbody>
                  {#each userStats as user}
                    <tr class="hover">
                      <td class="flex items-center gap-3 font-medium">
                        <div
                          class="w-8 h-8 rounded-full bg-gradient-to-br from-success to-green-600 text-white flex items-center justify-center font-semibold text-sm"
                        >
                          {(user.username || "?").charAt(0).toUpperCase()}
                        </div>
                        {user.username || tr("unknown")}
                      </td>
                      <td
                        ><span class="badge badge-info"
                          >{user.total_files?.toLocaleString() || 0}</span
                        ></td
                      >
                      <td class="font-mono font-semibold"
                        >{user.total_size_formatted || "0 B"}</td
                      >
                      <td class="text-base-content/60"
                        >{formatDate(user.last_upload)}</td
                      >
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <div
              class="flex flex-col items-center justify-center p-12 text-base-content/60"
            >
              <i class="bi bi-people text-5xl mb-4 opacity-50"></i>
              <p>{tr("noUsersFound")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "folders"}
        <div
          class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden mb-6"
        >
          <div
            class="flex justify-between items-center p-4 px-5 border-b border-base-300"
          >
            <h2
              class="text-base font-semibold flex items-center gap-2 text-base-content"
            >
              <i class="bi bi-folder"></i>
              {tr("storageByFolder")}
            </h2>
          </div>
          {#if folderStats.length > 0}
            <div class="overflow-x-auto">
              <table class="table">
                <thead class="bg-base-200">
                  <tr>
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("folder")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("files")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("storage")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("percentage")}</th
                    >
                  </tr>
                </thead>
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
                    <tr class="hover">
                      <td class="flex items-center gap-3 font-medium">
                        <i class="bi bi-folder-fill text-xl text-warning"></i>
                        {folder.folder || "/"}
                      </td>
                      <td
                        ><span class="badge badge-warning"
                          >{folder.file_count?.toLocaleString() || 0}</span
                        ></td
                      >
                      <td class="font-mono font-semibold"
                        >{folder.total_size_formatted || "0 B"}</td
                      >
                      <td>
                        <div class="flex items-center gap-3">
                          <div
                            class="flex-1 h-1.5 bg-base-300 rounded overflow-hidden max-w-24"
                          >
                            <div
                              class="h-full bg-gradient-to-r from-amber-400 to-amber-600 rounded"
                              style="width: {percentage}%"
                            ></div>
                          </div>
                          <span class="text-base-content/60 text-sm"
                            >{percentage.toFixed(1)}%</span
                          >
                        </div>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <div
              class="flex flex-col items-center justify-center p-12 text-base-content/60"
            >
              <i class="bi bi-folder text-5xl mb-4 opacity-50"></i>
              <p>{tr("noFoldersFound")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "top-files"}
        <div
          class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden mb-6"
        >
          <div
            class="flex justify-between items-center p-4 px-5 border-b border-base-300"
          >
            <h2
              class="text-base font-semibold flex items-center gap-2 text-base-content"
            >
              <i class="bi bi-file-earmark-fill"></i>
              {tr("topLargestFiles")}
            </h2>
          </div>
          {#if topFiles.length > 0}
            <div class="overflow-x-auto">
              <table class="table">
                <thead class="bg-base-200">
                  <tr>
                    <th class="uppercase text-xs tracking-wider">#</th>
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("fileName")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("size")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("type")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("created")}</th
                    >
                  </tr>
                </thead>
                <tbody>
                  {#each topFiles.slice(0, 50) as file, index}
                    <tr class="hover">
                      <td>
                        <span
                          class="inline-flex items-center justify-center w-7 h-7 rounded font-semibold text-sm {index <
                          3
                            ? 'bg-gradient-to-br from-amber-400 to-amber-600 text-white'
                            : 'bg-base-200 text-base-content/60'}"
                        >
                          {index + 1}
                        </span>
                      </td>
                      <td class="flex items-center gap-3 font-medium">
                        <i
                          class="bi {getFileIcon(
                            file.mime_type
                          )} text-xl text-base-content/60"
                        ></i>
                        <div class="flex flex-col min-w-0">
                          <span class="truncate max-w-48" title={file.filename}
                            >{file.filename}</span
                          >
                          <span
                            class="text-xs text-base-content/50 truncate max-w-48"
                            title={file.file_path}>{file.file_path}</span
                          >
                        </div>
                      </td>
                      <td class="font-mono font-semibold text-error"
                        >{file.size_formatted ||
                          formatBytes(file.size_bytes)}</td
                      >
                      <td class="text-base-content/60"
                        >{file.mime_type || "-"}</td
                      >
                      <td class="text-base-content/60"
                        >{formatDate(file.created_at)}</td
                      >
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
            {#if topFiles.length > 50}
              <div
                class="py-3 px-4 text-center text-sm text-base-content/60 border-t border-base-300"
              >
                {tr("showingTop50of")}
                {topFiles.length}
                {tr("files")}
              </div>
            {/if}
          {:else}
            <div
              class="flex flex-col items-center justify-center p-12 text-base-content/60"
            >
              <i class="bi bi-file-earmark text-5xl mb-4 opacity-50"></i>
              <p>{tr("noFilesFound")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "growth"}
        <div
          class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden mb-6"
        >
          <div
            class="flex justify-between items-center p-4 px-5 border-b border-base-300"
          >
            <h2
              class="text-base font-semibold flex items-center gap-2 text-base-content"
            >
              <i class="bi bi-graph-up"></i>
              {tr("storageGrowth")}
              <span class="text-base-content/60 font-normal"
                >({tr("last30Days")})</span
              >
            </h2>
          </div>
          {#if growth.length > 0}
            <!-- Bar Chart -->
            <div class="p-6 border-b border-base-300">
              <div class="flex items-end gap-0.5 h-40">
                {#each growth as point}
                  {@const maxSize = Math.max(
                    ...growth.map((g) => g.size_added_bytes || 0),
                    1
                  )}
                  {@const height =
                    ((point.size_added_bytes || 0) / maxSize) * 100}
                  <div
                    class="flex-1 bg-gradient-to-t from-success to-green-400 rounded-t cursor-pointer hover:opacity-80 transition-opacity relative group"
                    style="height: {Math.max(height, 2)}%"
                    title="{point.period}: +{point.files_added} files"
                  >
                    <div
                      class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1 bg-base-300 text-base-content text-xs rounded whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity z-10 pointer-events-none"
                    >
                      {point.period}<br />+{point.files_added} files<br
                      />+{point.size_added_formatted}
                    </div>
                  </div>
                {/each}
              </div>
              <div
                class="flex justify-between mt-2 text-xs text-base-content/60"
              >
                <span>{growth[0]?.period || ""}</span>
                <span>{growth[growth.length - 1]?.period || ""}</span>
              </div>
            </div>
            <!-- Growth Table -->
            <div class="overflow-x-auto max-h-80">
              <table class="table">
                <thead class="bg-base-200 sticky top-0">
                  <tr>
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("date")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("newFiles")}</th
                    >
                    <th class="uppercase text-xs tracking-wider"
                      >{tr("sizeAdded")}</th
                    >
                  </tr>
                </thead>
                <tbody>
                  {#each growth.slice().reverse() as point}
                    <tr class="hover">
                      <td>{point.period}</td>
                      <td
                        ><span class="badge badge-success"
                          >+{point.files_added?.toLocaleString() || 0}</span
                        ></td
                      >
                      <td class="font-mono font-semibold"
                        >{point.size_added_formatted || "0 B"}</td
                      >
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else}
            <div
              class="flex flex-col items-center justify-center p-12 text-base-content/60"
            >
              <i class="bi bi-graph-up text-5xl mb-4 opacity-50"></i>
              <p>{tr("noGrowthData")}</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeTab === "duplicates"}
        {#if duplicateWaste}
          <!-- Duplicate Stats Cards -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
            <div
              class="p-6 rounded-xl bg-gradient-to-br from-rose-500 to-rose-600 text-white text-center"
            >
              <div class="text-2xl mb-2 opacity-90">
                <i class="bi bi-files"></i>
              </div>
              <div class="text-3xl font-bold mb-1">
                {duplicateWaste.duplicate_groups?.toLocaleString() || 0}
              </div>
              <div class="text-sm opacity-90">{tr("duplicateGroups")}</div>
            </div>
            <div
              class="p-6 rounded-xl bg-gradient-to-br from-amber-500 to-amber-600 text-white text-center"
            >
              <div class="text-2xl mb-2 opacity-90">
                <i class="bi bi-exclamation-triangle"></i>
              </div>
              <div class="text-3xl font-bold mb-1">
                {duplicateWaste.wasted_formatted || "0 B"}
              </div>
              <div class="text-sm opacity-90">{tr("wastedSpace")}</div>
            </div>
            <div
              class="p-6 rounded-xl bg-gradient-to-br from-success to-green-600 text-white text-center"
            >
              <div class="text-2xl mb-2 opacity-90">
                <i class="bi bi-piggy-bank"></i>
              </div>
              <div class="text-3xl font-bold mb-1">
                {duplicateWaste.savings_potential_formatted || "0 B"}
              </div>
              <div class="text-sm opacity-90">{tr("savingsPotential")}</div>
            </div>
          </div>
          <!-- Info Card -->
          <div class="card bg-info/10 border border-info/30">
            <div class="card-body flex-row gap-4">
              <div
                class="w-12 h-12 rounded-lg bg-gradient-to-br from-info to-blue-600 text-white flex items-center justify-center text-2xl flex-shrink-0"
              >
                <i class="bi bi-info-circle"></i>
              </div>
              <div>
                <h3 class="font-semibold text-info mb-1">
                  {tr("duplicateInfo")}
                </h3>
                <p class="text-sm text-info/80 mb-3">
                  {tr("duplicateInfoDescription")}
                </p>
                <button
                  class="btn btn-outline btn-info btn-sm gap-2"
                  onclick={() => {
                    window.location.hash = "#/duplicates";
                  }}
                >
                  <i class="bi bi-search"></i>
                  {tr("viewDuplicates")}
                </button>
              </div>
            </div>
          </div>
        {:else}
          <div
            class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
          >
            <div
              class="card-body flex-col items-center justify-center p-12 text-base-content/60"
            >
              <i class="bi bi-files text-5xl mb-4 opacity-50"></i>
              <p>{tr("noDuplicatesFound")}</p>
            </div>
          </div>
        {/if}
      {/if}
    {/if}
  </div>
</PageWrapper>

<style>
  /* All styles migrated to Tailwind/DaisyUI classes */
  /* Only keep custom animations if needed */
</style>
