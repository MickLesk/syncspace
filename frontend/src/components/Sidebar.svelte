<script>
  import { currentView, sidebarCollapsed, currentLang, favoritesEnabled } from "../stores/ui";
  import { auth } from "../stores/auth";
  import { t } from "../i18n";

  // Navigation items - Storage & Backup sind jetzt in Settings-Tabs
  $: navItems = [
    {
      id: "files",
      icon: "folder-fill",
      label: t($currentLang, "files"),
      category: "main",
    },
    {
      id: "shared",
      icon: "share-fill",
      label: t($currentLang, "shared"),
      category: "main",
    },
    // Favorites nur anzeigen wenn enabled
    ...($favoritesEnabled ? [{
      id: "favorites",
      icon: "star-fill",
      label: t($currentLang, "favorites"),
      category: "main",
    }] : []),
    {
      id: "activity",
      icon: "clock-history",
      label: "Activity",
      category: "tools",
    },
    { id: "duplicates", icon: "files", label: "Duplicates", category: "tools" },
    {
      id: "trash",
      icon: "trash-fill",
      label: t($currentLang, "trash"),
      category: "system",
    },
    {
      id: "settings",
      icon: "gear-fill",
      label: t($currentLang, "settings"),
      category: "system",
    },
  ];

  // Group by category
  $: mainItems = navItems.filter((item) => item.category === "main");
  $: toolsItems = navItems.filter((item) => item.category === "tools");
  $: systemItems = navItems.filter((item) => item.category === "system");

  function selectView(viewId) {
    currentView.set(viewId);
  }

  function toggleSidebar() {
    sidebarCollapsed.toggle();
  }
</script>

<aside class="sidebar-container" class:collapsed={$sidebarCollapsed}>
  <!-- Sidebar Header -->
  <div class="sidebar-header">
    <div class="brand">
      {#if !$sidebarCollapsed}
        <div class="logo-wrapper">
          <svg
            class="logo-icon"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M13 2L3 14h8l-1 8 10-12h-8l1-8z"
              fill="url(#logo-gradient)"
            />
            <defs>
              <linearGradient id="logo-gradient" x1="3" y1="2" x2="21" y2="22">
                <stop offset="0%" stop-color="#667eea" />
                <stop offset="100%" stop-color="#764ba2" />
              </linearGradient>
            </defs>
          </svg>
          <div class="brand-text">
            <h1 class="brand-name">SyncSpace</h1>
            <p class="brand-tagline">Cloud Storage</p>
          </div>
        </div>
      {:else}
        <svg
          class="logo-icon-collapsed"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M13 2L3 14h8l-1 8 10-12h-8l1-8z"
            fill="url(#logo-gradient-collapsed)"
          />
          <defs>
            <linearGradient
              id="logo-gradient-collapsed"
              x1="3"
              y1="2"
              x2="21"
              y2="22"
            >
              <stop offset="0%" stop-color="#667eea" />
              <stop offset="100%" stop-color="#764ba2" />
            </linearGradient>
          </defs>
        </svg>
      {/if}
    </div>

    <button
      class="btn btn-ghost btn-sm btn-circle collapse-toggle"
      on:click={toggleSidebar}
      aria-label="Toggle sidebar"
    >
      <i
        class="bi bi-chevron-{$sidebarCollapsed ? 'right' : 'left'} text-lg"
      ></i>
    </button>
  </div>

  <!-- Navigation -->
  <nav class="sidebar-nav">
    <!-- Main Section -->
    {#if !$sidebarCollapsed}
      <div class="menu-category">MAIN</div>
    {/if}
    <ul class="menu menu-sm p-0 gap-1">
      {#each mainItems as item}
        <li>
          <button
            class="menu-item {$currentView === item.id ? 'active' : ''}"
            on:click={() => selectView(item.id)}
          >
            <i class="bi bi-{item.icon} text-lg"></i>
            {#if !$sidebarCollapsed}
              <span>{item.label}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- Tools Section -->
    <div class="divider my-2"></div>
    {#if !$sidebarCollapsed}
      <div class="menu-category">TOOLS</div>
    {/if}
    <ul class="menu menu-sm p-0 gap-1">
      {#each toolsItems as item}
        <li>
          <button
            class="menu-item {$currentView === item.id ? 'active' : ''}"
            on:click={() => selectView(item.id)}
          >
            <i class="bi bi-{item.icon} text-lg"></i>
            {#if !$sidebarCollapsed}
              <span>{item.label}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- System Section -->
    <div class="divider my-2"></div>
    {#if !$sidebarCollapsed}
      <div class="menu-category">SYSTEM</div>
    {/if}
    <ul class="menu menu-sm p-0 gap-1">
      {#each systemItems as item}
        <li>
          <button
            class="menu-item {$currentView === item.id ? 'active' : ''}"
            on:click={() => selectView(item.id)}
          >
            <i class="bi bi-{item.icon} text-lg"></i>
            {#if !$sidebarCollapsed}
              <span>{item.label}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  </nav>
</aside>

<style>
  .sidebar-container {
    display: flex;
    flex-direction: column;
    width: 240px;
    height: 100vh;
    background: var(--md-sys-color-surface);
    border-right: 1px solid var(--md-sys-color-outline-variant);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
  }

  .sidebar-container.collapsed {
    width: 64px;
  }

  /* Header */
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
    min-height: 64px;
  }

  .brand {
    flex: 1;
    min-width: 0;
  }

  .logo-wrapper {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .logo-icon {
    width: 32px;
    height: 32px;
    flex-shrink: 0;
  }

  .logo-icon-collapsed {
    width: 28px;
    height: 28px;
  }

  .brand-text {
    flex: 1;
    min-width: 0;
  }

  .brand-name {
    font-size: 1.125rem;
    font-weight: 700;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin: 0;
    line-height: 1.2;
  }

  .brand-tagline {
    font-size: 0.625rem;
    opacity: 0.6;
    margin: 0;
    line-height: 1.2;
  }

  .collapse-toggle {
    flex-shrink: 0;
  }

  /* Navigation */
  .sidebar-nav {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 1rem 0.75rem;
  }

  .menu-category {
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    opacity: 0.5;
    padding: 0.5rem 0.75rem 0.25rem;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.625rem 0.75rem;
    border-radius: 0.5rem;
    transition: all 0.2s;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .menu-item:hover {
    background: var(--md-sys-color-surface-variant);
  }

  .menu-item.active {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
    font-weight: 600;
  }

  .collapsed .menu-item {
    justify-content: center;
    padding: 0.625rem;
  }

  /* Scrollbar */
  .sidebar-nav::-webkit-scrollbar {
    width: 4px;
  }

  .sidebar-nav::-webkit-scrollbar-track {
    background: transparent;
  }

  .sidebar-nav::-webkit-scrollbar-thumb {
    background: var(--md-sys-color-outline-variant);
    border-radius: 2px;
  }

  .sidebar-nav::-webkit-scrollbar-thumb:hover {
    background: var(--md-sys-color-outline);
  }
</style>
