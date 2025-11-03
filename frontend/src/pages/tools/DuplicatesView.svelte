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

  let scanning = false;
  let scanProgress = { phase: "", current: 0, total: 0 };
  let duplicateGroups = [];
  let totalWastedSpace = 0;
  let selectedDuplicates = new Set();

  $: totalDuplicates = duplicateGroups.reduce((sum, g) => sum + g.count - 1, 0);

  async function scanCurrentFolder() {
    if (scanning) return;

    scanning = true;
    duplicateGroups = [];
    selectedDuplicates.clear();
    scanProgress = { phase: "scanning", current: 0, total: 100 };

    try {
      success("Scanning for duplicates...");

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
          `Found ${groups.length} duplicate groups (${formatBytes(totalWastedSpace)} wasted)`
        );
      } else {
        success("No duplicates found!");
      }
    } catch (e) {
      console.error("Failed to scan for duplicates:", e);
      errorToast("Failed to scan for duplicates");
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
      errorToast("No duplicates selected");
      return;
    }

    const confirmed = confirm(
      `Delete ${selectedDuplicates.size} duplicate files?`
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
      success(`Deleted ${successCount} duplicate files`);
      selectedDuplicates.clear();
      selectedDuplicates = selectedDuplicates;
      scanCurrentFolder();
    }

    if (failCount > 0) {
      errorToast(`Failed to delete ${failCount} files`);
    }
  }
</script>

<PageWrapper>
  <div class="page-fade-in">
  <PageHeader
    title="Duplicate File Finder"
    subtitle="Scan and remove duplicate files to free up storage space"
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
          Scanning...
        {:else}
          <i class="bi bi-search mr-2"></i>
          Scan Current Folder
        {/if}
      </ModernButton>

      {#if selectedDuplicates.size > 0}
        <ModernButton variant="danger" onclick={deleteDuplicates}>
          <i class="bi bi-trash mr-2"></i>
          Delete {selectedDuplicates.size} Selected
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
                Quick scanning files...
              {:else if scanProgress.phase === "full-scan"}
                Deep scanning potential duplicates...
              {:else if scanProgress.phase === "scanning"}
                Scanning files...
              {:else}
                Initializing...
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
                Duplicate Groups
              </div>
              <div
                class="text-3xl font-bold text-amber-600 dark:text-amber-500"
              >
                {duplicateGroups.length}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {totalDuplicates} duplicate files
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
                Wasted Space
              </div>
              <div class="text-3xl font-bold text-red-600 dark:text-red-500">
                {formatBytes(totalWastedSpace)}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                Can be reclaimed
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
                Selected
              </div>
              <div
                class="text-3xl font-bold text-primary-600 dark:text-primary-500"
              >
                {selectedDuplicates.size}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                Files to delete
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
            class="bi bi-check-circle text-7xl text-green-500 dark:text-green-400 mb-4"
          ></i>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            No Duplicates Found
          </h2>
          <p class="text-gray-600 dark:text-gray-400 mb-6 max-w-md">
            Click "Scan Current Folder" to search for duplicate files
          </p>
          <ModernButton variant="primary" onclick={scanCurrentFolder}>
            <i class="bi bi-search mr-2"></i>
            Start Scanning
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
                  <span class="badge-glass-warning flex items-center gap-2">
                    <i class="bi bi-files"></i>
                    {group.count} copies
                  </span>
                  <span class="badge-glass-info">
                    {formatBytes(group.size)} each
                  </span>
                  <span class="badge-glass-error">
                    {formatBytes(group.wastedSpace)} wasted
                  </span>
                </div>
                <div class="flex gap-2">
                  <ModernButton
                    variant="ghost"
                    size="sm"
                    onclick={() => selectAllInGroup(group)}
                  >
                    <i class="bi bi-check-all mr-1"></i>
                    Select All
                  </ModernButton>
                  <ModernButton
                    variant="ghost"
                    size="sm"
                    onclick={() => deselectGroup(group)}
                  >
                    <i class="bi bi-x mr-1"></i>
                    Deselect
                  </ModernButton>
                </div>
              </div>

              <!-- File List -->
              <div class="overflow-x-auto">
                <table class="w-full">
                  <thead class="border-b border-gray-200 dark:border-gray-700">
                    <tr class="text-left">
                      <th class="p-3 w-12"></th>
                      <th
                        class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                        >Filename</th
                      >
                      <th
                        class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                        >Size</th
                      >
                      <th
                        class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-300"
                        >Modified</th
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
                            onchange={() => toggleDuplicateSelection(file.name)}
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
                                  Original (keep)
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
                            <span class="badge-glass-error">Will delete</span>
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
