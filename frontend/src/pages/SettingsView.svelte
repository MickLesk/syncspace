<script>
  import { currentTheme, currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
</script>

<div class="view-container">
  <div class="page-header">
    <h2>⚙️ {t($currentLang, "settings")}</h2>
  </div>

  <div class="settings-section">
    <h3>{t($currentLang, "appearance")}</h3>
    <div class="setting-row">
      <div class="setting-info">
        <div class="setting-label">{t($currentLang, "theme")}</div>
        <div class="setting-description">
          {t($currentLang, "themeDescription")}
        </div>
      </div>
      <div class="theme-selector">
        <button
          class="theme-btn"
          class:active={$currentTheme === "light"}
          on:click={() => currentTheme.set("light")}
        >
          ☀️ {t($currentLang, "light")}
        </button>
        <button
          class="theme-btn"
          class:active={$currentTheme === "dark"}
          on:click={() => currentTheme.set("dark")}
        >
          🌙 Dark
        </button>
      </div>
    </div>
  </div>

  <div class="settings-section">
    <h3>Language</h3>
    <div class="setting-row">
      <div class="setting-info">
        <div class="setting-label">Interface Language</div>
        <div class="setting-description">Choose your preferred language</div>
      </div>
      <div class="lang-selector">
        <button
          class="lang-btn"
          class:active={$currentLang === "de"}
          on:click={() => currentLang.set("de")}
        >
          🇩🇪 Deutsch
        </button>
        <button
          class="lang-btn"
          class:active={$currentLang === "en"}
          on:click={() => currentLang.set("en")}
        >
          🇬🇧 English
        </button>
      </div>
    </div>
  </div>

  <div class="settings-section">
    <h3>Storage</h3>
    <div class="setting-row">
      <div class="setting-info">
        <div class="setting-label">Storage Location</div>
        <div class="setting-description">
          Files are stored in: <code>./data</code>
        </div>
      </div>
    </div>
    <div class="setting-row">
      <div class="setting-info">
        <div class="setting-label">Cache</div>
        <div class="setting-description">Clear application cache</div>
      </div>
      <button class="action-btn">Clear Cache</button>
    </div>
  </div>

  <div class="settings-section">
    <h3>About</h3>
    <div class="about-content">
      <div class="logo-section">
        <div class="app-logo">📁</div>
        <div class="app-info">
          <div class="app-name">SyncSpace</div>
          <div class="app-version">Version 2.0.0</div>
        </div>
      </div>
      <div class="tech-stack">
        <div class="tech-item">
          <span class="tech-label">Frontend:</span>
          <span class="tech-value">Svelte + Vite</span>
        </div>
        <div class="tech-item">
          <span class="tech-label">Backend:</span>
          <span class="tech-value">Rust/Warp</span>
        </div>
        <div class="tech-item">
          <span class="tech-label">Design:</span>
          <span class="tech-value">Material 3 Expressive</span>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .view-container {
    padding: 24px;
    max-width: 900px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 32px;
  }

  h2 {
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }

  .settings-section {
    background: var(--md-sys-color-surface);
    border-radius: 20px;
    padding: 24px;
    margin-bottom: 16px;
    box-shadow: var(--md-elevation-1);
  }

  h3 {
    font-size: 18px;
    font-weight: 500;
    color: var(--md-sys-color-primary);
    margin: 0 0 20px 0;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-bottom: 1px solid var(--md-sys-color-outline);
  }

  .setting-row:last-child {
    border-bottom: none;
  }

  .setting-info {
    flex: 1;
  }

  .setting-label {
    font-size: 15px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 4px;
  }

  .setting-description {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  code {
    background: var(--md-sys-color-surface-variant);
    padding: 2px 6px;
    border-radius: 4px;
    font-family: "Courier New", monospace;
    font-size: 12px;
  }

  .theme-selector,
  .lang-selector {
    display: flex;
    gap: 8px;
  }

  .theme-btn,
  .lang-btn {
    padding: 8px 16px;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 16px;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 14px;
  }

  .theme-btn:hover,
  .lang-btn:hover {
    background: var(--md-sys-color-surface-variant);
  }

  .theme-btn.active,
  .lang-btn.active {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    border-color: var(--md-sys-color-primary);
  }

  .action-btn {
    padding: 8px 20px;
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
    border: none;
    border-radius: 16px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .action-btn:hover {
    box-shadow: var(--md-elevation-1);
  }

  .about-content {
    padding: 8px 0;
  }

  .logo-section {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 24px;
  }

  .app-logo {
    width: 64px;
    height: 64px;
    border-radius: 16px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 32px;
    box-shadow: var(--md-elevation-2);
  }

  .app-name {
    font-size: 22px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 4px;
  }

  .app-version {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .tech-stack {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .tech-item {
    display: flex;
    gap: 12px;
  }

  .tech-label {
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    min-width: 80px;
  }

  .tech-value {
    color: var(--md-sys-color-on-surface);
  }
</style>
