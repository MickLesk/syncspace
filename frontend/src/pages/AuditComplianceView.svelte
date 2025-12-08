<script>
  import { onMount } from "svelte";
  import api from "../lib/api.js";
  import { success, error } from "../stores/toast.js";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let activeTab = $state("logs"); // logs, reports, policies, archives
  let logs = $state([]);
  let reports = $state([]);
  let policies = $state([]);
  let archives = $state([]);
  let templates = $state([]);
  let stats = $state(null);
  let loading = $state(true);

  // Filters
  let filters = $state({
    user_id: "",
    action: "",
    action_category: "",
    severity: "",
    start_date: "",
    end_date: "",
    search: "",
    compliance_only: false,
  });

  // Pagination
  let limit = $state(50);
  let offset = $state(0);
  let hasMore = $state(true);

  // Modals
  let showGenerateReportModal = $state(false);
  let showCreatePolicyModal = $state(false);
  let showLogDetailModal = $state(false);
  let selectedLog = $state(null);

  // Report form
  let newReport = $state({
    report_type: "custom",
    report_name: "",
    description: "",
    start_date: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000)
      .toISOString()
      .split("T")[0],
    end_date: new Date().toISOString().split("T")[0],
    file_format: "json",
    template_id: null,
  });

  // Policy form
  let newPolicy = $state({
    name: "",
    description: "",
    resource_type: "audit_logs",
    retention_days: 365,
    auto_delete: false,
    archive_before_delete: true,
    notify_before_delete: true,
    notify_days_before: 7,
  });

  onMount(() => {
    loadData();
  });

  async function loadData() {
    loading = true;
    try {
      const [statsRes, templatesRes] = await Promise.all([
        api.audit.getStats(),
        api.audit.listTemplates(),
      ]);
      stats = statsRes.data || statsRes;
      templates = templatesRes.data || templatesRes || [];

      if (activeTab === "logs") await loadLogs();
      else if (activeTab === "reports") await loadReports();
      else if (activeTab === "policies") await loadPolicies();
      else if (activeTab === "archives") await loadArchives();
    } catch (err) {
      console.error("Failed to load audit data:", err);
    } finally {
      loading = false;
    }
  }

  async function loadLogs() {
    try {
      const res = await api.audit.listLogs({ ...filters, limit, offset });
      const data = res.data || res || [];
      logs = offset === 0 ? data : [...logs, ...data];
      hasMore = data.length === limit;
    } catch (err) {
      console.error("Failed to load logs:", err);
    }
  }

  async function loadReports() {
    try {
      const res = await api.audit.listReports();
      reports = res.data || res || [];
    } catch (err) {
      console.error("Failed to load reports:", err);
    }
  }

  async function loadPolicies() {
    try {
      const res = await api.audit.listPolicies();
      policies = res.data || res || [];
    } catch (err) {
      console.error("Failed to load policies:", err);
    }
  }

  async function loadArchives() {
    try {
      const res = await api.audit.listArchives();
      archives = res.data || res || [];
    } catch (err) {
      console.error("Failed to load archives:", err);
    }
  }

  function changeTab(tab) {
    activeTab = tab;
    if (tab === "logs") loadLogs();
    else if (tab === "reports") loadReports();
    else if (tab === "policies") loadPolicies();
    else if (tab === "archives") loadArchives();
  }

  async function applyFilters() {
    offset = 0;
    await loadLogs();
  }

  async function loadMore() {
    offset += limit;
    await loadLogs();
  }

  async function generateReport() {
    try {
      await api.audit.generateReport(newReport);
      success(tr("audit.reportGenerated"));
      showGenerateReportModal = false;
      resetNewReport();
      loadReports();
    } catch (err) {
      error(tr("audit.reportGenerationFailed"));
    }
  }

  async function downloadReport(report) {
    try {
      const data = await api.audit.downloadReport(report.id);
      const blob = new Blob([JSON.stringify(data.data || data, null, 2)], {
        type: "application/json",
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `${report.report_name}_${report.start_date}_${report.end_date}.json`;
      a.click();
      URL.revokeObjectURL(url);
      success(tr("audit.reportDownloaded"));
    } catch (err) {
      error(tr("audit.downloadFailed"));
    }
  }

  async function deleteReport(report) {
    if (!confirm(tr("audit.confirmDeleteReport"))) return;
    try {
      await api.audit.deleteReport(report.id);
      success(tr("audit.reportDeleted"));
      loadReports();
    } catch (err) {
      error(tr("audit.deleteReportFailed"));
    }
  }

  async function createPolicy() {
    try {
      await api.audit.createPolicy(newPolicy);
      success(tr("audit.policyCreated"));
      showCreatePolicyModal = false;
      resetNewPolicy();
      loadPolicies();
    } catch (err) {
      error(tr("audit.policyCreateFailed"));
    }
  }

  async function applyPolicy(policy) {
    try {
      const result = await api.audit.applyPolicy(policy.id);
      const data = result.data || result;
      success(tr("audit.policyApplied", data.records_deleted));
      loadPolicies();
    } catch (err) {
      error(tr("audit.policyApplyFailed"));
    }
  }

  async function togglePolicy(policy) {
    try {
      await api.audit.updatePolicy(policy.id, { is_active: !policy.is_active });
      loadPolicies();
    } catch (err) {
      error(tr("audit.togglePolicyFailed"));
    }
  }

  async function deletePolicy(policy) {
    if (!confirm(tr("audit.confirmDeletePolicy"))) return;
    try {
      await api.audit.deletePolicy(policy.id);
      success(tr("audit.policyDeleted"));
      loadPolicies();
    } catch (err) {
      error(tr("audit.deletePolicyFailed"));
    }
  }

  async function createArchive() {
    try {
      await api.audit.createArchive();
      success(tr("audit.archiveCreated"));
      loadArchives();
    } catch (err) {
      error(tr("audit.archiveCreateFailed"));
    }
  }

  async function applyAllPolicies() {
    try {
      const results = await api.audit.applyAllPolicies();
      const data = results.data || results;
      const total = data.reduce((sum, r) => sum + r.records_deleted, 0);
      success(tr("audit.allPoliciesApplied", total));
      loadPolicies();
    } catch (err) {
      error(tr("audit.applyAllFailed"));
    }
  }

  function resetNewReport() {
    newReport = {
      report_type: "custom",
      report_name: "",
      description: "",
      start_date: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000)
        .toISOString()
        .split("T")[0],
      end_date: new Date().toISOString().split("T")[0],
      file_format: "json",
      template_id: null,
    };
  }

  function resetNewPolicy() {
    newPolicy = {
      name: "",
      description: "",
      resource_type: "audit_logs",
      retention_days: 365,
      auto_delete: false,
      archive_before_delete: true,
      notify_before_delete: true,
      notify_days_before: 7,
    };
  }

  function formatDate(dateString) {
    if (!dateString) return "-";
    return new Date(dateString).toLocaleString();
  }

  function getSeverityColor(severity) {
    switch (severity) {
      case "critical":
        return "bg-red-600 text-white";
      case "error":
        return "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300";
      case "warning":
        return "bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300";
      case "info":
        return "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300";
      default:
        return "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300";
    }
  }

  function getCategoryIcon(category) {
    switch (category) {
      case "auth":
        return "bi-shield-lock";
      case "file":
        return "bi-folder";
      case "admin":
        return "bi-gear";
      case "sharing":
        return "bi-share";
      case "security":
        return "bi-shield-exclamation";
      default:
        return "bi-journal";
    }
  }

  function useTemplate(template) {
    newReport.report_type = template.report_type;
    newReport.report_name = template.name;
    newReport.description = template.description || "";
    newReport.template_id = template.id;
  }
</script>

<div class="h-full flex flex-col bg-gray-50 dark:bg-gray-900">
  <!-- Header -->
  <div
    class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4"
  >
    <div class="flex justify-between items-center">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-indigo-100 dark:bg-indigo-900/30 rounded-lg">
          <i
            class="bi bi-clipboard-data text-2xl text-indigo-600 dark:text-indigo-400"
          ></i>
        </div>
        <div>
          <h1 class="text-xl font-bold text-gray-900 dark:text-white">
            {tr("audit.title")}
          </h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("audit.subtitle")}
          </p>
        </div>
      </div>
      <div class="flex gap-2">
        {#if activeTab === "reports"}
          <button
            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium flex items-center gap-2 transition-colors"
            onclick={() => (showGenerateReportModal = true)}
          >
            <i class="bi bi-file-earmark-plus" aria-hidden="true"></i>
            {tr("audit.generateReport")}
          </button>
        {:else if activeTab === "policies"}
          <button
            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium flex items-center gap-2 transition-colors"
            onclick={() => (showCreatePolicyModal = true)}
          >
            <i class="bi bi-plus-lg" aria-hidden="true"></i>
            {tr("audit.createPolicy")}
          </button>
        {:else if activeTab === "archives"}
          <button
            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium flex items-center gap-2 transition-colors"
            onclick={createArchive}
          >
            <i class="bi bi-archive" aria-hidden="true"></i>
            {tr("audit.createArchive")}
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Stats Cards -->
  {#if stats}
    <div class="grid grid-cols-2 lg:grid-cols-4 xl:grid-cols-8 gap-3 px-6 py-4">
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-indigo-600 dark:text-indigo-400">
          {stats.total_logs?.toLocaleString() || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.totalLogs")}
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-green-600 dark:text-green-400">
          {stats.logs_today || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.today")}
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-green-600 dark:text-green-400">
          {stats.logs_this_week || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.thisWeek")}
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-purple-600 dark:text-purple-400">
          {stats.logs_this_month || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.thisMonth")}
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-red-600 dark:text-red-400">
          {stats.critical_events || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.critical")}
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-orange-600 dark:text-orange-400">
          {stats.error_events || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.errors")}
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-amber-600 dark:text-amber-400">
          {stats.warning_events || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.warnings")}
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700"
      >
        <div class="text-2xl font-bold text-teal-600 dark:text-teal-400">
          {stats.compliance_events || 0}
        </div>
        <div class="text-xs text-gray-500 dark:text-gray-400">
          {tr("audit.compliance")}
        </div>
      </div>
    </div>
  {/if}

  <!-- Tabs -->
  <div class="flex border-b border-gray-200 dark:border-gray-700 px-6">
    {#each [{ id: "logs", icon: "bi-journal-text", label: tr("audit.auditLogs") }, { id: "reports", icon: "bi-file-earmark-text", label: tr("audit.complianceReports") }, { id: "policies", icon: "bi-clock-history", label: tr("audit.retentionPolicies") }, { id: "archives", icon: "bi-archive", label: tr("audit.archives") }] as tab}
      <button
        class="px-4 py-3 text-sm font-medium border-b-2 transition-colors flex items-center gap-2
          {activeTab === tab.id
          ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
          : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
        onclick={() => changeTab(tab.id)}
      >
        <i class="bi {tab.icon}"></i>
        {tab.label}
      </button>
    {/each}
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-auto p-6">
    {#if loading}
      <div class="flex justify-center items-center h-64">
        <div
          class="w-12 h-12 border-4 border-indigo-200 dark:border-indigo-900 border-t-indigo-600 dark:border-t-indigo-400 rounded-full animate-spin"
        ></div>
      </div>
    {:else if activeTab === "logs"}
      <!-- Filters -->
      <div
        class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4 mb-4"
      >
        <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-3">
          <input
            type="text"
            placeholder={tr("audit.searchLogs")}
            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            bind:value={filters.search}
          />
          <select
            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            bind:value={filters.action_category}
          >
            <option value="">{tr("audit.allCategories")}</option>
            <option value="auth">{tr("audit.categoryAuth")}</option>
            <option value="file">{tr("audit.categoryFile")}</option>
            <option value="admin">{tr("audit.categoryAdmin")}</option>
            <option value="sharing">{tr("audit.categorySharing")}</option>
            <option value="security">{tr("audit.categorySecurity")}</option>
          </select>
          <select
            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            bind:value={filters.severity}
          >
            <option value="">{tr("audit.allSeverities")}</option>
            <option value="critical">{tr("audit.severityCritical")}</option>
            <option value="error">{tr("audit.severityError")}</option>
            <option value="warning">{tr("audit.severityWarning")}</option>
            <option value="info">{tr("audit.severityInfo")}</option>
          </select>
          <input
            type="date"
            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            bind:value={filters.start_date}
          />
          <input
            type="date"
            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            bind:value={filters.end_date}
          />
          <button
            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm font-medium"
            onclick={applyFilters}
          >
            <i class="bi bi-funnel mr-1" aria-hidden="true"></i>
            {tr("audit.applyFilters")}
          </button>
        </div>
        <label class="flex items-center gap-2 mt-3 cursor-pointer">
          <input
            type="checkbox"
            class="w-4 h-4 rounded"
            bind:checked={filters.compliance_only}
          />
          <span class="text-sm text-gray-700 dark:text-gray-200"
            >{tr("audit.complianceOnlyFilter")}</span
          >
        </label>
      </div>

      <!-- Logs Table -->
      <div
        class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
      >
        <table class="w-full text-sm">
          <thead class="bg-gray-50 dark:bg-gray-700/50">
            <tr>
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.timestamp")}</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.user")}</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.action")}</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.resource")}</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.severity")}</th
              >
              <th
                class="px-4 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("common.actions")}</th
              >
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
            {#each logs as log}
              <tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
                <td
                  class="px-4 py-3 text-gray-600 dark:text-gray-300 whitespace-nowrap"
                >
                  {formatDate(log.created_at)}
                </td>
                <td class="px-4 py-3">
                  <span class="font-medium text-gray-900 dark:text-white"
                    >{log.username || log.user_id}</span
                  >
                </td>
                <td class="px-4 py-3">
                  <div class="flex items-center gap-2">
                    <i
                      class="bi {getCategoryIcon(
                        log.action_category
                      )} text-gray-400"
                    ></i>
                    <span class="text-gray-900 dark:text-white"
                      >{log.action}</span
                    >
                  </div>
                </td>
                <td
                  class="px-4 py-3 text-gray-600 dark:text-gray-300 max-w-xs truncate"
                >
                  {log.resource_name || log.resource_id || "-"}
                </td>
                <td class="px-4 py-3">
                  <span
                    class="px-2 py-1 text-xs font-medium rounded-full {getSeverityColor(
                      log.severity
                    )}"
                  >
                    {log.severity || "info"}
                  </span>
                </td>
                <td class="px-4 py-3 text-right">
                  <button
                    class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg text-gray-500 dark:text-gray-400"
                    onclick={() => {
                      selectedLog = log;
                      showLogDetailModal = true;
                    }}
                    title={tr("audit.viewDetails")}
                  >
                    <i class="bi bi-eye" aria-hidden="true"></i>
                  </button>
                </td>
              </tr>
            {:else}
              <tr>
                <td
                  colspan="6"
                  class="px-4 py-12 text-center text-gray-500 dark:text-gray-400"
                >
                  <i class="bi bi-journal text-4xl mb-2" aria-hidden="true"></i>
                  <p>{tr("audit.noLogs")}</p>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
        {#if hasMore && logs.length > 0}
          <div class="p-4 border-t border-gray-200 dark:border-gray-700">
            <button
              class="w-full py-2 text-sm font-medium text-indigo-600 dark:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-900/20 rounded-lg"
              onclick={loadMore}
            >
              {tr("audit.loadMore")}
            </button>
          </div>
        {/if}
      </div>
    {:else if activeTab === "reports"}
      <!-- Reports Grid -->
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {#each reports as report}
          <div
            class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4"
          >
            <div class="flex justify-between items-start mb-3">
              <div class="flex items-center gap-2">
                <div class="p-2 bg-indigo-100 dark:bg-indigo-900/30 rounded-lg">
                  <i
                    class="bi bi-file-earmark-text text-indigo-600 dark:text-indigo-400"
                  ></i>
                </div>
                <div>
                  <h3 class="font-semibold text-gray-900 dark:text-white">
                    {report.report_name}
                  </h3>
                  <p class="text-xs text-gray-500 dark:text-gray-400 uppercase">
                    {report.report_type}
                  </p>
                </div>
              </div>
              <span
                class="px-2 py-1 text-xs font-medium rounded-full {report.status ===
                'completed'
                  ? 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300'
                  : 'bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300'}"
              >
                {report.status}
              </span>
            </div>
            {#if report.description}
              <p
                class="text-sm text-gray-600 dark:text-gray-400 mb-3 line-clamp-2"
              >
                {report.description}
              </p>
            {/if}
            <div
              class="text-xs text-gray-500 dark:text-gray-400 mb-3 space-y-1"
            >
              <div>
                <i class="bi bi-calendar mr-1" aria-hidden="true"
                ></i>{report.start_date} - {report.end_date}
              </div>
              <div>
                <i class="bi bi-file-text mr-1" aria-hidden="true"
                ></i>{report.total_records?.toLocaleString()}
                {tr("audit.records")}
              </div>
              <div>
                <i class="bi bi-download mr-1" aria-hidden="true"
                ></i>{report.download_count || 0}
                {tr("audit.downloads")}
              </div>
            </div>
            <div class="flex gap-2">
              {#if report.status === "completed"}
                <button
                  class="flex-1 px-3 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm font-medium"
                  onclick={() => downloadReport(report)}
                >
                  <i class="bi bi-download mr-1" aria-hidden="true"></i>
                  {tr("common.download")}
                </button>
              {/if}
              <button
                class="px-3 py-2 hover:bg-red-50 dark:hover:bg-red-900/20 text-red-500 rounded-lg"
                onclick={() => deleteReport(report)}
                aria-label="Delete"
              >
                <i class="bi bi-trash" aria-hidden="true"></i>
              </button>
            </div>
          </div>
        {:else}
          <div
            class="col-span-full text-center py-12 text-gray-500 dark:text-gray-400"
          >
            <i class="bi bi-file-earmark-x text-4xl mb-2" aria-hidden="true"
            ></i>
            <p>{tr("audit.noReports")}</p>
          </div>
        {/each}
      </div>
    {:else if activeTab === "policies"}
      <!-- Policies Actions -->
      <div class="flex justify-end mb-4">
        <button
          class="px-4 py-2 bg-amber-600 hover:bg-amber-700 text-white rounded-lg font-medium flex items-center gap-2"
          onclick={applyAllPolicies}
        >
          <i class="bi bi-play-fill" aria-hidden="true"></i>
          {tr("audit.applyAllPolicies")}
        </button>
      </div>

      <!-- Policies List -->
      <div class="space-y-4">
        {#each policies as policy}
          <div
            class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4"
          >
            <div class="flex justify-between items-start">
              <div class="flex items-start gap-3">
                <div class="p-2 bg-amber-100 dark:bg-amber-900/30 rounded-lg">
                  <i
                    class="bi bi-clock-history text-xl text-amber-600 dark:text-amber-400"
                  ></i>
                </div>
                <div>
                  <div class="flex items-center gap-2">
                    <h3 class="font-semibold text-gray-900 dark:text-white">
                      {policy.name}
                    </h3>
                    {#if !policy.is_active}
                      <span
                        class="px-2 py-0.5 text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-500 rounded-full"
                      >
                        {tr("audit.inactive")}
                      </span>
                    {/if}
                  </div>
                  {#if policy.description}
                    <p class="text-sm text-gray-500 dark:text-gray-400">
                      {policy.description}
                    </p>
                  {/if}
                  <div class="flex flex-wrap gap-3 mt-2 text-sm">
                    <span class="text-gray-600 dark:text-gray-300">
                      <i class="bi bi-database mr-1" aria-hidden="true"
                      ></i>{policy.resource_type}
                    </span>
                    <span class="text-gray-600 dark:text-gray-300">
                      <i class="bi bi-calendar-check mr-1" aria-hidden="true"
                      ></i>{policy.retention_days}
                      {tr("audit.days")}
                    </span>
                    {#if policy.auto_delete}
                      <span class="text-red-600 dark:text-red-400">
                        <i class="bi bi-trash mr-1" aria-hidden="true"></i>{tr(
                          "audit.autoDelete"
                        )}
                      </span>
                    {/if}
                    {#if policy.archive_before_delete}
                      <span class="text-green-600 dark:text-green-400">
                        <i class="bi bi-archive mr-1" aria-hidden="true"
                        ></i>{tr("audit.archiveFirst")}
                      </span>
                    {/if}
                  </div>
                  {#if policy.last_applied_at}
                    <p class="text-xs text-gray-400 mt-2">
                      {tr("audit.lastApplied")}: {formatDate(
                        policy.last_applied_at
                      )}
                    </p>
                  {/if}
                </div>
              </div>
              <div class="flex items-center gap-2">
                <button
                  class="p-2 rounded-lg transition-colors {policy.is_active
                    ? 'bg-green-100 dark:bg-green-900/30 text-green-600'
                    : 'bg-gray-100 dark:bg-gray-700 text-gray-400'}"
                  onclick={() => togglePolicy(policy)}
                  title={policy.is_active
                    ? tr("audit.disable")
                    : tr("audit.enable")}
                >
                  <i
                    class="bi {policy.is_active
                      ? 'bi-toggle-on'
                      : 'bi-toggle-off'} text-xl"
                  ></i>
                </button>
                <button
                  class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg text-gray-500"
                  onclick={() => applyPolicy(policy)}
                  title={tr("audit.applyNow")}
                >
                  <i class="bi bi-play-fill" aria-hidden="true"></i>
                </button>
                <button
                  class="p-2 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg text-red-500"
                  onclick={() => deletePolicy(policy)}
                  aria-label="Delete"
                >
                  <i class="bi bi-trash" aria-hidden="true"></i>
                </button>
              </div>
            </div>
          </div>
        {:else}
          <div class="text-center py-12 text-gray-500 dark:text-gray-400">
            <i class="bi bi-clock text-4xl mb-2" aria-hidden="true"></i>
            <p>{tr("audit.noPolicies")}</p>
          </div>
        {/each}
      </div>
    {:else if activeTab === "archives"}
      <!-- Archives List -->
      <div
        class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
      >
        <table class="w-full">
          <thead class="bg-gray-50 dark:bg-gray-700/50">
            <tr>
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.archiveName")}</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.dateRange")}</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.records")}</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase"
                >{tr("audit.createdAt")}</th
              >
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
            {#each archives as archive}
              <tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
                <td class="px-4 py-3">
                  <div class="flex items-center gap-2">
                    <i class="bi bi-archive text-indigo-500" aria-hidden="true"
                    ></i>
                    <span class="font-medium text-gray-900 dark:text-white"
                      >{archive.archive_name}</span
                    >
                    {#if archive.is_encrypted}
                      <i
                        class="bi bi-lock-fill text-amber-500"
                        title="Encrypted"
                      ></i>
                    {/if}
                  </div>
                </td>
                <td class="px-4 py-3 text-sm text-gray-600 dark:text-gray-300">
                  {archive.start_date?.split("T")[0]} - {archive.end_date?.split(
                    "T"
                  )[0]}
                </td>
                <td class="px-4 py-3 text-sm text-gray-600 dark:text-gray-300">
                  {archive.record_count?.toLocaleString()}
                </td>
                <td class="px-4 py-3 text-sm text-gray-600 dark:text-gray-300">
                  {formatDate(archive.created_at)}
                </td>
              </tr>
            {:else}
              <tr>
                <td
                  colspan="4"
                  class="px-4 py-12 text-center text-gray-500 dark:text-gray-400"
                >
                  <i class="bi bi-archive text-4xl mb-2" aria-hidden="true"></i>
                  <p>{tr("audit.noArchives")}</p>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>

<!-- Generate Report Modal -->
{#if showGenerateReportModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-xl max-w-lg w-full mx-4 border border-gray-200 dark:border-gray-700 max-h-[90vh] overflow-y-auto"
    >
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h3 class="font-bold text-lg text-gray-900 dark:text-white">
          {tr("audit.generateReport")}
        </h3>
      </div>
      <div class="p-6 space-y-4">
        <!-- Templates -->
        {#if templates.length > 0}
          <div>
            <div
              class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-2"
            >
              {tr("audit.useTemplate")}
            </div>
            <div class="grid grid-cols-2 gap-2">
              {#each templates as template}
                <button
                  class="p-2 text-left text-sm border rounded-lg transition-colors
                    {newReport.template_id === template.id
                    ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-900/20'
                    : 'border-gray-300 dark:border-gray-600 hover:border-indigo-300'}"
                  onclick={() => useTemplate(template)}
                >
                  <div class="font-medium text-gray-900 dark:text-white">
                    {template.name}
                  </div>
                  <div class="text-xs text-gray-500">
                    {template.report_type}
                  </div>
                </button>
              {/each}
            </div>
          </div>
        {/if}

        <div>
          <div
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
          >
            {tr("audit.reportName")} *
          </div>
          <input
            type="text"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
            bind:value={newReport.report_name}
          />
        </div>
        <div>
          <label
            for="report-type-select"
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
            >{tr("audit.reportType")}</label
          >
          <select
            id="report-type-select"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
            bind:value={newReport.report_type}
          >
            <option value="custom">{tr("audit.typeCustom")}</option>
            <option value="gdpr">{tr("audit.typeGDPR")}</option>
            <option value="security">{tr("audit.typeSecurity")}</option>
            <option value="activity">{tr("audit.typeActivity")}</option>
          </select>
        </div>
        <div>
          <label
            for="report-description-input"
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
            >{tr("audit.description")}</label
          >
          <textarea
            id="report-description-input"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
            rows="2"
            bind:value={newReport.description}
          ></textarea>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label
              for="report-start-date"
              class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
              >{tr("audit.startDate")}</label
            >
            <input
              id="report-start-date"
              type="date"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
              bind:value={newReport.start_date}
            />
          </div>
          <div>
            <label
              for="report-end-date"
              class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
              >{tr("audit.endDate")}</label
            >
            <input
              id="report-end-date"
              type="date"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
              bind:value={newReport.end_date}
            />
          </div>
        </div>
        <div>
          <label
            for="report-file-format"
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
            >{tr("audit.fileFormat")}</label
          >
          <select
            id="report-file-format"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
            bind:value={newReport.file_format}
          >
            <option value="json">JSON</option>
            <option value="csv">CSV</option>
          </select>
        </div>
      </div>
      <div
        class="flex justify-end gap-2 px-6 py-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700"
          onclick={() => {
            showGenerateReportModal = false;
            resetNewReport();
          }}
        >
          {tr("common.cancel")}
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50"
          onclick={generateReport}
          disabled={!newReport.report_name.trim()}
        >
          {tr("audit.generate")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Create Policy Modal -->
{#if showCreatePolicyModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-xl max-w-lg w-full mx-4 border border-gray-200 dark:border-gray-700"
    >
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h3 class="font-bold text-lg text-gray-900 dark:text-white">
          {tr("audit.createPolicy")}
        </h3>
      </div>
      <div class="p-6 space-y-4">
        <div>
          <label
            for="policy-name-input"
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
            >{tr("audit.policyName")} *</label
          >
          <input
            id="policy-name-input"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
            bind:value={newPolicy.name}
          />
        </div>
        <div>
          <label
            for="policy-description-input"
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
            >{tr("audit.description")}</label
          >
          <input
            id="policy-description-input"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
            bind:value={newPolicy.description}
          />
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label
              for="policy-resource-type"
              class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
              >{tr("audit.resourceType")}</label
            >
            <select
              id="policy-resource-type"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
              bind:value={newPolicy.resource_type}
            >
              <option value="audit_logs">{tr("audit.resourceAuditLogs")}</option
              >
              <option value="trash">{tr("audit.resourceTrash")}</option>
              <option value="sessions">{tr("audit.resourceSessions")}</option>
              <option value="file_versions"
                >{tr("audit.resourceVersions")}</option
              >
            </select>
          </div>
          <div>
            <label
              for="policy-retention-days"
              class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
              >{tr("audit.retentionDays")}</label
            >
            <input
              id="policy-retention-days"
              type="number"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
              bind:value={newPolicy.retention_days}
              min="1"
            />
          </div>
        </div>
        <div class="space-y-2">
          <label class="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              class="w-4 h-4 rounded"
              bind:checked={newPolicy.auto_delete}
            />
            <span class="text-sm text-gray-700 dark:text-gray-200"
              >{tr("audit.autoDeleteEnabled")}</span
            >
          </label>
          <label class="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              class="w-4 h-4 rounded"
              bind:checked={newPolicy.archive_before_delete}
            />
            <span class="text-sm text-gray-700 dark:text-gray-200"
              >{tr("audit.archiveBeforeDelete")}</span
            >
          </label>
          <label class="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              class="w-4 h-4 rounded"
              bind:checked={newPolicy.notify_before_delete}
            />
            <span class="text-sm text-gray-700 dark:text-gray-200"
              >{tr("audit.notifyBeforeDelete")}</span
            >
          </label>
        </div>
      </div>
      <div
        class="flex justify-end gap-2 px-6 py-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700"
          onclick={() => {
            showCreatePolicyModal = false;
            resetNewPolicy();
          }}
        >
          {tr("common.cancel")}
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50"
          onclick={createPolicy}
          disabled={!newPolicy.name.trim()}
        >
          {tr("audit.create")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Log Detail Modal -->
{#if showLogDetailModal && selectedLog}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-xl max-w-2xl w-full mx-4 border border-gray-200 dark:border-gray-700 max-h-[90vh] overflow-y-auto"
    >
      <div
        class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center"
      >
        <h3 class="font-bold text-lg text-gray-900 dark:text-white">
          {tr("audit.logDetails")}
        </h3>
        <button
          aria-label="Close"
          class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
          onclick={() => {
            showLogDetailModal = false;
            selectedLog = null;
          }}
        >
          <i class="bi bi-x-lg" aria-hidden="true"></i>
        </button>
      </div>
      <div class="p-6 space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <span
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
              >{tr("audit.timestamp")}</span
            >
            <p class="text-gray-900 dark:text-white">
              {formatDate(selectedLog.created_at)}
            </p>
          </div>
          <div>
            <span
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
              >{tr("audit.severity")}</span
            >
            <span
              class="px-2 py-1 text-xs font-medium rounded-full {getSeverityColor(
                selectedLog.severity
              )}"
            >
              {selectedLog.severity || "info"}
            </span>
          </div>
          <div>
            <span
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
              >{tr("audit.user")}</span
            >
            <p class="text-gray-900 dark:text-white">
              {selectedLog.username || selectedLog.user_id}
            </p>
          </div>
          <div>
            <span
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
              >{tr("audit.category")}</span
            >
            <p class="text-gray-900 dark:text-white capitalize">
              {selectedLog.action_category || "general"}
            </p>
          </div>
          <div class="col-span-2">
            <span
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
              >{tr("audit.action")}</span
            >
            <p class="text-gray-900 dark:text-white">{selectedLog.action}</p>
          </div>
          <div class="col-span-2">
            <span
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
              >{tr("audit.resource")}</span
            >
            <p class="text-gray-900 dark:text-white">
              {selectedLog.resource_name || selectedLog.resource_id || "-"}
            </p>
          </div>
          {#if selectedLog.ip_address}
            <div>
              <span
                class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
                >{tr("audit.ipAddress")}</span
              >
              <p class="text-gray-900 dark:text-white font-mono">
                {selectedLog.ip_address}
              </p>
            </div>
          {/if}
          {#if selectedLog.user_agent}
            <div class="col-span-2">
              <span
                class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
                >{tr("audit.userAgent")}</span
              >
              <p class="text-gray-900 dark:text-white text-sm truncate">
                {selectedLog.user_agent}
              </p>
            </div>
          {/if}
        </div>
        {#if selectedLog.metadata}
          <div>
            <span
              class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
              >{tr("audit.metadata")}</span
            >
            <pre
              class="mt-1 p-3 bg-gray-100 dark:bg-gray-800 rounded-lg text-sm overflow-x-auto">{JSON.stringify(
                JSON.parse(selectedLog.metadata),
                null,
                2
              )}</pre>
          </div>
        {/if}
        {#if selectedLog.old_value || selectedLog.new_value}
          <div class="grid grid-cols-2 gap-4">
            {#if selectedLog.old_value}
              <div>
                <span
                  class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
                  >{tr("audit.oldValue")}</span
                >
                <pre
                  class="mt-1 p-3 bg-red-50 dark:bg-red-900/20 rounded-lg text-sm overflow-x-auto">{JSON.stringify(
                    JSON.parse(selectedLog.old_value),
                    null,
                    2
                  )}</pre>
              </div>
            {/if}
            {#if selectedLog.new_value}
              <div>
                <span
                  class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase block"
                  >{tr("audit.newValue")}</span
                >
                <pre
                  class="mt-1 p-3 bg-green-50 dark:bg-green-900/20 rounded-lg text-sm overflow-x-auto">{JSON.stringify(
                    JSON.parse(selectedLog.new_value),
                    null,
                    2
                  )}</pre>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
