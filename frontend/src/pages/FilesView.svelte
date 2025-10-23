<script>
  import { onMount, onDestroy } from "svelte";
  import { files, currentPath, currentLang, currentView } from "../stores/ui";
  import { auth } from "../stores/auth";
  import { favorites } from "../stores/favorites";
  import { activity } from "../stores/activity";
  import { comments, tags } from "../stores/comments";
  import { t } from "../i18n";
  import { success, error as errorToast } from "../stores/toast";
  import { getFileIcon, getFileType, isPreviewable } from "../utils/fileIcons";
  import { shortcuts } from "../utils/keyboardShortcuts";
  import {
    getThumbnail,
    canGenerateThumbnail,
    cleanupCache,
    invalidateThumbnails,
  } from "../utils/thumbnailGenerator";
  import { TouchGestureHandler } from "../utils/touchGestures";
  import { RequestQueue, retryWithBackoff } from "../utils/debounce";
  import Button from "../components/ui/Button.svelte";
  import SearchBar from "../components/ui/SearchBar.svelte";
  import FilterPanel from "../components/ui/FilterPanel.svelte";
  import Dialog from "../components/ui/Dialog.svelte";
  import InputDialog from "../components/ui/InputDialog.svelte";
  import PreviewModal from "../components/ui/PreviewModal.svelte";
  import Icon from "../components/ui/Icon.svelte";
  import SkeletonLoader from "../components/ui/SkeletonLoader.svelte";
  import ContextMenu from "../components/ui/ContextMenu.svelte";
  import CommentsPanel from "../components/CommentsPanel.svelte";
  import api from "../lib/api";
  let loading = true;

  // Get current language
  const lang = localStorage.getItem("language") || "de";

  // Upload queue to prevent parallel uploads
  const uploadQueue = new RequestQueue(2); // Max 2 concurrent uploads

  /**
   * Convert UI path (e.g. "/testfolder/") to backend-compatible path (e.g. "testfolder")
   * Backend expects paths WITHOUT leading slash (relative to DATA_DIR)
   */
  function toBackendPath(uiPath) {
    if (!uiPath || uiPath === "/") return "";
    // Remove leading and trailing slashes
    return uiPath.replace(/^\/+|\/+$/g, "");
  }

  /**
   * Build full file path for backend API
   * Example: buildFilePath("/testfolder/", "file.pdf") → "testfolder/file.pdf"
   * IMPORTANT: If fileName already contains path separators, use it as-is
   */
  function buildFilePath(dirPath, fileName) {
    // If fileName already has path info (from search results etc.), use it directly
    if (fileName.includes("/")) {
      return fileName.startsWith("/") ? fileName.slice(1) : fileName;
    }

    const cleanDir = toBackendPath(dirPath);
    return cleanDir ? `${cleanDir}/${fileName}` : fileName;
  }

  // Helper: return a display-friendly filename (decode percent-encoding for UI only)

  function displayName(name) {
    try {
      return decodeURIComponent(name);
    } catch (e) {
      return name;
    }
  }

  function toggleUploadPanel() {
    showUploadPanel = !showUploadPanel;
  }

  let uploadInput;
  let dragOver = false;
  let uploading = false;
  let searchQuery = "";
  let searchResults = [];
  let isSearching = false;
  let searchTimeout;

  // Advanced filter state
  let showFilterPanel = false;
  let activeFilters = {
    fileTypes: [],
    sizeRange: { min: 0, max: Infinity },
    dateRange: { from: null, to: null },
  };
  let viewMode = localStorage.getItem("filesViewMode") || "grid"; // 'grid' or 'list'
  let uploadProgress = { current: 0, total: 0, uploading: false };

  // Auto-open drop zone for first-time users
  const hasUploadedBefore =
    localStorage.getItem("hasUploadedBefore") === "true";
  let showUploadPanel = !hasUploadedBefore; // Show by default if never uploaded

  let fileUploadProgress = {}; // Track individual file upload progress { filename: { percent, loaded, total } }

  // Multi-select state
  let multiSelectMode = false;
  let selectedFiles = new Set();

  // Drag & Drop for file moving
  let draggedFile = null;
  let dropTargetFolder = null;

  // Context menu state
  let contextMenuVisible = false;
  let contextMenuX = 0;
  let contextMenuY = 0;
  let contextMenuFile = null;

  // Touch gesture handler
  let touchGesture = new TouchGestureHandler();
  let pullToRefreshThreshold = 80; // pixels
  let pullDistance = 0;
  let isPulling = false;

  let mounted = false;
  let lastLoadPath = null; // Track last loaded path to prevent duplicate loads

  // Thumbnail cache
  let thumbnails = new Map(); // filePath -> dataUrl

  // Dialog states
  let showDeleteDialog = false;
  let showRenameDialog = false;
  let showNewFolderDialog = false;
  let showPreviewModal = false;
  let showCommentsPanel = false;
  let showMoveDialog = false;
  let showPropertiesDialog = false;
  let fileToDelete = null;
  let fileToRename = null;
  let fileToPreview = null;
  let fileForComments = null;
  let fileToMove = null;
  let fileForProperties = null;
  let availableFolders = [];

  function toggleViewMode() {
    viewMode = viewMode === "grid" ? "list" : "grid";
    localStorage.setItem("filesViewMode", viewMode);
  }

  function navigateTo(folderName) {
    currentPath.update((path) => path + folderName + "/");
    searchQuery = ""; // Clear search when navigating
    searchResults = [];
    // loadFiles() will be triggered by reactive statement
  }

  function navigateToPath(targetPath) {
    currentPath.set(targetPath);
    searchQuery = "";
    searchResults = [];
    // loadFiles() will be triggered by reactive statement
  }

  // Parse currentPath into breadcrumb parts
  $: breadcrumbs = (() => {
    if ($currentPath === "/") return [{ name: "Home", path: "/" }];

    const parts = $currentPath.split("/").filter(Boolean);
    const crumbs = [{ name: "Home", path: "/" }];

    let accumulated = "/";
    for (const part of parts) {
      accumulated += part + "/";
      crumbs.push({ name: part, path: accumulated });
    }

    return crumbs;
  })();

  // Tantivy Search with debouncing
  async function performSearch(query) {
    if (!query || query.length < 2) {
      searchResults = [];
      isSearching = false;
      return;
    }

    isSearching = true;
    try {
      const data = await api.search.query(query, 50, true);
      searchResults = data.results || [];
      console.log(`ðŸ” Found ${searchResults.length} results for "${query}"`);
    } catch (error) {
      console.error("Search failed:", error);
      searchResults = [];
    }
    isSearching = false;
  }

  function handleSearchInput(event) {
    const query = event.detail || event.target?.value || "";
    searchQuery = query;

    // Debouncing: Wait 300ms after last keystroke
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      performSearch(query);
    }, 300);
  }

  // Use search results if searching, otherwise local filtered files
  $: displayedFiles =
    searchQuery && searchQuery.length >= 2
      ? applyFilters(
          searchResults
            .filter((result) => {
              // Only show files in current directory
              const resultDir =
                result.path.substring(0, result.path.lastIndexOf("/") + 1) ||
                "/";
              return resultDir === $currentPath;
            })
            .map((result) => ({
              name: result.filename,
              path: result.path,
              type: result.file_type === "unknown" ? "file" : result.file_type,
              is_dir: false,
              size: result.size,
              modified: result.modified,
              _searchScore: result.score,
              _snippet: result.snippet,
            }))
        )
      : searchQuery
        ? applyFilters(
            $files.filter((f) =>
              f.name.toLowerCase().includes(searchQuery.toLowerCase())
            )
          )
        : applyFilters($files);

  /**
   * Apply active filters to file list
   */
  function applyFilters(files) {
    let filtered = [...files];

    // File type filter
    if (activeFilters.fileTypes.length > 0) {
      filtered = filtered.filter((file) => {
        if (file.is_dir) return true; // Always show folders

        const ext = file.name.toLowerCase().slice(file.name.lastIndexOf("."));
        const type = getFileTypeCategory(ext);
        return activeFilters.fileTypes.includes(type);
      });
    }

    // Size filter
    if (
      activeFilters.sizeRange.min > 0 ||
      activeFilters.sizeRange.max < Infinity
    ) {
      filtered = filtered.filter((file) => {
        if (file.is_dir) return true; // Always show folders
        return (
          file.size >= activeFilters.sizeRange.min &&
          file.size <= activeFilters.sizeRange.max
        );
      });
    }

    // Date filter
    if (activeFilters.dateRange.from || activeFilters.dateRange.to) {
      filtered = filtered.filter((file) => {
        const fileDate = new Date(file.modified_at || file.modified);

        if (
          activeFilters.dateRange.from &&
          fileDate < activeFilters.dateRange.from
        ) {
          return false;
        }
        if (activeFilters.dateRange.to) {
          const endOfDay = new Date(activeFilters.dateRange.to);
          endOfDay.setHours(23, 59, 59, 999);
          if (fileDate > endOfDay) return false;
        }
        return true;
      });
    }

    return filtered;
  }

  /**
   * Get file type category from extension
   */
  function getFileTypeCategory(ext) {
    const categories = {
      images: [
        ".jpg",
        ".jpeg",
        ".png",
        ".gif",
        ".bmp",
        ".webp",
        ".svg",
        ".ico",
      ],
      documents: [
        ".pdf",
        ".doc",
        ".docx",
        ".txt",
        ".md",
        ".rtf",
        ".odt",
        ".xls",
        ".xlsx",
        ".ppt",
        ".pptx",
      ],
      videos: [".mp4", ".avi", ".mov", ".mkv", ".webm", ".flv", ".wmv"],
      audio: [".mp3", ".wav", ".ogg", ".m4a", ".flac", ".aac"],
      archives: [".zip", ".rar", ".7z", ".tar", ".gz", ".bz2"],
    };

    for (const [category, extensions] of Object.entries(categories)) {
      if (extensions.includes(ext)) return category;
    }
    return "other";
  }

  onMount(async () => {
    console.log(
      `[FilesView] onMount - currentPath: "${$currentPath}", currentView: "${$currentView}"`
    );
    mounted = true;

    // Load favorites from API on mount
    await favorites.load();
    
    // Load activities from backend
    await activity.load({ limit: 50 });

    // loadFiles() will be called by reactive statement

    // Cleanup old thumbnails
    cleanupCache().catch((err) =>
      console.error("Thumbnail cleanup failed:", err)
    );

    // Setup keyboard shortcuts
    shortcuts.init();

    // Ctrl+A - Select all files
    shortcuts.register(
      "ctrl+a",
      () => {
        if ($currentView === "files" && $files.length > 0) {
          multiSelectMode = true;
          selectedFiles = new Set(
            $files.filter((f) => !f.is_dir).map((f) => f.name)
          );
          success(`${selectedFiles.size} Dateien ausgewählt`);
        }
      },
      { description: "Select all files" }
    );

    // Delete - Delete selected files
    shortcuts.register(
      "delete",
      () => {
        if ($currentView === "files" && selectedFiles.size > 0) {
          bulkDelete();
        }
      },
      { description: "Delete selected files" }
    );

    // F2 - Rename file
    shortcuts.register(
      "f2",
      () => {
        if ($currentView === "files" && selectedFiles.size === 1) {
          const fileName = Array.from(selectedFiles)[0];
          const file = $files.find((f) => f.name === fileName);
          if (file) renameFile(file);
        }
      },
      { description: "Rename file" }
    );

    // Ctrl+U - Upload
    shortcuts.register(
      "ctrl+u",
      () => {
        if ($currentView === "files") {
          handleUploadClick();
        }
      },
      { description: "Upload files" }
    );

    // Ctrl+D - Download
    shortcuts.register(
      "ctrl+d",
      () => {
        if ($currentView === "files" && selectedFiles.size > 0) {
          bulkDownload();
        }
      },
      { description: "Download selected" }
    );

    // Escape - Clear selection/close modals
    shortcuts.register(
      "escape",
      () => {
        if (multiSelectMode) {
          toggleMultiSelect();
        }
      },
      { description: "Clear selection" }
    );

    // Ctrl+M - Toggle multi-select mode
    shortcuts.register(
      "ctrl+m",
      () => {
        if ($currentView === "files") {
          toggleMultiSelect();
          success(
            multiSelectMode
              ? "Multi-select enabled 📋"
              : "Multi-select disabled"
          );
        }
      },
      { description: "Toggle multi-select mode" }
    );

    // Watch for view changes - reset path when returning to files view
    const unsubscribe = currentView.subscribe((view) => {
      console.log(`[FilesView] currentView changed to: "${view}"`);
      if (view === "files" && mounted) {
        // Ensure we have a valid path when entering files view
        if (!$currentPath || $currentPath === "") {
          console.log(`[FilesView] Resetting empty path to "/"`);
          currentPath.set("/");
        }
      }
    });

    return () => {
      mounted = false;
      unsubscribe();
    };
  });

  // Reactive statement: Reload files whenever currentPath changes
  // This handles breadcrumb navigation, folder clicks, and initial mount
  $: if (
    mounted &&
    $currentPath &&
    $currentView === "files" &&
    $currentPath !== lastLoadPath
  ) {
    console.log(
      `[FilesView] Reactive: mounted=${mounted}, path="${$currentPath}", view="${$currentView}", lastLoadPath="${lastLoadPath}"`
    );
    console.log(`[FilesView] Loading path: ${$currentPath}`);
    lastLoadPath = $currentPath;
    loadFiles();
  } else if (mounted) {
    console.log(
      `[FilesView] Reactive SKIPPED: mounted=${mounted}, path="${$currentPath}", view="${$currentView}", lastLoadPath="${lastLoadPath}"`
    );
  }

  async function loadFiles() {
    loading = true;
    try {
      const backendPath = toBackendPath($currentPath);
      console.log(
        `[FilesView] API call: list("${backendPath}") (UI path: "${$currentPath}")`
      );
      const data = await api.files.list(backendPath);
      console.log(`[FilesView] Received ${data.length} files`);
      files.set(data);

      // Generate thumbnails for image files
      loadThumbnails(data);
    } catch (error) {
      console.error("Failed to load files:", error);
      errorToast(t(lang, "failedToLoadFiles") + ": " + error.message);
      files.set([]);
    }
    loading = false;
  }

  /**
   * Load thumbnails for image files
   */
  async function loadThumbnails(fileList) {
    for (const file of fileList) {
      if (file.is_dir || !canGenerateThumbnail(file.name)) continue;

      const filePath = buildFilePath($currentPath, file.name);
      const fileId = file.id || filePath; // Use ID if available

      // Load thumbnail asynchronously with new API
      getThumbnail({
        id: fileId,
        path: $currentPath,
        name: file.name,
        modified: file.modified_at,
      })
        .then((dataUrl) => {
          if (dataUrl) {
            thumbnails.set(fileId, dataUrl); // Use fileId as key
            thumbnails = thumbnails; // Trigger reactivity
          }
        })
        .catch((err) => {
          console.error(`Failed to load thumbnail for ${file.name}:`, err);
        });
    }
  }

  async function handleUpload(fileList) {
    if (!fileList || fileList.length === 0) return;

    uploading = true;
    uploadProgress = { current: 0, total: fileList.length, uploading: true };
    fileUploadProgress = {}; // Reset progress tracking
    let successCount = 0;
    let failCount = 0;
    const failedFiles = [];

    // Process uploads through queue (max 2 concurrent)
    for (const file of fileList) {
      await uploadQueue.add(async () => {
        try {
          const path = buildFilePath($currentPath, file.name);
          console.log(
            `[Upload] Current path: "${$currentPath}", File: "${file.name}", Full path: "${path}"`
          );

          // Initialize progress for this file
          fileUploadProgress[file.name] = {
            percent: 0,
            loaded: 0,
            total: file.size,
          };

          // Upload with progress tracking and retry logic
          await retryWithBackoff(
            () =>
              api.files.uploadWithProgress(
                path,
                file,
                (percent, loaded, total) => {
                  fileUploadProgress[file.name] = { percent, loaded, total };
                  // Trigger reactivity
                  fileUploadProgress = { ...fileUploadProgress };
                }
              ),
            {
              maxRetries: 2,
              initialDelay: 1000,
              shouldRetry: (error) => {
                // Only retry on network errors, not on validation errors
                return (
                  error.name === "TypeError" || error.message.includes("fetch")
                );
              },
            }
          );

          successCount++;
          uploadProgress.current++;

          // Backend logs upload automatically, no need to track here

          // Mark file as complete
          fileUploadProgress[file.name].percent = 100;
          fileUploadProgress = { ...fileUploadProgress };

          // Show progress toast for each file
          if (fileList.length > 1) {
            success(
              `ðŸ“¤ ${uploadProgress.current}/${uploadProgress.total}: ${file.name}`,
              1000
            );
          }
        } catch (err) {
          console.error(`Upload error for ${file.name}:`, err);
          failCount++;
          failedFiles.push(file.name);

          // Mark file as failed
          if (fileUploadProgress[file.name]) {
            fileUploadProgress[file.name].error = true;
            fileUploadProgress = { ...fileUploadProgress };
          }
        }
      });
    }

    uploading = false;
    uploadProgress.uploading = false;

    // Reload files to reflect changes
    await loadFiles();

    // Summary toast
    if (successCount > 0 && failCount === 0) {
      success(
        `âœ… ${successCount} ${successCount === 1 ? "Datei" : "Dateien"} erfolgreich hochgeladen!`
      );
    } else if (successCount > 0 && failCount > 0) {
      success(
        `âœ… ${successCount} erfolgreich, âŒ ${failCount} fehlgeschlagen`
      );
      if (failedFiles.length > 0) {
        errorToast(`Fehlgeschlagen: ${failedFiles.join(", ")}`);
      }
    } else if (failCount > 0) {
      errorToast(`âŒ Alle ${failCount} Uploads fehlgeschlagen`);
    }

    // Clear progress after a delay
    setTimeout(() => {
      fileUploadProgress = {};
    }, 3000);

    // Mark that user has uploaded at least once
    if (successCount > 0) {
      localStorage.setItem("hasUploadedBefore", "true");
    }

    // Reload files with a small delay to ensure backend processing is done
    setTimeout(() => loadFiles(), 300);
  }

  function handleUploadClick() {
    uploadInput.click();
  }

  function handleFileInputChange(e) {
    handleUpload(e.target.files);
  }

  // Drag and drop handlers
  function handleDragOver(e) {
    e.preventDefault();
    // Only show dropzone highlight for file uploads (not internal moves)
    if (!draggedFile) {
      dragOver = true;
    }
  }

  function handleDragLeave(e) {
    e.preventDefault();
    dragOver = false;
  }

  function handleDrop(e) {
    e.preventDefault();
    dragOver = false;
    // Only handle file uploads, not internal file moves
    if (e.dataTransfer.files.length > 0) {
      handleUpload(e.dataTransfer.files);
    }
  }

  // Drag & Drop for moving files into folders
  function handleFileDragStart(e, file) {
    draggedFile = file;
    e.dataTransfer.effectAllowed = "move";

    // If file is part of multi-selection, show count badge
    if (multiSelectMode && selectedFiles.has(file.name)) {
      const count = selectedFiles.size;
      const dragImage = createDragGhost(count);
      e.dataTransfer.setDragImage(dragImage, 25, 25);
    }

    e.target.style.opacity = "0.5";
  }

  function handleFileDragEnd(e) {
    e.target.style.opacity = "1";
    draggedFile = null;
    dropTargetFolder = null;
  }

  /**
   * Create custom drag ghost element
   */
  function createDragGhost(count) {
    const ghost = document.createElement("div");
    ghost.style.cssText = `
      position: absolute;
      top: -9999px;
      padding: 8px 16px;
      background: var(--md-sys-color-primary);
      color: var(--md-sys-color-on-primary);
      border-radius: 20px;
      font-size: 14px;
      font-weight: 600;
      box-shadow: 0 4px 12px rgba(0,0,0,0.3);
      display: flex;
      align-items: center;
      gap: 8px;
    `;
    ghost.textContent = count > 1 ? `${count} files` : "1 file";
    document.body.appendChild(ghost);

    // Clean up after a short delay
    setTimeout(() => document.body.removeChild(ghost), 0);

    return ghost;
  }

  function handleFolderDragOver(e, folder) {
    if (!draggedFile || !folder.is_dir || draggedFile.name === folder.name) {
      return;
    }
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
    dropTargetFolder = folder.name;
  }

  function handleFolderDragLeave(e, folder) {
    if (dropTargetFolder === folder.name) {
      dropTargetFolder = null;
    }
  }

  async function handleFolderDrop(e, folder) {
    e.preventDefault();
    e.stopPropagation();
    dropTargetFolder = null;

    if (!draggedFile || !folder.is_dir) return;

    // Save draggedFile reference before it gets cleared
    const fileToMove = draggedFile;

    try {
      const oldPath = buildFilePath($currentPath, fileToMove.name);
      const newPath = buildFilePath(
        $currentPath,
        `${folder.name}/${fileToMove.name}`
      );

      await api.files.rename(oldPath, newPath);
      success(`ðŸ“ "${draggedFile.name}" â†’ "${folder.name}"`);
      await loadFiles();
    } catch (err) {
      console.error("Failed to move file:", err);
      errorToast(`âŒ Fehler beim Verschieben: ${err.message}`);
    }

    draggedFile = null;
  }

  /**
   * Enhanced folder drop handler with multi-select support
   */
  async function handleFolderDropMulti(e, folder) {
    e.preventDefault();
    e.stopPropagation();
    dropTargetFolder = null;

    if (!draggedFile || !folder.is_dir) return;

    // Check if multi-select mode and file is in selection
    const filesToMove =
      multiSelectMode && selectedFiles.has(draggedFile.name)
        ? Array.from(selectedFiles)
            .map((name) => $files.find((f) => f.name === name))
            .filter(Boolean)
        : [draggedFile];

    try {
      let successCount = 0;
      let failCount = 0;

      for (const file of filesToMove) {
        if (file.is_dir) continue; // Skip folders for now

        try {
          const oldPath = buildFilePath($currentPath, file.name);
          const newPath = buildFilePath(
            $currentPath,
            `${folder.name}/${file.name}`
          );

          await api.files.rename(oldPath, newPath);
          successCount++;
        } catch (err) {
          console.error(`Failed to move ${file.name}:`, err);
          failCount++;
        }
      }

      if (successCount > 0) {
        success(
          `📁 Moved ${successCount} file${successCount > 1 ? "s" : ""} to "${folder.name}"`
        );
      }
      if (failCount > 0) {
        errorToast(
          `${t(lang, "failedToMove")} ${failCount} ${failCount > 1 ? "Dateien" : "Datei"}`
        );
      }
      await loadFiles();

      // Clear selection after successful move
      if (multiSelectMode && successCount > 0) {
        selectedFiles.clear();
        selectedFiles = selectedFiles;
      }
    } catch (err) {
      console.error("Failed to move files:", err);
      errorToast(`Error moving files: ${err.message}`);
    }

    draggedFile = null;
  }

  async function createFolder() {
    showNewFolderDialog = true;
  }

  async function handleCreateFolder(event) {
    const folderName = event.detail;
    if (!folderName) return;

    try {
      const path = buildFilePath($currentPath, folderName);
      await api.files.createDir(path);
      success(
        `${t($currentLang, "folder")} "${folderName}" ${t($currentLang, "created")}`
      );
      await loadFiles();
    } catch (err) {
      console.error("Failed to create folder:", err);
      errorToast(`${t(lang, "failedToCreateFolder")}: ${err.message}`);
    }
  }

  async function downloadFile(file) {
    try {
      const path = buildFilePath($currentPath, file.name);
      const blob = await api.files.download(path);
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = file.name;
      a.click();
      URL.revokeObjectURL(url);

      // Backend can log downloads if needed

      success(`"${file.name}" ${t($currentLang, "downloading")}`);
    } catch (err) {
      console.error("Failed to download file:", err);
      errorToast(`${t(lang, "failedToDownload")}: ${err.message}`);
    }
  }

  async function deleteFile(file) {
    fileToDelete = file;
    showDeleteDialog = true;
  }

  async function handleDeleteConfirm() {
    if (!fileToDelete) return;

    const fileName = fileToDelete.name;

    try {
      const path = buildFilePath($currentPath, fileName);
      const fileId = fileToDelete.id || path;

      await api.files.delete(path);

      // Invalidate thumbnail cache
      await invalidateThumbnails(fileId);

      success(`"${fileName}" ${t($currentLang, "deleted")}`);
      await loadFiles();
    } catch (err) {
      console.error("Failed to delete file:", err);
      errorToast(`${t(lang, "failedToDelete")}: ${err.message}`);
    }

    fileToDelete = null;
  }

  async function renameFile(file) {
    fileToRename = file;
    showRenameDialog = true;
  }

  function previewFile(file) {
    if (!file.is_dir && isPreviewable(file.name)) {
      fileToPreview = file;
      showPreviewModal = true;
    }
  }

  function handleFileClick(file) {
    console.log(
      `[handleFileClick] File: ${file.name}, is_dir: ${file.is_dir}, multiSelectMode: ${multiSelectMode}`
    );

    if (multiSelectMode && !file.is_dir) {
      console.log("[handleFileClick] Multi-select mode - toggling selection");
      toggleFileSelection(file);
    } else if (file.is_dir) {
      console.log("[handleFileClick] Directory - navigating");
      navigateTo(file.name);
    } else if (isPreviewable(file.name)) {
      console.log(
        `[handleFileClick] Previewable file - opening preview for ${file.name}`
      );
      previewFile(file);
    } else {
      console.log(`[handleFileClick] Not previewable: ${file.name}`);
    }
  }

  function toggleMultiSelect() {
    multiSelectMode = !multiSelectMode;
    if (!multiSelectMode) {
      selectedFiles.clear();
      selectedFiles = selectedFiles;
    }
  }

  function toggleFileSelection(file) {
    if (selectedFiles.has(file.name)) {
      selectedFiles.delete(file.name);
    } else {
      selectedFiles.add(file.name);
      // Auto-enable multi-select mode when first file is selected
      if (!multiSelectMode) {
        multiSelectMode = true;
      }
    }

    // Auto-disable when no files selected
    if (selectedFiles.size === 0 && multiSelectMode) {
      multiSelectMode = false;
    }

    selectedFiles = selectedFiles; // Trigger reactivity
  }

  async function toggleFavorite(file) {
    if (!file) return;
    const fullPath = buildFilePath($currentPath, file.name);
    console.log(`[toggleFavorite] Toggling favorite: ${fullPath}`);

    try {
      await favorites.toggle(fullPath, file.is_dir ? "folder" : "file");
      // Check if now favorite by checking if it exists in store
      const isFav = favorites.getAll().some((f) => f.item_id === fullPath);
      success(
        isFav
          ? `⭐ ${file.name} zu Favoriten hinzugefügt`
          : `${file.name} aus Favoriten entfernt`
      );
    } catch (err) {
      console.error("Failed to toggle favorite:", err);
      errorToast(`${t(lang, "failedToToggleFavorite")}: ${err.message}`);
    }
  }

  function isFavorite(file) {
    if (!file) return false;
    const fullPath = buildFilePath($currentPath, file.name);
    return $favorites.has(fullPath);
  }

  /**
   * Show context menu on right-click
   */
  function handleContextMenu(event, file) {
    event.preventDefault();
    contextMenuFile = file;
    contextMenuX = event.clientX;
    contextMenuY = event.clientY;
    contextMenuVisible = true;
  }

  /**
   * Hide context menu
   */
  function hideContextMenu() {
    contextMenuVisible = false;
    contextMenuFile = null;
  }

  /**
   * Handle context menu item clicks
   */
  function handleContextMenuAction(action) {
    if (!contextMenuFile) return;

    switch (action) {
      case "open":
        handleFileClick(contextMenuFile);
        break;
      case "download":
        downloadFile(contextMenuFile);
        break;
      case "rename":
        renameFile(contextMenuFile);
        break;
      case "move":
        openMoveDialog(contextMenuFile);
        break;
      case "delete":
        deleteFile(contextMenuFile);
        break;
      case "favorite":
        toggleFavorite(contextMenuFile);
        break;
      case "copyLink":
        copyFileLink(contextMenuFile);
        break;
      case "comments":
        showComments(contextMenuFile);
        break;
      case "properties":
        openPropertiesDialog(contextMenuFile);
        break;
    }

    hideContextMenu();
  }

  /**
   * Copy file download link to clipboard
   */
  async function copyFileLink(file) {
    if (file.is_dir) return;
    const filePath = $currentPath ? `${$currentPath}/${file.name}` : file.name;
    const link = `${window.location.origin}/api/file/${encodeURIComponent(filePath)}`;

    try {
      await navigator.clipboard.writeText(link);
      success(`🔗 Link copied to clipboard`);
    } catch (err) {
      errorToast(t(lang, "failedToCopyLink"));
    }
  }

  /**
   * Open move dialog with folder selection
   */
  async function openMoveDialog(file) {
    fileToMove = file;
    // Load all available folders recursively
    await loadAvailableFolders();
    showMoveDialog = true;
  }

  /**
   * Load all folders in the system for move dialog
   */
  async function loadAvailableFolders() {
    try {
      const allFolders = [{ name: "Home", path: "/" }];
      await recursivelyLoadFolders("/", allFolders);
      availableFolders = allFolders;
    } catch (err) {
      console.error("Failed to load folders:", err);
      availableFolders = [{ name: "Home", path: "/" }];
    }
  }

  /**
   * Recursively load folders from a path
   */
  async function recursivelyLoadFolders(path, foldersList) {
    try {
      const backendPath = toBackendPath(path);
      const response = await api.files.list(backendPath);
      const folders = response.files.filter(f => f.is_dir);
      
      for (const folder of folders) {
        const folderPath = path === "/" ? `/${folder.name}/` : `${path}${folder.name}/`;
        foldersList.push({ name: folderPath, path: folderPath });
        // Recursively load subfolders (limit depth to avoid infinite loops)
        if (foldersList.length < 100) {
          await recursivelyLoadFolders(folderPath, foldersList);
        }
      }
    } catch (err) {
      console.error(`Failed to load folders from ${path}:`, err);
    }
  }

  /**
   * Execute file move to selected folder
   */
  async function executeMoveFile(targetPath) {
    if (!fileToMove) return;

    try {
      const oldPath = buildFilePath($currentPath, fileToMove.name);
      const newPath = targetPath === "/" 
        ? fileToMove.name 
        : `${toBackendPath(targetPath)}/${fileToMove.name}`;

      await api.files.rename(oldPath, newPath);
      success(`📁 "${fileToMove.name}" → "${targetPath}"`);
      
      showMoveDialog = false;
      fileToMove = null;
      
      await loadFiles();
    } catch (err) {
      console.error("Failed to move file:", err);
      errorToast(`❌ Fehler beim Verschieben: ${err.message}`);
    }
  }

  /**
   * Open properties dialog with file details
   */
  function openPropertiesDialog(file) {
    fileForProperties = file;
    showPropertiesDialog = true;
  }

  /**
   * Show file properties dialog (placeholder)
   */
  function showFileProperties(file) {
    const props = [
      `Name: ${file.name}`,
      `Size: ${formatSize(file.size)}`,
      `Modified: ${new Date(file.modified_at).toLocaleString()}`,
      `Type: ${file.is_dir ? "Folder" : getFileType(file.name)}`,
    ].join("\n");

    alert(`File Properties\n\n${props}`);
  }

  /**
   * Show comments & tags panel for file
   */
  function showComments(file) {
    fileForComments = file;
    showCommentsPanel = true;
  }

  // ============================================
  // TOUCH GESTURE HANDLERS
  // ============================================

  /**
   * Handle swipe gestures on file cards
   */
  function handleFileSwipe(file, direction, distance) {
    if (direction === "right" && distance > 80) {
      // Swipe right: Toggle favorite
      toggleFavorite(file);
      success(
        `${favorites.isFavorite(file.name) ? "Added to" : "Removed from"} favorites`
      );
    } else if (direction === "left" && distance > 80) {
      // Swipe left: Quick delete
      fileToDelete = file;
      showDeleteDialog = true;
    }
  }

  /**
   * Handle double-tap on file card
   */
  function handleFileDoubleTap(file) {
    if (file.is_directory) {
      navigateTo(file.name);
    } else {
      // Open preview modal
      selectedFile = file;
      showPreviewModal = true;
    }
  }

  /**
   * Handle long-press on file card
   */
  function handleFileLongPress(file, event) {
    // Show context menu on long press
    contextMenuFile = file;
    contextMenuX = event.touches?.[0]?.clientX || event.clientX;
    contextMenuY = event.touches?.[0]?.clientY || event.clientY;
    contextMenuVisible = true;

    // Haptic feedback if available
    if (navigator.vibrate) {
      navigator.vibrate(50);
    }
  }

  /**
   * Handle pull-to-refresh gesture
   */
  function handlePullStart(e) {
    if (window.scrollY === 0) {
      touchGesture.handleTouchStart(e);
      isPulling = true;
    }
  }

  function handlePullMove(e) {
    if (!isPulling) return;

    const touch = e.touches[0];
    const startY = touchGesture.startY;
    pullDistance = Math.max(
      0,
      Math.min(touch.clientY - startY, pullToRefreshThreshold * 1.5)
    );

    if (pullDistance > 0) {
      e.preventDefault(); // Prevent scroll
    }
  }

  function handlePullEnd(e) {
    if (!isPulling) return;

    if (pullDistance >= pullToRefreshThreshold) {
      // Trigger refresh
      loadFiles();
      success("Refreshing files...");
    }

    isPulling = false;
    pullDistance = 0;
  }

  /**
   * Apply touch gesture handlers to file card element
   */
  function attachFileGestures(element, file) {
    if (!element || !TouchGestureHandler.isMobile()) return;

    element.addEventListener("touchstart", (e) => {
      touchGesture.handleTouchStart(e);
    });

    element.addEventListener("touchend", (e) => {
      touchGesture.handleTouchEnd(e, {
        onSwipeRight: (dist) => handleFileSwipe(file, "right", dist),
        onSwipeLeft: (dist) => handleFileSwipe(file, "left", dist),
        onDoubleTap: () => handleFileDoubleTap(file),
      });
    });

    // Long press handler
    touchGesture.handleLongPress(
      element,
      (e) => handleFileLongPress(file, e),
      600
    );
  }

  function selectAll() {
    displayedFiles.forEach((file) => {
      if (!file.is_dir) selectedFiles.add(file.name);
    });
    selectedFiles = selectedFiles;
  }

  function deselectAll() {
    selectedFiles.clear();
    selectedFiles = selectedFiles;
  }

  async function bulkDelete() {
    if (selectedFiles.size === 0) return;

    const confirmed = confirm(`Delete ${selectedFiles.size} selected files?`);
    if (!confirmed) return;

    let successCount = 0;
    let failCount = 0;

    for (const filename of selectedFiles) {
      try {
        const path = buildFilePath($currentPath, filename);
        await api.files.delete(path);
        successCount++;
      } catch (err) {
        console.error(`Failed to delete ${filename}:`, err);
        failCount++;
      }
    }

    if (successCount > 0) {
      success(`âœ… Deleted ${successCount} file(s)`);
    }
    if (failCount > 0) {
      errorToast(`âŒ Failed to delete ${failCount} file(s)`);
    }

    selectedFiles.clear();
    selectedFiles = selectedFiles;
    await loadFiles();
  }

  async function bulkDownload() {
    if (selectedFiles.size === 0) return;

    success(`📥 Downloading ${selectedFiles.size} file(s)...`);

    for (const filename of selectedFiles) {
      try {
        const file = $files.find((f) => f.name === filename);
        if (file && !file.is_dir) {
          await downloadFile(file);
          // Small delay to avoid overwhelming the browser
          await new Promise((resolve) => setTimeout(resolve, 100));
        }
      } catch (err) {
        console.error(`Failed to download ${filename}:`, err);
        errorToast(`${t(lang, "failedToDownload")} ${filename}`);
      }
    }
  }

  function selectAllFiles() {
    if ($files.length === 0) return;
    multiSelectMode = true;
    selectedFiles = new Set($files.filter((f) => !f.is_dir).map((f) => f.name));
    success(`${selectedFiles.size} Dateien ausgewählt`);
  }

  async function handleRenameConfirm(event) {
    const newName = event.detail;
    if (!fileToRename || !newName || newName === fileToRename.name) return;

    const oldName = fileToRename.name;

    try {
      const oldPath = buildFilePath($currentPath, oldName);
      const newPath = buildFilePath($currentPath, newName);
      await api.files.rename(oldPath, newPath);
      success(`"${oldName}" â†’ "${newName}"`);
      await loadFiles();
    } catch (err) {
      console.error("Failed to rename file:", err);
      errorToast(`Error: ${err.message}`);
    }

    fileToRename = null;
  }

  function formatSize(bytes) {
    if (!bytes) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }
</script>

<input
  type="file"
  multiple
  bind:this={uploadInput}
  on:change={handleFileInputChange}
  style="display: none;"
/>

<div class="files-view">
  <div class="page-header">
    <h2><Icon name="folder-fill" size={24} /> {t($currentLang, "files")}</h2>
    <div class="header-actions">
      <SearchBar
        bind:value={searchQuery}
        placeholder="{t($currentLang, 'search')}..."
        on:input={handleSearchInput}
        on:clear={() => {
          searchQuery = "";
          searchResults = [];
          loadFiles();
        }}
      />
      <button
        class="btn-filter"
        on:click={() => (showFilterPanel = true)}
        title="Advanced Filters"
        class:active={activeFilters.fileTypes.length > 0 ||
          activeFilters.sizeRange.min > 0 ||
          activeFilters.sizeRange.max < Infinity ||
          activeFilters.dateRange.from ||
          activeFilters.dateRange.to}
      >
        <Icon name="funnel" size={18} />
        {#if activeFilters.fileTypes.length > 0}
          <span class="filter-badge">{activeFilters.fileTypes.length}</span>
        {/if}
      </button>
      <button
        class="btn-view-toggle"
        on:click={toggleViewMode}
        title={viewMode === "grid" ? "Listen-Ansicht" : "Karten-Ansicht"}
      >
        <Icon
          name={viewMode === "grid" ? "list-ul" : "grid-3x3-gap"}
          size={18}
        />
      </button>
      <Button onClick={createFolder} variant="outlined">
        <Icon name="folder-plus" size={16} />
        {t($currentLang, "newFolder")}
      </Button>
    </div>
  </div>

  <!-- Multi-select toolbar -->
  {#if selectedFiles.size > 0}
    <div class="multiselect-toolbar">
      <span class="selected-count">{selectedFiles.size} selected</span>
      <div class="multiselect-actions">
        <button class="btn-multiselect" on:click={selectAll}>
          <Icon name="check-all" size={16} />
          Select All
        </button>
        <button class="btn-multiselect" on:click={deselectAll}>
          <Icon name="x-square" size={16} />
          Deselect All
        </button>
        <button class="btn-multiselect btn-danger" on:click={bulkDelete}>
          <Icon name="trash" size={16} />
          Delete Selected
        </button>
      </div>
    </div>
  {/if}

  <!-- Floating Upload Button (FAB - Floating Action Button) -->
  <button
    class="fab-upload"
    class:uploading
    on:click={handleUploadClick}
    disabled={uploading}
    title="Dateien hochladen"
  >
    <Icon name={uploading ? "clock-history" : "cloud-upload-fill"} size={24} />
    {#if uploadProgress.uploading}
      <span class="upload-badge"
        >{uploadProgress.current}/{uploadProgress.total}</span
      >
    {/if}
  </button>

  <!-- Breadcrumb Navigation -->
  <div class="breadcrumb-nav">
    {#each breadcrumbs as crumb, i}
      {#if i > 0}
        <span class="breadcrumb-separator">/</span>
      {/if}
      <button
        class="breadcrumb-item"
        class:active={i === breadcrumbs.length - 1}
        on:click={() => navigateToPath(crumb.path)}
        disabled={i === breadcrumbs.length - 1}
      >
        {#if i === 0}
          <Icon name="house-fill" size={14} />
        {/if}
        {crumb.name}
      </button>
    {/each}
  </div>

  <!-- Compact Drop Zone -->
  {#if showUploadPanel}
    <div
      class="drop-zone-compact"
      class:drag-over={dragOver}
      on:dragover={handleDragOver}
      on:dragleave={handleDragLeave}
      on:drop={handleDrop}
      role="region"
    >
      <Icon name="cloud-arrow-up" size={16} />
      <span>{dragOver ? "ðŸ“¦ Drop hier!" : "Drag & Drop Dateien hier"}</span>
    </div>
  {/if}

  <!-- Upload Progress Panel -->
  {#if Object.keys(fileUploadProgress).length > 0}
    <div class="upload-progress-panel">
      <div class="upload-progress-header">
        <Icon name="cloud-upload" size={16} />
        <span
          >Uploading {Object.keys(fileUploadProgress).length}
          {Object.keys(fileUploadProgress).length === 1
            ? "file"
            : "files"}...</span
        >
      </div>
      <div class="upload-files-list">
        {#each Object.entries(fileUploadProgress) as [filename, progress]}
          <div class="upload-file-item" class:error={progress.error}>
            <div class="upload-file-info">
              <Icon
                name={progress.error
                  ? "x-circle"
                  : progress.percent === 100
                    ? "check-circle"
                    : "file-earmark"}
                size={14}
              />
              <span class="upload-filename">{filename}</span>
              <span class="upload-size">
                {progress.error ? "Failed" : `${Math.round(progress.percent)}%`}
              </span>
            </div>
            {#if !progress.error}
              <div class="upload-progress-bar">
                <div
                  class="upload-progress-fill"
                  style="width: {progress.percent}%"
                ></div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Search Mode Indicator -->
  {#if searchQuery && searchQuery.length >= 2}
    <div class="search-mode-indicator">
      <span class="search-icon">ðŸ”</span>
      <span class="search-info">
        Search results for <strong>"{searchQuery}"</strong>
        {#if searchResults.length > 0}
          â€” {searchResults.length}
          {searchResults.length === 1 ? "file" : "files"} found
        {/if}
      </span>
      <button
        class="btn-clear-search"
        on:click={() => {
          searchQuery = "";
          searchResults = [];
          loadFiles();
        }}
      >
        Clear search
      </button>
    </div>
  {/if}

  {#if loading}
    <SkeletonLoader count={12} type={viewMode} />
  {:else if isSearching}
    <div class="loading">
      <div class="spinner"></div>
      <p>ðŸ” Searching...</p>
    </div>
  {:else if displayedFiles.length === 0}
    <div class="empty-state">
      <p class="empty-icon">{searchQuery ? "ï¿½" : "ï¿½ðŸ“‚"}</p>
      <p class="empty-title">
        {searchQuery ? "No results found" : t($currentLang, "noFiles")}
      </p>
      <p class="empty-subtitle">
        {searchQuery
          ? `Try different keywords`
          : t($currentLang, "dragAndDropHere")}
      </p>
    </div>
  {:else if viewMode === "grid"}
    <div class="file-grid">
      {#each displayedFiles as file}
        <div
          class="file-card"
          class:folder={file.is_dir}
          class:selected={multiSelectMode && selectedFiles.has(file.name)}
          class:drag-over={file.is_dir && dropTargetFolder === file.name}
          draggable="true"
          on:dragstart={(e) => handleFileDragStart(e, file)}
          on:dragend={handleFileDragEnd}
          on:dragover={(e) => file.is_dir && handleFolderDragOver(e, file)}
          on:dragleave={(e) => file.is_dir && handleFolderDragLeave(e, file)}
          on:drop={(e) => file.is_dir && handleFolderDrop(e, file)}
          on:click={() => handleFileClick(file)}
          on:contextmenu={(e) => handleContextMenu(e, file)}
          on:keydown={(e) => e.key === "Enter" && handleFileClick(file)}
          role="button"
          tabindex="0"
        >
          {#if multiSelectMode && !file.is_dir}
            <div class="file-checkbox">
              <Icon
                name={selectedFiles.has(file.name)
                  ? "check-square-fill"
                  : "square"}
                size={20}
              />
            </div>
          {/if}
          <div class="file-icon">
            {#if !file.is_dir && thumbnails.has($currentPath ? `${$currentPath}${file.name}` : file.name)}
              <div class="file-thumbnail">
                <img
                  src={thumbnails.get(
                    $currentPath ? `${$currentPath}${file.name}` : file.name
                  )}
                  alt={file.name}
                  loading="lazy"
                />
              </div>
            {:else}
              <Icon name={getFileIcon(file.name, file.is_dir)} size={48} />
            {/if}
          </div>
          <div class="file-name" title={displayName(file.name)}>
            {displayName(file.name)}
          </div>
          <div class="file-meta">
            {#if file.is_dir}
              <Icon name="folder" size={14} /> Ordner
            {:else}
              {formatSize(file.size)}
            {/if}
          </div>

          <!-- Comments & Tags badges -->
          {#if comments.getCount($currentPath + file.name, $comments) > 0 || tags.getTags($currentPath + file.name, $tags).length > 0}
            <div class="file-badges">
              {#if comments.getCount($currentPath + file.name, $comments) > 0}
                <span
                  class="badge badge-comments"
                  title="{comments.getCount(
                    $currentPath + file.name,
                    $comments
                  )} comments"
                >
                  <Icon name="chat-dots" size={12} />
                  {comments.getCount($currentPath + file.name, $comments)}
                </span>
              {/if}
              {#if tags.getTags($currentPath + file.name, $tags).length > 0}
                <span
                  class="badge badge-tags"
                  title="{tags.getTags($currentPath + file.name, $tags)
                    .length} tags"
                >
                  <Icon name="tag" size={12} />
                  {tags.getTags($currentPath + file.name, $tags).length}
                </span>
              {/if}
            </div>
          {/if}

          <div class="file-actions">
            <button
              class="btn-icon btn-favorite"
              on:click|stopPropagation={() => toggleFavorite(file)}
              title={isFavorite(file)
                ? "Aus Favoriten entfernen"
                : "Zu Favoriten hinzufügen"}
            >
              <Icon name={isFavorite(file) ? "star-fill" : "star"} size={16} />
            </button>
            {#if !file.is_dir}
              <button
                class="btn-icon"
                on:click|stopPropagation={() => downloadFile(file)}
                title={t($currentLang, "download")}
              >
                <Icon name="download" size={16} />
              </button>
            {/if}
            <button
              class="btn-icon"
              on:click|stopPropagation={() => renameFile(file)}
              title={t($currentLang, "rename")}
            >
              <Icon name="pencil" size={16} />
            </button>
            <button
              class="btn-icon btn-delete"
              on:click|stopPropagation={() => deleteFile(file)}
              title={t($currentLang, "delete")}
            >
              <Icon name="trash" size={16} />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <!-- List View -->
    <div class="file-list">
      {#each displayedFiles as file}
        <div
          class="file-row"
          class:folder={file.is_dir}
          class:drag-over={file.is_dir && dropTargetFolder === file.name}
          draggable="true"
          on:dragstart={(e) => handleFileDragStart(e, file)}
          on:dragend={handleFileDragEnd}
          on:dragover={(e) => file.is_dir && handleFolderDragOver(e, file)}
          on:dragleave={(e) => file.is_dir && handleFolderDragLeave(e, file)}
          on:drop={(e) => file.is_dir && handleFolderDrop(e, file)}
          on:click={() => handleFileClick(file)}
          on:contextmenu={(e) => handleContextMenu(e, file)}
          on:keydown={(e) => e.key === "Enter" && handleFileClick(file)}
          role="button"
          tabindex="0"
        >
          <div class="file-icon-small">
            <Icon name={getFileIcon(file.name, file.is_dir)} size={24} />
          </div>
          <div class="file-name-list" title={displayName(file.name)}>
            {displayName(file.name)}
          </div>
          <div class="file-size-list">
            {#if file.is_dir}
              <Icon name="folder" size={14} /> Ordner
            {:else}
              {formatSize(file.size)}
            {/if}
          </div>
          <div class="file-actions-list">
            <button
              class="btn-icon-small btn-favorite"
              on:click|stopPropagation={() => toggleFavorite(file)}
              title={isFavorite(file)
                ? "Aus Favoriten entfernen"
                : "Zu Favoriten hinzufügen"}
            >
              <Icon name={isFavorite(file) ? "star-fill" : "star"} size={14} />
            </button>
            {#if !file.is_dir}
              <button
                class="btn-icon-small"
                on:click|stopPropagation={() => downloadFile(file)}
                title={t($currentLang, "download")}
              >
                <Icon name="download" size={14} />
              </button>
            {/if}
            <button
              class="btn-icon-small"
              on:click|stopPropagation={() => renameFile(file)}
              title={t($currentLang, "rename")}
            >
              <Icon name="pencil" size={14} />
            </button>
            <button
              class="btn-icon-small btn-delete"
              on:click|stopPropagation={() => deleteFile(file)}
              title={t($currentLang, "delete")}
            >
              <Icon name="trash" size={14} />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Dialogs -->
<InputDialog
  bind:open={showNewFolderDialog}
  title="Neuer Ordner"
  label="Ordnername"
  placeholder="Mein Ordner"
  confirmText="Erstellen"
  on:confirm={handleCreateFolder}
/>

<InputDialog
  bind:open={showRenameDialog}
  title="Umbenennen"
  label="Neuer Name"
  placeholder={fileToRename?.name || ""}
  initialValue={fileToRename?.name || ""}
  confirmText="Umbenennen"
  on:confirm={handleRenameConfirm}
/>

<Dialog
  bind:open={showDeleteDialog}
  title="Löschen bestätigen"
  confirmText="Löschen"
  cancelText="Abbrechen"
  danger={true}
  on:confirm={handleDeleteConfirm}
>
  <p>
    Möchten Sie <strong>"{displayName(fileToDelete?.name || "")}"</strong> wirklich
    löschen?
  </p>
  <p style="color: var(--md-sys-color-error); margin-top: 12px;">
    Diese Aktion kann nicht rückgängig gemacht werden.
  </p>
</Dialog>

{#if showPreviewModal && fileToPreview}
  <PreviewModal
    file={fileToPreview}
    files={displayedFiles}
    currentPath={$currentPath}
    on:close={() => {
      showPreviewModal = false;
      fileToPreview = null;
    }}
  />
{/if}

<!-- Comments & Tags Panel -->
<CommentsPanel bind:visible={showCommentsPanel} bind:file={fileForComments} />

<!-- Filter Panel -->
<FilterPanel
  bind:visible={showFilterPanel}
  on:apply={(e) => {
    activeFilters = e.detail;
    success(
      `Filters applied: ${activeFilters.fileTypes.length} types selected`
    );
  }}
  on:reset={() => {
    activeFilters = {
      fileTypes: [],
      sizeRange: { min: 0, max: Infinity },
      dateRange: { from: null, to: null },
    };
    success("Filters cleared");
  }}
/>

<!-- Context Menu -->
<ContextMenu
  visible={contextMenuVisible}
  x={contextMenuX}
  y={contextMenuY}
  items={[
    {
      label: contextMenuFile?.is_dir ? "Open" : "Preview",
      icon: contextMenuFile?.is_dir ? "folder-open" : "eye",
      action: "open",
      shortcut: "Enter",
    },
    {
      label: "Download",
      icon: "download",
      action: "download",
      shortcut: "Ctrl+D",
      disabled: contextMenuFile?.is_dir,
    },
    { divider: true },
    {
      label: "Rename",
      icon: "pencil",
      action: "rename",
      shortcut: "F2",
    },
    {
      label: "Move to...",
      icon: "arrows-move",
      action: "move",
      shortcut: "Ctrl+M",
    },
    {
      label: "Delete",
      icon: "trash",
      action: "delete",
      shortcut: "Del",
    },
    { divider: true },
    {
      label: isFavorite(contextMenuFile)
        ? "Remove from Favorites"
        : "Add to Favorites",
      icon: isFavorite(contextMenuFile) ? "star-fill" : "star",
      action: "favorite",
    },
    {
      label: "Copy Link",
      icon: "link",
      action: "copyLink",
      disabled: contextMenuFile?.is_dir,
    },
    { divider: true },
    {
      label: "Comments & Tags",
      icon: "chat-dots",
      action: "comments",
    },
    {
      label: "Properties",
      icon: "info-circle",
      action: "properties",
    },
  ]}
  on:close={hideContextMenu}
  on:itemClick={(e) => handleContextMenuAction(e.detail.action)}
/>

<!-- Move File Dialog -->
{#if showMoveDialog && fileToMove}
  <div class="custom-dialog-backdrop" on:click={() => { showMoveDialog = false; fileToMove = null; }}>
    <div class="custom-dialog" on:click|stopPropagation>
      <div class="dialog-header">
        <h2>Move '{fileToMove.name}' to...</h2>
        <button class="close-btn" onclick={() => { showMoveDialog = false; fileToMove = null; }}>
          <Icon name="x" />
        </button>
      </div>
      
      <div class="folder-list">
        {#if availableFolders.length === 0}
          <div class="empty-state">
            <p>Loading folders...</p>
          </div>
        {:else}
          {#each availableFolders as folder}
            <button
              class="folder-item"
              onclick={() => executeMoveFile(folder.path)}
            >
              <Icon name="folder" />
              <span>{folder.name}</span>
            </button>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

<!-- File Properties Dialog -->
{#if showPropertiesDialog && fileForProperties}
  <div class="custom-dialog-backdrop" on:click={() => { showPropertiesDialog = false; fileForProperties = null; }}>
    <div class="custom-dialog" on:click|stopPropagation>
      <div class="dialog-header">
        <h2>Properties</h2>
        <button class="close-btn" onclick={() => { showPropertiesDialog = false; fileForProperties = null; }}>
          <Icon name="x" />
        </button>
      </div>
      
      <div class="properties-grid">
        <div class="prop-row">
          <span class="prop-label">Name:</span>
          <span class="prop-value">{fileForProperties.name}</span>
        </div>
        <div class="prop-row">
          <span class="prop-label">Size:</span>
          <span class="prop-value">{formatSize(fileForProperties.size)}</span>
        </div>
        <div class="prop-row">
          <span class="prop-label">Type:</span>
          <span class="prop-value">
            {fileForProperties.is_dir ? "Folder" : getFileType(fileForProperties.name)}
          </span>
        </div>
        <div class="prop-row">
          <span class="prop-label">Modified:</span>
          <span class="prop-value">
            {new Date(fileForProperties.modified_at).toLocaleString()}
          </span>
        </div>
        <div class="prop-row">
          <span class="prop-label">Location:</span>
          <span class="prop-value">{$currentPath}</span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .files-view {
    padding: 32px;
    max-width: 1400px;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    flex-wrap: wrap;
    gap: 16px;
  }

  .header-actions {
    display: flex;
    gap: 12px;
    align-items: center;
    flex-wrap: wrap;
  }

  .btn-view-toggle,
  .btn-filter {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    border: 1px solid var(--md-sys-color-outline);
    background: var(--md-sys-color-surface);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .btn-view-toggle:hover,
  .btn-filter:hover {
    background: var(--md-sys-color-secondary-container);
    border-color: var(--md-sys-color-secondary);
  }

  .btn-filter.active {
    background: var(--md-sys-color-primary-container);
    border-color: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary-container);
  }

  .filter-badge {
    position: absolute;
    top: -4px;
    right: -4px;
    background: var(--md-sys-color-error);
    color: var(--md-sys-color-on-error);
    border-radius: 10px;
    min-width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-weight: 600;
    padding: 0 5px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  /* Breadcrumb Navigation */
  .breadcrumb-nav {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
    margin-bottom: 24px;
    flex-wrap: wrap;
  }

  .breadcrumb-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--md-sys-color-primary);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .breadcrumb-item:hover:not(:disabled) {
    background: var(--md-sys-color-primary-container);
  }

  .breadcrumb-item:disabled,
  .breadcrumb-item.active {
    color: var(--md-sys-color-on-surface);
    cursor: default;
    font-weight: 600;
  }

  .breadcrumb-separator {
    color: var(--md-sys-color-outline);
    user-select: none;
  }

  h2 {
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  /* Floating Action Button for Upload */
  .fab-upload {
    position: fixed;
    bottom: 32px;
    right: 32px;
    width: 64px;
    height: 64px;
    border-radius: 16px;
    border: none;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    box-shadow: 0 8px 24px rgba(103, 80, 164, 0.4);
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    overflow: visible;
  }

  .fab-upload:hover:not(:disabled) {
    transform: scale(1.1) translateY(-4px);
    box-shadow: 0 12px 32px rgba(103, 80, 164, 0.5);
  }

  .fab-upload:active:not(:disabled) {
    transform: scale(0.95);
  }

  .fab-upload:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .fab-upload.uploading {
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      box-shadow: 0 8px 24px rgba(103, 80, 164, 0.4);
    }
    50% {
      box-shadow: 0 8px 32px rgba(103, 80, 164, 0.7);
    }
  }

  .upload-badge {
    position: absolute;
    top: -8px;
    right: -8px;
    background: #ff5252;
    color: white;
    border-radius: 12px;
    padding: 4px 8px;
    font-size: 11px;
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  }

  /* Compact Drop Zone */
  .drop-zone-compact {
    border: 2px dashed var(--md-sys-color-outline);
    border-radius: 16px;
    padding: 24px;
    text-align: center;
    transition: all 0.3s ease;
    background: var(--md-sys-color-surface-container-lowest);
    margin-bottom: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    font-size: 14px;
    color: var(--md-sys-color-on-surface-variant);
    margin-bottom: 32px;
    cursor: pointer;
    position: relative;
    overflow: hidden;
  }

  .drop-zone-compact::before {
    content: "";
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(103, 80, 164, 0.1),
      transparent
    );
    transition: left 0.5s ease;
  }

  .drop-zone-compact:hover {
    border-color: var(--md-sys-color-primary);
    background: var(--md-sys-color-surface-container);
    transform: translateY(-2px);
  }

  .drop-zone-compact:hover::before {
    left: 100%;
  }

  .drop-zone-compact.drag-over {
    border-color: var(--md-sys-color-primary);
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
    border-width: 3px;
    transform: scale(1.02);
    box-shadow: 0 8px 24px rgba(103, 80, 164, 0.2);
  }

  /* Upload Progress Panel */
  .upload-progress-panel {
    background: var(--md-sys-color-surface-container);
    border-radius: 16px;
    padding: 16px;
    margin-bottom: 24px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .upload-progress-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 12px;
    font-size: 14px;
  }

  .upload-files-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .upload-file-item {
    background: var(--md-sys-color-surface-container-low);
    border-radius: 8px;
    padding: 8px 12px;
  }

  .upload-file-item.error {
    background: rgba(179, 38, 30, 0.1);
  }

  .upload-file-info {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;
  }

  .upload-filename {
    flex: 1;
    font-size: 13px;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .upload-size {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  .upload-progress-bar {
    height: 4px;
    background: var(--md-sys-color-surface-container-highest);
    border-radius: 2px;
    overflow: hidden;
  }

  .upload-progress-fill {
    height: 100%;
    background: var(--md-sys-color-primary);
    transition: width 0.3s ease;
  }

  /* Multi-select UI */
  .btn-multiselect-toggle {
    background: transparent;
    border: none;
    color: var(--md-sys-color-on-surface);
    cursor: pointer;
    padding: 8px 12px;
    border-radius: 8px;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .btn-multiselect-toggle:hover {
    background: var(--md-sys-color-surface-container-highest);
  }

  .btn-multiselect-toggle.active {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .multiselect-toolbar {
    background: var(--md-sys-color-primary-container);
    border-radius: 16px;
    padding: 12px 16px;
    margin-bottom: 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .selected-count {
    font-weight: 500;
    color: var(--md-sys-color-on-primary-container);
  }

  .multiselect-actions {
    display: flex;
    gap: 8px;
  }

  .btn-multiselect {
    background: var(--md-sys-color-surface);
    border: none;
    color: var(--md-sys-color-on-surface);
    cursor: pointer;
    padding: 8px 12px;
    border-radius: 8px;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    font-weight: 500;
  }

  .btn-multiselect:hover {
    background: var(--md-sys-color-surface-container-high);
    transform: translateY(-1px);
  }

  .btn-multiselect.btn-danger {
    background: var(--md-sys-color-error);
    color: var(--md-sys-color-on-error);
  }

  .btn-multiselect.btn-danger:hover {
    background: var(--md-sys-color-error-container);
    color: var(--md-sys-color-on-error-container);
  }

  .file-checkbox {
    position: absolute;
    top: 8px;
    left: 8px;
    background: var(--md-sys-color-surface);
    border-radius: 4px;
    padding: 4px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    z-index: 10;
  }

  .file-card.selected {
    border: 2px solid var(--md-sys-color-primary);
    background: var(--md-sys-color-primary-container);
  }

  .drop-zone:hover,
  .drop-zone.drag-over {
    background: rgba(103, 80, 164, 0.15);
    border-color: var(--md-sys-color-secondary);
    transform: scale(1.01);
  }

  .drop-icon {
    font-size: 48px;
    margin-bottom: 16px;
  }

  .drop-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 8px;
  }

  .drop-subtitle {
    font-size: 14px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .loading {
    text-align: center;
    padding: 80px 20px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .spinner {
    width: 48px;
    height: 48px;
    border: 4px solid var(--md-sys-color-outline);
    border-top-color: var(--md-sys-color-primary);
    border-radius: 50%;
    margin: 0 auto 16px;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .empty-state {
    text-align: center;
    padding: 80px 20px;
  }

  .empty-icon {
    font-size: 64px;
    margin-bottom: 16px;
  }

  .empty-title {
    font-size: 20px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 8px;
  }

  .empty-subtitle {
    color: var(--md-sys-color-on-surface-variant);
  }

  .file-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 20px;
  }

  .file-card {
    background: var(--md-sys-color-surface);
    border-radius: 20px;
    padding: 20px;
    box-shadow: var(--md-elevation-1);
    border: 1px solid var(--md-sys-color-outline);
    transition: all 0.2s ease;
    cursor: default;
    animation: fadeInUp 0.3s ease-out backwards;
  }

  /* Stagger animation for grid items */
  .file-card:nth-child(1) {
    animation-delay: 0s;
  }
  .file-card:nth-child(2) {
    animation-delay: 0.05s;
  }
  .file-card:nth-child(3) {
    animation-delay: 0.1s;
  }
  .file-card:nth-child(4) {
    animation-delay: 0.15s;
  }
  .file-card:nth-child(5) {
    animation-delay: 0.2s;
  }
  .file-card:nth-child(6) {
    animation-delay: 0.25s;
  }

  @keyframes fadeInUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .file-card.folder {
    cursor: pointer;
    background: var(--md-sys-color-secondary-container);
    border-color: var(--md-sys-color-secondary);
  }

  .file-card:hover {
    box-shadow: var(--md-elevation-3);
    transform: translateY(-4px);
    border-color: var(--md-sys-color-primary);
  }

  .file-card.folder:hover {
    background: var(--md-sys-color-tertiary-container);
    border-color: var(--md-sys-color-tertiary);
  }

  .file-card.drag-over {
    background: var(--md-sys-color-primary-container);
    border: 2px dashed var(--md-sys-color-primary);
    box-shadow: var(--md-elevation-4);
    transform: scale(1.05);
    animation: pulse 0.6s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
      box-shadow: var(--md-elevation-4);
    }
    50% {
      opacity: 0.9;
      box-shadow: var(--md-elevation-5);
    }
  }

  /* Dragging state */
  .file-card[draggable="true"]:active {
    cursor: grabbing !important;
  }

  .file-card.selected {
    background: var(--md-sys-color-secondary-container);
    border-color: var(--md-sys-color-secondary);
  }

  .file-icon {
    display: flex;
    justify-content: center;
    margin-bottom: 12px;
    color: var(--md-sys-color-primary);
  }

  .file-card.folder .file-icon {
    color: var(--md-sys-color-tertiary);
  }

  .file-thumbnail {
    width: 100%;
    height: 120px;
    border-radius: 12px;
    overflow: hidden;
    background: var(--md-sys-color-surface-variant);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .file-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s ease;
  }

  .file-card:hover .file-thumbnail img {
    transform: scale(1.05);
  }

  .file-name {
    font-weight: 500;
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 8px;
  }

  .file-meta {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    margin-bottom: 8px;
  }

  .file-badges {
    display: flex;
    gap: 6px;
    justify-content: center;
    margin-bottom: 12px;
    flex-wrap: wrap;
  }

  .badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .badge-comments {
    background: #e3f2fd;
    color: #1976d2;
  }

  .badge-tags {
    background: #fff3e0;
    color: #f57c00;
  }

  .file-actions {
    display: flex;
    gap: 8px;
    justify-content: center;
  }

  .btn-icon {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: none;
    background: var(--md-sys-color-surface-variant);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
  }

  .btn-icon:hover {
    background: var(--md-sys-color-secondary-container);
    transform: scale(1.1);
  }

  .btn-icon.btn-favorite {
    color: var(--md-sys-color-tertiary);
  }

  .btn-icon.btn-favorite:hover {
    background: var(--md-sys-color-tertiary-container);
    color: var(--md-sys-color-on-tertiary-container);
  }

  /* List View Styles */
  .file-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .file-row {
    background: var(--md-sys-color-surface);
    border-radius: 12px;
    padding: 16px 20px;
    box-shadow: var(--md-elevation-1);
    border: 1px solid var(--md-sys-color-outline);
    display: flex;
    align-items: center;
    gap: 16px;
    transition: all 0.2s ease;
    cursor: default;
  }

  .file-row.folder {
    cursor: pointer;
    background: var(--md-sys-color-secondary-container);
    border-color: var(--md-sys-color-secondary);
  }

  .file-row:hover {
    box-shadow: var(--md-elevation-2);
    border-color: var(--md-sys-color-primary);
  }

  .file-row.folder:hover {
    background: var(--md-sys-color-tertiary-container);
    border-color: var(--md-sys-color-tertiary);
  }

  .file-row.drag-over {
    background: var(--md-sys-color-primary-container);
    border: 2px dashed var(--md-sys-color-primary);
    box-shadow: var(--md-elevation-3);
  }

  .file-icon-small {
    display: flex;
    align-items: center;
    min-width: 32px;
    color: var(--md-sys-color-primary);
  }

  .file-row.folder .file-icon-small {
    color: var(--md-sys-color-tertiary);
  }

  .file-name-list {
    flex: 1;
    font-weight: 500;
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-size-list {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    min-width: 80px;
    text-align: right;
  }

  .file-actions-list {
    display: flex;
    gap: 8px;
  }

  .btn-icon-small {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: none;
    background: var(--md-sys-color-surface-variant);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
  }

  .btn-icon-small:hover {
    background: var(--md-sys-color-secondary-container);
    transform: scale(1.1);
  }

  /* Search Mode Indicator */
  .search-mode-indicator {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 20px;
    margin: 16px 0;
    background: linear-gradient(
      135deg,
      var(--md-sys-color-primary-container) 0%,
      var(--md-sys-color-secondary-container) 100%
    );
    border-radius: 12px;
    border-left: 4px solid var(--md-sys-color-primary);
    animation: slideDown 0.3s ease;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .search-icon {
    font-size: 20px;
  }

  .search-info {
    flex: 1;
    color: var(--md-sys-color-on-primary-container);
    font-size: 14px;
  }

  .search-info strong {
    font-weight: 600;
    color: var(--md-sys-color-primary);
  }

  .btn-clear-search {
    padding: 6px 16px;
    border-radius: 20px;
    border: none;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-clear-search:hover {
    background: var(--md-sys-color-secondary);
    transform: scale(1.05);
  }

  /* ============================================
     MOBILE & RESPONSIVE STYLES
     ============================================ */

  /* Tablet breakpoint (768px - 1024px) */
  @media (max-width: 1024px) {
    .files-view {
      padding: 24px 16px;
    }

    .file-grid {
      grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
      gap: 16px;
    }

    .page-header {
      flex-direction: column;
      gap: 16px;
      align-items: stretch;
    }

    .header-actions {
      justify-content: space-between;
    }

    .filter-badge {
      top: -6px;
      right: -6px;
    }
  }

  /* Mobile breakpoint (< 768px) */
  @media (max-width: 768px) {
    .files-view {
      padding: 16px 12px;
    }

    .page-header h2 {
      font-size: 20px;
    }

    .file-grid {
      grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
      gap: 12px;
    }

    .file-card {
      padding: 16px;
      border-radius: 16px;
    }

    .file-name {
      font-size: 13px;
    }

    .file-meta {
      font-size: 11px;
    }

    .file-actions {
      gap: 8px;
    }

    .btn-icon {
      width: 32px;
      height: 32px;
    }

    /* Hide some buttons on mobile */
    .btn-view-toggle,
    .btn-filter {
      width: 36px;
      height: 36px;
    }

    /* Simplify breadcrumbs on mobile */
    .breadcrumb-nav {
      gap: 4px;
      flex-wrap: wrap;
    }

    .breadcrumb-item {
      padding: 6px 12px;
      font-size: 12px;
    }

    /* Floating action button positioning */
    .fab-upload {
      bottom: 20px;
      right: 20px;
      width: 56px;
      height: 56px;
    }

    /* List view optimized for mobile */
    .file-row {
      padding: 12px;
      gap: 12px;
    }

    .file-name-list {
      font-size: 14px;
    }

    .file-size-list {
      font-size: 12px;
    }

    /* Drop zone on mobile */
    .drop-zone-compact {
      padding: 16px;
      font-size: 13px;
    }

    /* Multi-select toolbar mobile */
    .multi-select-toolbar {
      padding: 12px;
      gap: 8px;
    }

    /* Touch-friendly tap targets */
    .file-card,
    .file-row {
      min-height: 44px; /* iOS minimum tap target */
    }
  }

  /* Small mobile (< 480px) */
  @media (max-width: 480px) {
    .files-view {
      padding: 12px 8px;
    }

    .file-grid {
      grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
      gap: 8px;
    }

    .file-card {
      padding: 12px;
    }

    .page-header h2 {
      font-size: 18px;
    }

    /* Stack header actions vertically on very small screens */
    .header-actions {
      flex-wrap: wrap;
    }

    /* Simplify file list */
    .file-icon {
      font-size: 32px;
    }

    /* Compact stats */
    .stats-summary {
      flex-direction: column;
    }

    /* Hide less important UI on tiny screens */
    .file-meta {
      display: none;
    }

    /* Bottom sheet style for modals on mobile */
    :global(.modal-content) {
      border-radius: 24px 24px 0 0 !important;
      position: fixed !important;
      bottom: 0 !important;
      top: auto !important;
      left: 0 !important;
      right: 0 !important;
      max-height: 90vh !important;
      animation: slideUp 0.3s ease-out !important;
    }

    @keyframes slideUp {
      from {
        transform: translateY(100%);
      }
      to {
        transform: translateY(0);
      }
    }
  }

  /* Landscape mobile adjustments */
  @media (max-width: 768px) and (orientation: landscape) {
    .file-grid {
      grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    }

    .fab-upload {
      bottom: 12px;
      right: 12px;
      width: 48px;
      height: 48px;
    }
  }

  /* Dark mode adjustments for mobile */
  @media (max-width: 768px) and (prefers-color-scheme: dark) {
    .file-card {
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
    }

    .drop-zone-compact {
      background: var(--md-sys-color-surface-container-high);
    }
  }

  /* Reduce animations on mobile for performance */
  @media (max-width: 768px) {
    .file-card {
      animation-duration: 0.2s;
    }

    .file-card:hover {
      transform: translateY(-2px); /* Reduced transform */
    }
  }

  /* Safe area insets for notched devices */
  @supports (padding: max(0px)) {
    .files-view {
      padding-left: max(12px, env(safe-area-inset-left));
      padding-right: max(12px, env(safe-area-inset-right));
      padding-bottom: max(12px, env(safe-area-inset-bottom));
    }

    .fab-upload {
      bottom: max(20px, env(safe-area-inset-bottom));
      right: max(20px, env(safe-area-inset-right));
    }
  }

  /* Move Dialog Styles */
  .folder-list {
    max-height: 400px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px;
  }

  .folder-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border: none;
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
    cursor: pointer;
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
    transition: all 0.2s ease;
    text-align: left;
    width: 100%;
  }

  .folder-item:hover {
    background: var(--md-sys-color-surface-container-high);
    transform: translateX(4px);
  }

  .folder-item :global(.icon) {
    font-size: 20px;
    color: var(--md-sys-color-primary);
  }

  .empty-state {
    padding: 40px;
    text-align: center;
    color: var(--md-sys-color-on-surface-variant);
  }

  /* Properties Dialog Styles */
  .properties-grid {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 8px;
  }

  .prop-row {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 16px;
    align-items: center;
    padding: 12px;
    background: var(--md-sys-color-surface-container-low);
    border-radius: 8px;
  }

  .prop-label {
    font-weight: 600;
    color: var(--md-sys-color-on-surface-variant);
    font-size: 13px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .prop-value {
    color: var(--md-sys-color-on-surface);
    font-size: 14px;
    word-break: break-word;
  }
</style>

