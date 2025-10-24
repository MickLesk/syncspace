<script>
  import { onMount } from "svelte";
  import { files, currentPath } from "../stores/ui";
  import { favorites } from "../stores/favorites";
  import { success, error as errorToast } from "../stores/toast";
  import { getFileIcon } from "../utils/fileIcons";
  import ContextMenu from "../components/ui/ContextMenu.svelte";
  import Breadcrumb from "../components/Breadcrumb.svelte";
  import Modal from "../components/ui/Modal.svelte";
  import api from "../lib/api";

  let loading = true;
  let uploading = false;
  let searchQuery = "";
  let viewMode = "grid"; // 'grid' or 'list'
  let dragOver = false;

  // Modals
  let showUploadModal = false;
  let showNewFolderModal = false;
  let showRenameModal = false;
  let showDeleteModal = false;

  // Current action targets
  let fileToRename = null;
  let fileToDelete = null;
  let newFolderName = "";
  let newFileName = "";

  // Context Menu
  let contextMenuVisible = false;
  let contextMenuX = 0;
  let contextMenuY = 0;
  let contextMenuFile = null;

  // File upload
  let uploadInput;
  let uploadFiles = [];

  $: filteredFiles = searchQuery
    ? $files.filter((f) =>
        f.name.toLowerCase().includes(searchQuery.toLowerCase())
      )
    : $files;

  $: breadcrumbPath = $currentPath.split("/").filter(Boolean);

  onMount(async () => {
    await loadFiles();
  });

  async function loadFiles() {
    loading = true;
    try {
      const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
      const data = await api.files.list(backendPath);
      files.set(data);
    } catch (err) {
      errorToast(err.message || "Failed to load files");
    } finally {
      loading = false;
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
    try {
      for (const file of uploadFiles) {
        const backendPath = $currentPath.replace(/^\/+|\/+$/g, "");
        const fullPath = backendPath
          ? `${backendPath}/${file.name}`
          : file.name;
        await api.files.upload(fullPath, file);
      }
      success(`Uploaded ${uploadFiles.length} file(s)`);
      await loadFiles();
      showUploadModal = false;
      uploadFiles = [];
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

  function openRenameModal(file) {
    fileToRename = file;
    newFileName = file.name;
    showRenameModal = true;
  }

  function openDeleteModal(file) {
    fileToDelete = file;
    showDeleteModal = true;
  }

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
      items.push({
        label: $favorites.includes(contextMenuFile.name)
          ? "Remove from Favorites"
          : "Add to Favorites",
        icon: "star",
      });
    }

    items.push({ divider: true });
    items.push({ label: "Copy", icon: "copy", shortcut: "Ctrl+C" });
    items.push({ label: "Move", icon: "arrows-move" });

    items.push({ divider: true });
    items.push({ label: "Details", icon: "info-circle" });

    if (!contextMenuFile.is_directory) {
      items.push({ label: "Version History", icon: "clock-history" });
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
        // TODO: Implement preview panel (Item #8)
        success("Preview coming soon");
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
        // TODO: Implement clipboard copy (Item #7)
        success("Copy to clipboard coming soon");
        break;
      case "Move":
        // TODO: Implement move dialog (Item #7)
        success("Move dialog coming soon");
        break;
      case "Details":
        // TODO: Implement details panel (Item #8)
        success("Details panel coming soon");
        break;
      case "Version History":
        // TODO: Implement versioning UI (Item #19)
        success("Version history coming soon");
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
</script>

<svelte:window on:click={handleClickOutside} />

<div
  class="files-view"
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:drop={handleDrop}
  role="main"
>
  <!-- Toolbar -->
  <div class="toolbar card bg-base-100 border border-base-300 mb-6 shadow-sm">
    <div class="card-body p-4">
      <div class="flex flex-wrap items-center gap-3">
        <!-- Left: Actions -->
        <div class="flex gap-2 flex-1">
          <button
            class="btn btn-primary gap-2"
            on:click={() => uploadInput?.click()}
          >
            <i class="bi bi-upload"></i>
            Upload
          </button>
          <button
            class="btn btn-secondary gap-2"
            on:click={() => (showNewFolderModal = true)}
          >
            <i class="bi bi-folder-plus"></i>
            New Folder
          </button>
          <button class="btn btn-ghost gap-2" on:click={() => loadFiles()}>
            <i class="bi bi-arrow-clockwise"></i>
            Refresh
          </button>
        </div>

        <!-- Right: View Mode -->
        <div class="flex items-center gap-2">
          <div class="join">
            <button
              class="btn btn-sm join-item {viewMode === 'grid'
                ? 'btn-active'
                : ''}"
              on:click={() => (viewMode = "grid")}
              aria-label="Grid view"
            >
              <i class="bi bi-grid-3x3"></i>
            </button>
            <button
              class="btn btn-sm join-item {viewMode === 'list'
                ? 'btn-active'
                : ''}"
              on:click={() => (viewMode = "list")}
              aria-label="List view"
            >
              <i class="bi bi-list-ul"></i>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Breadcrumb Navigation -->
  <Breadcrumb
    path={$currentPath}
    maxVisibleSegments={4}
    on:navigate={handleBreadcrumbNavigate}
  />

  <!-- Drag & Drop Overlay -->
  {#if dragOver}
    <div class="drop-overlay">
      <div class="drop-content">
        <i class="bi bi-cloud-upload text-6xl mb-4"></i>
        <h3 class="text-2xl font-bold">Drop files here</h3>
        <p class="opacity-70">Release to upload</p>
      </div>
    </div>
  {/if}

  <!-- Files Display -->
  {#if loading}
    <div class="flex justify-center items-center h-64">
      <span class="loading loading-spinner loading-lg text-primary"></span>
    </div>
  {:else if filteredFiles.length === 0}
    <div class="hero min-h-[400px]">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <i class="bi bi-folder2-open text-7xl text-base-300 mb-4"></i>
          <h1 class="text-3xl font-bold">No files found</h1>
          <p class="py-6">Upload files or create a folder to get started</p>
          <button
            class="btn btn-primary gap-2"
            on:click={() => uploadInput?.click()}
          >
            <i class="bi bi-upload"></i>
            Upload Files
          </button>
        </div>
      </div>
    </div>
  {:else if viewMode === "grid"}
    <!-- Grid View -->
    <div
      class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4"
    >
      {#each filteredFiles as file}
        <div
          class="card bg-base-100 border border-base-300 hover:shadow-lg transition-all cursor-pointer group"
          on:click={() => file.is_dir && navigateToFolder(file)}
          on:contextmenu={(e) => handleContextMenu(e, file)}
          on:keydown={(e) =>
            e.key === "Enter" && file.is_dir && navigateToFolder(file)}
          role="button"
          tabindex="0"
        >
          <div class="card-body p-4 items-center text-center">
            <div
              class="text-5xl mb-2 {file.is_dir
                ? 'text-warning'
                : 'text-primary'}"
            >
              <i
                class="bi bi-{file.is_dir
                  ? 'folder-fill'
                  : getFileIcon(file.name)}"
              ></i>
            </div>
            <h3
              class="card-title text-sm font-semibold truncate w-full"
              title={file.name}
            >
              {file.name}
            </h3>
            <div class="text-xs opacity-70">
              {file.is_dir ? "Folder" : formatFileSize(file.size)}
            </div>
            <div
              class="card-actions justify-center mt-2 opacity-0 group-hover:opacity-100 transition-opacity"
            >
              {#if !file.is_dir}
                <button
                  class="btn btn-ghost btn-xs btn-circle"
                  on:click|stopPropagation={() => handleDownload(file)}
                  aria-label="Download"
                >
                  <i class="bi bi-download"></i>
                </button>
              {/if}
              <button
                class="btn btn-ghost btn-xs btn-circle"
                on:click|stopPropagation={() => openRenameModal(file)}
                aria-label="Rename"
              >
                <i class="bi bi-pencil"></i>
              </button>
              <button
                class="btn btn-ghost btn-xs btn-circle text-error"
                on:click|stopPropagation={() => openDeleteModal(file)}
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
    <div class="overflow-x-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Size</th>
            <th>Modified</th>
            <th class="text-right">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredFiles as file}
            <tr
              class="hover cursor-pointer"
              on:click={() => file.is_dir && navigateToFolder(file)}
              on:contextmenu={(e) => handleContextMenu(e, file)}
            >
              <td>
                <div class="flex items-center gap-3">
                  <div
                    class="text-2xl {file.is_dir
                      ? 'text-warning'
                      : 'text-primary'}"
                  >
                    <i
                      class="bi bi-{file.is_dir
                        ? 'folder-fill'
                        : getFileIcon(file.name)}"
                    ></i>
                  </div>
                  <div class="font-semibold">{file.name}</div>
                </div>
              </td>
              <td>
                <span class="badge badge-ghost">
                  {file.is_dir
                    ? "Folder"
                    : (file.name.split(".").pop() || "File").toUpperCase()}
                </span>
              </td>
              <td>{file.is_dir ? "—" : formatFileSize(file.size)}</td>
              <td>{file.modified_at ? formatDate(file.modified_at) : "—"}</td>
              <td>
                <div class="flex gap-1 justify-end">
                  {#if !file.is_dir}
                    <button
                      class="btn btn-ghost btn-sm btn-circle"
                      on:click|stopPropagation={() => handleDownload(file)}
                      aria-label="Download"
                    >
                      <i class="bi bi-download"></i>
                    </button>
                  {/if}
                  <button
                    class="btn btn-ghost btn-sm btn-circle"
                    on:click|stopPropagation={() => openRenameModal(file)}
                    aria-label="Rename"
                  >
                    <i class="bi bi-pencil"></i>
                  </button>
                  <button
                    class="btn btn-ghost btn-sm btn-circle text-error"
                    on:click|stopPropagation={() => openDeleteModal(file)}
                    aria-label="Delete"
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
  on:change={handleFilesSelected}
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
        class="max-h-96 overflow-y-auto space-y-2 rounded-xl bg-base-200/50 p-3"
      >
        {#each uploadFiles as file, index}
          <div
            class="flex justify-between items-center p-3 bg-base-100 rounded-xl hover:shadow-md transition-all duration-200"
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
              </div>
            </div>
            <div class="badge badge-success badge-outline">
              <i class="bi bi-check-circle mr-1"></i>
              Ready
            </div>
          </div>
        {/each}
      </div>

      <!-- Upload Progress (shown when uploading) -->
      {#if uploading}
        <div class="space-y-2">
          <div class="flex justify-between text-sm">
            <span class="font-medium">Uploading files...</span>
            <span class="opacity-60">Please wait</span>
          </div>
          <progress
            class="progress progress-success w-full"
            value="70"
            max="100"
          ></progress>
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
      on:click={() => (showUploadModal = false)}
      disabled={uploading}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      class="btn btn-success rounded-xl gap-2"
      class:loading={uploading}
      on:click={handleFileUpload}
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
      <label class="label">
        <span class="label-text font-semibold text-base-content"
          >Folder Name</span
        >
      </label>
      <input
        type="text"
        bind:value={newFolderName}
        placeholder="e.g., Documents, Photos, Projects..."
        class="input input-bordered rounded-xl focus:ring-2 focus:ring-success/50 text-base-content"
        on:keypress={(e) => e.key === "Enter" && handleCreateFolder()}
        autofocus
      />
      <label class="label">
        <span class="label-text-alt text-xs text-base-content/60">
          💡 Choose a descriptive name for easy organization
        </span>
      </label>
    </div>

    <!-- Preview -->
    <div class="alert alert-info rounded-xl bg-info/10 border-info/20">
      <i class="bi bi-info-circle text-xl text-info"></i>
      <div class="text-sm text-base-content">
        <strong>Location:</strong>
        <code class="px-2 py-1 bg-base-200 rounded ml-2 text-base-content">
          {$currentPath || "/"}{newFolderName || "new-folder"}
        </code>
      </div>
    </div>
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="btn btn-ghost rounded-xl"
      on:click={() => (showNewFolderModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      class="btn btn-success rounded-xl gap-2"
      on:click={handleCreateFolder}
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
    <div class="flex items-center gap-3 p-4 bg-base-200 rounded-xl">
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
      <label class="label">
        <span class="label-text font-semibold text-base-content">New Name</span>
      </label>
      <input
        type="text"
        bind:value={newFileName}
        placeholder="Enter new name..."
        class="input input-bordered rounded-xl focus:ring-2 focus:ring-primary/50 text-base-content"
        on:keypress={(e) => e.key === "Enter" && handleRename()}
        autofocus
      />
      <label class="label">
        <span class="label-text-alt text-xs text-base-content/60">
          ⚠️ File extension will be preserved automatically
        </span>
      </label>
    </div>
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="btn btn-ghost rounded-xl"
      on:click={() => (showRenameModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      class="btn btn-primary rounded-xl gap-2"
      on:click={handleRename}
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
    <div class="flex items-center gap-3 p-4 bg-base-200 rounded-xl">
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
      on:click={() => (showDeleteModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button class="btn btn-error rounded-xl gap-2" on:click={handleDelete}>
      <i class="bi bi-trash3-fill"></i>
      Delete Permanently
    </button>
  </div>
</Modal>

<!-- Context Menu -->
<ContextMenu
  visible={contextMenuVisible}
  x={contextMenuX}
  y={contextMenuY}
  items={getContextMenuItems()}
  on:select={handleContextAction}
/>

<style>
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
