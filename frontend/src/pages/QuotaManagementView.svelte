<script>
  import { onMount } from "svelte";
  import api from "../lib/api.js";
  import { t } from "../i18n.js";

  // State
  let quotas = $state([]);
  let myQuota = $state(null);
  let loading = $state(true);
  let error = $state("");
  let successMessage = $state("");
  let searchQuery = $state("");
  let isAdmin = $state(false);

  // Edit modal state
  let showEditModal = $state(false);
  let editingUser = $state(null);
  let editQuotaValue = $state(0);
  let editQuotaUnit = $state("GB");
  let saving = $state(false);

  // Quota units
  const quotaUnits = [
    { value: "MB", multiplier: 1024 * 1024, label: "MB" },
    { value: "GB", multiplier: 1024 * 1024 * 1024, label: "GB" },
    { value: "TB", multiplier: 1024 * 1024 * 1024 * 1024, label: "TB" },
  ];

  // Format bytes to human readable
  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    if (bytes === -1) return $t("quota.unlimited");
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  // Calculate usage percentage
  function getUsagePercent(used, total) {
    if (total <= 0) return 0;
    return Math.min(100, Math.round((used / total) * 100));
  }

  // Get progress bar color based on usage
  function getProgressColor(percent) {
    if (percent >= 90) return "bg-error";
    if (percent >= 75) return "bg-warning";
    return "bg-success";
  }

  // Filter quotas by search
  let filteredQuotas = $derived(
    quotas.filter(
      (q) =>
        q.username?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        q.user_id?.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  // Load quota data
  async function loadQuotas() {
    loading = true;
    error = "";
    try {
      // Load my quota first
      myQuota = await api.quota.getMyQuota();

      // Try to load all quotas (admin only)
      try {
        quotas = await api.quota.listAllQuotas();
        isAdmin = true;
      } catch (e) {
        // Not admin, only show own quota
        isAdmin = false;
        quotas = myQuota ? [myQuota] : [];
      }
    } catch (e) {
      error = e.message || $t("quota.loadError");
    } finally {
      loading = false;
    }
  }

  // Open edit modal
  function openEditModal(quota) {
    editingUser = quota;
    // Convert bytes to appropriate unit
    const bytes = quota.quota_bytes || 0;
    if (bytes >= 1024 * 1024 * 1024 * 1024) {
      editQuotaValue = bytes / (1024 * 1024 * 1024 * 1024);
      editQuotaUnit = "TB";
    } else if (bytes >= 1024 * 1024 * 1024) {
      editQuotaValue = bytes / (1024 * 1024 * 1024);
      editQuotaUnit = "GB";
    } else {
      editQuotaValue = bytes / (1024 * 1024);
      editQuotaUnit = "MB";
    }
    showEditModal = true;
  }

  // Close edit modal
  function closeEditModal() {
    showEditModal = false;
    editingUser = null;
    editQuotaValue = 0;
    editQuotaUnit = "GB";
  }

  // Save quota
  async function saveQuota() {
    if (!editingUser) return;

    saving = true;
    error = "";
    try {
      const unit = quotaUnits.find((u) => u.value === editQuotaUnit);
      const quotaBytes = Math.round(editQuotaValue * unit.multiplier);

      await api.quota.setUserQuota(editingUser.user_id, quotaBytes);

      successMessage = $t("quota.updateSuccess");
      setTimeout(() => (successMessage = ""), 3000);

      closeEditModal();
      await loadQuotas();
    } catch (e) {
      error = e.message || $t("quota.updateError");
    } finally {
      saving = false;
    }
  }

  // Set unlimited quota
  async function setUnlimited() {
    if (!editingUser) return;

    saving = true;
    error = "";
    try {
      await api.quota.setUserQuota(editingUser.user_id, -1);

      successMessage = $t("quota.updateSuccess");
      setTimeout(() => (successMessage = ""), 3000);

      closeEditModal();
      await loadQuotas();
    } catch (e) {
      error = e.message || $t("quota.updateError");
    } finally {
      saving = false;
    }
  }

  onMount(() => {
    loadQuotas();
  });
</script>

<div class="p-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div class="flex items-center gap-3">
      <i class="bi bi-pie-chart text-2xl text-primary"></i>
      <h1 class="text-2xl font-bold">{$t("quota.title")}</h1>
    </div>
    <button
      class="btn btn-ghost btn-sm"
      onclick={loadQuotas}
      disabled={loading}
    >
      <i class="bi bi-arrow-clockwise" class:animate-spin={loading}></i>
      {$t("common.refresh")}
    </button>
  </div>

  <!-- Messages -->
  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-triangle"></i>
      <span>{error}</span>
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => (error = "")}
        aria-label="Close"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if successMessage}
    <div class="alert alert-success mb-4">
      <i class="bi bi-check-circle"></i>
      <span>{successMessage}</span>
    </div>
  {/if}

  <!-- My Quota Card -->
  {#if myQuota}
    <div class="card bg-base-200 shadow-lg mb-6">
      <div class="card-body">
        <h2 class="card-title">
          <i class="bi bi-person-circle text-primary"></i>
          {$t("quota.myStorage")}
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
          <div class="stat bg-base-100 rounded-lg">
            <div class="stat-figure text-primary">
              <i class="bi bi-hdd text-3xl"></i>
            </div>
            <div class="stat-title">{$t("quota.used")}</div>
            <div class="stat-value text-lg">
              {formatBytes(myQuota.used_bytes || 0)}
            </div>
          </div>
          <div class="stat bg-base-100 rounded-lg">
            <div class="stat-figure text-secondary">
              <i class="bi bi-database text-3xl"></i>
            </div>
            <div class="stat-title">{$t("quota.total")}</div>
            <div class="stat-value text-lg">
              {formatBytes(myQuota.quota_bytes || 0)}
            </div>
          </div>
          <div class="stat bg-base-100 rounded-lg">
            <div class="stat-figure text-accent">
              <i class="bi bi-box text-3xl"></i>
            </div>
            <div class="stat-title">{$t("quota.available")}</div>
            <div class="stat-value text-lg">
              {formatBytes(
                Math.max(
                  0,
                  (myQuota.quota_bytes || 0) - (myQuota.used_bytes || 0)
                )
              )}
            </div>
          </div>
        </div>
        <!-- Progress bar -->
        <div class="mt-4">
          <div class="flex justify-between text-sm mb-1">
            <span>{$t("quota.usageLabel")}</span>
            <span
              >{getUsagePercent(
                myQuota.used_bytes || 0,
                myQuota.quota_bytes || 0
              )}%</span
            >
          </div>
          <progress
            class="progress {getProgressColor(
              getUsagePercent(myQuota.used_bytes || 0, myQuota.quota_bytes || 0)
            )} w-full h-4"
            value={myQuota.used_bytes || 0}
            max={myQuota.quota_bytes || 1}
          ></progress>
        </div>
      </div>
    </div>
  {/if}

  <!-- Admin Section: All Users -->
  {#if isAdmin}
    <div class="card bg-base-200 shadow-lg">
      <div class="card-body">
        <div class="flex items-center justify-between mb-4">
          <h2 class="card-title">
            <i class="bi bi-people text-primary"></i>
            {$t("quota.allUsers")}
          </h2>
          <!-- Search -->
          <div class="form-control">
            <div class="input-group">
              <span class="bg-base-300">
                <i class="bi bi-search"></i>
              </span>
              <input
                type="text"
                placeholder={$t("quota.searchPlaceholder")}
                class="input input-bordered input-sm w-64"
                bind:value={searchQuery}
              />
            </div>
          </div>
        </div>

        {#if loading}
          <div class="flex justify-center py-8">
            <span class="loading loading-spinner loading-lg"></span>
          </div>
        {:else if filteredQuotas.length === 0}
          <div class="text-center py-8 text-base-content/60">
            <i class="bi bi-inbox text-4xl mb-2"></i>
            <p>{$t("quota.noUsers")}</p>
          </div>
        {:else}
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{$t("quota.user")}</th>
                  <th>{$t("quota.used")}</th>
                  <th>{$t("quota.total")}</th>
                  <th>{$t("quota.usage")}</th>
                  <th class="text-right">{$t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each filteredQuotas as quota}
                  {@const percent = getUsagePercent(
                    quota.used_bytes || 0,
                    quota.quota_bytes || 0
                  )}
                  <tr>
                    <td>
                      <div class="flex items-center gap-3">
                        <div class="avatar placeholder">
                          <div
                            class="bg-neutral text-neutral-content rounded-full w-10"
                          >
                            <span class="text-sm"
                              >{(quota.username || "U")
                                .charAt(0)
                                .toUpperCase()}</span
                            >
                          </div>
                        </div>
                        <div>
                          <div class="font-bold">
                            {quota.username || quota.user_id}
                          </div>
                          <div class="text-sm opacity-50">{quota.user_id}</div>
                        </div>
                      </div>
                    </td>
                    <td>{formatBytes(quota.used_bytes || 0)}</td>
                    <td>{formatBytes(quota.quota_bytes || 0)}</td>
                    <td>
                      <div class="flex items-center gap-2">
                        <progress
                          class="progress {getProgressColor(percent)} w-24 h-2"
                          value={quota.used_bytes || 0}
                          max={quota.quota_bytes || 1}
                        ></progress>
                        <span class="text-sm">{percent}%</span>
                      </div>
                    </td>
                    <td class="text-right">
                      <button
                        class="btn btn-ghost btn-sm"
                        onclick={() => openEditModal(quota)}
                        title={$t("quota.edit")}
                      >
                        <i class="bi bi-pencil"></i>
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>

          <!-- Summary stats -->
          <div class="divider"></div>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="text-center">
              <div class="text-2xl font-bold text-primary">
                {filteredQuotas.length}
              </div>
              <div class="text-sm opacity-70">{$t("quota.totalUsers")}</div>
            </div>
            <div class="text-center">
              <div class="text-2xl font-bold text-secondary">
                {formatBytes(
                  filteredQuotas.reduce(
                    (sum, q) => sum + (q.used_bytes || 0),
                    0
                  )
                )}
              </div>
              <div class="text-sm opacity-70">{$t("quota.totalUsed")}</div>
            </div>
            <div class="text-center">
              <div class="text-2xl font-bold text-accent">
                {formatBytes(
                  filteredQuotas.reduce(
                    (sum, q) => sum + (q.quota_bytes || 0),
                    0
                  )
                )}
              </div>
              <div class="text-sm opacity-70">{$t("quota.totalAllocated")}</div>
            </div>
            <div class="text-center">
              <div class="text-2xl font-bold text-warning">
                {filteredQuotas.filter(
                  (q) =>
                    getUsagePercent(q.used_bytes || 0, q.quota_bytes || 0) >= 90
                ).length}
              </div>
              <div class="text-sm opacity-70">{$t("quota.nearLimit")}</div>
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<!-- Edit Quota Modal -->
{#if showEditModal && editingUser}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <i class="bi bi-pencil-square text-primary"></i>
        {$t("quota.editTitle")}
      </h3>

      <div class="py-4">
        <div class="flex items-center gap-3 mb-4 p-3 bg-base-200 rounded-lg">
          <div class="avatar placeholder">
            <div class="bg-neutral text-neutral-content rounded-full w-12">
              <span
                >{(editingUser.username || "U").charAt(0).toUpperCase()}</span
              >
            </div>
          </div>
          <div>
            <div class="font-bold">
              {editingUser.username || editingUser.user_id}
            </div>
            <div class="text-sm opacity-50">
              {$t("quota.currentUsage")}: {formatBytes(
                editingUser.used_bytes || 0
              )}
            </div>
          </div>
        </div>

        <div class="form-control">
          <div class="label">
            <span class="label-text">{$t("quota.newQuota")}</span>
          </div>
          <div class="flex gap-2">
            <input
              type="number"
              class="input input-bordered flex-1"
              bind:value={editQuotaValue}
              min="0"
              step="0.1"
              aria-label={$t("quota.newQuota")}
            />
            <select
              class="select select-bordered w-24"
              bind:value={editQuotaUnit}
            >
              {#each quotaUnits as unit}
                <option value={unit.value}>{unit.label}</option>
              {/each}
            </select>
          </div>
        </div>

        <!-- Quick presets -->
        <div class="mt-4">
          <div class="label">
            <span class="label-text">{$t("quota.presets")}</span>
          </div>
          <div class="flex flex-wrap gap-2">
            <button
              class="btn btn-sm btn-outline"
              onclick={() => {
                editQuotaValue = 1;
                editQuotaUnit = "GB";
              }}
            >
              1 GB
            </button>
            <button
              class="btn btn-sm btn-outline"
              onclick={() => {
                editQuotaValue = 5;
                editQuotaUnit = "GB";
              }}
            >
              5 GB
            </button>
            <button
              class="btn btn-sm btn-outline"
              onclick={() => {
                editQuotaValue = 10;
                editQuotaUnit = "GB";
              }}
            >
              10 GB
            </button>
            <button
              class="btn btn-sm btn-outline"
              onclick={() => {
                editQuotaValue = 50;
                editQuotaUnit = "GB";
              }}
            >
              50 GB
            </button>
            <button
              class="btn btn-sm btn-outline"
              onclick={() => {
                editQuotaValue = 100;
                editQuotaUnit = "GB";
              }}
            >
              100 GB
            </button>
            <button
              class="btn btn-sm btn-outline"
              onclick={() => {
                editQuotaValue = 1;
                editQuotaUnit = "TB";
              }}
            >
              1 TB
            </button>
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={setUnlimited} disabled={saving}>
          <i class="bi bi-infinity"></i>
          {$t("quota.setUnlimited")}
        </button>
        <div class="flex-1"></div>
        <button
          class="btn btn-ghost"
          onclick={closeEditModal}
          disabled={saving}
        >
          {$t("common.cancel")}
        </button>
        <button class="btn btn-primary" onclick={saveQuota} disabled={saving}>
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            <i class="bi bi-check-lg"></i>
          {/if}
          {$t("common.save")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeEditModal}
      onkeydown={(e) => e.key === "Escape" && closeEditModal()}
    ></div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: -1;
  }
</style>
