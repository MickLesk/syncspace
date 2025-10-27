<script>
  import { auth } from "../stores/auth.js";
  import api from "../lib/api.js";

  let username = $state("");
  let password = $state("");
  let twoFactorCode = $state("");
  let showTwoFactor = $state(false);
  let rememberMe = $state(false);
  let loading = $state(false);
  let errorMessage = $state("");

  async function handleLogin(e) {
    e.preventDefault();

    if (!username || !password) {
      errorMessage = "Please fill in all fields";
      return;
    }

    loading = true;
    errorMessage = "";

    try {
      // Step 1: Try to login without 2FA first
      const loginData = { username, password };

      // Add 2FA code if we're in 2FA mode
      if (showTwoFactor && twoFactorCode) {
        loginData.totp_code = twoFactorCode;
      }

      const response = await fetch("http://localhost:8080/api/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(loginData),
      });

      const data = await response.json();

      if (!response.ok) {
        // Check if 2FA is required
        if (data.requires_2fa === true) {
          showTwoFactor = true;
          errorMessage = "Please enter your 2FA code";
          loading = false;
          return;
        }

        throw new Error(data.error || "Login failed");
      }

      // Login successful - store token and update auth store
      if (data.token) {
        localStorage.setItem("authToken", data.token);

        // Update auth store with user info
        auth.login({
          username: data.user?.username || username,
          email: data.user?.email || "",
          id: data.user?.id || null,
        });

        // Redirect to main app
        window.location.hash = "#/files";
        // Force reload to trigger App.svelte onMount
        setTimeout(() => window.location.reload(), 100);
      } else {
        throw new Error("No token received");
      }
    } catch (err) {
      console.error("Login error:", err);
      errorMessage = err.message || "Login failed. Please try again.";
      loading = false;
    }
  }
</script>

<div class="login-container">
  <div class="login-card">
    <div class="login-header">
      <i class="bi bi-cloud-fill logo-icon"></i>
      <h1 class="logo-text">SyncSpace</h1>
      <p class="tagline">Secure Cloud Storage</p>
    </div>

    <form onsubmit={handleLogin}>
      {#if errorMessage}
        <div class="alert alert-error mb-4">
          <i class="bi bi-exclamation-triangle-fill"></i>
          <span>{errorMessage}</span>
        </div>
      {/if}

      <div class="form-control">
        <label class="label"><span>Username</span></label>
        <input
          type="text"
          bind:value={username}
          class="input input-bordered"
          placeholder="admin"
          disabled={loading}
        />
      </div>

      <div class="form-control">
        <label class="label"><span>Password</span></label>
        <input
          type="password"
          bind:value={password}
          class="input input-bordered"
          placeholder="••••••••"
          disabled={loading}
        />
      </div>

      {#if showTwoFactor}
        <div class="form-control fade-in">
          <label class="label"><span>2FA Code</span></label>
          <input
            type="text"
            bind:value={twoFactorCode}
            class="input input-bordered"
            placeholder="Enter 6-digit code"
            maxlength="6"
            disabled={loading}
          />
        </div>
      {/if}

      <div class="flex justify-between items-center mb-4">
        <label class="label cursor-pointer gap-2">
          <input
            type="checkbox"
            bind:checked={rememberMe}
            class="checkbox checkbox-sm checkbox-primary"
          />
          <span class="text-sm">Remember me</span>
        </label>
        <a href="#/forgot" class="link link-primary text-sm">Forgot password?</a
        >
      </div>

      <button
        type="submit"
        class="btn btn-primary w-full gap-2"
        disabled={loading}
      >
        {#if loading}<span class="loading loading-spinner loading-sm"
          ></span>{/if}
        {showTwoFactor ? "Verify & Login" : "Login"}
      </button>
    </form>

    <div class="divider">OR</div>

    <div class="social-login">
      <button class="btn btn-outline gap-2"
        ><i class="bi bi-google"></i> Google</button
      >
      <button class="btn btn-outline gap-2"
        ><i class="bi bi-github"></i> GitHub</button
      >
    </div>

    <p class="signup-link">
      Don't have an account? <a href="#/signup" class="link link-primary"
        >Sign up</a
      >
    </p>
  </div>
</div>

<style>
  .login-container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(
      135deg,
      hsl(var(--p)) 0%,
      hsl(var(--s)) 50%,
      hsl(var(--a)) 100%
    );
    position: relative;
    overflow: hidden;
  }
  .login-container::before {
    content: "";
    position: absolute;
    width: 200%;
    height: 200%;
    background: radial-gradient(
      circle,
      hsl(var(--p) / 0.3) 0%,
      transparent 70%
    );
    animation: pulse 8s ease-in-out infinite;
  }
  .login-card {
    width: 100%;
    max-width: 420px;
    padding: 2.5rem;
    background: hsl(var(--b1) / 0.95);
    backdrop-filter: blur(20px);
    border-radius: var(--rounded-box);
    box-shadow:
      0 20px 60px hsl(var(--bc) / 0.3),
      0 0 0 1px hsl(var(--bc) / 0.1);
    animation: slideUp 0.6s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    z-index: 1;
  }
  .login-header {
    text-align: center;
    margin-bottom: 2rem;
  }
  .logo-icon {
    font-size: 3.5rem;
    color: hsl(var(--p));
    margin-bottom: 0.75rem;
    animation: bounce 2s ease-in-out infinite;
  }
  .logo-text {
    font-size: 2.25rem;
    font-weight: 800;
    margin: 0;
    background: linear-gradient(135deg, hsl(var(--p)), hsl(var(--s)));
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }
  .tagline {
    color: hsl(var(--bc) / 0.6);
    margin-top: 0.5rem;
    font-weight: 500;
  }
  .social-login {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
    margin-bottom: 1.5rem;
  }
  .signup-link {
    text-align: center;
    color: hsl(var(--bc) / 0.6);
    font-size: 0.875rem;
  }
  .fade-in {
    animation: fadeIn 0.4s ease-in;
  }
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(40px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
  @keyframes pulse {
    0%,
    100% {
      transform: translate(-50%, -50%) scale(1);
      opacity: 0.5;
    }
    50% {
      transform: translate(-50%, -50%) scale(1.1);
      opacity: 0.3;
    }
  }
  @keyframes bounce {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-10px);
    }
  }
</style>
