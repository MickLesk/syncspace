<script>
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import { auth } from "../stores/auth";
  import { setup, auth as authApi } from "../lib/api.js";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";

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
      const data = await authApi.register(username, password, email || undefined);

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

<div class="register-page">
  <div class="register-container">
    <div class="register-header">
      <div class="logo">
        <i
          class="bi bi-person-plus-fill text-5xl text-primary-600"
          aria-hidden="true"
        ></i>
      </div>
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
        Create Account
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mt-2">
        Join SyncSpace and start syncing your files
      </p>
    </div>

    {#if registrationEnabled === false}
      <ModernCard>
        <div class="text-center py-8">
          <i class="bi bi-x-circle text-5xl text-error mb-4" aria-hidden="true"
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
              <div class="alert alert-error">
                <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
                <span>{error}</span>
              </div>
            {/if}

            <!-- Username -->
            <div class="form-control">
              <label class="label" for="username">
                <span class="label-text">Username *</span>
              </label>
              <input
                id="username"
                type="text"
                bind:value={username}
                class="input input-bordered w-full"
                placeholder="Enter username"
                required
                minlength="3"
                disabled={loading}
              />
              <div class="label">
                <span class="label-text-alt">Minimum 3 characters</span>
              </div>
            </div>

            <!-- Email (optional) -->
            <div class="form-control">
              <label class="label" for="email">
                <span class="label-text">Email (optional)</span>
              </label>
              <input
                id="email"
                type="email"
                bind:value={email}
                class="input input-bordered w-full"
                placeholder="your@email.com"
                disabled={loading}
              />
            </div>

            <!-- Password -->
            <div class="form-control">
              <label class="label" for="password">
                <span class="label-text">{tr("password")} *</span>
              </label>
              <input
                id="password"
                type="password"
                bind:value={password}
                class="input input-bordered w-full"
                placeholder="••••••••"
                required
                minlength="8"
                disabled={loading}
              />
              <div class="label">
                <span class="label-text-alt">{tr("passwordMinimum8Chars")}</span
                >
              </div>
            </div>

            <!-- Password Confirmation -->
            <div class="form-control">
              <label class="label" for="password-confirm">
                <span class="label-text">{tr("confirmPassword")} *</span>
              </label>
              <input
                id="password-confirm"
                type="password"
                bind:value={passwordConfirm}
                class="input input-bordered w-full"
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
          <span class="loading loading-spinner loading-lg text-primary"></span>
          <p class="text-gray-600 dark:text-gray-400 mt-4">
            Checking registration status...
          </p>
        </div>
      </ModernCard>
    {/if}
  </div>
</div>

<style>
  .register-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }

  .register-container {
    width: 100%;
    max-width: 450px;
  }

  .register-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .logo {
    margin-bottom: 1rem;
  }

  .form-control {
    margin-bottom: 1rem;
  }

  .alert {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    border-radius: 0.5rem;
  }

  .alert-error {
    background-color: rgba(239, 68, 68, 0.1);
    color: #dc2626;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }
</style>
