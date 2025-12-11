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
    general: false,
    users: false,
    storage: false,
    automation: false,
  });

  function toggleGroup(groupId) {
    collapsedGroups[groupId] = !collapsedGroups[groupId];
  }

  // Check if user is admin
  let isAdmin = $derived(
    $auth?.user?.role === "admin" || $auth?.user?.is_admin
  );

  // New organized tabs structure
  const allTabs = [
    // ALLGEMEINES (Header)
    {
      id: "general",
      icon: "sliders",
      label: "Allgemein",
      group: "general",
      adminOnly: false,
    },
    {
      id: "security",
      icon: "shield-lock-fill",
      label: "Sicherheit",
      group: "general",
      adminOnly: false,
    },
    {
      id: "notifications",
      icon: "bell-fill",
      label: "Benachrichtigungen",
      group: "general",
      adminOnly: false,
    },

    // BENUTZER / GRUPPEN (Admin only) - Combined into one tab
    {
      id: "user-management",
      icon: "people-fill",
      label: "Benutzerverwaltung",
      group: "users",
      adminOnly: true,
    },
    {
      id: "oauth",
      icon: "shield-check",
      label: "OAuth Settings",
      group: "users",
      adminOnly: true,
    },
    {
      id: "ldap",
      icon: "diagram-3",
      label: "LDAP",
      group: "users",
      adminOnly: true,
    },

    // SPEICHER / DATEIEN (Admin only) - Combined into one tab
    {
      id: "storage",
      icon: "hdd-fill",
      label: "Speicherverwaltung",
      group: "storage",
      adminOnly: true,
    },

    // AUTOMATISIERUNGEN / WORKFLOWS (Admin only)
    {
      id: "jobs",
      icon: "list-task",
      label: "Hintergrundaufträge",
      group: "automation",
      adminOnly: true,
    },
    {
      id: "workflows",
      icon: "diagram-3-fill",
      label: "Workflows",
      group: "automation",
      adminOnly: true,
    },
    {
      id: "webhooks",
      icon: "link-45deg",
      label: "Webhooks",
      group: "automation",
      adminOnly: true,
    },
    {
      id: "ftp-sync",
      icon: "cloud-arrow-down-fill",
      label: "FTP",
      group: "automation",
      adminOnly: true,
    },
    {
      id: "email-integration",
      icon: "envelope-fill",
      label: "E-Mail-Integration",
      group: "automation",
      adminOnly: true,
    },

    // ÜBER SYNCSPACE (everyone)
    {
      id: "about",
      icon: "info-circle-fill",
      label: "Über SyncSpace",
      group: "about",
      adminOnly: false,
    },
  ];

  // Filter tabs based on user role
  let visibleTabs = $derived(
    allTabs.filter((tab) => !tab.adminOnly || isAdmin)
  );

  // Group tabs by category
  let generalTabs = $derived(visibleTabs.filter((t) => t.group === "general"));
  let usersTabs = $derived(visibleTabs.filter((t) => t.group === "users"));
  let storageTabs = $derived(visibleTabs.filter((t) => t.group === "storage"));
  let automationTabs = $derived(
    visibleTabs.filter((t) => t.group === "automation")
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
      <!-- ALLGEMEINES Section -->
      <div class="settings-group">
        <button
          class="group-header"
          onclick={() => toggleGroup("general")}
          aria-expanded={!collapsedGroups.general}
        >
          <span class="group-title">Allgemeines</span>
          <i
            class="bi bi-chevron-down toggle-icon"
            class:collapsed={collapsedGroups.general}
          ></i>
        </button>
        {#if !collapsedGroups.general}
          <nav class="settings-nav">
            {#each generalTabs as tab}
              <button
                class="nav-item"
                class:active={activeTab === tab.id}
                onclick={() => setTab(tab.id)}
              >
                <i class="bi bi-{tab.icon}"></i>
                <span>{tab.label}</span>
              </button>
            {/each}
          </nav>
        {/if}
      </div>

      {#if isAdmin}
        <!-- BENUTZER / GRUPPEN -->
        <div class="settings-group">
          <button
            class="group-header"
            onclick={() => toggleGroup("users")}
            aria-expanded={!collapsedGroups.users}
          >
            <span class="group-title">Benutzer / Gruppen</span>
            <i
              class="bi bi-chevron-down toggle-icon"
              class:collapsed={collapsedGroups.users}
            ></i>
          </button>
          {#if !collapsedGroups.users}
            <nav class="settings-nav">
              {#each usersTabs as tab}
                <button
                  class="nav-item"
                  class:active={activeTab === tab.id}
                  onclick={() => setTab(tab.id)}
                >
                  <i class="bi bi-{tab.icon}"></i>
                  <span>{tab.label}</span>
                </button>
              {/each}
            </nav>
          {/if}
        </div>

        <!-- SPEICHER / DATEIEN -->
        <div class="settings-group">
          <button
            class="group-header"
            onclick={() => toggleGroup("storage")}
            aria-expanded={!collapsedGroups.storage}
          >
            <span class="group-title">Speicher / Dateien</span>
            <i
              class="bi bi-chevron-down toggle-icon"
              class:collapsed={collapsedGroups.storage}
            ></i>
          </button>
          {#if !collapsedGroups.storage}
            <nav class="settings-nav">
              {#each storageTabs as tab}
                <button
                  class="nav-item"
                  class:active={activeTab === tab.id}
                  onclick={() => setTab(tab.id)}
                >
                  <i class="bi bi-{tab.icon}"></i>
                  <span>{tab.label}</span>
                </button>
              {/each}
            </nav>
          {/if}
        </div>

        <!-- AUTOMATISIERUNGEN / WORKFLOWS -->
        <div class="settings-group">
          <button
            class="group-header"
            onclick={() => toggleGroup("automation")}
            aria-expanded={!collapsedGroups.automation}
          >
            <span class="group-title">Automatisierungen / Workflows</span>
            <i
              class="bi bi-chevron-down toggle-icon"
              class:collapsed={collapsedGroups.automation}
            ></i>
          </button>
          {#if !collapsedGroups.automation}
            <nav class="settings-nav">
              {#each automationTabs as tab}
                <button
                  class="nav-item"
                  class:active={activeTab === tab.id}
                  onclick={() => setTab(tab.id)}
                >
                  <i class="bi bi-{tab.icon}"></i>
                  <span>{tab.label}</span>
                </button>
              {/each}
            </nav>
          {/if}
        </div>
      {/if}

      <!-- ÜBER SYNCSPACE -->
      <div class="settings-group">
        <nav class="settings-nav">
          {#each aboutTabs as tab}
            <button
              class="nav-item"
              class:active={activeTab === tab.id}
              onclick={() => setTab(tab.id)}
            >
              <i class="bi bi-{tab.icon}"></i>
              <span>{tab.label}</span>
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
      {:else if activeTab === "user-management"}
        {#await import("./UserManagement.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "oauth"}
        {#await import("./OAuthSettings.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "ldap"}
        {#await import("./LdapSettings.svelte") then module}
          <module.default />
        {/await}

        <!-- Admin: System & Storage -->
      {:else if activeTab === "storage"}
        {#await import("./StorageSettings.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "backup"}
        {#await import("../system/BackupView.svelte") then module}
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
      {:else if activeTab === "ftp-sync"}
        {#await import("../tools/FtpSyncView.svelte") then module}
          <module.default />
        {/await}
      {:else if activeTab === "email-integration"}
        {#await import("../tools/EmailIntegrationView.svelte") then module}
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
