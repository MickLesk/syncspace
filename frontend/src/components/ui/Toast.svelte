<script>
  import { toasts, removeToast } from "../../stores/toast.js";
  import { onMount } from "svelte";

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
  $: {
    $toasts.forEach((toast) => {
      if (!toast.dismissed && !progressIntervals.has(toast.id)) {
        const duration = toast.duration || 5000;
        startProgress(toast.id, duration);

        setTimeout(() => {
          removeToast(toast.id);
        }, duration);
      }
    });
  }

  onMount(() => {
    return () => {
      // Cleanup intervals
      progressIntervals.forEach((interval) => clearInterval(interval));
      progressIntervals.clear();
    };
  });
</script>

<div class="toast toast-end toast-bottom z-[9999]">
  {#each $toasts as toast (toast.id)}
    <div class="toast-item {getAlertClass(toast.type)}" role="alert">
      <div class="toast-content">
        <div class="toast-icon-wrapper">
          <i class="bi bi-{getIcon(toast.type)} toast-icon"></i>
        </div>

        <div class="toast-text-wrapper">
          {#if toast.title}
            <p class="toast-title">{toast.title}</p>
          {/if}
          <p class="toast-message">{toast.message}</p>
        </div>

        <button
          class="toast-close-btn"
          on:click={() => removeToast(toast.id)}
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
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: 12px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.2);
    overflow: hidden;
    animation: slideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    backdrop-filter: blur(8px);
    margin-bottom: 0.75rem;
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
    background: hsl(var(--su) / 0.15);
    color: hsl(var(--su));
  }

  .alert-error .toast-icon-wrapper {
    background: hsl(var(--er) / 0.15);
    color: hsl(var(--er));
  }

  .alert-warning .toast-icon-wrapper {
    background: hsl(var(--wa) / 0.15);
    color: hsl(var(--wa));
  }

  .alert-info .toast-icon-wrapper {
    background: hsl(var(--in) / 0.15);
    color: hsl(var(--in));
  }

  .toast-text-wrapper {
    flex: 1;
    min-width: 0;
    padding-top: 0.25rem;
  }

  .toast-title {
    font-size: 0.9375rem;
    font-weight: 700;
    color: hsl(var(--bc));
    margin: 0 0 0.25rem 0;
  }

  .toast-message {
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.8);
    margin: 0;
    line-height: 1.4;
  }

  .toast-close-btn {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    background: hsl(var(--bc) / 0.05);
    color: hsl(var(--bc) / 0.6);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .toast-close-btn:hover {
    background: hsl(var(--bc) / 0.1);
    color: hsl(var(--bc));
  }

  /* Progress Bar */
  .toast-progress-bg {
    height: 3px;
    background: hsl(var(--bc) / 0.05);
    overflow: hidden;
  }

  .toast-progress-bar {
    height: 100%;
    transition: width 50ms linear;
  }

  .toast-progress-bar.success {
    background: hsl(var(--su));
  }

  .toast-progress-bar.error {
    background: hsl(var(--er));
  }

  .toast-progress-bar.warning {
    background: hsl(var(--wa));
  }

  .toast-progress-bar.info {
    background: hsl(var(--in));
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
