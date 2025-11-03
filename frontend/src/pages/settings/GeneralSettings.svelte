<script>
  import {
    currentTheme,
    currentLang,
    favoritesEnabled,
  } from "../../stores/ui.js";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

  const languageOptions = [
    { value: "de", label: "🇩🇪 Deutsch" },
    { value: "en", label: "🇬🇧 English" },
    { value: "fr", label: "🇫🇷 Français" },
    { value: "es", label: "🇪🇸 Español" },
  ];

  let enableNotifications = true;
  let autoBackup = true;
</script>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <!-- Theme Settings -->
  <ModernCard variant="glass" hoverable>
    <div class="p-6 space-y-4">
      <div class="flex items-center gap-3">
        <i
          class="bi bi-palette-fill text-2xl text-primary-600 dark:text-primary-400"
        ></i>
        <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
          Theme
        </h2>
      </div>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Choose your preferred color scheme
      </p>
      <div class="flex gap-3">
        <ModernButton
          variant={$currentTheme === "light" ? "primary" : "secondary"}
          class="flex-1"
          onclick={() => {
            currentTheme.set("light");
            const html = document.documentElement;
            html.classList.remove("dark");
            html.classList.add("light");
            html.setAttribute("data-theme", "light");
            html.style.colorScheme = "light";
          }}
        >
          <i class="bi bi-sun-fill mr-2"></i>
          Light
        </ModernButton>
        <ModernButton
          variant={$currentTheme === "dark" ? "primary" : "secondary"}
          class="flex-1"
          onclick={() => {
            currentTheme.set("dark");
            const html = document.documentElement;
            html.classList.remove("light");
            html.classList.add("dark");
            html.setAttribute("data-theme", "dark");
            html.style.colorScheme = "dark";
          }}
        >
          <i class="bi bi-moon-fill mr-2"></i>
          Dark
        </ModernButton>
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
          class="glass-input w-full"
          bind:value={$currentLang}
        >
          {#each languageOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </div>
  </ModernCard>

  <!-- Notification Settings -->
  <ModernCard variant="glass" hoverable>
    <div class="p-6 space-y-4">
      <div class="flex items-center gap-3">
        <i class="bi bi-bell-fill text-2xl text-purple-600 dark:text-purple-400"
        ></i>
        <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
          Notifications
        </h2>
      </div>
      <div class="space-y-3">
        <div
          class="flex items-center justify-between gap-4 p-3 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer"
        >
          <label for="enable-notifications" class="flex-1 cursor-pointer">
            <span class="block font-medium text-gray-900 dark:text-gray-100"
              >Enable Notifications</span
            >
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Receive desktop notifications for important events
            </p>
          </label>
          <div class="flex items-center gap-2">
            <span
              class="badge-glass-{enableNotifications ? 'success' : 'error'}"
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
            <span class="block font-medium text-gray-900 dark:text-gray-100"
              >Auto Backup Notifications</span
            >
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Get notified when automatic backups complete
            </p>
          </label>
          <div class="flex items-center gap-2">
            <span class="badge-glass-{autoBackup ? 'success' : 'error'}">
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
  </ModernCard>

  <!-- Features Settings -->
  <ModernCard variant="glass" hoverable>
    <div class="p-6 space-y-4">
      <div class="flex items-center gap-3">
        <i class="bi bi-toggles text-2xl text-yellow-600 dark:text-yellow-400"
        ></i>
        <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
          Features
        </h2>
      </div>
      <div class="space-y-3">
        <div
          class="flex items-center justify-between gap-4 p-3 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer"
        >
          <label for="favorites-enabled" class="flex-1 cursor-pointer">
            <span class="block font-medium text-gray-900 dark:text-gray-100"
              >Enable Favorites System</span
            >
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Show favorites in sidebar and enable favorite markers on
              files/folders
            </p>
          </label>
          <div class="flex items-center gap-2">
            <span class="badge-glass-{$favoritesEnabled ? 'warning' : 'error'}">
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
  </ModernCard>
</div>
