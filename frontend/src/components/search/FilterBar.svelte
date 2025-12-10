<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const dispatch = createEventDispatcher();

  let { filters = {}, onRemoveFilter, onClearAll } = $props();

  // Get filter labels - only return label if value is valid
  function getFilterLabel(key, value) {
    // Skip empty/null/undefined values
    if (value === null || value === undefined || value === "") return null;

    switch (key) {
      case "fileType":
        if (value === "all") return null;
        return `${tr("fileType")}: ${tr(value)}`;
      case "sizeMin":
        if (!value || isNaN(Number(value))) return null;
        return `${tr("minSize")}: ${value} MB`;
      case "sizeMax":
        if (!value || isNaN(Number(value))) return null;
        return `${tr("maxSize")}: ${value} MB`;
      case "dateFrom":
        if (!value) return null;
        const fromDate = new Date(value);
        if (isNaN(fromDate.getTime())) return null;
        return `${tr("from")}: ${fromDate.toLocaleDateString()}`;
      case "dateTo":
        if (!value) return null;
        const toDate = new Date(value);
        if (isNaN(toDate.getTime())) return null;
        return `${tr("to")}: ${toDate.toLocaleDateString()}`;
      case "modifiedBy":
        if (!value.trim()) return null;
        return `${tr("modifiedBy")}: ${value}`;
      default:
        return null;
    }
  }

  // Get active filters as array
  const activeFilters = $derived(
    Object.entries(filters)
      .map(([key, value]) => ({
        key,
        value,
        label: getFilterLabel(key, value),
      }))
      .filter((f) => f.label !== null)
  );

  function handleRemove(key) {
    // Support both callback props and dispatch
    if (onRemoveFilter) {
      onRemoveFilter(key);
    } else {
      dispatch("removeFilter", { key });
    }
  }

  function handleClearAll() {
    // Support both callback props and dispatch
    if (onClearAll) {
      onClearAll();
    } else {
      dispatch("clearAll");
    }
  }
</script>

{#if activeFilters.length > 0}
  <div class="bg-base-200 rounded-lg px-4 py-3 mb-4 border border-base-content/10">
    <div class="flex items-center gap-3 flex-wrap">
      <span class="text-sm font-semibold flex items-center gap-2 text-base-content">
        <i class="bi bi-funnel" aria-hidden="true"></i>
        {tr("activeFilters")}:
      </span>

      <div class="flex flex-wrap gap-2 flex-1">
        {#each activeFilters as filter}
          <div class="inline-flex items-center gap-2 bg-primary text-primary-content pl-3 pr-2 py-1 rounded-full text-sm transition-all hover:opacity-90 hover:-translate-y-px">
            <span class="whitespace-nowrap">{filter.label}</span>
            <button
              class="flex items-center justify-center w-5 h-5 rounded-full bg-black/20 hover:bg-black/30 transition-colors cursor-pointer"
              onclick={() => handleRemove(filter.key)}
              title={tr("removeFilter")}
              aria-label={tr("removeFilter")}
            >
              <i class="bi bi-x text-xs" aria-hidden="true"></i>
            </button>
          </div>
        {/each}
      </div>

      <button
        class="px-2 py-1 text-xs hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded-lg transition-colors flex items-center gap-1"
        onclick={handleClearAll}
      >
        <i class="bi bi-x-circle" aria-hidden="true"></i>
        {tr("clearAll")}
      </button>
    </div>
  </div>
{/if}

