<script>
  import { currentView, sidebarCollapsed, currentLang } from "../../stores/ui";
  import { auth } from "../../stores/auth";
  import { t } from "../../i18n";
  import FolderTree from "./FolderTree.svelte";
  import { onMount } from "svelte";
  import api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Badge counts (reactive)
  let sharedCount = $state(0);
  let trashCount = $state(0);
  let notificationCount = $state(0);

  // Fetch badge counts
  onMount(async () => {
    try {
      // Shared files count
      const sharedResponse = await api.sharing?.list();
      sharedCount = sharedResponse?.data?.length || sharedResponse?.length || 0;

      // Trash count (simulated - would need backend endpoint)
      // trashCount = await api.trash.count();
      trashCount = 5; // Placeholder

      // Activity unread count from backend stats (smart badge)
      const activityStats = await fetchActivityStats();
      notificationCount = activityStats?.unread_count || 0;

      // Update activity count periodically (every 30 seconds)
      setInterval(async () => {
        const stats = await fetchActivityStats();
        notificationCount = stats?.unread_count || 0;
      }, 30000);
    } catch (error) {
      console.error("Failed to load badge counts:", error);
    }
  });

  // Fetch activity statistics
  async function fetchActivityStats() {
    try {
      const token = localStorage.getItem("authToken");
      if (!token) return null;

      const response = await fetch(
        `${new URL(window.location.href).protocol}//${new URL(window.location.href).hostname}:8080/api/activity/stats`,
        {
          headers: { Authorization: `Bearer ${token}` },
        }
      );

      if (response.ok) {
        return await response.json();
      }
    } catch (e) {
      console.error("Failed to fetch activity stats:", e);
    }
    return null;
  }

  // Navigation items - Sinnvoll organisiert
  // Main (primäre Navigation - immer sichtbar)
  // Tools (optionale Features - collapsible)
  // System (Settings, Admin - in separatem Settings-Tab)
  let navItems = $derived([
    {
      id: "dashboard",
      icon: "speedometer2",
      label: t($currentLang, "dashboard.title"),
      category: "main",
    },
    {
      id: "files",
      icon: "folder-fill",
      label: t($currentLang, "files"),
      category: "main",
    },
    {
      id: "favorites",
      icon: "star-fill",
      label: t($currentLang, "favorites"),
      category: "main",
    },
    {
      id: "recent",
      icon: "clock-history",
      label: t($currentLang, "recentFiles"),
      category: "main",
    },
    {
      id: "shared",
      icon: "share-fill",
      label: t($currentLang, "shared"),
      category: "main",
      badge: sharedCount,
      badgeClass:
        "bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200",
    },
    {
      id: "activity",
      label: t($currentLang, "activity"),
      category: "tools",
      icon: "activity",
      badge: notificationCount,
      badgeClass:
        "bg-purple-100 dark:bg-purple-900 text-purple-700 dark:text-purple-200",
    },
    {
      id: "trash",
      icon: "trash-fill",
      label: t($currentLang, "trash"),
      category: "tools",
      badge: trashCount,
      badgeClass:
        "bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-200",
    },
    {
      id: "settings",
      icon: "gear-fill",
      label: t($currentLang, "settings"),
      category: "system",
    },
  ]);

  // Group by category
  let mainItems = $derived(navItems.filter((item) => item.category === "main"));
  let toolsItems = $derived(
    navItems.filter((item) => item.category === "tools")
  );
  let systemItems = $derived(
    navItems.filter((item) => item.category === "system")
  );

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
                <stop offset="0%" stop-color="#3b82f6" />
                <stop offset="50%" stop-color="#a855f7" />
                <stop offset="100%" stop-color="#ec4899" />
              </linearGradient>
            </defs>
          </svg>
          <div class="brand-text">
            <h1 class="brand-name">SyncSpace</h1>
            <p class="brand-tagline">{t($currentLang, "cloudStorage")}</p>
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
              <stop offset="0%" stop-color="#3b82f6" />
              <stop offset="50%" stop-color="#a855f7" />
              <stop offset="100%" stop-color="#ec4899" />
            </linearGradient>
          </defs>
        </svg>
      {/if}
    </div>

    <button
      class="w-8 h-8 rounded-full flex items-center justify-center hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400 transition-colors collapse-toggle"
      onclick={toggleSidebar}
      aria-label="Toggle sidebar"
    >
      <i class="bi bi-chevron-{$sidebarCollapsed ? 'right' : 'left'} text-lg"
      ></i>
    </button>
  </div>

  <!-- Navigation -->
  <nav class="sidebar-nav">
    <!-- Main Section -->
    {#if !$sidebarCollapsed}
      <div class="menu-category">{t($currentLang, "main")}</div>
    {/if}
    <ul class="menu menu-sm p-0 gap-1">
      {#each mainItems as item}
        <li>
          <button
            class="menu-item {$currentView === item.id
              ? 'active'
              : ''} {$sidebarCollapsed ? 'group relative' : ''}"
            data-tip={$sidebarCollapsed ? item.label : ""}
            onclick={() => selectView(item.id)}
          >
            <i class="bi bi-{item.icon} text-lg"></i>
            {#if !$sidebarCollapsed}
              <span class="menu-label">{item.label}</span>
              {#if item.badge && item.badge > 0}
                <span
                  class="px-1.5 py-0.5 text-xs font-medium rounded-full ml-auto {item.badgeClass}"
                  >{item.badge}</span
                >
              {/if}
            {:else if $sidebarCollapsed}
              <span
                class="absolute left-full ml-2 px-2 py-1 text-xs font-medium bg-gray-900 dark:bg-gray-700 text-white rounded opacity-0 group-hover:opacity-100 pointer-events-none whitespace-nowrap z-50 transition-opacity"
              >
                {item.label}
              </span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- Folder Tree - DISABLED: Folders are now shown inline in the main file view -->
    <!-- {#if $currentView === "files" && !$sidebarCollapsed}
      <FolderTree />
    {/if} -->

    <!-- Tools Section -->
    <div class="h-px bg-gray-200 dark:bg-gray-700 my-2"></div>
    {#if !$sidebarCollapsed}
      <div class="menu-category">{t($currentLang, "tools")}</div>
    {/if}
    <ul class="menu menu-sm p-0 gap-1">
      {#each toolsItems as item}
        <li>
          <button
            class="menu-item {$currentView === item.id
              ? 'active'
              : ''} {$sidebarCollapsed ? 'group relative' : ''}"
            data-tip={$sidebarCollapsed ? item.label : ""}
            onclick={() => selectView(item.id)}
          >
            <i class="bi bi-{item.icon} text-lg"></i>
            {#if !$sidebarCollapsed}
              <span class="menu-label">{item.label}</span>
              {#if item.badge && item.badge > 0}
                <span
                  class="px-1.5 py-0.5 text-xs font-medium rounded-full ml-auto {item.badgeClass}"
                  >{item.badge}</span
                >
              {/if}
            {:else if $sidebarCollapsed}
              <span
                class="absolute left-full ml-2 px-2 py-1 text-xs font-medium bg-gray-900 dark:bg-gray-700 text-white rounded opacity-0 group-hover:opacity-100 pointer-events-none whitespace-nowrap z-50 transition-opacity"
              >
                {item.label}
              </span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- System Section -->
    <div class="h-px bg-gray-200 dark:bg-gray-700 my-2"></div>
    {#if !$sidebarCollapsed}
      <div class="menu-category">{t($currentLang, "system")}</div>
    {/if}
    <ul class="menu menu-sm p-0 gap-1">
      {#each systemItems as item}
        <li>
          <button
            class="menu-item {$currentView === item.id
              ? 'active'
              : ''} {$sidebarCollapsed ? 'group relative' : ''}"
            data-tip={$sidebarCollapsed ? item.label : ""}
            onclick={() => selectView(item.id)}
          >
            <i class="bi bi-{item.icon} text-lg"></i>
            {#if !$sidebarCollapsed}
              <span class="menu-label">{item.label}</span>
              {#if item.badge && item.badge > 0}
                <span
                  class="px-1.5 py-0.5 text-xs font-medium rounded-full ml-auto {item.badgeClass}"
                  >{item.badge}</span
                >
              {/if}
            {:else if $sidebarCollapsed}
              <span
                class="absolute left-full ml-2 px-2 py-1 text-xs font-medium bg-gray-900 dark:bg-gray-700 text-white rounded opacity-0 group-hover:opacity-100 pointer-events-none whitespace-nowrap z-50 transition-opacity"
              >
                {item.label}
              </span>
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
    width: 260px;
    height: 100vh;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.95), rgba(248, 250, 252, 0.9));
    backdrop-filter: blur(20px) saturate(180%);
    border-right: 1px solid rgba(59, 130, 246, 0.1);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
    box-shadow: 2px 0 16px rgba(59, 130, 246, 0.08);
  }

  :global(.dark) .sidebar-container {
    background: linear-gradient(135deg, rgba(31, 41, 55, 0.95), rgba(17, 24, 39, 0.9));
    border-right: 1px solid rgba(59, 130, 246, 0.15);
    box-shadow: 2px 0 16px rgba(59, 130, 246, 0.12);
  }

  .sidebar-container.collapsed {
    width: 72px;
  }

  /* Header */
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1rem;
    border-bottom: 1px solid rgba(229, 231, 235, 0.8);
    min-height: 72px;
  }

  :global(.dark) .sidebar-header {
    border-bottom: 1px solid rgba(55, 65, 81, 0.8);
  }

  .brand {
    flex: 1;
    min-width: 0;
  }

  .logo-wrapper {
    display: flex;
    align-items: center;
    gap: 0.875rem;
  }

  .logo-icon {
    width: 36px;
    height: 36px;
    flex-shrink: 0;
    filter: drop-shadow(0 2px 8px rgba(59, 130, 246, 0.35));
  }

  .logo-icon-collapsed {
    width: 32px;
    height: 32px;
    filter: drop-shadow(0 2px 8px rgba(59, 130, 246, 0.35));
  }

  .brand-text {
    flex: 1;
    min-width: 0;
  }

  .brand-name {
    font-size: 1.25rem;
    font-weight: 800;
    background: linear-gradient(135deg, #3b82f6 0%, #a855f7 50%, #ec4899 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin: 0;
    line-height: 1.3;
  }

  .brand-tagline {
    font-size: 0.6875rem;
    color: rgb(107 114 128);
    font-weight: 500;
    margin: 0;
    line-height: 1.2;
  }

  :global(.dark) .brand-tagline {
    color: rgb(156 163 175);
  }

  .collapse-toggle {
    flex-shrink: 0;
    color: rgb(107 114 128);
  }

  :global(.dark) .collapse-toggle {
    color: rgb(156 163 175);
  }

  .collapse-toggle:hover {
    background: rgba(229, 231, 235, 0.5);
    color: rgb(59 130 246);
  }

  :global(.dark) .collapse-toggle:hover {
    background: rgba(55, 65, 81, 0.5);
    color: rgb(96 165 250);
  }

  /* Navigation */
  .sidebar-nav {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 1rem 0.875rem;
  }

  .menu-category {
    font-size: 0.6875rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: rgb(107 114 128);
    padding: 0.625rem 0.875rem 0.375rem;
  }

  :global(.dark) .menu-category {
    color: rgb(156 163 175);
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    width: 100%;
    padding: 0.75rem 0.875rem;
    border-radius: 12px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    font-size: 0.9375rem;
    font-weight: 500;
    position: relative;
    white-space: nowrap;
    color: rgb(55 65 81);
    background: transparent;
    border: none;
    cursor: pointer;
  }

  :global(.dark) .menu-item {
    color: rgb(209 213 219);
  }

  .sidebar-container.collapsed .menu-item {
    justify-content: center;
    padding: 0.875rem;
  }

  .menu-item i {
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    flex-shrink: 0;
    color: rgb(107 114 128);
  }

  :global(.dark) .menu-item i {
    color: rgb(156 163 175);
  }

  .menu-item .menu-label {
    opacity: 1;
    transition: opacity 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    flex: 1;
    text-align: left;
  }

  .sidebar-container.collapsed .menu-label {
    opacity: 0;
    width: 0;
    overflow: hidden;
  }

  .menu-item:hover {
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.08),
      rgba(168, 85, 247, 0.08)
    );
    color: rgb(59 130 246);
  }

  :global(.dark) .menu-item:hover {
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.15),
      rgba(168, 85, 247, 0.15)
    );
    color: rgb(96 165 250);
  }

  .menu-item:hover i {
    transform: scale(1.1);
    color: rgb(59 130 246);
  }

  :global(.dark) .menu-item:hover i {
    color: rgb(96 165 250);
  }

  .menu-item.active {
    background: linear-gradient(135deg, #3b82f6, #a855f7);
    color: white;
    font-weight: 600;
    box-shadow: 0 6px 16px rgba(59, 130, 246, 0.3);
  }

  .menu-item.active i {
    color: white;
  }

  /* Badge styling */
  .menu-item .badge {
    font-size: 0.6875rem;
    padding: 0.25rem 0.5rem;
    line-height: 1;
    font-weight: 600;
    border-radius: 100px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .sidebar-container.collapsed .menu-item .badge {
    position: absolute;
    top: 0.375rem;
    right: 0.375rem;
    min-width: 1.125rem;
    height: 1.125rem;
    padding: 0 0.25rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.5625rem;
  }

  .collapsed .menu-item {
    justify-content: center;
    padding: 0.75rem;
  }

  /* Scrollbar */
  .sidebar-nav::-webkit-scrollbar {
    width: 6px;
  }

  .sidebar-nav::-webkit-scrollbar-track {
    background: transparent;
  }

  .sidebar-nav::-webkit-scrollbar-thumb {
    background: rgba(156, 163, 175, 0.3);
    border-radius: 3px;
  }

  .sidebar-nav::-webkit-scrollbar-thumb:hover {
    background: rgba(156, 163, 175, 0.5);
  }

  :global(.dark) .sidebar-nav::-webkit-scrollbar-thumb {
    background: rgba(75, 85, 99, 0.5);
  }

  :global(.dark) .sidebar-nav::-webkit-scrollbar-thumb:hover {
    background: rgba(75, 85, 99, 0.7);
  }
</style>
