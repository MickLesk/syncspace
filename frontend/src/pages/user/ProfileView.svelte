<script>
  import { onMount } from "svelte";
  import { showToast } from "../../stores/toast.js";
  import { theme, language } from "../../stores/ui.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import api from "../../lib/api.js";

  let user = $state({
    username: "Loading...",
    email: "",
    displayName: "",
    bio: "",
    avatar: null,
    coverPhoto: null,
    joinedDate: new Date(),
    stats: { files: 0, storage: 0, shares: 0, uploads: 0 },
  });

  let loading = $state(true);
  let settings = $state({
    theme: $theme,
    language: $language,
    defaultView: "grid",
  });

  let recentFiles = $state([]);
  let editMode = $state(false);
  let settingsTab = $state(false); // Toggle between Profile and Settings view

  onMount(async () => {
    await loadUserProfile();
    await loadUserSettings();
    await loadRecentFiles();
  });

  async function loadUserProfile() {
    try {
      const profile = await api.users.getProfile();
      user = {
        username: profile.username || user.username,
        email: profile.email || "",
        displayName: profile.display_name || profile.username,
        bio: profile.bio || "",
        avatar: profile.avatar_base64 || null,
        coverPhoto: null,
        joinedDate: profile.created_at
          ? new Date(profile.created_at)
          : new Date(),
        stats: {
          files: profile.file_count || 0,
          storage: profile.storage_used_bytes || 0,
          shares: profile.share_count || 0,
          uploads: profile.upload_count || 0,
        },
      };
    } catch (err) {
      console.error("[Profile] Failed to load profile:", err);
      showToast("Failed to load profile", "error");
    } finally {
      loading = false;
    }
  }

  async function loadUserSettings() {
    try {
      const userSettings = await api.users.getSettings();
      settings = {
        theme: userSettings.theme || "light",
        language: userSettings.language || "en",
        defaultView: userSettings.default_view || "grid",
      };
    } catch (err) {
      console.error("[Profile] Failed to load settings:", err);
    }
  }

  async function loadRecentFiles() {
    try {
      const files = await api.recent.list(5);
      recentFiles = files.slice(0, 5).map((f) => ({
        id: f.file_path,
        name: f.name,
        size: f.size_bytes,
        modified: new Date(f.accessed_at),
      }));
    } catch (err) {
      console.error("[Profile] Failed to load recent files:", err);
    }
  }

  function formatBytes(bytes) {
    return `${(bytes / 1024 ** 3).toFixed(1)} GB`;
  }

  function formatDate(date) {
    return date.toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function getInitials(name) {
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  }

  function handleAvatarUpload(e) {
    const file = e.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        user.avatar = e.target.result;
      };
      reader.readAsDataURL(file);
      showToast("Avatar updated", "success");
    }
  }

  async function saveProfile() {
    try {
      await api.users.updateProfile({
        display_name: user.displayName,
        email: user.email,
        avatar_base64: user.avatar,
      });
      editMode = false;
      showToast("Profile saved successfully", "success");
    } catch (err) {
      console.error("[Profile] Failed to save profile:", err);
      showToast("Failed to save profile", "error");
    }
  }

  async function saveSettings() {
    try {
      await api.users.updateSettings({
        theme: settings.theme,
        language: settings.language,
        default_view: settings.defaultView,
      });

      // Update global stores
      theme.set(settings.theme);
      language.set(settings.language);

      showToast("Settings saved successfully", "success");
    } catch (err) {
      console.error("[Profile] Failed to save settings:", err);
      showToast("Failed to save settings", "error");
    }
  }
</script>

<PageWrapper>
  <div class="space-y-6">
    <!-- Tab Navigation -->
    <div class="flex gap-4 border-b-2 border-gray-200 dark:border-gray-700">
      <button
        type="button"
        class="px-6 py-3 font-semibold transition-colors border-b-2 -mb-0.5"
        class:border-primary-500={!settingsTab}
        class:text-primary-600={!settingsTab}
        class:dark:text-primary-400={!settingsTab}
        class:border-transparent={settingsTab}
        class:text-gray-600={settingsTab}
        class:dark:text-gray-400={settingsTab}
        onclick={() => (settingsTab = false)}
      >
        <i class="bi bi-person-circle mr-2"></i>
        Profile
      </button>
      <button
        type="button"
        class="px-6 py-3 font-semibold transition-colors border-b-2 -mb-0.5"
        class:border-primary-500={settingsTab}
        class:text-primary-600={settingsTab}
        class:dark:text-primary-400={settingsTab}
        class:border-transparent={!settingsTab}
        class:text-gray-600={!settingsTab}
        class:dark:text-gray-400={!settingsTab}
        onclick={() => (settingsTab = true)}
      >
        <i class="bi bi-gear-fill mr-2"></i>
        Settings
      </button>
    </div>

    {#if !settingsTab}
      <!-- Profile Tab -->
      <div class="space-y-6">
        <!-- Cover Photo & Profile -->
        <div class="relative">
          <!-- Cover Photo -->
          <div class="relative h-48 md:h-64 rounded-2xl overflow-hidden">
            {#if user.coverPhoto}
              <img
                src={user.coverPhoto}
                alt="Cover"
                class="w-full h-full object-cover"
              />
            {:else}
              <div
                class="w-full h-full bg-gradient-to-br from-primary-500 via-primary-600 to-secondary-600"
              ></div>
            {/if}
            <div class="absolute inset-0 bg-black/20"></div>
            <ModernButton
              variant="ghost"
              size="sm"
              class="absolute top-4 right-4 backdrop-blur-md bg-white/20 text-white hover:bg-white/30"
            >
              <i class="bi bi-camera mr-1"></i>
              Change Cover
            </ModernButton>
          </div>

          <!-- Profile Card -->
          <ModernCard variant="glass" class="-mt-20 mx-4 md:mx-8">
            <div class="flex flex-col md:flex-row items-center gap-6 p-6">
              <!-- Avatar -->
              <div class="relative -mt-16 md:mt-0">
                <div
                  class="w-32 h-32 rounded-full border-4 border-white dark:border-gray-900 overflow-hidden shadow-xl"
                >
                  {#if user.avatar}
                    <img
                      src={user.avatar}
                      alt={user.displayName}
                      class="w-full h-full object-cover"
                    />
                  {:else}
                    <div
                      class="w-full h-full bg-gradient-to-br from-primary-500 to-primary-600 flex items-center justify-center text-white text-4xl font-bold"
                    >
                      {getInitials(user.displayName)}
                    </div>
                  {/if}
                </div>
                <label
                  class="absolute bottom-0 right-0 w-10 h-10 rounded-full gradient-bg-primary border-4 border-white dark:border-gray-900 flex items-center justify-center cursor-pointer hover:scale-110 transition-transform shadow-lg"
                >
                  <i class="bi bi-camera-fill text-white"></i>
                  <input
                    type="file"
                    accept="image/*"
                    onchange={handleAvatarUpload}
                    hidden
                  />
                </label>
              </div>

              <!-- Profile Info -->
              <div class="flex-1 text-center md:text-left">
                {#if editMode}
                  <div class="space-y-3">
                    <input
                      type="text"
                      bind:value={user.displayName}
                      class="glass-input w-full"
                      placeholder="Display Name"
                    />
                    <textarea
                      bind:value={user.bio}
                      class="glass-input w-full"
                      rows="2"
                      placeholder="Bio"
                    ></textarea>
                  </div>
                {:else}
                  <h1
                    class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1"
                  >
                    {user.displayName}
                  </h1>
                  <p class="text-gray-600 dark:text-gray-400 mb-2">
                    @{user.username}
                  </p>
                  <p class="text-gray-700 dark:text-gray-300 mb-4">
                    {user.bio}
                  </p>
                {/if}

                <div
                  class="flex flex-wrap gap-4 justify-center md:justify-start text-sm text-gray-600 dark:text-gray-400 mb-4"
                >
                  <span class="flex items-center gap-2">
                    <i class="bi bi-envelope"></i>
                    {user.email}
                  </span>
                  <span class="flex items-center gap-2">
                    <i class="bi bi-calendar"></i>
                    Joined {formatDate(user.joinedDate)}
                  </span>
                </div>

                <div class="flex gap-3 justify-center md:justify-start">
                  {#if editMode}
                    <ModernButton variant="gradient" onclick={saveProfile}>
                      <i class="bi bi-check-lg mr-2"></i>
                      Save Changes
                    </ModernButton>
                    <ModernButton
                      variant="ghost"
                      onclick={() => (editMode = false)}
                    >
                      Cancel
                    </ModernButton>
                  {:else}
                    <ModernButton
                      variant="gradient"
                      onclick={() => (editMode = true)}
                    >
                      <i class="bi bi-pencil mr-2"></i>
                      Edit Profile
                    </ModernButton>
                  {/if}
                </div>
              </div>
            </div>
          </ModernCard>
        </div>

        <!-- Stats Grid -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <ModernCard variant="glass" hoverable class="text-center p-6">
            <i
              class="bi bi-files text-4xl text-primary-600 dark:text-primary-400 mb-3 block"
            ></i>
            <div
              class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1"
            >
              {user.stats.files.toLocaleString()}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-400">Files</div>
          </ModernCard>

          <ModernCard variant="glass" hoverable class="text-center p-6">
            <i
              class="bi bi-hdd text-4xl text-primary-600 dark:text-primary-400 mb-3 block"
            ></i>
            <div
              class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1"
            >
              {formatBytes(user.stats.storage)}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-400">
              Storage Used
            </div>
          </ModernCard>

          <ModernCard variant="glass" hoverable class="text-center p-6">
            <i
              class="bi bi-share text-4xl text-primary-600 dark:text-primary-400 mb-3 block"
            ></i>
            <div
              class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1"
            >
              {user.stats.shares}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-400">Shares</div>
          </ModernCard>

          <ModernCard variant="glass" hoverable class="text-center p-6">
            <i
              class="bi bi-upload text-4xl text-primary-600 dark:text-primary-400 mb-3 block"
            ></i>
            <div
              class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1"
            >
              {user.stats.uploads}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-400">
              Uploads This Month
            </div>
          </ModernCard>
        </div>

        <!-- Recent Files -->
        <ModernCard variant="glass">
          <h2
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6 flex items-center gap-3"
          >
            <i
              class="bi bi-clock-history text-primary-600 dark:text-primary-400"
            ></i>
            Recent Files
          </h2>

          <div class="space-y-3">
            {#each recentFiles as file (file.id)}
              <div
                class="bg-white/50 dark:bg-gray-800/50 p-4 rounded-xl border border-gray-200/50 dark:border-gray-700/50 hover:scale-[1.01] transition-transform cursor-pointer"
              >
                <div class="flex items-center gap-4">
                  <i
                    class="bi bi-file-earmark-fill text-3xl text-primary-600 dark:text-primary-400"
                  ></i>
                  <div class="flex-1">
                    <div class="font-semibold text-gray-900 dark:text-gray-100">
                      {file.name}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                      {formatBytes(file.size)} • {formatDate(file.modified)}
                    </div>
                  </div>
                  <ModernButton variant="ghost" size="sm">
                    <i class="bi bi-three-dots-vertical"></i>
                  </ModernButton>
                </div>
              </div>
            {/each}
          </div>
        </ModernCard>
      </div>
    {:else}
      <!-- Settings Tab -->
      <div class="space-y-6">
        <ModernCard variant="glass">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">
            <i class="bi bi-gear-fill mr-2"></i>
            Preferences
          </h2>

          <div class="space-y-6">
            <!-- Theme Setting -->
            <div>
              <label
                class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
              >
                Theme
              </label>
              <select
                bind:value={settings.theme}
                class="w-full px-4 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100"
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="auto">Auto (System)</option>
              </select>
            </div>

            <!-- Language Setting -->
            <div>
              <label
                class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
              >
                Language
              </label>
              <select
                bind:value={settings.language}
                class="w-full px-4 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100"
              >
                <option value="en">English</option>
                <option value="de">Deutsch</option>
                <option value="fr">Français</option>
                <option value="es">Español</option>
              </select>
            </div>

            <!-- Default View Setting -->
            <div>
              <label
                class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
              >
                Default File View
              </label>
              <select
                bind:value={settings.defaultView}
                class="w-full px-4 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100"
              >
                <option value="grid">Grid View</option>
                <option value="list">List View</option>
              </select>
            </div>

            <!-- Save Button -->
            <div class="flex justify-end gap-3 pt-4">
              <ModernButton variant="gradient" onclick={saveSettings}>
                <i class="bi bi-check-lg mr-2"></i>
                Save Settings
              </ModernButton>
            </div>
          </div>
        </ModernCard>
      </div>
    {/if}
  </div>
</PageWrapper>
