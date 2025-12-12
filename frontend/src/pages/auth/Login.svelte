<script>
  import { auth } from "../../stores/auth.js";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { loading, currentLang, theme } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import * as api from "../../lib/api.js";
  import { onMount } from "svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let username = $state("");
  let password = $state("");
  let showPassword = $state(false);
  let twoFactorCode = $state("");
  let showTwoFactor = $state(false);
  let rememberMe = $state(false);
  let loginInProgress = $state(false);
  let errorMessage = $state("");
  let backendOnline = $state(false);
  let checkingBackend = $state(true);

  // Theme state: 'system', 'light', 'dark'
  let currentTheme = $state("system");
  let isDark = $state(false);

  // Server selector state
  let showServerDialog = $state(false);
  let servers = $state([]);
  let activeServerId = $state(null);
  let editingServer = $state(null);
  let newServerName = $state("");
  let newServerUrl = $state("");
  let testingConnection = $state(false);
  let connectionTestResult = $state(null); // { success: boolean, message: string }

  // Get active server URL
  const activeServer = $derived(() => {
    const server = servers.find(s => s.id === activeServerId);
    return server || { id: 'default', name: 'Local', url: `${window.location.protocol}//${window.location.hostname}:8080` };
  });

  function loadServers() {
    try {
      const stored = localStorage.getItem("syncspace_servers");
      if (stored) {
        servers = JSON.parse(stored);
      }
      // Add default server if none exist
      if (servers.length === 0) {
        const defaultServer = {
          id: 'default',
          name: 'Local Server',
          url: `${window.location.protocol}//${window.location.hostname}:8080`
        };
        servers = [defaultServer];
        saveServers();
      }
      // Load active server
      activeServerId = localStorage.getItem("syncspace_active_server") || servers[0]?.id;
    } catch (e) {
      console.error("Failed to load servers:", e);
    }
  }

  function saveServers() {
    localStorage.setItem("syncspace_servers", JSON.stringify(servers));
  }

  function selectServer(serverId) {
    activeServerId = serverId;
    localStorage.setItem("syncspace_active_server", serverId);
    connectionTestResult = null;
    // Trigger backend check for new server
    checkBackendStatus();
  }

  function addServer() {
    if (!newServerName.trim() || !newServerUrl.trim()) return;
    
    // Normalize URL (remove trailing slash, ensure protocol)
    let url = newServerUrl.trim();
    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      url = 'http://' + url;
    }
    url = url.replace(/\/+$/, '');
    
    const newServer = {
      id: crypto.randomUUID(),
      name: newServerName.trim(),
      url: url
    };
    
    servers = [...servers, newServer];
    saveServers();
    newServerName = "";
    newServerUrl = "";
    connectionTestResult = null;
  }

  function updateServer() {
    if (!editingServer || !newServerName.trim() || !newServerUrl.trim()) return;
    
    let url = newServerUrl.trim();
    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      url = 'http://' + url;
    }
    url = url.replace(/\/+$/, '');
    
    servers = servers.map(s => 
      s.id === editingServer.id 
        ? { ...s, name: newServerName.trim(), url: url }
        : s
    );
    saveServers();
    editingServer = null;
    newServerName = "";
    newServerUrl = "";
    connectionTestResult = null;
    
    // Re-check if edited server is active
    if (activeServerId === editingServer?.id) {
      checkBackendStatus();
    }
  }

  function startEditServer(server) {
    editingServer = server;
    newServerName = server.name;
    newServerUrl = server.url;
    connectionTestResult = null;
  }

  function cancelEdit() {
    editingServer = null;
    newServerName = "";
    newServerUrl = "";
    connectionTestResult = null;
  }

  function deleteServer(serverId) {
    if (servers.length <= 1) return; // Keep at least one server
    servers = servers.filter(s => s.id !== serverId);
    saveServers();
    
    // If deleted server was active, switch to first available
    if (activeServerId === serverId) {
      selectServer(servers[0]?.id);
    }
  }

  async function testConnection(urlToTest = null) {
    testingConnection = true;
    connectionTestResult = null;
    
    const url = urlToTest || (editingServer ? newServerUrl : activeServer().url);
    let normalizedUrl = url.trim();
    if (!normalizedUrl.startsWith('http://') && !normalizedUrl.startsWith('https://')) {
      normalizedUrl = 'http://' + normalizedUrl;
    }
    normalizedUrl = normalizedUrl.replace(/\/+$/, '');
    
    try {
      const response = await fetch(`${normalizedUrl}/health`, {
        method: 'GET',
        signal: AbortSignal.timeout(5000) // 5 second timeout
      });
      
      if (response.ok) {
        connectionTestResult = { success: true, message: tr("connectionSuccessful") };
      } else {
        connectionTestResult = { success: false, message: `HTTP ${response.status}` };
      }
    } catch (err) {
      connectionTestResult = { 
        success: false, 
        message: err.name === 'TimeoutError' ? tr("connectionTimeout") : tr("connectionFailed")
      };
    } finally {
      testingConnection = false;
    }
  }

  function openServerDialog() {
    showServerDialog = true;
    connectionTestResult = null;
  }

  function closeServerDialog() {
    showServerDialog = false;
    editingServer = null;
    newServerName = "";
    newServerUrl = "";
    connectionTestResult = null;
  }

  // Check backend status on mount and periodically
  onMount(() => {
    loadServers();
    checkBackendStatus();
    const interval = setInterval(checkBackendStatus, 3000); // Check every 3 seconds

    // Initialize theme from localStorage or system preference
    initTheme();

    // Listen for system theme changes
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", handleSystemThemeChange);

    return () => {
      clearInterval(interval);
      mediaQuery.removeEventListener("change", handleSystemThemeChange);
    };
  });

  function initTheme() {
    const stored = localStorage.getItem("loginTheme") || "system";
    currentTheme = stored;
    applyTheme(stored);
  }

  function handleSystemThemeChange(e) {
    if (currentTheme === "system") {
      isDark = e.matches;
      document.documentElement.classList.toggle("dark", isDark);
    }
  }

  function applyTheme(themeValue) {
    if (themeValue === "system") {
      isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    } else {
      isDark = themeValue === "dark";
    }
    document.documentElement.classList.toggle("dark", isDark);
  }

  function cycleTheme() {
    const themes = ["system", "light", "dark"];
    const currentIndex = themes.indexOf(currentTheme);
    const nextIndex = (currentIndex + 1) % themes.length;
    currentTheme = themes[nextIndex];
    localStorage.setItem("loginTheme", currentTheme);
    applyTheme(currentTheme);
  }

  async function checkBackendStatus() {
    try {
      // Use active server URL
      const serverUrl = activeServer().url;
      const response = await fetch(`${serverUrl}/health`, {
        method: "GET",
        signal: AbortSignal.timeout(3000)
      });
      // If we get a response (even error 404/500), backend is online
      backendOnline = response.ok || response.status >= 400;
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
      errorMessage = tr("pleaseFillinAllFields");
      return;
    }

    loginInProgress = true;
    loading.show(tr("loggingIn"));
    errorMessage = "";

    try {
      // Save active server URL for api.js to use
      const serverUrl = activeServer().url;
      localStorage.setItem("syncspace_api_base", serverUrl + "/api");
      localStorage.setItem("syncspace_api_host", serverUrl);
      
      // Step 1: Try to login without 2FA first
      const loginData = { username, password };

      // Add 2FA code if we're in 2FA mode
      if (showTwoFactor && twoFactorCode) {
        loginData.totp_code = twoFactorCode;
      }

      // Direct login call using active server
      const response = await fetch(`${serverUrl}/api/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          username,
          password,
          totp_code: showTwoFactor ? twoFactorCode : undefined
        })
      });
      
      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || tr("loginFailed"));
      }
      
      const data = await response.json();

      // Check if 2FA is required
      if (data.requires_2fa === true) {
        showTwoFactor = true;
        errorMessage = tr("pleaseEnter2FACode");
        loginInProgress = false;
        loading.hide();
        return;
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

        loading.show(tr("loginSuccessfulRedirecting"));

        // Redirect to dashboard (main app)
        window.location.hash = "#/dashboard";
        // Force reload to trigger App.svelte onMount
        setTimeout(() => window.location.reload(), 100);
      } else {
        throw new Error("No token received");
      }
    } catch (err) {
      console.error("Login error:", err);
      errorMessage = err.message || tr("loginFailed");
      loginInProgress = false;
      loading.hide();
    }
  }
</script>

<!-- Background with animated gradient -->
<div class="relative min-h-screen flex items-center justify-center overflow-hidden p-4 transition-colors duration-300 {isDark ? 'bg-gradient-to-br from-emerald-950 via-emerald-900 to-emerald-800' : 'bg-gradient-to-br from-emerald-500 via-emerald-600 to-emerald-700'}">
  <!-- Theme Toggle -->
  <button
    class="absolute top-6 left-6 z-20 flex items-center gap-2 bg-white/15 backdrop-blur-md px-4 py-2 rounded-full border border-white/25 shadow-lg text-white text-sm font-medium cursor-pointer transition-all hover:bg-white/25 hover:border-white/35 hover:-translate-y-0.5 max-sm:top-4 max-sm:left-4 max-sm:px-3 max-sm:py-1.5"
    onclick={cycleTheme}
    aria-label={tr("toggleTheme")}
    title={currentTheme === "system"
      ? tr("themeSystem")
      : currentTheme === "light"
        ? tr("themeLight")
        : tr("themeDark")}
  >
    {#if currentTheme === "system"}
      <i class="bi bi-circle-half text-base" aria-hidden="true"></i>
    {:else if currentTheme === "light"}
      <i class="bi bi-sun-fill text-base" aria-hidden="true"></i>
    {:else}
      <i class="bi bi-moon-fill text-base" aria-hidden="true"></i>
    {/if}
    <span class="text-xs uppercase tracking-wide max-sm:hidden">
      {currentTheme === "system"
        ? "Auto"
        : currentTheme === "light"
          ? "Light"
          : "Dark"}
    </span>
  </button>

  <!-- Backend Status Indicator (clickable to open server dialog) -->
  <button
    class="absolute top-6 right-6 z-20 flex items-center gap-2 bg-white/10 backdrop-blur-md px-4 py-2 rounded-full border border-white/20 shadow-lg text-white cursor-pointer transition-all hover:bg-white/20 hover:border-white/30 hover:-translate-y-0.5 max-sm:top-4 max-sm:right-4 max-sm:px-3 max-sm:py-1.5"
    onclick={openServerDialog}
    aria-label={tr("manageServers")}
    title={tr("clickToManageServers")}
  >
    <div class="status-circle {backendOnline ? 'online' : 'offline'}">
      <span class="status-pulse"></span>
      <span class="status-dot"></span>
    </div>
    <div class="flex items-center">
      {#if checkingBackend}
        <span class="text-white/90 text-xs font-medium"
          >{tr("checkingBackend")}</span
        >
      {:else if backendOnline}
        <span class="text-green-300 text-xs font-semibold"
          >{activeServer().name}</span
        >
      {:else}
        <span class="text-red-300 text-xs font-semibold"
          >{tr("backendOffline")}</span
        >
      {/if}
    </div>
    <i class="bi bi-chevron-down text-white/70 text-xs ml-1" aria-hidden="true"></i>
  </button>

  <!-- Animated background blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Login Card with Glassmorphism -->
  <div class="relative z-10 w-full max-w-[460px] p-10 bg-white/95 dark:bg-gray-900/95 backdrop-blur-xl rounded-xl shadow-[0_8px_32px_0_rgba(16,185,129,0.15)] animate-slide-up max-sm:p-6">
    <!-- Logo Section -->
    <div class="text-center mb-8">
      <div class="logo-circle">
        <i
          class="bi bi-cloud-fill text-5xl text-green-600 dark:text-green-400"
          aria-hidden="true"
        ></i>
      </div>
      <h1
        class="text-4xl font-bold mt-6 mb-2 bg-gradient-to-r from-green-600 via-emerald-600 to-teal-600 bg-clip-text text-transparent"
      >
        SyncSpace
      </h1>
      <p class="text-gray-600 dark:text-gray-400 font-medium">
        {tr("loginSubtitle")}
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
          for="username-input"
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
        >
          <i
            class="bi bi-person-fill mr-2 text-green-600 dark:text-green-400"
            aria-hidden="true"
          ></i>{tr("username")}
        </label>
        <div class="relative">
          <input
            id="username-input"
            type="text"
            bind:value={username}
            class="w-full px-4 py-3 pl-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-green-500 focus:ring-4 focus:ring-green-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
            placeholder={tr("enterUsername")}
            disabled={loginInProgress}
          />
          <i
            class="bi bi-person absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
        </div>
      </div>

      <!-- Password Input -->
      <div class="space-y-2">
        <label
          for="password-input"
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
        >
          <i
            class="bi bi-shield-lock-fill mr-2 text-green-600 dark:text-green-400"
          ></i>{tr("password")}
        </label>
        <div class="relative">
          {#if showPassword}
            <input
              id="password-input"
              type="text"
              bind:value={password}
              class="w-full px-4 py-3 pl-12 pr-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-green-500 focus:ring-4 focus:ring-green-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
              placeholder={tr("enterPassword")}
              disabled={loginInProgress}
            />
          {:else}
            <input
              id="password-input"
              type="password"
              bind:value={password}
              class="w-full px-4 py-3 pl-12 pr-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-green-500 focus:ring-4 focus:ring-green-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
              placeholder={tr("enterPassword")}
              disabled={loginInProgress}
            />
          {/if}
          <i
            class="bi bi-lock absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
          <button
            type="button"
            onclick={() => (showPassword = !showPassword)}
            class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
            aria-label="Toggle password visibility"
          >
            <i
              class="bi bi-{showPassword ? 'eye-slash' : 'eye'}"
              aria-hidden="true"
            ></i>
          </button>
        </div>
      </div>

      <!-- 2FA Code Input -->
      {#if showTwoFactor}
        <div class="space-y-2 animate-fade-in">
          <label
            for="twofa-input"
            class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
          >
            <i
              class="bi bi-shield-check mr-2 text-green-600 dark:text-green-400"
            ></i>{tr("enter2FACode")}
          </label>
          <div class="relative">
            <input
              id="twofa-input"
              type="text"
              bind:value={twoFactorCode}
              class="w-full px-4 py-3 pl-12 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-green-500 focus:ring-4 focus:ring-green-500/20 outline-none transition-all duration-200 text-gray-900 dark:text-white placeholder:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed tracking-widest text-center text-lg font-mono"
              placeholder={tr("enter2FACode")}
              maxlength="6"
              disabled={loginInProgress}
            />
            <i
              class="bi bi-shield-check absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
            ></i>
          </div>
        </div>
      {/if}

      <!-- Remember Me -->
      <div class="flex justify-between items-center">
        <label class="flex items-center gap-2 cursor-pointer group">
          <input
            type="checkbox"
            bind:checked={rememberMe}
            class="w-4 h-4 text-green-600 bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 rounded focus:ring-2 focus:ring-green-500 accent-green-600"
          />
          <span
            class="text-sm font-medium text-gray-700 dark:text-gray-300 group-hover:text-green-600 dark:group-hover:text-green-400 transition-colors"
            >{tr("rememberMe")}</span
          >
        </label>
      </div>

      <!-- Login Button -->
      <button
        type="submit"
        class="w-full py-3.5 px-6 bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 text-white font-bold rounded-xl shadow-lg shadow-green-500/30 hover:shadow-xl hover:shadow-green-500/40 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 group"
        disabled={loginInProgress}
      >
        {#if loginInProgress}
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
        <span>{showTwoFactor ? tr("verifyAndLogin") : tr("login")}</span>
      </button>
    </form>
  </div>

  <!-- Footer -->
  <div class="absolute bottom-6 text-center text-sm text-white/80">
    <p>© 2025 SyncSpace. Secure Cloud Storage Platform.</p>
  </div>

  <!-- Server Selection Dialog -->
  {#if showServerDialog}
    <div 
      class="fixed inset-0 z-50 flex items-center justify-center p-4"
      role="dialog"
      aria-modal="true"
      aria-labelledby="server-dialog-title"
    >
      <!-- Backdrop -->
      <div 
        class="absolute inset-0 bg-black/60 backdrop-blur-sm"
        onclick={closeServerDialog}
        onkeydown={(e) => e.key === 'Escape' && closeServerDialog()}
        role="button"
        tabindex="-1"
        aria-label={tr("closeDialog")}
      ></div>
      
      <!-- Dialog -->
      <div class="relative w-full max-w-lg bg-white dark:bg-gray-900 rounded-2xl shadow-2xl overflow-hidden animate-scale-in">
        <!-- Header -->
        <div class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center">
              <i class="bi bi-hdd-network text-white text-lg" aria-hidden="true"></i>
            </div>
            <div>
              <h2 id="server-dialog-title" class="text-lg font-bold text-gray-900 dark:text-white">
                {tr("serverManagement")}
              </h2>
              <p class="text-xs text-gray-500 dark:text-gray-400">{tr("selectOrAddServer")}</p>
            </div>
          </div>
          <button 
            onclick={closeServerDialog}
            class="p-2 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            aria-label={tr("close")}
          >
            <i class="bi bi-x-lg" aria-hidden="true"></i>
          </button>
        </div>
        
        <!-- Server List -->
        <div class="px-6 py-4 max-h-64 overflow-y-auto">
          <div class="space-y-2">
            {#each servers as server (server.id)}
              <div 
                class="flex items-center gap-3 p-3 rounded-xl border-2 transition-all cursor-pointer
                  {activeServerId === server.id 
                    ? 'border-green-500 bg-green-50 dark:bg-green-900/20' 
                    : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 bg-gray-50 dark:bg-gray-800/50'}"
                onclick={() => selectServer(server.id)}
                onkeydown={(e) => e.key === 'Enter' && selectServer(server.id)}
                role="button"
                tabindex="0"
              >
                <div class="flex-shrink-0">
                  <div class="w-10 h-10 rounded-lg bg-gradient-to-br {activeServerId === server.id ? 'from-green-500 to-emerald-600' : 'from-gray-400 to-gray-500'} flex items-center justify-center">
                    <i class="bi bi-server text-white" aria-hidden="true"></i>
                  </div>
                </div>
                <div class="flex-1 min-w-0">
                  <p class="font-semibold text-gray-900 dark:text-white truncate">{server.name}</p>
                  <p class="text-xs text-gray-500 dark:text-gray-400 truncate">{server.url}</p>
                </div>
                <div class="flex items-center gap-1">
                  {#if activeServerId === server.id}
                    <span class="px-2 py-0.5 text-xs font-medium bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300 rounded-full">
                      {tr("active")}
                    </span>
                  {/if}
                  <button 
                    onclick={(e) => { e.stopPropagation(); startEditServer(server); }}
                    class="p-1.5 rounded-lg text-gray-400 hover:text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/30 transition-colors"
                    aria-label={tr("editServer")}
                  >
                    <i class="bi bi-pencil text-sm" aria-hidden="true"></i>
                  </button>
                  {#if servers.length > 1}
                    <button 
                      onclick={(e) => { e.stopPropagation(); deleteServer(server.id); }}
                      class="p-1.5 rounded-lg text-gray-400 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/30 transition-colors"
                      aria-label={tr("deleteServer")}
                    >
                      <i class="bi bi-trash text-sm" aria-hidden="true"></i>
                    </button>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </div>
        
        <!-- Add/Edit Server Form -->
        <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
            {editingServer ? tr("editServer") : tr("addNewServer")}
          </h3>
          <div class="space-y-3">
            <div>
              <label for="server-name" class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                {tr("serverName")}
              </label>
              <input
                id="server-name"
                type="text"
                bind:value={newServerName}
                placeholder={tr("serverNamePlaceholder")}
                class="w-full px-3 py-2 text-sm bg-white dark:bg-gray-900 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition-all text-gray-900 dark:text-white placeholder:text-gray-400"
              />
            </div>
            <div>
              <label for="server-url" class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                {tr("serverUrl")}
              </label>
              <div class="flex gap-2">
                <input
                  id="server-url"
                  type="text"
                  bind:value={newServerUrl}
                  placeholder="http://localhost:8080"
                  class="flex-1 px-3 py-2 text-sm bg-white dark:bg-gray-900 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition-all text-gray-900 dark:text-white placeholder:text-gray-400"
                />
                <button
                  onclick={() => testConnection(newServerUrl)}
                  disabled={!newServerUrl.trim() || testingConnection}
                  class="px-3 py-2 text-sm font-medium bg-blue-500 hover:bg-blue-600 disabled:bg-gray-300 dark:disabled:bg-gray-600 text-white rounded-lg transition-colors flex items-center gap-1.5"
                  aria-label={tr("testConnection")}
                >
                  {#if testingConnection}
                    <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                  {:else}
                    <i class="bi bi-plug" aria-hidden="true"></i>
                  {/if}
                  <span class="hidden sm:inline">{tr("test")}</span>
                </button>
              </div>
            </div>
            
            <!-- Connection Test Result -->
            {#if connectionTestResult}
              <div class="flex items-center gap-2 p-2 rounded-lg {connectionTestResult.success ? 'bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-300' : 'bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-300'}">
                <i class="bi {connectionTestResult.success ? 'bi-check-circle-fill' : 'bi-x-circle-fill'}" aria-hidden="true"></i>
                <span class="text-sm font-medium">{connectionTestResult.message}</span>
              </div>
            {/if}
            
            <!-- Action Buttons -->
            <div class="flex gap-2 pt-2">
              {#if editingServer}
                <button
                  onclick={cancelEdit}
                  class="flex-1 px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-lg transition-colors"
                >
                  {tr("cancel")}
                </button>
                <button
                  onclick={updateServer}
                  disabled={!newServerName.trim() || !newServerUrl.trim()}
                  class="flex-1 px-4 py-2 text-sm font-semibold text-white bg-green-600 hover:bg-green-700 disabled:bg-gray-300 dark:disabled:bg-gray-600 rounded-lg transition-colors flex items-center justify-center gap-2"
                >
                  <i class="bi bi-check-lg" aria-hidden="true"></i>
                  {tr("saveChanges")}
                </button>
              {:else}
                <button
                  onclick={addServer}
                  disabled={!newServerName.trim() || !newServerUrl.trim()}
                  class="w-full px-4 py-2 text-sm font-semibold text-white bg-green-600 hover:bg-green-700 disabled:bg-gray-300 dark:disabled:bg-gray-600 rounded-lg transition-colors flex items-center justify-center gap-2"
                >
                  <i class="bi bi-plus-lg" aria-hidden="true"></i>
                  {tr("addServer")}
                </button>
              {/if}
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Animated Background Blobs - requires custom CSS */
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
    background: linear-gradient(135deg, #34d399, #10b981);
    top: -10%;
    left: -10%;
  }

  .blob-2 {
    width: 400px;
    height: 400px;
    background: linear-gradient(135deg, #6ee7b7, #059669);
    top: 40%;
    right: -5%;
    animation-delay: 5s;
  }

  .blob-3 {
    width: 350px;
    height: 350px;
    background: linear-gradient(135deg, #a7f3d0, #047857);
    bottom: -10%;
    left: 30%;
    animation-delay: 10s;
  }

  /* Logo Circle with Pulse */
  .logo-circle {
    width: 100px;
    height: 100px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #10b981, #059669);
    border-radius: 50%;
    box-shadow:
      0 10px 40px rgba(16, 185, 129, 0.4),
      0 0 0 8px rgba(16, 185, 129, 0.1);
    animation: pulse 3s ease-in-out infinite;
  }

  .logo-circle i {
    color: white !important;
  }

  /* Status indicator pulse */
  .status-circle {
    position: relative;
    width: 14px;
    height: 14px;
  }

  .status-dot {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    z-index: 2;
  }

  .status-pulse {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    z-index: 1;
    animation: statusPulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  .status-circle.online .status-dot {
    background: #10b981;
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.6);
  }

  .status-circle.online .status-pulse {
    background: #10b981;
  }

  .status-circle.offline .status-dot {
    background: #ef4444;
    box-shadow: 0 0 8px rgba(239, 68, 68, 0.6);
  }

  .status-circle.offline .status-pulse {
    background: #ef4444;
  }

  /* Animations */
  @keyframes float {
    0%, 100% {
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
    0%, 100% {
      transform: scale(1);
      box-shadow:
        0 10px 40px rgba(16, 185, 129, 0.4),
        0 0 0 8px rgba(16, 185, 129, 0.1);
    }
    50% {
      transform: scale(1.05);
      box-shadow:
        0 15px 60px rgba(16, 185, 129, 0.6),
        0 0 0 12px rgba(16, 185, 129, 0.15);
    }
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

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    25% { transform: translateX(-10px); }
    75% { transform: translateX(10px); }
  }

  .animate-shake {
    animation: shake 0.4s ease-in-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .animate-fade-in {
    animation: fadeIn 0.4s ease-in;
  }

  @keyframes scaleIn {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }

  .animate-scale-in {
    animation: scaleIn 0.2s ease-out;
  }

  @keyframes slideUp {
    from { opacity: 0; transform: translateY(60px) scale(0.9); }
    to { opacity: 1; transform: translateY(0) scale(1); }
  }

  .animate-slide-up {
    animation: slideUp 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  /* Responsive adjustments for logo */
  @media (max-width: 640px) {
    .logo-circle {
      width: 80px;
      height: 80px;
    }
    .logo-circle i {
      font-size: 2.5rem !important;
    }
  }
</style>
