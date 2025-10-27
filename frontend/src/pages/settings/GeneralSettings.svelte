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
  <div class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow">
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-palette-fill text-primary"></i>
        Theme
      </h2>
      <p class="text-sm opacity-70 mb-4">Choose your preferred color scheme</p>
      <div class="flex gap-3">
        <button
          class="btn btn-{$currentTheme === 'light'
            ? 'primary'
            : 'outline'} flex-1"
          on:click={() => {
            currentTheme.set("light");
            document.documentElement.setAttribute("data-theme", "syncspace");
          }}
        >
          <i class="bi bi-sun-fill"></i>
          Light
        </button>
        <button
          class="btn btn-{$currentTheme === 'dark'
            ? 'primary'
            : 'outline'} flex-1"
          on:click={() => {
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
  <div class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow">
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-translate text-secondary"></i>
        Language
      </h2>
      <p class="text-sm opacity-70 mb-4">Select your preferred language</p>
      <label class="form-control w-full">
        <div class="label">
          <span class="label-text">Interface Language</span>
        </div>
        <select class="select select-bordered w-full" bind:value={$currentLang}>
          {#each languageOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </label>
    </div>
  </div>

  <!-- Notification Settings -->
  <div class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow">
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-bell-fill text-accent"></i>
        Notifications
      </h2>
      <div class="space-y-3">
        <div class="form-control">
          <label class="label cursor-pointer">
            <div>
              <span class="label-text font-medium">Enable Notifications</span>
              <p class="text-sm opacity-70">
                Receive desktop notifications for important events
              </p>
            </div>
            <div class="flex items-center gap-2">
              <span
                class="text-xs font-medium {enableNotifications
                  ? 'text-primary'
                  : 'text-base-content/40'}"
              >
                {enableNotifications ? "ON" : "OFF"}
              </span>
              <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={enableNotifications}
              />
            </div>
          </label>
        </div>
        <div class="form-control">
          <label class="label cursor-pointer">
            <div>
              <span class="label-text font-medium"
                >Auto Backup Notifications</span
              >
              <p class="text-sm opacity-70">
                Get notified when automatic backups complete
              </p>
            </div>
            <div class="flex items-center gap-2">
              <span
                class="text-xs font-medium {autoBackup
                  ? 'text-secondary'
                  : 'text-base-content/40'}"
              >
                {autoBackup ? "ON" : "OFF"}
              </span>
              <input
                type="checkbox"
                class="toggle toggle-secondary"
                bind:checked={autoBackup}
              />
            </div>
          </label>
        </div>
      </div>
    </div>
  </div>

  <!-- Features Settings -->
  <div class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow">
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-toggles text-warning"></i>
        Features
      </h2>
      <div class="space-y-3">
        <div class="form-control">
          <label class="label cursor-pointer">
            <div>
              <span class="label-text font-medium">Enable Favorites System</span
              >
              <p class="text-sm opacity-70">
                Show favorites in sidebar and enable favorite markers on
                files/folders
              </p>
            </div>
            <div class="flex items-center gap-2">
              <span
                class="text-xs font-medium {$favoritesEnabled
                  ? 'text-warning'
                  : 'text-base-content/40'}"
              >
                {$favoritesEnabled ? "ON" : "OFF"}
              </span>
              <input
                type="checkbox"
                class="toggle toggle-warning"
                bind:checked={$favoritesEnabled}
              />
            </div>
          </label>
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
