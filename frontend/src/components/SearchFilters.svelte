<script>
  let { onFilterChange = () => {} } = $props();

  let filters = $state({
    fileType: "all",
    dateRange: "anytime",
    minSize: "",
    maxSize: "",
    tags: [],
  });

  const fileTypes = [
    "all",
    "images",
    "videos",
    "documents",
    "audio",
    "archives",
  ];
  const dateRanges = ["anytime", "today", "week", "month", "year"];

  function updateFilters() {
    onFilterChange(filters);
  }
</script>

<div class="filter-panel">
  <div class="filter-group">
    <label class="block mb-1"
      ><span class="text-sm font-semibold text-gray-700 dark:text-gray-200"
        >File Type</span
      ></label
    >
    <select
      bind:value={filters.fileType}
      class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
      onchange={updateFilters}
    >
      {#each fileTypes as type}
        <option value={type}
          >{type.charAt(0).toUpperCase() + type.slice(1)}</option
        >
      {/each}
    </select>
  </div>

  <div class="filter-group">
    <label class="block mb-1"
      ><span class="text-sm font-semibold text-gray-700 dark:text-gray-200"
        >Date Modified</span
      ></label
    >
    <select
      bind:value={filters.dateRange}
      class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
      onchange={updateFilters}
    >
      {#each dateRanges as range}
        <option value={range}
          >{range.charAt(0).toUpperCase() + range.slice(1)}</option
        >
      {/each}
    </select>
  </div>

  <div class="filter-group">
    <label class="block mb-1"
      ><span class="text-sm font-semibold text-gray-700 dark:text-gray-200"
        >File Size</span
      ></label
    >
    <div class="flex gap-2">
      <input
        type="number"
        bind:value={filters.minSize}
        placeholder="Min MB"
        class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 flex-1"
        onchange={updateFilters}
      />
      <input
        type="number"
        bind:value={filters.maxSize}
        placeholder="Max MB"
        class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 flex-1"
        onchange={updateFilters}
      />
    </div>
  </div>

  <button
    class="px-3 py-1.5 text-sm hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded-lg transition-colors w-full flex items-center justify-center gap-2"
    onclick={() => {
      filters = {
        fileType: "all",
        dateRange: "anytime",
        minSize: "",
        maxSize: "",
        tags: [],
      };
      updateFilters();
    }}
  >
    <i class="bi bi-x-circle"></i> Clear Filters
  </button>
</div>

<style>
  .filter-panel {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
  }
  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
</style>
