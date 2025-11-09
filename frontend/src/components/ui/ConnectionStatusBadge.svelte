<script>
  /**
   * Connection Status Badge Component
   * 
   * Visual indicator for WebSocket connection status with:
   * - Connection status display (Connected/Connecting/Reconnecting/Failed)
   * - Auto-hide when connected
   * - Retry countdown display
   * - Tooltip with details
   * - Dark mode support
   * - Accessible ARIA labels
   * 
   * @component
   * @example
   *   <ConnectionStatusBadge />
   */

  import { wsState, wsReconnectAttempts, WS_STATES } from '../stores/websocket.js';
  import { websocketManager } from '../stores/websocket.js';

  // Props
  export let position = 'bottom-left'; // bottom-left, bottom-right, top-left, top-right

  // State
  let nextRetryIn = 0;
  let countdownInterval = null;

  /**
   * Get badge config
   */
  function getBadgeConfig(status) {
    const configs = {
      [WS_STATES.CONNECTED]: {
        icon: 'bi-wifi',
        label: 'Verbunden',
        color: 'bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300',
        dotColor: 'bg-green-500',
        show: false, // Hide when connected
      },
      [WS_STATES.CONNECTING]: {
        icon: 'bi-wifi-1',
        label: 'Verbinden...',
        color: 'bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300',
        dotColor: 'bg-blue-500 animate-pulse',
        show: true,
      },
      [WS_STATES.RECONNECTING]: {
        icon: 'bi-arrow-repeat',
        label: 'Erneut verbinden...',
        color: 'bg-yellow-100 dark:bg-yellow-900 text-yellow-700 dark:text-yellow-300',
        dotColor: 'bg-yellow-500 animate-pulse',
        show: true,
      },
      [WS_STATES.ERROR]: {
        icon: 'bi-exclamation-triangle',
        label: 'Fehler',
        color: 'bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-300',
        dotColor: 'bg-red-500',
        show: true,
      },
      [WS_STATES.MAX_RETRIES_REACHED]: {
        icon: 'bi-x-circle',
        label: 'Verbindung verloren',
        color: 'bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-300',
        dotColor: 'bg-red-500',
        show: true,
      },
      [WS_STATES.DISCONNECTED]: {
        icon: 'bi-wifi-off',
        label: 'Getrennt',
        color: 'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300',
        dotColor: 'bg-gray-500',
        show: false,
      },
    };

    return configs[status] || configs[WS_STATES.DISCONNECTED];
  }

  /**
   * Get position classes
   */
  function getPositionClasses() {
    const positions = {
      'bottom-left': 'bottom-4 left-4',
      'bottom-right': 'bottom-4 right-4',
      'top-left': 'top-4 left-4',
      'top-right': 'top-4 right-4',
    };
    return positions[position] || positions['bottom-left'];
  }

  /**
   * Format countdown
   */
  function formatCountdown(ms) {
    const seconds = Math.ceil(ms / 1000);
    if (seconds < 60) return `${seconds}s`;
    const minutes = Math.ceil(seconds / 60);
    return `${minutes}m`;
  }

  /**
   * Update countdown
   */
  function updateCountdown() {
    const stats = websocketManager.getStats();
    if (stats && stats.nextRetryIn) {
      const remaining = new Date(stats.nextRetryIn).getTime() - Date.now();
      if (remaining > 0) {
        nextRetryIn = remaining;
      }
    }
  }

  $: {
    if ($wsState === WS_STATES.RECONNECTING) {
      if (!countdownInterval) {
        countdownInterval = setInterval(() => {
          updateCountdown();
        }, 100);
      }
    } else {
      if (countdownInterval) {
        clearInterval(countdownInterval);
        countdownInterval = null;
      }
      nextRetryIn = 0;
    }
  }

  $: config = getBadgeConfig($wsState);
  $: positionClasses = getPositionClasses();
  $: showBadge = config.show;
</script>

{#if showBadge}
  <div
    class="fixed {positionClasses} z-40 transition-all duration-300"
    role="status"
    aria-label="WebSocket Verbindungsstatus: {config.label}"
    aria-live="polite"
  >
    <div
      class="flex items-center gap-2 px-3 py-2 rounded-lg shadow-lg border {config.color} backdrop-blur-sm"
      title="{config.label}{$wsReconnectAttempts > 0 ? ` (Versuch ${$wsReconnectAttempts})` : ''}"
    >
      <!-- Status dot -->
      <div class="flex-shrink-0 w-2 h-2 rounded-full {config.dotColor}" aria-hidden="true" />

      <!-- Icon + Label -->
      <div class="flex items-center gap-1">
        <i class="bi {config.icon} text-sm" />
        <span class="text-sm font-medium whitespace-nowrap">
          {config.label}
        </span>
      </div>

      <!-- Countdown (for reconnecting state) -->
      {#if $wsState === WS_STATES.RECONNECTING && nextRetryIn > 0}
        <span class="text-xs opacity-75 whitespace-nowrap">
          in {formatCountdown(nextRetryIn)}
        </span>
      {/if}

      <!-- Retry count -->
      {#if $wsReconnectAttempts > 0}
        <span class="text-xs opacity-75 whitespace-nowrap">
          ({$wsReconnectAttempts}/15)
        </span>
      {/if}

      <!-- Close button (for error states) -->
      {#if $wsState === WS_STATES.ERROR || $wsState === WS_STATES.MAX_RETRIES_REACHED}
        <button
          class="ml-2 text-sm opacity-75 hover:opacity-100 transition-opacity"
          on:click={() => location.reload()}
          title="Seite neu laden"
          aria-label="Verbindung zurücksetzen durch Seite neu laden"
        >
          <i class="bi bi-arrow-clockwise" />
        </button>
      {/if}
    </div>

    <!-- Details tooltip on hover -->
    <div class="opacity-0 hover:opacity-100 transition-opacity absolute top-full mt-2 left-0 pointer-events-none text-xs text-gray-600 dark:text-gray-400 whitespace-nowrap">
      <div class="bg-gray-900 dark:bg-gray-200 text-white dark:text-gray-900 px-2 py-1 rounded">
        {$wsState === WS_STATES.CONNECTED ? 'Alle Systeme funktionieren' : ''}
        {$wsState === WS_STATES.CONNECTING ? 'Verbindung wird hergestellt...' : ''}
        {$wsState === WS_STATES.RECONNECTING ? `Erneut verbinden... (${$wsReconnectAttempts}/15)` : ''}
        {$wsState === WS_STATES.ERROR ? 'Verbindungsfehler aufgetreten' : ''}
        {$wsState === WS_STATES.MAX_RETRIES_REACHED ? 'Maximale Versuche erreicht' : ''}
        {$wsState === WS_STATES.DISCONNECTED ? 'Vom Server getrennt' : ''}
      </div>
    </div>
  </div>
{/if}

<style>
  :global(.animate-pulse) {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
