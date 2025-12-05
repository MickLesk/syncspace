<script>
  import GeneralSettings from "./GeneralSettings.svelte";
  import SecuritySettings from "./SecuritySettings.svelte";
  import UsersSettings from "./UsersSettings.svelte";
  import StorageSettings from "./StorageSettings.svelte";
  import BackupSettings from "./BackupSettings.svelte";
  import PerformanceSettings from "./PerformanceSettings.svelte";
  import AboutSettings from "./AboutSettings.svelte";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let activeTab = $state("general");

  const tabs = [
    { id: "general", icon: "bi-sliders", label: "general" },
    { id: "security", icon: "bi-shield-lock-fill", label: "security" },
    { id: "users", icon: "bi-people-fill", label: "users" },
    { id: "storage", icon: "bi-hdd-fill", label: "storage" },
    { id: "backup", icon: "bi-cloud-arrow-up-fill", label: "backup" },
    { id: "performance", icon: "bi-speedometer2", label: "performance" },
    { id: "about", icon: "bi-info-circle-fill", label: "about" },
  ];
</script>

<PageWrapper title={tr("settings")} showSidebar={true}>
  <div class="settings-view">
    <!-- Header -->
    <div class="view-header">
      <h1 class="view-title">
        <i class="bi bi-gear-fill"></i>
        {tr("settings")}
      </h1>
    </div>

    <!-- Tab Navigation -->
    <div class="tabs-card">
      <div class="tabs-list">
        {#each tabs as tab}
          <button
            class="tab-btn"
            class:active={activeTab === tab.id}
            onclick={() => (activeTab = tab.id)}
          >
            <i class="bi {tab.icon}"></i>
            <span>{tr(tab.label)}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- Tab Content -->
    <div class="tab-content">
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
  </div>
</PageWrapper>

<style>
  .settings-view {
    padding: 1.5rem;
  }

  /* Header */
  .view-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  .view-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #111827;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
  }
  :global(.dark) .view-title {
    color: #f9fafb;
  }

  /* Tabs */
  .tabs-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 0.5rem;
    margin-bottom: 1.5rem;
  }
  :global(.dark) .tabs-card {
    background: #1f2937;
    border-color: #374151;
  }

  .tabs-list {
    display: flex;
    gap: 0.25rem;
    flex-wrap: wrap;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    background: transparent;
    border: none;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.15s;
  }
  .tab-btn:hover {
    background: #f3f4f6;
    color: #111827;
  }
  .tab-btn.active {
    background: #22c55e;
    color: white;
  }
  :global(.dark) .tab-btn {
    color: #9ca3af;
  }
  :global(.dark) .tab-btn:hover {
    background: #374151;
    color: #f9fafb;
  }
  :global(.dark) .tab-btn.active {
    background: #22c55e;
    color: white;
  }

  .tab-btn i {
    font-size: 1rem;
  }

  /* Content */
  .tab-content {
    animation: fadeIn 0.2s ease;
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @media (max-width: 768px) {
    .settings-view {
      padding: 1rem;
    }
    .tabs-list {
      gap: 0.125rem;
    }
    .tab-btn {
      padding: 0.5rem 0.75rem;
      font-size: 0.8125rem;
    }
    .tab-btn span {
      display: none;
    }
    .tab-btn i {
      font-size: 1.125rem;
    }
  }
</style>
