<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import {
    success as toastSuccess,
    error as toastError,
  } from "../../../stores/toast.js";
  import { admin } from "../../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let saving = $state(false);
  let testing = $state(false);

  // User Notification Preferences
  let enableNotifications = $state(true);
  let autoBackupNotify = $state(true);

  // SMTP Settings
  let smtpEnabled = $state(false);
  let smtpHost = $state("");
  let smtpPort = $state(587);
  let smtpSecure = $state("tls");
  let smtpUsername = $state("");
  let smtpPassword = $state("");
  let smtpFromEmail = $state("");

  // Push Services
  let gotifyEnabled = $state(false);
  let gotifyUrl = $state("");
  let gotifyToken = $state("");
  let gotifyPriority = $state(5);

  let appriseEnabled = $state(false);
  let appriseUrl = $state("");

  let ntfyEnabled = $state(false);
  let ntfyUrl = $state("https://ntfy.sh");
  let ntfyTopic = $state("");
  let ntfyToken = $state("");

  // Events
  let notifyOnShare = $state(true);
  let notifyOnComment = $state(true);
  let notifyOnUpload = $state(false);
  let notifyOnBackup = $state(true);
  let notifyOnLogin = $state(false);
  let notifyOnError = $state(true);

  let testEmailAddress = $state("");

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    loading = true;
    try {
      const data = await admin.getNotificationSettings();
      smtpEnabled = data.smtp_enabled || false;
      smtpHost = data.smtp_host || "";
      smtpPort = data.smtp_port || 587;
      smtpSecure = data.smtp_secure || "tls";
      smtpUsername = data.smtp_username || "";
      smtpFromEmail = data.smtp_from_email || "";
      gotifyEnabled = data.gotify_enabled || false;
      gotifyUrl = data.gotify_url || "";
      gotifyToken = data.gotify_token || "";
      gotifyPriority = data.gotify_priority || 5;
      appriseEnabled = data.apprise_enabled || false;
      appriseUrl = data.apprise_url || "";
      ntfyEnabled = data.ntfy_enabled || false;
      ntfyUrl = data.ntfy_url || "https://ntfy.sh";
      ntfyTopic = data.ntfy_topic || "";
      ntfyToken = data.ntfy_token || "";
      notifyOnShare = data.notify_on_share ?? true;
      notifyOnComment = data.notify_on_comment ?? true;
      notifyOnUpload = data.notify_on_upload ?? false;
      notifyOnBackup = data.notify_on_backup ?? true;
      notifyOnLogin = data.notify_on_login ?? false;
      notifyOnError = data.notify_on_error ?? true;
    } catch (err) {
      console.error("Failed to load notification settings:", err);
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    saving = true;
    try {
      await admin.updateNotificationSettings({
        smtp_enabled: smtpEnabled,
        smtp_host: smtpHost,
        smtp_port: smtpPort,
        smtp_secure: smtpSecure,
        smtp_username: smtpUsername,
        smtp_password: smtpPassword,
        smtp_from_email: smtpFromEmail,
        gotify_enabled: gotifyEnabled,
        gotify_url: gotifyUrl,
        gotify_token: gotifyToken,
        gotify_priority: gotifyPriority,
        apprise_enabled: appriseEnabled,
        apprise_url: appriseUrl,
        ntfy_enabled: ntfyEnabled,
        ntfy_url: ntfyUrl,
        ntfy_topic: ntfyTopic,
        ntfy_token: ntfyToken,
        notify_on_share: notifyOnShare,
        notify_on_comment: notifyOnComment,
        notify_on_upload: notifyOnUpload,
        notify_on_backup: notifyOnBackup,
        notify_on_login: notifyOnLogin,
        notify_on_error: notifyOnError,
      });
      toastSuccess(tr("settings.notifications.saved"));
    } catch (err) {
      console.error("Failed to save settings:", err);
      toastError(tr("settings.notifications.save_failed"));
    } finally {
      saving = false;
    }
  }

  async function testService(service) {
    if (!testEmailAddress && service === "smtp") {
      toastError(tr("settings.notifications.enter_test_email"));
      return;
    }
    testing = true;
    try {
      await admin.testNotificationService(service, { email: testEmailAddress });
      toastSuccess(tr("settings.notifications.test_sent"));
    } catch (err) {
      console.error("Failed to test service:", err);
      toastError(tr("settings.notifications.test_failed"));
    } finally {
      testing = false;
    }
  }
</script>

{#if loading}
  <div class="flex items-center justify-center py-12">
    <span class="loading loading-spinner loading-lg text-primary"></span>
  </div>
{:else}
  <div class="space-y-6">
    <!-- User Preferences Section -->
    <div>
      <h2 class="text-lg font-semibold text-base-content mb-4">
        {tr("settings.notifications.preferences")}
      </h2>
      <div class="settings-grid">
        <div class="card">
          <div class="card-header">
            <div class="card-icon green"><i class="bi bi-bell-fill"></i></div>
            <div class="card-title">
              <h3>{tr("notifications")}</h3>
              <p>{tr("notificationSettings")}</p>
            </div>
          </div>
          <div class="toggle-list">
            <div class="toggle-item">
              <div class="toggle-info">
                <span class="toggle-label">{tr("emailNotifications")}</span>
                <span class="toggle-desc"
                  >{tr("receiveEmailNotifications")}</span
                >
              </div>
              <button
                class="toggle-switch"
                class:active={enableNotifications}
                onclick={() => (enableNotifications = !enableNotifications)}
                role="switch"
                aria-checked={enableNotifications}
                aria-label="Toggle email notifications"
              >
                <span class="toggle-knob"></span>
              </button>
            </div>
            <div class="toggle-item">
              <div class="toggle-info">
                <span class="toggle-label">{tr("autoBackupNotifications")}</span
                >
                <span class="toggle-desc"
                  >{tr("getNotifiedWhenBackupsComplete")}</span
                >
              </div>
              <button
                class="toggle-switch"
                class:active={autoBackupNotify}
                onclick={() => (autoBackupNotify = !autoBackupNotify)}
                role="switch"
                aria-checked={autoBackupNotify}
                aria-label="Toggle backup notifications"
              >
                <span class="toggle-knob"></span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Admin Settings Section -->
    <div>
      <h2 class="text-lg font-semibold text-base-content mb-4">
        {tr("settings.notifications.admin_settings")}
      </h2>
      <div class="space-y-4">
        <!-- SMTP Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(59,130,246,0.15)] transition-all overflow-hidden"
          class:border-primary={smtpEnabled}
          class:shadow-sm={smtpEnabled}
        >
          <div class="flex items-center gap-3 p-4">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-primary/20 text-primary shrink-0"
            >
              <i class="bi bi-envelope-fill"></i>
            </div>
            <div class="flex-1 min-w-0">
              <h3
                class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5"
              >
                {tr("settings.notifications.smtp")}
              </h3>
              <p class="text-xs text-base-content/60 m-0">
                {tr("settings.notifications.smtp_desc")}
              </p>
            </div>
            <input
              type="checkbox"
              class="toggle toggle-primary toggle-sm"
              bind:checked={smtpEnabled}
              aria-label="Enable SMTP"
            />
          </div>
          {#if smtpEnabled}
            <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
              <div class="grid grid-cols-2 gap-3 mb-3">
                <UIInput
                  label={tr("settings.notifications.smtp_host")}
                  bind:value={smtpHost}
                  placeholder="smtp.gmail.com"
                />
                <UIInput
                  type="number"
                  label={tr("settings.notifications.smtp_port")}
                  bind:value={smtpPort}
                  placeholder="587"
                />
                <UIInput
                  label={tr("settings.notifications.smtp_user")}
                  bind:value={smtpUsername}
                  placeholder="user@example.com"
                />
                <UIInput
                  type="password"
                  label={tr("settings.notifications.smtp_password")}
                  bind:value={smtpPassword}
                  placeholder="••••••••"
                />
                <UIInput
                  type="email"
                  label={tr("settings.notifications.smtp_from")}
                  bind:value={smtpFromEmail}
                  placeholder="noreply@example.com"
                />
                <UISelect
                  label={tr("settings.notifications.smtp_security")}
                  bind:value={smtpSecure}
                  options={[
                    { value: "tls", label: "TLS" },
                    { value: "ssl", label: "SSL" },
                    { value: "none", label: tr("common.none") },
                  ]}
                />
              </div>
              <div class="flex gap-2">
                <input
                  type="email"
                  class="input input-sm input-bordered flex-1"
                  bind:value={testEmailAddress}
                  placeholder={tr("settings.notifications.test_email")}
                />
                <UIButton
                  variant="ghost"
                  size="sm"
                  onclick={() => testService("smtp")}
                  disabled={testing || !testEmailAddress}
                >
                  <i class="bi bi-send"></i>
                  {tr("settings.notifications.test")}
                </UIButton>
              </div>
            </div>
          {/if}
        </div>

        <!-- Gotify Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(234,179,8,0.15)] transition-all overflow-hidden"
          class:border-warning={gotifyEnabled}
          class:shadow-sm={gotifyEnabled}
        >
          <div class="flex items-center gap-3 p-4">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-warning/20 text-warning shrink-0"
            >
              <i class="bi bi-bell-fill"></i>
            </div>
            <div class="flex-1 min-w-0">
              <h3
                class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5"
              >
                {tr("settings.notifications.gotify")}
              </h3>
              <p class="text-xs text-base-content/60 m-0">
                {tr("settings.notifications.gotify_desc")}
              </p>
            </div>
            <input
              type="checkbox"
              class="toggle toggle-warning toggle-sm"
              bind:checked={gotifyEnabled}
              aria-label="Enable Gotify"
            />
          </div>
          {#if gotifyEnabled}
            <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
              <div class="grid grid-cols-2 gap-3 mb-3">
                <UIInput
                  label={tr("settings.notifications.gotify_url")}
                  bind:value={gotifyUrl}
                  placeholder="https://gotify.example.com"
                />
                <UIInput
                  label={tr("settings.notifications.gotify_token")}
                  bind:value={gotifyToken}
                  placeholder="A...."
                />
                <UIInput
                  type="number"
                  label={tr("settings.notifications.gotify_priority")}
                  bind:value={gotifyPriority}
                  placeholder="5"
                />
              </div>
              <UIButton
                variant="ghost"
                size="sm"
                onclick={() => testService("gotify")}
                disabled={testing}
              >
                <i class="bi bi-send"></i>
                {tr("settings.notifications.test")}
              </UIButton>
            </div>
          {/if}
        </div>

        <!-- Apprise Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(168,85,247,0.15)] transition-all overflow-hidden"
          class:border-purple-500={appriseEnabled}
          class:shadow-sm={appriseEnabled}
        >
          <div class="flex items-center gap-3 p-4">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-purple-500/20 text-purple-500 shrink-0"
            >
              <i class="bi bi-megaphone-fill"></i>
            </div>
            <div class="flex-1 min-w-0">
              <h3
                class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5"
              >
                {tr("settings.notifications.apprise")}
              </h3>
              <p class="text-xs text-base-content/60 m-0">
                {tr("settings.notifications.apprise_desc")}
              </p>
            </div>
            <input
              type="checkbox"
              class="toggle toggle-sm"
              bind:checked={appriseEnabled}
              aria-label="Enable Apprise"
            />
          </div>
          {#if appriseEnabled}
            <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
              <UIInput
                label={tr("settings.notifications.apprise_url")}
                bind:value={appriseUrl}
                placeholder="apprise://..."
              />
              <UIButton
                variant="ghost"
                size="sm"
                onclick={() => testService("apprise")}
                disabled={testing}
                class="mt-3"
              >
                <i class="bi bi-send"></i>
                {tr("settings.notifications.test")}
              </UIButton>
            </div>
          {/if}
        </div>

        <!-- Ntfy Card -->
        <div
          class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden"
          class:border-success={ntfyEnabled}
          class:shadow-sm={ntfyEnabled}
        >
          <div class="flex items-center gap-3 p-4">
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-success/20 text-success shrink-0"
            >
              <i class="bi bi-chat-dots-fill"></i>
            </div>
            <div class="flex-1 min-w-0">
              <h3
                class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5"
              >
                {tr("settings.notifications.ntfy")}
              </h3>
              <p class="text-xs text-base-content/60 m-0">
                {tr("settings.notifications.ntfy_desc")}
              </p>
            </div>
            <input
              type="checkbox"
              class="toggle toggle-success toggle-sm"
              bind:checked={ntfyEnabled}
              aria-label="Enable Ntfy"
            />
          </div>
          {#if ntfyEnabled}
            <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
              <div class="grid grid-cols-2 gap-3 mb-3">
                <UIInput
                  label={tr("settings.notifications.ntfy_url")}
                  bind:value={ntfyUrl}
                  placeholder="https://ntfy.sh"
                />
                <UIInput
                  label={tr("settings.notifications.ntfy_topic")}
                  bind:value={ntfyTopic}
                  placeholder="my-topic"
                />
                <UIInput
                  label={tr("settings.notifications.ntfy_token")}
                  bind:value={ntfyToken}
                  placeholder="Optional"
                />
              </div>
              <UIButton
                variant="ghost"
                size="sm"
                onclick={() => testService("ntfy")}
                disabled={testing}
              >
                <i class="bi bi-send"></i>
                {tr("settings.notifications.test")}
              </UIButton>
            </div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Event Triggers Section -->
    <div>
      <h2 class="text-lg font-semibold text-base-content mb-4">
        {tr("settings.notifications.event_triggers")}
      </h2>
      <div class="space-y-2">
        <UICheckbox
          label={tr("settings.notifications.notify_on_share")}
          bind:checked={notifyOnShare}
        />
        <UICheckbox
          label={tr("settings.notifications.notify_on_comment")}
          bind:checked={notifyOnComment}
        />
        <UICheckbox
          label={tr("settings.notifications.notify_on_upload")}
          bind:checked={notifyOnUpload}
        />
        <UICheckbox
          label={tr("settings.notifications.notify_on_backup")}
          bind:checked={notifyOnBackup}
        />
        <UICheckbox
          label={tr("settings.notifications.notify_on_login")}
          bind:checked={notifyOnLogin}
        />
        <UICheckbox
          label={tr("settings.notifications.notify_on_error")}
          bind:checked={notifyOnError}
        />
      </div>
    </div>

    <!-- Save Button -->
    <div class="flex justify-end pt-4 border-t border-base-300">
      <UIButton variant="primary" onclick={saveSettings} disabled={saving}>
        {#if saving}
          <span class="loading loading-spinner loading-sm"></span>
        {/if}
        {tr("common.save")}
      </UIButton>
    </div>
  </div>
{/if}

<style>
  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
  }
  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-header {
    display: flex;
    align-items: flex-start;
    gap: 0.875rem;
    margin-bottom: 1.25rem;
  }
  .card-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }
  .card-icon.green {
    background: #dcfce7;
    color: #16a34a;
  }
  :global(.dark) .card-icon.green {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }

  .card-title {
    flex: 1;
  }
  .card-title h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }
  .card-title p {
    font-size: 0.8125rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .card-title h3 {
    color: #f9fafb;
  }
  :global(.dark) .card-title p {
    color: #9ca3af;
  }

  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .toggle-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .toggle-item {
    background: #374151;
  }
  .toggle-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .toggle-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }
  .toggle-desc {
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .toggle-label {
    color: #f9fafb;
  }
  :global(.dark) .toggle-desc {
    color: #9ca3af;
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    width: 44px;
    height: 24px;
    background: #d1d5db;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.2s;
    flex-shrink: 0;
  }
  .toggle-switch.active {
    background: #22c55e;
  }
  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    background: white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s;
  }
  .toggle-switch.active .toggle-knob {
    transform: translateX(20px);
  }
  :global(.dark) .toggle-switch {
    background: #4b5563;
  }
</style>
