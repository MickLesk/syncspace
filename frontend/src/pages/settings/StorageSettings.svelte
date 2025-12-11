<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import StorageView from "./storage/StorageView.svelte";
  import PerformanceSettings from "./storage/PerformanceSettings.svelte";
  import QuotasView from "./storage/QuotasView.svelte";
  import QuotaManagementView from "./storage/QuotaManagementView.svelte";
  import EncryptionView from "./storage/EncryptionView.svelte";
  import ShareAnalyticsView from "./storage/ShareAnalyticsView.svelte";
  import AuditComplianceView from "./storage/AuditComplianceView.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const tabs = [
    {
      id: "storage",
      icon: "hdd-fill",
      label: "Speicher",
      component: StorageView,
    },
    {
      id: "performance",
      icon: "speedometer",
      label: "Performance",
      component: PerformanceSettings,
    },
    {
      id: "quotas",
      icon: "speedometer2",
      label: "Quotas",
      component: QuotasView,
    },
    {
      id: "quota-mgmt",
      icon: "bar-chart-fill",
      label: "Quota Management",
      component: QuotaManagementView,
    },
    {
      id: "encryption",
      icon: "shield-lock-fill",
      label: "Verschlüsselung",
      component: EncryptionView,
    },
    {
      id: "share-analytics",
      icon: "graph-up",
      label: "Share Analytics",
      component: ShareAnalyticsView,
    },
    {
      id: "audit",
      icon: "file-text-fill",
      label: "Audit & Compliance",
      component: AuditComplianceView,
    },
  ];

  let activeTab = $state("storage");
  let ActiveComponent = $derived(
    tabs.find((t) => t.id === activeTab)?.component || StorageView
  );
</script>

<div class="tabs-header">
  {#each tabs as tab}
    <button
      class="tab-button"
      class:active={activeTab === tab.id}
      onclick={() => (activeTab = tab.id)}
      aria-label={tab.label}
    >
      <i class="bi bi-{tab.icon}"></i>
      <span>{tab.label}</span>
    </button>
  {/each}
</div>

<div class="tab-content">
  <ActiveComponent />
</div>

<style>
  .tabs-header {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 2px solid #e5e7eb;
    padding-bottom: 0;
    flex-wrap: wrap;
  }
  :global(.dark) .tabs-header {
    border-bottom-color: #374151;
  }

  .tab-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    margin-bottom: -2px;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s;
  }
  .tab-button:hover {
    color: #111827;
    background: #f9fafb;
  }
  .tab-button.active {
    color: #22c55e;
    border-bottom-color: #22c55e;
  }
  .tab-button i {
    font-size: 1rem;
  }
  :global(.dark) .tab-button {
    color: #9ca3af;
  }
  :global(.dark) .tab-button:hover {
    color: #f9fafb;
    background: #374151;
  }
  :global(.dark) .tab-button.active {
    color: #22c55e;
  }

  .tab-content {
    animation: fadeIn 0.3s ease-in-out;
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
