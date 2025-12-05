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
  <div class="flex items-center justify-center py-12">
    <div
      class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"
    ></div>
  </div>
{:else}
  <div class="space-y-6">
    <!-- Password Policy -->
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center gap-3 mb-6">
        <div
          class="w-10 h-10 rounded-lg bg-red-100 dark:bg-red-900/30 flex items-center justify-center"
        >
          <i class="bi bi-key-fill text-red-600 dark:text-red-400"></i>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {tr("settings.security.password_policy")}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("settings.security.password_policy_desc")}
          </p>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Min Length -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("settings.security.min_length")}
          </label>
          <input
            type="number"
            bind:value={minPasswordLength}
            min="6"
            max="32"
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <!-- Password Expiry -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("settings.security.password_expiry")}
          </label>
          <select
            bind:value={passwordExpiryDays}
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          >
            <option value={0}>{tr("settings.security.never")}</option>
            <option value={30}>30 {tr("common.days")}</option>
            <option value={60}>60 {tr("common.days")}</option>
            <option value={90}>90 {tr("common.days")}</option>
            <option value={180}>180 {tr("common.days")}</option>
            <option value={365}>365 {tr("common.days")}</option>
          </select>
        </div>

        <!-- Prevent Reuse -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("settings.security.prevent_reuse")}
          </label>
          <select
            bind:value={preventPasswordReuse}
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          >
            <option value={0}>{tr("settings.security.disabled")}</option>
            <option value={3}
              >{tr("settings.security.last_n_passwords", 3)}</option
            >
            <option value={5}
              >{tr("settings.security.last_n_passwords", 5)}</option
            >
            <option value={10}
              >{tr("settings.security.last_n_passwords", 10)}</option
            >
          </select>
        </div>
      </div>

      <div class="mt-4 space-y-2">
        <label
          class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <input
            type="checkbox"
            bind:checked={requireUppercase}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span class="text-gray-900 dark:text-white"
            >{tr("settings.security.require_uppercase")}</span
          >
        </label>
        <label
          class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <input
            type="checkbox"
            bind:checked={requireLowercase}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span class="text-gray-900 dark:text-white"
            >{tr("settings.security.require_lowercase")}</span
          >
        </label>
        <label
          class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <input
            type="checkbox"
            bind:checked={requireNumbers}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span class="text-gray-900 dark:text-white"
            >{tr("settings.security.require_numbers")}</span
          >
        </label>
        <label
          class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <input
            type="checkbox"
            bind:checked={requireSpecialChars}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span class="text-gray-900 dark:text-white"
            >{tr("settings.security.require_special")}</span
          >
        </label>
      </div>
    </div>

    <!-- 2FA Policy -->
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center gap-3 mb-6">
        <div
          class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center"
        >
          <i class="bi bi-shield-lock-fill text-green-600 dark:text-green-400"
          ></i>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {tr("settings.security.2fa_policy")}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("settings.security.2fa_policy_desc")}
          </p>
        </div>
      </div>

      <div class="space-y-3">
        <label
          class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="font-medium text-gray-900 dark:text-white"
              >{tr("settings.security.require_2fa_all")}</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("settings.security.require_2fa_all_desc")}
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={require2FA}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>

        <label
          class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="font-medium text-gray-900 dark:text-white"
              >{tr("settings.security.require_2fa_admins")}</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("settings.security.require_2fa_admins_desc")}
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={enforce2FAForAdmins}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>

        <div class="p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
          <span class="font-medium text-gray-900 dark:text-white block mb-3"
            >{tr("settings.security.allowed_2fa_methods")}</span
          >
          <div class="flex flex-wrap gap-3">
            <label
              class="flex items-center gap-2 px-3 py-2 bg-white dark:bg-gray-600 rounded-lg cursor-pointer border border-gray-200 dark:border-gray-500"
            >
              <input
                type="checkbox"
                checked={allowed2FAMethods.includes("totp")}
                onchange={() => toggle2FAMethod("totp")}
                class="w-4 h-4 rounded border-gray-300 text-blue-600"
              />
              <i class="bi bi-phone"></i>
              <span>TOTP (Authenticator)</span>
            </label>
            <label
              class="flex items-center gap-2 px-3 py-2 bg-white dark:bg-gray-600 rounded-lg cursor-pointer border border-gray-200 dark:border-gray-500 opacity-50"
            >
              <input
                type="checkbox"
                disabled
                class="w-4 h-4 rounded border-gray-300 text-blue-600"
              />
              <i class="bi bi-chat-dots"></i>
              <span>SMS ({tr("common.coming_soon")})</span>
            </label>
            <label
              class="flex items-center gap-2 px-3 py-2 bg-white dark:bg-gray-600 rounded-lg cursor-pointer border border-gray-200 dark:border-gray-500 opacity-50"
            >
              <input
                type="checkbox"
                disabled
                class="w-4 h-4 rounded border-gray-300 text-blue-600"
              />
              <i class="bi bi-envelope"></i>
              <span>Email ({tr("common.coming_soon")})</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Session & Login Security -->
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center gap-3 mb-6">
        <div
          class="w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center"
        >
          <i class="bi bi-clock-history text-blue-600 dark:text-blue-400"></i>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {tr("settings.security.session_policy")}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("settings.security.session_policy_desc")}
          </p>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Session Timeout -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("settings.security.session_timeout")}
          </label>
          <select
            bind:value={sessionTimeoutMinutes}
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          >
            <option value={30}>30 {tr("common.minutes")}</option>
            <option value={60}>1 {tr("common.hour")}</option>
            <option value={240}>4 {tr("common.hours")}</option>
            <option value={480}>8 {tr("common.hours")}</option>
            <option value={1440}>24 {tr("common.hours")}</option>
            <option value={10080}>7 {tr("common.days")}</option>
          </select>
        </div>

        <!-- Max Concurrent Sessions -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("settings.security.max_sessions")}
          </label>
          <input
            type="number"
            bind:value={maxConcurrentSessions}
            min="1"
            max="20"
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <!-- Max Login Attempts -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("settings.security.max_login_attempts")}
          </label>
          <input
            type="number"
            bind:value={maxLoginAttempts}
            min="3"
            max="20"
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <!-- Lockout Duration -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("settings.security.lockout_duration")}
          </label>
          <select
            bind:value={lockoutDurationMinutes}
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          >
            <option value={5}>5 {tr("common.minutes")}</option>
            <option value={15}>15 {tr("common.minutes")}</option>
            <option value={30}>30 {tr("common.minutes")}</option>
            <option value={60}>1 {tr("common.hour")}</option>
            <option value={1440}>24 {tr("common.hours")}</option>
          </select>
        </div>
      </div>

      <div class="mt-4">
        <label
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="font-medium text-gray-900 dark:text-white"
              >{tr("settings.security.reauth_sensitive")}</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("settings.security.reauth_sensitive_desc")}
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={enforceReauthForSensitive}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>
      </div>
    </div>

    <!-- IP Security -->
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center gap-3 mb-6">
        <div
          class="w-10 h-10 rounded-lg bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center"
        >
          <i class="bi bi-globe text-amber-600 dark:text-amber-400"></i>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {tr("settings.security.ip_security")}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("settings.security.ip_security_desc")}
          </p>
        </div>
      </div>

      <div class="space-y-4">
        <!-- IP Whitelist -->
        <div>
          <label
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer mb-2"
          >
            <div>
              <span class="font-medium text-gray-900 dark:text-white"
                >{tr("settings.security.ip_whitelist")}</span
              >
              <p class="text-sm text-gray-500 dark:text-gray-400">
                {tr("settings.security.ip_whitelist_desc")}
              </p>
            </div>
            <input
              type="checkbox"
              bind:checked={enableIPWhitelist}
              class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
          </label>
          {#if enableIPWhitelist}
            <textarea
              bind:value={ipWhitelist}
              placeholder="192.168.1.0/24&#10;10.0.0.0/8"
              rows="3"
              class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 font-mono text-sm"
            ></textarea>
          {/if}
        </div>

        <!-- IP Blacklist -->
        <div>
          <label
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer mb-2"
          >
            <div>
              <span class="font-medium text-gray-900 dark:text-white"
                >{tr("settings.security.ip_blacklist")}</span
              >
              <p class="text-sm text-gray-500 dark:text-gray-400">
                {tr("settings.security.ip_blacklist_desc")}
              </p>
            </div>
            <input
              type="checkbox"
              bind:checked={enableIPBlacklist}
              class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
          </label>
          {#if enableIPBlacklist}
            <textarea
              bind:value={ipBlacklist}
              placeholder="192.168.1.100&#10;10.0.0.50"
              rows="3"
              class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 font-mono text-sm"
            ></textarea>
          {/if}
        </div>

        <label
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <div>
            <span class="font-medium text-gray-900 dark:text-white"
              >{tr("settings.security.block_tor")}</span
            >
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("settings.security.block_tor_desc")}
            </p>
          </div>
          <input
            type="checkbox"
            bind:checked={blockTorExits}
            class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
        </label>
      </div>
    </div>

    <!-- Save Button -->
    <div class="flex justify-end">
      <button
        onclick={savePolicy}
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
