<script>
  import {
    currentTheme,
    currentLang,
    favoritesEnabled,
  } from "../../stores/ui.js";

  const languageOptions = [
    { value: "de", label: "🇩🇪 Deutsch" },
    { value: "en", label: "🇬🇧 English" },
    { value: "fr", label: "🇫🇷 Français" },
    { value: "es", label: "🇪🇸 Español" },
  ];

  let enableNotifications = true;
  let autoBackup = true;
</script>

<div class="settings-grid">
  <!-- Theme Settings -->
  <div
    class="bg-white dark:bg-slate-900 rounded-2xl shadow-xl hover:shadow-2xl transition-all p-6"
  >
    <div class="space-y-4">
      <div class="flex items-center gap-3">
        <i
          class="bi bi-palette-fill text-2xl text-primary-600 dark:text-primary-400"
        ></i>
        <h2 class="text-xl font-bold text-gray-900 dark:text-white">Theme</h2>
      </div>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Choose your preferred color scheme
      </p>
      <div class="flex gap-3">
        <button
          class="flex-1 px-4 py-3 rounded-xl font-medium transition-all duration-200 flex items-center justify-center gap-2 {$currentTheme ===
          'light'
            ? 'bg-primary-600 text-white shadow-lg shadow-primary-500/30 hover:bg-primary-700'
            : 'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 border-2 border-gray-200 dark:border-gray-700'}"
          onclick={() => {
            currentTheme.set("light");
            document.documentElement.setAttribute("data-theme", "syncspace");
          }}
        >
          <i class="bi bi-sun-fill"></i>
          Light
        </button>
        <button
          class="flex-1 px-4 py-3 rounded-xl font-medium transition-all duration-200 flex items-center justify-center gap-2 {$currentTheme ===
          'dark'
            ? 'bg-primary-600 text-white shadow-lg shadow-primary-500/30 hover:bg-primary-700'
            : 'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 border-2 border-gray-200 dark:border-gray-700'}"
          onclick={() => {
            currentTheme.set("dark");
            document.documentElement.setAttribute(
              "data-theme",
              "syncspace-dark"
            );
          }}
        >
          <i class="bi bi-moon-fill"></i>
          Dark
        </button>
      </div>
    </div>
  </div>

  <!-- Language Settings -->
  <div
    class="bg-white dark:bg-slate-900 rounded-2xl shadow-xl hover:shadow-2xl transition-all p-6"
  >
    <div class="space-y-4">
      <div class="flex items-center gap-3">
        <i
          class="bi bi-translate text-2xl text-secondary-600 dark:text-secondary-400"
        ></i>
        <h2 class="text-xl font-bold text-gray-900 dark:text-white">
          Language
        </h2>
      </div>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Select your preferred language
      </p>
      <div class="w-full">
        <label
          for="language-select"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
        >
          Interface Language
        </label>
        <select
          id="language-select"
          class="w-full px-4 py-3 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl text-gray-900 dark:text-white focus:border-primary-500 dark:focus:border-primary-400 focus:ring-2 focus:ring-primary-500/20 transition-all"
          bind:value={$currentLang}
        >
          {#each languageOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </div>
  </div>

  <!-- Notification Settings -->
  <div
    class="bg-white dark:bg-slate-900 rounded-2xl shadow-xl hover:shadow-2xl transition-all p-6"
  >
    <div class="space-y-4">
      <div class="flex items-center gap-3">
        <i class="bi bi-bell-fill text-2xl text-accent"></i>
        <h2 class="text-xl font-bold text-gray-900 dark:text-white">
          Notifications
        </h2>
      </div>
      <div class="space-y-3">
        <div
          class="flex items-center justify-between gap-4 p-3 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer"
        >
          <label for="enable-notifications" class="flex-1 cursor-pointer">
            <span class="block font-medium text-gray-900 dark:text-white"
              >Enable Notifications</span
            >
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Receive desktop notifications for important events
            </p>
          </label>
          <div class="flex items-center gap-2">
            <span
              class="text-xs font-semibold uppercase tracking-wider {enableNotifications
                ? 'text-primary-600 dark:text-primary-400'
                : 'text-gray-400 dark:text-gray-600'}"
            >
              {enableNotifications ? "ON" : "OFF"}
            </span>
            <button
              id="enable-notifications"
              role="switch"
              aria-checked={enableNotifications}
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
            <span class="block font-medium text-gray-900 dark:text-white"
              >Auto Backup Notifications</span
            >
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Get notified when automatic backups complete
            </p>
          </label>
          <div class="flex items-center gap-2">
            <span
              class="text-xs font-semibold uppercase tracking-wider {autoBackup
                ? 'text-secondary-600 dark:text-secondary-400'
                : 'text-gray-400 dark:text-gray-600'}"
            >
              {autoBackup ? "ON" : "OFF"}
            </span>
            <button
              id="auto-backup"
              role="switch"
              aria-checked={autoBackup}
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
  </div>

  <!-- Features Settings -->
  <!-- Features -->
  <div
    class="bg-white dark:bg-slate-900 rounded-2xl shadow-xl hover:shadow-2xl transition-all p-6"
  >
    <div class="space-y-4">
      <div class="flex items-center gap-3">
        <i class="bi bi-toggles text-2xl text-warning"></i>
        <h2 class="text-xl font-bold text-gray-900 dark:text-white">
          Features
        </h2>
      </div>
      <div class="space-y-3">
        <div
          class="flex items-center justify-between gap-4 p-3 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer"
        >
          <label for="favorites-enabled" class="flex-1 cursor-pointer">
            <span class="block font-medium text-gray-900 dark:text-white"
              >Enable Favorites System</span
            >
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Show favorites in sidebar and enable favorite markers on
              files/folders
            </p>
          </label>
          <div class="flex items-center gap-2">
            <span
              class="text-xs font-semibold uppercase tracking-wider {$favoritesEnabled
                ? 'text-warning'
                : 'text-gray-400 dark:text-gray-600'}"
            >
              {$favoritesEnabled ? "ON" : "OFF"}
            </span>
            <button
              id="favorites-enabled"
              role="switch"
              aria-checked={$favoritesEnabled}
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
  </div>
</div>

<style>
  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
    gap: var(--spacing-6);
  }

  .space-y-3 > * + * {
    margin-top: var(--spacing-3);
  }

  .space-y-4 > * + * {
    margin-top: var(--spacing-4);
  }

  @media (max-width: 1024px) {
    .settings-grid {
      grid-template-columns: 1fr;
    }
  }
</style>

