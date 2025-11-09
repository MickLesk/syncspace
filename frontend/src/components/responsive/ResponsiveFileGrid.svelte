<script>
  // ResponsiveFileGrid.svelte - Mobile-optimized file grid
  import { onMount } from 'svelte';
  
  let { files = [], viewMode = 'grid' } = $props();
  let isMobile = $state(window.innerWidth < 640);
  let isTablet = $state(window.innerWidth >= 640 && window.innerWidth < 1024);
  
  // Handle resize
  onMount(() => {
    const handleResize = () => {
      isMobile = window.innerWidth < 640;
      isTablet = window.innerWidth >= 640 && window.innerWidth < 1024;
    };
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  });
  
  // Determine columns based on screen size
  let gridCols = $derived.by(() => {
    if (isMobile) return 'grid-cols-1 sm:grid-cols-2';
    if (isTablet) return 'grid-cols-3 lg:grid-cols-4';
    return 'grid-cols-4 xl:grid-cols-6';
  });
</script>

<!-- Responsive file grid -->
{#if viewMode === 'grid'}
  <div class={`grid gap-3 md:gap-4 ${gridCols} auto-rows-max`}>
    {#each files as file (file.id)}
      <div class="min-w-0">
        <slot name="fileCard" {file} />
      </div>
    {/each}
  </div>
{:else}
  <!-- List view - full width on mobile, constrained on desktop -->
  <div class="space-y-2 md:space-y-1">
    {#each files as file (file.id)}
      <div class="px-2 md:px-0">
        <slot name="fileItem" {file} />
      </div>
    {/each}
  </div>
{/if}
