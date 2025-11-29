<script>
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import PerformanceMonitor from "../../components/tools/PerformanceMonitor.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import {
    performanceMetrics,
    performanceHistory,
    cacheStats,
    backgroundJobs,
    systemInfo,
    performanceScore,
    performanceStatus,
  } from "../../stores/performance.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<PageWrapper>
  <PageHeader
    title={tr("settings")}
    subtitle={tr("optimizePerformanceDescription")}
    icon="speedometer2"
  />

  <div class="space-y-6">
    <!-- Main Performance Monitor Component -->
    <PerformanceMonitor />

    <!-- Quick Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <ModernCard variant="glass" hoverable class="flex items-center gap-4 p-6">
        <div class="text-4xl">⚡</div>
        <div class="flex-1">
          <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
            Performance Score
          </div>
          <div
            class="text-2xl font-bold text-gray-900 dark:text-gray-100 {$performanceStatus.color}"
          >
            {$performanceScore}/100
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500">
            {$performanceStatus.level}
          </div>
        </div>
      </ModernCard>

      <ModernCard variant="glass" hoverable class="flex items-center gap-4 p-6">
        <div class="text-4xl">🖥️</div>
        <div class="flex-1">
          <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
            {tr("cpuUsage")}
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {$performanceMetrics.cpu_usage.toFixed(1)}%
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500">
            {tr("systemLoad")}
          </div>
        </div>
      </ModernCard>

      <ModernCard variant="glass" hoverable class="flex items-center gap-4 p-6">
        <div class="text-4xl">💾</div>
        <div class="flex-1">
          <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
            Memory Usage
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {$performanceMetrics.memory_usage.toFixed(1)}%
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500">
            RAM Utilization
          </div>
        </div>
      </ModernCard>

      <ModernCard variant="glass" hoverable class="flex items-center gap-4 p-6">
        <div class="text-4xl">🗄️</div>
        <div class="flex-1">
          <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
            Cache Hit Ratio
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {($performanceMetrics.cache_hit_ratio * 100).toFixed(1)}%
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500">
            Cache Efficiency
          </div>
        </div>
      </ModernCard>

      <ModernCard variant="glass" hoverable class="flex items-center gap-4 p-6">
        <div class="text-4xl">🔗</div>
        <div class="flex-1">
          <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
            Active Connections
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {$performanceMetrics.active_connections}
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500">
            WebSocket Clients
          </div>
        </div>
      </ModernCard>

      <ModernCard variant="glass" hoverable class="flex items-center gap-4 p-6">
        <div class="text-4xl">⏱️</div>
        <div class="flex-1">
          <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
            Avg Response Time
          </div>
          <div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {$performanceMetrics.average_response_time.toFixed(1)}ms
          </div>
          <div class="text-xs text-gray-500 dark:text-gray-500">
            API Performance
          </div>
        </div>
      </ModernCard>
    </div>

    <!-- System Information Panel -->
    <ModernCard variant="glass">
      <h3
        class="text-xl font-bold text-gray-900 dark:text-gray-100 mb-4 flex items-center gap-2"
      >
        <i class="bi bi-info-circle text-primary-600 dark:text-primary-400"></i>
        System Information
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          class="flex justify-between items-center pb-3 border-b border-gray-200 dark:border-gray-700"
        >
          <span class="text-sm text-gray-600 dark:text-gray-400"
            >CPU Cores:</span
          >
          <span class="font-medium text-gray-900 dark:text-gray-100"
            >{$systemInfo.cpu_cores}</span
          >
        </div>
        <div
          class="flex justify-between items-center pb-3 border-b border-gray-200 dark:border-gray-700"
        >
          <span class="text-sm text-gray-600 dark:text-gray-400"
            >Total Memory:</span
          >
          <span class="font-medium text-gray-900 dark:text-gray-100"
            >{$systemInfo.memory_total}</span
          >
        </div>
        <div
          class="flex justify-between items-center pb-3 border-b border-gray-200 dark:border-gray-700"
        >
          <span class="text-sm text-gray-600 dark:text-gray-400"
            >Disk Space:</span
          >
          <span class="font-medium text-gray-900 dark:text-gray-100"
            >{$systemInfo.disk_space}</span
          >
        </div>
        <div
          class="flex justify-between items-center pb-3 border-b border-gray-200 dark:border-gray-700"
        >
          <span class="text-sm text-gray-600 dark:text-gray-400">Uptime:</span>
          <span class="font-medium text-gray-900 dark:text-gray-100"
            >{$systemInfo.uptime}</span
          >
        </div>
        <div
          class="flex justify-between items-center pb-3 border-b border-gray-200 dark:border-gray-700"
        >
          <span class="text-sm text-gray-600 dark:text-gray-400">Version:</span>
          <span class="font-medium text-gray-900 dark:text-gray-100"
            >{$systemInfo.version}</span
          >
        </div>
        <div
          class="flex justify-between items-center pb-3 border-b border-gray-200 dark:border-gray-700"
        >
          <span class="text-sm text-gray-600 dark:text-gray-400"
            >Rust Version:</span
          >
          <span class="font-medium text-gray-900 dark:text-gray-100"
            >{$systemInfo.rust_version}</span
          >
        </div>
      </div>
    </ModernCard>

    <!-- Background Jobs Panel -->
    <ModernCard variant="glass">
      <h3
        class="text-xl font-bold text-gray-900 dark:text-gray-100 mb-4 flex items-center gap-2"
      >
        <i class="bi bi-gear-fill text-primary-600 dark:text-primary-400"></i>
        Background Jobs
      </h3>
      <div class="flex gap-8 mb-4">
        <div class="flex flex-col gap-1">
          <span class="text-sm text-gray-600 dark:text-gray-400"
            >Queue Length:</span
          >
          <span class="text-xl font-bold text-gray-900 dark:text-gray-100"
            >{$backgroundJobs.queue_length}</span
          >
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-sm text-gray-600 dark:text-gray-400"
            >Active Workers:</span
          >
          <span class="text-xl font-bold text-gray-900 dark:text-gray-100"
            >{$backgroundJobs.active_workers}</span
          >
        </div>
      </div>

      {#if $backgroundJobs.jobs.length > 0}
        <div class="space-y-3">
          {#each $backgroundJobs.jobs as job}
            <div
              class="bg-white/50 dark:bg-gray-800/50 rounded-lg p-4 border border-gray-200/50 dark:border-gray-700/50"
            >
              <div class="flex justify-between items-center mb-2">
                <span class="font-medium text-gray-900 dark:text-gray-100"
                  >{job.job_type}</span
                >
                <span
                  class="badge-glass-{job.status === 'completed'
                    ? 'success'
                    : job.status === 'failed'
                      ? 'error'
                      : job.status === 'running'
                        ? 'info'
                        : 'warning'}"
                >
                  {job.status}
                </span>
              </div>
              {#if job.progress !== undefined}
                <div class="flex items-center gap-3">
                  <div
                    class="flex-1 h-1.5 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden"
                  >
                    <div
                      class="h-full bg-gradient-to-r from-primary-500 to-primary-600 transition-all duration-300"
                      style="width: {job.progress}%"
                    ></div>
                  </div>
                  <span
                    class="text-sm text-gray-600 dark:text-gray-400 min-w-[3rem] text-right"
                    >{job.progress}%</span
                  >
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {:else}
        <div class="text-center py-8 text-gray-500 dark:text-gray-400 italic">
          No background jobs in queue
        </div>
      {/if}
    </ModernCard>
  </div>
</PageWrapper>

<style>
  .excellent {
    color: rgb(var(--color-primary-600));
  }
  :global(.dark) .excellent {
    color: rgb(var(--color-primary-400));
  }
  .good {
    color: rgb(var(--color-secondary-600));
  }
  :global(.dark) .good {
    color: rgb(var(--color-secondary-400));
  }
  .fair,
  .poor {
    color: rgb(239 68 68); /* red-500 */
  }
  :global(.dark) .fair,
  :global(.dark) .poor {
    color: rgb(248 113 113); /* red-400 */
  }
</style>
