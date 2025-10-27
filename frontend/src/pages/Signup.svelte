<script>
  import { auth } from "../stores/auth.js";

  let username = $state("");
  let email = $state("");
  let password = $state("");
  let confirmPassword = $state("");
  let agreedToTerms = $state(false);
  let loading = $state(false);
  let errorMessage = $state("");
  let successMessage = $state("");

  async function handleSignup(e) {
    e.preventDefault();

    // Validation
    if (!username || !email || !password || !confirmPassword) {
      errorMessage = "Please fill in all fields";
      return;
    }

    if (password !== confirmPassword) {
      errorMessage = "Passwords do not match";
      return;
    }

    if (password.length < 8) {
      errorMessage = "Password must be at least 8 characters";
      return;
    }

    if (!agreedToTerms) {
      errorMessage = "Please accept the terms and conditions";
      return;
    }

    loading = true;
    errorMessage = "";
    successMessage = "";

    try {
      const response = await fetch("http://localhost:8080/api/auth/register", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, email, password }),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.error || "Registration failed");
      }

      successMessage = "Account created successfully! Redirecting to login...";

      // Redirect to login after 2 seconds
      setTimeout(() => {
        window.location.hash = "#/login";
      }, 2000);
    } catch (err) {
      console.error("Signup error:", err);
      errorMessage = err.message || "Registration failed. Please try again.";
      loading = false;
    }
  }
</script>

<!-- Background with animated gradient -->
<div class="signup-container">
  <!-- Animated background blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Signup Card with Glassmorphism -->
  <div class="signup-card">
    <!-- Logo Section -->
    <div class="text-center mb-8">
      <div class="logo-circle">
        <i
          class="bi bi-cloud-plus-fill text-5xl text-blue-600 dark:text-blue-400"
        ></i>
      </div>
      <h1
        class="text-4xl font-bold mt-6 mb-2 bg-gradient-to-r from-blue-600 via-purple-600 to-pink-600 bg-clip-text text-transparent"
      >
        Join SyncSpace
      </h1>
      <p class="text-gray-600 dark:text-gray-400 font-medium">
        Create your free account
      </p>
    </div>

    <form onsubmit={handleSignup} class="space-y-5">
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

      {#if successMessage}
        <div
          class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-xl p-4 flex items-start gap-3 animate-fade-in"
        >
          <i
            class="bi bi-check-circle-fill text-green-600 dark:text-green-400 text-xl mt-0.5"
          ></i>
          <span class="text-green-800 dark:text-green-200 text-sm font-medium"
            >{successMessage}</span
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
            placeholder="Choose a username"
            disabled={loading}
          />
          <i
            class="bi bi-person absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
        </div>
      </div>

      <!-- Email Input -->
      <div class="space-y-2">
        <label
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
        >
          <i class="bi bi-envelope-fill mr-2 text-blue-600 dark:text-blue-400"
          ></i>Email
        </label>
        <div class="relative">
          <input
            type="email"
            bind:value={email}
            class="w-full px-4 py-3 pl-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-4 focus:ring-blue-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
            placeholder="your.email@example.com"
            disabled={loading}
          />
          <i
            class="bi bi-envelope absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
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
            placeholder="Create a strong password"
            disabled={loading}
          />
          <i
            class="bi bi-lock absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
        </div>
        <p class="text-xs text-gray-500 dark:text-gray-400 ml-1">
          Minimum 8 characters
        </p>
      </div>

      <!-- Confirm Password Input -->
      <div class="space-y-2">
        <label
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
        >
          <i class="bi bi-shield-check mr-2 text-green-600 dark:text-green-400"
          ></i>Confirm Password
        </label>
        <div class="relative">
          <input
            type="password"
            bind:value={confirmPassword}
            class="w-full px-4 py-3 pl-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-green-500 focus:ring-4 focus:ring-green-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
            placeholder="Confirm your password"
            disabled={loading}
          />
          <i
            class="bi bi-shield-check absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
        </div>
      </div>

      <!-- Terms & Conditions -->
      <div class="flex items-start gap-3 pt-2">
        <input
          type="checkbox"
          bind:checked={agreedToTerms}
          id="terms"
          class="w-4 h-4 mt-1 text-blue-600 bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded focus:ring-2 focus:ring-blue-500 accent-blue-600"
        />
        <label for="terms" class="text-sm text-gray-700 dark:text-gray-300">
          I agree to the <a
            href="#/terms"
            class="text-blue-600 dark:text-blue-400 hover:underline font-semibold"
            >Terms and Conditions</a
          >
          and
          <a
            href="#/privacy"
            class="text-blue-600 dark:text-blue-400 hover:underline font-semibold"
            >Privacy Policy</a
          >
        </label>
      </div>

      <!-- Signup Button -->
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
            class="bi bi-person-plus-fill group-hover:scale-110 transition-transform"
          ></i>
        {/if}
        <span>{loading ? "Creating Account..." : "Create Account"}</span>
      </button>
    </form>

    <!-- Login Link -->
    <p class="text-center mt-8 text-sm text-gray-600 dark:text-gray-400">
      Already have an account?
      <a
        href="#/login"
        class="font-bold text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 hover:underline transition-colors"
      >
        Sign in
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
  .signup-container {
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
  .signup-card {
    position: relative;
    z-index: 10;
    width: 100%;
    max-width: 520px;
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
    .signup-card {
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
    .signup-card {
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
</style>
