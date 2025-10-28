<script>
  import { auth } from "../../stores/auth.js";
  import api from "../../lib/api.js";
  import { onMount } from "svelte";

  let username = $state("");
  let password = $state("");
  let twoFactorCode = $state("");
  let showTwoFactor = $state(false);
  let rememberMe = $state(false);
  let loading = $state(false);
  let errorMessage = $state("");
  let backendOnline = $state(false);
  let checkingBackend = $state(true);

  // Check backend status on mount and periodically
  onMount(() => {
    checkBackendStatus();
    const interval = setInterval(checkBackendStatus, 3000); // Check every 3 seconds
    return () => clearInterval(interval);
  });

  async function checkBackendStatus() {
    try {
      // Simple health check - just try to reach the backend
      const response = await fetch("http://localhost:8080/api/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: "", password: "" }),
      });
      // If we get ANY response (even error), backend is online
      backendOnline = true;
      checkingBackend = false;
    } catch (err) {
      // Network error = backend offline
      backendOnline = false;
      checkingBackend = false;
    }
  }

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

<!-- Background with animated gradient -->
<div class="login-container">
  <!-- Backend Status Indicator -->
  <div class="backend-status-indicator">
    <div class="status-circle {backendOnline ? 'online' : 'offline'}">
      <span class="status-pulse"></span>
      <span class="status-dot"></span>
    </div>
    <div class="status-text">
      {#if checkingBackend}
        <span class="text-white/90 text-xs font-medium">Checking...</span>
      {:else if backendOnline}
        <span class="text-green-300 text-xs font-semibold">Backend Online</span>
      {:else}
        <span class="text-red-300 text-xs font-semibold">Backend Offline</span>
      {/if}
    </div>
  </div>

  <!-- Animated background blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Login Card with Glassmorphism -->
  <div class="login-card">
    <!-- Logo Section -->
    <div class="text-center mb-8">
      <div class="logo-circle">
        <i class="bi bi-cloud-fill text-5xl text-blue-600 dark:text-blue-400"
        ></i>
      </div>
      <h1
        class="text-4xl font-bold mt-6 mb-2 bg-gradient-to-r from-blue-600 via-purple-600 to-pink-600 bg-clip-text text-transparent"
      >
        SyncSpace
      </h1>
      <p class="text-gray-600 dark:text-gray-400 font-medium">
        Welcome back! Please login to continue
      </p>
    </div>

    <form onsubmit={handleLogin} class="space-y-5">
      {#if errorMessage}
        <div
          class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4 flex items-start gap-3 animate-shake"
        >
          <i
            class="bi bi-exclamation-triangle-fill text-red-600 dark:text-red-400 text-xl mt-0.5"
          ></i>
          <span class="text-red-800 dark:text-red-200 text-sm font-medium"
            >{errorMessage}</span
          >
        </div>
      {/if}

      <!-- Username Input -->
      <div class="space-y-2">
        <label
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
        >
          <i class="bi bi-person-fill mr-2 text-blue-600 dark:text-blue-400"
          ></i>Username
        </label>
        <div class="relative">
          <input
            type="text"
            bind:value={username}
            class="w-full px-4 py-3 pl-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
            placeholder="Enter your username"
            disabled={loading}
          />
          <i
            class="bi bi-person absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
        </div>
      </div>

      <!-- Password Input -->
      <div class="space-y-2">
        <label
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
        >
          <i
            class="bi bi-shield-lock-fill mr-2 text-blue-600 dark:text-blue-400"
          ></i>Password
        </label>
        <div class="relative">
          <input
            type="password"
            bind:value={password}
            class="w-full px-4 py-3 pl-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
            placeholder="Enter your password"
            disabled={loading}
          />
          <i
            class="bi bi-lock absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
        </div>
      </div>

      <!-- 2FA Code Input -->
      {#if showTwoFactor}
        <div class="space-y-2 animate-fade-in">
          <label
            class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
          >
            <i
              class="bi bi-shield-check mr-2 text-green-600 dark:text-green-400"
            ></i>2FA Code
          </label>
          <div class="relative">
            <input
              type="text"
              bind:value={twoFactorCode}
              class="w-full px-4 py-3 pl-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-green-500 focus:ring-4 focus:ring-green-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed tracking-widest text-center text-lg font-mono"
              placeholder="000000"
              maxlength="6"
              disabled={loading}
            />
            <i
              class="bi bi-shield-check absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
            ></i>
          </div>
        </div>
      {/if}

      <!-- Remember Me & Forgot Password -->
      <div class="flex justify-between items-center">
        <label class="flex items-center gap-2 cursor-pointer group">
          <input
            type="checkbox"
            bind:checked={rememberMe}
            class="w-4 h-4 text-blue-600 bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded focus:ring-2 focus:ring-blue-500 accent-blue-600"
          />
          <span
            class="text-sm font-medium text-gray-700 dark:text-gray-300 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors"
            >Remember me</span
          >
        </label>
        <a
          href="#/forgot"
          class="text-sm font-semibold text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors hover:underline"
        >
          Forgot password?
        </a>
      </div>

      <!-- Login Button -->
      <button
        type="submit"
        class="w-full py-3.5 px-6 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-bold rounded-xl shadow-lg shadow-blue-500/30 hover:shadow-xl hover:shadow-blue-500/40 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 group"
        disabled={loading}
      >
        {#if loading}
          <svg
            class="animate-spin h-5 w-5"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
        {:else}
          <i
            class="bi bi-box-arrow-in-right group-hover:translate-x-1 transition-transform"
          ></i>
        {/if}
        <span>{showTwoFactor ? "Verify & Login" : "Login to Account"}</span>
      </button>
    </form>

    <!-- Divider -->
    <div class="relative my-8">
      <div class="absolute inset-0 flex items-center">
        <div class="w-full border-t border-gray-300 dark:border-gray-700"></div>
      </div>
      <div class="relative flex justify-center text-sm">
        <span
          class="px-4 bg-white dark:bg-gray-900 text-gray-500 dark:text-gray-400 font-medium"
          >Or continue with</span
        >
      </div>
    </div>

    <!-- Social Login -->
    <div class="grid grid-cols-2 gap-3">
      <button
        class="py-3 px-4 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl hover:border-gray-300 dark:hover:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-750 transition-all duration-200 flex items-center justify-center gap-2 font-medium text-gray-700 dark:text-gray-300 group"
      >
        <i
          class="bi bi-google text-lg text-red-500 group-hover:scale-110 transition-transform"
        ></i>
        <span>Google</span>
      </button>
      <button
        class="py-3 px-4 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl hover:border-gray-300 dark:hover:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-750 transition-all duration-200 flex items-center justify-center gap-2 font-medium text-gray-700 dark:text-gray-300 group"
      >
        <i
          class="bi bi-github text-lg group-hover:scale-110 transition-transform"
        ></i>
        <span>GitHub</span>
      </button>
    </div>

    <!-- Sign Up Link -->
    <p class="text-center mt-8 text-sm text-gray-600 dark:text-gray-400">
      Don't have an account?
      <a
        href="#/signup"
        class="font-bold text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 hover:underline transition-colors"
      >
        Sign up for free
      </a>
    </p>
  </div>

  <!-- Footer -->
  <div class="absolute bottom-6 text-center text-sm text-white/80">
    <p>© 2025 SyncSpace. Secure Cloud Storage Platform.</p>
  </div>
</div>

<style>
  /* Main Container with Gradient Background */
  .login-container {
    position: relative;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
    overflow: hidden;
    padding: 1rem;
  }

  /* Animated Background Blobs */
  .blob {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.4;
    animation: float 20s infinite ease-in-out;
  }

  .blob-1 {
    width: 500px;
    height: 500px;
    background: linear-gradient(135deg, #667eea, #764ba2);
    top: -10%;
    left: -10%;
    animation-delay: 0s;
  }

  .blob-2 {
    width: 400px;
    height: 400px;
    background: linear-gradient(135deg, #f093fb, #f5576c);
    top: 40%;
    right: -5%;
    animation-delay: 5s;
  }

  .blob-3 {
    width: 350px;
    height: 350px;
    background: linear-gradient(135deg, #4facfe, #00f2fe);
    bottom: -10%;
    left: 30%;
    animation-delay: 10s;
  }

  /* Glassmorphism Card */
  .login-card {
    position: relative;
    z-index: 10;
    width: 100%;
    max-width: 460px;
    padding: 2.5rem;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px) saturate(180%);
    border-radius: 24px;
    box-shadow:
      0 8px 32px 0 rgba(31, 38, 135, 0.15),
      0 0 0 1px rgba(255, 255, 255, 0.18) inset;
    animation: slideUp 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  /* Dark mode card */
  @media (prefers-color-scheme: dark) {
    .login-card {
      background: rgba(17, 24, 39, 0.95);
      box-shadow:
        0 8px 32px 0 rgba(0, 0, 0, 0.3),
        0 0 0 1px rgba(255, 255, 255, 0.08) inset;
    }
  }

  /* Logo Circle with Pulse */
  .logo-circle {
    width: 100px;
    height: 100px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea, #764ba2);
    border-radius: 50%;
    box-shadow:
      0 10px 40px rgba(102, 126, 234, 0.4),
      0 0 0 8px rgba(102, 126, 234, 0.1);
    animation: pulse 3s ease-in-out infinite;
  }

  .logo-circle i {
    color: white !important;
  }

  /* Animations */
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(60px) scale(0.9);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  @keyframes float {
    0%,
    100% {
      transform: translate(0, 0) rotate(0deg);
    }
    33% {
      transform: translate(30px, -30px) rotate(120deg);
    }
    66% {
      transform: translate(-20px, 20px) rotate(240deg);
    }
  }

  @keyframes pulse {
    0%,
    100% {
      transform: scale(1);
      box-shadow:
        0 10px 40px rgba(102, 126, 234, 0.4),
        0 0 0 8px rgba(102, 126, 234, 0.1);
    }
    50% {
      transform: scale(1.05);
      box-shadow:
        0 15px 60px rgba(102, 126, 234, 0.6),
        0 0 0 12px rgba(102, 126, 234, 0.15);
    }
  }

  @keyframes shake {
    0%,
    100% {
      transform: translateX(0);
    }
    25% {
      transform: translateX(-10px);
    }
    75% {
      transform: translateX(10px);
    }
  }

  .animate-shake {
    animation: shake 0.4s ease-in-out;
  }

  .animate-fade-in {
    animation: fadeIn 0.4s ease-in;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Responsive */
  @media (max-width: 640px) {
    .login-card {
      padding: 1.5rem;
    }

    .logo-circle {
      width: 80px;
      height: 80px;
    }

    .logo-circle i {
      font-size: 2.5rem !important;
    }
  }

  /* Backend Status Indicator */
  .backend-status-indicator {
    position: absolute;
    top: 1.5rem;
    right: 1.5rem;
    z-index: 20;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    padding: 0.5rem 1rem;
    border-radius: 50px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  }

  .status-circle {
    position: relative;
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .status-dot {
    position: relative;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    z-index: 2;
  }

  .status-pulse {
    position: absolute;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    z-index: 1;
  }

  /* Online State - Green */
  .status-circle.online .status-dot {
    background: #10b981;
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.6);
  }

  .status-circle.online .status-pulse {
    background: #10b981;
    animation: statusPulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  /* Offline State - Red */
  .status-circle.offline .status-dot {
    background: #ef4444;
    box-shadow: 0 0 8px rgba(239, 68, 68, 0.6);
  }

  .status-circle.offline .status-pulse {
    background: #ef4444;
    animation: statusPulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes statusPulse {
    0% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(2);
      opacity: 0.3;
    }
    100% {
      transform: scale(2.5);
      opacity: 0;
    }
  }

  .status-text {
    display: flex;
    align-items: center;
  }

  @media (max-width: 640px) {
    .backend-status-indicator {
      top: 1rem;
      right: 1rem;
      padding: 0.375rem 0.75rem;
    }

    .status-text span {
      font-size: 0.625rem;
    }

    .status-circle,
    .status-dot,
    .status-pulse {
      width: 10px;
      height: 10px;
    }
  }
</style>

