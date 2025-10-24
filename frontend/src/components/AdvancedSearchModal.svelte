<script>
  import { createEventDispatcher, onMount } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import SelectV2 from "./ui/SelectV2.svelte";
  import InputV2 from "./ui/InputV2.svelte";
  import FilterBar from "./FilterBar.svelte";
  import Modal from "./ui/Modal.svelte";

  const dispatch = createEventDispatcher();

  // Props
  export let visible = false;

  // Search State
  let searchQuery = "";
  let activeFilters = {
    fileType: "all",
    sizeMin: "",
    sizeMax: "",
    dateFrom: "",
    dateTo: "",
    modifiedBy: "",
  };

  // Sort State
  let sortBy = "name"; // name, date, size, type
  let sortOrder = "asc"; // asc, desc

  // Recent Searches (from localStorage)
  let recentSearches = [];

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

  // Load recent searches from localStorage
  onMount(() => {
    const saved = localStorage.getItem("recentSearches");
    if (saved) {
      try {
        recentSearches = JSON.parse(saved);
      } catch (e) {
        console.error("Failed to parse recent searches:", e);
      }
    }
  });

  // Save search to recent searches
  function saveRecentSearch(query, filters) {
    if (!query.trim()) return;

    const search = {
      query,
      filters: { ...filters },
      timestamp: Date.now(),
    };

    // Remove duplicates and limit to 10
    recentSearches = [
      search,
      ...recentSearches.filter((s) => s.query !== query),
    ].slice(0, 10);

    localStorage.setItem("recentSearches", JSON.stringify(recentSearches));
  }

  // Handle search submission
  function handleSearch() {
    if (!searchQuery.trim()) return;

    saveRecentSearch(searchQuery, activeFilters);

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
    searchQuery = search.query;
    activeFilters = { ...search.filters };
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
    dispatch("close");
  }

  // Get active filter count
  $: activeFilterCount = Object.entries(activeFilters).filter(
    ([key, value]) => {
      if (key === "fileType") return value !== "all";
      return value !== "";
    }
  ).length;

  // Format file size (MB)
  function formatFileSize(mb) {
    if (!mb) return "";
    return `${mb} MB`;
  }

  // Keyboard shortcut handler
  function handleKeydown(e) {
    if (e.key === "Escape") {
      close();
    } else if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
      handleSearch();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

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
    <form on:submit|preventDefault={handleSearch} class="space-y-4">
      <div class="form-control">
        <div class="input-group flex">
          <span
            class="px-4 bg-base-200 flex items-center rounded-l-xl border border-r-0 border-base-300 text-base-content"
          >
            <i class="bi bi-search text-lg"></i>
          </span>
          <input
            type="text"
            placeholder={t($currentLang, "searchPlaceholder")}
            class="input input-bordered flex-1 rounded-l-none rounded-r-xl focus:ring-2 focus:ring-primary/50 text-base-content"
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
      <div class="divider text-sm text-base-content/60">
        {t($currentLang, "filters")}
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- File Type Filter -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base-content">
              <i class="bi bi-file-earmark mr-2"></i>
              {t($currentLang, "fileType")}
            </span>
          </label>
          <SelectV2
            bind:value={activeFilters.fileType}
            options={fileTypeOptions}
            placeholder={t($currentLang, "selectFileType")}
          />
        </div>

        <!-- Date Range Filter -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base-content">
              <i class="bi bi-calendar mr-2"></i>
              {t($currentLang, "dateModified")}
            </span>
          </label>
          <div class="flex gap-2">
            <InputV2
              type="date"
              bind:value={activeFilters.dateFrom}
              placeholder={t($currentLang, "from")}
              size="sm"
            />
            <InputV2
              type="date"
              bind:value={activeFilters.dateTo}
              placeholder={t($currentLang, "to")}
              size="sm"
            />
          </div>
        </div>

        <!-- File Size Filter -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base-content">
              <i class="bi bi-hdd mr-2"></i>
              {t($currentLang, "fileSize")}
            </span>
          </label>
          <div class="flex gap-2">
            <InputV2
              type="number"
              bind:value={activeFilters.sizeMin}
              placeholder={t($currentLang, "minSizeMB")}
              size="sm"
              min="0"
            />
            <InputV2
              type="number"
              bind:value={activeFilters.sizeMax}
              placeholder={t($currentLang, "maxSizeMB")}
              size="sm"
              min="0"
            />
          </div>
        </div>

        <!-- Modified By Filter (if multi-user) -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base-content">
              <i class="bi bi-person mr-2"></i>
              {t($currentLang, "modifiedBy")}
            </span>
          </label>
          <InputV2
            type="text"
            bind:value={activeFilters.modifiedBy}
            placeholder={t($currentLang, "username")}
            size="sm"
          />
        </div>
      </div>

      <!-- Sort Options -->
      <div class="divider text-sm text-base-content/60">
        {t($currentLang, "sortOptions")}
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Sort By -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base-content">
              <i class="bi bi-sort-down mr-2"></i>
              {t($currentLang, "sortBy")}
            </span>
          </label>
          <SelectV2 bind:value={sortBy} options={sortOptions} />
        </div>

        <!-- Sort Order -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base-content">
              <i class="bi bi-arrow-down-up mr-2"></i>
              {t($currentLang, "sortOrder")}
            </span>
          </label>
          <div class="btn-group w-full">
            <button
              type="button"
              class="btn btn-sm flex-1 {sortOrder === 'asc'
                ? 'btn-active'
                : ''}"
              on:click={() => (sortOrder = "asc")}
            >
              <i class="bi bi-sort-alpha-down mr-2"></i>
              {t($currentLang, "ascending")}
            </button>
            <button
              type="button"
              class="btn btn-sm flex-1 {sortOrder === 'desc'
                ? 'btn-active'
                : ''}"
              on:click={() => (sortOrder = "desc")}
            >
              <i class="bi bi-sort-alpha-down-alt mr-2"></i>
              {t($currentLang, "descending")}
            </button>
          </div>
        </div>
      </div>

      <!-- Recent Searches -->
      {#if recentSearches.length > 0}
        <div class="divider text-sm text-base-content/60">
          {t($currentLang, "recentSearches")}
        </div>

        <div class="flex flex-wrap gap-2">
          {#each recentSearches as search}
            <button
              type="button"
              class="btn btn-sm btn-ghost gap-2 text-base-content"
              on:click={() => handleRecentSearchClick(search)}
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
        class="btn btn-outline btn-sm gap-2"
        on:click={clearFilters}
        disabled={activeFilterCount === 0}
      >
        <i class="bi bi-x-circle"></i>
        {t($currentLang, "clearFilters")}
      </button>

      <div class="flex gap-3">
        <button type="button" class="btn btn-ghost" on:click={close}>
          {t($currentLang, "cancel")}
        </button>
        <button
          type="button"
          class="btn btn-primary gap-2"
          on:click={handleSearch}
        >
          <i class="bi bi-search"></i>
          {t($currentLang, "search")}
          {#if activeFilterCount > 0}
            <span class="badge badge-sm">{activeFilterCount}</span>
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
