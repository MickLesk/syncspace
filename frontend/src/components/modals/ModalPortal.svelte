<script>
  /**
   * Global Modal Container
   * Central hub for all application modals
   * Renders all modals in a single location at the root level
   * Communicates with pages via event dispatching
   */
  import { modals, modalEvents } from "../../stores/modals.js";
  import { currentPath, currentLang } from "../../stores/ui.js";
  import api from "../../lib/api.js";
  import { t } from "../../i18n.js";

  // Modal Components
  import Modal from "../ui/Modal.svelte";
  import FileUploadZone from "../files/FileUploadZone.svelte";
  import AdvancedSearchModal from "../search/AdvancedSearchModal.svelte";
  import FilePreviewModal from "../files/FilePreviewModal.svelte";
  import ShareModal from "../sharing/ShareModal.svelte";
  import VersionHistoryModal from "../files/VersionHistoryModal.svelte";

  let newFolderName = $state("");
  let newFileName = $state("");
  let folderColor = $state("#3B82F6"); // Default to first color
  let randomButtonColor = $state("#3B82F6"); // Random button's own generated color
  let customColor = $state("#3B82F6"); // Custom picker's color

  // Move Modal State
  let selectedDestinationPath = $state("");
  let folderFilter = $state("");
  let allFolders = $state([]);

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // 8 fixed colors with good variety (total 10 with Random + Custom)
  const folderColors = [
    { nameKey: "colorBlue", value: "#3B82F6", emoji: "💙" },
    { nameKey: "colorGreen", value: "#10B981", emoji: "💚" },
    { nameKey: "colorPurple", value: "#8B5CF6", emoji: "💜" },
    { nameKey: "colorOrange", value: "#F59E0B", emoji: "🧡" },
    { nameKey: "colorPink", value: "#EC4899", emoji: "🩷" },
    { nameKey: "colorRed", value: "#EF4444", emoji: "❤️" },
    { nameKey: "colorYellow", value: "#EAB308", emoji: "💛" },
    { nameKey: "colorCyan", value: "#06B6D4", emoji: "🩵" },
  ];

  // Generate completely random color (not from palette)
  function generateRandomColor() {
    const r = Math.floor(Math.random() * 256);
    const g = Math.floor(Math.random() * 256);
    const b = Math.floor(Math.random() * 256);
    const randomHex = `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
    randomButtonColor = randomHex;
    folderColor = randomHex;
  }

  // Filtered folders for Move Modal (excludes source file/folder and descendants)
  const availableFolders = $derived.by(() => {
    let filtered = allFolders.filter((folder) => {
      // Filter by search term
      if (
        folderFilter &&
        !folder.name.toLowerCase().includes(folderFilter.toLowerCase()) &&
        !folder.path.toLowerCase().includes(folderFilter.toLowerCase())
      ) {
        return false;
      }

      // Exclude the file/folder being moved
      if ($modals.move.data) {
        const sourcePath =
          $modals.move.data.path ||
          $modals.move.data.file_path ||
          $modals.move.data.name;

        // Don't allow moving into itself
        if (folder.path === sourcePath) {
          return false;
        }

        // Don't allow moving folder into its own descendants
        if (
          $modals.move.data.is_directory &&
          folder.path.startsWith(sourcePath + "/")
        ) {
          return false;
        }
      }

      return true;
    });

    // Sort alphabetically
    return filtered.sort((a, b) => a.path.localeCompare(b.path));
  });

  // Initialize on New Folder modal open
  $effect(() => {
    if ($modals.newFolder.visible) {
      folderColor = folderColors[0].value;
      randomButtonColor = folderColors[0].value;
      customColor = folderColors[0].value;
    }
  });

  // Auto-populate rename field with current filename
  $effect(() => {
    if ($modals.rename.visible && $modals.rename.data) {
      newFileName = $modals.rename.data.name || "";
    }
  });

  // Load current folder color when changeFolderColor modal opens
  $effect(() => {
    if ($modals.changeFolderColor.visible && $modals.changeFolderColor.data) {
      const savedColors = JSON.parse(
        localStorage.getItem("folderColors") || "{}"
      );
      const currentColor =
        savedColors[$modals.changeFolderColor.data?.file_path];

      // Set to current color if exists, otherwise default
      if (currentColor) {
        folderColor = currentColor;
        customColor = currentColor;
        randomButtonColor = currentColor;
      } else {
        folderColor = folderColors[0].value;
        randomButtonColor = folderColors[0].value;
        customColor = folderColors[0].value;
      }
    }
  });

  // Load all folders when Move Modal opens
  $effect(() => {
    if ($modals.move.visible) {
      loadAllFolders();
    }
  });

  // Recursively load all folders from the file system
  async function loadAllFolders() {
    const folders = [];
    const savedColors = JSON.parse(
      localStorage.getItem("folderColors") || "{}"
    );

    // Helper function to recursively scan directories
    async function scanDirectory(path = "") {
      try {
        const response = await api.files.list(path);
        const files = response.files || [];

        for (const file of files) {
          if (file.is_directory) {
            const folderPath =
              file.path ||
              file.file_path ||
              (path ? `${path}/${file.name}` : file.name);
            folders.push({
              name: file.name,
              path: folderPath,
              color: savedColors[folderPath] || null,
            });

            // Recursively scan subdirectories
            await scanDirectory(folderPath);
          }
        }
      } catch (error) {
        console.error("Error loading folders:", error);
      }
    }

    // Add root directory
    folders.push({
      name: "/ (Root)",
      path: "",
      color: null,
    });

    await scanDirectory("");
    allFolders = folders;
  }
</script>

<!-- Upload Modal -->
<Modal
  visible={$modals.upload.visible}
  title={tr("uploadFilesTitle")}
  onclose={() => modals.close("upload")}
>
  <FileUploadZone
    onFilesSelected={(files) => {
      modalEvents.emit("upload", files);
      modals.close("upload");
    }}
    currentPath={$currentPath}
  />
</Modal>

<!-- New Folder Modal -->
<Modal
  visible={$modals.newFolder.visible}
  title={tr("createNewFolder")}
  onclose={() => modals.close("newFolder")}
>
  <div class="space-y-4">
    <div>
      <label
        for="folder-name"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        {tr("folderName")}
      </label>
      <input
        id="folder-name"
        type="text"
        bind:value={newFolderName}
        placeholder={tr("enterFolderName")}
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent"
        onkeydown={(e) => {
          if (e.key === "Enter" && newFolderName.trim()) {
            modalEvents.emit("createFolder", {
              name: newFolderName,
              color: folderColor,
            });
            newFolderName = "";
            modals.close("newFolder");
          }
        }}
      />
    </div>

    <!-- Folder Color Selection -->
    <div>
      <div
        class="block text-sm font-medium mb-3 text-gray-700 dark:text-gray-300"
      >
        {tr("folderColor")}
      </div>
      <div class="grid grid-cols-5 gap-2">
        <!-- First row: 10 fixed colors -->
        {#each folderColors as color}
          <button
            type="button"
            class="aspect-square rounded-lg border-2 transition-all duration-200 hover:scale-105 flex flex-col items-center justify-center gap-0.5 p-1.5"
            class:border-gray-300={folderColor !== color.value}
            class:dark:border-gray-600={folderColor !== color.value}
            class:border-primary-500={folderColor === color.value}
            class:ring-2={folderColor === color.value}
            class:ring-primary-500={folderColor === color.value}
            style="background-color: {color.value}20;"
            onclick={() => (folderColor = color.value)}
            title={tr(color.nameKey)}
          >
            <span class="text-xl">{color.emoji}</span>
            <span
              class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
              >{tr(color.nameKey)}</span
            >
          </button>
        {/each}

        <!-- Random Button - Shows its own randomized color -->
        <button
          type="button"
          class="aspect-square rounded-lg border-2 transition-all duration-200 hover:scale-105 flex flex-col items-center justify-center gap-0.5 p-1.5"
          class:border-gray-300={folderColor !== randomButtonColor}
          class:dark:border-gray-600={folderColor !== randomButtonColor}
          class:border-primary-500={folderColor === randomButtonColor}
          class:ring-2={folderColor === randomButtonColor}
          class:ring-primary-500={folderColor === randomButtonColor}
          style="background-color: {randomButtonColor}20;"
          onclick={generateRandomColor}
          title={tr("randomColor")}
        >
          <span class="text-xl">🎲</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >{tr("random")}</span
          >
        </button>

        <!-- Custom Color Button - Shows user's selected color -->
        <label
          class="aspect-square rounded-lg border-2 transition-all duration-200 hover:scale-105 flex flex-col items-center justify-center gap-0.5 p-1.5 cursor-pointer"
          class:border-gray-300={folderColor !== customColor}
          class:dark:border-gray-600={folderColor !== customColor}
          class:border-primary-500={folderColor === customColor}
          class:ring-2={folderColor === customColor}
          class:ring-primary-500={folderColor === customColor}
          style="background-color: {customColor}20;"
          title={tr("customColor")}
        >
          <input
            type="color"
            bind:value={customColor}
            onchange={() => (folderColor = customColor)}
            class="opacity-0 absolute pointer-events-none"
          />
          <span class="text-xl">🎨</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >{tr("custom")}</span
          >
        </label>
      </div>

      <!-- Color Preview -->
      <div
        class="mt-3 p-3 rounded-lg bg-gray-50 dark:bg-gray-800 flex items-center gap-3"
      >
        <div
          class="w-10 h-10 rounded-lg border-2 border-gray-300 dark:border-gray-600 shadow-sm"
          style="background-color: {folderColor};"
        ></div>
        <div class="flex-1">
          <div class="text-xs font-medium text-gray-500 dark:text-gray-400">
            {tr("selectedColor")}
          </div>
          <div class="text-sm font-mono text-gray-900 dark:text-gray-100">
            {folderColor}
          </div>
        </div>
      </div>
    </div>

    <div class="flex justify-end gap-2 pt-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
        onclick={() => modals.close("newFolder")}
      >
        {tr("cancel")}
      </button>
      <button
        type="button"
        class="px-4 py-2 rounded-lg font-medium text-white transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
        style="background-color: {folderColor};"
        onclick={() => {
          if (newFolderName.trim()) {
            modalEvents.emit("createFolder", {
              name: newFolderName,
              color: folderColor,
            });
            newFolderName = "";
            modals.close("newFolder");
          }
        }}
        disabled={!newFolderName.trim()}
      >
        <i class="bi bi-folder-plus"></i>
        {tr("create")}
      </button>
    </div>
  </div>
</Modal>

<!-- Rename Modal -->
<Modal
  visible={$modals.rename.visible}
  title={tr("renameFile")}
  onclose={() => modals.close("rename")}
>
  <div class="space-y-4">
    <div>
      <label
        for="new-name"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        {tr("newName")}
      </label>
      <input
        id="new-name"
        type="text"
        bind:value={newFileName}
        placeholder={tr("enterNewName")}
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
        onkeydown={(e) => {
          if (e.key === "Enter" && newFileName.trim()) {
            modalEvents.emit("renameFile", {
              file: $modals.rename.data,
              newName: newFileName,
            });
            newFileName = "";
            modals.close("rename");
          }
        }}
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("rename")}
      >
        {tr("cancel")}
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          if (newFileName.trim()) {
            modalEvents.emit("renameFile", {
              file: $modals.rename.data,
              newName: newFileName,
            });
            newFileName = "";
            modals.close("rename");
          }
        }}
        disabled={!newFileName.trim()}
      >
        <i class="bi bi-pencil"></i>
        {tr("rename")}
      </button>
    </div>
  </div>
</Modal>

<!-- Change Folder Color Modal -->
<Modal
  visible={$modals.changeFolderColor.visible}
  title={tr("changeFolderColor")}
  onclose={() => modals.close("changeFolderColor")}
>
  <div class="space-y-4">
    <!-- Current Folder Info -->
    <div class="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg">
      <div class="flex items-center gap-2">
        <i
          class="bi bi-folder text-xl"
          style={folderColor ? `color: ${folderColor}` : ""}
        ></i>
        <span class="font-medium text-gray-900 dark:text-gray-100">
          {$modals.changeFolderColor.data?.name || tr("folder")}
        </span>
      </div>
    </div>

    <!-- Folder Color Selection (Identical to New Folder Modal) -->
    <div>
      <div
        class="block text-sm font-medium mb-3 text-gray-700 dark:text-gray-300"
      >
        {tr("selectColor")}
      </div>
      <div class="grid grid-cols-5 gap-2">
        <!-- 8 fixed colors -->
        {#each folderColors as color}
          <button
            type="button"
            class="aspect-square rounded-lg border-2 transition-all duration-200 hover:scale-105 flex flex-col items-center justify-center gap-0.5 p-1.5"
            class:border-gray-300={folderColor !== color.value}
            class:dark:border-gray-600={folderColor !== color.value}
            class:border-primary-500={folderColor === color.value}
            class:ring-2={folderColor === color.value}
            class:ring-primary-500={folderColor === color.value}
            style="background-color: {color.value}20;"
            onclick={() => (folderColor = color.value)}
            title={tr(color.nameKey)}
          >
            <span class="text-xl">{color.emoji}</span>
            <span
              class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
              >{tr(color.nameKey)}</span
            >
          </button>
        {/each}

        <!-- Random Button - Shows its own randomized color -->
        <button
          type="button"
          class="aspect-square rounded-lg border-2 transition-all duration-200 hover:scale-105 flex flex-col items-center justify-center gap-0.5 p-1.5"
          class:border-gray-300={folderColor !== randomButtonColor}
          class:dark:border-gray-600={folderColor !== randomButtonColor}
          class:border-primary-500={folderColor === randomButtonColor}
          class:ring-2={folderColor === randomButtonColor}
          class:ring-primary-500={folderColor === randomButtonColor}
          style="background-color: {randomButtonColor}20;"
          onclick={generateRandomColor}
          title={tr("randomColor")}
        >
          <span class="text-xl">🎲</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >{tr("random")}</span
          >
        </button>

        <!-- Custom Color Button - Shows user's selected color -->
        <label
          class="aspect-square rounded-lg border-2 transition-all duration-200 hover:scale-105 flex flex-col items-center justify-center gap-0.5 p-1.5 cursor-pointer"
          class:border-gray-300={folderColor !== customColor}
          class:dark:border-gray-600={folderColor !== customColor}
          class:border-primary-500={folderColor === customColor}
          class:ring-2={folderColor === customColor}
          class:ring-primary-500={folderColor === customColor}
          style="background-color: {customColor}20;"
          title={tr("customColor")}
        >
          <input
            type="color"
            bind:value={customColor}
            onchange={() => (folderColor = customColor)}
            class="opacity-0 absolute pointer-events-none"
          />
          <span class="text-xl">🎨</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >{tr("custom")}</span
          >
        </label>
      </div>

      <!-- Color Preview -->
      <div
        class="mt-3 p-3 rounded-lg bg-gray-50 dark:bg-gray-800 flex items-center gap-3"
      >
        <div
          class="w-10 h-10 rounded-lg border-2 border-gray-300 dark:border-gray-600 shadow-sm"
          style="background-color: {folderColor};"
        ></div>
        <div class="flex-1">
          <div class="text-xs font-medium text-gray-500 dark:text-gray-400">
            {tr("selectedColor")}
          </div>
          <div class="text-sm font-mono text-gray-900 dark:text-gray-100">
            {folderColor}
          </div>
        </div>
      </div>
    </div>

    <!-- Action Buttons (Identical to New Folder Modal) -->
    <div class="flex justify-end gap-2 pt-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
        onclick={() => modals.close("changeFolderColor")}
      >
        {tr("cancel")}
      </button>
      <button
        type="button"
        class="px-4 py-2 rounded-lg font-medium text-white transition-all duration-200"
        style="background-color: {folderColor};"
        onclick={() => {
          modalEvents.emit("changeFolderColor", {
            file: $modals.changeFolderColor.data,
            color: folderColor,
          });
          modals.close("changeFolderColor");
        }}
      >
        <i class="bi bi-palette"></i>
        {tr("saveColor")}
      </button>
    </div>
  </div>
</Modal>

<!-- Delete Confirmation Modal -->
<Modal
  visible={$modals.delete.visible}
  title={tr("deleteConfirmTitle")}
  onclose={() => modals.close("delete")}
  variant="danger"
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      {tr("sureDeleteFile", $modals.delete.data?.name)}
    </p>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("delete")}
      >
        {tr("cancel")}
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded font-medium"
        onclick={() => {
          modalEvents.emit("deleteFile", $modals.delete.data);
          modals.close("delete");
        }}
      >
        <i class="bi bi-trash"></i>
        {tr("delete")}
      </button>
    </div>
  </div>
</Modal>

<!-- Preview Modal -->
<FilePreviewModal
  bind:visible={$modals.preview.visible}
  bind:file={$modals.preview.data}
  allFiles={$modals.preview.allFiles || []}
  currentIndex={$modals.preview.currentIndex || 0}
/>

<!-- Advanced Search Modal -->
<AdvancedSearchModal
  bind:visible={$modals.advancedSearch.visible}
  on:search={(e) => modalEvents.emit("search", e.detail)}
/>

<!-- Move File Modal -->
<Modal
  visible={$modals.move.visible}
  title={tr("moveFile")}
  onclose={() => modals.close("move")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      {tr("moveFile")} <strong>{$modals.move.data?.name}</strong>
      {tr("to")}:
    </p>

    <!-- Folder Selection -->
    <div>
      <div
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        {tr("selectDestinationFolder")}
      </div>

      <!-- Filter Input -->
      <div class="relative mb-2">
        <i
          class="bi bi-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
        ></i>
        <input
          type="text"
          bind:value={folderFilter}
          placeholder={tr("filterFolders")}
          class="w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent"
        />
      </div>

      <!-- Folder List -->
      <div
        class="max-h-60 overflow-y-auto border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-900"
      >
        {#if availableFolders.length === 0}
          <div class="p-4 text-center text-gray-500 dark:text-gray-400">
            <i class="bi bi-folder-x text-2xl mb-2"></i>
            <p class="text-sm">{tr("noFoldersAvailable")}</p>
          </div>
        {:else}
          {#each availableFolders as folder}
            <button
              type="button"
              class="w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center gap-2 border-b border-gray-200 dark:border-gray-700 last:border-0"
              class:bg-primary-50={selectedDestinationPath === folder.path}
              class:dark:bg-primary-900={selectedDestinationPath ===
                folder.path}
              class:text-primary-700={selectedDestinationPath === folder.path}
              class:dark:text-primary-300={selectedDestinationPath ===
                folder.path}
              onclick={() => (selectedDestinationPath = folder.path)}
            >
              <i
                class="bi bi-folder-fill text-xl"
                style={folder.color ? `color: ${folder.color}` : ""}
              ></i>
              <span class="flex-1 truncate">{folder.name}</span>
              <span
                class="text-xs text-gray-500 dark:text-gray-400 truncate max-w-[200px]"
              >
                {folder.path}
              </span>
            </button>
          {/each}
        {/if}
      </div>

      <!-- Selected Path Display -->
      {#if selectedDestinationPath}
        <div
          class="mt-2 p-3 bg-primary-50 dark:bg-primary-900/30 rounded-lg border border-primary-200 dark:border-primary-800"
        >
          <div
            class="text-xs font-medium text-primary-700 dark:text-primary-300 mb-1"
          >
            {tr("selectedDestination")}
          </div>
          <div class="text-sm font-mono text-gray-900 dark:text-gray-100">
            {selectedDestinationPath}
          </div>
        </div>
      {/if}
    </div>

    <!-- Action Buttons -->
    <div class="flex justify-end gap-2 pt-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
        onclick={() => {
          selectedDestinationPath = "";
          folderFilter = "";
          modals.close("move");
        }}
      >
        {tr("cancel")}
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded-lg font-medium hover:bg-primary-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        onclick={() => {
          if (selectedDestinationPath) {
            modalEvents.emit("moveFile", {
              file: $modals.move.data,
              destinationPath: selectedDestinationPath,
            });
            selectedDestinationPath = "";
            folderFilter = "";
            modals.close("move");
          }
        }}
        disabled={!selectedDestinationPath}
      >
        <i class="bi bi-arrow-right"></i>
        {tr("moveHere")}
      </button>
    </div>
  </div>
</Modal>

<!-- Copy File Modal -->
<Modal
  visible={$modals.copy.visible}
  title={tr("copyFile")}
  onclose={() => modals.close("copy")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      {tr("copyFile")} <strong>{$modals.copy.data?.name}</strong>
      {tr("to")}:
    </p>
    <div>
      <label
        for="copy-destination"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        {tr("destinationPath")}
      </label>
      <input
        id="copy-destination"
        type="text"
        placeholder={tr("pathToDestination")}
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("copy")}
      >
        {tr("cancel")}
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          console.log("Copy file:", $modals.copy.data);
          modals.close("copy");
        }}
      >
        <i class="bi bi-files"></i>
        {tr("copy")}
      </button>
    </div>
  </div>
</Modal>

<!-- Share File Modal - NEW ENHANCED VERSION -->
<ShareModal
  bind:isOpen={$modals.share.visible}
  file={$modals.share.data}
  onClose={() => modals.close("share")}
/>

<!-- Version History Modal - NEW ENHANCED VERSION -->
<VersionHistoryModal
  bind:isOpen={$modals.versionHistory.visible}
  file={$modals.versionHistory.data}
  onClose={() => modals.close("versionHistory")}
/>
