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

  let loading = true;
  let uploadInput;
  let dragOver = false;
  let uploading = false;
  let searchQuery = "";
  let viewMode = localStorage.getItem('filesViewMode') || 'grid'; // 'grid' or 'list'

  // Dialog states
  let showDeleteDialog = false;
  let showRenameDialog = false;
  let showNewFolderDialog = false;
  let fileToDelete = null;
  let fileToRename = null;

  function toggleViewMode() {
    viewMode = viewMode === 'grid' ? 'list' : 'grid';
    localStorage.setItem('filesViewMode', viewMode);
  }

  function navigateTo(folderName) {
    currentPath.update(path => path + folderName + '/');
    loadFiles();
  }

  $: filteredFiles = searchQuery
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
      const response = await fetch(
        `http://localhost:8080/api/files/${$currentPath}`,
        {
          headers: {
            Authorization: `Bearer ${$auth.token}`,
          },
        }
      );

      if (response.ok) {
        const data = await response.json();
        files.set(data);
      }
    } catch (error) {
      console.error("Failed to load files:", error);
      files.set([]);
    }
    loading = false;
  }

  async function handleUpload(fileList) {
    if (!fileList || fileList.length === 0) return;

    console.log("🚀 Starting upload:", fileList.length, "files");
    console.log("📦 Auth token:", $auth.token.substring(0, 20) + "...");
    console.log("📁 Current path:", $currentPath);

    uploading = true;
    let successCount = 0;
    let failCount = 0;

    for (const file of fileList) {
      try {
        const arrayBuffer = await file.arrayBuffer();
        const uploadUrl = `http://localhost:8080/api/upload/${$currentPath}${file.name}`;
        
        console.log(`⬆️ Uploading ${file.name} to:`, uploadUrl);
        
        const response = await fetch(uploadUrl, {
          method: "POST",
          headers: {
            Authorization: `Bearer ${$auth.token}`,
            "Content-Type": "application/octet-stream",
          },
          body: arrayBuffer,
        });

        console.log(`📡 Response status for ${file.name}:`, response.status);

        if (response.ok) {
          console.log(`✅ ${file.name} uploaded successfully`);
          successCount++;
        } else {
          // Check for auth errors
          if (auth.checkAuthError(response)) {
            errorToast("Sitzung abgelaufen. Bitte neu anmelden.");
            return;
          }
          const errorText = await response.text();
          console.error(`❌ ${file.name} upload failed (${response.status}):`, errorText);
          failCount++;
        }
      } catch (err) {
        console.error(`💥 Upload error for ${file.name}:`, err);
        failCount++;
      }
    }

    uploading = false;
    
    if (successCount > 0) {
      success(`${successCount} Datei(en) erfolgreich hochgeladen`);
    }
    if (failCount > 0) {
      errorToast(`${failCount} Datei(en) konnten nicht hochgeladen werden`);
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

    const createUrl = `http://localhost:8080/api/dirs/${$currentPath}${folderName}`;
    console.log("📁 Creating folder:", folderName, "at URL:", createUrl);
    console.log("🔑 Using token:", $auth.token.substring(0, 20) + "...");

    try {
      const response = await fetch(createUrl, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${$auth.token}`,
        },
      });

      console.log("📡 Create folder response:", response.status);

      if (response.ok) {
        console.log("✅ Folder created successfully");
        success(`Ordner "${folderName}" wurde erstellt`);
        await loadFiles();
      } else {
        // Check for auth errors
        if (auth.checkAuthError(response)) {
          errorToast("Sitzung abgelaufen. Bitte neu anmelden.");
          return;
        }
        const errorText = await response.text();
        console.error("❌ Folder creation failed:", response.status, errorText);
        errorToast(`Fehler beim Erstellen: ${errorText}`);
      }
    } catch (err) {
      console.error("💥 Failed to create folder:", err);
      errorToast(`Fehler beim Erstellen: ${err.message}`);
    }
  }

  async function downloadFile(file) {
    try {
      const response = await fetch(
        `http://localhost:8080/api/file/${$currentPath}${file.name}`,
        {
          headers: {
            Authorization: `Bearer ${$auth.token}`,
          },
        }
      );

      if (response.ok) {
        const blob = await response.blob();
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = file.name;
        a.click();
        URL.revokeObjectURL(url);
        success(`"${file.name}" wird heruntergeladen`);
      } else {
        errorToast('Fehler beim Herunterladen');
      }
    } catch (err) {
      console.error("Failed to download file:", err);
      errorToast(`Fehler: ${err.message}`);
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
      const response = await fetch(
        `http://localhost:8080/api/files/${$currentPath}${fileName}`,
        {
          method: "DELETE",
          headers: {
            Authorization: `Bearer ${$auth.token}`,
          },
        }
      );

      if (response.ok) {
        success(`"${fileName}" wurde gelöscht`);
        await loadFiles();
      } else {
        const errorText = await response.text();
        errorToast(`Fehler: ${errorText}`);
      }
    } catch (err) {
      console.error("Failed to delete file:", err);
      errorToast(`Fehler: ${err.message}`);
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
      const response = await fetch(
        `http://localhost:8080/api/rename/${$currentPath}${oldName}`,
        {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${$auth.token}`,
          },
          body: JSON.stringify({ new_path: `${$currentPath}${newName}` }),
        }
      );

      if (response.ok) {
        success(`"${oldName}" → "${newName}"`);
        await loadFiles();
      } else {
        const errorText = await response.text();
        errorToast(`Fehler: ${errorText}`);
      }
    } catch (err) {
      console.error("Failed to rename file:", err);
      errorToast(`Fehler: ${err.message}`);
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
        placeholder="🔍 {t($currentLang, 'search')}..."
      />
      <button 
        class="btn-view-toggle"
        on:click={toggleViewMode}
        title={viewMode === 'grid' ? 'Listen-Ansicht' : 'Karten-Ansicht'}
      >
        <Icon name={viewMode === 'grid' ? 'list-ul' : 'grid-3x3-gap'} size={18} />
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

  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading files...</p>
    </div>
  {:else if filteredFiles.length === 0}
    <div class="empty-state">
      <p class="empty-icon">📂</p>
      <p class="empty-title">{t($currentLang, "noFiles")}</p>
      <p class="empty-subtitle">{t($currentLang, "dragAndDropHere")}</p>
    </div>
  {:else}
    {#if viewMode === 'grid'}
    <div class="file-grid">
      {#each filteredFiles as file}
        <div 
          class="file-card" 
          class:folder={file.is_dir}
          on:click={() => file.is_dir && navigateTo(file.name)}
          on:keydown={(e) => e.key === 'Enter' && file.is_dir && navigateTo(file.name)}
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
      {#each filteredFiles as file}
        <div 
          class="file-row" 
          class:folder={file.is_dir}
          on:click={() => file.is_dir && navigateTo(file.name)}
          on:keydown={(e) => e.key === 'Enter' && file.is_dir && navigateTo(file.name)}
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
  placeholder={fileToRename.name || ''}
  initialValue={fileToRename.name || ''}
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
</style>
