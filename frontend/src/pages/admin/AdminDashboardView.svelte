<script>
  import { onMount } from "svelte";
  import { currentLang, currentView } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import api from "../../lib/api.js";

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

  // Search
  let searchInput = $state("");

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
          fetch("http://localhost:8080/api/dashboard/stats", { headers }),
          fetch("http://localhost:8080/api/activity?limit=8", { headers }),
          fetch("http://localhost:8080/api/favorites/list", { headers }).catch(
            () => ({ ok: false })
          ),
          fetch("http://localhost:8080/api/files", { headers }).catch(() => ({
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

  // Search state
  let searchResults = $state([]);
  let isSearching = $state(false);

  async function handleSearch(e) {
    e.preventDefault();
    if (!searchInput.trim()) return;

    isSearching = true;
    try {
      const token = localStorage.getItem("authToken");
      const res = await fetch(
        `http://localhost:8080/api/search?q=${encodeURIComponent(searchInput.trim())}`,
        {
          headers: { Authorization: `Bearer ${token}` },
        }
      );
      if (res.ok) {
        const data = await res.json();
        searchResults = data.results || data || [];
      }
    } catch (err) {
      console.error("Search failed:", err);
    } finally {
      isSearching = false;
    }
  }

  function clearSearch() {
    searchInput = "";
    searchResults = [];
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

        const res = await fetch("http://localhost:8080/api/upload", {
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
  <div class="dashboard">
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
      </div>
    {:else}
      <!-- Top Row: Search + Quick Stats -->
      <div class="top-row">
        <!-- Search Card -->
        <div class="card search-card">
          <form onsubmit={handleSearch}>
            <div class="search-input-wrapper">
              <i class="bi bi-search"></i>
              <input
                type="text"
                bind:value={searchInput}
                placeholder="Dateien suchen..."
                class="search-input"
              />
              {#if searchInput}
                <button type="button" class="clear-btn" onclick={clearSearch}>
                  <i class="bi bi-x"></i>
                </button>
              {/if}
              <button type="submit" class="search-btn" disabled={isSearching}>
                {#if isSearching}
                  <i class="bi bi-hourglass-split"></i>
                {:else}
                  Suchen
                {/if}
              </button>
            </div>
          </form>

          {#if searchResults.length > 0}
            <div class="search-results">
              <div class="search-results-header">
                <span
                  >{searchResults.length} Ergebnis{searchResults.length !== 1
                    ? "se"
                    : ""}</span
                >
                <button class="link-btn" onclick={clearSearch}>Schließen</button
                >
              </div>
              <div class="search-results-list">
                {#each searchResults.slice(0, 5) as result}
                  <button
                    class="search-result-item"
                    onclick={() => {
                      selectedFile = result;
                      showPreview = true;
                    }}
                  >
                    <i
                      class="bi {result.is_directory
                        ? 'bi-folder-fill'
                        : 'bi-file-earmark'}"
                    ></i>
                    <span class="result-name"
                      >{result.name ||
                        result.filename ||
                        extractFileName(result.path)}</span
                    >
                  </button>
                {/each}
                {#if searchResults.length > 5}
                  <button
                    class="link-btn show-more"
                    onclick={() => navigateTo("search")}
                  >
                    Alle {searchResults.length} Ergebnisse anzeigen →
                  </button>
                {/if}
              </div>
            </div>
          {/if}
        </div>

        <!-- Quick Stats -->
        <div class="stats-row">
          <div class="stat-item">
            <div class="stat-icon files">
              <i class="bi bi-file-earmark"></i>
            </div>
            <div class="stat-info">
              <span class="stat-value">{fileCount}</span>
              <span class="stat-label">Dateien</span>
            </div>
          </div>
          <div class="stat-item">
            <div class="stat-icon folders">
              <i class="bi bi-folder"></i>
            </div>
            <div class="stat-info">
              <span class="stat-value">{folderCount}</span>
              <span class="stat-label">Ordner</span>
            </div>
          </div>
          <div class="stat-item">
            <div class="stat-icon storage">
              <i class="bi bi-hdd"></i>
            </div>
            <div class="stat-info">
              <span class="stat-value">{storageUsed.toFixed(1)} GB</span>
              <span class="stat-label">Speicher</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Main Grid -->
      <div class="main-grid">
        <!-- Left Column -->
        <div class="left-column">
          <!-- Upload Zone -->
          <div
            class="card upload-card"
            class:dragging={isDragging}
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
              style="display: none;"
            />
            <div class="upload-content">
              <div class="upload-icon">
                <i class="bi bi-cloud-arrow-up"></i>
              </div>
              <h3>Dateien hochladen</h3>
              <p>Ziehe Dateien hierher oder</p>
              <button class="upload-btn" onclick={triggerUpload}>
                <i class="bi bi-folder2-open"></i>
                Dateien auswählen
              </button>

              {#if uploadProgress.length > 0}
                <div class="upload-progress-list">
                  {#each uploadProgress as up}
                    <div
                      class="upload-progress-item"
                      class:complete={up.status === "complete"}
                      class:error={up.status === "error"}
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

          <!-- Quick Actions -->
          <div class="card">
            <h3 class="card-title">
              <i class="bi bi-lightning"></i>
              Schnellzugriff
            </h3>
            <div class="quick-actions">
              <button class="quick-action" onclick={() => navigateTo("files")}>
                <i class="bi bi-folder2-open"></i>
                <span>Dateien</span>
              </button>
              <button class="quick-action" onclick={() => navigateTo("shared")}>
                <i class="bi bi-share"></i>
                <span>Freigaben</span>
              </button>
              <button
                class="quick-action"
                onclick={() => navigateTo("favorites")}
              >
                <i class="bi bi-star"></i>
                <span>Favoriten</span>
              </button>
              <button class="quick-action" onclick={() => navigateTo("trash")}>
                <i class="bi bi-trash"></i>
                <span>Papierkorb</span>
              </button>
            </div>
          </div>

          <!-- Storage -->
          <div class="card">
            <h3 class="card-title">
              <i class="bi bi-hdd-stack"></i>
              Speicherplatz
            </h3>
            <div class="storage-bar-container">
              <div class="storage-bar" style="width: {storagePercent}%"></div>
            </div>
            <div class="storage-info">
              <span>{storageUsed.toFixed(2)} GB belegt</span>
              <span>{(storageTotal - storageUsed).toFixed(2)} GB frei</span>
            </div>
          </div>
        </div>

        <!-- Right Column -->
        <div class="right-column">
          <!-- Favorites -->
          <div class="card">
            <div class="card-header">
              <h3 class="card-title">
                <i class="bi bi-star-fill" style="color: #eab308;"></i>
                Favoriten
              </h3>
              <button class="link-btn" onclick={() => navigateTo("favorites")}>
                Alle anzeigen →
              </button>
            </div>
            {#if favorites.length === 0}
              <div class="empty-state small">
                <i class="bi bi-star"></i>
                <p>Keine Favoriten</p>
              </div>
            {:else}
              <div class="favorites-list">
                {#each favorites as fav (fav.id)}
                  <button
                    class="favorite-item"
                    onclick={() => openFavorite(fav)}
                  >
                    <i
                      class="bi {fav.type === 'folder'
                        ? 'bi-folder-fill'
                        : 'bi-file-earmark'}"
                      style="color: {fav.type === 'folder'
                        ? '#3b82f6'
                        : '#6b7280'}"
                    ></i>
                    <span class="fav-name">{fav.name}</span>
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Recent Activity -->
          <div class="card">
            <div class="card-header">
              <h3 class="card-title">
                <i class="bi bi-clock-history"></i>
                Letzte Aktivitäten
              </h3>
              <button class="link-btn" onclick={() => navigateTo("activity")}>
                Alle anzeigen →
              </button>
            </div>
            {#if recentActivity.length === 0}
              <div class="empty-state small">
                <i class="bi bi-clock"></i>
                <p>Keine Aktivitäten</p>
              </div>
            {:else}
              <div class="activity-list">
                {#each recentActivity as activity (activity.id)}
                  <button
                    class="activity-item"
                    onclick={() => openActivityItem(activity)}
                  >
                    <div class="activity-icon {activity.color}">
                      <i class="bi {activity.icon}"></i>
                    </div>
                    <div class="activity-info">
                      <span class="activity-name">{activity.fileName}</span>
                      <span class="activity-time">{activity.time}</span>
                    </div>
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="dashboard-footer">
        <span>SyncSpace v1.0</span>
      </div>
    {/if}
  </div>

  <!-- File Preview Modal -->
  {#if showPreview && selectedFile}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="preview-overlay"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={closePreview}
      onkeydown={(e) => e.key === "Escape" && closePreview()}
    >
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="preview-modal"
        role="document"
        onclick={(e) => e.stopPropagation()}
        onkeydown={() => {}}
      >
        <div class="preview-header">
          <h3>
            {selectedFile.name ||
              extractFileName(selectedFile.path || selectedFile.file_path)}
          </h3>
          <button
            class="close-btn"
            onclick={closePreview}
            aria-label="Schließen"
          >
            <i class="bi bi-x-lg"></i>
          </button>
        </div>
        <div class="preview-actions">
          <button
            class="action-btn"
            onclick={() => {
              navigateTo("files");
              closePreview();
            }}
          >
            <i class="bi bi-folder2-open"></i>
            Im Ordner öffnen
          </button>
          <a
            href="http://localhost:8080/api/download/{encodeURIComponent(
              selectedFile.path || selectedFile.file_path || ''
            )}"
            class="action-btn primary"
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
  .dashboard {
    width: 100%;
    padding: 1.5rem;
    min-height: calc(100vh - 80px);
  }

  .loading {
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

  /* Top Row */
  .top-row {
    display: flex;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .search-card {
    flex: 1;
    padding: 1rem;
  }

  .search-input-wrapper {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: #f3f4f6;
    border-radius: 0.5rem;
    padding: 0.5rem 1rem;
  }

  :global(.dark) .search-input-wrapper {
    background: #374151;
  }

  .search-input-wrapper i {
    color: #9ca3af;
  }

  .search-input {
    flex: 1;
    border: none;
    background: transparent;
    font-size: 0.95rem;
    color: #111827;
    outline: none;
  }

  :global(.dark) .search-input {
    color: #f9fafb;
  }

  .search-input::placeholder {
    color: #9ca3af;
  }

  .search-btn {
    background: #22c55e;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .search-btn:hover {
    background: #16a34a;
  }

  .stats-row {
    display: flex;
    gap: 1rem;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1rem 1.25rem;
  }

  :global(.dark) .stat-item {
    background: #1f2937;
    border-color: #374151;
  }

  .stat-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
  }

  .stat-icon.files {
    background: #dbeafe;
    color: #3b82f6;
  }

  .stat-icon.folders {
    background: #fef3c7;
    color: #f59e0b;
  }

  .stat-icon.storage {
    background: #dcfce7;
    color: #22c55e;
  }

  :global(.dark) .stat-icon.files {
    background: rgba(59, 130, 246, 0.2);
  }
  :global(.dark) .stat-icon.folders {
    background: rgba(245, 158, 11, 0.2);
  }
  :global(.dark) .stat-icon.storage {
    background: rgba(34, 197, 94, 0.2);
  }

  .stat-info {
    display: flex;
    flex-direction: column;
  }

  .stat-value {
    font-size: 1.25rem;
    font-weight: 700;
    color: #111827;
  }

  :global(.dark) .stat-value {
    color: #f9fafb;
  }

  .stat-label {
    font-size: 0.75rem;
    color: #6b7280;
  }

  /* Main Grid */
  .main-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
  }

  @media (max-width: 1024px) {
    .main-grid {
      grid-template-columns: 1fr;
    }
    .top-row {
      flex-direction: column;
    }
    .stats-row {
      flex-wrap: wrap;
    }
  }

  .left-column,
  .right-column {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* Cards */
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
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .card-title {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0 0 1rem 0;
  }

  .card-header .card-title {
    margin: 0;
  }

  :global(.dark) .card-title {
    color: #f9fafb;
  }

  .link-btn {
    background: none;
    border: none;
    color: #22c55e;
    font-size: 0.875rem;
    cursor: pointer;
  }

  .link-btn:hover {
    text-decoration: underline;
  }

  /* Upload Card */
  .upload-card {
    border: 2px dashed #d1d5db;
    text-align: center;
    transition: all 0.2s;
  }

  .upload-card.dragging {
    border-color: #22c55e;
    background: rgba(34, 197, 94, 0.05);
  }

  :global(.dark) .upload-card {
    border-color: #4b5563;
  }

  :global(.dark) .upload-card.dragging {
    background: rgba(34, 197, 94, 0.1);
  }

  .upload-content {
    padding: 1.5rem 0;
  }

  .upload-icon {
    font-size: 3rem;
    color: #22c55e;
    margin-bottom: 0.75rem;
  }

  .upload-content h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }

  :global(.dark) .upload-content h3 {
    color: #f9fafb;
  }

  .upload-content p {
    color: #6b7280;
    margin: 0 0 1rem 0;
  }

  .upload-btn {
    background: #22c55e;
    color: white;
    border: none;
    padding: 0.625rem 1.25rem;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    transition: background 0.2s;
  }

  .upload-btn:hover {
    background: #16a34a;
  }

  /* Quick Actions */
  .quick-actions {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.75rem;
  }

  .quick-action {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  :global(.dark) .quick-action {
    background: #111827;
    border-color: #374151;
  }

  .quick-action:hover {
    border-color: #22c55e;
    transform: translateY(-2px);
  }

  .quick-action i {
    font-size: 1.5rem;
    color: #22c55e;
  }

  .quick-action span {
    font-size: 0.75rem;
    color: #6b7280;
  }

  /* Storage */
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

  /* Favorites */
  .favorites-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .favorite-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    background: #f9fafb;
    border: 1px solid transparent;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
  }

  :global(.dark) .favorite-item {
    background: #111827;
  }

  .favorite-item:hover {
    border-color: #22c55e;
  }

  .favorite-item i {
    font-size: 1.125rem;
  }

  .fav-name {
    font-size: 0.875rem;
    color: #374151;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark) .fav-name {
    color: #d1d5db;
  }

  /* Activity */
  .activity-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .activity-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
  }

  .activity-icon {
    width: 32px;
    height: 32px;
    border-radius: 0.375rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.875rem;
  }

  .activity-icon.emerald {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }
  .activity-icon.blue {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }
  .activity-icon.red {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }
  .activity-icon.amber {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
  }
  .activity-icon.purple {
    background: rgba(168, 85, 247, 0.15);
    color: #a855f7;
  }
  .activity-icon.indigo {
    background: rgba(99, 102, 241, 0.15);
    color: #6366f1;
  }
  .activity-icon.violet {
    background: rgba(139, 92, 246, 0.15);
    color: #8b5cf6;
  }
  .activity-icon.yellow {
    background: rgba(234, 179, 8, 0.15);
    color: #eab308;
  }
  .activity-icon.pink {
    background: rgba(236, 72, 153, 0.15);
    color: #ec4899;
  }
  .activity-icon.gray {
    background: rgba(107, 114, 128, 0.15);
    color: #6b7280;
  }

  .activity-info {
    flex: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
    min-width: 0;
  }

  .activity-name {
    font-size: 0.875rem;
    color: #374151;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark) .activity-name {
    color: #d1d5db;
  }

  .activity-time {
    font-size: 0.75rem;
    color: #9ca3af;
    flex-shrink: 0;
    margin-left: 0.5rem;
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    color: #9ca3af;
  }

  .empty-state.small {
    padding: 1.5rem;
  }

  .empty-state i {
    font-size: 2rem;
    margin-bottom: 0.5rem;
    opacity: 0.5;
  }

  .empty-state.small i {
    font-size: 1.5rem;
  }

  .empty-state p {
    font-size: 0.875rem;
    margin: 0;
  }

  /* Footer */
  .dashboard-footer {
    text-align: center;
    padding: 2rem 0 1rem;
    font-size: 0.75rem;
    color: #9ca3af;
  }

  /* Search Results */
  .search-results {
    margin-top: 1rem;
    border-top: 1px solid #e5e7eb;
    padding-top: 1rem;
  }

  :global(.dark) .search-results {
    border-color: #374151;
  }

  .search-results-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
    font-size: 0.875rem;
    color: #6b7280;
  }

  .search-results-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .search-result-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0.75rem;
    background: #f9fafb;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
    text-align: left;
    width: 100%;
    transition: background 0.15s;
  }

  :global(.dark) .search-result-item {
    background: #111827;
  }

  .search-result-item:hover {
    background: #e5e7eb;
  }

  :global(.dark) .search-result-item:hover {
    background: #374151;
  }

  .search-result-item i {
    color: #6b7280;
  }

  .result-name {
    font-size: 0.875rem;
    color: #374151;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark) .result-name {
    color: #d1d5db;
  }

  .show-more {
    margin-top: 0.5rem;
    text-align: center;
    width: 100%;
  }

  .clear-btn {
    background: none;
    border: none;
    color: #9ca3af;
    cursor: pointer;
    padding: 0.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .clear-btn:hover {
    color: #6b7280;
  }

  /* Upload Progress */
  .upload-progress-list {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px dashed #d1d5db;
  }

  :global(.dark) .upload-progress-list {
    border-color: #4b5563;
  }

  .upload-progress-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    padding: 0.25rem 0;
    color: #6b7280;
  }

  .upload-progress-item.complete {
    color: #22c55e;
  }

  .upload-progress-item.error {
    color: #ef4444;
  }

  /* Activity button style */
  button.activity-item {
    width: 100%;
    background: none;
    border: none;
    cursor: pointer;
    border-radius: 0.375rem;
    transition: background 0.15s;
  }

  button.activity-item:hover {
    background: #f3f4f6;
  }

  :global(.dark) button.activity-item:hover {
    background: #374151;
  }

  /* Preview Modal */
  .preview-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .preview-modal {
    background: white;
    border-radius: 0.75rem;
    width: 90%;
    max-width: 400px;
    padding: 1.5rem;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
  }

  :global(.dark) .preview-modal {
    background: #1f2937;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1.5rem;
  }

  .preview-header h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
    word-break: break-word;
  }

  :global(.dark) .preview-header h3 {
    color: #f9fafb;
  }

  .close-btn {
    background: none;
    border: none;
    color: #6b7280;
    cursor: pointer;
    padding: 0.25rem;
    margin: -0.25rem -0.25rem 0 0;
  }

  .close-btn:hover {
    color: #111827;
  }

  :global(.dark) .close-btn:hover {
    color: #f9fafb;
  }

  .preview-actions {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    text-decoration: none;
    transition: all 0.15s;
    background: #f3f4f6;
    border: 1px solid #e5e7eb;
    color: #374151;
  }

  :global(.dark) .action-btn {
    background: #374151;
    border-color: #4b5563;
    color: #d1d5db;
  }

  .action-btn:hover {
    background: #e5e7eb;
  }

  :global(.dark) .action-btn:hover {
    background: #4b5563;
  }

  .action-btn.primary {
    background: #22c55e;
    border-color: #22c55e;
    color: white;
  }

  .action-btn.primary:hover {
    background: #16a34a;
  }
</style>
