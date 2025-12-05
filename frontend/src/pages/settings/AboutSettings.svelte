<script>
  import { t } from "../../i18n.js";

  const appInfo = {
    name: "SyncSpace",
    version: "1.0.0",
    build: "2024.01.15",
    license: "MIT",
    author: "SyncSpace Team",
    website: "https://syncspace.dev",
  };

  const techStack = [
    { name: "Rust", version: "1.75", icon: "bi-gear-fill", color: "amber" },
    { name: "Axum", version: "0.8", icon: "bi-hdd-stack", color: "green" },
    { name: "SQLite", version: "3.44", icon: "bi-database", color: "blue" },
    { name: "Svelte", version: "5.0", icon: "bi-filetype-js", color: "purple" },
    { name: "Vite", version: "6.0", icon: "bi-lightning", color: "amber" },
    { name: "Tailwind", version: "4.0", icon: "bi-palette", color: "blue" },
  ];

  const features = [
    { key: "file_sync", icon: "bi-arrow-repeat" },
    { key: "search", icon: "bi-search" },
    { key: "sharing", icon: "bi-share" },
    { key: "versioning", icon: "bi-clock-history" },
    { key: "collaboration", icon: "bi-people" },
    { key: "2fa", icon: "bi-shield-check" },
  ];

  function openWebsite() {
    window.open(appInfo.website, "_blank");
  }

  function openGithub() {
    window.open("https://github.com/syncspace/syncspace", "_blank");
  }

  function checkUpdates() {
    alert($t("settings.about.no_updates"));
  }
</script>

<div class="about-settings">
  <!-- App Info Card -->
  <div class="card hero-card">
    <div class="hero-content">
      <div class="app-logo">
        <i class="bi bi-cloud-arrow-up-fill"></i>
      </div>
      <div class="app-info">
        <h1>{appInfo.name}</h1>
        <p class="version">Version {appInfo.version}</p>
        <p class="build">Build {appInfo.build}</p>
      </div>
    </div>

    <div class="hero-actions">
      <button class="btn btn-primary" onclick={checkUpdates}>
        <i class="bi bi-arrow-repeat"></i>
        {$t("settings.about.check_updates")}
      </button>
      <button class="btn btn-secondary" onclick={openWebsite}>
        <i class="bi bi-globe"></i>
        {$t("settings.about.website")}
      </button>
      <button class="btn btn-secondary" onclick={openGithub}>
        <i class="bi bi-github"></i>
        GitHub
      </button>
    </div>
  </div>

  <!-- Tech Stack -->
  <div class="card">
    <div class="card-header">
      <div class="card-icon purple">
        <i class="bi bi-stack"></i>
      </div>
      <div>
        <h3>{$t("settings.about.tech_stack")}</h3>
        <p class="card-subtitle">{$t("settings.about.tech_stack_desc")}</p>
      </div>
    </div>

    <div class="card-body">
      <div class="tech-grid">
        {#each techStack as tech}
          <div class="tech-item">
            <div class="tech-icon {tech.color}">
              <i class="bi {tech.icon}"></i>
            </div>
            <div class="tech-info">
              <span class="tech-name">{tech.name}</span>
              <span class="tech-version">v{tech.version}</span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Features -->
  <div class="card">
    <div class="card-header">
      <div class="card-icon green">
        <i class="bi bi-check2-circle"></i>
      </div>
      <div>
        <h3>{$t("settings.about.features")}</h3>
        <p class="card-subtitle">{$t("settings.about.features_desc")}</p>
      </div>
    </div>

    <div class="card-body">
      <div class="features-grid">
        {#each features as feature}
          <div class="feature-item">
            <i class="bi {feature.icon}"></i>
            <span>{$t("settings.about.feature_" + feature.key)}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- License & Credits -->
  <div class="card">
    <div class="card-header">
      <div class="card-icon blue">
        <i class="bi bi-file-text"></i>
      </div>
      <div>
        <h3>{$t("settings.about.license")}</h3>
        <p class="card-subtitle">{$t("settings.about.license_desc")}</p>
      </div>
    </div>

    <div class="card-body">
      <div class="license-info">
        <div class="license-badge">
          <i class="bi bi-award"></i>
          <span>{appInfo.license} License</span>
        </div>
        <p class="license-text">
          {$t("settings.about.license_text")}
        </p>
        <p class="credits">
          {$t("settings.about.made_with")}
          <i class="bi bi-heart-fill text-red"></i>
          {$t("settings.about.by")}
          {appInfo.author}
        </p>
      </div>
    </div>
  </div>
</div>

<style>
  .about-settings {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* Cards */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    overflow: hidden;
  }

  :global([data-theme="dark"]) .card {
    background: #1f2937;
    border-color: #374151;
  }

  /* Hero Card */
  .hero-card {
    padding: 2rem;
    text-align: center;
    background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
    border-color: #bbf7d0;
  }

  :global([data-theme="dark"]) .hero-card {
    background: linear-gradient(
      135deg,
      rgba(34, 197, 94, 0.1) 0%,
      rgba(22, 163, 74, 0.15) 100%
    );
    border-color: rgba(34, 197, 94, 0.3);
  }

  .hero-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .app-logo {
    width: 80px;
    height: 80px;
    border-radius: 1rem;
    background: #22c55e;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2.5rem;
    box-shadow: 0 8px 24px rgba(34, 197, 94, 0.3);
  }

  .app-info h1 {
    font-size: 1.75rem;
    font-weight: 700;
    color: #111827;
    margin: 0;
  }

  :global([data-theme="dark"]) .app-info h1 {
    color: #f9fafb;
  }

  .version {
    font-size: 1rem;
    color: #22c55e;
    font-weight: 600;
    margin: 0.25rem 0 0 0;
  }

  .build {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0.25rem 0 0 0;
  }

  .hero-actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.75rem;
  }

  /* Buttons */
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 0.5rem;
    border: none;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-primary {
    background: #22c55e;
    color: white;
  }

  .btn-primary:hover {
    background: #16a34a;
  }

  .btn-secondary {
    background: white;
    color: #374151;
    border: 1px solid #d1d5db;
  }

  .btn-secondary:hover {
    background: #f9fafb;
    border-color: #9ca3af;
  }

  :global([data-theme="dark"]) .btn-secondary {
    background: #374151;
    color: #e5e7eb;
    border-color: #4b5563;
  }

  :global([data-theme="dark"]) .btn-secondary:hover {
    background: #4b5563;
  }

  /* Card Header */
  .card-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .card-header {
    border-bottom-color: #374151;
  }

  .card-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  :global([data-theme="dark"]) .card-header h3 {
    color: #f9fafb;
  }

  .card-subtitle {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0.25rem 0 0 0;
  }

  .card-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }

  .card-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }

  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }

  .card-icon.purple {
    background: #f3e8ff;
    color: #9333ea;
  }

  :global([data-theme="dark"]) .card-icon.green {
    background: rgba(22, 163, 74, 0.2);
  }

  :global([data-theme="dark"]) .card-icon.blue {
    background: rgba(37, 99, 235, 0.2);
  }

  :global([data-theme="dark"]) .card-icon.purple {
    background: rgba(147, 51, 234, 0.2);
  }

  .card-body {
    padding: 1.25rem;
  }

  /* Tech Grid */
  .tech-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }

  .tech-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }

  :global([data-theme="dark"]) .tech-item {
    background: #374151;
  }

  .tech-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
  }

  .tech-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }

  .tech-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }

  .tech-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }

  .tech-icon.purple {
    background: #f3e8ff;
    color: #9333ea;
  }

  :global([data-theme="dark"]) .tech-icon.green {
    background: rgba(22, 163, 74, 0.2);
  }

  :global([data-theme="dark"]) .tech-icon.blue {
    background: rgba(37, 99, 235, 0.2);
  }

  :global([data-theme="dark"]) .tech-icon.amber {
    background: rgba(217, 119, 6, 0.2);
  }

  :global([data-theme="dark"]) .tech-icon.purple {
    background: rgba(147, 51, 234, 0.2);
  }

  .tech-info {
    display: flex;
    flex-direction: column;
  }

  .tech-name {
    font-size: 0.875rem;
    font-weight: 600;
    color: #111827;
  }

  :global([data-theme="dark"]) .tech-name {
    color: #f9fafb;
  }

  .tech-version {
    font-size: 0.75rem;
    color: #6b7280;
  }

  /* Features Grid */
  .features-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 0.75rem;
  }

  .feature-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: #f0fdf4;
    border-radius: 0.5rem;
    color: #16a34a;
    font-size: 0.875rem;
    font-weight: 500;
  }

  :global([data-theme="dark"]) .feature-item {
    background: rgba(22, 163, 74, 0.1);
    color: #4ade80;
  }

  .feature-item i {
    font-size: 1.125rem;
  }

  /* License Info */
  .license-info {
    text-align: center;
  }

  .license-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #dbeafe;
    color: #2563eb;
    border-radius: 9999px;
    font-weight: 600;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  :global([data-theme="dark"]) .license-badge {
    background: rgba(37, 99, 235, 0.2);
    color: #60a5fa;
  }

  .license-text {
    color: #6b7280;
    font-size: 0.875rem;
    margin: 0 0 1rem 0;
    line-height: 1.6;
  }

  .credits {
    color: #374151;
    font-size: 0.875rem;
    margin: 0;
  }

  :global([data-theme="dark"]) .credits {
    color: #e5e7eb;
  }

  .text-red {
    color: #ef4444;
  }
</style>
