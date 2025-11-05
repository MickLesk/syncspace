<script>
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";
  import {
    findRemoteDuplicates,
    formatBytes,
  } from "../../utils/duplicateDetector";
  import { files } from "../../stores/ui";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let scanning = false;
  let scanProgress = { phase: "", current: 0, total: 0 };
  let duplicateGroups = [];
  let totalWastedSpace = 0;
  let selectedDuplicates = new Set();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  $: totalDuplicates = duplicateGroups.reduce((sum, g) => sum + g.count - 1, 0);

  async function scanCurrentFolder() {
    if (scanning) return;

    scanning = true;
    duplicateGroups = [];
    selectedDuplicates.clear();
    scanProgress = { phase: "scanning", current: 0, total: 100 };

    try {
      success(tr("duplicateFinderScanningToast"));

      // Use backend API to find duplicates
      const groups = await api.duplicates.find(0); // min_size_bytes = 0 to find all

      // Transform backend response to match UI format
      duplicateGroups = groups.map((group) => ({
        hash: group.checksum,
        count: group.file_count,
        wastedSpace: group.potential_savings_bytes,
        files: group.files.map((file) => ({
          id: file.id,
          name: file.name,
          path: file.path,
          size: file.size_bytes,
          created_at: file.created_at,
        })),
      }));

      totalWastedSpace = groups.reduce(
        (sum, g) => sum + g.potential_savings_bytes,
        0
      );

      if (groups.length > 0) {
        success(
          tr(
            "duplicateFinderScanResult",
            groups.length,
            formatBytes(totalWastedSpace)
          )
        );
      } else {
        success(tr("duplicateFinderNoDuplicatesToast"));
      }
    } catch (e) {
      console.error("Failed to scan for duplicates:", e);
      errorToast(tr("duplicateFinderScanFailed"));
    } finally {
      scanning = false;
      scanProgress = { phase: "", current: 0, total: 0 };
    }
  }

  function toggleDuplicateSelection(fileName) {
    if (selectedDuplicates.has(fileName)) {
      selectedDuplicates.delete(fileName);
    } else {
      selectedDuplicates.add(fileName);
    }
    selectedDuplicates = selectedDuplicates;
  }

  function selectAllInGroup(group) {
    group.files.slice(1).forEach((f) => selectedDuplicates.add(f.name));
    selectedDuplicates = selectedDuplicates;
  }

  function deselectGroup(group) {
    group.files.forEach((f) => selectedDuplicates.delete(f.name));
    selectedDuplicates = selectedDuplicates;
  }

  async function deleteDuplicates() {
    if (selectedDuplicates.size === 0) {
      errorToast(tr("duplicateFinderNoSelection"));
      return;
    }

    const confirmed = confirm(
      tr("duplicateFinderDeleteConfirm", selectedDuplicates.size)
    );
    if (!confirmed) return;

    let successCount = 0;
    let failCount = 0;

    for (const fileName of selectedDuplicates) {
      try {
        const cleanPath = fileName.replace(/^\/+/, "");
        await api.files.delete(cleanPath);
        successCount++;
      } catch (e) {
        failCount++;
      }
    }

    if (successCount > 0) {
      success(tr("duplicateFinderDeleteSuccess", successCount));
      selectedDuplicates.clear();
      selectedDuplicates = selectedDuplicates;
      scanCurrentFolder();
    }

    if (failCount > 0) {
      errorToast(tr("duplicateFinderDeleteFailed", failCount));
    }
  }
</script>

<PageWrapper>
  <div class="page-fade-in">
    <PageHeader
      title={tr("duplicateFinderTitle")}
      subtitle={tr("duplicateFinderSubtitle")}
      icon="bi-files"
    >
      <svelte:fragment slot="actions">
        <ModernButton
          variant="primary"
          onclick={scanCurrentFolder}
          disabled={scanning}
        >
          {#if scanning}
            <div
              class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin mr-2"
            ></div>
            {tr("duplicateFinderScanInProgress")}
          {:else}
            <i class="bi bi-search mr-2"></i>
            {tr("duplicateFinderScanCurrentFolder")}
          {/if}
        </ModernButton>

        {#if selectedDuplicates.size > 0}
          <ModernButton variant="danger" onclick={deleteDuplicates}>
            <i class="bi bi-trash mr-2"></i>
            {tr("duplicateFinderDeleteSelected", selectedDuplicates.size)}
          </ModernButton>
        {/if}
      </svelte:fragment>
    </PageHeader>

    <div class="space-y-6">
      <div class="space-y-6">
        <!-- Scan Progress -->
        {#if scanning}
          <ModernCard variant="glass">
            <div class="flex justify-between items-center mb-3">
              <span class="font-semibold text-gray-900 dark:text-gray-100">
                {#if scanProgress.phase === "quick-scan"}
                  {tr("duplicateFinderQuickScan")}
                {:else if scanProgress.phase === "full-scan"}
                  {tr("duplicateFinderDeepScan")}
                {:else if scanProgress.phase === "scanning"}
                  {tr("duplicateFinderScanningFiles")}
                {:else}
                  {tr("duplicateFinderInitializing")}
                {/if}
              </span>
              <span class="text-sm text-gray-500 dark:text-gray-400">
                {scanProgress.current} / {scanProgress.total}
              </span>
            </div>
            <div
              class="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden"
            >
              <div
                class="h-full bg-gradient-to-r from-primary-500 to-primary-600 transition-all duration-300"
                style="width: {(scanProgress.current / scanProgress.total) *
                  100}%"
              ></div>
            </div>
          </ModernCard>
        {/if}

        <!-- Stats -->
        {#if !scanning && duplicateGroups.length > 0}
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <ModernCard
              variant="glass"
              hoverable
              class="flex items-center justify-between p-6"
            >
              <div>
                <div
                  class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1"
                >
                  {tr("duplicateFinderStatsGroups")}
                </div>
                <div
                  class="text-3xl font-bold text-amber-600 dark:text-amber-500"
                >
                  {duplicateGroups.length}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  {tr("duplicateFinderStatsFiles", totalDuplicates)}
                </div>
              </div>
              <i
                class="bi bi-files text-4xl text-amber-600 dark:text-amber-500 opacity-50"
              ></i>
            </ModernCard>

            <ModernCard
              variant="glass"
              hoverable
              class="flex items-center justify-between p-6"
            >
              <div>
                <div
                  class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1"
                >
                  {tr("duplicateFinderStatsWasted")}
                </div>
                <div class="text-3xl font-bold text-red-600 dark:text-red-500">
                  {formatBytes(totalWastedSpace)}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  {tr("duplicateFinderStatsReclaim")}
                </div>
              </div>
              <i
                class="bi bi-hdd text-4xl text-red-600 dark:text-red-500 opacity-50"
              ></i>
            </ModernCard>

            <ModernCard
              variant="glass"
              hoverable
              class="flex items-center justify-between p-6"
            >
              <div>
                <div
                  class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1"
                >
                  {tr("duplicateFinderStatsSelected")}
                </div>
                <div
                  class="text-3xl font-bold text-primary-600 dark:text-primary-500"
                >
                  {selectedDuplicates.size}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  {tr("duplicateFinderStatsToDelete")}
                </div>
              </div>
              <i
                class="bi bi-check-circle text-4xl text-primary-600 dark:text-primary-500 opacity-50"
              ></i>
            </ModernCard>
          </div>
        {/if}

        <!-- Empty State -->
        {#if !scanning && duplicateGroups.length === 0}
          <ModernCard
            variant="glass"
            class="flex flex-col items-center justify-center py-16 text-center"
          >
            <i
              class="bi bi-check-circle text-7xl text-green-500 dark:text-green-400 mb-4 bounce-in"
            ></i>
            <h2
              class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2"
            >
              {tr("duplicateFinderEmptyTitle")}
            </h2>
            <p class="text-gray-600 dark:text-gray-400 mb-6 max-w-md">
              {tr(
                "duplicateFinderEmptyDescription",
                tr("duplicateFinderScanCurrentFolder")
              )}
            </p>
            <ModernButton variant="primary" onclick={scanCurrentFolder}>
              <i class="bi bi-search mr-2"></i>
              {tr("duplicateFinderStartScanning")}
            </ModernButton>
          </ModernCard>
        {/if}

        <!-- Duplicate Groups -->
        {#if !scanning && duplicateGroups.length > 0}
          <div class="space-y-4 list-stagger">
            {#each duplicateGroups as group, groupIndex}
              <ModernCard variant="glass" hoverable class="hover-lift">
                <!-- Group Header -->
                <div
                  class="flex flex-wrap items-center justify-between gap-3 mb-4"
                >
                  <div class="flex items-center gap-3">
                    <span
                      class="badge-glass-warning flex items-center gap-2 bounce-in"
                    >
                      <i class="bi bi-files"></i>
                      {tr("duplicateFinderCopiesBadge", group.count)}
                    </span>
                    <span class="badge-glass-info bounce-in">
                      {tr(
                        "duplicateFinderEachBadge",
                        formatBytes(group.files?.[0]?.size || 0)
                      )}
                    </span>
                    <span class="badge-glass-error bounce-in">
                      {tr(
                        "duplicateFinderWastedBadge",
                        formatBytes(group.wastedSpace)
                      )}
                    </span>
                  </div>
                  <div class="flex gap-2">
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      onclick={() => selectAllInGroup(group)}
                    >
                      <i class="bi bi-check-all mr-1"></i>
                      {tr("selectAll")}
                    </ModernButton>
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      onclick={() => deselectGroup(group)}
                    >
                      <i class="bi bi-x mr-1"></i>
                      {tr("deselect")}
                    </ModernButton>
                  </div>
                </div>

                <!-- File List -->
                <div class="overflow-x-auto">
                  <table class="w-full">
                    <thead
                      class="border-b border-gray-200 dark:border-gray-700"
                    >
                      <tr class="text-left">
                        <th class="p-3 w-12"></th>
                        <th
                          class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                          >{tr("name")}</th
                        >
                        <th
                          class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                          >{tr("size")}</th
                        >
                        <th
                          class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                          >{tr("modified")}</th
                        >
                        <th class="p-3 w-32"></th>
                      </tr>
                    </thead>
                    <tbody>
                      {#each group.files as file, fileIndex}
                        <tr
                          class="border-b border-gray-100 dark:border-gray-800 last:border-0 {fileIndex ===
                          0
                            ? 'bg-white/50 dark:bg-gray-800/50'
                            : ''}"
                        >
                          <td class="p-3">
                            <input
                              type="checkbox"
                              class="glass-input w-4 h-4 disabled:opacity-50 disabled:cursor-not-allowed"
                              checked={selectedDuplicates.has(file.name)}
                              onchange={() =>
                                toggleDuplicateSelection(file.name)}
                              disabled={fileIndex === 0}
                            />
                          </td>
                          <td class="p-3">
                            <div class="flex items-center gap-2">
                              <i
                                class="bi bi-file-earmark-fill text-primary-600 dark:text-primary-400"
                              ></i>
                              <div>
                                <div
                                  class="font-medium text-gray-900 dark:text-gray-100"
                                >
                                  {file.name}
                                </div>
                                {#if fileIndex === 0}
                                  <span class="badge-glass-success mt-1">
                                    {tr("duplicateFinderOriginalBadge")}
                                  </span>
                                {/if}
                              </div>
                            </div>
                          </td>
                          <td class="p-3">
                            <span class="badge-glass-info"
                              >{formatBytes(file.size)}</span
                            >
                          </td>
                          <td class="p-3">
                            <span
                              class="text-sm text-gray-500 dark:text-gray-400"
                            >
                              {file.modified_at
                                ? new Date(file.modified_at).toLocaleString()
                                : "—"}
                            </span>
                          </td>
                          <td class="p-3">
                            {#if fileIndex > 0 && selectedDuplicates.has(file.name)}
                              <span class="badge-glass-error">
                                {tr("duplicateFinderWillDeleteBadge")}
                              </span>
                            {/if}
                          </td>
                        </tr>
                      {/each}
                    </tbody>
                  </table>
                </div>
              </ModernCard>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
</PageWrapper>
