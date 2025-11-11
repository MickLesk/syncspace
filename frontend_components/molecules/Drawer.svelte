<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "../atoms/index.ts";

  interface Props {
    open?: boolean;
    position?: "left" | "right" | "top" | "bottom";
    size?: "sm" | "md" | "lg" | "xl" | "full";
    title?: string;
    closable?: boolean;
    class?: string;
    onclose?: () => void;
    children?: Snippet;
  }

  let {
    open = $bindable(false),
    position = "right",
    size = "md",
    title = "",
    closable = true,
    class: customClass = "",
    onclose,
    children,
  }: Props = $props();

  const sizeClasses = {
    sm: "max-w-sm",
    md: "max-w-md",
    lg: "max-w-lg",
    xl: "max-w-xl",
    full: "max-w-full",
  };

  const positionClasses = {
    left: "top-0 left-0 h-full",
    right: "top-0 right-0 h-full",
    top: "top-0 left-0 w-full",
    bottom: "bottom-0 left-0 w-full",
  };

  const slideClasses = {
    left: open ? "translate-x-0" : "-translate-x-full",
    right: open ? "translate-x-0" : "translate-x-full",
    top: open ? "translate-y-0" : "-translate-y-full",
    bottom: open ? "translate-y-0" : "translate-y-full",
  };

  function handleClose() {
    open = false;
    onclose?.();
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget && closable) {
      handleClose();
    }
  }
</script>

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-40 transition-opacity"
    onclick={handleBackdropClick}
    role="presentation"
  ></div>

  <!-- Drawer -->
  <div
    class={`
      fixed z-50 bg-white dark:bg-gray-900 shadow-2xl
      transition-transform duration-300 ease-out
      ${positionClasses[position]}
      ${slideClasses[position]}
      ${position === "left" || position === "right" ? sizeClasses[size] + " w-full" : ""}
      ${customClass}
    `}
    role="dialog"
    aria-modal="true"
    aria-labelledby="drawer-title"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700"
    >
      {#if title}
        <h2
          id="drawer-title"
          class="text-xl font-semibold text-gray-900 dark:text-gray-100"
        >
          {title}
        </h2>
      {/if}

      {#if closable}
        <button
          onclick={handleClose}
          class="ml-auto p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          aria-label="Close drawer"
        >
          <i class="bi bi-x text-2xl text-gray-600 dark:text-gray-400"></i>
        </button>
      {/if}
    </div>

    <!-- Content -->
    <div class="p-6 overflow-y-auto" style="max-height: calc(100vh - 5rem)">
      {#if children}
        {@render children()}
      {/if}
    </div>
  </div>
{/if}
