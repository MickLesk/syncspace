<script>
  import { onMount } from "svelte";
  import UIInput from "../../components/ui/UIInput.svelte";
  import UITextarea from "../../components/ui/UITextarea.svelte";
  import { showToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let user = $state({
    username: "@Loading...",
    email: "",
    displayName: "",
    bio: "",
    avatar: null,
    joinedDate: new Date(),
    stats: { files: 0, storage: 0, shares: 0, uploads: 0 },
  });

  let loading = $state(true);
  let editMode = $state(false);

  onMount(async () => {
    await loadUserProfile();
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
      showToast(tr("failedToLoadProfile"), "error");
    } finally {
      loading = false;
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
    }
  }

  async function saveProfile() {
    try {
      await api.users.updateProfile({
        display_name: user.displayName,
        bio: user.bio,
        email: user.email || null, // Send null instead of empty string
        avatar_base64: user.avatar,
      });
      editMode = false;
      await loadUserProfile(); // Reload to get updated data
      showToast(tr("profileSavedSuccessfully"), "success");
    } catch (err) {
      console.error("[Profile] Failed to save profile:", err);
      showToast(tr("failedToSaveProfile"), "error");
    }
  }
</script>

<div class="max-w-4xl mx-auto p-4 space-y-4">
  <!-- Profile Card -->
  <ModernCard variant="glass">
    {#snippet children()}
      <div class="p-6">
        <div class="flex items-start gap-6">
          <!-- Avatar -->
          <div class="relative shrink-0">
            <div
              class="w-24 h-24 rounded-full overflow-hidden border-2 border-gray-200 dark:border-gray-700 shadow-md"
            >
              {#if user.avatar}
                <img
                  src={user.avatar}
                  alt={user.displayName}
                  class="w-full h-full object-cover"
                />
              {:else}
                <div
                  class="w-full h-full bg-gradient-to-br from-primary-500 to-primary-600 flex items-center justify-center text-white text-2xl font-bold"
                >
                  {getInitials(user.displayName)}
                </div>
              {/if}
            </div>
            <label
              class="absolute bottom-0 right-0 w-8 h-8 rounded-full gradient-bg-primary flex items-center justify-center cursor-pointer hover:scale-110 transition-transform shadow-md"
            >
              <i class="bi bi-camera-fill text-white text-sm" aria-hidden="true"></i>
              <input
                type="file"
                accept="image/*"
                onchange={handleAvatarUpload}
                hidden
              />
            </label>
          </div>

          <!-- Profile Info -->
          <div class="flex-1">
            {#if editMode}
              <div class="space-y-3">
                <input
                  type="text"
                  bind:value={user.displayName}
                  class="w-full px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition text-sm"
                  placeholder={tr("displayName")}
                />
                <input
                  type="email"
                  bind:value={user.email}
                  class="w-full px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition text-sm"
                  placeholder={tr("email")}
                />
                <textarea
                  bind:value={user.bio}
                  class="w-full px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition text-sm resize-none"
                  rows="2"
                  placeholder={tr("bio")}
                ></textarea>
              </div>
            {:else}
              <h1
                class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-1"
              >
                {user.displayName}
              </h1>
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">
                @{user.username}
              </p>
              <p class="text-sm text-gray-700 dark:text-gray-300 mb-3">
                {user.bio || tr("noBioYet")}
              </p>
            {/if}

            <div
              class="flex flex-wrap gap-4 text-xs text-gray-600 dark:text-gray-400 mb-4"
            >
              <span class="flex items-center gap-1">
                <i class="bi bi-envelope" aria-hidden="true"></i>
                {user.email || tr("noEmail")}
              </span>
              <span class="flex items-center gap-1">
                <i class="bi bi-calendar" aria-hidden="true"></i>
                {tr("memberSince")}
                {formatDate(user.joinedDate)}
              </span>
            </div>

            <div class="flex gap-2">
              {#if editMode}
                <ModernButton
                  variant="gradient"
                  size="sm"
                  onclick={saveProfile}
                >
                  <i class="bi bi-check-lg mr-1" aria-hidden="true"></i>
                  {tr("save")}
                </ModernButton>
                <ModernButton
                  variant="ghost"
                  size="sm"
                  onclick={() => (editMode = false)}
                >
                  {tr("cancel")}
                </ModernButton>
              {:else}
                <ModernButton
                  variant="gradient"
                  size="sm"
                  onclick={() => (editMode = true)}
                >
                  <i class="bi bi-pencil mr-1" aria-hidden="true"></i>
                  {tr("editProfile")}
                </ModernButton>
              {/if}
            </div>
          </div>
        </div>
      </div>
    {/snippet}
  </ModernCard>

  <!-- Stats Grid -->
  <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
    <ModernCard variant="glass" hoverable class="text-center p-4 hover-scale">
      {#snippet children()}
        <i
          class="bi bi-files text-3xl text-primary-600 dark:text-primary-400 mb-2 block"
        ></i>
        <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
          {user.stats.files.toLocaleString()}
        </div>
        <div class="text-xs text-gray-600 dark:text-gray-400">Files</div>
      {/snippet}
    </ModernCard>

    <ModernCard variant="glass" hoverable class="text-center p-4 hover-scale">
      {#snippet children()}
        <i
          class="bi bi-hdd text-3xl text-primary-600 dark:text-primary-400 mb-2 block"
        ></i>
        <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
          {formatBytes(user.stats.storage)}
        </div>
        <div class="text-xs text-gray-600 dark:text-gray-400">Storage</div>
      {/snippet}
    </ModernCard>

    <ModernCard variant="glass" hoverable class="text-center p-4 hover-scale">
      {#snippet children()}
        <i
          class="bi bi-share text-3xl text-primary-600 dark:text-primary-400 mb-2 block"
        ></i>
        <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
          {user.stats.shares}
        </div>
        <div class="text-xs text-gray-600 dark:text-gray-400">Shares</div>
      {/snippet}
    </ModernCard>

    <ModernCard variant="glass" hoverable class="text-center p-4 hover-scale">
      {#snippet children()}
        <i
          class="bi bi-upload text-3xl text-primary-600 dark:text-primary-400 mb-2 block"
        ></i>
        <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
          {user.stats.uploads}
        </div>
        <div class="text-xs text-gray-600 dark:text-gray-400">Uploads</div>
      {/snippet}
    </ModernCard>
  </div>
</div>
