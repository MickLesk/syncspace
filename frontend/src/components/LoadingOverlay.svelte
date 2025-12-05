<script>
  import { loading } from "../stores/ui.js";

  // Subscribe to loading state (Svelte 5)
  let isLoading = $derived($loading.isLoading);
  let message = $derived($loading.message);
  let progress = $derived($loading.progress);

  // Calculate progress percentage
  let progressPercent = $derived(
    progress ? Math.round((progress.current / progress.total) * 100) : 0
  );
</script>

{#if isLoading}
  <div class="loading-overlay" role="status" aria-live="polite">
    <div class="loading-container">
      <!-- Spinner -->
      <div class="spinner"></div>

      <!-- Loading Message -->
      <p class="loading-message">{message}</p>

      <!-- Progress Bar (if available) -->
      {#if progress}
        <div class="progress-section">
          <div class="progress-container">
            <div class="progress-bar" style="width: {progressPercent}%"></div>
          </div>
          <p class="progress-text">
            {progress.current} / {progress.total} ({progressPercent}%)
          </p>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .loading-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .loading-container {
    background: white;
    padding: 2rem;
    border-radius: 0.75rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    min-width: 280px;
    max-width: 400px;
  }

  :global(.dark) .loading-container {
    background: #1f2937;
  }

  /* Spinner - Styleguide compliant */
  .spinner {
    width: 48px;
    height: 48px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  :global(.dark) .spinner {
    border-color: #374151;
    border-top-color: #22c55e;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Loading Message */
  .loading-message {
    font-size: 0.9375rem;
    font-weight: 500;
    color: #374151;
    margin: 0;
    text-align: center;
  }

  :global(.dark) .loading-message {
    color: #d1d5db;
  }

  /* Progress Section */
  .progress-section {
    width: 100%;
    margin-top: 0.5rem;
  }

  .progress-container {
    height: 8px;
    background: #e5e7eb;
    border-radius: 4px;
    overflow: hidden;
  }

  :global(.dark) .progress-container {
    background: #374151;
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #22c55e, #16a34a);
    border-radius: 4px;
    transition: width 0.3s ease;
  }

  .progress-text {
    font-size: 0.8125rem;
    color: #6b7280;
    margin: 0.5rem 0 0 0;
    text-align: center;
  }

  :global(.dark) .progress-text {
    color: #9ca3af;
  }
</style>
