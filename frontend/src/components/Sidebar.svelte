<script>
  import {
    currentView,
    currentTheme,
    sidebarCollapsed,
    currentLang,
  } from "../stores/ui";
  import { auth } from "../stores/auth";
  import { t } from "../i18n";
  import Icon from "./ui/Icon.svelte";

  // Navigation items mit modernen Bootstrap Icons
  $: navItems = [
    { id: "files", icon: "folder", label: t($currentLang, "files"), category: "main" },
    { id: "shared", icon: "share", label: t($currentLang, "shared"), category: "main" },
    { id: "favorites", icon: "star-fill", label: t($currentLang, "favorites"), category: "main" },
    { id: "storage", icon: "pie-chart", label: "Storage", category: "tools" },
    { id: "activity", icon: "clock-history", label: "Activity", category: "tools" },
    { id: "duplicates", icon: "files", label: "Duplicates", category: "tools" },
    { id: "backup", icon: "cloud-arrow-up", label: "Backup", category: "tools" },
    { id: "trash", icon: "trash", label: t($currentLang, "trash"), category: "system" },
    { id: "users", icon: "people", label: t($currentLang, "users"), category: "system" },
    { id: "settings", icon: "gear", label: t($currentLang, "settings"), category: "system" },
  ];

  // Gruppiere Items nach Kategorie
  $: mainItems = navItems.filter(item => item.category === "main");
  $: toolsItems = navItems.filter(item => item.category === "tools");
  $: systemItems = navItems.filter(item => item.category === "system");

  let hoveredItem = null;

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
  <!-- Modern Header with Glassmorphism -->
  <div class="sidebar-header">
    <div class="brand-container">
      {#if !$sidebarCollapsed}
        <div class="brand">
          <div class="logo">
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M13 2L3 14h8l-1 8 10-12h-8l1-8z" fill="currentColor"/>
            </svg>
          </div>
          <div class="brand-text">
            <span class="brand-name">SyncSpace</span>
            <span class="brand-tagline">Cloud Storage</span>
          </div>
        </div>
      {:else}
        <div class="logo logo-collapsed">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M13 2L3 14h8l-1 8 10-12h-8l1-8z" fill="currentColor"/>
          </svg>
        </div>
      {/if}
    </div>
    
    <button
      class="toggle-btn"
      onclick={toggleSidebar}
      title={$sidebarCollapsed ? "Expand sidebar" : "Collapse sidebar"}
    >
      <Icon name={$sidebarCollapsed ? "chevron-right" : "chevron-left"} size={20} />
    </button>
  </div>

  <!-- Navigation mit Kategorien -->
  <div class="nav-container">
    <!-- Main Section -->
    {#if !$sidebarCollapsed}
      <div class="nav-category-label">Main</div>
    {/if}
    <div class="nav-section">
      {#each mainItems as item}
        <button
          class="nav-item"
          class:active={$currentView === item.id}
          class:hovered={hoveredItem === item.id}
          onclick={() => selectView(item.id)}
          onmouseenter={() => hoveredItem = item.id}
          onmouseleave={() => hoveredItem = null}
          type="button"
          title={$sidebarCollapsed ? item.label : ""}
        >
          <div class="nav-item-icon">
            <Icon name={item.icon} size={20} />
          </div>
          {#if !$sidebarCollapsed}
            <span class="nav-item-label">{item.label}</span>
            {#if $currentView === item.id}
              <div class="active-indicator"></div>
            {/if}
          {/if}
        </button>
      {/each}
    </div>

    <!-- Tools Section -->
    {#if !$sidebarCollapsed}
      <div class="nav-category-label">Tools</div>
    {/if}
    <div class="nav-section">
      {#each toolsItems as item}
        <button
          class="nav-item"
          class:active={$currentView === item.id}
          class:hovered={hoveredItem === item.id}
          onclick={() => selectView(item.id)}
          onmouseenter={() => hoveredItem = item.id}
          onmouseleave={() => hoveredItem = null}
          type="button"
          title={$sidebarCollapsed ? item.label : ""}
        >
          <div class="nav-item-icon">
            <Icon name={item.icon} size={20} />
          </div>
          {#if !$sidebarCollapsed}
            <span class="nav-item-label">{item.label}</span>
            {#if $currentView === item.id}
              <div class="active-indicator"></div>
            {/if}
          {/if}
        </button>
      {/each}
    </div>

    <!-- System Section -->
    {#if !$sidebarCollapsed}
      <div class="nav-category-label">System</div>
    {/if}
    <div class="nav-section">
      {#each systemItems as item}
        <button
          class="nav-item"
          class:active={$currentView === item.id}
          class:hovered={hoveredItem === item.id}
          onclick={() => selectView(item.id)}
          onmouseenter={() => hoveredItem = item.id}
          onmouseleave={() => hoveredItem = null}
          type="button"
          title={$sidebarCollapsed ? item.label : ""}
        >
          <div class="nav-item-icon">
            <Icon name={item.icon} size={20} />
          </div>
          {#if !$sidebarCollapsed}
            <span class="nav-item-label">{item.label}</span>
            {#if $currentView === item.id}
              <div class="active-indicator"></div>
            {/if}
          {/if}
        </button>
      {/each}
    </div>
  </div>

  <!-- Modern Footer -->
  <div class="sidebar-footer">
    <!-- Theme Toggle mit Animation -->
    <button
      class="action-btn theme-toggle"
      onclick={toggleTheme}
      title={$currentTheme === "light" ? "Switch to dark mode" : "Switch to light mode"}
    >
      <div class="theme-toggle-icon">
        <Icon name={$currentTheme === "light" ? "moon-stars-fill" : "sun-fill"} size={20} />
      </div>
      {#if !$sidebarCollapsed}
        <span class="action-label">
          {$currentTheme === "light" ? "Dark Mode" : "Light Mode"}
        </span>
      {/if}
    </button>

    <!-- User Profile -->
    {#if !$sidebarCollapsed}
      <div class="user-card">
        <div class="user-avatar">
          <span class="avatar-text">{$auth.username?.charAt(0).toUpperCase()}</span>
          <div class="status-indicator"></div>
        </div>
        <div class="user-info">
          <div class="user-name">{$auth.username}</div>
          <div class="user-role">Administrator</div>
        </div>
        <div class="user-actions">
          <button
            class="icon-btn"
            onclick={() => selectView("profile")}
            title="Profile"
          >
            <Icon name="person" size={18} />
          </button>
          <button
            class="icon-btn logout-btn"
            onclick={handleLogout}
            title={t($currentLang, "logout")}
          >
            <Icon name="box-arrow-right" size={18} />
          </button>
        </div>
      </div>
    {:else}
      <button
        class="action-btn"
        onclick={() => selectView("profile")}
        title="Profile"
      >
        <div class="user-avatar-collapsed">
          <span>{$auth.username?.charAt(0).toUpperCase()}</span>
        </div>
      </button>
      <button
        class="action-btn logout-btn-collapsed"
        onclick={handleLogout}
        title={t($currentLang, "logout")}
      >
        <Icon name="box-arrow-right" size={20} />
      </button>
    {/if}
  </div>
</nav>

<style>
  .sidebar {
    width: 80px;
    min-width: 80px;
    height: 100vh;
    background: linear-gradient(
      180deg,
      rgba(255, 255, 255, 0.95) 0%,
      rgba(250, 250, 252, 0.98) 100%
    );
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-right: 1px solid rgba(0, 0, 0, 0.06);
    display: flex;
    flex-direction: column;
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    box-shadow: 
      0 8px 32px rgba(0, 0, 0, 0.04),
      0 1px 2px rgba(0, 0, 0, 0.02);
    z-index: 100;
  }

  :global([data-theme="dark"]) .sidebar {
    background: linear-gradient(
      180deg,
      rgba(20, 20, 28, 0.95) 0%,
      rgba(15, 15, 22, 0.98) 100%
    );
    border-right: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: 
      0 8px 32px rgba(0, 0, 0, 0.3),
      0 1px 2px rgba(0, 0, 0, 0.2);
  }

  .sidebar:not(.collapsed) {
    width: 280px;
    min-width: 280px;
  }

  /* Modern Header */
  .sidebar-header {
    padding: 20px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.06);
    background: rgba(255, 255, 255, 0.5);
    backdrop-filter: blur(10px);
  }

  :global([data-theme="dark"]) .sidebar-header {
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.02);
  }

  .brand-container {
    flex: 1;
    min-width: 0;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .logo {
    width: 44px;
    height: 44px;
    border-radius: 14px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow: 
      0 4px 16px rgba(99, 102, 241, 0.4),
      0 2px 8px rgba(139, 92, 246, 0.3),
      inset 0 1px 0 rgba(255, 255, 255, 0.2);
    flex-shrink: 0;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .logo svg {
    width: 24px;
    height: 24px;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
  }

  .logo:hover {
    transform: scale(1.05) rotate(5deg);
    box-shadow: 
      0 6px 20px rgba(99, 102, 241, 0.5),
      0 3px 10px rgba(139, 92, 246, 0.4);
  }

  .logo-collapsed {
    margin: 0 auto;
  }

  .brand-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .brand-name {
    font-size: 18px;
    font-weight: 700;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    letter-spacing: -0.02em;
    white-space: nowrap;
  }

  .brand-tagline {
    font-size: 11px;
    font-weight: 500;
    color: rgba(0, 0, 0, 0.5);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global([data-theme="dark"]) .brand-tagline {
    color: rgba(255, 255, 255, 0.5);
  }

  .toggle-btn {
    width: 36px;
    height: 36px;
    border-radius: 12px;
    border: none;
    background: rgba(99, 102, 241, 0.1);
    color: #6366f1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    flex-shrink: 0;
  }

  .toggle-btn:hover {
    background: rgba(99, 102, 241, 0.15);
    transform: scale(1.05);
  }

  .toggle-btn:active {
    transform: scale(0.95);
  }

  /* Navigation Container */
  .nav-container {
    flex: 1;
    padding: 16px 12px;
    overflow-y: auto;
    overflow-x: hidden;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .nav-container::-webkit-scrollbar {
    width: 6px;
  }

  .nav-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .nav-container::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.1);
    border-radius: 3px;
  }

  :global([data-theme="dark"]) .nav-container::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
  }

  .nav-category-label {
    font-size: 11px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.4);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 12px 16px 4px 16px;
    margin-top: 8px;
  }

  :global([data-theme="dark"]) .nav-category-label {
    color: rgba(255, 255, 255, 0.4);
  }

  .nav-category-label:first-child {
    margin-top: 0;
  }

  .nav-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  /* Modern Nav Items */
  .nav-item {
    position: relative;
    width: 100%;
    height: 48px;
    padding: 12px 16px;
    border-radius: 14px;
    border: none;
    background: transparent;
    color: rgba(0, 0, 0, 0.7);
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 14px;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    font-size: 14px;
    font-weight: 500;
    text-align: left;
    overflow: hidden;
  }

  :global([data-theme="dark"]) .nav-item {
    color: rgba(255, 255, 255, 0.7);
  }

  .collapsed .nav-item {
    justify-content: center;
    padding: 12px;
  }

  .nav-item-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 2;
  }

  .nav-item-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    z-index: 2;
  }

  .active-indicator {
    position: absolute;
    right: 12px;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    box-shadow: 0 0 8px rgba(99, 102, 241, 0.6);
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.5;
      transform: scale(0.8);
    }
  }

  .nav-item::before {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 14px;
    background: linear-gradient(135deg, 
      rgba(99, 102, 241, 0.08) 0%, 
      rgba(139, 92, 246, 0.08) 100%);
    opacity: 0;
    transition: opacity 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .nav-item:hover::before {
    opacity: 1;
  }

  .nav-item:hover {
    color: rgba(0, 0, 0, 0.9);
    transform: translateX(4px);
  }

  :global([data-theme="dark"]) .nav-item:hover {
    color: rgba(255, 255, 255, 0.9);
  }

  .nav-item:hover .nav-item-icon {
    transform: scale(1.1);
  }

  .nav-item.active {
    color: #6366f1;
    font-weight: 600;
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.12) 0%,
      rgba(139, 92, 246, 0.12) 50%,
      rgba(217, 70, 239, 0.12) 100%
    );
    box-shadow: 
      0 2px 8px rgba(99, 102, 241, 0.15),
      inset 0 1px 0 rgba(255, 255, 255, 0.2);
  }

  :global([data-theme="dark"]) .nav-item.active {
    color: #a5b4fc;
    box-shadow: 
      0 2px 8px rgba(99, 102, 241, 0.3),
      inset 0 1px 0 rgba(255, 255, 255, 0.05);
  }

  .nav-item.active::before {
    opacity: 1;
  }

  .nav-item.active:hover {
    transform: translateX(0);
  }

  .nav-item:active {
    transform: scale(0.98);
  }

  /* Footer Section */
  .sidebar-footer {
    padding: 12px;
    border-top: 1px solid rgba(0, 0, 0, 0.06);
    background: rgba(255, 255, 255, 0.5);
    backdrop-filter: blur(10px);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  :global([data-theme="dark"]) .sidebar-footer {
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.02);
  }

  .action-btn {
    width: 100%;
    height: 44px;
    padding: 10px 14px;
    border-radius: 12px;
    border: none;
    background: transparent;
    color: rgba(0, 0, 0, 0.7);
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 12px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    font-size: 14px;
    font-weight: 500;
  }

  :global([data-theme="dark"]) .action-btn {
    color: rgba(255, 255, 255, 0.7);
  }

  .collapsed .action-btn {
    justify-content: center;
    padding: 10px;
  }

  .action-btn:hover {
    background: rgba(0, 0, 0, 0.05);
    color: rgba(0, 0, 0, 0.9);
  }

  :global([data-theme="dark"]) .action-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.9);
  }

  .theme-toggle-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .theme-toggle:hover .theme-toggle-icon {
    animation: rotate 0.6s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes rotate {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .action-label {
    flex: 1;
    text-align: left;
  }

  /* Modern User Card */
  .user-card {
    padding: 14px;
    border-radius: 16px;
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.08) 0%,
      rgba(139, 92, 246, 0.08) 100%
    );
    border: 1px solid rgba(99, 102, 241, 0.15);
    display: flex;
    align-items: center;
    gap: 12px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .user-card:hover {
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.12) 0%,
      rgba(139, 92, 246, 0.12) 100%
    );
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15);
  }

  .user-avatar {
    position: relative;
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-weight: 600;
    font-size: 16px;
    flex-shrink: 0;
    box-shadow: 
      0 4px 12px rgba(99, 102, 241, 0.3),
      inset 0 1px 0 rgba(255, 255, 255, 0.2);
  }

  .avatar-text {
    z-index: 1;
  }

  .status-indicator {
    position: absolute;
    bottom: -2px;
    right: -2px;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #10b981;
    border: 2px solid white;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  :global([data-theme="dark"]) .status-indicator {
    border-color: rgba(15, 15, 22, 0.98);
  }

  .user-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .user-name {
    font-size: 14px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.9);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  :global([data-theme="dark"]) .user-name {
    color: rgba(255, 255, 255, 0.9);
  }

  .user-role {
    font-size: 11px;
    font-weight: 500;
    color: rgba(0, 0, 0, 0.5);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global([data-theme="dark"]) .user-role {
    color: rgba(255, 255, 255, 0.5);
  }

  .user-actions {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }

  .icon-btn {
    width: 32px;
    height: 32px;
    border-radius: 10px;
    border: none;
    background: rgba(0, 0, 0, 0.05);
    color: rgba(0, 0, 0, 0.6);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  :global([data-theme="dark"]) .icon-btn {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.6);
  }

  .icon-btn:hover {
    background: rgba(0, 0, 0, 0.1);
    color: rgba(0, 0, 0, 0.9);
    transform: scale(1.05);
  }

  :global([data-theme="dark"]) .icon-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    color: rgba(255, 255, 255, 0.9);
  }

  .icon-btn:active {
    transform: scale(0.95);
  }

  .logout-btn:hover {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .user-avatar-collapsed {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-weight: 600;
    font-size: 16px;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .logout-btn-collapsed:hover {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .sidebar:not(.collapsed) {
      width: 260px;
      min-width: 260px;
    }
  }
</style>
