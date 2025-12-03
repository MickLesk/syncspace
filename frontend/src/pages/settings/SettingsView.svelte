<script>
  import GeneralSettings from "./GeneralSettings.svelte";
  import SecuritySettings from "./SecuritySettings.svelte";
  import UsersSettings from "./UsersSettings.svelte";
  import StorageSettings from "./StorageSettings.svelte";
  import BackupSettings from "./BackupSettings.svelte";
  import PerformanceSettings from "./PerformanceSettings.svelte";
  import AboutSettings from "./AboutSettings.svelte";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let activeTab = $state("general");
  let searchQuery = $state("");

  const tabsConfig = [
    {
      id: "general",
      labelKey: "general",
      icon: "sliders",
      keywords: ["language", "theme", "notifications", "general"],
    },
    {
      id: "security",
      labelKey: "security",
      icon: "shield-lock-fill",
      keywords: ["security", "2fa", "password", "authentication"],
    },
    {
      id: "users",
      labelKey: "users",
      icon: "people-fill",
      keywords: ["users", "accounts", "permissions", "roles"],
    },
    {
      id: "storage",
      labelKey: "storage",
      icon: "hdd-fill",
      keywords: ["storage", "quota", "disk", "space"],
    },
    {
      id: "backup",
      labelKey: "backup",
      icon: "cloud-arrow-up-fill",
      keywords: ["backup", "restore", "schedule", "retention"],
    },
    {
      id: "performance",
      labelKey: "performance",
      icon: "speedometer2",
      keywords: ["performance", "cache", "optimization", "speed"],
    },
    {
      id: "about",
      labelKey: "about",
      icon: "info-circle",
      keywords: ["about", "version", "info", "credits"],
    },
  ];

  const tabs = $derived(
    tabsConfig.map((tab) => ({
      ...tab,
      label: tr(tab.labelKey),
    }))
  );

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
      title={tr("settings")}
      subtitle={tr("manageFiles")}
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
            placeholder={tr("searchPlaceholder")}
            class="glass-input w-full pl-10 pr-10"
          />
          {#if searchQuery.length > 0}
            <button aria-label="Close"
            ><i class="bi bi-x" aria-hidden="true"></i></button>
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
            <i class="bi bi-{tab.icon} mr-2" aria-hidden="true"></i>
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
           aria-hidden="true"></i>
          <h3 class="text-2xl font-bold mb-3 text-gray-900 dark:text-gray-100">
            {tr("noFilesFound")}
          </h3>
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            {tr("tryDifferentSearch")}
          </p>
          <ModernButton variant="gradient" onclick={clearSearch}>
            <i class="bi bi-x-lg mr-2" aria-hidden="true"></i>
            {tr("clearFilters")}
          </ModernButton>
        </div>
      </ModernCard>
    {:else}
      <!-- Tab Content -->
      <ModernCard variant="glass">
        <div class="p-6">
          {#if activeTab === "general"}
            <GeneralSettings />
          {:else if activeTab === "security"}
            <SecuritySettings />
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
