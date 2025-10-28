<script>
  import { onMount, onDestroy } from "svelte";
  import { files, currentPath } from "../stores/ui";
  import { favorites } from "../stores/favorites";
  import { success, error as errorToast } from "../stores/toast";
  import { getFileIcon, getFileIconColor } from "../utils/fileIcons";
  import ContextMenu from "../components/ui/ContextMenu.svelte";
  import Modal from "../components/ui/Modal.svelte";
  import FileThumbnail from "../components/ui/FileThumbnail.svelte";
  import AdvancedSearchModal from "../components/AdvancedSearchModal.svelte";
  import ShareModal from "../components/ui/ShareModal.svelte";
  import VersionHistoryModal from "../components/ui/VersionHistoryModal.svelte";
  import FilePreviewModal from "../components/ui/FilePreviewModal.svelte";
  import api from "../lib/api";
  import { wsConnected, onFileEvent, websocketManager } from "../stores/websocket.js";
  import Loading from "../components/Loading.svelte";

  // ==================== STATE ====================
  let loading = $state(true);
  let uploading = $state(false);
  let searchQuery = $state("");
  let viewMode = $state("grid"); // 'grid' or 'list'
  let sortBy = $state("name"); // 'name', 'modified', 'size', 'type'
  let sortOrder = $state("asc"); // 'asc' or 'desc'
  let showFoldersOnly = $state(false);
  let dragOver = $state(false);
  let searchResults = $state([]);
  let isSearchActive = $state(false);

  // Modals
  let showNewFolderModal = $state(false);
  let showRenameModal = $state(false);
  let showDeleteModal = $state(false);
  let showAdvancedSearchModal = $state(false);
  let showShareModal = $state(false);
  let showVersionHistoryModal = $state(false);
  let showFilePreview = $state(false);
  let showMoveModal = $state(false);
  let showCopyModal = $state(false);

  // Current action targets
  let fileToPreview = $state(null);
  let fileToRename = $state(null);
  let fileToDelete = $state(null);
  let fileToShare = $state(null);
  let fileToViewVersions = $state(null);
  let fileToMove = $state(null);
  let fileToCopy = $state(null);
  let moveTargetPath = $state("");
  let copyTargetPath = $state("");
  let newFolderName = $state("");
  let newFileName = $state("");

  // Context Menu
  let contextMenuVisible = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuFile = $state(null);

  // File upload with progress
  let uploadInput;
  let uploadFiles = $state([]);
  let uploadProgress = $state(new Map());
  let overallProgress = $state(0);

  // Multi-Select
  let selectedFiles = $state([]);
  let lastSelectedIndex = $state(-1);
  let selectionMode = $state(false);

  // ==================== COMPUTED ====================
  let filteredFiles = $derived(
    isSearchActive
      ? searchResults
      : searchQuery
        ? $files.filter((f) => f.name.toLowerCase().includes(searchQuery.toLowerCase()))
        : $files
  );

  let sortedFiles = $derived.by(() => {
    let result = showFoldersOnly
      ? filteredFiles.filter((f) => f.is_dir)
      : filteredFiles;

    result = [...result].sort((a, b) => {
      let comparison = 0;
      switch (sortBy) {
        case "name":
          comparison = a.name.localeCompare(b.name);
          break;
        case "modified":
          comparison = new Date(a.modified_at) - new Date(b.modified_at);
          break;
        case "size":
          comparison = (a.size_bytes || 0) - (b.size_bytes || 0);
          break;
        case "type":
          const extA = a.name.split(".").pop() || "";
          const extB = b.name.split(".").pop() || "";
          comparison = extA.localeCompare(extB);
          break;
      }
      return sortOrder === "asc" ? comparison : -comparison;
    });

    // Always put folders first
    return result.sort((a, b) => {
      if (a.is_dir && !b.is_dir) return -1;
      if (!a.is_dir && b.is_dir) return 1;
      return 0;
    });
  });

  // ==================== LIFECYCLE ====================
  onMount(async () => {
    await loadFiles();
    setupKeyboardShortcuts();
    setupWebSocket();
  });

  onDestroy(() => {
    if (keyboardHandler) {
      window.removeEventListener("keydown", keyboardHandler);
    }
  });

  // ==================== FUNCTIONS ====================
  async function loadFiles(path = null) {
    loading = true;
    try {
      const targetPath = path || $currentPath;
      const response = await api.files.list(targetPath);
      files.set(response.data || []);
      if (path) currentPath.set(path);
    } catch (err) {
      errorToast("Failed to load files");
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function handleFileUpload() {
    if (!uploadInput?.files?.length) return;

    uploading = true;
    uploadFiles = Array.from(uploadInput.files);
    uploadProgress = new Map();

    for (let i = 0; i < uploadFiles.length; i++) {
      const file = uploadFiles[i];
      try {
        const formData = new FormData();
        formData.append("file", file);

        await api.files.upload($currentPath, formData, (percent) => {
          uploadProgress.set(i, { percent, file: file.name });
          updateOverallProgress();
        });

        success(`Uploaded: ${file.name}`);
      } catch (err) {
        errorToast(`Failed to upload: ${file.name}`);
      }
    }

    uploading = false;
    uploadFiles = [];
    uploadProgress.clear();
    uploadInput.value = "";
    await loadFiles();
  }

  function updateOverallProgress() {
    if (uploadProgress.size === 0) {
      overallProgress = 0;
      return;
    }
    const total = Array.from(uploadProgress.values()).reduce(
      (sum, p) => sum + p.percent,
      0
    );
    overallProgress = Math.round(total / uploadProgress.size);
  }

  async function handleCreateFolder() {
    if (!newFolderName.trim()) return;

    try {
      await api.directories.create($currentPath, newFolderName);
      success(`Folder created: ${newFolderName}`);
      showNewFolderModal = false;
      newFolderName = "";
      await loadFiles();
    } catch (err) {
      errorToast("Failed to create folder");
    }
  }

  async function handleRename() {
    if (!fileToRename || !newFileName.trim()) return;

    try {
      await api.files.rename(
        $currentPath + fileToRename.name,
        $currentPath + newFileName
      );
      success(`Renamed to: ${newFileName}`);
      showRenameModal = false;
      fileToRename = null;
      newFileName = "";
      await loadFiles();
    } catch (err) {
      errorToast("Failed to rename");
    }
  }

  async function handleDelete() {
    if (!fileToDelete) return;

    try {
      await api.files.delete($currentPath + fileToDelete.name);
      success(`Deleted: ${fileToDelete.name}`);
      showDeleteModal = false;
      fileToDelete = null;
      await loadFiles();
    } catch (err) {
      errorToast("Failed to delete");
    }
  }

  async function handleDownload(file) {
    try {
      const response = await fetch(
        `http://localhost:8080/api/files/${$currentPath}${file.name}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (!response.ok) throw new Error("Download failed");

      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = file.name;
      a.click();
      window.URL.revokeObjectURL(url);
      success(`Downloaded: ${file.name}`);
    } catch (err) {
      errorToast("Failed to download");
    }
  }

  async function handleMove() {
    if (!fileToMove || !moveTargetPath) return;

    try {
      await api.files.move(
        $currentPath + fileToMove.name,
        moveTargetPath + fileToMove.name
      );
      success(`Moved: ${fileToMove.name}`);
      showMoveModal = false;
      fileToMove = null;
      moveTargetPath = "";
      await loadFiles();
    } catch (err) {
      errorToast("Failed to move");
    }
  }

  async function handleCopy() {
    if (!fileToCopy || !copyTargetPath) return;

    try {
      await api.files.copy(
        $currentPath + fileToCopy.name,
        copyTargetPath + fileToCopy.name
      );
      success(`Copied: ${fileToCopy.name}`);
      showCopyModal = false;
      fileToCopy = null;
      copyTargetPath = "";
      await loadFiles();
    } catch (err) {
      errorToast("Failed to copy");
    }
  }

  async function handleBatchDelete() {
    if (selectedFiles.length === 0) return;

    try {
      for (const file of selectedFiles) {
        await api.files.delete($currentPath + file.name);
      }
      success(`Deleted ${selectedFiles.length} file(s)`);
      selectedFiles = [];
      selectionMode = false;
      await loadFiles();
    } catch (err) {
      errorToast("Failed to delete files");
    }
  }

  async function handleBatchDownload() {
    for (const file of selectedFiles) {
      await handleDownload(file);
    }
  }

  async function toggleFavorite(file) {
    const filePath = $currentPath + file.name;
    if ($favorites.some((f) => f.path === filePath)) {
      await favorites.remove(filePath);
      success("Removed from favorites");
    } else {
      await favorites.add({ path: filePath, name: file.name });
      success("Added to favorites");
    }
  }

  async function handleAdvancedSearch(event) {
    isSearchActive = true;
    searchResults = event.detail.results || [];
    showAdvancedSearchModal = false;
  }

  function clearSearch() {
    isSearchActive = false;
    searchResults = [];
    searchQuery = "";
  }

  // ==================== FILE INTERACTION ====================
  function openFile(file) {
    if (file.is_dir) {
      const newPath = $currentPath + file.name + "/";
      loadFiles(newPath);
    } else {
      fileToPreview = file;
      showFilePreview = true;
    }
  }

  function handleFileClick(file, index, event) {
    if (event.ctrlKey || event.metaKey) {
      toggleSelection(file);
    } else if (event.shiftKey && lastSelectedIndex >= 0) {
      selectRange(lastSelectedIndex, index);
    } else {
      selectedFiles = [];
      openFile(file);
    }
    lastSelectedIndex = index;
  }

  function toggleSelection(file) {
    const idx = selectedFiles.findIndex((f) => f.name === file.name);
    if (idx >= 0) {
      selectedFiles.splice(idx, 1);
    } else {
      selectedFiles.push(file);
    }
    selectionMode = selectedFiles.length > 0;
  }

  function selectRange(start, end) {
    const [from, to] = start < end ? [start, end] : [end, start];
    selectedFiles = sortedFiles.slice(from, to + 1);
    selectionMode = true;
  }

  function selectAllFiles() {
    selectedFiles = [...sortedFiles];
    selectionMode = true;
  }

  function deselectAllFiles() {
    selectedFiles = [];
    selectionMode = false;
  }

  // ==================== DRAG & DROP ====================
  function handleDragOver(e) {
    e.preventDefault();
    dragOver = true;
  }

  function handleDragLeave() {
    dragOver = false;
  }

  async function handleDrop(e) {
    e.preventDefault();
    dragOver = false;

    const items = e.dataTransfer.files;
    if (items.length > 0) {
      uploadInput.files = items;
      await handleFileUpload();
    }
  }

  // ==================== CONTEXT MENU ====================
  function handleContextMenu(e, file) {
    e.preventDefault();
    contextMenuFile = file;
    contextMenuX = e.clientX;
    contextMenuY = e.clientY;
    contextMenuVisible = true;
  }

  async function handleContextAction(detail) {
    const { action, file } = detail;
    contextMenuVisible = false;

    switch (action) {
      case "open":
        openFile(file);
        break;
      case "rename":
        fileToRename = file;
        newFileName = file.name;
        showRenameModal = true;
        break;
      case "delete":
        fileToDelete = file;
        showDeleteModal = true;
        break;
      case "download":
        await handleDownload(file);
        break;
      case "move":
        fileToMove = file;
        showMoveModal = true;
        break;
      case "copy":
        fileToCopy = file;
        showCopyModal = true;
        break;
      case "share":
        fileToShare = file;
        showShareModal = true;
        break;
      case "versions":
        fileToViewVersions = file;
        showVersionHistoryModal = true;
        break;
      case "favorite":
        await toggleFavorite(file);
        break;
    }
  }

  // ==================== KEYBOARD SHORTCUTS ====================
  let keyboardHandler;

  function setupKeyboardShortcuts() {
    keyboardHandler = (e) => {
      // Ctrl+A - Select All
      if (e.ctrlKey && e.key === "a") {
        e.preventDefault();
        selectAllFiles();
      }
      // Escape - Clear selection
      if (e.key === "Escape") {
        deselectAllFiles();
        contextMenuVisible = false;
      }
      // Delete - Delete selected
      if (e.key === "Delete" && selectedFiles.length > 0) {
        handleBatchDelete();
      }
      // Ctrl+F - Search
      if (e.ctrlKey && e.key === "f") {
        e.preventDefault();
        document.querySelector('input[type="search"]')?.focus();
      }
    };

    window.addEventListener("keydown", keyboardHandler);
  }

  // ==================== WEBSOCKET ====================
  function setupWebSocket() {
    onFileEvent((event) => {
      if (event.path?.startsWith($currentPath)) {
        loadFiles();
      }
    });
  }

  // ==================== BREADCRUMB ====================
  function navigateToBreadcrumb(path) {
    loadFiles(path);
  }

  function getPathSegments() {
    return $currentPath.split("/").filter(Boolean);
  }
</script>

<!-- Loading State -->
{#if loading}
  <Loading />
{:else}
  <!-- Main Container -->
  <div
    class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
  >
    <!-- Upload Input (hidden) -->
    <input
      type="file"
      multiple
      bind:this={uploadInput}
      onchange={handleFileUpload}
      class="hidden"
    />

    <!-- Page Header -->
    <div class="max-w-7xl mx-auto mb-6">
      <div class="glass-card p-6 animate-slide-down">
        <div class="flex flex-wrap items-center justify-between gap-4">
          <!-- Title -->
          <div>
            <h1 class="text-3xl font-bold gradient-text mb-2">My Files</h1>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {sortedFiles.length} items in current folder
            </p>
          </div>

          <!-- Actions -->
          <div class="flex gap-2">
            <button
              onclick={() => uploadInput?.click()}
              class="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold rounded-xl shadow-lg shadow-blue-500/30 hover:shadow-xl hover:shadow-blue-500/40 transition-all duration-200 flex items-center gap-2"
            >
              <i class="bi bi-upload"></i>
              <span>Upload</span>
            </button>

            <button
              onclick={() => (showNewFolderModal = true)}
              class="px-4 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 font-semibold rounded-xl hover:border-gray-300 dark:hover:border-gray-600 transition-all duration-200 flex items-center gap-2"
            >
              <i class="bi bi-folder-plus"></i>
              <span>New Folder</span>
            </button>

            <button
              onclick={() => loadFiles()}
              class="px-4 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 font-semibold rounded-xl hover:border-gray-300 dark:hover:border-gray-600 transition-all duration-200"
              aria-label="Refresh"
            >
              <i class="bi bi-arrow-clockwise"></i>
            </button>
          </div>
        </div>

        <!-- Toolbar -->
        <div class="mt-6 flex flex-wrap items-center gap-4">
          <!-- Search -->
          <div class="flex-1 min-w-[200px]">
            <div class="relative">
              <input
                type="search"
                bind:value={searchQuery}
                placeholder="Search files..."
                class="w-full px-4 py-2 pl-10 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none transition-all"
              />
              <i class="bi bi-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"></i>
            </div>
          </div>

          <!-- Advanced Search -->
          <button
            onclick={() => (showAdvancedSearchModal = true)}
            class="px-4 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl hover:border-blue-500 transition-all flex items-center gap-2 text-sm font-medium"
          >
            <i class="bi bi-filter"></i>
            <span>Advanced</span>
          </button>

          <!-- Clear Search -->
          {#if isSearchActive || searchQuery}
            <button
              onclick={clearSearch}
              class="px-4 py-2 bg-red-50 dark:bg-red-900/20 border-2 border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 rounded-xl hover:bg-red-100 dark:hover:bg-red-900/30 transition-all flex items-center gap-2 text-sm font-medium"
            >
              <i class="bi bi-x-circle"></i>
              <span>Clear</span>
            </button>
          {/if}

          <!-- View Mode Toggle -->
          <div class="flex bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl overflow-hidden">
            <button
              onclick={() => (viewMode = "grid")}
              class="px-4 py-2 {viewMode === 'grid'
                ? 'bg-blue-600 text-white'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'} transition-all"
            >
              <i class="bi bi-grid-3x3"></i>
            </button>
            <button
              onclick={() => (viewMode = "list")}
              class="px-4 py-2 {viewMode === 'list'
                ? 'bg-blue-600 text-white'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'} transition-all"
            >
              <i class="bi bi-list-ul"></i>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="max-w-7xl mx-auto mb-6">
      <div class="glass-card-light p-4">
        <nav class="flex items-center gap-2 text-sm">
          <button
            onclick={() => navigateToBreadcrumb("/")}
            class="text-blue-600 dark:text-blue-400 hover:underline font-medium"
          >
            <i class="bi bi-house-fill"></i>
            Home
          </button>
          {#each getPathSegments() as segment, i}
            <i class="bi bi-chevron-right text-gray-400"></i>
            <button
              onclick={() => {
                const path =
                  "/" + getPathSegments().slice(0, i + 1).join("/") + "/";
                navigateToBreadcrumb(path);
              }}
              class="text-blue-600 dark:text-blue-400 hover:underline font-medium"
            >
              {segment}
            </button>
          {/each}
        </nav>
      </div>
    </div>

    <!-- Selection Toolbar -->
    {#if selectionMode && selectedFiles.length > 0}
      <div class="max-w-7xl mx-auto mb-6">
        <div class="glass-card-light border-l-4 border-blue-500 p-4 animate-slide-down">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <i class="bi bi-check-square text-2xl text-blue-600"></i>
              <div>
                <h3 class="font-bold text-lg">{selectedFiles.length} file(s) selected</h3>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  Ctrl+Click to select more, Shift+Click for range
                </p>
              </div>
            </div>

            <div class="flex gap-2">
              <button
                onclick={selectAllFiles}
                class="px-4 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-all flex items-center gap-2"
              >
                <i class="bi bi-check-all"></i>
                Select All
              </button>
              <button
                onclick={deselectAllFiles}
                class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-300 dark:hover:bg-gray-600 transition-all flex items-center gap-2"
              >
                <i class="bi bi-x-circle"></i>
                Clear
              </button>
              <button
                onclick={handleBatchDownload}
                class="px-4 py-2 bg-green-600 text-white rounded-xl hover:bg-green-700 transition-all flex items-center gap-2"
              >
                <i class="bi bi-download"></i>
                Download
              </button>
              <button
                onclick={handleBatchDelete}
                class="px-4 py-2 bg-red-600 text-white rounded-xl hover:bg-red-700 transition-all flex items-center gap-2"
              >
                <i class="bi bi-trash3"></i>
                Delete
              </button>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Upload Progress -->
    {#if uploading}
      <div class="max-w-7xl mx-auto mb-6">
        <div class="glass-card p-6 animate-slide-down">
          <h3 class="font-bold mb-4">Uploading Files...</h3>
          <div class="space-y-3">
            {#each Array.from(uploadProgress.entries()) as [index, prog]}
              <div>
                <div class="flex justify-between text-sm mb-1">
                  <span>{prog.file}</span>
                  <span>{prog.percent}%</span>
                </div>
                <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                  <div
                    class="bg-blue-600 h-2 rounded-full transition-all duration-300"
                    style="width: {prog.percent}%"
                  ></div>
                </div>
              </div>
            {/each}
          </div>
          <div class="mt-4">
            <div class="flex justify-between text-sm mb-1">
              <span class="font-semibold">Overall Progress</span>
              <span class="font-semibold">{overallProgress}%</span>
            </div>
            <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3">
              <div
                class="bg-gradient-to-r from-blue-600 to-purple-600 h-3 rounded-full transition-all duration-300"
                style="width: {overallProgress}%"
              ></div>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Drag Over Overlay -->
    {#if dragOver}
      <div class="fixed inset-0 z-50 bg-blue-500/20 backdrop-blur-sm flex items-center justify-center animate-fade-in">
        <div class="glass-card p-12 text-center">
          <i class="bi bi-cloud-upload text-6xl text-blue-600 mb-4"></i>
          <h2 class="text-2xl font-bold mb-2">Drop files to upload</h2>
          <p class="text-gray-600 dark:text-gray-400">Release to start uploading</p>
        </div>
      </div>
    {/if}

    <!-- Files Grid/List -->
    <div class="max-w-7xl mx-auto">
      {#if sortedFiles.length === 0}
        <!-- Empty State -->
        <div class="glass-card p-12 text-center animate-slide-up">
          <i class="bi bi-folder2-open text-6xl text-gray-400 mb-4"></i>
          <h3 class="text-xl font-bold mb-2">No files found</h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {isSearchActive
              ? "Try adjusting your search criteria"
              : "Upload files or create a folder to get started"}
          </p>
          {#if !isSearchActive}
            <button
              onclick={() => uploadInput?.click()}
              class="px-6 py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl transition-all"
            >
              <i class="bi bi-upload mr-2"></i>
              Upload Files
            </button>
          {/if}
        </div>
      {:else if viewMode === "grid"}
        <!-- Grid View -->
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          {#each sortedFiles as file, index}
            <div
              class="glass-card p-4 cursor-pointer hover:scale-105 transition-all duration-200 {selectedFiles.some(
                (f) => f.name === file.name
              )
                ? 'ring-2 ring-blue-500'
                : ''}"
              onclick={(e) => handleFileClick(file, index, e)}
              oncontextmenu={(e) => handleContextMenu(e, file)}
            >
              <!-- Thumbnail -->
              <div class="aspect-square mb-3 flex items-center justify-center bg-gray-100 dark:bg-gray-800 rounded-xl">
                {#if file.is_dir}
                  <i class="bi bi-folder-fill text-5xl text-blue-500"></i>
                {:else}
                  <FileThumbnail {file} size="large" />
                {/if}
              </div>

              <!-- File Info -->
              <div class="text-center">
                <p class="font-semibold text-sm truncate mb-1" title={file.name}>
                  {file.name}
                </p>
                {#if !file.is_dir}
                  <p class="text-xs text-gray-500">
                    {(file.size_bytes / 1024).toFixed(1)} KB
                  </p>
                {/if}
              </div>

              <!-- Favorite Icon -->
              {#if $favorites.some((f) => f.path === $currentPath + file.name)}
                <div class="absolute top-2 right-2">
                  <i class="bi bi-star-fill text-yellow-500"></i>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {:else}
        <!-- List View -->
        <div class="glass-card overflow-hidden">
          <table class="w-full">
            <thead class="bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider">
                  Name
                </th>
                <th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider">
                  Size
                </th>
                <th class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider">
                  Modified
                </th>
                <th class="px-6 py-3 text-right text-xs font-semibold uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
              {#each sortedFiles as file, index}
                <tr
                  class="hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer {selectedFiles.some(
                    (f) => f.name === file.name
                  )
                    ? 'bg-blue-50 dark:bg-blue-900/20'
                    : ''}"
                  onclick={(e) => handleFileClick(file, index, e)}
                  oncontextmenu={(e) => handleContextMenu(e, file)}
                >
                  <td class="px-6 py-4 flex items-center gap-3">
                    <i
                      class="bi bi-{file.is_dir
                        ? 'folder-fill text-blue-500'
                        : getFileIcon(file.name)} text-xl {getFileIconColor(file.name)}"
                    ></i>
                    <span class="font-medium">{file.name}</span>
                    {#if $favorites.some((f) => f.path === $currentPath + file.name)}
                      <i class="bi bi-star-fill text-yellow-500 text-sm"></i>
                    {/if}
                  </td>
                  <td class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400">
                    {#if !file.is_dir}
                      {(file.size_bytes / 1024).toFixed(1)} KB
                    {:else}
                      —
                    {/if}
                  </td>
                  <td class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400">
                    {new Date(file.modified_at).toLocaleDateString()}
                  </td>
                  <td class="px-6 py-4 text-right">
                    <button
                      onclick={(e) => {
                        e.stopPropagation();
                        handleContextMenu(e, file);
                      }}
                      class="px-3 py-1 bg-gray-100 dark:bg-gray-700 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-all"
                    >
                      <i class="bi bi-three-dots-vertical"></i>
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>
{/if}

<!-- Context Menu -->
{#if contextMenuVisible}
  <ContextMenu
    x={contextMenuX}
    y={contextMenuY}
    file={contextMenuFile}
    onaction={handleContextAction}
    onclose={() => (contextMenuVisible = false)}
  />
{/if}

<!-- Modals -->
{#if showNewFolderModal}
  <Modal title="Create New Folder" onclose={() => (showNewFolderModal = false)}>
    <div class="space-y-4">
      <input
        type="text"
        bind:value={newFolderName}
        placeholder="Folder name"
        class="w-full px-4 py-2 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none"
      />
      <div class="flex gap-3 justify-end">
        <button
          onclick={() => (showNewFolderModal = false)}
          class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded-xl hover:bg-gray-300 dark:hover:bg-gray-600 transition-all"
        >
          Cancel
        </button>
        <button
          onclick={handleCreateFolder}
          class="px-4 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-all"
        >
          Create
        </button>
      </div>
    </div>
  </Modal>
{/if}

{#if showRenameModal}
  <Modal title="Rename" onclose={() => (showRenameModal = false)}>
    <div class="space-y-4">
      <input
        type="text"
        bind:value={newFileName}
        class="w-full px-4 py-2 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none"
      />
      <div class="flex gap-3 justify-end">
        <button
          onclick={() => (showRenameModal = false)}
          class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded-xl hover:bg-gray-300 dark:hover:bg-gray-600 transition-all"
        >
          Cancel
        </button>
        <button
          onclick={handleRename}
          class="px-4 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-all"
        >
          Rename
        </button>
      </div>
    </div>
  </Modal>
{/if}

{#if showDeleteModal}
  <Modal title="Confirm Delete" onclose={() => (showDeleteModal = false)}>
    <div class="space-y-4">
      <p>Are you sure you want to delete <strong>{fileToDelete?.name}</strong>?</p>
      <div class="flex gap-3 justify-end">
        <button
          onclick={() => (showDeleteModal = false)}
          class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded-xl hover:bg-gray-300 dark:hover:bg-gray-600 transition-all"
        >
          Cancel
        </button>
        <button
          onclick={handleDelete}
          class="px-4 py-2 bg-red-600 text-white rounded-xl hover:bg-red-700 transition-all"
        >
          Delete
        </button>
      </div>
    </div>
  </Modal>
{/if}

{#if showMoveModal}
  <Modal title="Move File" onclose={() => (showMoveModal = false)}>
    <div class="space-y-4">
      <p>Moving: <strong>{fileToMove?.name}</strong></p>
      <input
        type="text"
        bind:value={moveTargetPath}
        placeholder="/target/path/"
        class="w-full px-4 py-2 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none"
      />
      <div class="flex gap-3 justify-end">
        <button
          onclick={() => (showMoveModal = false)}
          class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded-xl hover:bg-gray-300 dark:hover:bg-gray-600 transition-all"
        >
          Cancel
        </button>
        <button
          onclick={handleMove}
          class="px-4 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-all"
        >
          Move
        </button>
      </div>
    </div>
  </Modal>
{/if}

{#if showCopyModal}
  <Modal title="Copy File" onclose={() => (showCopyModal = false)}>
    <div class="space-y-4">
      <p>Copying: <strong>{fileToCopy?.name}</strong></p>
      <input
        type="text"
        bind:value={copyTargetPath}
        placeholder="/target/path/"
        class="w-full px-4 py-2 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none"
      />
      <div class="flex gap-3 justify-end">
        <button
          onclick={() => (showCopyModal = false)}
          class="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded-xl hover:bg-gray-300 dark:hover:bg-gray-600 transition-all"
        >
          Cancel
        </button>
        <button
          onclick={handleCopy}
          class="px-4 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-all"
        >
          Copy
        </button>
      </div>
    </div>
  </Modal>
{/if}

{#if showAdvancedSearchModal}
  <AdvancedSearchModal
    onclose={() => (showAdvancedSearchModal = false)}
    onsearch={handleAdvancedSearch}
  />
{/if}

{#if showShareModal && fileToShare}
  <ShareModal file={fileToShare} onclose={() => (showShareModal = false)} />
{/if}

{#if showVersionHistoryModal && fileToViewVersions}
  <VersionHistoryModal
    file={fileToViewVersions}
    onclose={() => (showVersionHistoryModal = false)}
  />
{/if}

{#if showFilePreview && fileToPreview}
  <FilePreviewModal
    file={fileToPreview}
    onclose={() => (showFilePreview = false)}
  />
{/if}

<style>
  /* Additional custom styles if needed */
</style>
