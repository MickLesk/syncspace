<script>
  import { onMount, onDestroy } from "svelte";
  import Modal from "../ui/Modal.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { visible = false, onClose = () => {} } = $props();

  let serverInfo = $state(null);
  let backendOnline = $state(false);
  let loading = $state(true);
  let pollInterval;

  async function fetchBackendStatus() {
    try {
      const response = await fetch("http://localhost:8080/api/status");
      if (response.ok) {
        serverInfo = await response.json();
        backendOnline = true;
      } else {
        backendOnline = false;
        serverInfo = null;
      }
    } catch (error) {
      console.error("[BackendInfo] Failed to fetch backend status:", error);
      backendOnline = false;
      serverInfo = null;
    } finally {
      loading = false;
    }
  }

  function formatUptime(seconds) {
    if (!seconds) return "N/A";

    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (days > 0) {
      return `${days}d ${hours}h ${minutes}m`;
    } else if (hours > 0) {
      return `${hours}h ${minutes}m`;
    } else if (minutes > 0) {
      return `${minutes}m ${secs}s`;
    } else {
      return `${secs}s`;
    }
  }

  onMount(() => {
    fetchBackendStatus();
    // Poll every 5 seconds
    pollInterval = setInterval(fetchBackendStatus, 5000);
  });

  onDestroy(() => {
    if (pollInterval) {
      clearInterval(pollInterval);
    }
  });

  function handleClose() {
    onClose();
  }
</script>

<Modal
  {visible}
  title="Backend Information"
  icon="server"
  size="lg"
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
      <div class="flex items-center justify-center py-12">
        <div class="flex flex-col items-center gap-4">
          <div
            class="w-12 h-12 border-4 border-primary-500 border-t-transparent rounded-full animate-spin"
          ></div>
          <p class="text-gray-600 dark:text-gray-400">
            Loading server information...
          </p>
        </div>
      </div>
    {:else if serverInfo}
      <!-- Server Details Grid -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Version -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform cursor-pointer"
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
            {formatUptime(serverInfo.uptime_seconds)}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            Since server start
          </div>
        </div>

        <!-- Database Pool -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-database text-2xl text-blue-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">
              DB Pool Size
            </div>
          </div>
          <div class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {serverInfo.database?.pool_size || 10}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            Max connections
          </div>
        </div>

        <!-- Active Connections -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-plug text-2xl text-purple-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">
              Active Connections
            </div>
          </div>
          <div class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {serverInfo.database?.active_connections || 0}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            Database connections
          </div>
        </div>

        <!-- WebSocket Connections -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-wifi text-2xl text-orange-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">
              WebSocket
            </div>
          </div>
          <div class="text-xl font-bold text-gray-900 dark:text-gray-100">
            {serverInfo.websocket?.active_connections || 0}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            Active connections
          </div>
        </div>

        <!-- Status -->
        <div
          class="glass-card p-4 rounded-xl hover:scale-105 transition-transform"
        >
          <div class="flex items-center gap-3 mb-2">
            <i class="bi bi-heart-pulse text-2xl text-red-500"></i>
            <div class="text-xs text-gray-600 dark:text-gray-400">Status</div>
          </div>
          <div
            class="text-xl font-bold text-gray-900 dark:text-gray-100 capitalize"
          >
            {serverInfo.status || "operational"}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500 mt-1">
            System health
          </div>
        </div>
      </div>

      <!-- Search Index Status -->
      {#if serverInfo.search_index}
        <div class="glass-card p-4 rounded-xl">
          <div class="flex items-center gap-3 mb-3">
            <i class="bi bi-search text-xl text-indigo-500"></i>
            <h3 class="font-semibold text-gray-900 dark:text-gray-100">
              Search Index
            </h3>
          </div>
          <div class="grid grid-cols-2 gap-3 text-sm">
            <div>
              <span class="text-gray-600 dark:text-gray-400">Status:</span>
              <span class="ml-2 font-medium text-gray-900 dark:text-gray-100">
                {serverInfo.search_index.initialized
                  ? "✅ Initialized"
                  : "❌ Not initialized"}
              </span>
            </div>
            <div>
              <span class="text-gray-600 dark:text-gray-400">Path:</span>
              <span
                class="ml-2 font-mono text-xs text-gray-900 dark:text-gray-100"
              >
                {serverInfo.search_index.index_path || "./data/search_index"}
              </span>
            </div>
          </div>
        </div>
      {/if}

      <!-- WebSocket Details -->
      {#if serverInfo.websocket}
        <div class="glass-card p-4 rounded-xl">
          <div class="flex items-center gap-3 mb-3">
            <i class="bi bi-broadcast text-xl text-teal-500"></i>
            <h3 class="font-semibold text-gray-900 dark:text-gray-100">
              WebSocket Server
            </h3>
          </div>
          <div class="grid grid-cols-2 gap-3 text-sm">
            <div>
              <span class="text-gray-600 dark:text-gray-400">Enabled:</span>
              <span class="ml-2 font-medium text-gray-900 dark:text-gray-100">
                {serverInfo.websocket.enabled ? "✅ Yes" : "❌ No"}
              </span>
            </div>
            <div>
              <span class="text-gray-600 dark:text-gray-400">Endpoint:</span>
              <span
                class="ml-2 font-mono text-xs text-gray-900 dark:text-gray-100"
              >
                {serverInfo.websocket.endpoint || "ws://localhost:8080/api/ws"}
              </span>
            </div>
          </div>
        </div>
      {/if}
    {:else}
      <div class="flex items-center justify-center py-12">
        <div class="text-center">
          <i class="bi bi-exclamation-triangle text-5xl text-red-500 mb-4"></i>
          <p class="text-gray-900 dark:text-gray-100 font-semibold">
            {tr("failedToConnectBackend")}
          </p>
          <p class="text-gray-600 dark:text-gray-400 text-sm mt-2">
            {tr("pleaseCheckIfServerRunning")}
          </p>
        </div>
      </div>
    {/if}
  </div>

  <div slot="actions">
    <button
      type="button"
      class="px-4 py-2 text-sm font-medium text-white bg-primary-600 dark:bg-primary-500 rounded-xl hover:bg-primary-700 dark:hover:bg-primary-600 transition-colors flex items-center gap-2"
      onclick={handleClose}
    >
      <i class="bi bi-check-lg"></i>
      {tr("close")}
    </button>
  </div>
</Modal>

<style>
  .glass-card {
    background: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.3);
  }

  :global(.dark) .glass-card {
    background: rgba(31, 41, 55, 0.7);
    border: 1px solid rgba(75, 85, 99, 0.3);
  }
</style>
