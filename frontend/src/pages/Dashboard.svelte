<script>
  import { onMount } from "svelte";
  import { currentLang } from "../stores/ui";
  import { t } from "../i18n.js";
  import PageWrapper from "../components/PageWrapper.svelte";
  import api from "../lib/api";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Dashboard state
  let stats = $state({
    storageUsed: "2.4 GB",
    storageTotal: "100 GB",
    storagePercent: 24,
    fileCount: 247,
    filesThisWeek: 12,
    activeSyncs: 5,
    lastSync: "vor 2 Minuten",
  });

  let recentActivity = $state([
    {
      id: 1,
      action: "Datei hochgeladen",
      file: "Presentation.pdf",
      time: "vor 5 min",
      icon: "bi-file-earmark-pdf",
      color: "red",
    },
    {
      id: 2,
      action: "Ordner erstellt",
      file: "Projekt Q4",
      time: "vor 15 min",
      icon: "bi-folder",
      color: "blue",
    },
    {
      id: 3,
      action: "Datei aktualisiert",
      file: "Budget.xlsx",
      time: "vor 1 h",
      icon: "bi-file-earmark-spreadsheet",
      color: "green",
    },
    {
      id: 4,
      action: "Datei freigegeben",
      file: "Design.fig",
      time: "vor 3 h",
      icon: "bi-share",
      color: "purple",
    },
  ]);

  let recentFiles = $state([
    {
      name: "Presentation.pdf",
      size: "2.4 MB",
      modified: "heute",
      icon: "bi-file-earmark-pdf",
    },
    {
      name: "Budget.xlsx",
      size: "156 KB",
      modified: "gestern",
      icon: "bi-file-earmark-spreadsheet",
    },
    {
      name: "Design.fig",
      size: "12.5 MB",
      modified: "vor 2 Tagen",
      icon: "bi-image",
    },
    {
      name: "Notes.txt",
      size: "24 KB",
      modified: "vor 1 Woche",
      icon: "bi-file-earmark-text",
    },
  ]);

  let greeting = $derived.by(() => {
    const hour = new Date().getHours();
    if (hour < 12) return "Guten Morgen!";
    if (hour < 18) return "Guten Nachmittag!";
    return "Guten Abend!";
  });

  onMount(async () => {
    // Load dashboard data from backend
    try {
      const response = await fetch(
        "http://localhost:8080/api/dashboard/stats",
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (response.ok) {
        const data = await response.json();

        // Map backend data to frontend state
        if (data.overview) {
          const overview = data.overview;

          // Calculate storage percentage
          const storageUsedGB = (
            overview.total_storage_bytes /
            1024 ** 3
          ).toFixed(1);
          const storageTotalGB = 100; // Default 100GB quota
          const storagePercent = Math.round(
            (storageUsedGB / storageTotalGB) * 100
          );

          stats = {
            storageUsed: `${storageUsedGB} GB`,
            storageTotal: `${storageTotalGB} GB`,
            storagePercent: Math.min(storagePercent, 100),
            fileCount: overview.total_files || 0,
            filesThisWeek: 12, // TODO: Get from backend trends
            activeSyncs: overview.pending_jobs || 0,
            lastSync: "vor 2 Minuten", // TODO: Get from backend
          };
        }

        // Map activity data if available
        if (data.recent_activity && Array.isArray(data.recent_activity)) {
          recentActivity = data.recent_activity.map((item, idx) => ({
            id: idx + 1,
            action: item.action || "Datei geändert",
            file: item.file_path || "Datei",
            time: item.time || "vor Kurzem",
            icon: getIconForAction(item.action || ""),
            color: getColorForAction(item.action || ""),
          }));
        }

        // Map recent files if available
        if (data.recent_files && Array.isArray(data.recent_files)) {
          recentFiles = data.recent_files.map((file) => ({
            name: file.name || "Datei",
            size: formatBytes(file.size_bytes || 0),
            modified: formatDate(file.modified_at),
            icon: getIconForFile(file.name || ""),
          }));
        }
      }
    } catch (err) {
      console.error("Failed to load dashboard:", err);
    }
  });

  // Helper functions
  const formatBytes = (bytes) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  };

  const formatDate = (dateStr) => {
    if (!dateStr) return "vor Kurzem";
    const date = new Date(dateStr);
    const now = new Date();
    const diffTime = now - date;
    const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return "heute";
    if (diffDays === 1) return "gestern";
    if (diffDays < 7) return `vor ${diffDays} Tagen`;
    if (diffDays < 30) return `vor ${Math.floor(diffDays / 7)} Wochen`;
    return `vor ${Math.floor(diffDays / 30)} Monaten`;
  };

  const getIconForAction = (action) => {
    if (action.includes("upload")) return "bi-upload";
    if (action.includes("delete")) return "bi-trash";
    if (action.includes("create")) return "bi-folder-plus";
    if (action.includes("share")) return "bi-share";
    return "bi-file-earmark";
  };

  const getColorForAction = (action) => {
    if (action.includes("upload")) return "green";
    if (action.includes("delete")) return "red";
    if (action.includes("create")) return "blue";
    if (action.includes("share")) return "purple";
    return "gray";
  };

  const getIconForFile = (filename) => {
    if (filename.endsWith(".pdf")) return "bi-file-earmark-pdf";
    if (filename.endsWith(".xlsx") || filename.endsWith(".xls"))
      return "bi-file-earmark-spreadsheet";
    if (filename.endsWith(".doc") || filename.endsWith(".docx"))
      return "bi-file-earmark-word";
    if (
      filename.endsWith(".jpg") ||
      filename.endsWith(".png") ||
      filename.endsWith(".gif")
    )
      return "bi-image";
    if (filename.endsWith(".txt")) return "bi-file-earmark-text";
    return "bi-file-earmark";
  };

  const handleFileClick = (file) => {
    // Navigate to file
    console.log("Open file:", file);
  };
</script>

<PageWrapper title="Dashboard" showSidebar={true}>
  <div class="space-y-8">
    <!-- Welcome Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1
          class="text-4xl font-bold bg-gradient-to-r from-blue-600 via-purple-600 to-pink-600 bg-clip-text text-transparent"
        >
          Dashboard
        </h1>
        <p class="text-gray-600 dark:text-gray-400 text-sm mt-2">
          {greeting} 👋
        </p>
      </div>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <!-- Storage Card -->
      <div class="glass-stat-card">
        <div class="flex items-center justify-between mb-4">
          <div
            class="w-10 h-10 rounded-lg bg-blue-500/20 flex items-center justify-center"
          >
            <i class="bi bi-pie-chart-fill text-blue-500" aria-hidden="true"></i>
          </div>
          <span
            class="text-xs bg-blue-500/20 text-blue-600 dark:text-blue-400 px-2 py-1 rounded-full"
          >
            {stats.storagePercent}%
          </span>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-sm">Speicherplatz</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white mt-2">
          {stats.storageUsed}
        </p>
        <p class="text-xs text-gray-500 dark:text-gray-500 mt-2">
          von {stats.storageTotal}
        </p>
      </div>

      <!-- Files Card -->
      <div class="glass-stat-card">
        <div class="flex items-center justify-between mb-4">
          <div
            class="w-10 h-10 rounded-lg bg-purple-500/20 flex items-center justify-center"
          >
            <i class="bi bi-file-earmark-fill text-purple-500" aria-hidden="true"></i>
          </div>
          <span
            class="text-xs bg-green-500/20 text-green-600 dark:text-green-400 px-2 py-1 rounded-full"
          >
            +{stats.filesThisWeek}
          </span>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-sm">Dateien</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white mt-2">
          {stats.fileCount}
        </p>
        <p class="text-xs text-gray-500 dark:text-gray-500 mt-2">diese Woche</p>
      </div>

      <!-- Syncs Card -->
      <div class="glass-stat-card">
        <div class="flex items-center justify-between mb-4">
          <div
            class="w-10 h-10 rounded-lg bg-emerald-500/20 flex items-center justify-center"
          >
            <i class="bi bi-arrow-repeat text-emerald-500" aria-hidden="true"></i>
          </div>
          <span
            class="text-xs bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 px-2 py-1 rounded-full"
          >
            Online
          </span>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-sm">Aktive Syncs</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white mt-2">
          {stats.activeSyncs}
        </p>
        <p class="text-xs text-gray-500 dark:text-gray-500 mt-2">
          alle synchronisiert
        </p>
      </div>

      <!-- Last Sync Card -->
      <div class="glass-stat-card">
        <div class="flex items-center justify-between mb-4">
          <div
            class="w-10 h-10 rounded-lg bg-pink-500/20 flex items-center justify-center"
          >
            <i class="bi bi-clock-history text-pink-500" aria-hidden="true"></i>
          </div>
          <span
            class="text-xs bg-slate-500/20 text-slate-600 dark:text-slate-400 px-2 py-1 rounded-full"
          >
            ✓
          </span>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-sm">Letzte Sync</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white mt-2">
          {stats.lastSync}
        </p>
        <p class="text-xs text-gray-500 dark:text-gray-500 mt-2">erfolgreich</p>
      </div>
    </div>

    <!-- Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- Recent Activity -->
      <div class="lg:col-span-2">
        <div class="glass-card-primary">
          <div
            class="flex items-center justify-between mb-6 pb-6 border-b border-gray-200/50 dark:border-gray-700/50"
          >
            <h2
              class="text-xl font-semibold text-gray-900 dark:text-white flex items-center gap-2"
            >
              <i class="bi bi-lightning-fill text-amber-500" aria-hidden="true"></i>
              Aktuelle Aktivität
            </h2>
            <a
              href="#"
              class="text-blue-600 dark:text-blue-400 text-sm hover:underline"
              >Alle anzeigen</a
            >
          </div>
          <div class="space-y-3">
            {#each recentActivity as activity (activity.id)}
              <div class="glass-activity-item">
                <div class="flex items-center gap-4">
                  <div
                    class="w-10 h-10 rounded-lg bg-gradient-to-br from-gray-200/50 to-gray-300/50 dark:from-gray-700/50 dark:to-gray-600/50 flex items-center justify-center flex-shrink-0"
                  >
                    <i
                      class="bi {activity.icon} text-gray-700 dark:text-gray-300"
                    ></i>
                  </div>
                  <div class="flex-1 min-w-0">
                    <p
                      class="text-gray-900 dark:text-white font-medium text-sm"
                    >
                      {activity.action}
                    </p>
                    <p
                      class="text-gray-600 dark:text-gray-400 text-xs truncate"
                    >
                      {activity.file}
                    </p>
                  </div>
                  <span
                    class="text-gray-500 dark:text-gray-500 text-xs flex-shrink-0"
                    >{activity.time}</span
                  >
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Recent Files Sidebar -->
      <div>
        <div class="glass-card-primary">
          <h2
            class="text-xl font-semibold text-gray-900 dark:text-white mb-6 pb-6 border-b border-gray-200/50 dark:border-gray-700/50 flex items-center gap-2"
          >
            <i class="bi bi-bookmark-fill text-blue-500" aria-hidden="true"></i>
            Zuletzt geöffnet
          </h2>
          <div class="space-y-3">
            {#each recentFiles as file (file.name)}
              <button
                onclick={() => handleFileClick(file)}
                class="glass-file-card w-full text-left group"
              >
                <div class="flex items-center gap-3">
                  <div
                    class="w-8 h-8 rounded-lg bg-gray-200/50 dark:bg-gray-700/50 flex items-center justify-center flex-shrink-0 group-hover:bg-gray-300/50 dark:group-hover:bg-gray-600/50 transition-colors"
                  >
                    <i
                      class="bi {file.icon} text-gray-700 dark:text-gray-300 text-sm"
                    ></i>
                  </div>
                  <div class="flex-1 min-w-0">
                    <p
                      class="text-gray-900 dark:text-white font-medium text-sm truncate"
                    >
                      {file.name}
                    </p>
                    <p class="text-gray-600 dark:text-gray-400 text-xs">
                      {file.size}
                    </p>
                  </div>
                </div>
                <p class="text-gray-500 dark:text-gray-500 text-xs mt-2">
                  {file.modified}
                </p>
              </button>
            {/each}
          </div>
        </div>
      </div>
    </div>
  </div>
</PageWrapper>

<style>
  :global(body) {
    --color-gradient-start: #3b82f6;
    --color-gradient-mid: #a855f7;
    --color-gradient-end: #ec4899;
  }
</style>
