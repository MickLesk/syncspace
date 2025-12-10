<script>
  import { currentView, currentLang } from "../../stores/ui";
  import { t } from "../../i18n";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Quick access items for bottom nav
  let navItems = $derived([
    {
      id: "files",
      icon: "folder-fill",
      label: tr("files"),
    },
    {
      id: "recent",
      icon: "clock-fill",
      label: tr("recentFiles") || "Recent",
    },
    {
      id: "favorites",
      icon: "star-fill",
      label: tr("favorites"),
    },
    {
      id: "shared",
      icon: "share-fill",
      label: tr("shared"),
    },
    {
      id: "activity",
      icon: "clock-history",
      label: tr("activity") || "Activity",
    },
  ]);

  function selectView(viewId) {
    currentView.set(viewId);
    const newHash = `#/${viewId}`;
    if (window.location.hash !== newHash) {
      window.history.pushState({ view: viewId }, "", newHash);
    }
  }
</script>

<!-- Mobile Bottom Navigation Bar -->
<nav class="fixed bottom-0 left-0 right-0 z-40 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 pb-[env(safe-area-inset-bottom)] shadow-[0_-2px_10px_rgba(0,0,0,0.1)] md:hidden">
  <div class="flex justify-around items-center max-w-full py-1">
    {#each navItems as item (item.id)}
      <button
        class="flex flex-col items-center justify-center px-3 py-2 min-w-16 min-h-14 rounded-xl border-none cursor-pointer transition-all duration-200 [-webkit-tap-highlight-color:transparent] active:scale-95 {$currentView === item.id 
          ? 'text-green-500 dark:text-green-400 bg-green-500/10 dark:bg-green-500/15' 
          : 'text-gray-500 dark:text-gray-400 bg-transparent active:bg-gray-100 dark:active:bg-gray-700'}"
        onclick={() => selectView(item.id)}
        aria-label={item.label}
      >
        <i class="bi bi-{item.icon} text-xl mb-0.5" aria-hidden="true"></i>
        <span class="text-[0.65rem] font-medium capitalize whitespace-nowrap overflow-hidden text-ellipsis max-w-14">{item.label}</span>
      </button>
    {/each}
  </div>
</nav>

<style>
  .mobile-bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 40;
    background: var(--bg-primary, white);
    border-top: 1px solid var(--border-color, #e5e7eb);
    padding-bottom: env(safe-area-inset-bottom);
    box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
  }

  :global(.dark) .mobile-bottom-nav {
    background: #1f2937;
    border-top-color: #374151;
  }

  .nav-container {
    display: flex;
    justify-content: space-around;
    align-items: center;
    max-width: 100%;
    padding: 0.25rem 0;
  }

  .nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 0.75rem;
    min-width: 64px;
    min-height: 56px;
    border-radius: 0.75rem;
    color: var(--text-secondary, #6b7280);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s ease;
    -webkit-tap-highlight-color: transparent;
  }

  :global(.dark) .nav-item {
    color: #9ca3af;
  }

  .nav-item:active {
    transform: scale(0.95);
    background: var(--bg-secondary, #f3f4f6);
  }

  :global(.dark) .nav-item:active {
    background: #374151;
  }

  .nav-item.active {
    color: #22c55e;
    background: rgba(34, 197, 94, 0.1);
  }

  :global(.dark) .nav-item.active {
    color: #4ade80;
    background: rgba(34, 197, 94, 0.15);
  }

  .nav-item i {
    font-size: 1.25rem;
    margin-bottom: 0.125rem;
  }

  .nav-label {
    font-size: 0.65rem;
    font-weight: 500;
    text-transform: capitalize;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 56px;
  }

  /* Touch target for accessibility */
  .touch-target {
    min-height: 44px;
    min-width: 44px;
  }

  /* Add padding to main content area when bottom nav is visible */
  @media (max-width: 767px) {
    :global(main.main-content) {
      padding-bottom: calc(72px + env(safe-area-inset-bottom));
    }
  }
</style>
