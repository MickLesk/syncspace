<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { auth } from "../../stores/auth.js";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Active tab state
  let activeTab = $state("general");

  // Collapsible group state
  let collapsedGroups = $state({
    personal: false,
    "admin-users": false,
    "admin-system": false,
    "admin-tools": false,
    "admin-audit": false,
    about: false,
  });

  function toggleGroup(groupId) {
    collapsedGroups[groupId] = !collapsedGroups[groupId];
  }

  // Check if user is admin
  let isAdmin = $derived(
    $auth?.user?.role === "admin" || $auth?.user?.is_admin
  );

  // All tabs configuration - organized by category
  const allTabs = [
    // Personal Settings (everyone)
    {
      id: "general",
      icon: "sliders",
      label: "general",
      group: "personal",
      adminOnly: false,
    },
    {
      id: "profile",
      icon: "person-fill",
      label: "profile",
      group: "personal",
      adminOnly: false,
    },
    {
      id: "security",
      icon: "shield-lock-fill",
      label: "security",
      group: "personal",
      adminOnly: false,
    },
    {
      id: "notifications",
      icon: "bell-fill",
      label: "notifications",
      group: "personal",
      adminOnly: false,
    },

    // User & Access Management (admin only)
    {
      id: "users",
      icon: "people-fill",
      label: "users",
      group: "admin-users",
      adminOnly: true,
    },
    {
      id: "guests",
      icon: "person-badge",
      label: "guests.title",
      group: "admin-users",
      adminOnly: true,
    },
    {
      id: "roles",
      icon: "person-badge-fill",
      label: "roles.title",
      group: "admin-users",
      adminOnly: true,
    },
    {
      id: "groups",
      icon: "people",
      label: "groups.title",
      group: "admin-users",
      adminOnly: true,
    },

    // System & Storage (admin only)
    {
      id: "backup",
      icon: "cloud-arrow-up-fill",
      label: "backup.title",
      group: "admin-system",
      adminOnly: true,
    },
    {
      id: "cloud-storage",
      icon: "cloud-fill",
      label: "cloudStorage.title",
      group: "admin-system",
      adminOnly: true,
    },
    {
      id: "quotas",
      icon: "speedometer2",
      label: "quotas.title",
      group: "admin-system",
      adminOnly: true,
    },
    {
      id: "encryption",
      icon: "shield-lock-fill",
      label: "encryption.title",
      group: "admin-system",
      adminOnly: true,
    },

    // Automation & Tools (admin only)
    {
      id: "jobs",
      icon: "list-task",
      label: "jobs.title",
      group: "admin-tools",
      adminOnly: true,
    },
    {
      id: "workflows",
      icon: "diagram-3-fill",
      label: "workflows.title",
      group: "admin-tools",
      adminOnly: true,
    },
    {
      id: "webhooks",
      icon: "link-45deg",
      label: "webhooks.title",
      group: "admin-tools",
      adminOnly: true,
    },

    // Analytics & Audit (admin only)
    {
      id: "analytics",
      icon: "bar-chart-line-fill",
      label: "storageAnalytics",
      group: "admin-audit",
      adminOnly: true,
    },
    {
      id: "audit",
      icon: "shield-check",
      label: "audit.title",
      group: "admin-audit",
      adminOnly: true,
    },
    {
      id: "config",
      icon: "gear-fill",
      label: "systemConfig.title",
      group: "admin-audit",
      adminOnly: true,
    },

    // About (everyone)
    {
      id: "about",
      icon: "info-circle-fill",
      label: "about",
      group: "about",
      adminOnly: false,
    },
  ];

  // Filter tabs based on user role
  let visibleTabs = $derived(
    allTabs.filter((tab) => !tab.adminOnly || isAdmin)
  );

  // Group tabs by category
  let personalTabs = $derived(
    visibleTabs.filter((t) => t.group === "personal")
  );
  let adminUserTabs = $derived(
    visibleTabs.filter((t) => t.group === "admin-users")
  );
  let adminSystemTabs = $derived(
    visibleTabs.filter((t) => t.group === "admin-system")
  );
  let adminToolsTabs = $derived(
    visibleTabs.filter((t) => t.group === "admin-tools")
  );
  let adminAuditTabs = $derived(
    visibleTabs.filter((t) => t.group === "admin-audit")
  );
  let aboutTabs = $derived(visibleTabs.filter((t) => t.group === "about"));

  // Handle URL hash for deep linking
  onMount(() => {
    const hash = window.location.hash;
    if (hash.includes("?tab=")) {
      const tabParam = hash.split("?tab=")[1];
      const tab = visibleTabs.find((t) => t.id === tabParam);
      if (tab) {
        activeTab = tabParam;
      }
    }
  });

  function setTab(tabId) {
    activeTab = tabId;
    const newHash = `#/settings?tab=${tabId}`;
    window.history.replaceState(null, "", newHash);
  }
</script>

<PageWrapper title={tr("settings")} showSidebar={true}>
  <div class="settings-hub">
    <!-- Sidebar Navigation -->
    <aside class="settings-sidebar">
      <!-- Personal Section -->
      <div class="settings-group">
        <button
          class="group-header"
          onclick={() => toggleGroup("personal")}
          aria-expanded={!collapsedGroups.personal}
        >
          <span class="group-title"
            >{tr("settingsHub.personal") || "Personal"}</span
          >
          <i
            class="bi bi-chevron-down toggle-icon"
            class:collapsed={collapsedGroups.personal}
          ></i>
        </button>
        {#if !collapsedGroups.personal}
          <nav class="settings-nav">
            {#each personalTabs as tab}
              <button
                class="nav-item"
                class:active={activeTab === tab.id}
                onclick={() => setTab(tab.id)}
              >
                <i class="bi bi-{tab.icon}"></i>
                <span>{tr(tab.label) || tab.label}</span>
              </button>
            {/each}
          </nav>
        {/if}
      </div>

      {#if isAdmin}
        <!-- Admin: Users -->
        <div class="settings-group">
          <button
            class="group-header"
            onclick={() => toggleGroup("admin-users")}
            aria-expanded={!collapsedGroups["admin-users"]}
          >
            <span class="group-title"
              >{tr("settingsHub.userManagement") || "User Management"}</span
            >
            <i
              class="bi bi-chevron-down toggle-icon"
              class:collapsed={collapsedGroups["admin-users"]}
            ></i>
          </button>
          {#if !collapsedGroups["admin-users"]}
            <nav class="settings-nav">
              {#each adminUserTabs as tab}
                <button
                  class="nav-item"
                  class:active={activeTab === tab.id}
                  onclick={() => setTab(tab.id)}
                >
                  <i class="bi bi-{tab.icon}"></i>
                  <span>{tr(tab.label) || tab.label}</span>
                </button>
              {/each}
            </nav>
          {/if}
        </div>

        <!-- Admin: Storage -->
        <div class="settings-group">
          <button
            class="group-header"
            onclick={() => toggleGroup("admin-system")}
            aria-expanded={!collapsedGroups["admin-system"]}
          >
            <span class="group-title"
              >{tr("settingsHub.storage") || "Storage"}</span
            >
            <i
              class="bi bi-chevron-down toggle-icon"
              class:collapsed={collapsedGroups["admin-system"]}
            ></i>
          </button>
          {#if !collapsedGroups["admin-system"]}
            <nav class="settings-nav">
              {#each adminSystemTabs as tab}
                <button
                  class="nav-item"
                  class:active={activeTab === tab.id}
                  onclick={() => setTab(tab.id)}
                >
                  <i class="bi bi-{tab.icon}"></i>
                  <span>{tr(tab.label) || tab.label}</span>
                </button>
              {/each}
            </nav>
          {/if}
        </div>

        <!-- Admin: Automation -->
        <div class="settings-group">
          <button
            class="group-header"
            onclick={() => toggleGroup("admin-tools")}
            aria-expanded={!collapsedGroups["admin-tools"]}
          >
            <span class="group-title"
              >{tr("settingsHub.automation") || "Automation"}</span
            >
            <i
              class="bi bi-chevron-down toggle-icon"
              class:collapsed={collapsedGroups["admin-tools"]}
            ></i>
          </button>
          {#if !collapsedGroups["admin-tools"]}
            <nav class="settings-nav">
              {#each adminToolsTabs as tab}
                <button
                  class="nav-item"
                  class:active={activeTab === tab.id}
                  onclick={() => setTab(tab.id)}
                >
                  <i class="bi bi-{tab.icon}"></i>
                  <span>{tr(tab.label) || tab.label}</span>
                </button>
              {/each}
            </nav>
          {/if}
        </div>

        <!-- Admin: Analytics & Audit -->
        <div class="settings-group">
          <button
            class="group-header"
            onclick={() => toggleGroup("admin-audit")}
            aria-expanded={!collapsedGroups["admin-audit"]}
          >
            <span class="group-title"
              >{tr("settingsHub.analytics") || "Analytics & Audit"}</span
            >
            <i
              class="bi bi-chevron-down toggle-icon"
              class:collapsed={collapsedGroups["admin-audit"]}
            ></i>
          </button>
          {#if !collapsedGroups["admin-audit"]}
            <nav class="settings-nav">
              {#each adminAuditTabs as tab}
                <button
                  class="nav-item"
                  class:active={activeTab === tab.id}
                  onclick={() => setTab(tab.id)}
                >
                  <i class="bi bi-{tab.icon}"></i>
                  <span>{tr(tab.label) || tab.label}</span>
                </button>
              {/each}
            </nav>
          {/if}
        </div>
      {/if}

      <!-- About -->
      <div class="settings-group">
        <nav class="settings-nav">
          {#each aboutTabs as tab}
            <button
              class="nav-item"
              class:active={activeTab === tab.id}
              onclick={() => setTab(tab.id)}
            >
              <i class="bi bi-{tab.icon}"></i>
              <span>{tr(tab.label) || tab.label}</span>
            </button>
          {/each}
        </nav>
      </div>
    </aside>

    <!-- Content Area -->
    <main class="settings-content">
      <!-- Personal Settings -->
      {#if activeTab === "general"}
        {#await import("./GeneralSettings.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "profile"}
        {#await import("../user/UserProfileView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "security"}
        {#await import("./SecurityPolicySettings.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "notifications"}
        {#await import("./NotificationsSettings.svelte") then module}
          <module.default />
        {/await}

        <!-- Admin: Users & Access -->
      {:else if activeTab === "users"}
        {#await import("../system/UsersView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "guests"}
        {#await import("../GuestAccessView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "roles"}
        {#await import("../rbac/RoleManagementView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "groups"}
        {#await import("../admin/UserGroupsView.svelte") then module}
          <module.default />
        {/await}

        <!-- Admin: System & Storage -->
      {:else if activeTab === "backup"}
        {#await import("../system/BackupView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "cloud-storage"}
        {#await import("../admin/CloudStorageView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "quotas"}
        {#await import("../system/QuotasView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "encryption"}
        {#await import("../EncryptionView.svelte") then module}
          <module.default />
        {/await}

        <!-- Admin: Tools -->
      {:else if activeTab === "jobs"}
        {#await import("../jobs/JobsQueueView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "workflows"}
        {#await import("../workflow/WorkflowBuilderView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "webhooks"}
        {#await import("../admin/WebhooksView.svelte") then module}
          <module.default />
        {/await}

        <!-- Admin: Analytics & Audit -->
      {:else if activeTab === "analytics"}
        {#await import("../analytics/StorageAnalyticsView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "audit"}
        {#await import("../AuditComplianceView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "config"}
        {#await import("../admin/SystemConfigView.svelte") then module}
          <module.default />
        {/await}

        <!-- About -->
      {:else if activeTab === "about"}
        {#await import("./AboutSettings.svelte") then module}
          <module.default />
        {/await}
      {/if}
    </main>
  </div>
</PageWrapper>

<style>
  .settings-hub {
    display: flex;
    gap: 1.5rem;
    min-height: calc(100vh - 200px);
  }

  /* Sidebar */
  .settings-sidebar {
    width: 240px;
    flex-shrink: 0;
    background: var(--card-bg, #ffffff);
    border-radius: 12px;
    padding: 1rem;
    border: 1px solid var(--border-color, #e5e7eb);
    height: fit-content;
    position: sticky;
    top: 1rem;
  }

  :global(.dark) .settings-sidebar {
    background: var(--card-bg-dark, #1e293b);
    border-color: var(--border-color-dark, #334155);
  }

  .settings-group {
    margin-bottom: 1.5rem;
  }

  .settings-group:last-child {
    margin-bottom: 0;
  }

  .group-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 6px;
    transition: background-color 0.15s ease;
  }

  .group-header:hover {
    background: var(--hover-bg, #f3f4f6);
  }

  :global(.dark) .group-header:hover {
    background: var(--hover-bg-dark, #334155);
  }

  .group-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted, #6b7280);
  }

  :global(.dark) .group-title {
    color: var(--text-muted-dark, #9ca3af);
  }

  .toggle-icon {
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
    transition: transform 0.2s ease;
  }

  .toggle-icon.collapsed {
    transform: rotate(-90deg);
  }

  :global(.dark) .toggle-icon {
    color: var(--text-muted-dark, #9ca3af);
  }

  .settings-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding-top: 0.25rem;
    animation: slideDown 0.2s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--text-secondary, #4b5563);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: left;
    width: 100%;
  }

  .nav-item:hover {
    background: var(--hover-bg, #f3f4f6);
    color: var(--text-primary, #111827);
  }

  :global(.dark) .nav-item {
    color: var(--text-secondary-dark, #9ca3af);
  }

  :global(.dark) .nav-item:hover {
    background: var(--hover-bg-dark, #334155);
    color: var(--text-primary-dark, #f9fafb);
  }

  .nav-item.active {
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
  }

  .nav-item.active:hover {
    background: linear-gradient(135deg, #16a34a 0%, #15803d 100%);
    color: white;
  }

  .nav-item i {
    font-size: 1rem;
    width: 1.25rem;
    text-align: center;
  }

  /* Content */
  .settings-content {
    flex: 1;
    min-width: 0;
    background: var(--card-bg, #ffffff);
    border-radius: 12px;
    border: 1px solid var(--border-color, #e5e7eb);
    overflow: hidden;
  }

  :global(.dark) .settings-content {
    background: var(--card-bg-dark, #1e293b);
    border-color: var(--border-color-dark, #334155);
  }

  /* Responsive */
  @media (max-width: 768px) {
    .settings-hub {
      flex-direction: column;
    }

    .settings-sidebar {
      width: 100%;
      position: relative;
      top: 0;
    }

    .settings-nav {
      flex-direction: row;
      flex-wrap: wrap;
      gap: 0.5rem;
    }

    .nav-item {
      flex: 0 0 auto;
      padding: 0.5rem 0.75rem;
    }

    .nav-item span {
      display: none;
    }

    .nav-item i {
      margin: 0;
    }
  }
</style>
