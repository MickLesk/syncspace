<script>
  import { onMount } from "svelte";
  import { showToast } from "../../stores/toast.js";
  import { theme, language } from "../../stores/ui.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import api from "../../lib/api.js";

  let loading = $state(false);
  let settings = $state({
    theme: $theme,
    language: $language,
    defaultView: "grid",
    emailNotifications: true,
    desktopNotifications: false,
    soundEffects: true,
    compactMode: false,
  });

  onMount(async () => {
    await loadUserSettings();
  });

  async function loadUserSettings() {
    loading = true;
    try {
      const userSettings = await api.users.getSettings();
      settings = {
        theme: userSettings.theme || "light",
        language: userSettings.language || "en",
        defaultView: userSettings.default_view || "grid",
        emailNotifications: userSettings.email_notifications ?? true,
        desktopNotifications: userSettings.desktop_notifications ?? false,
        soundEffects: userSettings.sound_effects ?? true,
        compactMode: userSettings.compact_mode ?? false,
      };
    } catch (err) {
      console.error("[UserSettings] Failed to load settings:", err);
      showToast("Failed to load settings", "error");
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    loading = true;
    try {
      await api.users.updateSettings({
        theme: settings.theme,
        language: settings.language,
        default_view: settings.defaultView,
        email_notifications: settings.emailNotifications,
        desktop_notifications: settings.desktopNotifications,
        sound_effects: settings.soundEffects,
        compact_mode: settings.compactMode,
      });

      // Update global stores
      theme.set(settings.theme);
      language.set(settings.language);

      showToast("Settings saved successfully", "success");
    } catch (err) {
      console.error("[UserSettings] Failed to save settings:", err);
      showToast("Failed to save settings", "error");
    } finally {
      loading = false;
    }
  }
</script>

<PageWrapper>
  <PageHeader
    title="User Settings"
    subtitle="Customize your personal preferences and application behavior"
    icon="sliders"
  >
    {#snippet actions()}
      <ModernButton
        variant="gradient"
        onclick={saveSettings}
        disabled={loading}
      >
        <i class="bi bi-check-lg mr-2"></i>
        {loading ? "Saving..." : "Save Changes"}
      </ModernButton>
    {/snippet}
  </PageHeader>

  <div class="page-fade-in space-y-6">
    <!-- Appearance Settings -->
    <ModernCard variant="glass">
      {#snippet children()}
        <div class="p-6 space-y-6">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-3"
          >
            <i class="bi bi-palette text-primary-600 dark:text-primary-400"></i>
            Appearance
          </h2>

          <!-- Theme Setting -->
          <div>
            <label
              for="theme-select"
              class="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2"
            >
              Theme
            </label>
            <select
              id="theme-select"
              bind:value={settings.theme}
              class="w-full px-4 py-3 bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded-xl text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition"
            >
              <option value="light">☀️ Light Mode</option>
              <option value="dark">🌙 Dark Mode</option>
              <option value="auto">🔄 Auto (System)</option>
            </select>
            <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
              Choose how SyncSpace looks on this device
            </p>
          </div>

          <!-- Language Setting -->
          <div>
            <label
              for="language-select"
              class="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2"
            >
              Language
            </label>
            <select
              id="language-select"
              bind:value={settings.language}
              class="w-full px-4 py-3 bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded-xl text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition"
            >
              <option value="en">🇬🇧 English</option>
              <option value="de">🇩🇪 Deutsch</option>
              <option value="fr">🇫🇷 Français</option>
              <option value="es">🇪🇸 Español</option>
            </select>
            <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
              Select your preferred language
            </p>
          </div>

          <!-- Default View Setting -->
          <div>
            <label
              for="default-view-select"
              class="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2"
            >
              Default File View
            </label>
            <select
              id="default-view-select"
              bind:value={settings.defaultView}
              class="w-full px-4 py-3 bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded-xl text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition"
            >
              <option value="grid">📱 Grid View</option>
              <option value="list">📋 List View</option>
            </select>
            <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
              Choose how files are displayed by default
            </p>
          </div>

          <!-- Compact Mode Toggle -->
          <div
            class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800/50 rounded-xl"
          >
            <div>
              <div class="font-semibold text-gray-900 dark:text-gray-100">
                Compact Mode
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Reduce spacing and use smaller elements
              </div>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.compactMode}
                class="sr-only peer"
              />
              <div
                class="w-14 h-7 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-300 dark:peer-focus:ring-primary-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[4px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
          </div>
        </div>
      {/snippet}
    </ModernCard>

    <!-- Notifications Settings -->
    <ModernCard variant="glass">
      {#snippet children()}
        <div class="p-6 space-y-6">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-3"
          >
            <i class="bi bi-bell text-primary-600 dark:text-primary-400"></i>
            Notifications
          </h2>

          <!-- Email Notifications -->
          <div
            class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800/50 rounded-xl"
          >
            <div>
              <div class="font-semibold text-gray-900 dark:text-gray-100">
                Email Notifications
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Receive notifications via email
              </div>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.emailNotifications}
                class="sr-only peer"
              />
              <div
                class="w-14 h-7 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-300 dark:peer-focus:ring-primary-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[4px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
          </div>

          <!-- Desktop Notifications -->
          <div
            class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800/50 rounded-xl"
          >
            <div>
              <div class="font-semibold text-gray-900 dark:text-gray-100">
                Desktop Notifications
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Show browser notifications
              </div>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.desktopNotifications}
                class="sr-only peer"
              />
              <div
                class="w-14 h-7 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-300 dark:peer-focus:ring-primary-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[4px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
          </div>

          <!-- Sound Effects -->
          <div
            class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800/50 rounded-xl"
          >
            <div>
              <div class="font-semibold text-gray-900 dark:text-gray-100">
                Sound Effects
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Play sounds for actions and notifications
              </div>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.soundEffects}
                class="sr-only peer"
              />
              <div
                class="w-14 h-7 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary-300 dark:peer-focus:ring-primary-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[4px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
          </div>
        </div>
      {/snippet}
    </ModernCard>
  </div>
</PageWrapper>
