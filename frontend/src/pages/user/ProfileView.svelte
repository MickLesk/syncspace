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
  let activeTab = $state("profile"); // profile | settings | storage
  let storage = $state({
    used: 0,
    quota: 10 * 1024 ** 3, // 10GB default
    breakdown: [],
  });

  onMount(async () => {
    await loadUserProfile();
    await loadUserSettings();
    await loadRecentFiles();
    await loadStorageInfo();
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

  async function loadStorageInfo() {
    try {
      const profile = await api.users.getProfile();
      storage.used = profile.storage_used_bytes || 0;
      storage.quota = profile.storage_quota_bytes || 10 * 1024 ** 3;

      // Calculate file type breakdown (mock data - replace with real API)
      storage.breakdown = [
        { type: "Images", size: storage.used * 0.4, color: "#3b82f6" },
        { type: "Videos", size: storage.used * 0.3, color: "#8b5cf6" },
        { type: "Documents", size: storage.used * 0.2, color: "#10b981" },
        { type: "Other", size: storage.used * 0.1, color: "#f59e0b" },
      ];
    } catch (err) {
      console.error("[Profile] Failed to load storage info:", err);
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

  async function reindexSearch() {
    try {
      showToast("Rebuilding search index...", "info");
      const response = await fetch("http://localhost:8080/api/search/reindex", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const result = await response.json();
      showToast(
        `Search index rebuilt: ${result.files_indexed} files indexed`,
        "success"
      );
    } catch (err) {
      console.error("[Profile] Failed to rebuild search index:", err);
      showToast("Failed to rebuild search index", "error");
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
        class:border-primary-500={activeTab === "profile"}
        class:text-primary-600={activeTab === "profile"}
        class:dark:text-primary-400={activeTab === "profile"}
        class:border-transparent={activeTab !== "profile"}
        class:text-gray-600={activeTab !== "profile"}
        class:dark:text-gray-400={activeTab !== "profile"}
        onclick={() => (activeTab = "profile")}
      >
        <i class="bi bi-person-circle mr-2"></i>
        Profile
      </button>
      <button
        type="button"
        class="px-6 py-3 font-semibold transition-colors border-b-2 -mb-0.5"
        class:border-primary-500={activeTab === "settings"}
        class:text-primary-600={activeTab === "settings"}
        class:dark:text-primary-400={activeTab === "settings"}
        class:border-transparent={activeTab !== "settings"}
        class:text-gray-600={activeTab !== "settings"}
        class:dark:text-gray-400={activeTab !== "settings"}
        onclick={() => (activeTab = "settings")}
      >
        <i class="bi bi-gear-fill mr-2"></i>
        Settings
      </button>
      <button
        type="button"
        class="px-6 py-3 font-semibold transition-colors border-b-2 -mb-0.5"
        class:border-primary-500={activeTab === "storage"}
        class:text-primary-600={activeTab === "storage"}
        class:dark:text-primary-400={activeTab === "storage"}
        class:border-transparent={activeTab !== "storage"}
        class:text-gray-600={activeTab !== "storage"}
        class:dark:text-gray-400={activeTab !== "storage"}
        onclick={() => (activeTab = "storage")}
      >
        <i class="bi bi-hdd-fill mr-2"></i>
        Storage
      </button>
    </div>

    {#if activeTab === "profile"}
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
    {:else if activeTab === "settings"}
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

        <!-- Admin Tools -->
        <ModernCard variant="glass">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">
            <i class="bi bi-tools mr-2"></i>
            Admin Tools
          </h2>

          <div class="space-y-4">
            <div
              class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"
            >
              <h3
                class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2"
              >
                <i class="bi bi-search mr-2"></i>
                Search Index
              </h3>
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
                Rebuild the search index to include all existing files. This may
                take a few moments.
              </p>
              <ModernButton variant="outline" onclick={reindexSearch}>
                <i class="bi bi-arrow-clockwise mr-2"></i>
                Rebuild Search Index
              </ModernButton>
            </div>
          </div>
        </ModernCard>
      </div>
    {:else if activeTab === "storage"}
      <!-- Storage Tab -->
      <div class="space-y-6">
        <!-- Storage Overview -->
        <ModernCard variant="glass">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">
            <i class="bi bi-hdd-fill mr-2"></i>
            Storage Usage
          </h2>

          <!-- Storage Bar -->
          <div class="mb-6">
            <div class="flex justify-between text-sm mb-2">
              <span class="text-gray-700 dark:text-gray-300">
                {formatBytes(storage.used)} used
              </span>
              <span class="text-gray-600 dark:text-gray-400">
                {formatBytes(storage.quota)} total
              </span>
            </div>
            <div
              class="w-full h-4 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden"
            >
              <div
                class="h-full bg-gradient-to-r from-primary-500 to-primary-600 transition-all duration-500"
                style="width: {((storage.used / storage.quota) * 100).toFixed(
                  1
                )}%"
              ></div>
            </div>
            <div class="mt-2 text-center">
              <span class="text-2xl font-bold text-gray-900 dark:text-gray-100">
                {((storage.used / storage.quota) * 100).toFixed(1)}%
              </span>
              <span class="text-gray-600 dark:text-gray-400 ml-2">
                of storage used
              </span>
            </div>
          </div>

          <!-- File Type Breakdown -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
              Storage Breakdown
            </h3>
            {#each storage.breakdown as item}
              <div class="flex items-center gap-4">
                <div
                  class="w-4 h-4 rounded-full"
                  style="background-color: {item.color}"
                ></div>
                <div class="flex-1">
                  <div class="flex justify-between mb-1">
                    <span
                      class="text-sm font-medium text-gray-900 dark:text-gray-100"
                    >
                      {item.type}
                    </span>
                    <span class="text-sm text-gray-600 dark:text-gray-400">
                      {formatBytes(item.size)}
                    </span>
                  </div>
                  <div
                    class="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden"
                  >
                    <div
                      class="h-full transition-all duration-500"
                      style="width: {((item.size / storage.used) * 100).toFixed(
                        1
                      )}%; background-color: {item.color}"
                    ></div>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </ModernCard>

        <!-- Quick Actions -->
        <ModernCard variant="glass">
          <h3
            class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4"
          >
            Storage Management
          </h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <ModernButton variant="outline" class="justify-start">
              <i class="bi bi-trash mr-2"></i>
              Clear Trash
            </ModernButton>
            <ModernButton variant="outline" class="justify-start">
              <i class="bi bi-arrow-clockwise mr-2"></i>
              Optimize Storage
            </ModernButton>
            <ModernButton variant="outline" class="justify-start">
              <i class="bi bi-download mr-2"></i>
              Download All Files
            </ModernButton>
            <ModernButton variant="outline" class="justify-start">
              <i class="bi bi-arrow-up-circle mr-2"></i>
              Upgrade Storage
            </ModernButton>
          </div>
        </ModernCard>
      </div>
    {/if}
  </div>
</PageWrapper>
