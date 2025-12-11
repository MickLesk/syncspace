<script>
  import { onMount } from "svelte";
  import { currentLang, currentView } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import api, { API_BASE } from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  function navigateTo(view) {
    currentView.set(view);
  }

  // Dashboard state
  let storageUsed = $state(0);
  let storageTotal = $state(100);
  let storagePercent = $state(0);
  let fileCount = $state(0);
  let folderCount = $state(0);
  let favorites = $state([]);
  let recentActivity = $state([]);
  let loading = $state(true);

  // Upload
  let isDragging = $state(false);
  let uploadInput;

  onMount(async () => {
    await loadDashboardData();
  });

  async function loadDashboardData() {
    loading = true;
    try {
      const token = localStorage.getItem("authToken");
      const headers = { Authorization: `Bearer ${token}` };

      const [statsRes, activityRes, favoritesRes, filesRes] = await Promise.all(
        [
          fetch(`${API_BASE}/dashboard/stats`, { headers }),
          fetch(`${API_BASE}/activity?limit=8`, { headers }),
          fetch(`${API_BASE}/favorites/list`, { headers }).catch(() => ({
            ok: false,
          })),
          fetch(`${API_BASE}/files`, { headers }).catch(() => ({
            ok: false,
          })),
        ]
      );

      // Process storage stats
      if (statsRes.ok) {
        const data = await statsRes.json();
        if (data.overview) {
          const usedGB = data.overview.total_storage_bytes / 1024 ** 3;
          storageUsed = usedGB;
          storageTotal = 100;
          storagePercent = Math.min((usedGB / storageTotal) * 100, 100);
          fileCount = data.overview.total_files || 0;
        }
      }

      // Count folders from files API (more accurate)
      if (filesRes.ok) {
        const files = await filesRes.json();
        folderCount = (files || []).filter((f) => f.is_directory).length;
        // Also update file count from actual data if stats didn't load
        if (fileCount === 0) {
          fileCount = (files || []).filter((f) => !f.is_directory).length;
        }
      }

      // Process activity
      if (activityRes.ok) {
        const activities = await activityRes.json();
        recentActivity = (activities || []).slice(0, 6).map((a) => ({
          id: a.id,
          action: a.action,
          fileName: extractFileName(a.file_path || a.file_name),
          filePath: a.file_path || a.file_name || "",
          time: formatTimeAgo(a.created_at),
          icon: getActionIcon(a.action),
          color: getActionColor(a.action),
        }));
      }

      // Process favorites
      if (favoritesRes.ok) {
        const favs = await favoritesRes.json();
        favorites = (favs || []).slice(0, 6).map((f) => ({
          id: f.id,
          name: extractFileName(f.item_path || f.item_id),
          type: f.item_type,
          path: f.item_path || f.item_id,
        }));
      }
    } catch (err) {
      console.error("Failed to load dashboard:", err);
    } finally {
      loading = false;
    }
  }

  function extractFileName(path) {
    if (!path) return "Unbekannt";
    return path.split("/").pop() || path;
  }

  function formatTimeAgo(timestamp) {
    if (!timestamp) return "";
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
  }

  const actionConfig = {
    upload: { icon: "bi-cloud-upload", color: "emerald" },
    download: { icon: "bi-cloud-download", color: "blue" },
    delete: { icon: "bi-trash", color: "red" },
    rename: { icon: "bi-pencil", color: "amber" },
    move: { icon: "bi-arrow-right", color: "purple" },
    copy: { icon: "bi-files", color: "indigo" },
    share: { icon: "bi-share", color: "violet" },
    favorite: { icon: "bi-star-fill", color: "yellow" },
    folder_create: { icon: "bi-folder-plus", color: "blue" },
    folder_color: { icon: "bi-palette", color: "pink" },
  };

  function getActionIcon(action) {
    return actionConfig[action]?.icon || "bi-file-earmark";
  }

  function getActionColor(action) {
    return actionConfig[action]?.color || "gray";
  }

  function handleDragOver(e) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e) {
    e.preventDefault();
    isDragging = false;
  }

  async function handleDrop(e) {
    e.preventDefault();
    isDragging = false;
    const files = e.dataTransfer?.files;
    if (files?.length) {
      await uploadFiles(files);
    }
  }

  function triggerUpload() {
    const input = document.getElementById("dashboard-file-input");
    if (input) input.click();
  }

  async function handleFileSelect(e) {
    const files = e.target.files;
    if (files?.length) {
      await uploadFiles(files);
    }
    // Reset input for re-upload of same file
    e.target.value = "";
  }

  let uploadProgress = $state([]);
  let isUploading = $state(false);

  async function uploadFiles(fileList) {
    isUploading = true;
    uploadProgress = Array.from(fileList).map((f) => ({
      name: f.name,
      progress: 0,
      status: "pending",
    }));

    try {
      const token = localStorage.getItem("authToken");

      for (let i = 0; i < fileList.length; i++) {
        const file = fileList[i];
        uploadProgress[i].status = "uploading";

        const formData = new FormData();
        formData.append("file", file);

        const res = await fetch(`${API_BASE}/upload`, {
          method: "POST",
          headers: {
            Authorization: `Bearer ${token}`,
          },
          body: formData,
        });

        if (res.ok) {
          uploadProgress[i].progress = 100;
          uploadProgress[i].status = "complete";
        } else {
          uploadProgress[i].status = "error";
        }
      }

      // Refresh data after all uploads
      await loadDashboardData();

      // Clear progress after delay
      setTimeout(() => {
        uploadProgress = [];
        isUploading = false;
      }, 2000);
    } catch (err) {
      console.error("Upload failed:", err);
      isUploading = false;
    }
  }

  function openFavorite(fav) {
    navigateTo("files");
  }

  // Activity item click - open preview
  let selectedFile = $state(null);
  let showPreview = $state(false);

  function openActivityItem(activity) {
    if (activity.filePath) {
      selectedFile = {
        name: activity.fileName,
        path: activity.filePath,
        file_path: activity.filePath,
      };
      showPreview = true;
    }
  }

  function closePreview() {
    showPreview = false;
    selectedFile = null;
  }
</script>

<PageWrapper title="Dashboard" showSidebar={true}>
  <div class="w-full p-6 min-h-[calc(100vh-80px)]">
    {#if loading}
      <div class="flex justify-center items-center min-h-[400px]">
        <span class="loading loading-spinner loading-lg text-success"></span>
      </div>
    {:else}
      <!-- Top Row: Quick Stats Only -->
      <div class="flex gap-6 mb-6 flex-col lg:flex-row">
        <!-- Quick Stats -->
        <div class="flex gap-4 flex-wrap">
          <div class="flex items-center gap-3 bg-base-100 border border-base-300 rounded-xl px-5 py-4">
            <div class="w-10 h-10 rounded-lg flex items-center justify-center text-xl bg-blue-100 text-blue-500 dark:bg-blue-500/20">
              <i class="bi bi-file-earmark"></i>
            </div>
            <div class="flex flex-col">
              <span class="text-xl font-bold text-base-content">{fileCount}</span>
              <span class="text-xs text-base-content/60">Dateien</span>
            </div>
          </div>
          <div class="flex items-center gap-3 bg-base-100 border border-base-300 rounded-xl px-5 py-4">
            <div class="w-10 h-10 rounded-lg flex items-center justify-center text-xl bg-amber-100 text-amber-500 dark:bg-amber-500/20">
              <i class="bi bi-folder"></i>
            </div>
            <div class="flex flex-col">
              <span class="text-xl font-bold text-base-content">{folderCount}</span>
              <span class="text-xs text-base-content/60">Ordner</span>
            </div>
          </div>
          <div class="flex items-center gap-3 bg-base-100 border border-base-300 rounded-xl px-5 py-4">
            <div class="w-10 h-10 rounded-lg flex items-center justify-center text-xl bg-emerald-100 text-emerald-500 dark:bg-emerald-500/20">
              <i class="bi bi-hdd"></i>
            </div>
            <div class="flex flex-col">
              <span class="text-xl font-bold text-base-content">{storageUsed.toFixed(1)} GB</span>
              <span class="text-xs text-base-content/60">Speicher</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Main Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Left Column -->
        <div class="flex flex-col gap-6">
          <!-- Upload Zone -->
          <div
            class="card bg-base-100 border-2 border-dashed border-base-300 rounded-xl p-5 text-center transition-all {isDragging ? 'border-success bg-success/5' : ''}"
            role="button"
            tabindex="0"
            aria-label="Upload-Bereich"
            ondragover={handleDragOver}
            ondragleave={handleDragLeave}
            ondrop={handleDrop}
          >
            <input
              type="file"
              id="dashboard-file-input"
              multiple
              onchange={handleFileSelect}
              class="hidden"
            />
            <div class="py-6">
              <div class="text-5xl text-success mb-3">
                <i class="bi bi-cloud-arrow-up"></i>
              </div>
              <h3 class="text-lg font-semibold text-base-content mb-1">Dateien hochladen</h3>
              <p class="text-base-content/60 mb-4">Ziehe Dateien hierher oder</p>
              <button 
                class="btn btn-success gap-2"
                onclick={triggerUpload}
              >
                <i class="bi bi-folder2-open"></i>
                Dateien auswählen
              </button>

              {#if uploadProgress.length > 0}
                <div class="mt-4 pt-4 border-t border-dashed border-base-300">
                  {#each uploadProgress as up}
                    <div class="flex items-center gap-2 text-sm py-1 {up.status === 'complete' ? 'text-success' : up.status === 'error' ? 'text-error' : 'text-base-content/60'}">
                      <i class="bi {up.status === 'complete' ? 'bi-check-circle-fill' : up.status === 'error' ? 'bi-x-circle-fill' : 'bi-hourglass-split'}"></i>
                      <span>{up.name}</span>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          </div>

          <!-- Quick Actions -->
          <div class="card bg-base-100 border border-base-300 rounded-xl p-5">
            <h3 class="text-base font-semibold text-base-content flex items-center gap-2 mb-4">
              <i class="bi bi-lightning"></i>
              Schnellzugriff
            </h3>
            <div class="grid grid-cols-4 gap-3">
              <button class="flex flex-col items-center gap-2 p-4 bg-base-200 border border-base-300 rounded-lg hover:border-success hover:-translate-y-0.5 transition-all" onclick={() => navigateTo("files")}>
                <i class="bi bi-folder2-open text-2xl text-success"></i>
                <span class="text-xs text-base-content/60">Dateien</span>
              </button>
              <button class="flex flex-col items-center gap-2 p-4 bg-base-200 border border-base-300 rounded-lg hover:border-success hover:-translate-y-0.5 transition-all" onclick={() => navigateTo("shared")}>
                <i class="bi bi-share text-2xl text-success"></i>
                <span class="text-xs text-base-content/60">Freigaben</span>
              </button>
              <button class="flex flex-col items-center gap-2 p-4 bg-base-200 border border-base-300 rounded-lg hover:border-success hover:-translate-y-0.5 transition-all" onclick={() => navigateTo("favorites")}>
                <i class="bi bi-star text-2xl text-success"></i>
                <span class="text-xs text-base-content/60">Favoriten</span>
              </button>
              <button class="flex flex-col items-center gap-2 p-4 bg-base-200 border border-base-300 rounded-lg hover:border-success hover:-translate-y-0.5 transition-all" onclick={() => navigateTo("trash")}>
                <i class="bi bi-trash text-2xl text-success"></i>
                <span class="text-xs text-base-content/60">Papierkorb</span>
              </button>
            </div>
          </div>

          <!-- Storage -->
          <div class="card bg-base-100 border border-base-300 rounded-xl p-5">
            <h3 class="text-base font-semibold text-base-content flex items-center gap-2 mb-4">
              <i class="bi bi-hdd-stack"></i>
              Speicherplatz
            </h3>
            <div class="h-2 bg-base-300 rounded overflow-hidden mb-3">
              <div class="h-full bg-gradient-to-r from-success to-green-600 rounded transition-all duration-500" style="width: {storagePercent}%"></div>
            </div>
            <div class="flex justify-between text-sm text-base-content/60">
              <span>{storageUsed.toFixed(2)} GB belegt</span>
              <span>{(storageTotal - storageUsed).toFixed(2)} GB frei</span>
            </div>
          </div>
        </div>

        <!-- Right Column -->
        <div class="flex flex-col gap-6">
          <!-- Favorites -->
          <div class="card bg-base-100 border border-base-300 rounded-xl p-5">
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-base font-semibold text-base-content flex items-center gap-2">
                <i class="bi bi-star-fill text-yellow-500"></i>
                Favoriten
              </h3>
              <button class="btn btn-ghost btn-sm text-success" onclick={() => navigateTo("favorites")}>
                Alle anzeigen →
              </button>
            </div>
            {#if favorites.length === 0}
              <div class="flex flex-col items-center justify-center py-6 text-base-content/40">
                <i class="bi bi-star text-3xl mb-2 opacity-50"></i>
                <p class="text-sm">Keine Favoriten</p>
              </div>
            {:else}
              <div class="flex flex-col gap-2">
                {#each favorites as fav (fav.id)}
                  <button
                    class="flex items-center gap-3 px-3 py-2.5 bg-base-200 border border-transparent rounded-lg text-left hover:border-success transition-all"
                    onclick={() => openFavorite(fav)}
                  >
                    <i class="bi {fav.type === 'folder' ? 'bi-folder-fill text-blue-500' : 'bi-file-earmark text-base-content/60'} text-lg"></i>
                    <span class="text-sm text-base-content truncate">{fav.name}</span>
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Recent Activity -->
          <div class="card bg-base-100 border border-base-300 rounded-xl p-5">
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-base font-semibold text-base-content flex items-center gap-2">
                <i class="bi bi-clock-history"></i>
                Letzte Aktivitäten
              </h3>
              <button class="btn btn-ghost btn-sm text-success" onclick={() => navigateTo("activity")}>
                Alle anzeigen →
              </button>
            </div>
            {#if recentActivity.length === 0}
              <div class="flex flex-col items-center justify-center py-6 text-base-content/40">
                <i class="bi bi-clock text-3xl mb-2 opacity-50"></i>
                <p class="text-sm">Keine Aktivitäten</p>
              </div>
            {:else}
              <div class="flex flex-col gap-2">
                {#each recentActivity as activity (activity.id)}
                  <button
                    class="flex items-center gap-3 py-2 w-full bg-transparent border-none cursor-pointer rounded-md hover:bg-base-200 transition-colors"
                    onclick={() => openActivityItem(activity)}
                  >
                    <div class="w-8 h-8 rounded-md flex items-center justify-center text-sm
                      {activity.color === 'emerald' ? 'bg-emerald-500/15 text-emerald-500' : ''}
                      {activity.color === 'blue' ? 'bg-blue-500/15 text-blue-500' : ''}
                      {activity.color === 'red' ? 'bg-red-500/15 text-red-500' : ''}
                      {activity.color === 'amber' ? 'bg-amber-500/15 text-amber-500' : ''}
                      {activity.color === 'purple' ? 'bg-purple-500/15 text-purple-500' : ''}
                      {activity.color === 'indigo' ? 'bg-indigo-500/15 text-indigo-500' : ''}
                      {activity.color === 'violet' ? 'bg-violet-500/15 text-violet-500' : ''}
                      {activity.color === 'yellow' ? 'bg-yellow-500/15 text-yellow-500' : ''}
                      {activity.color === 'pink' ? 'bg-pink-500/15 text-pink-500' : ''}
                      {activity.color === 'gray' ? 'bg-gray-500/15 text-gray-500' : ''}
                    ">
                      <i class="bi {activity.icon}"></i>
                    </div>
                    <div class="flex-1 flex justify-between items-center min-w-0">
                      <span class="text-sm text-base-content truncate">{activity.fileName}</span>
                      <span class="text-xs text-base-content/40 shrink-0 ml-2">{activity.time}</span>
                    </div>
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="text-center py-8 text-xs text-base-content/40">
        <span>SyncSpace v1.0</span>
      </div>
    {/if}
  </div>

  <!-- File Preview Modal -->
  {#if showPreview && selectedFile}
    <div
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-[1000]"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={closePreview}
      onkeydown={(e) => e.key === "Escape" && closePreview()}
    >
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="bg-base-100 rounded-xl w-[90%] max-w-[400px] p-6 shadow-2xl"
        role="document"
        onclick={(e) => e.stopPropagation()}
        onkeydown={() => {}}
      >
        <div class="flex justify-between items-start mb-6">
          <h3 class="text-lg font-semibold text-base-content break-words">
            {selectedFile.name ||
              extractFileName(selectedFile.path || selectedFile.file_path)}
          </h3>
          <button
            class="btn btn-ghost btn-sm btn-square -mt-1 -mr-1"
            onclick={closePreview}
            aria-label="Schließen"
          >
            <i class="bi bi-x-lg"></i>
          </button>
        </div>
        <div class="flex flex-col gap-3">
          <button
            class="btn btn-ghost justify-center gap-2"
            onclick={() => {
              navigateTo("files");
              closePreview();
            }}
          >
            <i class="bi bi-folder2-open"></i>
            Im Ordner öffnen
          </button>
          <a
            href="{API_BASE}/download/{encodeURIComponent(
              selectedFile.path || selectedFile.file_path || ''
            )}"
            class="btn btn-success justify-center gap-2"
            download
          >
            <i class="bi bi-download"></i>
            Herunterladen
          </a>
        </div>
      </div>
    </div>
  {/if}
</PageWrapper>

<style>
  /* Minimal custom styles - only for complex animations not available in Tailwind */
</style>
