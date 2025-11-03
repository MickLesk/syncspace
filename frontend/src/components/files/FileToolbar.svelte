<script>
  let {
    viewMode = $bindable("grid"),
    sortBy = $bindable("name"),
    sortOrder = $bindable("asc"),
    showFoldersOnly = $bindable(false),
    onRefresh,
    onUpload,
    onNewFolder,
    onAdvancedSearch,
    selectionMode = false,
    onSelectionToggle,
    selectedCount = 0,
    onBatchDelete,
  } = $props();

  const sortOptions = [
    { value: "name", label: "Name", icon: "bi-text-left" },
    { value: "modified", label: "Modified", icon: "bi-calendar" },
    { value: "size", label: "Size", icon: "bi-hdd" },
    { value: "type", label: "Type", icon: "bi-file-earmark" },
  ];
</script>

<div class="file-toolbar bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
  <div class="flex flex-wrap items-center gap-3">
    <!-- View Mode Toggle -->
    <div class="btn-group">
      <button
        type="button"
        class="btn btn-sm"
        class:btn-primary={viewMode === "grid"}
        class:btn-ghost={viewMode !== "grid"}
        onclick={() => (viewMode = "grid")}
        title="Grid View"
      >
        <i class="bi bi-grid-3x3-gap"></i>
      </button>
      <button
        type="button"
        class="btn btn-sm"
        class:btn-primary={viewMode === "list"}
        class:btn-ghost={viewMode !== "list"}
        onclick={() => (viewMode = "list")}
        title="List View"
      >
        <i class="bi bi-list-ul"></i>
      </button>
    </div>

    <!-- Sort Dropdown -->
    <div class="dropdown">
      <button type="button" tabindex="0" class="btn btn-sm btn-ghost">
        <i class="bi bi-sort-down"></i>
        <span class="hidden md:inline"
          >Sort: {sortOptions.find((o) => o.value === sortBy)?.label}</span
        >
        <i class="bi bi-chevron-down text-xs"></i>
      </button>
      <ul
        role="menu"
        tabindex="0"
        class="dropdown-content menu bg-base-100 rounded-box z-10 w-52 p-2 shadow-lg"
      >
        {#each sortOptions as option}
          <li>
            <button
              type="button"
              class:active={sortBy === option.value}
              onclick={() => (sortBy = option.value)}
            >
              <i class={option.icon}></i>
              {option.label}
              {#if sortBy === option.value}
                <i class="bi bi-check ml-auto"></i>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
    </div>

    <!-- Sort Order Toggle -->
    <button
      type="button"
      class="btn btn-sm btn-ghost"
      onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
      title="Sort Order"
    >
      <i class="bi bi-sort-{sortOrder === 'asc' ? 'up' : 'down'}-alt"></i>
    </button>

    <!-- Folders Only Toggle -->
    <label class="label cursor-pointer gap-2">
      <input
        type="checkbox"
        class="checkbox checkbox-sm"
        bind:checked={showFoldersOnly}
      />
      <span class="label-text hidden md:inline">Folders Only</span>
      <i class="bi bi-folder md:hidden" title="Folders Only"></i>
    </label>

    <div class="divider divider-horizontal mx-0"></div>

    <!-- Action Buttons -->
    <div class="flex flex-wrap gap-2 flex-1">
      <button type="button" class="btn btn-sm btn-primary" onclick={onUpload}>
        <i class="bi bi-upload"></i>
        <span class="hidden md:inline">Upload</span>
      </button>

      <button type="button" class="btn btn-sm btn-ghost" onclick={onNewFolder}>
        <i class="bi bi-folder-plus"></i>
        <span class="hidden md:inline">New Folder</span>
      </button>

      <button
        type="button"
        class="btn btn-sm btn-ghost"
        onclick={onRefresh}
        aria-label="Refresh files"
        title="Refresh files"
      >
        <i class="bi bi-arrow-clockwise"></i>
      </button>

      <button
        type="button"
        class="btn btn-sm btn-ghost"
        onclick={onAdvancedSearch}
        aria-label="Advanced search"
        title="Advanced search"
      >
        <i class="bi bi-search"></i>
      </button>

      <!-- Selection Mode -->
      {#if onSelectionToggle}
        <button
          type="button"
          class="btn btn-sm"
          class:btn-info={selectionMode}
          class:btn-ghost={!selectionMode}
          onclick={onSelectionToggle}
        >
          <i class="bi bi-check2-square"></i>
          <span class="hidden md:inline">Select</span>
        </button>
      {/if}

      {#if selectionMode && selectedCount > 0}
        <div class="badge badge-info gap-1">
          {selectedCount} selected
        </div>
        <button
          type="button"
          class="btn btn-sm btn-error"
          onclick={onBatchDelete}
        >
          <i class="bi bi-trash"></i>
          Delete
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
  }

  .btn-group {
    display: flex;
    gap: 0;
  }

  .btn-group .btn {
    border-radius: 0;
  }

  .btn-group .btn:first-child {
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .btn-group .btn:last-child {
    border-top-right-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
  }
</style>
