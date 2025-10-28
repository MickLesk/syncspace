<script>
  import { onMount, onDestroy } from "svelte";
  import { currentPath } from "../stores/ui";
  import { onFileEvent } from "../stores/websocket";
  import api from "../lib/api";
  import { success, error as errorToast } from "../stores/toast";

  let folders = $state([]);
  let expandedFolders = $state(new Set());
  let loading = $state(false);
  let reloadTimeout = null;
  let unsubscribeWS = null;

  // Context menu state
  let contextMenu = $state({
    visible: false,
    x: 0,
    y: 0,
    folder: null,
  });

  async function loadFolders(path = "/") {
    try {
      const files = await api.files.list(path);
      return files
        .filter((f) => f.is_dir)
        .map((f) => ({
          name: f.name,
          path: path === "/" ? `/${f.name}` : `${path}/${f.name}`,
          children: null, // Lazy load
          hasChildren: true,
        }));
    } catch (err) {
      console.error("Failed to load folders:", err);
      return [];
    }
  }

  async function toggleFolder(folder) {
    const folderPath = folder.path;

    if (expandedFolders.has(folderPath)) {
      // Collapse
      expandedFolders.delete(folderPath);
      expandedFolders = new Set(expandedFolders);
    } else {
      // Expand - load children if not loaded
      if (folder.children === null) {
        folder.children = await loadFolders(folderPath);
      }
      expandedFolders.add(folderPath);
      expandedFolders = new Set(expandedFolders);
    }
  }

  function navigateToFolder(path) {
    currentPath.set(path.endsWith("/") ? path : path + "/");
  }

  function showContextMenu(event, folder) {
    event.preventDefault();
    event.stopPropagation();

    contextMenu = {
      visible: true,
      x: event.clientX,
      y: event.clientY,
      folder: folder,
    };
  }

  function hideContextMenu() {
    contextMenu = {
      visible: false,
      x: 0,
      y: 0,
      folder: null,
    };
  }

  async function handleRename(folder) {
    hideContextMenu();
    const newName = prompt("Rename folder:", folder.name);
    if (newName && newName !== folder.name) {
      try {
        await api.files.rename(folder.path, newName);
        success("Folder renamed successfully");
        // Reload folders
        folders = await loadFolders("/");
      } catch (err) {
        errorToast("Failed to rename folder: " + err.message);
      }
    }
  }

  async function handleDelete(folder) {
    hideContextMenu();
    if (confirm(`Delete folder "${folder.name}" and all its contents?`)) {
      try {
        await api.files.delete(folder.path);
        success("Folder deleted successfully");
        folders = await loadFolders("/");
      } catch (err) {
        errorToast("Failed to delete folder: " + err.message);
      }
    }
  }

  async function handleShare(folder) {
    hideContextMenu();
    success(`Share functionality for "${folder.name}" coming soon!`);
  }

  async function handleFavorite(folder) {
    hideContextMenu();
    success(`Favorite functionality for "${folder.name}" coming soon!`);
  }

  async function handleNewSubfolder(folder) {
    hideContextMenu();
    const name = prompt("New subfolder name:");
    if (name) {
      try {
        const newPath = `${folder.path}/${name}`;
        await api.files.createDir(newPath);
        success("Subfolder created successfully");
        // Expand parent and reload
        if (!expandedFolders.has(folder.path)) {
          folder.children = await loadFolders(folder.path);
          expandedFolders.add(folder.path);
          expandedFolders = new Set(expandedFolders);
        } else {
          folder.children = await loadFolders(folder.path);
        }
      } catch (err) {
        errorToast("Failed to create subfolder: " + err.message);
      }
    }
  }

  // Close context menu on outside click
  function handleDocumentClick(event) {
    if (contextMenu.visible) {
      hideContextMenu();
    }
  }

  onMount(async () => {
    loading = true;
    folders = await loadFolders("/");
    loading = false;

    // Add document click listener for context menu
    document.addEventListener("click", handleDocumentClick);

    // Listen for file system events to refresh folder tree
    unsubscribeWS = onFileEvent((event) => {
      // Ignore system files
      const ignoredPatterns = [
        "syncspace.db",
        ".db-shm",
        ".db-wal",
        "search_index",
        ".tmp",
        ".lock",
      ];
      if (ignoredPatterns.some((pattern) => event.path.includes(pattern))) {
        return;
      }

      // Debounce reload
      if (reloadTimeout) {
        clearTimeout(reloadTimeout);
      }

      reloadTimeout = setTimeout(async () => {
        console.log("🌳 Refreshing folder tree due to file system change");
        folders = await loadFolders("/");
        // Re-expand previously expanded folders
        for (const folderPath of expandedFolders) {
          const folder = findFolderByPath(folders, folderPath);
          if (folder && folder.children === null) {
            folder.children = await loadFolders(folderPath);
          }
        }
      }, 1500); // Longer debounce for tree
    });
  });

  onDestroy(() => {
    if (unsubscribeWS) {
      unsubscribeWS();
    }
    if (reloadTimeout) {
      clearTimeout(reloadTimeout);
    }
    document.removeEventListener("click", handleDocumentClick);
  });

  function findFolderByPath(folders, path) {
    for (const folder of folders) {
      if (folder.path === path) {
        return folder;
      }
      if (folder.children) {
        const found = findFolderByPath(folder.children, path);
        if (found) return found;
      }
    }
    return null;
  }

  // Get icon and color based on folder name/type
  function getFolderIcon(folderName, folderPath) {
    const lowerName = folderName.toLowerCase();

    // System folders
    const systemFolders = {
      documents: { icon: "file-earmark-text-fill", color: "text-info" },
      downloads: { icon: "download", color: "text-success" },
      pictures: { icon: "image-fill", color: "text-primary" },
      photos: { icon: "image-fill", color: "text-primary" },
      music: { icon: "music-note-beamed", color: "text-secondary" },
      videos: { icon: "film", color: "text-accent" },
      desktop: { icon: "display", color: "text-neutral" },
      public: { icon: "folder-symlink-fill", color: "text-warning" },
      shared: { icon: "share-fill", color: "text-primary" },
      favorites: { icon: "star-fill", color: "text-warning" },
      temp: { icon: "folder-x", color: "text-error" },
      trash: { icon: "trash-fill", color: "text-error" },
    };

    for (const [key, value] of Object.entries(systemFolders)) {
      if (lowerName.includes(key)) {
        return value;
      }
    }

    // Project folders (detected by common project markers)
    if (lowerName.includes("project") || lowerName.includes("work")) {
      return { icon: "briefcase-fill", color: "text-accent" };
    }

    // Archive folders
    if (lowerName.includes("archive") || lowerName.includes("backup")) {
      return { icon: "archive-fill", color: "text-warning" };
    }

    // Default folder icon
    return { icon: "folder-fill", color: "text-warning" };
  }

  function renderFolder(folder, depth = 0) {
    const isExpanded = expandedFolders.has(folder.path);
    const isActive =
      $currentPath === folder.path || $currentPath === folder.path + "/";
    const iconInfo = getFolderIcon(folder.name, folder.path);

    return {
      folder,
      depth,
      isExpanded,
      isActive,
      icon: iconInfo.icon,
      iconColor: iconInfo.color,
    };
  }

  function flattenFolders(folders, depth = 0) {
    const result = [];
    for (const folder of folders) {
      result.push(renderFolder(folder, depth));
      if (expandedFolders.has(folder.path) && folder.children) {
        result.push(...flattenFolders(folder.children, depth + 1));
      }
    }
    return result;
  }

  let flatFolders = $derived(flattenFolders(folders));
</script>

<div class="folder-tree">
  <div class="folder-tree-header">
    <i class="bi bi-folder-fill text-warning"></i>
    <span class="font-semibold">Folders</span>
  </div>

  {#if loading}
    <div class="loading-state">
      <span class="loading loading-spinner loading-sm"></span>
      <span class="text-sm">Loading folders...</span>
    </div>
  {:else if flatFolders.length === 0}
    <div class="empty-state">
      <i class="bi bi-folder-x text-base-content/40"></i>
      <span class="text-sm text-base-content/60">No folders</span>
    </div>
  {:else}
    <div class="folder-list">
      {#each flatFolders as { folder, depth, isExpanded, isActive, icon, iconColor }}
        <div
          class="folder-item"
          class:active={isActive}
          style="padding-left: {depth * 1.5 + 0.5}rem"
        >
          <button
            class="folder-toggle"
            onclick={() => toggleFolder(folder)}
            aria-label="Toggle folder"
          >
            <i class="bi bi-chevron-{isExpanded ? 'down' : 'right'} text-xs"
            ></i>
          </button>

          <button
            class="folder-button"
            onclick={() => navigateToFolder(folder.path)}
            oncontextmenu={(e) => showContextMenu(e, folder)}
            title={folder.path}
          >
            <i class="bi bi-{icon} {iconColor}"></i>
            <span class="folder-name">{folder.name}</span>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Context Menu -->
{#if contextMenu.visible && contextMenu.folder}
  <div
    class="context-menu"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
  >
    <button
      class="context-menu-item"
      onclick={() => handleNewSubfolder(contextMenu.folder)}
    >
      <i class="bi bi-folder-plus"></i>
      <span>New Subfolder</span>
      <kbd class="kbd kbd-xs ml-auto">N</kbd>
    </button>

    <div class="divider my-0"></div>

    <button
      class="context-menu-item"
      onclick={() => handleRename(contextMenu.folder)}
    >
      <i class="bi bi-pencil"></i>
      <span>Rename</span>
      <kbd class="kbd kbd-xs ml-auto">F2</kbd>
    </button>

    <button
      class="context-menu-item"
      onclick={() => handleShare(contextMenu.folder)}
    >
      <i class="bi bi-share"></i>
      <span>Share</span>
    </button>

    <button
      class="context-menu-item"
      onclick={() => handleFavorite(contextMenu.folder)}
    >
      <i class="bi bi-star"></i>
      <span>Add to Favorites</span>
      <kbd class="kbd kbd-xs ml-auto">⭐</kbd>
    </button>

    <div class="divider my-0"></div>

    <button
      class="context-menu-item text-error"
      onclick={() => handleDelete(contextMenu.folder)}
    >
      <i class="bi bi-trash"></i>
      <span>Delete</span>
      <kbd class="kbd kbd-xs ml-auto">Del</kbd>
    </button>
  </div>
{/if}

<style>
  .folder-tree {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.1));
  }

  .folder-tree-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 0.75rem;
    color: var(--fallback-bc, oklch(var(--bc) / 0.7));
    font-size: 0.875rem;
  }

  .loading-state,
  .empty-state {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    color: var(--fallback-bc, oklch(var(--bc) / 0.5));
  }

  .folder-list {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .folder-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    transition: background-color 0.15s ease;
  }

  .folder-item:hover {
    background: var(--fallback-bc, oklch(var(--bc) / 0.05));
  }

  .folder-item.active {
    background: var(--fallback-p, oklch(var(--p) / 0.1));
  }

  .folder-item.active .folder-button {
    color: var(--fallback-p, oklch(var(--p) / 1));
    font-weight: 600;
  }

  .folder-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 0.25rem;
    color: var(--fallback-bc, oklch(var(--bc) / 0.5));
    transition: all 0.15s ease;
  }

  .folder-toggle:hover {
    background: var(--fallback-bc, oklch(var(--bc) / 0.1));
    color: var(--fallback-bc, oklch(var(--bc) / 1));
  }

  .folder-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    padding: 0.5rem 0.75rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 0.375rem;
    color: var(--fallback-bc, oklch(var(--bc) / 0.8));
    font-size: 0.875rem;
    text-align: left;
    transition: all 0.15s ease;
  }

  .folder-button:hover {
    background: var(--fallback-bc, oklch(var(--bc) / 0.05));
  }

  .folder-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Context Menu */
  .context-menu {
    position: fixed;
    z-index: 1000;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: 0.5rem;
    box-shadow:
      0 10px 25px -5px rgba(0, 0, 0, 0.1),
      0 8px 10px -6px rgba(0, 0, 0, 0.1);
    padding: 0.25rem;
    min-width: 200px;
    backdrop-filter: blur(8px);
    animation: contextMenuAppear 0.15s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes contextMenuAppear {
    from {
      opacity: 0;
      transform: scale(0.95) translateY(-4px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  .context-menu-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.625rem 0.75rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 0.375rem;
    color: hsl(var(--bc));
    font-size: 0.875rem;
    text-align: left;
    transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .context-menu-item:hover {
    background: hsl(var(--bc) / 0.05);
  }

  .context-menu-item.text-error:hover {
    background: hsl(var(--er) / 0.1);
    color: hsl(var(--er));
  }

  .context-menu-item i {
    font-size: 1rem;
    width: 1.25rem;
    flex-shrink: 0;
  }

  .context-menu-item .kbd {
    opacity: 0.5;
    font-size: 0.625rem;
  }
</style>

