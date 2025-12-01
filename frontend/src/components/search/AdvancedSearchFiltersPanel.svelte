<script>
  import { onMount } from 'svelte';
  import { advancedFilters } from '../../lib/advancedFilters';
  import { t } from '../../lib/i18n';

  let showFilters = $state(false);
  let presetName = $state('');
  let selectedPreset = $state(null);
  let fileTypes = $state([]);

  $: activeCount = $advancedFilters.activeFilterCount;

  let { onFiltersChange = null } = $props();

  async function loadFileTypes() {
    // Common file types
    fileTypes = [
      { id: 'image', label: t('fileTypes.image'), icon: 'bi-image' },
      { id: 'video', label: t('fileTypes.video'), icon: 'bi-film' },
      { id: 'audio', label: t('fileTypes.music'), icon: 'bi-music-note' },
      { id: 'document', label: t('fileTypes.document'), icon: 'bi-file-earmark-text' },
      { id: 'spreadsheet', label: t('fileTypes.spreadsheet'), icon: 'bi-file-earmark-spreadsheet' },
      { id: 'archive', label: t('fileTypes.archive'), icon: 'bi-file-earmark-zip' },
      { id: 'code', label: t('fileTypes.code'), icon: 'bi-file-earmark-code' },
    ];
  }

  function handleClearFilters() {
    $advancedFilters.clearFilters();
    onFiltersChange?.();
  }

  function handleSavePreset() {
    if (presetName.trim()) {
      $advancedFilters.savePreset(presetName);
      presetName = '';
    }
  }

  function handleLoadPreset() {
    if (selectedPreset) {
      $advancedFilters.loadPreset(selectedPreset);
      onFiltersChange?.();
    }
  }

  function handleFilterChange() {
    onFiltersChange?.();
  }

  onMount(() => {
    loadFileTypes();
  });
</script>

<div class="border-b border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800">
  <!-- Filter Toggle Button -->
  <div class="flex items-center justify-between mb-3">
    <button
      onclick={() => (showFilters = !showFilters)}
      class="flex items-center gap-2 px-3 py-2 rounded text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
      aria-expanded={showFilters}
    >
      <i class="bi bi-funnel"></i>
      {t('filters.advanced')}
      {#if $activeCount > 0}
        <span class="ml-2 inline-block min-w-5 px-2 py-0.5 rounded-full bg-blue-500 text-white text-xs font-semibold">
          {$activeCount}
        </span>
      {/if}
    </button>

    {#if $activeCount > 0}
      <button
        onclick={handleClearFilters}
        class="text-xs text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
      >
        {t('filters.clearAll')}
      </button>
    {/if}
  </div>

  <!-- Filters Panel -->
  {#if showFilters}
    <div class="space-y-4 mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
      <!-- Search Query -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          {t('filters.search')}
        </label>
        <input
          type="text"
          value={$advancedFilters.filters.query}
          onchange={(e) => {
            $advancedFilters.updateFilter('query', e.target.value);
            handleFilterChange();
          }}
          placeholder={t('common.searchFilesAndFolders')}
          class="w-full rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
        />
      </div>

      <!-- File Type -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          {t('filters.fileType')}
        </label>
        <div class="flex flex-wrap gap-2">
          {#each fileTypes as type}
            <button
              onclick={() => {
                $advancedFilters.updateFilter('fileType', $advancedFilters.filters.fileType === type.id ? null : type.id);
                handleFilterChange();
              }}
              class={`px-3 py-1 rounded text-sm transition-colors ${
                $advancedFilters.filters.fileType === type.id
                  ? 'bg-blue-500 text-white'
                  : 'border border-gray-300 text-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700'
              }`}
            >
              <i class={`bi ${type.icon} mr-1`}></i>
              {type.label}
            </button>
          {/each}
        </div>
      </div>

      <!-- Size Range -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          {t('filters.sizeRange')}
        </label>
        <div class="grid grid-cols-2 gap-2">
          <input
            type="number"
            value={$advancedFilters.filters.sizeRange.min || ''}
            onchange={(e) => {
              $advancedFilters.updateNestedFilter('sizeRange.min', e.target.value ? parseInt(e.target.value) * 1024 * 1024 : null);
              handleFilterChange();
            }}
            placeholder={t('filters.minSize')}
            class="rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          />
          <input
            type="number"
            value={$advancedFilters.filters.sizeRange.max || ''}
            onchange={(e) => {
              $advancedFilters.updateNestedFilter('sizeRange.max', e.target.value ? parseInt(e.target.value) * 1024 * 1024 : null);
              handleFilterChange();
            }}
            placeholder={t('filters.maxSize')}
            class="rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          />
        </div>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">{t('filters.sizeInMB')}</p>
      </div>

      <!-- Date Range -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          {t('filters.dateRange')}
        </label>
        <div class="grid grid-cols-2 gap-2">
          <input
            type="date"
            value={$advancedFilters.filters.dateRange.from ? new Date($advancedFilters.filters.dateRange.from).toISOString().split('T')[0] : ''}
            onchange={(e) => {
              $advancedFilters.updateNestedFilter('dateRange.from', e.target.value ? new Date(e.target.value) : null);
              handleFilterChange();
            }}
            class="rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          />
          <input
            type="date"
            value={$advancedFilters.filters.dateRange.to ? new Date($advancedFilters.filters.dateRange.to).toISOString().split('T')[0] : ''}
            onchange={(e) => {
              $advancedFilters.updateNestedFilter('dateRange.to', e.target.value ? new Date(e.target.value) : null);
              handleFilterChange();
            }}
            class="rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          />
        </div>
      </div>

      <!-- Quick Filters -->
      <div class="space-y-2">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
          {t('filters.quickFilters')}
        </label>
        <div class="space-y-2">
          <label class="flex items-center gap-2 text-sm text-gray-700 dark:text-gray-300">
            <input
              type="checkbox"
              checked={$advancedFilters.filters.isFavorite === true}
              onchange={(e) => {
                $advancedFilters.updateFilter('isFavorite', e.target.checked ? true : null);
                handleFilterChange();
              }}
              class="rounded border-gray-300"
            />
            <i class="bi bi-star"></i>
            {t('filters.favorites')}
          </label>
          <label class="flex items-center gap-2 text-sm text-gray-700 dark:text-gray-300">
            <input
              type="checkbox"
              checked={$advancedFilters.filters.shared === true}
              onchange={(e) => {
                $advancedFilters.updateFilter('shared', e.target.checked ? true : null);
                handleFilterChange();
              }}
              class="rounded border-gray-300"
            />
            <i class="bi bi-share"></i>
            {t('filters.shared')}
          </label>
        </div>
      </div>

      <!-- Presets -->
      <div class="border-t border-gray-200 pt-4 dark:border-gray-700">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          {t('filters.presets')}
        </label>
        <div class="flex gap-2 mb-2">
          <input
            type="text"
            bind:value={presetName}
            placeholder={t('filters.presetName')}
            class="flex-1 rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          />
          <button
            onclick={handleSavePreset}
            disabled={!presetName.trim()}
            class="px-3 py-2 rounded bg-blue-500 text-white text-sm font-medium hover:bg-blue-600 disabled:opacity-50"
          >
            {t('filters.save')}
          </button>
        </div>

        {#if $advancedFilters.presets.length > 0}
          <select
            bind:value={selectedPreset}
            onchange={handleLoadPreset}
            class="w-full rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          >
            <option value={null}>{t('filters.loadPreset')}</option>
            {#each $advancedFilters.presets as preset}
              <option value={preset.id}>{preset.name}</option>
            {/each}
          </select>
        {/if}
      </div>
    </div>
  {/if}
</div>
