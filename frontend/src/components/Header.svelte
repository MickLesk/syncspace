<script>
  import { auth } from "../stores/auth";
  import { currentTheme, currentLang } from "../stores/ui";

  function toggleTheme() {
    currentTheme.update((t) => {
      const newTheme = t === "light" ? "dark" : "light";
      localStorage.setItem("theme", newTheme);
      return newTheme;
    });
  }

  function toggleLang() {
    currentLang.update((l) => {
      const newLang = l === "de" ? "en" : "de";
      localStorage.setItem("lang", newLang);
      return newLang;
    });
  }

  function handleLogout() {
    auth.logout();
  }

  $: langFlag = $currentLang === "de" ? "🇩🇪" : "🇬🇧";
  $: langText = $currentLang === "de" ? "Deutsch" : "English";
  $: themeIcon = $currentTheme === "dark" ? "☀️" : "🌙";
  $: themeText = $currentTheme === "dark" ? "Light" : "Dark";
</script>

<header class="header">
  <div class="header-left">
    <div class="header-logo">S</div>
    <h1>SyncSpace</h1>
  </div>

  <div class="header-right">
    <button class="lang-selector" on:click={toggleLang}>
      <span class="lang-flag">{langFlag}</span>
      <span>{langText}</span>
    </button>

    <button class="theme-toggle" on:click={toggleTheme}>
      <span>{themeIcon}</span>
      <span>{themeText}</span>
    </button>

    <div class="user-badge">
      <div class="user-avatar">
        {$auth.username ? $auth.username.charAt(0).toUpperCase() : "U"}
      </div>
      <span>{$auth.username || "User"}</span>
    </div>

    <button class="logout-btn" on:click={handleLogout}> 🚪 Logout </button>
  </div>
</header>

<style>
  .header {
    background: var(--md-sys-color-surface);
    border-bottom: 1px solid var(--md-sys-color-outline);
    padding: 8px 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: var(--md-elevation-1);
    z-index: 100;
    height: 56px;
    min-height: 56px;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-logo {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: linear-gradient(
      135deg,
      var(--md-sys-color-primary),
      var(--md-sys-color-secondary)
    );
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    color: var(--md-sys-color-on-primary);
    font-size: 16px;
    box-shadow: var(--md-elevation-1);
    flex-shrink: 0;
  }

  h1 {
    font-size: 20px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  button {
    padding: 8px 12px;
    height: 36px;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 18px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.2s ease;
    background: var(--md-sys-color-surface-variant);
    color: var(--md-sys-color-on-surface);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  button:hover {
    background: var(--md-sys-color-secondary-container);
    border-color: var(--md-sys-color-primary);
  }

  .theme-toggle {
    width: 36px;
    height: 36px;
    padding: 0;
    border-radius: 50%;
  }

  .theme-toggle:hover {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
  }

  .user-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    height: 36px;
    background: var(--md-sys-color-primary-container);
    border-radius: 18px;
    font-size: 13px;
    border: none;
    cursor: default;
  }

  .user-badge:hover {
    background: var(--md-sys-color-primary-container);
    border-color: transparent;
  }

  .user-avatar {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: var(--md-sys-color-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--md-sys-color-on-primary);
    font-size: 12px;
    font-weight: 600;
    flex-shrink: 0;
  }

  .logout-btn {
    color: var(--md-sys-color-error);
    border-color: var(--md-sys-color-error);
  }

  .logout-btn:hover {
    background: rgba(179, 38, 30, 0.1);
  }
</style>
