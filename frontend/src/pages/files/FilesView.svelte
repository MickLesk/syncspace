<script>
  import { onMount, onDestroy } from "svelte";
  import { files, currentPath, currentLang } from "../../stores/ui";
  import { favorites } from "../../stores/favorites";
  import { userPreferences, preferences } from "../../stores/preferences";
  import { success, error as errorToast } from "../../stores/toast";
  import { modals, modalEvents } from "../../stores/modals";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import Breadcrumbs from "../../components/Breadcrumbs.svelte";
  import SearchFilters from "../../components/search/SearchFilters.svelte";
  import UploadManager from "../../components/files/UploadManager.svelte";
  import Modal from "../../components/ui/Modal.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";
  import VirtualList from "../../components/ui/VirtualList.svelte";
  import FileCard from "../../components/files/FileCard.svelte";
  import FileToolbar from "../../components/files/FileToolbar.svelte";
  import FileActionsMenu from "../../components/files/FileActionsMenu.svelte";
  import FilePreviewPanel from "../../components/files/FilePreviewPanel.svelte";
  import BulkTaggingModal from "../../components/files/BulkTaggingModal.svelte";
  import BatchRenameModal from "../../components/files/BatchRenameModal.svelte";
  import BatchProgressPanel from "../../components/files/BatchProgressPanel.svelte";
  import CopyFileModal from "../../components/modals/CopyFileModal.svelte";
  import MoveFileModal from "../../components/modals/MoveFileModal.svelte";
  import FileEditorModal from "../../components/editor/FileEditorModal.svelte";
  import TemplateLibraryModal from "../../components/templates/TemplateLibraryModal.svelte";
  import api from "../../lib/api";
  import { websocketManager } from "@stores/websocket.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let searchQuery = $state("");
  let viewMode = $state("grid");
  let sortBy = $state("name");
  let sortOrder = $state("asc");
  let showFoldersOnly = $state(false);
  let showFavoritesOnly = $state(false);
  let selectedFiles = $state(new Set());
  let uploadProgress = $state([]);
  let selectionMode = $state(false);

  // Search State
  let isSearchMode = $state(false);
  let searchResults = $state([]);
  let searchLoading = $state(false);

  // Context Menu
  let contextMenu = $state(null);
  let contextMenuPosition = $state({ x: 0, y: 0 });

  // Operation State
  let currentFile = $state(null);

  // Preview Panel State
  let showPreviewPanel = $state(false);
  let previewFile = $state(null);
  let previewFileIndex = $state(0);

  // Copy/Move Modal State
  let showCopyModal = $state(false);
  let selectedFileForCopy = $state(null);
  let showMoveModal = $state(false);
  let selectedFileForMove = $state(null);

  // File Editor Modal State
  let showEditorModal = $state(false);
  let fileToEdit = $state(null);

  // Template Library Modal State
  let showTemplateLibrary = $state(false);

  // Bulk Tagging Modal State
  let showBulkTaggingModal = $state(false);

  // Batch Rename Modal State
  let showBatchRenameModal = $state(false);

  // Batch Progress Panel State
  let showBatchProgress = $state(false);

  let searchFilters = $state({
    type: "all",
    dateFrom: null,
    dateTo: null,
    minSize: null,
    maxSize: null,
  });

  // Computed: Filtered & Sorted Files
  let displayFiles = $derived(() => {
    // If in search mode, use search results instead
    if (isSearchMode) {
      return searchResults;
    }

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

    // Apply favorites filter
    if (showFavoritesOnly) {
      // Use path as consistent identifier (same as FileCard)
      result = result.filter((f) => {
        const itemId = f.path || f.file_path || f.name;
        return favorites.isFavorite(itemId);
      });
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
  let handlePopstateRef = null;
  let preferencesLoaded = $state(false);

  // Save view mode to backend when it changes (only after initial load)
  $effect(() => {
    if (viewMode && preferencesLoaded) {
      console.log("💾 Saving view mode to backend:", viewMode);
      userPreferences.updatePreference("view_mode", viewMode);
    }
  });

  onMount(async () => {
    // Load preferences and favorites
    try {
      await userPreferences.load();

      // Subscribe to preferences store to get current values
      const unsubscribe = preferences.subscribe((prefs) => {
        if (prefs && typeof prefs.view_mode === "string") {
          viewMode = prefs.view_mode;
        }
      });
      unsubscribe(); // Immediately unsubscribe after getting value

      preferencesLoaded = true; // Enable auto-save after loading
    } catch (err) {
      console.error("Failed to load preferences:", err);
      preferencesLoaded = true; // Enable even if load fails
    }

    try {
      await favorites.load();
    } catch (err) {
      console.error("Failed to load favorites:", err);
    } // Initialize from URL hash or current path
    const urlPath =
      window.location.hash.replace("#/files", "").replace("#", "") || "/";
    currentPath.set(urlPath);

    await loadFiles(urlPath);

    unsubscribeFileEvent = websocketManager.on("file_change", (event) => {
      console.log("File event:", event);
      loadFiles();
    });

    // Handle browser back/forward navigation
    handlePopstateRef = () => {
      const urlPath =
        window.location.hash
          .replace("#/files/", "")
          .replace("#/files", "")
          .replace("#", "") || "/";
      console.log("[FilesView] Popstate triggered, navigating to:", urlPath);

      // Update currentPath store
      const cleanPath = urlPath.replace(/^\/+|\/+$/g, "");
      currentPath.set(cleanPath ? `/${cleanPath}` : "/");

      // Load files for the new path
      loadFiles(urlPath);
      selectedFiles = new Set();
    };

    window.addEventListener("keydown", handleKeydown);
    window.addEventListener("popstate", handlePopstateRef);
    window.addEventListener("openFileFromSearch", handleOpenFileFromSearch);
  });

  // Define handleOpenFileFromSearch outside of onMount so it can be referenced in onMount and onDestroy
  const handleOpenFileFromSearch = async (event) => {
    const { filePath, fileName, fileId, isFolder } = event.detail;
    console.log("[FilesView] Opening file from search:", event.detail);

    // Extract directory path and filename
    const pathParts = filePath.split("/").filter((p) => p);
    const fileNameFromPath = pathParts.pop() || fileName;
    const directoryPath =
      pathParts.length > 0 ? "/" + pathParts.join("/") : "/";

    console.log(
      "[FilesView] Parsed - directory:",
      directoryPath,
      "filename:",
      fileNameFromPath
    );

    // Navigate to the directory containing the file
    if (directoryPath !== $currentPath) {
      console.log("[FilesView] Navigating to directory:", directoryPath);
      await navigateTo(directoryPath);
      // Warte nur kurz - die Animation soll schneller kommen!
      await new Promise((resolve) => setTimeout(resolve, 400));
    } else {
      // Already in correct directory, fast animation
      await new Promise((resolve) => setTimeout(resolve, 100));
    }

    // Find the file in the current file list (use $files to get the actual array from the store)
    const currentFiles = $files; // Get current value from store
    console.log("[FilesView] Current files array:", currentFiles);

    const targetFile = currentFiles.find(
      (f) =>
        f.name === fileNameFromPath ||
        f.path === filePath ||
        f.file_path === filePath ||
        f.id === fileId
    );

    console.log("[FilesView] Target file found:", targetFile);

    if (targetFile) {
      if (isFolder) {
        // If it's a folder, navigate into it
        console.log("[FilesView] Navigating into folder:", filePath);
        navigateTo(filePath);
      } else {
        // If it's a file, highlight it (user can click to open)
        console.log("[FilesView] Highlighting file:", targetFile.name);

        // Scroll to file card - sofort ohne Verzögerung!
        const scrollAndHighlight = () => {
          console.log(
            "[FilesView] Searching for file card with data-file-name:",
            targetFile.name
          );
          const fileCards = document.querySelectorAll("[data-file-name]");
          console.log(
            "[FilesView] Found",
            fileCards.length,
            "file cards in DOM"
          );

          for (const card of fileCards) {
            const cardName = card.getAttribute("data-file-name");
            if (cardName === targetFile.name) {
              console.log(
                "[FilesView] ✅ Found matching card, scrolling and highlighting"
              );
              card.scrollIntoView({ behavior: "smooth", block: "center" });
              // Add highlight effect (5 Sekunden!)
              card.classList.add("highlight-search-result");
              setTimeout(
                () => card.classList.remove("highlight-search-result"),
                5000 // 5 Sekunden Animation!
              );
              break;
            }
          }
        };

        // Versuche sofort, wenn nicht gefunden, warte kurz und versuche nochmal
        scrollAndHighlight();
        setTimeout(scrollAndHighlight, 150); // Backup falls DOM noch nicht bereit

        // Don't open preview automatically - let user click to open
      }
    } else {
      console.warn(
        "[FilesView] Could not find file in current directory:",
        fileNameFromPath
      );
      errorToast(tr("fileNotFound"));
    }
  };

  onDestroy(() => {
    if (unsubscribeFileEvent) unsubscribeFileEvent();
    if (handlePopstateRef) {
      window.removeEventListener("popstate", handlePopstateRef);
    }
    window.removeEventListener("keydown", handleKeydown);
    window.removeEventListener("openFileFromSearch", handleOpenFileFromSearch);
  });

  // Modal Event Listeners - Listen to events from ModalPortal
  onMount(() => {
    const unsubUpload = modalEvents.on("upload", handleUpload);
    const unsubCreateFolder = modalEvents.on("createFolder", createNewFolder);
    const unsubRename = modalEvents.on("renameFile", ({ file, newName }) => {
      renameFile(file, newName);
    });
    const unsubDelete = modalEvents.on("deleteFile", deleteFile);
    const unsubSearch = modalEvents.on("search", handleAdvancedSearch);
    const unsubChangeFolderColor = modalEvents.on(
      "changeFolderColor",
      async ({ file, color }) => {
        console.log("[FilesView] Changing folder color:", {
          file_path: file.path,
          name: file.name,
          color: color,
        });

        try {
          // Save to backend database
          await api.folderColors.set(file.path, color);
          console.log("[FilesView] Folder color saved to backend");
          success(tr("folderColorUpdated"));

          // Refresh the file list to show new color
          await loadFiles();
        } catch (err) {
          console.error("[FilesView] Failed to save folder color:", err);
          errorToast(tr("failedToUpdateFolderColor"));
        }
      }
    );
    const unsubMove = modalEvents.on(
      "moveFile",
      async ({ file, destinationPath }) => {
        console.log("[FilesView] Moving file:", {
          file_path: file.path || file.file_path || file.name,
          destination: destinationPath,
        });

        try {
          const sourcePath = file.path || file.file_path || file.name;
          const destPath = destinationPath
            ? `${destinationPath}/${file.name}`
            : file.name;

          await api.files.move(sourcePath, destPath);
          success(tr("moved", file.name, destinationPath || tr("root")));

          // Refresh the file list
          await loadFiles();
        } catch (err) {
          console.error("[FilesView] Failed to move file:", err);
          errorToast(err.message || tr("failedToMoveFile"));
        }
      }
    );

    // FilePreview event handlers
    const handleRenameFromPreview = (e) => {
      const { file, newName } = e.detail;
      renameFile(file, newName);
    };
    const handleCopyFromPreview = (e) => {
      const { file } = e.detail;
      // Open copy modal for the file
      modals.openCopy(file);
    };
    const handleToggleFavoriteFromPreview = async (e) => {
      const { file } = e.detail;
      try {
        if (favorites.has(file.id)) {
          await favorites.remove(file.id);
        } else {
          await favorites.add(file.id, "file");
        }
      } catch (err) {
        errorToast(tr("failedToToggleFavorite"));
      }
    };

    window.addEventListener("renameFile", handleRenameFromPreview);
    window.addEventListener("copyFile", handleCopyFromPreview);
    window.addEventListener("toggleFavorite", handleToggleFavoriteFromPreview);

    return () => {
      unsubUpload();
      unsubCreateFolder();
      unsubRename();
      unsubDelete();
      unsubSearch();
      unsubChangeFolderColor();
      unsubMove();
      window.removeEventListener("renameFile", handleRenameFromPreview);
      window.removeEventListener("copyFile", handleCopyFromPreview);
      window.removeEventListener(
        "toggleFavorite",
        handleToggleFavoriteFromPreview
      );
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
      errorToast(tr("failedToLoadFiles"));
      console.error(err);
    } finally {
      loading = false;
    }
  }

  function navigateTo(pathOrEvent) {
    // Exit search mode when navigating
    if (isSearchMode) {
      exitSearchMode();
    }

    // Handle both direct path string and event object with {path: "..."}
    const path =
      typeof pathOrEvent === "string"
        ? pathOrEvent
        : pathOrEvent?.path || pathOrEvent?.detail?.path || "/";

    // Update browser history for back/forward navigation
    const cleanPath = path.replace(/^\/+|\/+$/g, ""); // Remove leading/trailing slashes
    const newHash = cleanPath ? `#/files/${cleanPath}` : "#/files";

    // Update URL and add to history (allows browser back/forward)
    if (window.location.hash !== newHash) {
      window.history.pushState({ path: cleanPath }, "", newHash);
    }

    // Update currentPath store
    currentPath.set(cleanPath ? `/${cleanPath}` : "/");

    loadFiles(path);
    selectedFiles = new Set();
  }

  // Advanced Search Handler
  async function handleAdvancedSearch(event) {
    const {
      query,
      filters,
      sortBy: searchSortBy,
      sortOrder: searchSortOrder,
    } = event.detail;

    console.log("[FilesView] Advanced search triggered:", {
      query,
      filters,
      searchSortBy,
      searchSortOrder,
    });

    searchLoading = true;
    isSearchMode = true;

    try {
      // Call backend search API
      const results = await api.search.query(query, 100, true);

      console.log("[FilesView] Search results:", results);

      // Transform results to match file structure
      searchResults = (results || []).map((file) => ({
        id: file.id || file.path,
        name: file.name || file.filename,
        path: file.path || file.file_path,
        is_directory: file.is_directory || false,
        size_bytes: file.size_bytes || file.size || 0,
        modified_at:
          file.modified_at || file.last_modified || new Date().toISOString(),
        mime_type: file.mime_type || "application/octet-stream",
      }));

      // Apply additional filters from advanced search
      if (filters.fileType && filters.fileType !== "all") {
        searchResults = searchResults.filter((file) => {
          if (filters.fileType === "folder") return file.is_directory;
          if (filters.fileType === "file") return !file.is_directory;
          // Add more file type filters as needed
          return true;
        });
      }

      // Apply sort from search parameters
      if (searchSortBy) {
        sortBy = searchSortBy;
        sortOrder = searchSortOrder || "asc";
      }

      success(
        `${tr("found")} ${searchResults.length} ${searchResults.length === 1 ? tr("item") : tr("items")}`
      );

      // Close the search modal
      modals.close("advancedSearch");
    } catch (err) {
      console.error("[FilesView] Search failed:", err);
      errorToast(`${tr("searchFailed")}: ${err.message}`);
      searchResults = [];
    } finally {
      searchLoading = false;
    }
  }

  // Exit search mode and return to normal file browsing
  function exitSearchMode() {
    isSearchMode = false;
    searchResults = [];
    searchQuery = "";
  }

  function openFile(file) {
    if (file.is_directory) {
      // Exit search mode when navigating into a folder
      if (isSearchMode) {
        exitSearchMode();
      }
      // Navigate to the folder - use file.path for search results, construct path otherwise
      const folderPath = isSearchMode
        ? file.path
        : `${$currentPath.replace(/\/$/, "")}/${file.name}/`;
      navigateTo(folderPath);
    } else {
      // Show preview panel instead of modal
      const files = displayFiles().filter((f) => !f.is_directory);
      const index = files.findIndex((f) => f.id === file.id);
      previewFile = file;
      previewFileIndex = index >= 0 ? index : 0;
      showPreviewPanel = true;
    }
  }

  function closePreviewPanel() {
    showPreviewPanel = false;
    previewFile = null;
  }

  function navigatePreview(newIndex) {
    const files = displayFiles().filter((f) => !f.is_directory);
    if (newIndex >= 0 && newIndex < files.length) {
      previewFile = files[newIndex];
      previewFileIndex = newIndex;
    }
  }

  // Handle dropped items (files and folders)
  async function handleDroppedItems(dataTransfer) {
    const items = dataTransfer.items;
    const files = [];
    let folderName = null;

    if (items && items.length > 0) {
      // IMPORTANT: Get all entries FIRST before any async operations
      // webkitGetAsEntry() becomes invalid after the drop event ends
      const entries = [];
      for (let i = 0; i < items.length; i++) {
        if (items[i].kind === "file") {
          const entry = items[i].webkitGetAsEntry();
          if (entry) {
            entries.push(entry);
            // Track folder name if uploading a folder
            if (entry.isDirectory && !folderName) {
              folderName = entry.name;
            }
          }
        }
      }

      // Now process all entries
      for (const entry of entries) {
        await traverseFileTree(entry, "", files);
      }
    } else if (dataTransfer.files && dataTransfer.files.length > 0) {
      // Fallback to regular files
      files.push(...Array.from(dataTransfer.files));
    }

    console.log(`[DragDrop] Processing ${files.length} files`);

    if (files.length > 0) {
      handleUpload(files, folderName, files.length);
    }
  }

  // Recursively traverse file tree for folder uploads
  async function traverseFileTree(item, path, files) {
    if (item.isFile) {
      // Get the file
      const file = await new Promise((resolve) => {
        item.file((f) => {
          // Add relative path to file object
          Object.defineProperty(f, "relativePath", {
            value: path + f.name,
            writable: false,
          });
          resolve(f);
        });
      });
      files.push(file);
    } else if (item.isDirectory) {
      // Create directory first
      const dirPath = path + item.name;
      try {
        // Properly construct the full path
        const fullPath = $currentPath
          ? `${$currentPath.replace(/\/$/, "")}/${dirPath}`
          : dirPath;
        await api.files.createDir(fullPath);
        console.log("[Upload] Created directory:", fullPath);
      } catch (err) {
        console.error("[Upload] Failed to create directory:", dirPath, err);
        // Continue anyway, backend might create it on file upload
      }

      // Read directory contents
      const dirReader = item.createReader();
      const entries = await new Promise((resolve) => {
        dirReader.readEntries((entries) => resolve(entries));
      });

      // Recursively process subdirectories and files
      for (const entry of entries) {
        await traverseFileTree(entry, dirPath + "/", files);
      }
    }
  }

  async function handleUpload(
    filesToUpload,
    batchName = null,
    totalFiles = null
  ) {
    const fileList = Array.from(filesToUpload);
    const MAX_RETRIES = 3;
    const RETRY_DELAY_MS = 1000;

    // Generate batch ID for folder uploads
    const batchId = batchName ? Date.now() + Math.random() : null;

    for (let file of fileList) {
      const uploadId = Date.now() + Math.random();
      let retryCount = 0;
      let uploadSuccess = false;

      // Get relative path if available (for folder uploads)
      const relativePath =
        file.relativePath || file.webkitRelativePath || file.name;
      const displayName = relativePath.includes("/") ? relativePath : file.name;

      // Initialize upload progress entry
      uploadProgress = [
        ...uploadProgress,
        {
          id: uploadId,
          name: displayName,
          progress: 0,
          status: "uploading",
          retries: 0,
          error: null,
          size: file.size,
          batchId: batchId,
          batchName: batchName,
          batchTotal: totalFiles,
          batchIndex: fileList.indexOf(file) + 1,
        },
      ];

      while (retryCount <= MAX_RETRIES && !uploadSuccess) {
        try {
          // Update retry count in UI if retrying
          if (retryCount > 0) {
            uploadProgress = uploadProgress.map((up) =>
              up.id === uploadId
                ? {
                    ...up,
                    status: "retrying",
                    retries: retryCount,
                    progress: 0,
                  }
                : up
            );

            // Wait before retry with exponential backoff
            await new Promise((resolve) =>
              setTimeout(resolve, RETRY_DELAY_MS * retryCount)
            );
          }

          // Determine upload path (include subdirectory if present)
          let uploadPath = $currentPath;
          if (relativePath.includes("/")) {
            // Extract directory path from relative path
            const pathParts = relativePath.split("/");
            pathParts.pop(); // Remove filename
            const subdir = pathParts.join("/");
            // Properly construct path with normalized slashes
            uploadPath = $currentPath
              ? `${$currentPath.replace(/\/$/, "")}/${subdir}`
              : subdir;
          }

          // Perform upload with progress tracking
          await api.files.uploadWithProgress(uploadPath, file, (percent) => {
            uploadProgress = uploadProgress.map((up) =>
              up.id === uploadId
                ? { ...up, progress: percent, status: "uploading" }
                : up
            );
          });

          // Upload successful
          uploadProgress = uploadProgress.map((up) =>
            up.id === uploadId
              ? { ...up, status: "complete", progress: 100, error: null }
              : up
          );

          // Success is shown in UploadManager, no toast needed
          uploadSuccess = true;
        } catch (err) {
          retryCount++;
          console.error(
            `[Upload] Failed to upload ${file.name} (attempt ${retryCount}/${MAX_RETRIES + 1}):`,
            err
          );

          // Parse error message
          let errorMessage = "Unknown error";
          if (err.message) {
            errorMessage = err.message;
          } else if (typeof err === "string") {
            errorMessage = err;
          }

          // Check if we should retry
          const isNetworkError =
            errorMessage.includes("network") ||
            errorMessage.includes("timeout") ||
            errorMessage.includes("Failed to fetch");
          const shouldRetry = isNetworkError && retryCount <= MAX_RETRIES;

          if (!shouldRetry) {
            // Final failure - update UI with error
            uploadProgress = uploadProgress.map((up) =>
              up.id === uploadId
                ? {
                    ...up,
                    status: "error",
                    error: errorMessage,
                    retries: retryCount,
                  }
                : up
            );

            // Show detailed error toast
            if (retryCount > MAX_RETRIES) {
              errorToast(
                `${tr("failedToUploadFile", file.name)} - Max retries (${MAX_RETRIES}) reached. ${errorMessage}`
              );
            } else {
              errorToast(
                `${tr("failedToUploadFile", file.name)} - ${errorMessage}`
              );
            }
            break;
          }
        }
      }
    }

    // Reload files to show newly uploaded files
    await loadFiles();

    // User can manually clear completed uploads via UploadManager
  }

  function handleUploadClear(type) {
    if (type === "completed") {
      uploadProgress = uploadProgress.filter((up) => up.status !== "complete");
    } else if (type === "all") {
      uploadProgress = [];
    }
  }

  function handleUploadRetry(uploadId) {
    const upload = uploadProgress.find((up) => up.id === uploadId);
    if (upload && upload.file) {
      // Reset the upload and retry
      upload.status = "uploading";
      upload.progress = 0;
      upload.retries = (upload.retries || 0) + 1;
      uploadFiles([upload.file]);
    }
  }

  function handleUploadCancel(uploadId) {
    // Remove the upload from progress
    uploadProgress = uploadProgress.filter((up) => up.id !== uploadId);
  }

  function handleFileClick(upload) {
    // Reload files and try to select the uploaded file
    loadFiles().then(() => {
      // Find the file in the current list
      const uploadedFile = files.find((f) => f.name === upload.name);
      if (uploadedFile) {
        // If we have a file details modal, open it
        currentFile = uploadedFile;
        showFileModal = true;
      }
    });
  }

  async function createNewFolder(data) {
    const folderName = typeof data === "string" ? data : data.name;
    const folderColor = typeof data === "object" ? data.color : null;

    if (!folderName.trim()) return;

    try {
      const fullPath = $currentPath
        ? `${$currentPath}${folderName}`
        : folderName;
      await api.files.createDir(fullPath);

      // Save folder color to backend database
      if (folderColor) {
        try {
          await api.folderColors.set(fullPath, folderColor);
          console.log(
            "[FilesView] Folder color saved to backend:",
            fullPath,
            folderColor
          );
        } catch (err) {
          console.error("[FilesView] Failed to save folder color:", err);
          // Continue anyway - folder was created
        }
      }

      success(tr("folderCreated"));
      await loadFiles();
    } catch (error) {
      errorToast(tr("failedToCreateFolder"));
      console.error(error);
    }
  }

  async function renameFile(file, newName) {
    if (!newName.trim() || !file) return;

    try {
      const dir = file.file_path?.split("/").slice(0, -1).join("/") || "";
      const newPath = dir ? `${dir}/${newName}` : newName;
      await api.files.rename(file.file_path || file.name, newPath);
      success(tr("fileRenamed"));
      await loadFiles();
    } catch (error) {
      errorToast(tr("failedToRenameFile"));
      console.error(error);
    }
  }

  async function deleteFile(file) {
    if (!file) return;

    try {
      await api.files.delete(file.file_path || file.name);
      currentFile = null;
      success(tr("fileDeleted"));
      await loadFiles();
    } catch (error) {
      errorToast(tr("failedToDeleteFile"));
      console.error(error);
    }
  }

  async function batchDelete() {
    if (selectedFiles.size === 0) return;

    if (!confirm(tr("deleteFilesConfirm", selectedFiles.size))) return;

    for (const filePath of selectedFiles) {
      try {
        await api.files.delete(filePath);
      } catch (error) {
        errorToast(`Failed to delete: ${filePath}`);
        console.error(error);
      }
    }

    selectedFiles = new Set();
    success(tr("filesDeleted"));
    await loadFiles();
  }

  // Drag & Drop Handler
  // Handle drop on breadcrumb items
  async function handleBreadcrumbDrop(draggedFile, destinationPath) {
    try {
      console.log(
        "[FilesView] Dropping file on breadcrumb:",
        draggedFile.name,
        "to",
        destinationPath
      );

      // Build source path
      const sourcePath =
        draggedFile.path || draggedFile.file_path || draggedFile.name;

      // Build destination path
      const cleanDestPath = destinationPath.replace(/^\/+|\/+$/g, "");
      const destPath = cleanDestPath
        ? `${cleanDestPath}/${draggedFile.name}`
        : draggedFile.name;

      console.log("[FilesView] Moving:", sourcePath, "to", destPath);

      // Call backend move API
      await api.files.move(sourcePath, destPath);
      success(tr("moved", draggedFile.name, destinationPath || tr("root")));

      // Refresh file list
      await loadFiles();
    } catch (err) {
      console.error("[FilesView] Failed to move file:", err);
      errorToast(err.message || tr("failedToMoveFile"));
    }
  }

  async function handleFileDrop(draggedFile, targetFolder) {
    try {
      console.log(
        "[FilesView] Dropping file:",
        draggedFile.name,
        "into",
        targetFolder.name
      );

      // Get current path components
      const currentDir = $currentPath.replace(/^\/+|\/+$/g, "");

      // Build source path
      const sourcePath =
        draggedFile.path || draggedFile.file_path || draggedFile.name;

      // Build destination path (inside target folder)
      const targetFolderPath =
        targetFolder.path || targetFolder.file_path || targetFolder.name;
      const destPath = targetFolderPath
        ? `${targetFolderPath}/${draggedFile.name}`
        : draggedFile.name;

      console.log("[FilesView] Moving:", sourcePath, "to", destPath);

      // Call backend move API
      await api.files.move(sourcePath, destPath);

      success(tr("moved", draggedFile.name, targetFolder.name));
      await loadFiles();
    } catch (err) {
      console.error("[FilesView] Failed to move file:", err);
      errorToast(`Failed to move ${draggedFile.name}`);
    }
  }

  async function downloadFile(file) {
    const filePath = file.file_path || file.path || file.name;
    const encodedPath = filePath
      .split("/")
      .map((segment) => encodeURIComponent(segment))
      .join("/");

    try {
      // Use API to download file
      const response = await fetch(
        `${new URL(window.location.href).protocol}//${new URL(window.location.href).hostname}:8080/api/file/${encodedPath}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );

      if (!response.ok) {
        throw new Error(`Download failed: ${response.status}`);
      }

      // Create blob and download
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = file.name || "download";
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);

      success(tr("fileDownloaded"));
    } catch (err) {
      errorToast(`${tr("downloadFailed")}: ${err.message}`);
    }
  }

  function openFileEditor(file) {
    // Only open editor for text-based files
    const editableExtensions = [
      "txt",
      "md",
      "json",
      "js",
      "jsx",
      "ts",
      "tsx",
      "py",
      "rs",
      "go",
      "java",
      "cpp",
      "c",
      "cs",
      "php",
      "rb",
      "swift",
      "kt",
      "scala",
      "sh",
      "bash",
      "sql",
      "html",
      "css",
      "scss",
      "sass",
      "less",
      "xml",
      "yaml",
      "yml",
      "log",
      "toml",
      "ini",
      "cfg",
      "conf",
    ];

    const ext = file.name.split(".").pop()?.toLowerCase();
    if (!editableExtensions.includes(ext)) {
      errorToast(tr("fileNotEditable"));
      return;
    }

    fileToEdit = file;
    showEditorModal = true;
  }

  function handleEditorSave() {
    // Reload files after save
    loadFiles($currentPath);
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
      modals.openUpload();
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

    // Handle both files and folders
    handleDroppedItems(e.dataTransfer);
  }}
  ondragover={(e) => e.preventDefault()}
/>

<PageWrapper>
  <div class="p-4 md:p-6">
    <!-- Search Mode Banner -->
    {#if isSearchMode}
      <div
        class="mb-4 bg-gradient-to-r from-green-500/10 via-emerald-500/10 to-teal-500/10 dark:from-green-500/20 dark:via-emerald-500/20 dark:to-teal-500/20 backdrop-blur-sm border border-green-200/50 dark:border-green-700/50 rounded-2xl p-4 flex items-center justify-between shadow-lg"
      >
        <div class="flex items-center gap-3">
          <svg
            class="w-5 h-5 text-green-600 dark:text-green-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <div>
            <div class="text-sm font-medium text-green-900 dark:text-green-100">
              {tr("searchResults")}
            </div>
            <div class="text-xs text-green-700 dark:text-green-300">
              {tr("found")}
              {searchResults.length}
              {searchResults.length === 1 ? tr("item") : tr("items")}
              {tr("matchingYourSearch")}
            </div>
          </div>
        </div>
        <button
          onclick={exitSearchMode}
          class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white text-sm font-medium rounded-lg transition-colors flex items-center gap-2"
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
          {tr("exitSearch")}
        </button>
      </div>
    {/if}

    <!-- Unified Toolbar -->
    <FileToolbar
      bind:viewMode
      bind:sortBy
      bind:sortOrder
      bind:showFoldersOnly
      bind:showFavoritesOnly
      {selectionMode}
      selectedCount={selectedFiles.size}
      selectedFiles={Array.from(selectedFiles)}
      onRefresh={loadFiles}
      onUpload={() => modals.openUpload()}
      onNewFolder={() => modals.openNewFolder()}
      onNewFromTemplate={() => (showTemplateLibrary = true)}
      onAdvancedSearch={() => modals.openAdvancedSearch()}
      onSelectionToggle={toggleSelectionMode}
      onBatchDelete={batchDelete}
      onBatchTag={() => (showBulkTaggingModal = true)}
      onBatchRename={() => (showBatchRenameModal = true)}
    />

    <!-- Breadcrumbs (below toolbar) -->
    {#if !isSearchMode}
      <div
        class="mb-4 bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm rounded-xl shadow-md border border-gray-100 dark:border-gray-700 px-3"
      >
        <Breadcrumbs
          path={$currentPath}
          on:navigate={(e) => navigateTo(e.detail.path)}
          onDrop={handleBreadcrumbDrop}
        />
      </div>
    {/if}

    <!-- Upload Manager -->
    <UploadManager
      uploads={uploadProgress}
      onClear={handleUploadClear}
      onRetry={handleUploadRetry}
      onCancel={handleUploadCancel}
      onFileClick={handleFileClick}
    />

    <!-- File Grid/List -->
    {#if loading || searchLoading}
      <LoadingState
        variant={viewMode}
        count={8}
        message={searchLoading ? tr("searchingFiles") : tr("loadingFiles")}
      />
    {:else if displayFiles().length === 0}
      <EmptyState
        icon={isSearchMode ? "🔍" : "📂"}
        title={isSearchMode
          ? tr("noFilesMatch")
          : searchQuery
            ? tr("noFilesMatch")
            : tr("folderIsEmpty")}
        description={isSearchMode
          ? tr("tryAdjustingSearch")
          : searchQuery
            ? tr("tryAdjustingSearch")
            : tr("uploadFilesOrCreate")}
        actionText={isSearchMode ? tr("newSearch") : tr("uploadFiles")}
        onAction={() =>
          isSearchMode ? modals.openAdvancedSearch() : modals.openUpload()}
      />
    {:else}
      <!-- Use VirtualList for medium-large file lists (>50 items) for better performance -->
      {#if displayFiles().length > 50}
        <div
          class="virtual-list-wrapper"
          style="height: calc(100vh - 280px); min-height: 400px;"
        >
          <VirtualList
            items={displayFiles()}
            itemHeight={viewMode === "grid" ? 220 : 72}
            isGrid={viewMode === "grid"}
            persistKey={$currentPath}
            overscan={5}
            let:item={file}
            let:index
          >
            <FileCard
              {file}
              {viewMode}
              {selectionMode}
              selected={selectedFiles.has(file.file_path || file.name)}
              onSelect={() => handleFileSelection(file)}
              onOpen={() => openFile(file)}
              onContextMenu={(f, e) => handleContextMenu(f, e)}
              onDragStart={(f) =>
                console.log("[FilesView] Drag started:", f.name)}
              onDragEnd={(f) => console.log("[FilesView] Drag ended:", f.name)}
              onDrop={(draggedFile, targetFolder) =>
                handleFileDrop(draggedFile, targetFolder)}
            />
          </VirtualList>
        </div>
      {:else}
        <!-- Regular rendering for small file lists (≤50 items) -->
        <div
          class={viewMode === "grid"
            ? "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 grid-stagger"
            : "flex flex-col gap-2 list-stagger"}
        >
          {#each displayFiles() as file (file.file_path || file.name)}
            <FileCard
              {file}
              {viewMode}
              {selectionMode}
              selected={selectedFiles.has(file.file_path || file.name)}
              onSelect={() => handleFileSelection(file)}
              onOpen={() => openFile(file)}
              onContextMenu={(f, e) => handleContextMenu(f, e)}
              onDragStart={(f) =>
                console.log("[FilesView] Drag started:", f.name)}
              onDragEnd={(f) => console.log("[FilesView] Drag ended:", f.name)}
              onDrop={(draggedFile, targetFolder) =>
                handleFileDrop(draggedFile, targetFolder)}
            />
          {/each}
        </div>
      {/if}
    {/if}
  </div>
</PageWrapper>

<!-- Preview Panel Overlay -->
{#if showPreviewPanel && previewFile}
  <FilePreviewPanel
    file={previewFile}
    allFiles={displayFiles().filter((f) => !f.is_directory)}
    currentIndex={previewFileIndex}
    onClose={closePreviewPanel}
    onNavigate={navigatePreview}
  />
{/if}

<!-- Context Menu -->
{#if contextMenu}
  <FileActionsMenu
    file={contextMenu}
    position={contextMenuPosition}
    onClose={closeContextMenu}
    onRename={() => modals.openRename(currentFile)}
    onDelete={() => modals.openDelete(currentFile)}
    onMove={() => {
      selectedFileForMove = currentFile;
      showMoveModal = true;
      closeContextMenu();
    }}
    onCopy={() => {
      selectedFileForCopy = currentFile;
      showCopyModal = true;
      closeContextMenu();
    }}
    onShare={() => modals.openShare(currentFile)}
    onDownload={() => downloadFile(currentFile)}
    onEdit={() => {
      openFileEditor(currentFile);
      closeContextMenu();
    }}
    onVersionHistory={() => modals.openVersionHistory(currentFile)}
    onPreview={() => openFile(currentFile)}
    onChangeFolderColor={() => modals.openChangeFolderColor(currentFile)}
  />
{/if}

<!-- Copy File Modal -->
<CopyFileModal
  bind:visible={showCopyModal}
  file={selectedFileForCopy}
  onClose={() => {
    showCopyModal = false;
    selectedFileForCopy = null;
  }}
  onSuccess={async () => {
    await loadFiles();
    showCopyModal = false;
    selectedFileForCopy = null;
  }}
/>

<!-- Move File Modal -->
<MoveFileModal
  bind:visible={showMoveModal}
  file={selectedFileForMove}
  onClose={() => {
    showMoveModal = false;
    selectedFileForMove = null;
  }}
  onSuccess={async () => {
    await loadFiles();
    showMoveModal = false;
    selectedFileForMove = null;
  }}
/>

<!-- File Editor Modal -->
{#if showEditorModal && fileToEdit}
  <FileEditorModal
    bind:isOpen={showEditorModal}
    filePath={fileToEdit.file_path || fileToEdit.path}
    fileId={fileToEdit.id}
    onSave={handleEditorSave}
  />
{/if}

<!-- Template Library Modal -->
<TemplateLibraryModal
  bind:show={showTemplateLibrary}
  onTemplateUsed={(filePath) => {
    success(`File created from template: ${filePath}`);
    loadFiles();
  }}
/>

<!-- Bulk Tagging Modal -->
{#if showBulkTaggingModal}
  <BulkTaggingModal
    selectedFiles={Array.from(selectedFiles)
      .map((path) => displayFiles().find((f) => f.path === path))
      .filter(Boolean)}
    onClose={() => {
      showBulkTaggingModal = false;
      selectedFiles.clear();
      selectedFiles = selectedFiles;
      selectionMode = false;
      loadFiles();
    }}
  />
{/if}

<!-- Batch Rename Modal -->
{#if showBatchRenameModal}
  <BatchRenameModal
    isOpen={showBatchRenameModal}
    files={Array.from(selectedFiles)
      .map((path) => displayFiles().find((f) => f.path === path))
      .filter(Boolean)}
    onClose={() => {
      showBatchRenameModal = false;
    }}
    onComplete={() => {
      selectedFiles.clear();
      selectedFiles = selectedFiles;
      selectionMode = false;
      loadFiles();
    }}
  />
{/if}

<!-- Batch Progress Panel -->
<BatchProgressPanel bind:visible={showBatchProgress} />

<!-- All modals now rendered globally in ModalPortal.svelte -->

<style>
  /* Ensure smooth transitions */
  :global(.file-card-grid),
  :global(.file-card-list) {
    transition: all 0.2s ease;
  }

  /* Search Result Highlight Animation - SUPER AUFFÄLLIG! */
  :global(.highlight-search-result) {
    animation:
      highlight-border-spin 5s ease-in-out,
      highlight-glow 1s ease-in-out infinite;
    position: relative;
    z-index: 100;
    transform: scale(1.08);
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.1),
      rgba(147, 51, 234, 0.1)
    ) !important;
  }

  /* Umlaufender animierter Rahmen */
  @keyframes highlight-border-spin {
    0% {
      box-shadow:
        0 0 0 4px rgba(59, 130, 246, 1),
        0 0 20px rgba(59, 130, 246, 0.5),
        inset 0 0 20px rgba(59, 130, 246, 0.2);
    }
    25% {
      box-shadow:
        0 0 0 6px rgba(147, 51, 234, 1),
        0 0 30px rgba(147, 51, 234, 0.6),
        inset 0 0 30px rgba(147, 51, 234, 0.2);
    }
    50% {
      box-shadow:
        0 0 0 8px rgba(59, 130, 246, 1),
        0 0 40px rgba(59, 130, 246, 0.7),
        inset 0 0 40px rgba(59, 130, 246, 0.3);
    }
    75% {
      box-shadow:
        0 0 0 6px rgba(147, 51, 234, 1),
        0 0 30px rgba(147, 51, 234, 0.6),
        inset 0 0 30px rgba(147, 51, 234, 0.2);
    }
    100% {
      box-shadow:
        0 0 0 4px rgba(59, 130, 246, 1),
        0 0 20px rgba(59, 130, 246, 0.5),
        inset 0 0 20px rgba(59, 130, 246, 0.2);
    }
  }

  /* Zusätzliches Glühen/Pulsieren */
  @keyframes highlight-glow {
    0%,
    100% {
      filter: brightness(1);
    }
    50% {
      filter: brightness(1.2);
    }
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
