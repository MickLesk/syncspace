<script>
  // MobileNavigation.svelte - Bottom navigation bar for mobile
  import { t } from '../../lib/i18n';
  
  let { currentView = 'files', onNavigate } = $props();
  
  const navItems = [
    { id: 'files', icon: 'bi-folder-fill', label: 'Files' },
    { id: 'search', icon: 'bi-search', label: 'Search' },
    { id: 'favorites', icon: 'bi-star-fill', label: 'Favorites' },
    { id: 'settings', icon: 'bi-gear-fill', label: 'Settings' },
    { id: 'profile', icon: 'bi-person-circle', label: 'Profile' },
  ];
</script>

<!-- Mobile bottom nav - hidden on md+ screens -->
<nav class="md:hidden fixed bottom-0 left-0 right-0 bg-white dark:bg-gray-900 border-t border-gray-200 dark:border-gray-800 z-40">
  <div class="flex justify-around">
    {#each navItems as item}
      <button
        on:click={() => onNavigate(item.id)}
        class={`
          flex-1 flex flex-col items-center justify-center py-3 px-2 transition-colors
          ${currentView === item.id 
            ? 'text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900 bg-opacity-30' 
            : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'}
        `}
        title={item.label}
      >
        <i class={`${item.icon} text-xl mb-1`}></i>
        <span class="text-xs font-medium">{item.label}</span>
      </button>
    {/each}
  </div>
</nav>

<!-- Spacer for bottom nav -->
<div class="h-20 md:hidden"></div>
