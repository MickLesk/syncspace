<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Feature flags
  const ENABLE_TEMPLATE_LIBRARY = false; // Disabled for now - template feature not fully implemented

  let {
    viewMode = $bindable("grid"),
    sortBy = $bindable("name"),
    sortOrder = $bindable("asc"),
    showFoldersOnly = $bindable(false),
    showFavoritesOnly = $bindable(false),
    onRefresh,
    onUpload,
    onNewFolder,
    onNewFromTemplate,
    onAdvancedSearch,
    selectionMode = false,
    onSelectionToggle,
    selectedCount = 0,
    onBatchDelete,
    onBatchTag,
    onBatchRename,
    onBatchCopy,
    onBatchMove,
    onBatchDownload,
    onBatchShare,
    onBatchFavorite,
    selectedFiles = [],
  } = $props();

  const sortOptions = $derived([
    { value: "name", label: tr("name"), icon: "bi-text-left" },
    { value: "modified", label: tr("modified"), icon: "bi-calendar" },
    { value: "size", label: tr("size"), icon: "bi-hdd" },
    { value: "type", label: tr("type"), icon: "bi-file-earmark" },
  ]);
</script>

<div class="file-toolbar">
  <!-- Selection Mode Active Banner -->
  {#if selectionMode}
    <div class="selection-banner">
      <div class="selection-info">
        <div class="selection-label">
          <i class="bi bi-check2-square"></i>
          <span>Selection Mode</span>
        </div>
        {#if selectedCount > 0}
          <div class="selection-count">{selectedCount} selected</div>
        {:else}
          <span class="selection-hint">Click on files to select them</span>
        {/if}
      </div>
      <div class="selection-actions">
        {#if selectedCount > 0}
          {#if onBatchCopy}
            <button
              type="button"
              class="btn-action blue"
              onclick={onBatchCopy}
              title={tr("copy")}
            >
              <i class="bi bi-copy"></i>
              <span class="hidden sm:inline">{tr("copy")}</span>
            </button>
          {/if}
          {#if onBatchMove}
            <button
              type="button"
              class="btn-action cyan"
              onclick={onBatchMove}
              title={tr("move")}
            >
              <i class="bi bi-arrow-right-square"></i>
              <span class="hidden sm:inline">{tr("move")}</span>
            </button>
          {/if}
          {#if onBatchDownload}
            <button
              type="button"
              class="btn-action teal"
              onclick={onBatchDownload}
              title={tr("download")}
            >
              <i class="bi bi-download"></i>
              <span class="hidden sm:inline">{tr("download")}</span>
            </button>
          {/if}
          {#if onBatchShare}
            <button
              type="button"
              class="btn-action indigo"
              onclick={onBatchShare}
              title={tr("share")}
            >
              <i class="bi bi-share"></i>
              <span class="hidden sm:inline">{tr("share")}</span>
            </button>
          {/if}
          {#if onBatchFavorite}
            <button
              type="button"
              class="btn-action yellow"
              onclick={onBatchFavorite}
              title={tr("favorite")}
            >
              <i class="bi bi-star"></i>
              <span class="hidden sm:inline">{tr("favorite")}</span>
            </button>
          {/if}
          {#if onBatchRename}
            <button
              type="button"
              class="btn-action purple"
              onclick={onBatchRename}
              title={tr("rename")}
            >
              <i class="bi bi-type"></i>
              <span class="hidden sm:inline">{tr("rename")}</span>
            </button>
          {/if}
          {#if onBatchTag}
            <button
              type="button"
              class="btn-action green"
              onclick={onBatchTag}
              title={tr("tags")}
            >
              <i class="bi bi-tags"></i>
              <span class="hidden sm:inline">{tr("tags")}</span>
            </button>
          {/if}
          <button
            type="button"
            class="btn-action red"
            onclick={onBatchDelete}
            title={tr("delete")}
          >
            <i class="bi bi-trash"></i>
            <span class="hidden sm:inline">{tr("delete")}</span>
          </button>
        {/if}
        <button
          type="button"
          class="btn-action gray"
          onclick={onSelectionToggle}
        >
          <i class="bi bi-x-lg"></i>
          <span class="hidden sm:inline">{tr("cancel")}</span>
        </button>
      </div>
    </div>
  {/if}

  <div class="toolbar-content">
    <!-- View Mode Toggle -->
    <div class="toggle-group">
      <button
        type="button"
        class="toggle-btn"
        class:active={viewMode === "grid"}
        onclick={() => (viewMode = "grid")}
        title="Grid View"
      >
        <i class="bi bi-grid-3x3-gap"></i>
        <span class="hidden sm:inline">Grid</span>
      </button>
      <button
        type="button"
        class="toggle-btn"
        class:active={viewMode === "list"}
        onclick={() => (viewMode = "list")}
        title="List View"
      >
        <i class="bi bi-list-ul"></i>
        <span class="hidden sm:inline">List</span>
      </button>
    </div>

    <div class="divider hidden md:block"></div>

    <!-- Sort Controls -->
    <div class="sort-controls">
      <span class="sort-label hidden md:inline">Sort by</span>
      <div class="toggle-group">
        {#each sortOptions as option}
          <button
            type="button"
            class="toggle-btn"
            class:active={sortBy === option.value}
            onclick={() => (sortBy = option.value)}
            title="Sort by {option.label}"
          >
            <i class={option.icon}></i>
            <span class="hidden lg:inline">{option.label}</span>
          </button>
        {/each}
      </div>

      <!-- Sort Order Toggle -->
      <button
        type="button"
        class="icon-btn"
        onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
        title={sortOrder === "asc" ? "Sort Descending" : "Sort Ascending"}
      >
        <i class="bi bi-sort-{sortOrder === 'asc' ? 'up' : 'down'}-alt"></i>
      </button>
    </div>

    <div class="divider hidden lg:block"></div>

    <!-- Quick Filter Buttons -->
    <button
      type="button"
      class="filter-btn"
      class:active={showFoldersOnly}
      onclick={() => (showFoldersOnly = !showFoldersOnly)}
      title="Toggle Folders Only"
    >
      <i class="bi bi-folder"></i>
    </button>

    <button
      type="button"
      class="filter-btn favorite"
      class:active={showFavoritesOnly}
      onclick={() => (showFavoritesOnly = !showFavoritesOnly)}
      title="Toggle Favorites Only"
    >
      <i
        class="bi"
        class:bi-star={!showFavoritesOnly}
        class:bi-star-fill={showFavoritesOnly}
      ></i>
    </button>

    <div class="flex-1"></div>

    <!-- Action Buttons -->
    <div class="action-buttons">
      <!-- Upload Button - Primary -->
      <button type="button" class="btn-upload" onclick={onUpload}>
        <i class="bi bi-cloud-arrow-up"></i>
        <span>Upload</span>
      </button>

      <button type="button" class="btn-secondary" onclick={onNewFolder}>
        <i class="bi bi-folder-plus"></i>
        <span class="hidden sm:inline">New Folder</span>
      </button>

      <!-- Template Library Button - Feature Flag controlled -->
      {#if ENABLE_TEMPLATE_LIBRARY && onNewFromTemplate}
        <button
          type="button"
          class="btn-secondary"
          onclick={onNewFromTemplate}
          title={tr("templates.newFromTemplate")}
        >
          <i class="bi bi-file-earmark-plus"></i>
          <span class="hidden md:inline">{tr("templates.newFromTemplate")}</span
          >
        </button>
      {/if}

      <button
        type="button"
        class="icon-btn"
        onclick={() => onRefresh?.()}
        aria-label="Refresh files"
        title="Refresh files"
      >
        <i class="bi bi-arrow-clockwise"></i>
      </button>

      <button
        type="button"
        class="icon-btn"
        onclick={onAdvancedSearch}
        aria-label="Advanced search"
        title="Advanced search"
      >
        <i class="bi bi-funnel"></i>
      </button>

      <!-- Selection Mode Toggle -->
      {#if onSelectionToggle && !selectionMode}
        <button
          type="button"
          class="icon-btn"
          onclick={onSelectionToggle}
          title="Select multiple files"
        >
          <i class="bi bi-check2-square"></i>
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .file-toolbar {
    position: sticky;
    top: 0;
    z-index: 10;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border-radius: 0.75rem;
    border: 1px solid #e5e7eb;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  :global(.dark) .file-toolbar {
    background: rgba(31, 41, 55, 0.9);
    border-color: #374151;
  }

  /* Selection Banner */
  .selection-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding-bottom: 1rem;
    margin-bottom: 1rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global(.dark) .selection-banner {
    border-bottom-color: #374151;
  }

  .selection-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .selection-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #22c55e;
    font-weight: 600;
  }

  .selection-label i {
    font-size: 1.25rem;
  }

  .selection-count {
    padding: 0.25rem 0.75rem;
    background: rgba(34, 197, 94, 0.1);
    color: #22c55e;
    border-radius: 9999px;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .selection-hint {
    font-size: 0.875rem;
    color: #6b7280;
  }

  :global(.dark) .selection-hint {
    color: #9ca3af;
  }

  .selection-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  /* Action Buttons */
  .btn-action {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    font-weight: 500;
    font-size: 0.875rem;
    border: none;
    cursor: pointer;
    transition: all 0.2s ease;
    color: white;
  }

  .btn-action.green {
    background: #22c55e;
  }
  .btn-action.green:hover {
    background: #16a34a;
  }
  .btn-action.purple {
    background: #a855f7;
  }
  .btn-action.purple:hover {
    background: #9333ea;
  }
  .btn-action.red {
    background: #ef4444;
  }
  .btn-action.red:hover {
    background: #dc2626;
  }
  .btn-action.blue {
    background: #3b82f6;
  }
  .btn-action.blue:hover {
    background: #2563eb;
  }
  .btn-action.cyan {
    background: #06b6d4;
  }
  .btn-action.cyan:hover {
    background: #0891b2;
  }
  .btn-action.teal {
    background: #14b8a6;
  }
  .btn-action.teal:hover {
    background: #0d9488;
  }
  .btn-action.indigo {
    background: #6366f1;
  }
  .btn-action.indigo:hover {
    background: #4f46e5;
  }
  .btn-action.yellow {
    background: #f59e0b;
  }
  .btn-action.yellow:hover {
    background: #d97706;
  }
  .btn-action.gray {
    background: #e5e7eb;
    color: #374151;
  }
  .btn-action.gray:hover {
    background: #d1d5db;
  }

  :global(.dark) .btn-action.gray {
    background: #374151;
    color: #e5e7eb;
  }
  :global(.dark) .btn-action.gray:hover {
    background: #4b5563;
  }

  /* Toolbar Content */
  .toolbar-content {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
  }

  /* Toggle Group */
  .toggle-group {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    background: #f3f4f6;
    border-radius: 0.5rem;
    padding: 0.25rem;
  }

  :global(.dark) .toggle-group {
    background: #1f2937;
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .toggle-btn:hover {
    color: #22c55e;
    background: rgba(34, 197, 94, 0.1);
  }

  :global(.dark) .toggle-btn:hover {
    color: #4ade80;
    background: rgba(34, 197, 94, 0.15);
  }

  .toggle-btn.active {
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
    box-shadow: 0 2px 6px rgba(34, 197, 94, 0.3);
  }

  :global(.dark) .toggle-btn.active {
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
    box-shadow: 0 2px 6px rgba(34, 197, 94, 0.3);
  }

  /* Divider */
  .divider {
    width: 1px;
    height: 1.5rem;
    background: #d1d5db;
  }

  :global(.dark) .divider {
    background: #4b5563;
  }

  /* Sort Controls */
  .sort-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .sort-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: #9ca3af;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* Filter Buttons */
  .filter-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    background: #f3f4f6;
    border: none;
    color: #6b7280;
    font-size: 1.125rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  :global(.dark) .filter-btn {
    background: #374151;
    color: #9ca3af;
  }

  .filter-btn:hover {
    background: #e5e7eb;
    color: #111827;
  }

  :global(.dark) .filter-btn:hover {
    background: #4b5563;
    color: white;
  }

  .filter-btn.active {
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
    box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
  }

  .filter-btn.favorite.active {
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
    box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
  }

  /* Icon Button */
  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    background: white;
    border: 1px solid #e5e7eb;
    color: #6b7280;
    font-size: 1.125rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  :global(.dark) .icon-btn {
    background: #374151;
    border-color: #4b5563;
    color: #9ca3af;
  }

  .icon-btn:hover {
    background: #f3f4f6;
    color: #111827;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  :global(.dark) .icon-btn:hover {
    background: #4b5563;
    color: white;
  }

  /* Action Buttons */
  .action-buttons {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .btn-upload {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
  }

  .btn-upload:hover {
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(34, 197, 94, 0.4);
  }

  .btn-upload i {
    font-size: 1.125rem;
  }

  .btn-secondary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    background: white;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  :global(.dark) .btn-secondary {
    background: #374151;
    color: #e5e7eb;
    border-color: #4b5563;
  }

  .btn-secondary:hover {
    background: #f3f4f6;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  :global(.dark) .btn-secondary:hover {
    background: #4b5563;
  }

  .btn-secondary i {
    font-size: 1.125rem;
  }

  /* Utility classes */
  .flex-1 {
    flex: 1;
  }
  .hidden {
    display: none;
  }
  @media (min-width: 640px) {
    .sm\:inline {
      display: inline;
    }
  }
  @media (min-width: 768px) {
    .md\:block {
      display: block;
    }
    .md\:inline {
      display: inline;
    }
  }
  @media (min-width: 1024px) {
    .lg\:block {
      display: block;
    }
    .lg\:inline {
      display: inline;
    }
  }
</style>
