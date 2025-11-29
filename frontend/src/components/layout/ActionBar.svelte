<script>
  import { createEventDispatcher } from "svelte";
  import { colors, transitions } from "../../lib/DesignSystem.js";

  const dispatch = createEventDispatcher();

  let {
    sticky = true,
    glass = true,
    searchEnabled = true,
    searchPlaceholder = "Search",
    searchValue = "",
    filterChips = [],
    viewModes = [],
    bulkActions = [],
    class: className = "",
  } = $props();

  let search = $state(searchValue);
  $effect(() => {
    search = searchValue;
  });

  function handleSearch(event) {
    search = event.currentTarget.value;
    dispatch("search", search);
  }

  function handleFilterClick(chip) {
    dispatch("filter", { id: chip.id, chip });
  }

  function handleViewModeClick(mode) {
    dispatch("view", { id: mode.id, mode });
  }

  function handleBulkActionClick(action) {
    dispatch("bulk", { id: action.id, action });
  }

  const barClasses = $derived(
    () =>
      `w-full rounded-3xl border ${glass ? "border-white/40 bg-white/80 dark:bg-slate-900/70 backdrop-blur-xl shadow-lg shadow-slate-900/10" : "border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900"} ${sticky ? "sticky top-20 z-20" : ""} ${className}`
  );
</script>

<section
  class="{barClasses} px-4 sm:px-6 py-4 transition-all duration-200"
  style={`border-color:${colors.glass.border};transition-timing-function:${transitions.easing};`}
>
  <div
    class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between"
  >
    <div class="flex flex-wrap items-center gap-3">
      {#if searchEnabled}
        <label
          class="flex items-center gap-2 px-3 py-2 rounded-2xl bg-white dark:bg-slate-800/80 border border-slate-200/70 dark:border-slate-700/70 shadow-inner min-w-[220px] focus-within:ring-2 focus-within:ring-indigo-400"
        >
          <i class="bi bi-search text-slate-400"></i>
          <input
            type="search"
            class="bg-transparent focus:outline-none text-sm flex-1"
            placeholder={searchPlaceholder}
            value={search}
            on:input={handleSearch}
          />
        </label>
      {/if}

      {#if filterChips?.length}
        <div class="flex flex-wrap gap-2" aria-label="Filters">
          {#each filterChips as chip}
            <button
              type="button"
              class={`px-3 py-1.5 rounded-full text-sm border transition-all duration-200 ${
                chip.active
                  ? "bg-indigo-500 text-white border-transparent shadow"
                  : "border-slate-200 dark:border-slate-700 text-slate-600 dark:text-slate-300 hover:bg-slate-100/60 dark:hover:bg-slate-800/60"
              }`}
              on:click={() => handleFilterClick(chip)}
            >
              {#if chip.icon}
                <i class={`bi bi-${chip.icon} mr-1.5`}></i>
              {/if}
              {chip.label}
            </button>
          {/each}
        </div>
      {/if}

      <slot />
    </div>

    <div class="flex flex-wrap items-center gap-3 justify-end">
      {#if viewModes?.length}
        <div
          class="inline-flex rounded-2xl border border-slate-200 dark:border-slate-700 bg-white/80 dark:bg-slate-800/60 shadow-sm"
          role="group"
          aria-label="View mode"
        >
          {#each viewModes as mode}
            <button
              type="button"
              class={`px-4 py-2 text-sm font-medium flex items-center gap-2 transition-colors duration-200 ${
                mode.active
                  ? "bg-gradient-to-tr from-indigo-500 to-purple-500 text-white shadow"
                  : "text-slate-500 dark:text-slate-300 hover:text-indigo-500"
              }`}
              on:click={() => handleViewModeClick(mode)}
            >
              {#if mode.icon}
                <i class={`bi bi-${mode.icon}`}></i>
              {/if}
              <span class="hidden sm:inline">{mode.label}</span>
            </button>
          {/each}
        </div>
      {/if}

      {#if bulkActions?.length}
        <div class="flex flex-wrap gap-2">
          {#each bulkActions as action}
            <button
              type="button"
              class={`px-4 py-2 rounded-2xl text-sm font-medium transition-colors duration-200 border ${
                action.primary
                  ? "bg-gradient-to-r from-indigo-500 to-purple-500 text-white border-transparent shadow"
                  : "border-slate-200 dark:border-slate-700 text-slate-600 dark:text-slate-200 hover:border-indigo-400"
              }`}
              on:click={() => handleBulkActionClick(action)}
            >
              {#if action.icon}
                <i class={`bi bi-${action.icon} mr-1.5`}></i>
              {/if}
              {action.label}
            </button>
          {/each}
        </div>
      {/if}

      <slot name="right" />
    </div>
  </div>
</section>
