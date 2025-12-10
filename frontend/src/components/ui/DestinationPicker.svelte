<script>
  import { onMount } from "svelte";
  import api from "../../lib/api";
  import { toast } from "../../stores/ui";
  import { t } from "../../lib/i18n";

  let isOpen = $state(false);
  let currentPath = $state("/");
  let breadcrumbs = $state([]);
  let items = $state([]);
  let isLoading = $state(false);
  let selectedPath = $state(null);
  let searchQuery = $state("");
  let filteredItems = $state([]);

  let { onSelect = null, onCancel = null, excludePaths = [] } = $props();

  const MAX_BREADCRUMB_LENGTH = 50;

  function open(startPath = "/") {
    currentPath = startPath;
    selectedPath = startPath;
    loadDirectory(startPath);
    isOpen = true;
  }

  async function loadDirectory(path) {
    try {
      isLoading = true;
      const response = await api.files.listDirectory(path);

      // Filter nur Ordner, ausser excludePaths
      items = (response.data || [])
        .filter(
          (item) =>
            item.type === "directory" && !excludePaths.includes(item.path)
        )
        .sort((a, b) => a.name.localeCompare(b.name));

      updateBreadcrumbs(path);
      filterItems();
    } catch (error) {
      console.error("Error loading directory:", error);
      toast.show(t("errors.loadDirectory"), "error");
    } finally {
      isLoading = false;
    }
  }

  function updateBreadcrumbs(path) {
    const parts = path.split("/").filter((p) => p);
    breadcrumbs = [
      { name: t("common.root"), path: "/" },
      ...parts.map((part, i) => ({
        name:
          part.length > MAX_BREADCRUMB_LENGTH
            ? part.substring(0, MAX_BREADCRUMB_LENGTH) + "..."
            : part,
        path: "/" + parts.slice(0, i + 1).join("/"),
      })),
    ];
  }

  function filterItems() {
    if (!searchQuery.trim()) {
      filteredItems = items;
      return;
    }

    const query = searchQuery.toLowerCase();
    filteredItems = items.filter((item) =>
      item.name.toLowerCase().includes(query)
    );
  }

  function navigateTo(path) {
    currentPath = path;
    selectedPath = path;
    loadDirectory(path);
  }

  function selectDirectory(item) {
    navigateTo(item.path);
  }

  function handleConfirm() {
    if (onSelect && selectedPath) {
      onSelect(selectedPath);
    }
    close();
  }

  function close() {
    isOpen = false;
    searchQuery = "";
    filteredItems = [];
  }

  function handleCancel() {
    if (onCancel) {
      onCancel();
    }
    close();
  }

  function handleKeyDown(e) {
    if (e.key === "Enter") {
      handleConfirm();
    } else if (e.key === "Escape") {
      handleCancel();
    }
  }

  onMount(() => {
    // Make open function available to parent
    return open;
  });
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
  >
    <div
      class="w-full max-w-2xl rounded-lg bg-white shadow-xl dark:bg-gray-800"
      role="dialog"
      aria-labelledby="destination-title"
      aria-modal="true"
      tabindex="0"
    >
      <!-- Header -->
      <div class="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
        <h2
          id="destination-title"
          class="text-lg font-semibold text-gray-900 dark:text-white"
        >
          {t("batch.selectDestination")}
        </h2>
      </div>

      <!-- Breadcrumbs -->
      <div class="border-b border-gray-200 px-6 py-2 dark:border-gray-700">
        <nav class="flex items-center gap-2 text-sm overflow-x-auto">
          {#each breadcrumbs as crumb, i}
            <button
              onclick={() => navigateTo(crumb.path)}
              class="whitespace-nowrap px-2 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 transition-colors"
              aria-label={t("common.navigateTo", { path: crumb.name })}
            >
              {crumb.name}
            </button>
            {#if i < breadcrumbs.length - 1}
              <span class="text-gray-400">/</span>
            {/if}
          {/each}
        </nav>
      </div>

      <!-- Search -->
      <div class="border-b border-gray-200 px-6 py-3 dark:border-gray-700">
        <input
          type="text"
          placeholder={t("common.search")}
          bind:value={searchQuery}
          onchange={() => filterItems()}
          oninput={() => filterItems()}
          class="w-full rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          aria-label={t("common.searchDirectories")}
        />
      </div>

      <!-- Directory List -->
      <div class="max-h-96 overflow-y-auto scrollbar-modern">
        {#if isLoading}
          <div class="flex items-center justify-center p-8">
            <div
              class="h-6 w-6 animate-spin rounded-full border-2 border-green-200 border-t-green-500"
            ></div>
          </div>
        {:else if filteredItems.length === 0}
          <div class="p-8 text-center text-gray-500 dark:text-gray-400">
            {t("common.noItems")}
          </div>
        {:else}
          <div class="divide-y divide-gray-200 dark:divide-gray-700">
            {#each filteredItems as item (item.path)}
              <button
                onclick={() => selectDirectory(item)}
                class={`w-full px-6 py-3 text-left transition-colors hover:bg-gray-50 dark:hover:bg-gray-700 ${
                  selectedPath === item.path
                    ? "bg-green-50 dark:bg-green-900/20"
                    : "bg-white dark:bg-gray-800"
                }`}
                aria-label={item.name}
                role="option"
                aria-selected={selectedPath === item.path}
              >
                <div class="flex items-center gap-3">
                  <i
                    class="bi bi-folder text-gray-400 dark:text-gray-500"
                    aria-hidden="true"
                  ></i>
                  <div>
                    <div class="font-medium text-gray-900 dark:text-white">
                      {item.name}
                    </div>
                    {#if item.path !== currentPath}
                      <div class="text-xs text-gray-500 dark:text-gray-400">
                        {item.path}
                      </div>
                    {/if}
                  </div>
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Current Selection Info -->
      <div
        class="border-t border-gray-200 bg-gray-50 px-6 py-3 dark:border-gray-700 dark:bg-gray-700/50"
      >
        <div class="text-sm text-gray-700 dark:text-gray-300">
          <span class="font-medium">{t("common.selected")}:</span>
          <span
            class="ml-2 font-mono text-gray-600 dark:text-gray-400 truncate"
          >
            {selectedPath}
          </span>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="border-t border-gray-200 flex gap-3 px-6 py-4 dark:border-gray-700"
      >
        <button
          onclick={handleCancel}
          class="flex-1 rounded border border-gray-300 px-4 py-2 font-medium text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
        >
          {t("common.cancel")}
        </button>
        <button
          onclick={handleConfirm}
          disabled={!selectedPath || selectedPath === currentPath}
          onkeydown={handleKeyDown}
          class="flex-1 rounded bg-green-500 px-4 py-2 font-medium text-white transition-colors hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-green-600 dark:hover:bg-green-700"
        >
          {t("common.select")}
        </button>
      </div>
    </div>
  </div>
{/if}

