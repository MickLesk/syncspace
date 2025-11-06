<script>
  /**
   * Performance Demo - Test page for performance monitoring components
   * Created: October 25, 2025
   */
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import PerformanceMonitor from "../../components/tools/PerformanceMonitor.svelte";
  import {
    performanceMonitor,
    performanceMetrics,
    cacheStats,
    backgroundJobs,
  } from "../../stores/performance.js";

  let testResults = [];

  async function testPerformanceAPI() {
    testResults = [];
    addResult("🔄 Testing Performance API endpoints...");

    try {
      // Test metrics endpoint
      addResult("Testing GET /api/performance/metrics...");
      await performanceMonitor.loadMetrics();
      addResult(`✅ Metrics loaded successfully`);
    } catch (error) {
      addResult(`❌ Metrics failed: ${error.message}`);
    }

    try {
      // Test cache stats
      addResult("Testing GET /api/performance/cache/stats...");
      await performanceMonitor.loadCacheStats();
      addResult(`✅ Cache stats loaded successfully`);
    } catch (error) {
      addResult(`❌ Cache stats failed: ${error.message}`);
    }

    try {
      // Test system info
      addResult("Testing GET /api/performance/system/info...");
      await performanceMonitor.loadSystemInfo();
      addResult(`✅ System info loaded successfully`);
    } catch (error) {
      addResult(`❌ System info failed: ${error.message}`);
    }

    try {
      // Test background jobs
      addResult("Testing GET /api/performance/jobs...");
      await performanceMonitor.loadBackgroundJobs();
      addResult(`✅ Background jobs loaded successfully`);
    } catch (error) {
      addResult(`❌ Background jobs failed: ${error.message}`);
    }
  }

  async function testCacheClear() {
    try {
      addResult("🗑️ Testing cache clear...");
      await performanceMonitor.clearCache();
      addResult("✅ Cache cleared successfully");
    } catch (error) {
      addResult(`❌ Cache clear failed: ${error.message}`);
    }
  }

  async function testJobQueue() {
    try {
      addResult("⚙️ Testing job queue...");
      const jobId = await performanceMonitor.queueJob(
        "test_job",
        { test: true },
        1
      );
      addResult(`✅ Job queued: ${jobId}`);
    } catch (error) {
      addResult(`❌ Job queue failed: ${error.message}`);
    }
  }

  function addResult(message) {
    testResults = [
      ...testResults,
      {
        timestamp: new Date().toLocaleTimeString(),
        message,
      },
    ];
  }

  function clearResults() {
    testResults = [];
  }
</script>

<div class="performance-demo">
  <div class="demo-header">
    <h1>Performance Demo</h1>
    <p class="demo-description">
      Test page for performance monitoring components and API endpoints
    </p>
  </div>

  <div class="demo-content">
    <!-- Performance Monitor Component Demo -->
    <div class="demo-section">
      <h2>Performance Monitor Component</h2>
      <div class="monitor-container">
        <PerformanceMonitor compact={false} />
      </div>
    </div>

    <!-- Compact Monitor Demo -->
    <div class="demo-section">
      <h2>Compact Performance Monitor</h2>
      <div class="compact-container">
        <PerformanceMonitor compact={true} />
      </div>
    </div>

    <!-- API Testing Section -->
    <div class="demo-section">
      <h2>API Endpoint Testing</h2>
      <div class="test-controls">
        <button
          class="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-xl hover:from-blue-700 hover:to-purple-700 transition-all flex items-center gap-2 shadow-lg shadow-blue-500/25"
          onclick={testPerformanceAPI}
        >
          🧪 Test All Endpoints
        </button>
        <button
          class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-xl transition-all flex items-center gap-2"
          onclick={testCacheClear}
        >
          🗑️ Test Cache Clear
        </button>
        <button
          class="px-4 py-2 bg-pink-600 hover:bg-pink-700 text-white rounded-xl transition-all flex items-center gap-2"
          onclick={testJobQueue}
        >
          ⚙️ Test Job Queue
        </button>
        <button
          class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-xl transition-all flex items-center gap-2"
          onclick={clearResults}
        >
          🧹 Clear Results
        </button>
      </div>

      <div class="test-results">
        <h3>Test Results</h3>
        <div class="results-container">
          {#if testResults.length === 0}
            <p class="no-results">
              No test results yet. Click a test button above.
            </p>
          {:else}
            {#each testResults as result}
              <div class="result-item">
                <span class="result-timestamp">{result.timestamp}</span>
                <span class="result-message">{result.message}</span>
              </div>
            {/each}
          {/if}
        </div>
      </div>
    </div>

    <!-- Current Store Values -->
    <div class="demo-section">
      <h2>Current Store Values</h2>
      <div class="store-values">
        <div class="store-group">
          <h3>Performance Metrics</h3>
          <pre>{JSON.stringify($performanceMetrics, null, 2)}</pre>
        </div>

        <div class="store-group">
          <h3>Cache Stats</h3>
          <pre>{JSON.stringify($cacheStats, null, 2)}</pre>
        </div>

        <div class="store-group">
          <h3>Background Jobs</h3>
          <pre>{JSON.stringify($backgroundJobs, null, 2)}</pre>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .performance-demo {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .demo-header {
    margin-bottom: 2rem;
    text-align: center;
  }

  .demo-header h1 {
    font-size: 2.5rem;
    font-weight: 700;
    color: var(--md-sys-color-on-background);
    margin-bottom: 0.5rem;
  }

  .demo-description {
    font-size: 1.125rem;
    color: var(--md-sys-color-on-surface-variant);
    margin: 0;
  }

  .demo-content {
    display: flex;
    flex-direction: column;
    gap: 3rem;
  }

  .demo-section {
    background: var(--md-sys-color-surface);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 16px;
    padding: 2rem;
  }

  .demo-section h2 {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 1.5rem;
  }

  .demo-section h3 {
    font-size: 1.25rem;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 1rem;
  }

  .monitor-container {
    background: var(--md-sys-color-surface-container);
    border-radius: 12px;
    padding: 1rem;
  }

  .compact-container {
    background: var(--md-sys-color-surface-container);
    border-radius: 8px;
    padding: 1rem;
    max-width: 400px;
  }

  .test-controls {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .test-results {
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
    padding: 1.5rem;
  }

  .results-container {
    max-height: 400px;
    overflow-y: auto;
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 8px;
    padding: 1rem;
    background: var(--md-sys-color-surface);
  }

  .no-results {
    text-align: center;
    color: var(--md-sys-color-on-surface-variant);
    font-style: italic;
    margin: 2rem 0;
  }

  .result-item {
    display: flex;
    gap: 1rem;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
    font-family: "JetBrains Mono", "Courier New", monospace;
    font-size: 0.875rem;
  }

  .result-item:last-child {
    border-bottom: none;
  }

  .result-timestamp {
    color: var(--md-sys-color-on-surface-variant);
    min-width: 5rem;
    flex-shrink: 0;
  }

  .result-message {
    color: var(--md-sys-color-on-surface);
    flex: 1;
  }

  .store-values {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .store-group {
    background: var(--md-sys-color-surface-container-low);
    border-radius: 12px;
    padding: 1.5rem;
  }

  .store-group pre {
    background: var(--md-sys-color-surface);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 8px;
    padding: 1rem;
    font-family: "JetBrains Mono", "Courier New", monospace;
    font-size: 0.75rem;
    color: var(--md-sys-color-on-surface);
    overflow-x: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
    max-height: 300px;
    overflow-y: auto;
  }

  @media (max-width: 768px) {
    .performance-demo {
      padding: 1rem;
    }

    .demo-header h1 {
      font-size: 2rem;
    }

    .test-controls {
      flex-direction: column;
    }

    .store-values {
      grid-template-columns: 1fr;
    }
  }
</style>
