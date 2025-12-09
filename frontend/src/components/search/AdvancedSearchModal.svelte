<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { modals, modalEvents } from "../../stores/modals.js";
  import Modal from "../ui/Modal.svelte";
  import { userPreferences } from "../../stores/preferences.js";
  import { savedSearches } from "../../stores/savedSearches.js";
  import { success, error as errorToast } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Props
  let { visible = false } = $props();

  // Search State
  let searchQuery = $state("");
  let activeFilters = $state({
    fileType: "all",
    sizeMin: "",
    sizeMax: "",
    dateFrom: "",
    dateTo: "",
    modifiedBy: "",
  });

  // Sort State
  let sortBy = $state("name");
  let sortOrder = $state("asc");

  // Save Search Dialog
  let showSaveDialog = $state(false);
  let saveSearchName = $state("");

  // Loading State
  let isSearching = $state(false);

  // Recent Searches from backend
  let recentSearches = $derived($userPreferences.recent_searches || []);

  // File type options
  const fileTypeOptions = [
    { value: "all", labelKey: "allFileTypes" },
    { value: "folder", labelKey: "folders" },
    { value: "image", labelKey: "images" },
    { value: "video", labelKey: "videos" },
    { value: "audio", labelKey: "audio" },
    { value: "document", labelKey: "documents" },
    { value: "archive", labelKey: "archives" },
    { value: "code", labelKey: "code" },
    { value: "pdf", labelKey: "pdf" },
    { value: "text", labelKey: "text" },
  ];

  // Sort options
  const sortOptions = [
    { value: "name", labelKey: "name" },
    { value: "date", labelKey: "dateModified" },
    { value: "size", labelKey: "size" },
    { value: "type", labelKey: "type" },
  ];

  // Size presets
  const sizePresets = [
    { label: "< 1 MB", min: "", max: "1" },
    { label: "1-10 MB", min: "1", max: "10" },
    { label: "10-100 MB", min: "10", max: "100" },
    { label: "> 100 MB", min: "100", max: "" },
  ];

  // Active filter count
  let activeFilterCount = $derived(
    Object.entries(activeFilters).filter(([key, value]) => {
      if (key === "fileType") return value !== "all";
      return value !== "";
    }).length
  );

  // Save to recent searches
  async function saveRecentSearch(query) {
    if (!query.trim()) return;
    let searches = [...recentSearches];
    searches = searches.filter((s) =>
      typeof s === "string" ? s !== query : s.query !== query
    );
    searches = [query, ...searches].slice(0, 10);
    await userPreferences.updatePreferences({ recent_searches: searches });
  }

  // Execute search - uses modalEvents to communicate with FilesView
  function executeSearch() {
    if (!searchQuery.trim()) {
      errorToast(tr("enterSearchQuery") || "Bitte Suchbegriff eingeben");
      return;
    }

    isSearching = true;
    saveRecentSearch(searchQuery);

    // Emit search event via modalEvents - FilesView listens to this
    modalEvents.emit("search", {
      detail: {
        query: searchQuery,
        filters: { ...activeFilters },
        sortBy,
        sortOrder,
      },
    });

    isSearching = false;
  }

  // Use recent search
  function useRecentSearch(search) {
    searchQuery = typeof search === "string" ? search : search.query;
  }

  // Apply saved search
  function applySavedSearch(search) {
    searchQuery = search.query;
    activeFilters = { ...search.filters };
    sortBy = search.sortBy || "name";
    sortOrder = search.sortOrder || "asc";
  }

  // Set size preset
  function setSizePreset(preset) {
    activeFilters.sizeMin = preset.min;
    activeFilters.sizeMax = preset.max;
    activeFilters = { ...activeFilters };
  }

  // Check if size preset is active
  function isSizePresetActive(preset) {
    return (
      activeFilters.sizeMin === preset.min &&
      activeFilters.sizeMax === preset.max
    );
  }

  // Clear size filter
  function clearSizeFilter() {
    activeFilters.sizeMin = "";
    activeFilters.sizeMax = "";
    activeFilters = { ...activeFilters };
  }

  // Clear all filters
  function clearAllFilters() {
    activeFilters = {
      fileType: "all",
      sizeMin: "",
      sizeMax: "",
      dateFrom: "",
      dateTo: "",
      modifiedBy: "",
    };
    sortBy = "name";
    sortOrder = "asc";
  }

  // Save current search
  async function saveCurrentSearch() {
    if (!saveSearchName.trim()) {
      errorToast(tr("enterSearchName") || "Name eingeben");
      return;
    }
    try {
      await savedSearches.saveSearch({
        name: saveSearchName,
        query: searchQuery,
        filters: { ...activeFilters },
        sortBy,
        sortOrder,
      });
      success(tr("searchSaved") || "Suche gespeichert");
      showSaveDialog = false;
      saveSearchName = "";
    } catch (err) {
      errorToast(tr("saveFailed") || "Speichern fehlgeschlagen");
    }
  }

  // Close modal
  function close() {
    modals.close("advancedSearch");
  }

  // Handle form submit (Enter key)
  function handleFormSubmit(e) {
    e.preventDefault();
    executeSearch();
  }

  // Keyboard shortcuts
  function handleKeydown(e) {
    if (!visible) return;
    if (e.key === "Escape") {
      close();
    } else if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      executeSearch();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if visible}
  <Modal
    {visible}
    title={tr("advancedSearch") || "Erweiterte Suche"}
    icon="funnel"
    size="lg"
    variant="primary"
    onclose={close}
  >
    <form onsubmit={handleFormSubmit} class="space-y-5">
      <!-- Search Input -->
      <div class="relative">
        <div
          class="flex rounded-xl overflow-hidden border-2 border-gray-200 dark:border-gray-700 focus-within:border-green-500 focus-within:ring-2 focus-within:ring-green-500/20 transition-all"
        >
          <span
            class="px-4 bg-gray-50 dark:bg-gray-800 flex items-center text-gray-500"
          >
            <i class="bi bi-search text-lg" aria-hidden="true"></i>
          </span>
          <input
            type="text"
            bind:value={searchQuery}
            placeholder={tr("searchPlaceholder") || "Dateien suchen..."}
            class="flex-1 px-4 py-3 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 outline-none text-base"
          />
          <button
            type="button"
            class="px-6 bg-gradient-to-r from-green-600 to-emerald-600 text-white hover:from-green-700 hover:to-emerald-700 transition-all flex items-center gap-2 font-medium disabled:opacity-50"
            onclick={executeSearch}
            disabled={!searchQuery.trim() || isSearching}
          >
            {#if isSearching}
              <i class="bi bi-hourglass-split animate-spin" aria-hidden="true"
              ></i>
            {:else}
              <i class="bi bi-search" aria-hidden="true"></i>
            {/if}
            {tr("search") || "Suchen"}
          </button>
        </div>
      </div>

      <!-- Active Filter Pills -->
      {#if activeFilterCount > 0}
        <div class="flex flex-wrap gap-2 items-center">
          <span class="text-xs text-gray-500 dark:text-gray-400"
            >{tr("activeFilters") || "Aktive Filter"}:</span
          >

          {#if activeFilters.fileType !== "all"}
            <span
              class="inline-flex items-center gap-1 px-2 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 rounded-lg text-xs"
            >
              <i class="bi bi-file-earmark" aria-hidden="true"></i>
              {tr(activeFilters.fileType) || activeFilters.fileType}
              <button
                type="button"
                class="hover:text-green-900 dark:hover:text-green-100"
                onclick={() => {
                  activeFilters.fileType = "all";
                  activeFilters = { ...activeFilters };
                }}
                aria-label="Remove file type filter"
              >
                <i class="bi bi-x" aria-hidden="true"></i>
              </button>
            </span>
          {/if}

          {#if activeFilters.sizeMin || activeFilters.sizeMax}
            <span
              class="inline-flex items-center gap-1 px-2 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded-lg text-xs"
            >
              <i class="bi bi-hdd" aria-hidden="true"></i>
              {activeFilters.sizeMin || "0"} - {activeFilters.sizeMax || "∞"} MB
              <button
                type="button"
                class="hover:text-blue-900 dark:hover:text-blue-100"
                onclick={clearSizeFilter}
                aria-label="Remove size filter"
              >
                <i class="bi bi-x" aria-hidden="true"></i>
              </button>
            </span>
          {/if}

          {#if activeFilters.dateFrom || activeFilters.dateTo}
            <span
              class="inline-flex items-center gap-1 px-2 py-1 bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300 rounded-lg text-xs"
            >
              <i class="bi bi-calendar" aria-hidden="true"></i>
              {activeFilters.dateFrom || "..."} - {activeFilters.dateTo ||
                "..."}
              <button
                type="button"
                class="hover:text-purple-900 dark:hover:text-purple-100"
                onclick={() => {
                  activeFilters.dateFrom = "";
                  activeFilters.dateTo = "";
                  activeFilters = { ...activeFilters };
                }}
                aria-label="Remove date filter"
              >
                <i class="bi bi-x" aria-hidden="true"></i>
              </button>
            </span>
          {/if}

          <button
            type="button"
            class="text-xs text-red-600 dark:text-red-400 hover:underline"
            onclick={clearAllFilters}
          >
            {tr("clearAll") || "Alle löschen"}
          </button>
        </div>
      {/if}

      <!-- Filters Panel -->
      <div class="border-2 border-gray-200 dark:border-gray-700 rounded-xl p-4">
        <details open>
          <summary
            class="text-sm font-semibold text-gray-700 dark:text-gray-300 cursor-pointer select-none flex items-center gap-2"
          >
            <i
              class="bi bi-funnel text-green-600 dark:text-green-400"
              aria-hidden="true"
            ></i>
            {tr("filters") || "Filter"}
            {#if activeFilterCount > 0}
              <span
                class="px-2 py-0.5 bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300 rounded-full text-xs"
              >
                {activeFilterCount}
              </span>
            {/if}
          </summary>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
            <!-- File Type -->
            <div class="space-y-2">
              <label
                for="filter-filetype"
                class="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-2"
              >
                <i
                  class="bi bi-file-earmark text-green-600 dark:text-green-400"
                  aria-hidden="true"
                ></i>
                {tr("fileType") || "Dateityp"}
              </label>
              <select
                id="filter-filetype"
                bind:value={activeFilters.fileType}
                class="w-full px-3 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl text-gray-900 dark:text-gray-100 focus:border-green-500 focus:ring-2 focus:ring-green-500/20 outline-none"
              >
                {#each fileTypeOptions as opt}
                  <option value={opt.value}
                    >{tr(opt.labelKey) || opt.labelKey}</option
                  >
                {/each}
              </select>
            </div>

            <!-- Date Range -->
            <div class="space-y-2">
              <div
                class="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-2"
              >
                <i
                  class="bi bi-calendar text-green-600 dark:text-green-400"
                  aria-hidden="true"
                ></i>
                {tr("dateRange") || "Zeitraum"}
              </div>
              <div class="flex gap-2">
                <input
                  type="date"
                  bind:value={activeFilters.dateFrom}
                  class="flex-1 px-3 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-green-500 outline-none"
                  aria-label="From date"
                />
                <span class="text-gray-400 self-center">–</span>
                <input
                  type="date"
                  bind:value={activeFilters.dateTo}
                  class="flex-1 px-3 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-green-500 outline-none"
                  aria-label="To date"
                />
              </div>
            </div>

            <!-- File Size -->
            <div class="space-y-2 md:col-span-2">
              <div
                class="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-2"
              >
                <i
                  class="bi bi-hdd text-green-600 dark:text-green-400"
                  aria-hidden="true"
                ></i>
                {tr("fileSize") || "Dateigröße"}
              </div>

              <!-- Quick Presets -->
              <div class="flex flex-wrap gap-2">
                {#each sizePresets as preset}
                  <button
                    type="button"
                    class="px-3 py-1.5 text-xs rounded-lg transition-all {isSizePresetActive(
                      preset
                    )
                      ? 'bg-green-600 text-white'
                      : 'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-green-100 dark:hover:bg-green-900/50'}"
                    onclick={() => setSizePreset(preset)}
                  >
                    {preset.label}
                  </button>
                {/each}
                {#if activeFilters.sizeMin || activeFilters.sizeMax}
                  <button
                    type="button"
                    class="px-3 py-1.5 text-xs rounded-lg bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 hover:bg-red-200"
                    onclick={clearSizeFilter}
                  >
                    <i class="bi bi-x" aria-hidden="true"></i>
                    {tr("clear") || "Löschen"}
                  </button>
                {/if}
              </div>

              <!-- Custom Range -->
              <div class="flex gap-2 items-center">
                <input
                  type="number"
                  bind:value={activeFilters.sizeMin}
                  placeholder={tr("min") || "Min"}
                  min="0"
                  class="flex-1 px-3 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-green-500 outline-none"
                  aria-label="Minimum size in MB"
                />
                <span class="text-gray-400">–</span>
                <input
                  type="number"
                  bind:value={activeFilters.sizeMax}
                  placeholder={tr("max") || "Max"}
                  min="0"
                  class="flex-1 px-3 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-green-500 outline-none"
                  aria-label="Maximum size in MB"
                />
                <span class="text-xs text-gray-500">MB</span>
              </div>
            </div>
          </div>
        </details>
      </div>

      <!-- Sort Options -->
      <div class="flex gap-4">
        <div class="flex-1 space-y-2">
          <label
            for="sort-by"
            class="text-xs font-medium text-gray-600 dark:text-gray-400 flex items-center gap-1"
          >
            <i
              class="bi bi-sort-down text-green-600 dark:text-green-400"
              aria-hidden="true"
            ></i>
            {tr("sortBy") || "Sortieren nach"}
          </label>
          <select
            id="sort-by"
            bind:value={sortBy}
            class="w-full px-3 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-green-500 outline-none"
          >
            {#each sortOptions as opt}
              <option value={opt.value}
                >{tr(opt.labelKey) || opt.labelKey}</option
              >
            {/each}
          </select>
        </div>

        <div class="flex-1 space-y-2">
          <div
            class="text-xs font-medium text-gray-600 dark:text-gray-400 flex items-center gap-1"
          >
            <i
              class="bi bi-arrow-down-up text-green-600 dark:text-green-400"
              aria-hidden="true"
            ></i>
            {tr("order") || "Reihenfolge"}
          </div>
          <div
            class="flex rounded-lg overflow-hidden border-2 border-gray-200 dark:border-gray-700"
            role="group"
          >
            <button
              type="button"
              class="flex-1 px-3 py-2 text-sm transition-all {sortOrder ===
              'asc'
                ? 'bg-green-600 text-white'
                : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              onclick={() => (sortOrder = "asc")}
              aria-label="Ascending"
              title={tr("ascending") || "Aufsteigend"}
            >
              <i class="bi bi-sort-alpha-down" aria-hidden="true"></i>
            </button>
            <button
              type="button"
              class="flex-1 px-3 py-2 text-sm transition-all border-l-2 border-gray-200 dark:border-gray-700 {sortOrder ===
              'desc'
                ? 'bg-green-600 text-white'
                : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
              onclick={() => (sortOrder = "desc")}
              aria-label="Descending"
              title={tr("descending") || "Absteigend"}
            >
              <i class="bi bi-sort-alpha-down-alt" aria-hidden="true"></i>
            </button>
          </div>
        </div>
      </div>

      <!-- Recent Searches -->
      {#if recentSearches.length > 0}
        <div class="space-y-2">
          <div
            class="text-xs font-medium text-gray-600 dark:text-gray-400 flex items-center gap-1"
          >
            <i
              class="bi bi-clock-history text-green-600 dark:text-green-400"
              aria-hidden="true"
            ></i>
            {tr("recentSearches") || "Letzte Suchen"}
          </div>
          <div class="flex flex-wrap gap-2">
            {#each recentSearches.slice(0, 6) as search}
              <button
                type="button"
                class="px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-green-100 dark:hover:bg-green-900/50 hover:text-green-700 dark:hover:text-green-300 transition-all text-xs flex items-center gap-1"
                onclick={() => useRecentSearch(search)}
              >
                <i class="bi bi-clock-history opacity-50" aria-hidden="true"
                ></i>
                {typeof search === "string"
                  ? search
                  : search.query || "Unknown"}
              </button>
            {/each}
          </div>
        </div>
      {/if}
    </form>

    <!-- Footer Actions -->
    <div
      slot="actions"
      class="flex justify-between items-center w-full border-t-2 border-gray-200 dark:border-gray-700 pt-4"
    >
      <div class="flex gap-2">
        <button
          type="button"
          class="px-3 py-2 border-2 border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-all text-xs flex items-center gap-1 disabled:opacity-50"
          onclick={clearAllFilters}
          disabled={activeFilterCount === 0 &&
            sortBy === "name" &&
            sortOrder === "asc"}
        >
          <i class="bi bi-x-circle" aria-hidden="true"></i>
          {tr("reset") || "Zurücksetzen"}
        </button>

        <button
          type="button"
          class="px-3 py-2 border-2 border-green-200 dark:border-green-800 text-green-700 dark:text-green-300 rounded-lg hover:bg-green-50 dark:hover:bg-green-900/30 transition-all text-xs flex items-center gap-1 disabled:opacity-50"
          onclick={() => {
            if (!searchQuery.trim()) {
              errorToast(tr("enterSearchFirst") || "Erst Suchbegriff eingeben");
              return;
            }
            saveSearchName = searchQuery.substring(0, 30);
            showSaveDialog = true;
          }}
          disabled={!searchQuery.trim()}
        >
          <i class="bi bi-bookmark-plus" aria-hidden="true"></i>
          {tr("saveSearch") || "Speichern"}
        </button>
      </div>

      <button
        type="button"
        class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-all text-sm"
        onclick={close}
      >
        {tr("close") || "Schließen"}
      </button>
    </div>
  </Modal>
{/if}

<!-- Save Search Dialog -->
{#if showSaveDialog}
  <div
    class="fixed inset-0 z-[60] bg-black/50 flex items-center justify-center"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-2xl max-w-md w-full mx-4 p-6"
    >
      <h3 class="text-lg font-bold text-gray-900 dark:text-gray-100 mb-4">
        {tr("saveSearch") || "Suche speichern"}
      </h3>

      <div class="space-y-4">
        <div>
          <label
            for="save-search-name"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("searchName") || "Name der Suche"}
          </label>
          <input
            id="save-search-name"
            type="text"
            bind:value={saveSearchName}
            placeholder={tr("enterSearchName") || "Name eingeben..."}
            class="w-full px-4 py-2 border-2 border-gray-200 dark:border-gray-700 rounded-xl bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:border-green-500 outline-none"
            onkeydown={(e) => e.key === "Enter" && saveCurrentSearch()}
          />
        </div>

        <div
          class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 text-sm space-y-1"
        >
          <div class="font-medium text-gray-700 dark:text-gray-300">
            {tr("searchDetails") || "Details"}:
          </div>
          <div class="text-gray-600 dark:text-gray-400">
            {tr("query") || "Suche"}: "{searchQuery}"
          </div>
          {#if activeFilterCount > 0}
            <div class="text-gray-600 dark:text-gray-400">
              {tr("filters") || "Filter"}: {activeFilterCount}
              {tr("active") || "aktiv"}
            </div>
          {/if}
        </div>
      </div>

      <div class="flex justify-end gap-3 mt-6">
        <button
          type="button"
          class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-all"
          onclick={() => (showSaveDialog = false)}
        >
          {tr("cancel") || "Abbrechen"}
        </button>
        <button
          type="button"
          class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-all flex items-center gap-2"
          onclick={saveCurrentSearch}
        >
          <i class="bi bi-bookmark-check" aria-hidden="true"></i>
          {tr("save") || "Speichern"}
        </button>
      </div>
    </div>
  </div>
{/if}
