<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let saving = $state(false);
  let error = $state("");
  let success = $state("");

  // OAuth Providers
  let providers = $state([]);
  let linkedAccounts = $state([]);

  // Add/Edit Provider Modal
  let showProviderModal = $state(false);
  let editingProvider = $state(null);
  let providerForm = $state({
    provider: "google",
    client_id: "",
    client_secret: "",
    redirect_uri: "",
    enabled: true,
  });

  const providerOptions = [
    { id: "google", name: "Google", icon: "google", color: "#DB4437" },
    { id: "github", name: "GitHub", icon: "github", color: "#333" },
    { id: "microsoft", name: "Microsoft", icon: "microsoft", color: "#00A4EF" },
    { id: "apple", name: "Apple", icon: "apple", color: "#000" },
    { id: "discord", name: "Discord", icon: "discord", color: "#5865F2" },
    { id: "gitlab", name: "GitLab", icon: "gitlab", color: "#FC6D26" },
  ];

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    error = "";
    try {
      const [providersData, accountsData] = await Promise.all([
        api.oauth.listProviders(),
        api.oauth.getLinkedAccounts(),
      ]);
      providers = providersData || [];
      linkedAccounts = accountsData || [];
    } catch (e) {
      console.error("Failed to load OAuth data:", e);
      error = "Failed to load OAuth configuration";
    } finally {
      loading = false;
    }
  }

  function openAddProvider() {
    editingProvider = null;
    providerForm = {
      provider: "google",
      client_id: "",
      client_secret: "",
      redirect_uri: window.location.origin + "/api/oauth/callback",
      enabled: true,
    };
    showProviderModal = true;
  }

  function openEditProvider(provider) {
    editingProvider = provider;
    providerForm = {
      provider: provider.provider,
      client_id: provider.client_id,
      client_secret: "", // Don't show existing secret
      redirect_uri:
        provider.redirect_uri || window.location.origin + "/api/oauth/callback",
      enabled: provider.enabled,
    };
    showProviderModal = true;
  }

  async function saveProvider() {
    saving = true;
    error = "";
    try {
      if (editingProvider) {
        // Update existing
        await api.oauth.configureProvider(providerForm.provider, {
          client_id: providerForm.client_id,
          client_secret: providerForm.client_secret || undefined,
          redirect_uri: providerForm.redirect_uri,
          enabled: providerForm.enabled,
        });
        success = "Provider updated successfully";
      } else {
        // Create new
        await api.oauth.configureProvider(providerForm.provider, {
          client_id: providerForm.client_id,
          client_secret: providerForm.client_secret,
          redirect_uri: providerForm.redirect_uri,
          enabled: providerForm.enabled,
        });
        success = "Provider added successfully";
      }
      showProviderModal = false;
      await loadData();
      setTimeout(() => (success = ""), 3000);
    } catch (e) {
      error = e.message || "Failed to save provider";
    } finally {
      saving = false;
    }
  }

  async function deleteProvider(provider) {
    if (!confirm(`Delete OAuth provider "${provider.provider}"?`)) return;
    try {
      await api.oauth.deleteProvider(provider.provider);
      success = "Provider deleted successfully";
      await loadData();
      setTimeout(() => (success = ""), 3000);
    } catch (e) {
      error = e.message || "Failed to delete provider";
    }
  }

  async function unlinkAccount(account) {
    if (!confirm(`Unlink ${account.provider} account?`)) return;
    try {
      await api.oauth.unlink(account.provider);
      success = "Account unlinked successfully";
      await loadData();
      setTimeout(() => (success = ""), 3000);
    } catch (e) {
      error = e.message || "Failed to unlink account";
    }
  }

  async function linkProvider(providerId) {
    try {
      const response = await api.oauth.getAuthUrl(providerId);
      if (response.url) {
        window.location.href = response.url;
      }
    } catch (e) {
      error = e.message || "Failed to get authentication URL";
    }
  }

  function getProviderInfo(providerId) {
    return (
      providerOptions.find((p) => p.id === providerId) || {
        name: providerId,
        icon: "link",
        color: "#666",
      }
    );
  }

  function closeModal() {
    showProviderModal = false;
    editingProvider = null;
  }
</script>

<div class="oauth-settings">
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
    <!-- OAuth Providers Configuration (Admin) -->
    <div class="card bg-base-200 mb-6">
      <div class="card-body">
        <div class="flex justify-between items-center mb-4">
          <h3 class="card-title">
            <i class="bi bi-gear text-primary"></i>
            OAuth Providers Configuration
          </h3>
          <button class="btn btn-primary btn-sm" onclick={openAddProvider}>
            <i class="bi bi-plus-lg"></i>
            Add Provider
          </button>
        </div>

        {#if providers.length === 0}
          <div class="text-center py-8 text-base-content/60">
            <i class="bi bi-shield-x text-4xl mb-2"></i>
            <p>No OAuth providers configured</p>
            <p class="text-sm">Add a provider to enable social login</p>
          </div>
        {:else}
          <div class="overflow-x-auto">
            <table class="table">
              <thead>
                <tr>
                  <th>Provider</th>
                  <th>Client ID</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each providers as provider}
                  {@const info = getProviderInfo(provider.provider)}
                  <tr>
                    <td>
                      <div class="flex items-center gap-2">
                        <i class="bi bi-{info.icon}" style="color: {info.color}"
                        ></i>
                        <span class="font-medium">{info.name}</span>
                      </div>
                    </td>
                    <td>
                      <code class="text-xs bg-base-300 px-2 py-1 rounded">
                        {provider.client_id?.substring(0, 20)}...
                      </code>
                    </td>
                    <td>
                      {#if provider.enabled}
                        <span class="badge badge-success">Enabled</span>
                      {:else}
                        <span class="badge badge-ghost">Disabled</span>
                      {/if}
                    </td>
                    <td>
                      <div class="flex gap-1">
                        <button
                          class="btn btn-ghost btn-xs"
                          onclick={() => openEditProvider(provider)}
                          aria-label="Edit provider"
                        >
                          <i class="bi bi-pencil"></i>
                        </button>
                        <button
                          class="btn btn-ghost btn-xs text-error"
                          onclick={() => deleteProvider(provider)}
                          aria-label="Delete provider"
                        >
                          <i class="bi bi-trash"></i>
                        </button>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
    </div>

    <!-- Linked Accounts (User) -->
    <div class="card bg-base-200">
      <div class="card-body">
        <h3 class="card-title mb-4">
          <i class="bi bi-link-45deg text-primary"></i>
          Linked Accounts
        </h3>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each providerOptions as option}
            {@const linked = linkedAccounts.find(
              (a) => a.provider === option.id
            )}
            {@const providerConfig = providers.find(
              (p) => p.provider === option.id
            )}
            <div
              class="flex items-center justify-between p-4 bg-base-100 rounded-lg"
            >
              <div class="flex items-center gap-3">
                <div
                  class="w-10 h-10 rounded-full flex items-center justify-center"
                  style="background-color: {option.color}20"
                >
                  <i
                    class="bi bi-{option.icon} text-lg"
                    style="color: {option.color}"
                  ></i>
                </div>
                <div>
                  <p class="font-medium">{option.name}</p>
                  {#if linked}
                    <p class="text-xs text-success">Connected</p>
                  {:else}
                    <p class="text-xs text-base-content/60">Not connected</p>
                  {/if}
                </div>
              </div>
              <div>
                {#if linked}
                  <button
                    class="btn btn-ghost btn-sm text-error"
                    onclick={() => unlinkAccount(linked)}
                  >
                    Unlink
                  </button>
                {:else if providerConfig?.enabled}
                  <button
                    class="btn btn-primary btn-sm"
                    onclick={() => linkProvider(option.id)}
                  >
                    Link
                  </button>
                {:else}
                  <span class="text-xs text-base-content/40">Not available</span
                  >
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>

<!-- Add/Edit Provider Modal -->
{#if showProviderModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeModal}
      onkeydown={(e) => e.key === "Escape" && closeModal()}
    ></div>
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        {editingProvider ? "Edit OAuth Provider" : "Add OAuth Provider"}
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="provider-select">
            <span class="label-text">Provider</span>
          </label>
          <select
            id="provider-select"
            class="select select-bordered w-full"
            bind:value={providerForm.provider}
            disabled={!!editingProvider}
          >
            {#each providerOptions as option}
              <option value={option.id}>{option.name}</option>
            {/each}
          </select>
        </div>

        <div class="form-control">
          <label class="label" for="client-id">
            <span class="label-text">Client ID</span>
          </label>
          <input
            id="client-id"
            type="text"
            class="input input-bordered w-full"
            bind:value={providerForm.client_id}
            placeholder="Enter client ID"
          />
        </div>

        <div class="form-control">
          <label class="label" for="client-secret">
            <span class="label-text">Client Secret</span>
          </label>
          <input
            id="client-secret"
            type="password"
            class="input input-bordered w-full"
            bind:value={providerForm.client_secret}
            placeholder={editingProvider
              ? "Leave empty to keep existing"
              : "Enter client secret"}
          />
        </div>

        <div class="form-control">
          <label class="label" for="redirect-uri">
            <span class="label-text">Redirect URI</span>
          </label>
          <input
            id="redirect-uri"
            type="url"
            class="input input-bordered w-full"
            bind:value={providerForm.redirect_uri}
            placeholder="https://yourdomain.com/api/oauth/callback"
          />
          <label class="label">
            <span class="label-text-alt text-base-content/60">
              Use this URL in your OAuth provider settings
            </span>
          </label>
        </div>

        <div class="form-control">
          <label class="label cursor-pointer">
            <span class="label-text">Enabled</span>
            <input
              type="checkbox"
              class="toggle toggle-primary"
              bind:checked={providerForm.enabled}
            />
          </label>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={closeModal}>Cancel</button>
        <button
          class="btn btn-primary"
          onclick={saveProvider}
          disabled={saving}
        >
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          {editingProvider ? "Update" : "Add"} Provider
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
  }
</style>
