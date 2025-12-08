<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Active tab state
  let activeTab = $state("users");

  // Admin tabs configuration - using correct i18n keys
  const tabs = [
    // User & Access Management
    { id: "users", icon: "people-fill", label: "users", group: "users" },
    { id: "guests", icon: "person-badge", label: "guests.title", group: "users" },
    { id: "roles", icon: "person-badge-fill", label: "roles.title", group: "users" },
    { id: "groups", icon: "people", label: "groups.title", group: "users" },
    
    // System Management
    { id: "backup", icon: "cloud-arrow-up-fill", label: "backup.title", group: "system" },
    { id: "jobs", icon: "list-task", label: "jobs.title", group: "system" },
    { id: "workflows", icon: "diagram-3-fill", label: "workflows.title", group: "system" },
    { id: "webhooks", icon: "link-45deg", label: "webhooks.title", group: "system" },
    
    // Storage & Security
    { id: "cloud-storage", icon: "cloud-fill", label: "cloudStorage.title", group: "storage" },
    { id: "quotas", icon: "speedometer2", label: "quotas.title", group: "storage" },
    { id: "encryption", icon: "shield-lock-fill", label: "encryption.title", group: "storage" },
    { id: "analytics", icon: "bar-chart-line-fill", label: "storageAnalytics", group: "storage" },
    
    // Configuration & Audit
    { id: "config", icon: "sliders", label: "systemConfig.title", group: "config" },
    { id: "audit", icon: "shield-check", label: "audit.title", group: "config" },
  ];

  // Group tabs by category
  let userTabs = $derived(tabs.filter(t => t.group === "users"));
  let systemTabs = $derived(tabs.filter(t => t.group === "system"));
  let storageTabs = $derived(tabs.filter(t => t.group === "storage"));
  let configTabs = $derived(tabs.filter(t => t.group === "config"));

  // Handle URL hash for deep linking
  onMount(() => {
    const hash = window.location.hash;
    if (hash.includes("?tab=")) {
      const tabParam = hash.split("?tab=")[1];
      if (tabs.find(t => t.id === tabParam)) {
        activeTab = tabParam;
      }
    }
  });

  function setTab(tabId) {
    activeTab = tabId;
    // Update URL hash for deep linking
    const newHash = `#/admin?tab=${tabId}`;
    window.history.replaceState(null, "", newHash);
  }
</script>

<PageWrapper title={tr("admin.title") || "Administration"} showSidebar={true}>
  <div class="admin-view">
    <!-- Tab Navigation - Horizontal scrollable -->
    <div class="admin-tabs-container">
      <div class="admin-tabs">
        <!-- User & Access -->
        <div class="tab-group">
          <span class="tab-group-label">{tr("admin.userAccess") || "Users & Access"}</span>
          <div class="tab-group-items">
            {#each userTabs as tab}
              <button
                class="admin-tab"
                class:active={activeTab === tab.id}
                onclick={() => setTab(tab.id)}
              >
                <i class="bi bi-{tab.icon}"></i>
                <span>{tr(tab.label) || tab.label}</span>
              </button>
            {/each}
          </div>
        </div>

        <!-- System Management -->
        <div class="tab-group">
          <span class="tab-group-label">{tr("admin.systemManagement") || "System"}</span>
          <div class="tab-group-items">
            {#each systemTabs as tab}
              <button
                class="admin-tab"
                class:active={activeTab === tab.id}
                onclick={() => setTab(tab.id)}
              >
                <i class="bi bi-{tab.icon}"></i>
                <span>{tr(tab.label) || tab.label}</span>
              </button>
            {/each}
          </div>
        </div>

        <!-- Storage & Security -->
        <div class="tab-group">
          <span class="tab-group-label">{tr("admin.storageSecurity") || "Storage"}</span>
          <div class="tab-group-items">
            {#each storageTabs as tab}
              <button
                class="admin-tab"
                class:active={activeTab === tab.id}
                onclick={() => setTab(tab.id)}
              >
                <i class="bi bi-{tab.icon}"></i>
                <span>{tr(tab.label) || tab.label}</span>
              </button>
            {/each}
          </div>
        </div>

        <!-- Configuration & Audit -->
        <div class="tab-group">
          <span class="tab-group-label">{tr("admin.configuration") || "Config"}</span>
          <div class="tab-group-items">
            {#each configTabs as tab}
              <button
                class="admin-tab"
                class:active={activeTab === tab.id}
                onclick={() => setTab(tab.id)}
              >
                <i class="bi bi-{tab.icon}"></i>
                <span>{tr(tab.label) || tab.label}</span>
              </button>
            {/each}
          </div>
        </div>
      </div>
    </div>

    <!-- Tab Content - Lazy load components -->
    <div class="admin-content">
      {#if activeTab === "users"}
        {#await import("../system/UsersView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "guests"}
        {#await import("../GuestAccessView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "roles"}
        {#await import("../rbac/RoleManagementView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "groups"}
        {#await import("./UserGroupsView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "backup"}
        {#await import("../system/BackupView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "jobs"}
        {#await import("../jobs/JobsQueueView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "workflows"}
        {#await import("../workflow/WorkflowBuilderView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "webhooks"}
        {#await import("./WebhooksView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "cloud-storage"}
        {#await import("./CloudStorageView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "quotas"}
        {#await import("../system/QuotasView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "encryption"}
        {#await import("../EncryptionView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "analytics"}
        {#await import("../analytics/StorageAnalyticsView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "config"}
        {#await import("./SystemConfigView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {:else if activeTab === "audit"}
        {#await import("../AuditComplianceView.svelte") then module}
          <svelte:component this={module.default} />
        {/await}
      {/if}
    </div>
  </div>
</PageWrapper>

<style>
  .admin-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  /* Tab Container */
  .admin-tabs-container {
    background: var(--card-bg, #ffffff);
    border-bottom: 1px solid var(--border-color, #e5e7eb);
    padding: 0.75rem 1rem;
    overflow-x: auto;
    flex-shrink: 0;
  }

  :global(.dark) .admin-tabs-container {
    background: var(--card-bg-dark, #1e293b);
    border-color: var(--border-color-dark, #334155);
  }

  .admin-tabs {
    display: flex;
    gap: 1.5rem;
    min-width: max-content;
  }

  /* Tab Groups */
  .tab-group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .tab-group-label {
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted, #6b7280);
    padding-left: 0.5rem;
  }

  :global(.dark) .tab-group-label {
    color: var(--text-muted-dark, #9ca3af);
  }

  .tab-group-items {
    display: flex;
    gap: 0.25rem;
  }

  /* Tab Button */
  .admin-tab {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.75rem;
    border: none;
    background: transparent;
    color: var(--text-secondary, #4b5563);
    font-size: 0.8rem;
    font-weight: 500;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .admin-tab:hover {
    background: var(--hover-bg, #f3f4f6);
    color: var(--text-primary, #111827);
  }

  :global(.dark) .admin-tab {
    color: var(--text-secondary-dark, #9ca3af);
  }

  :global(.dark) .admin-tab:hover {
    background: var(--hover-bg-dark, #334155);
    color: var(--text-primary-dark, #f3f4f6);
  }

  .admin-tab.active {
    background: var(--primary-light, #dcfce7);
    color: var(--primary, #16a34a);
  }

  :global(.dark) .admin-tab.active {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .admin-tab i {
    font-size: 0.9rem;
  }

  /* Content Area */
  .admin-content {
    flex: 1;
    overflow: auto;
    background: var(--bg-secondary, #f9fafb);
  }

  :global(.dark) .admin-content {
    background: var(--bg-secondary-dark, #0f172a);
  }

  /* Hide PageWrapper in embedded components */
  .admin-content :global(.page-wrapper) {
    padding: 0;
  }

  .admin-content :global(.page-wrapper > .page-header) {
    display: none;
  }

  /* Scrollbar styling */
  .admin-tabs-container::-webkit-scrollbar {
    height: 4px;
  }

  .admin-tabs-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .admin-tabs-container::-webkit-scrollbar-thumb {
    background: var(--border-color, #e5e7eb);
    border-radius: 2px;
  }

  :global(.dark) .admin-tabs-container::-webkit-scrollbar-thumb {
    background: var(--border-color-dark, #334155);
  }

  /* Mobile Responsive */
  @media (max-width: 768px) {
    .admin-tabs-container {
      padding: 0.5rem;
    }

    .admin-tabs {
      gap: 1rem;
    }

    .tab-group-label {
      font-size: 0.6rem;
    }

    .admin-tab {
      padding: 0.375rem 0.5rem;
      font-size: 0.75rem;
    }

    .admin-tab span {
      display: none;
    }

    .admin-tab i {
      font-size: 1rem;
    }
  }
</style>
