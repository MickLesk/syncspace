<script>
  import { onMount } from "svelte";
  import { currentTheme, currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import api from "../../../lib/api.js";
  import RangeSlider from "../../../components/ui/RangeSlider.svelte";

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
  let gridColumns = $state(4);
  let saveMessage = $state("");
  let saveSuccess = $state(false);

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    try {
      loading = true;
      const response = await api.users.getSettings();
      const prefs = await api.users.getPreferences();
      selectedTheme = response.theme || "auto";
      selectedLanguage = response.language || "en";
      selectedDefaultView = response.default_view || "grid";
      gridColumns = prefs?.grid_columns || 4;
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
      await api.users.updatePreferences({
        grid_columns: gridColumns,
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
        <div class="card-icon amber">
          <i class="bi bi-palette-fill"></i>
        </div>
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

      <!-- Grid Columns Slider (only visible in grid view) -->
      {#if selectedDefaultView === "grid"}
        <div class="mt-4">
          <RangeSlider
            bind:value={gridColumns}
            min={1}
            max={8}
            label={tr("columnsPerRow")}
            showValue={true}
            showMarkers={true}
            disabled={saving}
            oninput={async (e) => {
              try {
                await api.users.updatePreferences({
                  grid_columns: gridColumns,
                });
                console.log("✅ Grid columns saved:", gridColumns);
              } catch (error) {
                console.error("Failed to save grid columns:", error);
              }
            }}
          />
        </div>
      {/if}
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

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }

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
    background: #e9d5ff;
    color: #9333ea;
  }
  :global(.dark) .card-icon.amber {
    background: rgba(251, 191, 36, 0.2);
    color: #fbbf24;
  }
  :global(.dark) .card-icon.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #3b82f6;
  }
  :global(.dark) .card-icon.purple {
    background: rgba(147, 51, 234, 0.2);
    color: #a855f7;
  }

  .card-title {
    flex: 1;
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

  .theme-buttons {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
  }
  .theme-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 0.875rem 0.5rem;
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
  }
  .theme-btn:hover:not(:disabled) {
    border-color: #22c55e;
    background: #f0fdf4;
  }
  .theme-btn.active {
    border-color: #22c55e;
    background: #dcfce7;
    color: #166534;
  }
  .theme-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .theme-btn i {
    font-size: 1.5rem;
  }
  :global(.dark) .theme-btn {
    background: #1f2937;
    border-color: #374151;
    color: #9ca3af;
  }
  :global(.dark) .theme-btn:hover:not(:disabled) {
    border-color: #22c55e;
    background: rgba(34, 197, 94, 0.1);
  }
  :global(.dark) .theme-btn.active {
    border-color: #22c55e;
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }

  .select-wrapper {
    position: relative;
  }
  .select-input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    background: white;
    font-size: 0.875rem;
    color: #111827;
    cursor: pointer;
  }
  .select-input:focus {
    outline: none;
    border-color: #22c55e;
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
  }
  :global(.dark) .select-input {
    background: #1f2937;
    border-color: #4b5563;
    color: #f9fafb;
  }

  .view-buttons {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.5rem;
  }
  .view-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
  }
  .view-btn:hover:not(:disabled) {
    border-color: #22c55e;
    background: #f0fdf4;
  }
  .view-btn.active {
    border-color: #22c55e;
    background: #dcfce7;
    color: #166534;
  }
  .view-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .view-btn i {
    font-size: 1.125rem;
  }
  :global(.dark) .view-btn {
    background: #1f2937;
    border-color: #374151;
    color: #9ca3af;
  }
  :global(.dark) .view-btn:hover:not(:disabled) {
    border-color: #22c55e;
    background: rgba(34, 197, 94, 0.1);
  }
  :global(.dark) .view-btn.active {
    border-color: #22c55e;
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }
</style>
