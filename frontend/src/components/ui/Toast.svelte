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
      class="min-w-[360px] max-w-[500px] max-sm:min-w-0 max-sm:max-w-[calc(100vw-2rem)] bg-white dark:bg-gray-800 border border-gray-900/10 dark:border-white/10 rounded-xl shadow-xl overflow-hidden animate-slide-in pointer-events-auto backdrop-blur-sm"
      role="alert"
    >
      <div class="flex items-start gap-3.5 px-5 py-4">
        <!-- Icon -->
        <div
          class="shrink-0 flex items-center justify-center w-10 h-10 rounded-[10px] {toast.type ===
          'success'
            ? 'bg-green-500/15 text-green-500'
            : toast.type === 'error'
              ? 'bg-red-500/15 text-red-500'
              : toast.type === 'warning'
                ? 'bg-amber-500/15 text-amber-500'
                : 'bg-blue-500/15 text-blue-500'}"
        >
          <i
            class="bi bi-{getIcon(toast.type)} text-xl {toast.type === 'success'
              ? 'animate-success-check'
              : toast.type === 'error'
                ? 'animate-error-shake'
                : ''}"
          ></i>
        </div>

        <!-- Text -->
        <div class="flex-1 min-w-0 pt-1">
          {#if toast.title}
            <p
              class="text-[15px] font-bold text-gray-900 dark:text-gray-50 m-0 mb-1"
            >
              {toast.title}
            </p>
          {/if}
          <p
            class="text-sm text-gray-900/80 dark:text-gray-50/80 m-0 leading-snug"
          >
            {toast.message}
          </p>
        </div>

        <!-- Close button -->
        <button
          class="shrink-0 flex items-center justify-center w-8 h-8 border-none bg-gray-900/5 dark:bg-gray-50/5 text-gray-900/60 dark:text-gray-50/60 rounded-md cursor-pointer transition-all duration-200 hover:bg-gray-900/10 dark:hover:bg-gray-50/10 hover:text-gray-900 dark:hover:text-gray-50"
          onclick={() => removeToast(toast.id)}
          aria-label="Close"
        >
          <i class="bi bi-x-lg" aria-hidden="true"></i>
        </button>
      </div>

      <!-- Progress bar -->
      <div class="h-[3px] bg-gray-900/5 dark:bg-gray-50/5 overflow-hidden">
        <div
          id="toast-progress-{toast.id}"
          class="h-full transition-[width] duration-[50ms] ease-linear {toast.type ===
          'success'
            ? 'bg-green-500'
            : toast.type === 'error'
              ? 'bg-red-500'
              : toast.type === 'warning'
                ? 'bg-amber-500'
                : 'bg-blue-500'}"
          style="width: 100%"
        ></div>
      </div>
    </div>
  {/each}
</div>

<style>
  @keyframes slide-in {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  .animate-slide-in {
    animation: slide-in 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes success-check {
    0%,
    100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.2);
    }
  }

  .animate-success-check {
    animation: success-check 0.5s ease-out;
  }

  @keyframes error-shake {
    0%,
    100% {
      transform: translateX(0);
    }
    20%,
    60% {
      transform: translateX(-3px);
    }
    40%,
    80% {
      transform: translateX(3px);
    }
  }

  .animate-error-shake {
    animation: error-shake 0.5s ease-out;
  }
</style>
