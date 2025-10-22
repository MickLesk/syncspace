<script>
  import {
    currentView,
    currentTheme,
    sidebarCollapsed,
    currentLang,
  } from "../stores/ui";
  import { auth } from "../stores/auth";
  import { t } from "../i18n";

  $: navItems = [
    { id: "files", icon: "📁", label: t($currentLang, "files") },
    { id: "shared", icon: "🔗", label: t($currentLang, "shared") },
    { id: "favorites", icon: "⭐", label: t($currentLang, "favorites") },
    { id: "storage", icon: "📊", label: "Storage" },
    { id: "activity", icon: "🕐", label: "Activity" },
    { id: "duplicates", icon: "🔍", label: "Duplicates" },
    { id: "backup", icon: "💾", label: "Backup" },
    { id: "trash", icon: "🗑️", label: t($currentLang, "trash") },
    { id: "users", icon: "👥", label: t($currentLang, "users") },
    { id: "settings", icon: "⚙️", label: t($currentLang, "settings") },
  ];

  function selectView(viewId) {
    currentView.set(viewId);
  }

  function toggleTheme() {
    currentTheme.set($currentTheme === "light" ? "dark" : "light");
  }

  function toggleSidebar() {
    sidebarCollapsed.toggle();
  }

  function handleLogout() {
    auth.logout();
  }
</script>

<nav class="sidebar" class:collapsed={$sidebarCollapsed}>
  <!-- Sidebar Header -->
  <div class="sidebar-header">
    {#if !$sidebarCollapsed}
      <div class="brand">
        <div class="logo">S</div>
        <span class="brand-name">SyncSpace</span>
      </div>
    {:else}
      <div class="logo-collapsed">S</div>
    {/if}
    <button
      class="collapse-btn"
      on:click={toggleSidebar}
      title={$sidebarCollapsed ? "Sidebar erweitern" : "Sidebar einklappen"}
    >
      {$sidebarCollapsed ? "→" : "←"}
    </button>
  </div>

  <!-- Navigation Items -->
  <div class="nav-section">
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={$currentView === item.id}
        on:click={() => selectView(item.id)}
        type="button"
        title={$sidebarCollapsed ? item.label : ""}
      >
        <span class="icon">{item.icon}</span>
        {#if !$sidebarCollapsed}
          <span class="label">{item.label}</span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Divider -->
  <div class="divider"></div>

  <!-- Bottom Section -->
  <div class="bottom-section">
    <!-- Theme Toggle -->
    <button
      class="action-btn"
      on:click={toggleTheme}
      title={$currentTheme === "light"
        ? t($currentLang, "dark")
        : t($currentLang, "light")}
    >
      <span class="icon">{$currentTheme === "light" ? "🌙" : "☀️"}</span>
      {#if !$sidebarCollapsed}
        <span class="label"
          >{$currentTheme === "light"
            ? t($currentLang, "dark")
            : t($currentLang, "light")}</span
        >
      {/if}
    </button>

    <!-- User Section -->
    {#if !$sidebarCollapsed}
      <div class="user-section">
        <div class="user-info">
          <div class="user-avatar">
            {$auth.username?.charAt(0).toUpperCase()}
          </div>
          <div class="user-details">
            <div class="user-name">{$auth.username}</div>
            <div class="user-actions">
              <button
                class="user-action-btn"
                on:click={() => selectView("profile")}
                title="Profil"
              >
                👤
              </button>
              <button
                class="user-action-btn"
                on:click={handleLogout}
                title={t($currentLang, "logout")}
              >
                🚪
              </button>
            </div>
          </div>
        </div>
      </div>
    {:else}
      <button
        class="action-btn"
        on:click={() => selectView("profile")}
        title="Profil"
      >
        <span class="icon">👤</span>
      </button>
      <button
        class="action-btn"
        on:click={handleLogout}
        title={t($currentLang, "logout")}
      >
        <span class="icon">🚪</span>
      </button>
    {/if}
  </div>
</nav>

<style>
  .sidebar {
    width: 72px;
    min-width: 72px;
    background: var(--md-sys-color-surface);
    border-right: 1px solid var(--md-sys-color-outline);
    display: flex;
    flex-direction: column;
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
  }

  .sidebar:not(.collapsed) {
    width: 260px;
    min-width: 260px;
  }

  /* Sidebar Header */
  .sidebar-header {
    padding: 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--md-sys-color-outline);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
    flex: 1;
  }

  .logo {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: linear-gradient(
      135deg,
      var(--md-sys-color-primary),
      var(--md-sys-color-secondary)
    );
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    font-weight: 700;
    color: var(--md-sys-color-on-primary);
    box-shadow: var(--md-elevation-1);
  }

  .logo-collapsed {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: linear-gradient(
      135deg,
      var(--md-sys-color-primary),
      var(--md-sys-color-secondary)
    );
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    font-weight: 700;
    color: var(--md-sys-color-on-primary);
    box-shadow: var(--md-elevation-1);
    margin: 0 auto;
  }

  .brand-name {
    font-size: 18px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    white-space: nowrap;
  }

  .collapse-btn {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: none;
    background: var(--md-sys-color-surface-variant);
    color: var(--md-sys-color-on-surface-variant);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    flex-shrink: 0;
  }

  .collapse-btn:hover {
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
  }

  /* Navigation Section */
  .nav-section {
    flex: 1;
    padding: 8px;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .nav-section::-webkit-scrollbar {
    width: 0px;
  }

  .nav-item {
    width: 100%;
    padding: 12px;
    margin: 2px 0;
    border-radius: 24px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    transition: all 0.2s ease;
    border: none;
    background: transparent;
    display: flex;
    align-items: center;
    gap: 12px;
    text-align: left;
    white-space: nowrap;
  }

  .collapsed .nav-item {
    justify-content: center;
    padding: 12px;
  }

  .nav-item:hover {
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
  }

  .nav-item.active {
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
    font-weight: 600;
  }

  .icon {
    font-size: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .label {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Divider */
  .divider {
    height: 1px;
    background: var(--md-sys-color-outline);
    margin: 8px 16px;
  }

  /* Bottom Section */
  .bottom-section {
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .action-btn {
    width: 100%;
    padding: 12px;
    border-radius: 24px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    transition: all 0.2s ease;
    border: none;
    background: transparent;
    display: flex;
    align-items: center;
    gap: 12px;
    text-align: left;
    white-space: nowrap;
  }

  .collapsed .action-btn {
    justify-content: center;
  }

  .action-btn:hover {
    background: var(--md-sys-color-surface-variant);
  }

  /* User Section */
  .user-section {
    padding: 12px;
    border-radius: 16px;
    background: var(--md-sys-color-surface-variant);
  }

  .user-info {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .user-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
    flex-shrink: 0;
  }

  .user-details {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 4px;
  }

  .user-actions {
    display: flex;
    gap: 8px;
  }

  .user-action-btn {
    background: none;
    border: none;
    font-size: 16px;
    cursor: pointer;
    padding: 0;
    opacity: 0.7;
    transition: opacity 0.2s ease;
  }

  .user-action-btn:hover {
    opacity: 1;
  }

  /* Scrollbar */
  .nav-section::-webkit-scrollbar {
    width: 4px;
  }

  .nav-section::-webkit-scrollbar-track {
    background: transparent;
  }

  .nav-section::-webkit-scrollbar-thumb {
    background: var(--md-sys-color-outline);
    border-radius: 2px;
  }

  .nav-section::-webkit-scrollbar-thumb:hover {
    background: var(--md-sys-color-on-surface-variant);
  }
</style>
