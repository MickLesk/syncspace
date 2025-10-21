<script>
  import { onMount } from "svelte";
  import { files, currentPath, currentLang } from "../stores/ui";
  import { auth } from "../stores/auth";
  import { t } from "../i18n";
  import { success, error as errorToast } from "../stores/toast";
  import { getFileIcon, getFileType, isPreviewable } from "../utils/fileIcons";
  import Button from "../components/ui/Button.svelte";
  import SearchBar from "../components/ui/SearchBar.svelte";
  import Dialog from "../components/ui/Dialog.svelte";
  import InputDialog from "../components/ui/InputDialog.svelte";
  import Icon from "../components/ui/Icon.svelte";
  import api from "../lib/api";

  let loading = true;
  let uploadInput;
  let dragOver = false;
  let uploading = false;
  let searchQuery = "";
  let searchResults = [];
  let isSearching = false;
  let searchTimeout;
  let viewMode = localStorage.getItem("filesViewMode") || "grid"; // 'grid' or 'list'

  // Dialog states
  let showDeleteDialog = false;
  let showRenameDialog = false;
  let showNewFolderDialog = false;
  let fileToDelete = null;
  let fileToRename = null;

  function toggleViewMode() {
    viewMode = viewMode === "grid" ? "list" : "grid";
    localStorage.setItem("filesViewMode", viewMode);
  }

  function navigateTo(folderName) {
    currentPath.update((path) => path + folderName + "/");
    searchQuery = ""; // Clear search when navigating
    searchResults = [];
    loadFiles();
  }

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
      console.log(`🔍 Found ${searchResults.length} results for "${query}"`);
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
      ? searchResults.map((result) => ({
          name: result.filename,
          path: result.path,
          type: result.file_type === "unknown" ? "file" : result.file_type,
          is_dir: false,
          size: result.size,
          modified: result.modified,
          _searchScore: result.score,
          _snippet: result.snippet,
        }))
      : searchQuery
        ? $files.filter((f) =>
            f.name.toLowerCase().includes(searchQuery.toLowerCase())
          )
        : $files;

  onMount(() => {
    loadFiles();
  });

  async function loadFiles() {
    loading = true;
    try {
      const data = await api.files.list($currentPath);
      files.set(data);
    } catch (error) {
      console.error("Failed to load files:", error);
      errorToast("Failed to load files: " + error.message);
      files.set([]);
    }
    loading = false;
  }

  async function handleUpload(fileList) {
    if (!fileList || fileList.length === 0) return;

    uploading = true;
    let successCount = 0;
    let failCount = 0;

    for (const file of fileList) {
      try {
        const path = `${$currentPath}${file.name}`;
        await api.files.upload(path, file);
        successCount++;
      } catch (err) {
        console.error(`Upload error for ${file.name}:`, err);
        failCount++;
      }
    }

    uploading = false;

    if (successCount > 0) {
      success(`${successCount} ${t($currentLang, "filesUploaded")}`);
    }
    if (failCount > 0) {
      errorToast(`${failCount} files failed to upload`);
    }

    await loadFiles();
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
    dragOver = true;
  }

  function handleDragLeave(e) {
    e.preventDefault();
    dragOver = false;
  }

  function handleDrop(e) {
    e.preventDefault();
    dragOver = false;
    handleUpload(e.dataTransfer.files);
  }

  async function createFolder() {
    showNewFolderDialog = true;
  }

  async function handleCreateFolder(event) {
    const folderName = event.detail;
    if (!folderName) return;

    try {
      const path = `${$currentPath}${folderName}`;
      await api.files.createDir(path);
      success(`${t($currentLang, "folder")} "${folderName}" ${t($currentLang, "created")}`);
      await loadFiles();
    } catch (err) {
      console.error("Failed to create folder:", err);
      errorToast(`Error: ${err.message}`);
    }
  }

  async function downloadFile(file) {
    try {
      const path = `${$currentPath}${file.name}`;
      const blob = await api.files.download(path);
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = file.name;
      a.click();
      URL.revokeObjectURL(url);
      success(`"${file.name}" ${t($currentLang, "downloading")}`);
    } catch (err) {
      console.error("Failed to download file:", err);
      errorToast(`Error: ${err.message}`);
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
      const path = `${$currentPath}${fileName}`;
      await api.files.delete(path);
      success(`"${fileName}" ${t($currentLang, "deleted")}`);
      await loadFiles();
    } catch (err) {
      console.error("Failed to delete file:", err);
      errorToast(`Error: ${err.message}`);
    }

    fileToDelete = null;
  }

  async function renameFile(file) {
    fileToRename = file;
    showRenameDialog = true;
  }

  async function handleRenameConfirm(event) {
    const newName = event.detail;
    if (!fileToRename || !newName || newName === fileToRename.name) return;

    const oldName = fileToRename.name;

    try {
      const oldPath = `${$currentPath}${oldName}`;
      const newPath = `${$currentPath}${newName}`;
      await api.files.rename(oldPath, newPath);
      success(`"${oldName}" → "${newName}"`);
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
        class="btn-view-toggle"
        on:click={toggleViewMode}
        title={viewMode === "grid" ? "Listen-Ansicht" : "Karten-Ansicht"}
      >
        <Icon
          name={viewMode === "grid" ? "list-ul" : "grid-3x3-gap"}
          size={18}
        />
      </button>
      <Button onClick={handleUploadClick} disabled={uploading}>
        <Icon name="upload" size={16} />
        {uploading ? "Wird hochgeladen..." : t($currentLang, "upload")}
      </Button>
      <Button variant="outlined" onClick={createFolder}>
        <Icon name="folder-plus" size={16} />
        {t($currentLang, "newFolder")}
      </Button>
    </div>
  </div>

  <div
    class="drop-zone"
    class:drag-over={dragOver}
    on:dragover={handleDragOver}
    on:dragleave={handleDragLeave}
    on:drop={handleDrop}
    on:click={handleUploadClick}
    on:keydown={(e) => e.key === "Enter" && handleUploadClick()}
    role="button"
    tabindex="0"
  >
    <p class="drop-icon">📤</p>
    <p class="drop-title">{t($currentLang, "dragAndDropHere")}</p>
    <p class="drop-subtitle">{t($currentLang, "uploadFiles")}</p>
  </div>

  <!-- Search Mode Indicator -->
  {#if searchQuery && searchQuery.length >= 2}
    <div class="search-mode-indicator">
      <span class="search-icon">🔍</span>
      <span class="search-info">
        Search results for <strong>"{searchQuery}"</strong>
        {#if searchResults.length > 0}
          — {searchResults.length} {searchResults.length === 1 ? "file" : "files"} found
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
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading files...</p>
    </div>
  {:else if isSearching}
    <div class="loading">
      <div class="spinner"></div>
      <p>🔍 Searching...</p>
    </div>
  {:else if displayedFiles.length === 0}
    <div class="empty-state">
      <p class="empty-icon">{searchQuery ? "�" : "�📂"}</p>
      <p class="empty-title">{searchQuery ? "No results found" : t($currentLang, "noFiles")}</p>
      <p class="empty-subtitle">{searchQuery ? `Try different keywords` : t($currentLang, "dragAndDropHere")}</p>
    </div>
  {:else if viewMode === "grid"}
    <div class="file-grid">
      {#each displayedFiles as file}
        <div
          class="file-card"
          class:folder={file.is_dir}
          on:click={() => file.is_dir && navigateTo(file.name)}
          on:keydown={(e) =>
            e.key === "Enter" && file.is_dir && navigateTo(file.name)}
          role={file.is_dir ? "button" : undefined}
          tabindex={file.is_dir ? 0 : undefined}
        >
          <div class="file-icon">
            <Icon name={getFileIcon(file.name, file.is_dir)} size={48} />
          </div>
          <div class="file-name" title={file.name}>{file.name}</div>
          <div class="file-meta">
            {#if file.is_dir}
              <Icon name="folder" size={14} /> Ordner
            {:else}
              {formatSize(file.size)}
            {/if}
          </div>
          <div class="file-actions">
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
          on:click={() => file.is_dir && navigateTo(file.name)}
          on:keydown={(e) =>
            e.key === "Enter" && file.is_dir && navigateTo(file.name)}
          role={file.is_dir ? "button" : undefined}
          tabindex={file.is_dir ? 0 : undefined}
        >
          <div class="file-icon-small">
            <Icon name={getFileIcon(file.name, file.is_dir)} size={24} />
          </div>
          <div class="file-name-list" title={file.name}>{file.name}</div>
          <div class="file-size-list">
            {#if file.is_dir}
              <Icon name="folder" size={14} /> Ordner
            {:else}
              {formatSize(file.size)}
            {/if}
          </div>
          <div class="file-actions-list">
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
  <p>Möchten Sie <strong>"{fileToDelete.name}"</strong> wirklich löschen</p>
  <p style="color: var(--md-sys-color-error); margin-top: 12px;">
    Diese Aktion kann nicht rückgängig gemacht werden.
  </p>
</Dialog>

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

  .btn-view-toggle {
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
  }

  .btn-view-toggle:hover {
    background: var(--md-sys-color-secondary-container);
    border-color: var(--md-sys-color-secondary);
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

  .drop-zone {
    border: 2px dashed var(--md-sys-color-primary);
    border-radius: 20px;
    padding: 48px;
    text-align: center;
    cursor: pointer;
    transition: all 0.3s ease;
    background: rgba(103, 80, 164, 0.05);
    margin-bottom: 32px;
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

  .file-icon {
    display: flex;
    justify-content: center;
    margin-bottom: 12px;
    color: var(--md-sys-color-primary);
  }

  .file-card.folder .file-icon {
    color: var(--md-sys-color-tertiary);
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
    margin-bottom: 16px;
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
</style>
