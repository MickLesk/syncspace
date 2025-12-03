<script>
  import { onMount } from 'svelte';
  import { userPreferences } from '../../stores/preferences';
  import { t } from '../../lib/i18n';

  let viewMode = $state('grid');
  let sortBy = $state('name');
  let sortOrder = $state('asc');
  let filterDefaults = $state({
    showHidden: false,
    showSystem: false,
    extensions: []
  });
  let defaultSearchType = $state('all');
  let enableThumbnails = $state(true);
  let enablePreview = $state(true);
  let itemsPerPage = $state(50);

  const VIEW_MODES = [
    { id: 'grid', label: t('settings.viewMode.grid'), icon: 'bi-grid-3x3-gap' },
    { id: 'list', label: t('settings.viewMode.list'), icon: 'bi-list-ul' },
    { id: 'details', label: t('settings.viewMode.details'), icon: 'bi-list-check' }
  ];

  const SORT_OPTIONS = [
    { id: 'name', label: t('common.name') },
    { id: 'size', label: t('common.size') },
    { id: 'modified', label: t('common.modified') },
    { id: 'created', label: t('common.created') },
    { id: 'type', label: t('common.type') }
  ];

  const SORT_ORDERS = [
    { id: 'asc', label: t('common.ascending') },
    { id: 'desc', label: t('common.descending') }
  ];

  const SEARCH_TYPES = [
    { id: 'all', label: t('search.allFiles') },
    { id: 'files', label: t('fileTypes.file') },
    { id: 'folders', label: t('fileTypes.folder') }
  ];

  async function loadPreferences() {
    const prefs = $userPreferences.preferences;
    if (prefs) {
      viewMode = prefs.viewMode || 'grid';
      sortBy = prefs.sortBy || 'name';
      sortOrder = prefs.sortOrder || 'asc';
      filterDefaults = prefs.filterDefaults || filterDefaults;
      defaultSearchType = prefs.defaultSearchType || 'all';
      enableThumbnails = prefs.enableThumbnails !== false;
      enablePreview = prefs.enablePreview !== false;
      itemsPerPage = prefs.itemsPerPage || 50;
    }
  }

  async function savePreferences() {
    await $userPreferences.updatePreferences({
      viewMode,
      sortBy,
      sortOrder,
      filterDefaults,
      defaultSearchType,
      enableThumbnails,
      enablePreview,
      itemsPerPage
    });
  }

  function handleViewModeChange(mode) {
    viewMode = mode;
    savePreferences();
  }

  function handleSortChange() {
    savePreferences();
  }

  function handleFilterChange() {
    savePreferences();
  }

  function handleToggleHidden() {
    filterDefaults.showHidden = !filterDefaults.showHidden;
    handleFilterChange();
  }

  function handleToggleSystem() {
    filterDefaults.showSystem = !filterDefaults.showSystem;
    handleFilterChange();
  }

  onMount(() => {
    loadPreferences();
  });
</script>

<div class="space-y-6">
  <!-- View Mode -->
  <div>
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3">
      {t('settings.viewMode.title')}
    </h3>
    <div class="flex gap-2 flex-wrap">
      {#each VIEW_MODES as mode}
        <button
          onclick={() => handleViewModeChange(mode.id)}
          class={`flex items-center gap-2 px-4 py-2 rounded transition-colors ${
            viewMode === mode.id
              ? 'bg-blue-500 text-white'
              : 'border border-gray-300 text-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700'
          }`}
        >
          <i class={`bi ${mode.icon}`}></i>
          {mode.label}
        </button>
      {/each}
    </div>
  </div>

  <!-- Sorting -->
  <div class="grid grid-cols-2 gap-4">
    <div>
      <div class="block text-sm font-medium text-gray-900 dark:text-white mb-2">
        {t('common.sortBy')}
      </div>
      <select
        bind:value={sortBy}
        onchange={handleSortChange}
        class="w-full rounded border border-gray-300 bg-white px-3 py-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
      >
        {#each SORT_OPTIONS as option}
          <option value={option.id}>{option.label}</option>
        {/each}
      </select>
    </div>

    <div>
      <div class="block text-sm font-medium text-gray-900 dark:text-white mb-2">
        {t('common.order')}
      </div>
      <select
        bind:value={sortOrder}
        onchange={handleSortChange}
        class="w-full rounded border border-gray-300 bg-white px-3 py-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
      >
        {#each SORT_ORDERS as option}
          <option value={option.id}>{option.label}</option>
        {/each}
      </select>
    </div>
  </div>

  <!-- Filter Defaults -->
  <div>
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3">
      {t('settings.filterDefaults')}
    </h3>
    <div class="space-y-3">
      <label class="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          checked={filterDefaults.showHidden}
          onchange={handleToggleHidden}
          class="rounded border-gray-300"
        />
        <span class="text-sm text-gray-700 dark:text-gray-300">
          {t('settings.showHiddenFiles')}
        </span>
      </label>

      <label class="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          checked={filterDefaults.showSystem}
          onchange={handleToggleSystem}
          class="rounded border-gray-300"
        />
        <span class="text-sm text-gray-700 dark:text-gray-300">
          {t('settings.showSystemFiles')}
        </span>
      </label>
    </div>
  </div>

  <!-- Search Settings -->
  <div>
    <div class="block text-sm font-medium text-gray-900 dark:text-white mb-2">
      {t('settings.defaultSearchType')}
    </div>
    <select
      bind:value={defaultSearchType}
      onchange={() => {
        savePreferences();
      }}
      class="w-full rounded border border-gray-300 bg-white px-3 py-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
    >
      {#each SEARCH_TYPES as option}
        <option value={option.id}>{option.label}</option>
      {/each}
    </select>
  </div>

  <!-- Display Settings -->
  <div>
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3">
      {t('settings.displaySettings')}
    </h3>
    <div class="space-y-3">
      <label class="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          bind:checked={enableThumbnails}
          onchange={() => {
            savePreferences();
          }}
          class="rounded border-gray-300"
        />
        <span class="text-sm text-gray-700 dark:text-gray-300">
          {t('settings.enableThumbnails')}
        </span>
      </label>

      <label class="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          bind:checked={enablePreview}
          onchange={() => {
            savePreferences();
          }}
          class="rounded border-gray-300"
        />
        <span class="text-sm text-gray-700 dark:text-gray-300">
          {t('settings.enablePreview')}
        </span>
      </label>
    </div>
  </div>

  <!-- Items Per Page -->
  <div>
    <div class="block text-sm font-medium text-gray-900 dark:text-white mb-2">
      {t('settings.itemsPerPage')}
    </div>
    <div class="flex items-center gap-3">
      <input
        type="range"
        min="10"
        max="200"
        step="10"
        bind:value={itemsPerPage}
        onchange={() => {
          savePreferences();
        }}
        class="flex-1"
      />
      <span class="w-12 text-sm font-medium text-gray-700 dark:text-gray-300 text-center">
        {itemsPerPage}
      </span>
    </div>
  </div>
</div>
