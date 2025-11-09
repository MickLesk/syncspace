<script>
  // ResponsiveSidebar.svelte - Mobile-aware sidebar component
  import { currentTheme } from '../../stores/ui';
  import { t } from '../../lib/i18n';
  
  let { isOpen = true } = $props();
  let isMobile = $state(window.innerWidth < 768);
  let isTablet = $state(window.innerWidth >= 768 && window.innerWidth < 1024);
  
  // Listen to resize events
  if (typeof window !== 'undefined') {
    window.addEventListener('resize', () => {
      isMobile = window.innerWidth < 768;
      isTablet = window.innerWidth >= 768 && window.innerWidth < 1024;
    });
  }
</script>

<!-- Mobile: Full-width sidebar overlay, Desktop: Fixed sidebar -->
<aside
  class={`
    ${isMobile ? 'fixed inset-0 z-40 w-full md:w-80 md:fixed md:inset-y-0 md:left-0 md:block' : 'hidden md:block w-80'}
    ${isOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'}
    bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 
    transition-transform duration-300 ease-in-out
  `}
>
  <slot></slot>
</aside>

<!-- Mobile: Backdrop overlay -->
{#if isMobile && isOpen}
  <div
    class="fixed inset-0 z-30 bg-black bg-opacity-50 md:hidden transition-opacity"
  ></div>
{/if}
