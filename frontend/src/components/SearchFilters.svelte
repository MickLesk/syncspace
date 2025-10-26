<script>
  let { onFilterChange = () => {} } = $props();
  
  let filters = $state({
    fileType: "all",
    dateRange: "anytime",
    minSize: "",
    maxSize: "",
    tags: []
  });

  const fileTypes = ["all", "images", "videos", "documents", "audio", "archives"];
  const dateRanges = ["anytime", "today", "week", "month", "year"];

  function updateFilters() { onFilterChange(filters); }
</script>

<div class="filter-panel">
  <div class="filter-group">
    <label class="label"><span class="label-text font-semibold">File Type</span></label>
    <select bind:value={filters.fileType} class="select select-bordered select-sm w-full" onchange={updateFilters}>
      {#each fileTypes as type}
        <option value={type}>{type.charAt(0).toUpperCase() + type.slice(1)}</option>
      {/each}
    </select>
  </div>

  <div class="filter-group">
    <label class="label"><span class="label-text font-semibold">Date Modified</span></label>
    <select bind:value={filters.dateRange} class="select select-bordered select-sm w-full" onchange={updateFilters}>
      {#each dateRanges as range}
        <option value={range}>{range.charAt(0).toUpperCase() + range.slice(1)}</option>
      {/each}
    </select>
  </div>

  <div class="filter-group">
    <label class="label"><span class="label-text font-semibold">File Size</span></label>
    <div class="flex gap-2">
      <input type="number" bind:value={filters.minSize} placeholder="Min MB" class="input input-bordered input-sm flex-1" onchange={updateFilters} />
      <input type="number" bind:value={filters.maxSize} placeholder="Max MB" class="input input-bordered input-sm flex-1" onchange={updateFilters} />
    </div>
  </div>

  <button class="btn btn-sm btn-ghost w-full" onclick={() => { filters = { fileType: "all", dateRange: "anytime", minSize: "", maxSize: "", tags: [] }; updateFilters(); }}>
    <i class="bi bi-x-circle"></i> Clear Filters
  </button>
</div>

<style>
  .filter-panel { display: flex; flex-direction: column; gap: 1rem; padding: 1rem; background: hsl(var(--b1)); border: 1px solid hsl(var(--bc) / 0.1); border-radius: var(--rounded-box); }
  .filter-group { display: flex; flex-direction: column; gap: 0.25rem; }
</style>
