<script>
  import { onMount } from "svelte";
  import {
    currentTheme,
    currentLang,
    favoritesEnabled,
  } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const languageOptions = [
    { value: "de", label: "🇩🇪 Deutsch" },
    { value: "en", label: "🇬🇧 English" },
  ];

  // Theme options will use tr() directly in template
  const themeOptions = [
    { value: "light", icon: "sun-fill" },
    { value: "dark", icon: "moon-fill" },
    { value: "auto", icon: "circle-half" },
  ];

  // View options will use tr() directly in template
  const defaultViewOptions = [{ value: "grid" }, { value: "list" }];

  let loading = $state(true);
  let saving = $state(false);
  let selectedTheme = $state("auto");
  let selectedLanguage = $state("en");
  let selectedDefaultView = $state("grid");
  let enableNotifications = $state(true);
  let autoBackup = $state(true);
  let saveMessage = $state("");

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

      // Apply theme immediately
      currentTheme.set(
        selectedTheme === "auto" ? detectSystemTheme() : selectedTheme
      );
      currentLang.set(selectedLanguage);

      loading = false;
    } catch (error) {
      console.error("Failed to load settings:", error);
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

      // Apply theme
      applyTheme(selectedTheme);
      currentLang.set(selectedLanguage);

      saveMessage = tr("settingsSavedSuccess");
      setTimeout(() => (saveMessage = ""), 3000);
    } catch (error) {
      console.error("Failed to save settings:", error);
      saveMessage = tr("failedToSaveSettings");
      setTimeout(() => (saveMessage = ""), 3000);
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

  // Listen for system theme changes
  if (typeof window !== "undefined") {
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (e) => {
        if (selectedTheme === "auto") {
          applyTheme("auto");
        }
      });
  }
</script>

{#if loading}
  <div class="flex items-center justify-center p-12">
    <div class="flex items-center gap-3">
      <div
        class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"
      ></div>
      <span class="text-gray-600 dark:text-gray-400"
        >{tr("loadingSettings")}</span
      >
    </div>
  </div>
{:else}
  {#if saveMessage}
    <div
      class="mb-4 p-4 rounded-xl {saveMessage.startsWith('✅')
        ? 'bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-300'
        : 'bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-300'}"
    >
      {saveMessage}
    </div>
  {/if}

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- Theme Settings -->
    <ModernCard variant="glass" hoverable>
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-3">
          <i
            class="bi bi-palette-fill text-2xl text-primary-600 dark:text-primary-400"
          ></i>
          <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {tr("theme")}
          </h2>
        </div>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          {tr("themeDescription")}
        </p>
        <div class="flex gap-3">
          {#each themeOptions as option}
            <ModernButton
              variant={selectedTheme === option.value ? "primary" : "secondary"}
              class="flex-1"
              disabled={saving}
              onclick={() => handleThemeChange(option.value)}
            >
              <i class="bi bi-{option.icon} mr-2"></i>
              {tr(
                option.value === "light"
                  ? "light"
                  : option.value === "dark"
                    ? "dark"
                    : "auto"
              )}
            </ModernButton>
          {/each}
        </div>
      </div>
    </ModernCard>

    <!-- Language Settings -->
    <ModernCard variant="glass" hoverable>
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-3">
          <i
            class="bi bi-translate text-2xl text-secondary-600 dark:text-secondary-400"
          ></i>
          <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {tr("language")}
          </h2>
        </div>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          {tr("selectDefaultView")}
        </p>
        <div class="w-full">
          <label
            for="language-select"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            {tr("interfaceLanguage")}
          </label>
          <select
            id="language-select"
            class="glass-input w-full"
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
    </ModernCard>

    <!-- Default View Settings -->
    <ModernCard variant="glass" hoverable>
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-3">
          <i
            class="bi bi-grid-fill text-2xl text-purple-600 dark:text-purple-400"
          ></i>
          <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {tr("defaultView")}
          </h2>
        </div>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          {tr("defaultViewDescription")}
        </p>
        <div class="w-full">
          <label
            for="view-select"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            {tr("defaultView")}
          </label>
          <select
            id="view-select"
            class="glass-input w-full"
            disabled={saving}
            bind:value={selectedDefaultView}
            onchange={handleDefaultViewChange}
          >
            {#each defaultViewOptions as option}
              <option value={option.value}
                >{tr(option.value === "grid" ? "gridView" : "listView")}</option
              >
            {/each}
          </select>
        </div>
      </div>
    </ModernCard>

    <!-- Notification Settings -->
    <ModernCard variant="glass" hoverable>
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-3">
          <i
            class="bi bi-bell-fill text-2xl text-purple-600 dark:text-purple-400"
          ></i>
          <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {tr("notifications")}
          </h2>
        </div>
        <div class="space-y-3">
          <div
            class="flex items-center justify-between gap-4 p-3 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer"
          >
            <label for="enable-notifications" class="flex-1 cursor-pointer">
              <span class="block font-medium text-gray-900 dark:text-gray-100"
                >{tr("emailNotifications")}</span
              >
              <p class="text-sm text-gray-600 dark:text-gray-400">
                {tr("receiveEmailNotifications")}
              </p>
            </label>
            <div class="flex items-center gap-2">
              <span
                class="badge-glass-{enableNotifications ? 'success' : 'error'}"
              >
                {enableNotifications ? tr("on") : tr("off")}
              </span>
              <button
                id="enable-notifications"
                role="switch"
                aria-checked={enableNotifications}
                aria-label="Toggle notifications"
                class="relative inline-flex h-7 w-12 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 {enableNotifications
                  ? 'bg-primary-600'
                  : 'bg-gray-300 dark:bg-gray-700'}"
                onclick={() => (enableNotifications = !enableNotifications)}
              >
                <span
                  class="pointer-events-none inline-block h-6 w-6 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {enableNotifications
                    ? 'translate-x-5'
                    : 'translate-x-0'}"
                ></span>
              </button>
            </div>
          </div>
          <div
            class="flex items-center justify-between gap-4 p-3 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer"
          >
            <label for="auto-backup" class="flex-1 cursor-pointer">
              <span class="block font-medium text-gray-900 dark:text-gray-100"
                >{tr("autoBackupNotifications")}</span
              >
              <p class="text-sm text-gray-600 dark:text-gray-400">
                {tr("getNotifiedWhenBackupsComplete")}
              </p>
            </label>
            <div class="flex items-center gap-2">
              <span class="badge-glass-{autoBackup ? 'success' : 'error'}">
                {autoBackup ? tr("on") : tr("off")}
              </span>
              <button
                id="auto-backup"
                role="switch"
                aria-checked={autoBackup}
                aria-label="Toggle auto backup notifications"
                class="relative inline-flex h-7 w-12 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-secondary-500 focus:ring-offset-2 {autoBackup
                  ? 'bg-secondary-600'
                  : 'bg-gray-300 dark:bg-gray-700'}"
                onclick={() => (autoBackup = !autoBackup)}
              >
                <span
                  class="pointer-events-none inline-block h-6 w-6 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {autoBackup
                    ? 'translate-x-5'
                    : 'translate-x-0'}"
                ></span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </ModernCard>

    <!-- Features Settings -->
    <ModernCard variant="glass" hoverable>
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-3">
          <i class="bi bi-toggles text-2xl text-yellow-600 dark:text-yellow-400"
          ></i>
          <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {tr("features")}
          </h2>
        </div>
        <div class="space-y-3">
          <div
            class="flex items-center justify-between gap-4 p-3 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer"
          >
            <label for="favorites-enabled" class="flex-1 cursor-pointer">
              <span class="block font-medium text-gray-900 dark:text-gray-100"
                >{tr("enableFavoritesSystem")}</span
              >
              <p class="text-sm text-gray-600 dark:text-gray-400">
                {tr("showFavoritesInSidebar")}
              </p>
            </label>
            <div class="flex items-center gap-2">
              <span
                class="badge-glass-{$favoritesEnabled ? 'warning' : 'error'}"
              >
                {$favoritesEnabled ? tr("on") : tr("off")}
              </span>
              <button
                id="favorites-enabled"
                role="switch"
                aria-checked={$favoritesEnabled}
                aria-label="Toggle favorites system"
                class="relative inline-flex h-7 w-12 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 {$favoritesEnabled
                  ? 'bg-yellow-500'
                  : 'bg-gray-300 dark:bg-gray-700'}"
                onclick={() => favoritesEnabled.set(!$favoritesEnabled)}
              >
                <span
                  class="pointer-events-none inline-block h-6 w-6 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {$favoritesEnabled
                    ? 'translate-x-5'
                    : 'translate-x-0'}"
                ></span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </ModernCard>
  </div>
{/if}
