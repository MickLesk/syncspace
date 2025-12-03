<script>
  import { backupRestore } from '../../stores/backupRestore';
  import { t } from '../../lib/i18n';

  let showRestoreConfirm = false;
  let restoreBackupId = null;
  let expandedBackupId = null;
  let showDeleteConfirm = false;
  let deleteBackupId = null;

  const { 
    filteredBackups, 
    stats, 
    restoreProgress, 
    isRestoring, 
    restoreError,
    selectedBackup,
    filterType,
    searchQuery,
    sortBy,
    sortOrder,
    loadBackups,
    restoreBackup,
    cancelRestore,
    verifyBackup,
    deleteBackup,
    formatSize,
    formatDuration,
    getStatusColor,
    exportBackupList
  } = backupRestore;

  onMount(() => {
    loadBackups();
  });

  function handleRestore(backup) {
    restoreBackupId = backup.id;
    showRestoreConfirm = true;
  }

  async function confirmRestore() {
    await restoreBackup(restoreBackupId);
    showRestoreConfirm = false;
  }

  function handleDelete(backup) {
    deleteBackupId = backup.id;
    showDeleteConfirm = true;
  }

  function confirmDelete() {
    deleteBackup(deleteBackupId);
    showDeleteConfirm = false;
  }

  function toggleExpanded(backupId) {
    expandedBackupId = expandedBackupId === backupId ? null : backupId;
  }

  import { onMount } from 'svelte';
</script>

<div class="p-6 max-w-7xl mx-auto">
  <!-- Header -->
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
      <i class="bi bi-arrow-counterclockwise inline-block mr-3" aria-hidden="true"></i>
      {$t('backup.title')}
    </h1>
    <p class="text-gray-600 dark:text-gray-400">
      {$t('backup.description')}
    </p>
  </div>

  <!-- Statistics Cards -->
  <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
    <div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700">
      <div class="text-sm text-gray-600 dark:text-gray-400">{$t('backup.total_backups')}</div>
      <div class="text-3xl font-bold text-gray-900 dark:text-white mt-1">{$stats.totalBackups}</div>
    </div>
    <div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700">
      <div class="text-sm text-gray-600 dark:text-gray-400">{$t('backup.total_size')}</div>
      <div class="text-3xl font-bold text-gray-900 dark:text-white mt-1">{formatSize($stats.totalSize)}</div>
    </div>
    <div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700">
      <div class="text-sm text-gray-600 dark:text-gray-400">{$t('backup.success_count')}</div>
      <div class="text-3xl font-bold text-green-600 dark:text-green-400 mt-1">{$stats.successCount}</div>
    </div>
    <div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700">
      <div class="text-sm text-gray-600 dark:text-gray-400">{$t('backup.avg_duration')}</div>
      <div class="text-3xl font-bold text-gray-900 dark:text-white mt-1">{formatDuration($stats.avgDuration)}</div>
    </div>
  </div>

  <!-- Controls -->
  <div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700 mb-8">
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <!-- Search -->
      <div class="relative">
        <i class="bi bi-search absolute left-3 top-3 text-gray-400" aria-hidden="true"></i>
        <input
          type="text"
          placeholder={$t('backup.search_placeholder')}
          bind:value={$searchQuery}
          class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <!-- Filter by type -->
      <select
        bind:value={$filterType}
        class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="all">{$t('backup.all_types')}</option>
        <option value="full">{$t('backup.type_full')}</option>
        <option value="incremental">{$t('backup.type_incremental')}</option>
        <option value="differential">{$t('backup.type_differential')}</option>
      </select>

      <!-- Sort by -->
      <select
        bind:value={$sortBy}
        class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="date">{$t('backup.sort_date')}</option>
        <option value="size">{$t('backup.sort_size')}</option>
        <option value="status">{$t('backup.sort_status')}</option>
      </select>

      <!-- Sort order -->
      <button
        on:click={() => $sortOrder = $sortOrder === 'asc' ? 'desc' : 'asc'}
        class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition"
        title={$t('backup.toggle_sort_order')}
      >
        <i class={`bi bi-arrow-${$sortOrder === 'asc' ? 'up' : 'down'}`}></i>
        {$sortOrder === 'asc' ? $t('backup.ascending') : $t('backup.descending')}
      </button>
    </div>

    <!-- Export button -->
    <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
      <button
        on:click={exportBackupList}
        class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition flex items-center gap-2"
      >
        <i class="bi bi-download" aria-hidden="true"></i>
        {$t('backup.export_list')}
      </button>
    </div>
  </div>

  <!-- Restore Progress Modal -->
  {#if $isRestoring}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 rounded-lg p-4">
      <div class="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
          {$t('backup.restoring_backup')}
        </h3>
        
        {#if $selectedBackup}
          <p class="text-gray-600 dark:text-gray-400 mb-4">
            {$selectedBackup.name}
          </p>
        {/if}

        <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-lg h-4 overflow-hidden mb-4">
          <div
            class="bg-green-500 h-full transition-all duration-300"
            style="width: {Math.min($restoreProgress, 100)}%"
          ></div>
        </div>

        <div class="text-center text-gray-600 dark:text-gray-400 mb-4">
          {Math.min(Math.round($restoreProgress), 100)}%
        </div>

        {#if $restoreError}
          <div class="bg-red-50 dark:bg-red-900 border border-red-200 dark:border-red-700 rounded-lg p-3 mb-4 text-red-700 dark:text-red-200">
            <i class="bi bi-exclamation-circle mr-2" aria-hidden="true"></i>
            {$restoreError}
          </div>
        {/if}

        <button
          on:click={cancelRestore}
          class="w-full px-4 py-2 bg-gray-300 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-400 dark:hover:bg-gray-600 transition"
        >
          {$t('common.cancel')}
        </button>
      </div>
    </div>
  {/if}

  <!-- Restore Confirmation Modal -->
  {#if showRestoreConfirm}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 rounded-lg p-4">
      <div class="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
          {$t('backup.confirm_restore')}
        </h3>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          {$t('backup.restore_warning')}
        </p>
        <div class="flex gap-4">
          <button
            on:click={() => showRestoreConfirm = false}
            class="flex-1 px-4 py-2 bg-gray-300 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-400 dark:hover:bg-gray-600 transition"
          >
            {$t('common.cancel')}
          </button>
          <button
            on:click={confirmRestore}
            class="flex-1 px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition"
          >
            {$t('backup.restore_confirm')}
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Delete Confirmation Modal -->
  {#if showDeleteConfirm}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 rounded-lg p-4">
      <div class="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
          {$t('backup.confirm_delete')}
        </h3>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          {$t('backup.delete_warning')}
        </p>
        <div class="flex gap-4">
          <button
            on:click={() => showDeleteConfirm = false}
            class="flex-1 px-4 py-2 bg-gray-300 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-400 dark:hover:bg-gray-600 transition"
          >
            {$t('common.cancel')}
          </button>
          <button
            on:click={confirmDelete}
            class="flex-1 px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition"
          >
            {$t('common.delete')}
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Backups List -->
  <div class="space-y-4">
    {#if $filteredBackups.length === 0}
      <div class="bg-white dark:bg-gray-800 rounded-lg p-12 border border-gray-200 dark:border-gray-700 text-center">
        <i class="bi bi-inbox text-4xl text-gray-400 mb-4 block" aria-hidden="true"></i>
        <p class="text-gray-600 dark:text-gray-400">
          {$t('backup.no_backups')}
        </p>
      </div>
    {:else}
      {#each $filteredBackups as backup (backup.id)}
        <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden hover:shadow-md transition">
          <!-- Main Row -->
          <button
            on:click={() => toggleExpanded(backup.id)}
            class="w-full p-4 text-left hover:bg-gray-50 dark:hover:bg-gray-700 transition flex items-center justify-between"
          >
            <div class="flex items-center gap-4 flex-1">
              <!-- Type Icon -->
              <div class="text-2xl">
                {#if backup.type === 'full'}
                  <i class="bi bi-folder-fill text-blue-500" aria-hidden="true"></i>
                {:else if backup.type === 'incremental'}
                  <i class="bi bi-arrow-up-circle-fill text-green-500" aria-hidden="true"></i>
                {:else}
                  <i class="bi bi-diagram-2-fill text-purple-500" aria-hidden="true"></i>
                {/if}
              </div>

              <!-- Info -->
              <div class="flex-1">
                <div class="font-semibold text-gray-900 dark:text-white">{backup.name}</div>
                <div class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  {new Date(backup.date).toLocaleString('de-DE')} • {formatSize(backup.size)} • {backup.fileCount.toLocaleString()} {$t('backup.files')}
                </div>
              </div>

              <!-- Status Badge -->
              <div class="flex items-center gap-2">
                {#if backup.status === 'success'}
                  <span class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-sm font-medium bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200">
                    <i class="bi bi-check-circle" aria-hidden="true"></i>
                    {$t('backup.status_success')}
                  </span>
                {:else if backup.status === 'failed'}
                  <span class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-sm font-medium bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200">
                    <i class="bi bi-x-circle" aria-hidden="true"></i>
                    {$t('backup.status_failed')}
                  </span>
                {:else if backup.status === 'in_progress'}
                  <span class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-sm font-medium bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200">
                    <i class="bi bi-hourglass-split" aria-hidden="true"></i>
                    {$t('backup.status_in_progress')}
                  </span>
                {/if}
              </div>
            </div>

            <!-- Expand Arrow -->
            <i class={`bi bi-chevron-down ml-4 transition ${expandedBackupId === backup.id ? 'rotate-180' : ''}`}></i>
          </button>

          <!-- Expanded Details -->
          {#if expandedBackupId === backup.id}
            <div class="border-t border-gray-200 dark:border-gray-700 p-4 bg-gray-50 dark:bg-gray-900">
              <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                <div>
                  <div class="text-xs text-gray-600 dark:text-gray-400 uppercase font-semibold">{$t('backup.duration')}</div>
                  <div class="text-lg font-semibold text-gray-900 dark:text-white">{formatDuration(backup.duration)}</div>
                </div>
                <div>
                  <div class="text-xs text-gray-600 dark:text-gray-400 uppercase font-semibold">{$t('backup.compression')}</div>
                  <div class="text-lg font-semibold text-gray-900 dark:text-white">{(backup.compressionRatio * 100).toFixed(0)}%</div>
                </div>
                <div>
                  <div class="text-xs text-gray-600 dark:text-gray-400 uppercase font-semibold">{$t('backup.retention')}</div>
                  <div class="text-lg font-semibold text-gray-900 dark:text-white">{backup.retentionDays}d</div>
                </div>
                <div>
                  <div class="text-xs text-gray-600 dark:text-gray-400 uppercase font-semibold">{$t('backup.verification')}</div>
                  <div class="text-lg font-semibold">
                    {#if backup.verificationStatus === 'verified'}
                      <span class="text-green-600 dark:text-green-400">{$t('backup.verified')}</span>
                    {:else if backup.verificationStatus === 'verifying'}
                      <span class="text-yellow-600 dark:text-yellow-400">{$t('backup.verifying')}</span>
                    {:else}
                      <span class="text-gray-600 dark:text-gray-400">{$t('backup.pending')}</span>
                    {/if}
                  </div>
                </div>
              </div>

              {#if backup.notes}
                <div class="mb-6 p-3 bg-blue-50 dark:bg-blue-900 border border-blue-200 dark:border-blue-700 rounded-lg">
                  <div class="text-sm text-blue-900 dark:text-blue-200">
                    <i class="bi bi-info-circle mr-2" aria-hidden="true"></i>
                    {backup.notes}
                  </div>
                </div>
              {/if}

              <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                <button
                  on:click={() => handleRestore(backup)}
                  class="px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded-lg transition flex items-center justify-center gap-2"
                >
                  <i class="bi bi-arrow-counterclockwise" aria-hidden="true"></i>
                  {$t('backup.restore')}
                </button>
                <button
                  on:click={() => verifyBackup(backup.id)}
                  class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition flex items-center justify-center gap-2"
                >
                  <i class="bi bi-shield-check" aria-hidden="true"></i>
                  {$t('backup.verify')}
                </button>
                <button
                  on:click={() => handleDelete(backup)}
                  class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition flex items-center justify-center gap-2"
                >
                  <i class="bi bi-trash" aria-hidden="true"></i>
                  {$t('common.delete')}
                </button>
                <button
                  class="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded-lg transition flex items-center justify-center gap-2"
                  title="Download backup"
                >
                  <i class="bi bi-download" aria-hidden="true"></i>
                  {$t('common.download')}
                </button>
              </div>

              <!-- Encryption indicator -->
              <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700 flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
                <i class={`bi ${backup.encryptionEnabled ? 'bi-lock-fill text-green-600 dark:text-green-400' : 'bi-unlock'}`}></i>
                {backup.encryptionEnabled ? $t('backup.encrypted') : $t('backup.not_encrypted')}
              </div>
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  :global(body) {
    --color-success: #10b981;
    --color-error: #ef4444;
    --color-warning: #f59e0b;
    --color-info: #3b82f6;
  }

  :global(.dark) {
    --color-success: #34d399;
    --color-error: #f87171;
    --color-warning: #fbbf24;
    --color-info: #60a5fa;
  }
</style>
