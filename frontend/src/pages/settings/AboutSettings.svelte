<script>
  import { t } from "../../i18n.js";
  import { currentLang } from "../../stores/ui.js";
  import {
    success as toastSuccess,
    error as toastError,
  } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

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
    alert(tr("settings.about.no_updates"));
  }
</script>

<div class="flex flex-col gap-6">
  <!-- App Info Card -->
  <div class="bg-gradient-to-br from-success/10 to-success/20 border border-success/30 rounded-xl p-8 text-center">
    <div class="flex flex-col items-center gap-4 mb-6">
      <div class="w-20 h-20 rounded-2xl bg-success text-white flex items-center justify-center text-4xl shadow-lg shadow-success/30">
        <i class="bi bi-cloud-arrow-up-fill"></i>
      </div>
      <div>
        <h1 class="text-2xl font-bold text-base-content m-0">{appInfo.name}</h1>
        <p class="text-base text-success font-semibold mt-1 mb-0">Version {appInfo.version}</p>
        <p class="text-xs text-base-content/60 mt-1 mb-0">Build {appInfo.build}</p>
      </div>
    </div>

    <div class="flex flex-wrap justify-center gap-3">
      <button class="btn btn-primary" onclick={checkUpdates}>
        <i class="bi bi-arrow-repeat"></i>
        {tr("settings.about.check_updates")}
      </button>
      <button class="btn btn-outline" onclick={openWebsite}>
        <i class="bi bi-globe"></i>
        {tr("settings.about.website")}
      </button>
      <button class="btn btn-outline" onclick={openGithub}>
        <i class="bi bi-github"></i>
        GitHub
      </button>
    </div>
  </div>

  <!-- Tech Stack -->
  <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden">
    <div class="flex items-center gap-4 p-5 border-b border-base-300">
      <div class="w-9 h-9 rounded-lg flex items-center justify-center text-lg bg-violet-500/20 text-violet-500 shrink-0">
        <i class="bi bi-stack"></i>
      </div>
      <div>
        <h3 class="text-base font-semibold text-base-content m-0">{tr("settings.about.tech_stack")}</h3>
        <p class="text-xs text-base-content/60 mt-1 mb-0">{tr("settings.about.tech_stack_desc")}</p>
      </div>
    </div>

    <div class="p-5">
      <div class="grid grid-cols-[repeat(auto-fit,minmax(150px,1fr))] gap-4">
        {#each techStack as tech}
          <div class="flex items-center gap-3 p-3 bg-base-200 rounded-lg">
            <div class="w-9 h-9 rounded-lg flex items-center justify-center text-base
              {tech.color === 'amber' ? 'bg-warning/20 text-warning' : ''}
              {tech.color === 'green' ? 'bg-success/20 text-success' : ''}
              {tech.color === 'blue' ? 'bg-info/20 text-info' : ''}
              {tech.color === 'purple' ? 'bg-violet-500/20 text-violet-500' : ''}
            ">
              <i class="bi {tech.icon}"></i>
            </div>
            <div class="flex flex-col">
              <span class="text-sm font-semibold text-base-content">{tech.name}</span>
              <span class="text-xs text-base-content/60">v{tech.version}</span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Features -->
  <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden">
    <div class="flex items-center gap-4 p-5 border-b border-base-300">
      <div class="w-9 h-9 rounded-lg flex items-center justify-center text-lg bg-success/20 text-success shrink-0">
        <i class="bi bi-check2-circle"></i>
      </div>
      <div>
        <h3 class="text-base font-semibold text-base-content m-0">{tr("settings.about.features")}</h3>
        <p class="text-xs text-base-content/60 mt-1 mb-0">{tr("settings.about.features_desc")}</p>
      </div>
    </div>

    <div class="p-5">
      <div class="grid grid-cols-[repeat(auto-fit,minmax(180px,1fr))] gap-3">
        {#each features as feature}
          <div class="flex items-center gap-3 px-4 py-3 bg-success/10 rounded-lg text-success text-sm font-medium">
            <i class="bi {feature.icon} text-lg"></i>
            <span>{tr("settings.about.feature_" + feature.key)}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- License & Credits -->
  <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden">
    <div class="flex items-center gap-4 p-5 border-b border-base-300">
      <div class="w-9 h-9 rounded-lg flex items-center justify-center text-lg bg-info/20 text-info shrink-0">
        <i class="bi bi-file-text"></i>
      </div>
      <div>
        <h3 class="text-base font-semibold text-base-content m-0">{tr("settings.about.license")}</h3>
        <p class="text-xs text-base-content/60 mt-1 mb-0">{tr("settings.about.license_desc")}</p>
      </div>
    </div>

    <div class="p-5">
      <div class="text-center">
        <div class="inline-flex items-center gap-2 px-4 py-2 bg-info/20 text-info rounded-full font-semibold text-sm mb-4">
          <i class="bi bi-award"></i>
          <span>{appInfo.license} License</span>
        </div>
        <p class="text-base-content/60 text-sm mb-4 leading-relaxed">
          {tr("settings.about.license_text")}
        </p>
        <p class="text-base-content text-sm m-0">
          {tr("settings.about.made_with")}
          <i class="bi bi-heart-fill text-error"></i>
          {tr("settings.about.by")}
          {appInfo.author}
        </p>
      </div>
    </div>
  </div>
</div>

<style>
  /* Minimal CSS - most styling via Tailwind utilities */
</style>
