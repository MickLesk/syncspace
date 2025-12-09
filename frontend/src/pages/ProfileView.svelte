<script>
  import { onMount } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import { showToast } from "../stores/toast.js";
  import PageWrapper from "../components/PageWrapper.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";

  let user = $state({
    username: "admin",
    email: "admin@syncspace.com",
    displayName: "Administrator",
    bio: "System administrator and cloud storage enthusiast",
    avatar: null,
    coverPhoto: null,
    joinedDate: new Date("2024-01-15"),
    stats: { files: 1247, storage: 15.4 * 1024 ** 3, shares: 23, uploads: 892 },
  });

  let recentFiles = $state([
    {
      id: 1,
      name: "Document.pdf",
      size: 2.4 * 1024 ** 2,
      modified: new Date(Date.now() - 30 * 60 * 1000),
    },
    {
      id: 2,
      name: "Image.png",
      size: 5.2 * 1024 ** 2,
      modified: new Date(Date.now() - 2 * 60 * 60 * 1000),
    },
    {
      id: 3,
      name: "Video.mp4",
      size: 124 * 1024 ** 2,
      modified: new Date(Date.now() - 24 * 60 * 60 * 1000),
    },
  ]);

  let editMode = $state(false);

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
      showToast(tr("avatarUpdated"), "success");
    }
  }

  function saveProfile() {
    editMode = false;
    showToast(tr("profileSavedSuccessfully"), "success");
  }
</script>

<PageWrapper>
  <!-- Animated Background Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <div class="relative z-10 max-w-6xl mx-auto">
    <!-- Cover Photo -->
    <div
      class="relative h-64 rounded-xl overflow-hidden mb-8 animate-slide-up"
    >
      {#if user.coverPhoto}
        <img
          src={user.coverPhoto}
          alt="Cover"
          class="w-full h-full object-cover"
        />
      {:else}
        <div class="w-full h-full gradient-bg-primary"></div>
      {/if}
      <div class="absolute inset-0 bg-black/20"></div>
      <ModernButton
        variant="ghost"
        size="sm"
        icon="camera"
        class="absolute top-4 right-4 backdrop-blur-md bg-white/20 text-white hover:bg-white/30"
      >
        Change Cover
      </ModernButton>
    </div>

    <!-- Profile Card -->
    <div class="animate-slide-up" style="animation-delay: 100ms;">
      <ModernCard variant="glass" class="-mt-32 mb-8">
        {#snippet children()}
          <div class="text-center pt-20 pb-6">
            <!-- Avatar -->
            <div class="relative inline-block mb-4">
              <div
                class="w-40 h-40 rounded-full border-4 border-base-100 overflow-hidden shadow-2xl"
              >
                {#if user.avatar}
                  <img
                    src={user.avatar}
                    alt={user.displayName}
                    class="w-full h-full object-cover"
                  />
                {:else}
                  <div
                    class="w-full h-full gradient-bg-primary flex items-center justify-center text-white text-5xl font-bold"
                  >
                    {getInitials(user.displayName)}
                  </div>
                {/if}
              </div>
              <label
                class="absolute bottom-0 right-0 w-12 h-12 rounded-full bg-primary text-primary-content border-4 border-base-100 flex items-center justify-center cursor-pointer hover:scale-110 transition-transform shadow-lg"
              >
                <i class="bi bi-camera-fill" aria-hidden="true"></i>
                <input
                  type="file"
                  accept="image/*"
                  onchange={handleAvatarUpload}
                  hidden
                />
              </label>
            </div>

            <!-- Profile Info -->
            {#if editMode}
              <div class="space-y-3 max-w-md mx-auto">
                <input
                  type="text"
                  bind:value={user.displayName}
                  class="input input-bordered glass-input w-full"
                  placeholder="Display Name"
                />
                <textarea
                  bind:value={user.bio}
                  class="textarea textarea-bordered glass-input w-full"
                  rows="2"
                  placeholder="Bio"
                ></textarea>
              </div>
            {:else}
              <h1 class="text-4xl font-bold mb-2">{user.displayName}</h1>
              <p class="text-base-content/60 mb-3">@{user.username}</p>
              <p class="text-base-content/80 max-w-2xl mx-auto mb-4">
                {user.bio}
              </p>
            {/if}

            <!-- Meta Info -->
            <div
              class="flex gap-6 justify-center text-sm text-base-content/60 mb-6"
            >
              <span class="flex items-center gap-2">
                <i class="bi bi-envelope" aria-hidden="true"></i>
                {user.email}
              </span>
              <span class="flex items-center gap-2">
                <i class="bi bi-calendar" aria-hidden="true"></i>
                Joined {formatDate(user.joinedDate)}
              </span>
            </div>

            <!-- Actions -->
            <div class="flex gap-3 justify-center">
              {#if editMode}
                <ModernButton
                  variant="gradient"
                  icon="check-lg"
                  onclick={saveProfile}
                >
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
                  icon="pencil"
                  onclick={() => (editMode = true)}
                >
                  Edit Profile
                </ModernButton>
              {/if}
            </div>
          </div>
        {/snippet}
      </ModernCard>
    </div>

    <!-- Stats Grid -->
    <div
      class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8 animate-slide-up"
      style="animation-delay: 200ms;"
    >
      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center py-6">
            <i class="bi bi-files text-5xl mb-3 block" aria-hidden="true"></i>
            <div class="text-4xl font-bold mb-1">
              {user.stats.files.toLocaleString()}
            </div>
            <div class="text-sm opacity-90">Files</div>
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center py-6">
            <i class="bi bi-hdd text-5xl mb-3 block" aria-hidden="true"></i>
            <div class="text-4xl font-bold mb-1">
              {formatBytes(user.stats.storage)}
            </div>
            <div class="text-sm opacity-90">Storage Used</div>
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center py-6">
            <i class="bi bi-share text-5xl mb-3 block" aria-hidden="true"></i>
            <div class="text-4xl font-bold mb-1">{user.stats.shares}</div>
            <div class="text-sm opacity-90">Shares</div>
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center py-6">
            <i class="bi bi-upload text-5xl mb-3 block" aria-hidden="true"></i>
            <div class="text-4xl font-bold mb-1">{user.stats.uploads}</div>
            <div class="text-sm opacity-90">Uploads This Month</div>
          </div>
        {/snippet}
      </ModernCard>
    </div>

    <!-- Recent Files -->
    <div class="animate-slide-up" style="animation-delay: 300ms;">
      <ModernCard variant="glass">
        {#snippet children()}
          <h2 class="text-2xl font-bold mb-6 flex items-center gap-3">
            <i class="bi bi-clock-history text-primary" aria-hidden="true"></i>
            Recent Files
          </h2>

          <div class="space-y-2">
            {#each recentFiles as file, i (file.id)}
              <div class="animate-fade-in" style="animation-delay: {i * 50}ms;">
                <div
                  class="glass-card p-4 rounded-xl hover:scale-[1.02] transition-transform cursor-pointer"
                >
                  <div class="flex items-center gap-4">
                    <i
                      class="bi bi-file-earmark-fill text-3xl text-primary"
                      aria-hidden="true"
                    ></i>
                    <div class="flex-1">
                      <div class="font-semibold text-base-content">
                        {file.name}
                      </div>
                      <div class="text-sm text-base-content/60">
                        {formatBytes(file.size)} • {formatDate(file.modified)}
                      </div>
                    </div>
                    <button
                      class="btn btn-ghost btn-sm btn-square"
                      aria-label="File options"
                    >
                      <i class="bi bi-three-dots-vertical" aria-hidden="true"
                      ></i>
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/snippet}
      </ModernCard>
    </div>
  </div>
</PageWrapper>

<style>
  /* ProfileView styles */
</style>
