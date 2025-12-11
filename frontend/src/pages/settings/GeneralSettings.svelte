<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import GeneralSettingsTab from "./general/GeneralSettingsTab.svelte";
  import NotificationsTab from "./general/NotificationsTab.svelte";
  import NotificationsSettings from "./general/NotificationsSettings.svelte";
  import SecurityTab from "./general/SecurityTab.svelte";
  import SecuritySettings from "./general/SecuritySettings.svelte";
  import SecurityPolicySettings from "./general/SecurityPolicySettings.svelte";
  import FeaturesTab from "./general/FeaturesTab.svelte";
  import ThemeCustomizationView from "./general/ThemeCustomizationView.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const tabs = [
    {
      id: "general",
      icon: "sliders",
      label: "Allgemein",
      component: GeneralSettingsTab,
    },
    {
      id: "theme",
      icon: "palette-fill",
      label: "Design",
      component: ThemeCustomizationView,
    },
    {
      id: "notifications",
      icon: "bell-fill",
      label: "Benachrichtigungen",
      component: NotificationsTab,
    },
    {
      id: "notif-settings",
      icon: "bell",
      label: "Benachrichtigungs-Details",
      component: NotificationsSettings,
    },
    {
      id: "security",
      icon: "shield-lock-fill",
      label: "Sicherheit",
      component: SecurityTab,
    },
    {
      id: "security-settings",
      icon: "shield-check",
      label: "Sicherheits-Einstellungen",
      component: SecuritySettings,
    },
    {
      id: "security-policy",
      icon: "shield-exclamation",
      label: "Sicherheitsrichtlinien",
      component: SecurityPolicySettings,
    },
    {
      id: "features",
      icon: "toggles",
      label: "Features",
      component: FeaturesTab,
    },
  ];

  let activeTab = $state("general");
  let ActiveComponent = $derived(
    tabs.find((t) => t.id === activeTab)?.component || GeneralSettingsTab
  );
</script>

<!-- Tab Navigation -->
<div class="tabs-header">
  {#each tabs as tab}
    <button
      class="tab-button"
      class:active={activeTab === tab.id}
      onclick={() => (activeTab = tab.id)}
    >
      <i class="bi bi-{tab.icon}"></i>
      <span>{tab.label}</span>
    </button>
  {/each}
</div>

<!-- Tab Content -->
<div class="tab-content">
  <ActiveComponent />
</div>

<style>
  .loading-container {
    display: flex;
    justify-content: center;
    padding: 4rem;
  }
  .spinner {
    width: 36px;
    height: 36px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Toast */
  .toast {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
    font-weight: 500;
  }
  .toast.success {
    background: #dcfce7;
    color: #166534;
  }
  .toast.error {
    background: #fee2e2;
    color: #991b1b;
  }
  :global(.dark) .toast.success {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }
  :global(.dark) .toast.error {
    background: rgba(220, 38, 38, 0.2);
    color: #fca5a5;
  }

  /* Tab Navigation */
  .tabs-header {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 2px solid #e5e7eb;
    padding-bottom: 0;
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

  /* Tab Content */
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

  /* Grid */
  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .col-span-full {
    grid-column: 1 / -1;
  }

  /* Card */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
  }
  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-header {
    display: flex;
    align-items: flex-start;
    gap: 0.875rem;
    margin-bottom: 1.25rem;
  }
  .card-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }
  .card-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }
  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }
  .card-icon.purple {
    background: #f3e8ff;
    color: #9333ea;
  }
  .card-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }
  .card-icon.rose {
    background: #ffe4e6;
    color: #e11d48;
  }
  :global(.dark) .card-icon.amber {
    background: rgba(245, 158, 11, 0.2);
    color: #fbbf24;
  }
  :global(.dark) .card-icon.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }
  :global(.dark) .card-icon.purple {
    background: rgba(147, 51, 234, 0.2);
    color: #c084fc;
  }
  :global(.dark) .card-icon.green {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }
  :global(.dark) .card-icon.rose {
    background: rgba(225, 29, 72, 0.2);
    color: #fb7185;
  }

  .card-title h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }
  .card-title p {
    font-size: 0.8125rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .card-title h3 {
    color: #f9fafb;
  }
  :global(.dark) .card-title p {
    color: #9ca3af;
  }

  /* Theme Buttons */
  .theme-buttons {
    display: flex;
    gap: 0.5rem;
  }
  .theme-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.375rem;
    padding: 0.75rem;
    background: #f9fafb;
    border: 2px solid transparent;
    border-radius: 0.5rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.15s;
  }
  .theme-btn:hover {
    border-color: #d1d5db;
  }
  .theme-btn.active {
    background: #dcfce7;
    border-color: #22c55e;
    color: #166534;
  }
  .theme-btn i {
    font-size: 1.25rem;
  }
  :global(.dark) .theme-btn {
    background: #374151;
    color: #9ca3af;
  }
  :global(.dark) .theme-btn:hover {
    border-color: #4b5563;
  }
  :global(.dark) .theme-btn.active {
    background: rgba(34, 197, 94, 0.2);
    border-color: #22c55e;
    color: #86efac;
  }

  /* View Buttons */
  .view-buttons {
    display: flex;
    gap: 0.5rem;
  }
  .view-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: #f9fafb;
    border: 2px solid transparent;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.15s;
  }
  .view-btn:hover {
    border-color: #d1d5db;
  }
  .view-btn.active {
    background: #dcfce7;
    border-color: #22c55e;
    color: #166534;
  }
  .view-btn i {
    font-size: 1.125rem;
  }
  :global(.dark) .view-btn {
    background: #374151;
    color: #9ca3af;
  }
  :global(.dark) .view-btn:hover {
    border-color: #4b5563;
  }
  :global(.dark) .view-btn.active {
    background: rgba(34, 197, 94, 0.2);
    border-color: #22c55e;
    color: #86efac;
  }

  /* Select */
  .select-wrapper {
    position: relative;
  }
  .select-input {
    width: 100%;
    padding: 0.75rem 1rem;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    color: #111827;
    cursor: pointer;
    appearance: none;
  }
  .select-input:focus {
    outline: none;
    border-color: #22c55e;
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
  }
  :global(.dark) .select-input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  /* Toggle List */
  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .toggle-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .toggle-item {
    background: #374151;
  }
  .toggle-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .toggle-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }
  .toggle-desc {
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .toggle-label {
    color: #f9fafb;
  }
  :global(.dark) .toggle-desc {
    color: #9ca3af;
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    width: 44px;
    height: 24px;
    background: #d1d5db;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.2s;
    flex-shrink: 0;
  }
  .toggle-switch.active {
    background: #22c55e;
  }
  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    background: white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s;
  }
  .toggle-switch.active .toggle-knob {
    transform: translateX(20px);
  }
  :global(.dark) .toggle-switch {
    background: #4b5563;
  }

  @media (max-width: 768px) {
    .settings-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
