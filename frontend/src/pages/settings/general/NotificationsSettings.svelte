<script>
  import { onMount } from "svelte";
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
        smtp_password: smtpPassword || undefined,
        smtp_from_email: smtpFromEmail,
        gotify_enabled: gotifyEnabled,
        gotify_url: gotifyUrl,
        gotify_token: gotifyToken || undefined,
        gotify_priority: gotifyPriority,
        apprise_enabled: appriseEnabled,
        apprise_url: appriseUrl,
        ntfy_enabled: ntfyEnabled,
        ntfy_url: ntfyUrl,
        ntfy_topic: ntfyTopic,
        ntfy_token: ntfyToken || undefined,
        notify_on_share: notifyOnShare,
        notify_on_comment: notifyOnComment,
        notify_on_upload: notifyOnUpload,
        notify_on_backup: notifyOnBackup,
        notify_on_login: notifyOnLogin,
        notify_on_error: notifyOnError,
      });
      toastSuccess(tr("settings.notifications.saved"));
    } catch (err) {
      toastError(err.message || tr("settings.notifications.save_error"));
    } finally {
      saving = false;
    }
  }

  async function testService(service) {
    testing = true;
    try {
      await admin.testNotificationService(service, { email: testEmailAddress });
      toastSuccess(tr("settings.notifications.test_success"));
    } catch (err) {
      toastError(tr("settings.notifications.test_failed"));
    } finally {
      testing = false;
    }
  }
</script>

{#if loading}
  <div class="flex justify-center items-center min-h-[300px]">
    <span class="loading loading-spinner loading-lg text-primary"></span>
  </div>
{:else}
  <div class="max-w-full">
    <div class="grid grid-cols-2 gap-4 mb-4 max-lg:grid-cols-1">
      <!-- SMTP Card -->
      <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden transition-all" class:border-primary={smtpEnabled} class:shadow-sm={smtpEnabled}>
        <div class="flex items-center gap-3 p-4">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-info/20 text-info shrink-0">
            <i class="bi bi-envelope-fill"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5">{tr("settings.notifications.smtp")}</h3>
            <p class="text-xs text-base-content/60 m-0">{tr("settings.notifications.smtp_desc")}</p>
          </div>
          <input type="checkbox" class="toggle toggle-primary toggle-sm" bind:checked={smtpEnabled} aria-label="Enable SMTP" />
        </div>
        {#if smtpEnabled}
          <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
            <div class="grid grid-cols-2 gap-3 mb-3">
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.smtp_host")}</span>
                <input type="text" class="input input-sm input-bordered w-full" bind:value={smtpHost} placeholder="smtp.example.com" />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.smtp_port")}</span>
                <input type="number" class="input input-sm input-bordered w-full" bind:value={smtpPort} />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.smtp_user")}</span>
                <input type="text" class="input input-sm input-bordered w-full" bind:value={smtpUsername} placeholder="user@example.com" />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.smtp_password")}</span>
                <input type="password" class="input input-sm input-bordered w-full" bind:value={smtpPassword} placeholder="••••••••" />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.smtp_from")}</span>
                <input type="email" class="input input-sm input-bordered w-full" bind:value={smtpFromEmail} placeholder="noreply@example.com" />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.smtp_security")}</span>
                <select class="select select-sm select-bordered w-full" bind:value={smtpSecure}>
                  <option value="tls">TLS</option>
                  <option value="ssl">SSL</option>
                  <option value="none">{tr("common.none")}</option>
                </select>
              </div>
            </div>
            <div class="flex gap-2">
              <input type="email" class="input input-sm input-bordered flex-1" bind:value={testEmailAddress} placeholder={tr("settings.notifications.test_email")} />
              <button class="btn btn-sm btn-ghost" onclick={() => testService("smtp")} disabled={testing || !testEmailAddress}>
                <i class="bi bi-send"></i>
                {tr("settings.notifications.test")}
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Gotify Card -->
      <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden transition-all" class:border-primary={gotifyEnabled} class:shadow-sm={gotifyEnabled}>
        <div class="flex items-center gap-3 p-4">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-warning/20 text-warning shrink-0">
            <i class="bi bi-bell-fill"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5">{tr("settings.notifications.gotify")}</h3>
            <p class="text-xs text-base-content/60 m-0">{tr("settings.notifications.gotify_desc")}</p>
          </div>
          <input type="checkbox" class="toggle toggle-primary toggle-sm" bind:checked={gotifyEnabled} aria-label="Enable Gotify" />
        </div>
        {#if gotifyEnabled}
          <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
            <div class="grid grid-cols-2 gap-3 mb-3">
              <div class="flex flex-col gap-1 col-span-2">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.gotify_url")}</span>
                <input type="url" class="input input-sm input-bordered w-full" bind:value={gotifyUrl} placeholder="https://gotify.example.com" />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.gotify_token")}</span>
                <input type="password" class="input input-sm input-bordered w-full" bind:value={gotifyToken} placeholder="App Token" />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.gotify_priority")}</span>
                <select class="select select-sm select-bordered w-full" bind:value={gotifyPriority}>
                  <option value={1}>1 - Min</option>
                  <option value={3}>3 - Low</option>
                  <option value={5}>5 - Normal</option>
                  <option value={7}>7 - High</option>
                  <option value={10}>10 - Max</option>
                </select>
              </div>
            </div>
            <button class="btn btn-sm btn-ghost" onclick={() => testService("gotify")} disabled={testing}>
              <i class="bi bi-send"></i>
              {tr("settings.notifications.test")}
            </button>
          </div>
        {/if}
      </div>

      <!-- ntfy Card -->
      <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden transition-all" class:border-primary={ntfyEnabled} class:shadow-sm={ntfyEnabled}>
        <div class="flex items-center gap-3 p-4">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-success/20 text-success shrink-0">
            <i class="bi bi-broadcast"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5">{tr("settings.notifications.ntfy")}</h3>
            <p class="text-xs text-base-content/60 m-0">{tr("settings.notifications.ntfy_desc")}</p>
          </div>
          <input type="checkbox" class="toggle toggle-primary toggle-sm" bind:checked={ntfyEnabled} aria-label="Enable ntfy" />
        </div>
        {#if ntfyEnabled}
          <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
            <div class="grid grid-cols-2 gap-3 mb-3">
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.ntfy_url")}</span>
                <input type="url" class="input input-sm input-bordered w-full" bind:value={ntfyUrl} placeholder="https://ntfy.sh" />
              </div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.ntfy_topic")}</span>
                <input type="text" class="input input-sm input-bordered w-full" bind:value={ntfyTopic} placeholder="syncspace-alerts" />
              </div>
              <div class="flex flex-col gap-1 col-span-2">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.ntfy_token")}</span>
                <input type="password" class="input input-sm input-bordered w-full" bind:value={ntfyToken} placeholder={tr("common.optional")} />
              </div>
            </div>
            <button class="btn btn-sm btn-ghost" onclick={() => testService("ntfy")} disabled={testing}>
              <i class="bi bi-send"></i>
              {tr("settings.notifications.test")}
            </button>
          </div>
        {/if}
      </div>

      <!-- Apprise Card -->
      <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden transition-all" class:border-primary={appriseEnabled} class:shadow-sm={appriseEnabled}>
        <div class="flex items-center gap-3 p-4">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-pink-500/20 text-pink-500 shrink-0">
            <i class="bi bi-lightning-fill"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5">{tr("settings.notifications.apprise")}</h3>
            <p class="text-xs text-base-content/60 m-0">{tr("settings.notifications.apprise_desc")}</p>
          </div>
          <input type="checkbox" class="toggle toggle-primary toggle-sm" bind:checked={appriseEnabled} aria-label="Enable Apprise" />
        </div>
        {#if appriseEnabled}
          <div class="px-4 pb-4 border-t border-base-300 -mt-2 pt-4">
            <div class="grid grid-cols-2 gap-3 mb-3">
              <div class="flex flex-col gap-1 col-span-2">
                <span class="text-xs font-medium text-base-content/60">{tr("settings.notifications.apprise_url")}</span>
                <input type="url" class="input input-sm input-bordered w-full" bind:value={appriseUrl} placeholder="http://localhost:8000/notify" />
              </div>
            </div>
            <button class="btn btn-sm btn-ghost" onclick={() => testService("apprise")} disabled={testing}>
              <i class="bi bi-send"></i>
              {tr("settings.notifications.test")}
            </button>
          </div>
        {/if}
      </div>
    </div>

    <!-- Events Card -->
    <div class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all p-4 mb-4">
      <div class="flex items-center gap-3 mb-4">
        <div class="w-10 h-10 rounded-lg bg-violet-500/20 text-violet-500 flex items-center justify-center text-lg">
          <i class="bi bi-lightning-charge"></i>
        </div>
        <div>
          <h3 class="text-[0.9375rem] font-semibold text-base-content m-0 mb-0.5">{tr("settings.notifications.events")}</h3>
          <p class="text-xs text-base-content/60 m-0">{tr("settings.notifications.events_desc")}</p>
        </div>
      </div>
      <div class="grid grid-cols-3 gap-2 max-md:grid-cols-2 max-sm:grid-cols-1">
        <label class="flex items-center gap-2 px-3 py-2.5 bg-base-200 rounded-lg cursor-pointer hover:bg-base-300 transition-colors">
          <input type="checkbox" class="checkbox checkbox-sm checkbox-primary" bind:checked={notifyOnShare} />
          <i class="bi bi-share text-sm text-base-content/60"></i>
          <span class="text-[0.8125rem] text-base-content">{tr("settings.notifications.on_share")}</span>
        </label>
        <label class="flex items-center gap-2 px-3 py-2.5 bg-base-200 rounded-lg cursor-pointer hover:bg-base-300 transition-colors">
          <input type="checkbox" class="checkbox checkbox-sm checkbox-primary" bind:checked={notifyOnComment} />
          <i class="bi bi-chat text-sm text-base-content/60"></i>
          <span class="text-[0.8125rem] text-base-content">{tr("settings.notifications.on_comment")}</span>
        </label>
        <label class="flex items-center gap-2 px-3 py-2.5 bg-base-200 rounded-lg cursor-pointer hover:bg-base-300 transition-colors">
          <input type="checkbox" class="checkbox checkbox-sm checkbox-primary" bind:checked={notifyOnUpload} />
          <i class="bi bi-cloud-upload text-sm text-base-content/60"></i>
          <span class="text-[0.8125rem] text-base-content">{tr("settings.notifications.on_upload")}</span>
        </label>
        <label class="flex items-center gap-2 px-3 py-2.5 bg-base-200 rounded-lg cursor-pointer hover:bg-base-300 transition-colors">
          <input type="checkbox" class="checkbox checkbox-sm checkbox-primary" bind:checked={notifyOnBackup} />
          <i class="bi bi-archive text-sm text-base-content/60"></i>
          <span class="text-[0.8125rem] text-base-content">{tr("settings.notifications.on_backup")}</span>
        </label>
        <label class="flex items-center gap-2 px-3 py-2.5 bg-base-200 rounded-lg cursor-pointer hover:bg-base-300 transition-colors">
          <input type="checkbox" class="checkbox checkbox-sm checkbox-primary" bind:checked={notifyOnLogin} />
          <i class="bi bi-box-arrow-in-right text-sm text-base-content/60"></i>
          <span class="text-[0.8125rem] text-base-content">{tr("settings.notifications.on_login")}</span>
        </label>
        <label class="flex items-center gap-2 px-3 py-2.5 bg-base-200 rounded-lg cursor-pointer hover:bg-base-300 transition-colors">
          <input type="checkbox" class="checkbox checkbox-sm checkbox-primary" bind:checked={notifyOnError} />
          <i class="bi bi-exclamation-triangle text-sm text-base-content/60"></i>
          <span class="text-[0.8125rem] text-base-content">{tr("settings.notifications.on_error")}</span>
        </label>
      </div>
    </div>

    <div class="flex justify-end">
      <button class="btn btn-primary" onclick={saveSettings} disabled={saving}>
        {#if saving}
          <span class="loading loading-spinner loading-sm"></span>
        {:else}
          <i class="bi bi-check-lg"></i>
        {/if}
        {tr("common.save")}
      </button>
    </div>
  </div>
{/if}

<style>
  /* Minimal CSS - most styling via Tailwind utilities */
</style>
