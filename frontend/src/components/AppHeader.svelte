<script>
  import { createEventDispatcher } from "svelte";
  import { currentTheme, currentLang, currentView } from "../stores/ui";
  import { auth } from "../stores/auth";
  import { t } from "../i18n.js";
  import AdvancedSearchModal from "./AdvancedSearchModal.svelte";
  import Modal from "./ui/Modal.svelte";

  const dispatch = createEventDispatcher();

  // Map view IDs to display names with icons
  const viewNames = {
    files: { name: "Files", icon: "bi-folder-fill" },
    shared: { name: "Shared", icon: "bi-people-fill" },
    favorites: { name: "Favorites", icon: "bi-star-fill" },
    trash: { name: "Trash", icon: "bi-trash3-fill" },
    users: { name: "Users", icon: "bi-person-fill" },
    settings: { name: "Settings", icon: "bi-gear-fill" },
    profile: { name: "Profile", icon: "bi-person-circle" },
    storage: { name: "Storage", icon: "bi-hdd-fill" },
    activity: { name: "Activity", icon: "bi-clock-history" },
    duplicates: { name: "Duplicates", icon: "bi-files" },
    backup: { name: "Backup", icon: "bi-cloud-arrow-down-fill" },
    gallery: { name: "Gallery", icon: "bi-images" },
  };

  $: currentViewName = viewNames[$currentView]?.name || "SyncSpace";
  $: currentViewIcon =
    viewNames[$currentView]?.icon || "bi-cloud-arrow-up-fill";

  let searchQuery = "";
  let showSearchModal = false;
  let showAdvancedSearch = false;

  $: isDark = $currentTheme === "dark";
  $: unreadCount = 3;
  $: userInitials = $auth.user?.username
    ? $auth.user.username.substring(0, 2).toUpperCase()
    : "AD";

  function toggleTheme() {
    const newTheme = isDark ? "light" : "dark";
    currentTheme.set(newTheme);
    document.documentElement.setAttribute(
      "data-theme",
      newTheme === "dark" ? "syncspace-dark" : "syncspace"
    );
  }

  function handleLogout() {
    auth.logout();
    window.location.href = "/";
  }

  function handleSearch(e) {
    e.preventDefault();
    if (searchQuery.trim()) {
      dispatch("search", { query: searchQuery });
      showSearchModal = false;
    }
  }

  function handleAdvancedSearch(e) {
    const { query, filters, sortBy, sortOrder } = e.detail;
    dispatch("advancedSearch", { query, filters, sortBy, sortOrder });
  }

  function handleKeydown(e) {
    // Ctrl+K for Quick Search
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      showSearchModal = true;
    }
    // Ctrl+Shift+F for Advanced Search
    if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "F") {
      e.preventDefault();
      showAdvancedSearch = true;
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- Material 3 Expressive Header with Glassmorphism -->
<header class="app-header">
  <div class="header-container">
    <!-- Left: Logo & Current View Name -->
    <div class="header-left">
      <div class="brand-logo">
        <div class="logo-icon">
          <i class={currentViewIcon}></i>
        </div>
        <span class="brand-name">{currentViewName}</span>
      </div>
    </div>

    <!-- Center: Search Bar -->
    <div class="header-center">
      <div class="search-container">
        <button class="search-button" on:click={() => (showSearchModal = true)}>
          <i class="bi bi-search search-icon"></i>
          <span class="search-text">{t($currentLang, "searchPlaceholder")}</span
          >
          <kbd class="search-kbd">Ctrl K</kbd>
        </button>
        <button
          class="advanced-button"
          on:click={() => (showAdvancedSearch = true)}
          title="{t($currentLang, 'advancedSearch')} (Ctrl+Shift+F)"
        >
          <i class="bi bi-funnel"></i>
        </button>
      </div>
    </div>

    <!-- Right: Actions -->
    <div class="header-right">
      <!-- Theme Toggle -->
      <button
        class="action-button theme-toggle"
        on:click={toggleTheme}
        aria-label="Toggle theme"
      >
        <div class="theme-icon-wrapper">
          {#if isDark}
            <i class="bi bi-sun-fill theme-icon"></i>
          {:else}
            <i class="bi bi-moon-fill theme-icon"></i>
          {/if}
        </div>
      </button>

      <!-- Notifications -->
      <div class="dropdown dropdown-end">
        <button class="action-button notification-button" tabindex="0">
          <div class="notification-icon-wrapper">
            <i class="bi bi-bell"></i>
            {#if unreadCount > 0}
              <span class="notification-badge">{unreadCount}</span>
            {/if}
          </div>
        </button>
        <div class="dropdown-content notification-dropdown">
          <div class="notification-header">
            <h3 class="notification-title">
              {t($currentLang, "notifications")}
            </h3>
          </div>
          <ul class="notification-list">
            <li>
              <button class="notification-item">
                <div class="notification-icon success">
                  <i class="bi bi-check-circle"></i>
                </div>
                <div class="notification-content">
                  <p class="notification-text">File uploaded successfully</p>
                  <p class="notification-time">2 minutes ago</p>
                </div>
              </button>
            </li>
            <li>
              <button class="notification-item">
                <div class="notification-icon info">
                  <i class="bi bi-share"></i>
                </div>
                <div class="notification-content">
                  <p class="notification-text">New share request</p>
                  <p class="notification-time">1 hour ago</p>
                </div>
              </button>
            </li>
            <li>
              <button class="notification-item">
                <div class="notification-icon warning">
                  <i class="bi bi-exclamation-triangle"></i>
                </div>
                <div class="notification-content">
                  <p class="notification-text">Storage almost full</p>
                  <p class="notification-time">85% used • 3 hours ago</p>
                </div>
              </button>
            </li>
          </ul>
          <div class="notification-footer">
            <button class="btn btn-sm btn-ghost w-full">View All</button>
          </div>
        </div>
      </div>

      <!-- User Menu -->
      <div class="dropdown dropdown-end">
        <button class="user-avatar-button" tabindex="0">
          <div class="user-avatar">
            <span class="user-initials">{userInitials}</span>
            <div class="user-status-indicator"></div>
          </div>
        </button>
        <div class="dropdown-content user-dropdown">
          <div class="user-dropdown-header">
            <div class="user-avatar-large">
              <span class="user-initials-large">{userInitials}</span>
            </div>
            <div class="user-info">
              <p class="user-name">{$auth.user?.username || "Admin"}</p>
              <p class="user-email">
                {$auth.user?.email || "admin@syncspace.local"}
              </p>
            </div>
          </div>
          <ul class="user-menu">
            <li>
              <button
                class="user-menu-item"
                on:click={() => dispatch("navigate", "profile")}
              >
                <i class="bi bi-person"></i>
                <span>{t($currentLang, "profile")}</span>
              </button>
            </li>
            <li>
              <button
                class="user-menu-item"
                on:click={() => dispatch("navigate", "settings")}
              >
                <i class="bi bi-gear"></i>
                <span>{t($currentLang, "settings")}</span>
              </button>
            </li>
            <li class="divider"></li>
            <li>
              <button class="user-menu-item logout" on:click={handleLogout}>
                <i class="bi bi-box-arrow-right"></i>
                <span>{t($currentLang, "logout")}</span>
              </button>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</header>

<!-- Material 3 Quick Search Modal -->
<Modal
  visible={showSearchModal}
  title="Quick Search"
  icon="search"
  size="md"
  variant="primary"
  on:close={() => (showSearchModal = false)}
>
  <form on:submit={handleSearch}>
    <div class="form-control">
      <label class="label">
        <span class="label-text font-medium">Search Query</span>
        <span class="label-text-alt text-xs opacity-60">
          <kbd class="kbd kbd-xs">Ctrl</kbd> + <kbd class="kbd kbd-xs">K</kbd>
        </span>
      </label>
      <div class="input-group flex">
        <span
          class="px-4 bg-base-200 flex items-center rounded-l-xl border border-r-0 border-base-300"
        >
          <i class="bi bi-search text-lg"></i>
        </span>
        <input
          type="text"
          placeholder="Search for files, folders..."
          class="input input-bordered flex-1 rounded-l-none rounded-r-xl focus:ring-2 focus:ring-primary/50"
          bind:value={searchQuery}
          autofocus
        />
      </div>
      <label class="label">
        <span class="label-text-alt text-xs opacity-60">
          💡 Use <strong>Ctrl+Shift+F</strong> for advanced search with filters
        </span>
      </label>
    </div>
  </form>

  <div slot="actions">
    <button
      type="button"
      class="btn btn-ghost rounded-xl"
      on:click={() => (showSearchModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      type="button"
      class="btn btn-primary rounded-xl gap-2"
      on:click={handleSearch}
    >
      <i class="bi bi-search"></i>
      Search
    </button>
  </div>
</Modal>

<!-- Advanced Search Modal -->
<AdvancedSearchModal
  visible={showAdvancedSearch}
  on:search={handleAdvancedSearch}
  on:close={() => (showAdvancedSearch = false)}
/>

<style>
  /* Material 3 Expressive Header */
  .app-header {
    position: sticky;
    top: 0;
    z-index: 1000;
    height: 64px;
    /* Light Mode: Clean white with gradient border */
    background: hsl(var(--b1));
    border-bottom: 3px solid transparent;
    border-image: linear-gradient(90deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%)
      1;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
    color: hsl(var(--bc));
  }

  /* Dark Mode: Dark background with gradient border */
  [data-theme="dark"] .app-header,
  [data-theme="syncspace-dark"] .app-header {
    background: hsl(var(--b1));
    border-image: linear-gradient(90deg, #4f46e5 0%, #7c3aed 50%, #c026d3 100%)
      1;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .header-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    padding: 0 1.5rem;
    max-width: 100%;
    margin: 0 auto;
  }

  /* Left Section - Brand */
  .header-left {
    display: flex;
    align-items: center;
    min-width: 200px;
  }

  .brand-logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
    transition: transform 0.2s ease;
  }

  .brand-logo:hover {
    transform: scale(1.05);
  }

  .logo-icon {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    color: white;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .brand-name {
    font-size: 1.25rem;
    font-weight: 700;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    letter-spacing: -0.02em;
  }

  /* Center Section - Search */
  .header-center {
    flex: 1;
    display: flex;
    justify-content: center;
    max-width: 600px;
    margin: 0 2rem;
  }

  .search-container {
    display: flex;
    width: 100%;
    gap: 0.5rem;
  }

  .search-button {
    flex: 1;
    height: 44px;
    background: hsl(var(--b2));
    border: 1px solid hsl(var(--bc) / 0.2);
    border-radius: 12px;
    padding: 0 1rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: hsl(var(--bc));
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .search-button:hover {
    background: hsl(var(--b3));
    border-color: hsl(var(--bc) / 0.3);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .search-icon {
    font-size: 1.125rem;
    color: hsl(var(--bc) / 0.7);
  }

  .search-text {
    flex: 1;
    text-align: left;
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.6);
  }

  .search-kbd {
    padding: 0.25rem 0.5rem;
    background: hsl(var(--b3));
    border: 1px solid hsl(var(--bc) / 0.2);
    border-radius: 6px;
    font-size: 0.75rem;
    font-family: ui-monospace, monospace;
    color: hsl(var(--bc) / 0.7);
  }

  .advanced-button {
    width: 44px;
    height: 44px;
    background: hsl(var(--b2));
    border: 1px solid hsl(var(--bc) / 0.2);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: hsl(var(--bc));
    font-size: 1.125rem;
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .advanced-button:hover {
    background: hsl(var(--b3));
    border-color: hsl(var(--bc) / 0.3);
    transform: translateY(-1px);
  }

  /* Right Section - Actions */
  .header-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 200px;
    justify-content: flex-end;
  }

  .action-button {
    width: 44px;
    height: 44px;
    background: hsl(var(--b2));
    border: 1px solid hsl(var(--bc) / 0.2);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: hsl(var(--bc));
    font-size: 1.125rem;
    transition: all 0.2s ease;
    cursor: pointer;
    position: relative;
  }

  .action-button:hover {
    background: hsl(var(--b3));
    border-color: hsl(var(--bc) / 0.3);
    transform: translateY(-1px);
  }

  /* Theme Toggle Animation */
  .theme-icon-wrapper {
    position: relative;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .theme-icon {
    animation: themeRotate 0.5s ease;
  }

  @keyframes themeRotate {
    from {
      transform: rotate(0deg) scale(0.8);
      opacity: 0;
    }
    to {
      transform: rotate(360deg) scale(1);
      opacity: 1;
    }
  }

  /* Notification Badge */
  .notification-icon-wrapper {
    position: relative;
  }

  .notification-badge {
    position: absolute;
    top: -6px;
    right: -6px;
    width: 18px;
    height: 18px;
    background: hsl(var(--er));
    color: white;
    border-radius: 50%;
    font-size: 0.625rem;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid hsl(var(--p));
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.1);
    }
  }

  /* User Avatar */
  .user-avatar-button {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
  }

  .user-avatar {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1, #8b5cf6, #d946ef);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    transition: all 0.2s ease;
    border: 2px solid hsl(var(--bc) / 0.1);
    box-shadow: 0 2px 8px rgba(99, 102, 241, 0.2);
  }

  .user-avatar-button:hover .user-avatar {
    transform: scale(1.05);
    border-color: hsl(var(--bc) / 0.2);
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .user-initials {
    font-size: 0.875rem;
    font-weight: 700;
    color: white;
  }

  .user-status-indicator {
    position: absolute;
    bottom: -2px;
    right: -2px;
    width: 12px;
    height: 12px;
    background: hsl(var(--su));
    border: 2px solid hsl(var(--p));
    border-radius: 50%;
  }

  /* Dropdown Styles */
  .dropdown-content {
    animation: slideDown 0.2s ease;
    margin-top: 0.75rem;
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

  /* Notification Dropdown */
  .notification-dropdown {
    width: 360px;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--b3));
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    overflow: hidden;
  }

  .notification-header {
    padding: 1rem 1.25rem;
    border-bottom: 1px solid hsl(var(--b3));
  }

  .notification-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: hsl(var(--bc));
  }

  .notification-list {
    list-style: none;
    padding: 0;
    margin: 0;
    max-height: 400px;
    overflow-y: auto;
  }

  .notification-item {
    all: unset;
    display: flex;
    align-items: start;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    width: 100%;
    cursor: pointer;
    transition: background 0.2s ease;
    border-bottom: 1px solid hsl(var(--b3));
  }

  .notification-item:hover {
    background: hsl(var(--b2));
  }

  .notification-item:last-child {
    border-bottom: none;
  }

  .notification-icon {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    font-size: 1rem;
  }

  .notification-icon.success {
    background: hsl(var(--su) / 0.15);
    color: hsl(var(--su));
  }

  .notification-icon.info {
    background: hsl(var(--in) / 0.15);
    color: hsl(var(--in));
  }

  .notification-icon.warning {
    background: hsl(var(--wa) / 0.15);
    color: hsl(var(--wa));
  }

  .notification-content {
    flex: 1;
    min-width: 0;
  }

  .notification-text {
    font-size: 0.875rem;
    font-weight: 500;
    color: hsl(var(--bc));
    margin: 0;
  }

  .notification-time {
    font-size: 0.75rem;
    color: hsl(var(--bc) / 0.6);
    margin: 0.25rem 0 0 0;
  }

  .notification-footer {
    padding: 0.75rem;
    border-top: 1px solid hsl(var(--b3));
  }

  /* User Dropdown */
  .user-dropdown {
    width: 280px;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--b3));
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    overflow: hidden;
  }

  .user-dropdown-header {
    padding: 1.25rem;
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.08),
      rgba(139, 92, 246, 0.08),
      rgba(217, 70, 239, 0.08)
    );
    border-bottom: 1px solid hsl(var(--b3));
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .user-avatar-large {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    background: linear-gradient(135deg, #6366f1, #8b5cf6, #d946ef);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid hsl(var(--b1));
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.2);
  }

  .user-initials-large {
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
  }

  .user-info {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 0.9375rem;
    font-weight: 600;
    color: hsl(var(--bc));
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-email {
    font-size: 0.75rem;
    color: hsl(var(--bc) / 0.6);
    margin: 0.25rem 0 0 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-menu {
    list-style: none;
    padding: 0.5rem;
    margin: 0;
  }

  .user-menu-item {
    all: unset;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    width: 100%;
    cursor: pointer;
    border-radius: 10px;
    transition: all 0.2s ease;
    font-size: 0.875rem;
    color: hsl(var(--bc));
  }

  .user-menu-item:hover {
    background: hsl(var(--b2));
  }

  .user-menu-item i {
    font-size: 1.125rem;
    width: 20px;
    text-align: center;
  }

  .user-menu-item.logout {
    color: hsl(var(--er));
  }

  .user-menu-item.logout:hover {
    background: hsl(var(--er) / 0.1);
  }

  .divider {
    height: 1px;
    background: hsl(var(--b3));
    margin: 0.5rem 0;
  }

  /* Responsive */
  @media (max-width: 1024px) {
    .brand-name {
      display: none;
    }

    .header-center {
      margin: 0 1rem;
    }
  }

  @media (max-width: 768px) {
    .search-text {
      display: none;
    }

    .search-kbd {
      display: none;
    }

    .header-center {
      max-width: 400px;
    }
  }
</style>
