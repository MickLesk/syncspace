<script>
  import { onMount } from "svelte";
  import { currentLang } from "../stores/ui";
  import { t } from "../i18n.js";
  import PageWrapper from "../components/PageWrapper.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Dashboard state
  let stats = $state({
    storageUsed: 0,
    storageTotal: 100,
    storagePercent: 0,
    fileCount: 0,
    folderCount: 0,
    filesThisWeek: 0,
    uploadsToday: 0,
    downloadsToday: 0,
    sharedFiles: 0,
    favoriteCount: 0,
  });

  // Activity data for charts
  let weeklyActivity = $state([]);
  let fileTypeDistribution = $state([]);
  let recentActivity = $state([]);
  let quickStats = $state({ uploads: 0, downloads: 0, shares: 0, deletes: 0 });
  
  let loading = $state(true);

  // Greeting based on time
  let greeting = $derived.by(() => {
    const hour = new Date().getHours();
    if (hour < 12) return "Guten Morgen";
    if (hour < 18) return "Guten Tag";
    return "Guten Abend";
  });

  // Current date formatted
  let currentDate = $derived.by(() => {
    return new Date().toLocaleDateString("de-DE", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  });

  onMount(async () => {
    await loadDashboardData();
  });

  async function loadDashboardData() {
    loading = true;
    try {
      const token = localStorage.getItem("authToken");
      const headers = { Authorization: `Bearer ${token}` };

      // Load dashboard stats
      const [statsRes, activityRes, filesRes] = await Promise.all([
        fetch("http://localhost:8080/api/dashboard/stats", { headers }),
        fetch("http://localhost:8080/api/activity?limit=50", { headers }),
        fetch("http://localhost:8080/api/files", { headers }),
      ]);

      // Process stats
      if (statsRes.ok) {
        const data = await statsRes.json();
        if (data.overview) {
          const o = data.overview;
          const storageUsedGB = parseFloat((o.total_storage_bytes / 1024 ** 3).toFixed(2));
          stats = {
            storageUsed: storageUsedGB,
            storageTotal: 100,
            storagePercent: Math.min(Math.round((storageUsedGB / 100) * 100), 100),
            fileCount: o.total_files || 0,
            folderCount: o.total_folders || 0,
            filesThisWeek: o.files_this_week || 0,
            uploadsToday: o.uploads_today || 0,
            downloadsToday: o.downloads_today || 0,
            sharedFiles: o.shared_files || 0,
            favoriteCount: o.favorite_count || 0,
          };
        }
      }

      // Process activity for charts
      if (activityRes.ok) {
        const activities = await activityRes.json();
        processActivityData(activities || []);
      }

      // Process files for type distribution
      if (filesRes.ok) {
        const files = await filesRes.json();
        processFileTypes(files || []);
      }
    } catch (err) {
      console.error("Failed to load dashboard:", err);
    } finally {
      loading = false;
    }
  }

  function processActivityData(activities) {
    // Recent activity (last 8)
    recentActivity = activities.slice(0, 8).map((item, idx) => ({
      id: item.id || idx,
      action: item.action,
      actionLabel: formatAction(item.action),
      fileName: extractFileName(item.file_path || item.file_name),
      time: formatTimeAgo(item.created_at),
      icon: getIconForAction(item.action),
      color: getColorForAction(item.action),
    }));

    // Weekly activity (group by day)
    const days = ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"];
    const today = new Date();
    const weekData = [];
    
    for (let i = 6; i >= 0; i--) {
      const date = new Date(today);
      date.setDate(date.getDate() - i);
      const dayName = days[date.getDay()];
      const dateStr = date.toISOString().split("T")[0];
      
      const dayActivities = activities.filter(a => {
        const actDate = new Date(a.created_at).toISOString().split("T")[0];
        return actDate === dateStr;
      });

      weekData.push({
        day: dayName,
        count: dayActivities.length,
        uploads: dayActivities.filter(a => a.action === "upload").length,
        downloads: dayActivities.filter(a => a.action === "download").length,
      });
    }
    weeklyActivity = weekData;

    // Quick stats from activity
    quickStats = {
      uploads: activities.filter(a => a.action === "upload").length,
      downloads: activities.filter(a => a.action === "download").length,
      shares: activities.filter(a => a.action === "share").length,
      deletes: activities.filter(a => a.action === "delete").length,
    };
  }

  function processFileTypes(files) {
    const typeMap = {};
    const typeColors = {
      "Dokumente": "#3B82F6",
      "Bilder": "#10B981",
      "Videos": "#F59E0B",
      "Audio": "#8B5CF6",
      "Archive": "#EF4444",
      "Andere": "#6B7280",
    };

    files.forEach(file => {
      if (file.is_directory) return;
      const ext = (file.name || "").split(".").pop()?.toLowerCase() || "";
      let type = "Andere";
      
      if (["pdf", "doc", "docx", "xls", "xlsx", "txt", "csv", "ppt", "pptx"].includes(ext)) type = "Dokumente";
      else if (["jpg", "jpeg", "png", "gif", "svg", "webp", "bmp"].includes(ext)) type = "Bilder";
      else if (["mp4", "avi", "mov", "mkv", "webm"].includes(ext)) type = "Videos";
      else if (["mp3", "wav", "ogg", "flac", "aac"].includes(ext)) type = "Audio";
      else if (["zip", "rar", "7z", "tar", "gz"].includes(ext)) type = "Archive";
      
      typeMap[type] = (typeMap[type] || 0) + 1;
    });

    fileTypeDistribution = Object.entries(typeMap).map(([name, count]) => ({
      name,
      count,
      color: typeColors[name] || "#6B7280",
      percent: files.length > 0 ? Math.round((count / files.filter(f => !f.is_directory).length) * 100) : 0,
    }));
  }

  // Action config
  const actionConfig = {
    upload: { label: "Hochgeladen", icon: "bi-upload", color: "emerald" },
    download: { label: "Heruntergeladen", icon: "bi-download", color: "blue" },
    delete: { label: "Gelöscht", icon: "bi-trash", color: "red" },
    rename: { label: "Umbenannt", icon: "bi-pencil", color: "amber" },
    move: { label: "Verschoben", icon: "bi-arrow-right", color: "purple" },
    copy: { label: "Kopiert", icon: "bi-files", color: "indigo" },
    share: { label: "Geteilt", icon: "bi-share", color: "violet" },
    favorite: { label: "Favorisiert", icon: "bi-star-fill", color: "yellow" },
    folder_create: { label: "Ordner erstellt", icon: "bi-folder-plus", color: "blue" },
    folder_color: { label: "Farbe geändert", icon: "bi-palette", color: "pink" },
  };

  const formatAction = (action) => actionConfig[action]?.label || action;
  const getIconForAction = (action) => actionConfig[action]?.icon || "bi-file-earmark";
  const getColorForAction = (action) => actionConfig[action]?.color || "gray";

  const extractFileName = (path) => {
    if (!path) return "Datei";
    return path.split("/").pop() || path;
  };

  const formatTimeAgo = (timestamp) => {
    if (!timestamp) return "gerade eben";
    const date = new Date(timestamp);
    const now = new Date();
    const diffMs = now - date;
    const diffMins = Math.floor(diffMs / (1000 * 60));
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffMins < 1) return "gerade eben";
    if (diffMins < 60) return `vor ${diffMins}m`;
    if (diffHours < 24) return `vor ${diffHours}h`;
    if (diffDays < 7) return `vor ${diffDays}d`;
    return date.toLocaleDateString("de-DE");
  };

  const formatBytes = (gb) => {
    if (gb < 1) return `${Math.round(gb * 1024)} MB`;
    return `${gb.toFixed(1)} GB`;
  };

  // Calculate max for chart scaling
  let maxActivity = $derived(Math.max(...weeklyActivity.map(d => d.count), 1));

  // Color classes
  const colorClasses = {
    emerald: "bg-emerald-500/20 text-emerald-600 dark:text-emerald-400",
    blue: "bg-blue-500/20 text-blue-600 dark:text-blue-400",
    red: "bg-red-500/20 text-red-600 dark:text-red-400",
    amber: "bg-amber-500/20 text-amber-600 dark:text-amber-400",
    purple: "bg-purple-500/20 text-purple-600 dark:text-purple-400",
    indigo: "bg-indigo-500/20 text-indigo-600 dark:text-indigo-400",
    violet: "bg-violet-500/20 text-violet-600 dark:text-violet-400",
    yellow: "bg-yellow-500/20 text-yellow-600 dark:text-yellow-400",
    pink: "bg-pink-500/20 text-pink-600 dark:text-pink-400",
    gray: "bg-gray-500/20 text-gray-600 dark:text-gray-400",
  };
</script>

<PageWrapper title="Dashboard" showSidebar={true}>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
      <div>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          {greeting}! 👋
        </h1>
        <p class="text-gray-500 dark:text-gray-400 mt-1">{currentDate}</p>
      </div>
      <button 
        onclick={() => loadDashboardData()} 
        class="self-start px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg flex items-center gap-2 transition-colors"
      >
        <i class="bi bi-arrow-clockwise" class:animate-spin={loading}></i>
        Aktualisieren
      </button>
    </div>

    {#if loading}
      <div class="flex items-center justify-center py-20">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent"></div>
      </div>
    {:else}
      <!-- Quick Stats Row -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-emerald-500/20 flex items-center justify-center">
              <i class="bi bi-upload text-emerald-500 text-lg"></i>
            </div>
            <div>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{quickStats.uploads}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">Uploads</p>
            </div>
          </div>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-blue-500/20 flex items-center justify-center">
              <i class="bi bi-download text-blue-500 text-lg"></i>
            </div>
            <div>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{quickStats.downloads}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">Downloads</p>
            </div>
          </div>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-violet-500/20 flex items-center justify-center">
              <i class="bi bi-share text-violet-500 text-lg"></i>
            </div>
            <div>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{quickStats.shares}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">Geteilt</p>
            </div>
          </div>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-red-500/20 flex items-center justify-center">
              <i class="bi bi-trash text-red-500 text-lg"></i>
            </div>
            <div>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">{quickStats.deletes}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">Gelöscht</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Main Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Storage Card -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
            <i class="bi bi-hdd text-blue-500"></i>
            Speicherplatz
          </h3>
          
          <!-- Circular Progress -->
          <div class="flex items-center justify-center mb-4">
            <div class="relative w-32 h-32">
              <svg class="w-full h-full transform -rotate-90">
                <circle
                  cx="64" cy="64" r="56"
                  stroke="currentColor"
                  stroke-width="12"
                  fill="none"
                  class="text-gray-200 dark:text-gray-700"
                />
                <circle
                  cx="64" cy="64" r="56"
                  stroke="currentColor"
                  stroke-width="12"
                  fill="none"
                  stroke-dasharray={2 * Math.PI * 56}
                  stroke-dashoffset={2 * Math.PI * 56 * (1 - stats.storagePercent / 100)}
                  stroke-linecap="round"
                  class="text-blue-500 transition-all duration-1000"
                />
              </svg>
              <div class="absolute inset-0 flex flex-col items-center justify-center">
                <span class="text-2xl font-bold text-gray-900 dark:text-white">{stats.storagePercent}%</span>
                <span class="text-xs text-gray-500">belegt</span>
              </div>
            </div>
          </div>
          
          <div class="text-center">
            <p class="text-lg font-semibold text-gray-900 dark:text-white">
              {formatBytes(stats.storageUsed)} <span class="text-gray-400 font-normal">von</span> {stats.storageTotal} GB
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
              {formatBytes(stats.storageTotal - stats.storageUsed)} verfügbar
            </p>
          </div>
        </div>

        <!-- Weekly Activity Chart -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700 lg:col-span-2">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
            <i class="bi bi-graph-up text-emerald-500"></i>
            Aktivität (Letzte 7 Tage)
          </h3>
          
          <!-- Bar Chart -->
          <div class="flex items-end justify-between gap-2 h-40">
            {#each weeklyActivity as day, i}
              <div class="flex-1 flex flex-col items-center gap-2">
                <div class="w-full flex flex-col items-center gap-1 h-32 justify-end">
                  {#if day.count > 0}
                    <span class="text-xs font-medium text-gray-600 dark:text-gray-400">{day.count}</span>
                  {/if}
                  <div 
                    class="w-full bg-gradient-to-t from-blue-500 to-blue-400 rounded-t-md transition-all duration-500"
                    style="height: {day.count > 0 ? (day.count / maxActivity) * 100 : 4}%"
                  ></div>
                </div>
                <span class="text-xs text-gray-500 dark:text-gray-400">{day.day}</span>
              </div>
            {/each}
          </div>
          
          <!-- Legend -->
          <div class="flex items-center justify-center gap-6 mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded bg-emerald-500"></div>
              <span class="text-xs text-gray-500">Uploads: {weeklyActivity.reduce((a, b) => a + b.uploads, 0)}</span>
            </div>
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded bg-blue-500"></div>
              <span class="text-xs text-gray-500">Downloads: {weeklyActivity.reduce((a, b) => a + b.downloads, 0)}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Second Row -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- File Stats -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
            <i class="bi bi-folder text-amber-500"></i>
            Dateien & Ordner
          </h3>
          
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg bg-purple-500/20 flex items-center justify-center">
                  <i class="bi bi-file-earmark text-purple-500"></i>
                </div>
                <span class="text-gray-600 dark:text-gray-400">Dateien</span>
              </div>
              <span class="text-xl font-bold text-gray-900 dark:text-white">{stats.fileCount}</span>
            </div>
            
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg bg-amber-500/20 flex items-center justify-center">
                  <i class="bi bi-folder text-amber-500"></i>
                </div>
                <span class="text-gray-600 dark:text-gray-400">Ordner</span>
              </div>
              <span class="text-xl font-bold text-gray-900 dark:text-white">{stats.folderCount}</span>
            </div>
            
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg bg-yellow-500/20 flex items-center justify-center">
                  <i class="bi bi-star-fill text-yellow-500"></i>
                </div>
                <span class="text-gray-600 dark:text-gray-400">Favoriten</span>
              </div>
              <span class="text-xl font-bold text-gray-900 dark:text-white">{stats.favoriteCount}</span>
            </div>
            
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg bg-violet-500/20 flex items-center justify-center">
                  <i class="bi bi-share text-violet-500"></i>
                </div>
                <span class="text-gray-600 dark:text-gray-400">Geteilt</span>
              </div>
              <span class="text-xl font-bold text-gray-900 dark:text-white">{stats.sharedFiles}</span>
            </div>
          </div>
        </div>

        <!-- File Type Distribution -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
            <i class="bi bi-pie-chart text-pink-500"></i>
            Dateitypen
          </h3>
          
          {#if fileTypeDistribution.length === 0}
            <div class="text-center py-8 text-gray-500">
              <i class="bi bi-inbox text-3xl mb-2 block opacity-50"></i>
              <p class="text-sm">Keine Dateien vorhanden</p>
            </div>
          {:else}
            <div class="space-y-3">
              {#each fileTypeDistribution as type}
                <div>
                  <div class="flex items-center justify-between mb-1">
                    <span class="text-sm text-gray-600 dark:text-gray-400">{type.name}</span>
                    <span class="text-sm font-medium text-gray-900 dark:text-white">{type.count} ({type.percent}%)</span>
                  </div>
                  <div class="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                    <div 
                      class="h-full rounded-full transition-all duration-500"
                      style="width: {type.percent}%; background-color: {type.color}"
                    ></div>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Recent Activity -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2">
              <i class="bi bi-clock-history text-cyan-500"></i>
              Letzte Aktivitäten
            </h3>
            <a href="#/activity" class="text-xs text-blue-500 hover:underline">Alle →</a>
          </div>
          
          {#if recentActivity.length === 0}
            <div class="text-center py-8 text-gray-500">
              <i class="bi bi-inbox text-3xl mb-2 block opacity-50"></i>
              <p class="text-sm">Keine Aktivitäten</p>
            </div>
          {:else}
            <div class="space-y-3">
              {#each recentActivity as activity (activity.id)}
                <div class="flex items-center gap-3 p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
                  <div class="w-8 h-8 rounded-lg flex items-center justify-center {colorClasses[activity.color] || colorClasses.gray}">
                    <i class="bi {activity.icon} text-sm"></i>
                  </div>
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-gray-900 dark:text-white truncate">{activity.fileName}</p>
                    <p class="text-xs text-gray-500">{activity.actionLabel}</p>
                  </div>
                  <span class="text-xs text-gray-400 flex-shrink-0">{activity.time}</span>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
          <i class="bi bi-lightning text-amber-500"></i>
          Schnellaktionen
        </h3>
        
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <a 
            href="#/files" 
            class="flex flex-col items-center gap-2 p-4 rounded-xl bg-blue-50 dark:bg-blue-900/20 hover:bg-blue-100 dark:hover:bg-blue-900/30 transition-colors group"
          >
            <div class="w-12 h-12 rounded-full bg-blue-500 flex items-center justify-center group-hover:scale-110 transition-transform">
              <i class="bi bi-folder text-white text-xl"></i>
            </div>
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Dateien</span>
          </a>
          
          <a 
            href="#/favorites" 
            class="flex flex-col items-center gap-2 p-4 rounded-xl bg-yellow-50 dark:bg-yellow-900/20 hover:bg-yellow-100 dark:hover:bg-yellow-900/30 transition-colors group"
          >
            <div class="w-12 h-12 rounded-full bg-yellow-500 flex items-center justify-center group-hover:scale-110 transition-transform">
              <i class="bi bi-star text-white text-xl"></i>
            </div>
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Favoriten</span>
          </a>
          
          <a 
            href="#/shared" 
            class="flex flex-col items-center gap-2 p-4 rounded-xl bg-violet-50 dark:bg-violet-900/20 hover:bg-violet-100 dark:hover:bg-violet-900/30 transition-colors group"
          >
            <div class="w-12 h-12 rounded-full bg-violet-500 flex items-center justify-center group-hover:scale-110 transition-transform">
              <i class="bi bi-share text-white text-xl"></i>
            </div>
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Geteilt</span>
          </a>
          
          <a 
            href="#/settings" 
            class="flex flex-col items-center gap-2 p-4 rounded-xl bg-gray-50 dark:bg-gray-700/50 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors group"
          >
            <div class="w-12 h-12 rounded-full bg-gray-500 flex items-center justify-center group-hover:scale-110 transition-transform">
              <i class="bi bi-gear text-white text-xl"></i>
            </div>
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Einstellungen</span>
          </a>
        </div>
      </div>
    {/if}
  </div>
</PageWrapper>

<style>
  .animate-spin {
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
