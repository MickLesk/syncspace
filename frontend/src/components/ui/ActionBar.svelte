<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { debounce } from "../../lib/designSystem.js";

  /**
   * ActionBar - Toolbar for page actions (search, filters, view modes, etc.)
   * @prop {boolean} showSearch - Show search input
   * @prop {string} searchPlaceholder - Search placeholder text
   * @prop {Function} onSearch - Search callback
   * @prop {boolean} showViewMode - Show view mode switcher (grid/list)
   * @prop {string} viewMode - Current view mode
   * @prop {Function} onViewModeChange - View mode change callback
   * @prop {boolean} showSort - Show sort dropdown
   * @prop {string} sortBy - Current sort field
   * @prop {string} sortOrder - Current sort order (asc/desc)
   * @prop {Array} sortOptions - Sort options [{value, label}]
   * @prop {Function} onSortChange - Sort change callback
   * @prop {boolean} sticky - Make toolbar sticky
   * @prop {boolean} showBulkActions - Show bulk action buttons
   * @prop {number} selectedCount - Number of selected items
   */

  let {
    showSearch = true,
    searchPlaceholder = "Search...",
    searchQuery = $bindable(""),
    onSearch = () => {},
    showViewMode = true,
    viewMode = $bindable("grid"),
    onViewModeChange = () => {},
    showSort = true,
    sortBy = $bindable("name"),
    sortOrder = $bindable("asc"),
    sortOptions = [
      { value: "name", label: "Name" },
      { value: "modified", label: "Modified" },
      { value: "size", label: "Size" },
      { value: "type", label: "Type" },
    ],
    onSortChange = () => {},
    sticky = true,
    showBulkActions = false,
    selectedCount = 0,
    class: className = "",
    leftActions,
    rightActions,
    bulkActions,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Debounce search
  const debouncedSearch = debounce((query) => {
    onSearch(query);
  }, 300);

  function handleSearchInput(e) {
    const value = e.target.value;
    searchQuery = value;
    debouncedSearch(value);
  }

  function handleViewModeChange(mode) {
    viewMode = mode;
    onViewModeChange(mode);
  }

  function handleSortChange(field) {
    if (sortBy === field) {
      // Toggle order if same field
      sortOrder = sortOrder === "asc" ? "desc" : "asc";
    } else {
      sortBy = field;
      sortOrder = "asc";
    }
    onSortChange(sortBy, sortOrder);
  }
</script>

<div 
  class="action-bar {sticky ? 'action-bar-sticky' : ''} {className}"
  class:has-bulk-actions={showBulkActions && selectedCount > 0}
>
  <div class="action-bar-content">
    <!-- Left Section: Search & Custom Actions -->
    <div class="action-bar-left">
      {#if showSearch}
        <div class="search-container">
          <i class="bi bi-search search-icon"></i>
          <input
            type="text"
            class="search-input"
            placeholder={searchPlaceholder}
            value={searchQuery}
            oninput={handleSearchInput}
          />
          {#if searchQuery}
            <button 
              class="search-clear"
              onclick={() => {
                searchQuery = "";
                onSearch("");
              }}
              aria-label="Clear search"
            >
              <i class="bi bi-x"></i>
            </button>
          {/if}
        </div>
      {/if}

      {#if leftActions}
        <div class="custom-actions">
          {@render leftActions?.()}
        </div>
      {/if}
    </div>

    <!-- Right Section: View Mode, Sort, Custom Actions -->
    <div class="action-bar-right">
      {#if rightActions}
        <div class="custom-actions">
          {@render rightActions?.()}
        </div>
      {/if}

      {#if showSort}
        <div class="sort-container">
          <button class="sort-button" title="Sort">
            <i class="bi bi-sort-{sortOrder === 'asc' ? 'down' : 'up'}"></i>
          </button>
          <div class="sort-dropdown">
            {#each sortOptions as option}
              <button
                class="sort-option"
                class:active={sortBy === option.value}
                onclick={() => handleSortChange(option.value)}
              >
                <i class="bi bi-{sortBy === option.value ? 'check2' : 'blank'}"></i>
                {option.label}
              </button>
            {/each}
          </div>
        </div>
      {/if}

      {#if showViewMode}
        <div class="view-mode-container">
          <button
            class="view-mode-btn"
            class:active={viewMode === "grid"}
            onclick={() => handleViewModeChange("grid")}
            title="Grid view"
          >
            <i class="bi bi-grid-3x3-gap"></i>
          </button>
          <button
            class="view-mode-btn"
            class:active={viewMode === "list"}
            onclick={() => handleViewModeChange("list")}
            title="List view"
          >
            <i class="bi bi-list-ul"></i>
          </button>
        </div>
      {/if}
    </div>
  </div>

  <!-- Bulk Actions Bar (slides down when items selected) -->
  {#if showBulkActions && selectedCount > 0}
    <div class="bulk-actions-bar">
      <div class="bulk-selection-info">
        <i class="bi bi-check-circle-fill text-primary-500"></i>
        <span>{selectedCount} {selectedCount === 1 ? 'item' : 'items'} selected</span>
      </div>
      
      {#if bulkActions}
        <div class="bulk-actions-buttons">
          {@render bulkActions?.()}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .action-bar {
    background: white;
    border-radius: 12px;
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
    margin-bottom: 1.5rem;
    transition: all 200ms cubic-bezier(0.4, 0, 0.2, 1);
  }

  :global(.dark) .action-bar {
    background: rgb(31 41 55);
    border: 1px solid rgb(55 65 81);
  }

  .action-bar-sticky {
    position: sticky;
    top: 80px;
    z-index: 20;
  }

  .action-bar.has-bulk-actions {
    padding-bottom: 0;
  }

  .action-bar-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem 1.25rem;
  }

  .action-bar-left,
  .action-bar-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .action-bar-left {
    flex: 1;
    min-width: 0;
  }

  /* Search */
  .search-container {
    position: relative;
    flex: 1;
    max-width: 400px;
  }

  .search-icon {
    position: absolute;
    left: 1rem;
    top: 50%;
    transform: translateY(-50%);
    color: rgb(156 163 175);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: 0.625rem 2.5rem 0.625rem 2.75rem;
    border: 1px solid rgb(229 231 235);
    border-radius: 8px;
    background: rgb(249 250 251);
    font-size: 0.875rem;
    transition: all 200ms cubic-bezier(0.4, 0, 0.2, 1);
  }

  :global(.dark) .search-input {
    background: rgb(17 24 39);
    border-color: rgb(55 65 81);
    color: rgb(243 244 246);
  }

  .search-input:focus {
    outline: none;
    border-color: #667eea;
    background: white;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  :global(.dark) .search-input:focus {
    background: rgb(31 41 55);
    border-color: #667eea;
  }

  .search-clear {
    position: absolute;
    right: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: rgb(156 163 175);
    cursor: pointer;
    border-radius: 4px;
    transition: all 150ms;
  }

  .search-clear:hover {
    background: rgb(243 244 246);
    color: rgb(107 114 128);
  }

  :global(.dark) .search-clear:hover {
    background: rgb(55 65 81);
    color: rgb(209 213 219);
  }

  /* View Mode */
  .view-mode-container {
    display: flex;
    gap: 0.25rem;
    background: rgb(243 244 246);
    padding: 0.25rem;
    border-radius: 8px;
  }

  :global(.dark) .view-mode-container {
    background: rgb(17 24 39);
  }

  .view-mode-btn {
    padding: 0.5rem 0.75rem;
    border: none;
    background: transparent;
    color: rgb(107 114 128);
    border-radius: 6px;
    cursor: pointer;
    transition: all 150ms;
    font-size: 1.125rem;
  }

  .view-mode-btn:hover {
    background: white;
    color: rgb(55 65 81);
  }

  :global(.dark) .view-mode-btn:hover {
    background: rgb(31 41 55);
    color: rgb(209 213 219);
  }

  .view-mode-btn.active {
    background: white;
    color: #667eea;
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  }

  :global(.dark) .view-mode-btn.active {
    background: rgb(31 41 55);
    color: #667eea;
  }

  /* Sort */
  .sort-container {
    position: relative;
  }

  .sort-button {
    padding: 0.5rem 0.75rem;
    border: 1px solid rgb(229 231 235);
    background: white;
    color: rgb(107 114 128);
    border-radius: 8px;
    cursor: pointer;
    transition: all 150ms;
    font-size: 1.125rem;
  }

  :global(.dark) .sort-button {
    background: rgb(31 41 55);
    border-color: rgb(55 65 81);
    color: rgb(209 213 219);
  }

  .sort-button:hover {
    border-color: #667eea;
    color: #667eea;
  }

  .sort-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    background: white;
    border: 1px solid rgb(229 231 235);
    border-radius: 8px;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
    min-width: 150px;
    display: none;
    z-index: 50;
  }

  :global(.dark) .sort-dropdown {
    background: rgb(31 41 55);
    border-color: rgb(55 65 81);
  }

  .sort-container:hover .sort-dropdown,
  .sort-container:focus-within .sort-dropdown {
    display: block;
  }

  .sort-option {
    width: 100%;
    padding: 0.625rem 1rem;
    border: none;
    background: transparent;
    text-align: left;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: background 150ms;
    color: rgb(55 65 81);
  }

  :global(.dark) .sort-option {
    color: rgb(209 213 219);
  }

  .sort-option:hover {
    background: rgb(243 244 246);
  }

  :global(.dark) .sort-option:hover {
    background: rgb(55 65 81);
  }

  .sort-option.active {
    color: #667eea;
    font-weight: 500;
  }

  .sort-option:first-child {
    border-radius: 8px 8px 0 0;
  }

  .sort-option:last-child {
    border-radius: 0 0 8px 8px;
  }

  /* Bulk Actions */
  .bulk-actions-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.875rem 1.25rem;
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
    border-top: 1px solid rgba(102, 126, 234, 0.2);
    animation: slideDown 200ms cubic-bezier(0.4, 0, 0.2, 1);
  }

  :global(.dark) .bulk-actions-bar {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
  }

  .bulk-selection-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
    color: rgb(55 65 81);
  }

  :global(.dark) .bulk-selection-info {
    color: rgb(209 213 219);
  }

  .bulk-actions-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .custom-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  /* Animations */
  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Responsive */
  @media (max-width: 768px) {
    .action-bar-content {
      flex-direction: column;
      gap: 0.75rem;
    }

    .action-bar-left,
    .action-bar-right {
      width: 100%;
      justify-content: space-between;
    }

    .search-container {
      max-width: none;
    }

    .bulk-actions-bar {
      flex-direction: column;
      gap: 0.75rem;
      align-items: stretch;
    }

    .bulk-actions-buttons {
      width: 100%;
      flex-wrap: wrap;
    }

    .bulk-actions-buttons :global(button) {
      flex: 1;
      min-width: 120px;
    }
  }

  @media (max-width: 640px) {
    .action-bar-sticky {
      top: 60px;
    }

    .sort-dropdown {
      left: 0;
      right: auto;
    }
  }
</style>
