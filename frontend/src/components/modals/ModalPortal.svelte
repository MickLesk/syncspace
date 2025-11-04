<script>
  /**
   * Global Modal Container
   * Central hub for all application modals
   * Renders all modals in a single location at the root level
   * Communicates with pages via event dispatching
   */
  import { modals, modalEvents } from "../../stores/modals.js";
  import { currentPath } from "../../stores/ui.js";

  // Modal Components
  import Modal from "../ui/Modal.svelte";
  import FileUploadZone from "../files/FileUploadZone.svelte";
  import AdvancedSearchModal from "../search/AdvancedSearchModal.svelte";
  import FilePreviewModal from "../files/FilePreviewModal.svelte";

  let newFolderName = $state("");
  let newFileName = $state("");
  let folderColor = $state("#3B82F6"); // Default to first color
  let randomButtonColor = $state("#3B82F6"); // Random button's own generated color
  let customColor = $state("#3B82F6"); // Custom picker's color

  // 8 fixed colors with good variety (total 10 with Random + Custom)
  const folderColors = [
    { name: "Blue", value: "#3B82F6", emoji: "💙" },
    { name: "Green", value: "#10B981", emoji: "💚" },
    { name: "Purple", value: "#8B5CF6", emoji: "💜" },
    { name: "Orange", value: "#F59E0B", emoji: "🧡" },
    { name: "Pink", value: "#EC4899", emoji: "🩷" },
    { name: "Red", value: "#EF4444", emoji: "❤️" },
    { name: "Yellow", value: "#EAB308", emoji: "💛" },
    { name: "Cyan", value: "#06B6D4", emoji: "🩵" },
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
</script>

<!-- Upload Modal -->
<Modal
  visible={$modals.upload.visible}
  title="Upload Files"
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
  title="Create New Folder"
  onclose={() => modals.close("newFolder")}
>
  <div class="space-y-4">
    <div>
      <label
        for="folder-name"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Folder Name
      </label>
      <input
        id="folder-name"
        type="text"
        bind:value={newFolderName}
        placeholder="Enter folder name"
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
      <label
        class="block text-sm font-medium mb-3 text-gray-700 dark:text-gray-300"
      >
        Folder Color
      </label>
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
            title={color.name}
          >
            <span class="text-xl">{color.emoji}</span>
            <span
              class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
              >{color.name}</span
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
          title="Random Color"
        >
          <span class="text-xl">🎲</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >Random</span
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
          title="Custom Color"
        >
          <input
            type="color"
            bind:value={customColor}
            onchange={() => (folderColor = customColor)}
            class="opacity-0 absolute pointer-events-none"
          />
          <span class="text-xl">🎨</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >Custom</span
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
            Selected Color
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
        Cancel
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
        Create Folder
      </button>
    </div>
  </div>
</Modal>

<!-- Rename Modal -->
<Modal
  visible={$modals.rename.visible}
  title="Rename File"
  onclose={() => modals.close("rename")}
>
  <div class="space-y-4">
    <div>
      <label
        for="new-name"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        New Name
      </label>
      <input
        id="new-name"
        type="text"
        bind:value={newFileName}
        placeholder="Enter new name"
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
        Cancel
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
        Rename
      </button>
    </div>
  </div>
</Modal>

<!-- Change Folder Color Modal -->
<Modal
  visible={$modals.changeFolderColor.visible}
  title="Change Folder Color"
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
          {$modals.changeFolderColor.data?.name || "Folder"}
        </span>
      </div>
    </div>

    <!-- Folder Color Selection (Identical to New Folder Modal) -->
    <div>
      <label
        class="block text-sm font-medium mb-3 text-gray-700 dark:text-gray-300"
      >
        Select Color
      </label>
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
            title={color.name}
          >
            <span class="text-xl">{color.emoji}</span>
            <span
              class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
              >{color.name}</span
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
          title="Random Color"
        >
          <span class="text-xl">🎲</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >Random</span
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
          title="Custom Color"
        >
          <input
            type="color"
            bind:value={customColor}
            onchange={() => (folderColor = customColor)}
            class="opacity-0 absolute pointer-events-none"
          />
          <span class="text-xl">🎨</span>
          <span class="text-[9px] font-medium text-gray-700 dark:text-gray-300"
            >Custom</span
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
            Selected Color
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
        Cancel
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
        Save Color
      </button>
    </div>
  </div>
</Modal>

<!-- Delete Confirmation Modal -->
<Modal
  visible={$modals.delete.visible}
  title="Delete File"
  onclose={() => modals.close("delete")}
  variant="danger"
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Are you sure you want to delete <strong
        >{$modals.delete.data?.name}</strong
      >? This action cannot be undone.
    </p>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("delete")}
      >
        Cancel
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
        Delete
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
  title="Move File"
  onclose={() => modals.close("move")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Move <strong>{$modals.move.data?.name}</strong> to:
    </p>
    <div>
      <label
        for="move-destination"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Destination Path
      </label>
      <input
        id="move-destination"
        type="text"
        placeholder="/path/to/destination"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("move")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          console.log("Move file:", $modals.move.data);
          modals.close("move");
        }}
      >
        <i class="bi bi-arrow-right"></i>
        Move
      </button>
    </div>
  </div>
</Modal>

<!-- Copy File Modal -->
<Modal
  visible={$modals.copy.visible}
  title="Copy File"
  onclose={() => modals.close("copy")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Copy <strong>{$modals.copy.data?.name}</strong> to:
    </p>
    <div>
      <label
        for="copy-destination"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Destination Path
      </label>
      <input
        id="copy-destination"
        type="text"
        placeholder="/path/to/destination"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
      />
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("copy")}
      >
        Cancel
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
        Copy
      </button>
    </div>
  </div>
</Modal>

<!-- Share File Modal -->
<Modal
  visible={$modals.share.visible}
  title="Share File"
  onclose={() => modals.close("share")}
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      Share <strong>{$modals.share.data?.name}</strong>
    </p>
    <div>
      <label
        for="share-link"
        class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
      >
        Share Link (will be generated)
      </label>
      <input
        id="share-link"
        type="text"
        readonly
        value="https://syncspace.local/share/{$modals.share.data?.id ||
          'pending'}"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100"
      />
    </div>
    <div class="flex items-center gap-2">
      <input id="share-password" type="checkbox" class="rounded" />
      <label
        for="share-password"
        class="text-sm text-gray-700 dark:text-gray-300"
      >
        Password protect
      </label>
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("share")}
      >
        Cancel
      </button>
      <button
        type="button"
        class="px-4 py-2 bg-primary-500 text-white rounded font-medium"
        onclick={() => {
          console.log("Create share link:", $modals.share.data);
          modals.close("share");
        }}
      >
        <i class="bi bi-share"></i>
        Create Link
      </button>
    </div>
  </div>
</Modal>

<!-- Version History Modal -->
<Modal
  visible={$modals.versionHistory.visible}
  title="Version History"
  onclose={() => modals.close("versionHistory")}
  size="large"
>
  <div class="space-y-4">
    <p class="text-gray-700 dark:text-gray-300">
      History for <strong>{$modals.versionHistory.data?.name}</strong>
    </p>
    <div class="space-y-2">
      <div class="p-3 bg-gray-100 dark:bg-gray-800 rounded-lg">
        <div class="flex justify-between items-center">
          <div>
            <p class="font-medium text-gray-900 dark:text-gray-100">
              Version 1.0 (Current)
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Modified: {new Date().toLocaleString()}
            </p>
          </div>
          <button
            type="button"
            class="px-3 py-1 text-sm bg-primary-500 text-white rounded"
          >
            <i class="bi bi-download"></i>
            Download
          </button>
        </div>
      </div>
      <div class="p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
        <p class="text-gray-500 dark:text-gray-400 text-center">
          No previous versions available
        </p>
      </div>
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded"
        onclick={() => modals.close("versionHistory")}
      >
        Close
      </button>
    </div>
  </div>
</Modal>
