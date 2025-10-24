<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";

  const dispatch = createEventDispatcher();

  export let filters = {};

  // Get filter labels
  function getFilterLabel(key, value) {
    switch (key) {
      case "fileType":
        if (value === "all") return null;
        return `${t($currentLang, "fileType")}: ${t($currentLang, value)}`;
      case "sizeMin":
        return `${t($currentLang, "minSize")}: ${value} MB`;
      case "sizeMax":
        return `${t($currentLang, "maxSize")}: ${value} MB`;
      case "dateFrom":
        return `${t($currentLang, "from")}: ${new Date(value).toLocaleDateString()}`;
      case "dateTo":
        return `${t($currentLang, "to")}: ${new Date(value).toLocaleDateString()}`;
      case "modifiedBy":
        return `${t($currentLang, "modifiedBy")}: ${value}`;
      default:
        return null;
    }
  }

  // Get active filters as array
  $: activeFilters = Object.entries(filters)
    .map(([key, value]) => ({ key, value, label: getFilterLabel(key, value) }))
    .filter((f) => f.label !== null);

  function handleRemove(key) {
    dispatch("removeFilter", { key });
  }

  function handleClearAll() {
    dispatch("clearAll");
  }
</script>

{#if activeFilters.length > 0}
  <div class="filter-bar">
    <div class="filter-bar-content">
      <span class="filter-bar-title">
        <i class="bi bi-funnel"></i>
        {t($currentLang, "activeFilters")}:
      </span>

      <div class="filter-chips">
        {#each activeFilters as filter}
          <div class="chip">
            <span class="chip-label">{filter.label}</span>
            <button
              class="chip-remove"
              on:click={() => handleRemove(filter.key)}
              title={t($currentLang, "removeFilter")}
            >
              <i class="bi bi-x"></i>
            </button>
          </div>
        {/each}
      </div>

      <button class="btn btn-ghost btn-xs gap-1" on:click={handleClearAll}>
        <i class="bi bi-x-circle"></i>
        {t($currentLang, "clearAll")}
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
