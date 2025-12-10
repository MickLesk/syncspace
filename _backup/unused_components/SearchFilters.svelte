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
    sizeUnit: "MB",
    dateFrom: "",
    dateTo: "",
    tags: [],
  });

  let facets = $state({
    file_types: {},
    size_ranges: {},
    date_ranges: {},
  });

  let loading = $state(false);
  let showAdvanced = $state(false);

  const fileTypeIcons = {
    document: "file-earmark-text",
    image: "file-earmark-image",
    video: "file-earmark-play",
    audio: "file-earmark-music",
    archive: "file-earmark-zip",
    code: "file-earmark-code",
    data: "file-earmark-spreadsheet",
    other: "file-earmark",
  };

  const sizePresets = [
    { label: "Any size", value: "" },
    { label: "< 1 MB", min: "", max: "1" },
    { label: "1-10 MB", min: "1", max: "10" },
    { label: "10-100 MB", min: "10", max: "100" },
    { label: "> 100 MB", min: "100", max: "" },
  ];

  const datePresets = [
    { label: "Anytime", value: "anytime" },
    { label: "Today", value: "today" },
    { label: "Last 7 days", value: "week" },
    { label: "Last 30 days", value: "month" },
    { label: "Last year", value: "year" },
    { label: "Custom range", value: "custom" },
  ];

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

  function applySizePreset(preset) {
    filters.minSize = preset.min;
    filters.maxSize = preset.max;
    updateFilters();
  }

  function formatCount(count) {
    if (!count) return "";
    return `(${count})`;
  }

  function clearAllFilters() {
    filters = {
      fileType: "all",
      dateRange: "anytime",
      minSize: "",
      maxSize: "",
      sizeUnit: "MB",
      dateFrom: "",
      dateTo: "",
      tags: [],
    };
    updateFilters();
  }

  function hasActiveFilters() {
    return (
      filters.fileType !== "all" ||
      filters.dateRange !== "anytime" ||
      filters.minSize ||
      filters.maxSize ||
      filters.dateFrom ||
      filters.dateTo
    );
  }
</script>

<div class="filter-panel">
  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span class="text-sm text-gray-500">Loading filters...</span>
    </div>
  {:else}
    <!-- Active Filters Badge -->
    {#if hasActiveFilters()}
      <div class="active-filters-badge">
        <i class="bi bi-funnel-fill text-primary-500" aria-hidden="true"></i>
        <span>Filters active</span>
        <button
          class="clear-btn"
          onclick={clearAllFilters}
          aria-label="Clear all filters"
        >
          <i class="bi bi-x" aria-hidden="true"></i>
        </button>
      </div>
    {/if}

    <!-- File Type Facets -->
    <div class="filter-group">
      <div class="filter-group-header">
        <i class="bi bi-file-earmark" aria-hidden="true"></i>
        <span>File Type</span>
      </div>
      <div class="facet-list">
        {#each Object.entries(facets.file_types || {}) as [type, count]}
          <label class="facet-item {filters.fileType === type ? 'active' : ''}">
            <input
              type="radio"
              name="fileType"
              value={type}
              bind:group={filters.fileType}
              onchange={updateFilters}
              class="facet-radio"
            />
            <i
              class="bi bi-{fileTypeIcons[type] || 'file-earmark'} text-base"
              aria-hidden="true"
            ></i>
            <span class="facet-label">
              {type.charAt(0).toUpperCase() + type.slice(1)}
            </span>
            <span class="facet-count">{count}</span>
          </label>
        {/each}
        {#if Object.keys(facets.file_types || {}).length === 0}
          <div class="empty-facet">
            <i class="bi bi-inbox" aria-hidden="true"></i>
            <span>No files found</span>
          </div>
        {/if}
      </div>
    </div>

    <!-- Date Range -->
    <div class="filter-group">
      <div class="filter-group-header">
        <i class="bi bi-calendar3" aria-hidden="true"></i>
        <span>Modified Date</span>
      </div>
      <div class="date-presets">
        {#each datePresets as preset}
          <button
            class="preset-btn {filters.dateRange === preset.value
              ? 'active'
              : ''}"
            onclick={() => {
              filters.dateRange = preset.value;
              updateFilters();
            }}
          >
            {preset.label}
          </button>
        {/each}
      </div>

      {#if filters.dateRange === "custom"}
        <div class="custom-date-range">
          <div class="date-input-group">
            <label for="date-from">From</label>
            <input
              id="date-from"
              type="date"
              bind:value={filters.dateFrom}
              onchange={updateFilters}
              class="date-input"
            />
          </div>
          <div class="date-input-group">
            <label for="date-to">To</label>
            <input
              id="date-to"
              type="date"
              bind:value={filters.dateTo}
              onchange={updateFilters}
              class="date-input"
            />
          </div>
        </div>
      {/if}
    </div>

    <!-- Size Range -->
    <div class="filter-group">
      <div class="filter-group-header">
        <i class="bi bi-hdd" aria-hidden="true"></i>
        <span>File Size</span>
      </div>
      <div class="size-presets">
        {#each sizePresets as preset}
          <button
            class="preset-btn {filters.minSize === preset.min &&
            filters.maxSize === preset.max
              ? 'active'
              : ''}"
            onclick={() => applySizePreset(preset)}
          >
            {preset.label}
          </button>
        {/each}
      </div>

      <!-- Advanced: Toggle for custom size -->
      <button
        class="advanced-toggle"
        onclick={() => (showAdvanced = !showAdvanced)}
      >
        <i
          class="bi bi-{showAdvanced ? 'chevron-up' : 'chevron-down'}"
          aria-hidden="true"
        ></i>
        Custom size range
      </button>

      {#if showAdvanced}
        <div class="custom-size-range">
          <div class="size-input-group">
            <input
              type="number"
              placeholder="Min"
              bind:value={filters.minSize}
              onchange={updateFilters}
              class="size-input"
              min="0"
            />
            <span class="size-separator">-</span>
            <input
              type="number"
              placeholder="Max"
              bind:value={filters.maxSize}
              onchange={updateFilters}
              class="size-input"
              min="0"
            />
            <select
              bind:value={filters.sizeUnit}
              onchange={updateFilters}
              class="size-unit"
            >
              <option value="KB">KB</option>
              <option value="MB">MB</option>
              <option value="GB">GB</option>
            </select>
          </div>
        </div>
      {/if}
    </div>

    <!-- Clear Button -->
    <button
      class="clear-all-btn"
      onclick={clearAllFilters}
      disabled={!hasActiveFilters()}
    >
      <i class="bi bi-x-circle" aria-hidden="true"></i>
      Clear All Filters
    </button>
  {/if}
</div>

<style>
  .filter-panel {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    padding: 1rem;
    background: var(--md-sys-color-surface);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 0.75rem;
  }

  .active-filters-badge {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .clear-btn {
    margin-left: auto;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 0.25rem;
    color: inherit;
    opacity: 0.7;
    transition: opacity 0.2s;
  }

  .clear-btn:hover {
    opacity: 1;
  }

  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .filter-group-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    padding-bottom: 0.25rem;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
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
    border: 3px solid var(--md-sys-color-outline-variant);
    border-top-color: var(--md-sys-color-primary);
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
    gap: 0.25rem;
  }

  .facet-item {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0.625rem;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.15s ease;
    background: transparent;
  }

  .facet-item:hover {
    background: var(--md-sys-color-surface-container);
  }

  .facet-item.active {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .facet-radio {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
    accent-color: var(--md-sys-color-primary);
  }

  .facet-label {
    flex: 1;
    font-size: 0.8125rem;
  }

  .facet-count {
    font-size: 0.75rem;
    padding: 0.125rem 0.5rem;
    background: var(--md-sys-color-surface-container-highest);
    border-radius: 1rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .empty-facet {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    color: var(--md-sys-color-on-surface-variant);
    font-size: 0.8125rem;
  }

  /* Date & Size Presets */
  .date-presets,
  .size-presets {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
  }

  .preset-btn {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
    background: var(--md-sys-color-surface-container);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 1rem;
    cursor: pointer;
    transition: all 0.15s ease;
    color: var(--md-sys-color-on-surface);
  }

  .preset-btn:hover {
    background: var(--md-sys-color-surface-container-high);
  }

  .preset-btn.active {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    border-color: var(--md-sys-color-primary);
  }

  /* Custom Date Range */
  .custom-date-range {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .date-input-group {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .date-input-group label {
    font-size: 0.6875rem;
    text-transform: uppercase;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  .date-input {
    padding: 0.5rem;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 0.375rem;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-size: 0.8125rem;
  }

  /* Advanced Toggle */
  .advanced-toggle {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.75rem;
    color: var(--md-sys-color-primary);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem 0;
    margin-top: 0.25rem;
  }

  .advanced-toggle:hover {
    text-decoration: underline;
  }

  /* Custom Size Range */
  .custom-size-range {
    margin-top: 0.5rem;
  }

  .size-input-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .size-input {
    width: 80px;
    padding: 0.5rem;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 0.375rem;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-size: 0.8125rem;
  }

  .size-separator {
    color: var(--md-sys-color-on-surface-variant);
  }

  .size-unit {
    padding: 0.5rem;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 0.375rem;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-size: 0.8125rem;
  }

  /* Clear All Button */
  .clear-all-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    font-size: 0.8125rem;
    background: var(--md-sys-color-surface-container);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.15s ease;
    color: var(--md-sys-color-on-surface);
    margin-top: 0.5rem;
  }

  .clear-all-btn:hover:not(:disabled) {
    background: var(--md-sys-color-error-container);
    color: var(--md-sys-color-on-error-container);
    border-color: var(--md-sys-color-error);
  }

  .clear-all-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
