<script>
  import GeneralSettings from "./GeneralSettings.svelte";
  import UsersSettings from "./UsersSettings.svelte";
  import StorageSettings from "./StorageSettings.svelte";
  import BackupSettings from "./BackupSettings.svelte";
  import AboutSettings from "./AboutSettings.svelte";

  let activeTab = $state("general");
  let searchQuery = $state("");

  const tabs = [
    {
      id: "general",
      label: "General",
      icon: "sliders",
      keywords: ["language", "theme", "notifications", "general"],
    },
    {
      id: "users",
      label: "Users",
      icon: "people-fill",
      keywords: ["users", "accounts", "permissions", "roles"],
    },
    {
      id: "storage",
      label: "Storage",
      icon: "hdd-fill",
      keywords: ["storage", "quota", "disk", "space"],
    },
    {
      id: "backup",
      label: "Backup",
      icon: "cloud-arrow-up-fill",
      keywords: ["backup", "restore", "schedule", "retention"],
    },
    {
      id: "about",
      label: "About",
      icon: "info-circle",
      keywords: ["about", "version", "info", "credits"],
    },
  ];

  // Filter tabs based on search query
  let filteredTabs = $derived(
    searchQuery.length === 0
      ? tabs
      : tabs.filter(
          (tab) =>
            tab.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
            tab.keywords.some((keyword) =>
              keyword.toLowerCase().includes(searchQuery.toLowerCase())
            )
        )
  );

  // Auto-switch to first filtered tab if current tab is filtered out
  $effect(() => {
    if (
      searchQuery.length > 0 &&
      filteredTabs.length > 0 &&
      !filteredTabs.some((t) => t.id === activeTab)
    ) {
      activeTab = filteredTabs[0].id;
    }
  });

  function handleTabChange(tab) {
    activeTab = tab;
  }

  function clearSearch() {
    searchQuery = "";
  }
</script>

<!-- Main Container -->
<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
>
  <!-- Animated Background Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Content Container -->
  <div class="relative z-10 max-w-7xl mx-auto">
    <!-- Settings Header -->
    <div class="glass-card mb-6 p-6">
      <div class="flex flex-wrap items-center justify-between gap-6">
        <div class="flex-1 min-w-[250px]">
          <h1
            class="text-3xl font-bold gradient-text mb-2 flex items-center gap-3"
          >
            <i class="bi bi-gear-fill"></i>
            Settings
          </h1>
          <p class="text-gray-600 dark:text-gray-400">
            Configure your SyncSpace preferences and system settings
          </p>
        </div>

        <!-- Search Bar -->
        <div class="relative w-full md:w-80">
          <i
            class="absolute left-3 top-1/2 -translate-y-1/2 bi bi-search text-gray-400"
          ></i>
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search settings..."
            class="w-full pl-10 pr-10 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
          />
          {#if searchQuery.length > 0}
            <button
              onclick={clearSearch}
              class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
              aria-label="Clear search"
            >
              <i class="bi bi-x-lg"></i>
            </button>
          {/if}
        </div>
      </div>
    </div>

    <!-- Tab Navigation -->
    <div class="glass-card mb-6 p-2">
      <div class="flex flex-wrap gap-2">
        {#each filteredTabs as tab}
          <button
            onclick={() => handleTabChange(tab.id)}
            class="flex items-center gap-2 px-4 py-2.5 rounded-lg font-medium transition-all"
            class:bg-gradient-to-r={activeTab === tab.id}
            class:from-blue-500={activeTab === tab.id}
            class:to-purple-600={activeTab === tab.id}
            class:text-white={activeTab === tab.id}
            class:shadow-lg={activeTab === tab.id}
            class:text-gray-700={activeTab !== tab.id}
            class:dark:text-gray-300={activeTab !== tab.id}
            class:hover:bg-gray-100={activeTab !== tab.id}
            class:dark:hover:bg-gray-800={activeTab !== tab.id}
          >
            <i class="bi bi-{tab.icon}"></i>
            <span>{tab.label}</span>
          </button>
        {/each}
      </div>
    </div>

    {#if filteredTabs.length === 0}
      <!-- No Results State -->
      <div class="glass-card text-center py-16">
        <div class="animate-fade-in">
          <i class="bi bi-search text-8xl text-gray-300 dark:text-gray-600 mb-6"
          ></i>
          <h3 class="text-2xl font-bold mb-3 text-gray-900 dark:text-gray-100">
            No settings found
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Try searching for something else
          </p>
          <button
            onclick={clearSearch}
            class="px-6 py-3 rounded-lg font-medium bg-gradient-to-r from-blue-500 to-purple-600 text-white hover:from-blue-600 hover:to-purple-700 shadow-lg hover:shadow-xl transition-all inline-flex items-center gap-2"
          >
            <i class="bi bi-x-lg"></i>
            Clear Search
          </button>
        </div>
      </div>
    {:else}
      <!-- Tab Content -->
      <div class="glass-card p-6">
        {#if activeTab === "general"}
          <GeneralSettings />
        {:else if activeTab === "users"}
          <UsersSettings />
        {:else if activeTab === "storage"}
          <StorageSettings />
        {:else if activeTab === "backup"}
          <BackupSettings />
        {:else if activeTab === "about"}
          <AboutSettings />
        {/if}
      </div>
    {/if}
  </div>
</div>

