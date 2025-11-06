<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { toasts, removeToast } from "../../stores/toast.js";
  import { onMount } from "svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let progressIntervals = new Map();

  function getIcon(type) {
    switch (type) {
      case "success":
        return "check-circle-fill";
      case "error":
        return "x-circle-fill";
      case "warning":
        return "exclamation-triangle-fill";
      case "info":
        return "info-circle-fill";
      default:
        return "info-circle-fill";
    }
  }

  function getAlertClass(type) {
    switch (type) {
      case "success":
        return "alert-success";
      case "error":
        return "alert-error";
      case "warning":
        return "alert-warning";
      case "info":
        return "alert-info";
      default:
        return "alert-info";
    }
  }

  // Start progress bar animation
  function startProgress(toastId, duration) {
    const interval = setInterval(() => {
      const progressEl = document.getElementById(`toast-progress-${toastId}`);
      if (progressEl) {
        const currentWidth = parseFloat(progressEl.style.width || "100");
        const newWidth = currentWidth - 100 / (duration / 50);
        if (newWidth <= 0) {
          clearInterval(interval);
          progressIntervals.delete(toastId);
        } else {
          progressEl.style.width = `${newWidth}%`;
        }
      }
    }, 50);
    progressIntervals.set(toastId, interval);
  }

  // Auto-dismiss toasts
  $effect(() => {
    $toasts.forEach((toast) => {
      if (!toast.dismissed && !progressIntervals.has(toast.id)) {
        const duration = toast.duration || 5000;
        startProgress(toast.id, duration);

        setTimeout(() => {
          removeToast(toast.id);
        }, duration);
      }
    });
  });

  onMount(() => {
    return () => {
      // Cleanup intervals
      progressIntervals.forEach((interval) => clearInterval(interval));
      progressIntervals.clear();
    };
  });
</script>

<div
  class="fixed bottom-4 right-4 z-[9999] flex flex-col gap-3 pointer-events-none"
>
  {#each $toasts as toast (toast.id)}
    <div
      class="toast-item {getAlertClass(
        toast.type
      )} pointer-events-auto toast-slide-in"
      role="alert"
    >
      <div class="toast-content">
        <div class="toast-icon-wrapper">
          <i
            class="bi bi-{getIcon(toast.type)} toast-icon {toast.type ===
            'success'
              ? 'success-checkmark'
              : toast.type === 'error'
                ? 'error-shake'
                : ''}"
          ></i>
        </div>

        <div class="toast-text-wrapper">
          {#if toast.title}
            <p class="toast-title">{toast.title}</p>
          {/if}
          <p class="toast-message">{toast.message}</p>
        </div>

        <button
          class="toast-close-btn"
          onclick={() => removeToast(toast.id)}
          aria-label="Close"
        >
          <i class="bi bi-x-lg"></i>
        </button>
      </div>

      <!-- Progress bar -->
      <div class="toast-progress-bg">
        <div
          id="toast-progress-{toast.id}"
          class="toast-progress-bar {toast.type}"
          style="width: 100%"
        ></div>
      </div>
    </div>
  {/each}
</div>

<style>
  .toast-item {
    min-width: 360px;
    max-width: 500px;
    background: white;
    border: 1px solid rgba(17, 24, 39, 0.1);
    border-radius: 12px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.2);
    overflow: hidden;
    animation: slideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    backdrop-filter: blur(8px);
  }

  :global(.dark) .toast-item {
    background: #1f2937;
    border-color: rgba(255, 255, 255, 0.1);
  }

  .toast-content {
    display: flex;
    align-items: start;
    gap: 0.875rem;
    padding: 1rem 1.25rem;
  }

  .toast-icon-wrapper {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 10px;
  }

  .toast-icon {
    font-size: 1.25rem;
  }

  .alert-success .toast-icon-wrapper {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
  }

  .alert-error .toast-icon-wrapper {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }

  .alert-warning .toast-icon-wrapper {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
  }

  .alert-info .toast-icon-wrapper {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }

  .toast-text-wrapper {
    flex: 1;
    min-width: 0;
    padding-top: 0.25rem;
  }

  .toast-title {
    font-size: 0.9375rem;
    font-weight: 700;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }

  :global(.dark) .toast-title {
    color: #f9fafb;
  }

  .toast-message {
    font-size: 0.875rem;
    color: rgba(17, 24, 39, 0.8);
    margin: 0;
    line-height: 1.4;
  }

  :global(.dark) .toast-message {
    color: rgba(249, 250, 251, 0.8);
  }

  .toast-close-btn {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    background: rgba(17, 24, 39, 0.05);
    color: rgba(17, 24, 39, 0.6);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  :global(.dark) .toast-close-btn {
    background: rgba(249, 250, 251, 0.05);
    color: rgba(249, 250, 251, 0.6);
  }

  .toast-close-btn:hover {
    background: rgba(17, 24, 39, 0.1);
    color: #111827;
  }

  :global(.dark) .toast-close-btn:hover {
    background: rgba(249, 250, 251, 0.1);
    color: #f9fafb;
  }

  /* Progress Bar */
  .toast-progress-bg {
    height: 3px;
    background: rgba(17, 24, 39, 0.05);
    overflow: hidden;
  }

  :global(.dark) .toast-progress-bg {
    background: rgba(249, 250, 251, 0.05);
  }

  .toast-progress-bar {
    height: 100%;
    transition: width 50ms linear;
  }

  .toast-progress-bar.success {
    background: #10b981;
  }

  .toast-progress-bar.error {
    background: #ef4444;
  }

  .toast-progress-bar.warning {
    background: #f59e0b;
  }

  .toast-progress-bar.info {
    background: #3b82f6;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @media (max-width: 640px) {
    .toast-item {
      min-width: unset;
      max-width: calc(100vw - 2rem);
    }
  }
</style>
