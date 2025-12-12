<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIModal from "../../../components/ui/UIModal.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { apiTokens } from "../../lib/api.js";
  import { t } from "../../i18n.js";

  // State
  let tokens = $state([]);
  let loading = $state(true);
  let error = $state(null);

  // Modal states
  let showCreateModal = $state(false);
  let showTokenModal = $state(false);
  let showRevokeConfirm = $state(false);
  let showDeleteConfirm = $state(false);

  // Form state
  let tokenName = $state("");
  let selectedScopes = $state([]);
  let expiresInDays = $state(null);
  let creating = $state(false);

  // Created token (shown once)
  let createdToken = $state(null);
  let tokenCopied = $state(false);

  // Selected token for actions
  let selectedToken = $state(null);

  // Available scopes
  const availableScopes = [
    { id: "read_files", label: "scopeReadFiles", icon: "bi-eye" },
    { id: "write_files", label: "scopeWriteFiles", icon: "bi-pencil" },
    { id: "delete_files", label: "scopeDeleteFiles", icon: "bi-trash" },
    { id: "manage_shares", label: "scopeManageShares", icon: "bi-share" },
    { id: "manage_users", label: "scopeManageUsers", icon: "bi-people" },
    { id: "admin", label: "scopeAdmin", icon: "bi-shield-check" },
    { id: "full", label: "scopeFull", icon: "bi-key" },
  ];

  // Expiration options
  const expirationOptions = [
    { value: null, label: "neverExpires" },
    { value: 30, label: "days30" },
    { value: 60, label: "days60" },
    { value: 90, label: "days90" },
    { value: 365, label: "days365" },
  ];

  onMount(async () => {
    await loadTokens();
  });

  async function loadTokens() {
    loading = true;
    error = null;
    try {
      tokens = await apiTokens.list();
    } catch (e) {
      console.error("Failed to load tokens:", e);
      error = $t("apiTokens.loadError");
    } finally {
      loading = false;
    }
  }

  function openCreateModal() {
    tokenName = "";
    selectedScopes = [];
    expiresInDays = null;
    creating = false;
    showCreateModal = true;
  }

  function closeCreateModal() {
    showCreateModal = false;
    tokenName = "";
    selectedScopes = [];
    expiresInDays = null;
  }

  function toggleScope(scopeId) {
    if (selectedScopes.includes(scopeId)) {
      selectedScopes = selectedScopes.filter((s) => s !== scopeId);
    } else {
      selectedScopes = [...selectedScopes, scopeId];
    }
  }

  async function createToken() {
    if (!tokenName.trim()) {
      error = $t("apiTokens.nameRequired");
      return;
    }
    if (selectedScopes.length === 0) {
      error = $t("apiTokens.scopeRequired");
      return;
    }

    creating = true;
    error = null;
    try {
      const result = await apiTokens.create({
        name: tokenName.trim(),
        scopes: selectedScopes,
        expires_in_days: expiresInDays,
      });
      createdToken = result;
      tokenCopied = false;
      showCreateModal = false;
      showTokenModal = true;
      await loadTokens();
    } catch (e) {
      console.error("Failed to create token:", e);
      error = $t("apiTokens.createError");
    } finally {
      creating = false;
    }
  }

  function closeTokenModal() {
    showTokenModal = false;
    createdToken = null;
    tokenCopied = false;
  }

  async function copyToken() {
    if (createdToken?.token) {
      try {
        await navigator.clipboard.writeText(createdToken.token);
        tokenCopied = true;
        setTimeout(() => {
          tokenCopied = false;
        }, 3000);
      } catch (e) {
        console.error("Failed to copy token:", e);
      }
    }
  }

  function confirmRevoke(token) {
    selectedToken = token;
    showRevokeConfirm = true;
  }

  async function revokeToken() {
    if (!selectedToken) return;

    try {
      await apiTokens.revoke(selectedToken.id);
      showRevokeConfirm = false;
      selectedToken = null;
      await loadTokens();
    } catch (e) {
      console.error("Failed to revoke token:", e);
      error = $t("apiTokens.revokeError");
    }
  }

  function confirmDelete(token) {
    selectedToken = token;
    showDeleteConfirm = true;
  }

  async function deleteToken() {
    if (!selectedToken) return;

    try {
      await apiTokens.delete(selectedToken.id);
      showDeleteConfirm = false;
      selectedToken = null;
      await loadTokens();
    } catch (e) {
      console.error("Failed to delete token:", e);
      error = $t("apiTokens.deleteError");
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return $t("apiTokens.neverUsed");
    const date = new Date(dateStr);
    return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function isExpired(token) {
    if (!token.expires_at) return false;
    return new Date(token.expires_at) < new Date();
  }

  function getScopeLabel(scopeId) {
    const scope = availableScopes.find((s) => s.id === scopeId);
    return scope ? $t(`apiTokens.${scope.label}`) : scopeId;
  }

  function getScopeIcon(scopeId) {
    const scope = availableScopes.find((s) => s.id === scopeId);
    return scope?.icon || "bi-key";
  }
</script>

<div class="p-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div>
      <h1 class="text-2xl font-bold text-base-content flex items-center gap-3">
        <i class="bi bi-key text-primary" aria-hidden="true"></i>
        {$t("apiTokens.title")}
      </h1>
      <p class="text-base-content/60 mt-1">{$t("apiTokens.description")}</p>
    </div>
    <button
      aria-label="Add"
      onclick={openCreateModal}
      class="btn btn-primary gap-2"
      ><i class="bi bi-plus-lg" aria-hidden="true"></i>
      {$t("apiTokens.create")}
    </button>
  </div>

  <!-- Error Alert -->
  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
      <span>{error}</span>
      <button
        aria-label="Close"
        onclick={() => (error = null)}
        class="btn btn-ghost btn-sm"
        ><i class="bi bi-x-lg" aria-hidden="true"></i></button
      >
    </div>
  {/if}

  <!-- Loading State -->
  {#if loading}
    <div class="flex flex-col items-center justify-center py-16">
      <span class="loading loading-spinner loading-lg text-primary"></span>
      <p class="mt-4 text-base-content/60">{$t("common.loading")}</p>
    </div>
  {:else if tokens.length === 0}
    <!-- Empty State -->
    <div
      class="card bg-gradient-to-br from-base-200/80 to-base-200/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
    >
      <div class="card-body items-center text-center py-16">
        <i
          class="bi bi-key text-6xl text-base-content/30 mb-4"
          aria-hidden="true"
        ></i>
        <h3 class="text-xl font-semibold text-base-content">
          {$t("apiTokens.noTokens")}
        </h3>
        <p class="text-base-content/60 max-w-md">
          {$t("apiTokens.noTokensDesc")}
        </p>
        <button
          aria-label="Add"
          onclick={openCreateModal}
          class="btn btn-primary mt-4 gap-2"
          ><i class="bi bi-plus-lg" aria-hidden="true"></i>
          {$t("apiTokens.createFirst")}
        </button>
      </div>
    </div>
  {:else}
    <!-- Token List -->
    <div class="space-y-4">
      {#each tokens as token (token.id)}
        <div
          class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="card-body p-4">
            <div class="flex items-start justify-between gap-4">
              <!-- Token Info -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2">
                  <h3 class="font-semibold text-base-content truncate">
                    {token.name}
                  </h3>
                  {#if token.is_revoked}
                    <span class="badge badge-error badge-sm gap-1">
                      <i class="bi bi-x-circle text-xs" aria-hidden="true"></i>
                      {$t("apiTokens.revoked")}
                    </span>
                  {:else if isExpired(token)}
                    <span class="badge badge-warning badge-sm gap-1">
                      <i class="bi bi-clock text-xs" aria-hidden="true"></i>
                      {$t("apiTokens.expired")}
                    </span>
                  {:else}
                    <span class="badge badge-success badge-sm gap-1">
                      <i class="bi bi-check-circle text-xs" aria-hidden="true"
                      ></i>
                      {$t("apiTokens.active")}
                    </span>
                  {/if}
                </div>

                <!-- Token ID preview -->
                <p class="text-sm text-base-content/50 font-mono mt-1">
                  pat_...{token.id.slice(-8)}
                </p>

                <!-- Scopes -->
                <div class="flex flex-wrap gap-1 mt-2">
                  {#each token.scopes as scope}
                    <span class="badge badge-outline badge-sm gap-1">
                      <i class="{getScopeIcon(scope)} text-xs"></i>
                      {getScopeLabel(scope)}
                    </span>
                  {/each}
                </div>

                <!-- Metadata -->
                <div
                  class="flex flex-wrap gap-4 mt-3 text-sm text-base-content/60"
                >
                  <span class="flex items-center gap-1">
                    <i class="bi bi-calendar text-xs" aria-hidden="true"></i>
                    {$t("apiTokens.created")}: {formatDate(token.created_at)}
                  </span>
                  <span class="flex items-center gap-1">
                    <i class="bi bi-clock-history text-xs" aria-hidden="true"
                    ></i>
                    {$t("apiTokens.lastUsed")}: {token.last_used_at
                      ? formatDate(token.last_used_at)
                      : $t("apiTokens.neverUsed")}
                  </span>
                  {#if token.expires_at}
                    <span
                      class="flex items-center gap-1"
                      class:text-error={isExpired(token)}
                    >
                      <i class="bi bi-hourglass text-xs" aria-hidden="true"></i>
                      {$t("apiTokens.expires")}: {formatDate(token.expires_at)}
                    </span>
                  {:else}
                    <span class="flex items-center gap-1">
                      <i class="bi bi-infinity text-xs" aria-hidden="true"></i>
                      {$t("apiTokens.noExpiration")}
                    </span>
                  {/if}
                </div>
              </div>

              <!-- Actions -->
              <div class="flex items-center gap-2">
                {#if !token.is_revoked}
                  <button
                    onclick={() => confirmRevoke(token)}
                    class="btn btn-ghost btn-sm text-warning"
                    title={$t("apiTokens.revoke")}
                  >
                    <i class="bi bi-slash-circle" aria-hidden="true"></i>
                  </button>
                {/if}
                <button
                  onclick={() => confirmDelete(token)}
                  class="btn btn-ghost btn-sm text-error"
                  title={$t("apiTokens.delete")}
                  aria-label="Delete"
                >
                  <i class="bi bi-trash" aria-hidden="true"></i>
                </button>
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Create Token Modal -->
{#if showCreateModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-lg">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <i class="bi bi-plus-circle text-primary" aria-hidden="true"></i>
        {$t("apiTokens.createToken")}
      </h3>

      <div class="form-control mt-4">
        <div class="label"><span>{$t("apiTokens.name")}</span></div>
        <input
          type="text"
          bind:value={tokenName}
          placeholder={$t("apiTokens.namePlaceholder")}
          class="input input-bordered w-full"
        />
      </div>

      <div class="form-control mt-4">
        <div class="label"><span>{$t("apiTokens.scopes")}</span></div>
        <p class="text-sm text-base-content/60 mb-2">
          {$t("apiTokens.scopesHelp")}
        </p>
        <div class="grid grid-cols-2 gap-2">
          {#each availableScopes as scope}
            <label
              class="flex items-center gap-2 p-2 rounded-lg border cursor-pointer hover:bg-base-200 transition-colors {selectedScopes.includes(
                scope.id
              )
                ? 'bg-primary/10 border-primary'
                : 'border-base-300'}"
            >
              <input
                type="checkbox"
                class="checkbox checkbox-sm checkbox-primary"
                checked={selectedScopes.includes(scope.id)}
                onchange={() => toggleScope(scope.id)}
              />
              <i class="{scope.icon} text-base-content/70"></i>
              <span class="text-sm">{$t(`apiTokens.${scope.label}`)}</span>
            </label>
          {/each}
        </div>
      </div>

      <div class="form-control mt-4">
        <label for="expires-in-select" class="label">
          <span class="label-text font-medium">{$t("apiTokens.expiresIn")}</span
          >
        </label>
        <select
          id="expires-in-select"
          bind:value={expiresInDays}
          class="select select-bordered w-full"
        >
          {#each expirationOptions as option}
            <option value={option.value}
              >{$t(`apiTokens.${option.label}`)}</option
            >
          {/each}
        </select>
      </div>

      <div class="modal-action">
        <button onclick={closeCreateModal} class="btn btn-ghost">
          {$t("cancel")}
        </button>
        <button
          onclick={createToken}
          disabled={creating ||
            !tokenName.trim() ||
            selectedScopes.length === 0}
          class="btn btn-primary gap-2"
        >
          {#if creating}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            <i class="bi bi-plus-lg" aria-hidden="true"></i>
          {/if}
          {$t("apiTokens.create")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="dialog"
      tabindex="0"
      onclick={closeModal}
      onkeydown={(e = tabindex = "0" > e.key === "Escape" && closeCreateModal)}
    ></div>
  </div>
{/if}

<!-- Token Created Modal -->
{#if showTokenModal && createdToken}
  <div class="modal modal-open">
    <div class="modal-box max-w-xl">
      <div class="flex items-center gap-2 text-success mb-4">
        <i class="bi bi-check-circle text-2xl" aria-hidden="true"></i>
        <h3 class="font-bold text-lg">{$t("apiTokens.tokenCreated")}</h3>
      </div>

      <div class="alert alert-warning mb-4">
        <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
        <span class="text-sm">{$t("apiTokens.tokenWarning")}</span>
      </div>

      <div class="form-control">
        <div class="label"><span>{$t("apiTokens.token")}</span></div>
        <div class="flex gap-2">
          <input
            type="text"
            readonly
            value={createdToken.token}
            class="input input-bordered flex-1 font-mono text-sm"
          />
          <button onclick={copyToken} class="btn btn-primary gap-2">
            {#if tokenCopied}
              <i class="bi bi-check-lg" aria-hidden="true"></i>
              {$t("apiTokens.tokenCopied")}
            {:else}
              <i class="bi bi-clipboard" aria-hidden="true"></i>
              {$t("apiTokens.copyToken")}
            {/if}
          </button>
        </div>
      </div>

      <div class="modal-action">
        <button onclick={closeTokenModal} class="btn btn-primary">
          {$t("close")}
        </button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/50"></div>
  </div>
{/if}

<!-- Revoke Confirmation Modal -->
{#if showRevokeConfirm && selectedToken}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2 text-warning">
        <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
        {$t("apiTokens.revokeToken")}
      </h3>
      <p class="py-4 text-base-content/80">
        {$t("apiTokens.revokeConfirm").replace("{0}", selectedToken.name)}
      </p>
      <div class="modal-action">
        <button
          onclick={() => {
            showRevokeConfirm = false;
            selectedToken = null;
          }}
          class="btn btn-ghost"
        >
          {$t("cancel")}
        </button>
        <button onclick={revokeToken} class="btn btn-warning gap-2">
          <i class="bi bi-slash-circle" aria-hidden="true"></i>
          {$t("apiTokens.revoke")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop bg-black/50"
      onclick={() => {
        showRevokeConfirm = false;
        selectedToken = null;
      }}
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " " || e.key === "Escape") {
          showRevokeConfirm = false;
          selectedToken = null;
        }
      }}
      role="button"
      tabindex="0"
      aria-label="Close modal"
    ></div>
  </div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteConfirm && selectedToken}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2 text-error">
        <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
        {$t("apiTokens.deleteToken")}
      </h3>
      <p class="py-4 text-base-content/80">
        {$t("apiTokens.deleteConfirm").replace("{0}", selectedToken.name)}
      </p>
      <div class="modal-action">
        <button
          onclick={() => {
            showDeleteConfirm = false;
            selectedToken = null;
          }}
          class="btn btn-ghost"
        >
          {$t("cancel")}
        </button>
        <button onclick={deleteToken} class="btn btn-error gap-2">
          <i class="bi bi-trash" aria-hidden="true"></i>
          {$t("apiTokens.delete")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop bg-black/50"
      onclick={() => {
        showDeleteConfirm = false;
        selectedToken = null;
      }}
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " " || e.key === "Escape") {
          showDeleteConfirm = false;
          selectedToken = null;
        }
      }}
      role="button"
      tabindex="0"
      aria-label="Close modal"
    ></div>
  </div>
{/if}
