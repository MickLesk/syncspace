<script>
  import { t } from "../../i18n.js";
  import { currentLang } from "../../stores/ui.js";
  import { onMount } from "svelte";
  import { backup as backupApi, backupSchedules } from "../../lib/api.js";
  import { success as toastSuccess, error as toastError } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let autoBackupEnabled = $state(false);
  let backupFrequency = $state("daily");
  let lastBackup = $state(null);
  let backupHistory = $state([]);
  let schedules = $state([]);
  let loading = $state(true);
  let saving = $state(false);
  let creatingBackup = $state(false);
  let error = $state(null);
  let success = $state(null);

  async function loadBackupSettings() {
    loading = true;
    try {
      // Load backup list
      const backupsResponse = await backupApi.list();
      backupHistory = backupsResponse?.data || [];
      
      // Load schedules
      try {
        const schedulesResponse = await backupApi.listSchedules();
        schedules = Array.isArray(schedulesResponse) ? schedulesResponse : schedulesResponse?.data || [];
        
        // Find active schedule
        const activeSchedule = schedules.find(s => s.is_active);
        if (activeSchedule) {
          autoBackupEnabled = true;
          backupFrequency = activeSchedule.frequency || "daily";
        }
      } catch (schedErr) {
        console.warn("Failed to load schedules:", schedErr);
      }
      
      // Get last backup
      if (backupHistory.length > 0) {
        lastBackup = backupHistory[0]?.created_at || null;
      }
    } catch (err) {
      console.error("Failed to load backup settings:", err);
      // Use defaults - backup system might not be fully set up
      backupHistory = [];
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    saving = true;
    error = null;
    try {
      // TODO: Update schedule settings when backend supports it
      toastSuccess(tr("settings.backup.saved"));
    } catch (err) {
      toastError(err.message || tr("settings.backup.save_error"));
    } finally {
      saving = false;
    }
  }

  async function createBackupNow() {
    creatingBackup = true;
    error = null;
    try {
      await backupApi.create({
        backup_type: "full",
        include_versions: true,
        include_database: true,
        description: "Manual backup from settings"
      });
      toastSuccess(tr("settings.backup.created"));
      lastBackup = new Date().toISOString();
      await loadBackupSettings();
    } catch (err) {
      toastError(err.message || tr("settings.backup.create_error"));
    } finally {
      creatingBackup = false;
    }
  }

  async function restoreBackup(backupId) {
    if (!confirm(tr("settings.backup.restore_confirm"))) return;

    try {
      await backupApi.restore({ backup_id: backupId });
      toastSuccess(tr("settings.backup.restored"));
    } catch (err) {
      toastError(err.message || tr("settings.backup.restore_error"));
    }
  }

  async function deleteBackup(backupId) {
    if (!confirm(tr("settings.backup.delete_confirm"))) return;

    try {
      await backupApi.delete(backupId);
      backupHistory = backupHistory.filter((b) => b.id !== backupId);
      toastSuccess(tr("settings.backup.deleted"));
    } catch (err) {
      toastError(err.message || tr("settings.backup.delete_error"));
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    return new Date(dateStr).toLocaleString();
  }

  function formatSize(bytes) {
    if (!bytes) return "-";
    const units = ["B", "KB", "MB", "GB"];
    let size = bytes;
    let unitIndex = 0;
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  }

  onMount(() => {
    loadBackupSettings();
  });
</script>

<div class="backup-settings">
  {#if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-circle"></i>
      <span>{error}</span>
      <button
        class="alert-close"
        onclick={() => (error = null)}
        aria-label="Close"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if success}
    <div class="alert alert-success">
      <i class="bi bi-check-circle"></i>
      <span>{success}</span>
    </div>
  {/if}

  {#if loading}
    <div class="loading-container">
      <div class="loading-spinner"></div>
      <p>{tr("common.loading")}</p>
    </div>
  {:else}
    <!-- Automatische Backups -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon blue">
          <i class="bi bi-clock-history"></i>
        </div>
        <div>
          <h3>{tr("settings.backup.auto_backup")}</h3>
          <p class="card-subtitle">{tr("settings.backup.auto_backup_desc")}</p>
        </div>
      </div>

      <div class="card-body">
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label"
              >{tr("settings.backup.enable_auto")}</span
            >
            <span class="setting-hint"
              >{tr("settings.backup.enable_auto_hint")}</span
            >
          </div>
          <label class="toggle-switch">
            <input
              type="checkbox"
              bind:checked={autoBackupEnabled}
              onchange={saveSettings}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        {#if autoBackupEnabled}
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label"
                >{tr("settings.backup.frequency")}</span
              >
              <span class="setting-hint"
                >{tr("settings.backup.frequency_hint")}</span
              >
            </div>
            <select
              class="select-input"
              bind:value={backupFrequency}
              onchange={saveSettings}
            >
              <option value="hourly">{tr("settings.backup.hourly")}</option>
              <option value="daily">{tr("settings.backup.daily")}</option>
              <option value="weekly">{tr("settings.backup.weekly")}</option>
              <option value="monthly">{tr("settings.backup.monthly")}</option>
            </select>
          </div>
        {/if}

        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label"
              >{tr("settings.backup.last_backup")}</span
            >
            <span class="setting-value"
              >{lastBackup
                ? formatDate(lastBackup)
                : tr("settings.backup.never")}</span
            >
          </div>
        </div>
      </div>
    </div>

    <!-- Manuelles Backup -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon green">
          <i class="bi bi-download"></i>
        </div>
        <div>
          <h3>{tr("settings.backup.manual_backup")}</h3>
          <p class="card-subtitle">
            {tr("settings.backup.manual_backup_desc")}
          </p>
        </div>
      </div>

      <div class="card-body">
        <button
          class="btn btn-primary"
          onclick={createBackupNow}
          disabled={creatingBackup}
        >
          {#if creatingBackup}
            <span class="btn-spinner"></span>
            {tr("settings.backup.creating")}
          {:else}
            <i class="bi bi-plus-circle"></i>
            {tr("settings.backup.create_now")}
          {/if}
        </button>
      </div>
    </div>

    <!-- Backup-Verlauf -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon amber">
          <i class="bi bi-archive"></i>
        </div>
        <div>
          <h3>{tr("settings.backup.history")}</h3>
          <p class="card-subtitle">{tr("settings.backup.history_desc")}</p>
        </div>
      </div>

      <div class="card-body">
        {#if backupHistory.length === 0}
          <div class="empty-state">
            <i class="bi bi-archive"></i>
            <p>{tr("settings.backup.no_backups")}</p>
          </div>
        {:else}
          <div class="table-container">
            <table class="data-table">
              <thead>
                <tr>
                  <th>{tr("settings.backup.date")}</th>
                  <th>{tr("settings.backup.size")}</th>
                  <th>{tr("settings.backup.type")}</th>
                  <th>{tr("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each backupHistory as backup}
                  <tr>
                    <td>{formatDate(backup.created_at)}</td>
                    <td>{formatSize(backup.size_bytes)}</td>
                    <td>
                      <span
                        class="badge {backup.type === 'auto'
                          ? 'badge-blue'
                          : 'badge-green'}"
                      >
                        {backup.type === "auto"
                          ? tr("settings.backup.automatic")
                          : tr("settings.backup.manual")}
                      </span>
                    </td>
                    <td>
                      <div class="action-buttons">
                        <button
                          class="btn-icon"
                          onclick={() => restoreBackup(backup.id)}
                          title={tr("settings.backup.restore")}
                        >
                          <i class="bi bi-arrow-counterclockwise"></i>
                        </button>
                        <button
                          class="btn-icon btn-danger"
                          onclick={() => deleteBackup(backup.id)}
                          title={tr("common.delete")}
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
  {/if}
</div>

<style>
  .backup-settings {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* Alerts */
  .alert {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
  }

  .alert-error {
    background: #fef2f2;
    color: #dc2626;
    border: 1px solid #fecaca;
  }

  .alert-success {
    background: #f0fdf4;
    color: #16a34a;
    border: 1px solid #bbf7d0;
  }

  :global([data-theme="dark"]) .alert-error {
    background: rgba(220, 38, 38, 0.1);
    border-color: rgba(220, 38, 38, 0.3);
  }

  :global([data-theme="dark"]) .alert-success {
    background: rgba(22, 163, 74, 0.1);
    border-color: rgba(22, 163, 74, 0.3);
  }

  .alert-close {
    margin-left: auto;
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
    opacity: 0.6;
  }

  .alert-close:hover {
    opacity: 1;
  }

  /* Loading */
  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    gap: 1rem;
    color: #6b7280;
  }

  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Cards */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    overflow: hidden;
  }

  :global([data-theme="dark"]) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .card-header {
    border-bottom-color: #374151;
  }

  .card-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  :global([data-theme="dark"]) .card-header h3 {
    color: #f9fafb;
  }

  .card-subtitle {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0.25rem 0 0 0;
  }

  .card-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }

  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }

  .card-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }

  .card-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }

  :global([data-theme="dark"]) .card-icon.blue {
    background: rgba(37, 99, 235, 0.2);
  }

  :global([data-theme="dark"]) .card-icon.green {
    background: rgba(22, 163, 74, 0.2);
  }

  :global([data-theme="dark"]) .card-icon.amber {
    background: rgba(217, 119, 6, 0.2);
  }

  .card-body {
    padding: 1.25rem;
  }

  /* Settings Rows */
  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    border-bottom: 1px solid #f3f4f6;
  }

  .setting-row:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .setting-row:first-child {
    padding-top: 0;
  }

  :global([data-theme="dark"]) .setting-row {
    border-bottom-color: #374151;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .setting-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
  }

  :global([data-theme="dark"]) .setting-label {
    color: #e5e7eb;
  }

  .setting-hint {
    font-size: 0.75rem;
    color: #9ca3af;
  }

  .setting-value {
    font-size: 0.75rem;
    color: #6b7280;
    font-weight: 500;
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    flex-shrink: 0;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #d1d5db;
    transition: 0.2s;
    border-radius: 24px;
  }

  .toggle-slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: 0.2s;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .toggle-switch input:checked + .toggle-slider {
    background-color: #22c55e;
  }

  .toggle-switch input:checked + .toggle-slider:before {
    transform: translateX(20px);
  }

  /* Select Input */
  .select-input {
    padding: 0.5rem 2rem 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    background: white;
    color: #374151;
    cursor: pointer;
    appearance: none;
    background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
    background-position: right 0.5rem center;
    background-repeat: no-repeat;
    background-size: 1.25rem;
  }

  :global([data-theme="dark"]) .select-input {
    background-color: #374151;
    border-color: #4b5563;
    color: #e5e7eb;
  }

  /* Buttons */
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 0.5rem;
    border: none;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #22c55e;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #16a34a;
  }

  .btn-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  /* Table */
  .table-container {
    overflow-x: auto;
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
  }

  .data-table th {
    text-align: left;
    padding: 0.75rem 1rem;
    background: #f9fafb;
    font-weight: 500;
    color: #6b7280;
    border-bottom: 1px solid #e5e7eb;
  }

  :global([data-theme="dark"]) .data-table th {
    background: #374151;
    color: #9ca3af;
    border-bottom-color: #4b5563;
  }

  .data-table td {
    padding: 0.75rem 1rem;
    border-top: 1px solid #f3f4f6;
    color: #374151;
  }

  :global([data-theme="dark"]) .data-table td {
    border-top-color: #374151;
    color: #e5e7eb;
  }

  .data-table tbody tr:hover {
    background: #f9fafb;
  }

  :global([data-theme="dark"]) .data-table tbody tr:hover {
    background: #374151;
  }

  /* Badges */
  .badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.625rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 9999px;
  }

  .badge-blue {
    background: #dbeafe;
    color: #2563eb;
  }

  .badge-green {
    background: #dcfce7;
    color: #16a34a;
  }

  :global([data-theme="dark"]) .badge-blue {
    background: rgba(37, 99, 235, 0.2);
  }

  :global([data-theme="dark"]) .badge-green {
    background: rgba(22, 163, 74, 0.2);
  }

  /* Action Buttons */
  .action-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .btn-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    border: 1px solid #e5e7eb;
    background: white;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-icon:hover {
    background: #f3f4f6;
    color: #374151;
  }

  .btn-icon.btn-danger:hover {
    background: #fef2f2;
    color: #dc2626;
    border-color: #fecaca;
  }

  :global([data-theme="dark"]) .btn-icon {
    background: #374151;
    border-color: #4b5563;
    color: #9ca3af;
  }

  :global([data-theme="dark"]) .btn-icon:hover {
    background: #4b5563;
    color: #e5e7eb;
  }

  :global([data-theme="dark"]) .btn-icon.btn-danger:hover {
    background: rgba(220, 38, 38, 0.1);
    border-color: rgba(220, 38, 38, 0.3);
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    color: #9ca3af;
    text-align: center;
  }

  .empty-state i {
    font-size: 2rem;
    margin-bottom: 0.75rem;
    opacity: 0.5;
  }

  .empty-state p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
