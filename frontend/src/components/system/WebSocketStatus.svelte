<script>
  import {
    wsState,
    wsReconnectAttempts,
    WS_STATES,
    websocketManager,
  } from "@stores/websocket.js";
  import { fade, fly } from "svelte/transition";

  let { compact = false } = $props();

  const statusConfig = {
    [WS_STATES.DISCONNECTED]: {
      icon: "bi-x-circle-fill",
      color: "text-gray-400 dark:text-gray-600",
      bgColor: "bg-gray-100 dark:bg-gray-800",
      label: "Disconnected",
      pulse: false,
    },
    [WS_STATES.CONNECTING]: {
      icon: "bi-arrow-repeat",
      color: "text-blue-500",
      bgColor: "bg-blue-50 dark:bg-blue-900/20",
      label: "Connecting...",
      pulse: true,
      spin: true,
    },
    [WS_STATES.CONNECTED]: {
      icon: "bi-check-circle-fill",
      color: "text-green-500",
      bgColor: "bg-green-50 dark:bg-green-900/20",
      label: "Connected",
      pulse: false,
    },
    [WS_STATES.RECONNECTING]: {
      icon: "bi-arrow-clockwise",
      color: "text-orange-500",
      bgColor: "bg-orange-50 dark:bg-orange-900/20",
      label: "Reconnecting...",
      pulse: true,
      spin: true,
    },
    [WS_STATES.ERROR]: {
      icon: "bi-exclamation-triangle-fill",
      color: "text-red-500",
      bgColor: "bg-red-50 dark:bg-red-900/20",
      label: "Error",
      pulse: true,
    },
    [WS_STATES.MAX_RETRIES_REACHED]: {
      icon: "bi-x-octagon-fill",
      color: "text-red-600",
      bgColor: "bg-red-100 dark:bg-red-900/30",
      label: "Connection Failed",
      pulse: false,
    },
  };

  const config = $derived(statusConfig[$wsState]);

  function handleReconnect() {
    websocketManager.forceReconnect();
  }
</script>

{#if !compact}
  <!-- Full status indicator -->
  <div
    class="flex items-center gap-2 px-3 py-1.5 rounded-lg {config.bgColor} transition-all duration-300"
    transition:fly={{ y: -10, duration: 200 }}
  >
    <div class="relative">
      <i
        class="bi {config.icon} {config.color} text-sm"
        class:animate-spin={config.spin}
      ></i>
      {#if config.pulse}
        <span
          class="absolute inset-0 {config.icon} {config.color} animate-ping opacity-50"
        ></span>
      {/if}
    </div>

    <span class="text-xs font-medium {config.color}">
      {config.label}
    </span>

    {#if $wsState === WS_STATES.RECONNECTING && $wsReconnectAttempts > 0}
      <span class="text-xs text-gray-500 dark:text-gray-400">
        ({$wsReconnectAttempts}/10)
      </span>
    {/if}

    {#if $wsState === WS_STATES.MAX_RETRIES_REACHED}
      <button
        onclick={handleReconnect}
        class="ml-2 text-xs font-semibold px-2 py-0.5 rounded bg-red-500 text-white hover:bg-red-600 transition-colors"
      >
        Retry
      </button>
    {/if}
  </div>
{:else}
  <!-- Compact indicator (just icon) -->
  <div
    class="relative w-2 h-2 rounded-full {config.bgColor}"
    title={config.label}
    transition:fade={{ duration: 150 }}
  >
    {#if config.pulse}
      <span
        class="absolute inset-0 rounded-full {config.bgColor} animate-ping opacity-75"
      ></span>
    {/if}
  </div>
{/if}

<style>
  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }

  .animate-ping {
    animation: ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
  }

  @keyframes ping {
    75%,
    100% {
      transform: scale(1.5);
      opacity: 0;
    }
  }
</style>
