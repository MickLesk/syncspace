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
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    padding: 16px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: var(--shadow-sm);
    z-index: 100;
  }
  
  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  .header-logo {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--primary), var(--secondary));
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    color: white;
    font-size: 18px;
  }
  
  h1 {
    font-size: 22px;
    font-weight: 600;
    color: var(--text);
    margin: 0;
  }
  
  .header-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  button {
    padding: 10px 16px;
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
    background: var(--surface);
    color: var(--text);
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 500;
  }
  
  button:hover {
    background: var(--bg);
  }
  
  .theme-toggle {
    border-width: 2px;
    border-radius: 24px;
    font-weight: 600;
  }
  
  .theme-toggle:hover {
    background: var(--primary);
    color: white;
    border-color: var(--primary);
    transform: scale(1.05);
  }
  
  .user-badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    background: var(--bg);
    border-radius: 16px;
    font-size: 13px;
    border: none;
  }
  
  .user-avatar {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: var(--primary);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 12px;
    font-weight: bold;
  }
  
  .logout-btn {
    color: var(--text-secondary);
  }
  
  .logout-btn:hover {
    color: #b3261e;
    border-color: #b3261e;
  }
</style>
