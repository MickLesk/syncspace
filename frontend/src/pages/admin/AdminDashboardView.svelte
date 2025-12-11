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
      <!-- Top Row: Quick Stats with Soft Glow Design -->
      <div class="flex gap-4 mb-6 flex-wrap">
        <!-- Quick Stats with Soft Shadows (No Borders) -->
        <div
          class="flex items-center gap-4 bg-gradient-to-br from-blue-500/5 to-transparent rounded-2xl px-6 py-5 backdrop-blur-xl transition-all hover:scale-[1.02] shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)]"
        >
          <div
            class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl bg-blue-500/15 text-blue-500 shadow-inner"
          >
            <i class="bi bi-file-earmark"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-2xl font-bold text-base-content tracking-tight"
              >{fileCount}</span
            >
            <span class="text-sm text-base-content/60 font-medium">Dateien</span
            >
          </div>
        </div>

        <div
          class="flex items-center gap-4 bg-gradient-to-br from-amber-500/5 to-transparent rounded-2xl px-6 py-5 backdrop-blur-xl transition-all hover:scale-[1.02] shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)]"
        >
          <div
            class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl bg-amber-500/15 text-amber-500 shadow-inner"
          >
            <i class="bi bi-folder"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-2xl font-bold text-base-content tracking-tight"
              >{folderCount}</span
            >
            <span class="text-sm text-base-content/60 font-medium">Ordner</span>
          </div>
        </div>

        <div
          class="flex items-center gap-4 bg-gradient-to-br from-emerald-500/5 to-transparent rounded-2xl px-6 py-5 backdrop-blur-xl transition-all hover:scale-[1.02] shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)]"
        >
          <div
            class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl bg-emerald-500/15 text-emerald-500 shadow-inner"
          >
            <i class="bi bi-hdd"></i>
          </div>
          <div class="flex flex-col">
            <span class="text-2xl font-bold text-base-content tracking-tight"
              >{storageUsed.toFixed(1)} GB</span
            >
            <span class="text-sm text-base-content/60 font-medium"
              >Speicher</span
            >
          </div>
        </div>
      </div>

      <!-- Main Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Left Column -->
        <div class="flex flex-col gap-6">
          <!-- Upload Zone with Soft Glow Design -->
          <div
            class="relative bg-gradient-to-br from-emerald-500/5 via-transparent to-emerald-500/5 rounded-2xl p-8 text-center transition-all backdrop-blur-xl overflow-hidden {isDragging
              ? 'bg-emerald-500/10 scale-[1.02] shadow-[0_12px_32px_rgba(34,197,94,0.25)]'
              : 'shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)]'}"
            role="button"
            tabindex="0"
            aria-label="Upload-Bereich"
            ondragover={handleDragOver}
            ondragleave={handleDragLeave}
            ondrop={handleDrop}
          >
            <!-- Decorative background gradient -->
            <div
              class="absolute inset-0 bg-gradient-to-br from-emerald-500/0 via-emerald-500/5 to-emerald-500/0 pointer-events-none"
            ></div>

            <input
              type="file"
              id="dashboard-file-input"
              multiple
              onchange={handleFileSelect}
              class="hidden"
            />
            <div class="relative py-6">
              <div
                class="text-6xl mb-4 bg-gradient-to-br from-emerald-500 to-green-600 bg-clip-text text-transparent"
              >
                <i class="bi bi-cloud-arrow-up"></i>
              </div>
              <h3 class="text-xl font-bold text-base-content mb-2">
                Dateien hochladen
              </h3>
              <p class="text-base-content/60 mb-5 text-sm">
                Ziehe Dateien hierher oder
              </p>
              <button
                class="btn btn-success gap-2 shadow-lg hover:shadow-emerald-500/50 hover:scale-105 transition-all"
                onclick={triggerUpload}
              >
                <i class="bi bi-folder2-open"></i>
                Dateien auswählen
              </button>

              {#if uploadProgress.length > 0}
                <div class="mt-4 pt-4 border-t border-dashed border-base-300">
                  {#each uploadProgress as up}
                    <div
                      class="flex items-center gap-2 text-sm py-1 {up.status ===
                      'complete'
                        ? 'text-success'
                        : up.status === 'error'
                          ? 'text-error'
                          : 'text-base-content/60'}"
                    >
                      <i
                        class="bi {up.status === 'complete'
                          ? 'bi-check-circle-fill'
                          : up.status === 'error'
                            ? 'bi-x-circle-fill'
                            : 'bi-hourglass-split'}"
                      ></i>
                      <span>{up.name}</span>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          </div>

          <!-- Quick Actions with Soft Glow Design -->
          <div
            class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-2xl p-6 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)]"
          >
            <h3
              class="text-lg font-bold text-base-content flex items-center gap-2 mb-5"
            >
              <i class="bi bi-lightning-charge-fill text-emerald-500"></i>
              Schnellzugriff
            </h3>
            <div class="grid grid-cols-4 gap-3">
              <button
                class="group flex flex-col items-center gap-3 p-5 bg-gradient-to-br from-emerald-500/5 to-transparent rounded-xl hover:scale-105 transition-all shadow-[0_2px_8px_rgba(0,0,0,0.06)] hover:shadow-[0_4px_16px_rgba(34,197,94,0.2)]"
                onclick={() => navigateTo("files")}
              >
                <div
                  class="w-10 h-10 rounded-lg bg-emerald-500/20 flex items-center justify-center group-hover:bg-emerald-500/30 transition-colors"
                >
                  <i class="bi bi-folder2-open text-xl text-emerald-500"></i>
                </div>
                <span class="text-xs font-medium text-base-content"
                  >Dateien</span
                >
              </button>
              <button
                class="group flex flex-col items-center gap-3 p-5 bg-gradient-to-br from-blue-500/5 to-transparent rounded-xl hover:scale-105 transition-all shadow-[0_2px_8px_rgba(0,0,0,0.06)] hover:shadow-[0_4px_16px_rgba(59,130,246,0.2)]"
                onclick={() => navigateTo("shared")}
              >
                <div
                  class="w-10 h-10 rounded-lg bg-blue-500/20 flex items-center justify-center group-hover:bg-blue-500/30 transition-colors"
                >
                  <i class="bi bi-share text-xl text-blue-500"></i>
                </div>
                <span class="text-xs font-medium text-base-content"
                  >Freigaben</span
                >
              </button>
              <button
                class="group flex flex-col items-center gap-3 p-5 bg-gradient-to-br from-yellow-500/5 to-transparent rounded-xl hover:scale-105 transition-all shadow-[0_2px_8px_rgba(0,0,0,0.06)] hover:shadow-[0_4px_16px_rgba(234,179,8,0.2)]"
                onclick={() => navigateTo("favorites")}
              >
                <div
                  class="w-10 h-10 rounded-lg bg-yellow-500/20 flex items-center justify-center group-hover:bg-yellow-500/30 transition-colors"
                >
                  <i class="bi bi-star-fill text-xl text-yellow-500"></i>
                </div>
                <span class="text-xs font-medium text-base-content"
                  >Favoriten</span
                >
              </button>
              <button
                class="group flex flex-col items-center gap-3 p-5 bg-gradient-to-br from-red-500/5 to-transparent rounded-xl hover:scale-105 transition-all shadow-[0_2px_8px_rgba(0,0,0,0.06)] hover:shadow-[0_4px_16px_rgba(239,68,68,0.2)]"
                onclick={() => navigateTo("trash")}
              >
                <div
                  class="w-10 h-10 rounded-lg bg-red-500/20 flex items-center justify-center group-hover:bg-red-500/30 transition-colors"
                >
                  <i class="bi bi-trash text-xl text-red-500"></i>
                </div>
                <span class="text-xs font-medium text-base-content"
                  >Papierkorb</span
                >
              </button>
            </div>
          </div>

          <!-- Storage with Soft Glow Design -->
          <div
            class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-2xl p-6 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
          >
            <h3
              class="text-lg font-bold text-base-content flex items-center gap-2 mb-5"
            >
              <i class="bi bi-hdd-stack-fill text-emerald-500"></i>
              Speicherplatz
            </h3>
            <div
              class="h-3 bg-base-300/50 rounded-full overflow-hidden mb-4 shadow-inner"
            >
              <div
                class="h-full bg-gradient-to-r from-emerald-500 via-green-500 to-green-600 rounded-full transition-all duration-700 shadow-lg"
                style="width: {storagePercent}%"
              ></div>
            </div>
            <div class="flex justify-between text-sm font-medium">
              <span class="text-base-content"
                >{storageUsed.toFixed(2)} GB
                <span class="text-base-content/60">belegt</span></span
              >
              <span class="text-base-content"
                >{(storageTotal - storageUsed).toFixed(2)} GB
                <span class="text-base-content/60">frei</span></span
              >
            </div>
          </div>
        </div>

        <!-- Right Column -->
        <div class="flex flex-col gap-6">
          <!-- Favorites with Soft Glow Design -->
          <div
            class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-2xl p-6 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)]"
          >
            <div class="flex justify-between items-center mb-5">
              <h3
                class="text-lg font-bold text-base-content flex items-center gap-2"
              >
                <i class="bi bi-star-fill text-yellow-500"></i>
                Favoriten
              </h3>
              <button
                class="text-sm font-medium text-emerald-500 hover:text-emerald-600 transition-colors flex items-center gap-1"
                onclick={() => navigateTo("favorites")}
              >
                Alle anzeigen
                <i class="bi bi-arrow-right"></i>
              </button>
            </div>
            {#if favorites.length === 0}
              <div
                class="flex flex-col items-center justify-center py-6 text-base-content/40"
              >
                <i class="bi bi-star text-3xl mb-2 opacity-50"></i>
                <p class="text-sm">Keine Favoriten</p>
              </div>
            {:else}
              <div class="flex flex-col gap-2">
                {#each favorites as fav (fav.id)}
                  <button
                    class="group flex items-center gap-3 px-4 py-3 bg-gradient-to-r from-base-200/50 to-transparent rounded-xl text-left hover:bg-gradient-to-r hover:from-emerald-500/10 hover:to-transparent transition-all shadow-[0_2px_6px_rgba(0,0,0,0.04)] hover:shadow-[0_4px_12px_rgba(34,197,94,0.15)]"
                    onclick={() => openFavorite(fav)}
                  >
                    <div
                      class="w-8 h-8 rounded-lg bg-base-300/50 flex items-center justify-center group-hover:bg-emerald-500/20 transition-colors"
                    >
                      <i
                        class="bi {fav.type === 'folder'
                          ? 'bi-folder-fill text-blue-500'
                          : 'bi-file-earmark-fill text-base-content/60'} text-base"
                      ></i>
                    </div>
                    <span class="text-sm font-medium text-base-content truncate"
                      >{fav.name}</span
                    >
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Recent Activity with Soft Glow Design -->
          <div
            class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-2xl p-6 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)]"
          >
            <div class="flex justify-between items-center mb-5">
              <h3
                class="text-lg font-bold text-base-content flex items-center gap-2"
              >
                <i class="bi bi-clock-history text-emerald-500"></i>
                Letzte Aktivitäten
              </h3>
              <button
                class="text-sm font-medium text-emerald-500 hover:text-emerald-600 transition-colors flex items-center gap-1"
                onclick={() => navigateTo("activity")}
              >
                Alle anzeigen
                <i class="bi bi-arrow-right"></i>
              </button>
            </div>
            {#if recentActivity.length === 0}
              <div
                class="flex flex-col items-center justify-center py-6 text-base-content/40"
              >
                <i class="bi bi-clock text-3xl mb-2 opacity-50"></i>
                <p class="text-sm">Keine Aktivitäten</p>
              </div>
            {:else}
              <div class="flex flex-col gap-2">
                {#each recentActivity as activity (activity.id)}
                  <button
                    class="group flex items-center gap-3 px-4 py-3 w-full bg-transparent rounded-xl cursor-pointer hover:bg-gradient-to-r hover:from-base-200/50 hover:to-transparent transition-all shadow-[0_1px_3px_rgba(0,0,0,0.03)] hover:shadow-[0_4px_12px_rgba(0,0,0,0.08)]"
                    onclick={() => openActivityItem(activity)}
                  >
                    <div
                      class="w-9 h-9 rounded-xl flex items-center justify-center text-sm shadow-sm group-hover:shadow-md transition-shadow
                      {activity.color === 'emerald'
                        ? 'bg-emerald-500/15 text-emerald-500'
                        : ''}
                      {activity.color === 'blue'
                        ? 'bg-blue-500/15 text-blue-500'
                        : ''}
                      {activity.color === 'red'
                        ? 'bg-red-500/15 text-red-500'
                        : ''}
                      {activity.color === 'amber'
                        ? 'bg-amber-500/15 text-amber-500'
                        : ''}
                      {activity.color === 'purple'
                        ? 'bg-purple-500/15 text-purple-500'
                        : ''}
                      {activity.color === 'indigo'
                        ? 'bg-indigo-500/15 text-indigo-500'
                        : ''}
                      {activity.color === 'violet'
                        ? 'bg-violet-500/15 text-violet-500'
                        : ''}
                      {activity.color === 'yellow'
                        ? 'bg-yellow-500/15 text-yellow-500'
                        : ''}
                      {activity.color === 'pink'
                        ? 'bg-pink-500/15 text-pink-500'
                        : ''}
                      {activity.color === 'gray'
                        ? 'bg-gray-500/15 text-gray-500'
                        : ''}
                    "
                    >
                      <i class="bi {activity.icon}"></i>
                    </div>
                    <div
                      class="flex-1 flex justify-between items-center min-w-0"
                    >
                      <span
                        class="text-sm font-medium text-base-content truncate"
                        >{activity.fileName}</span
                      >
                      <span
                        class="text-xs text-base-content/50 shrink-0 ml-2 font-medium"
                        >{activity.time}</span
                      >
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

  <!-- File Preview Modal with Glassmorphism -->
  {#if showPreview && selectedFile}
    <div
      class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-[1000]"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={closePreview}
      onkeydown={(e) => e.key === "Escape" && closePreview()}
    >
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="bg-base-100/95 backdrop-blur-xl rounded-2xl w-[90%] max-w-[450px] p-6 shadow-[0_24px_48px_rgba(0,0,0,0.2)]"
        onclick={(e) => e.stopPropagation()}
      >
        <div class="flex justify-between items-start mb-6">
          <h3 class="text-lg font-bold text-base-content break-words pr-4">
            {selectedFile.name ||
              extractFileName(selectedFile.path || selectedFile.file_path)}
          </h3>
          <button
            class="btn btn-ghost btn-sm btn-circle hover:bg-base-200"
            onclick={closePreview}
            aria-label="Schließen"
          >
            <i class="bi bi-x-lg text-lg"></i>
          </button>
        </div>
        <div class="flex flex-col gap-3">
          <button
            class="btn btn-ghost justify-center gap-2 border border-base-300/50 hover:border-emerald-500/50 hover:bg-emerald-500/10"
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
            class="btn btn-success justify-center gap-2 shadow-lg hover:shadow-emerald-500/50"
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
