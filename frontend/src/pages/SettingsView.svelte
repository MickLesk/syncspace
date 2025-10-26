<script>
  import GeneralSettings from "./settings/GeneralSettings.svelte";
  import UsersSettings from "./settings/UsersSettings.svelte";
  import StorageSettings from "./settings/StorageSettings.svelte";
  import BackupSettings from "./settings/BackupSettings.svelte";
  import AboutSettings from "./settings/AboutSettings.svelte";

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

<div class="settings-container">
  <!-- Settings Header with Search -->
  <div class="settings-header">
    <div class="header-content">
      <div class="header-title">
        <i class="bi bi-gear-fill"></i>
        <h1>Settings</h1>
      </div>
      <div class="header-description">
        Configure your SyncSpace preferences and system settings
      </div>
    </div>

    <!-- Search Bar -->
    <div class="search-wrapper">
      <div class="input-group">
        <i class="input-icon-left bi bi-search"></i>
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search settings..."
          class="input input-bordered input-primary input-with-icon-left"
          class:input-with-icon-right={searchQuery.length > 0}
        />
        {#if searchQuery.length > 0}
          <button
            class="search-clear-btn"
            onclick={clearSearch}
            aria-label="Clear search"
          >
            <i class="bi bi-x-lg"></i>
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Tab Navigation -->
  <div class="tab-nav">
    {#each filteredTabs as tab}
      <button
        class="tab-btn"
        class:active={activeTab === tab.id}
        onclick={() => handleTabChange(tab.id)}
      >
        <i class="bi bi-{tab.icon}"></i>
        <span>{tab.label}</span>
      </button>
    {/each}
  </div>

  {#if filteredTabs.length === 0}
    <!-- No Results State -->
    <div class="no-results">
      <i class="bi bi-search"></i>
      <h3>No settings found</h3>
      <p>Try searching for something else</p>
      <button class="btn btn-primary" onclick={clearSearch}>
        <i class="bi bi-x-lg"></i>
        Clear Search
      </button>
    </div>
  {:else}
    <!-- Tab Content -->
    <div class="tab-panel">
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

<style>
  .settings-container {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .settings-header {
    margin-bottom: 2rem;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 2rem;
    flex-wrap: wrap;
  }

  .header-content {
    flex: 1;
    min-width: 250px;
  }

  .header-title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .header-title i {
    font-size: 2rem;
    color: hsl(var(--p));
  }

  .header-title h1 {
    font-size: 2rem;
    font-weight: 700;
    color: hsl(var(--bc));
    margin: 0;
  }

  .header-description {
    font-size: 0.9375rem;
    color: hsl(var(--bc) / 0.6);
    margin-left: 2.75rem;
  }

  .search-wrapper {
    flex: 0 1 400px;
    min-width: 250px;
  }

  .search-clear-btn {
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: none;
    background: hsl(var(--bc) / 0.1);
    color: hsl(var(--bc) / 0.6);
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .search-clear-btn:hover {
    background: hsl(var(--bc) / 0.2);
    color: hsl(var(--bc));
    transform: translateY(-50%) scale(1.1);
  }

  .search-clear-btn i {
    font-size: 0.75rem;
  }

  .tab-nav {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    background: hsl(var(--b2));
    border-radius: 1rem;
    margin-bottom: 2rem;
    border: 2px solid hsl(var(--bc) / 0.05);
    flex-wrap: wrap;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.875rem 1.5rem;
    border: none;
    background: transparent;
    color: hsl(var(--bc) / 0.7);
    border-radius: 0.75rem;
    font-size: 0.9375rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
  }

  .tab-btn i {
    font-size: 1.25rem;
  }

  .tab-btn:hover {
    background: hsl(var(--bc) / 0.08);
    color: hsl(var(--bc));
    transform: translateY(-1px);
  }

  .tab-btn.active {
    background: hsl(var(--p));
    color: hsl(var(--pc));
    box-shadow: 0 4px 12px hsl(var(--p) / 0.3);
  }

  .tab-btn.active:hover {
    transform: translateY(-1px);
  }

  .tab-panel {
    animation: slideEnter 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .no-results {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
    animation: fadeIn 0.3s ease-out;
  }

  .no-results i {
    font-size: 4rem;
    color: hsl(var(--bc) / 0.3);
    margin-bottom: 1rem;
  }

  .no-results h3 {
    font-size: 1.5rem;
    font-weight: 700;
    color: hsl(var(--bc));
    margin: 0 0 0.5rem 0;
  }

  .no-results p {
    font-size: 1rem;
    color: hsl(var(--bc) / 0.6);
    margin: 0 0 1.5rem 0;
  }

  @media (max-width: 768px) {
    .settings-container {
      padding: 1rem;
    }

    .settings-header {
      flex-direction: column;
      gap: 1.5rem;
    }

    .search-wrapper {
      flex: 1 1 100%;
    }

    .header-title h1 {
      font-size: 1.5rem;
    }

    .header-title i {
      font-size: 1.5rem;
    }

    .tab-nav {
      padding: 0.375rem;
      gap: 0.375rem;
    }

    .tab-btn {
      padding: 0.625rem 1rem;
      font-size: 0.875rem;
    }

    .tab-btn i {
      font-size: 1rem;
    }
  }
</style>
