<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import QRCode from "qrcode";
  import { users, auth as authApi } from "../../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let twoFAEnabled = $state(false);
  let twoFASetup = $state(null);
  let qrCodeDataUrl = $state("");
  let showSetup = $state(false);
  let verificationCode = $state("");
  let loading = $state(false);
  let pageLoading = $state(true);
  let error = $state("");
  let success = $state("");

  let showPasswordChange = $state(false);
  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");

  onMount(async () => {
    await loadSecurityStatus();
  });

  async function loadSecurityStatus() {
    pageLoading = true;
    try {
      const user = await users.me();
      twoFAEnabled = user.totp_enabled === 1 || user.totp_enabled === true;
    } catch (e) {
      console.error("Failed to load security status:", e);
    } finally {
      pageLoading = false;
    }
  }

  async function setup2FA() {
    loading = true;
    error = "";
    success = "";
    try {
      twoFASetup = await authApi.setup2FA();
      qrCodeDataUrl = await QRCode.toDataURL(twoFASetup.qr_code_url, {
        width: 200,
        margin: 2,
        color: { dark: "#000000", light: "#FFFFFF" },
      });
      showSetup = true;
    } catch (e) {
      console.error("2FA setup error:", e);
      error = tr("connectionErrorBackendRunning");
    } finally {
      loading = false;
    }
  }

  async function enable2FA() {
    if (!verificationCode || verificationCode.length !== 6) {
      error = tr("enterValid6DigitCode2");
      return;
    }
    loading = true;
    error = "";
    try {
      await authApi.enable2FA(verificationCode);
      success = tr("twoFAEnabledSuccess2");
      twoFAEnabled = true;
      showSetup = false;
      twoFASetup = null;
      verificationCode = "";
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      console.error("2FA enable error:", e);
      error = e.message || tr("invalidVerificationCode2");
    } finally {
      loading = false;
    }
  }

  async function disable2FA() {
    if (!confirm(tr("areSureDisable2FA"))) return;
    loading = true;
    error = "";
    try {
      await authApi.disable2FA();
      success = tr("twoFADisabledSuccess2");
      twoFAEnabled = false;
      showSetup = false;
      twoFASetup = null;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      console.error("2FA disable error:", e);
      error = tr("failedToDisable2FA2");
    } finally {
      loading = false;
    }
  }

  function cancelSetup() {
    showSetup = false;
    twoFASetup = null;
    qrCodeDataUrl = "";
    verificationCode = "";
    error = "";
  }

  async function changePassword() {
    if (!currentPassword || !newPassword || !confirmPassword) {
      error = tr("fillAllFields");
      return;
    }
    if (newPassword !== confirmPassword) {
      error = tr("passwordsDoNotMatch");
      return;
    }
    if (newPassword.length < 8) {
      error = tr("passwordTooShort");
      return;
    }
    loading = true;
    error = "";
    try {
      await authApi.changePassword(currentPassword, newPassword);
      success = tr("passwordChangedSuccess");
      showPasswordChange = false;
      currentPassword = "";
      newPassword = "";
      confirmPassword = "";
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || tr("failedToChangePassword");
    } finally {
      loading = false;
    }
  }
</script>

{#if pageLoading}
  <div class="flex justify-center p-16"><div class="spinner"></div></div>
{:else}
  {#if error}
    <div class="alert error">
      <i class="bi bi-exclamation-circle-fill"></i>
      {error}
    </div>
  {/if}
  {#if success}
    <div class="alert success">
      <i class="bi bi-check-circle-fill"></i>
      {success}
    </div>
  {/if}

  <div class="settings-grid">
    <!-- 2FA Card -->
    <div class="card col-span-full">
      <div class="card-header">
        <div class="card-icon green">
          <i class="bi bi-shield-lock-fill"></i>
        </div>
        <div class="card-title">
          <h3>{tr("twoFactorAuthentication")}</h3>
          <p>{tr("addExtraLayerOfSecurity")}</p>
        </div>
        <span class="badge {twoFAEnabled ? 'success' : 'error'}">
          {twoFAEnabled ? tr("enabled") : tr("disabled")}
        </span>
      </div>

      {#if !showSetup}
        <div class="flex flex-col gap-4">
          <div class="status-box">
            <i
              class="bi {twoFAEnabled
                ? 'bi-shield-fill-check'
                : 'bi-shield-fill-x'} text-3xl {twoFAEnabled
                ? 'text-green-500'
                : 'text-red-600'}"
            ></i>
            <div class="flex flex-col gap-0.5">
              <strong class="text-sm text-gray-900 dark:text-gray-50"
                >{twoFAEnabled
                  ? tr("twoFAIsActive")
                  : tr("twoFAIsNotActive")}</strong
              >
              <span class="text-[0.8125rem] text-gray-500 dark:text-gray-400"
                >{twoFAEnabled
                  ? tr("yourAccountIsProtected")
                  : tr("enableTwoFAForBetterSecurity")}</span
              >
            </div>
          </div>
          <div class="flex gap-2">
            {#if twoFAEnabled}
              <button
                class="btn btn-danger"
                onclick={disable2FA}
                disabled={loading}
              >
                <i class="bi bi-shield-x"></i>
                {tr("disable2FA")}
              </button>
            {:else}
              <button
                class="btn btn-success"
                onclick={setup2FA}
                disabled={loading}
              >
                <i class="bi bi-shield-plus"></i>
                {tr("enable2FA")}
              </button>
            {/if}
          </div>
        </div>
      {:else}
        <div
          class="grid grid-cols-[auto_1fr] gap-8 items-start max-sm:grid-cols-1"
        >
          <div class="flex flex-col items-center gap-4 max-sm:order-1">
            {#if qrCodeDataUrl}
              <div class="qr-code">
                <img
                  src={qrCodeDataUrl}
                  alt="2FA QR Code"
                  class="block w-[180px] h-[180px]"
                />
              </div>
            {/if}
            <div class="text-center">
              <span class="block text-xs text-gray-500 mb-1"
                >{tr("manualEntryKey")}:</span
              >
              <code class="secret-code">{twoFASetup?.secret || ""}</code>
            </div>
          </div>
          <div class="flex flex-col gap-4 max-sm:order-2">
            <label
              for="verification-code"
              class="text-sm font-medium text-gray-700 dark:text-gray-300"
              >{tr("enterVerificationCode")}</label
            >
            <input
              id="verification-code"
              type="text"
              class="verification-input"
              placeholder="000000"
              maxlength="6"
              bind:value={verificationCode}
            />
            <div class="flex gap-2 justify-end">
              <button
                class="btn btn-secondary"
                onclick={cancelSetup}
                disabled={loading}>{tr("cancel")}</button
              >
              <button
                class="btn btn-success"
                onclick={enable2FA}
                disabled={loading || verificationCode.length !== 6}
              >
                {loading ? tr("verifying") : tr("verify")}
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Password Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon amber"><i class="bi bi-key-fill"></i></div>
        <div class="card-title">
          <h3>{tr("password")}</h3>
          <p>{tr("changeYourPassword")}</p>
        </div>
      </div>
      {#if !showPasswordChange}
        <div class="flex flex-col gap-4">
          <div
            class="flex items-center gap-2 text-gray-500 dark:text-gray-400 text-sm"
          >
            <i class="bi bi-asterisk"></i>
            <span>{tr("lastPasswordChange")}: {tr("unknown")}</span>
          </div>
          <button
            class="btn btn-secondary"
            onclick={() => (showPasswordChange = true)}
          >
            <i class="bi bi-pencil"></i>
            {tr("changePassword")}
          </button>
        </div>
      {:else}
        <div class="flex flex-col gap-4">
          <div class="input-group">
            <label for="current-pw">{tr("currentPassword")}</label>
            <input
              id="current-pw"
              type="password"
              class="input"
              bind:value={currentPassword}
            />
          </div>
          <div class="input-group">
            <label for="new-pw">{tr("newPassword")}</label>
            <input
              id="new-pw"
              type="password"
              class="input"
              bind:value={newPassword}
            />
          </div>
          <div class="input-group">
            <label for="confirm-pw">{tr("confirmPassword")}</label>
            <input
              id="confirm-pw"
              type="password"
              class="input"
              bind:value={confirmPassword}
            />
          </div>
          <div class="flex gap-2 justify-end mt-2">
            <button
              class="btn btn-secondary"
              onclick={() => {
                showPasswordChange = false;
                error = "";
              }}>{tr("cancel")}</button
            >
            <button
              class="btn btn-success"
              onclick={changePassword}
              disabled={loading}
            >
              {loading ? tr("saving") : tr("savePassword")}
            </button>
          </div>
        </div>
      {/if}
    </div>

    <!-- Sessions Card -->
    <div class="card">
      <div class="card-header">
        <div class="card-icon blue"><i class="bi bi-laptop"></i></div>
        <div class="card-title">
          <h3>{tr("activeSessions")}</h3>
          <p>{tr("manageActiveSessions")}</p>
        </div>
      </div>
      <div class="flex flex-col gap-4">
        <div class="session-item">
          <div class="session-icon">
            <i class="bi bi-display"></i>
          </div>
          <div class="flex-1 flex flex-col gap-0.5">
            <span class="text-sm font-medium text-gray-900 dark:text-gray-50"
              >{tr("currentSession")}</span
            >
            <span class="text-xs text-gray-500 dark:text-gray-400"
              >Windows · Chrome · {tr("justNow")}</span
            >
          </div>
          <span class="badge success">{tr("current")}</span>
        </div>
        <button class="btn btn-secondary w-full">
          <i class="bi bi-box-arrow-right"></i>
          {tr("logoutAllOtherSessions")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .spinner {
    width: 36px;
    height: 36px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .alert {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
    font-weight: 500;
  }
  .alert.success {
    background: #dcfce7;
    color: #166534;
  }
  .alert.error {
    background: #fee2e2;
    color: #991b1b;
  }
  :global(.dark) .alert.success {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }
  :global(.dark) .alert.error {
    background: rgba(220, 38, 38, 0.2);
    color: #fca5a5;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
    gap: 1.5rem;
  }
  .col-span-full {
    grid-column: 1 / -1;
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
    flex-wrap: wrap;
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
  .card-icon.amber {
    background: #fef3c7;
    color: #d97706;
  }
  .card-icon.blue {
    background: #dbeafe;
    color: #2563eb;
  }
  :global(.dark) .card-icon.green {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }
  :global(.dark) .card-icon.amber {
    background: rgba(251, 191, 36, 0.2);
    color: #fbbf24;
  }
  :global(.dark) .card-icon.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #3b82f6;
  }

  .card-title {
    flex: 1;
    min-width: 200px;
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

  .badge {
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    white-space: nowrap;
  }
  .badge.success {
    background: #dcfce7;
    color: #166534;
  }
  .badge.error {
    background: #fee2e2;
    color: #991b1b;
  }
  :global(.dark) .badge.success {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }
  :global(.dark) .badge.error {
    background: rgba(220, 38, 38, 0.2);
    color: #fca5a5;
  }

  .status-box {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .status-box {
    background: #374151;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }
  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .btn-success {
    background: #22c55e;
    color: white;
  }
  .btn-success:hover:not(:disabled) {
    background: #16a34a;
  }
  .btn-danger {
    background: #dc2626;
    color: white;
  }
  .btn-danger:hover:not(:disabled) {
    background: #b91c1c;
  }
  .btn-secondary {
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #d1d5db;
  }
  .btn-secondary:hover:not(:disabled) {
    background: #e5e7eb;
  }
  :global(.dark) .btn-secondary {
    background: #374151;
    color: #d1d5db;
    border-color: #4b5563;
  }
  :global(.dark) .btn-secondary:hover:not(:disabled) {
    background: #4b5563;
  }

  .qr-code {
    padding: 1rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
  }
  .secret-code {
    font-size: 0.8125rem;
    font-family: monospace;
    background: #f3f4f6;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
  }
  :global(.dark) .secret-code {
    background: #374151;
  }

  .verification-input {
    padding: 0.75rem 1rem;
    font-size: 1.5rem;
    text-align: center;
    letter-spacing: 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    background: #f9fafb;
  }
  .verification-input:focus {
    outline: none;
    border-color: #22c55e;
  }
  :global(.dark) .verification-input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
  .input-group label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: #374151;
  }
  :global(.dark) .input-group label {
    color: #d1d5db;
  }
  .input {
    padding: 0.625rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    background: #f9fafb;
    font-size: 0.875rem;
  }
  .input:focus {
    outline: none;
    border-color: #22c55e;
  }
  :global(.dark) .input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  .session-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .session-item {
    background: #374151;
  }
  .session-icon {
    width: 36px;
    height: 36px;
    background: #dbeafe;
    color: #2563eb;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  :global(.dark) .session-icon {
    background: rgba(59, 130, 246, 0.2);
    color: #3b82f6;
  }
</style>
