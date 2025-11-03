<script>
  import { onMount, onDestroy } from "svelte";
  import { files, currentPath } from "../../stores/ui";
  import { success, error as errorToast } from "../../stores/toast";
  import { modals } from "../../stores/modals";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import Breadcrumbs from "../../components/Breadcrumbs.svelte";
  import SearchFilters from "../../components/search/SearchFilters.svelte";
  import UploadProgress from "../../components/files/UploadProgress.svelte";
  import Modal from "../../components/ui/Modal.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";
  import FileCard from "../../components/files/FileCard.svelte";
  import FileToolbar from "../../components/files/FileToolbar.svelte";
  import FileActionsMenu from "../../components/files/FileActionsMenu.svelte";
  import api from "../../lib/api";
  import { websocketManager } from "@stores/websocket.js";

  let loading = $state(true);
  let searchQuery = $state("");
  let viewMode = $state("grid");
  let sortBy = $state("name");
  let sortOrder = $state("asc");
  let showFoldersOnly = $state(false);
  let selectedFiles = $state(new Set());
  let uploadProgress = $state([]);
  let selectionMode = $state(false);

  // Context Menu
  let contextMenu = $state(null);
  let contextMenuPosition = $state({ x: 0, y: 0 });

  // Operation State  
  let currentFile = $state(null);

  let searchFilters = $state({
    type: "all",
    dateFrom: null,
    dateTo: null,
    minSize: null,
    maxSize: null,
  });

  // Computed: Filtered & Sorted Files
  let displayFiles = $derived(() => {
    let result = [...$files];

    // CRITICAL: Filter out system files that should NEVER be shown
    const systemFiles = [
      "syncspace.db",
      "syncspace.db-shm",
      "syncspace.db-wal",
      "search_index",
      ".git",
      ".gitignore",
      "node_modules",
      ".env",
      ".DS_Store",
      "Thumbs.db",
      "desktop.ini",
    ];

    result = result.filter((f) => {
      const fileName = f.name.toLowerCase();
      // Filter exact matches
      if (systemFiles.some((sys) => fileName === sys.toLowerCase())) {
        return false;
      }
      // Filter files starting with .
      if (fileName.startsWith(".")) {
        return false;
      }
      return true;
    });

    // Apply folder filter
    if (showFoldersOnly) {
      result = result.filter((f) => f.is_directory);
    }

    // Apply search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      result = result.filter((f) => f.name.toLowerCase().includes(query));
    }

    // Apply advanced filters
    if (searchFilters.type !== "all") {
      result = result.filter((f) => {
        if (searchFilters.type === "folder") return f.is_directory;
        if (searchFilters.type === "file") return !f.is_directory;
        return true;
      });
    }

    if (searchFilters.minSize) {
      result = result.filter((f) => f.size_bytes >= searchFilters.minSize);
    }

    if (searchFilters.maxSize) {
      result = result.filter((f) => f.size_bytes <= searchFilters.maxSize);
    }

    // Apply sorting
    result.sort((a, b) => {
      let compareValue = 0;

      switch (sortBy) {
        case "name":
          compareValue = a.name.localeCompare(b.name);
          break;
        case "modified":
          compareValue =
            new Date(a.modified_at || 0).getTime() -
            new Date(b.modified_at || 0).getTime();
          break;
        case "size":
          compareValue = (a.size_bytes || 0) - (b.size_bytes || 0);
          break;
        case "type":
          const extA = a.name.split(".").pop() || "";
          const extB = b.name.split(".").pop() || "";
          compareValue = extA.localeCompare(extB);
          break;
      }

      return sortOrder === "asc" ? compareValue : -compareValue;
    });

    return result;
  });

  let unsubscribeFileEvent;

  onMount(async () => {
    await loadFiles();
    unsubscribeFileEvent = websocketManager.on("file_change", (event) => {
      console.log("File event:", event);
      loadFiles();
    });
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    if (unsubscribeFileEvent) unsubscribeFileEvent();
    window.removeEventListener("keydown", handleKeydown);
  });

  // Modal Event Listeners - Listen to events from ModalPortal
  onMount(() => {
    const unsubUpload = modalEvents.on('upload', handleUpload);
    const unsubCreateFolder = modalEvents.on('createFolder', createNewFolder);
    const unsubRename = modalEvents.on('renameFile', ({ file, newName }) => {
      renameFile(file, newName);
    });
    const unsubDelete = modalEvents.on('deleteFile', deleteFile);

    return () => {
      unsubUpload();
      unsubCreateFolder();
      unsubRename();
      unsubDelete();
    };
  });

  async function loadFiles(path = null) {
    loading = true;
    try {
      const targetPath = path || $currentPath;
      const response = await api.files.list(targetPath);
      const fileList = Array.isArray(response) ? response : response.data || [];
      files.set(fileList);
      if (path) currentPath.set(path);
    } catch (err) {
      errorToast("Failed to load files");
      console.error(err);
    } finally {
      loading = false;
    }
  }

  function navigateTo(path) {
    loadFiles(path);
    selectedFiles = new Set();
  }

  function openFile(file) {
    if (file.is_directory) {
      navigateTo($currentPath + file.name + "/");
    } else {
      currentFile = file;
      showPreviewModal = true;
    }
  }

  async function handleUpload(filesToUpload) {
    const fileList = Array.from(filesToUpload);

    for (let file of fileList) {
      const uploadId = Date.now() + Math.random();
      uploadProgress = [
        ...uploadProgress,
        { id: uploadId, name: file.name, progress: 0, status: "uploading" },
      ];

      try {
        await api.files.uploadWithProgress($currentPath, file, (percent) => {
          uploadProgress = uploadProgress.map((up) =>
            up.id === uploadId ? { ...up, progress: percent } : up
          );
        });

        uploadProgress = uploadProgress.map((up) =>
          up.id === uploadId ? { ...up, status: "complete", progress: 100 } : up
        );

        success(`Uploaded: ${file.name}`);
      } catch (err) {
        uploadProgress = uploadProgress.map((up) =>
          up.id === uploadId ? { ...up, status: "error" } : up
        );
        errorToast(`Failed to upload: ${file.name}`);
      }
    }

    await loadFiles();
    showUploadModal = false;

    setTimeout(() => {
      uploadProgress = uploadProgress.filter((up) => up.status === "uploading");
    }, 3000);
  }

  function openUploadModal() {
    showUploadModal = true;
  }

  function openNewFolderModal() {
    showNewFolderModal = true;
  }

  async function createNewFolder() {
    if (!newFolderName.trim()) return;

    try {
      const fullPath = $currentPath
        ? `${$currentPath}${newFolderName}`
        : newFolderName;
      await api.files.createDirectory(fullPath);
      newFolderName = "";
      showNewFolderModal = false;
      success("Folder created");
      await loadFiles();
    } catch (error) {
      errorToast("Failed to create folder");
      console.error(error);
    }
  }

  async function renameFile() {
    if (!newFileName.trim() || !currentFile) return;

    try {
      const dir =
        currentFile.file_path?.split("/").slice(0, -1).join("/") || "";
      const newPath = dir ? `${dir}/${newFileName}` : newFileName;
      await api.files.rename(
        currentFile.file_path || currentFile.name,
        newPath
      );
      newFileName = "";
      showRenameModal = false;
      currentFile = null;
      success("File renamed");
      await loadFiles();
    } catch (error) {
      errorToast("Failed to rename file");
      console.error(error);
    }
  }

  async function deleteFile() {
    if (!currentFile) return;

    try {
      await api.files.delete(currentFile.file_path || currentFile.name);
      showDeleteModal = false;
      currentFile = null;
      success("File deleted");
      await loadFiles();
    } catch (error) {
      errorToast("Failed to delete file");
      console.error(error);
    }
  }

  async function batchDelete() {
    if (selectedFiles.size === 0) return;

    if (!confirm(`Delete ${selectedFiles.size} files?`)) return;

    for (const filePath of selectedFiles) {
      try {
        await api.files.delete(filePath);
      } catch (error) {
        errorToast(`Failed to delete: ${filePath}`);
        console.error(error);
      }
    }

    selectedFiles = new Set();
    success("Files deleted");
    await loadFiles();
  }

  function downloadFile(file) {
    window.open(
      `http://localhost:8080/api/download/${file.file_path || file.name}`,
      "_blank"
    );
  }

  function selectAll() {
    selectedFiles = new Set(displayFiles().map((f) => f.file_path || f.name));
  }

  function deselectAll() {
    selectedFiles = new Set();
  }

  function toggleSelectionMode() {
    selectionMode = !selectionMode;
    if (!selectionMode) {
      selectedFiles = new Set();
    }
  }

  function handleFileSelection(file) {
    if (selectionMode) {
      const newSelection = new Set(selectedFiles);
      const filePath = file.file_path || file.name;
      if (newSelection.has(filePath)) {
        newSelection.delete(filePath);
      } else {
        newSelection.add(filePath);
      }
      selectedFiles = newSelection;
    } else {
      openFile(file);
    }
  }

  function handleContextMenu(file, event) {
    currentFile = file;
    contextMenuPosition = { x: event.clientX, y: event.clientY };
    contextMenu = file;
  }

  function closeContextMenu() {
    contextMenu = null;
  }

  function handleKeydown(e) {
    if (e.ctrlKey && e.key === "a") {
      e.preventDefault();
      selectAll();
    } else if (e.key === "Delete" && selectedFiles.size > 0) {
      e.preventDefault();
      batchDelete();
    } else if (e.key === "Escape") {
      deselectAll();
      closeContextMenu();
    } else if (e.ctrlKey && e.key === "u") {
      e.preventDefault();
      showUploadModal = true;
    } else if (e.ctrlKey && e.key === "f") {
      e.preventDefault();
      document.getElementById("search-input")?.focus();
    }
  }
</script>

<!-- Global Drag & Drop Zone -->
<svelte:window
  ondragenter={(e) => {
    e.preventDefault();
    if (e.dataTransfer?.types.includes("Files")) {
      document.body.classList.add("dragging-files");
    }
  }}
  ondragleave={(e) => {
    e.preventDefault();
    if (e.target === document.body || e.relatedTarget === null) {
      document.body.classList.remove("dragging-files");
    }
  }}
  ondrop={(e) => {
    e.preventDefault();
    document.body.classList.remove("dragging-files");
    const files = Array.from(e.dataTransfer?.files || []);
    if (files.length > 0) {
      handleUpload(files);
    }
  }}
  ondragover={(e) => e.preventDefault()}
/>

<PageWrapper>
  <div class="p-4 md:p-6">
    <!-- Header with Breadcrumbs -->
    <div class="mb-4">
      <Breadcrumbs
        path={$currentPath}
        on:navigate={(e) => navigateTo(e.detail)}
      />
    </div>

    <!-- Unified Toolbar -->
    <FileToolbar
      bind:viewMode
      bind:sortBy
      bind:sortOrder
      bind:showFoldersOnly
      {selectionMode}
      selectedCount={selectedFiles.size}
      onRefresh={loadFiles}
      onUpload={openUploadModal}
      onNewFolder={openNewFolderModal}
      onAdvancedSearch={() => (showAdvancedSearchModal = true)}
      onSelectionToggle={toggleSelectionMode}
      onBatchDelete={batchDelete}
    />

    <!-- Upload Progress -->
    {#if uploadProgress.length > 0}
      <div class="mb-4">
        <UploadProgress uploads={uploadProgress} />
      </div>
    {/if}

    <!-- File Grid/List -->
    {#if loading}
      <LoadingState variant={viewMode} count={8} message="Loading files..." />
    {:else if displayFiles().length === 0}
      <EmptyState
        icon="📂"
        title={searchQuery
          ? "No files match your search"
          : "This folder is empty"}
        description={searchQuery
          ? "Try adjusting your search criteria"
          : "Upload files or create a new folder to get started"}
        actionText="Upload Files"
        onAction={() => (showUploadModal = true)}
      />
    {:else}
      <div
        class={viewMode === "grid"
          ? "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
          : "flex flex-col gap-2"}
      >
        {#each displayFiles() as file (file.file_path || file.name)}
          <FileCard
            {file}
            {viewMode}
            selected={selectedFiles.has(file.file_path || file.name)}
            onSelect={() => handleFileSelection(file)}
            onOpen={() => openFile(file)}
            onContextMenu={(f, e) => handleContextMenu(f, e)}
          />
        {/each}
      </div>
    {/if}
  </div>
</PageWrapper>

<!-- Context Menu -->
{#if contextMenu}
  <FileActionsMenu
    file={contextMenu}
    position={contextMenuPosition}
    onClose={closeContextMenu}
    onRename={() => {
      newFileName = currentFile?.name || "";
      showRenameModal = true;
    }}
    onDelete={() => (showDeleteModal = true)}
    onMove={() => (showMoveModal = true)}
    onCopy={() => (showCopyModal = true)}
    onShare={() => (showShareModal = true)}
    onDownload={() => downloadFile(currentFile)}
    onVersionHistory={() => (showVersionHistoryModal = true)}
    onPreview={() => (showPreviewModal = true)}
  />
{/if}

<!-- Upload Modal -->
<Modal 
  visible={showUploadModal}
  title="Upload Files" 
  onclose={() => (showUploadModal = false)}
>
  <FileUploadZone onFilesSelected={handleUpload} currentPath={$currentPath} />
</Modal>

<!-- New Folder Modal -->
<Modal 
  visible={showNewFolderModal}
  title="Create New Folder" 
  onclose={() => (showNewFolderModal = false)}
>
  <div class="space-y-4">
    <div>
      <label for="folder-name" class="block text-sm font-medium mb-2">
        Folder Name
      </label>
      <input
        id="folder-name"
        type="text"
        bind:value={newFolderName}
        placeholder="Enter folder name"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
        onkeydown={(e) => e.key === "Enter" && createNewFolder()}
      />
    </div>
      <div class="flex justify-end gap-2">
        <button
          type="button"
          class="btn btn-ghost"
          onclick={() => (showNewFolderModal = false)}
        >
          Cancel
        </button>
        <button
          type="button"
          class="btn btn-primary"
          onclick={createNewFolder}
          disabled={!newFolderName.trim()}
        >
          <i class="bi bi-folder-plus"></i>
          Create
        </button>
      </div>
    </div>
  </Modal>
{/if}

<!-- Rename Modal -->
<Modal 
  visible={showRenameModal}
  title="Rename File" 
  onclose={() => (showRenameModal = false)}
>
  <div class="space-y-4">
    <div>
      <label for="new-name" class="block text-sm font-medium mb-2">
        New Name
      </label>
      <input
        id="new-name"
        type="text"
        bind:value={newFileName}
        placeholder="Enter new name"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
        onkeydown={(e) => e.key === "Enter" && renameFile()}
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="btn btn-ghost"
        onclick={() => (showRenameModal = false)}
      >
        Cancel
      </button>
      <button
        type="button"
        class="btn btn-primary"
        onclick={renameFile}
        disabled={!newFileName.trim()}
      >
        <i class="bi bi-pencil"></i>
        Rename
      </button>
    </div>
  </div>
</Modal>

<!-- Delete Modal -->
<Modal 
  visible={showDeleteModal}
  title="Delete File" 
  onclose={() => (showDeleteModal = false)}
>
    <div class="space-y-4">
      <p class="text-gray-700 dark:text-gray-300">
        Are you sure you want to delete <strong>{currentFile?.name}</strong>?
        This action cannot be undone.
      </p>
      <div class="flex justify-end gap-2">
        <button
          type="button"
          class="btn btn-ghost"
          onclick={() => (showDeleteModal = false)}
        >
          Cancel
        </button>
        <button type="button" class="btn btn-error" onclick={deleteFile}>
          <i class="bi bi-trash"></i>
          Delete
        </button>
      </div>
    </div>
  </Modal>
{/if}

<!-- Preview Modal -->
<Modal
  visible={showPreviewModal && currentFile}
  title="File Preview"
  onclose={() => (showPreviewModal = false)}
  size="large"
>
    <div class="space-y-4">
      <div class="bg-gray-100 dark:bg-gray-900 rounded-lg p-6 text-center">
        <i
          class="bi bi-file-earmark text-6xl text-gray-400 dark:text-gray-500 mb-4"
        ></i>
        <p class="font-semibold text-lg text-gray-900 dark:text-gray-100">
          {currentFile.name}
        </p>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
          {((currentFile.size_bytes || 0) / 1024).toFixed(1)} KB
        </p>
      </div>
      <div class="flex justify-center gap-2">
        <button
          type="button"
          class="btn btn-primary"
          onclick={() => downloadFile(currentFile)}
        >
          <i class="bi bi-download"></i>
          Download
        </button>
        <button
          type="button"
          class="btn btn-ghost"
          onclick={() => (showPreviewModal = false)}
        >
          Close
        </button>
      </div>
    </div>
  </Modal>
{/if}

<style>
  /* Ensure smooth transitions */
  :global(.file-card-grid),
  :global(.file-card-list) {
    transition: all 0.2s ease;
  }

  /* Drag & Drop Visual Feedback */
  :global(body.dragging-files) {
    cursor: copy !important;
  }

  :global(body.dragging-files)::after {
    content: "";
    position: fixed;
    inset: 0;
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.1) 0%,
      rgba(147, 51, 234, 0.1) 100%
    );
    border: 3px dashed rgba(59, 130, 246, 0.5);
    border-radius: 1rem;
    margin: 1rem;
    pointer-events: none;
    z-index: 9999;
    animation: pulse-border 2s ease-in-out infinite;
  }

  :global(body.dragging-files) :global(.main-content) {
    position: relative;
  }

  :global(body.dragging-files) :global(.main-content)::before {
    content: "📁 Drop files here to upload";
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.95) 0%,
      rgba(147, 51, 234, 0.95) 100%
    );
    color: white;
    padding: 2rem 3rem;
    border-radius: 1rem;
    font-size: 1.5rem;
    font-weight: 600;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
    z-index: 10000;
    pointer-events: none;
    animation: bounce-in 0.3s ease-out;
  }

  @keyframes pulse-border {
    0%,
    100% {
      opacity: 1;
      border-color: rgba(59, 130, 246, 0.5);
    }
    50% {
      opacity: 0.6;
      border-color: rgba(147, 51, 234, 0.7);
    }
  }

  @keyframes bounce-in {
    0% {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.5);
    }
    50% {
      transform: translate(-50%, -50%) scale(1.05);
    }
    100% {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
  }
</style>
