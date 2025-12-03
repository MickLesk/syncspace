<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import * as api from "../../lib/api.js";
  import { onMount } from "svelte";

  let { onFilterChange = () => {}, searchQuery = "" } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let filters = $state({
    fileType: "all",
    dateRange: "anytime",
    minSize: "",
    maxSize: "",
    tags: [],
  });

  let facets = $state({
    file_types: {},
    size_ranges: {},
    date_ranges: {},
  });

  let loading = $state(false);

  const fileTypes = [
    "all",
    "images",
    "videos",
    "documents",
    "audio",
    "archives",
  ];
  const dateRanges = ["anytime", "today", "week", "month", "year"];

  // Load facets on mount and when search query changes
  onMount(async () => {
    await loadFacets();
  });

  $effect(() => {
    // Reload facets when search query changes
    if (searchQuery !== undefined) {
      loadFacets();
    }
  });

  async function loadFacets() {
    if (loading) return;
    loading = true;
    try {
      const result = await api.search.facets(searchQuery || "");
      facets = result;
    } catch (error) {
      console.error("Failed to load facets:", error);
    } finally {
      loading = false;
    }
  }

  function updateFilters() {
    onFilterChange(filters);
  }

  function formatCount(count) {
    if (!count) return "";
    return `(${count})`;
  }
</script>

<div class="filter-panel">
  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span class="text-sm text-gray-500">Loading filters...</span>
    </div>
  {:else}
    <!-- File Type Facets -->
    <div class="filter-group">
      <div class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-2">
        File Type
      </div>
      <div class="facet-list">
        {#each Object.entries(facets.file_types || {}) as [type, count]}
          <label class="facet-item">
            <input
              type="radio"
              name="fileType"
              value={type}
              bind:group={filters.fileType}
              onchange={updateFilters}
              class="facet-radio"
            />
            <span class="facet-label">
              {type.charAt(0).toUpperCase() + type.slice(1)}
              <span class="facet-count">{formatCount(count)}</span>
            </span>
          </label>
        {/each}
        {#if Object.keys(facets.file_types || {}).length === 0}
          <div class="text-sm text-gray-500">No files found</div>
        {/if}
      </div>
    </div>

    <!-- Size Range Facets -->
    <div class="filter-group">
      <div class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-2">
        File Size
      </div>
      <div class="facet-list">
        {#each Object.entries(facets.size_ranges || {}) as [range, count]}
          <label class="facet-item">
            <input type="checkbox" class="facet-checkbox" />
            <span class="facet-label">
              {range}
              <span class="facet-count">{formatCount(count)}</span>
            </span>
          </label>
        {/each}
      </div>
    </div>

    <!-- Date Range Facets -->
    <div class="filter-group">
      <div class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-2">
        Modified
      </div>
      <div class="facet-list">
        {#each Object.entries(facets.date_ranges || {}) as [range, count]}
          <label class="facet-item">
            <input
              type="radio"
              name="dateRange"
              value={range}
              bind:group={filters.dateRange}
              onchange={updateFilters}
              class="facet-radio"
            />
            <span class="facet-label">
              {range}
              <span class="facet-count">{formatCount(count)}</span>
            </span>
          </label>
        {/each}
      </div>
    </div>

    <button
      class="px-3 py-1.5 text-sm hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded-lg transition-colors w-full flex items-center justify-center gap-2"
      onclick={() => {
        filters = {
          fileType: "all",
          dateRange: "anytime",
          minSize: "",
          maxSize: "",
          tags: [],
        };
        updateFilters();
      }}
    >
      <i class="bi bi-x-circle" aria-hidden="true"></i> Clear Filters
    </button>
  {/if}
</div>

<style>
  .filter-panel {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1rem;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
  }
  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem;
  }
  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid hsl(var(--bc) / 0.2);
    border-top-color: hsl(var(--p));
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  .facet-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .facet-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  .facet-item:hover {
    background: hsl(var(--bc) / 0.05);
  }
  .facet-radio,
  .facet-checkbox {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }
  .facet-label {
    flex: 1;
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .facet-count {
    color: hsl(var(--bc) / 0.5);
    font-size: 0.75rem;
  }
</style>
