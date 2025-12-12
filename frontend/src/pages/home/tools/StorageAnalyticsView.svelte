<script>
  import { onMount } from "svelte";
  import api from "../../../lib/api.js";
  import { currentLang } from "../../../stores/ui.js";
  import t from "../../../i18n.js";

  // Standard UI Components
  import StandardGlassCard from "../../../components/ui/StandardGlassCard.svelte";
  import StandardTabs from "../../../components/ui/StandardTabs.svelte";
  import StandardButton from "../../../components/ui/StandardButton.svelte";
  import StandardModal from "../../../components/ui/StandardModal.svelte";

  // State with Svelte 5 runes
  let overview = $state(null);
  let userStats = $state([]);
  let folderStats = $state([]);
  let topFiles = $state([]);
  let growth = $state([]);
  let duplicateWaste = $state(null);
  let loading = $state(true);
  let error = $state("");
  let activeTab = $state("overview");
  let showExportModal = $state(false);
  let exportLoading = $state(false);

  // Standardized tabs with proper labels and badges
  const tabs = $derived([
    {
      id: "overview",
      icon: "pie-chart",
      label: t("storage.overview") || "Übersicht",
      badge: overview?.totalFiles ? formatNumber(overview.totalFiles) : null,
    },
    {
      id: "users",
      icon: "people",
      label: t("storage.byUser") || "Nach Benutzer",
      badge: userStats?.length || null,
    },
    {
      id: "folders",
      icon: "folder",
      label: t("storage.byFolder") || "Nach Ordner",
      badge: folderStats?.length || null,
    },
    {
      id: "top-files",
      icon: "file-earmark-fill",
      label: t("storage.topFiles") || "Größte Dateien",
      badge: topFiles?.length || null,
    },
    {
      id: "growth",
      icon: "graph-up",
      label: t("storage.growth") || "Wachstum",
    },
    {
      id: "duplicates",
      icon: "files",
      label: t("storage.duplicates") || "Duplikate",
      badge: duplicateWaste?.duplicateCount || null,
    },
  ]);

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

      // Process results safely
      if (results[0].status === "fulfilled") overview = results[0].value;
      if (results[1].status === "fulfilled") userStats = results[1].value || [];
      if (results[2].status === "fulfilled")
        folderStats = results[2].value || [];
      if (results[3].status === "fulfilled") topFiles = results[3].value || [];
      if (results[4].status === "fulfilled") growth = results[4].value || [];
      if (results[5].status === "fulfilled") duplicateWaste = results[5].value;

      // Check if all requests failed
      const allFailed = results.every((r) => r.status === "rejected");
      if (allFailed) {
        error = t("storage.loadError") || "Fehler beim Laden der Analysedaten";
      }
    } catch (err) {
      console.error("Storage Analytics Error:", err);
      error = t("storage.loadError") || "Fehler beim Laden der Analysedaten";
    } finally {
      loading = false;
    }
  }

  async function handleExport() {
    showExportModal = true;
  }

  async function exportToCSV() {
    exportLoading = true;
    try {
      // Simulate export process
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // Create CSV content
      const csvContent = generateCSVContent();

      // Download CSV
      const blob = new Blob([csvContent], { type: "text/csv" });
      const url = URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = `storage-analytics-${new Date().toISOString().split("T")[0]}.csv`;
      link.click();
      URL.revokeObjectURL(url);

      showExportModal = false;
    } catch (err) {
      error = t("storage.exportError") || "Fehler beim Export";
    } finally {
      exportLoading = false;
    }
  }

  function generateCSVContent() {
    const rows = [["Type", "Name", "Files", "Size (Bytes)", "Percentage"]];

    if (overview) {
      rows.push([
        "Overview",
        "Total",
        overview.totalFiles,
        overview.totalSizeBytes,
        "100%",
      ]);
    }

    userStats.forEach((user) => {
      const percentage = overview
        ? ((user.totalSizeBytes / overview.totalSizeBytes) * 100).toFixed(1)
        : "0";
      rows.push([
        "User",
        user.username,
        user.fileCount,
        user.totalSizeBytes,
        `${percentage}%`,
      ]);
    });

    return rows.map((row) => row.join(",")).join("\n");
  }

  function formatBytes(bytes) {
    if (!bytes) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function formatNumber(num) {
    if (!num) return "0";
    return new Intl.NumberFormat($currentLang || "de").format(num);
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    try {
      const date = new Date(dateStr);
      return date.toLocaleDateString($currentLang || "de");
    } catch {
      return "-";
    }
  }

  function handleTabChange(event) {
    activeTab = event.detail.tabId;
  }

  // Export modal actions with reactive disabled state
  const exportActions = $derived([
    {
      label: t("common.cancel") || "Abbrechen",
      variant: "default",
      onClick: () => (showExportModal = false),
      disabled: exportLoading,
    },
    {
      label: t("storage.exportCSV") || "CSV Export",
      icon: "filetype-csv",
      variant: "primary",
      onClick: exportToCSV,
      disabled: exportLoading,
    },
  ]);
</script>

<!-- Page Container with Standard Background -->
<div
  class="min-h-screen bg-gradient-to-br from-blue-50/50 to-indigo-100/50 dark:from-slate-900 dark:to-slate-800 p-4 sm:p-6 lg:p-8"
>
  <div class="max-w-7xl mx-auto">
    <!-- Page Header -->
    <div class="mb-6 sm:mb-8">
      <div class="flex items-center justify-between">
        <div>
          <h1
            class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-white mb-2"
          >
            <i
              class="bi bi-bar-chart text-blue-600 dark:text-blue-400 mr-3"
              aria-hidden="true"
            ></i>
            {t("storage.title") || "Speicher-Analyse"}
          </h1>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {t("storage.subtitle") ||
              "Detaillierte Analyse der Speichernutzung"}
          </p>
        </div>

        <!-- Action Buttons -->
        <div class="flex items-center space-x-2">
          <StandardButton
            variant="default"
            icon="arrow-clockwise"
            onclick={loadAllData}
            disabled={loading}
          >
            {t("common.refresh") || "Aktualisieren"}
          </StandardButton>

          <StandardButton
            variant="primary"
            icon="download"
            onclick={handleExport}
            disabled={loading || !overview}
          >
            {t("common.export") || "Exportieren"}
          </StandardButton>
        </div>
      </div>
    </div>

    <!-- Overview Stats Cards (only show on overview tab) -->
    {#if activeTab === "overview" && overview}
      <div class="grid-4 grid-gap mb-6">
        <StandardGlassCard
          title={t("storage.totalFiles") || "Gesamte Dateien"}
          icon="file-earmark"
          padding="p-4"
        >
          <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
            {formatNumber(overview.totalFiles)}
          </div>
          <p class="text-caption mt-1">
            {t("storage.filesInSystem") || "Dateien im System"}
          </p>
        </StandardGlassCard>

        <StandardGlassCard
          title={t("storage.totalSize") || "Gesamtgröße"}
          icon="hdd"
          padding="p-4"
        >
          <div class="text-2xl font-bold text-green-600 dark:text-green-400">
            {formatBytes(overview.totalSizeBytes)}
          </div>
          <p class="text-caption mt-1">
            {t("storage.usedSpace") || "Verwendeter Speicherplatz"}
          </p>
        </StandardGlassCard>

        <StandardGlassCard
          title={t("storage.averageFileSize") || "Durchschnittsgröße"}
          icon="file-bar-graph"
          padding="p-4"
        >
          <div class="text-2xl font-bold text-purple-600 dark:text-purple-400">
            {formatBytes(overview.averageFileSizeBytes)}
          </div>
          <p class="text-caption mt-1">{t("storage.perFile") || "pro Datei"}</p>
        </StandardGlassCard>

        <StandardGlassCard
          title={t("storage.activeUsers") || "Aktive Benutzer"}
          icon="people"
          padding="p-4"
        >
          <div class="text-2xl font-bold text-orange-600 dark:text-orange-400">
            {formatNumber(userStats?.length || 0)}
          </div>
          <p class="text-caption mt-1">
            {t("storage.withFiles") || "mit Dateien"}
          </p>
        </StandardGlassCard>
      </div>
    {/if}

    <!-- Main Analytics Content -->
    <StandardGlassCard {loading} {error}>
      <!-- Tabs -->
      <StandardTabs
        {tabs}
        {activeTab}
        variant="default"
        on:change={handleTabChange}
      />

      <!-- Tab Content -->
      {#if activeTab === "overview" && overview}
        <div class="section-spacing">
          <!-- File Type Distribution Chart Placeholder -->
          <StandardGlassCard
            title={t("storage.fileTypeDistribution") || "Dateityp-Verteilung"}
            icon="pie-chart"
            padding="p-6"
          >
            <div
              class="h-64 bg-gray-100 dark:bg-slate-700/50 rounded-lg flex items-center justify-center"
            >
              <div class="text-center">
                <i
                  class="bi bi-pie-chart text-4xl text-gray-400 mb-2"
                  aria-hidden="true"
                ></i>
                <p class="text-caption">
                  {t("storage.chartPlaceholder") || "Diagramm wird geladen..."}
                </p>
              </div>
            </div>
          </StandardGlassCard>
        </div>
      {:else if activeTab === "users"}
        <div class="section-spacing">
          {#if userStats?.length}
            <div class="space-y-3">
              {#each userStats as user, index}
                <div class="glass-card-hover p-4">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-10 h-10 bg-blue-100 dark:bg-blue-900/50 rounded-full flex items-center justify-center"
                      >
                        <i
                          class="bi bi-person text-blue-600 dark:text-blue-400"
                          aria-hidden="true"
                        ></i>
                      </div>
                      <div>
                        <h4 class="text-subheading">{user.username}</h4>
                        <p class="text-caption">
                          {formatNumber(user.fileCount)}
                          {t("storage.files") || "Dateien"}
                        </p>
                      </div>
                    </div>
                    <div class="text-right">
                      <p class="text-heading">
                        {formatBytes(user.totalSizeBytes)}
                      </p>
                      <p class="text-caption">
                        {overview
                          ? (
                              (user.totalSizeBytes / overview.totalSizeBytes) *
                              100
                            ).toFixed(1)
                          : 0}%
                      </p>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="text-center py-8">
              <i
                class="bi bi-person-x text-4xl text-gray-400 mb-2"
                aria-hidden="true"
              ></i>
              <p class="text-caption">
                {t("storage.noUserData") || "Keine Benutzerdaten verfügbar"}
              </p>
            </div>
          {/if}
        </div>
      {:else if activeTab === "folders"}
        <div class="section-spacing">
          {#if folderStats?.length}
            <div class="space-y-3">
              {#each folderStats as folder}
                <div class="glass-card-hover p-4">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-10 h-10 bg-yellow-100 dark:bg-yellow-900/50 rounded-full flex items-center justify-center"
                      >
                        <i
                          class="bi bi-folder text-yellow-600 dark:text-yellow-400"
                          aria-hidden="true"
                        ></i>
                      </div>
                      <div class="min-w-0 flex-1">
                        <h4 class="text-subheading truncate">
                          {folder.folderPath ||
                            t("storage.rootFolder") ||
                            "Hauptordner"}
                        </h4>
                        <p class="text-caption">
                          {formatNumber(folder.fileCount)}
                          {t("storage.files") || "Dateien"}
                        </p>
                      </div>
                    </div>
                    <div class="text-right flex-shrink-0">
                      <p class="text-heading">
                        {formatBytes(folder.totalSizeBytes)}
                      </p>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="text-center py-8">
              <i
                class="bi bi-folder-x text-4xl text-gray-400 mb-2"
                aria-hidden="true"
              ></i>
              <p class="text-caption">
                {t("storage.noFolderData") || "Keine Ordnerdaten verfügbar"}
              </p>
            </div>
          {/if}
        </div>
      {:else if activeTab === "top-files"}
        <div class="section-spacing">
          {#if topFiles?.length}
            <div class="space-y-3">
              {#each topFiles as file, index}
                <div class="glass-card-hover p-4">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                      <div
                        class="w-8 h-8 bg-red-100 dark:bg-red-900/50 rounded-full flex items-center justify-center text-sm font-bold text-red-600 dark:text-red-400"
                      >
                        {index + 1}
                      </div>
                      <div class="min-w-0 flex-1">
                        <h4 class="text-subheading truncate">
                          {file.filename}
                        </h4>
                        <p class="text-caption truncate">{file.filePath}</p>
                      </div>
                    </div>
                    <div class="text-right flex-shrink-0">
                      <p class="text-heading">{formatBytes(file.sizeBytes)}</p>
                      <p class="text-caption">{formatDate(file.createdAt)}</p>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="text-center py-8">
              <i
                class="bi bi-file-x text-4xl text-gray-400 mb-2"
                aria-hidden="true"
              ></i>
              <p class="text-caption">
                {t("storage.noFileData") || "Keine Dateidaten verfügbar"}
              </p>
            </div>
          {/if}
        </div>
      {:else if activeTab === "duplicates"}
        <div class="section-spacing">
          {#if duplicateWaste}
            <div class="text-center py-8">
              <div
                class="text-6xl font-bold text-red-600 dark:text-red-400 mb-4"
              >
                {formatNumber(duplicateWaste.duplicateCount || 0)}
              </div>
              <h3 class="text-subheading mb-2">
                {t("storage.duplicateFiles") || "Doppelte Dateien"}
              </h3>
              <p class="text-body mb-6">
                {t("storage.wastedSpace") || "Verschwendeter Speicherplatz"}:
                <span class="font-semibold text-red-600 dark:text-red-400">
                  {formatBytes(duplicateWaste.wastedSpaceBytes || 0)}
                </span>
              </p>

              <StandardButton
                variant="danger"
                icon="trash"
                onclick={() => {
                  /* Handle cleanup */
                }}
              >
                {t("storage.cleanupDuplicates") || "Duplikate bereinigen"}
              </StandardButton>
            </div>
          {:else}
            <div class="text-center py-8">
              <i
                class="bi bi-check-circle text-4xl text-green-400 mb-2"
                aria-hidden="true"
              ></i>
              <p class="text-caption">
                {t("storage.noDuplicates") || "Keine Duplikate gefunden"}
              </p>
            </div>
          {/if}
        </div>
      {:else if activeTab === "growth"}
        <div class="section-spacing">
          <StandardGlassCard
            title={t("storage.growthChart") || "Wachstumsdiagramm"}
            icon="graph-up"
            padding="p-6"
          >
            <div
              class="h-64 bg-gray-100 dark:bg-slate-700/50 rounded-lg flex items-center justify-center"
            >
              <div class="text-center">
                <i
                  class="bi bi-graph-up text-4xl text-gray-400 mb-2"
                  aria-hidden="true"
                ></i>
                <p class="text-caption">
                  {t("storage.growthPlaceholder") ||
                    "Wachstumsdiagramm wird geladen..."}
                </p>
              </div>
            </div>
          </StandardGlassCard>
        </div>
      {:else}
        <!-- Loading State -->
        <div class="text-center py-8">
          <div
            class="animate-spin rounded-full h-8 w-8 border-2 border-blue-600 border-t-transparent mx-auto mb-2"
          ></div>
          <p class="text-caption">
            {t("storage.loadingTabData") || "Laden der Tab-Daten..."}
          </p>
        </div>
      {/if}
    </StandardGlassCard>
  </div>
</div>

<!-- Export Modal -->
<StandardModal
  bind:show={showExportModal}
  title={t("storage.exportData") || "Daten exportieren"}
  size="md"
  actions={exportActions}
  loading={exportLoading}
>
  <div class="space-y-4">
    <p class="text-body">
      {t("storage.exportDescription") ||
        "Wählen Sie die zu exportierenden Daten aus:"}
    </p>

    <div class="space-y-3">
      <label class="flex items-center">
        <input type="checkbox" checked class="form-input w-4 h-4 mr-3" />
        <span class="text-body"
          >{t("storage.includeOverview") || "Übersicht einschließen"}</span
        >
      </label>
      <label class="flex items-center">
        <input type="checkbox" checked class="form-input w-4 h-4 mr-3" />
        <span class="text-body"
          >{t("storage.includeUserStats") ||
            "Benutzerstatistiken einschließen"}</span
        >
      </label>
      <label class="flex items-center">
        <input type="checkbox" class="form-input w-4 h-4 mr-3" />
        <span class="text-body"
          >{t("storage.includeFolderStats") ||
            "Ordnerstatistiken einschließen"}</span
        >
      </label>
    </div>
  </div>
</StandardModal>
