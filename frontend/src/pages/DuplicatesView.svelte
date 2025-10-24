<script>
  import {
    findDuplicates,
    findRemoteDuplicates,
    formatBytes,
  } from "../utils/duplicateDetector";
  import { files, currentPath } from "../stores/ui";
  import { success, error } from "../stores/toast";
  import api from "../lib/api";
  import Icon from "../components/ui/Icon.svelte";
  import Button from "../components/ui/Button.svelte";
  import Badge from "../components/ui/Badge.svelte";
  import ProgressBar from "../components/ui/ProgressBar.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";

  let scanning = false;
  let scanProgress = { phase: "", current: 0, total: 0 };
  let duplicateGroups = [];
  let totalWastedSpace = 0;
  let selectedDuplicates = new Set(); // file names to delete
  let scanType = "current"; // 'current' or 'all'

  $: displayedGroups = duplicateGroups;
  $: totalDuplicates = duplicateGroups.reduce((sum, g) => sum + g.count - 1, 0);

  async function scanCurrentFolder() {
    if (scanning) return;

    scanning = true;
    duplicateGroups = [];
    selectedDuplicates.clear();
    scanProgress = { phase: "initializing", current: 0, total: 0 };

    try {
      // Get all files in current folder (non-recursive)
      const fileList = $files.filter((f) => !f.is_dir);

      if (fileList.length === 0) {
        error("No files to scan in current folder");
        scanning = false;
        return;
      }

      success(`Scanning ${fileList.length} files for duplicates...`);

      // Use remote duplicate detection (fetches from server)
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
      console.error("Scan failed:", e);
      error("Failed to scan for duplicates");
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
    // Keep first file, select others for deletion
    group.files.slice(1).forEach((f) => selectedDuplicates.add(f.name));
    selectedDuplicates = selectedDuplicates;
  }

  function deselectGroup(group) {
    group.files.forEach((f) => selectedDuplicates.delete(f.name));
    selectedDuplicates = selectedDuplicates;
  }

  async function deleteDuplicates() {
    if (selectedDuplicates.size === 0) {
      error("No duplicates selected");
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
        await api.deleteFile(fileName);
        successCount++;
      } catch (e) {
        console.error(`Failed to delete ${fileName}:`, e);
        failCount++;
      }
    }

    if (successCount > 0) {
      success(`Deleted ${successCount} duplicate files`);
      selectedDuplicates.clear();
      selectedDuplicates = selectedDuplicates;

      // Rescan
      scanCurrentFolder();
    }

    if (failCount > 0) {
      error(`Failed to delete ${failCount} files`);
    }
  }

  function getFileExtension(fileName) {
    const parts = fileName.split(".");
    return parts.length > 1 ? parts[parts.length - 1].toUpperCase() : "FILE";
  }

  function getProgressPercent() {
    if (scanProgress.total === 0) return 0;
    return Math.round((scanProgress.current / scanProgress.total) * 100);
  }
</script>

<div class="duplicates-view">
  <!-- Crystal Glass Header -->
  <PageHeader
    title="Duplicate Files"
    subtitle="Find and remove duplicate files to save space"
    icon="copy"
    gradient="orange"
  >
    <div slot="actions" class="header-actions">
      <Button
        variant="outlined"
        size="medium"
        onClick={scanCurrentFolder}
        disabled={scanning}
      >
        {#if scanning}
          <Icon name="hourglass-split" size={16} />
          Scanning...
        {:else}
          <Icon name="search" size={16} />
          Scan Current Folder
        {/if}
      </Button>

      {#if selectedDuplicates.size > 0}
        <Button variant="danger" size="medium" onClick={deleteDuplicates}>
          <Icon name="trash" size={16} />
          Delete {selectedDuplicates.size} Selected
        </Button>
      {/if}
    </div>
  </PageHeader>

  {#if scanning}
    <div class="scan-progress">
      <div class="progress-header">
        <span class="progress-phase">
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
        <span class="progress-text"
          >{scanProgress.current} / {scanProgress.total}</span
        >
      </div>
      <ProgressBar
        value={getProgressPercent()}
        variant="primary"
        showLabel={true}
      />
    </div>
  {/if}

  {#if !scanning && duplicateGroups.length === 0}
    <div class="empty-state">
      <div class="empty-icon">🎉</div>
      <h3>No Duplicates Found</h3>
      <p>
        Your files are clean! Click "Scan Current Folder" to check for
        duplicates.
      </p>
    </div>
  {:else if !scanning && duplicateGroups.length > 0}
    <div class="summary-cards">
      <div class="summary-card">
        <div class="summary-icon">📊</div>
        <div class="summary-content">
          <div class="summary-value">{duplicateGroups.length}</div>
          <div class="summary-label">Duplicate Groups</div>
        </div>
      </div>
      <div class="summary-card">
        <div class="summary-icon">📁</div>
        <div class="summary-content">
          <div class="summary-value">{totalDuplicates}</div>
          <div class="summary-label">Duplicate Files</div>
        </div>
      </div>
      <div class="summary-card danger">
        <div class="summary-icon">💾</div>
        <div class="summary-content">
          <div class="summary-value">{formatBytes(totalWastedSpace)}</div>
          <div class="summary-label">Wasted Space</div>
        </div>
      </div>
    </div>

    <div class="duplicate-groups">
      {#each displayedGroups as group, i}
        <div class="duplicate-group">
          <div class="group-header">
            <div class="group-info">
              <h4 class="group-title">
                Group {i + 1}: {group.count} Identical Files
              </h4>
              <span class="group-meta">
                {formatBytes(group.size)} each • {formatBytes(
                  group.wastedSpace
                )} wasted
              </span>
            </div>
            <div class="group-actions">
              <Button
                variant="outlined"
                size="small"
                onclick={() => selectAllInGroup(group)}
              >
                Select Duplicates
              </Button>
              <Button
                variant="outlined"
                size="small"
                onclick={() => deselectGroup(group)}
              >
                Deselect All
              </Button>
            </div>
          </div>

          <div class="group-files">
            {#each group.files as file, fileIndex}
              <div
                class="duplicate-file"
                class:selected={selectedDuplicates.has(file.name)}
                class:original={fileIndex === 0}
              >
                <label class="file-checkbox">
                  <input
                    type="checkbox"
                    checked={selectedDuplicates.has(file.name)}
                    onchange={() => toggleDuplicateSelection(file.name)}
                    disabled={fileIndex === 0}
                  />
                </label>
                <div class="file-info">
                  <div class="file-name">
                    {file.name}
                    {#if fileIndex === 0}
                      <Badge variant="success" size="small">KEEP</Badge>
                    {:else}
                      <Badge variant="warning" size="small">DUPLICATE</Badge>
                    {/if}
                  </div>
                  <div class="file-meta">
                    <span class="file-type">{getFileExtension(file.name)}</span>
                    •
                    <span class="file-size">{formatBytes(file.size)}</span>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .duplicates-view {
    padding: 0;
    max-width: 1400px;
    margin: 0 auto;
  }

  /* Scan Progress */
  .scan-progress {
    background: var(--md-sys-color-surface-container-low);
    padding: 24px;
    border-radius: 16px;
    margin: 0 32px 24px 32px;
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .progress-phase {
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-primary);
  }

  .progress-text {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  /* Empty State */
  .empty-state {
    text-align: center;
    padding: 80px 20px;
    margin: 0 32px;
  }

  .empty-icon {
    font-size: 80px;
    margin-bottom: 24px;
  }

  .empty-state h3 {
    margin: 0 0 12px 0;
    font-size: 24px;
    color: var(--md-sys-color-on-surface);
  }

  .empty-state p {
    margin: 0;
    font-size: 14px;
    color: var(--md-sys-color-on-surface-variant);
  }

  /* Summary Cards */
  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    margin: 0 32px 24px 32px;
  }

  .summary-card {
    background: var(--md-sys-color-surface-container-low);
    padding: 24px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    gap: 16px;
    transition: all 0.2s ease;
  }

  .summary-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .summary-card.danger {
    background: linear-gradient(135deg, #ffebee 0%, #ffcdd2 100%);
  }

  .summary-icon {
    font-size: 32px;
  }

  .summary-value {
    font-size: 28px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    line-height: 1;
    margin-bottom: 4px;
  }

  .summary-label {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  /* Duplicate Groups */
  .duplicate-groups {
    display: flex;
    flex-direction: column;
    gap: 24px;
    margin: 0 32px;
  }

  .duplicate-group {
    background: var(--md-sys-color-surface-container-lowest);
    border-radius: 20px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .group-header {
    padding: 20px 24px;
    background: var(--md-sys-color-surface-container-low);
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .group-title {
    margin: 0 0 6px 0;
    font-size: 16px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .group-meta {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .group-actions {
    display: flex;
    gap: 8px;
  }

  .group-files {
    padding: 12px;
  }

  .duplicate-file {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 16px;
    border-radius: 12px;
    transition: all 0.2s ease;
  }

  .duplicate-file:hover {
    background: var(--md-sys-color-surface-container);
  }

  .duplicate-file.selected {
    background: #ffebee;
  }

  .duplicate-file.original {
    background: #e8f5e9;
    border: 2px solid #4caf50;
  }

  .file-checkbox input {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .file-checkbox input:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .file-info {
    flex: 1;
  }

  .file-name {
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 4px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .file-meta {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    display: flex;
    gap: 8px;
  }

  .file-type {
    padding: 2px 6px;
    border-radius: 4px;
    background: var(--md-sys-color-surface-container-high);
    font-weight: 500;
  }

  /* Mobile Responsive */
  @media (max-width: 768px) {
    .duplicates-view {
      padding: 16px;
    }

    .header-actions {
      width: 100%;
      flex-direction: column;
    }

    .summary-cards {
      grid-template-columns: 1fr;
    }

    .group-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 16px;
    }

    .group-actions {
      width: 100%;
      flex-direction: column;
    }
  }
</style>
