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
  <div class="filter-bar">
    <div class="filter-bar-content">
      <span class="filter-bar-title">
        <i class="bi bi-funnel" aria-hidden="true"></i>
        {tr("activeFilters")}:
      </span>

      <div class="filter-chips">
        {#each activeFilters as filter}
          <div class="chip">
            <span class="chip-label">{filter.label}</span>
            <button
              class="chip-remove"
              onclick={() => handleRemove(filter.key)}
              title={tr("removeFilter")}
            >
              <i class="bi bi-x" aria-hidden="true"></i>
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

<style>
  .filter-bar {
    background: var(--fallback-b2, oklch(var(--b2)));
    border-radius: 0.5rem;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.1));
  }

  .filter-bar-content {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .filter-bar-title {
    font-size: 0.875rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--fallback-bc, oklch(var(--bc)));
  }

  .filter-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    flex: 1;
  }

  .chip {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: var(--fallback-p, oklch(var(--p)));
    color: var(--fallback-pc, oklch(var(--pc)));
    padding: 0.25rem 0.5rem 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.875rem;
    transition: all 0.2s ease;
  }

  .chip:hover {
    opacity: 0.9;
    transform: translateY(-1px);
  }

  .chip-label {
    white-space: nowrap;
  }

  .chip-remove {
    all: unset;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    border-radius: 50%;
    background: rgba(0, 0, 0, 0.2);
    transition: background 0.2s ease;
  }

  .chip-remove:hover {
    background: rgba(0, 0, 0, 0.3);
  }

  .chip-remove i {
    font-size: 0.75rem;
  }
</style>
