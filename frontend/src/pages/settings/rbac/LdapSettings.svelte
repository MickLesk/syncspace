<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import api from "../../../lib/api.js";
  import ModernButton from "../../../components/ui/ModernButton.svelte";
  import StandardGlassCard from "../../../components/ui/StandardGlassCard.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let saving = $state(false);
  let testing = $state(false);
  let syncing = $state(false);
  let error = $state("");
  let success = $state("");

  // LDAP Configurations
  let configs = $state([]);

  // Add/Edit Config Modal
  let showConfigModal = $state(false);
  let editingConfig = $state(null);
  let configForm = $state({
    name: "",
    host: "",
    port: 389,
    use_ssl: false,
    bind_dn: "",
    bind_password: "",
    base_dn: "",
    user_filter: "(objectClass=person)",
    username_attribute: "uid",
    email_attribute: "mail",
    display_name_attribute: "cn",
    group_filter: "(objectClass=group)",
    group_membership_attribute: "memberOf",
    enabled: true,
    sync_interval_minutes: 60,
  });

  onMount(async () => {
    await loadConfigs();
  });

  async function loadConfigs() {
    loading = true;
    error = "";
    try {
      configs = (await api.ldap.listConfigs()) || [];
    } catch (e) {
      console.error("Failed to load LDAP configs:", e);
      error = "Failed to load LDAP configurations";
    } finally {
      loading = false;
    }
  }

  function openAddConfig() {
    editingConfig = null;
    configForm = {
      name: "",
      host: "",
      port: 389,
      use_ssl: false,
      bind_dn: "",
      bind_password: "",
      base_dn: "",
      user_filter: "(objectClass=person)",
      username_attribute: "uid",
      email_attribute: "mail",
      display_name_attribute: "cn",
      group_filter: "(objectClass=group)",
      group_membership_attribute: "memberOf",
      enabled: true,
      sync_interval_minutes: 60,
    };
    showConfigModal = true;
  }

  function openEditConfig(config) {
    editingConfig = config;
    configForm = {
      name: config.name || "",
      host: config.host || "",
      port: config.port || 389,
      use_ssl: config.use_ssl || false,
      bind_dn: config.bind_dn || "",
      bind_password: "", // Don't show existing password
      base_dn: config.base_dn || "",
      user_filter: config.user_filter || "(objectClass=person)",
      username_attribute: config.username_attribute || "uid",
      email_attribute: config.email_attribute || "mail",
      display_name_attribute: config.display_name_attribute || "cn",
      group_filter: config.group_filter || "(objectClass=group)",
      group_membership_attribute:
        config.group_membership_attribute || "memberOf",
      enabled: config.enabled ?? true,
      sync_interval_minutes: config.sync_interval_minutes || 60,
    };
    showConfigModal = true;
  }

  async function saveConfig() {
    if (!configForm.name || !configForm.host || !configForm.base_dn) {
      error = "Name, Host, and Base DN are required";
      return;
    }

    saving = true;
    error = "";
    try {
      const data = { ...configForm };
      // Don't send empty password on update
      if (editingConfig && !data.bind_password) {
        delete data.bind_password;
      }

      if (editingConfig) {
        await api.ldap.updateConfig(editingConfig.id, data);
        success = "LDAP configuration updated successfully";
      } else {
        await api.ldap.createConfig(data);
        success = "LDAP configuration created successfully";
      }
      showConfigModal = false;
      await loadConfigs();
      setTimeout(() => (success = ""), 3000);
    } catch (e) {
      error = e.message || "Failed to save configuration";
    } finally {
      saving = false;
    }
  }

  async function deleteConfig(config) {
    if (!confirm(`Delete LDAP configuration "${config.name}"?`)) return;
    try {
      await api.ldap.deleteConfig(config.id);
      success = "Configuration deleted successfully";
      await loadConfigs();
      setTimeout(() => (success = ""), 3000);
    } catch (e) {
      error = e.message || "Failed to delete configuration";
    }
  }

  async function testConnection(config) {
    testing = true;
    error = "";
    try {
      const result = await api.ldap.testConnection(config.id);
      if (result.success) {
        success = `Connection successful! Found ${result.users_found || 0} users`;
      } else {
        error = result.error || "Connection test failed";
      }
      setTimeout(() => {
        success = "";
        error = "";
      }, 5000);
    } catch (e) {
      error = e.message || "Connection test failed";
    } finally {
      testing = false;
    }
  }

  async function syncUsers(config) {
    syncing = true;
    error = "";
    try {
      const result = await api.ldap.sync(config.id);
      success = `Sync complete: ${result.synced || 0} users synced, ${result.created || 0} created, ${result.updated || 0} updated`;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Sync failed";
    } finally {
      syncing = false;
    }
  }

  async function syncAllConfigs() {
    syncing = true;
    error = "";
    try {
      const result = await api.ldap.syncAll();
      success = `Sync complete: ${result.total_synced || 0} users synced across all configurations`;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Sync failed";
    } finally {
      syncing = false;
    }
  }

  function closeModal() {
    showConfigModal = false;
    editingConfig = null;
  }

  function handlePortChange() {
    // Auto-adjust port based on SSL toggle
    if (configForm.use_ssl && configForm.port === 389) {
      configForm.port = 636;
    } else if (!configForm.use_ssl && configForm.port === 636) {
      configForm.port = 389;
    }
  }
</script>

<div class="ldap-settings">
  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-circle-fill"></i>
      <span>{error}</span>
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => (error = "")}
        aria-label="Dismiss"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if success}
    <div class="alert alert-success mb-4">
      <i class="bi bi-check-circle-fill"></i>
      <span>{success}</span>
    </div>
  {/if}

  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else}
    <!-- LDAP Configurations -->
    <div class="card bg-base-200">
      <div class="card-body">
        <div class="flex justify-between items-center mb-4">
          <h3 class="card-title">
            <i class="bi bi-diagram-3 text-primary"></i>
            LDAP/Active Directory Configurations
          </h3>
          <div class="flex gap-2">
            {#if configs.length > 0}
              <ModernButton
                variant="outline"
                size="sm"
                onclick={syncAllConfigs}
                disabled={syncing}
              >
                {#if syncing}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  <i class="bi bi-arrow-repeat"></i>
                {/if}
                Sync All
              </ModernButton>
            {/if}
            <ModernButton variant="primary" size="sm" onclick={openAddConfig}>
              <i class="bi bi-plus-lg"></i>
              Add Configuration
            </ModernButton>
          </div>
        </div>

        {#if configs.length === 0}
          <div class="text-center py-8 text-base-content/60">
            <i class="bi bi-diagram-3 text-4xl mb-2"></i>
            <p>No LDAP configurations</p>
            <p class="text-sm">Add an LDAP/AD server to sync users</p>
          </div>
        {:else}
          <div class="space-y-4">
            {#each configs as config}
              <div class="bg-base-100 rounded-lg p-4">
                <div class="flex justify-between items-start">
                  <div class="flex-1">
                    <div class="flex items-center gap-3 mb-2">
                      <h4 class="font-bold text-lg">{config.name}</h4>
                      {#if config.enabled}
                        <span class="badge badge-success badge-sm">Enabled</span
                        >
                      {:else}
                        <span class="badge badge-ghost badge-sm">Disabled</span>
                      {/if}
                    </div>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                      <div>
                        <span class="text-base-content/60">Host:</span>
                        <p class="font-mono">{config.host}:{config.port}</p>
                      </div>
                      <div>
                        <span class="text-base-content/60">Base DN:</span>
                        <p class="font-mono text-xs">{config.base_dn}</p>
                      </div>
                      <div>
                        <span class="text-base-content/60">SSL:</span>
                        <p>{config.use_ssl ? "Yes" : "No"}</p>
                      </div>
                      <div>
                        <span class="text-base-content/60">Sync Interval:</span>
                        <p>{config.sync_interval_minutes} min</p>
                      </div>
                    </div>
                    {#if config.last_sync}
                      <p class="text-xs text-base-content/50 mt-2">
                        Last synced: {new Date(
                          config.last_sync
                        ).toLocaleString()}
                      </p>
                    {/if}
                  </div>
                  <div class="flex gap-1 ml-4">
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      onclick={() => testConnection(config)}
                      disabled={testing}
                      aria-label="Test connection"
                    >
                      {#if testing}
                        <span class="loading loading-spinner loading-xs"></span>
                      {:else}
                        <i class="bi bi-plug"></i>
                      {/if}
                    </ModernButton>
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      onclick={() => syncUsers(config)}
                      disabled={syncing}
                      aria-label="Sync users"
                    >
                      {#if syncing}
                        <span class="loading loading-spinner loading-xs"></span>
                      {:else}
                        <i class="bi bi-arrow-repeat"></i>
                      {/if}
                    </ModernButton>
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      onclick={() => openEditConfig(config)}
                      aria-label="Edit configuration"
                    >
                      <i class="bi bi-pencil"></i>
                    </ModernButton>
                    <ModernButton
                      variant="danger"
                      size="sm"
                      onclick={() => deleteConfig(config)}
                      aria-label="Delete configuration"
                    >
                      <i class="bi bi-trash"></i>
                    </ModernButton>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}

        <!-- Help Section -->
        <div class="divider"></div>
        <div class="text-sm text-base-content/70">
          <h4 class="font-medium mb-2">Configuration Tips:</h4>
          <ul class="list-disc list-inside space-y-1">
            <li>For Active Directory, use port 389 (LDAP) or 636 (LDAPS)</li>
            <li>
              Bind DN format: <code class="bg-base-300 px-1 rounded"
                >cn=admin,dc=example,dc=com</code
              >
            </li>
            <li>
              User filter example: <code class="bg-base-300 px-1 rounded"
                >(objectClass=user)</code
              >
            </li>
            <li>
              Group filter for AD: <code class="bg-base-300 px-1 rounded"
                >(objectClass=group)</code
              >
            </li>
          </ul>
        </div>
      </div>
    </div>
  {/if}
</div>

<!-- Add/Edit Config Modal -->
{#if showConfigModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeModal}
      onkeydown={(e) => e.key === "Escape" && closeModal()}
    ></div>
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-lg mb-4">
        {editingConfig ? "Edit LDAP Configuration" : "Add LDAP Configuration"}
      </h3>

      <div class="space-y-4 max-h-[60vh] overflow-y-auto pr-2">
        <!-- Basic Settings -->
        <div class="bg-base-200 rounded-lg p-4">
          <h4 class="font-medium mb-3 flex items-center gap-2">
            <i class="bi bi-gear"></i>
            Basic Settings
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control md:col-span-2">
              <label class="label" for="config-name">
                <span class="label-text">Configuration Name *</span>
              </label>
              <input
                id="config-name"
                type="text"
                class="input input-bordered w-full"
                bind:value={configForm.name}
                placeholder="e.g., Corporate AD"
              />
            </div>
            <div class="form-control">
              <label class="label" for="config-host">
                <span class="label-text">Host *</span>
              </label>
              <input
                id="config-host"
                type="text"
                class="input input-bordered w-full"
                bind:value={configForm.host}
                placeholder="ldap.example.com"
              />
            </div>
            <div class="form-control">
              <label class="label" for="config-port">
                <span class="label-text">Port</span>
              </label>
              <input
                id="config-port"
                type="number"
                class="input input-bordered w-full"
                bind:value={configForm.port}
              />
            </div>
            <div class="form-control md:col-span-2">
              <label class="label cursor-pointer justify-start gap-3">
                <input
                  type="checkbox"
                  class="toggle toggle-primary"
                  bind:checked={configForm.use_ssl}
                  onchange={handlePortChange}
                />
                <span class="label-text">Use SSL/TLS (LDAPS)</span>
              </label>
            </div>
          </div>
        </div>

        <!-- Bind Credentials -->
        <div class="bg-base-200 rounded-lg p-4">
          <h4 class="font-medium mb-3 flex items-center gap-2">
            <i class="bi bi-key"></i>
            Bind Credentials
          </h4>
          <div class="space-y-4">
            <div class="form-control">
              <label class="label" for="bind-dn">
                <span class="label-text">Bind DN</span>
              </label>
              <input
                id="bind-dn"
                type="text"
                class="input input-bordered w-full font-mono text-sm"
                bind:value={configForm.bind_dn}
                placeholder="cn=admin,dc=example,dc=com"
              />
            </div>
            <div class="form-control">
              <label class="label" for="bind-password">
                <span class="label-text">Bind Password</span>
              </label>
              <input
                id="bind-password"
                type="password"
                class="input input-bordered w-full"
                bind:value={configForm.bind_password}
                placeholder={editingConfig
                  ? "Leave empty to keep existing"
                  : "Enter password"}
              />
            </div>
            <div class="form-control">
              <label class="label" for="base-dn">
                <span class="label-text">Base DN *</span>
              </label>
              <input
                id="base-dn"
                type="text"
                class="input input-bordered w-full font-mono text-sm"
                bind:value={configForm.base_dn}
                placeholder="dc=example,dc=com"
              />
            </div>
          </div>
        </div>

        <!-- User Mapping -->
        <div class="bg-base-200 rounded-lg p-4">
          <h4 class="font-medium mb-3 flex items-center gap-2">
            <i class="bi bi-person"></i>
            User Mapping
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control md:col-span-2">
              <label class="label" for="user-filter">
                <span class="label-text">User Filter</span>
              </label>
              <input
                id="user-filter"
                type="text"
                class="input input-bordered w-full font-mono text-sm"
                bind:value={configForm.user_filter}
                placeholder="(objectClass=person)"
              />
            </div>
            <div class="form-control">
              <label class="label" for="username-attr">
                <span class="label-text">Username Attribute</span>
              </label>
              <input
                id="username-attr"
                type="text"
                class="input input-bordered w-full"
                bind:value={configForm.username_attribute}
                placeholder="uid"
              />
            </div>
            <div class="form-control">
              <label class="label" for="email-attr">
                <span class="label-text">Email Attribute</span>
              </label>
              <input
                id="email-attr"
                type="text"
                class="input input-bordered w-full"
                bind:value={configForm.email_attribute}
                placeholder="mail"
              />
            </div>
            <div class="form-control md:col-span-2">
              <label class="label" for="display-name-attr">
                <span class="label-text">Display Name Attribute</span>
              </label>
              <input
                id="display-name-attr"
                type="text"
                class="input input-bordered w-full"
                bind:value={configForm.display_name_attribute}
                placeholder="cn"
              />
            </div>
          </div>
        </div>

        <!-- Group Mapping -->
        <div class="bg-base-200 rounded-lg p-4">
          <h4 class="font-medium mb-3 flex items-center gap-2">
            <i class="bi bi-people"></i>
            Group Mapping
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label" for="group-filter">
                <span class="label-text">Group Filter</span>
              </label>
              <input
                id="group-filter"
                type="text"
                class="input input-bordered w-full font-mono text-sm"
                bind:value={configForm.group_filter}
                placeholder="(objectClass=group)"
              />
            </div>
            <div class="form-control">
              <label class="label" for="group-membership-attr">
                <span class="label-text">Group Membership Attribute</span>
              </label>
              <input
                id="group-membership-attr"
                type="text"
                class="input input-bordered w-full"
                bind:value={configForm.group_membership_attribute}
                placeholder="memberOf"
              />
            </div>
          </div>
        </div>

        <!-- Sync Settings -->
        <div class="bg-base-200 rounded-lg p-4">
          <h4 class="font-medium mb-3 flex items-center gap-2">
            <i class="bi bi-arrow-repeat"></i>
            Sync Settings
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label" for="sync-interval">
                <span class="label-text">Sync Interval (minutes)</span>
              </label>
              <input
                id="sync-interval"
                type="number"
                class="input input-bordered w-full"
                bind:value={configForm.sync_interval_minutes}
                min="5"
              />
            </div>
            <div class="form-control flex items-end">
              <label class="label cursor-pointer justify-start gap-3">
                <input
                  type="checkbox"
                  class="toggle toggle-primary"
                  bind:checked={configForm.enabled}
                />
                <span class="label-text">Enable Configuration</span>
              </label>
            </div>
          </div>
        </div>
      </div>

      <div class="modal-action">
        <ModernButton variant="ghost" onclick={closeModal}>Cancel</ModernButton>
        <ModernButton variant="primary" onclick={saveConfig} disabled={saving}>
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          {editingConfig ? "Update" : "Create"} Configuration
        </ModernButton>
      </div>
    </div>
  </div>
{/if}
