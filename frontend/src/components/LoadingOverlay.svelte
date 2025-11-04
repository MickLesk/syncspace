<script>
  import { loading } from "../stores/ui.js";

  // Subscribe to loading state
  $: isLoading = $loading.isLoading;
  $: message = $loading.message;
  $: progress = $loading.progress;

  // Calculate progress percentage
  $: progressPercent = progress
    ? Math.round((progress.current / progress.total) * 100)
    : 0;

  // DEBUG: Log when loading state changes
  $: {
    console.log("[LoadingOverlay] isLoading:", isLoading, "message:", message);
  }
</script>

{#if isLoading}
  <div class="loading-overlay" role="status" aria-live="polite">
    <div class="loading-container">
      <!-- Spinner -->
      <div class="spinner">
        <svg class="spinner-svg" viewBox="0 0 50 50">
          <circle
            class="spinner-circle"
            cx="25"
            cy="25"
            r="20"
            fill="none"
            stroke-width="4"
          />
        </svg>
      </div>

      <!-- Loading Message -->
      <p class="loading-message">{message}</p>

      <!-- Progress Bar (if available) -->
      {#if progress}
        <div class="progress-container">
          <div class="progress-bar">
            <div class="progress-fill" style="width: {progressPercent}%"></div>
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
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.6);
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
    background: var(--surface-container-high, #ffffff);
    padding: 2rem;
    border-radius: 16px;
    box-shadow: var(--elevation-3, 0 4px 12px rgba(0, 0, 0, 0.15));
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    min-width: 280px;
    max-width: 400px;
  }

  /* Dark mode */
  :global(.dark) .loading-container {
    background: var(--surface-container-high-dark, #2a2a2a);
  }

  /* Spinner */
  .spinner {
    width: 64px;
    height: 64px;
  }

  .spinner-svg {
    animation: rotate 2s linear infinite;
    width: 100%;
    height: 100%;
  }

  .spinner-circle {
    stroke: var(--primary, #6200ea);
    stroke-linecap: round;
    animation: dash 1.5s ease-in-out infinite;
  }

  :global(.dark) .spinner-circle {
    stroke: var(--primary-dark, #bb86fc);
  }

  @keyframes rotate {
    100% {
      transform: rotate(360deg);
    }
  }

  @keyframes dash {
    0% {
      stroke-dasharray: 1, 150;
      stroke-dashoffset: 0;
    }
    50% {
      stroke-dasharray: 90, 150;
      stroke-dashoffset: -35;
    }
    100% {
      stroke-dasharray: 90, 150;
      stroke-dashoffset: -124;
    }
  }

  /* Loading Message */
  .loading-message {
    font-size: 1rem;
    font-weight: 500;
    color: var(--on-surface, #1c1b1f);
    margin: 0;
    text-align: center;
  }

  :global(.dark) .loading-message {
    color: var(--on-surface-dark, #e6e1e5);
  }

  /* Progress Bar */
  .progress-container {
    width: 100%;
    margin-top: 0.5rem;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: var(--surface-variant, #e7e0ec);
    border-radius: 4px;
    overflow: hidden;
  }

  :global(.dark) .progress-bar {
    background: var(--surface-variant-dark, #49454f);
  }

  .progress-fill {
    height: 100%;
    background: var(--primary, #6200ea);
    border-radius: 4px;
    transition: width 0.3s ease;
  }

  :global(.dark) .progress-fill {
    background: var(--primary-dark, #bb86fc);
  }

  .progress-text {
    font-size: 0.875rem;
    color: var(--on-surface-variant, #49454f);
    margin: 0.5rem 0 0 0;
    text-align: center;
  }

  :global(.dark) .progress-text {
    color: var(--on-surface-variant-dark, #cac4d0);
  }
</style>
