<script>
  import { onMount } from "svelte";
  import { showToast } from "../../stores/toast.js";
  import { theme, language } from "../../stores/ui.js";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import api from "../../lib/api.js";

  let loading = $state(false);
  let settings = $state({
    theme: "auto",
    language: "en",
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
        theme: userSettings.theme || "auto", // Default to AUTO
        language: userSettings.language || "en",
        defaultView: userSettings.default_view || "grid",
        emailNotifications: userSettings.email_notifications ?? true,
        desktopNotifications: userSettings.desktop_notifications ?? false,
        soundEffects: userSettings.sound_effects ?? true,
        compactMode: userSettings.compact_mode ?? false,
      };

      // Update global stores
      theme.set(settings.theme);
      language.set(settings.language);
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
      // Save user settings (theme, language, default_view)
      await api.users.updateSettings({
        theme: settings.theme,
        language: settings.language,
        default_view: settings.defaultView,
      });

      // Save user preferences (client-specific settings)
      await api.users.updatePreferences({
        email_notifications: settings.emailNotifications,
        desktop_notifications: settings.desktopNotifications,
        sound_effects: settings.soundEffects,
        compact_mode: settings.compactMode,
        view_mode: settings.defaultView, // Also save in preferences for consistency
      });

      // Update global stores
      theme.set(settings.theme);
      language.set(settings.language);

      // Reload settings to verify persistence
      await loadUserSettings();

      showToast("Settings saved successfully", "success");
    } catch (err) {
      console.error("[UserSettings] Failed to save settings:", err);
      showToast("Failed to save settings", "error");
    } finally {
      loading = false;
    }
  }
</script>

<div class="max-w-4xl mx-auto p-4 space-y-4">
  <!-- Appearance Settings -->
  <ModernCard variant="glass">
    {#snippet children()}
      <div class="p-4 space-y-4">
        <h2
          class="text-lg font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2"
        >
          <i class="bi bi-palette text-primary-600 dark:text-primary-400"></i>
          Appearance
        </h2>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Theme Setting -->
          <div>
            <label
              for="theme-select"
              class="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1"
            >
              Theme
            </label>
            <select
              id="theme-select"
              bind:value={settings.theme}
              class="w-full px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition"
            >
              <option value="auto">🔄 Auto (System)</option>
              <option value="light">☀️ Light Mode</option>
              <option value="dark">🌙 Dark Mode</option>
            </select>
          </div>

          <!-- Language Setting -->
          <div>
            <label
              for="language-select"
              class="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1"
            >
              Language
            </label>
            <select
              id="language-select"
              bind:value={settings.language}
              class="w-full px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition"
            >
              <option value="en">🇬🇧 English</option>
              <option value="de">🇩🇪 Deutsch</option>
              <option value="fr">🇫🇷 Français</option>
              <option value="es">🇪🇸 Español</option>
            </select>
          </div>

          <!-- Default View Setting -->
          <div>
            <label
              for="default-view-select"
              class="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1"
            >
              Default File View
            </label>
            <select
              id="default-view-select"
              bind:value={settings.defaultView}
              class="w-full px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-sm text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition"
            >
              <option value="grid">📱 Grid View</option>
              <option value="list">📋 List View</option>
            </select>
          </div>

          <!-- Compact Mode Toggle -->
          <div class="flex items-center gap-3">
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.compactMode}
                class="sr-only peer"
              />
              <div
                class="w-11 h-6 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-primary-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
            <div class="text-sm text-gray-700 dark:text-gray-300">
              Compact Mode
            </div>
          </div>
        </div>
      </div>
    {/snippet}
  </ModernCard>

  <!-- Notifications Settings -->
  <ModernCard variant="glass">
    {#snippet children()}
      <div class="p-4 space-y-4">
        <h2
          class="text-lg font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2"
        >
          <i class="bi bi-bell text-primary-600 dark:text-primary-400"></i>
          Notifications
        </h2>

        <div class="space-y-3">
          <!-- Email Notifications -->
          <div
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg"
          >
            <div class="text-sm text-gray-900 dark:text-gray-100">
              Email Notifications
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.emailNotifications}
                class="sr-only peer"
              />
              <div
                class="w-11 h-6 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-primary-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
          </div>

          <!-- Desktop Notifications -->
          <div
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg"
          >
            <div class="text-sm text-gray-900 dark:text-gray-100">
              Desktop Notifications
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.desktopNotifications}
                class="sr-only peer"
              />
              <div
                class="w-11 h-6 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-primary-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
          </div>

          <!-- Sound Effects -->
          <div
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg"
          >
            <div class="text-sm text-gray-900 dark:text-gray-100">
              Sound Effects
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={settings.soundEffects}
                class="sr-only peer"
              />
              <div
                class="w-11 h-6 bg-gray-300 dark:bg-gray-600 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-primary-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-600"
              ></div>
            </label>
          </div>
        </div>
      </div>
    {/snippet}
  </ModernCard>

  <!-- Save Button -->
  <div class="flex justify-end">
    <ModernButton variant="gradient" onclick={saveSettings} disabled={loading}>
      <i class="bi bi-check-lg mr-2"></i>
      {loading ? "Saving..." : "Save Changes"}
    </ModernButton>
  </div>
</div>
