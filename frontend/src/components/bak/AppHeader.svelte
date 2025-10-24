<!--
  AppHeader.svelte - Modern Navigation Header using DaisyUI
  
  A comprehensive header component with search, theme toggle, notifications, and user menu.
  
  Features:
  - Global search with keyboard shortcut (Ctrl+K / Cmd+K)
  - Theme switcher (light/dark)
  - Notifications with badge
  - User dropdown menu
  - Mobile responsive with drawer toggle
  - Glass effect option
  - Breadcrumb navigation
  
  Props:
  - glass: boolean - Enable glassmorphism effect (default: false)
  - showSearch: boolean - Show search bar (default: true)
  - showNotifications: boolean - Show notifications (default: true)
  - showUserMenu: boolean - Show user menu (default: true)
  
  Events:
  - on:search - Fired when search is performed
  - on:themeToggle - Fired when theme is toggled
  - on:logout - Fired when user logs out
-->

<script>
  import { createEventDispatcher } from "svelte";
  import { currentTheme, sidebarCollapsed } from "../stores/ui";
  import { auth } from "../stores/auth";
  import ButtonV2 from "./ui/ButtonV2.svelte";
  import InputV2 from "./ui/InputV2.svelte";

  export let glass = true;
  export let showSearch = true;
  export let showNotifications = true;
  export let showUserMenu = true;

  const dispatch = createEventDispatcher();

  let searchQuery = "";
  let showSearchModal = false;
  let showUserDropdown = false;
  let showNotificationsDropdown = false;

  // Mock notifications
  let notifications = [
    {
      id: 1,
      title: "File uploaded",
      message: "document.pdf was uploaded successfully",
      time: "2 min ago",
      unread: true,
      icon: "file-earmark-arrow-up",
      type: "success",
    },
    {
      id: 2,
      title: "Share request",
      message: "John Doe wants to share a folder with you",
      time: "1 hour ago",
      unread: true,
      icon: "share",
      type: "info",
    },
    {
      id: 3,
      title: "Storage warning",
      message: "You're using 85% of your storage",
      time: "3 hours ago",
      unread: false,
      icon: "exclamation-triangle",
      type: "warning",
    },
  ];

  $: unreadCount = notifications.filter((n) => n.unread).length;
  $: isDark = $currentTheme === "dark";

  function toggleTheme() {
    const newTheme = isDark ? "light" : "dark";
    currentTheme.set(newTheme);
    document.documentElement.setAttribute(
      "data-theme",
      newTheme === "dark" ? "syncspace-dark" : "syncspace"
    );
    dispatch("themeToggle", { theme: newTheme });
  }

  function toggleSidebar() {
    sidebarCollapsed.toggle();
  }

  function handleSearch() {
    if (searchQuery.trim()) {
      dispatch("search", { query: searchQuery });
      showSearchModal = false;
    }
  }

  function handleLogout() {
    auth.logout();
    dispatch("logout");
  }

  function markAsRead(notificationId) {
    notifications = notifications.map((n) =>
      n.id === notificationId ? { ...n, unread: false } : n
    );
  }

  function markAllAsRead() {
    notifications = notifications.map((n) => ({ ...n, unread: false }));
  }

  // Keyboard shortcut for search (Ctrl/Cmd + K)
  function handleKeydown(e) {
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      showSearchModal = !showSearchModal;
    }
  }

  // Close dropdowns when clicking outside
  function handleClickOutside(e) {
    if (!e.target.closest(".user-menu")) {
      showUserDropdown = false;
    }
    if (!e.target.closest(".notifications-menu")) {
      showNotificationsDropdown = false;
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} on:click={handleClickOutside} />

<header class="app-header" class:glass-header={glass}>
  <div class="navbar bg-base-100" class:glass>
    <!-- Left: Menu Toggle + Logo -->
    <div class="navbar-start">
      <button
        class="btn btn-ghost btn-circle lg:hidden"
        on:click={toggleSidebar}
        aria-label="Toggle sidebar"
      >
        <i class="bi bi-list text-2xl"></i>
      </button>

      <a
        href="/"
        class="btn btn-ghost normal-case text-xl hidden lg:flex gap-2"
      >
        <div class="logo-icon">
          <svg
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
        </div>
        <span class="font-bold brand-gradient">SyncSpace</span>
      </a>
    </div>

    <!-- Center: Search -->
    {#if showSearch}
      <div class="navbar-center hidden lg:flex flex-1 max-w-xl px-4">
        <div class="form-control w-full relative">
          <button
            class="search-trigger"
            on:click={() => (showSearchModal = true)}
          >
            <i class="bi bi-search"></i>
            <span>Search files...</span>
            <kbd class="kbd kbd-sm">⌘K</kbd>
          </button>
        </div>
      </div>
    {/if}

    <!-- Right: Actions -->
    <div class="navbar-end gap-2">
      <!-- Mobile Search -->
      {#if showSearch}
        <button
          class="btn btn-ghost btn-circle lg:hidden"
          on:click={() => (showSearchModal = true)}
          aria-label="Search"
        >
          <i class="bi bi-search text-xl"></i>
        </button>
      {/if}

      <!-- Theme Toggle -->
      <div
        class="tooltip tooltip-bottom"
        data-tip={isDark ? "Light mode" : "Dark mode"}
      >
        <button
          class="btn btn-ghost btn-circle swap swap-rotate"
          on:click={toggleTheme}
          aria-label="Toggle theme"
        >
          {#if isDark}
            <i class="bi bi-sun-fill text-xl swap-on"></i>
          {:else}
            <i class="bi bi-moon-fill text-xl swap-off"></i>
          {/if}
        </button>
      </div>

      <!-- Notifications -->
      {#if showNotifications}
        <div class="dropdown dropdown-end notifications-menu">
          <button
            class="btn btn-ghost btn-circle indicator"
            on:click={() =>
              (showNotificationsDropdown = !showNotificationsDropdown)}
            aria-label="Notifications"
          >
            <i class="bi bi-bell text-xl"></i>
            {#if unreadCount > 0}
              <span class="indicator-item badge badge-sm badge-error">
                {unreadCount}
              </span>
            {/if}
          </button>

          {#if showNotificationsDropdown}
            <div
              class="dropdown-content z-50 card card-compact w-80 sm:w-96 p-0 shadow-2xl bg-base-100 border border-base-300 mt-3"
            >
              <div class="card-body p-0">
                <div
                  class="flex items-center justify-between p-4 border-b border-base-300"
                >
                  <h3 class="font-bold text-lg">Notifications</h3>
                  {#if unreadCount > 0}
                    <button
                      class="btn btn-ghost btn-xs"
                      on:click={markAllAsRead}
                    >
                      Mark all read
                    </button>
                  {/if}
                </div>

                <ul class="menu p-0 max-h-96 overflow-y-auto">
                  {#each notifications as notification}
                    <li>
                      <button
                        class="notification-item"
                        class:unread={notification.unread}
                        on:click={() => markAsRead(notification.id)}
                      >
                        <div class="notification-icon {notification.type}">
                          <i class="bi bi-{notification.icon}"></i>
                        </div>
                        <div class="flex-1 text-left">
                          <div class="font-semibold">{notification.title}</div>
                          <div class="text-sm opacity-70">
                            {notification.message}
                          </div>
                          <div class="text-xs opacity-50 mt-1">
                            {notification.time}
                          </div>
                        </div>
                        {#if notification.unread}
                          <div class="unread-dot"></div>
                        {/if}
                      </button>
                    </li>
                  {/each}
                </ul>

                <div class="p-4 border-t border-base-300">
                  <a
                    href="/notifications"
                    class="btn btn-ghost btn-sm btn-block"
                  >
                    View all notifications
                  </a>
                </div>
              </div>
            </div>
          {/if}
        </div>
      {/if}

      <!-- User Menu -->
      {#if showUserMenu}
        <div class="dropdown dropdown-end user-menu">
          <button
            class="btn btn-ghost btn-circle avatar"
            on:click={() => (showUserDropdown = !showUserDropdown)}
            aria-label="User menu"
          >
            <div
              class="w-10 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2"
            >
              <img
                src={$auth.user?.avatar ||
                  "https://ui-avatars.com/api/?name=User&background=667eea&color=fff"}
                alt="User avatar"
              />
            </div>
          </button>

          {#if showUserDropdown}
            <div
              class="dropdown-content z-50 card card-compact w-64 p-0 shadow-2xl bg-base-100 border border-base-300 mt-3"
            >
              <div class="card-body p-0">
                <!-- User Info -->
                <div class="p-4 border-b border-base-300">
                  <div class="font-bold">{$auth.user?.name || "User"}</div>
                  <div class="text-sm opacity-70">
                    {$auth.user?.email || "user@example.com"}
                  </div>
                </div>

                <!-- Menu Items -->
                <ul class="menu p-2">
                  <li>
                    <a href="/profile">
                      <i class="bi bi-person"></i>
                      Profile
                    </a>
                  </li>
                  <li>
                    <a href="/settings">
                      <i class="bi bi-gear"></i>
                      Settings
                    </a>
                  </li>
                  <li>
                    <a href="/storage">
                      <i class="bi bi-pie-chart"></i>
                      Storage
                    </a>
                  </li>
                  <li>
                    <a href="/help">
                      <i class="bi bi-question-circle"></i>
                      Help & Support
                    </a>
                  </li>
                </ul>

                <!-- Logout -->
                <div class="p-2 border-t border-base-300">
                  <button
                    class="btn btn-ghost btn-sm btn-block justify-start text-error"
                    on:click={handleLogout}
                  >
                    <i class="bi bi-box-arrow-right"></i>
                    Logout
                  </button>
                </div>
              </div>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</header>

<!-- Search Modal -->
{#if showSearchModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      on:click={() => (showSearchModal = false)}
    ></div>
    <div class="modal-box w-11/12 max-w-2xl">
      <h3 class="font-bold text-lg mb-4">Search Files</h3>
      <form on:submit|preventDefault={handleSearch}>
        <InputV2
          bind:value={searchQuery}
          placeholder="Type to search..."
          iconLeft="search"
          size="lg"
          autofocus
        />
        <div class="modal-action">
          <ButtonV2 variant="ghost" on:click={() => (showSearchModal = false)}>
            Cancel
          </ButtonV2>
          <ButtonV2 variant="primary" iconLeft="search" type="submit">
            Search
          </ButtonV2>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .app-header {
    position: sticky;
    top: 0;
    z-index: var(--z-header);
    border-bottom: 1px solid var(--color-outline);
    background: var(--color-surface);
    transition: all var(--duration-200) var(--ease-standard);
  }

  .glass-header {
    background: var(--glass-background);
    backdrop-filter: blur(var(--glass-blur));
    border-bottom: 1px solid var(--glass-border);
  }

  .navbar {
    min-height: 4rem;
    padding: 0 1rem;
  }

  .navbar.glass {
    background: transparent;
  }

  /* Logo */
  .logo-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .brand-gradient {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  /* Search Trigger */
  .search-trigger {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.5rem 1rem;
    background: var(--color-surface-container);
    border: 1px solid var(--color-outline);
    border-radius: var(--radius-full);
    cursor: pointer;
    transition: all var(--duration-200) var(--ease-standard);
  }

  .search-trigger:hover {
    background: var(--color-surface-container-high);
    border-color: var(--color-primary);
    box-shadow: var(--shadow-md);
  }

  .search-trigger i {
    font-size: 1.25rem;
    opacity: 0.7;
  }

  .search-trigger span {
    flex: 1;
    text-align: left;
    opacity: 0.6;
  }

  .kbd {
    opacity: 0.5;
  }

  /* Notification Item */
  .notification-item {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
    width: 100%;
    text-align: left;
    transition: background var(--duration-200) var(--ease-standard);
    position: relative;
  }

  .notification-item:hover {
    background: var(--color-surface-container-high);
  }

  .notification-item.unread {
    background: var(--color-surface-container);
  }

  .notification-icon {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .notification-icon.success {
    background: rgba(34, 197, 94, 0.1);
    color: rgb(34, 197, 94);
  }

  .notification-icon.info {
    background: rgba(59, 130, 246, 0.1);
    color: rgb(59, 130, 246);
  }

  .notification-icon.warning {
    background: rgba(245, 158, 11, 0.1);
    color: rgb(245, 158, 11);
  }

  .unread-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--color-primary);
    flex-shrink: 0;
    margin-top: 0.5rem;
  }

  /* Avatar Ring */
  .avatar .ring {
    transition: all var(--duration-200) var(--ease-standard);
  }

  .avatar:hover .ring {
    ring-color: var(--color-secondary);
    transform: scale(1.05);
  }

  /* Dropdown Content */
  .dropdown-content {
    animation: slideDown var(--duration-200) var(--ease-emphasized);
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Modal Backdrop */
  .modal-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
  }

  /* Responsive */
  @media (max-width: 1024px) {
    .navbar-center {
      display: none;
    }
  }
</style>
