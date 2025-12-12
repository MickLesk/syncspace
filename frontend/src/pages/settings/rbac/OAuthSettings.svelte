<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import { showToast } from "../../../stores/toast.js";
  import api from "../../../lib/api.js";
  import PageWrapper from "../../../components/PageWrapper.svelte";
  import ModernCard from "../../../components/ui/ModernCard.svelte";
  import ModernButton from "../../../components/ui/ModernButton.svelte";
  import EmptyState from "../../../components/ui/EmptyState.svelte";
  import LoadingState from "../../../components/ui/LoadingState.svelte";
  import UIModal from "../../../components/ui/UIModal.svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";

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

<PageWrapper gradient>
  <ModernCard>
    <div class="flex items-center justify-between flex-wrap gap-4 mb-8">
      <div>
        <h1
          class="text-3xl font-bold text-neutral-content flex items-center gap-3"
        >
          <i class="bi bi-shield-lock-fill" aria-hidden="true"></i>
          OAuth Einstellungen
        </h1>
        <p class="text-base-content/70 mt-2">
          Konfigurieren Sie OAuth-Provider für Single Sign-On
        </p>
      </div>
      <ModernButton variant="primary" onclick={openAddProvider}>
        <i class="bi bi-plus-lg" aria-hidden="true"></i>
        Provider hinzufügen
      </ModernButton>
    </div>

    {#if error}
      <div class="alert alert-error mb-6">
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
      <div class="alert alert-success mb-6">
        <i class="bi bi-check-circle-fill"></i>
        <span>{success}</span>
      </div>
    {/if}

    {#if loading}
      <LoadingState />
    {:else}
      <!-- OAuth Providers Configuration -->
      <div class="mb-8">
        <h2
          class="text-xl font-semibold text-neutral-content mb-4 flex items-center gap-2"
        >
          <i class="bi bi-gear text-primary"></i>
          OAuth Providers Configuration
        </h2>

        {#if providers.length === 0}
          <EmptyState
            icon="shield-x"
            title="Keine OAuth Provider konfiguriert"
            description="Füge einen Provider hinzu um Social Login zu aktivieren"
          >
            <ModernButton variant="primary" onclick={openAddProvider}>
              <i class="bi bi-plus-lg"></i>
              Provider hinzufügen
            </ModernButton>
          </EmptyState>
        {:else}
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
            {#each providers as provider}
              {@const info = getProviderInfo(provider.provider)}
              <ModernCard variant="glass">
                <div class="p-4">
                  <div class="flex items-center justify-between mb-4">
                    <div class="flex items-center gap-3">
                      <div
                        class="w-12 h-12 rounded-xl flex items-center justify-center"
                        style="background-color: {info.color}20"
                      >
                        <i
                          class="bi bi-{info.icon} text-2xl"
                          style="color: {info.color}"
                          aria-hidden="true"
                        ></i>
                      </div>
                      <div>
                        <h3 class="font-bold text-lg text-neutral-content">
                          {info.name}
                        </h3>
                        <p class="text-sm text-base-content/60">
                          {#if provider.enabled}
                            <span class="text-success flex items-center gap-1">
                              <i class="bi bi-check-circle-fill"></i>
                              Aktiviert
                            </span>
                          {:else}
                            <span
                              class="text-base-content/40 flex items-center gap-1"
                            >
                              <i class="bi bi-x-circle"></i>
                              Deaktiviert
                            </span>
                          {/if}
                        </p>
                      </div>
                    </div>
                    <div class="flex gap-2">
                      <ModernButton
                        variant="ghost"
                        size="sm"
                        onclick={() => openEditProvider(provider)}
                        aria-label="Edit provider"
                      >
                        <i class="bi bi-pencil"></i>
                      </ModernButton>
                      <ModernButton
                        variant="ghost"
                        size="sm"
                        class="text-error hover:text-error"
                        onclick={() => deleteProvider(provider)}
                        aria-label="Delete provider"
                      >
                        <i class="bi bi-trash"></i>
                      </ModernButton>
                    </div>
                  </div>
                  <div class="text-sm">
                    <p class="text-base-content/60 mb-1">Client ID:</p>
                    <code class="text-xs bg-base-300 px-2 py-1 rounded">
                      {provider.client_id?.substring(0, 30)}...
                    </code>
                  </div>
                </div>
              </ModernCard>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Linked Accounts -->
      <div>
        <h2
          class="text-xl font-semibold text-neutral-content mb-4 flex items-center gap-2"
        >
          <i class="bi bi-link-45deg text-primary"></i>
          Verknüpfte Accounts
        </h2>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each providerOptions as option}
            {@const linked = linkedAccounts.find(
              (a) => a.provider === option.id
            )}
            {@const providerConfig = providers.find(
              (p) => p.provider === option.id
            )}
            <ModernCard variant="glass">
              <div class="p-4 flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div
                    class="w-10 h-10 rounded-full flex items-center justify-center"
                    style="background-color: {option.color}20"
                  >
                    <i
                      class="bi bi-{option.icon} text-lg"
                      style="color: {option.color}"
                      aria-hidden="true"
                    ></i>
                  </div>
                  <div>
                    <p class="font-medium text-neutral-content">
                      {option.name}
                    </p>
                    {#if linked}
                      <p class="text-xs text-success flex items-center gap-1">
                        <i class="bi bi-check-circle-fill"></i>
                        Verbunden
                      </p>
                    {:else}
                      <p class="text-xs text-base-content/60">
                        Nicht verbunden
                      </p>
                    {/if}
                  </div>
                </div>
                <div>
                  {#if linked}
                    <ModernButton
                      variant="outline"
                      size="sm"
                      class="text-error hover:text-error"
                      onclick={() => unlinkAccount(linked)}
                    >
                      Trennen
                    </ModernButton>
                  {:else if providerConfig?.enabled}
                    <ModernButton
                      variant="primary"
                      size="sm"
                      onclick={() => linkProvider(option.id)}
                    >
                      Verknüpfen
                    </ModernButton>
                  {:else}
                    <span class="text-xs text-base-content/40"
                      >Nicht verfügbar</span
                    >
                  {/if}
                </div>
              </div>
            </ModernCard>
          {/each}
        </div>
      </div>
    {/if}
  </ModernCard>
</PageWrapper>

<!-- Add/Edit Provider Modal -->
<UIModal
  bind:show={showProviderModal}
  title={editingProvider
    ? "OAuth Provider bearbeiten"
    : "OAuth Provider hinzufügen"}
  size="lg"
  actions={[
    {
      label: "Abbrechen",
      variant: "secondary",
      onClick: closeModal,
    },
    {
      label: editingProvider ? "Aktualisieren" : "Hinzufügen",
      variant: "primary",
      onClick: saveProvider,
      disabled: saving,
    },
  ]}
  loading={saving}
>
  <div class="space-y-4">
    <UISelect
      label="Provider"
      bind:value={providerForm.provider}
      options={providerOptions.map((p) => ({ value: p.id, label: p.name }))}
      disabled={!!editingProvider}
    />

    <UIInput
      label="Client ID"
      bind:value={providerForm.client_id}
      placeholder="Client ID eingeben"
      required
    />

    <UIInput
      label="Client Secret"
      type="password"
      bind:value={providerForm.client_secret}
      placeholder={editingProvider
        ? "Leer lassen um beizubehalten"
        : "Client Secret eingeben"}
    />

    <UIInput
      label="Redirect URI"
      type="url"
      bind:value={providerForm.redirect_uri}
      placeholder="https://yourdomain.com/api/oauth/callback"
      description="Verwende diese URL in deinen OAuth Provider Einstellungen"
    />

    <UIToggle
      label="Aktiviert"
      bind:checked={providerForm.enabled}
      description="Provider für Benutzer verfügbar machen"
    />
  </div>
</UIModal>
