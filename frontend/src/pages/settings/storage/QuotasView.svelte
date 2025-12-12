<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIModal from "../../../components/ui/UIModal.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import { rateLimiting, users } from "../../../lib/api.js";
  import { showToast } from "../../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // State
  let activeTab = $state("storage");
  let loading = $state(true);
  let error = $state(null);

  // Data
  let storageQuotas = $state([]);
  let bandwidthQuotas = $state([]);
  let rateLimits = $state([]);
  let quotaStats = $state(null);
  let rateLimitStats = $state(null);
  let usersList = $state([]);

  // Modals
  let showQuotaModal = $state(false);
  let showRateLimitModal = $state(false);
  let selectedUser = $state(null);
  let selectedRateLimit = $state(null);

  // Form data
  let quotaForm = $state({
    storage_limit_bytes: 10737418240, // 10GB
    warning_threshold_percent: 80,
    is_unlimited: false,
  });

  let bandwidthForm = $state({
    daily_upload_limit_bytes: null,
    daily_download_limit_bytes: null,
    monthly_upload_limit_bytes: null,
    monthly_download_limit_bytes: null,
    is_unlimited: true,
  });

  let rateLimitForm = $state({
    user_id: null,
    role_name: null,
    endpoint_pattern: "*",
    requests_per_minute: 60,
    requests_per_hour: 1000,
    requests_per_day: 10000,
    burst_limit: 10,
    is_enabled: true,
  });

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    error = null;
    try {
      const [
        storageRes,
        bandwidthRes,
        rateLimitRes,
        statsRes,
        rateLimitStatsRes,
        usersRes,
      ] = await Promise.all([
        rateLimiting.listStorageQuotas(),
        rateLimiting.listBandwidthQuotas(),
        rateLimiting.listRateLimits(),
        rateLimiting.getQuotaStats(),
        rateLimiting.getRateLimitStats(),
        users.list(),
      ]);

      storageQuotas = storageRes.quotas || [];
      bandwidthQuotas = bandwidthRes.quotas || [];
      rateLimits = rateLimitRes.rateLimits || [];
      quotaStats = statsRes;
      rateLimitStats = rateLimitStatsRes;
      usersList = usersRes.users || usersRes || [];
    } catch (e) {
      console.error("Failed to load quota data:", e);
      error = e.message;
    } finally {
      loading = false;
    }
  }

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function bytesToGB(bytes) {
    return bytes / (1024 * 1024 * 1024);
  }

  function gbToBytes(gb) {
    return gb * 1024 * 1024 * 1024;
  }

  function getUserName(userId) {
    const user = usersList.find((u) => u.id === userId);
    return user?.username || user?.display_name || userId;
  }

  async function openQuotaModal(user) {
    selectedUser = user;
    try {
      const usage = await rateLimiting.getStorageUsage(user.id || user.user_id);
      quotaForm = {
        storage_limit_bytes: usage.storage_limit_bytes || 10737418240,
        warning_threshold_percent: 80,
        is_unlimited: usage.is_unlimited || false,
      };

      const bandwidth = await rateLimiting.getBandwidthQuota(
        user.id || user.user_id
      );
      bandwidthForm = {
        daily_upload_limit_bytes: bandwidth.daily_upload_limit_bytes,
        daily_download_limit_bytes: bandwidth.daily_download_limit_bytes,
        monthly_upload_limit_bytes: bandwidth.monthly_upload_limit_bytes,
        monthly_download_limit_bytes: bandwidth.monthly_download_limit_bytes,
        is_unlimited: bandwidth.is_unlimited || true,
      };
    } catch (e) {
      console.error("Failed to load user quota:", e);
    }
    showQuotaModal = true;
  }

  async function saveQuota() {
    if (!selectedUser) return;
    try {
      await rateLimiting.updateStorageQuota(
        selectedUser.id || selectedUser.user_id,
        quotaForm
      );
      await rateLimiting.updateBandwidthQuota(
        selectedUser.id || selectedUser.user_id,
        bandwidthForm
      );
      showToast(tr("quotas.saved"), "success");
      showQuotaModal = false;
      await loadData();
    } catch (e) {
      showToast(tr("quotas.saveError") + ": " + e.message, "error");
    }
  }

  function openRateLimitModal(limit = null) {
    selectedRateLimit = limit;
    if (limit) {
      rateLimitForm = { ...limit };
    } else {
      rateLimitForm = {
        user_id: null,
        role_name: null,
        endpoint_pattern: "*",
        requests_per_minute: 60,
        requests_per_hour: 1000,
        requests_per_day: 10000,
        burst_limit: 10,
        is_enabled: true,
      };
    }
    showRateLimitModal = true;
  }

  async function saveRateLimit() {
    try {
      if (selectedRateLimit) {
        await rateLimiting.updateRateLimit(selectedRateLimit.id, rateLimitForm);
      } else {
        await rateLimiting.createRateLimit(rateLimitForm);
      }
      showToast(tr("quotas.rateLimitSaved"), "success");
      showRateLimitModal = false;
      await loadData();
    } catch (e) {
      showToast(tr("quotas.saveError") + ": " + e.message, "error");
    }
  }

  async function deleteRateLimit(id) {
    if (!confirm(tr("quotas.confirmDeleteRateLimit"))) return;
    try {
      await rateLimiting.deleteRateLimit(id);
      showToast(tr("quotas.rateLimitDeleted"), "success");
      await loadData();
    } catch (e) {
      showToast(tr("quotas.deleteError") + ": " + e.message, "error");
    }
  }

  async function toggleRateLimit(limit) {
    try {
      await rateLimiting.updateRateLimit(limit.id, {
        is_enabled: !limit.is_enabled,
      });
      await loadData();
    } catch (e) {
      showToast(tr("quotas.toggleError") + ": " + e.message, "error");
    }
  }

  // Helper to calculate quota usage percent
  function calcPercent(quota) {
    if (!quota || quota.is_unlimited) return 0;
    const used = quota.storage_used_bytes || 0;
    const limit = quota.storage_limit_bytes || 10737418240;
    return (used / limit) * 100;
  }

  function getQuotaForUser(userId) {
    return storageQuotas.find((q) => q.user_id === userId);
  }
</script>

<div class="p-6">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div>
      <h1 class="text-2xl font-bold">{tr("quotas.title")}</h1>
      <p class="text-base-content/60">{tr("quotas.description")}</p>
    </div>
    <button class="btn btn-primary" onclick={() => openRateLimitModal()}>
      <i class="bi bi-plus-lg mr-2"></i>
      {tr("quotas.addRateLimit")}
    </button>
  </div>

  {#if loading}
    <div class="flex justify-center items-center h-64">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-triangle"></i>
      <span>{error}</span>
      <button class="btn btn-sm btn-ghost" onclick={loadData}
        >{tr("common.retry")}</button
      >
    </div>
  {:else}
    <!-- Stats Cards -->
    {#if quotaStats}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-primary">
            <i class="bi bi-people text-3xl"></i>
          </div>
          <div class="stat-title">{tr("quotas.totalUsers")}</div>
          <div class="stat-value text-primary">{quotaStats.total_users}</div>
          <div class="stat-desc">
            {quotaStats.users_with_quotas}
            {tr("quotas.withQuotas")}
          </div>
        </div>

        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-secondary">
            <i class="bi bi-hdd text-3xl"></i>
          </div>
          <div class="stat-title">{tr("quotas.storageAllocated")}</div>
          <div class="stat-value text-secondary">
            {formatBytes(quotaStats.total_storage_allocated)}
          </div>
          <div class="stat-desc">
            {formatBytes(quotaStats.total_storage_used)}
            {tr("quotas.used")}
          </div>
        </div>

        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-warning">
            <i class="bi bi-exclamation-triangle text-3xl"></i>
          </div>
          <div class="stat-title">{tr("quotas.nearLimit")}</div>
          <div class="stat-value text-warning">
            {quotaStats.users_near_limit}
          </div>
          <div class="stat-desc">{tr("quotas.usersNearLimit")}</div>
        </div>

        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-error">
            <i class="bi bi-x-circle text-3xl"></i>
          </div>
          <div class="stat-title">{tr("quotas.overLimit")}</div>
          <div class="stat-value text-error">{quotaStats.users_over_limit}</div>
          <div class="stat-desc">{tr("quotas.usersOverLimit")}</div>
        </div>
      </div>
    {/if}

    <!-- Tabs -->
    <div class="tabs tabs-boxed mb-6">
      <button
        class="tab"
        class:tab-active={activeTab === "storage"}
        onclick={() => (activeTab = "storage")}
      >
        <i class="bi bi-hdd mr-2"></i>
        {tr("quotas.storageQuotas")}
      </button>
      <button
        class="tab"
        class:tab-active={activeTab === "bandwidth"}
        onclick={() => (activeTab = "bandwidth")}
      >
        <i class="bi bi-speedometer mr-2"></i>
        {tr("quotas.bandwidthQuotas")}
      </button>
      <button
        class="tab"
        class:tab-active={activeTab === "rateLimits"}
        onclick={() => (activeTab = "rateLimits")}
      >
        <i class="bi bi-shield-check mr-2"></i>
        {tr("quotas.rateLimits")}
      </button>
    </div>

    <!-- Storage Quotas Tab -->
    {#if activeTab === "storage"}
      <div
        class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
      >
        <div class="card-body">
          <h2 class="card-title">{tr("quotas.userStorageQuotas")}</h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{tr("quotas.user")}</th>
                  <th>{tr("quotas.limit")}</th>
                  <th>{tr("quotas.used")}</th>
                  <th>{tr("quotas.usage")}</th>
                  <th>{tr("quotas.status")}</th>
                  <th>{tr("quotas.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each usersList as user}
                  {@const quota = getQuotaForUser(user.id)}
                  {@const percent = calcPercent(quota)}
                  <tr>
                    <td class="flex items-center gap-2">
                      <div class="avatar placeholder">
                        <div
                          class="bg-neutral text-neutral-content rounded-full w-8"
                        >
                          <span
                            >{(user.username ||
                              user.display_name ||
                              "?")[0].toUpperCase()}</span
                          >
                        </div>
                      </div>
                      <span>{user.username || user.display_name}</span>
                    </td>
                    <td>
                      {#if quota?.is_unlimited}
                        <span class="badge badge-info"
                          >{tr("quotas.unlimited")}</span
                        >
                      {:else}
                        {formatBytes(quota?.storage_limit_bytes || 10737418240)}
                      {/if}
                    </td>
                    <td>{formatBytes(quota?.storage_used_bytes || 0)}</td>
                    <td>
                      <div class="flex items-center gap-2">
                        <progress
                          class="progress w-24"
                          class:progress-success={percent < 60}
                          class:progress-warning={percent >= 60 && percent < 80}
                          class:progress-error={percent >= 80}
                          value={percent}
                          max="100"
                        ></progress>
                        <span class="text-sm">{percent.toFixed(0)}%</span>
                      </div>
                    </td>
                    <td>
                      {#if quota?.is_unlimited}
                        <span class="badge badge-ghost"
                          >{tr("quotas.unlimited")}</span
                        >
                      {:else if percent >= 100}
                        <span class="badge badge-error"
                          >{tr("quotas.exceeded")}</span
                        >
                      {:else if percent >= 80}
                        <span class="badge badge-warning"
                          >{tr("quotas.warning")}</span
                        >
                      {:else}
                        <span class="badge badge-success"
                          >{tr("quotas.ok")}</span
                        >
                      {/if}
                    </td>
                    <td>
                      <button
                        class="btn btn-sm btn-ghost"
                        onclick={() => openQuotaModal(user)}
                        aria-label="Edit quota"
                      >
                        <i class="bi bi-pencil"></i>
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}

    <!-- Bandwidth Quotas Tab -->
    {#if activeTab === "bandwidth"}
      <div
        class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
      >
        <div class="card-body">
          <h2 class="card-title">{tr("quotas.userBandwidthQuotas")}</h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{tr("quotas.user")}</th>
                  <th>{tr("quotas.dailyUpload")}</th>
                  <th>{tr("quotas.dailyDownload")}</th>
                  <th>{tr("quotas.monthlyUpload")}</th>
                  <th>{tr("quotas.monthlyDownload")}</th>
                  <th>{tr("quotas.status")}</th>
                  <th>{tr("quotas.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each usersList as user}
                  {@const quota = bandwidthQuotas.find(
                    (q) => q.user_id === user.id
                  )}
                  <tr>
                    <td class="flex items-center gap-2">
                      <div class="avatar placeholder">
                        <div
                          class="bg-neutral text-neutral-content rounded-full w-8"
                        >
                          <span
                            >{(user.username ||
                              user.display_name ||
                              "?")[0].toUpperCase()}</span
                          >
                        </div>
                      </div>
                      <span>{user.username || user.display_name}</span>
                    </td>
                    <td>
                      {#if quota?.is_unlimited || !quota?.daily_upload_limit_bytes}
                        <span class="text-base-content/50">∞</span>
                      {:else}
                        {formatBytes(quota.daily_upload_limit_bytes)}
                      {/if}
                    </td>
                    <td>
                      {#if quota?.is_unlimited || !quota?.daily_download_limit_bytes}
                        <span class="text-base-content/50">∞</span>
                      {:else}
                        {formatBytes(quota.daily_download_limit_bytes)}
                      {/if}
                    </td>
                    <td>
                      {#if quota?.is_unlimited || !quota?.monthly_upload_limit_bytes}
                        <span class="text-base-content/50">∞</span>
                      {:else}
                        {formatBytes(quota.monthly_upload_limit_bytes)}
                      {/if}
                    </td>
                    <td>
                      {#if quota?.is_unlimited || !quota?.monthly_download_limit_bytes}
                        <span class="text-base-content/50">∞</span>
                      {:else}
                        {formatBytes(quota.monthly_download_limit_bytes)}
                      {/if}
                    </td>
                    <td>
                      {#if quota?.is_unlimited}
                        <span class="badge badge-ghost"
                          >{tr("quotas.unlimited")}</span
                        >
                      {:else}
                        <span class="badge badge-info"
                          >{tr("quotas.limited")}</span
                        >
                      {/if}
                    </td>
                    <td>
                      <button
                        class="btn btn-sm btn-ghost"
                        onclick={() => openQuotaModal(user)}
                        aria-label="Edit quota"
                      >
                        <i class="bi bi-pencil"></i>
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}

    <!-- Rate Limits Tab -->
    {#if activeTab === "rateLimits"}
      <!-- Rate Limit Stats -->
      {#if rateLimitStats}
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
          <div class="stat bg-base-200 rounded-lg">
            <div class="stat-title">{tr("quotas.totalRules")}</div>
            <div class="stat-value text-lg">{rateLimitStats.total_rules}</div>
          </div>
          <div class="stat bg-base-200 rounded-lg">
            <div class="stat-title">{tr("quotas.activeRules")}</div>
            <div class="stat-value text-lg text-success">
              {rateLimitStats.active_rules}
            </div>
          </div>
          <div class="stat bg-base-200 rounded-lg">
            <div class="stat-title">{tr("quotas.requestsToday")}</div>
            <div class="stat-value text-lg">
              {rateLimitStats.requests_today}
            </div>
          </div>
          <div class="stat bg-base-200 rounded-lg">
            <div class="stat-title">{tr("quotas.rateLimitedToday")}</div>
            <div class="stat-value text-lg text-warning">
              {rateLimitStats.requests_rate_limited_today}
            </div>
          </div>
        </div>
      {/if}

      <div
        class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
      >
        <div class="card-body">
          <h2 class="card-title">{tr("quotas.rateLimitRules")}</h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{tr("quotas.target")}</th>
                  <th>{tr("quotas.endpoint")}</th>
                  <th>{tr("quotas.perMinute")}</th>
                  <th>{tr("quotas.perHour")}</th>
                  <th>{tr("quotas.perDay")}</th>
                  <th>{tr("quotas.burst")}</th>
                  <th>{tr("quotas.status")}</th>
                  <th>{tr("quotas.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each rateLimits as limit}
                  <tr>
                    <td>
                      {#if limit.role_name}
                        <span class="badge badge-primary"
                          >{tr("quotas.role")}: {limit.role_name}</span
                        >
                      {:else if limit.user_id}
                        <span class="badge badge-secondary"
                          >{tr("quotas.user")}: {getUserName(
                            limit.user_id
                          )}</span
                        >
                      {:else}
                        <span class="badge badge-ghost"
                          >{tr("quotas.global")}</span
                        >
                      {/if}
                    </td>
                    <td class="font-mono text-sm">{limit.endpoint_pattern}</td>
                    <td>{limit.requests_per_minute}</td>
                    <td>{limit.requests_per_hour}</td>
                    <td>{limit.requests_per_day}</td>
                    <td>{limit.burst_limit}</td>
                    <td>
                      <input
                        type="checkbox"
                        class="toggle toggle-success toggle-sm"
                        checked={limit.is_enabled}
                        onchange={() => toggleRateLimit(limit)}
                      />
                    </td>
                    <td>
                      <div class="flex gap-1">
                        <button
                          class="btn btn-sm btn-ghost"
                          onclick={() => openRateLimitModal(limit)}
                          aria-label="Edit rate limit"
                        >
                          <i class="bi bi-pencil"></i>
                        </button>
                        <button
                          class="btn btn-sm btn-ghost text-error"
                          onclick={() => deleteRateLimit(limit.id)}
                          aria-label="Delete rate limit"
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
        </div>
      </div>
    {/if}
  {/if}
</div>

<!-- Quota Edit Modal -->
{#if showQuotaModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-lg mb-4">
        {tr("quotas.editQuotasFor")}
        {selectedUser?.username || selectedUser?.display_name}
      </h3>

      <!-- Storage Quota -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer">
          <span class="label-text font-semibold"
            >{tr("quotas.unlimitedStorage")}</span
          >
          <input
            type="checkbox"
            class="toggle"
            bind:checked={quotaForm.is_unlimited}
          />
        </label>
      </div>

      {#if !quotaForm.is_unlimited}
        <div class="form-control mb-4">
          <div class="label">
            <span class="label-text">{tr("quotas.storageLimit")} (GB)</span>
          </div>
          <input
            type="number"
            class="input input-bordered"
            value={bytesToGB(quotaForm.storage_limit_bytes)}
            onchange={(e) =>
              (quotaForm.storage_limit_bytes = gbToBytes(
                parseFloat(e.target.value) || 10
              ))}
            min="1"
            aria-label="Storage limit in GB"
          />
        </div>

        <div class="form-control mb-4">
          <div class="label">
            <span class="label-text">{tr("quotas.warningThreshold")} (%)</span>
          </div>
          <input
            type="number"
            class="input input-bordered"
            bind:value={quotaForm.warning_threshold_percent}
            min="50"
            max="99"
            aria-label="Warning threshold percentage"
          />
        </div>
      {/if}

      <div class="divider">{tr("quotas.bandwidthLimits")}</div>

      <!-- Bandwidth Quota -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer">
          <span class="label-text font-semibold"
            >{tr("quotas.unlimitedBandwidth")}</span
          >
          <input
            type="checkbox"
            class="toggle"
            bind:checked={bandwidthForm.is_unlimited}
          />
        </label>
      </div>

      {#if !bandwidthForm.is_unlimited}
        <div class="grid grid-cols-2 gap-4">
          <div class="form-control">
            <div class="label">
              <span class="label-text">{tr("quotas.dailyUpload")} (GB)</span>
            </div>
            <input
              type="number"
              class="input input-bordered"
              value={bandwidthForm.daily_upload_limit_bytes
                ? bytesToGB(bandwidthForm.daily_upload_limit_bytes)
                : ""}
              onchange={(e) =>
                (bandwidthForm.daily_upload_limit_bytes = e.target.value
                  ? gbToBytes(parseFloat(e.target.value))
                  : null)}
              placeholder={tr("quotas.noLimit")}
              aria-label="Daily upload limit in GB"
            />
          </div>
          <div class="form-control">
            <div class="label">
              <span class="label-text">{tr("quotas.dailyDownload")} (GB)</span>
            </div>
            <input
              type="number"
              class="input input-bordered"
              value={bandwidthForm.daily_download_limit_bytes
                ? bytesToGB(bandwidthForm.daily_download_limit_bytes)
                : ""}
              onchange={(e) =>
                (bandwidthForm.daily_download_limit_bytes = e.target.value
                  ? gbToBytes(parseFloat(e.target.value))
                  : null)}
              placeholder={tr("quotas.noLimit")}
              aria-label="Daily download limit in GB"
            />
          </div>
          <div class="form-control">
            <div class="label">
              <span class="label-text">{tr("quotas.monthlyUpload")} (GB)</span>
            </div>
            <input
              type="number"
              class="input input-bordered"
              value={bandwidthForm.monthly_upload_limit_bytes
                ? bytesToGB(bandwidthForm.monthly_upload_limit_bytes)
                : ""}
              onchange={(e) =>
                (bandwidthForm.monthly_upload_limit_bytes = e.target.value
                  ? gbToBytes(parseFloat(e.target.value))
                  : null)}
              placeholder={tr("quotas.noLimit")}
              aria-label="Monthly upload limit in GB"
            />
          </div>
          <div class="form-control">
            <div class="label">
              <span class="label-text">{tr("quotas.monthlyDownload")} (GB)</span
              >
            </div>
            <input
              type="number"
              class="input input-bordered"
              value={bandwidthForm.monthly_download_limit_bytes
                ? bytesToGB(bandwidthForm.monthly_download_limit_bytes)
                : ""}
              onchange={(e) =>
                (bandwidthForm.monthly_download_limit_bytes = e.target.value
                  ? gbToBytes(parseFloat(e.target.value))
                  : null)}
              placeholder={tr("quotas.noLimit")}
              aria-label="Monthly download limit in GB"
            />
          </div>
        </div>
      {/if}

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={() => (showQuotaModal = false)}
          >{tr("common.cancel")}</button
        >
        <button class="btn btn-primary" onclick={saveQuota}
          >{tr("common.save")}</button
        >
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={() => (showQuotaModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showQuotaModal = false)}
    ></div>
  </div>
{/if}

<!-- Rate Limit Edit Modal -->
{#if showRateLimitModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        {selectedRateLimit
          ? tr("quotas.editRateLimit")
          : tr("quotas.addRateLimit")}
      </h3>

      <div class="form-control mb-4">
        <div class="label">
          <span class="label-text">{tr("quotas.targetType")}</span>
        </div>
        <select
          class="select select-bordered"
          bind:value={rateLimitForm.role_name}
          onchange={() => (rateLimitForm.user_id = null)}
          aria-label="Target type"
        >
          <option value={null}>{tr("quotas.selectRole")}</option>
          <option value="admin">Admin</option>
          <option value="user">User</option>
          <option value="guest">Guest</option>
          <option value="viewer">Viewer</option>
        </select>
      </div>

      <div class="form-control mb-4">
        <div class="label">
          <span class="label-text">{tr("quotas.endpointPattern")}</span>
        </div>
        <input
          type="text"
          class="input input-bordered font-mono"
          bind:value={rateLimitForm.endpoint_pattern}
          placeholder="* or /api/files/*"
          aria-label="Endpoint pattern"
        />
        <div class="label">
          <span class="label-text-alt">{tr("quotas.endpointHelp")}</span>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("quotas.perMinute")}</span>
          </div>
          <input
            type="number"
            class="input input-bordered"
            bind:value={rateLimitForm.requests_per_minute}
            min="1"
            aria-label="Requests per minute"
          />
        </div>
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("quotas.perHour")}</span>
          </div>
          <input
            type="number"
            class="input input-bordered"
            bind:value={rateLimitForm.requests_per_hour}
            min="1"
            aria-label="Requests per hour"
          />
        </div>
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("quotas.perDay")}</span>
          </div>
          <input
            type="number"
            class="input input-bordered"
            bind:value={rateLimitForm.requests_per_day}
            min="1"
            aria-label="Requests per day"
          />
        </div>
        <div class="form-control">
          <div class="label">
            <span class="label-text">{tr("quotas.burstLimit")}</span>
          </div>
          <input
            type="number"
            class="input input-bordered"
            bind:value={rateLimitForm.burst_limit}
            min="1"
            aria-label="Burst limit"
          />
        </div>
      </div>

      <div class="form-control mt-4">
        <label class="label cursor-pointer">
          <span class="label-text">{tr("quotas.enabled")}</span>
          <input
            type="checkbox"
            class="toggle toggle-success"
            bind:checked={rateLimitForm.is_enabled}
          />
        </label>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => (showRateLimitModal = false)}
          >{tr("common.cancel")}</button
        >
        <button class="btn btn-primary" onclick={saveRateLimit}
          >{tr("common.save")}</button
        >
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={() => (showRateLimitModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showRateLimitModal = false)}
    ></div>
  </div>
{/if}
