<script>
  import { onMount } from "svelte";
  import { savedSearches } from "../../stores/savedSearches.svelte";
  import { advancedFilters } from "../../lib/advancedFilters";
  import { t } from "../../lib/i18n";

  let showPanel = $state(false);
  let searchName = $state("");
  let showImportExport = $state(false);
  let selectedTags = $state([]);
  let filterQuery = $state("");
  let allTags = $derived.by(() => {
    let tags;
    $savedSearches.getAllTags().subscribe((t) => (tags = t))();
    return tags || [];
  });
  let filteredSearches = $derived.by(() => {
    const fn = $savedSearches.searchSavedSearches(filterQuery);
    let result;
    fn.subscribe((s) => (result = s))();

    if (selectedTags.length === 0) return result;
    return result.filter((s) =>
      selectedTags.some((tag) => s.tags?.includes(tag))
    );
  });

  let { onSearchSelect = null } = $props();

  function handleSaveCurrentSearch() {
    if (!searchName.trim()) return;

    let filters, query;
    $advancedFilters.filters.subscribe((f) => (filters = f))();
    query = filters.query || "";

    $savedSearches.saveSearch(searchName, query, filters, selectedTags);

    searchName = "";
    selectedTags = [];
  }

  function handleUseSearch(search) {
    $savedSearches.useSearch(search.id);

    // Apply search filters
    if (search.filters) {
      for (const [key, value] of Object.entries(search.filters)) {
        if (value !== null && value !== undefined) {
          $advancedFilters.updateFilter(key, value);
        }
      }
    }

    if (onSearchSelect) {
      onSearchSelect(search);
    }
  }

  function handleDeleteSearch(id) {
    if (confirm(t("common.confirmDelete"))) {
      $savedSearches.deleteSearch(id);
    }
  }

  function handleDuplicateSearch(search) {
    const newName = prompt(
      t("savedSearches.duplicateName"),
      `${search.name} (Copy)`
    );
    if (newName) {
      $savedSearches.duplicateSearch(search.id, newName);
    }
  }

  function handleToggleFavorite(id) {
    $savedSearches.toggleFavorite(id);
  }

  function handleExport() {
    $savedSearches.exportSearches();
  }

  function handleImportFile(e) {
    const file = e.target.files?.[0];
    if (file) {
      $savedSearches.importSearches(file);
    }
  }

  function toggleTagFilter(tag) {
    if (selectedTags.includes(tag)) {
      selectedTags = selectedTags.filter((t) => t !== tag);
    } else {
      selectedTags = [...selectedTags, tag];
    }
  }

  onMount(() => {
    $savedSearches.loadSearches();
  });
</script>

<!-- Saved Searches Panel -->
<div
  class="border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800"
>
  <!-- Toggle Button -->
  <button
    onclick={() => (showPanel = !showPanel)}
    class="w-full flex items-center justify-between px-4 py-3 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
  >
    <div class="flex items-center gap-2">
      <i class="bi bi-bookmark" aria-hidden="true"></i>
      {t("savedSearches.title")}
      {#if $savedSearches.searches.length > 0}
        <span
          class="inline-block min-w-5 px-2 py-0.5 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 text-xs font-semibold"
        >
          {$savedSearches.searches.length}
        </span>
      {/if}
    </div>
    <i class={`bi bi-chevron-${showPanel ? "up" : "down"}`}></i>
  </button>

  {#if showPanel}
    <div class="border-t border-gray-200 p-4 dark:border-gray-700 space-y-4">
      <!-- Save Current Search -->
      <div class="space-y-2">
        <div class="block text-xs font-medium text-gray-700 dark:text-gray-300">
          {t("savedSearches.saveCurrent")}
        </div>
        <div class="flex gap-2">
          <input
            type="text"
            bind:value={searchName}
            placeholder={t("savedSearches.searchName")}
            class="flex-1 rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          />
          <button
            onclick={handleSaveCurrentSearch}
            disabled={!searchName.trim()}
            class="px-3 py-2 rounded bg-green-500 text-white text-sm font-medium hover:bg-green-600 disabled:opacity-50 transition-colors"
          >
            {t("common.save")}
          </button>
        </div>
      </div>

      <!-- Search & Filter -->
      <div class="space-y-2">
        <div class="block text-xs font-medium text-gray-700 dark:text-gray-300">
          {t("common.search")}
        </div>
        <input
          type="text"
          bind:value={filterQuery}
          placeholder={t("savedSearches.searchPlaceholder")}
          class="w-full rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
        />
      </div>

      <!-- Tag Filter -->
      {#if allTags.length > 0}
        <div class="space-y-2">
          <div
            class="block text-xs font-medium text-gray-700 dark:text-gray-300"
          >
            {t("common.tags")}
          </div>
          <div class="flex flex-wrap gap-2">
            {#each allTags as tag}
              <button
                onclick={() => toggleTagFilter(tag)}
                class={`px-2 py-1 rounded text-xs transition-colors ${
                  selectedTags.includes(tag)
                    ? "bg-green-500 text-white"
                    : "border border-gray-300 text-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
                }`}
              >
                {tag}
              </button>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Saved Searches List -->
      {#if filteredSearches.length === 0}
        <div class="py-4 text-center text-sm text-gray-500 dark:text-gray-400">
          {t("savedSearches.empty")}
        </div>
      {:else}
        <div class="space-y-2 max-h-64 overflow-y-auto">
          {#each filteredSearches as search (search.id)}
            <div
              class="flex items-start justify-between gap-2 rounded border border-gray-200 bg-gray-50 p-3 dark:border-gray-700 dark:bg-gray-700/30 hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
            >
              <button
                onclick={() => handleUseSearch(search)}
                class="flex-1 text-left"
              >
                <div
                  class="text-sm font-medium text-gray-900 dark:text-white truncate"
                >
                  {search.name}
                </div>
                {#if search.query}
                  <div
                    class="text-xs text-gray-600 dark:text-gray-400 truncate"
                  >
                    "{search.query}"
                  </div>
                {/if}
                {#if search.tags?.length > 0}
                  <div class="flex gap-1 mt-1 flex-wrap">
                    {#each search.tags as tag}
                      <span
                        class="inline-block px-1.5 py-0.5 rounded bg-gray-200 dark:bg-gray-600 text-xs text-gray-700 dark:text-gray-300"
                      >
                        {tag}
                      </span>
                    {/each}
                  </div>
                {/if}
                <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  {t("savedSearches.used", { count: search.usageCount || 0 })}
                </div>
              </button>

              <!-- Actions -->
              <div class="flex gap-1">
                <button
                  onclick={() => handleToggleFavorite(search.id)}
                  title={search.isFavorite
                    ? t("common.unfavorite")
                    : t("common.favorite")}
                  class={`p-1.5 rounded transition-colors ${
                    search.isFavorite
                      ? "text-yellow-500 hover:text-yellow-600"
                      : "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  }`}
                >
                  <i
                    class={`bi ${search.isFavorite ? "bi-star-fill" : "bi-star"}`}
                  ></i>
                </button>

                <button
                  onclick={() => handleDuplicateSearch(search)}
                  title={t("common.duplicate")}
                  class="p-1.5 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
                >
                  <i class="bi bi-files" aria-hidden="true"></i>
                </button>

                <button
                  onclick={() => handleDeleteSearch(search.id)}
                  title={t("common.delete")}
                  class="p-1.5 rounded text-gray-400 hover:text-red-600 dark:hover:text-red-400 transition-colors"
                  aria-label="Delete"
                >
                  <i class="bi bi-trash" aria-hidden="true"></i>
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}

      <!-- Import/Export & Clear -->
      <div
        class="border-t border-gray-200 pt-3 dark:border-gray-700 flex gap-2 flex-wrap"
      >
        <button
          onclick={() => (showImportExport = !showImportExport)}
          class="text-xs font-medium text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
        >
          {showImportExport ? t("common.hide") : t("common.importExport")}
        </button>

        {#if $savedSearches.searches.length > 0}
          <button
            onclick={() => $savedSearches.clearAll()}
            class="text-xs font-medium text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300 transition-colors"
          >
            {t("common.clearAll")}
          </button>
        {/if}
      </div>

      <!-- Import/Export Section -->
      {#if showImportExport}
        <div
          class="border-t border-gray-200 pt-3 space-y-2 dark:border-gray-700"
        >
          <button
            onclick={handleExport}
            disabled={$savedSearches.searches.length === 0}
            class="w-full px-3 py-2 rounded border border-gray-300 text-sm font-medium text-gray-700 dark:border-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-50 transition-colors"
          >
            <i class="bi bi-download mr-1" aria-hidden="true"></i>
            {t("common.export")}
          </button>

          <label
            class="w-full px-3 py-2 rounded border border-gray-300 text-sm font-medium text-gray-700 dark:border-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors cursor-pointer"
          >
            <i class="bi bi-upload mr-1" aria-hidden="true"></i>
            {t("common.import")}
            <input
              type="file"
              accept=".json"
              onchange={handleImportFile}
              class="hidden"
            />
          </label>
        </div>
      {/if}
    </div>
  {/if}
</div>
