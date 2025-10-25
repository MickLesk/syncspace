<script>
  import PerformanceMonitor from "../components/PerformanceMonitor.svelte";
  import {
    performanceMetrics,
    performanceHistory,
    cacheStats,
    backgroundJobs,
    systemInfo,
    performanceScore,
    performanceStatus,
  } from "../stores/performance.js";
</script>

<div class="performance-view">
  <div class="view-header">
    <h1>Performance Dashboard</h1>
    <p class="view-description">
      Monitor system performance, cache statistics, and background job
      processing
    </p>
  </div>

  <div class="performance-content">
    <!-- Main Performance Monitor Component -->
    <PerformanceMonitor />

    <!-- Quick Stats Grid -->
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon">⚡</div>
        <div class="stat-content">
          <div class="stat-title">Performance Score</div>
          <div class="stat-value {$performanceStatus.color}">
            {$performanceScore}/100
          </div>
          <div class="stat-desc">{$performanceStatus.level}</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">🖥️</div>
        <div class="stat-content">
          <div class="stat-title">CPU Usage</div>
          <div class="stat-value">
            {$performanceMetrics.cpu_usage.toFixed(1)}%
          </div>
          <div class="stat-desc">System Load</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">💾</div>
        <div class="stat-content">
          <div class="stat-title">Memory Usage</div>
          <div class="stat-value">
            {$performanceMetrics.memory_usage.toFixed(1)}%
          </div>
          <div class="stat-desc">RAM Utilization</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">🗄️</div>
        <div class="stat-content">
          <div class="stat-title">Cache Hit Ratio</div>
          <div class="stat-value">
            {($performanceMetrics.cache_hit_ratio * 100).toFixed(1)}%
          </div>
          <div class="stat-desc">Cache Efficiency</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">🔗</div>
        <div class="stat-content">
          <div class="stat-title">Active Connections</div>
          <div class="stat-value">
            {$performanceMetrics.active_connections}
          </div>
          <div class="stat-desc">WebSocket Clients</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">⏱️</div>
        <div class="stat-content">
          <div class="stat-title">Avg Response Time</div>
          <div class="stat-value">
            {$performanceMetrics.average_response_time.toFixed(1)}ms
          </div>
          <div class="stat-desc">API Performance</div>
        </div>
      </div>
    </div>

    <!-- System Information Panel -->
    <div class="system-info-panel">
      <h3>System Information</h3>
      <div class="system-grid">
        <div class="system-item">
          <span class="system-label">CPU Cores:</span>
          <span class="system-value">{$systemInfo.cpu_cores}</span>
        </div>
        <div class="system-item">
          <span class="system-label">Total Memory:</span>
          <span class="system-value">{$systemInfo.memory_total}</span>
        </div>
        <div class="system-item">
          <span class="system-label">Disk Space:</span>
          <span class="system-value">{$systemInfo.disk_space}</span>
        </div>
        <div class="system-item">
          <span class="system-label">Uptime:</span>
          <span class="system-value">{$systemInfo.uptime}</span>
        </div>
        <div class="system-item">
          <span class="system-label">Version:</span>
          <span class="system-value">{$systemInfo.version}</span>
        </div>
        <div class="system-item">
          <span class="system-label">Rust Version:</span>
          <span class="system-value">{$systemInfo.rust_version}</span>
        </div>
      </div>
    </div>

    <!-- Background Jobs Panel -->
    <div class="jobs-panel">
      <h3>Background Jobs</h3>
      <div class="jobs-summary">
        <div class="jobs-stat">
          <span class="jobs-label">Queue Length:</span>
          <span class="jobs-value">{$backgroundJobs.queue_length}</span>
        </div>
        <div class="jobs-stat">
          <span class="jobs-label">Active Workers:</span>
          <span class="jobs-value">{$backgroundJobs.active_workers}</span>
        </div>
      </div>

      {#if $backgroundJobs.jobs.length > 0}
        <div class="jobs-list">
          {#each $backgroundJobs.jobs as job}
            <div class="job-item">
              <div class="job-info">
                <span class="job-type">{job.job_type}</span>
                <span class="job-status {job.status}">{job.status}</span>
              </div>
              <div class="job-progress">
                {#if job.progress !== undefined}
                  <div class="progress-bar">
                    <div
                      class="progress-fill"
                      style="width: {job.progress}%"
                    ></div>
                  </div>
                  <span class="progress-text">{job.progress}%</span>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="no-jobs">
          <p>No background jobs in queue</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .performance-view {
    padding: 1.5rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .view-header {
    margin-bottom: 2rem;
  }

  .view-header h1 {
    font-size: 2rem;
    font-weight: 600;
    color: var(--md-sys-color-on-background);
    margin-bottom: 0.5rem;
  }

  .view-description {
    color: var(--md-sys-color-on-surface-variant);
    font-size: 1rem;
    margin: 0;
  }

  .performance-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .stat-card {
    background: var(--md-sys-color-surface);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 12px;
    padding: 1.5rem;
    display: flex;
    align-items: center;
    gap: 1rem;
    transition: all 0.2s ease;
  }

  .stat-card:hover {
    background: var(--md-sys-color-surface-container-low);
    border-color: var(--md-sys-color-outline);
  }

  .stat-icon {
    font-size: 2rem;
    opacity: 0.8;
  }

  .stat-content {
    flex: 1;
  }

  .stat-title {
    font-size: 0.875rem;
    color: var(--md-sys-color-on-surface-variant);
    margin-bottom: 0.25rem;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 0.25rem;
  }

  .stat-value.excellent {
    color: var(--md-sys-color-primary);
  }
  .stat-value.good {
    color: var(--md-sys-color-tertiary);
  }
  .stat-value.fair {
    color: var(--md-sys-color-error);
  }
  .stat-value.poor {
    color: var(--md-sys-color-error);
  }

  .stat-desc {
    font-size: 0.75rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .system-info-panel,
  .jobs-panel {
    background: var(--md-sys-color-surface);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 12px;
    padding: 1.5rem;
  }

  .system-info-panel h3,
  .jobs-panel h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 1rem;
  }

  .system-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 0.75rem;
  }

  .system-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .system-item:last-child {
    border-bottom: none;
  }

  .system-label {
    color: var(--md-sys-color-on-surface-variant);
    font-size: 0.875rem;
  }

  .system-value {
    color: var(--md-sys-color-on-surface);
    font-weight: 500;
  }

  .jobs-summary {
    display: flex;
    gap: 2rem;
    margin-bottom: 1rem;
  }

  .jobs-stat {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .jobs-label {
    font-size: 0.875rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .jobs-value {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .jobs-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .job-item {
    background: var(--md-sys-color-surface-container);
    border-radius: 8px;
    padding: 1rem;
  }

  .job-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .job-type {
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
  }

  .job-status {
    font-size: 0.875rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-weight: 500;
  }

  .job-status.pending {
    background: var(--md-sys-color-surface-variant);
    color: var(--md-sys-color-on-surface-variant);
  }

  .job-status.running {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .job-status.completed {
    background: var(--md-sys-color-tertiary-container);
    color: var(--md-sys-color-on-tertiary-container);
  }

  .job-status.failed {
    background: var(--md-sys-color-error-container);
    color: var(--md-sys-color-on-error-container);
  }

  .job-progress {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .progress-bar {
    flex: 1;
    height: 6px;
    background: var(--md-sys-color-surface-variant);
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--md-sys-color-primary);
    transition: width 0.3s ease;
  }

  .progress-text {
    font-size: 0.875rem;
    color: var(--md-sys-color-on-surface-variant);
    min-width: 3rem;
    text-align: right;
  }

  .no-jobs {
    text-align: center;
    padding: 2rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .no-jobs p {
    margin: 0;
    font-style: italic;
  }

  @media (max-width: 768px) {
    .performance-view {
      padding: 1rem;
    }

    .stats-grid {
      grid-template-columns: 1fr;
    }

    .system-grid {
      grid-template-columns: 1fr;
    }

    .jobs-summary {
      flex-direction: column;
      gap: 1rem;
    }
  }
</style>
