<script>
  /**
   * Global Modal Container
   * Central hub for all application modals
   * Renders all modals in a single location at the root level
   * Communicates with pages via event dispatching
   */
  import { modals, modalEvents } from "../../stores/modals.js";
  import { currentPath } from "../../stores/ui.js";

  // Modal Components
  import Modal from "../ui/Modal.svelte";
  import FileUploadZone from "../files/FileUploadZone.svelte";
  import AdvancedSearchModal from "../search/AdvancedSearchModal.svelte";

  let newFolderName = $state("");
  let newFileName = $state("");

  // Auto-populate rename field with current filename
  $effect(() => {
    if ($modals.rename.visible && $modals.rename.data) {
      newFileName = $modals.rename.data.name || "";
    }
  });
</script>

<!-- Upload Modal -->
<Modal
  visible={$modals.upload.visible}
  title="Upload Files"
  onclose={() => modals.close("upload")}
>
  <FileUploadZone
    onFilesSelected={(files) => {
      modalEvents.emit("upload", files);
      modals.close("upload");
    }}
    currentPath={$currentPath}
  />
</Modal>

<!-- New Folder Modal -->
<Modal
  visible={$modals.newFolder.visible}
  title="Create New Folder"
  onclose={() => modals.close("newFolder")}
>
  <div class="space-y-4">
    <div>
      <label
        for="folder-name"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Folder Name
      </label>
      <input
        id="folder-name"
        type="text"
        bind:value={newFolderName}
        placeholder="Enter folder name"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
        onkeydown={(e) => {
          if (e.key === "Enter" && newFolderName.trim()) {
            modalEvents.emit("createFolder", newFolderName);
            newFolderName = "";
            modals.close("newFolder");
          }
        }}
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("newFolder")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-gradient-to-r from-primary-500 to-secondary-500 text-white rounded font-medium"
        onclick={() => {
          if (newFolderName.trim()) {
            modalEvents.emit("createFolder", newFolderName);
            newFolderName = "";
            modals.close("newFolder");
          }
        }}
        disabled={!newFolderName.trim()}
      >
        <i class="bi bi-folder-plus"></i>
        Create
      </button>
    </div>
  </div>
</Modal>

<!-- Rename Modal -->
<Modal
  visible={$modals.rename.visible}
  title="Rename File"
  onclose={() => modals.close("rename")}
>
  <div class="space-y-4">
    <div>
      <label
        for="new-name"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        New Name
      </label>
      <input
        id="new-name"
        type="text"
        bind:value={newFileName}
        placeholder="Enter new name"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
        onkeydown={(e) => {
          if (e.key === "Enter" && newFileName.trim()) {
            modalEvents.emit("renameFile", {
              file: $modals.rename.data,
              newName: newFileName,
            });
            newFileName = "";
            modals.close("rename");
          }
        }}
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("rename")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          if (newFileName.trim()) {
            modalEvents.emit("renameFile", {
              file: $modals.rename.data,
              newName: newFileName,
            });
            newFileName = "";
            modals.close("rename");
          }
        }}
        disabled={!newFileName.trim()}
      >
        <i class="bi bi-pencil"></i>
        Rename
      </button>
    </div>
  </div>
</Modal>

<!-- Delete Confirmation Modal -->
<Modal
  visible={$modals.delete.visible}
  title="Delete File"
  onclose={() => modals.close("delete")}
  variant="danger"
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Are you sure you want to delete <strong
        >{$modals.delete.data?.name}</strong
      >? This action cannot be undone.
    </p>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("delete")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded font-medium"
        onclick={() => {
          modalEvents.emit("deleteFile", $modals.delete.data);
          modals.close("delete");
        }}
      >
        <i class="bi bi-trash"></i>
        Delete
      </button>
    </div>
  </div>
</Modal>

<!-- Preview Modal -->
<Modal
  visible={$modals.preview.visible}
  title="File Preview"
  onclose={() => modals.close("preview")}
  size="large"
>
  {#if $modals.preview.data}
    <div class="space-y-4">
      <div class="bg-gray-100 dark:bg-gray-900 rounded-lg p-6 text-center">
        <i
          class="bi bi-file-earmark text-6xl text-gray-400 dark:text-gray-500 mb-4"
        ></i>
        <p class="font-semibold text-lg text-gray-900 dark:text-gray-100">
          {$modals.preview.data.name}
        </p>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
          {(($modals.preview.data.size_bytes || 0) / 1024).toFixed(1)} KB
        </p>
      </div>
      <div class="flex justify-center gap-2">
        <button
          type="button"
          class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
          onclick={() => {
            // Trigger download
            window.open(
              `/api/files/${$modals.preview.data.file_path}`,
              "_blank"
            );
          }}
        >
          <i class="bi bi-download"></i>
          Download
        </button>
        <button
          type="button"
          class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
          onclick={() => modals.close("preview")}
        >
          Close
        </button>
      </div>
    </div>
  {/if}
</Modal>

<!-- Advanced Search Modal -->
<AdvancedSearchModal
  bind:visible={$modals.advancedSearch.visible}
  on:search={(e) => modalEvents.emit("search", e.detail)}
/>

<!-- Move File Modal -->
<Modal
  visible={$modals.move.visible}
  title="Move File"
  onclose={() => modals.close("move")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Move <strong>{$modals.move.data?.name}</strong> to:
    </p>
    <div>
      <label
        for="move-destination"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Destination Path
      </label>
      <input
        id="move-destination"
        type="text"
        placeholder="/path/to/destination"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("move")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          console.log("Move file:", $modals.move.data);
          modals.close("move");
        }}
      >
        <i class="bi bi-arrow-right"></i>
        Move
      </button>
    </div>
  </div>
</Modal>

<!-- Copy File Modal -->
<Modal
  visible={$modals.copy.visible}
  title="Copy File"
  onclose={() => modals.close("copy")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Copy <strong>{$modals.copy.data?.name}</strong> to:
    </p>
    <div>
      <label
        for="copy-destination"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Destination Path
      </label>
      <input
        id="copy-destination"
        type="text"
        placeholder="/path/to/destination"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("copy")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          console.log("Copy file:", $modals.copy.data);
          modals.close("copy");
        }}
      >
        <i class="bi bi-files"></i>
        Copy
      </button>
    </div>
  </div>
</Modal>

<!-- Share File Modal -->
<Modal
  visible={$modals.share.visible}
  title="Share File"
  onclose={() => modals.close("share")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Share <strong>{$modals.share.data?.name}</strong>
    </p>
    <div>
      <label
        for="share-link"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Share Link (will be generated)
      </label>
      <input
        id="share-link"
        type="text"
        readonly
        value="https://syncspace.local/share/{$modals.share.data?.id ||
          'pending'}"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100"
      />
    </div>
    <div class="flex items-center gap-2">
      <input id="share-password" type="checkbox" class="rounded" />
      <label
        for="share-password"
        class="text-sm text-gray-700 dark:text-gray-300"
      >
        Password protect
      </label>
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("share")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          console.log("Create share link:", $modals.share.data);
          modals.close("share");
        }}
      >
        <i class="bi bi-share"></i>
        Create Link
      </button>
    </div>
  </div>
</Modal>

<!-- Version History Modal -->
<Modal
  visible={$modals.versionHistory.visible}
  title="Version History"
  onclose={() => modals.close("versionHistory")}
  size="large"
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      History for <strong>{$modals.versionHistory.data?.name}</strong>
    </p>
    <div class="space-y-2">
      <div class="p-3 bg-gray-100 dark:bg-gray-800 rounded-lg">
        <div class="flex justify-between items-center">
          <div>
            <p class="font-medium text-gray-900 dark:text-gray-100">
              Version 1.0 (Current)
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Modified: {new Date().toLocaleString()}
            </p>
          </div>
          <button
            type="button"
            class="px-3 py-1 text-sm bg-primary-500 text-white rounded"
          >
            <i class="bi bi-download"></i>
            Download
          </button>
        </div>
      </div>
      <div class="p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
        <p class="text-gray-500 dark:text-gray-400 text-center">
          No previous versions available
        </p>
      </div>
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("versionHistory")}
      >
        Close
      </button>
    </div>
  </div>
</Modal>
