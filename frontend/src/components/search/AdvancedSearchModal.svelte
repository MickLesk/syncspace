<script>
  import { createEventDispatcher, onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import Input from "../ui/Input.svelte";
  import FilterBar from "./FilterBar.svelte";
  import Modal from "../ui/Modal.svelte";
  import { userPreferences } from "../../stores/preferences.js";

  const dispatch = createEventDispatcher();

  // Props
  let { visible = $bindable(false) } = $props();

  // Search State
  let searchQuery = $state("");
  let activeFilters = $state({
    fileType: "all",
    sizeMin: "",
    sizeMax: "",
    dateFrom: "",
    dateTo: "",
    modifiedBy: "",
  });

  // Sort State
  let sortBy = $state("name"); // name, date, size, type
  let sortOrder = $state("asc"); // asc, desc

  // Recent Searches (from backend preferences)
  let recentSearches = $derived($userPreferences.recent_searches || []);

  // File type options
  const fileTypeOptions = [
    { value: "all", label: t($currentLang, "allFileTypes") },
    { value: "image", label: t($currentLang, "images") },
    { value: "video", label: t($currentLang, "videos") },
    { value: "audio", label: t($currentLang, "audio") },
    { value: "document", label: t($currentLang, "documents") },
    { value: "archive", label: t($currentLang, "archives") },
    { value: "code", label: t($currentLang, "code") },
    { value: "pdf", label: "PDF" },
    { value: "text", label: t($currentLang, "text") },
  ];

  // Sort options
  const sortOptions = [
    { value: "name", label: t($currentLang, "name") },
    { value: "date", label: t($currentLang, "dateModified") },
    { value: "size", label: t($currentLang, "size") },
    { value: "type", label: t($currentLang, "type") },
  ];

  // Save search to recent searches via backend
  async function saveRecentSearch(query) {
    if (!query.trim()) return;

    let searches = [...recentSearches];

    // Remove duplicates
    searches = searches.filter((s) => {
      if (typeof s === "string") return s !== query;
      return s.query !== query;
    });

    // Add new search (just the query string for simplicity)
    searches = [query, ...searches].slice(0, 10);

    await userPreferences.updatePreferences({ recent_searches: searches });
  }

  // Handle search submission
  function handleSearch() {
    if (!searchQuery.trim()) return;

    saveRecentSearch(searchQuery);

    dispatch("search", {
      query: searchQuery,
      filters: { ...activeFilters },
      sortBy,
      sortOrder,
    });

    // Don't close modal - show results
  }

  // Handle recent search click
  function handleRecentSearchClick(search) {
    // Support both string and object formats
    const query = typeof search === "string" ? search : search.query;
    searchQuery = query;
    handleSearch();
  }

  // Clear all filters
  function clearFilters() {
    activeFilters = {
      fileType: "all",
      sizeMin: "",
      sizeMax: "",
      dateFrom: "",
      dateTo: "",
      modifiedBy: "",
    };
  }

  // Remove filter
  function handleRemoveFilter(filterKey) {
    if (filterKey === "fileType") {
      activeFilters.fileType = "all";
    } else {
      activeFilters[filterKey] = "";
    }
    activeFilters = { ...activeFilters };
  }

  // Close modal
  function close() {
    visible = false;
    dispatch("close");
  }

  // Get active filter count
  let activeFilterCount = $derived(
    Object.entries(activeFilters).filter(([key, value]) => {
      if (key === "fileType") return value !== "all";
      return value !== "";
    }).length
  );

  // Format file size (MB)
  function formatFileSize(mb) {
    if (!mb) return "";
    return `${mb} MB`;
  }

  // Keyboard shortcut handler
  function handleKeydown(e) {
    if (e.key === "Escape") {
      close();
    } else if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      handleSearch();
    }
  }

  // Handle form submission (ENTER key in input)
  function handleFormSubmit(e) {
    e.preventDefault();
    handleSearch();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if visible}
  <Modal
    {visible}
    title={t($currentLang, "advancedSearch")}
    icon="funnel"
    size="lg"
    variant="primary"
    on:close={close}
  >
    <!-- Search Input -->
    <form onsubmit={handleFormSubmit} class="space-y-4">
      <div class="space-y-2">
        <div
          class="flex rounded-xl overflow-hidden border-2 border-gray-200 dark:border-gray-700 focus-within:border-blue-500 focus-within:ring-2 focus-within:ring-blue-500/20"
        >
          <span
            class="px-4 bg-gray-50 dark:bg-gray-800 flex items-center text-gray-600 dark:text-gray-400"
          >
            <i class="bi bi-search text-lg"></i>
          </span>
          <input
            type="text"
            placeholder={t($currentLang, "searchPlaceholder")}
            class="flex-1 px-4 py-3 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 outline-none"
            bind:value={searchQuery}
            autofocus
          />
        </div>
      </div>

      <!-- Active Filters Bar -->
      {#if activeFilterCount > 0}
        <FilterBar
          filters={activeFilters}
          on:removeFilter={(e) => handleRemoveFilter(e.detail.key)}
          on:clearAll={clearFilters}
        />
      {/if}

      <!-- Filters Section -->
      <hr class="my-6 border-gray-200 dark:border-gray-700" />
      <div class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-4">
        {t($currentLang, "filters")}
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- File Type Filter -->
        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            <i class="bi bi-file-earmark text-blue-600 dark:text-blue-400 mr-2"
            ></i>
            {t($currentLang, "fileType")}
          </label>
          <select
            class="w-full px-4 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-gray-900 dark:text-gray-100"
            bind:value={activeFilters.fileType}
          >
            <option value="">{t($currentLang, "selectFileType")}</option>
            {#each fileTypeOptions as option}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>
        </div>

        <!-- Date Range Filter -->
        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            <i class="bi bi-calendar text-blue-600 dark:text-blue-400 mr-2"></i>
            {t($currentLang, "dateModified")}
          </label>
          <div class="flex gap-2">
            <input
              type="date"
              class="flex-1 px-3 py-1.5 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-sm text-gray-900 dark:text-gray-100"
              bind:value={activeFilters.dateFrom}
              placeholder={t($currentLang, "from")}
            />
            <input
              type="date"
              class="flex-1 px-3 py-1.5 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-sm text-gray-900 dark:text-gray-100"
              bind:value={activeFilters.dateTo}
              placeholder={t($currentLang, "to")}
            />
          </div>
        </div>

        <!-- File Size Filter -->
        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            <i class="bi bi-hdd text-blue-600 dark:text-blue-400 mr-2"></i>
            {t($currentLang, "fileSize")}
          </label>
          <div class="flex gap-2">
            <input
              type="number"
              class="flex-1 px-3 py-1.5 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-sm text-gray-900 dark:text-gray-100"
              bind:value={activeFilters.sizeMin}
              placeholder={t($currentLang, "minSizeMB")}
              min="0"
            />
            <input
              type="number"
              class="flex-1 px-3 py-1.5 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-sm text-gray-900 dark:text-gray-100"
              bind:value={activeFilters.sizeMax}
              placeholder={t($currentLang, "maxSizeMB")}
              min="0"
            />
          </div>
        </div>

        <!-- Modified By Filter (if multi-user) -->
        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            <i class="bi bi-person text-blue-600 dark:text-blue-400 mr-2"></i>
            {t($currentLang, "modifiedBy")}
          </label>
          <input
            type="text"
            class="w-full px-3 py-1.5 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-sm text-gray-900 dark:text-gray-100"
            bind:value={activeFilters.modifiedBy}
            placeholder={t($currentLang, "username")}
          />
        </div>
      </div>

      <!-- Sort Options -->
      <hr class="my-6 border-gray-200 dark:border-gray-700" />
      <div class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-4">
        {t($currentLang, "sortOptions")}
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Sort By -->
        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            <i class="bi bi-sort-down text-blue-600 dark:text-blue-400 mr-2"
            ></i>
            {t($currentLang, "sortBy")}
          </label>
          <select
            class="w-full px-4 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-gray-900 dark:text-gray-100"
            bind:value={sortBy}
          >
            {#each sortOptions as option}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>
        </div>

        <!-- Sort Order -->
        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          >
            <i class="bi bi-arrow-down-up text-blue-600 dark:text-blue-400 mr-2"
            ></i>
            {t($currentLang, "sortOrder")}
          </label>
          <div
            class="flex rounded-xl overflow-hidden border-2 border-gray-200 dark:border-gray-700"
          >
            <button
              type="button"
              class="flex-1 px-4 py-2 text-sm font-medium transition-all {sortOrder ===
              'asc'
                ? 'bg-blue-600 text-white'
                : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              onclick={() => (sortOrder = "asc")}
            >
              <i class="bi bi-sort-alpha-down mr-2"></i>
              {t($currentLang, "ascending")}
            </button>
            <button
              type="button"
              class="flex-1 px-4 py-2 text-sm font-medium transition-all border-l-2 border-gray-200 dark:border-gray-700 {sortOrder ===
              'desc'
                ? 'bg-blue-600 text-white'
                : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              onclick={() => (sortOrder = "desc")}
            >
              <i class="bi bi-sort-alpha-down-alt mr-2"></i>
              {t($currentLang, "descending")}
            </button>
          </div>
        </div>
      </div>

      <!-- Recent Searches -->
      {#if recentSearches.length > 0}
        <hr class="my-6 border-gray-200 dark:border-gray-700" />
        <div
          class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-4"
        >
          {t($currentLang, "recentSearches")}
        </div>

        <div class="flex flex-wrap gap-2">
          {#each recentSearches as search}
            <button
              type="button"
              class="px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-all text-sm flex items-center gap-2"
              onclick={() => handleRecentSearchClick(search)}
            >
              <i class="bi bi-clock-history"></i>
              {search.query}
            </button>
          {/each}
        </div>
      {/if}
    </form>

    <!-- Action Buttons Slot -->
    <div slot="actions" class="flex gap-3 justify-between w-full">
      <button
        type="button"
        class="px-4 py-2 border-2 border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800 transition-all text-sm flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
        onclick={clearFilters}
        disabled={activeFilterCount === 0}
      >
        <i class="bi bi-x-circle"></i>
        {t($currentLang, "clearFilters")}
      </button>

      <div class="flex gap-3">
        <button
          type="button"
          class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-xl transition-all"
          onclick={close}
        >
          {t($currentLang, "cancel")}
        </button>
        <button
          type="button"
          class="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-xl hover:from-blue-700 hover:to-purple-700 transition-all flex items-center gap-2 shadow-lg shadow-blue-500/25"
          onclick={handleSearch}
        >
          <i class="bi bi-search"></i>
          {t($currentLang, "search")}
          {#if activeFilterCount > 0}
            <span
              class="px-2 py-0.5 bg-white/20 rounded-full text-xs font-medium"
              >{activeFilterCount}</span
            >
          {/if}
        </button>
      </div>
    </div>
  </Modal>
{/if}

<style>
  .btn-group {
    display: flex;
    border-radius: 0.75rem;
    overflow: hidden;
  }

  .btn-group .btn {
    border-radius: 0;
    border-right: 1px solid rgba(0, 0, 0, 0.1);
  }

  .btn-group .btn:first-child {
    border-top-left-radius: 0.75rem;
    border-bottom-left-radius: 0.75rem;
  }

  .btn-group .btn:last-child {
    border-top-right-radius: 0.75rem;
    border-bottom-right-radius: 0.75rem;
    border-right: none;
  }
</style>
