<script>
  import {
    createFileFuse,
    fuzzySearch,
    sortSearchResults,
    filterResultsByType,
    debounceSearch,
  } from "../../lib/fuzzySearch.js";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import FileCard from "./FileCard.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import LoadingState from "../ui/LoadingState.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    files = [],
    onSelect = null,
    onOpen = null,
    onContextMenu = null,
    viewMode = "grid",
  } = $props();

  let query = $state("");
  let filterType = $state("all");
  let sortBy = $state("relevance");
  let results = $state([]);
  let fuse = $state(null);
  let isSearching = $state(false);

  // Initialize Fuse when files change
  $effect(() => {
    if (files && files.length > 0) {
      fuse = createFileFuse(files);
    }
  });

  // Debounced search function
  const debouncedSearch = debounceSearch(async (searchQuery) => {
    if (!fuse || !searchQuery.trim()) {
      results = [];
      isSearching = false;
      return;
    }

    isSearching = true;

    // Simulate small delay for UX feedback
    await new Promise((resolve) => setTimeout(resolve, 50));

    let searchResults = fuzzySearch(fuse, searchQuery);

    // Apply filters
    searchResults = filterResultsByType(searchResults, filterType);

    // Apply sorting
    searchResults = sortSearchResults(searchResults, sortBy);

    results = searchResults;
    isSearching = false;
  }, 300);

  // React to query changes
  $effect(() => {
    debouncedSearch(query);
  });

  function getDisplayResults() {
    // If no query, show all files
    if (!query.trim()) {
      let allFiles = [...files];
      allFiles = filterResultsByType(allFiles, filterType);
      allFiles = sortSearchResults(allFiles, sortBy);
      return allFiles;
    }
    return results;
  }

  const displayResults = $derived(getDisplayResults());
  const hasResults = $derived(displayResults.length > 0);
  const isEmpty = $derived(query.trim() !== "" && !hasResults);
</script>

<div class="fuzzy-search-panel space-y-4">
  <!-- Search Bar -->
  <div class="flex gap-2">
    <div class="flex-1 relative">
      <input
        type="text"
        placeholder={tr("searchFiles")}
        bind:value={query}
        class="w-full px-4 py-2 rounded-lg border-2 border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:border-green-500 focus:outline-none"
      />
      {#if isSearching}
        <div class="absolute right-3 top-2.5">
          <div
            class="animate-spin rounded-full h-5 w-5 border-b-2 border-green-500"
          ></div>
        </div>
      {/if}
    </div>
  </div>

  <!-- Filter & Sort Controls -->
  <div class="flex gap-2 flex-wrap">
    <select
      bind:value={filterType}
      class="px-3 py-2 rounded-lg border-2 border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 text-sm"
    >
      <option value="all">{tr("all")}</option>
      <option value="files">{tr("files")}</option>
      <option value="folders">{tr("folders")}</option>
    </select>

    <select
      bind:value={sortBy}
      class="px-3 py-2 rounded-lg border-2 border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 text-sm"
    >
      <option value="relevance">{tr("relevance")}</option>
      <option value="name">{tr("name")}</option>
      <option value="modified">{tr("modified")}</option>
      <option value="size">{tr("size")}</option>
    </select>

    {#if query.trim()}
      <div class="text-sm text-gray-600 dark:text-gray-400">
        {displayResults.length}
        {tr("results")}
      </div>
    {/if}
  </div>

  <!-- Results Grid/List -->
  {#if isEmpty}
    <EmptyState
      icon="search"
      title={tr("noSearchResults")}
      message={tr("tryDifferentSearchTerm")}
    />
  {:else if displayResults.length > 0}
    <div
      class={viewMode === "grid"
        ? "grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-4"
        : "space-y-2"}
    >
      {#each displayResults as file (file.id || file.name)}
        <FileCard
          {file}
          {viewMode}
          onSelect={(f) => onSelect?.(f)}
          onOpen={(f) => onOpen?.(f)}
          onContextMenu={(f, e) => onContextMenu?.(f, e)}
        />
      {/each}
    </div>
  {:else}
    <LoadingState message={tr("searching")} />
  {/if}
</div>
