<script>
  // ResponsiveLayout.svelte - Main responsive container
  import { onMount } from 'svelte';
  import { screen } from '../../lib/mobileResponsive';
  
  let { showSidebar = true } = $props();
  let sidebarOpen = $state(false);
  
  // Close sidebar on larger screens
  onMount(() => {
    const unsubscribe = screen.isDesktop.subscribe((isDesktop) => {
      if (isDesktop) {
        sidebarOpen = true;
      }
    });
    return unsubscribe;
  });
</script>

<div class="h-screen flex flex-col bg-white dark:bg-gray-950">
  <!-- Header -->
  <header class="sticky top-0 z-30 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 shadow-sm">
    <div class="flex items-center justify-between px-4 md:px-6 h-16">
      <!-- Mobile menu toggle -->
      <button
        onclick={() => sidebarOpen = !sidebarOpen}
        class="md:hidden p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition"
        title="Toggle menu"
      >
        <i class={`bi bi-list text-xl text-gray-700 dark:text-gray-300`}></i>
      </button>

      <!-- Logo/Title -->
      <div class="flex-1 text-center md:text-left">
        <slot name="header" />
      </div>

      <!-- Right actions -->
      <div class="flex items-center gap-2 md:gap-4">
        <slot name="headerActions" />
      </div>
    </div>
  </header>

  <!-- Main content area -->
  <div class="flex flex-1 overflow-hidden">
    <!-- Sidebar (mobile overlay or desktop fixed) -->
    {#if showSidebar}
      <aside
        class={`
          fixed md:relative inset-0 md:inset-auto z-20
          w-full md:w-64 lg:w-80
          bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800
          transform transition-transform duration-300 ease-in-out md:translate-x-0
          ${sidebarOpen ? 'translate-x-0' : '-translate-x-full'}
          overflow-y-auto
        `}
      >
        <!-- Mobile sidebar header with close button -->
        <div class="md:hidden flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-800">
          <h2 class="font-semibold text-gray-900 dark:text-white">Menu</h2>
          <button
            onclick={() => sidebarOpen = false}
            class="p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition"
          >
            <i class="bi bi-x text-xl" aria-hidden="true"></i>
          </button>
        </div>

        <slot name="sidebar" />
      </aside>

      <!-- Mobile backdrop -->
      {#if sidebarOpen}
        <div
          onclick={() => sidebarOpen = false}
          class="fixed inset-0 z-10 bg-black bg-opacity-50 md:hidden"
        ></div>
      {/if}
    {/if}

    <!-- Main content -->
    <main class="flex-1 overflow-y-auto">
      <slot name="content" />
    </main>
  </div>

  <!-- Bottom mobile navigation (if provided) -->
  <nav class="md:hidden sticky bottom-0 z-30 bg-white dark:bg-gray-900 border-t border-gray-200 dark:border-gray-800">
    <slot name="bottomNav" />
  </nav>
</div>

<style>
  /* iOS safe area support */
  :global(html) {
    --safe-area-inset-top: max(env(safe-area-inset-top), 0px);
    --safe-area-inset-bottom: max(env(safe-area-inset-bottom), 0px);
    --safe-area-inset-left: max(env(safe-area-inset-left), 0px);
    --safe-area-inset-right: max(env(safe-area-inset-right), 0px);
  }

  :global(body) {
    padding-top: var(--safe-area-inset-top);
    padding-bottom: var(--safe-area-inset-bottom);
    padding-left: var(--safe-area-inset-left);
    padding-right: var(--safe-area-inset-right);
  }
</style>
