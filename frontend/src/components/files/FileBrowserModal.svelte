<script>
  import { onMount } from "svelte";
  import api from "../../lib/api";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import ModernButton from "../ui/ModernButton.svelte";
  import ModernCard from "../ui/ModernCard.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    isOpen = $bindable(false),
    onSelect = () => {},
    mode = "file", // "file", "folder", or "both"
    title = "Select File",
  } = $props();

  let files = $state([]);
  let folders = $state([]);
  let currentPath = $state("");
  let loading = $state(false);
  let selectedItems = $state(new Set());
  let searchTerm = $state("");

  onMount(async () => {
    await loadDirectory("");
  });

  async function loadDirectory(path) {
    try {
      loading = true;
      const response = await api.files.list(path);

      // Separate files and folders
      folders = response.filter(
        (item) => item.is_directory || item.type === "directory"
      );
      files = response.filter(
        (item) => !item.is_directory && item.type !== "directory"
      );

      currentPath = path;
    } catch (err) {
      console.error("Failed to load directory:", err);
    } finally {
      loading = false;
    }
  }

  async function navigateToFolder(folderName) {
    const newPath = currentPath
      ? `${currentPath.replace(/\/$/, "")}/${folderName}`
      : folderName;
    await loadDirectory(newPath);
  }

  async function navigateUp() {
    const parts = currentPath.split("/").filter((p) => p);
    parts.pop();
    await loadDirectory(parts.join("/"));
  }

  function toggleSelection(itemPath) {
    if (selectedItems.has(itemPath)) {
      selectedItems.delete(itemPath);
    } else {
      selectedItems.add(itemPath);
    }
    selectedItems = selectedItems; // Trigger reactivity
  }

  function handleConfirm() {
    const selected = Array.from(selectedItems);
    onSelect(selected);
    isOpen = false;
    selectedItems.clear();
  }

  function handleCancel() {
    isOpen = false;
    selectedItems.clear();
  }

  $effect(() => {
    if (isOpen) {
      loadDirectory("");
      selectedItems.clear();
    }
  });

  const filteredFolders = $derived(() => {
    if (!searchTerm) return folders;
    return folders.filter((f) =>
      f.name.toLowerCase().includes(searchTerm.toLowerCase())
    );
  });

  const filteredFiles = $derived(() => {
    if (!searchTerm) return files;
    return files.filter((f) =>
      f.name.toLowerCase().includes(searchTerm.toLowerCase())
    );
  });
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-[60] p-4 modal-backdrop"
  >
    <ModernCard
      variant="glass"
      padding="none"
      class="max-w-4xl w-full max-h-[80vh] flex flex-col modal-content"
    >
      {#snippet children()}
        <!-- Header -->
        <div
          class="border-b border-gray-200 dark:border-gray-700 p-6 bg-gray-50 dark:bg-gray-800/50"
        >
          <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2">
            <i
              class="bi bi-folder2-open text-primary-600 dark:text-primary-400 mr-2"
            ></i>
            {title}
          </h2>

          <!-- Search and breadcrumb -->
          <div class="flex gap-3 mt-4">
            <div class="flex-1 relative">
              <i
                class="bi bi-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
              ></i>
              <input
                type="text"
                bind:value={searchTerm}
                placeholder={tr("searchFiles")}
                class="glass-input w-full pl-10 pr-4 py-2 rounded-lg"
              />
            </div>
            {#if currentPath}
              <ModernButton
                variant="secondary"
                icon="arrow-up"
                onclick={navigateUp}
              >
                {tr("back")}
              </ModernButton>
            {/if}
          </div>

          <!-- Breadcrumb -->
          <div
            class="flex items-center gap-2 mt-3 text-sm text-gray-600 dark:text-gray-400"
          >
            <button aria-label="Home" onclick={() => loadDirectory("")} class="hover:text-primary-600 dark:hover:text-primary-400"><i class="bi bi-house-fill" aria-hidden="true"></i></button>
            {#each currentPath.split("/").filter((p) => p) as part, i}
              <i class="bi bi-chevron-right text-xs"></i>
              <button
                onclick={() => {
                  const parts = currentPath
                    .split("/")
                    .filter((p) => p)
                    .slice(0, i + 1);
                  loadDirectory(parts.join("/"));
                }}
                class="hover:text-primary-600 dark:hover:text-primary-400"
              >
                {part}
              </button>
            {/each}
          </div>
        </div>

        <!-- File list -->
        <div
          class="flex-1 overflow-y-auto p-6 space-y-2"
          style="max-height: 50vh;"
        >
          {#if loading}
            <div class="space-y-2">
              {#each Array(5) as _}
                <div class="skeleton h-16 w-full rounded-lg"></div>
              {/each}
            </div>
          {:else}
            <!-- Folders -->
            {#if (mode === "folder" || mode === "both") && filteredFolders().length > 0}
              <div class="space-y-1">
                <h3
                  class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2"
                >
                  {tr("folders")}
                </h3>
                {#each filteredFolders() as folder}
                  <div
                    class="glass-card p-3 flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-800/50 cursor-pointer group"
                  >
                    {#if mode === "both"}
                      <input
                        type="checkbox"
                        checked={selectedItems.has(folder.path || folder.name)}
                        onchange={() =>
                          toggleSelection(folder.path || folder.name)}
                        class="checkbox checkbox-primary"
                      />
                    {/if}
                    <button
                      onclick={() => navigateToFolder(folder.name)}
                      class="flex-1 flex items-center gap-3 text-left"
                    >
                      <i
                        class="bi bi-folder-fill text-primary-600 dark:text-primary-400 text-2xl"
                      ></i>
                      <span
                        class="font-medium text-gray-900 dark:text-gray-100"
                      >
                        {folder.name}
                      </span>
                    </button>
                    <i
                      class="bi bi-chevron-right text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"
                    ></i>
                  </div>
                {/each}
              </div>
            {/if}

            <!-- Files -->
            {#if (mode === "file" || mode === "both") && filteredFiles().length > 0}
              <div
                class="space-y-1 {filteredFolders().length > 0 ? 'mt-6' : ''}"
              >
                <h3
                  class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2"
                >
                  {tr("files")}
                </h3>
                {#each filteredFiles() as file}
                  <div
                    class="glass-card p-3 flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-800/50 cursor-pointer transition-colors"
                    class:bg-primary-50={selectedItems.has(
                      file.path || file.name
                    )}
                  >
                    <input
                      type="checkbox"
                      checked={selectedItems.has(file.path || file.name)}
                      onchange={() => toggleSelection(file.path || file.name)}
                      class="checkbox checkbox-primary"
                    />
                    <i class="bi bi-file-earmark text-gray-400 text-2xl"></i>
                    <div class="flex-1">
                      <div class="font-medium text-gray-900 dark:text-gray-100">
                        {file.name}
                      </div>
                      <div class="text-xs text-gray-500 dark:text-gray-400">
                        {#if file.size}
                          {(file.size / 1024).toFixed(1)} KB
                        {/if}
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}

            {#if filteredFolders().length === 0 && filteredFiles().length === 0}
              <div class="text-center py-12">
                <i
                  class="bi bi-inbox text-6xl text-gray-300 dark:text-gray-600 mb-3"
                ></i>
                <p class="text-gray-500 dark:text-gray-400">
                  {searchTerm ? tr("noMatchingFiles") : tr("emptyFolder")}
                </p>
              </div>
            {/if}
          {/if}
        </div>

        <!-- Footer -->
        <div
          class="border-t border-gray-200 dark:border-gray-700 p-6 bg-gray-50 dark:bg-gray-800/50 flex justify-between items-center"
        >
          <div class="text-sm text-gray-600 dark:text-gray-400">
            {#if selectedItems.size > 0}
              <span
                class="font-semibold text-primary-600 dark:text-primary-400"
              >
                {selectedItems.size}
              </span>
              {tr("itemsSelected", selectedItems.size)}
            {:else}
              {tr("noItemsSelected")}
            {/if}
          </div>
          <div class="flex gap-3">
            <ModernButton variant="ghost" onclick={handleCancel}>
              {tr("cancel")}
            </ModernButton>
            <ModernButton
              variant="gradient"
              onclick={handleConfirm}
              disabled={selectedItems.size === 0}
            >
              {tr("select")} ({selectedItems.size})
            </ModernButton>
          </div>
        </div>
      {/snippet}
    </ModernCard>
  </div>
{/if}
