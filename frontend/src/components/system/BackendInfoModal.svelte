<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { onMount } from "svelte";
  import Modal from "../ui/Modal.svelte";
  import * as api from "../../lib/api.js";

  let { visible = $bindable(false) } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let serverInfo = $state(null);
  let backendOnline = $state(false);
  let backendUptime = $state("0s");
  let loading = $state(false);

  async function loadServerInfo() {
    loading = true;
    try {
      // Check backend status
      const response = await fetch(
        `${new URL(window.location.href).protocol}//${new URL(window.location.href).hostname}:8080/health`
      );
      backendOnline = response.ok;

      // Load server info
      serverInfo = await api.system.getStatus();

      // Calculate uptime
      if (serverInfo?.uptime_seconds) {
        backendUptime = formatUptime(serverInfo.uptime_seconds);
      }
    } catch (err) {
      console.error("Failed to load server info:", err);
      backendOnline = false;
      serverInfo = null;
    } finally {
      loading = false;
    }
  }

  function formatUptime(seconds) {
    if (!seconds || seconds === 0) return "Just started";

    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (days > 0) return `${days}d ${hours}h ${minutes}m`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    if (minutes > 0) return `${minutes}m ${secs}s`;
    return `${secs}s`;
  }

  function handleClose() {
    visible = false;
  }

  function handleRefresh() {
    loadServerInfo();
  }

  // Load server info when modal becomes visible
  $effect(() => {
    if (visible) {
      loadServerInfo();
    }
  });
</script>

<Modal
  bind:visible
  title="Backend Information"
  icon="server"
  size="md"
  variant="primary"
  on:close={handleClose}
>
  <div class="space-y-6">
    <!-- Status Banner -->
    <div
      class="rounded-xl p-4 {backendOnline
        ? 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800'
        : 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800'}"
    >
      <div class="flex items-center gap-3">
        <div class="relative flex h-4 w-4">
          {#if backendOnline}
            <span
              class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-500 opacity-75"
            ></span>
            <span class="relative inline-flex rounded-full h-4 w-4 bg-green-500"
            ></span>
          {:else}
            <span class="relative inline-flex rounded-full h-4 w-4 bg-red-500"
            ></span>
          {/if}
        </div>
        <div class="flex-1">
          <div
            class="font-bold {backendOnline
              ? 'text-green-700 dark:text-green-300'
              : 'text-red-700 dark:text-red-300'}"
          >
            Backend is {backendOnline ? "Online" : "Offline"}
          </div>
          <div
            class="text-sm {backendOnline
              ? 'text-green-600 dark:text-green-400'
              : 'text-red-600 dark:text-red-400'}"
          >
            {backendOnline
              ? "All systems operational"
              : "Connection to backend failed"}
          </div>
        </div>
      </div>
    </div>

    {#if loading}
      <!-- Loading State -->
      <div class="flex flex-col items-center justify-center py-12">
        <span class="loading loading-spinner loading-lg text-primary"></span>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-4">
          Loading server information...
        </p>
      </div>
    {:else if serverInfo}
      <!-- Server Details Grid -->
      <div class="grid grid-cols-2 gap-4">
        <!-- Version -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-box-seam text-2xl text-primary-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">Version</div>
          </div>
          <div class="text-xl font-bold text-gray-900 dark:text-gray-100">
            v{serverInfo.version || "1.0.0"}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            SyncSpace Backend
          </div>
        </div>

        <!-- Uptime -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-clock-history text-2xl text-green-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">Uptime</div>
          </div>
          <div class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {backendUptime}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            Running since startup
          </div>
        </div>

        <!-- Database -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-database text-2xl text-blue-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">Database</div>
          </div>
          <div class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {serverInfo.database?.active_connections || 0}/{serverInfo.database
              ?.pool_size || 10}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            Active Connections
          </div>
        </div>

        <!-- WebSocket -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-broadcast text-2xl text-purple-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">
              WebSocket
            </div>
          </div>
          <div class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {serverInfo.websocket?.active_connections || 0}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            Connected Clients
          </div>
        </div>
      </div>

      <!-- API Endpoint -->
      <div class="glass-card p-4 rounded-xl">
        <div class="flex items-center gap-2 mb-2">
          <i class="bi bi-link-45deg text-xl text-indigo-500"></i>
          <div class="text-sm font-semibold text-gray-700 dark:text-gray-300">
            API Endpoint
          </div>
        </div>
        <div
          class="flex items-center gap-2 bg-gray-100 dark:bg-gray-800 rounded-lg p-2 font-mono text-sm"
        >
          <code class="flex-1 text-gray-900 dark:text-gray-100"
            >http://localhost:8080/api</code
          >
          <button
            class="btn btn-ghost btn-xs btn-square"
            onclick={() => {
              navigator.clipboard.writeText("http://localhost:8080/api");
            }}
            title="Copy to clipboard"
          >
            <i class="bi bi-clipboard"></i>
          </button>
          <a
            href="http://localhost:8080/api"
            target="_blank"
            rel="noopener noreferrer"
            class="btn btn-ghost btn-xs btn-square"
            title="Open in new tab"
          >
            <i class="bi bi-box-arrow-up-right"></i>
          </a>
        </div>
      </div>

      <!-- WebSocket Endpoint -->
      {#if serverInfo.websocket?.endpoint}
        <div class="glass-card p-4 rounded-xl">
          <div class="flex items-center gap-2 mb-2">
            <i class="bi bi-broadcast-pin text-xl text-purple-500"></i>
            <div class="text-sm font-semibold text-gray-700 dark:text-gray-300">
              WebSocket Endpoint
            </div>
          </div>
          <div
            class="flex items-center gap-2 bg-gray-100 dark:bg-gray-800 rounded-lg p-2 font-mono text-sm"
          >
            <code class="flex-1 text-gray-900 dark:text-gray-100"
              >{serverInfo.websocket.endpoint}</code
            >
            <button
              class="btn btn-ghost btn-xs btn-square"
              onclick={() => {
                navigator.clipboard.writeText(serverInfo.websocket.endpoint);
              }}
              title="Copy to clipboard"
            >
              <i class="bi bi-clipboard"></i>
            </button>
          </div>
        </div>
      {/if}

      <!-- Search Index -->
      {#if serverInfo.search_index}
        <div class="glass-card p-4 rounded-xl">
          <div class="flex items-center gap-2 mb-3">
            <i class="bi bi-search text-xl text-amber-500"></i>
            <div class="text-sm font-semibold text-gray-700 dark:text-gray-300">
              Search Index
            </div>
          </div>
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-600 dark:text-gray-400">Status:</span>
              <span class="font-medium text-gray-900 dark:text-gray-100">
                {serverInfo.search_index.initialized
                  ? "✅ Initialized"
                  : "❌ Not initialized"}
              </span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600 dark:text-gray-400">Path:</span>
              <code class="text-xs font-mono text-gray-700 dark:text-gray-300">
                {serverInfo.search_index.index_path}
              </code>
            </div>
          </div>
        </div>
      {/if}

      <!-- API Endpoints Count -->
      {#if serverInfo.endpoints && serverInfo.endpoints.length > 0}
        <div class="glass-card p-4 rounded-xl">
          <div class="flex items-center gap-2 mb-3">
            <i class="bi bi-list-check text-xl text-cyan-500"></i>
            <div class="text-sm font-semibold text-gray-700 dark:text-gray-300">
              Available Endpoints
            </div>
          </div>
          <div class="text-center">
            <div
              class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-1"
            >
              {serverInfo.endpoints.length}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-500">
              API Routes Registered
            </div>
          </div>
          <div class="mt-3 text-xs text-center">
            <a
              href="http://localhost:8080/status"
              target="_blank"
              rel="noopener noreferrer"
              class="text-primary-600 dark:text-primary-400 hover:underline"
            >
              View full API documentation →
            </a>
          </div>
        </div>
      {/if}
    {:else}
      <!-- Error State -->
      <div class="flex flex-col items-center justify-center py-12">
        <i class="bi bi-exclamation-triangle text-6xl text-red-500 mb-4"></i>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Failed to load server information
        </p>
      </div>
    {/if}
  </div>

  <div slot="actions" class="flex gap-3 justify-end">
    <button
      class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors flex items-center gap-2"
      onclick={handleClose}
    >
      <i class="bi bi-x-lg"></i>
      Close
    </button>
    <button
      class="px-4 py-2 text-sm font-medium text-white bg-blue-600 dark:bg-blue-500 rounded-xl hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
      onclick={handleRefresh}
    >
      <i class="bi bi-arrow-clockwise"></i>
      Refresh
    </button>
  </div>
</Modal>
