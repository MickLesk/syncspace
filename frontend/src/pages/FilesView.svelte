<script>
  import { onMount, onDestroy } from "svelte";
  import { files, currentPath } from "../stores/ui";
  import { favorites } from "../stores/favorites";
  import { success, error as errorToast } from "../stores/toast";
  import { getFileIcon, getFileIconColor } from "../utils/fileIcons";
  import ContextMenu from "../components/ui/ContextMenu.svelte";
  import Breadcrumb from "../components/navigation/Breadcrumb.svelte";
  import Modal from "../components/ui/Modal.svelte";
  import FileThumbnail from "../components/files/FileThumbnail.svelte";
  import AdvancedSearchModal from "../components/search/AdvancedSearchModal.svelte";
  import ShareModal from "../components/sharing/ShareModal.svelte";
  import VersionHistoryModal from "../components/files/VersionHistoryModal.svelte";
  import FilePreviewModal from "../components/files/FilePreviewModal.svelte";
  import FolderColorPicker from "../components/files/FolderColorPicker.svelte";
  import api from "../lib/api";
  import {
    wsConnected,
    onFileEvent,
    websocketManager,
  } from "@stores/websocket.js";
  import PageWrapper from "../components/PageWrapper.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";

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
  let showUploadModal = $state(false);
  let showNewFolderModal = $state(false);
  let showRenameModal = $state(false);
  let showDeleteModal = $state(false);
  let showAdvancedSearchModal = $state(false);
  let showShareModal = $state(false);
  let showVersionHistoryModal = $state(false);
  let showFilePreview = $state(false);
  let fileToPreview = $state(null);
  let showMoveModal = $state(false);
  let showCopyModal = $state(false);
  let fileToMove = $state(null);
  let fileToCopy = $state(null);
  let moveTargetPath = $state("");
  let copyTargetPath = $state("");

  // Current action targets
  let fileToRename = $state(null);
  let fileToDelete = $state(null);
  let fileToShare = $state(null);
  let fileToViewVersions = $state(null);
  let folderForColorPicker = $state(null);
  let showFolderColorPicker = $state(false);
  let newFolderName = $state("");
  let newFileName = $state("");

  // Context Menu
  let contextMenuVisible = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuFile = $state(null);

  // File upload with progress tracking
  let uploadInput;
  let uploadFiles = $state([]);
  let uploadProgress = $state(new Map()); // Map<fileIndex, {percent, loaded, total}>
  let overallProgress = $state(0);

  // Multi-Select
  let selectedFiles = $state([]);
  let lastSelectedIndex = $state(-1);
  let selectionMode = $state(false);

  // Global keyboard handler (needs to be outside onMount for cleanup)
  let keyboardHandler = null;

  let filteredFiles = $derived(
    isSearchActive
      ? searchResults
      : searchQuery
        ? $files.filter((f) =>
            f.name.toLowerCase().includes(searchQuery.toLowerCase())
          )
        : $files
  );

  // Sort and filter files
  let sortedFiles = $derived.by(() => {
    let result = showFoldersOnly
      ? filteredFiles.filter((f) => f.is_dir)
      : filteredFiles;

    // Sort the files
    result = [...result].sort((a, b) => {
      let comparison = 0;

      switch (sortBy) {
        case "name":
          comparison = a.name.localeCompare(b.name);
          break;
        case "modified":
          const aTime = new Date(a.modified_at || 0).getTime();
          const bTime = new Date(b.modified_at || 0).getTime();
          comparison = aTime - bTime;
          break;
        case "size":
          comparison = (a.size || 0) - (b.size || 0);
          break;
        case "type":
          const aExt = a.name.split(".").pop() || "";
          const bExt = b.name.split(".").pop() || "";
          comparison = aExt.localeCompare(bExt);
          break;
      }

      return sortOrder === "asc" ? comparison : -comparison;
    });

    // Always show folders first
    const folders = result.filter((f) => f.is_dir);
    const files = result.filter((f) => !f.is_dir);
    return [...folders, ...files];
  });

  let breadcrumbPath = $derived($currentPath.split("/").filter(Boolean));

  // WebSocket subscription for real-time file updates
  let unsubscribeWebSocket;
  let reloadTimeout = null;

  // React to currentPath changes (e.g. from sidebar navigation)
  $effect(() => {
    // Re-load files when path changes
    const path = $currentPath;
    loadFiles();
  });

  onMount(async () => {
    await loadFiles();

    // Load favorites from backend
    await favorites.load();

    // Initialize WebSocket connection for real-time file updates
    // Only connect if not already connected
    if (
      !websocketManager.ws ||
      websocketManager.ws.readyState !== WebSocket.OPEN
    ) {
      websocketManager.connect();
    }

    // Subscribe to WebSocket file events
    unsubscribeWebSocket = onFileEvent((event) => {
      // Filter out system files and temp files
      const ignoredPatterns = [
        "syncspace.db",
        ".db-shm",
        ".db-wal",
        "search_index",
        ".tmp",
        ".lock",
        "hallo", // Ignore test folder
      ];

      if (ignoredPatterns.some((pattern) => event.path.includes(pattern))) {
        console.log("🚫 Ignored system file event:", event.path);
        return; // Ignore system files
      }

      // Only log non-ignored events
      console.log("📁 FilesView received file event:", event);

      // Check if the event affects the current directory
      const eventDir = event.path.substring(0, event.path.lastIndexOf("/") + 1);
      const currentDir = $currentPath;

      console.log("Event dir:", eventDir, "Current dir:", currentDir);

      // If the event is in the current directory, reload files (debounced)
      if (eventDir === currentDir || eventDir.startsWith(currentDir)) {
        // Clear existing timeout
        if (reloadTimeout) {
          clearTimeout(reloadTimeout);
        }

        // Debounce: Only reload after 1 second of no more events
        reloadTimeout = setTimeout(() => {
          console.log("🔄 Reloading files due to file system change");
          loadFiles();
          // Don't show notifications for every file change - too spammy
        }, 1000);
      }
    });

    // Global keyboard shortcuts
    keyboardHandler = (e) => {
      // Ignore if typing in input/textarea
      if (e.target.tagName === "INPUT" || e.target.tagName === "TEXTAREA") {
        return;
      }

      // Ctrl+K - Quick search (already handled by input)
      if (e.ctrlKey && e.key === "k") {
        e.preventDefault();
        const searchInput = document.querySelector(
          'input[placeholder*="Quick search"]'
        );
        if (searchInput instanceof HTMLInputElement) {
          searchInput.focus();
        }
        return;
      }

      // Escape - Close all modals
      if (e.key === "Escape") {
        showUploadModal = false;
        showNewFolderModal = false;
        showRenameModal = false;
        showDeleteModal = false;
        showAdvancedSearchModal = false;
        showShareModal = false;
        showVersionHistoryModal = false;
        showFilePreview = false;
        showMoveModal = false;
        showCopyModal = false;
        contextMenuVisible = false;
        return;
      }

      // Delete - Delete selected file (if context menu file is set)
      if (e.key === "Delete" && contextMenuFile) {
        e.preventDefault();
        fileToDelete = contextMenuFile;
        showDeleteModal = true;
        return;
      }

      // F2 - Rename selected file
      if (e.key === "F2" && contextMenuFile) {
        e.preventDefault();
        fileToRename = contextMenuFile;
        newFileName = contextMenuFile.name;
        showRenameModal = true;
        return;
      }

      // Ctrl+R - Refresh files
      if (e.ctrlKey && e.key === "r") {
        e.preventDefault();
        loadFiles();
        return;
      }
    };

    window.addEventListener("keydown", keyboardHandler);
  });

  onDestroy(() => {
    // Cleanup keyboard listener
    if (keyboardHandler) {
      window.removeEventListener("keydown", keyboardHandler);
    }

    if (unsubscribeWebSocket) {
      unsubscribeWebSocket();
    }
    if (reloadTimeout) {
      clearTimeout(reloadTimeout);
    }
  });

  async function loadFiles() {
    loading = true;
    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
      console.log("🔍 Loading files from path:", backendPath || "(root)");

      const data = await api.files.list(backendPath);
      console.log("📦 Received files:", data);

      // Filter out system files (database, search index, etc.)
      const systemFilePatterns = [
        "syncspace.db",
        "syncspace.db-shm",
        "syncspace.db-wal",
        "search_index",
        ".tmp",
        ".lock",
        ".git",
      ];

      const filteredData = data.filter((file) => {
        return !systemFilePatterns.some(
          (pattern) =>
            file.name.includes(pattern) || file.path?.includes(pattern)
        );
      });

      console.log("✅ Filtered files:", filteredData.length);
      files.set(filteredData);
    } catch (err) {
      console.error("❌ Failed to load files:", err);

      // Specific error handling based on status code
      const errorMessage = err.message || "";
      if (
        errorMessage.includes("401") ||
        errorMessage.includes("Unauthorized")
      ) {
        errorToast("Session expired. Please login again.");
        // Auth redirect is handled by api.js handleResponse
      } else if (
        errorMessage.includes("403") ||
        errorMessage.includes("Forbidden")
      ) {
        errorToast("Permission denied. You don't have access to this folder.");
      } else if (
        errorMessage.includes("404") ||
        errorMessage.includes("Not Found")
      ) {
        errorToast("Folder not found. It may have been deleted.");
        // Navigate to root
        currentPath.set("/");
      } else if (
        errorMessage.includes("500") ||
        errorMessage.includes("Internal Server Error")
      ) {
        errorToast("Server error. Please try again later.");
      } else {
        errorToast(errorMessage || "Failed to load files");
      }

      // Set empty array on error so UI doesn't stay in loading state
      files.set([]);
    } finally {
      loading = false;
      console.log("📊 Loading complete, loading state:", loading);
    }
  }

  function navigateToFolder(folder) {
    currentPath.set($currentPath + folder.name + "/");
    loadFiles();
  }

  function navigateToBreadcrumb(index) {
    const newPath = "/" + breadcrumbPath.slice(0, index + 1).join("/") + "/";
    currentPath.set(newPath);
    loadFiles();
  }

  function handleBreadcrumbNavigate(event) {
    currentPath.set(event.detail.path);
    loadFiles();
  }

  function handleFilesSelected(e) {
    const selectedFiles = Array.from(e.target.files || []);
    if (selectedFiles.length > 0) {
      uploadFiles = selectedFiles;
      showUploadModal = true;
    }
  }

  async function handleFileUpload() {
    if (!uploadFiles.length) return;

    uploading = true;
    uploadProgress.clear();
    overallProgress = 0;

    try {
      const totalFiles = uploadFiles.length;
      let completedFiles = 0;

      // Upload files with progress tracking
      for (let i = 0; i < uploadFiles.length; i++) {
        const file = uploadFiles[i];
        const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
        const fullPath = backendPath
          ? `${backendPath}/${file.name}`
          : file.name;

        console.log(`📤 Starting upload ${i + 1}/${totalFiles}: ${file.name}`);

        // Track progress for this specific file
        await api.files.uploadWithProgress(
          fullPath,
          file,
          (percent, loaded, total) => {
            // Update individual file progress
            uploadProgress.set(i, { percent, loaded, total });

            // Calculate overall progress
            let totalLoaded = 0;
            let totalSize = 0;

            for (let j = 0; j < uploadFiles.length; j++) {
              if (j < completedFiles) {
                // Completed files count as 100%
                totalLoaded += uploadFiles[j].size;
                totalSize += uploadFiles[j].size;
              } else if (j === i) {
                // Current file uses actual progress
                totalLoaded += loaded;
                totalSize += total;
              } else {
                // Future files count as 0%
                totalSize += uploadFiles[j].size;
              }
            }

            overallProgress =
              totalSize > 0 ? (totalLoaded / totalSize) * 100 : 0;
            console.log(
              `📊 File ${i + 1} progress: ${percent.toFixed(1)}%, Overall: ${overallProgress.toFixed(1)}%`
            );

            // Force reactive update
            uploadProgress = new Map(uploadProgress);
          }
        );

        completedFiles++;
        console.log(`✅ Completed upload ${i + 1}/${totalFiles}: ${file.name}`);
      }

      success(`Uploaded ${uploadFiles.length} file(s) successfully`);
      await loadFiles();
      showUploadModal = false;
      uploadFiles = [];
      uploadProgress.clear();
      overallProgress = 0;
    } catch (err) {
      errorToast(err.message || "Upload failed");
    } finally {
      uploading = false;
    }
  }

  async function handleCreateFolder() {
    if (!newFolderName.trim()) return;

    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
      const fullPath = backendPath
        ? `${backendPath}/${newFolderName}`
        : newFolderName;
      await api.files.createDir(fullPath);
      success("Folder created");
      await loadFiles();
      showNewFolderModal = false;
      newFolderName = "";
    } catch (err) {
      errorToast(err.message || "Failed to create folder");
    }
  }

  async function handleRename() {
    if (!fileToRename || !newFileName.trim()) return;

    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
      const oldPath = backendPath
        ? `${backendPath}/${fileToRename.name}`
        : fileToRename.name;
      const newPath = backendPath
        ? `${backendPath}/${newFileName}`
        : newFileName;
      await api.files.rename(oldPath, newPath);
      success("Renamed successfully");
      await loadFiles();
      showRenameModal = false;
      fileToRename = null;
      newFileName = "";
    } catch (err) {
      errorToast(err.message || "Rename failed");
    }
  }

  async function handleDelete() {
    if (!fileToDelete) return;

    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
      const fullPath = backendPath
        ? `${backendPath}/${fileToDelete.name}`
        : fileToDelete.name;
      await api.files.delete(fullPath);
      success("Deleted successfully");
      await loadFiles();
      showDeleteModal = false;
      fileToDelete = null;
    } catch (err) {
      errorToast(err.message || "Delete failed");
    }
  }

  async function handleDownload(file) {
    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
      const fullPath = backendPath ? `${backendPath}/${file.name}` : file.name;
      const blob = await api.files.download(fullPath);
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = file.name;
      a.click();
      URL.revokeObjectURL(url);
      success("Download started");
    } catch (err) {
      errorToast(err.message || "Download failed");
    }
  }

  async function handleMove() {
    if (!fileToMove) return;

    try {
      const sourcePath = fileToMove.path || fileToMove.name;
      const targetPath = moveTargetPath
        ? `${moveTargetPath}/${fileToMove.name}`
        : fileToMove.name;

      await api.files.move(sourcePath, targetPath);
      success(`Moved ${fileToMove.name} successfully`);
      await loadFiles();
      showMoveModal = false;
      fileToMove = null;
      moveTargetPath = "";
    } catch (err) {
      errorToast(err.message || "Move failed");
    }
  }

  async function handleCopy() {
    if (!fileToCopy) return;

    try {
      const sourcePath = fileToCopy.path || fileToCopy.name;
      const targetPath = copyTargetPath
        ? `${copyTargetPath}/${fileToCopy.name}`
        : fileToCopy.name;

      await api.files.copy(sourcePath, targetPath);
      success(`Copied ${fileToCopy.name} successfully`);
      await loadFiles();
      showCopyModal = false;
      fileToCopy = null;
      copyTargetPath = "";
    } catch (err) {
      errorToast(err.message || "Copy failed");
    }
  }

  function openRenameModal(file) {
    fileToRename = file;
    newFileName = file.name;
    showRenameModal = true;
  }

  // ===== MULTI-SELECT FUNCTIONS =====

  function toggleFileSelection(file, index, event) {
    const isCtrlPressed = event?.ctrlKey || event?.metaKey;
    const isShiftPressed = event?.shiftKey;

    if (isShiftPressed && lastSelectedIndex >= 0) {
      // Shift+Click: Range select
      const start = Math.min(lastSelectedIndex, index);
      const end = Math.max(lastSelectedIndex, index);
      const rangeFiles = sortedFiles.slice(start, end + 1);

      rangeFiles.forEach((f) => {
        if (!selectedFiles.find((sf) => sf.name === f.name)) {
          selectedFiles.push(f);
        }
      });
      selectedFiles = [...selectedFiles];
    } else if (isCtrlPressed) {
      // Ctrl+Click: Toggle individual
      const existingIndex = selectedFiles.findIndex(
        (f) => f.name === file.name
      );
      if (existingIndex >= 0) {
        selectedFiles.splice(existingIndex, 1);
      } else {
        selectedFiles.push(file);
      }
      selectedFiles = [...selectedFiles];
      lastSelectedIndex = index;
    } else {
      // Regular click: Select only this file
      selectedFiles = [file];
      lastSelectedIndex = index;
    }

    selectionMode = selectedFiles.length > 0;
  }

  function selectAllFiles() {
    selectedFiles = [...sortedFiles];
    selectionMode = true;
    success(`Selected ${selectedFiles.length} files`);
  }

  function deselectAllFiles() {
    selectedFiles = [];
    selectionMode = false;
    lastSelectedIndex = -1;
  }

  function isFileSelected(file) {
    return selectedFiles.some(
      (f) => f.name === file.name && f.path === file.path
    );
  }

  async function handleBatchDelete() {
    if (selectedFiles.length === 0) return;

    const confirmed = confirm(
      `Delete ${selectedFiles.length} selected file(s)?`
    );
    if (!confirmed) return;

    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");

      for (const file of selectedFiles) {
        const fullPath = backendPath
          ? `${backendPath}/${file.name}`
          : file.name;
        await api.files.delete(fullPath);
      }

      success(`Deleted ${selectedFiles.length} file(s) successfully`);
      deselectAllFiles();
      await loadFiles();
    } catch (err) {
      errorToast(err.message || "Batch delete failed");
    }
  }

  async function handleBatchDownload() {
    if (selectedFiles.length === 0) return;

    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");

      for (const file of selectedFiles) {
        if (file.is_dir) continue; // Skip directories
        const fullPath = backendPath
          ? `${backendPath}/${file.name}`
          : file.name;
        const blob = await api.files.download(fullPath);
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = file.name;
        a.click();
        URL.revokeObjectURL(url);
        await new Promise((resolve) => setTimeout(resolve, 100)); // Small delay between downloads
      }

      success(`Started downloading ${selectedFiles.length} file(s)`);
    } catch (err) {
      errorToast(err.message || "Batch download failed");
    }
  }

  function openDeleteModal(file) {
    fileToDelete = file;
    showDeleteModal = true;
  }

  function openShareModal(file) {
    fileToShare = file;
    showShareModal = true;
  }

  function openVersionHistoryModal(file) {
    fileToViewVersions = file;
    showVersionHistoryModal = true;
  }

  function handleDragOver(e) {
    e.preventDefault();
    e.stopPropagation();
    dragOver = true;
  }

  function handleDragLeave(e) {
    e.preventDefault();
    e.stopPropagation();
    // Only hide if leaving the main container
    if (e.target.classList.contains("files-view")) {
      dragOver = false;
    }
  }

  function handleDrop(e) {
    e.preventDefault();
    e.stopPropagation();
    dragOver = false;

    const droppedFiles = Array.from(e.dataTransfer.files || []);
    if (droppedFiles.length > 0) {
      uploadFiles = droppedFiles;
      showUploadModal = true;
    }
  }

  function formatFileSize(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatDate(dateString) {
    return new Date(dateString).toLocaleDateString();
  }

  // Context Menu Handlers
  function handleContextMenu(e, file) {
    e.preventDefault();
    contextMenuFile = file;
    contextMenuX = e.pageX;
    contextMenuY = e.pageY;
    contextMenuVisible = true;
  }

  function getContextMenuItems() {
    if (!contextMenuFile) return [];

    const items = [];

    if (!contextMenuFile.is_directory) {
      items.push({ label: "Download", icon: "download", shortcut: "Ctrl+D" });
      items.push({ label: "Preview", icon: "eye", shortcut: "Space" });
    }

    items.push({ label: "Rename", icon: "pencil", shortcut: "F2" });

    if (!contextMenuFile.is_directory) {
      const isFavorited = Array.from($favorites.values()).some(
        (fav) =>
          fav.item_path === contextMenuFile.name ||
          fav.item_id === contextMenuFile.id
      );
      items.push({
        label: isFavorited ? "Remove from Favorites" : "Add to Favorites",
        icon: "star",
      });
    }

    items.push({ divider: true });
    items.push({ label: "Copy", icon: "copy", shortcut: "Ctrl+C" });
    items.push({ label: "Move", icon: "arrows-move" });
    items.push({ label: "Share", icon: "share", shortcut: "Ctrl+S" });

    items.push({ divider: true });
    items.push({ label: "Details", icon: "info-circle" });

    if (!contextMenuFile.is_directory) {
      items.push({ label: "Version History", icon: "clock-history" });
    } else {
      items.push({ label: "Set Folder Color", icon: "palette" });
    }

    items.push({ divider: true });
    items.push({ label: "Delete", icon: "trash", shortcut: "Del" });

    return items;
  }

  async function handleContextAction(detail) {
    const { label } = detail;
    if (!contextMenuFile) return;

    contextMenuVisible = false;

    switch (label) {
      case "Download":
        await handleDownload(contextMenuFile);
        break;
      case "Preview":
        openFilePreview(contextMenuFile);
        break;
      case "Rename":
        openRenameModal(contextMenuFile);
        break;
      case "Add to Favorites":
        favorites.add(contextMenuFile.name);
        success("Added to favorites");
        break;
      case "Remove from Favorites":
        favorites.remove(contextMenuFile.name);
        success("Removed from favorites");
        break;
      case "Copy":
        fileToCopy = contextMenuFile;
        copyTargetPath = $currentPath;
        showCopyModal = true;
        break;
      case "Move":
        fileToMove = contextMenuFile;
        moveTargetPath = $currentPath;
        showMoveModal = true;
        break;
      case "Details":
        // TODO: Implement details panel (Item #8)
        success("Details panel coming soon");
        break;
      case "Set Folder Color":
        folderForColorPicker = contextMenuFile;
        showFolderColorPicker = true;
        break;
      case "Share":
        openShareModal(contextMenuFile);
        break;
      case "Version History":
        openVersionHistoryModal(contextMenuFile);
        break;
      case "Delete":
        openDeleteModal(contextMenuFile);
        break;
    }

    contextMenuFile = null;
  }

  function handleClickOutside() {
    contextMenuVisible = false;
  }

  // File Preview Function
  function openFilePreview(file) {
    if (file.is_dir) return; // Only preview files, not folders
    fileToPreview = file;
    showFilePreview = true;
  }

  // Get current file index for preview navigation
  let currentFileIndex = $derived(filteredFiles.findIndex(f => f.name === fileToPreview?.name));

  // Advanced Search Functions
  async function handleAdvancedSearch(event) {
    const { query, filters, sortBy, sortOrder } = event.detail;
    console.log("🔍 Advanced search:", { query, filters, sortBy, sortOrder });

    loading = true;
    isSearchActive = true;

    try {
      // Use the search API with advanced parameters
      const results = await api.search.query(query, 100, true);

      // Apply client-side filters (backend could be enhanced to support these)
      let filteredResults = results;

      // File type filter
      if (filters.fileType !== "all") {
        filteredResults = filteredResults.filter((file) => {
          const ext = file.name.split(".").pop()?.toLowerCase() || "";
          switch (filters.fileType) {
            case "image":
              return [
                "jpg",
                "jpeg",
                "png",
                "gif",
                "webp",
                "bmp",
                "svg",
              ].includes(ext);
            case "video":
              return ["mp4", "avi", "mkv", "mov", "webm", "flv"].includes(ext);
            case "audio":
              return ["mp3", "wav", "flac", "aac", "ogg"].includes(ext);
            case "document":
              return ["doc", "docx", "odt", "rtf"].includes(ext);
            case "pdf":
              return ext === "pdf";
            case "archive":
              return ["zip", "rar", "7z", "tar", "gz"].includes(ext);
            case "code":
              return [
                "js",
                "ts",
                "py",
                "java",
                "cpp",
                "c",
                "html",
                "css",
                "rs",
              ].includes(ext);
            case "text":
              return ["txt", "md", "log"].includes(ext);
            default:
              return true;
          }
        });
      }

      // Size filters (assuming size is in bytes)
      if (filters.sizeMin) {
        const minBytes = parseFloat(filters.sizeMin) * 1024 * 1024; // Convert MB to bytes
        filteredResults = filteredResults.filter(
          (file) => !file.is_dir && file.size >= minBytes
        );
      }

      if (filters.sizeMax) {
        const maxBytes = parseFloat(filters.sizeMax) * 1024 * 1024; // Convert MB to bytes
        filteredResults = filteredResults.filter(
          (file) => !file.is_dir && file.size <= maxBytes
        );
      }

      // Date filters (would need backend support for proper implementation)
      if (filters.dateFrom || filters.dateTo) {
        console.log("Date filters not yet implemented in backend");
      }

      // Sort results
      filteredResults.sort((a, b) => {
        let aVal, bVal;

        switch (sortBy) {
          case "name":
            aVal = a.name.toLowerCase();
            bVal = b.name.toLowerCase();
            break;
          case "size":
            aVal = a.size || 0;
            bVal = b.size || 0;
            break;
          case "type":
            aVal = a.is_dir ? "folder" : a.name.split(".").pop() || "";
            bVal = b.is_dir ? "folder" : b.name.split(".").pop() || "";
            break;
          case "date":
            // Would need modification_date from backend
            aVal = a.modification_date || "";
            bVal = b.modification_date || "";
            break;
          default:
            aVal = a.name;
            bVal = b.name;
        }

        if (sortOrder === "desc") {
          return aVal > bVal ? -1 : aVal < bVal ? 1 : 0;
        } else {
          return aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
        }
      });

      searchResults = filteredResults;
      showAdvancedSearchModal = false;

      success(`Found ${filteredResults.length} files matching "${query}"`);
    } catch (err) {
      errorToast(err.message || "Search failed");
      isSearchActive = false;
      searchResults = [];
    } finally {
      loading = false;
    }
  }

  function clearSearch() {
    isSearchActive = false;
    searchResults = [];
    searchQuery = "";
  }
</script>

<svelte:window onclick={handleClickOutside} />

<PageWrapper gradient fullHeight={false}>
  <!-- Animated Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <div
    class="relative z-10"
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    role="main"
  >
    <!-- Toolbar -->
    <ModernCard variant="glass" class="mb-6">
      {#snippet children()}
        <div class="flex flex-wrap items-center gap-4">
          <!-- Left: Actions -->
          <div class="flex gap-2 flex-1">
            <ModernButton
              variant="gradient"
              icon="upload"
              onclick={() => uploadInput?.click()}
            >
              Upload
            </ModernButton>
            <ModernButton
              variant="primary"
              icon="folder-plus"
              onclick={() => (showNewFolderModal = true)}
            >
              New Folder
            </ModernButton>
            <ModernButton
              variant="ghost"
              icon="arrow-clockwise"
              onclick={() => loadFiles()}
            >
              Refresh
            </ModernButton>
          </div>

          <!-- Right: Search and View Mode -->
          <div class="flex items-center gap-3">
            <!-- Advanced Search / Filter Button -->
            <ModernButton
              variant="ghost"
              size="sm"
              icon="funnel"
              onclick={() => (showAdvancedSearchModal = true)}
            >
              Search & Filter
            </ModernButton>

            <!-- Clear Search Results -->
            {#if isSearchActive}
              <ModernButton
                variant="secondary"
                size="sm"
                icon="x-circle"
                onclick={clearSearch}
              >
                Clear Search
              </ModernButton>
            {/if}
          </div>

          <!-- Sort Dropdown -->
          <div class="dropdown dropdown-end">
            <ModernButton
              variant="ghost"
              size="sm"
              icon="sort-down"
              tabindex="0"
            >
              <span class="hidden sm:inline">Sort</span>
            </ModernButton>
            <ul
              role="menu"
              class="dropdown-content z-[1] menu p-2 shadow glass-card rounded-box w-52 mt-1"
            >
              <li class="menu-title"><span>Sort By</span></li>
              <li>
                <button
                  onclick={() => {
                    sortBy = "name";
                    sortOrder = sortOrder === "asc" && sortBy === "name" ? "desc" : "asc";
                  }}
                >
                  <i class="bi bi-sort-alpha-{sortBy === 'name' && sortOrder === 'asc' ? 'down' : 'up'}"></i>
                  Name {sortBy === "name" ? sortOrder === "asc" ? "(A-Z)" : "(Z-A)" : ""}
                </button>
              </li>
              <li>
                <button
                  onclick={() => {
                    sortBy = "modified";
                    sortOrder = sortOrder === "asc" && sortBy === "modified" ? "desc" : "asc";
                  }}
                >
                  <i class="bi bi-clock-history"></i>
                  Modified {sortBy === "modified" ? sortOrder === "desc" ? "(Newest)" : "(Oldest)" : ""}
                </button>
              </li>
              <li>
                <button
                  onclick={() => {
                    sortBy = "size";
                    sortOrder = sortOrder === "asc" && sortBy === "size" ? "desc" : "asc";
                  }}
                >
                  <i class="bi bi-file-earmark-bar-graph"></i>
                  Size {sortBy === "size" ? sortOrder === "desc" ? "(Largest)" : "(Smallest)" : ""}
                </button>
              </li>
              <li>
                <button
                  onclick={() => {
                    sortBy = "type";
                    sortOrder = sortOrder === "asc" && sortBy === "type" ? "desc" : "asc";
                  }}
                >
                  <i class="bi bi-file-earmark-code"></i>
                  Type
                </button>
              </li>
              <div class="divider my-1"></div>
              <li class="menu-title"><span>Filter</span></li>
              <li>
                <label class="label cursor-pointer justify-start gap-2">
                  <input
                    type="checkbox"
                    class="toggle toggle-sm toggle-success"
                    bind:checked={showFoldersOnly}
                  />
                  <span
                    class={showFoldersOnly ? "font-semibold text-success" : ""}
                  >
                    Folders Only {showFoldersOnly ? "(ON)" : ""}
                  </span>
                </label>
              </li>
            </ul>
          </div>

          <!-- View Mode -->
          <div class="join">
            <ModernButton
              variant={viewMode === "grid" ? "primary" : "ghost"}
              size="sm"
              icon="grid-3x3"
              class="join-item"
              onclick={() => (viewMode = "grid")}
            />
            <ModernButton
              variant={viewMode === "list" ? "primary" : "ghost"}
              size="sm"
              icon="list-ul"
              class="join-item"
              onclick={() => (viewMode = "list")}
            />
          </div>
        </div>
      {/snippet}
    </ModernCard>

    <!-- Batch Action Toolbar -->
    {#if selectionMode && selectedFiles.length > 0}
      <div class="glass-card border-l-4 border-primary p-4 mb-6 animate-slide-up">
        <div class="flex items-center gap-3">
          <i class="bi bi-check-square text-2xl text-primary"></i>
          <div class="flex-1">
            <h3 class="font-bold text-lg">{selectedFiles.length} file(s) selected</h3>
            <div class="text-sm opacity-70">Ctrl+Click to select more, Shift+Click for range</div>
          </div>
          <div class="flex gap-2">
            <ModernButton variant="success" size="sm" icon="check-all" onclick={selectAllFiles}>
              Select All
            </ModernButton>
            <ModernButton variant="ghost" size="sm" icon="x-circle" onclick={deselectAllFiles}>
              Clear
            </ModernButton>
            <div class="divider divider-horizontal mx-2"></div>
            <ModernButton variant="primary" size="sm" icon="download" onclick={handleBatchDownload}>
              Download
            </ModernButton>
            <ModernButton variant="danger" size="sm" icon="trash3" onclick={handleBatchDelete}>
              Delete
            </ModernButton>
          </div>
        </div>
      </div>
    {/if}

  <!-- Breadcrumb Navigation -->
  {#if !isSearchActive}
    <Breadcrumb
      path={$currentPath}
      maxVisibleSegments={4}
      on:navigate={handleBreadcrumbNavigate}
    />
  {/if}

  <!-- Search Results Header -->
  {#if isSearchActive}
    <div class="mb-4 p-4 bg-info/10 border border-info/30 rounded-xl">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <i class="bi bi-search text-info text-lg"></i>
          <span class="font-medium">Search Results</span>
          <div class="badge badge-info">{filteredFiles.length} files</div>
        </div>
        <button class="btn btn-sm btn-ghost" onclick={clearSearch}>
          <i class="bi bi-x"></i>
          Close
        </button>
      </div>
    </div>
  {/if}

  <!-- Drag & Drop Overlay -->
  {#if dragOver}
    <div class="drop-zone-overlay">
      <div class="drop-content">
        <div class="drop-animation mb-6">
          <i class="bi bi-cloud-upload text-8xl text-primary animate-bounce"
          ></i>
          <div
            class="absolute inset-0 rounded-full border-4 border-primary border-dashed animate-ping opacity-50"
          ></div>
        </div>
        <h3 class="text-3xl font-bold mb-2">Drop files here</h3>
        <p class="text-lg opacity-70">Release to upload to current folder</p>
        <div class="flex gap-4 mt-6 text-sm opacity-60">
          <div class="flex items-center gap-2">
            <i class="bi bi-file-earmark"></i>
            <span>Documents</span>
          </div>
          <div class="flex items-center gap-2">
            <i class="bi bi-image"></i>
            <span>Images</span>
          </div>
          <div class="flex items-center gap-2">
            <i class="bi bi-file-zip"></i>
            <span>Archives</span>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Files Display -->
  {#if loading}
    <div class="flex flex-col justify-center items-center h-64 gap-4">
      <span class="loading loading-spinner loading-lg text-primary"></span>
      <p class="text-sm opacity-70">Loading files...</p>
    </div>
  {:else if sortedFiles.length === 0}
    <div class="hero min-h-[500px] bg-gray-50 dark:bg-gray-800 rounded-2xl m-4">
      <div class="hero-content text-center">
        <div class="max-w-lg">
          <div class="mb-8">
            <div class="relative inline-block">
              <i class="bi bi-folder2-open text-9xl text-primary/20"></i>
              <i
                class="bi bi-inbox absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-6xl text-primary"
              ></i>
            </div>
          </div>
          <h1 class="text-4xl font-bold mb-4 text-gray-900 dark:text-white">This folder is empty</h1>
          <p class="text-lg mb-8 text-gray-600 dark:text-gray-400">
            Get started by uploading files or creating a new folder
          </p>
          <div class="flex flex-col sm:flex-row gap-3 justify-center">
            <button
              class="btn btn-primary gap-2"
              onclick={() => uploadInput?.click()}
            >
              <i class="bi bi-upload"></i>
              Upload Files
            </button>
            <button
              class="btn btn-outline gap-2"
              onclick={() => (showNewFolderModal = true)}
            >
              <i class="bi bi-folder-plus"></i>
              New Folder
            </button>
          </div>
          <div class="divider mt-8">OR</div>
          <p class="text-sm opacity-60">
            Drag and drop files anywhere to upload
          </p>
        </div>
      </div>
    </div>
  {:else if viewMode === "grid"}
    <!-- Grid View -->
    <div
      class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-4 p-4"
    >
      {#each sortedFiles as file, index}
        <div
          class="card bg-white dark:bg-gray-800 border-2 transition-all duration-300 cursor-pointer group relative overflow-hidden"
          class:border-gray-200={!isFileSelected(file)}
          class:dark:border-gray-700={!isFileSelected(file)}
          class:border-primary={isFileSelected(file)}
          class:hover:border-primary={!isFileSelected(file)}
          class:hover:shadow-2xl={true}
          class:hover:scale-105={!isFileSelected(file)}
          style={isFileSelected(file)
            ? "background-color: hsl(var(--p) / 0.1); border-color: hsl(var(--p));"
            : ""}
          draggable="true"
          ondragstart={(e) => {
            e.dataTransfer.effectAllowed = "move";
            e.dataTransfer.setData("text/plain", JSON.stringify(file));
          }}
          ondragover={(e) => {
            if (file.is_dir) {
              e.preventDefault();
              e.currentTarget.classList.add("drag-over");
            }
          }}
          ondragleave={(e) => {
            e.currentTarget.classList.remove("drag-over");
          }}
          ondrop={async (e) => {
            e.preventDefault();
            e.currentTarget.classList.remove("drag-over");
            if (!file.is_dir) return;

            try {
              const draggedFile = JSON.parse(
                e.dataTransfer.getData("text/plain")
              );
              if (draggedFile.name === file.name) return; // Can't drop on itself

              const sourcePath = draggedFile.path || draggedFile.name;
              const targetPath = `${file.path || file.name}/${draggedFile.name}`;

              await api.files.move(sourcePath, targetPath);
              success(`Moved ${draggedFile.name} to ${file.name}`);
              await loadFiles();
            } catch (err) {
              errorToast(`Failed to move file: ${err.message}`);
            }
          }}
          onclick={(e) => {
            // Check if Ctrl or Shift is pressed for multi-select
            if (e.ctrlKey || e.metaKey || e.shiftKey) {
              e.preventDefault();
              toggleFileSelection(file, index, e);
            } else if (!isFileSelected(file)) {
              // Regular click
              if (file.is_dir) {
                navigateToFolder(file);
              } else {
                openFilePreview(file);
              }
            }
          }}
          oncontextmenu={(e) => handleContextMenu(e, file)}
          onkeydown={(e) => {
            if (e.key === "Enter") {
              if (file.is_dir) {
                navigateToFolder(file);
              } else {
                openFilePreview(file);
              }
            }
          }}
          role="button"
          tabindex="0"
        >
          <!-- Selection Checkbox -->
          <div class="absolute top-2 left-2 z-20">
            <input
              type="checkbox"
              class="checkbox checkbox-primary checkbox-sm"
              checked={isFileSelected(file)}
              onclick={(e) => {
                e.stopPropagation();
                toggleFileSelection(file, index, e);
              }}
            />
          </div>

          <!-- File Type Badge -->
          <div class="absolute top-2 right-2 z-10">
            {#if file.is_dir}
              <span class="badge badge-warning badge-sm gap-1">
                <i class="bi bi-folder-fill"></i>
                Folder
              </span>
            {:else}
              <span class="badge badge-primary badge-sm">
                {file.name.split(".").pop()?.toUpperCase() || "FILE"}
              </span>
            {/if}
          </div>

          <!-- Favorite Star (visible on hover) -->
          <div
            class="absolute top-2 left-2 z-10 opacity-0 group-hover:opacity-100 transition-opacity"
          >
            <button
              class="btn btn-circle btn-xs btn-ghost bg-white dark:bg-slate-900/80 backdrop-blur-sm"
              onclick={async (e) => {
                e.stopPropagation();
                const itemId = file.path || file.name;
                const isFav = $favorites.has(itemId);
                try {
                  if (isFav) {
                    await favorites.remove(itemId);
                    success(`${file.name} removed from favorites`);
                  } else {
                    await favorites.add(
                      itemId,
                      file.is_dir ? "folder" : "file"
                    );
                    success(`${file.name} added to favorites`);
                  }
                } catch (err) {
                  errorToast(`Failed to update favorites: ${err.message}`);
                }
              }}
              title={$favorites.has(file.path || file.name)
                ? "Remove from favorites"
                : "Add to favorites"}
            >
              <i
                class="bi {$favorites.has(file.path || file.name)
                  ? 'bi-star-fill'
                  : 'bi-star'} text-warning"
              ></i>
            </button>
          </div>

          <div class="card-body p-4 items-center text-center">
            <!-- File Thumbnail or Icon with background -->
            <div
              class="relative w-full aspect-square mb-3 rounded-lg bg-slate-50 dark:bg-slate-800 flex items-center justify-center overflow-hidden"
            >
              {#if file.is_dir}
                <div class="text-6xl text-warning drop-shadow-lg">
                  <i class="bi bi-folder-fill"></i>
                </div>
              {:else}
                <!-- Large icon for files (same size as folders) -->
                <div
                  class="text-6xl drop-shadow-lg {getFileIconColor(file.name)}"
                >
                  <i class="bi bi-{getFileIcon(file.name)}"></i>
                </div>
              {/if}
            </div>

            <!-- File Name with better truncation -->
            <h3
              class="text-sm font-semibold truncate w-full mb-1 group-hover:text-primary transition-colors"
              title={file.name}
            >
              {file.name}
            </h3>

            <!-- File Info -->
            <div
              class="flex items-center justify-between w-full text-xs opacity-70"
            >
              <span>{file.is_dir ? "Folder" : formatFileSize(file.size)}</span>
              {#if !file.is_dir && file.modified_at}
                <span class="hidden sm:inline"
                  >{new Date(file.modified_at).toLocaleDateString()}</span
                >
              {/if}
            </div>
            <div
              class="card-actions justify-center mt-2 opacity-0 group-hover:opacity-100 transition-opacity"
            >
              {#if !file.is_dir}
                <button
                  class="btn btn-ghost btn-xs btn-circle"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleDownload(file);
                  }}
                  aria-label="Download"
                >
                  <i class="bi bi-download"></i>
                </button>
              {/if}
              <button
                class="btn btn-ghost btn-xs btn-circle"
                onclick={(e) => {
                  e.stopPropagation();
                  openRenameModal(file);
                }}
                aria-label="Rename"
              >
                <i class="bi bi-pencil"></i>
              </button>
              <button
                class="btn btn-ghost btn-xs btn-circle text-error"
                onclick={(e) => {
                  e.stopPropagation();
                  openDeleteModal(file);
                }}
                aria-label="Delete"
              >
                <i class="bi bi-trash"></i>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <!-- List View -->
    <div class="overflow-x-auto p-4">
      <table class="table table-zebra table-pin-rows">
        <thead class="bg-gray-50 dark:bg-gray-800">
          <tr>
            <th class="bg-gray-50 dark:bg-gray-800">
              <button
                class="flex items-center gap-1 hover:text-primary transition-colors"
                onclick={() => {
                  sortBy = "name";
                  sortOrder =
                    sortOrder === "asc" && sortBy === "name" ? "desc" : "asc";
                }}
              >
                Name
                {#if sortBy === "name"}
                  <i
                    class="bi bi-chevron-{sortOrder === 'asc'
                      ? 'up'
                      : 'down'} text-xs"
                  ></i>
                {/if}
              </button>
            </th>
            <th class="bg-slate-50 dark:bg-slate-800">
              <button
                class="flex items-center gap-1 hover:text-primary transition-colors"
                onclick={() => {
                  sortBy = "type";
                  sortOrder =
                    sortOrder === "asc" && sortBy === "type" ? "desc" : "asc";
                }}
              >
                Type
                {#if sortBy === "type"}
                  <i
                    class="bi bi-chevron-{sortOrder === 'asc'
                      ? 'up'
                      : 'down'} text-xs"
                  ></i>
                {/if}
              </button>
            </th>
            <th class="bg-slate-50 dark:bg-slate-800">
              <button
                class="flex items-center gap-1 hover:text-primary transition-colors"
                onclick={() => {
                  sortBy = "size";
                  sortOrder =
                    sortOrder === "asc" && sortBy === "size" ? "desc" : "asc";
                }}
              >
                Size
                {#if sortBy === "size"}
                  <i
                    class="bi bi-chevron-{sortOrder === 'asc'
                      ? 'up'
                      : 'down'} text-xs"
                  ></i>
                {/if}
              </button>
            </th>
            <th class="bg-slate-50 dark:bg-slate-800">
              <button
                class="flex items-center gap-1 hover:text-primary transition-colors"
                onclick={() => {
                  sortBy = "modified";
                  sortOrder =
                    sortOrder === "asc" && sortBy === "modified"
                      ? "desc"
                      : "asc";
                }}
              >
                Modified
                {#if sortBy === "modified"}
                  <i
                    class="bi bi-chevron-{sortOrder === 'asc'
                      ? 'up'
                      : 'down'} text-xs"
                  ></i>
                {/if}
              </button>
            </th>
            <th class="text-right bg-slate-50 dark:bg-slate-800">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each sortedFiles as file}
            <tr
              class="hover:bg-slate-50 dark:bg-slate-800 cursor-pointer transition-colors group"
              draggable="true"
              ondragstart={(e) => {
                e.dataTransfer.effectAllowed = "move";
                e.dataTransfer.setData("text/plain", JSON.stringify(file));
              }}
              ondragover={(e) => {
                if (file.is_dir) {
                  e.preventDefault();
                  e.currentTarget.classList.add("bg-primary/10");
                }
              }}
              ondragleave={(e) => {
                e.currentTarget.classList.remove("bg-primary/10");
              }}
              ondrop={async (e) => {
                e.preventDefault();
                e.currentTarget.classList.remove("bg-primary/10");
                if (!file.is_dir) return;

                try {
                  const draggedFile = JSON.parse(
                    e.dataTransfer.getData("text/plain")
                  );
                  if (draggedFile.name === file.name) return;

                  const sourcePath = draggedFile.path || draggedFile.name;
                  const targetPath = `${file.path || file.name}/${draggedFile.name}`;

                  await api.files.move(sourcePath, targetPath);
                  success(`Moved ${draggedFile.name} to ${file.name}`);
                  await loadFiles();
                } catch (err) {
                  errorToast(`Failed to move file: ${err.message}`);
                }
              }}
              onclick={() => {
                if (file.is_dir) {
                  navigateToFolder(file);
                } else {
                  openFilePreview(file);
                }
              }}
              oncontextmenu={(e) => handleContextMenu(e, file)}
            >
              <td>
                <div class="flex items-center gap-3">
                  <!-- Thumbnail or Icon -->
                  {#if file.is_dir}
                    <div class="text-2xl text-warning">
                      <i class="bi bi-folder-fill"></i>
                    </div>
                  {:else}
                    <FileThumbnail {file} size="md" />
                  {/if}
                  <div>
                    <div
                      class="font-semibold group-hover:text-primary transition-colors"
                    >
                      {file.name}
                    </div>
                    {#if file.is_dir}
                      <div class="text-xs opacity-60">Folder</div>
                    {/if}
                  </div>
                </div>
              </td>
              <td>
                {#if file.is_dir}
                  <span class="badge badge-warning badge-sm gap-1">
                    <i class="bi bi-folder-fill"></i>
                    Folder
                  </span>
                {:else}
                  <span class="badge badge-primary badge-sm">
                    {(file.name.split(".").pop() || "File").toUpperCase()}
                  </span>
                {/if}
              </td>
              <td>
                <span class="badge badge-ghost">
                  {file.is_dir
                    ? "Folder"
                    : (file.name.split(".").pop() || "File").toUpperCase()}
                </span>
              </td>
              <td>
                <span class="font-mono text-sm">
                  {file.is_dir ? "—" : formatFileSize(file.size)}
                </span>
              </td>
              <td>
                <span class="text-sm opacity-70">
                  {file.modified_at ? formatDate(file.modified_at) : "—"}
                </span>
              </td>
              <td>
                <div
                  class="flex gap-1 justify-end opacity-0 group-hover:opacity-100 transition-opacity"
                >
                  {#if !file.is_dir}
                    <button
                      class="btn btn-ghost btn-sm btn-square hover:btn-primary"
                      onclick={(e) => {
                        e.stopPropagation();
                        handleDownload(file);
                      }}
                      aria-label="Download"
                      title="Download"
                    >
                      <i class="bi bi-download"></i>
                    </button>
                  {/if}
                  <button
                    class="btn btn-ghost btn-sm btn-square hover:btn-info"
                    onclick={(e) => {
                      e.stopPropagation();
                      openRenameModal(file);
                    }}
                    aria-label="Rename"
                    title="Rename"
                  >
                    <i class="bi bi-pencil"></i>
                  </button>
                  <button
                    class="btn btn-ghost btn-sm btn-square hover:btn-error"
                    onclick={(e) => {
                      e.stopPropagation();
                      openDeleteModal(file);
                    }}
                    aria-label="Delete"
                    title="Delete"
                  >
                    <i class="bi bi-trash"></i>
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<!-- Hidden File Input -->
<input
  type="file"
  multiple
  bind:this={uploadInput}
  onchange={handleFilesSelected}
  class="hidden"
/>

<!-- Material 3 Upload Modal -->
<Modal
  visible={showUploadModal}
  title="Upload Files"
  icon="cloud-upload"
  size="md"
  variant="success"
  on:close={() => (showUploadModal = false)}
>
  <div class="space-y-4">
    <!-- Upload Summary -->
    <div class="stats stats-horizontal shadow-sm w-full">
      <div class="stat place-items-center">
        <div class="stat-title">Files Selected</div>
        <div class="stat-value text-success">{uploadFiles.length}</div>
        <div class="stat-desc">Ready to upload</div>
      </div>
      <div class="stat place-items-center">
        <div class="stat-title">Total Size</div>
        <div class="stat-value text-primary text-2xl">
          {formatFileSize(uploadFiles.reduce((acc, f) => acc + f.size, 0))}
        </div>
        <div class="stat-desc">
          ~{Math.ceil(
            uploadFiles.reduce((acc, f) => acc + f.size, 0) / 1024 / 1024
          )}
          MB
        </div>
      </div>
    </div>

    <!-- File List -->
    {#if uploadFiles.length > 0}
      <div
        class="max-h-96 overflow-y-auto space-y-2 rounded-xl bg-slate-50 dark:bg-slate-800/50 p-3"
      >
        {#each uploadFiles as file, index}
          <div
            class="flex justify-between items-center p-3 bg-white dark:bg-slate-900 rounded-xl hover:shadow-md transition-all duration-200"
          >
            <div class="flex items-center gap-3 flex-1 min-w-0">
              <div class="badge badge-lg badge-primary font-mono">
                {index + 1}
              </div>
              <div
                class="w-12 h-12 rounded-xl bg-gradient-to-br from-primary/20 to-secondary/20 flex items-center justify-center"
              >
                <i class="bi {getFileIcon(file.name)} text-primary text-xl"></i>
              </div>
              <div class="flex-1 min-w-0">
                <p class="truncate font-semibold text-sm">{file.name}</p>
                <div class="flex items-center gap-2 text-xs opacity-60">
                  <span>{formatFileSize(file.size)}</span>
                  <span>•</span>
                  <span>{file.type || "Unknown"}</span>
                </div>

                <!-- Individual File Progress Bar -->
                {#if uploading && uploadProgress.has(index)}
                  {@const progress = uploadProgress.get(index)}
                  <div class="mt-2">
                    <div class="flex justify-between text-xs mb-1">
                      <span>Uploading...</span>
                      <span>{Math.round(progress.percent)}%</span>
                    </div>
                    <progress
                      class="progress progress-success w-full h-2"
                      value={progress.percent}
                      max="100"
                    ></progress>
                  </div>
                {/if}
              </div>
            </div>

            <!-- Status Badge -->
            {#if uploading && uploadProgress.has(index)}
              {@const progress = uploadProgress.get(index)}
              {#if progress.percent >= 100}
                <div class="badge badge-success">
                  <i class="bi bi-check-circle mr-1"></i>
                  Complete
                </div>
              {:else}
                <div class="badge badge-warning">
                  <i class="bi bi-upload mr-1"></i>
                  {Math.round(progress.percent)}%
                </div>
              {/if}
            {:else if uploading}
              <div class="badge badge-ghost">
                <i class="bi bi-clock mr-1"></i>
                Waiting
              </div>
            {:else}
              <div class="badge badge-success badge-outline">
                <i class="bi bi-check-circle mr-1"></i>
                Ready
              </div>
            {/if}
          </div>
        {/each}
      </div>

      <!-- Overall Upload Progress (shown when uploading) -->
      {#if uploading}
        <div
          class="space-y-3 bg-success/5 border border-success/20 rounded-xl p-4"
        >
          <div class="flex justify-between items-center">
            <div class="flex items-center gap-2">
              <i class="bi bi-cloud-upload text-success text-lg"></i>
              <span class="font-medium text-success">Uploading files...</span>
            </div>
            <span class="text-sm opacity-60"
              >{Math.round(overallProgress)}%</span
            >
          </div>

          <!-- Overall Progress Bar -->
          <progress
            class="progress progress-success w-full h-3"
            value={overallProgress}
            max="100"
          ></progress>

          <!-- Upload Stats -->
          <div class="flex justify-between text-xs opacity-70">
            <span>
              {Object.keys(uploadProgress).filter(
                (i) => uploadProgress.get(parseInt(i))?.percent >= 100
              ).length}
              of {uploadFiles.length} files completed
            </span>
            <span>Please don't close this window</span>
          </div>
        </div>
      {/if}
    {:else}
      <div class="alert alert-warning rounded-xl">
        <i class="bi bi-exclamation-triangle"></i>
        <span>No files selected. Please choose files to upload.</span>
      </div>
    {/if}

    <!-- Upload Tips -->
    <div class="alert alert-info rounded-xl">
      <i class="bi bi-lightbulb"></i>
      <div class="text-sm">
        <strong>💡 Pro Tips:</strong>
        <ul class="list-disc list-inside ml-2 mt-1 opacity-80">
          <li>Drag & drop files directly into the file list</li>
          <li>Maximum file size: 100 MB per file</li>
          <li>Supported: All file types</li>
        </ul>
      </div>
    </div>
  </div>

  <div slot="actions">
    <button
      class="btn btn-ghost rounded-xl"
      onclick={() => (showUploadModal = false)}
      disabled={uploading}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      class="btn btn-success rounded-xl gap-2"
      class:loading={uploading}
      onclick={handleFileUpload}
      disabled={uploading || uploadFiles.length === 0}
    >
      {#if uploading}
        <span class="loading loading-spinner"></span>
        Uploading...
      {:else}
        <i class="bi bi-cloud-upload-fill"></i>
        Upload {uploadFiles.length} File{uploadFiles.length !== 1 ? "s" : ""}
      {/if}
    </button>
  </div>
</Modal>

<!-- Material 3 Create Folder Modal -->
<Modal
  visible={showNewFolderModal}
  title="Create New Folder"
  icon="folder-plus"
  size="sm"
  variant="success"
  on:close={() => (showNewFolderModal = false)}
>
  <div class="space-y-4">
    <div class="form-control">
      <label class="label" for="newFolderNameInput">
        <span class="label-text font-semibold text-base-content"
          >Folder Name</span
        >
      </label>
      <input
        id="newFolderNameInput"
        type="text"
        bind:value={newFolderName}
        placeholder="e.g., Documents, Photos, Projects..."
        class="input input-bordered rounded-xl focus:ring-2 focus:ring-success/50 text-base-content"
        onkeypress={(e) => e.key === "Enter" && handleCreateFolder()}
      />
      <div class="label">
        <span class="label-text-alt text-xs text-base-content/60">
          💡 Choose a descriptive name for easy organization
        </span>
      </div>
    </div>

    <!-- Preview -->
    <div class="alert alert-info rounded-xl bg-info/10 border-info/20">
      <i class="bi bi-info-circle text-xl text-info"></i>
      <div class="text-sm text-base-content">
        <strong>Location:</strong>
        <code class="px-2 py-1 bg-slate-50 dark:bg-slate-800 rounded ml-2 text-base-content">
          {$currentPath || "/"}{newFolderName || "new-folder"}
        </code>
      </div>
    </div>
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="btn btn-ghost rounded-xl"
      onclick={() => (showNewFolderModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      class="btn btn-success rounded-xl gap-2"
      onclick={handleCreateFolder}
      disabled={!newFolderName.trim()}
    >
      <i class="bi bi-folder-plus"></i>
      Create Folder
    </button>
  </div>
</Modal>

<!-- Material 3 Rename Modal -->
<Modal
  visible={showRenameModal}
  title={`Rename ${fileToRename?.name || ""}`}
  icon="pencil-square"
  size="sm"
  variant="primary"
  on:close={() => (showRenameModal = false)}
>
  <div class="space-y-4">
    <!-- File Preview -->
    <div class="flex items-center gap-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-xl">
      <div class="text-4xl text-primary">
        <i class="bi {getFileIcon(fileToRename?.name)}"></i>
      </div>
      <div class="flex-1">
        <div class="font-semibold text-base-content">{fileToRename?.name}</div>
        <div class="text-sm text-base-content/60">
          {fileToRename?.type === "folder" ? "Folder" : "File"} •
          {fileToRename?.size || "0 B"}
        </div>
      </div>
    </div>

    <div class="form-control">
      <label class="label" for="renameFileInput">
        <span class="label-text font-semibold text-base-content">New Name</span>
      </label>
      <input
        id="renameFileInput"
        type="text"
        bind:value={newFileName}
        placeholder="Enter new name..."
        class="input input-bordered rounded-xl focus:ring-2 focus:ring-primary/50 text-base-content"
        onkeypress={(e) => e.key === "Enter" && handleRename()}
      />
      <div class="label">
        <span class="label-text-alt text-xs text-base-content/60">
          ⚠️ File extension will be preserved automatically
        </span>
      </div>
    </div>
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="btn btn-ghost rounded-xl"
      onclick={() => (showRenameModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      class="btn btn-primary rounded-xl gap-2"
      onclick={handleRename}
      disabled={!newFileName.trim()}
    >
      <i class="bi bi-check-lg"></i>
      Rename
    </button>
  </div>
</Modal>

<!-- Material 3 Delete Confirmation Modal -->
<Modal
  visible={showDeleteModal}
  title="Delete Confirmation"
  icon="trash3"
  size="sm"
  variant="danger"
  on:close={() => (showDeleteModal = false)}
>
  <div class="space-y-4">
    <!-- Warning Banner -->
    <div class="alert alert-error rounded-xl bg-error/10 border-error/30">
      <i class="bi bi-exclamation-triangle-fill text-2xl text-error"></i>
      <div class="text-base-content">
        <h4 class="font-bold">Permanent Deletion</h4>
        <p class="text-sm">This action cannot be undone!</p>
      </div>
    </div>

    <!-- File Preview -->
    <div class="flex items-center gap-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-xl">
      <div class="text-4xl text-error/50">
        <i class="bi {getFileIcon(fileToDelete?.name)}"></i>
      </div>
      <div class="flex-1">
        <div class="font-semibold text-base-content">{fileToDelete?.name}</div>
        <div class="text-sm text-base-content/60">
          {fileToDelete?.type === "folder" ? "Folder" : "File"} •
          {fileToDelete?.size || "0 B"}
        </div>
      </div>
    </div>

    <!-- Confirmation Text -->
    <div class="text-center py-4">
      <p class="text-base-content">
        Are you absolutely sure you want to delete
        <strong class="text-error">"{fileToDelete?.name}"</strong>?
      </p>
      <p class="text-sm text-base-content/60 mt-2">
        {#if fileToDelete?.type === "folder"}
          ⚠️ All files and subfolders will be permanently deleted.
        {:else}
          This file will be permanently removed from your storage.
        {/if}
      </p>
    </div>
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="btn btn-ghost rounded-xl"
      onclick={() => (showDeleteModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button class="btn btn-error rounded-xl gap-2" onclick={handleDelete}>
      <i class="bi bi-trash3-fill"></i>
      Delete Permanently
    </button>
  </div>
</Modal>

<!-- Move File Modal -->
<Modal
  visible={showMoveModal}
  title={`Move ${fileToMove?.name || ""}`}
  icon="arrows-move"
  size="sm"
  variant="primary"
  on:close={() => (showMoveModal = false)}
>
  <div class="space-y-4">
    <div class="flex items-center gap-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-xl">
      <div class="text-4xl text-primary">
        <i class="bi {getFileIcon(fileToMove?.name)}"></i>
      </div>
      <div class="flex-1">
        <div class="font-semibold text-base-content">{fileToMove?.name}</div>
        <div class="text-sm text-base-content/60">
          {fileToMove?.is_dir ? "Folder" : "File"}
        </div>
      </div>
    </div>

    <div class="form-control">
      <label class="label" for="moveTargetPath">
        <span class="label-text font-semibold">Target Path</span>
      </label>
      <input
        id="moveTargetPath"
        type="text"
        bind:value={moveTargetPath}
        placeholder="Enter destination path..."
        class="input input-bordered rounded-xl"
        onkeypress={(e) => e.key === "Enter" && handleMove()}
      />
      <div class="label">
        <span class="label-text-alt text-xs">
          Current location: {$currentPath || "/"}
        </span>
      </div>
    </div>
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="btn btn-ghost rounded-xl"
      onclick={() => (showMoveModal = false)}
    >
      Cancel
    </button>
    <button class="btn btn-primary rounded-xl gap-2" onclick={handleMove}>
      <i class="bi bi-arrows-move"></i>
      Move
    </button>
  </div>
</Modal>

<!-- Copy File Modal -->
<Modal
  visible={showCopyModal}
  title={`Copy ${fileToCopy?.name || ""}`}
  icon="clipboard"
  size="sm"
  variant="primary"
  on:close={() => (showCopyModal = false)}
>
  <div class="space-y-4">
    <div class="flex items-center gap-3 p-4 bg-slate-50 dark:bg-slate-800 rounded-xl">
      <div class="text-4xl text-primary">
        <i class="bi {getFileIcon(fileToCopy?.name)}"></i>
      </div>
      <div class="flex-1">
        <div class="font-semibold text-base-content">{fileToCopy?.name}</div>
        <div class="text-sm text-base-content/60">
          {fileToCopy?.is_dir ? "Folder" : "File"}
        </div>
      </div>
    </div>

    <div class="form-control">
      <label class="label" for="copyTargetPath">
        <span class="label-text font-semibold">Target Path</span>
      </label>
      <input
        id="copyTargetPath"
        type="text"
        bind:value={copyTargetPath}
        placeholder="Enter destination path..."
        class="input input-bordered rounded-xl"
        onkeypress={(e) => e.key === "Enter" && handleCopy()}
      />
      <div class="label">
        <span class="label-text-alt text-xs">
          Current location: {$currentPath || "/"}
        </span>
      </div>
    </div>
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="btn btn-ghost rounded-xl"
      onclick={() => (showCopyModal = false)}
    >
      Cancel
    </button>
    <button class="btn btn-primary rounded-xl gap-2" onclick={handleCopy}>
      <i class="bi bi-clipboard"></i>
      Copy
    </button>
  </div>
</Modal>

<!-- Context Menu -->
{#if contextMenuVisible}
<ContextMenu
  x={contextMenuX}
  y={contextMenuY}
  items={getContextMenuItems()}
  onClose={() => (contextMenuVisible = false)}
/>
{/if}

<!-- Share Modal -->
<ShareModal
  file={fileToShare}
  isOpen={showShareModal}
  onClose={() => {
    showShareModal = false;
    fileToShare = null;
  }}
/>

<!-- Version History Modal -->
<VersionHistoryModal
  file={fileToViewVersions}
  isOpen={showVersionHistoryModal}
  onClose={() => {
    showVersionHistoryModal = false;
    fileToViewVersions = null;
  }}
/>

<!-- Advanced Search Modal -->
<AdvancedSearchModal
  visible={showAdvancedSearchModal}
  on:search={handleAdvancedSearch}
  on:close={() => (showAdvancedSearchModal = false)}
/>

<!-- File Preview Modal -->
<FilePreviewModal 
  bind:visible={showFilePreview} 
  bind:file={fileToPreview}
  allFiles={filteredFiles.filter(f => !f.is_dir)}
  currentIndex={currentFileIndex}
/>

<!-- Folder Color Picker Modal -->
<FolderColorPicker 
  bind:visible={showFolderColorPicker} 
  bind:folder={folderForColorPicker}
/>
</PageWrapper>

<style>
  /* Drag & Drop Zone Overlay */
  .drop-zone-overlay {
    position: fixed;
    inset: 0;
    z-index: 9999;
    background: hsl(var(--b1) / 0.95);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.2s ease-out;
    pointer-events: none;
  }

  .drop-content {
    text-align: center;
    padding: 3rem;
    border-radius: 2rem;
    border: 3px dashed hsl(var(--p));
    background: hsl(var(--b2) / 0.5);
    min-width: 400px;
    pointer-events: none;
  }

  .drop-animation {
    position: relative;
    display: inline-block;
    width: 120px;
    height: 120px;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  /* File Cards Hover Effects */
  :global(.card:hover .bi-star) {
    animation: starPulse 0.6s ease-in-out;
  }

  /* Drag and Drop Styles */
  :global(.drag-over) {
    border-color: hsl(var(--p)) !important;
    background: hsl(var(--p) / 0.1) !important;
    transform: scale(1.02) !important;
  }

  @keyframes starPulse {
    0%,
    100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.2);
    }
  }

  /* Additional Styles - Merged from duplicate block */
  .files-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }

  .drop-overlay {
    position: fixed;
    inset: 0;
    background: rgba(99, 102, 241, 0.95);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
    backdrop-filter: blur(8px);
  }

  .drop-content {
    text-align: center;
    color: white;
  }

  .card {
    transition: all 0.2s ease;
  }

  .card:hover {
    transform: translateY(-2px);
  }
</style>
