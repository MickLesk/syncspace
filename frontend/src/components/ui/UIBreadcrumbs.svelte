<!-- UIBreadcrumbs.svelte - Navigation breadcrumbs component -->
<script>
  let {
    items = [],
    onNavigate = (item) => {},
    class: className = "",
  } = $props();
</script>

<nav
  aria-label="Breadcrumb"
  class="flex items-center gap-2 text-sm {className}"
>
  {#each items as item, i}
    {#if i > 0}
      <i class="bi bi-chevron-right text-white/40 text-xs"></i>
    {/if}

    {#if i === items.length - 1}
      <span class="text-white font-medium">{item.label}</span>
    {:else}
      <button
        onclick={() => onNavigate(item)}
        class="text-white/60 hover:text-white transition-colors hover:underline"
      >
        {item.label}
      </button>
    {/if}
  {/each}
</nav>
