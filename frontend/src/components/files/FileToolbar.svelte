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
    selectedFiles = [],
  } = $props();

  const sortOptions = $derived([
    { value: "name", label: tr("name"), icon: "bi-text-left" },
    { value: "modified", label: tr("modified"), icon: "bi-calendar" },
    { value: "size", label: tr("size"), icon: "bi-hdd" },
    { value: "type", label: tr("type"), icon: "bi-file-earmark" },
  ]);
</script>

<div
  class="file-toolbar bg-white/80 dark:bg-gray-800/80 rounded-2xl shadow-xl border border-white/20 dark:border-gray-700/50 p-4 mb-6"
>
  <!-- Selection Mode Active Banner -->
  {#if selectionMode}
    <div
      class="flex items-center justify-between gap-4 mb-4 pb-4 border-b border-gray-200 dark:border-gray-700"
    >
      <div class="flex items-center gap-3">
        <div class="flex items-center gap-2 text-blue-600 dark:text-blue-400">
          <i class="bi bi-check2-square text-xl" aria-hidden="true"></i>
          <span class="font-semibold">Selection Mode</span>
        </div>
        {#if selectedCount > 0}
          <div
            class="px-3 py-1 bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 rounded-full text-sm font-semibold"
          >
            {selectedCount} selected
          </div>
        {:else}
          <span class="text-sm text-gray-500 dark:text-gray-400"
            >Click on files to select them</span
          >
        {/if}
      </div>
      <div class="flex items-center gap-2">
        {#if selectedCount > 0}
          {#if onBatchRename}
            <button
              type="button"
              class="px-3 py-2 bg-purple-500 hover:bg-purple-600 text-white rounded-lg font-medium transition-all flex items-center gap-2"
              onclick={onBatchRename}
            >
              <i class="bi bi-type" aria-hidden="true"></i>
              <span class="hidden sm:inline">Rename</span>
            </button>
          {/if}
          {#if onBatchTag}
            <button
              type="button"
              class="px-3 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-all flex items-center gap-2"
              onclick={onBatchTag}
            >
              <i class="bi bi-tags" aria-hidden="true"></i>
              <span class="hidden sm:inline">Tag</span>
            </button>
          {/if}
          <button
            type="button"
            class="px-3 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg font-medium transition-all flex items-center gap-2"
            onclick={onBatchDelete}
          >
            <i class="bi bi-trash" aria-hidden="true"></i>
            <span class="hidden sm:inline">Delete</span>
          </button>
        {/if}
        <button
          type="button"
          class="px-3 py-2 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-200 rounded-lg font-medium transition-all flex items-center gap-2"
          onclick={onSelectionToggle}
        >
          <i class="bi bi-x-lg" aria-hidden="true"></i>
          <span class="hidden sm:inline">Cancel</span>
        </button>
      </div>
    </div>
  {/if}

  <div class="flex flex-wrap items-center gap-3">
    <!-- View Mode Toggle - Modern Pills -->
    <div
      class="flex items-center gap-1 bg-gray-100/80 dark:bg-gray-900/80 rounded-xl p-1 shadow-inner"
    >
      <button
        type="button"
        class="px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-200"
        class:bg-white={viewMode === "grid"}
        class:dark:bg-gray-700={viewMode === "grid"}
        class:shadow-md={viewMode === "grid"}
        class:text-gray-900={viewMode === "grid"}
        class:dark:text-white={viewMode === "grid"}
        class:text-gray-500={viewMode !== "grid"}
        class:dark:text-gray-400={viewMode !== "grid"}
        class:hover:text-gray-900={viewMode !== "grid"}
        class:dark:hover:text-gray-200={viewMode !== "grid"}
        onclick={() => (viewMode = "grid")}
        title="Grid View"
      >
        <i class="bi bi-grid-3x3-gap" aria-hidden="true"></i>
        <span class="hidden sm:inline ml-1">Grid</span>
      </button>
      <button
        type="button"
        class="px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-200"
        class:bg-white={viewMode === "list"}
        class:dark:bg-gray-700={viewMode === "list"}
        class:shadow-md={viewMode === "list"}
        class:text-gray-900={viewMode === "list"}
        class:dark:text-white={viewMode === "list"}
        class:text-gray-500={viewMode !== "list"}
        class:dark:text-gray-400={viewMode !== "list"}
        class:hover:text-gray-900={viewMode !== "list"}
        class:dark:hover:text-gray-200={viewMode !== "list"}
        onclick={() => (viewMode = "list")}
        title="List View"
      >
        <i class="bi bi-list-ul" aria-hidden="true"></i>
        <span class="hidden sm:inline ml-1">List</span>
      </button>
    </div>

    <div
      class="h-6 w-px bg-gray-300/50 dark:bg-gray-600/50 hidden md:block"
    ></div>

    <!-- Sort Controls - Inline Modern Style -->
    <div class="flex items-center gap-2">
      <span
        class="text-xs font-medium text-gray-400 dark:text-gray-500 uppercase tracking-wider hidden md:inline"
        >Sort by</span
      >
      <div
        class="flex items-center gap-1 bg-gray-100/80 dark:bg-gray-900/80 rounded-xl p-1 shadow-inner"
      >
        {#each sortOptions as option}
          <button
            type="button"
            class="px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-200 flex items-center gap-1.5"
            class:bg-white={sortBy === option.value}
            class:dark:bg-gray-700={sortBy === option.value}
            class:shadow-md={sortBy === option.value}
            class:text-gray-900={sortBy === option.value}
            class:dark:text-white={sortBy === option.value}
            class:text-gray-500={sortBy !== option.value}
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
        class="p-2 rounded-xl hover:bg-gray-100/80 dark:hover:bg-gray-800/80 text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 transition-all duration-200"
        onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
        title={sortOrder === "asc" ? "Sort Descending" : "Sort Ascending"}
      >
        <i
          class="bi bi-sort-{sortOrder === 'asc' ? 'up' : 'down'}-alt text-lg"
          aria-hidden="true"
        ></i>
      </button>
    </div>

    <div
      class="h-6 w-px bg-gray-300/50 dark:bg-gray-600/50 hidden lg:block"
    ></div>

    <!-- Quick Filter Buttons -->
    <button
      type="button"
      class="p-2.5 rounded-xl font-medium transition-all duration-200 flex items-center gap-1.5 {showFoldersOnly
        ? 'bg-gradient-to-r from-emerald-500 to-teal-500 text-white shadow-lg'
        : 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600'}"
      onclick={() => (showFoldersOnly = !showFoldersOnly)}
      title="Toggle Folders Only"
    >
      <i class="bi bi-folder text-lg" aria-hidden="true"></i>
    </button>

    <button
      type="button"
      class="p-2.5 rounded-xl font-medium transition-all duration-200 flex items-center gap-1.5 {showFavoritesOnly
        ? 'bg-gradient-to-r from-amber-400 to-orange-500 text-white shadow-lg'
        : 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 hover:bg-amber-100 dark:hover:bg-amber-900/30'}"
      onclick={() => (showFavoritesOnly = !showFavoritesOnly)}
      title="Toggle Favorites Only"
    >
      <i
        class="bi text-lg"
        class:bi-star={!showFavoritesOnly}
        class:bi-star-fill={showFavoritesOnly}
      ></i>
    </button>

    <div class="flex-1"></div>

    <!-- Action Buttons - Modern Dashboard Style -->
    <div class="flex items-center gap-2">
      <!-- Upload Button - Prominent -->
      <button
        type="button"
        class="upload-btn px-5 py-2.5 bg-gradient-to-r from-cyan-500 via-blue-500 to-purple-500 text-white rounded-xl font-semibold shadow-lg hover:shadow-xl hover:shadow-blue-500/25 transform hover:scale-105 transition-all duration-300 flex items-center gap-2"
        onclick={onUpload}
      >
        <i class="bi bi-cloud-arrow-up text-xl" aria-hidden="true"></i>
        <span>Upload</span>
      </button>

      <button
        type="button"
        class="px-4 py-2.5 bg-white/80 dark:bg-gray-700/80 text-gray-700 dark:text-gray-200 border border-gray-200/50 dark:border-gray-600/50 rounded-xl font-medium hover:bg-white dark:hover:bg-gray-600 hover:shadow-md transition-all duration-200 flex items-center gap-2"
        onclick={onNewFolder}
      >
        <i class="bi bi-folder-plus text-lg" aria-hidden="true"></i>
        <span class="hidden sm:inline">New Folder</span>
      </button>

      <!-- Template Library Button - Feature Flag controlled -->
      {#if ENABLE_TEMPLATE_LIBRARY && onNewFromTemplate}
        <button
          type="button"
          class="px-4 py-2.5 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-xl font-medium shadow-lg hover:shadow-xl transform hover:scale-105 transition-all duration-200 flex items-center gap-2"
          onclick={onNewFromTemplate}
          title={tr("templates.newFromTemplate")}
        >
          <i class="bi bi-file-earmark-plus text-lg" aria-hidden="true"></i>
          <span class="hidden md:inline">{tr("templates.newFromTemplate")}</span
          >
        </button>
      {/if}

      <button
        type="button"
        class="p-2.5 rounded-xl bg-white dark:bg-gray-700 text-gray-500 dark:text-gray-400 border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-600 hover:shadow-md transition-all duration-200"
        onclick={onRefresh}
        aria-label="Refresh files"
        title="Refresh files"
      >
        <i class="bi bi-arrow-clockwise text-lg" aria-hidden="true"></i>
      </button>

      <button
        type="button"
        class="p-2.5 rounded-xl bg-white dark:bg-gray-700 text-gray-500 dark:text-gray-400 border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-600 hover:shadow-md transition-all duration-200"
        onclick={onAdvancedSearch}
        aria-label="Advanced search"
        title="Advanced search"
      >
        <i class="bi bi-funnel text-lg" aria-hidden="true"></i>
      </button>

      <!-- Selection Mode Toggle - only show when NOT in selection mode -->
      {#if onSelectionToggle && !selectionMode}
        <button
          type="button"
          class="p-2.5 rounded-xl bg-white dark:bg-gray-700 text-gray-500 dark:text-gray-400 border border-gray-200 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-600 hover:shadow-md transition-all duration-200"
          onclick={onSelectionToggle}
          title="Select multiple files"
        >
          <i class="bi bi-check2-square text-lg" aria-hidden="true"></i>
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
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
  }

  .upload-btn {
    animation: pulse-glow 2s ease-in-out infinite;
  }

  @keyframes pulse-glow {
    0%,
    100% {
      box-shadow: 0 4px 15px rgba(59, 130, 246, 0.3);
    }
    50% {
      box-shadow: 0 4px 25px rgba(59, 130, 246, 0.5);
    }
  }
</style>
