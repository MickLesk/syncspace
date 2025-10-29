<script>


  // Utility: split path into segments
  function getPathSegments(path) {
    if (!path || path === '/') return [];
    return path.replace(/^\/+|\/+$/g, '').split('/');
  }

  // Store: segments for current path
  export let path = '';
  $: segments = getPathSegments(path || $currentPath);

  // Emit navigation event
  function handleNavigate(idx) {
    const newPath = '/' + segments.slice(0, idx + 1).join('/') + '/';
    dispatch('navigate', { path: newPath });
  }

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();
</script>

<nav class="flex items-center gap-2 text-sm" aria-label="Breadcrumb">
  <button
    class="text-blue-600 dark:text-blue-400 hover:underline font-medium"
    on:click={() => dispatch('navigate', { path: '/' })}
    aria-label="Home"
  >
    <i class="bi bi-house-fill"></i>
    Home
  </button>
  {#each segments as segment, i}
    <i class="bi bi-chevron-right text-gray-400" aria-hidden="true"></i>
    <button
      class="text-blue-600 dark:text-blue-400 hover:underline font-medium"
      on:click={() => handleNavigate(i)}
      aria-label={segment}
    >
      {segment}
    </button>
  {/each}
</nav>

<style>
nav {
  user-select: none;
}
button[aria-label] {
  outline: none;
}
</style>
