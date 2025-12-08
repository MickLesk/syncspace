<script>
  import { currentView, sidebarCollapsed, currentLang } from "../../stores/ui";
  import { auth } from "../../stores/auth";
  import { t } from "../../i18n";
  import { onMount } from "svelte";
  import api from "../../lib/api.js";

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
      trashCount = 5; // Placeholder

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

  // Navigation items
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
      icon: "clock-fill",
      label: t($currentLang, "recentFiles") || "Recent Files",
      category: "main",
    },
    {
      id: "smart-folders",
      icon: "lightning-fill",
      label: t($currentLang, "smartFolders") || "Smart Folders",
      category: "main",
    },
    {
      id: "activity",
      icon: "clock-history",
      label: t($currentLang, "activityHistory"),
      category: "main",
      badge: notificationCount,
      badgeColor: "purple",
    },
    {
      id: "shared",
      icon: "share-fill",
      label: t($currentLang, "shared"),
      category: "main",
      badge: sharedCount,
      badgeColor: "blue",
    },
    {
      id: "trash",
      icon: "trash-fill",
      label: t($currentLang, "trash"),
      category: "tools",
      badge: trashCount,
      badgeColor: "amber",
    },
    {
      id: "duplicates",
      icon: "copy",
      label: t($currentLang, "duplicates.title") || "Duplicates",
      category: "tools",
    },
    {
      id: "tag-cloud",
      icon: "tags-fill",
      label: t($currentLang, "tagCloud") || "Tags",
      category: "tools",
    },
    {
      id: "users",
      icon: "people-fill",
      label: t($currentLang, "users") || "Users",
      category: "admin",
    },
    {
      id: "roles",
      icon: "person-badge-fill",
      label: t($currentLang, "roles.title") || "Role Management",
      category: "admin",
    },
    {
      id: "workflows",
      icon: "diagram-3-fill",
      label: t($currentLang, "workflows.title") || "Workflows",
      category: "admin",
    },
    {
      id: "backup",
      icon: "cloud-arrow-up-fill",
      label: t($currentLang, "backup.title") || "Backup & Restore",
      category: "admin",
    },
    {
      id: "jobs-queue",
      icon: "list-task",
      label: t($currentLang, "jobs.title") || "Jobs Queue",
      category: "admin",
    },
    {
      id: "cloud-storage",
      icon: "cloud-fill",
      label: t($currentLang, "cloudStorage.title") || "Cloud Storage",
      category: "admin",
    },
    {
      id: "webhooks",
      icon: "link-45deg",
      label: t($currentLang, "webhooks.title") || "Webhooks",
      category: "admin",
    },
    {
      id: "audit",
      icon: "shield-check",
      label: t($currentLang, "audit.title") || "Audit & Compliance",
      category: "system",
    },
    {
      id: "notifications",
      icon: "bell-fill",
      label: t($currentLang, "notifications") || "Notifications",
      category: "system",
    },
    {
      id: "storage-analytics",
      icon: "bar-chart-line-fill",
      label: t($currentLang, "storageAnalytics"),
      category: "system",
    },
    {
      id: "settings",
      icon: "gear-fill",
      label: t($currentLang, "settings"),
      category: "system",
    },
  ]);

  let mainItems = $derived(navItems.filter((item) => item.category === "main"));
  let toolsItems = $derived(
    navItems.filter((item) => item.category === "tools")
  );
  let adminItems = $derived(
    navItems.filter((item) => item.category === "admin")
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
  class="sidebar"
  class:collapsed={isCollapsed}
  class:resizing={isResizing}
  style="width: {isCollapsed ? minWidth : sidebarWidth}px"
>
  <!-- Header -->
  <div class="sidebar-header">
    <div class="brand" class:collapsed={isCollapsed}>
      <div class="logo-container">
        <svg
          class="logo"
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
        <div class="brand-text">
          <h1 class="brand-name">SyncSpace</h1>
          <p class="brand-tagline">{tr("cloudStorage")}</p>
        </div>
      {/if}
    </div>

    <button
      class="collapse-btn"
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
  <nav class="sidebar-nav">
    <!-- Main Section -->
    {#if showLabels}
      <div class="nav-category">{tr("main")}</div>
    {/if}
    <ul class="nav-list">
      {#each mainItems as item (item.id)}
        <li>
          <button
            class="nav-item"
            class:active={$currentView === item.id}
            class:collapsed={isCollapsed}
            onclick={() => selectView(item.id)}
            title={isCollapsed ? item.label : ""}
          >
            <span class="nav-icon">
              <i class="bi bi-{item.icon}" aria-hidden="true"></i>
            </span>
            {#if showLabels}
              <span class="nav-label">{item.label}</span>
            {/if}
            {#if item.badge && item.badge > 0}
              <span
                class="nav-badge {item.badgeColor}"
                class:mini={isCollapsed}
              >
                {item.badge}
              </span>
            {/if}
            {#if isCollapsed}
              <span class="tooltip">{item.label}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- Tools Section -->
    <div class="nav-divider"></div>
    {#if showLabels}
      <div class="nav-category">{tr("tools")}</div>
    {/if}
    <ul class="nav-list">
      {#each toolsItems as item (item.id)}
        <li>
          <button
            class="nav-item"
            class:active={$currentView === item.id}
            class:collapsed={isCollapsed}
            onclick={() => selectView(item.id)}
            title={isCollapsed ? item.label : ""}
          >
            <span class="nav-icon">
              <i class="bi bi-{item.icon}" aria-hidden="true"></i>
            </span>
            {#if showLabels}
              <span class="nav-label">{item.label}</span>
            {/if}
            {#if item.badge && item.badge > 0}
              <span
                class="nav-badge {item.badgeColor}"
                class:mini={isCollapsed}
              >
                {item.badge}
              </span>
            {/if}
            {#if isCollapsed}
              <span class="tooltip">{item.label}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- Admin Section -->
    <div class="nav-divider"></div>
    {#if showLabels}
      <div class="nav-category">{tr("admin") || "Admin"}</div>
    {/if}
    <ul class="nav-list">
      {#each adminItems as item (item.id)}
        <li>
          <button
            class="nav-item"
            class:active={$currentView === item.id}
            class:collapsed={isCollapsed}
            onclick={() => selectView(item.id)}
            title={isCollapsed ? item.label : ""}
          >
            <span class="nav-icon">
              <i class="bi bi-{item.icon}" aria-hidden="true"></i>
            </span>
            {#if showLabels}
              <span class="nav-label">{item.label}</span>
            {/if}
            {#if item.badge && item.badge > 0}
              <span
                class="nav-badge {item.badgeColor}"
                class:mini={isCollapsed}
              >
                {item.badge}
              </span>
            {/if}
            {#if isCollapsed}
              <span class="tooltip">{item.label}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>

    <!-- System Section -->
    <div class="nav-divider"></div>
    {#if showLabels}
      <div class="nav-category">{tr("system")}</div>
    {/if}
    <ul class="nav-list">
      {#each systemItems as item (item.id)}
        <li>
          <button
            class="nav-item"
            class:active={$currentView === item.id}
            class:collapsed={isCollapsed}
            onclick={() => selectView(item.id)}
            title={isCollapsed ? item.label : ""}
          >
            <span class="nav-icon">
              <i class="bi bi-{item.icon}" aria-hidden="true"></i>
            </span>
            {#if showLabels}
              <span class="nav-label">{item.label}</span>
            {/if}
            {#if isCollapsed}
              <span class="tooltip">{item.label}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  </nav>

  <!-- Version Footer -->
  <div class="sidebar-footer">
    {#if showLabels}
      <span class="version">v1.0-beta</span>
    {:else}
      <span class="version-mini">v1</span>
    {/if}
  </div>

  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="resize-handle"
    onmousedown={startResize}
    ondblclick={resetWidth}
    role="separator"
    aria-label="Resize sidebar"
  ></div>
</aside>

<style>
  /* ========================================
     SIDEBAR CONTAINER
     ======================================== */
  .sidebar {
    position: relative;
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: linear-gradient(180deg, #ffffff 0%, #f8faf9 100%);
    border-right: 1px solid rgba(34, 197, 94, 0.15);
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
    box-shadow: 2px 0 16px rgba(34, 197, 94, 0.06);
  }

  :global(.dark) .sidebar {
    background: linear-gradient(180deg, #1f2937 0%, #111827 100%);
    border-right: 1px solid rgba(34, 197, 94, 0.2);
    box-shadow: 2px 0 16px rgba(0, 0, 0, 0.3);
  }

  .sidebar.resizing {
    transition: none;
    user-select: none;
  }

  .sidebar.collapsed {
    width: 72px !important;
  }

  /* ========================================
     HEADER
     ======================================== */
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem;
    min-height: 64px;
    border-bottom: 1px solid rgba(34, 197, 94, 0.1);
  }

  :global(.dark) .sidebar-header {
    border-bottom-color: rgba(34, 197, 94, 0.15);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
    min-width: 0;
    transition: all 0.3s ease;
  }

  .brand.collapsed {
    justify-content: center;
  }

  .logo-container {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .logo {
    width: 32px;
    height: 32px;
    filter: drop-shadow(0 2px 8px rgba(34, 197, 94, 0.4));
    transition: all 0.3s ease;
  }

  .collapsed .logo {
    width: 36px;
    height: 36px;
  }

  .brand-text {
    flex: 1;
    min-width: 0;
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateX(-8px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .brand-name {
    font-size: 1.125rem;
    font-weight: 700;
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin: 0;
    line-height: 1.2;
    white-space: nowrap;
  }

  .brand-tagline {
    font-size: 0.6875rem;
    color: #6b7280;
    font-weight: 500;
    margin: 0;
    line-height: 1.2;
    white-space: nowrap;
  }

  :global(.dark) .brand-tagline {
    color: #9ca3af;
  }

  .collapse-btn {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(34, 197, 94, 0.1);
    border: none;
    color: #22c55e;
    cursor: pointer;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .collapse-btn:hover {
    background: rgba(34, 197, 94, 0.2);
    transform: scale(1.05);
  }

  :global(.dark) .collapse-btn {
    background: rgba(34, 197, 94, 0.15);
  }

  :global(.dark) .collapse-btn:hover {
    background: rgba(34, 197, 94, 0.25);
  }

  /* ========================================
     NAVIGATION
     ======================================== */
  .sidebar-nav {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 0.75rem;
  }

  .nav-category {
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: #9ca3af;
    padding: 0.5rem 0.75rem;
    margin-top: 0.25rem;
  }

  :global(.dark) .nav-category {
    color: #6b7280;
  }

  .nav-divider {
    height: 1px;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(34, 197, 94, 0.2),
      transparent
    );
    margin: 0.5rem 0;
  }

  .nav-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.625rem 0.75rem;
    border-radius: 10px;
    background: transparent;
    border: none;
    color: #4b5563;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
  }

  :global(.dark) .nav-item {
    color: #d1d5db;
  }

  .nav-item.collapsed {
    justify-content: center;
    padding: 0.75rem;
  }

  .nav-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    font-size: 1.125rem;
    color: #6b7280;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  :global(.dark) .nav-icon {
    color: #9ca3af;
  }

  .collapsed .nav-icon {
    width: 28px;
    height: 28px;
    font-size: 1.25rem;
  }

  .nav-label {
    flex: 1;
    transition: opacity 0.2s ease;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Hover State */
  .nav-item:hover {
    background: rgba(34, 197, 94, 0.08);
    color: #22c55e;
  }

  :global(.dark) .nav-item:hover {
    background: rgba(34, 197, 94, 0.12);
    color: #4ade80;
  }

  .nav-item:hover .nav-icon {
    color: #22c55e;
    transform: scale(1.1);
  }

  :global(.dark) .nav-item:hover .nav-icon {
    color: #4ade80;
  }

  /* Active State */
  .nav-item.active {
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
    box-shadow: 0 4px 12px rgba(34, 197, 94, 0.35);
  }

  .nav-item.active .nav-icon {
    color: white;
  }

  .nav-item.active:hover {
    background: linear-gradient(135deg, #16a34a 0%, #15803d 100%);
    color: white;
  }

  /* ========================================
     BADGES
     ======================================== */
  .nav-badge {
    padding: 0.125rem 0.5rem;
    font-size: 0.6875rem;
    font-weight: 600;
    border-radius: 100px;
    margin-left: auto;
    transition: all 0.2s ease;
  }

  .nav-badge.mini {
    position: absolute;
    top: 4px;
    right: 4px;
    padding: 0;
    width: 16px;
    height: 16px;
    font-size: 0.5625rem;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 0;
  }

  .nav-badge.purple {
    background: #f3e8ff;
    color: #9333ea;
  }
  :global(.dark) .nav-badge.purple {
    background: rgba(147, 51, 234, 0.2);
    color: #c084fc;
  }

  .nav-badge.blue {
    background: #dbeafe;
    color: #2563eb;
  }
  :global(.dark) .nav-badge.blue {
    background: rgba(37, 99, 235, 0.2);
    color: #60a5fa;
  }

  .nav-badge.amber {
    background: #fef3c7;
    color: #d97706;
  }
  :global(.dark) .nav-badge.amber {
    background: rgba(217, 119, 6, 0.2);
    color: #fbbf24;
  }

  .nav-item.active .nav-badge {
    background: rgba(255, 255, 255, 0.25);
    color: white;
  }

  /* ========================================
     TOOLTIP
     ======================================== */
  .tooltip {
    position: absolute;
    left: calc(100% + 8px);
    top: 50%;
    transform: translateY(-50%);
    padding: 0.375rem 0.75rem;
    background: #1f2937;
    color: white;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 6px;
    white-space: nowrap;
    opacity: 0;
    visibility: hidden;
    transition: all 0.2s ease;
    z-index: 100;
    pointer-events: none;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  :global(.dark) .tooltip {
    background: #374151;
  }

  .nav-item.collapsed:hover .tooltip {
    opacity: 1;
    visibility: visible;
  }

  /* ========================================
     FOOTER
     ======================================== */
  .sidebar-footer {
    padding: 0.75rem;
    border-top: 1px solid rgba(34, 197, 94, 0.1);
    text-align: center;
  }

  :global(.dark) .sidebar-footer {
    border-top-color: rgba(34, 197, 94, 0.15);
  }

  .version {
    font-size: 0.6875rem;
    color: #9ca3af;
    font-weight: 500;
  }

  .version-mini {
    font-size: 0.625rem;
    color: #9ca3af;
    font-weight: 600;
  }

  /* ========================================
     RESIZE HANDLE
     ======================================== */
  .resize-handle {
    position: absolute;
    top: 0;
    right: -4px;
    width: 8px;
    height: 100%;
    cursor: ew-resize;
    z-index: 10;
    transition: background 0.2s ease;
  }

  .resize-handle:hover,
  .resizing .resize-handle {
    background: rgba(34, 197, 94, 0.15);
  }

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

  .resize-handle:hover::after,
  .resizing .resize-handle::after {
    background: rgba(34, 197, 94, 0.5);
  }

  /* ========================================
     SCROLLBAR
     ======================================== */
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

  :global(.dark) .sidebar-nav::-webkit-scrollbar-thumb:hover {
    background: rgba(34, 197, 94, 0.5);
  }
</style>
