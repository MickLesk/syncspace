<script>
  import { currentTheme, currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import Icon from "../components/ui/Icon.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import TabBar from "../components/ui/TabBar.svelte";
  import Button from "../components/ui/Button.svelte";

  let activeTab = "general"; // general, advanced, about
  
  const tabs = [
    { id: "general", label: "General", icon: "bi-sliders" },
    { id: "advanced", label: "Advanced", icon: "bi-tools" },
    { id: "about", label: "About", icon: "bi-info-circle" }
  ];
</script>

<div class="settings-view">
  <PageHeader 
    title="Settings"
    subtitle=""
    icon="gear-fill"
    gradient="purple"
  />

  <!-- TabBar -->
  <div class="tabs-wrapper">
    <TabBar 
      {tabs} 
      bind:activeTab 
      onChange={(id) => activeTab = id}
      variant="pills"
    />
  </div>

  <!-- Tab Content -->
  <div class="content">
    {#if activeTab === "general"}
      <div class="settings-grid fade-in">
        <!-- Theme Card -->
        <div class="glass-card card">
          <div class="card-head">
            <div class="head-icon">
              <Icon name="palette" size={20} />
            </div>
            <h3>{t($currentLang, "theme")}</h3>
          </div>
          <div class="options">
            <button class="option glass-button" class:active={$currentTheme === "light"} onclick={() => currentTheme.set("light")}>
              <Icon name="sun-fill" size={22} />
              <span>{t($currentLang, "light")}</span>
            </button>
            <button class="option glass-button" class:active={$currentTheme === "dark"} onclick={() => currentTheme.set("dark")}>
              <Icon name="moon-fill" size={22} />
              <span>Dark</span>
            </button>
          </div>
        </div>

        <!-- Language Card -->
        <div class="glass-card card">
          <div class="card-head">
            <div class="head-icon">
              <Icon name="translate" size={20} />
            </div>
            <h3>Language</h3>
          </div>
          <div class="options">
            <button class="option glass-button" class:active={$currentLang === "de"} onclick={() => currentLang.set("de")}>
              <span class="flag">🇩🇪</span>
              <span>Deutsch</span>
            </button>
            <button class="option glass-button" class:active={$currentLang === "en"} onclick={() => currentLang.set("en")}>
              <span class="flag">🇬🇧</span>
              <span>English</span>
            </button>
          </div>
        </div>
      </div>
    {:else if activeTab === "advanced"}
      <div class="settings-grid fade-in">
        <!-- Storage Card -->
        <div class="glass-card card">
          <div class="card-head">
            <div class="head-icon">
              <Icon name="hdd-fill" size={20} />
            </div>
            <h3>Storage</h3>
          </div>
          <div class="row">
            <span class="label"><Icon name="folder-fill" size={16} /> Location</span>
            <code class="glass-badge">./data</code>
          </div>
          <div class="row">
            <span class="label"><Icon name="trash" size={16} /> Cache</span>
            <Button variant="primary" size="small">
              <Icon name="x-circle" size={14} /> Clear
            </Button>
          </div>
        </div>

        <!-- Performance Card -->
        <div class="glass-card card">
          <div class="card-head">
            <div class="head-icon">
              <Icon name="speedometer" size={20} />
            </div>
            <h3>Performance</h3>
          </div>
          <div class="row">
            <span class="label"><Icon name="lightning-fill" size={16} /> Hardware Acceleration</span>
            <label class="toggle">
              <input type="checkbox" checked />
              <span class="slider"></span>
            </label>
          </div>
          <div class="row">
            <span class="label"><Icon name="file-earmark-image" size={16} /> Thumbnails</span>
            <label class="toggle">
              <input type="checkbox" checked />
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </div>
    {:else if activeTab === "about"}
      <div class="about fade-in">
        <!-- App Info -->
        <div class="glass-card app-card">
          <div class="logo-container">
            <div class="logo gradient-bg">
              <Icon name="cloud-arrow-up-fill" size={56} />
            </div>
          </div>
          <h2 class="gradient-text">SyncSpace</h2>
          <p class="version glass-badge">v1.0.0-alpha</p>
          <p class="desc">Modern self-hosted file synchronization</p>
        </div>

        <!-- Tech Stack -->
        <div class="glass-card card">
          <div class="card-head">
            <div class="head-icon">
              <Icon name="stack" size={20} />
            </div>
            <h3>Technology Stack</h3>
          </div>
          <div class="tech">
            <div class="item glass-badge">
              <Icon name="gear-fill" size={18} />
              <span>Rust + Axum</span>
            </div>
            <div class="item glass-badge">
              <Icon name="palette-fill" size={18} />
              <span>Svelte 5</span>
            </div>
            <div class="item glass-badge">
              <Icon name="database-fill" size={18} />
              <span>SQLite</span>
            </div>
            <div class="item glass-badge">
              <Icon name="search" size={18} />
              <span>Tantivy Search</span>
            </div>
          </div>
        </div>

        <!-- Links -->
        <div class="glass-card card">
          <div class="card-head">
            <div class="head-icon">
              <Icon name="link-45deg" size={20} />
            </div>
            <h3>Resources</h3>
          </div>
          <div class="links">
            <a href="https://github.com/MickLesk/syncspace" target="_blank" class="glass-button">
              <Icon name="github" size={18} />
              <span>GitHub</span>
            </a>
            <a href="https://github.com/MickLesk/syncspace/blob/main/docs" target="_blank" class="glass-button">
              <Icon name="book" size={18} />
              <span>Documentation</span>
            </a>
            <a href="https://github.com/MickLesk/syncspace/issues" target="_blank" class="glass-button">
              <Icon name="bug" size={18} />
              <span>Report Issue</span>
            </a>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .settings-view {
    min-height: 100vh;
    background: var(--md-sys-color-surface);
  }

  /* Segment Control Tabs (iOS/macOS Style) */
  .tabs-wrapper {
    position: relative;
    max-width: 800px;
    margin: -30px auto 0;
    padding: 0 24px;
    z-index: 20;
    display: flex;
    justify-content: center;
  }

  /* Content */
  .content {
    max-width: 800px;
    margin: 0 auto;
    padding: 32px 24px;
  }

  .settings-grid {
    display: grid;
    gap: 20px;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  }

  /* Glass Cards */
  .card {
    padding: 24px;
  }

  .card-head {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 20px;
  }

  .head-icon {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.15);
  }

  .card-head h3 {
    font-size: 18px;
    font-weight: 700;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }

  /* Options Grid */
  .options {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
    gap: 12px;
  }

  .option {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 20px;
    border-radius: 16px;
    font-size: 14px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .option:hover {
    transform: translateY(-4px);
  }

  .option.active {
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    color: white;
    border-color: transparent;
    box-shadow: 0 8px 24px rgba(99, 102, 241, 0.3);
  }

  .flag {
    font-size: 32px;
  }

  /* Setting Rows */
  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .row:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .label {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 14px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  code {
    padding: 6px 12px;
    border-radius: 10px;
    font-family: "SF Mono", "Courier New", monospace;
    font-size: 13px;
    font-weight: 600;
  }

  /* Modern Toggle Switch */
  .toggle {
    position: relative;
    width: 52px;
    height: 30px;
    cursor: pointer;
  }

  .toggle input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    inset: 0;
    background: var(--md-sys-color-outline);
    border-radius: 30px;
    transition: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .slider::before {
    content: "";
    position: absolute;
    height: 24px;
    width: 24px;
    left: 3px;
    bottom: 3px;
    background: white;
    border-radius: 50%;
    transition: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
  }

  .toggle input:checked + .slider {
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
  }

  .toggle input:checked + .slider::before {
    transform: translateX(22px);
  }

  /* About Section */
  .about {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .app-card {
    text-align: center;
    padding: 40px 24px;
  }

  .logo-container {
    margin-bottom: 24px;
  }

  .logo {
    width: 100px;
    height: 100px;
    margin: 0 auto;
    border-radius: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow: 
      0 12px 40px rgba(99, 102, 241, 0.3),
      inset 0 1px 1px rgba(255, 255, 255, 0.2);
    animation: float 3s ease-in-out infinite;
  }

  .app-card h2 {
    font-size: 32px;
    font-weight: 800;
    margin: 0 0 12px 0;
    letter-spacing: -0.03em;
  }

  .version {
    margin: 0 0 16px 0;
  }

  .desc {
    font-size: 15px;
    color: var(--md-sys-color-on-surface-variant);
    margin: 0;
  }

  /* Tech Stack */
  .tech {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .tech .item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 16px;
    font-size: 14px;
    font-weight: 600;
  }

  /* Links */
  .links {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .links a {
    padding: 14px 18px;
    justify-content: flex-start;
    font-weight: 600;
    text-decoration: none;
    color: var(--md-sys-color-on-surface);
  }

  .links a:hover {
    color: var(--md-sys-color-primary);
  }

  /* Responsive */
  @media (max-width: 768px) {
    .tabs-wrapper {
      margin-top: -20px;
      padding: 0 16px;
    }

    .content {
      padding: 24px 16px;
    }

    .settings-grid {
      grid-template-columns: 1fr;
    }

    .options {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>

