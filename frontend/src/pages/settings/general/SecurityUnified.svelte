<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UITextarea from "../../../components/ui/UITextarea.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import QRCode from "qrcode";
  import { users, auth as authApi, admin } from "../../../lib/api.js";
  import {
    success as toastSuccess,
    error as toastError,
  } from "../../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Loading states
  let loading = $state(true);
  let saving2FA = $state(false);
  let savingPassword = $state(false);
  let savingPolicies = $state(false);

  // === 2FA & Authentication ===
  let twoFAEnabled = $state(false);
  let twoFASetup = $state(null);
  let qrCodeDataUrl = $state("");
  let showSetup = $state(false);
  let verificationCode = $state("");
  let error2FA = $state("");
  let success2FA = $state("");

  // === Password Management ===
  let showPasswordChange = $state(false);
  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");
  let errorPassword = $state("");
  let successPassword = $state("");

  // === Active Sessions ===
  // (Placeholder for now, can be expanded later)

  // === Security Policies (Admin) ===
  // Password Policy
  let minPasswordLength = $state(8);
  let requireUppercase = $state(true);
  let requireLowercase = $state(true);
  let requireNumbers = $state(true);
  let requireSpecialChars = $state(false);
  let passwordExpiryDays = $state(0);
  let preventPasswordReuse = $state(5);

  // 2FA Policy
  let require2FA = $state(false);
  let enforce2FAForAdmins = $state(true);
  let allowed2FAMethods = $state(["totp"]);

  // Session Policy
  let sessionTimeoutMinutes = $state(480);
  let maxConcurrentSessions = $state(5);
  let enforceReauthForSensitive = $state(true);

  // Login Security
  let maxLoginAttempts = $state(5);
  let lockoutDurationMinutes = $state(15);
  let enableCaptcha = $state(false);
  let captchaAfterFailures = $state(3);

  // IP Security
  let enableIPWhitelist = $state(false);
  let ipWhitelist = $state("");
  let enableIPBlacklist = $state(false);
  let ipBlacklist = $state("");
  let blockTorExits = $state(false);

  onMount(async () => {
    await Promise.all([loadSecurityStatus(), loadSecurityPolicy()]);
  });

  // === 2FA Functions ===
  async function loadSecurityStatus() {
    try {
      const user = await users.me();
      twoFAEnabled = user.totp_enabled === 1 || user.totp_enabled === true;
    } catch (e) {
      console.error("Failed to load security status:", e);
    }
  }

  async function setup2FA() {
    saving2FA = true;
    error2FA = "";
    success2FA = "";
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
      error2FA = tr("connectionErrorBackendRunning");
    } finally {
      saving2FA = false;
    }
  }

  async function enable2FA() {
    if (!verificationCode || verificationCode.length !== 6) {
      error2FA = tr("enterValid6DigitCode2");
      return;
    }
    saving2FA = true;
    error2FA = "";
    try {
      await authApi.enable2FA(verificationCode);
      success2FA = tr("twoFAEnabledSuccess2");
      twoFAEnabled = true;
      showSetup = false;
      twoFASetup = null;
      verificationCode = "";
      setTimeout(() => (success2FA = ""), 5000);
    } catch (e) {
      console.error("2FA enable error:", e);
      error2FA = e.message || tr("invalidVerificationCode2");
    } finally {
      saving2FA = false;
    }
  }

  async function disable2FA() {
    if (!confirm(tr("areSureDisable2FA"))) return;
    saving2FA = true;
    error2FA = "";
    try {
      await authApi.disable2FA();
      success2FA = tr("twoFADisabledSuccess2");
      twoFAEnabled = false;
      showSetup = false;
      twoFASetup = null;
      setTimeout(() => (success2FA = ""), 5000);
    } catch (e) {
      console.error("2FA disable error:", e);
      error2FA = tr("failedToDisable2FA2");
    } finally {
      saving2FA = false;
    }
  }

  function cancelSetup() {
    showSetup = false;
    twoFASetup = null;
    qrCodeDataUrl = "";
    verificationCode = "";
    error2FA = "";
  }

  // === Password Functions ===
  async function changePassword() {
    if (!currentPassword || !newPassword || !confirmPassword) {
      errorPassword = tr("fillAllFields");
      return;
    }
    if (newPassword !== confirmPassword) {
      errorPassword = tr("passwordsDoNotMatch");
      return;
    }
    if (newPassword.length < 8) {
      errorPassword = tr("passwordTooShort");
      return;
    }
    savingPassword = true;
    errorPassword = "";
    try {
      await authApi.changePassword(currentPassword, newPassword);
      successPassword = tr("passwordChangedSuccess");
      showPasswordChange = false;
      currentPassword = "";
      newPassword = "";
      confirmPassword = "";
      setTimeout(() => (successPassword = ""), 5000);
    } catch (e) {
      errorPassword = e.message || tr("failedToChangePassword");
    } finally {
      savingPassword = false;
    }
  }

  // === Security Policy Functions ===
  async function loadSecurityPolicy() {
    try {
      const data = await admin.getSecurityPolicy();

      // Password Policy
      minPasswordLength = data.min_password_length || 8;
      requireUppercase = data.require_uppercase !== false;
      requireLowercase = data.require_lowercase !== false;
      requireNumbers = data.require_numbers !== false;
      requireSpecialChars = data.require_special_chars || false;
      passwordExpiryDays = data.password_expiry_days || 0;
      preventPasswordReuse = data.prevent_password_reuse || 5;

      // 2FA Policy
      require2FA = data.require_2fa || false;
      enforce2FAForAdmins = data.enforce_2fa_for_admins !== false;
      allowed2FAMethods = data.allowed_2fa_methods || ["totp"];

      // Session Policy
      sessionTimeoutMinutes = data.session_timeout_minutes || 480;
      maxConcurrentSessions = data.max_concurrent_sessions || 5;
      enforceReauthForSensitive = data.enforce_reauth_for_sensitive !== false;

      // Login Security
      maxLoginAttempts = data.max_login_attempts || 5;
      lockoutDurationMinutes = data.lockout_duration_minutes || 15;
      enableCaptcha = data.enable_captcha || false;
      captchaAfterFailures = data.captcha_after_failures || 3;

      // IP Security
      enableIPWhitelist = data.enable_ip_whitelist || false;
      ipWhitelist = (data.ip_whitelist || []).join("\n");
      enableIPBlacklist = data.enable_ip_blacklist || false;
      ipBlacklist = (data.ip_blacklist || []).join("\n");
      blockTorExits = data.block_tor_exits || false;
    } catch (err) {
      console.error("Failed to load security policy:", err);
    } finally {
      loading = false;
    }
  }

  async function saveSecurityPolicies() {
    savingPolicies = true;
    try {
      await admin.updateSecurityPolicy({
        // Password Policy
        min_password_length: minPasswordLength,
        require_uppercase: requireUppercase,
        require_lowercase: requireLowercase,
        require_numbers: requireNumbers,
        require_special_chars: requireSpecialChars,
        password_expiry_days: passwordExpiryDays,
        prevent_password_reuse: preventPasswordReuse,

        // 2FA Policy
        require_2fa: require2FA,
        enforce_2fa_for_admins: enforce2FAForAdmins,
        allowed_2fa_methods: allowed2FAMethods,

        // Session Policy
        session_timeout_minutes: sessionTimeoutMinutes,
        max_concurrent_sessions: maxConcurrentSessions,
        enforce_reauth_for_sensitive: enforceReauthForSensitive,

        // Login Security
        max_login_attempts: maxLoginAttempts,
        lockout_duration_minutes: lockoutDurationMinutes,
        enable_captcha: enableCaptcha,
        captcha_after_failures: captchaAfterFailures,

        // IP Security
        enable_ip_whitelist: enableIPWhitelist,
        ip_whitelist: ipWhitelist
          .split("\n")
          .map((ip) => ip.trim())
          .filter((ip) => ip),
        enable_ip_blacklist: enableIPBlacklist,
        ip_blacklist: ipBlacklist
          .split("\n")
          .map((ip) => ip.trim())
          .filter((ip) => ip),
        block_tor_exits: blockTorExits,
      });

      toastSuccess(tr("settings.security.saved"));
    } catch (err) {
      toastError(err.message || tr("settings.security.save_error"));
    } finally {
      savingPolicies = false;
    }
  }

  function toggle2FAMethod(method) {
    if (allowed2FAMethods.includes(method)) {
      allowed2FAMethods = allowed2FAMethods.filter((m) => m !== method);
    } else {
      allowed2FAMethods = [...allowed2FAMethods, method];
    }
  }
</script>

{#if loading}
  <div class="flex justify-center p-16"><div class="spinner"></div></div>
{:else}
  <!-- Page Header -->
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-50 mb-2">
      {tr("security")}
    </h2>
    <p class="text-sm text-gray-500 dark:text-gray-400">
      Manage authentication, passwords, and security policies
    </p>
  </div>

  <!-- ============================================ -->
  <!-- SECTION 1: 2FA & Authentication -->
  <!-- ============================================ -->
  <div class="mb-8">
    <h3
      class="text-lg font-semibold text-gray-900 dark:text-gray-50 mb-4 flex items-center gap-2"
    >
      <i class="bi bi-shield-lock-fill text-green-600"></i>
      Two-Factor Authentication
    </h3>

    {#if error2FA}
      <div class="alert error mb-4">
        <i class="bi bi-exclamation-circle-fill"></i>
        {error2FA}
      </div>
    {/if}
    {#if success2FA}
      <div class="alert success mb-4">
        <i class="bi bi-check-circle-fill"></i>
        {success2FA}
      </div>
    {/if}

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
                disabled={saving2FA}
              >
                <i class="bi bi-shield-x"></i>
                {tr("disable2FA")}
              </button>
            {:else}
              <button
                class="btn btn-success"
                onclick={setup2FA}
                disabled={saving2FA}
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
                disabled={saving2FA}>{tr("cancel")}</button
              >
              <button
                class="btn btn-success"
                onclick={enable2FA}
                disabled={saving2FA || verificationCode.length !== 6}
              >
                {saving2FA ? tr("verifying") : tr("verify")}
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- ============================================ -->
  <!-- SECTION 2: Password Management -->
  <!-- ============================================ -->
  <div class="mb-8">
    <h3
      class="text-lg font-semibold text-gray-900 dark:text-gray-50 mb-4 flex items-center gap-2"
    >
      <i class="bi bi-key-fill text-amber-600"></i>
      Password Management
    </h3>

    {#if errorPassword}
      <div class="alert error mb-4">
        <i class="bi bi-exclamation-circle-fill"></i>
        {errorPassword}
      </div>
    {/if}
    {#if successPassword}
      <div class="alert success mb-4">
        <i class="bi bi-check-circle-fill"></i>
        {successPassword}
      </div>
    {/if}

    <div class="settings-grid">
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
                  errorPassword = "";
                }}>{tr("cancel")}</button
              >
              <button
                class="btn btn-success"
                onclick={changePassword}
                disabled={savingPassword}
              >
                {savingPassword ? tr("saving") : tr("savePassword")}
              </button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- ============================================ -->
  <!-- SECTION 3: Security Policies (Admin) -->
  <!-- ============================================ -->
  <div class="mb-8">
    <h3
      class="text-lg font-semibold text-gray-900 dark:text-gray-50 mb-4 flex items-center gap-2"
    >
      <i class="bi bi-shield-check text-blue-600"></i>
      Security Policies
    </h3>

    <div class="flex flex-col gap-6">
      <!-- Password Policy + 2FA Policy Row -->
      <div class="settings-grid">
        <!-- Password Policy Card -->
        <div class="card">
          <div class="card-header">
            <div class="card-icon red">
              <i class="bi bi-key-fill"></i>
            </div>
            <div class="card-title">
              <h3>Password Policy</h3>
              <p>Set password requirements for users</p>
            </div>
          </div>

          <div class="flex flex-col gap-4">
            <div class="grid grid-cols-2 gap-3">
              <div class="input-group">
                <label for="min-pw-len">Min Length</label>
                <input
                  id="min-pw-len"
                  type="number"
                  class="input"
                  bind:value={minPasswordLength}
                  min="6"
                  max="32"
                />
              </div>
              <div class="input-group">
                <label for="pw-expiry">Password Expiry</label>
                <select id="pw-expiry" class="input" bind:value={passwordExpiryDays}>
                  <option value={0}>Never</option>
                  <option value={30}>30 days</option>
                  <option value={90}>90 days</option>
                </select>
              </div>
            </div>

            <div class="flex flex-col gap-2">
              <label class="checkbox-label">
                <input type="checkbox" bind:checked={requireUppercase} />
                <span>Require uppercase letters</span>
              </label>
              <label class="checkbox-label">
                <input type="checkbox" bind:checked={requireLowercase} />
                <span>Require lowercase letters</span>
              </label>
              <label class="checkbox-label">
                <input type="checkbox" bind:checked={requireNumbers} />
                <span>Require numbers</span>
              </label>
              <label class="checkbox-label">
                <input type="checkbox" bind:checked={requireSpecialChars} />
                <span>Require special characters</span>
              </label>
            </div>
          </div>
        </div>

        <!-- 2FA Policy Card -->
        <div class="card">
          <div class="card-header">
            <div class="card-icon green">
              <i class="bi bi-shield-lock-fill"></i>
            </div>
            <div class="card-title">
              <h3>2FA Policy</h3>
              <p>Configure two-factor authentication requirements</p>
            </div>
          </div>

          <div class="flex flex-col gap-4">
            <label class="toggle-label">
              <div class="flex-1">
                <span class="font-medium">Require 2FA for all users</span>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
                  Force all users to enable 2FA
                </p>
              </div>
              <input
                type="checkbox"
                class="toggle"
                bind:checked={require2FA}
              />
            </label>

            <label class="toggle-label">
              <div class="flex-1">
                <span class="font-medium">Require 2FA for admins</span>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
                  Enforce 2FA for administrator accounts
                </p>
              </div>
              <input
                type="checkbox"
                class="toggle"
                bind:checked={enforce2FAForAdmins}
              />
            </label>

            <div class="input-group">
              <label>Allowed 2FA Methods</label>
              <div class="flex gap-2 flex-wrap">
                <label class="method-badge active">
                  <input
                    type="checkbox"
                    checked={allowed2FAMethods.includes("totp")}
                    onchange={() => toggle2FAMethod("totp")}
                  />
                  <i class="bi bi-phone"></i>
                  <span>TOTP</span>
                </label>
                <label class="method-badge disabled">
                  <input type="checkbox" disabled />
                  <i class="bi bi-chat-dots"></i>
                  <span>SMS (Coming Soon)</span>
                </label>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Session Policy + Login Security Row -->
      <div class="settings-grid">
        <!-- Session Policy Card -->
        <div class="card">
          <div class="card-header">
            <div class="card-icon blue">
              <i class="bi bi-clock-history"></i>
            </div>
            <div class="card-title">
              <h3>Session Policy</h3>
              <p>Manage user session settings</p>
            </div>
          </div>

          <div class="flex flex-col gap-4">
            <div class="grid grid-cols-2 gap-3">
              <div class="input-group">
                <label for="session-timeout">Session Timeout</label>
                <select
                  id="session-timeout"
                  class="input"
                  bind:value={sessionTimeoutMinutes}
                >
                  <option value={30}>30 minutes</option>
                  <option value={60}>1 hour</option>
                  <option value={240}>4 hours</option>
                  <option value={480}>8 hours</option>
                  <option value={1440}>24 hours</option>
                </select>
              </div>
              <div class="input-group">
                <label for="max-sessions">Max Sessions</label>
                <input
                  id="max-sessions"
                  type="number"
                  class="input"
                  bind:value={maxConcurrentSessions}
                  min="1"
                  max="20"
                />
              </div>
            </div>

            <label class="toggle-label">
              <div class="flex-1">
                <span class="font-medium">Require re-auth for sensitive actions</span>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
                  Users must re-authenticate for critical operations
                </p>
              </div>
              <input
                type="checkbox"
                class="toggle"
                bind:checked={enforceReauthForSensitive}
              />
            </label>
          </div>
        </div>

        <!-- Login Security Card -->
        <div class="card">
          <div class="card-header">
            <div class="card-icon amber">
              <i class="bi bi-box-arrow-in-right"></i>
            </div>
            <div class="card-title">
              <h3>Login Security</h3>
              <p>Configure login attempt protection</p>
            </div>
          </div>

          <div class="flex flex-col gap-4">
            <div class="grid grid-cols-2 gap-3">
              <div class="input-group">
                <label for="max-attempts">Max Login Attempts</label>
                <input
                  id="max-attempts"
                  type="number"
                  class="input"
                  bind:value={maxLoginAttempts}
                  min="3"
                  max="20"
                />
              </div>
              <div class="input-group">
                <label for="lockout-duration">Lockout Duration</label>
                <select
                  id="lockout-duration"
                  class="input"
                  bind:value={lockoutDurationMinutes}
                >
                  <option value={5}>5 minutes</option>
                  <option value={15}>15 minutes</option>
                  <option value={30}>30 minutes</option>
                  <option value={60}>1 hour</option>
                </select>
              </div>
            </div>

            <label class="toggle-label">
              <div class="flex-1">
                <span class="font-medium">Enable CAPTCHA</span>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
                  Show CAPTCHA after failed login attempts
                </p>
              </div>
              <input
                type="checkbox"
                class="toggle"
                bind:checked={enableCaptcha}
              />
            </label>
          </div>
        </div>
      </div>

      <!-- IP Security Card - Full Width -->
      <div class="card">
        <div class="card-header">
          <div class="card-icon purple">
            <i class="bi bi-globe"></i>
          </div>
          <div class="card-title">
            <h3>IP Security</h3>
            <p>Control IP-based access restrictions</p>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
          <div class="flex flex-col gap-2">
            <label class="toggle-label">
              <div class="flex-1">
                <span class="font-medium text-sm flex items-center gap-1.5">
                  <i class="bi bi-check-circle text-green-500"></i>
                  IP Whitelist
                </span>
              </div>
              <input
                type="checkbox"
                class="toggle"
                bind:checked={enableIPWhitelist}
              />
            </label>
            {#if enableIPWhitelist}
              <textarea
                class="input resize-none font-mono text-xs"
                bind:value={ipWhitelist}
                placeholder="192.168.1.0/24&#10;10.0.0.0/8"
                rows="3"
              ></textarea>
            {/if}
          </div>

          <div class="flex flex-col gap-2">
            <label class="toggle-label">
              <div class="flex-1">
                <span class="font-medium text-sm flex items-center gap-1.5">
                  <i class="bi bi-x-circle text-red-500"></i>
                  IP Blacklist
                </span>
              </div>
              <input
                type="checkbox"
                class="toggle"
                bind:checked={enableIPBlacklist}
              />
            </label>
            {#if enableIPBlacklist}
              <textarea
                class="input resize-none font-mono text-xs"
                bind:value={ipBlacklist}
                placeholder="192.168.1.100&#10;10.0.0.50"
                rows="3"
              ></textarea>
            {/if}
          </div>

          <div class="flex flex-col gap-2">
            <label class="toggle-label">
              <div class="flex-1">
                <span class="font-medium text-sm flex items-center gap-1.5">
                  <i class="bi bi-shield-x text-amber-500"></i>
                  Block Tor Exits
                </span>
              </div>
              <input
                type="checkbox"
                class="toggle"
                bind:checked={blockTorExits}
              />
            </label>
          </div>
        </div>
      </div>

      <!-- Save Policies Button -->
      <div class="flex justify-end">
        <button
          class="btn btn-success"
          onclick={saveSecurityPolicies}
          disabled={savingPolicies}
        >
          {#if savingPolicies}
            <span class="spinner-sm"></span>
          {:else}
            <i class="bi bi-check-lg"></i>
          {/if}
          Save Security Policies
        </button>
      </div>
    </div>
  </div>

  <!-- ============================================ -->
  <!-- SECTION 4: Active Sessions -->
  <!-- ============================================ -->
  <div class="mb-8">
    <h3
      class="text-lg font-semibold text-gray-900 dark:text-gray-50 mb-4 flex items-center gap-2"
    >
      <i class="bi bi-laptop text-blue-600"></i>
      Active Sessions
    </h3>

    <div class="settings-grid">
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
  .spinner-sm {
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid white;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
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
  .card-icon.red {
    background: #fee2e2;
    color: #dc2626;
  }
  .card-icon.purple {
    background: #f3e8ff;
    color: #9333ea;
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
  :global(.dark) .card-icon.red {
    background: rgba(220, 38, 38, 0.2);
    color: #ef4444;
  }
  :global(.dark) .card-icon.purple {
    background: rgba(147, 51, 234, 0.2);
    color: #a855f7;
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

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: #f9fafb;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 0.875rem;
  }
  :global(.dark) .checkbox-label {
    background: #374151;
  }
  .checkbox-label input[type="checkbox"] {
    width: 16px;
    height: 16px;
    cursor: pointer;
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }
  :global(.dark) .toggle-label {
    background: #374151;
  }

  .toggle {
    appearance: none;
    width: 44px;
    height: 24px;
    background: #d1d5db;
    border-radius: 12px;
    position: relative;
    cursor: pointer;
    transition: background 0.2s;
  }
  .toggle:checked {
    background: #22c55e;
  }
  .toggle::before {
    content: "";
    position: absolute;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: white;
    top: 3px;
    left: 3px;
    transition: left 0.2s;
  }
  .toggle:checked::before {
    left: 23px;
  }

  .method-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    cursor: pointer;
    background: white;
  }
  .method-badge.active {
    border-color: #22c55e;
    background: #dcfce7;
  }
  .method-badge.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  :global(.dark) .method-badge {
    background: #374151;
    border-color: #4b5563;
  }
  :global(.dark) .method-badge.active {
    background: rgba(34, 197, 94, 0.2);
    border-color: #22c55e;
  }
  .method-badge input[type="checkbox"] {
    width: 14px;
    height: 14px;
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
