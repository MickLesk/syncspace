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

<div
  class="sticky top-0 z-10 bg-white/90 dark:bg-gray-800/90 backdrop-blur-lg rounded-xl border border-gray-200 dark:border-gray-700 p-4 mb-6"
>
  <!-- Selection Mode Active Banner -->
  {#if selectionMode}
    <div
      class="flex items-center justify-between gap-4 pb-4 mb-4 border-b border-gray-200 dark:border-gray-700"
    >
      <div class="flex items-center gap-3">
        <div class="flex items-center gap-2 text-green-500 font-semibold">
          <i class="bi bi-check2-square text-xl"></i>
          <span>Selection Mode</span>
        </div>
        {#if selectedCount > 0}
          <div
            class="px-3 py-1 bg-green-500/10 text-green-500 rounded-full text-sm font-semibold"
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
          {#if onBatchCopy}
            <button
              type="button"
              class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-blue-500 hover:bg-blue-600 cursor-pointer transition-all"
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
              class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-cyan-500 hover:bg-cyan-600 cursor-pointer transition-all"
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
              class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-teal-500 hover:bg-teal-600 cursor-pointer transition-all"
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
              class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-indigo-500 hover:bg-indigo-600 cursor-pointer transition-all"
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
              class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-amber-500 hover:bg-amber-600 cursor-pointer transition-all"
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
              class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-purple-500 hover:bg-purple-600 cursor-pointer transition-all"
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
              class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-green-500 hover:bg-green-600 cursor-pointer transition-all"
              onclick={onBatchTag}
              title={tr("tags")}
            >
              <i class="bi bi-tags"></i>
              <span class="hidden sm:inline">{tr("tags")}</span>
            </button>
          {/if}
          <button
            type="button"
            class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm text-white bg-red-500 hover:bg-red-600 cursor-pointer transition-all"
            onclick={onBatchDelete}
            title={tr("delete")}
          >
            <i class="bi bi-trash"></i>
            <span class="hidden sm:inline">{tr("delete")}</span>
          </button>
        {/if}
        <button
          type="button"
          class="flex items-center gap-2 px-3 py-2 rounded-lg font-medium text-sm bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-300 dark:hover:bg-gray-600 cursor-pointer transition-all"
          onclick={onSelectionToggle}
        >
          <i class="bi bi-x-lg"></i>
          <span class="hidden sm:inline">{tr("cancel")}</span>
        </button>
      </div>
    </div>
  {/if}

  <div class="flex flex-wrap items-center gap-3">
    <!-- View Mode Toggle -->
    <div
      class="flex items-center gap-1 bg-gray-100 dark:bg-gray-900 rounded-lg p-1"
    >
      <button
        type="button"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-sm font-medium cursor-pointer transition-all {viewMode ===
        'grid'
          ? 'bg-gradient-to-br from-green-500 to-green-600 text-white shadow-md shadow-green-500/30'
          : 'text-gray-500 hover:text-green-500 hover:bg-green-500/10'}"
        onclick={() => (viewMode = "grid")}
        title="Grid View"
      >
        <i class="bi bi-grid-3x3-gap"></i>
        <span class="hidden sm:inline">Grid</span>
      </button>
      <button
        type="button"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-sm font-medium cursor-pointer transition-all {viewMode ===
        'list'
          ? 'bg-gradient-to-br from-green-500 to-green-600 text-white shadow-md shadow-green-500/30'
          : 'text-gray-500 hover:text-green-500 hover:bg-green-500/10'}"
        onclick={() => (viewMode = "list")}
        title="List View"
      >
        <i class="bi bi-list-ul"></i>
        <span class="hidden sm:inline">List</span>
      </button>
    </div>

    <div class="w-px h-6 bg-gray-300 dark:bg-gray-600 hidden md:block"></div>

    <!-- Sort Controls -->
    <div class="flex items-center gap-2">
      <span
        class="text-xs font-medium text-gray-400 uppercase tracking-wide hidden md:inline"
        >Sort by</span
      >
      <div
        class="flex items-center gap-1 bg-gray-100 dark:bg-gray-900 rounded-lg p-1"
      >
        {#each sortOptions as option}
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-sm font-medium cursor-pointer transition-all {sortBy ===
            option.value
              ? 'bg-gradient-to-br from-green-500 to-green-600 text-white shadow-md shadow-green-500/30'
              : 'text-gray-500 hover:text-green-500 hover:bg-green-500/10'}"
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
        class="flex items-center justify-center w-10 h-10 rounded-lg bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 text-gray-500 dark:text-gray-400 text-lg cursor-pointer transition-all hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white hover:shadow-md"
        onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
        title={sortOrder === "asc" ? "Sort Descending" : "Sort Ascending"}
      >
        <i class="bi bi-sort-{sortOrder === 'asc' ? 'up' : 'down'}-alt"></i>
      </button>
    </div>

    <div class="w-px h-6 bg-gray-300 dark:bg-gray-600 hidden lg:block"></div>

    <!-- Quick Filter Buttons -->
    <button
      type="button"
      class="flex items-center justify-center w-10 h-10 rounded-lg text-lg cursor-pointer transition-all {showFoldersOnly
        ? 'bg-gradient-to-br from-green-500 to-green-600 text-white shadow-lg shadow-green-500/30'
        : 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white'}"
      onclick={() => (showFoldersOnly = !showFoldersOnly)}
      title="Toggle Folders Only"
    >
      <i class="bi bi-folder"></i>
    </button>

    <button
      type="button"
      class="flex items-center justify-center w-10 h-10 rounded-lg text-lg cursor-pointer transition-all {showFavoritesOnly
        ? 'bg-gradient-to-br from-amber-500 to-amber-600 text-white shadow-lg shadow-amber-500/30'
        : 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white'}"
      onclick={() => (showFavoritesOnly = !showFavoritesOnly)}
      title="Toggle Favorites Only"
    >
      <i class="bi {showFavoritesOnly ? 'bi-star-fill' : 'bi-star'}"></i>
    </button>

    <div class="flex-1"></div>

    <!-- Action Buttons -->
    <div class="flex items-center gap-2">
      <!-- Upload Button - Primary -->
      <button
        type="button"
        class="flex items-center gap-2 px-5 py-2.5 bg-gradient-to-br from-green-500 to-green-600 text-white rounded-lg text-sm font-semibold cursor-pointer transition-all shadow-lg shadow-green-500/30 hover:-translate-y-0.5 hover:shadow-xl hover:shadow-green-500/40"
        onclick={onUpload}
      >
        <i class="bi bi-cloud-arrow-up text-lg"></i>
        <span>Upload</span>
      </button>

      <button
        type="button"
        class="flex items-center gap-2 px-4 py-2.5 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 rounded-lg text-sm font-medium cursor-pointer transition-all hover:bg-gray-100 dark:hover:bg-gray-600 hover:shadow-md"
        onclick={onNewFolder}
      >
        <i class="bi bi-folder-plus text-lg"></i>
        <span class="hidden sm:inline">New Folder</span>
      </button>

      <!-- Template Library Button - Feature Flag controlled -->
      {#if ENABLE_TEMPLATE_LIBRARY && onNewFromTemplate}
        <button
          type="button"
          class="flex items-center gap-2 px-4 py-2.5 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 rounded-lg text-sm font-medium cursor-pointer transition-all hover:bg-gray-100 dark:hover:bg-gray-600 hover:shadow-md"
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
        class="flex items-center justify-center w-10 h-10 rounded-lg bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 text-gray-500 dark:text-gray-400 text-lg cursor-pointer transition-all hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white hover:shadow-md"
        onclick={() => onRefresh?.()}
        aria-label="Refresh files"
        title="Refresh files"
      >
        <i class="bi bi-arrow-clockwise"></i>
      </button>

      <button
        type="button"
        class="flex items-center justify-center w-10 h-10 rounded-lg bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 text-gray-500 dark:text-gray-400 text-lg cursor-pointer transition-all hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white hover:shadow-md"
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
          class="flex items-center justify-center w-10 h-10 rounded-lg bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 text-gray-500 dark:text-gray-400 text-lg cursor-pointer transition-all hover:bg-gray-100 dark:hover:bg-gray-600 hover:text-gray-900 dark:hover:text-white hover:shadow-md"
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
  /* No custom styles - all Tailwind */
</style>
