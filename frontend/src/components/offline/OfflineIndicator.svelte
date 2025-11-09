<script>
  // OfflineIndicator.svelte - Shows offline status and queue
  import { offlineManager, formatCacheSize } from '../../stores/offlineManager';
  import { t } from '../../lib/i18n';

  let showDetails = $state(false);

  const {
    isOnline,
    isSyncing,
    offlineQueue,
    isOfflineMode,
  } = offlineManager;
</script>

<!-- Offline Banner (appears when offline or pending sync) -->
{#if $isOfflineMode}
  <div class="sticky top-16 md:top-0 z-20 bg-amber-50 dark:bg-amber-900 border-b-2 border-amber-300 dark:border-amber-700 px-4 py-3 shadow-sm">
    <div class="flex items-center justify-between gap-4 max-w-7xl mx-auto">
      <!-- Status -->
      <div class="flex items-center gap-2 flex-1 min-w-0">
        {#if !$isOnline}
          <div class="flex items-center gap-2">
            <i class="bi bi-wifi-off text-amber-700 dark:text-amber-200 text-lg flex-shrink-0"></i>
            <span class="text-sm font-semibold text-amber-900 dark:text-amber-100 truncate">
              {'Offline - Using cached data'}
            </span>
          </div>
        {:else if $isSyncing}
          <div class="flex items-center gap-2">
            <i class="bi bi-arrow-clockwise animate-spin text-blue-600 dark:text-blue-400 text-lg flex-shrink-0"></i>
            <span class="text-sm font-semibold text-blue-900 dark:text-blue-100 truncate">
              {'Syncing {' + $offlineQueue.length + '} pending changes...'}
            </span>
          </div>
        {:else if $offlineQueue.length > 0}
          <div class="flex items-center gap-2">
            <i class="bi bi-exclamation-circle text-orange-600 dark:text-orange-400 text-lg flex-shrink-0"></i>
            <span class="text-sm font-semibold text-orange-900 dark:text-orange-100 truncate">
              {$offlineQueue.length + ' pending change' + ($offlineQueue.length !== 1 ? 's' : '')}
            </span>
          </div>
        {/if}
      </div>

      <!-- Actions -->
      <div class="flex items-center gap-2 flex-shrink-0">
        <button
          on:click={() => showDetails = !showDetails}
          class="px-2 py-1 text-xs font-medium rounded-md transition hover:bg-white hover:bg-opacity-50 dark:hover:bg-gray-800"
          title="Show details"
        >
          <i class={`bi bi-chevron-${showDetails ? 'up' : 'down'}`}></i>
        </button>

        {#if $isSyncing}
          <button
            disabled
            class="px-3 py-1 text-xs font-medium rounded-md bg-gray-300 dark:bg-gray-700 text-gray-600 dark:text-gray-400 cursor-not-allowed"
          >
            Syncing...
          </button>
        {:else if $offlineQueue.length > 0 && $isOnline}
          <button
            on:click={() => offlineManager.syncOfflineQueue()}
            class="px-3 py-1 text-xs font-medium rounded-md bg-blue-600 hover:bg-blue-700 text-white transition"
          >
            Sync Now
          </button>
        {/if}
      </div>
    </div>

    <!-- Expandable Details -->
    {#if showDetails && $offlineQueue.length > 0}
      <div class="mt-3 pt-3 border-t border-amber-200 dark:border-amber-700">
        <p class="text-xs text-amber-900 dark:text-amber-200 mb-2 font-semibold">
          Pending changes:
        </p>
        <div class="space-y-1 max-h-32 overflow-y-auto">
          {#each $offlineQueue as operation, i}
            <div class="text-xs text-amber-800 dark:text-amber-300 flex items-center gap-2 px-2 py-1 bg-white dark:bg-gray-800 rounded">
              <i class={`bi bi-${getOperationIcon(operation.method)} flex-shrink-0`}></i>
              <span class="truncate flex-1">{operation.url}</span>
              <span class="text-amber-600 dark:text-amber-400 text-xs flex-shrink-0">
                #{i + 1}
              </span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
{/if}

<!-- Queue status in footer (small indicator) -->
<div class="fixed bottom-4 right-4 z-10">
  {#if !$isOnline}
    <div class="px-3 py-2 rounded-lg bg-amber-600 dark:bg-amber-700 text-white text-xs font-medium shadow-lg flex items-center gap-2">
      <i class="bi bi-wifi-off"></i>
      Offline
    </div>
  {:else if $isSyncing}
    <div class="px-3 py-2 rounded-lg bg-blue-600 dark:bg-blue-700 text-white text-xs font-medium shadow-lg flex items-center gap-2">
      <i class="bi bi-arrow-clockwise animate-spin"></i>
      Syncing ({$offlineQueue.length})
    </div>
  {:else if $offlineQueue.length > 0}
    <div class="px-3 py-2 rounded-lg bg-orange-600 dark:bg-orange-700 text-white text-xs font-medium shadow-lg flex items-center gap-2">
      <i class="bi bi-exclamation-circle"></i>
      {$offlineQueue.length} pending
    </div>
  {/if}
</div>

<script>
  function getOperationIcon(method) {
    switch (method?.toUpperCase()) {
      case 'POST':
        return 'plus-circle';
      case 'PUT':
        return 'pencil-square';
      case 'DELETE':
        return 'trash';
      default:
        return 'arrow-up';
    }
  }
</script>

<style>
  :global(.animate-spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
