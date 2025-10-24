<script>
  import GeneralSettings from "./settings/GeneralSettings.svelte";
  import UsersSettings from "./settings/UsersSettings.svelte";
  import StorageSettings from "./settings/StorageSettings.svelte";
  import BackupSettings from "./settings/BackupSettings.svelte";
  import AboutSettings from "./settings/AboutSettings.svelte";

  let activeTab = "general";

  const tabs = [
    { id: "general", label: "General", icon: "sliders" },
    { id: "users", label: "Users", icon: "people-fill" },
    { id: "storage", label: "Storage", icon: "hdd-fill" },
    { id: "backup", label: "Backup", icon: "cloud-arrow-up-fill" },
    { id: "about", label: "About", icon: "info-circle" },
  ];

  function handleTabChange(tab) {
    activeTab = tab;
  }
</script>

<div class="settings-container">
  <!-- Tab Navigation -->
  <div class="tab-nav">
    {#each tabs as tab}
      <button
        class="tab-btn"
        class:active={activeTab === tab.id}
        on:click={() => handleTabChange(tab.id)}
      >
        <i class="bi bi-{tab.icon}"></i>
        <span>{tab.label}</span>
      </button>
    {/each}
  </div>

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
</div>

<style>
  .settings-container {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .tab-nav {
    display: flex;
    gap: 0.5rem;
    padding: 0.25rem;
    background: var(--md-sys-color-surface-variant);
    border-radius: 12px;
    margin-bottom: 2rem;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    border: none;
    background: transparent;
    color: var(--md-sys-color-on-surface);
    border-radius: 8px;
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
  }

  .tab-btn i {
    font-size: 1.125rem;
  }

  .tab-btn:hover {
    background: var(--md-sys-color-surface-container-high);
  }

  .tab-btn.active {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .tab-panel {
    animation: fadeIn 0.3s ease-out;
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
