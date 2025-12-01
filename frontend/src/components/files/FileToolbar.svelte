<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

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
  } = $props();

  const sortOptions = $derived([
    { value: "name", label: tr("name"), icon: "bi-text-left" },
    { value: "modified", label: tr("modified"), icon: "bi-calendar" },
    { value: "size", label: tr("size"), icon: "bi-hdd" },
    { value: "type", label: tr("type"), icon: "bi-file-earmark" },
  ]);
</script>

<div
  class="file-toolbar bg-white dark:bg-gray-800 rounded-2xl shadow-lg border border-gray-200 dark:border-gray-700 p-4 mb-6"
>
  <div class="flex flex-wrap items-center gap-4">
    <!-- View Mode Toggle - Modern Pills -->
    <div
      class="flex items-center gap-1 bg-gray-100 dark:bg-gray-900 rounded-lg p-1"
    >
      <button
        type="button"
        class="px-3 py-1.5 rounded-md text-sm font-medium transition-all duration-200"
        class:bg-white={viewMode === "grid"}
        class:dark:bg-gray-700={viewMode === "grid"}
        class:shadow-sm={viewMode === "grid"}
        class:text-gray-900={viewMode === "grid"}
        class:dark:text-white={viewMode === "grid"}
        class:text-gray-600={viewMode !== "grid"}
        class:dark:text-gray-400={viewMode !== "grid"}
        class:hover:text-gray-900={viewMode !== "grid"}
        class:dark:hover:text-gray-200={viewMode !== "grid"}
        onclick={() => (viewMode = "grid")}
        title="Grid View"
      >
        <i class="bi bi-grid-3x3-gap"></i>
        <span class="hidden sm:inline ml-1">Grid</span>
      </button>
      <button
        type="button"
        class="px-3 py-1.5 rounded-md text-sm font-medium transition-all duration-200"
        class:bg-white={viewMode === "list"}
        class:dark:bg-gray-700={viewMode === "list"}
        class:shadow-sm={viewMode === "list"}
        class:text-gray-900={viewMode === "list"}
        class:dark:text-white={viewMode === "list"}
        class:text-gray-600={viewMode !== "list"}
        class:dark:text-gray-400={viewMode !== "list"}
        class:hover:text-gray-900={viewMode !== "list"}
        class:dark:hover:text-gray-200={viewMode !== "list"}
        onclick={() => (viewMode = "list")}
        title="List View"
      >
        <i class="bi bi-list-ul"></i>
        <span class="hidden sm:inline ml-1">List</span>
      </button>
    </div>

    <div class="h-6 w-px bg-gray-300 dark:bg-gray-600 hidden md:block"></div>

    <!-- Sort Controls - Inline Modern Style -->
    <div class="flex items-center gap-2">
      <span
        class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider hidden md:inline"
        >Sort by</span
      >
      <div
        class="flex items-center gap-1 bg-gray-100 dark:bg-gray-900 rounded-lg p-1"
      >
        {#each sortOptions as option}
          <button
            type="button"
            class="px-3 py-1.5 rounded-md text-sm font-medium transition-all duration-200 flex items-center gap-1.5"
            class:bg-white={sortBy === option.value}
            class:dark:bg-gray-700={sortBy === option.value}
            class:shadow-sm={sortBy === option.value}
            class:text-gray-900={sortBy === option.value}
            class:dark:text-white={sortBy === option.value}
            class:text-gray-600={sortBy !== option.value}
            class:dark:text-gray-400={sortBy !== option.value}
            class:hover:text-gray-900={sortBy !== option.value}
            class:dark:hover:text-gray-200={sortBy !== option.value}
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
        class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 transition-all duration-200"
        onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
        title={sortOrder === "asc" ? "Sort Descending" : "Sort Ascending"}
      >
        <i class="bi bi-sort-{sortOrder === 'asc' ? 'up' : 'down'}-alt text-lg"
        ></i>
      </button>
    </div>

    <div class="h-6 w-px bg-gray-300 dark:bg-gray-600 hidden lg:block"></div>

    <!-- Quick Filter Buttons -->
    <button
      type="button"
      class="p-2 rounded-lg font-medium transition-all duration-200 flex items-center gap-1.5"
      class:bg-success-500={showFoldersOnly}
      class:text-white={showFoldersOnly}
      class:shadow-md={showFoldersOnly}
      class:bg-gray-100={!showFoldersOnly}
      class:dark:bg-gray-700={!showFoldersOnly}
      class:text-gray-600={!showFoldersOnly}
      class:dark:text-gray-400={!showFoldersOnly}
      class:hover:bg-gray-200={!showFoldersOnly}
      class:dark:hover:bg-gray-600={!showFoldersOnly}
      onclick={() => (showFoldersOnly = !showFoldersOnly)}
      title="Toggle Folders Only"
    >
      <i class="bi bi-folder"></i>
    </button>

    <button
      type="button"
      class="p-2 rounded-lg font-medium transition-all duration-200 flex items-center gap-1.5"
      class:bg-yellow-500={showFavoritesOnly}
      class:text-white={showFavoritesOnly}
      class:shadow-lg={showFavoritesOnly}
      class:bg-gray-100={!showFavoritesOnly}
      class:dark:bg-gray-700={!showFavoritesOnly}
      class:text-gray-600={!showFavoritesOnly}
      class:dark:text-gray-300={!showFavoritesOnly}
      class:hover:bg-yellow-100={!showFavoritesOnly}
      class:dark:hover:bg-yellow-900={!showFavoritesOnly}
      class:hover:text-yellow-600={!showFavoritesOnly}
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

    <!-- Action Buttons - Modern Style -->
    <div class="flex items-center gap-3">
      <button
        type="button"
        class="px-4 py-2 bg-gradient-to-r from-primary-500 to-secondary-500 text-white rounded-lg font-medium shadow-lg hover:shadow-xl transform hover:scale-105 transition-all duration-200 flex items-center gap-2"
        onclick={onUpload}
      >
        <i class="bi bi-cloud-upload text-lg"></i>
        <span class="hidden sm:inline">Upload</span>
      </button>

      <button
        type="button"
        class="px-4 py-2 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-300 dark:border-gray-600 rounded-lg font-medium hover:bg-gray-50 dark:hover:bg-gray-600 transition-all duration-200 flex items-center gap-2"
        onclick={onNewFolder}
      >
        <i class="bi bi-folder-plus text-lg"></i>
        <span class="hidden sm:inline">New Folder</span>
      </button>

      {#if onNewFromTemplate}
        <button
          type="button"
          class="px-4 py-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg font-medium shadow-lg hover:shadow-xl transform hover:scale-105 transition-all duration-200 flex items-center gap-2"
          onclick={onNewFromTemplate}
          title={tr("templates.newFromTemplate")}
        >
          <i class="bi bi-file-earmark-plus text-lg"></i>
          <span class="hidden md:inline">{tr("templates.newFromTemplate")}</span
          >
        </button>
      {/if}

      <button
        type="button"
        class="p-2 rounded-lg bg-white dark:bg-gray-700 text-gray-600 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-600 transition-all duration-200"
        onclick={onRefresh}
        aria-label="Refresh files"
        title="Refresh files"
      >
        <i class="bi bi-arrow-clockwise text-lg"></i>
      </button>

      <button
        type="button"
        class="p-2 rounded-lg bg-white dark:bg-gray-700 text-gray-600 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-600 transition-all duration-200"
        onclick={onAdvancedSearch}
        aria-label="Advanced search"
        title="Advanced search"
      >
        <i class="bi bi-funnel text-lg"></i>
      </button>

      <!-- Selection Mode -->
      {#if onSelectionToggle}
        <button
          type="button"
          class="px-4 py-2 rounded-lg font-medium transition-all duration-200 flex items-center gap-2"
          class:bg-blue-500={selectionMode}
          class:text-white={selectionMode}
          class:bg-white={!selectionMode}
          class:dark:bg-gray-700={!selectionMode}
          class:text-gray-700={!selectionMode}
          class:dark:text-gray-200={!selectionMode}
          class:border={!selectionMode}
          class:border-gray-300={!selectionMode}
          class:dark:border-gray-600={!selectionMode}
          onclick={onSelectionToggle}
        >
          <i class="bi bi-check2-square text-lg"></i>
          <span class="hidden sm:inline">Select</span>
        </button>
      {/if}

      {#if selectionMode && selectedCount > 0}
        <div
          class="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 rounded-full text-sm font-medium"
        >
          {selectedCount} selected
        </div>
        {#if onBatchTag}
          <button
            type="button"
            class="px-4 py-2 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 transition-all duration-200 flex items-center gap-2"
            onclick={onBatchTag}
          >
            <i class="bi bi-tags text-lg"></i>
            <span class="hidden sm:inline">Tag</span>
          </button>
        {/if}
        <button
          type="button"
          class="px-4 py-2 bg-red-500 text-white rounded-lg font-medium hover:bg-red-600 transition-all duration-200 flex items-center gap-2"
          onclick={onBatchDelete}
        >
          <i class="bi bi-trash text-lg"></i>
          <span class="hidden sm:inline">Delete</span>
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
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
  }
</style>
