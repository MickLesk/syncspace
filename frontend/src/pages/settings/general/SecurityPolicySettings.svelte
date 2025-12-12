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

  async function savePolicy() {
    saving = true;
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
  <div class="flex justify-center items-center min-h-[300px]">
    <span class="loading loading-spinner loading-lg text-primary"></span>
  </div>
{:else}
  <div class="flex flex-col gap-4">
    <!-- Top Row: Password + 2FA -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <!-- Password Policy Card -->
      <div
        class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden"
      >
        <div class="flex items-center gap-3 p-4 border-b border-base-200">
          <div
            class="w-9 h-9 rounded-lg flex items-center justify-center text-base shrink-0 bg-red-100 text-red-600 dark:bg-red-500/20 dark:text-red-400"
          >
            <i class="bi bi-key-fill"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-sm font-semibold text-base-content m-0">
              {tr("settings.security.password_policy")}
            </h3>
            <p class="text-[0.6875rem] text-base-content/60 mt-0.5 m-0">
              {tr("settings.security.password_policy_desc")}
            </p>
          </div>
        </div>

        <div class="p-4">
          <div class="grid grid-cols-2 gap-3 mb-3">
            <div class="flex flex-col gap-1">
              <span
                class="text-[0.6875rem] font-medium text-base-content/60 uppercase tracking-wide"
                >{tr("settings.security.min_length")}</span
              >
              <input
                type="number"
                class="input input-sm input-bordered"
                bind:value={minPasswordLength}
                min="6"
                max="32"
              />
            </div>
            <div class="flex flex-col gap-1">
              <span
                class="text-[0.6875rem] font-medium text-base-content/60 uppercase tracking-wide"
                >{tr("settings.security.password_expiry")}</span
              >
              <select
                class="select select-sm select-bordered"
                bind:value={passwordExpiryDays}
              >
                <option value={0}>{tr("settings.security.never")}</option>
                <option value={30}>30 {tr("common.days")}</option>
                <option value={90}>90 {tr("common.days")}</option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-1.5">
            <label
              class="flex items-center gap-1.5 px-2 py-1.5 bg-base-200 rounded-md cursor-pointer text-xs text-base-content"
            >
              <input
                type="checkbox"
                class="checkbox checkbox-xs checkbox-primary"
                bind:checked={requireUppercase}
              />
              <span>{tr("settings.security.require_uppercase")}</span>
            </label>
            <label
              class="flex items-center gap-1.5 px-2 py-1.5 bg-base-200 rounded-md cursor-pointer text-xs text-base-content"
            >
              <input
                type="checkbox"
                class="checkbox checkbox-xs checkbox-primary"
                bind:checked={requireLowercase}
              />
              <span>{tr("settings.security.require_lowercase")}</span>
            </label>
            <label
              class="flex items-center gap-1.5 px-2 py-1.5 bg-base-200 rounded-md cursor-pointer text-xs text-base-content"
            >
              <input
                type="checkbox"
                class="checkbox checkbox-xs checkbox-primary"
                bind:checked={requireNumbers}
              />
              <span>{tr("settings.security.require_numbers")}</span>
            </label>
            <label
              class="flex items-center gap-1.5 px-2 py-1.5 bg-base-200 rounded-md cursor-pointer text-xs text-base-content"
            >
              <input
                type="checkbox"
                class="checkbox checkbox-xs checkbox-primary"
                bind:checked={requireSpecialChars}
              />
              <span>{tr("settings.security.require_special")}</span>
            </label>
          </div>
        </div>
      </div>

      <!-- 2FA Policy Card -->
      <div
        class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden"
      >
        <div class="flex items-center gap-3 p-4 border-b border-base-200">
          <div
            class="w-9 h-9 rounded-lg flex items-center justify-center text-base shrink-0 bg-green-100 text-green-600 dark:bg-green-500/20 dark:text-green-400"
          >
            <i class="bi bi-shield-lock-fill"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-sm font-semibold text-base-content m-0">
              {tr("settings.security.2fa_policy")}
            </h3>
            <p class="text-[0.6875rem] text-base-content/60 mt-0.5 m-0">
              {tr("settings.security.2fa_policy_desc")}
            </p>
          </div>
        </div>

        <div class="p-4">
          <div class="flex flex-col gap-2">
            <label
              class="flex items-center justify-between px-2.5 py-2 bg-base-200 rounded-md text-xs text-base-content"
            >
              <span>{tr("settings.security.require_2fa_all")}</span>
              <input
                type="checkbox"
                class="toggle toggle-sm toggle-primary"
                bind:checked={require2FA}
              />
            </label>
            <label
              class="flex items-center justify-between px-2.5 py-2 bg-base-200 rounded-md text-xs text-base-content"
            >
              <span>{tr("settings.security.require_2fa_admins")}</span>
              <input
                type="checkbox"
                class="toggle toggle-sm toggle-primary"
                bind:checked={enforce2FAForAdmins}
              />
            </label>
          </div>

          <div class="mt-3 pt-3 border-t border-base-200">
            <span
              class="text-[0.6875rem] font-medium text-base-content/60 uppercase block mb-2"
              >{tr("settings.security.allowed_2fa_methods")}</span
            >
            <div class="flex gap-1.5 flex-wrap">
              <label
                class="flex items-center gap-1 px-2 py-1.5 bg-base-100 border border-primary rounded-md text-[0.6875rem] cursor-pointer"
              >
                <input
                  type="checkbox"
                  class="checkbox checkbox-xs"
                  checked={allowed2FAMethods.includes("totp")}
                  onchange={() => toggle2FAMethod("totp")}
                />
                <i class="bi bi-phone"></i>
                <span>TOTP</span>
              </label>
              <label
                class="flex items-center gap-1 px-2 py-1.5 bg-base-200 border border-base-300 rounded-md text-[0.6875rem] opacity-50 cursor-not-allowed"
              >
                <input type="checkbox" class="checkbox checkbox-xs" disabled />
                <i class="bi bi-chat-dots"></i>
                <span>SMS</span>
              </label>
              <label
                class="flex items-center gap-1 px-2 py-1.5 bg-base-200 border border-base-300 rounded-md text-[0.6875rem] opacity-50 cursor-not-allowed"
              >
                <input type="checkbox" class="checkbox checkbox-xs" disabled />
                <i class="bi bi-envelope"></i>
                <span>Email</span>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Middle Row: Session + Login -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <!-- Session Policy Card -->
      <div
        class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden"
      >
        <div class="flex items-center gap-3 p-4 border-b border-base-200">
          <div
            class="w-9 h-9 rounded-lg flex items-center justify-center text-base shrink-0 bg-blue-100 text-blue-600 dark:bg-blue-500/20 dark:text-blue-400"
          >
            <i class="bi bi-clock-history"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-sm font-semibold text-base-content m-0">
              {tr("settings.security.session_policy")}
            </h3>
            <p class="text-[0.6875rem] text-base-content/60 mt-0.5 m-0">
              {tr("settings.security.session_policy_desc")}
            </p>
          </div>
        </div>

        <div class="p-4">
          <div class="grid grid-cols-2 gap-3 mb-3">
            <div class="flex flex-col gap-1">
              <span
                class="text-[0.6875rem] font-medium text-base-content/60 uppercase tracking-wide"
                >{tr("settings.security.session_timeout")}</span
              >
              <select
                class="select select-sm select-bordered"
                bind:value={sessionTimeoutMinutes}
              >
                <option value={30}>30 min</option>
                <option value={60}>1h</option>
                <option value={240}>4h</option>
                <option value={480}>8h</option>
                <option value={1440}>24h</option>
              </select>
            </div>
            <div class="flex flex-col gap-1">
              <span
                class="text-[0.6875rem] font-medium text-base-content/60 uppercase tracking-wide"
                >{tr("settings.security.max_sessions")}</span
              >
              <input
                type="number"
                class="input input-sm input-bordered"
                bind:value={maxConcurrentSessions}
                min="1"
                max="20"
              />
            </div>
          </div>

          <label
            class="flex items-center justify-between px-2.5 py-2 bg-base-200 rounded-md text-xs text-base-content"
          >
            <span>{tr("settings.security.reauth_sensitive")}</span>
            <input
              type="checkbox"
              class="toggle toggle-sm toggle-primary"
              bind:checked={enforceReauthForSensitive}
            />
          </label>
        </div>
      </div>

      <!-- Login Security Card -->
      <div
        class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden"
      >
        <div class="flex items-center gap-3 p-4 border-b border-base-200">
          <div
            class="w-9 h-9 rounded-lg flex items-center justify-center text-base shrink-0 bg-amber-100 text-amber-600 dark:bg-amber-500/20 dark:text-amber-400"
          >
            <i class="bi bi-box-arrow-in-right"></i>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-sm font-semibold text-base-content m-0">
              {tr("settings.security.login_security")}
            </h3>
            <p class="text-[0.6875rem] text-base-content/60 mt-0.5 m-0">
              {tr("settings.security.login_security_desc")}
            </p>
          </div>
        </div>

        <div class="p-4">
          <div class="grid grid-cols-2 gap-3 mb-3">
            <div class="flex flex-col gap-1">
              <span
                class="text-[0.6875rem] font-medium text-base-content/60 uppercase tracking-wide"
                >{tr("settings.security.max_login_attempts")}</span
              >
              <input
                type="number"
                class="input input-sm input-bordered"
                bind:value={maxLoginAttempts}
                min="3"
                max="20"
              />
            </div>
            <div class="flex flex-col gap-1">
              <span
                class="text-[0.6875rem] font-medium text-base-content/60 uppercase tracking-wide"
                >{tr("settings.security.lockout_duration")}</span
              >
              <select
                class="select select-sm select-bordered"
                bind:value={lockoutDurationMinutes}
              >
                <option value={5}>5 min</option>
                <option value={15}>15 min</option>
                <option value={30}>30 min</option>
                <option value={60}>1h</option>
              </select>
            </div>
          </div>

          <label
            class="flex items-center justify-between px-2.5 py-2 bg-base-200 rounded-md text-xs text-base-content"
          >
            <span>{tr("settings.security.enable_captcha")}</span>
            <input
              type="checkbox"
              class="toggle toggle-sm toggle-primary"
              bind:checked={enableCaptcha}
            />
          </label>
        </div>
      </div>
    </div>

    <!-- IP Security Card - Full Width -->
    <div
      class="bg-gradient-to-br from-base-100/80 to-base-100/40 rounded-xl backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all overflow-hidden"
    >
      <div class="flex items-center gap-3 p-4 border-b border-base-200">
        <div
          class="w-9 h-9 rounded-lg flex items-center justify-center text-base shrink-0 bg-purple-100 text-purple-600 dark:bg-purple-500/20 dark:text-purple-400"
        >
          <i class="bi bi-globe"></i>
        </div>
        <div class="flex-1 min-w-0">
          <h3 class="text-sm font-semibold text-base-content m-0">
            {tr("settings.security.ip_security")}
          </h3>
          <p class="text-[0.6875rem] text-base-content/60 mt-0.5 m-0">
            {tr("settings.security.ip_security_desc")}
          </p>
        </div>
      </div>

      <div class="p-4">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
          <div class="flex flex-col gap-2">
            <label
              class="flex items-center justify-between px-2 py-1.5 bg-base-200 rounded-md text-xs text-base-content"
            >
              <span class="flex items-center gap-1.5"
                ><i class="bi bi-check-circle text-green-500"></i>{tr(
                  "settings.security.ip_whitelist"
                )}</span
              >
              <input
                type="checkbox"
                class="toggle toggle-sm toggle-primary"
                bind:checked={enableIPWhitelist}
              />
            </label>
            {#if enableIPWhitelist}
              <textarea
                class="textarea textarea-bordered textarea-sm font-mono text-xs resize-none"
                bind:value={ipWhitelist}
                placeholder="192.168.1.0/24&#10;10.0.0.0/8"
                rows="2"
              ></textarea>
            {/if}
          </div>

          <div class="flex flex-col gap-2">
            <label
              class="flex items-center justify-between px-2 py-1.5 bg-base-200 rounded-md text-xs text-base-content"
            >
              <span class="flex items-center gap-1.5"
                ><i class="bi bi-x-circle text-red-500"></i>{tr(
                  "settings.security.ip_blacklist"
                )}</span
              >
              <input
                type="checkbox"
                class="toggle toggle-sm toggle-primary"
                bind:checked={enableIPBlacklist}
              />
            </label>
            {#if enableIPBlacklist}
              <textarea
                class="textarea textarea-bordered textarea-sm font-mono text-xs resize-none"
                bind:value={ipBlacklist}
                placeholder="192.168.1.100&#10;10.0.0.50"
                rows="2"
              ></textarea>
            {/if}
          </div>

          <div class="flex flex-col gap-2">
            <label
              class="flex items-center justify-between px-2 py-1.5 bg-base-200 rounded-md text-xs text-base-content"
            >
              <span class="flex items-center gap-1.5"
                ><i class="bi bi-shield-x text-amber-500"></i>{tr(
                  "settings.security.block_tor"
                )}</span
              >
              <input
                type="checkbox"
                class="toggle toggle-sm toggle-primary"
                bind:checked={blockTorExits}
              />
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Save Button -->
    <div class="flex justify-end mt-2">
      <button
        class="btn btn-primary gap-2"
        onclick={savePolicy}
        disabled={saving}
      >
        {#if saving}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <i class="bi bi-check-lg"></i>
        {/if}
        {tr("common.save")}
      </button>
    </div>
  </div>
{/if}

<style>
  /* Minimal custom styles */
</style>
