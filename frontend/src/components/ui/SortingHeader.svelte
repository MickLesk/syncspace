<script>
  /**
   * SortingHeader Component
   * Reusable table header with click-to-sort functionality
   * 
   * @prop {Array<{key: string, label: string, sortable?: boolean, align?: 'left'|'center'|'right'}>} columns - Column definitions
   * @prop {string} sortBy - Current sort key
   * @prop {'asc'|'desc'} sortOrder - Current sort order
   * @prop {Function} onSort - Callback when sort changes (key, order)
   * @prop {boolean} compact - Compact mode for smaller tables
   */
  
  let {
    columns = [],
    sortBy = $bindable(""),
    sortOrder = $bindable("asc"),
    onSort = () => {},
    compact = false
  } = $props();

  function handleSort(columnKey) {
    if (sortBy === columnKey) {
      // Toggle sort order
      sortOrder = sortOrder === "asc" ? "desc" : "asc";
    } else {
      // New column, default to ascending
      sortBy = columnKey;
      sortOrder = "asc";
    }
    onSort(sortBy, sortOrder);
  }

  function getAlignmentClass(align) {
    switch (align) {
      case "center":
        return "text-center";
      case "right":
        return "text-right";
      default:
        return "text-left";
    }
  }
</script>

<thead class="bg-gray-50 dark:bg-gray-800/50 border-b border-gray-200 dark:border-gray-700">
  <tr>
    {#each columns as column}
      <th
        class="px-4 py-3 {getAlignmentClass(column.align)} {compact ? 'text-xs' : 'text-sm'} font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider"
      >
        {#if column.sortable !== false}
          <button
            class="flex items-center gap-2 hover:text-primary-600 dark:hover:text-primary-400 transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-primary-500 rounded px-1 {column.align === 'center' ? 'justify-center w-full' : column.align === 'right' ? 'justify-end w-full' : ''}"
            onclick={() => handleSort(column.key)}
            aria-label="Sort by {column.label}"
          >
            <span>{column.label}</span>
            {#if sortBy === column.key}
              <i
                class="bi bi-{sortOrder === 'asc' ? 'arrow-up' : 'arrow-down'} text-primary-600 dark:text-primary-400"
                aria-hidden="true"
              ></i>
            {:else}
              <i
                class="bi bi-arrow-down-up text-gray-400 dark:text-gray-600 opacity-0 group-hover:opacity-100 transition-opacity"
                aria-hidden="true"
              ></i>
            {/if}
          </button>
        {:else}
          <span class="block px-1">{column.label}</span>
        {/if}
      </th>
    {/each}
  </tr>
</thead>

<style>
  button:hover i {
    opacity: 0.7;
  }
  
  thead tr:hover button i.bi-arrow-down-up {
    opacity: 0.5;
  }
</style>
