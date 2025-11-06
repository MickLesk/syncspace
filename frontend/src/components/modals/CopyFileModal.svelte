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
  let newName = $state("");
  let loading = $state(false);
  let error = $state(null);
  let availableFolders = $state([]);

  // Load available folders when modal opens
  $effect(() => {
    if (visible && file) {
      newName = file.name || "";
      loadFolders();
    }
  });

  async function loadFolders() {
    try {
      const response = await api.files.list("/");
      const data = await response.json();
      availableFolders = data.filter((item) => item.is_directory);
    } catch (err) {
      console.error("Failed to load folders:", err);
    }
  }

  async function handleCopy() {
    if (!file) return;

    loading = true;
    error = null;

    try {
      const sourcePath = file.path || file.file_path || file.name;
      const destName = newName || file.name;
      const destPath = destinationPath
        ? `${destinationPath}/${destName}`
        : destName;

      console.log("[CopyFileModal] Copying:", { sourcePath, destPath });

      await api.files.copy(sourcePath, destPath);

      success(tr("fileCopiedSuccessfully", file.name));
      onSuccess();
      handleClose();
    } catch (err) {
      console.error("[CopyFileModal] Error:", err);
      error = err.message || tr("failedToCopyFile");
      errorToast(error);
    } finally {
      loading = false;
    }
  }

  function handleClose() {
    visible = false;
    destinationPath = "";
    newName = "";
    error = null;
    onClose();
  }
</script>

<Modal
  bind:visible
  title={tr("copyFile")}
  icon="files"
  size="md"
  onclose={handleClose}
>
  {#if file}
    <div class="space-y-4">
      <!-- Source file info -->
      <div
        class="bg-primary/5 dark:bg-primary/10 rounded-lg p-4 border border-primary/20"
      >
        <div class="flex items-center gap-3">
          <i
            class="bi {file.is_directory
              ? 'bi-folder-fill'
              : 'bi-file-earmark-fill'} text-2xl text-primary"
          ></i>
          <div class="flex-1 min-w-0">
            <p
              class="font-semibold text-sm text-gray-900 dark:text-white truncate"
            >
              {file.name}
            </p>
            <p class="text-xs text-gray-600 dark:text-gray-400">
              {tr("originalFile")}
            </p>
          </div>
        </div>
      </div>

      <!-- Destination folder -->
      <label
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
      >
        {tr("destinationFolder")}
        <select
          bind:value={destinationPath}
          class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
        >
          <option value="">{tr("rootDirectory")}</option>
          {#each availableFolders as folder}
            <option value={folder.path || folder.file_path}>
              {folder.name}
            </option>
          {/each}
        </select>
      </label>

      <!-- New file name (optional) -->
      <label
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
      >
        {tr("newFileName")} ({tr("optional")})
        <input
          type="text"
          bind:value={newName}
          placeholder={file.name}
          class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent"
        />
        <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
          {tr("leaveEmptyToKeepOriginalName")}
        </p>
      </label>

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
      class="px-4 py-2 text-sm font-medium text-white bg-primary hover:bg-primary/90 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      onclick={handleCopy}
      disabled={loading || !file}
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
        {tr("copying")}
      {:else}
        <i class="bi bi-files"></i>
        {tr("copy")}
      {/if}
    </button>
  {/snippet}
</Modal>
