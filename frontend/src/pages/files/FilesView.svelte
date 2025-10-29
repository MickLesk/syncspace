<script>
  import { onMount, onDestroy } from "svelte";
  import { files, currentPath } from "../../stores/ui";
  import { favorites } from "../../stores/favorites";
  import { success, error as errorToast } from "../../stores/toast";
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import ContextMenu from "../../components/ui/ContextMenu.svelte";
  import Modal from "../../components/ui/Modal.svelte";
  import FileThumbnail from "../../components/files/FileThumbnail.svelte";
  import AdvancedSearchModal from "../../components/search/AdvancedSearchModal.svelte";
  import ShareModal from "../../components/sharing/ShareModal.svelte";
  import VersionHistoryModal from "../../components/files/VersionHistoryModal.svelte";
  import FilePreviewModal from "../../components/files/FilePreviewModal.svelte";
  import BatchOperationsToolbar from "../../components/ui/BatchOperationsToolbar.svelte";
  import api from "../../lib/api";
  import {
    wsConnected,
    onFileEvent,
    websocketManager,
  } from "../../stores/websocket.js";
  import Loading from "../../components/ui/Loading.svelte";
  import VirtualList from "../../components/ui/VirtualList.svelte";

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
  let uploadInput; // DOM element reference - NOT $state()
  let uploadFiles = $state([]);
  let uploadProgress = $state(new Map());
  let overallProgress = $state(0);

  // Multi-Select
  let selectedFiles = $state([]);
  let lastSelectedIndex = $state(-1);
  let selectionMode = $state(false);

  // Performance Optimization: Pagination
  const ITEMS_PER_PAGE = 50; // Show 50 items initially
  const LOAD_MORE_INCREMENT = 50; // Load 50 more each time
  let displayLimit = $state(ITEMS_PER_PAGE);

  // Accessibility: ESC cancels drag overlay
  let escHandler;
  onMount(() => {
    escHandler = (e) => {
      if (dragOver && e.key === "Escape") dragOver = false;
    };
    window.addEventListener("keydown", escHandler);
  });
  onDestroy(() => {
    if (escHandler) window.removeEventListener("keydown", escHandler);
  });

  // ==================== COMPUTED ====================
  let filteredFiles = $derived(
    isSearchActive
      ? searchResults
      : searchQuery
        ? $files.filter((f) =>
            f.name.toLowerCase().includes(searchQuery.toLowerCase())
          )
        : $files
  );

  let sortedFiles = $derived.by(() => {
    let result = showFoldersOnly
      ? filteredFiles.filter((f) => f.is_directory)
      : filteredFiles;

    result = [...result].sort((a, b) => {
      let comparison = 0;
      switch (sortBy) {
        case "name":
          comparison = a.name.localeCompare(b.name);
          break;
        case "modified":
          comparison =
            new Date(a.modified_at).getTime() -
            new Date(b.modified_at).getTime();
          break;
        case "size":
          comparison = (a.size || 0) - (b.size || 0);
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
      if (a.is_directory && !b.is_directory) return -1;
      if (!a.is_directory && b.is_directory) return 1;
      return 0;
    });
  });

  // Paginated files for display (defined AFTER sortedFiles)
  let paginatedFiles = $derived(sortedFiles.slice(0, displayLimit));
  let hasMoreFiles = $derived(sortedFiles.length > displayLimit);

  // Reset display limit when files change (new search, navigation, etc.)
  $effect(() => {
    sortedFiles; // Track changes
    displayLimit = ITEMS_PER_PAGE;
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

  // ==================== PERFORMANCE: INFINITE SCROLL ====================
  function handleInfiniteScroll(node) {
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasMoreFiles) {
          displayLimit += LOAD_MORE_INCREMENT;
        }
      },
      { threshold: 0.1 }
    );

    observer.observe(node);

    return {
      destroy() {
        observer.disconnect();
      },
    };
  }

  // ==================== FUNCTIONS ====================
  async function loadFiles(path = null) {
    loading = true;
    try {
      const targetPath = path || $currentPath;
      const response = await api.files.list(targetPath);
      // response is already the array, not wrapped in { data: ... }
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

  async function handleFileUpload() {
    if (!uploadInput?.files?.length) return;

    uploading = true;
    uploadFiles = Array.from(uploadInput.files);
    uploadProgress = new Map();

    for (let i = 0; i < uploadFiles.length; i++) {
      const file = uploadFiles[i];
      try {
        // Use uploadWithProgress which sends to /upload-multipart with FormData
        await api.files.uploadWithProgress($currentPath, file, (percent) => {
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
      // Construct the full path for the new folder
      const folderPath = $currentPath
        ? `${$currentPath.replace(/\/$/, "")}/${newFolderName}`
        : newFolderName;
      await api.files.createDir(folderPath);
      success(`Folder created: ${newFolderName}`);
      showNewFolderModal = false;
      newFolderName = "";
      await loadFiles();
    } catch (err) {
      errorToast("Failed to create folder");
      console.error(err);
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
    // $favorites is a Map, not an array - use .has() or check values
    const favoritesArray = Array.from($favorites.values());
    if (favoritesArray.some((f) => f.path === filePath)) {
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
    if (file.is_directory) {
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
      // Create a DataTransfer to assign multiple files to the input
      const dt = new DataTransfer();
      for (let i = 0; i < items.length; i++) {
        dt.items.add(items[i]);
      }
      uploadInput.files = dt.files;
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
  <PageWrapper gradient={true} maxWidth="7xl">
    <!-- Upload Input (hidden) -->
    <input
      type="file"
      multiple
      bind:this={uploadInput}
      onchange={handleFileUpload}
      class="hidden"
    />

    <!-- Drag & Drop Overlay (single, accessible, improved) -->
    {#if dragOver}
      <div
        class="fixed inset-0 z-50 bg-blue-600/30 backdrop-blur-sm flex items-center justify-center animate-fade-in"
        ondragover={handleDragOver}
        ondragleave={handleDragLeave}
        ondrop={handleDrop}
        aria-label="File upload drop zone"
        role="dialog"
        tabindex="0"
      >
        <div
          class="glass-card p-12 text-center border-4 border-blue-400 rounded-2xl shadow-2xl bg-white/80 dark:bg-gray-900/80"
          aria-live="assertive"
        >
          <i
            class="bi bi-cloud-upload text-7xl text-blue-600 mb-4"
            aria-hidden="true"
          ></i>
          <h2 class="text-2xl font-bold mb-2">Drop files to upload</h2>
          <p class="text-gray-700 dark:text-gray-300 mb-2">
            Release to start uploading. You can drop multiple files.
          </p>
          <p class="text-xs text-gray-500">(ESC to cancel)</p>
        </div>
      </div>
    {/if}

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
              class="px-4 py-2 bg-white dark:bg-gray-700 border-2 border-gray-300 dark:border-gray-600 text-gray-800 dark:text-gray-200 font-semibold rounded-xl hover:bg-gray-50 dark:hover:bg-gray-600 hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-200 flex items-center gap-2"
            >
              <i class="bi bi-folder-plus"></i>
              <span>New Folder</span>
            </button>

            <button
              onclick={() => loadFiles()}
              class="px-4 py-2 bg-white dark:bg-gray-700 border-2 border-gray-300 dark:border-gray-600 text-gray-800 dark:text-gray-200 font-semibold rounded-xl hover:bg-gray-50 dark:hover:bg-gray-600 hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
              aria-label="Refresh files"
              disabled={loading}
            >
              <i class="bi bi-arrow-clockwise {loading ? 'animate-spin' : ''}"
              ></i>
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
                class="w-full px-4 py-2 pl-10 bg-white border-2 border-gray-300 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none transition-all text-gray-900 placeholder-gray-500"
              />
              <i
                class="bi bi-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
              ></i>
            </div>
          </div>

          <!-- Advanced Search -->
          <button
            onclick={() => (showAdvancedSearchModal = true)}
            class="px-4 py-2 bg-white border-2 border-gray-300 text-gray-800 rounded-xl hover:bg-gray-50 hover:border-blue-500 transition-all flex items-center gap-2 text-sm font-medium"
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
          <div
            class="flex bg-white dark:bg-gray-700 border-2 border-gray-300 dark:border-gray-600 rounded-xl overflow-hidden"
          >
            <button
              onclick={() => (viewMode = "grid")}
              class="px-4 py-2 {viewMode === 'grid'
                ? 'bg-blue-600 text-white'
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600'} transition-all"
            >
              <i class="bi bi-grid-3x3"></i>
            </button>
            <button
              onclick={() => (viewMode = "list")}
              class="px-4 py-2 {viewMode === 'list'
                ? 'bg-blue-600 text-white'
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600'} transition-all"
            >
              <i class="bi bi-list-ul"></i>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="max-w-7xl mx-auto mb-6">
      <div class="glass-card p-4">
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
                  "/" +
                  getPathSegments()
                    .slice(0, i + 1)
                    .join("/") +
                  "/";
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
    <div class="max-w-7xl mx-auto">
      <BatchOperationsToolbar
        bind:selectedFiles
        onClearSelection={deselectAllFiles}
      />
    </div>

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
                <div
                  class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2"
                >
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

    <!-- (Removed duplicate drag overlay) -->

    <!-- Files Grid/List -->
    <div class="max-w-7xl mx-auto">
      {#if sortedFiles.length === 0}
        <!-- Empty State -->
        <div class="glass-card p-12 text-center animate-slide-up">
          <i
            class="bi bi-folder2-open text-6xl text-gray-400 dark:text-gray-600 mb-4"
          ></i>
          <h3 class="text-xl font-bold mb-2 text-gray-900 dark:text-gray-100">
            No files found
          </h3>
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
        <div
          class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6"
        >
          {#each paginatedFiles as file, index}
            <div
              class="bg-white dark:bg-gray-800 rounded-2xl p-6 shadow-md hover:shadow-2xl transition-all duration-300 cursor-pointer relative group border border-gray-100 dark:border-gray-700 {selectedFiles.some(
                (f) => f.name === file.name
              )
                ? 'ring-2 ring-blue-500'
                : ''}"
              onclick={(e) => handleFileClick(file, index, e)}
              oncontextmenu={(e) => handleContextMenu(e, file)}
            >
              <!-- Action Button (Three Dots) -->
              <div
                class="absolute top-3 left-3 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
              >
                <button
                  onclick={(e) => {
                    e.stopPropagation();
                    e.preventDefault();
                    handleContextMenu(e, file);
                  }}
                  type="button"
                  class="p-2 bg-white dark:bg-gray-700 rounded-lg shadow-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition-all border border-gray-300 dark:border-gray-600"
                  title="Actions"
                >
                  <i
                    class="bi bi-three-dots-vertical text-gray-700 dark:text-gray-300"
                  ></i>
                </button>
              </div>

              <!-- Thumbnail/Icon Container -->
              <div class="flex items-center justify-center mb-4 h-32">
                {#if file.is_directory}
                  <i
                    class="bi bi-folder-fill text-blue-500 dark:text-blue-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else if file.name.endsWith(".pdf")}
                  <i
                    class="bi bi-file-earmark-pdf-fill text-red-600 dark:text-red-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else if file.name.match(/\.(jpg|jpeg|png|gif|webp|svg)$/i)}
                  <i
                    class="bi bi-file-earmark-image-fill text-purple-600 dark:text-purple-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else if file.name.match(/\.(xlsx|xls|csv)$/i)}
                  <i
                    class="bi bi-file-earmark-excel-fill text-green-600 dark:text-green-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else if file.name.match(/\.(tar|zip|rar|7z|gz)$/i)}
                  <i
                    class="bi bi-file-earmark-zip-fill text-amber-700 dark:text-amber-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else if file.name.match(/\.(mp4|avi|mkv|mov|webm)$/i)}
                  <i
                    class="bi bi-file-earmark-play-fill text-pink-600 dark:text-pink-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else if file.name.match(/\.(mp3|wav|ogg|flac)$/i)}
                  <i
                    class="bi bi-file-earmark-music-fill text-yellow-600 dark:text-yellow-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else if file.name.match(/\.(js|ts|jsx|tsx|json|html|css|py|java|cpp|c|rs|go|php|rb)$/i)}
                  <i
                    class="bi bi-file-earmark-code-fill text-cyan-600 dark:text-cyan-400"
                    style="font-size: 5rem;"
                  ></i>
                {:else}
                  <i
                    class="bi bi-file-earmark-fill text-gray-500 dark:text-gray-400"
                    style="font-size: 5rem;"
                  ></i>
                {/if}
              </div>

              <!-- File Info -->
              <div class="text-center">
                <p
                  class="font-semibold text-sm truncate mb-1 text-gray-900 dark:text-gray-100"
                  title={file.name}
                >
                  {file.name}
                </p>
                {#if !file.is_directory}
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    {(file.size / 1024).toFixed(1)} KB
                  </p>
                {/if}
              </div>

              <!-- Favorite Icon -->
              {#if Array.from($favorites.values()).some((f) => f.path === $currentPath + file.name)}
                <div class="absolute top-3 right-3">
                  <i
                    class="bi bi-star-fill text-yellow-500 text-lg drop-shadow-lg"
                  ></i>
                </div>
              {/if}
            </div>
          {/each}
        </div>

        <!-- Load More Button (Grid View) -->
        {#if hasMoreFiles}
          <div class="flex justify-center mt-8 mb-4">
            <button
              onclick={() => (displayLimit += LOAD_MORE_INCREMENT)}
              class="px-8 py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-200 flex items-center gap-2"
            >
              <i class="bi bi-arrow-down-circle"></i>
              Load More ({sortedFiles.length - displayLimit} remaining)
            </button>
          </div>
          <!-- Infinite Scroll Sentinel -->
          <div use:handleInfiniteScroll class="h-4"></div>
        {/if}
      {:else}
        <!-- List View -->
        <div class="glass-card overflow-hidden">
          <table class="w-full">
            <thead
              class="bg-gray-50 dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700"
            >
              <tr>
                <th
                  class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >
                  Name
                </th>
                <th
                  class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >
                  Size
                </th>
                <th
                  class="px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >
                  Modified
                </th>
                <th
                  class="px-6 py-3 text-right text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >
                  Actions
                </th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
              {#each paginatedFiles as file, index}
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
                      class="bi bi-{file.is_directory
                        ? 'folder-fill text-blue-500 dark:text-blue-400'
                        : getFileIcon(file.name)} text-xl {getFileIconColor(
                        file.name
                      )}"
                    ></i>
                    <span class="font-medium text-gray-900 dark:text-gray-100"
                      >{file.name}</span
                    >
                    {#if Array.from($favorites.values()).some((f) => f.path === $currentPath + file.name)}
                      <i
                        class="bi bi-star-fill text-yellow-500 dark:text-yellow-400 text-sm"
                      ></i>
                    {/if}
                  </td>
                  <td
                    class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400"
                  >
                    {#if !file.is_directory}
                      {(file.size / 1024).toFixed(1)} KB
                    {:else}
                      —
                    {/if}
                  </td>
                  <td
                    class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400"
                  >
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

        <!-- Load More Button (List View) -->
        {#if hasMoreFiles}
          <div class="flex justify-center mt-6 mb-4">
            <button
              onclick={() => (displayLimit += LOAD_MORE_INCREMENT)}
              class="px-8 py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-200 flex items-center gap-2"
            >
              <i class="bi bi-arrow-down-circle"></i>
              Load More ({sortedFiles.length - displayLimit} remaining)
            </button>
          </div>
          <!-- Infinite Scroll Sentinel -->
          <div use:handleInfiniteScroll class="h-4"></div>
        {/if}
      {/if}
    </div>
  </PageWrapper>
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
<Modal
  visible={showNewFolderModal}
  title="Create New Folder"
  onclose={() => (showNewFolderModal = false)}
>
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

<Modal
  visible={showRenameModal}
  title="Rename"
  onclose={() => (showRenameModal = false)}
>
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

<Modal
  visible={showDeleteModal}
  title="Confirm Delete"
  onclose={() => (showDeleteModal = false)}
>
  <div class="space-y-4">
    <p>
      Are you sure you want to delete <strong>{fileToDelete?.name}</strong>?
    </p>
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

<Modal
  visible={showMoveModal}
  title="Move File"
  onclose={() => (showMoveModal = false)}
>
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

<Modal
  visible={showCopyModal}
  title="Copy File"
  onclose={() => (showCopyModal = false)}
>
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

<AdvancedSearchModal
  bind:visible={showAdvancedSearchModal}
  on:search={handleAdvancedSearch}
/>

<ShareModal
  isOpen={showShareModal && fileToShare}
  file={fileToShare}
  onClose={() => (showShareModal = false)}
/>

<VersionHistoryModal
  isOpen={showVersionHistoryModal && fileToViewVersions}
  file={fileToViewVersions}
  onClose={() => (showVersionHistoryModal = false)}
/>

<FilePreviewModal
  bind:visible={showFilePreview}
  bind:file={fileToPreview}
  allFiles={paginatedFiles.filter((f) => !f.is_directory)}
  currentIndex={fileToPreview
    ? paginatedFiles
        .filter((f) => !f.is_directory)
        .findIndex((f) => f.name === fileToPreview.name)
    : 0}
/>

<style>
  /* Additional custom styles if needed */
</style>
