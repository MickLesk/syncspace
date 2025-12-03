<script>
  import { onMount } from "svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import { auth } from "../../stores/auth.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import QRCode from "qrcode";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let twoFAEnabled = $state(false);
  let twoFASetup = $state(null); // { secret, qr_code_url }
  let qrCodeDataUrl = $state(""); // Generated QR code as data URL
  let showSetup = $state(false);
  let verificationCode = $state("");
  let loading = $state(false);
  let error = $state("");
  let success = $state("");

  onMount(async () => {
    await loadSecurityStatus();
  });

  async function loadSecurityStatus() {
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

        // Generate QR code from the otpauth URL
        try {
          qrCodeDataUrl = await QRCode.toDataURL(twoFASetup.qr_code_url, {
            width: 256,
            margin: 2,
            color: {
              dark: "#000000",
              light: "#FFFFFF",
            },
          });
        } catch (qrError) {
          console.error("QR code generation error:", qrError);
          error = tr("failedToGenerateQRCode2");
          return;
        }

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
          body: JSON.stringify({
            totp_code: verificationCode,
          }),
        }
      );

      if (response.ok) {
        success = tr("twoFAEnabledSuccess2");
        twoFAEnabled = true;
        showSetup = false;
        twoFASetup = null;
        verificationCode = "";
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
    if (!confirm(tr("areSureDisable2FA"))) {
      return;
    }

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
      } else {
        error = tr("failedToDisable2FA2");
      }
    } catch (e) {
      console.error("2FA disable error:", e);
      error = "Failed to disable 2FA. Please try again.";
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
</script>

<div class="space-y-6">
  <!-- 2FA Status Card -->
  <ModernCard variant="glass">
    {#snippet children()}
      <div class="p-6 space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <i
              class="bi bi-shield-lock-fill text-3xl text-primary-600 dark:text-primary-400"
            ></i>
            <div>
              <h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">
                Two-Factor Authentication (2FA)
              </h2>
              <p class="text-sm text-gray-600 dark:text-gray-400">
                Add an extra layer of security to your account
              </p>
            </div>
          </div>
          <div
            class="px-4 py-2 rounded-full text-sm font-semibold {twoFAEnabled
              ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
              : 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400'}"
          >
            {twoFAEnabled ? "Enabled" : "Disabled"}
          </div>
        </div>

        {#if error}
          <div
            class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 rounded-lg"
          >
            <i class="bi bi-exclamation-triangle-fill mr-2" aria-hidden="true"></i>
            {error}
          </div>
        {/if}

        {#if success}
          <div
            class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 text-green-700 dark:text-green-400 px-4 py-3 rounded-lg"
          >
            <i class="bi bi-check-circle-fill mr-2" aria-hidden="true"></i>
            {success}
          </div>
        {/if}

        {#if !showSetup}
          <div class="flex gap-3">
            {#if !twoFAEnabled}
              <ModernButton
                variant="primary"
                onclick={setup2FA}
                disabled={loading}
                class="flex-1"
              >
                <i class="bi bi-shield-plus mr-2" aria-hidden="true"></i>
                {loading ? "Loading..." : "Enable 2FA"}
              </ModernButton>
            {:else}
              <ModernButton
                variant="danger"
                onclick={disable2FA}
                disabled={loading}
                class="flex-1"
              >
                <i class="bi bi-shield-x mr-2" aria-hidden="true"></i>
                {loading ? "Disabling..." : "Disable 2FA"}
              </ModernButton>
            {/if}
          </div>
        {/if}
      </div>
    {/snippet}
  </ModernCard>

  <!-- 2FA Setup Card -->
  {#if showSetup && twoFASetup}
    <ModernCard variant="glass">
      {#snippet children()}
        <div class="p-6 space-y-6">
          <div class="text-center">
            <h3 class="text-lg font-bold text-gray-900 dark:text-gray-100 mb-2">
              Scan QR Code
            </h3>
            <p class="text-sm text-gray-600 dark:text-gray-400 mb-6">
              Use an authenticator app like Google Authenticator, Authy, or
              Microsoft Authenticator to scan this QR code
            </p>

            <!-- QR Code -->
            <div
              class="bg-white dark:bg-gray-700 p-6 rounded-xl inline-block shadow-lg mb-6"
            >
              {#if qrCodeDataUrl}
                <img src={qrCodeDataUrl} alt="2FA QR Code" class="w-64 h-64" />
              {:else}
                <div class="w-64 h-64 flex items-center justify-center">
                  <div
                    class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"
                  ></div>
                </div>
              {/if}
            </div>

            <!-- Manual Entry -->
            <details class="text-left mb-6">
              <summary
                class="cursor-pointer text-sm font-semibold text-primary-600 dark:text-primary-400 hover:text-primary-700 dark:hover:text-primary-300"
              >
                Can't scan? Enter manually
              </summary>
              <div
                class="mt-3 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700"
              >
                <p class="text-xs text-gray-600 dark:text-gray-400 mb-2">
                  Secret Key:
                </p>
                <code
                  class="block p-3 bg-white dark:bg-gray-900 rounded font-mono text-sm break-all border border-gray-300 dark:border-gray-600"
                >
                  {twoFASetup.secret}
                </code>
              </div>
            </details>
          </div>

          <!-- Verification -->
          <div class="space-y-4">
            <div>
              <label
                for="verification-code"
                class="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2"
              >
                Enter 6-digit code from your authenticator app
              </label>
              <input
                id="verification-code"
                type="text"
                bind:value={verificationCode}
                placeholder="000000"
                maxlength="6"
                class="w-full px-4 py-3 text-center text-2xl font-mono tracking-wider border-2 border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20 outline-none transition"
                onkeypress={(e) => {
                  if (!/[0-9]/.test(e.key)) {
                    e.preventDefault();
                  }
                }}
              />
            </div>

            <div class="flex gap-3">
              <ModernButton
                variant="secondary"
                onclick={cancelSetup}
                class="flex-1"
              >
                Cancel
              </ModernButton>
              <ModernButton
                variant="primary"
                onclick={enable2FA}
                disabled={loading || verificationCode.length !== 6}
                class="flex-1"
              >
                {loading ? "Verifying..." : "Verify & Enable"}
              </ModernButton>
            </div>
          </div>
        </div>
      {/snippet}
    </ModernCard>
  {/if}

  <!-- Security Tips Card -->
  <ModernCard variant="glass">
    {#snippet children()}
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-3 mb-4">
          <i
            class="bi bi-info-circle-fill text-2xl text-blue-600 dark:text-blue-400"
          ></i>
          <h3 class="text-lg font-bold text-gray-900 dark:text-gray-100">
            Security Tips
          </h3>
        </div>

        <ul class="space-y-3 text-sm text-gray-700 dark:text-gray-300">
          <li class="flex gap-3">
            <i class="bi bi-check-circle-fill text-green-500 mt-0.5" aria-hidden="true"></i>
            <span
              >Use a strong, unique password with at least 12 characters</span
            >
          </li>
          <li class="flex gap-3">
            <i class="bi bi-check-circle-fill text-green-500 mt-0.5" aria-hidden="true"></i>
            <span>Enable 2FA for enhanced account security</span>
          </li>
          <li class="flex gap-3">
            <i class="bi bi-check-circle-fill text-green-500 mt-0.5" aria-hidden="true"></i>
            <span
              >Save your 2FA backup codes in a secure location (coming soon)</span
            >
          </li>
          <li class="flex gap-3">
            <i class="bi bi-check-circle-fill text-green-500 mt-0.5" aria-hidden="true"></i>
            <span>Regularly review your active sessions and devices</span>
          </li>
        </ul>
      </div>
    {/snippet}
  </ModernCard>

  <!-- Change Password Card -->
  <ModernCard variant="glass">
    {#snippet children()}
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-3">
          <i
            class="bi bi-key-fill text-2xl text-primary-600 dark:text-primary-400"
          ></i>
          <div>
            <h3 class="text-lg font-bold text-gray-900 dark:text-gray-100">
              Change Password
            </h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Update your account password
            </p>
          </div>
        </div>

        <ModernButton variant="secondary" class="w-full">
          <i class="bi bi-arrow-clockwise mr-2" aria-hidden="true"></i>
          Change Password
        </ModernButton>
      </div>
    {/snippet}
  </ModernCard>
</div>

<style>
  details summary {
    list-style: none;
  }

  details summary::-webkit-details-marker {
    display: none;
  }
</style>
