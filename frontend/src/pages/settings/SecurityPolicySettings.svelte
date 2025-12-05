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

  // Password Policy
  let minPasswordLength = $state(8);
  let requireUppercase = $state(true);
  let requireLowercase = $state(true);
  let requireNumbers = $state(true);
  let requireSpecialChars = $state(false);
  let passwordExpiryDays = $state(0); // 0 = never
  let preventPasswordReuse = $state(5); // last N passwords

  // 2FA Policy
  let require2FA = $state(false);
  let enforce2FAForAdmins = $state(true);
  let allowed2FAMethods = $state(["totp"]); // totp, sms, email

  // Session Policy
  let sessionTimeoutMinutes = $state(480); // 8 hours
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
    await loadSecurityPolicy();
  });

  async function loadSecurityPolicy() {
    loading = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/admin/security-policy",
        {
          headers: { Authorization: `Bearer ${token}` },
        }
      );

      if (response.ok) {
        const data = await response.json();
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
      }
    } catch (err) {
      console.error("Failed to load security policy:", err);
    } finally {
      loading = false;
    }
  }

  async function savePolicy() {
    saving = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/admin/security-policy",
        {
          method: "PUT",
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
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
          }),
        }
      );

      if (response.ok) {
        toastSuccess(tr("settings.security.saved"));
      } else {
        toastError(tr("settings.security.save_error"));
      }
    } catch (err) {
      toastError(err.message);
    } finally {
      saving = false;
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
  <div class="loading-container">
    <div class="spinner"></div>
  </div>
{:else}
  <div class="security-settings">
    <!-- Top Row: Password + 2FA -->
    <div class="settings-grid">
      <!-- Password Policy Card -->
      <div class="setting-card">
        <div class="card-header">
          <div class="card-icon password-icon">
            <i class="bi bi-key-fill"></i>
          </div>
          <div class="card-info">
            <h3>{tr("settings.security.password_policy")}</h3>
            <p>{tr("settings.security.password_policy_desc")}</p>
          </div>
        </div>

        <div class="card-content">
          <div class="compact-grid">
            <div class="input-group">
              <span class="field-label"
                >{tr("settings.security.min_length")}</span
              >
              <input
                type="number"
                bind:value={minPasswordLength}
                min="6"
                max="32"
              />
            </div>
            <div class="input-group">
              <span class="field-label"
                >{tr("settings.security.password_expiry")}</span
              >
              <select bind:value={passwordExpiryDays}>
                <option value={0}>{tr("settings.security.never")}</option>
                <option value={30}>30 {tr("common.days")}</option>
                <option value={90}>90 {tr("common.days")}</option>
              </select>
            </div>
          </div>

          <div class="checkbox-grid">
            <label class="checkbox-item">
              <input type="checkbox" bind:checked={requireUppercase} />
              <span>{tr("settings.security.require_uppercase")}</span>
            </label>
            <label class="checkbox-item">
              <input type="checkbox" bind:checked={requireLowercase} />
              <span>{tr("settings.security.require_lowercase")}</span>
            </label>
            <label class="checkbox-item">
              <input type="checkbox" bind:checked={requireNumbers} />
              <span>{tr("settings.security.require_numbers")}</span>
            </label>
            <label class="checkbox-item">
              <input type="checkbox" bind:checked={requireSpecialChars} />
              <span>{tr("settings.security.require_special")}</span>
            </label>
          </div>
        </div>
      </div>

      <!-- 2FA Policy Card -->
      <div class="setting-card">
        <div class="card-header">
          <div class="card-icon twofa-icon">
            <i class="bi bi-shield-lock-fill"></i>
          </div>
          <div class="card-info">
            <h3>{tr("settings.security.2fa_policy")}</h3>
            <p>{tr("settings.security.2fa_policy_desc")}</p>
          </div>
        </div>

        <div class="card-content">
          <div class="toggle-list">
            <label class="toggle-item">
              <span>{tr("settings.security.require_2fa_all")}</span>
              <label class="toggle">
                <input type="checkbox" bind:checked={require2FA} />
                <span class="toggle-slider"></span>
              </label>
            </label>
            <label class="toggle-item">
              <span>{tr("settings.security.require_2fa_admins")}</span>
              <label class="toggle">
                <input type="checkbox" bind:checked={enforce2FAForAdmins} />
                <span class="toggle-slider"></span>
              </label>
            </label>
          </div>

          <div class="methods-section">
            <span class="methods-label"
              >{tr("settings.security.allowed_2fa_methods")}</span
            >
            <div class="methods-grid">
              <label class="method-item active">
                <input
                  type="checkbox"
                  checked={allowed2FAMethods.includes("totp")}
                  onchange={() => toggle2FAMethod("totp")}
                />
                <i class="bi bi-phone"></i>
                <span>TOTP</span>
              </label>
              <label class="method-item disabled">
                <input type="checkbox" disabled />
                <i class="bi bi-chat-dots"></i>
                <span>SMS</span>
              </label>
              <label class="method-item disabled">
                <input type="checkbox" disabled />
                <i class="bi bi-envelope"></i>
                <span>Email</span>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Middle Row: Session + Login -->
    <div class="settings-grid">
      <!-- Session Policy Card -->
      <div class="setting-card">
        <div class="card-header">
          <div class="card-icon session-icon">
            <i class="bi bi-clock-history"></i>
          </div>
          <div class="card-info">
            <h3>{tr("settings.security.session_policy")}</h3>
            <p>{tr("settings.security.session_policy_desc")}</p>
          </div>
        </div>

        <div class="card-content">
          <div class="compact-grid">
            <div class="input-group">
              <span class="field-label"
                >{tr("settings.security.session_timeout")}</span
              >
              <select bind:value={sessionTimeoutMinutes}>
                <option value={30}>30 min</option>
                <option value={60}>1h</option>
                <option value={240}>4h</option>
                <option value={480}>8h</option>
                <option value={1440}>24h</option>
              </select>
            </div>
            <div class="input-group">
              <span class="field-label"
                >{tr("settings.security.max_sessions")}</span
              >
              <input
                type="number"
                bind:value={maxConcurrentSessions}
                min="1"
                max="20"
              />
            </div>
          </div>

          <label class="toggle-item">
            <span>{tr("settings.security.reauth_sensitive")}</span>
            <label class="toggle">
              <input type="checkbox" bind:checked={enforceReauthForSensitive} />
              <span class="toggle-slider"></span>
            </label>
          </label>
        </div>
      </div>

      <!-- Login Security Card -->
      <div class="setting-card">
        <div class="card-header">
          <div class="card-icon login-icon">
            <i class="bi bi-box-arrow-in-right"></i>
          </div>
          <div class="card-info">
            <h3>{tr("settings.security.login_security")}</h3>
            <p>{tr("settings.security.login_security_desc")}</p>
          </div>
        </div>

        <div class="card-content">
          <div class="compact-grid">
            <div class="input-group">
              <span class="field-label"
                >{tr("settings.security.max_login_attempts")}</span
              >
              <input
                type="number"
                bind:value={maxLoginAttempts}
                min="3"
                max="20"
              />
            </div>
            <div class="input-group">
              <span class="field-label"
                >{tr("settings.security.lockout_duration")}</span
              >
              <select bind:value={lockoutDurationMinutes}>
                <option value={5}>5 min</option>
                <option value={15}>15 min</option>
                <option value={30}>30 min</option>
                <option value={60}>1h</option>
              </select>
            </div>
          </div>

          <div class="toggle-list">
            <label class="toggle-item">
              <span>{tr("settings.security.enable_captcha")}</span>
              <label class="toggle">
                <input type="checkbox" bind:checked={enableCaptcha} />
                <span class="toggle-slider"></span>
              </label>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- IP Security Card - Full Width -->
    <div class="setting-card full-width">
      <div class="card-header">
        <div class="card-icon ip-icon">
          <i class="bi bi-globe"></i>
        </div>
        <div class="card-info">
          <h3>{tr("settings.security.ip_security")}</h3>
          <p>{tr("settings.security.ip_security_desc")}</p>
        </div>
      </div>

      <div class="card-content">
        <div class="ip-grid">
          <div class="ip-section">
            <label class="toggle-item compact">
              <span
                ><i class="bi bi-check-circle text-green-500"></i>
                {tr("settings.security.ip_whitelist")}</span
              >
              <label class="toggle">
                <input type="checkbox" bind:checked={enableIPWhitelist} />
                <span class="toggle-slider"></span>
              </label>
            </label>
            {#if enableIPWhitelist}
              <textarea
                bind:value={ipWhitelist}
                placeholder="192.168.1.0/24&#10;10.0.0.0/8"
                rows="2"
              ></textarea>
            {/if}
          </div>

          <div class="ip-section">
            <label class="toggle-item compact">
              <span
                ><i class="bi bi-x-circle text-red-500"></i>
                {tr("settings.security.ip_blacklist")}</span
              >
              <label class="toggle">
                <input type="checkbox" bind:checked={enableIPBlacklist} />
                <span class="toggle-slider"></span>
              </label>
            </label>
            {#if enableIPBlacklist}
              <textarea
                bind:value={ipBlacklist}
                placeholder="192.168.1.100&#10;10.0.0.50"
                rows="2"
              ></textarea>
            {/if}
          </div>

          <div class="ip-section">
            <label class="toggle-item compact">
              <span
                ><i class="bi bi-shield-x text-amber-500"></i>
                {tr("settings.security.block_tor")}</span
              >
              <label class="toggle">
                <input type="checkbox" bind:checked={blockTorExits} />
                <span class="toggle-slider"></span>
              </label>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Save Button -->
    <div class="save-section">
      <button class="btn-save" onclick={savePolicy} disabled={saving}>
        {#if saving}
          <span class="spinner-small"></span>
        {:else}
          <i class="bi bi-check-lg"></i>
        {/if}
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

  .security-settings {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  @media (max-width: 900px) {
    .settings-grid {
      grid-template-columns: 1fr;
    }
  }

  .setting-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    overflow: hidden;
  }

  :global(.dark) .setting-card {
    background: #1f2937;
    border-color: #374151;
  }

  .setting-card.full-width {
    grid-column: span 2;
  }

  @media (max-width: 900px) {
    .setting-card.full-width {
      grid-column: span 1;
    }
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-bottom: 1px solid #f3f4f6;
  }

  :global(.dark) .card-header {
    border-bottom-color: #374151;
  }

  .card-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
    flex-shrink: 0;
  }

  .password-icon {
    background: #fee2e2;
    color: #dc2626;
  }
  :global(.dark) .password-icon {
    background: rgba(220, 38, 38, 0.2);
  }

  .twofa-icon {
    background: #dcfce7;
    color: #16a34a;
  }
  :global(.dark) .twofa-icon {
    background: rgba(22, 163, 74, 0.2);
  }

  .session-icon {
    background: #dbeafe;
    color: #2563eb;
  }
  :global(.dark) .session-icon {
    background: rgba(37, 99, 235, 0.2);
  }

  .login-icon {
    background: #fef3c7;
    color: #d97706;
  }
  :global(.dark) .login-icon {
    background: rgba(217, 119, 6, 0.2);
  }

  .ip-icon {
    background: #f3e8ff;
    color: #9333ea;
  }
  :global(.dark) .ip-icon {
    background: rgba(147, 51, 234, 0.2);
  }

  .card-info {
    flex: 1;
    min-width: 0;
  }

  .card-info h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  :global(.dark) .card-info h3 {
    color: #f9fafb;
  }

  .card-info p {
    font-size: 0.6875rem;
    color: #6b7280;
    margin: 0.125rem 0 0;
  }

  :global(.dark) .card-info p {
    color: #9ca3af;
  }

  .card-content {
    padding: 1rem;
  }

  /* Compact Grid */
  .compact-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .input-group .field-label {
    font-size: 0.6875rem;
    font-weight: 500;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.025em;
  }

  :global(.dark) .input-group .field-label {
    color: #9ca3af;
  }

  .input-group input,
  .input-group select {
    padding: 0.375rem 0.5rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    background: white;
    color: #111827;
  }

  :global(.dark) .input-group input,
  :global(.dark) .input-group select {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  /* Checkbox Grid */
  .checkbox-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.375rem;
  }

  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.5rem;
    background: #f9fafb;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 0.75rem;
    color: #374151;
  }

  :global(.dark) .checkbox-item {
    background: #374151;
    color: #e5e7eb;
  }

  .checkbox-item input {
    width: 0.875rem;
    height: 0.875rem;
    accent-color: #3b82f6;
  }

  /* Toggle List */
  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .toggle-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.625rem;
    background: #f9fafb;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    color: #374151;
  }

  :global(.dark) .toggle-item {
    background: #374151;
    color: #e5e7eb;
  }

  .toggle-item.compact {
    padding: 0.375rem 0.5rem;
  }

  .toggle-item span {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  /* Toggle Switch */
  .toggle {
    position: relative;
    display: inline-block;
    width: 36px;
    height: 20px;
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
    border-radius: 20px;
    transition: 0.2s;
  }

  :global(.dark) .toggle-slider {
    background: #4b5563;
  }

  .toggle-slider::before {
    position: absolute;
    content: "";
    height: 14px;
    width: 14px;
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
    transform: translateX(16px);
  }

  /* Methods Section */
  .methods-section {
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid #f3f4f6;
  }

  :global(.dark) .methods-section {
    border-top-color: #4b5563;
  }

  .methods-label {
    font-size: 0.6875rem;
    font-weight: 500;
    color: #6b7280;
    text-transform: uppercase;
    display: block;
    margin-bottom: 0.5rem;
  }

  :global(.dark) .methods-label {
    color: #9ca3af;
  }

  .methods-grid {
    display: flex;
    gap: 0.375rem;
    flex-wrap: wrap;
  }

  .method-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.375rem 0.5rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.6875rem;
    cursor: pointer;
  }

  :global(.dark) .method-item {
    background: #374151;
    border-color: #4b5563;
    color: #e5e7eb;
  }

  .method-item.active {
    border-color: #3b82f6;
    background: #eff6ff;
  }

  :global(.dark) .method-item.active {
    background: rgba(59, 130, 246, 0.1);
  }

  .method-item.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .method-item input {
    width: 0.75rem;
    height: 0.75rem;
  }

  /* IP Grid */
  .ip-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
  }

  @media (max-width: 900px) {
    .ip-grid {
      grid-template-columns: 1fr;
    }
  }

  .ip-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .ip-section textarea {
    padding: 0.375rem 0.5rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-family: monospace;
    resize: none;
    background: white;
    color: #111827;
  }

  :global(.dark) .ip-section textarea {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  /* Save Section */
  .save-section {
    display: flex;
    justify-content: flex-end;
    margin-top: 0.5rem;
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

  /* Utility colors */
  .text-green-500 {
    color: #22c55e;
  }
  .text-red-500 {
    color: #ef4444;
  }
  .text-amber-500 {
    color: #f59e0b;
  }
</style>
