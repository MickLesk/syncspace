<script lang="ts">
  import { radius, shadows, transitions } from "../shared/index.js";
  import type { Snippet } from "svelte";

  interface Props {
    open?: boolean;
    title?: string;
    size?: "sm" | "md" | "lg" | "xl";
    closeButton?: boolean;
    glass?: boolean; // Glassmorphism effect
    children?: Snippet;
    footer?: Snippet;
  }

  let {
    open = false,
    title,
    size = "md",
    closeButton = true,
    glass = false,
    children,
    footer,
  }: Props = $props();

  const sizeClasses = {
    sm: "max-w-sm",
    md: "max-w-md",
    lg: "max-w-lg",
    xl: "max-w-2xl",
  };

  function close() {
    open = false;
  }

  // 2025 Design: Glassmorphism for modern modals
  const modalClasses = glass
    ? "bg-white/80 dark:bg-gray-900/80 backdrop-blur-2xl border border-white/20 dark:border-gray-700/20"
    : "bg-white dark:bg-gray-800";
</script>

{#if open}
  <!-- Backdrop with blur -->
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm z-40"
    style="animation: fadeIn 0.3s ease-out;"
    onclick={close}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Escape" && close()}
    aria-label="Close modal backdrop"
  ></div>

  <!-- Modal -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    style="animation: modalSlideIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);"
    onclick={(e) => e.stopPropagation()}
    role="presentation"
  >
    <div
      class={`
        ${modalClasses}
        ${radius.xl}
        ${shadows.xl}
        w-full ${sizeClasses[size]}
        ${transitions.smooth}
      `}
      role="dialog"
      aria-modal="true"
    >
      <!-- Header -->
      {#if title || closeButton}
        <div
          class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700"
        >
          {#if title}
            <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
              {title}
            </h2>
          {/if}
          {#if closeButton}
            <button
              onclick={close}
              class={`
                text-gray-500 dark:text-gray-400
                hover:text-gray-700 dark:hover:text-gray-200
                hover:bg-gray-100 dark:hover:bg-gray-700
                focus:outline-none focus:ring-2 focus:ring-blue-500
                ${radius.lg} p-2
                ${transitions.smooth}
                hover:scale-110
              `}
              aria-label="Close modal"
            >
              <i class="bi bi-x text-xl"></i>
            </button>
          {/if}
        </div>
      {/if}

      <!-- Content -->
      <div class="px-6 py-4">
        {#if children}
          {@render children()}
        {/if}
      </div>

      <!-- Footer -->
      {#if footer}
        <div
          class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-2"
        >
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  /* 2025 Design: Smooth fade-in for backdrop */
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  /* Spring slide-in animation for modal */
  @keyframes modalSlideIn {
    0% {
      opacity: 0;
      transform: translateY(-2rem) scale(0.95);
    }
    60% {
      opacity: 1;
      transform: translateY(0.5rem) scale(1.02);
    }
    100% {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Prevent body scroll when modal is open */
  :global(body:has(div[role="dialog"])) {
    overflow: hidden;
  }
</style>
