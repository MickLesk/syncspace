<script>
  import { currentView, sidebarCollapsed, currentLang } from "../../stores/ui";
  import { auth } from "../../stores/auth";
  import { t } from "../../i18n";
  import { onMount, createEventDispatcher } from "svelte";
  import api from "../../lib/api.js";

  const dispatch = createEventDispatcher();
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Sidebar state
  let sidebarWidth = $state(260);
  let isResizing = $state(false);
  const minWidth = 72;
  const maxWidth = 400;
  const defaultWidth = 260;
  const collapseThreshold = 140;

  // Badge counts
  let sharedCount = $state(0);
  let trashCount = $state(0);
  let notificationCount = $state(0);

  // Fetch badge counts
  onMount(async () => {
    // Load saved width
    const savedWidth = localStorage.getItem("sidebarWidth");
    if (savedWidth) {
      sidebarWidth = parseInt(savedWidth);
      if (sidebarWidth <= minWidth) {
        sidebarCollapsed.set(true);
      }
    }

    try {
      const sharedResponse = await api.sharing?.list();
      sharedCount = sharedResponse?.data?.length || sharedResponse?.length || 0;

      // Fetch actual trash count from backend
      const trashResponse = await api.trash?.list();
      if (trashResponse?.ok) {
        const trashData = await trashResponse.json();
        trashCount = Array.isArray(trashData)
          ? trashData.length
          : trashData?.value?.length || 0;
      } else {
        trashCount = 0;
      }

      const activityStats = await fetchActivityStats();
      notificationCount = activityStats?.unread_count || 0;

      setInterval(async () => {
        const stats = await fetchActivityStats();
        notificationCount = stats?.unread_count || 0;
      }, 30000);
    } catch (error) {
      console.error("Failed to load badge counts:", error);
    }
  });

  async function fetchActivityStats() {
    try {
      const token = localStorage.getItem("authToken");
      if (!token) return null;

      const response = await fetch(
        `${new URL(window.location.href).protocol}//${new URL(window.location.href).hostname}:8080/api/activity/stats`,
        { headers: { Authorization: `Bearer ${token}` } }
      );

      if (response.ok) {
        return await response.json();
      }
    } catch (e) {
      console.error("Failed to fetch activity stats:", e);
    }
    return null;
  }

  // Navigation items - clean structure
  let navItems = $derived([
    // HOME - Main navigation
    {
      id: "dashboard",
      icon: "speedometer2",
      label: t($currentLang, "dashboard.title"),
      category: "home",
    },
    {
      id: "files",
      icon: "folder-fill",
      label: t($currentLang, "files"),
      category: "home",
    },
    {
      id: "shared",
      icon: "share-fill",
      label: t($currentLang, "shared"),
      category: "home",
      badge: sharedCount,
      badgeColor: "blue",
    },
    {
      id: "activity",
      icon: "clock-history",
      label: t($currentLang, "activity") || "Historie",
      category: "home",
      badge: notificationCount,
      badgeColor: "green",
    },
    // WERKZEUGE - Tools section
    {
      id: "storage-analytics",
      icon: "bar-chart-line-fill",
      label: t($currentLang, "storageAnalytics") || "Speicheranalyse",
      category: "tools",
    },
    {
      id: "tag-cloud",
      icon: "tags-fill",
      label: t($currentLang, "tagCloud") || "Tags",
      category: "tools",
    },
    {
      id: "backup",
      icon: "shield-fill-check",
      label: t($currentLang, "backup") || "Sicherung & Wiederherstellung",
      category: "tools",
    },
    {
      id: "trash",
      icon: "trash-fill",
      label: t($currentLang, "trash"),
      category: "tools",
      badge: trashCount,
      badgeColor: "amber",
    },
    // SETTINGS - Single entry point
    {
      id: "settings",
      icon: "gear-fill",
      label: t($currentLang, "settings"),
      category: "system",
    },
  ]);

  let homeItems = $derived(navItems.filter((item) => item.category === "home"));
  let toolsItems = $derived(
    navItems.filter((item) => item.category === "tools")
  );
  let systemItems = $derived(
    navItems.filter((item) => item.category === "system")
  );

  // Computed: is sidebar collapsed based on width
  let isCollapsed = $derived($sidebarCollapsed || sidebarWidth <= minWidth);
  let showLabels = $derived(!isCollapsed && sidebarWidth >= 180);

  function selectView(viewId) {
    currentView.set(viewId);
    const newHash = `#/${viewId}`;
    if (window.location.hash !== newHash) {
      window.history.pushState({ view: viewId }, "", newHash);
    }
    // Emit event to close mobile menu
    dispatch("navigate", { viewId });
  }

  function toggleSidebar() {
    if ($sidebarCollapsed) {
      sidebarCollapsed.set(false);
      sidebarWidth = defaultWidth;
    } else {
      sidebarCollapsed.set(true);
      sidebarWidth = minWidth;
    }
    localStorage.setItem("sidebarWidth", sidebarWidth.toString());
  }

  // Resize handlers
  function startResize(e) {
    isResizing = true;
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";

    document.addEventListener("mousemove", handleResize);
    document.addEventListener("mouseup", stopResize);
  }

  function handleResize(e) {
    if (!isResizing) return;

    const newWidth = Math.max(minWidth, Math.min(maxWidth, e.clientX));
    sidebarWidth = newWidth;

    // Auto-collapse when dragged below threshold
    if (newWidth <= collapseThreshold && !$sidebarCollapsed) {
      sidebarCollapsed.set(true);
      sidebarWidth = minWidth;
    } else if (newWidth > collapseThreshold && $sidebarCollapsed) {
      sidebarCollapsed.set(false);
    }
  }

  function stopResize() {
    isResizing = false;
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
    document.removeEventListener("mousemove", handleResize);
    document.removeEventListener("mouseup", stopResize);
    localStorage.setItem("sidebarWidth", sidebarWidth.toString());
  }

  // Double-click to reset width
  function resetWidth() {
    sidebarWidth = defaultWidth;
    sidebarCollapsed.set(false);
    localStorage.setItem("sidebarWidth", sidebarWidth.toString());
  }
</script>

<aside
  class="relative flex flex-col h-screen bg-gradient-to-b from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 border-r border-green-500/15 dark:border-green-500/20 shadow-[2px_0_16px_rgba(34,197,94,0.06)] dark:shadow-[2px_0_16px_rgba(0,0,0,0.3)] overflow-hidden {isResizing
    ? ''
    : 'transition-[width] duration-300 ease-out'} {isResizing
    ? 'select-none'
    : ''}"
  style="width: {isCollapsed ? minWidth : sidebarWidth}px"
>
  <!-- Header -->
  <div
    class="flex items-center justify-between p-4 min-h-[64px] border-b border-green-500/10 dark:border-green-500/15"
  >
    <div
      class="flex items-center gap-3 flex-1 min-w-0 transition-all duration-300 {isCollapsed
        ? 'justify-center'
        : ''}"
    >
      <div class="flex-shrink-0 flex items-center justify-center">
        <svg
          class="drop-shadow-[0_2px_8px_rgba(34,197,94,0.4)] transition-all duration-300 {isCollapsed
            ? 'w-9 h-9'
            : 'w-8 h-8'}"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M13 2L3 14h8l-1 8 10-12h-8l1-8z"
            fill="url(#sidebar-logo-gradient)"
          />
          <defs>
            <linearGradient
              id="sidebar-logo-gradient"
              x1="3"
              y1="2"
              x2="21"
              y2="22"
            >
              <stop offset="0%" stop-color="#22c55e" />
              <stop offset="100%" stop-color="#16a34a" />
            </linearGradient>
          </defs>
        </svg>
      </div>
      {#if showLabels}
        <div class="flex-1 min-w-0 animate-fadeIn">
          <h1
            class="text-lg font-bold bg-gradient-to-br from-green-500 to-green-600 bg-clip-text text-transparent m-0 leading-tight whitespace-nowrap"
          >
            SyncSpace
          </h1>
          <p
            class="text-[0.6875rem] text-gray-500 dark:text-gray-400 font-medium m-0 leading-tight whitespace-nowrap"
          >
            {tr("cloudStorage")}
          </p>
        </div>
      {/if}
    </div>

    <button
      class="w-7 h-7 rounded-md flex items-center justify-center bg-green-500/10 dark:bg-green-500/15 hover:bg-green-500/20 dark:hover:bg-green-500/25 border-none text-green-500 cursor-pointer transition-all duration-200 hover:scale-105 flex-shrink-0"
      onclick={toggleSidebar}
      aria-label="Toggle sidebar"
    >
      <i
        class="bi bi-chevron-{isCollapsed ? 'right' : 'left'}"
        aria-hidden="true"
      ></i>
    </button>
  </div>

  <!-- Navigation -->
  <nav class="sidebar-nav flex-1 overflow-y-auto overflow-x-hidden p-3">
    <!-- Main Section -->
    {#if showLabels}
      <div
        class="text-[0.625rem] font-bold tracking-widest uppercase text-gray-400 dark:text-gray-500 py-2 px-3 mt-1"
      >
        Home
      </div>
    {/if}
    <ul class="list-none p-0 m-0 flex flex-col gap-0.5">
      {#each homeItems as item (item.id)}
        <li>
          <button
            class="nav-item relative flex items-center gap-3 w-full py-2.5 px-3 rounded-[10px] bg-transparent border-none text-gray-600 dark:text-gray-300 text-sm font-medium cursor-pointer transition-all duration-200 text-left whitespace-nowrap overflow-hidden hover:bg-green-500/8 dark:hover:bg-green-500/12 hover:text-green-500 dark:hover:text-green-400 {$currentView ===
            item.id
              ? 'active-nav bg-gradient-to-br from-green-500 to-green-600 !text-white shadow-[0_4px_12px_rgba(34,197,94,0.35)]'
              : ''} {isCollapsed ? 'justify-center !p-3' : ''}"
            onclick={() => selectView(item.id)}
            title={isCollapsed ? item.label : ""}
          >
            <span
              class="flex items-center justify-center flex-shrink-0 transition-all duration-200 {isCollapsed
                ? 'w-7 h-7 text-xl'
                : 'w-6 h-6 text-lg'} {$currentView === item.id
                ? 'text-white'
                : 'text-gray-500 dark:text-gray-400'}"
            >
              <i class="bi bi-{item.icon}" aria-hidden="true"></i>
            </span>
            {#if showLabels}
              <span
                class="flex-1 transition-opacity duration-200 overflow-hidden text-ellipsis"
                >{item.label}</span
              >
            {/if}
            {#if item.badge && item.badge > 0}
              <span
                class="nav-badge py-0.5 px-2 text-[0.6875rem] font-semibold rounded-full ml-auto transition-all duration-200 {isCollapsed
                  ? 'absolute top-1 right-1 !p-0 w-4 h-4 text-[0.5625rem] flex items-center justify-center !ml-0'
                  : ''} {$currentView === item.id
                  ? 'bg-white/25 text-white'
                  : item.badgeColor === 'blue'
                    ? 'bg-blue-100 text-blue-600 dark:bg-blue-600/20 dark:text-blue-400'
                    : item.badgeColor === 'amber'
                      ? 'bg-amber-100 text-amber-600 dark:bg-amber-600/20 dark:text-amber-400'
                      : 'bg-purple-100 text-purple-600 dark:bg-purple-600/20 dark:text-purple-400'}"
              >
                {item.badge}
              </span>
            {/if}
            {#if isCollapsed}
              <span
                class="tooltip absolute left-[calc(100%+8px)] top-1/2 -translate-y-1/2 py-1.5 px-3 bg-gray-800 dark:bg-gray-700 text-white text-xs font-medium rounded-md whitespace-nowrap opacity-0 invisible transition-all duration-200 z-[100] pointer-events-none shadow-lg"
                >{item.label}</span
              >
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- Tools Section -->
    <div
      class="h-px bg-gradient-to-r from-transparent via-green-500/20 to-transparent my-2"
    ></div>
    {#if showLabels}
      <div
        class="text-[0.625rem] font-bold tracking-widest uppercase text-gray-400 dark:text-gray-500 py-2 px-3"
      >
        Werkzeuge
      </div>
    {/if}
    <ul class="list-none p-0 m-0 flex flex-col gap-0.5">
      {#each toolsItems as item (item.id)}
        <li>
          <button
            class="nav-item relative flex items-center gap-3 w-full py-2.5 px-3 rounded-[10px] bg-transparent border-none text-gray-600 dark:text-gray-300 text-sm font-medium cursor-pointer transition-all duration-200 text-left whitespace-nowrap overflow-hidden hover:bg-green-500/8 dark:hover:bg-green-500/12 hover:text-green-500 dark:hover:text-green-400 {$currentView ===
            item.id
              ? 'active-nav bg-gradient-to-br from-green-500 to-green-600 !text-white shadow-[0_4px_12px_rgba(34,197,94,0.35)]'
              : ''} {isCollapsed ? 'justify-center !p-3' : ''}"
            onclick={() => selectView(item.id)}
            title={isCollapsed ? item.label : ""}
          >
            <span
              class="flex items-center justify-center flex-shrink-0 transition-all duration-200 {isCollapsed
                ? 'w-7 h-7 text-xl'
                : 'w-6 h-6 text-lg'} {$currentView === item.id
                ? 'text-white'
                : 'text-gray-500 dark:text-gray-400'}"
            >
              <i class="bi bi-{item.icon}" aria-hidden="true"></i>
            </span>
            {#if showLabels}
              <span
                class="flex-1 transition-opacity duration-200 overflow-hidden text-ellipsis"
                >{item.label}</span
              >
            {/if}
            {#if item.badge && item.badge > 0}
              <span
                class="nav-badge py-0.5 px-2 text-[0.6875rem] font-semibold rounded-full ml-auto transition-all duration-200 {isCollapsed
                  ? 'absolute top-1 right-1 !p-0 w-4 h-4 text-[0.5625rem] flex items-center justify-center !ml-0'
                  : ''} {$currentView === item.id
                  ? 'bg-white/25 text-white'
                  : item.badgeColor === 'blue'
                    ? 'bg-blue-100 text-blue-600 dark:bg-blue-600/20 dark:text-blue-400'
                    : item.badgeColor === 'amber'
                      ? 'bg-amber-100 text-amber-600 dark:bg-amber-600/20 dark:text-amber-400'
                      : 'bg-purple-100 text-purple-600 dark:bg-purple-600/20 dark:text-purple-400'}"
              >
                {item.badge}
              </span>
            {/if}
            {#if isCollapsed}
              <span
                class="tooltip absolute left-[calc(100%+8px)] top-1/2 -translate-y-1/2 py-1.5 px-3 bg-gray-800 dark:bg-gray-700 text-white text-xs font-medium rounded-md whitespace-nowrap opacity-0 invisible transition-all duration-200 z-[100] pointer-events-none shadow-lg"
                >{item.label}</span
              >
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- System Section (Settings) -->
    <div
      class="h-px bg-gradient-to-r from-transparent via-green-500/20 to-transparent my-2"
    ></div>
    <ul class="list-none p-0 m-0 flex flex-col gap-0.5">
      {#each systemItems as item (item.id)}
        <li>
          <button
            class="nav-item relative flex items-center gap-3 w-full py-2.5 px-3 rounded-[10px] bg-transparent border-none text-gray-600 dark:text-gray-300 text-sm font-medium cursor-pointer transition-all duration-200 text-left whitespace-nowrap overflow-hidden hover:bg-green-500/8 dark:hover:bg-green-500/12 hover:text-green-500 dark:hover:text-green-400 {$currentView ===
            item.id
              ? 'active-nav bg-gradient-to-br from-green-500 to-green-600 !text-white shadow-[0_4px_12px_rgba(34,197,94,0.35)]'
              : ''} {isCollapsed ? 'justify-center !p-3' : ''}"
            onclick={() => selectView(item.id)}
            title={isCollapsed ? item.label : ""}
          >
            <span
              class="flex items-center justify-center flex-shrink-0 transition-all duration-200 {isCollapsed
                ? 'w-7 h-7 text-xl'
                : 'w-6 h-6 text-lg'} {$currentView === item.id
                ? 'text-white'
                : 'text-gray-500 dark:text-gray-400'}"
            >
              <i class="bi bi-{item.icon}" aria-hidden="true"></i>
            </span>
            {#if showLabels}
              <span
                class="flex-1 transition-opacity duration-200 overflow-hidden text-ellipsis"
                >{item.label}</span
              >
            {/if}
            {#if isCollapsed}
              <span
                class="tooltip absolute left-[calc(100%+8px)] top-1/2 -translate-y-1/2 py-1.5 px-3 bg-gray-800 dark:bg-gray-700 text-white text-xs font-medium rounded-md whitespace-nowrap opacity-0 invisible transition-all duration-200 z-[100] pointer-events-none shadow-lg"
                >{item.label}</span
              >
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  </nav>

  <!-- Version Footer -->
  <div
    class="p-3 border-t border-green-500/10 dark:border-green-500/15 text-center"
  >
    {#if showLabels}
      <span class="text-[0.6875rem] text-gray-400 font-medium">v1.0-beta</span>
    {:else}
      <span class="text-[0.625rem] text-gray-400 font-semibold">v1</span>
    {/if}
  </div>

  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="resize-handle absolute top-0 -right-1 w-2 h-full cursor-ew-resize z-10 transition-colors duration-200 hover:bg-green-500/15 {isResizing
      ? 'bg-green-500/15'
      : ''}"
    onmousedown={startResize}
    ondblclick={resetWidth}
    role="separator"
    aria-label="Resize sidebar"
  ></div>
</aside>

<style>
  /* Tooltip hover visibility */
  .nav-item:hover .tooltip {
    opacity: 1;
    visibility: visible;
  }

  /* Scrollbar styling */
  .sidebar-nav::-webkit-scrollbar {
    width: 4px;
  }
  .sidebar-nav::-webkit-scrollbar-track {
    background: transparent;
  }
  .sidebar-nav::-webkit-scrollbar-thumb {
    background: rgba(34, 197, 94, 0.2);
    border-radius: 2px;
  }
  .sidebar-nav::-webkit-scrollbar-thumb:hover {
    background: rgba(34, 197, 94, 0.4);
  }
  :global(.dark) .sidebar-nav::-webkit-scrollbar-thumb {
    background: rgba(34, 197, 94, 0.3);
  }

  /* Resize handle indicator */
  .resize-handle::after {
    content: "";
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 3px;
    height: 32px;
    background: transparent;
    border-radius: 3px;
    transition: all 0.2s ease;
  }
  .resize-handle:hover::after {
    background: rgba(34, 197, 94, 0.5);
  }
</style>
