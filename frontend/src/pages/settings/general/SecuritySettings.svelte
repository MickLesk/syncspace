<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import QRCode from "qrcode";
  import { users, auth as authApi } from "../../lib/api.js";

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
  <div class="flex justify-center p-16"><div class="w-9 h-9 border-3 border-gray-200 border-t-green-500 rounded-full animate-spin"></div></div>
{:else}
  {#if error}
    <div class="flex items-center gap-2 px-4 py-3 rounded-lg mb-6 text-sm font-medium bg-red-100 text-red-800 dark:bg-red-500/20 dark:text-red-300">
      <i class="bi bi-exclamation-circle-fill"></i>
      {error}
    </div>
  {/if}
  {#if success}
    <div class="flex items-center gap-2 px-4 py-3 rounded-lg mb-6 text-sm font-medium bg-green-100 text-green-800 dark:bg-green-500/20 dark:text-green-300">
      <i class="bi bi-check-circle-fill"></i>
      {success}
    </div>
  {/if}

  <div class="grid grid-cols-[repeat(auto-fit,minmax(340px,1fr))] gap-6 max-sm:grid-cols-1">
    <!-- 2FA Status Card -->
    <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-5 col-span-full">
      <div class="flex items-start gap-3.5 mb-5 flex-wrap">
        <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0 bg-green-100 text-green-600 dark:bg-green-500/20 dark:text-green-500">
          <i class="bi bi-shield-lock-fill"></i>
        </div>
        <div class="flex-1 min-w-[200px]">
          <h3 class="text-base font-semibold text-gray-900 dark:text-gray-50 m-0 mb-1">{tr("twoFactorAuthentication")}</h3>
          <p class="text-[0.8125rem] text-gray-500 dark:text-gray-400 m-0">{tr("addExtraLayerOfSecurity")}</p>
        </div>
        <span class="px-3 py-1 rounded-full text-xs font-semibold {twoFAEnabled ? 'bg-green-100 text-green-800 dark:bg-green-500/20 dark:text-green-300' : 'bg-red-100 text-red-800 dark:bg-red-500/20 dark:text-red-300'}">
          {twoFAEnabled ? tr("enabled") : tr("disabled")}
        </span>
      </div>

      {#if !showSetup}
        <div class="flex flex-col gap-4">
          <div class="flex items-center gap-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <i class="bi {twoFAEnabled ? 'bi-shield-fill-check' : 'bi-shield-fill-x'} text-3xl {twoFAEnabled ? 'text-green-500' : 'text-red-600'}"></i>
            <div class="flex flex-col gap-0.5">
              <strong class="text-sm text-gray-900 dark:text-gray-50">{twoFAEnabled ? tr("twoFAIsActive") : tr("twoFAIsNotActive")}</strong>
              <span class="text-[0.8125rem] text-gray-500 dark:text-gray-400">{twoFAEnabled ? tr("yourAccountIsProtected") : tr("enableTwoFAForBetterSecurity")}</span>
            </div>
          </div>
          <div class="flex gap-2">
            {#if twoFAEnabled}
              <button class="inline-flex items-center gap-2 px-4 py-2 bg-red-600 hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed text-white border-none rounded-lg font-medium text-sm cursor-pointer transition-colors" onclick={disable2FA} disabled={loading}>
                <i class="bi bi-shield-x"></i>
                {tr("disable2FA")}
              </button>
            {:else}
              <button class="inline-flex items-center gap-2 px-4 py-2 bg-green-500 hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed text-white border-none rounded-lg font-medium text-sm cursor-pointer transition-colors" onclick={setup2FA} disabled={loading}>
                <i class="bi bi-shield-plus"></i>
                {tr("enable2FA")}
              </button>
            {/if}
          </div>
        </div>
      {:else}
        <!-- 2FA Setup -->
        <div class="grid grid-cols-[auto_1fr] gap-8 items-start max-sm:grid-cols-1">
          <div class="flex flex-col items-center gap-4 max-sm:order-1">
            {#if qrCodeDataUrl}
              <div class="p-4 bg-white rounded-lg border border-gray-200">
                <img src={qrCodeDataUrl} alt="2FA QR Code" class="block w-[180px] h-[180px]" />
              </div>
            {/if}
            <div class="text-center">
              <span class="block text-xs text-gray-500 mb-1">{tr("manualEntryKey")}:</span>
              <code class="text-[0.8125rem] bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded font-mono">{twoFASetup?.secret || ""}</code>
            </div>
          </div>
          <div class="flex flex-col gap-4 max-sm:order-2">
            <label for="verification-code" class="text-sm font-medium text-gray-700 dark:text-gray-300">{tr("enterVerificationCode")}</label>
            <input
              id="verification-code"
              type="text"
              class="px-4 py-3 text-2xl text-center tracking-[0.5rem] border border-gray-200 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-gray-50 focus:outline-none focus:border-green-500"
              placeholder="000000"
              maxlength="6"
              bind:value={verificationCode}
            />
            <div class="flex gap-2 justify-end">
              <button class="inline-flex items-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium text-sm cursor-pointer transition-colors" onclick={cancelSetup} disabled={loading}>{tr("cancel")}</button>
              <button class="inline-flex items-center gap-2 px-4 py-2 bg-green-500 hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed text-white border-none rounded-lg font-medium text-sm cursor-pointer transition-colors" onclick={enable2FA} disabled={loading || verificationCode.length !== 6}>
                {loading ? tr("verifying") : tr("verify")}
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Password Card -->
    <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-5">
      <div class="flex items-start gap-3.5 mb-5 flex-wrap">
        <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0 bg-amber-100 text-amber-600 dark:bg-amber-500/20 dark:text-amber-400"><i class="bi bi-key-fill"></i></div>
        <div class="flex-1 min-w-[200px]">
          <h3 class="text-base font-semibold text-gray-900 dark:text-gray-50 m-0 mb-1">{tr("password")}</h3>
          <p class="text-[0.8125rem] text-gray-500 dark:text-gray-400 m-0">{tr("changeYourPassword")}</p>
        </div>
      </div>
      {#if !showPasswordChange}
        <div class="flex flex-col gap-4">
          <div class="flex items-center gap-2 text-gray-500 dark:text-gray-400 text-sm">
            <i class="bi bi-asterisk"></i>
            <span>{tr("lastPasswordChange")}: {tr("unknown")}</span>
          </div>
          <button class="inline-flex items-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium text-sm cursor-pointer transition-colors" onclick={() => (showPasswordChange = true)}>
            <i class="bi bi-pencil"></i>
            {tr("changePassword")}
          </button>
        </div>
      {:else}
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-1.5">
            <label for="current-pw" class="text-[0.8125rem] font-medium text-gray-700 dark:text-gray-300">{tr("currentPassword")}</label>
            <input id="current-pw" type="password" class="px-3 py-2.5 border border-gray-200 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-gray-50 text-sm focus:outline-none focus:border-green-500" bind:value={currentPassword} />
          </div>
          <div class="flex flex-col gap-1.5">
            <label for="new-pw" class="text-[0.8125rem] font-medium text-gray-700 dark:text-gray-300">{tr("newPassword")}</label>
            <input id="new-pw" type="password" class="px-3 py-2.5 border border-gray-200 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-gray-50 text-sm focus:outline-none focus:border-green-500" bind:value={newPassword} />
          </div>
          <div class="flex flex-col gap-1.5">
            <label for="confirm-pw" class="text-[0.8125rem] font-medium text-gray-700 dark:text-gray-300">{tr("confirmPassword")}</label>
            <input id="confirm-pw" type="password" class="px-3 py-2.5 border border-gray-200 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-gray-50 text-sm focus:outline-none focus:border-green-500" bind:value={confirmPassword} />
          </div>
          <div class="flex gap-2 justify-end mt-2">
            <button class="inline-flex items-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium text-sm cursor-pointer transition-colors" onclick={() => { showPasswordChange = false; error = ""; }}>{tr("cancel")}</button>
            <button class="inline-flex items-center gap-2 px-4 py-2 bg-green-500 hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed text-white border-none rounded-lg font-medium text-sm cursor-pointer transition-colors" onclick={changePassword} disabled={loading}>
              {loading ? tr("saving") : tr("savePassword")}
            </button>
          </div>
        </div>
      {/if}
    </div>

    <!-- Sessions Card -->
    <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-5">
      <div class="flex items-start gap-3.5 mb-5 flex-wrap">
        <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0 bg-blue-100 text-blue-600 dark:bg-blue-500/20 dark:text-blue-400"><i class="bi bi-laptop"></i></div>
        <div class="flex-1 min-w-[200px]">
          <h3 class="text-base font-semibold text-gray-900 dark:text-gray-50 m-0 mb-1">{tr("activeSessions")}</h3>
          <p class="text-[0.8125rem] text-gray-500 dark:text-gray-400 m-0">{tr("manageActiveSessions")}</p>
        </div>
      </div>
      <div class="flex flex-col gap-4">
        <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
          <div class="w-9 h-9 bg-blue-100 dark:bg-blue-500/20 text-blue-600 dark:text-blue-400 rounded-lg flex items-center justify-center">
            <i class="bi bi-display"></i>
          </div>
          <div class="flex-1 flex flex-col gap-0.5">
            <span class="text-sm font-medium text-gray-900 dark:text-gray-50">{tr("currentSession")}</span>
            <span class="text-xs text-gray-500 dark:text-gray-400">Windows · Chrome · {tr("justNow")}</span>
          </div>
          <span class="px-2 py-0.5 bg-green-100 dark:bg-green-500/20 text-green-800 dark:text-green-300 rounded-full text-[0.6875rem] font-semibold">{tr("current")}</span>
        </div>
        <button class="inline-flex items-center justify-center gap-2 w-full px-4 py-2 bg-transparent text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg font-medium text-sm cursor-pointer transition-colors">
          <i class="bi bi-box-arrow-right"></i>
          {tr("logoutAllOtherSessions")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
