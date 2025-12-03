<script>
  import { onMount } from "svelte";
  import { currentLang } from "../stores/ui";
  import { t } from "../i18n.js";
  import PageWrapper from "../components/PageWrapper.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Dashboard state
  let stats = $state({
    storageUsed: "0 GB",
    storageTotal: "100 GB",
    storagePercent: 0,
    fileCount: 0,
    filesThisWeek: 0,
    activeSessions: 0,
    actionsToday: 0,
    activeSyncs: 1,
    lastSync: "gerade eben",
  });

  // Unified activity timeline
  let activityTimeline = $state([]);
  let loading = $state(true);

  // Activity filter
  let activityFilter = $state("all");
  const filterOptions = [
    { value: "all", label: "Alle", icon: "bi-list" },
    { value: "upload", label: "Uploads", icon: "bi-upload" },
    { value: "download", label: "Downloads", icon: "bi-download" },
    { value: "favorite", label: "Favoriten", icon: "bi-star" },
    { value: "share", label: "Geteilt", icon: "bi-share" },
    { value: "delete", label: "Gelöscht", icon: "bi-trash" },
  ];

  let greeting = $derived.by(() => {
    const hour = new Date().getHours();
    if (hour < 12) return tr("greeting.morning") || "Guten Morgen!";
    if (hour < 18) return tr("greeting.afternoon") || "Guten Tag!";
    return tr("greeting.evening") || "Guten Abend!";
  });

  onMount(async () => {
    await loadDashboardData();
  });

  async function loadDashboardData() {
    loading = true;
    try {
      // Load dashboard stats
      const statsResponse = await fetch(
        "http://localhost:8080/api/dashboard/stats",
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (statsResponse.ok) {
        const data = await statsResponse.json();
        if (data.overview) {
          const overview = data.overview;
          const storageUsedGB = (overview.total_storage_bytes / 1024 ** 3).toFixed(2);
          const storageTotalGB = 100;
          const storagePercent = Math.round((storageUsedGB / storageTotalGB) * 100);

          stats = {
            storageUsed: `${storageUsedGB} GB`,
            storageTotal: `${storageTotalGB} GB`,
            storagePercent: Math.min(storagePercent, 100),
            fileCount: overview.total_files || 0,
            filesThisWeek: overview.files_this_week || 0,
            activeSessions: overview.active_sessions || 0,
            actionsToday: overview.active_users_today || 0,
            activeSyncs: 1,
            lastSync: "gerade eben",
          };
        }
      }

      // Load unified activity timeline
      await loadActivityTimeline();
    } catch (err) {
      console.error("Failed to load dashboard:", err);
    } finally {
      loading = false;
    }
  }

  async function loadActivityTimeline() {
    try {
      const filterParam = activityFilter !== "all" ? `&action=${activityFilter}` : "";
      const response = await fetch(
        `http://localhost:8080/api/activity?limit=20${filterParam}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (response.ok) {
        const activities = await response.json();
        activityTimeline = (activities || []).map((item, idx) => ({
          id: item.id || idx,
          action: item.action,
          actionLabel: formatAction(item.action),
          fileName: extractFileName(item.file_path || item.file_name),
          filePath: item.file_path,
          fileSize: item.file_size,
          username: item.username || item.display_name || "Du",
          time: formatTimeAgo(item.created_at),
          timestamp: item.created_at,
          icon: getIconForAction(item.action),
          color: getColorForAction(item.action),
          oldPath: item.old_path,
          metadata: item.metadata,
        }));
      }
    } catch (err) {
      console.error("Failed to load activity:", err);
    }
  }

  // Watch for filter changes
  $effect(() => {
    if (activityFilter) {
      loadActivityTimeline();
    }
  });

  // Comprehensive action configuration
  const actionConfig = {
    // File operations
    upload: { label: "Hochgeladen", icon: "bi-upload", color: "green" },
    download: { label: "Heruntergeladen", icon: "bi-download", color: "blue" },
    delete: { label: "Gelöscht", icon: "bi-trash", color: "red" },
    rename: { label: "Umbenannt", icon: "bi-pencil", color: "amber" },
    move: { label: "Verschoben", icon: "bi-arrow-right", color: "purple" },
    copy: { label: "Kopiert", icon: "bi-files", color: "indigo" },
    create: { label: "Erstellt", icon: "bi-plus-circle", color: "teal" },
    preview: { label: "Vorschau", icon: "bi-eye", color: "sky" },
    restore: { label: "Wiederhergestellt", icon: "bi-arrow-counterclockwise", color: "emerald" },
    
    // Folder operations
    folder_create: { label: "Ordner erstellt", icon: "bi-folder-plus", color: "blue" },
    folder_delete: { label: "Ordner gelöscht", icon: "bi-folder-x", color: "red" },
    folder_rename: { label: "Ordner umbenannt", icon: "bi-pencil", color: "amber" },
    folder_move: { label: "Ordner verschoben", icon: "bi-arrow-right", color: "purple" },
    folder_color: { label: "Farbe geändert", icon: "bi-palette", color: "pink" },
    
    // Favorites
    favorite: { label: "Favorisiert", icon: "bi-star-fill", color: "yellow" },
    unfavorite: { label: "Entfavorisiert", icon: "bi-star", color: "gray" },
    
    // Sharing
    share: { label: "Geteilt", icon: "bi-share", color: "violet" },
    unshare: { label: "Freigabe beendet", icon: "bi-share-fill", color: "slate" },
    share_access: { label: "Zugriff via Link", icon: "bi-link-45deg", color: "cyan" },
    
    // Comments & Tags
    comment_add: { label: "Kommentiert", icon: "bi-chat-dots", color: "lime" },
    comment_delete: { label: "Kommentar gelöscht", icon: "bi-chat-dots-fill", color: "gray" },
    tag_add: { label: "Tag hinzugefügt", icon: "bi-tag", color: "orange" },
    tag_remove: { label: "Tag entfernt", icon: "bi-tag-fill", color: "gray" },
    
    // Versioning
    version_create: { label: "Version erstellt", icon: "bi-clock-history", color: "fuchsia" },
    version_restore: { label: "Version wiederhergestellt", icon: "bi-arrow-counterclockwise", color: "emerald" },
    version_delete: { label: "Version gelöscht", icon: "bi-trash", color: "red" },
    
    // Auth
    login: { label: "Angemeldet", icon: "bi-box-arrow-in-right", color: "green" },
    logout: { label: "Abgemeldet", icon: "bi-box-arrow-right", color: "slate" },
    password_change: { label: "Passwort geändert", icon: "bi-key", color: "amber" },
    "2fa_enable": { label: "2FA aktiviert", icon: "bi-shield-check", color: "green" },
    "2fa_disable": { label: "2FA deaktiviert", icon: "bi-shield", color: "red" },
    
    // Settings
    settings_change: { label: "Einstellungen geändert", icon: "bi-gear", color: "slate" },
    profile_update: { label: "Profil aktualisiert", icon: "bi-person", color: "blue" },
  };

  const formatAction = (action) => actionConfig[action]?.label || "Geändert";
  const getIconForAction = (action) => actionConfig[action]?.icon || "bi-file-earmark";
  const getColorForAction = (action) => actionConfig[action]?.color || "gray";

  const extractFileName = (path) => {
    if (!path) return "Datei";
    const parts = path.split("/");
    return parts[parts.length - 1] || path;
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
    if (diffMins < 60) return `vor ${diffMins} min`;
    if (diffHours < 24) return `vor ${diffHours} h`;
    if (diffDays < 7) return `vor ${diffDays} d`;
    return date.toLocaleDateString("de-DE");
  };

  const formatBytes = (bytes) => {
    if (!bytes || bytes === 0) return "";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  };

  function navigateToFile(activity) {
    if (activity.filePath) {
      const path = activity.filePath.split("/").slice(0, -1).join("/") || "/";
      window.location.hash = `#/files?path=${encodeURIComponent(path)}`;
    }
  }

  function setFilter(filterValue) {
    activityFilter = filterValue;
  }

  // Color class helper
  const colorClasses = {
    green: "bg-green-500/20 text-green-600 dark:text-green-400",
    blue: "bg-blue-500/20 text-blue-600 dark:text-blue-400",
    red: "bg-red-500/20 text-red-600 dark:text-red-400",
    amber: "bg-amber-500/20 text-amber-600 dark:text-amber-400",
    purple: "bg-purple-500/20 text-purple-600 dark:text-purple-400",
    indigo: "bg-indigo-500/20 text-indigo-600 dark:text-indigo-400",
    teal: "bg-teal-500/20 text-teal-600 dark:text-teal-400",
    sky: "bg-sky-500/20 text-sky-600 dark:text-sky-400",
    emerald: "bg-emerald-500/20 text-emerald-600 dark:text-emerald-400",
    pink: "bg-pink-500/20 text-pink-600 dark:text-pink-400",
    yellow: "bg-yellow-500/20 text-yellow-600 dark:text-yellow-400",
    gray: "bg-gray-500/20 text-gray-600 dark:text-gray-400",
    violet: "bg-violet-500/20 text-violet-600 dark:text-violet-400",
    slate: "bg-slate-500/20 text-slate-600 dark:text-slate-400",
    cyan: "bg-cyan-500/20 text-cyan-600 dark:text-cyan-400",
    lime: "bg-lime-500/20 text-lime-600 dark:text-lime-400",
    orange: "bg-orange-500/20 text-orange-600 dark:text-orange-400",
    fuchsia: "bg-fuchsia-500/20 text-fuchsia-600 dark:text-fuchsia-400",
  };

  const getColorClass = (color) => colorClasses[color] || colorClasses.gray;
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

      <!-- Sessions Card -->
      <div class="glass-stat-card">
        <div class="flex items-center justify-between mb-4">
          <div
            class="w-10 h-10 rounded-lg bg-emerald-500/20 flex items-center justify-center"
          >
            <i class="bi bi-people-fill text-emerald-500" aria-hidden="true"></i>
          </div>
          <span
            class="text-xs bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 px-2 py-1 rounded-full"
          >
            Online
          </span>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-sm">Aktive Sitzungen</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white mt-2">
          {stats.activeSessions}
        </p>
        <p class="text-xs text-gray-500 dark:text-gray-500 mt-2">
          Benutzer aktiv
        </p>
      </div>

      <!-- Actions Today Card -->
      <div class="glass-stat-card">
        <div class="flex items-center justify-between mb-4">
          <div
            class="w-10 h-10 rounded-lg bg-pink-500/20 flex items-center justify-center"
          >
            <i class="bi bi-lightning-fill text-pink-500" aria-hidden="true"></i>
          </div>
          <span
            class="text-xs bg-slate-500/20 text-slate-600 dark:text-slate-400 px-2 py-1 rounded-full"
          >
            Heute
          </span>
        </div>
        <p class="text-gray-600 dark:text-gray-400 text-sm">Aktionen heute</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white mt-2">
          {stats.actionsToday}
        </p>
        <p class="text-xs text-gray-500 dark:text-gray-500 mt-2">Aktivitäten</p>
      </div>
    </div>

    <!-- Unified Activity Timeline -->
    <div class="glass-card-primary">
      <div
        class="flex flex-col sm:flex-row sm:items-center justify-between mb-6 pb-6 border-b border-gray-200/50 dark:border-gray-700/50 gap-4"
      >
        <h2
          class="text-xl font-semibold text-gray-900 dark:text-white flex items-center gap-2"
        >
          <i class="bi bi-activity text-amber-500" aria-hidden="true"></i>
          Aktivitätsverlauf
        </h2>
        
        <!-- Filter Buttons -->
        <div class="flex flex-wrap gap-2">
          {#each filterOptions as option}
            <button
              onclick={() => setFilter(option.value)}
              class="px-3 py-1.5 text-xs font-medium rounded-full transition-all flex items-center gap-1.5
                {activityFilter === option.value 
                  ? 'bg-blue-500 text-white' 
                  : 'bg-gray-200/50 dark:bg-gray-700/50 text-gray-600 dark:text-gray-400 hover:bg-gray-300/50 dark:hover:bg-gray-600/50'}"
            >
              <i class="bi {option.icon}" aria-hidden="true"></i>
              {option.label}
            </button>
          {/each}
        </div>
      </div>

      {#if loading}
        <div class="flex items-center justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-2 border-blue-500 border-t-transparent"></div>
        </div>
      {:else if activityTimeline.length === 0}
        <div class="text-center py-12 text-gray-500 dark:text-gray-400">
          <i class="bi bi-inbox text-4xl mb-3 block opacity-50" aria-hidden="true"></i>
          <p>Keine Aktivitäten gefunden</p>
          <p class="text-sm mt-1 opacity-75">
            {activityFilter !== "all" ? "Versuche einen anderen Filter" : "Aktivitäten werden hier angezeigt"}
          </p>
        </div>
      {:else}
        <div class="space-y-3">
          {#each activityTimeline as activity (activity.id)}
            <button
              onclick={() => navigateToFile(activity)}
              class="w-full text-left p-4 rounded-xl bg-gray-50/50 dark:bg-gray-800/30 
                     hover:bg-gray-100/50 dark:hover:bg-gray-700/30 transition-all group"
            >
              <div class="flex items-center gap-4">
                <!-- Icon with color -->
                <div
                  class="w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0 {getColorClass(activity.color)}"
                >
                  <i class="bi {activity.icon}" aria-hidden="true"></i>
                </div>
                
                <!-- Content -->
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="font-medium text-gray-900 dark:text-white text-sm">
                      {activity.actionLabel}
                    </span>
                    {#if activity.username && activity.username !== "Du"}
                      <span class="text-xs text-gray-500 dark:text-gray-500">
                        von {activity.username}
                      </span>
                    {/if}
                  </div>
                  <p class="text-gray-600 dark:text-gray-400 text-xs truncate mt-0.5">
                    {activity.fileName}
                    {#if activity.fileSize}
                      <span class="text-gray-400 dark:text-gray-500 ml-2">
                        ({formatBytes(activity.fileSize)})
                      </span>
                    {/if}
                  </p>
                </div>
                
                <!-- Time -->
                <span class="text-gray-500 dark:text-gray-500 text-xs flex-shrink-0">
                  {activity.time}
                </span>
                
                <!-- Hover Arrow -->
                <i 
                  class="bi bi-chevron-right text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0"
                  aria-hidden="true"
                ></i>
              </div>
            </button>
          {/each}
        </div>
        
        <!-- View All Link -->
        <div class="mt-6 pt-4 border-t border-gray-200/50 dark:border-gray-700/50 text-center">
          <a
            href="#/activity"
            class="text-blue-600 dark:text-blue-400 text-sm hover:underline inline-flex items-center gap-1"
          >
            Alle Aktivitäten anzeigen
            <i class="bi bi-arrow-right" aria-hidden="true"></i>
          </a>
        </div>
      {/if}
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
