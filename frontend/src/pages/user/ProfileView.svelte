<script>
  import { onMount } from "svelte";
  import { showToast } from "../../stores/toast.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

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
      showToast("Avatar updated", "success");
    }
  }

  function saveProfile() {
    editMode = false;
    showToast("Profile saved", "success");
  }
</script>

<PageWrapper>
  <div class="space-y-6">
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
                <p class="text-gray-700 dark:text-gray-300 mb-4">{user.bio}</p>
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
          <div class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1">
            {user.stats.files.toLocaleString()}
          </div>
          <div class="text-sm text-gray-600 dark:text-gray-400">Files</div>
        </ModernCard>

        <ModernCard variant="glass" hoverable class="text-center p-6">
          <i
            class="bi bi-hdd text-4xl text-primary-600 dark:text-primary-400 mb-3 block"
          ></i>
          <div class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1">
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
          <div class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1">
            {user.stats.shares}
          </div>
          <div class="text-sm text-gray-600 dark:text-gray-400">Shares</div>
        </ModernCard>

        <ModernCard variant="glass" hoverable class="text-center p-6">
          <i
            class="bi bi-upload text-4xl text-primary-600 dark:text-primary-400 mb-3 block"
          ></i>
          <div class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1">
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
          <i class="bi bi-clock-history text-primary-600 dark:text-primary-400"
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
  </div></PageWrapper
>
