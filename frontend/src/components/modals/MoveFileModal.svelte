<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import Modal from "../ui/Modal.svelte";
  import api from "../../lib/api.js";
  import { success, error as errorToast } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    visible = $bindable(false),
    file = null,
    onClose = () => {},
    onSuccess = () => {},
  } = $props();

  let destinationPath = $state("");
  let loading = $state(false);
  let error = $state(null);
  let availableFolders = $state([]);
  let folderTree = $state([]);
  let expandedFolders = $state(new Set());
  let createNewFolder = $state(false);
  let newFolderName = $state("");

  // Load available folders when modal opens
  $effect(() => {
    if (visible && file) {
      loadFolders();
    }
  });

  async function loadFolders() {
    try {
      const response = await api.files.list("/");
      // response.data ist bereits das Array, nicht response.json()!
      availableFolders = (response.data || []).filter(
        (item) => item.is_directory
      );

      // Build tree structure
      folderTree = buildFolderTree(availableFolders);

      console.log("[MoveFileModal] Loaded folders:", availableFolders);
      console.log("[MoveFileModal] Folder tree:", folderTree);
    } catch (err) {
      console.error("Failed to load folders:", err);
      errorToast(tr("failedToLoadFolders"));
    }
  }

  // Build hierarchical folder tree
  function buildFolderTree(folders) {
    const tree = [];
    const map = new Map();

    // Create map of all folders
    folders.forEach((folder) => {
      const path = folder.path || folder.file_path || folder.name;
      map.set(path, {
        ...folder,
        path: path,
        children: [],
      });
    });

    // Build tree
    folders.forEach((folder) => {
      const path = folder.path || folder.file_path || folder.name;
      const parentPath = path.split("/").slice(0, -1).join("/");

      if (parentPath && map.has(parentPath)) {
        map.get(parentPath).children.push(map.get(path));
      } else {
        tree.push(map.get(path));
      }
    });

    return tree;
  }

  function toggleFolder(path) {
    if (expandedFolders.has(path)) {
      expandedFolders.delete(path);
    } else {
      expandedFolders.add(path);
    }
    expandedFolders = new Set(expandedFolders);
  }

  function selectFolder(path) {
    destinationPath = path;
  }

  async function handleMove() {
    if (!file) return;

    loading = true;
    error = null;

    try {
      let targetPath = destinationPath;

      // Create new folder if requested
      if (createNewFolder && newFolderName.trim()) {
        try {
          await api.directories.create({ path: `/${newFolderName.trim()}` });
          targetPath = newFolderName.trim();
        } catch (err) {
          console.error("[MoveFileModal] Failed to create folder:", err);
          throw new Error(tr("failedToCreateFolder"));
        }
      }

      const sourcePath = file.path || file.file_path || file.name;
      const destPath = targetPath ? `${targetPath}/${file.name}` : file.name;

      console.log("[MoveFileModal] Moving:", { sourcePath, destPath });

      await api.files.move(sourcePath, destPath);

      success(tr("fileMovedSuccessfully", file.name));
      onSuccess();
      handleClose();
    } catch (err) {
      console.error("[MoveFileModal] Error:", err);
      error = err.message || tr("failedToMoveFile");
      errorToast(error);
    } finally {
      loading = false;
    }
  }

  function handleClose() {
    visible = false;
    destinationPath = "";
    createNewFolder = false;
    newFolderName = "";
    error = null;
    onClose();
  }
</script>

<Modal
  bind:visible
  title={tr("moveFile")}
  icon="arrow-right-square"
  size="md"
  onclose={handleClose}
>
  {#if file}
    <div class="space-y-4">
      <!-- Source file info -->
      <div
        class="bg-purple/5 dark:bg-purple/10 rounded-lg p-4 border border-purple/20"
      >
        <div class="flex items-center gap-3">
          <i
            class="bi {file.is_directory
              ? 'bi-folder-fill'
              : 'bi-file-earmark-fill'} text-2xl text-purple-600"
          ></i>
          <div class="flex-1 min-w-0">
            <p
              class="font-semibold text-sm text-gray-900 dark:text-white truncate"
            >
              {file.name}
            </p>
            <p class="text-xs text-gray-600 dark:text-gray-400">
              {tr("currentLocation")}: {file.path || tr("root")}
            </p>
          </div>
        </div>
      </div>

      <!-- Destination selection -->
      <div>
        <label class="flex items-center gap-2 mb-3">
          <input
            type="checkbox"
            bind:checked={createNewFolder}
            class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
          />
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
            {tr("createNewFolder")}
          </span>
        </label>

        {#if createNewFolder}
          <input
            type="text"
            bind:value={newFolderName}
            placeholder={tr("enterFolderName")}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
          />
        {:else}
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            {tr("selectDestination")}
          </label>
          <!-- Folder Tree View -->
          <div
            class="max-h-60 overflow-y-auto border-2 border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 p-2"
          >
            <!-- Root option -->
            <button
              type="button"
              onclick={() => selectFolder("")}
              class="w-full text-left px-3 py-2 rounded hover:bg-blue-50 dark:hover:bg-blue-900/20 transition-colors {destinationPath ===
              ''
                ? 'bg-blue-100 dark:bg-blue-900/40 font-semibold'
                : ''}"
            >
              <i class="bi bi-house-door text-blue-600 dark:text-blue-400 mr-2"
              ></i>
              {tr("rootDirectory")}
            </button>

            <!-- Folder list with visual hierarchy -->
            {#each availableFolders as folder}
              {@const folderPath =
                folder.path || folder.file_path || folder.name}
              {@const depth = (folderPath.match(/\//g) || []).length}
              {@const isSelected = destinationPath === folderPath}

              <button
                type="button"
                onclick={() => selectFolder(folderPath)}
                class="w-full text-left px-3 py-2 rounded hover:bg-blue-50 dark:hover:bg-blue-900/20 transition-colors {isSelected
                  ? 'bg-blue-100 dark:bg-blue-900/40 font-semibold'
                  : ''}"
                style="padding-left: {depth * 20 + 12}px"
              >
                <i class="bi bi-folder-fill text-amber-500 mr-2"></i>
                <span class="text-gray-900 dark:text-white text-sm"
                  >{folder.name}</span
                >
                {#if depth > 0}
                  <span class="text-xs text-gray-500 dark:text-gray-400 ml-2">
                    ({folderPath})
                  </span>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Preview destination -->
      <div
        class="bg-gray-50 dark:bg-gray-800/50 rounded-lg p-3 border border-gray-200 dark:border-gray-700"
      >
        <p class="text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
          {tr("newLocation")}:
        </p>
        <p class="text-sm font-mono text-gray-900 dark:text-white">
          {createNewFolder && newFolderName
            ? `/${newFolderName}/${file.name}`
            : destinationPath
              ? `${destinationPath}/${file.name}`
              : `/${file.name}`}
        </p>
      </div>

      {#if error}
        <div
          class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3"
        >
          <p class="text-sm text-red-800 dark:text-red-200">
            {error}
          </p>
        </div>
      {/if}
    </div>
  {/if}

  {#snippet actions()}
    <button
      type="button"
      class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
      onclick={handleClose}
      disabled={loading}
    >
      {tr("cancel")}
    </button>
    <button
      type="button"
      class="px-4 py-2 text-sm font-medium text-white bg-purple-600 hover:bg-purple-700 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      onclick={handleMove}
      disabled={loading || !file || (createNewFolder && !newFolderName)}
    >
      {#if loading}
        <svg
          class="animate-spin h-4 w-4"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="4"
          ></circle>
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          ></path>
        </svg>
        {tr("moving")}
      {:else}
        <i class="bi bi-arrow-right-square"></i>
        {tr("move")}
      {/if}
    </button>
  {/snippet}
</Modal>
