<script>
  import GeneralSettings from "./GeneralSettings.svelte";
  import UsersSettings from "./UsersSettings.svelte";
  import StorageSettings from "./StorageSettings.svelte";
  import BackupSettings from "./BackupSettings.svelte";
  import PerformanceSettings from "./PerformanceSettings.svelte";
  import AboutSettings from "./AboutSettings.svelte";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

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
      id: "performance",
      label: "Performance",
      icon: "speedometer2",
      keywords: ["performance", "cache", "optimization", "speed"],
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

<PageWrapper gradient>
  <div class="page-fade-in">
    <PageHeader
      title="Settings"
      subtitle="Configure your SyncSpace preferences and system settings"
      icon="gear-fill"
    >
      {#snippet actions()}
        <!-- Search Bar -->
        <div class="relative w-full md:w-80">
          <i
            class="absolute left-3 top-1/2 -translate-y-1/2 bi bi-search text-gray-400"
          ></i>
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search settings..."
            class="glass-input w-full pl-10 pr-10"
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
      {/snippet}
    </PageHeader>

    <!-- Tab Navigation -->
    <ModernCard variant="glass" class="mb-6">
      <div class="flex flex-wrap gap-2 p-2">
        {#each filteredTabs as tab}
          <ModernButton
            variant={activeTab === tab.id ? "primary" : "ghost"}
            onclick={() => handleTabChange(tab.id)}
            class="hover-lift"
          >
            <i class="bi bi-{tab.icon} mr-2"></i>
            {tab.label}
          </ModernButton>
        {/each}
      </div>
    </ModernCard>

    {#if filteredTabs.length === 0}
      <!-- No Results State -->
      <ModernCard variant="glass">
        <div class="text-center py-16">
          <i class="bi bi-search text-8xl text-gray-300 dark:text-gray-600 mb-6"
          ></i>
          <h3 class="text-2xl font-bold mb-3 text-gray-900 dark:text-gray-100">
            No settings found
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Try searching for something else
          </p>
          <ModernButton variant="gradient" onclick={clearSearch}>
            <i class="bi bi-x-lg mr-2"></i>
            Clear Search
          </ModernButton>
        </div>
      </ModernCard>
    {:else}
      <!-- Tab Content -->
      <ModernCard variant="glass">
        <div class="p-6">
          {#if activeTab === "general"}
            <GeneralSettings />
          {:else if activeTab === "users"}
            <UsersSettings />
          {:else if activeTab === "storage"}
            <StorageSettings />
          {:else if activeTab === "backup"}
            <BackupSettings />
          {:else if activeTab === "performance"}
            <PerformanceSettings />
          {:else if activeTab === "about"}
            <AboutSettings />
          {/if}
        </div>
      </ModernCard>
    {/if}
  </div>
</PageWrapper>
