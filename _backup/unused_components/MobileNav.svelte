<script>
  /**
   * Mobile Bottom Navigation Bar
   * Shows primary navigation items at bottom of screen
   */
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    activeView = "files",
    onNavigate = () => {},
    unreadNotifications = 0,
  } = $props();

  let navItems = $derived([
    { id: "files", icon: "folder", label: tr("files") },
    { id: "recent", icon: "clock-history", label: tr("recentFiles") },
    { id: "favorites", icon: "star", label: tr("favorites") },
    {
      id: "notifications",
      icon: "bell",
      label: tr("notifications"),
      badge: true,
    },
    { id: "more", icon: "three-dots", label: tr("more") },
  ]);

  function handleNavClick(itemId) {
    onNavigate(itemId);
  }
</script>

<nav class="mobile-nav" aria-label="Mobile bottom navigation">
  {#each navItems as item}
    <button
      class="nav-item {activeView === item.id ? 'active' : ''}"
      onclick={() => handleNavClick(item.id)}
      aria-label={item.label}
      aria-current={activeView === item.id ? "page" : undefined}
    >
      <div class="nav-icon-wrapper">
        <i class="bi bi-{item.icon} nav-icon" aria-hidden="true"></i>
        {#if item.badge && unreadNotifications > 0}
          <span class="notification-badge">
            {unreadNotifications > 99 ? "99+" : unreadNotifications}
          </span>
        {/if}
      </div>
      <span class="nav-label">{item.label}</span>
    </button>
  {/each}
</nav>

<style>
  .mobile-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    background: white;
    border-top: 1px solid rgb(229 231 235);
    display: flex;
    justify-content: space-around;
    align-items: center;
    padding: 8px 0;
    z-index: 40;
    box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.05);
    /* Safe area for iOS notch */
    padding-bottom: max(8px, env(safe-area-inset-bottom));
  }

  .nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 8px 12px;
    border: none;
    background: none;
    cursor: pointer;
    transition: all 0.2s;
    flex: 1;
    max-width: 80px;
    position: relative;
  }

  .nav-icon-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .nav-icon {
    font-size: 22px;
    color: rgb(107 114 128);
    transition:
      color 0.2s,
      transform 0.2s;
  }

  .nav-label {
    font-size: 11px;
    color: rgb(107 114 128);
    transition: color 0.2s;
    font-weight: 500;
  }

  .nav-item.active .nav-icon {
    color: rgb(59 130 246);
    transform: scale(1.1);
  }

  .nav-item.active .nav-label {
    color: rgb(59 130 246);
    font-weight: 600;
  }

  .nav-item:active {
    transform: scale(0.95);
  }

  .notification-badge {
    position: absolute;
    top: -4px;
    right: -8px;
    background: rgb(239 68 68);
    color: white;
    font-size: 10px;
    font-weight: 700;
    padding: 2px 5px;
    border-radius: 10px;
    min-width: 18px;
    text-align: center;
    line-height: 1;
  }

  /* Dark mode */
  @media (prefers-color-scheme: dark) {
    .mobile-nav {
      background: rgb(31 41 55);
      border-top-color: rgb(55 65 81);
    }

    .nav-icon,
    .nav-label {
      color: rgb(156 163 175);
    }

    .nav-item.active .nav-icon,
    .nav-item.active .nav-label {
      color: rgb(96 165 250);
    }
  }

  /* Hide on desktop */
  @media (min-width: 768px) {
    .mobile-nav {
      display: none;
    }
  }
</style>
