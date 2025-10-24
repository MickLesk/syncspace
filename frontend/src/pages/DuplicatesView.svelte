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
  <div class="card bg-base-100 border border-base-300 shadow-sm mb-6">
    <div class="card-body p-4">
      <div class="flex flex-wrap items-center justify-between gap-3">
        <button
          class="btn btn-primary gap-2"
          on:click={scanCurrentFolder}
          disabled={scanning}
        >
          {#if scanning}
            <span class="loading loading-spinner"></span>
            Scanning...
          {:else}
            <i class="bi bi-search"></i>
            Scan Current Folder
          {/if}
        </button>

        {#if selectedDuplicates.size > 0}
          <button class="btn btn-error gap-2" on:click={deleteDuplicates}>
            <i class="bi bi-trash"></i>
            Delete {selectedDuplicates.size} Selected
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Scan Progress -->
  {#if scanning}
    <div class="card bg-base-100 border border-base-300 shadow-sm mb-6">
      <div class="card-body">
        <div class="flex justify-between items-center mb-2">
          <span class="font-semibold">
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
          <span class="text-sm opacity-70">
            {scanProgress.current} / {scanProgress.total}
          </span>
        </div>
        <progress
          class="progress progress-primary"
          value={scanProgress.current}
          max={scanProgress.total}
        ></progress>
      </div>
    </div>
  {/if}

  <!-- Stats -->
  {#if !scanning && duplicateGroups.length > 0}
    <div class="stats stats-vertical lg:stats-horizontal shadow mb-6 w-full">
      <div class="stat">
        <div class="stat-figure text-warning">
          <i class="bi bi-files text-4xl"></i>
        </div>
        <div class="stat-title">Duplicate Groups</div>
        <div class="stat-value text-warning">{duplicateGroups.length}</div>
        <div class="stat-desc">{totalDuplicates} duplicate files</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-error">
          <i class="bi bi-hdd text-4xl"></i>
        </div>
        <div class="stat-title">Wasted Space</div>
        <div class="stat-value text-error">{formatBytes(totalWastedSpace)}</div>
        <div class="stat-desc">Can be reclaimed</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-info">
          <i class="bi bi-check-circle text-4xl"></i>
        </div>
        <div class="stat-title">Selected</div>
        <div class="stat-value text-info">{selectedDuplicates.size}</div>
        <div class="stat-desc">Files to delete</div>
      </div>
    </div>
  {/if}

  <!-- Empty State -->
  {#if !scanning && duplicateGroups.length === 0}
    <div class="hero min-h-[400px]">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <i class="bi bi-check-circle text-7xl text-success mb-4"></i>
          <h1 class="text-3xl font-bold">No Duplicates Found</h1>
          <p class="py-6">
            Click "Scan Current Folder" to search for duplicate files
          </p>
          <button class="btn btn-primary gap-2" on:click={scanCurrentFolder}>
            <i class="bi bi-search"></i>
            Start Scanning
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Duplicate Groups -->
  {#if !scanning && duplicateGroups.length > 0}
    <div class="space-y-6">
      {#each duplicateGroups as group, groupIndex}
        <div class="card bg-base-100 border border-base-300 shadow-sm">
          <div class="card-body">
            <!-- Group Header -->
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center gap-3">
                <div class="badge badge-warning badge-lg gap-2">
                  <i class="bi bi-files"></i>
                  {group.count} copies
                </div>
                <div class="badge badge-ghost">
                  {formatBytes(group.size)} each
                </div>
                <div class="badge badge-error">
                  {formatBytes(group.wastedSpace)} wasted
                </div>
              </div>
              <div class="join">
                <button
                  class="btn btn-sm btn-ghost join-item"
                  on:click={() => selectAllInGroup(group)}
                >
                  <i class="bi bi-check-all"></i>
                  Select All
                </button>
                <button
                  class="btn btn-sm btn-ghost join-item"
                  on:click={() => deselectGroup(group)}
                >
                  <i class="bi bi-x"></i>
                  Deselect
                </button>
              </div>
            </div>

            <!-- File List -->
            <div class="overflow-x-auto">
              <table class="table table-zebra">
                <thead>
                  <tr>
                    <th>
                      <label>
                        <input
                          type="checkbox"
                          class="checkbox checkbox-sm"
                          disabled
                        />
                      </label>
                    </th>
                    <th>Filename</th>
                    <th>Size</th>
                    <th>Modified</th>
                    <th></th>
                  </tr>
                </thead>
                <tbody>
                  {#each group.files as file, fileIndex}
                    <tr class:bg-base-200={fileIndex === 0}>
                      <td>
                        <label>
                          <input
                            type="checkbox"
                            class="checkbox checkbox-sm"
                            checked={selectedDuplicates.has(file.name)}
                            on:change={() =>
                              toggleDuplicateSelection(file.name)}
                            disabled={fileIndex === 0}
                          />
                        </label>
                      </td>
                      <td>
                        <div class="flex items-center gap-2">
                          <i class="bi bi-file-earmark-fill text-primary"></i>
                          <div>
                            <div class="font-semibold">{file.name}</div>
                            {#if fileIndex === 0}
                              <div class="badge badge-success badge-xs">
                                Original (keep)
                              </div>
                            {/if}
                          </div>
                        </div>
                      </td>
                      <td>
                        <span class="badge badge-ghost"
                          >{formatBytes(file.size)}</span
                        >
                      </td>
                      <td>
                        <span class="text-sm opacity-70">
                          {file.modified_at
                            ? new Date(file.modified_at).toLocaleString()
                            : "—"}
                        </span>
                      </td>
                      <td>
                        {#if fileIndex > 0 && selectedDuplicates.has(file.name)}
                          <div class="badge badge-error badge-sm">
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
