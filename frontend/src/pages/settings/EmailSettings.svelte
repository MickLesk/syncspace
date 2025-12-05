<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import {
    success as toastSuccess,
    error as toastError,
  } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let saving = $state(false);
  let testing = $state(false);

  let smtpEnabled = $state(false);
  let smtpHost = $state("");
  let smtpPort = $state(587);
  let smtpSecure = $state("tls");
  let smtpUsername = $state("");
  let smtpPassword = $state("");
  let smtpFromEmail = $state("");
  let smtpFromName = $state("SyncSpace");

  let notifyOnShare = $state(true);
  let notifyOnComment = $state(true);
  let notifyOnUpload = $state(false);
  let notifyOnBackup = $state(true);
  let notifyOnLogin = $state(false);

  let testEmailAddress = $state("");

  onMount(async () => {
    await loadEmailSettings();
  });

  async function loadEmailSettings() {
    loading = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/admin/email-settings",
        {
          headers: { Authorization: `Bearer ${token}` },
        }
      );
      if (response.ok) {
        const data = await response.json();
        smtpEnabled = data.smtp_enabled || false;
        smtpHost = data.smtp_host || "";
        smtpPort = data.smtp_port || 587;
        smtpSecure = data.smtp_secure || "tls";
        smtpUsername = data.smtp_username || "";
        smtpFromEmail = data.smtp_from_email || "";
        smtpFromName = data.smtp_from_name || "SyncSpace";
        notifyOnShare = data.notify_on_share ?? true;
        notifyOnComment = data.notify_on_comment ?? true;
        notifyOnUpload = data.notify_on_upload ?? false;
        notifyOnBackup = data.notify_on_backup ?? true;
        notifyOnLogin = data.notify_on_login ?? false;
      }
    } catch (err) {
      console.error("Failed to load email settings:", err);
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    saving = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/admin/email-settings",
        {
          method: "PUT",
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            smtp_enabled: smtpEnabled,
            smtp_host: smtpHost,
            smtp_port: smtpPort,
            smtp_secure: smtpSecure,
            smtp_username: smtpUsername,
            smtp_password: smtpPassword || undefined,
            smtp_from_email: smtpFromEmail,
            smtp_from_name: smtpFromName,
            notify_on_share: notifyOnShare,
            notify_on_comment: notifyOnComment,
            notify_on_upload: notifyOnUpload,
            notify_on_backup: notifyOnBackup,
            notify_on_login: notifyOnLogin,
          }),
        }
      );
      if (response.ok) {
        toastSuccess(tr("settings.email.saved"));
      } else {
        toastError(tr("settings.email.save_error"));
      }
    } catch (err) {
      toastError(err.message);
    } finally {
      saving = false;
    }
  }

  async function sendTestEmail() {
    if (!testEmailAddress) return;
    testing = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/admin/email-settings/test",
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ email: testEmailAddress }),
        }
      );
      if (response.ok) {
        toastSuccess(tr("settings.email.test_success"));
      } else {
        toastError(tr("settings.email.test_failed"));
      }
    } catch (err) {
      toastError(err.message);
    } finally {
      testing = false;
    }
  }
</script>

{#if loading}
  <div class="flex items-center justify-center py-12">
    <div
      class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"
    ></div>
  </div>
{:else}
  <div class="space-y-6">
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center gap-3 mb-6">
        <div
          class="w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center"
        >
          <i class="bi bi-envelope-fill text-blue-600 dark:text-blue-400"></i>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {tr("settings.email.smtp_settings")}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("settings.email.smtp_desc")}
          </p>
        </div>
      </div>

      <div class="space-y-4">
        <label
          class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <input
            type="checkbox"
            bind:checked={smtpEnabled}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span class="text-gray-900 dark:text-white font-medium"
            >SMTP aktivieren</span
          >
        </label>

        {#if smtpEnabled}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <span
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >{tr("settings.email.smtp_host")}</span
              >
              <input
                type="text"
                bind:value={smtpHost}
                placeholder="smtp.example.com"
                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>
            <div>
              <span
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >{tr("settings.email.smtp_port")}</span
              >
              <input
                type="number"
                bind:value={smtpPort}
                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>
            <div>
              <span
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >{tr("settings.email.smtp_user")}</span
              >
              <input
                type="text"
                bind:value={smtpUsername}
                placeholder="user@example.com"
                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>
            <div>
              <span
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >{tr("settings.email.smtp_password")}</span
              >
              <input
                type="password"
                bind:value={smtpPassword}
                placeholder="••••••••"
                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>
            <div>
              <span
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >{tr("settings.email.smtp_from")}</span
              >
              <input
                type="email"
                bind:value={smtpFromEmail}
                placeholder="noreply@example.com"
                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>
            <div>
              <span
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >{tr("settings.email.smtp_from_name")}</span
              >
              <input
                type="text"
                bind:value={smtpFromName}
                placeholder="SyncSpace"
                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
              />
            </div>
          </div>

          <div>
            <span
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >Verschlüsselung</span
            >
            <div class="flex gap-4">
              <label class="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  bind:group={smtpSecure}
                  value="none"
                  class="text-blue-600"
                />
                <span class="text-gray-700 dark:text-gray-300">Keine</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  bind:group={smtpSecure}
                  value="tls"
                  class="text-blue-600"
                />
                <span class="text-gray-700 dark:text-gray-300">TLS</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  bind:group={smtpSecure}
                  value="ssl"
                  class="text-blue-600"
                />
                <span class="text-gray-700 dark:text-gray-300">SSL</span>
              </label>
            </div>
          </div>

          <div
            class="flex items-center gap-3 pt-4 border-t border-gray-200 dark:border-gray-700"
          >
            <input
              type="email"
              bind:value={testEmailAddress}
              placeholder="test@example.com"
              class="flex-1 px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
            />
            <button
              onclick={sendTestEmail}
              disabled={testing || !testEmailAddress}
              class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg flex items-center gap-2 disabled:opacity-50"
            >
              {#if testing}
                <div
                  class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"
                ></div>
              {:else}
                <i class="bi bi-send"></i>
              {/if}
              Test senden
            </button>
          </div>
        {/if}
      </div>
    </div>

    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center gap-3 mb-6">
        <div
          class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center"
        >
          <i class="bi bi-bell-fill text-purple-600 dark:text-purple-400"></i>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {tr("settings.email.notifications")}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("settings.email.notifications_desc")}
          </p>
        </div>
      </div>

      <div class="space-y-3">
        <label
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="text-gray-900 dark:text-white font-medium"
              >{tr("settings.email.enable_share")}</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("settings.email.enable_share_desc")}
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={notifyOnShare}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>
        <label
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="text-gray-900 dark:text-white font-medium"
              >Kommentar-Benachrichtigungen</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              E-Mail senden bei neuen Kommentaren
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={notifyOnComment}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>
        <label
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="text-gray-900 dark:text-white font-medium"
              >Upload-Benachrichtigungen</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              E-Mail senden bei neuen Uploads
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={notifyOnUpload}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>
        <label
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="text-gray-900 dark:text-white font-medium"
              >{tr("settings.email.enable_backup")}</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("settings.email.enable_backup_desc")}
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={notifyOnBackup}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>
        <label
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="text-gray-900 dark:text-white font-medium"
              >Login-Benachrichtigungen</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              E-Mail senden bei neuen Anmeldungen
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={notifyOnLogin}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>
      </div>
    </div>

    <div class="flex justify-end">
      <button
        onclick={saveSettings}
        disabled={saving}
        class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg flex items-center gap-2 disabled:opacity-50"
      >
        {#if saving}
          <div
            class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"
          ></div>
        {:else}
          <i class="bi bi-check-lg"></i>
        {/if}
        {tr("common.save")}
      </button>
    </div>
  </div>
{/if}
