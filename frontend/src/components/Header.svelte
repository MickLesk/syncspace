<script>
  import { auth } from '../stores/auth';
  import { currentTheme, currentLang } from '../stores/ui';
  
  function toggleTheme() {
    currentTheme.update(t => {
      const newTheme = t === 'light' ? 'dark' : 'light';
      localStorage.setItem('theme', newTheme);
      return newTheme;
    });
  }
  
  function toggleLang() {
    currentLang.update(l => {
      const newLang = l === 'de' ? 'en' : 'de';
      localStorage.setItem('lang', newLang);
      return newLang;
    });
  }
  
  function handleLogout() {
    auth.logout();
  }
  
  $: langFlag = $currentLang === 'de' ? '🇩🇪' : '🇬🇧';
  $: langText = $currentLang === 'de' ? 'Deutsch' : 'English';
  $: themeIcon = $currentTheme === 'dark' ? '☀️' : '🌙';
  $: themeText = $currentTheme === 'dark' ? 'Light' : 'Dark';
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
        {$auth.username ? $auth.username.charAt(0).toUpperCase() : 'U'}
      </div>
      <span>{$auth.username || 'User'}</span>
    </div>
    
    <button class="logout-btn" on:click={handleLogout}>
      🚪 Logout
    </button>
  </div>
</header>

<style>
  .header {
    background: var(--md-sys-color-surface);
    border-bottom: 1px solid var(--md-sys-color-outline);
    padding: 12px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: var(--md-elevation-1);
    z-index: 100;
    min-height: 64px;
  }
  
  .header-left {
    display: flex;
    align-items: center;
    gap: 16px;
  }
  
  .header-logo {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, 
      var(--md-sys-color-primary), 
      var(--md-sys-color-secondary)
    );
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    color: var(--md-sys-color-on-primary);
    font-size: 18px;
    box-shadow: var(--md-elevation-1);
  }
  
  h1 {
    font-size: 22px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }
  
  .header-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  button {
    padding: 10px 16px;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 20px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s ease;
    background: var(--md-sys-color-surface-variant);
    color: var(--md-sys-color-on-surface);
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  button:hover {
    background: var(--md-sys-color-secondary-container);
    border-color: var(--md-sys-color-primary);
  }
  
  .theme-toggle {
    border-radius: 20px;
  }
  
  .theme-toggle:hover {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    transform: scale(1.05);
  }
  
  .user-badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--md-sys-color-primary-container);
    border-radius: 20px;
    font-size: 14px;
    border: none;
    cursor: default;
  }
  
  .user-badge:hover {
    background: var(--md-sys-color-primary-container);
    border-color: transparent;
  }
  
  .user-avatar {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: var(--md-sys-color-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--md-sys-color-on-primary);
    font-size: 13px;
    font-weight: 600;
  }
  
  .logout-btn {
    color: var(--md-sys-color-error);
    border-color: var(--md-sys-color-error);
  }
  
  .logout-btn:hover {
    background: rgba(179, 38, 30, 0.1);
  }
</style>
