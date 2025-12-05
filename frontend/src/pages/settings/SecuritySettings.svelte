<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import QRCode from "qrcode";

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

  // Password change
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
      const response = await fetch("http://localhost:8080/api/users/me", {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });
      if (response.ok) {
        const user = await response.json();
        twoFAEnabled = user.totp_enabled === 1 || user.totp_enabled === true;
      }
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
      const response = await fetch("http://localhost:8080/api/auth/2fa/setup", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          "Content-Type": "application/json",
        },
      });
      if (response.ok) {
        twoFASetup = await response.json();
        qrCodeDataUrl = await QRCode.toDataURL(twoFASetup.qr_code_url, {
          width: 200,
          margin: 2,
          color: { dark: "#000000", light: "#FFFFFF" },
        });
        showSetup = true;
      } else {
        error = tr("failedToGenerate2FASecret");
      }
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
      const response = await fetch(
        "http://localhost:8080/api/auth/2fa/enable",
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ totp_code: verificationCode }),
        }
      );
      if (response.ok) {
        success = tr("twoFAEnabledSuccess2");
        twoFAEnabled = true;
        showSetup = false;
        twoFASetup = null;
        verificationCode = "";
        setTimeout(() => (success = ""), 5000);
      } else {
        const data = await response.json();
        error = data.error || tr("invalidVerificationCode2");
      }
    } catch (e) {
      console.error("2FA enable error:", e);
      error = tr("failedToEnable2FA");
    } finally {
      loading = false;
    }
  }

  async function disable2FA() {
    if (!confirm(tr("areSureDisable2FA"))) return;
    loading = true;
    error = "";
    try {
      const response = await fetch(
        "http://localhost:8080/api/auth/2fa/disable",
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
          },
        }
      );
      if (response.ok) {
        success = tr("twoFADisabledSuccess2");
        twoFAEnabled = false;
        showSetup = false;
        twoFASetup = null;
        setTimeout(() => (success = ""), 5000);
      } else {
        error = tr("failedToDisable2FA2");
      }
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
      const response = await fetch(
        "http://localhost:8080/api/auth/change-password",
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${localStorage.getItem("authToken")}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            current_password: currentPassword,
            new_password: newPassword,
          }),
        }
      );
      if (response.ok) {
        success = tr("passwordChangedSuccess");
        showPasswordChange = false;
        currentPassword = "";
        newPassword = "";
        confirmPassword = "";
        setTimeout(() => (success = ""), 5000);
      } else {
        const data = await response.json();
        error = data.error || tr("failedToChangePassword");
      }
    } catch (e) {
      error = tr("connectionError");
    } finally {
      loading = false;
    }
  }
</script>

{#if pageLoading}
  <div class="loading-container"><div class="spinner"></div></div>
{:else}
  {#if error}
    <div class="toast error">
      <i class="bi bi-exclamation-circle-fill"></i>
      {error}
    </div>
  {/if}
  {#if success}
    <div class="toast success">
      <i class="bi bi-check-circle-fill"></i>
      {success}
    </div>
  {/if}

  <div class="security-grid">
    <!-- 2FA Status Card -->
    <div class="card featured">
      <div class="card-header">
        <div class="card-icon green">
          <i class="bi bi-shield-lock-fill"></i>
        </div>
        <div class="card-title">
          <h3>{tr("twoFactorAuthentication")}</h3>
          <p>{tr("addExtraLayerOfSecurity")}</p>
        </div>
        <span class="status-badge" class:enabled={twoFAEnabled}>
          {twoFAEnabled ? tr("enabled") : tr("disabled")}
        </span>
      </div>

      {#if !showSetup}
        <div class="card-content">
          <div class="security-info">
            <i
              class="bi {twoFAEnabled
                ? 'bi-shield-fill-check'
                : 'bi-shield-fill-x'} security-icon"
              class:enabled={twoFAEnabled}
            ></i>
            <div class="security-text">
              <strong
                >{twoFAEnabled
                  ? tr("twoFAIsActive")
                  : tr("twoFAIsNotActive")}</strong
              >
              <span
                >{twoFAEnabled
                  ? tr("yourAccountIsProtected")
                  : tr("enableTwoFAForBetterSecurity")}</span
              >
            </div>
          </div>
          <div class="card-actions">
            {#if twoFAEnabled}
              <button
                class="btn-danger"
                onclick={disable2FA}
                disabled={loading}
              >
                <i class="bi bi-shield-x"></i>
                {tr("disable2FA")}
              </button>
            {:else}
              <button class="btn-primary" onclick={setup2FA} disabled={loading}>
                <i class="bi bi-shield-plus"></i>
                {tr("enable2FA")}
              </button>
            {/if}
          </div>
        </div>
      {:else}
        <!-- 2FA Setup -->
        <div class="setup-container">
          <div class="qr-section">
            {#if qrCodeDataUrl}
              <div class="qr-wrapper">
                <img src={qrCodeDataUrl} alt="2FA QR Code" class="qr-code" />
              </div>
            {/if}
            <div class="secret-key">
              <span class="label">{tr("manualEntryKey")}:</span>
              <code>{twoFASetup?.secret || ""}</code>
            </div>
          </div>
          <div class="verify-section">
            <label for="verification-code">{tr("enterVerificationCode")}</label>
            <input
              id="verification-code"
              type="text"
              class="code-input"
              placeholder="000000"
              maxlength="6"
              bind:value={verificationCode}
            />
            <div class="setup-actions">
              <button
                class="btn-secondary"
                onclick={cancelSetup}
                disabled={loading}>{tr("cancel")}</button
              >
              <button
                class="btn-primary"
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
        <div class="card-content">
          <div class="password-info">
            <i class="bi bi-asterisk"></i>
            <span>{tr("lastPasswordChange")}: {tr("unknown")}</span>
          </div>
          <button
            class="btn-secondary"
            onclick={() => (showPasswordChange = true)}
          >
            <i class="bi bi-pencil"></i>
            {tr("changePassword")}
          </button>
        </div>
      {:else}
        <div class="password-form">
          <div class="form-group">
            <label for="current-pw">{tr("currentPassword")}</label>
            <input
              id="current-pw"
              type="password"
              class="form-input"
              bind:value={currentPassword}
            />
          </div>
          <div class="form-group">
            <label for="new-pw">{tr("newPassword")}</label>
            <input
              id="new-pw"
              type="password"
              class="form-input"
              bind:value={newPassword}
            />
          </div>
          <div class="form-group">
            <label for="confirm-pw">{tr("confirmPassword")}</label>
            <input
              id="confirm-pw"
              type="password"
              class="form-input"
              bind:value={confirmPassword}
            />
          </div>
          <div class="form-actions">
            <button
              class="btn-secondary"
              onclick={() => {
                showPasswordChange = false;
                error = "";
              }}>{tr("cancel")}</button
            >
            <button
              class="btn-primary"
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
      <div class="card-content">
        <div class="session-item current">
          <div class="session-icon"><i class="bi bi-display"></i></div>
          <div class="session-info">
            <span class="session-device">{tr("currentSession")}</span>
            <span class="session-location"
              >Windows · Chrome · {tr("justNow")}</span
            >
          </div>
          <span class="current-badge">{tr("current")}</span>
        </div>
        <button class="btn-outline full-width">
          <i class="bi bi-box-arrow-right"></i>
          {tr("logoutAllOtherSessions")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .loading-container {
    display: flex;
    justify-content: center;
    padding: 4rem;
  }
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

  /* Toast */
  .toast {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
    font-weight: 500;
  }
  .toast.success {
    background: #dcfce7;
    color: #166534;
  }
  .toast.error {
    background: #fee2e2;
    color: #991b1b;
  }
  :global(.dark) .toast.success {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }
  :global(.dark) .toast.error {
    background: rgba(220, 38, 38, 0.2);
    color: #fca5a5;
  }

  /* Grid */
  .security-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
    gap: 1.5rem;
  }

  /* Card */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
  }
  .card.featured {
    grid-column: 1 / -1;
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
    background: rgba(245, 158, 11, 0.2);
    color: #fbbf24;
  }
  :global(.dark) .card-icon.blue {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
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

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    background: #fee2e2;
    color: #991b1b;
  }
  .status-badge.enabled {
    background: #dcfce7;
    color: #166534;
  }
  :global(.dark) .status-badge {
    background: rgba(220, 38, 38, 0.2);
    color: #fca5a5;
  }
  :global(.dark) .status-badge.enabled {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }

  .card-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .card-actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Security Info */
  .security-info {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .security-info {
    background: #374151;
  }
  .security-icon {
    font-size: 2rem;
    color: #dc2626;
  }
  .security-icon.enabled {
    color: #22c55e;
  }
  .security-text {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .security-text strong {
    font-size: 0.875rem;
    color: #111827;
  }
  .security-text span {
    font-size: 0.8125rem;
    color: #6b7280;
  }
  :global(.dark) .security-text strong {
    color: #f9fafb;
  }
  :global(.dark) .security-text span {
    color: #9ca3af;
  }

  /* 2FA Setup */
  .setup-container {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 2rem;
    align-items: start;
  }
  .qr-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }
  .qr-wrapper {
    padding: 1rem;
    background: white;
    border-radius: 0.5rem;
    border: 1px solid #e5e7eb;
  }
  .qr-code {
    display: block;
    width: 180px;
    height: 180px;
  }
  .secret-key {
    text-align: center;
  }
  .secret-key .label {
    display: block;
    font-size: 0.75rem;
    color: #6b7280;
    margin-bottom: 0.25rem;
  }
  .secret-key code {
    font-size: 0.8125rem;
    background: #f3f4f6;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-family: monospace;
  }
  :global(.dark) .secret-key code {
    background: #374151;
  }

  .verify-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .verify-section label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
  }
  :global(.dark) .verify-section label {
    color: #d1d5db;
  }
  .code-input {
    padding: 0.75rem 1rem;
    font-size: 1.5rem;
    text-align: center;
    letter-spacing: 0.5rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    background: #f9fafb;
  }
  .code-input:focus {
    outline: none;
    border-color: #22c55e;
  }
  :global(.dark) .code-input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }
  .setup-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
  }

  /* Password */
  .password-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #6b7280;
    font-size: 0.875rem;
  }
  :global(.dark) .password-info {
    color: #9ca3af;
  }

  .password-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
  .form-group label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: #374151;
  }
  :global(.dark) .form-group label {
    color: #d1d5db;
  }
  .form-input {
    padding: 0.625rem 0.75rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    background: #f9fafb;
    font-size: 0.875rem;
  }
  .form-input:focus {
    outline: none;
    border-color: #22c55e;
  }
  :global(.dark) .form-input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }
  .form-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
    margin-top: 0.5rem;
  }

  /* Session */
  .session-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
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
    color: #60a5fa;
  }
  .session-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }
  .session-device {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }
  .session-location {
    font-size: 0.75rem;
    color: #6b7280;
  }
  :global(.dark) .session-device {
    color: #f9fafb;
  }
  :global(.dark) .session-location {
    color: #9ca3af;
  }
  .current-badge {
    padding: 0.125rem 0.5rem;
    background: #dcfce7;
    color: #166534;
    border-radius: 9999px;
    font-size: 0.6875rem;
    font-weight: 600;
  }
  :global(.dark) .current-badge {
    background: rgba(34, 197, 94, 0.2);
    color: #86efac;
  }

  /* Buttons */
  .btn-primary {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #22c55e;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 500;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-primary:hover:not(:disabled) {
    background: #16a34a;
  }
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    font-weight: 500;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.15s;
  }
  .btn-secondary:hover {
    background: #e5e7eb;
  }
  :global(.dark) .btn-secondary {
    background: #374151;
    color: #e5e7eb;
    border-color: #4b5563;
  }
  :global(.dark) .btn-secondary:hover {
    background: #4b5563;
  }

  .btn-danger {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #dc2626;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 500;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-danger:hover:not(:disabled) {
    background: #b91c1c;
  }
  .btn-danger:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-outline {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: transparent;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    font-weight: 500;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.15s;
  }
  .btn-outline:hover {
    background: #f3f4f6;
  }
  .btn-outline.full-width {
    width: 100%;
  }
  :global(.dark) .btn-outline {
    color: #e5e7eb;
    border-color: #4b5563;
  }
  :global(.dark) .btn-outline:hover {
    background: #374151;
  }

  @media (max-width: 640px) {
    .security-grid {
      grid-template-columns: 1fr;
    }
    .setup-container {
      grid-template-columns: 1fr;
    }
    .qr-section {
      order: 1;
    }
    .verify-section {
      order: 2;
    }
  }
</style>
