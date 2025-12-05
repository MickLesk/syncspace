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
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/admin/notification-settings",
        { headers: { Authorization: `Bearer ${token}` } }
      );
      if (response.ok) {
        const data = await response.json();
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
      }
    } catch (err) {
      console.error("Failed to load notification settings:", err);
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    saving = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/admin/notification-settings",
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
          }),
        }
      );
      if (response.ok) {
        toastSuccess(tr("settings.notifications.saved"));
      } else {
        toastError(tr("settings.notifications.save_error"));
      }
    } catch (err) {
      toastError(err.message);
    } finally {
      saving = false;
    }
  }

  async function testService(service) {
    testing = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        `http://localhost:8080/api/admin/notification-settings/test/${service}`,
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
        toastSuccess(tr("settings.notifications.test_success"));
      } else {
        toastError(tr("settings.notifications.test_failed"));
      }
    } catch (err) {
      toastError(err.message);
    } finally {
      testing = false;
    }
  }
</script>

{#if loading}
  <div class="loading-container">
    <div class="spinner"></div>
  </div>
{:else}
  <div class="notifications-settings">
    <div class="services-grid">
      <!-- SMTP Card -->
      <div class="service-card" class:enabled={smtpEnabled}>
        <div class="service-header">
          <div class="service-icon email-icon">
            <i class="bi bi-envelope-fill"></i>
          </div>
          <div class="service-info">
            <h3>{tr("settings.notifications.smtp")}</h3>
            <p>{tr("settings.notifications.smtp_desc")}</p>
          </div>
          <label class="toggle" aria-label="Enable SMTP">
            <input type="checkbox" bind:checked={smtpEnabled} />
            <span class="toggle-slider"></span>
          </label>
        </div>
        {#if smtpEnabled}
          <div class="service-config">
            <div class="config-grid">
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.smtp_host")}</span
                ><input
                  type="text"
                  bind:value={smtpHost}
                  placeholder="smtp.example.com"
                />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.smtp_port")}</span
                ><input type="number" bind:value={smtpPort} />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.smtp_user")}</span
                ><input
                  type="text"
                  bind:value={smtpUsername}
                  placeholder="user@example.com"
                />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.smtp_password")}</span
                ><input
                  type="password"
                  bind:value={smtpPassword}
                  placeholder="••••••••"
                />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.smtp_from")}</span
                ><input
                  type="email"
                  bind:value={smtpFromEmail}
                  placeholder="noreply@example.com"
                />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.smtp_security")}</span
                ><select bind:value={smtpSecure}
                  ><option value="tls">TLS</option><option value="ssl"
                    >SSL</option
                  ><option value="none">{tr("common.none")}</option></select
                >
              </div>
            </div>
            <div class="test-section">
              <input
                type="email"
                bind:value={testEmailAddress}
                placeholder={tr("settings.notifications.test_email")}
              />
              <button
                class="btn-test"
                onclick={() => testService("smtp")}
                disabled={testing || !testEmailAddress}
                ><i class="bi bi-send"></i>
                {tr("settings.notifications.test")}</button
              >
            </div>
          </div>
        {/if}
      </div>

      <!-- Gotify Card -->
      <div class="service-card" class:enabled={gotifyEnabled}>
        <div class="service-header">
          <div class="service-icon gotify-icon">
            <i class="bi bi-bell-fill"></i>
          </div>
          <div class="service-info">
            <h3>{tr("settings.notifications.gotify")}</h3>
            <p>{tr("settings.notifications.gotify_desc")}</p>
          </div>
          <label class="toggle" aria-label="Enable Gotify">
            <input type="checkbox" bind:checked={gotifyEnabled} />
            <span class="toggle-slider"></span>
          </label>
        </div>
        {#if gotifyEnabled}
          <div class="service-config">
            <div class="config-grid">
              <div class="config-field full">
                <span class="field-label"
                  >{tr("settings.notifications.gotify_url")}</span
                ><input
                  type="url"
                  bind:value={gotifyUrl}
                  placeholder="https://gotify.example.com"
                />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.gotify_token")}</span
                ><input
                  type="password"
                  bind:value={gotifyToken}
                  placeholder="App Token"
                />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.gotify_priority")}</span
                ><select bind:value={gotifyPriority}
                  ><option value={1}>1 - Min</option><option value={3}
                    >3 - Low</option
                  ><option value={5}>5 - Normal</option><option value={7}
                    >7 - High</option
                  ><option value={10}>10 - Max</option></select
                >
              </div>
            </div>
            <button
              class="btn-test"
              onclick={() => testService("gotify")}
              disabled={testing}
              ><i class="bi bi-send"></i>
              {tr("settings.notifications.test")}</button
            >
          </div>
        {/if}
      </div>

      <!-- ntfy Card -->
      <div class="service-card" class:enabled={ntfyEnabled}>
        <div class="service-header">
          <div class="service-icon ntfy-icon">
            <i class="bi bi-broadcast"></i>
          </div>
          <div class="service-info">
            <h3>{tr("settings.notifications.ntfy")}</h3>
            <p>{tr("settings.notifications.ntfy_desc")}</p>
          </div>
          <label class="toggle" aria-label="Enable ntfy">
            <input type="checkbox" bind:checked={ntfyEnabled} />
            <span class="toggle-slider"></span>
          </label>
        </div>
        {#if ntfyEnabled}
          <div class="service-config">
            <div class="config-grid">
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.ntfy_url")}</span
                ><input
                  type="url"
                  bind:value={ntfyUrl}
                  placeholder="https://ntfy.sh"
                />
              </div>
              <div class="config-field">
                <span class="field-label"
                  >{tr("settings.notifications.ntfy_topic")}</span
                ><input
                  type="text"
                  bind:value={ntfyTopic}
                  placeholder="syncspace-alerts"
                />
              </div>
              <div class="config-field full">
                <span class="field-label"
                  >{tr("settings.notifications.ntfy_token")}</span
                ><input
                  type="password"
                  bind:value={ntfyToken}
                  placeholder={tr("common.optional")}
                />
              </div>
            </div>
            <button
              class="btn-test"
              onclick={() => testService("ntfy")}
              disabled={testing}
              ><i class="bi bi-send"></i>
              {tr("settings.notifications.test")}</button
            >
          </div>
        {/if}
      </div>

      <!-- Apprise Card -->
      <div class="service-card" class:enabled={appriseEnabled}>
        <div class="service-header">
          <div class="service-icon apprise-icon">
            <i class="bi bi-lightning-fill"></i>
          </div>
          <div class="service-info">
            <h3>{tr("settings.notifications.apprise")}</h3>
            <p>{tr("settings.notifications.apprise_desc")}</p>
          </div>
          <label class="toggle" aria-label="Enable Apprise">
            <input type="checkbox" bind:checked={appriseEnabled} />
            <span class="toggle-slider"></span>
          </label>
        </div>
        {#if appriseEnabled}
          <div class="service-config">
            <div class="config-grid">
              <div class="config-field full">
                <span class="field-label"
                  >{tr("settings.notifications.apprise_url")}</span
                ><input
                  type="url"
                  bind:value={appriseUrl}
                  placeholder="http://localhost:8000/notify"
                />
              </div>
            </div>
            <button
              class="btn-test"
              onclick={() => testService("apprise")}
              disabled={testing}
              ><i class="bi bi-send"></i>
              {tr("settings.notifications.test")}</button
            >
          </div>
        {/if}
      </div>
    </div>

    <!-- Events Card -->
    <div class="events-card">
      <div class="events-header">
        <div class="events-icon"><i class="bi bi-lightning-charge"></i></div>
        <div class="events-info">
          <h3>{tr("settings.notifications.events")}</h3>
          <p>{tr("settings.notifications.events_desc")}</p>
        </div>
      </div>
      <div class="events-grid">
        <label class="event-item"
          ><input type="checkbox" bind:checked={notifyOnShare} /><i
            class="bi bi-share"
          ></i><span>{tr("settings.notifications.on_share")}</span></label
        >
        <label class="event-item"
          ><input type="checkbox" bind:checked={notifyOnComment} /><i
            class="bi bi-chat"
          ></i><span>{tr("settings.notifications.on_comment")}</span></label
        >
        <label class="event-item"
          ><input type="checkbox" bind:checked={notifyOnUpload} /><i
            class="bi bi-cloud-upload"
          ></i><span>{tr("settings.notifications.on_upload")}</span></label
        >
        <label class="event-item"
          ><input type="checkbox" bind:checked={notifyOnBackup} /><i
            class="bi bi-archive"
          ></i><span>{tr("settings.notifications.on_backup")}</span></label
        >
        <label class="event-item"
          ><input type="checkbox" bind:checked={notifyOnLogin} /><i
            class="bi bi-box-arrow-in-right"
          ></i><span>{tr("settings.notifications.on_login")}</span></label
        >
        <label class="event-item"
          ><input type="checkbox" bind:checked={notifyOnError} /><i
            class="bi bi-exclamation-triangle"
          ></i><span>{tr("settings.notifications.on_error")}</span></label
        >
      </div>
    </div>

    <div class="save-section">
      <button class="btn-save" onclick={saveSettings} disabled={saving}>
        {#if saving}<span class="spinner-small"></span>{:else}<i
            class="bi bi-check-lg"
          ></i>{/if}
        {tr("common.save")}
      </button>
    </div>
  </div>
{/if}

<style>
  .loading-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 300px;
  }
  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #e5e7eb;
    border-top-color: #3b82f6;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  .notifications-settings {
    max-width: 100%;
  }
  .services-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    margin-bottom: 1rem;
  }
  @media (max-width: 900px) {
    .services-grid {
      grid-template-columns: 1fr;
    }
  }
  .service-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    overflow: hidden;
    transition: all 0.2s;
  }
  :global(.dark) .service-card {
    background: #1f2937;
    border-color: #374151;
  }
  .service-card.enabled {
    border-color: #3b82f6;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2);
  }
  .service-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
  }
  .service-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }
  .email-icon {
    background: #dbeafe;
    color: #3b82f6;
  }
  :global(.dark) .email-icon {
    background: rgba(59, 130, 246, 0.2);
  }
  .gotify-icon {
    background: #fef3c7;
    color: #f59e0b;
  }
  :global(.dark) .gotify-icon {
    background: rgba(245, 158, 11, 0.2);
  }
  .ntfy-icon {
    background: #dcfce7;
    color: #22c55e;
  }
  :global(.dark) .ntfy-icon {
    background: rgba(34, 197, 94, 0.2);
  }
  .apprise-icon {
    background: #fce7f3;
    color: #ec4899;
  }
  :global(.dark) .apprise-icon {
    background: rgba(236, 72, 153, 0.2);
  }
  .service-info {
    flex: 1;
    min-width: 0;
  }
  .service-info h3 {
    font-size: 0.9375rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.125rem;
  }
  :global(.dark) .service-info h3 {
    color: #f9fafb;
  }
  .service-info p {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .service-info p {
    color: #9ca3af;
  }
  .toggle {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    flex-shrink: 0;
  }
  .toggle input {
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
    background: #d1d5db;
    border-radius: 24px;
    transition: 0.2s;
  }
  :global(.dark) .toggle-slider {
    background: #4b5563;
  }
  .toggle-slider::before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background: white;
    border-radius: 50%;
    transition: 0.2s;
  }
  .toggle input:checked + .toggle-slider {
    background: #3b82f6;
  }
  .toggle input:checked + .toggle-slider::before {
    transform: translateX(20px);
  }
  .service-config {
    padding: 0 1rem 1rem;
    border-top: 1px solid #e5e7eb;
    margin-top: -0.5rem;
    padding-top: 1rem;
  }
  :global(.dark) .service-config {
    border-top-color: #374151;
  }
  .config-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }
  .config-field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .config-field.full {
    grid-column: span 2;
  }
  .field-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: #6b7280;
  }
  :global(.dark) .field-label {
    color: #9ca3af;
  }
  .config-field input,
  .config-field select {
    padding: 0.5rem 0.75rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    background: white;
    color: #111827;
  }
  :global(.dark) .config-field input,
  :global(.dark) .config-field select {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }
  .config-field input:focus,
  .config-field select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.15);
  }
  .test-section {
    display: flex;
    gap: 0.5rem;
  }
  .test-section input {
    flex: 1;
    padding: 0.5rem 0.75rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    background: white;
    color: #111827;
  }
  :global(.dark) .test-section input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }
  .btn-test {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.875rem;
    background: #f3f4f6;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: #374151;
    cursor: pointer;
    transition: all 0.15s;
  }
  :global(.dark) .btn-test {
    background: #374151;
    border-color: #4b5563;
    color: #d1d5db;
  }
  .btn-test:hover:not(:disabled) {
    background: #e5e7eb;
  }
  :global(.dark) .btn-test:hover:not(:disabled) {
    background: #4b5563;
  }
  .btn-test:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .events-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1rem;
    margin-bottom: 1rem;
  }
  :global(.dark) .events-card {
    background: #1f2937;
    border-color: #374151;
  }
  .events-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  .events-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    background: #ede9fe;
    color: #8b5cf6;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
  }
  :global(.dark) .events-icon {
    background: rgba(139, 92, 246, 0.2);
  }
  .events-info h3 {
    font-size: 0.9375rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.125rem;
  }
  :global(.dark) .events-info h3 {
    color: #f9fafb;
  }
  .events-info p {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .events-info p {
    color: #9ca3af;
  }
  .events-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
  }
  @media (max-width: 768px) {
    .events-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
  @media (max-width: 500px) {
    .events-grid {
      grid-template-columns: 1fr;
    }
  }
  .event-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: background 0.15s;
  }
  :global(.dark) .event-item {
    background: #374151;
  }
  .event-item:hover {
    background: #f3f4f6;
  }
  :global(.dark) .event-item:hover {
    background: #4b5563;
  }
  .event-item input[type="checkbox"] {
    width: 1rem;
    height: 1rem;
    border-radius: 0.25rem;
    accent-color: #3b82f6;
  }
  .event-item i {
    font-size: 0.875rem;
    color: #6b7280;
  }
  :global(.dark) .event-item i {
    color: #9ca3af;
  }
  .event-item span {
    font-size: 0.8125rem;
    color: #374151;
  }
  :global(.dark) .event-item span {
    color: #e5e7eb;
  }
  .save-section {
    display: flex;
    justify-content: flex-end;
  }
  .btn-save {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-save:hover:not(:disabled) {
    background: #2563eb;
  }
  .btn-save:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  .spinner-small {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
</style>
