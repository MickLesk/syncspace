<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIModal from "../../../components/ui/UIModal.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { encryption } from "../../../lib/api.js";
  import { currentLang } from "../../../stores/ui.js";`nimport { t } from "../../../i18n.js";`n`n  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // State
  let keys = $state([]);
  let settings = $state({
    auto_encrypt_uploads: false,
    default_key_id: null,
    encrypt_file_names: false,
    encryption_enabled: false,
  });
  let loading = $state(true);
  let error = $state(null);

  // Modal states
  let showCreateKeyModal = $state(false);
  let showRotateKeyModal = $state(false);
  let showDeleteConfirm = $state(false);
  let selectedKey = $state(null);

  // Form states
  let newKeyName = $state("");
  let newKeyPassword = $state("");
  let confirmPassword = $state("");
  let rotateOldPassword = $state("");
  let rotateNewPassword = $state("");
  let rotateConfirmPassword = $state("");

  // Stats
  let totalEncryptedFiles = $derived(
    keys.reduce((sum, k) => sum + (k.files_count || 0), 0)
  );
  let activeKeys = $derived(keys.filter((k) => k.is_active).length);

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    error = null;
    try {
      const [keysData, settingsData] = await Promise.all([
        encryption.listKeys(),
        encryption.getSettings(),
      ]);
      keys = keysData || [];
      settings = settingsData || settings;
    } catch (err) {
      console.error("Failed to load encryption data:", err);
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function createKey() {
    if (!newKeyName.trim()) {
      error = tr("encryption.errorNameRequired");
      return;
    }
    if (newKeyPassword.length < 8) {
      error = tr("encryption.errorPasswordMin");
      return;
    }
    if (newKeyPassword !== confirmPassword) {
      error = tr("encryption.errorPasswordMismatch");
      return;
    }

    try {
      await encryption.createKey(newKeyName.trim(), newKeyPassword);
      showCreateKeyModal = false;
      newKeyName = "";
      newKeyPassword = "";
      confirmPassword = "";
      await loadData();
    } catch (err) {
      error = err.message;
    }
  }

  async function rotateKey() {
    if (rotateNewPassword.length < 8) {
      error = tr("encryption.errorPasswordMin");
      return;
    }
    if (rotateNewPassword !== rotateConfirmPassword) {
      error = tr("encryption.errorPasswordMismatch");
      return;
    }

    try {
      const result = await encryption.rotateKey(
        selectedKey.id,
        rotateOldPassword,
        rotateNewPassword
      );
      if (!result.success) {
        error = result.message;
        return;
      }
      showRotateKeyModal = false;
      rotateOldPassword = "";
      rotateNewPassword = "";
      rotateConfirmPassword = "";
      selectedKey = null;
      await loadData();
    } catch (err) {
      error = err.message;
    }
  }

  async function deleteKey() {
    if (!selectedKey) return;

    try {
      const result = await encryption.deleteKey(selectedKey.id);
      if (!result.success) {
        error = result.message;
        return;
      }
      showDeleteConfirm = false;
      selectedKey = null;
      await loadData();
    } catch (err) {
      error = err.message;
    }
  }

  async function updateSettings() {
    try {
      await encryption.updateSettings(settings);
    } catch (err) {
      error = err.message;
    }
  }

  function openRotateModal(key) {
    selectedKey = key;
    showRotateKeyModal = true;
  }

  function openDeleteConfirm(key) {
    selectedKey = key;
    showDeleteConfirm = true;
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    return new Date(dateStr).toLocaleString();
  }
</script>

<div class="flex flex-col h-full">
  <!-- Header -->
  <div class="flex items-center justify-between p-6 border-b border-base-300">
    <div>
      <h1 class="text-2xl font-bold flex items-center gap-3">
        <i class="bi bi-shield-lock-fill text-primary"></i>
        {tr("encryption.title")}
      </h1>
      <p class="text-base-content/60 mt-1">{tr("encryption.subtitle")}</p>
    </div>
    <button class="btn btn-primary" onclick={() => (showCreateKeyModal = true)}>
      <i class="bi bi-plus-lg"></i>
      {tr("encryption.createKey")}
    </button>
  </div>

  {#if error}
    <div class="alert alert-error m-4">
      <i class="bi bi-exclamation-triangle"></i>
      <span>{error}</span>
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => (error = null)}
        aria-label="Close"
      >
        <i class="bi bi-x-lg"></i>
      </button>
    </div>
  {/if}

  <div class="flex-1 overflow-auto p-6">
    {#if loading}
      <div class="flex items-center justify-center h-64">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
    {:else}
      <!-- Stats Cards -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-primary">
            <i class="bi bi-key-fill text-3xl"></i>
          </div>
          <div class="stat-title">{tr("encryption.totalKeys")}</div>
          <div class="stat-value text-primary">{keys.length}</div>
        </div>
        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-success">
            <i class="bi bi-check-circle-fill text-3xl"></i>
          </div>
          <div class="stat-title">{tr("encryption.activeKeys")}</div>
          <div class="stat-value text-success">{activeKeys}</div>
        </div>
        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-info">
            <i class="bi bi-file-earmark-lock-fill text-3xl"></i>
          </div>
          <div class="stat-title">{tr("encryption.encryptedFiles")}</div>
          <div class="stat-value text-info">{totalEncryptedFiles}</div>
        </div>
        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-warning">
            <i class="bi bi-shield-fill-check text-3xl"></i>
          </div>
          <div class="stat-title">{tr("encryption.status")}</div>
          <div class="stat-value text-warning text-lg">
            {settings.encryption_enabled
              ? tr("encryption.enabled")
              : tr("encryption.disabled")}
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Keys List -->
        <div class="lg:col-span-2">
          <div class="bg-base-200 rounded-lg p-4">
            <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <i class="bi bi-key"></i>
              {tr("encryption.encryptionKeys")}
            </h2>

            {#if keys.length === 0}
              <div class="text-center py-12 text-base-content/50">
                <i class="bi bi-key text-5xl mb-4 block"></i>
                <p>{tr("encryption.noKeys")}</p>
                <p class="text-sm mt-2">{tr("encryption.noKeysHint")}</p>
              </div>
            {:else}
              <div class="overflow-x-auto">
                <table class="table table-zebra">
                  <thead>
                    <tr>
                      <th>{tr("encryption.keyName")}</th>
                      <th>{tr("encryption.filesEncrypted")}</th>
                      <th>{tr("encryption.created")}</th>
                      <th>{tr("encryption.lastUsed")}</th>
                      <th>{tr("encryption.status")}</th>
                      <th>{tr("encryption.actions")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each keys as key}
                      <tr>
                        <td>
                          <div class="flex items-center gap-2">
                            <i class="bi bi-key-fill text-primary"></i>
                            <span class="font-medium">{key.name}</span>
                          </div>
                        </td>
                        <td>
                          <span class="badge badge-info badge-sm"
                            >{key.files_count}</span
                          >
                        </td>
                        <td class="text-sm text-base-content/70">
                          {formatDate(key.created_at)}
                        </td>
                        <td class="text-sm text-base-content/70">
                          {formatDate(key.last_used_at)}
                        </td>
                        <td>
                          {#if key.is_active}
                            <span class="badge badge-success badge-sm"
                              >{tr("encryption.active")}</span
                            >
                          {:else}
                            <span class="badge badge-error badge-sm"
                              >{tr("encryption.inactive")}</span
                            >
                          {/if}
                        </td>
                        <td>
                          <div class="flex gap-1">
                            <button
                              class="btn btn-ghost btn-xs"
                              title={tr("encryption.rotateKey")}
                              onclick={() => openRotateModal(key)}
                              disabled={!key.is_active}
                            >
                              <i class="bi bi-arrow-repeat"></i>
                            </button>
                            <button
                              class="btn btn-ghost btn-xs text-error"
                              title={tr("encryption.deleteKey")}
                              onclick={() => openDeleteConfirm(key)}
                              disabled={key.files_count > 0}
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

        <!-- Settings Panel -->
        <div>
          <div class="bg-base-200 rounded-lg p-4">
            <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <i class="bi bi-gear"></i>
              {tr("encryption.settings")}
            </h2>

            <div class="space-y-4">
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input
                    type="checkbox"
                    class="toggle toggle-primary"
                    bind:checked={settings.encryption_enabled}
                    onchange={updateSettings}
                  />
                  <div>
                    <span class="label-text font-medium"
                      >{tr("encryption.enableEncryption")}</span
                    >
                    <p class="text-xs text-base-content/60">
                      {tr("encryption.enableEncryptionDesc")}
                    </p>
                  </div>
                </label>
              </div>

              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input
                    type="checkbox"
                    class="toggle toggle-primary"
                    bind:checked={settings.auto_encrypt_uploads}
                    onchange={updateSettings}
                    disabled={!settings.encryption_enabled}
                  />
                  <div>
                    <span class="label-text font-medium"
                      >{tr("encryption.autoEncrypt")}</span
                    >
                    <p class="text-xs text-base-content/60">
                      {tr("encryption.autoEncryptDesc")}
                    </p>
                  </div>
                </label>
              </div>

              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input
                    type="checkbox"
                    class="toggle toggle-primary"
                    bind:checked={settings.encrypt_file_names}
                    onchange={updateSettings}
                    disabled={!settings.encryption_enabled}
                  />
                  <div>
                    <span class="label-text font-medium"
                      >{tr("encryption.encryptFileNames")}</span
                    >
                    <p class="text-xs text-base-content/60">
                      {tr("encryption.encryptFileNamesDesc")}
                    </p>
                  </div>
                </label>
              </div>

              {#if settings.encryption_enabled && keys.length > 0}
                <div class="form-control">
                  <div class="label">
                    <span class="label-text font-medium"
                      >{tr("encryption.defaultKey")}</span
                    >
                  </div>
                  <select
                    class="select select-bordered w-full"
                    bind:value={settings.default_key_id}
                    onchange={updateSettings}
                  >
                    <option value={null}>{tr("encryption.selectKey")}</option>
                    {#each keys.filter((k) => k.is_active) as key}
                      <option value={key.id}>{key.name}</option>
                    {/each}
                  </select>
                </div>
              {/if}
            </div>

            <!-- Info Box -->
            <div class="alert alert-info mt-6">
              <i class="bi bi-info-circle"></i>
              <div class="text-sm">
                <p class="font-medium">{tr("encryption.infoTitle")}</p>
                <p class="text-xs mt-1">{tr("encryption.infoDesc")}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Create Key Modal -->
{#if showCreateKeyModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <i class="bi bi-key-fill text-primary"></i>
        {tr("encryption.createNewKey")}
      </h3>
      <div class="py-4 space-y-4">
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("encryption.keyName")}</span>
          </div>
          <input
            type="text"
            class="input input-bordered w-full"
            placeholder={tr("encryption.keyNamePlaceholder")}
            bind:value={newKeyName}
            aria-label={tr("encryption.keyName")}
          />
        </div>
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("encryption.password")}</span>
          </div>
          <input
            type="password"
            class="input input-bordered w-full"
            placeholder={tr("encryption.passwordPlaceholder")}
            bind:value={newKeyPassword}
            aria-label={tr("encryption.password")}
          />
          <div class="label">
            <span class="label-text-alt text-base-content/60"
              >{tr("encryption.passwordHint")}</span
            >
          </div>
        </div>
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("encryption.confirmPassword")}</span>
          </div>
          <input
            type="password"
            class="input input-bordered w-full"
            placeholder={tr("encryption.confirmPasswordPlaceholder")}
            bind:value={confirmPassword}
            aria-label={tr("encryption.confirmPassword")}
          />
        </div>
      </div>
      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => {
            showCreateKeyModal = false;
            newKeyName = "";
            newKeyPassword = "";
            confirmPassword = "";
          }}
        >
          {tr("common.cancel")}
        </button>
        <button class="btn btn-primary" onclick={createKey}>
          <i class="bi bi-plus-lg"></i>
          {tr("encryption.create")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={() => (showCreateKeyModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showCreateKeyModal = false)}
    ></div>
  </div>
{/if}

<!-- Rotate Key Modal -->
{#if showRotateKeyModal && selectedKey}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <i class="bi bi-arrow-repeat text-warning"></i>
        {tr("encryption.rotateKeyTitle")}
      </h3>
      <p class="text-sm text-base-content/60 mt-2">
        {tr("encryption.rotateKeyDesc", { name: selectedKey.name })}
      </p>
      <div class="py-4 space-y-4">
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("encryption.currentPassword")}</span>
          </div>
          <input
            type="password"
            class="input input-bordered w-full"
            bind:value={rotateOldPassword}
            aria-label={tr("encryption.currentPassword")}
          />
        </div>
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("encryption.newPassword")}</span>
          </div>
          <input
            type="password"
            class="input input-bordered w-full"
            bind:value={rotateNewPassword}
            aria-label={tr("encryption.newPassword")}
          />
        </div>
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("encryption.confirmNewPassword")}</span
            >
          </div>
          <input
            type="password"
            class="input input-bordered w-full"
            bind:value={rotateConfirmPassword}
            aria-label={tr("encryption.confirmNewPassword")}
          />
        </div>
      </div>
      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => {
            showRotateKeyModal = false;
            rotateOldPassword = "";
            rotateNewPassword = "";
            rotateConfirmPassword = "";
            selectedKey = null;
          }}
        >
          {tr("common.cancel")}
        </button>
        <button class="btn btn-warning" onclick={rotateKey}>
          <i class="bi bi-arrow-repeat"></i>
          {tr("encryption.rotate")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={() => (showRotateKeyModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showRotateKeyModal = false)}
    ></div>
  </div>
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteConfirm && selectedKey}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2 text-error">
        <i class="bi bi-exclamation-triangle"></i>
        {tr("encryption.deleteKeyTitle")}
      </h3>
      <p class="py-4">
        {tr("encryption.deleteKeyConfirm", { name: selectedKey.name })}
      </p>
      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => {
            showDeleteConfirm = false;
            selectedKey = null;
          }}
        >
          {tr("common.cancel")}
        </button>
        <button class="btn btn-error" onclick={deleteKey}>
          <i class="bi bi-trash"></i>
          {tr("common.delete")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={() => (showDeleteConfirm = false)}
      onkeydown={(e) => e.key === "Escape" && (showDeleteConfirm = false)}
    ></div>
  </div>
{/if}
