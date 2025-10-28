<script>
  import {
    findRemoteDuplicates,
    formatBytes,
  } from "../utils/duplicateDetector";
  import { files } from "../stores/ui";
  import { success, error as errorToast } from "../stores/toast";
  import api from "../lib/api";

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
    scanProgress = { phase: "initializing", current: 0, total: 0 };

    try {
      const fileList = $files.filter((f) => !f.is_dir);

      if (fileList.length === 0) {
        errorToast("No files to scan in current folder");
        scanning = false;
        return;
      }

      success(`Scanning ${fileList.length} files for duplicates...`);

      const groups = await findRemoteDuplicates(fileList, api, (progress) => {
        scanProgress = progress;
      });

      duplicateGroups = groups;
      totalWastedSpace = groups.reduce((sum, g) => sum + g.wastedSpace, 0);

      if (groups.length > 0) {
        success(
          `Found ${groups.length} duplicate groups (${formatBytes(totalWastedSpace)} wasted)`
        );
      } else {
        success("No duplicates found!");
      }
    } catch (e) {
      errorToast("Failed to scan for duplicates");
    } finally {
      scanning = false;
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

<div class="duplicates-view">
  <!-- Controls -->
  <div
    class="bg-white dark:bg-gray-900 rounded-lg border border-gray-300 dark:border-gray-600 shadow-sm mb-6"
  >
    <div class="p-4">
      <div class="flex flex-wrap items-center justify-between gap-3">
        <button
          class="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={scanCurrentFolder}
          disabled={scanning}
        >
          {#if scanning}
            <div
              class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
            ></div>
            Scanning...
          {:else}
            <i class="bi bi-search"></i>
            Scan Current Folder
          {/if}
        </button>

        {#if selectedDuplicates.size > 0}
          <button
            class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors flex items-center gap-2"
            onclick={deleteDuplicates}
          >
            <i class="bi bi-trash"></i>
            Delete {selectedDuplicates.size} Selected
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Scan Progress -->
  {#if scanning}
    <div
      class="bg-white dark:bg-gray-900 rounded-lg border border-gray-300 dark:border-gray-600 shadow-sm mb-6 p-4"
    >
      <div class="flex justify-between items-center mb-2">
        <span class="font-semibold text-gray-900 dark:text-white">
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
      <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
        <div
          class="bg-blue-600 dark:bg-blue-500 h-2 rounded-full transition-all"
          style="width: {(scanProgress.current / scanProgress.total) * 100}%"
        ></div>
      </div>
    </div>
  {/if}

  <!-- Stats -->
  {#if !scanning && duplicateGroups.length > 0}
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4 shadow-md mb-6 w-full">
      <div
        class="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center justify-between">
          <div>
            <div
              class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1"
            >
              Duplicate Groups
            </div>
            <div class="text-3xl font-bold text-amber-600 dark:text-amber-500">
              {duplicateGroups.length}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
              {totalDuplicates} duplicate files
            </div>
          </div>
          <i
            class="bi bi-files text-4xl text-amber-600 dark:text-amber-500 opacity-50"
          ></i>
        </div>
      </div>

      <div
        class="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center justify-between">
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
        </div>
      </div>

      <div
        class="bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center justify-between">
          <div>
            <div
              class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1"
            >
              Selected
            </div>
            <div class="text-3xl font-bold text-blue-600 dark:text-blue-500">
              {selectedDuplicates.size}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
              Files to delete
            </div>
          </div>
          <i
            class="bi bi-check-circle text-4xl text-blue-600 dark:text-blue-500 opacity-50"
          ></i>
        </div>
      </div>
    </div>
  {/if}

  <!-- Empty State -->
  {#if !scanning && duplicateGroups.length === 0}
    <div class="flex items-center justify-center min-h-[400px]">
      <div class="text-center max-w-md">
        <i
          class="bi bi-check-circle text-7xl text-green-500 dark:text-green-400 mb-4"
        ></i>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">
          No Duplicates Found
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Click "Scan Current Folder" to search for duplicate files
        </p>
        <button
          class="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2 mx-auto"
          onclick={scanCurrentFolder}
        >
          <i class="bi bi-search"></i>
          Start Scanning
        </button>
      </div>
    </div>
  {/if}

  <!-- Duplicate Groups -->
  {#if !scanning && duplicateGroups.length > 0}
    <div class="space-y-6">
      {#each duplicateGroups as group, groupIndex}
        <div
          class="bg-white dark:bg-gray-900 rounded-lg border border-gray-300 dark:border-gray-600 shadow-sm"
        >
          <div class="p-4">
            <!-- Group Header -->
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center gap-3">
                <div
                  class="px-3 py-1.5 bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-200 rounded-lg flex items-center gap-2"
                >
                  <i class="bi bi-files"></i>
                  {group.count} copies
                </div>
                <div
                  class="px-2 py-0.5 text-xs bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-full"
                >
                  {formatBytes(group.size)} each
                </div>
                <div
                  class="px-2 py-0.5 text-xs bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded-full"
                >
                  {formatBytes(group.wastedSpace)} wasted
                </div>
              </div>
              <div
                class="inline-flex rounded-lg border border-gray-300 dark:border-gray-600"
              >
                <button
                  class="px-3 py-1.5 text-sm rounded-l-lg border-r border-gray-300 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 transition-colors flex items-center gap-2"
                  onclick={() => selectAllInGroup(group)}
                >
                  <i class="bi bi-check-all"></i>
                  Select All
                </button>
                <button
                  class="px-3 py-1.5 text-sm rounded-r-lg hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 transition-colors flex items-center gap-2"
                  onclick={() => deselectGroup(group)}
                >
                  <i class="bi bi-x"></i>
                  Deselect
                </button>
              </div>
            </div>

            <!-- File List -->
            <div class="overflow-x-auto">
              <table class="w-full">
                <thead class="border-b border-gray-200 dark:border-gray-700">
                  <tr class="text-left">
                    <th class="p-3">
                      <label>
                        <input
                          type="checkbox"
                          class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 opacity-50 cursor-not-allowed"
                          disabled
                        />
                      </label>
                    </th>
                    <th
                      class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                      >Filename</th
                    >
                    <th
                      class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                      >Size</th
                    >
                    <th
                      class="p-3 text-sm font-semibold text-gray-700 dark:text-gray-200"
                      >Modified</th
                    >
                    <th class="p-3"></th>
                  </tr>
                </thead>
                <tbody>
                  {#each group.files as file, fileIndex}
                    <tr
                      class="{fileIndex === 0
                        ? 'bg-gray-50 dark:bg-gray-800'
                        : ''} {fileIndex % 2 === 0 && fileIndex > 0
                        ? 'bg-white dark:bg-gray-900'
                        : 'bg-gray-50/50 dark:bg-gray-800/50'} border-b border-gray-100 dark:border-gray-800 last:border-0"
                    >
                      <td class="p-3">
                        <label>
                          <input
                            type="checkbox"
                            class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                            checked={selectedDuplicates.has(file.name)}
                            onchange={() =>
                              toggleDuplicateSelection(file.name)}
                            disabled={fileIndex === 0}
                          />
                        </label>
                      </td>
                      <td class="p-3">
                        <div class="flex items-center gap-2">
                          <i
                            class="bi bi-file-earmark-fill text-blue-600 dark:text-blue-400"
                          ></i>
                          <div>
                            <div
                              class="font-semibold text-gray-900 dark:text-white"
                            >
                              {file.name}
                            </div>
                            {#if fileIndex === 0}
                              <div
                                class="px-1.5 py-0.5 text-xs bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200 rounded inline-block mt-1"
                              >
                                Original (keep)
                              </div>
                            {/if}
                          </div>
                        </div>
                      </td>
                      <td class="p-3">
                        <span
                          class="px-2 py-0.5 text-xs bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-full"
                          >{formatBytes(file.size)}</span
                        >
                      </td>
                      <td class="p-3">
                        <span class="text-sm text-gray-500 dark:text-gray-400">
                          {file.modified_at
                            ? new Date(file.modified_at).toLocaleString()
                            : "—"}
                        </span>
                      </td>
                      <td class="p-3">
                        {#if fileIndex > 0 && selectedDuplicates.has(file.name)}
                          <div
                            class="px-2 py-0.5 text-xs bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded-full"
                          >
                            Will delete
                          </div>
                        {/if}
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .duplicates-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }
</style>

