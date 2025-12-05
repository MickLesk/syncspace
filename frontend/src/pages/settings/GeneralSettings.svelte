<script>
  import { onMount } from "svelte";
  import {
    currentTheme,
    currentLang,
    favoritesEnabled,
  } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const languageOptions = [
    { value: "de", label: "🇩🇪 Deutsch", flag: "🇩🇪" },
    { value: "en", label: "🇬🇧 English", flag: "🇬🇧" },
  ];

  const themeOptions = [
    { value: "light", icon: "bi-sun-fill", color: "amber" },
    { value: "dark", icon: "bi-moon-fill", color: "indigo" },
    { value: "auto", icon: "bi-circle-half", color: "gray" },
  ];

  const viewOptions = [
    { value: "grid", icon: "bi-grid-3x3-gap-fill" },
    { value: "list", icon: "bi-list-ul" },
  ];

  let loading = $state(true);
  let saving = $state(false);
  let selectedTheme = $state("auto");
  let selectedLanguage = $state("en");
  let selectedDefaultView = $state("grid");
  let enableNotifications = $state(true);
  let autoBackupNotify = $state(true);
  let saveMessage = $state("");
  let saveSuccess = $state(false);

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    try {
      loading = true;
      const response = await api.users.getSettings();
      selectedTheme = response.theme || "auto";
      selectedLanguage = response.language || "en";
      selectedDefaultView = response.default_view || "grid";
      currentTheme.set(
        selectedTheme === "auto" ? detectSystemTheme() : selectedTheme
      );
      currentLang.set(selectedLanguage);
    } catch (error) {
      console.error("Failed to load settings:", error);
    } finally {
      loading = false;
    }
  }

  function detectSystemTheme() {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  }

  async function saveSettings() {
    try {
      saving = true;
      saveMessage = "";
      await api.users.updateSettings({
        theme: selectedTheme,
        language: selectedLanguage,
        default_view: selectedDefaultView,
      });
      applyTheme(selectedTheme);
      currentLang.set(selectedLanguage);
      saveMessage = tr("settingsSavedSuccess");
      saveSuccess = true;
      setTimeout(() => {
        saveMessage = "";
      }, 3000);
    } catch (error) {
      console.error("Failed to save settings:", error);
      saveMessage = tr("failedToSaveSettings");
      saveSuccess = false;
      setTimeout(() => {
        saveMessage = "";
      }, 3000);
    } finally {
      saving = false;
    }
  }

  function applyTheme(theme) {
    const html = document.documentElement;
    const effectiveTheme = theme === "auto" ? detectSystemTheme() : theme;
    html.classList.remove("light", "dark");
    html.classList.add(effectiveTheme);
    html.setAttribute("data-theme", effectiveTheme);
    html.style.colorScheme = effectiveTheme;
    currentTheme.set(effectiveTheme);
  }

  function handleThemeChange(theme) {
    selectedTheme = theme;
    applyTheme(theme);
    saveSettings();
  }

  function handleLanguageChange() {
    saveSettings();
  }

  function handleDefaultViewChange() {
    saveSettings();
  }

  if (typeof window !== "undefined") {
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", () => {
        if (selectedTheme === "auto") applyTheme("auto");
      });
  }
</script>

{#if loading}
  <div class="loading-container"><div class="spinner"></div></div>
{:else}
  {#if saveMessage}
    <div class="toast" class:success={saveSuccess} class:error={!saveSuccess}>
      <i
        class="bi {saveSuccess
          ? 'bi-check-circle-fill'
          : 'bi-exclamation-circle-fill'}"
      ></i>
      {saveMessage}
    </div>
  {/if}

  <div class="settings-grid">
    <!-- Theme Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon amber"><i class="bi bi-palette-fill"></i></div>
        <div class="card-title">
          <h3>{tr("theme")}</h3>
          <p>{tr("themeDescription")}</p>
        </div>
      </div>
      <div class="theme-buttons">
        {#each themeOptions as option}
          <button
            class="theme-btn"
            class:active={selectedTheme === option.value}
            disabled={saving}
            onclick={() => handleThemeChange(option.value)}
          >
            <i class="bi {option.icon}"></i>
            <span>{tr(option.value)}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- Language Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon blue"><i class="bi bi-translate"></i></div>
        <div class="card-title">
          <h3>{tr("language")}</h3>
          <p>{tr("languageDescription")}</p>
        </div>
      </div>
      <div class="select-wrapper">
        <select
          class="select-input"
          disabled={saving}
          bind:value={selectedLanguage}
          onchange={handleLanguageChange}
        >
          {#each languageOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </div>

    <!-- Default View Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon purple"><i class="bi bi-grid-fill"></i></div>
        <div class="card-title">
          <h3>{tr("defaultView")}</h3>
          <p>{tr("defaultViewDescription")}</p>
        </div>
      </div>
      <div class="view-buttons">
        {#each viewOptions as option}
          <button
            class="view-btn"
            class:active={selectedDefaultView === option.value}
            disabled={saving}
            onclick={() => {
              selectedDefaultView = option.value;
              handleDefaultViewChange();
            }}
          >
            <i class="bi {option.icon}"></i>
            <span>{tr(option.value === "grid" ? "gridView" : "listView")}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- Notifications Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon green"><i class="bi bi-bell-fill"></i></div>
        <div class="card-title">
          <h3>{tr("notifications")}</h3>
          <p>{tr("notificationSettings")}</p>
        </div>
      </div>
      <div class="toggle-list">
        <div class="toggle-item">
          <div class="toggle-info">
            <span class="toggle-label">{tr("emailNotifications")}</span>
            <span class="toggle-desc">{tr("receiveEmailNotifications")}</span>
          </div>
          <button
            class="toggle-switch"
            class:active={enableNotifications}
            onclick={() => (enableNotifications = !enableNotifications)}
            role="switch"
            aria-checked={enableNotifications}
            aria-label="Toggle email notifications"
          >
            <span class="toggle-knob"></span>
          </button>
        </div>
        <div class="toggle-item">
          <div class="toggle-info">
            <span class="toggle-label">{tr("autoBackupNotifications")}</span>
            <span class="toggle-desc"
              >{tr("getNotifiedWhenBackupsComplete")}</span
            >
          </div>
          <button
            class="toggle-switch"
            class:active={autoBackupNotify}
            onclick={() => (autoBackupNotify = !autoBackupNotify)}
            role="switch"
            aria-checked={autoBackupNotify}
            aria-label="Toggle backup notifications"
          >
            <span class="toggle-knob"></span>
          </button>
        </div>
      </div>
    </div>

    <!-- Features Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon rose"><i class="bi bi-toggles"></i></div>
        <div class="card-title">
          <h3>{tr("features")}</h3>
          <p>{tr("customizeFeatures")}</p>
        </div>
      </div>
      <div class="toggle-list">
        <div class="toggle-item">
          <div class="toggle-info">
            <span class="toggle-label">{tr("enableFavoritesSystem")}</span>
            <span class="toggle-desc">{tr("showFavoritesInSidebar")}</span>
          </div>
          <button
            class="toggle-switch"
            class:active={$favoritesEnabled}
            onclick={() => favoritesEnabled.set(!$favoritesEnabled)}
            role="switch"
            aria-checked={$favoritesEnabled}
            aria-label="Toggle favorites system"
          >
            <span class="toggle-knob"></span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .loading-container {
    display: flex;
    justify-content: center;
    padding: 4rem;
  }
  .spinner {
    width: 36px;
    height: 36px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Toast */
  .toast {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
    font-weight: 500;
  }
  .toast.success {
    background: #dcfce7;
    color: #166534;
  }
  .toast.error {
    background: #fee2e2;
    color: #991b1b;
  }
  :global(.dark) .toast.success {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }
  :global(.dark) .toast.error {
    background: rgba(220, 38, 38, 0.2);
    color: #fca5a5;
  }

  /* Grid */
  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  /* Card */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
  }
  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-header {
    display: flex;
    align-items: flex-start;
    gap: 0.875rem;
    margin-bottom: 1.25rem;
  }
  .card-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }
  .card-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }
  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }
  .card-icon.purple {
    background: #f3e8ff;
    color: #9333ea;
  }
  .card-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }
  .card-icon.rose {
    background: #ffe4e6;
    color: #e11d48;
  }
  :global(.dark) .card-icon.amber {
    background: rgba(245, 158, 11, 0.2);
    color: #fbbf24;
  }
  :global(.dark) .card-icon.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }
  :global(.dark) .card-icon.purple {
    background: rgba(147, 51, 234, 0.2);
    color: #c084fc;
  }
  :global(.dark) .card-icon.green {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }
  :global(.dark) .card-icon.rose {
    background: rgba(225, 29, 72, 0.2);
    color: #fb7185;
  }

  .card-title h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }
  .card-title p {
    font-size: 0.8125rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .card-title h3 {
    color: #f9fafb;
  }
  :global(.dark) .card-title p {
    color: #9ca3af;
  }

  /* Theme Buttons */
  .theme-buttons {
    display: flex;
    gap: 0.5rem;
  }
  .theme-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.375rem;
    padding: 0.75rem;
    background: #f9fafb;
    border: 2px solid transparent;
    border-radius: 0.5rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.15s;
  }
  .theme-btn:hover {
    border-color: #d1d5db;
  }
  .theme-btn.active {
    background: #dcfce7;
    border-color: #22c55e;
    color: #166534;
  }
  .theme-btn i {
    font-size: 1.25rem;
  }
  :global(.dark) .theme-btn {
    background: #374151;
    color: #9ca3af;
  }
  :global(.dark) .theme-btn:hover {
    border-color: #4b5563;
  }
  :global(.dark) .theme-btn.active {
    background: rgba(34, 197, 94, 0.2);
    border-color: #22c55e;
    color: #86efac;
  }

  /* View Buttons */
  .view-buttons {
    display: flex;
    gap: 0.5rem;
  }
  .view-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: #f9fafb;
    border: 2px solid transparent;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.15s;
  }
  .view-btn:hover {
    border-color: #d1d5db;
  }
  .view-btn.active {
    background: #dcfce7;
    border-color: #22c55e;
    color: #166534;
  }
  .view-btn i {
    font-size: 1.125rem;
  }
  :global(.dark) .view-btn {
    background: #374151;
    color: #9ca3af;
  }
  :global(.dark) .view-btn:hover {
    border-color: #4b5563;
  }
  :global(.dark) .view-btn.active {
    background: rgba(34, 197, 94, 0.2);
    border-color: #22c55e;
    color: #86efac;
  }

  /* Select */
  .select-wrapper {
    position: relative;
  }
  .select-input {
    width: 100%;
    padding: 0.75rem 1rem;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    color: #111827;
    cursor: pointer;
    appearance: none;
  }
  .select-input:focus {
    outline: none;
    border-color: #22c55e;
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
  }
  :global(.dark) .select-input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  /* Toggle List */
  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .toggle-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .toggle-item {
    background: #374151;
  }
  .toggle-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .toggle-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }
  .toggle-desc {
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .toggle-label {
    color: #f9fafb;
  }
  :global(.dark) .toggle-desc {
    color: #9ca3af;
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    width: 44px;
    height: 24px;
    background: #d1d5db;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.2s;
    flex-shrink: 0;
  }
  .toggle-switch.active {
    background: #22c55e;
  }
  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    background: white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s;
  }
  .toggle-switch.active .toggle-knob {
    transform: translateX(20px);
  }
  :global(.dark) .toggle-switch {
    background: #4b5563;
  }

  @media (max-width: 768px) {
    .settings-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
