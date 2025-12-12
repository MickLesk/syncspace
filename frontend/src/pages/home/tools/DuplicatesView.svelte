<script>
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import t from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { formatBytes } from "../../lib/utils.js";
  
  // Standard UI Components
  import StandardGlassCard from "../../components/ui/StandardGlassCard.svelte";
  import StandardButton from "../../components/ui/StandardButton.svelte";
  import StandardModal from "../../components/ui/StandardModal.svelte";
  import StandardTabs from "../../components/ui/StandardTabs.svelte";

  // State with Svelte 5 runes
  let scanning = $state(false);
  let scanProgress = $state({ phase: "", current: 0, total: 0 });
  let duplicateGroups = $state([]);
  let totalWastedSpace = $state(0);
  let selectedDuplicates = $state(new Set());
  let loading = $state(true);
  let error = $state(null);
  let activeTab = $state("scan");
  let showDeleteModal = $state(false);
  let deleteLoading = $state(false);

  // Computed values
  const totalDuplicates = $derived(
    duplicateGroups.reduce((sum, g) => sum + (g.count || g.file_count || 0) - 1, 0)
  );

  const totalFiles = $derived(
    duplicateGroups.reduce((sum, g) => sum + (g.count || g.file_count || 0), 0)
  );

  const selectedCount = $derived(selectedDuplicates.size);

  // Tabs configuration
  const tabs = [
    {
      id: "scan",
      icon: "search",
      label: t("duplicates.scanTab") || "Suche",
      badge: null
    },
    {
      id: "results",
      icon: "files",
      label: t("duplicates.resultsTab") || "Ergebnisse", 
      badge: () => duplicateGroups.length || null
    },
    {
      id: "cleanup",
      icon: "trash",
      label: t("duplicates.cleanupTab") || "Bereinigung",
      badge: () => selectedCount || null
    }
  ];

  async function scanForDuplicates() {
    if (scanning) return;

    scanning = true;
    error = null;
    duplicateGroups = [];
    selectedDuplicates.clear();
    scanProgress = { phase: t("duplicates.scanning") || "Scanning...", current: 0, total: 100 };

    try {
      // Simulate scanning progress
      for (let i = 0; i <= 100; i += 10) {
        scanProgress.current = i;
        await new Promise(resolve => setTimeout(resolve, 100));
      }

      // Use backend API to find duplicates
      const response = await api.duplicates.find({ min_size_bytes: 0 });
      
      // Transform backend response to match UI format
      if (response && Array.isArray(response)) {
        duplicateGroups = response.map((group) => ({
          hash: group.checksum || group.hash,
          count: group.file_count || group.count,
          wastedSpace: group.potential_savings_bytes || group.wastedSpace || 0,
          totalSize: group.total_size_bytes || group.totalSize || 0,
          files: (group.files || []).map((file) => ({
            id: file.id,
            name: file.filename || file.name,
            path: file.file_path || file.path,
            size: file.size_bytes || file.size,
            lastModified: file.updated_at || file.lastModified,
            selected: false
          }))
        }));

        totalWastedSpace = duplicateGroups.reduce((sum, g) => sum + g.wastedSpace, 0);
        
        success(t("duplicates.scanComplete") || "Duplikate-Scan abgeschlossen");
        activeTab = "results";
      }
    } catch (err) {
      console.error("Failed to scan for duplicates:", err);
      error = t("duplicates.scanError") || "Fehler beim Suchen nach Duplikaten";
      errorToast(error);
    } finally {
      scanning = false;
      loading = false;
    }
  }

  function toggleFileSelection(groupHash, fileId) {
    const group = duplicateGroups.find(g => g.hash === groupHash);
    if (group) {
      const file = group.files.find(f => f.id === fileId);
      if (file) {
        file.selected = !file.selected;
        const selectionKey = `${groupHash}:${fileId}`;
        
        if (file.selected) {
          selectedDuplicates.add(selectionKey);
        } else {
          selectedDuplicates.delete(selectionKey);
        }
        selectedDuplicates = selectedDuplicates; // Trigger reactivity
        duplicateGroups = duplicateGroups; // Trigger reactivity
      }
    }
  }

  function autoSelectDuplicates() {
    selectedDuplicates.clear();
    
    duplicateGroups.forEach(group => {
      // Keep the first file (usually newest or largest), mark others for deletion
      const sortedFiles = [...group.files].sort((a, b) => {
        // Sort by last modified (newest first) then by size (largest first)
        const dateA = new Date(a.lastModified || 0);
        const dateB = new Date(b.lastModified || 0);
        if (dateB.getTime() !== dateA.getTime()) {
          return dateB.getTime() - dateA.getTime();
        }
        return (b.size || 0) - (a.size || 0);
      });

      // Select all but the first file for deletion
      sortedFiles.slice(1).forEach(file => {
        file.selected = true;
        selectedDuplicates.add(`${group.hash}:${file.id}`);
      });
      
      // Ensure first file is not selected
      if (sortedFiles[0]) {
        sortedFiles[0].selected = false;
        selectedDuplicates.delete(`${group.hash}:${sortedFiles[0].id}`);
      }
    });
    
    selectedDuplicates = selectedDuplicates; // Trigger reactivity
    duplicateGroups = duplicateGroups; // Trigger reactivity
  }

  async function deleteSelectedDuplicates() {
    if (selectedCount === 0) return;
    showDeleteModal = true;
  }

  async function confirmDeleteDuplicates() {
    deleteLoading = true;
    
    try {
      const filesToDelete = [];
      
      // Collect all selected files
      duplicateGroups.forEach(group => {
        group.files.forEach(file => {
          if (file.selected) {
            filesToDelete.push(file.id);
          }
        });
      });

      // Delete files via API
      await Promise.all(filesToDelete.map(fileId => api.files.delete(fileId)));
      
      // Remove deleted files from groups
      duplicateGroups = duplicateGroups.map(group => ({
        ...group,
        files: group.files.filter(file => !file.selected),
        count: group.files.filter(file => !file.selected).length
      })).filter(group => group.files.length > 1); // Remove groups with only one file left

      selectedDuplicates.clear();
      showDeleteModal = false;
      
      success(t("duplicates.deleteSuccess") || "Duplikate erfolgreich gelöscht");
    } catch (err) {
      console.error("Failed to delete duplicates:", err);
      errorToast(t("duplicates.deleteError") || "Fehler beim Löschen der Duplikate");
    } finally {
      deleteLoading = false;
    }
  }

  function handleTabChange(event) {
    activeTab = event.detail.tabId;
  }

  function formatDate(dateStr) {
    if (!dateStr) return t("common.unknown") || "Unbekannt";
    try {
      return new Date(dateStr).toLocaleDateString($currentLang || 'de');
    } catch {
      return t("common.unknown") || "Unbekannt";
    }
  }

  // Modal actions
  const deleteModalActions = $derived([
    {
      label: t("common.cancel") || "Abbrechen",
      variant: "default",
      onClick: () => showDeleteModal = false,
      disabled: deleteLoading
    },
    {
      label: t("duplicates.confirmDelete") || "Löschen bestätigen",
      icon: "trash",
      variant: "danger",
      onClick: confirmDeleteDuplicates,
      disabled: deleteLoading
    }
  ]);

  // Initialize
  loading = false;
</script>

<!-- Page Container with Standard Background -->
<div class="min-h-screen bg-gradient-to-br from-blue-50/50 to-indigo-100/50 dark:from-slate-900 dark:to-slate-800 p-4 sm:p-6 lg:p-8">
  <div class="max-w-7xl mx-auto">
    
    <!-- Page Header -->
    <div class="mb-6 sm:mb-8">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-white mb-2">
            <i class="bi bi-files text-orange-600 dark:text-orange-400 mr-3" aria-hidden="true"></i>
            {t("duplicates.title") || "Duplikate-Finder"}
          </h1>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {t("duplicates.subtitle") || "Finden und entfernen Sie doppelte Dateien"}
          </p>
        </div>
        
        <!-- Action Buttons -->
        <div class="flex items-center space-x-2">
          <StandardButton
            variant="primary"
            icon="search"
            onclick={scanForDuplicates}
            disabled={scanning}
            loading={scanning}
          >
            {scanning ? t("duplicates.scanning") || "Suche läuft..." : t("duplicates.startScan") || "Scan starten"}
          </StandardButton>
        </div>
      </div>
    </div>

    <!-- Stats Cards -->
    <div class="grid-4 grid-gap mb-6">
      <StandardGlassCard 
        title={t("duplicates.totalGroups") || "Duplikat-Gruppen"} 
        icon="collection"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-orange-600 dark:text-orange-400">
          {duplicateGroups.length}
        </div>
        <p class="text-caption mt-1">{t("duplicates.groupsFound") || "Gruppen gefunden"}</p>
      </StandardGlassCard>

      <StandardGlassCard 
        title={t("duplicates.totalFiles") || "Betroffene Dateien"} 
        icon="files"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-red-600 dark:text-red-400">
          {totalFiles}
        </div>
        <p class="text-caption mt-1">{t("duplicates.filesInGroups") || "Dateien in Gruppen"}</p>
      </StandardGlassCard>

      <StandardGlassCard 
        title={t("duplicates.wastedSpace") || "Verschwendeter Platz"} 
        icon="hdd-stack"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-purple-600 dark:text-purple-400">
          {formatBytes(totalWastedSpace)}
        </div>
        <p class="text-caption mt-1">{t("duplicates.couldBeSaved") || "Könnte gespart werden"}</p>
      </StandardGlassCard>

      <StandardGlassCard 
        title={t("duplicates.selectedForDeletion") || "Zum Löschen"} 
        icon="check-circle"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-green-600 dark:text-green-400">
          {selectedCount}
        </div>
        <p class="text-caption mt-1">{t("duplicates.selectedFiles") || "Ausgewählte Dateien"}</p>
      </StandardGlassCard>
    </div>

    <!-- Main Content -->
    <StandardGlassCard 
      loading={loading} 
      error={error}
    >
      <!-- Tabs -->
      <StandardTabs 
        {tabs} 
        {activeTab} 
        variant="default"
        on:change={handleTabChange}
      />

      <!-- Tab Content -->
      {#if activeTab === "scan"}
        <!-- Scan Tab -->
        <div class="section-spacing">
          <div class="text-center py-12">
            {#if scanning}
              <div class="mb-6">
                <div class="animate-spin rounded-full h-16 w-16 border-4 border-orange-600 border-t-transparent mx-auto mb-4"></div>
                <h3 class="text-subheading mb-2">{scanProgress.phase}</h3>
                <div class="w-full bg-gray-200 dark:bg-slate-700 rounded-full h-2 max-w-xs mx-auto">
                  <div 
                    class="bg-orange-600 h-2 rounded-full transition-all duration-300"
                    style="width: {scanProgress.current}%"
                  ></div>
                </div>
                <p class="text-caption mt-2">{scanProgress.current}%</p>
              </div>
            {:else}
              <i class="bi bi-search text-6xl text-gray-400 mb-4" aria-hidden="true"></i>
              <h3 class="text-subheading mb-2">{t("duplicates.readyToScan") || "Bereit für Duplikate-Suche"}</h3>
              <p class="text-caption mb-6">
                {t("duplicates.scanDescription") || "Durchsuchen Sie Ihre Dateien nach Duplikaten basierend auf Inhalt und Größe"}
              </p>
              
              <StandardButton
                variant="primary"
                icon="search"
                onclick={scanForDuplicates}
                size="lg"
              >
                {t("duplicates.startScan") || "Scan starten"}
              </StandardButton>
            {/if}
          </div>
        </div>

      {:else if activeTab === "results"}
        <!-- Results Tab -->
        <div class="section-spacing">
          {#if duplicateGroups.length > 0}
            <div class="space-y-4">
              {#each duplicateGroups as group, groupIndex}
                <StandardGlassCard
                  title={t("duplicates.group") || "Gruppe"} 
                  subtitle={`${group.count} ${t("duplicates.files") || "Dateien"} • ${formatBytes(group.totalSize || 0)}`}
                  icon="files"
                  padding="p-4"
                >
                  <div class="space-y-2">
                    {#each group.files as file}
                      <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-slate-700/50 rounded-lg">
                        <div class="flex items-center space-x-3 flex-1 min-w-0">
                          <i class="bi bi-file-earmark text-gray-400" aria-hidden="true"></i>
                          <div class="min-w-0 flex-1">
                            <p class="text-body font-medium truncate" title={file.name}>
                              {file.name}
                            </p>
                            <p class="text-caption truncate" title={file.path}>
                              {file.path}
                            </p>
                          </div>
                        </div>
                        
                        <div class="flex items-center space-x-4 flex-shrink-0">
                          <div class="text-right">
                            <p class="text-body">{formatBytes(file.size || 0)}</p>
                            <p class="text-caption">{formatDate(file.lastModified)}</p>
                          </div>
                          
                          <label class="flex items-center">
                            <input 
                              type="checkbox" 
                              checked={file.selected || false}
                              onchange={() => toggleFileSelection(group.hash, file.id)}
                              class="form-input w-4 h-4"
                            />
                          </label>
                        </div>
                      </div>
                    {/each}
                  </div>
                </StandardGlassCard>
              {/each}
            </div>
          {:else}
            <div class="text-center py-12">
              <i class="bi bi-check-circle text-6xl text-green-400 mb-4" aria-hidden="true"></i>
              <h3 class="text-subheading mb-2">{t("duplicates.noDuplicates") || "Keine Duplikate gefunden"}</h3>
              <p class="text-caption">{t("duplicates.systemClean") || "Ihr System ist sauber!"}</p>
            </div>
          {/if}
        </div>

      {:else if activeTab === "cleanup"}
        <!-- Cleanup Tab -->
        <div class="section-spacing">
          {#if duplicateGroups.length > 0}
            <!-- Auto-select controls -->
            <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4 mb-6">
              <div class="flex items-center justify-between">
                <div>
                  <h4 class="text-subheading mb-1">{t("duplicates.smartSelection") || "Intelligente Auswahl"}</h4>
                  <p class="text-caption">{t("duplicates.autoSelectDescription") || "Automatisch die besten Kandidaten zum Löschen auswählen"}</p>
                </div>
                
                <StandardButton
                  variant="primary"
                  icon="magic"
                  onclick={autoSelectDuplicates}
                >
                  {t("duplicates.autoSelect") || "Auto-Auswahl"}
                </StandardButton>
              </div>
            </div>

            <!-- Selected files summary -->
            {#if selectedCount > 0}
              <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-6">
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-3">
                    <i class="bi bi-exclamation-triangle text-red-600 dark:text-red-400" aria-hidden="true"></i>
                    <div>
                      <h4 class="text-subheading">{selectedCount} {t("duplicates.filesSelected") || "Dateien ausgewählt"}</h4>
                      <p class="text-caption">
                        {t("duplicates.willBeDeleted") || "Diese Dateien werden gelöscht"}
                      </p>
                    </div>
                  </div>
                  
                  <StandardButton
                    variant="danger"
                    icon="trash"
                    onclick={deleteSelectedDuplicates}
                  >
                    {t("duplicates.deleteSelected") || "Ausgewählte löschen"}
                  </StandardButton>
                </div>
              </div>
            {/if}

            <!-- File groups with selection -->
            <div class="space-y-4">
              {#each duplicateGroups as group}
                <StandardGlassCard
                  title={`${t("duplicates.group") || "Gruppe"} - ${group.files.filter(f => !f.selected).length} ${t("duplicates.remaining") || "verbleibend"}`}
                  subtitle={`${group.files.filter(f => f.selected).length} ${t("duplicates.toDelete") || "zum Löschen"}`}
                  icon="files"
                  padding="p-4"
                >
                  <div class="space-y-2">
                    {#each group.files as file}
                      <div class="flex items-center justify-between p-3 rounded-lg {file.selected ? 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800' : 'bg-gray-50 dark:bg-slate-700/50'}">
                        <div class="flex items-center space-x-3 flex-1 min-w-0">
                          <label class="flex items-center">
                            <input 
                              type="checkbox" 
                              checked={file.selected || false}
                              onchange={() => toggleFileSelection(group.hash, file.id)}
                              class="form-input w-4 h-4 mr-2"
                            />
                          </label>
                          
                          <i class="bi bi-file-earmark text-gray-400" aria-hidden="true"></i>
                          <div class="min-w-0 flex-1">
                            <p class="text-body font-medium truncate {file.selected ? 'line-through opacity-60' : ''}" title={file.name}>
                              {file.name}
                            </p>
                            <p class="text-caption truncate" title={file.path}>
                              {file.path}
                            </p>
                          </div>
                        </div>
                        
                        <div class="text-right flex-shrink-0">
                          <p class="text-body {file.selected ? 'line-through opacity-60' : ''}">{formatBytes(file.size || 0)}</p>
                          <p class="text-caption">{formatDate(file.lastModified)}</p>
                        </div>
                      </div>
                    {/each}
                  </div>
                </StandardGlassCard>
              {/each}
            </div>
          {:else}
            <div class="text-center py-12">
              <i class="bi bi-broom text-6xl text-gray-400 mb-4" aria-hidden="true"></i>
              <h3 class="text-subheading mb-2">{t("duplicates.noCleanupNeeded") || "Keine Bereinigung erforderlich"}</h3>
              <p class="text-caption">{t("duplicates.runScanFirst") || "Führen Sie zuerst einen Scan durch"}</p>
            </div>
          {/if}
        </div>
      {/if}
    </StandardGlassCard>
  </div>
</div>

<!-- Delete Confirmation Modal -->
<StandardModal
  bind:show={showDeleteModal}
  title={t("duplicates.confirmDeletion") || "Löschen bestätigen"}
  size="md"
  actions={deleteModalActions}
  loading={deleteLoading}
>
  <div class="space-y-4">
    <div class="flex items-start space-x-3">
      <i class="bi bi-exclamation-triangle text-red-600 dark:text-red-400 text-xl mt-1" aria-hidden="true"></i>
      <div>
        <p class="text-body font-medium mb-2">
          {t("duplicates.deleteWarning") || "Diese Aktion kann nicht rückgängig gemacht werden!"}
        </p>
        <p class="text-caption">
          {t("duplicates.deleteDescription") || "Die ausgewählten Duplikate werden permanent gelöscht."}
        </p>
      </div>
    </div>
    
    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3">
      <p class="text-caption">
        <strong>{selectedCount}</strong> {t("duplicates.filesWillBeDeleted") || "Dateien werden gelöscht"}
      </p>
    </div>
  </div>
</StandardModal>