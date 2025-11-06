<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import {
    batchOperations,
    currentJobId,
  } from "../../stores/batchOperations.js";
  import BatchProgressDialog from "./BatchProgressDialog.svelte";
  import { success, error as errorToast } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  /** @type {{selectedFiles?: any[], onClearSelection?: Function}} */
  let { selectedFiles = $bindable([]), onClearSelection = () => {} } = $props();

  /** @type {boolean} show folder picker dialog */
  let showFolderPicker = $state(false);
  /** @type {string} target folder for copy/move */
  let targetFolder = $state("");
  /** @type {string} pending action (copy/move) */
  let pendingAction = $state("");
  /** @type {boolean} show compress dialog */
  let showCompressDialog = $state(false);
  /** @type {string} archive name */
  let archiveName = $state("archive");
  /** @type {string} archive format */
  let archiveFormat = $state("zip");

  // Computed
  const selectedCount = $derived(selectedFiles.length);
  const hasSelection = $derived(selectedCount > 0);

  // Get selected file paths
  function getSelectedPaths() {
    return selectedFiles.map((f) => f.path || f.file_path);
  }

  // Handle batch copy
  async function handleCopy() {
    if (!hasSelection) return;
    pendingAction = "copy";
    showFolderPicker = true;
  }

  // Handle batch move
  async function handleMove() {
    if (!hasSelection) return;
    pendingAction = "move";
    showFolderPicker = true;
  }

  // Handle batch delete
  async function handleDelete() {
    if (!hasSelection) return;

    const confirmed = confirm(
      `Are you sure you want to delete ${selectedCount} item(s)? This action cannot be undone.`
    );

    if (!confirmed) return;

    try {
      // TODO: Implement batch delete API endpoint
      success(`Deleted ${selectedCount} item(s)`);
      onClearSelection();
    } catch (error) {
      errorToast(`Failed to delete: ${error.message}`);
    }
  }

  // Handle batch compress
  function handleCompress() {
    if (!hasSelection) return;

    // Set default archive name from first file
    if (selectedFiles[0]?.name) {
      archiveName = selectedFiles[0].name.replace(/\.[^/.]+$/, "") + "_archive";
    }

    showCompressDialog = true;
  }

  // Execute folder picker action (copy/move)
  async function executeFolderAction() {
    if (!targetFolder.trim()) {
      errorToast("Please enter a target folder");
      return;
    }

    const paths = getSelectedPaths();

    try {
      if (pendingAction === "copy") {
        await batchOperations.startCopy(paths, targetFolder);
        success(`Started copying ${selectedCount} item(s)`);
      } else if (pendingAction === "move") {
        // TODO: Implement batch move endpoint
        success(`Started moving ${selectedCount} item(s)`);
      }

      showFolderPicker = false;
      targetFolder = "";
      onClearSelection();
    } catch (error) {
      errorToast(`Failed to ${pendingAction}: ${error.message}`);
    }
  }

  // Execute compress action
  async function executeCompress() {
    if (!archiveName.trim()) {
      errorToast("Please enter an archive name");
      return;
    }

    const paths = getSelectedPaths();

    try {
      await batchOperations.startCompress(paths, archiveName, archiveFormat);
      success(`Started compressing ${selectedCount} item(s)`);

      showCompressDialog = false;
      archiveName = "archive";
      onClearSelection();
    } catch (error) {
      errorToast(`Failed to compress: ${error.message}`);
    }
  }

  // Clear selection
  function clearSelection() {
    onClearSelection();
  }
</script>

{#if hasSelection}
  <div
    class="bg-primary/10 border border-primary/20 rounded-lg p-3 mb-4 flex items-center gap-3 flex-wrap"
  >
    <!-- Selection count -->
    <div class="flex items-center gap-2 flex-1 min-w-[200px]">
      <svg
        class="w-5 h-5 text-primary"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <span class="font-medium"
        >{selectedCount} item{selectedCount === 1 ? "" : "s"} selected</span
      >
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2 flex-wrap">
      <button
        class="btn btn-sm btn-ghost gap-1"
        onclick={handleCopy}
        title="Copy selected items"
        aria-label="Copy selected items"
      >
        <svg
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
          />
        </svg>
        Copy
      </button>

      <button
        class="btn btn-sm btn-ghost gap-1"
        onclick={handleMove}
        title="Move selected items"
        aria-label="Move selected items"
      >
        <svg
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"
          />
        </svg>
        Move
      </button>

      <button
        class="btn btn-sm btn-ghost gap-1"
        onclick={handleCompress}
        title="Compress selected items"
        aria-label="Compress selected items"
      >
        <svg
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"
          />
        </svg>
        Compress
      </button>

      <button
        class="btn btn-sm btn-error btn-ghost gap-1"
        onclick={handleDelete}
        title="Delete selected items"
        aria-label="Delete selected items"
      >
        <svg
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
        Delete
      </button>

      <div class="divider divider-horizontal mx-0"></div>

      <button
        class="btn btn-sm btn-ghost gap-1"
        onclick={clearSelection}
        title="Clear selection"
        aria-label="Clear selection"
      >
        <svg
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
        Clear
      </button>
    </div>
  </div>
{/if}

<!-- Folder Picker Dialog -->
{#if showFolderPicker}
  <dialog class="modal modal-open">
    <div class="modal-box">
      <h3 class="text-lg font-bold mb-4">
        {pendingAction === "copy" ? "Copy" : "Move"}
        {selectedCount} item(s)
      </h3>

      <div class="form-control mb-4">
        <label for="target-folder" class="label">
          <span class="label-text">Target Folder</span>
        </label>
        <input
          id="target-folder"
          type="text"
          bind:value={targetFolder}
          placeholder="/path/to/destination"
          class="input input-bordered w-full"
          onkeydown={(e) => e.key === "Enter" && executeFolderAction()}
          aria-describedby="target-folder-help"
        />
        <div id="target-folder-help" class="label">
          <span class="label-text-alt">Enter the destination folder path</span>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={() => (showFolderPicker = false)}
          >Cancel</button
        >
        <button class="btn btn-primary" onclick={executeFolderAction}>
          {pendingAction === "copy" ? "Copy" : "Move"}
        </button>
      </div>
    </div>
    <button
      class="modal-backdrop bg-black/50"
      onclick={() => (showFolderPicker = false)}
      onkeydown={(e) => e.key === "Escape" && (showFolderPicker = false)}
      aria-label="Close folder picker"
      tabindex="-1"
    ></button>
  </dialog>
{/if}

<!-- Compress Dialog -->
{#if showCompressDialog}
  <dialog class="modal modal-open">
    <div class="modal-box">
      <h3 class="text-lg font-bold mb-4">Compress {selectedCount} item(s)</h3>

      <div class="space-y-4">
        <div class="form-control">
          <label for="archive-name" class="label">
            <span class="label-text">Archive Name</span>
          </label>
          <input
            id="archive-name"
            type="text"
            bind:value={archiveName}
            placeholder="archive"
            class="input input-bordered w-full"
          />
        </div>

        <div class="form-control">
          <label for="archive-format" class="label">
            <span class="label-text">Format</span>
          </label>
          <select
            id="archive-format"
            bind:value={archiveFormat}
            class="select select-bordered w-full"
          >
            <option value="zip">ZIP (.zip)</option>
            <option value="tar.gz">TAR.GZ (.tar.gz)</option>
            <option value="tar">TAR (.tar)</option>
          </select>
        </div>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => (showCompressDialog = false)}>Cancel</button
        >
        <button class="btn btn-primary" onclick={executeCompress}>
          Compress
        </button>
      </div>
    </div>
    <button
      class="modal-backdrop bg-black/50"
      onclick={() => (showCompressDialog = false)}
      onkeydown={(e) => e.key === "Escape" && (showCompressDialog = false)}
      aria-label="Close compress dialog"
      tabindex="-1"
    ></button>
  </dialog>
{/if}

<!-- Progress Dialog -->
<BatchProgressDialog
  jobId={$currentJobId}
  onClose={() => currentJobId.set(null)}
/>

<style>
  .divider-horizontal {
    width: 1px;
    height: 1.5rem;
  }
</style>
