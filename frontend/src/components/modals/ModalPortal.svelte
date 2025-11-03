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
<AdvancedSearchModal bind:visible={$modals.advancedSearch.visible} />
