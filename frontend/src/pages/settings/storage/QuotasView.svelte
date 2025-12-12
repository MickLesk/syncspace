<script>
  import { onMount } from "svelte";
  import { t } from "../../../i18n.js";
  import { rateLimiting, users } from "../../../lib/api.js";
  import { showToast } from "../../../stores/toast.js";

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
      showToast($t("quotas.saved"), "success");
      showQuotaModal = false;
      await loadData();
    } catch (e) {
      showToast($t("quotas.saveError") + ": " + e.message, "error");
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
      showToast($t("quotas.rateLimitSaved"), "success");
      showRateLimitModal = false;
      await loadData();
    } catch (e) {
      showToast($t("quotas.saveError") + ": " + e.message, "error");
    }
  }

  async function deleteRateLimit(id) {
    if (!confirm($t("quotas.confirmDeleteRateLimit"))) return;
    try {
      await rateLimiting.deleteRateLimit(id);
      showToast($t("quotas.rateLimitDeleted"), "success");
      await loadData();
    } catch (e) {
      showToast($t("quotas.deleteError") + ": " + e.message, "error");
    }
  }

  async function toggleRateLimit(limit) {
    try {
      await rateLimiting.updateRateLimit(limit.id, {
        is_enabled: !limit.is_enabled,
      });
      await loadData();
    } catch (e) {
      showToast($t("quotas.toggleError") + ": " + e.message, "error");
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
      <h1 class="text-2xl font-bold">{$t("quotas.title")}</h1>
      <p class="text-base-content/60">{$t("quotas.description")}</p>
    </div>
    <button class="btn btn-primary" onclick={() => openRateLimitModal()}>
      <i class="bi bi-plus-lg mr-2"></i>
      {$t("quotas.addRateLimit")}
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
        >{$t("common.retry")}</button
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
          <div class="stat-title">{$t("quotas.totalUsers")}</div>
          <div class="stat-value text-primary">{quotaStats.total_users}</div>
          <div class="stat-desc">
            {quotaStats.users_with_quotas}
            {$t("quotas.withQuotas")}
          </div>
        </div>

        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-secondary">
            <i class="bi bi-hdd text-3xl"></i>
          </div>
          <div class="stat-title">{$t("quotas.storageAllocated")}</div>
          <div class="stat-value text-secondary">
            {formatBytes(quotaStats.total_storage_allocated)}
          </div>
          <div class="stat-desc">
            {formatBytes(quotaStats.total_storage_used)}
            {$t("quotas.used")}
          </div>
        </div>

        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-warning">
            <i class="bi bi-exclamation-triangle text-3xl"></i>
          </div>
          <div class="stat-title">{$t("quotas.nearLimit")}</div>
          <div class="stat-value text-warning">
            {quotaStats.users_near_limit}
          </div>
          <div class="stat-desc">{$t("quotas.usersNearLimit")}</div>
        </div>

        <div class="stat bg-base-200 rounded-lg">
          <div class="stat-figure text-error">
            <i class="bi bi-x-circle text-3xl"></i>
          </div>
          <div class="stat-title">{$t("quotas.overLimit")}</div>
          <div class="stat-value text-error">{quotaStats.users_over_limit}</div>
          <div class="stat-desc">{$t("quotas.usersOverLimit")}</div>
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
        {$t("quotas.storageQuotas")}
      </button>
      <button
        class="tab"
        class:tab-active={activeTab === "bandwidth"}
        onclick={() => (activeTab = "bandwidth")}
      >
        <i class="bi bi-speedometer mr-2"></i>
        {$t("quotas.bandwidthQuotas")}
      </button>
      <button
        class="tab"
        class:tab-active={activeTab === "rateLimits"}
        onclick={() => (activeTab = "rateLimits")}
      >
        <i class="bi bi-shield-check mr-2"></i>
        {$t("quotas.rateLimits")}
      </button>
    </div>

    <!-- Storage Quotas Tab -->
    {#if activeTab === "storage"}
      <div
        class="card bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all"
      >
        <div class="card-body">
          <h2 class="card-title">{$t("quotas.userStorageQuotas")}</h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{$t("quotas.user")}</th>
                  <th>{$t("quotas.limit")}</th>
                  <th>{$t("quotas.used")}</th>
                  <th>{$t("quotas.usage")}</th>
                  <th>{$t("quotas.status")}</th>
                  <th>{$t("quotas.actions")}</th>
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
                          >{$t("quotas.unlimited")}</span
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
                          >{$t("quotas.unlimited")}</span
                        >
                      {:else if percent >= 100}
                        <span class="badge badge-error"
                          >{$t("quotas.exceeded")}</span
                        >
                      {:else if percent >= 80}
                        <span class="badge badge-warning"
                          >{$t("quotas.warning")}</span
                        >
                      {:else}
                        <span class="badge badge-success"
                          >{$t("quotas.ok")}</span
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
          <h2 class="card-title">{$t("quotas.userBandwidthQuotas")}</h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{$t("quotas.user")}</th>
                  <th>{$t("quotas.dailyUpload")}</th>
                  <th>{$t("quotas.dailyDownload")}</th>
                  <th>{$t("quotas.monthlyUpload")}</th>
                  <th>{$t("quotas.monthlyDownload")}</th>
                  <th>{$t("quotas.status")}</th>
                  <th>{$t("quotas.actions")}</th>
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
                          >{$t("quotas.unlimited")}</span
                        >
                      {:else}
                        <span class="badge badge-info"
                          >{$t("quotas.limited")}</span
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
            <div class="stat-title">{$t("quotas.totalRules")}</div>
            <div class="stat-value text-lg">{rateLimitStats.total_rules}</div>
          </div>
          <div class="stat bg-base-200 rounded-lg">
            <div class="stat-title">{$t("quotas.activeRules")}</div>
            <div class="stat-value text-lg text-success">
              {rateLimitStats.active_rules}
            </div>
          </div>
          <div class="stat bg-base-200 rounded-lg">
            <div class="stat-title">{$t("quotas.requestsToday")}</div>
            <div class="stat-value text-lg">
              {rateLimitStats.requests_today}
            </div>
          </div>
          <div class="stat bg-base-200 rounded-lg">
            <div class="stat-title">{$t("quotas.rateLimitedToday")}</div>
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
          <h2 class="card-title">{$t("quotas.rateLimitRules")}</h2>
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>{$t("quotas.target")}</th>
                  <th>{$t("quotas.endpoint")}</th>
                  <th>{$t("quotas.perMinute")}</th>
                  <th>{$t("quotas.perHour")}</th>
                  <th>{$t("quotas.perDay")}</th>
                  <th>{$t("quotas.burst")}</th>
                  <th>{$t("quotas.status")}</th>
                  <th>{$t("quotas.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each rateLimits as limit}
                  <tr>
                    <td>
                      {#if limit.role_name}
                        <span class="badge badge-primary"
                          >{$t("quotas.role")}: {limit.role_name}</span
                        >
                      {:else if limit.user_id}
                        <span class="badge badge-secondary"
                          >{$t("quotas.user")}: {getUserName(
                            limit.user_id
                          )}</span
                        >
                      {:else}
                        <span class="badge badge-ghost"
                          >{$t("quotas.global")}</span
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
        {$t("quotas.editQuotasFor")}
        {selectedUser?.username || selectedUser?.display_name}
      </h3>

      <!-- Storage Quota -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer">
          <span class="label-text font-semibold"
            >{$t("quotas.unlimitedStorage")}</span
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
            <span class="label-text">{$t("quotas.storageLimit")} (GB)</span>
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
            <span class="label-text">{$t("quotas.warningThreshold")} (%)</span>
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

      <div class="divider">{$t("quotas.bandwidthLimits")}</div>

      <!-- Bandwidth Quota -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer">
          <span class="label-text font-semibold"
            >{$t("quotas.unlimitedBandwidth")}</span
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
              <span class="label-text">{$t("quotas.dailyUpload")} (GB)</span>
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
              placeholder={$t("quotas.noLimit")}
              aria-label="Daily upload limit in GB"
            />
          </div>
          <div class="form-control">
            <div class="label">
              <span class="label-text">{$t("quotas.dailyDownload")} (GB)</span>
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
              placeholder={$t("quotas.noLimit")}
              aria-label="Daily download limit in GB"
            />
          </div>
          <div class="form-control">
            <div class="label">
              <span class="label-text">{$t("quotas.monthlyUpload")} (GB)</span>
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
              placeholder={$t("quotas.noLimit")}
              aria-label="Monthly upload limit in GB"
            />
          </div>
          <div class="form-control">
            <div class="label">
              <span class="label-text">{$t("quotas.monthlyDownload")} (GB)</span
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
              placeholder={$t("quotas.noLimit")}
              aria-label="Monthly download limit in GB"
            />
          </div>
        </div>
      {/if}

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={() => (showQuotaModal = false)}
          >{$t("common.cancel")}</button
        >
        <button class="btn btn-primary" onclick={saveQuota}
          >{$t("common.save")}</button
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
          ? $t("quotas.editRateLimit")
          : $t("quotas.addRateLimit")}
      </h3>

      <div class="form-control mb-4">
        <div class="label">
          <span class="label-text">{$t("quotas.targetType")}</span>
        </div>
        <select
          class="select select-bordered"
          bind:value={rateLimitForm.role_name}
          onchange={() => (rateLimitForm.user_id = null)}
          aria-label="Target type"
        >
          <option value={null}>{$t("quotas.selectRole")}</option>
          <option value="admin">Admin</option>
          <option value="user">User</option>
          <option value="guest">Guest</option>
          <option value="viewer">Viewer</option>
        </select>
      </div>

      <div class="form-control mb-4">
        <div class="label">
          <span class="label-text">{$t("quotas.endpointPattern")}</span>
        </div>
        <input
          type="text"
          class="input input-bordered font-mono"
          bind:value={rateLimitForm.endpoint_pattern}
          placeholder="* or /api/files/*"
          aria-label="Endpoint pattern"
        />
        <div class="label">
          <span class="label-text-alt">{$t("quotas.endpointHelp")}</span>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="form-control">
          <div class="label">
            <span class="label-text">{$t("quotas.perMinute")}</span>
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
            <span class="label-text">{$t("quotas.perHour")}</span>
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
            <span class="label-text">{$t("quotas.perDay")}</span>
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
            <span class="label-text">{$t("quotas.burstLimit")}</span>
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
          <span class="label-text">{$t("quotas.enabled")}</span>
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
          >{$t("common.cancel")}</button
        >
        <button class="btn btn-primary" onclick={saveRateLimit}
          >{$t("common.save")}</button
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
