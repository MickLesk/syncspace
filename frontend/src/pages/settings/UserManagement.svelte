<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const tabs = [
    { id: "users", icon: "people-fill", label: "Benutzer" },
    { id: "groups", icon: "people", label: "Gruppen" },
    { id: "roles", icon: "person-badge-fill", label: "Rollen" },
    { id: "guests", icon: "person-badge", label: "Gastzugänge" },
  ];

  let activeTab = $state("users");
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
  {#if activeTab === "users"}
    {#await import("../system/UsersView.svelte") then module}
      <module.default />
    {/await}
  {:else if activeTab === "groups"}
    {#await import("../admin/UserGroupsView.svelte") then module}
      <module.default />
    {/await}
  {:else if activeTab === "roles"}
    {#await import("../rbac/RoleManagementView.svelte") then module}
      <module.default />
    {/await}
  {:else if activeTab === "guests"}
    {#await import("../GuestAccessView.svelte") then module}
      <module.default />
    {/await}
  {/if}
</div>

<style>
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
    pointer-events: auto;
  }
  .tab-button:hover {
    color: #111827;
    background: #f9fafb;
  }
  .tab-button.active {
    color: #22c55e;
    border-bottom-color: #22c55e;
  }
  .tab-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
</style>
