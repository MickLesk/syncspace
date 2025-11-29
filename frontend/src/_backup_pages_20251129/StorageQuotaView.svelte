<script>
  import { onMount } from 'svelte';
  import { storageQuota, storageStatus, typeBreakdown, remainingStorage, formatStorageSize } from '$stores/storageQuota.js';
  import { t } from '$lib/i18n.js';

  let chartContainer;
  let chartInstance = null;
  let showFolderBreakdown = false;
  let selectedType = null;

  onMount(async () => {
    // Load storage data on mount
    await storageQuota.loadQuotaData();

    // Initialize pie chart for type distribution
    initializeChart();
  });

  function initializeChart() {
    if (!chartContainer || !$typeBreakdown.length) return;

    // Destroy existing chart
    if (chartInstance) {
      chartInstance.destroy?.();
    }

    // Create simple SVG pie chart (no external library)
    const svg = createPieChart($storageQuota.byType);
    chartContainer.innerHTML = '';
    chartContainer.appendChild(svg);
  }

  function createPieChart(typeData) {
    const size = 300;
    const center = size / 2;
    const radius = 100;

    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('viewBox', `0 0 ${size} ${size}`);
    svg.setAttribute('class', 'w-full h-full');

    let currentAngle = -90; // Start at top

    Object.entries(typeData).forEach(([type, data]) => {
      const sliceAngle = (data.percentage / 100) * 360;
      const startAngle = currentAngle * (Math.PI / 180);
      const endAngle = (currentAngle + sliceAngle) * (Math.PI / 180);

      const x1 = center + radius * Math.cos(startAngle);
      const y1 = center + radius * Math.sin(startAngle);
      const x2 = center + radius * Math.cos(endAngle);
      const y2 = center + radius * Math.sin(endAngle);

      const largeArc = sliceAngle > 180 ? 1 : 0;

      const pathData = [
        `M ${center} ${center}`,
        `L ${x1} ${y1}`,
        `A ${radius} ${radius} 0 ${largeArc} 1 ${x2} ${y2}`,
        'Z',
      ].join(' ');

      const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
      path.setAttribute('d', pathData);
      path.setAttribute('fill', data.color);
      path.setAttribute('stroke', 'white');
      path.setAttribute('stroke-width', '2');
      path.setAttribute('class', 'hover:opacity-80 cursor-pointer transition-opacity');
      path.setAttribute('data-type', type);

      path.addEventListener('click', () => {
        selectedType = selectedType === type ? null : type;
      });

      svg.appendChild(path);
      currentAngle += sliceAngle;
    });

    return svg;
  }

  function toggleFolderBreakdown() {
    showFolderBreakdown = !showFolderBreakdown;
  }

  function getStatusColor(status) {
    switch (status) {
      case 'success':
        return 'text-green-600 dark:text-green-400';
      case 'warning':
        return 'text-yellow-600 dark:text-yellow-400';
      case 'danger':
        return 'text-red-600 dark:text-red-400';
      default:
        return 'text-gray-600 dark:text-gray-400';
    }
  }

  function getProgressColor(status) {
    switch (status) {
      case 'success':
        return 'bg-green-500';
      case 'warning':
        return 'bg-yellow-500';
      case 'danger':
        return 'bg-red-500';
      default:
        return 'bg-blue-500';
    }
  }

  // Reactive: update chart when data changes
  $: if ($storageQuota.byType && Object.keys($storageQuota.byType).length > 0) {
    initializeChart();
  }
</script>

<div class="p-6 bg-white dark:bg-gray-900 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
  <!-- Header -->
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
      <i class="bi bi-pie-chart mr-2"></i>
      {t('storage.title')}
    </h2>
    <p class="text-sm text-gray-600 dark:text-gray-400">
      {t('storage.description')}
    </p>
  </div>

  {#if $storageQuota.isLoading}
    <div class="flex items-center justify-center h-64">
      <div class="text-center">
        <div class="animate-spin mb-4">
          <i class="bi bi-hourglass text-3xl text-gray-400"></i>
        </div>
        <p class="text-gray-600 dark:text-gray-400">{t('common.loading')}</p>
      </div>
    </div>
  {:else if $storageQuota.error}
    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-700 rounded-lg p-4 mb-6">
      <div class="flex items-center">
        <i class="bi bi-exclamation-circle text-red-600 dark:text-red-400 mr-3"></i>
        <p class="text-red-700 dark:text-red-300">{$storageQuota.error}</p>
      </div>
    </div>
  {:else}
    <!-- Storage Summary -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
      <!-- Used Storage -->
      <div class="bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-900/10 rounded-lg p-4 border border-blue-200 dark:border-blue-700">
        <p class="text-sm font-medium text-blue-700 dark:text-blue-300 mb-2">{t('storage.used')}</p>
        <p class="text-2xl font-bold text-blue-900 dark:text-blue-100">{formatStorageSize($storageQuota.totalUsed)}</p>
        <p class="text-xs text-blue-600 dark:text-blue-400 mt-1">{$storageQuota.percentUsed}% {t('storage.of_quota')}</p>
      </div>

      <!-- Available Storage -->
      <div class="bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-900/10 rounded-lg p-4 border border-green-200 dark:border-green-700">
        <p class="text-sm font-medium text-green-700 dark:text-green-300 mb-2">{t('storage.available')}</p>
        <p class="text-2xl font-bold text-green-900 dark:text-green-100">{$remainingStorage}</p>
        <p class="text-xs text-green-600 dark:text-green-400 mt-1">{formatStorageSize($storageQuota.totalQuota)}</p>
      </div>

      <!-- Status Indicator -->
      <div class="bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/20 dark:to-purple-900/10 rounded-lg p-4 border border-purple-200 dark:border-purple-700">
        <p class="text-sm font-medium text-purple-700 dark:text-purple-300 mb-2">{t('storage.status')}</p>
        <div class="flex items-center">
          <i class="bi bi-circle-fill text-xl mr-2" class:text-green-500={$storageStatus === 'success'} class:text-yellow-500={$storageStatus === 'warning'} class:text-red-500={$storageStatus === 'danger'}></i>
          <span class="text-lg font-semibold" class:text-green-700={$storageStatus === 'success'} class:text-yellow-700={$storageStatus === 'warning'} class:text-red-700={$storageStatus === 'danger'} class:dark:text-green-100={$storageStatus === 'success'} class:dark:text-yellow-100={$storageStatus === 'warning'} class:dark:text-red-100={$storageStatus === 'danger'}>
            {$storageStatus === 'success' ? t('storage.status_good') : $storageStatus === 'warning' ? t('storage.status_warning') : t('storage.status_critical')}
          </span>
        </div>
      </div>
    </div>

    <!-- Progress Bar -->
    <div class="mb-8">
      <div class="flex justify-between items-center mb-2">
        <p class="text-sm font-medium text-gray-700 dark:text-gray-300">{t('storage.usage')}</p>
        <p class="text-sm font-semibold {getStatusColor($storageStatus)}">{$storageQuota.percentUsed}%</p>
      </div>
      <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 overflow-hidden">
        <div 
          class="h-full transition-all duration-300 {getProgressColor($storageStatus)}"
          style="width: {Math.min($storageQuota.percentUsed, 100)}%"
        ></div>
      </div>
    </div>

    <!-- Charts Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <!-- Pie Chart -->
      <div>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          {t('storage.by_type')}
        </h3>
        <div bind:this={chartContainer} class="flex justify-center items-center h-64">
          <p class="text-gray-500 dark:text-gray-400">{t('common.loading')}</p>
        </div>
      </div>

      <!-- Type Breakdown List -->
      <div>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          {t('storage.breakdown')}
        </h3>
        <div class="space-y-3">
          {#each $typeBreakdown as item}
            <div 
              class="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors cursor-pointer"
              class:ring-2={selectedType === item.type}
              class:ring-blue-500={selectedType === item.type}
              on:click={() => { selectedType = selectedType === item.type ? null : item.type; }}
            >
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center">
                  <i class="bi {item.icon} text-lg mr-3" style="color: {item.color}"></i>
                  <div>
                    <p class="font-medium text-gray-900 dark:text-white text-sm">{item.type}</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">{item.count} {t('storage.files')}</p>
                  </div>
                </div>
                <div class="text-right">
                  <p class="font-semibold text-gray-900 dark:text-white text-sm">{item.percentage}%</p>
                  <p class="text-xs text-gray-500 dark:text-gray-400">{formatStorageSize(item.size)}</p>
                </div>
              </div>
              <div class="w-full bg-gray-300 dark:bg-gray-600 rounded-full h-2 overflow-hidden">
                <div 
                  class="h-full transition-all duration-300"
                  style="width: {item.percentage}%; background-color: {item.color};"
                ></div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Folder Breakdown -->
    <div class="mt-8 pt-8 border-t border-gray-200 dark:border-gray-700">
      <button
        on:click={toggleFolderBreakdown}
        class="flex items-center text-lg font-semibold text-gray-900 dark:text-white mb-4 hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
      >
        <i class="bi bi-folder mr-2"></i>
        {t('storage.by_folder')}
        <i class="bi ml-2 transition-transform" class:bi-chevron-down={!showFolderBreakdown} class:bi-chevron-up={showFolderBreakdown}></i>
      </button>

      {#if showFolderBreakdown}
        <div class="space-y-3">
          {#each $storageQuota.byFolder as folder}
            <div class="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center">
                  <i class="bi bi-folder-fill text-yellow-500 mr-3 text-lg"></i>
                  <div>
                    <p class="font-medium text-gray-900 dark:text-white">{folder.path}</p>
                  </div>
                </div>
                <div class="text-right">
                  <p class="font-semibold text-gray-900 dark:text-white">{folder.percentage}%</p>
                  <p class="text-xs text-gray-500 dark:text-gray-400">{formatStorageSize(folder.size)}</p>
                </div>
              </div>
              <div class="w-full bg-gray-300 dark:bg-gray-600 rounded-full h-2 overflow-hidden">
                <div 
                  class="h-full bg-yellow-500 transition-all duration-300"
                  style="width: {folder.percentage}%"
                ></div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Last Updated -->
    {#if $storageQuota.lastUpdated}
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-6 text-center">
        {t('storage.last_updated')} {new Date($storageQuota.lastUpdated).toLocaleString()}
      </p>
    {/if}
  {/if}
</div>

<style>
  :global(.dark) {
    color-scheme: dark;
  }
</style>
