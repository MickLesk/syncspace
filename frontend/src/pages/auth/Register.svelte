<script>
  import { currentLang } from "../../stores/ui.js";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import { auth } from "../../stores/auth";
  import { setup, auth as authApi } from "../../lib/api.js";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

  let username = $state("");
  let email = $state("");
  let password = $state("");
  let passwordConfirm = $state("");
  let loading = $state(false);
  let error = $state("");
  let registrationEnabled = $state(null);

  $effect(() => {
    checkRegistrationStatus();
  });

  async function checkRegistrationStatus() {
    try {
      const data = await setup.status();
      // Check if registration is allowed via API response
      registrationEnabled =
        data.setup_required === false &&
        (data.registration_enabled === true ||
          data.allow_registration === true);
    } catch (err) {
      console.error("Failed to check registration status:", err);
      registrationEnabled = false;
    }
  }

  async function handleRegister() {
    error = "";

    // Validation
    if (username.length < 3) {
      error = "Username must be at least 3 characters";
      return;
    }

    if (password.length < 8) {
      error = "Password must be at least 8 characters";
      return;
    }

    if (password !== passwordConfirm) {
      error = "Passwords do not match";
      return;
    }

    if (email && !isValidEmail(email)) {
      error = "Please enter a valid email address";
      return;
    }

    loading = true;

    try {
      const data = await authApi.register(
        username,
        password,
        email || undefined
      );

      // Auto-login after successful registration
      auth.login(data.token, {
        id: data.user.id,
        username: data.user.username,
        totp_enabled: data.user.totp_enabled || false,
      });

      // Redirect to main app
      window.location.hash = "#/files";
    } catch (err) {
      error = err.message || "Registration failed. Please try again.";
    } finally {
      loading = false;
    }
  }

  function isValidEmail(email) {
    const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return re.test(email);
  }
</script>

<div
  class="min-h-screen flex items-center justify-center p-8 bg-gradient-to-br from-indigo-500 to-purple-600"
>
  <div class="w-full max-w-md">
    <div class="text-center mb-8">
      <div class="mb-4">
        <i class="bi bi-person-plus-fill text-5xl text-white" aria-hidden="true"
        ></i>
      </div>
      <h1 class="text-3xl font-bold text-white">Create Account</h1>
      <p class="text-white/80 mt-2">
        Join SyncSpace and start syncing your files
      </p>
    </div>

    {#if registrationEnabled === false}
      <ModernCard>
        <div class="text-center py-8">
          <i
            class="bi bi-x-circle text-5xl text-red-500 mb-4"
            aria-hidden="true"
          ></i>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            Registration Disabled
          </h2>
          <p class="text-gray-600 dark:text-gray-400">
            User registration is currently disabled by the administrator.
          </p>
          <ModernButton
            variant="ghost"
            onClick={() => (window.location.hash = "#/login")}
            class="mt-4"
          >
            Back to Login
          </ModernButton>
        </div>
      </ModernCard>
    {:else if registrationEnabled === true}
      <ModernCard>
        <form
          onsubmit={(e) => {
            e.preventDefault();
            handleRegister();
          }}
        >
          <div class="space-y-4">
            {#if error}
              <div
                class="flex items-center gap-2 p-4 rounded-lg bg-red-500/10 text-red-600 border border-red-500/30"
              >
                <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
                <span>{error}</span>
              </div>
            {/if}

            <!-- Username -->
            <div class="mb-4">
              <label
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                for="username"
              >
                Username *
              </label>
              <input
                id="username"
                type="text"
                bind:value={username}
                class="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                placeholder="Enter username"
                required
                minlength="3"
                disabled={loading}
              />
              <span class="text-xs text-gray-500 dark:text-gray-400 mt-1"
                >Minimum 3 characters</span
              >
            </div>

            <!-- Email (optional) -->
            <div class="mb-4">
              <label
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                for="email"
              >
                Email (optional)
              </label>
              <input
                id="email"
                type="email"
                bind:value={email}
                class="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                placeholder="your@email.com"
                disabled={loading}
              />
            </div>

            <!-- Password -->
            <div class="mb-4">
              <label
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                for="password"
              >
                {tr("password")} *
              </label>
              <input
                id="password"
                type="password"
                bind:value={password}
                class="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                placeholder="••••••••"
                required
                minlength="8"
                disabled={loading}
              />
              <span class="text-xs text-gray-500 dark:text-gray-400 mt-1"
                >{tr("passwordMinimum8Chars")}</span
              >
            </div>

            <!-- Password Confirmation -->
            <div class="mb-4">
              <label
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                for="password-confirm"
              >
                {tr("confirmPassword")} *
              </label>
              <input
                id="password-confirm"
                type="password"
                bind:value={passwordConfirm}
                class="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                placeholder="••••••••"
                required
                minlength="8"
                disabled={loading}
              />
            </div>

            <!-- Submit Button -->
            <ModernButton
              type="submit"
              variant="primary"
              fullWidth={true}
              {loading}
              disabled={loading}
            >
              {loading ? tr("creatingAccount") : tr("createAccount")}
            </ModernButton>

            <!-- Login Link -->
            <div class="text-center mt-4">
              <span class="text-gray-600 dark:text-gray-400">
                {tr("alreadyHaveAccount")}
              </span>
              <a
                href="#/login"
                class="text-primary-600 hover:text-primary-700 font-medium ml-1"
              >
                {tr("signIn")}
              </a>
            </div>
          </div>
        </form>
      </ModernCard>
    {:else}
      <ModernCard>
        <div class="text-center py-8">
          <div
            class="w-8 h-8 border-2 border-primary-500 border-t-transparent rounded-full animate-spin mx-auto"
          ></div>
          <p class="text-gray-600 dark:text-gray-400 mt-4">
            Checking registration status...
          </p>
        </div>
      </ModernCard>
    {/if}
  </div>
</div>
