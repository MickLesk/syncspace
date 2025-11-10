<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    type?: "info" | "success" | "warning" | "danger";
    dismissible?: boolean;
    icon?: string;
    class?: string;
    children?: Snippet;
    ondismiss?: () => void;
  }

  let {
    type = "info",
    dismissible = false,
    icon,
    class: customClass = "",
    children,
    ondismiss,
  }: Props = $props();

  let visible = $state(true);

  const typeStyles = {
    info: {
      bg: "bg-blue-50 dark:bg-blue-900/20",
      border: "border-blue-200 dark:border-blue-800",
      text: "text-blue-800 dark:text-blue-300",
      icon: "bi-info-circle-fill",
    },
    success: {
      bg: "bg-green-50 dark:bg-green-900/20",
      border: "border-green-200 dark:border-green-800",
      text: "text-green-800 dark:text-green-300",
      icon: "bi-check-circle-fill",
    },
    warning: {
      bg: "bg-yellow-50 dark:bg-yellow-900/20",
      border: "border-yellow-200 dark:border-yellow-800",
      text: "text-yellow-800 dark:text-yellow-300",
      icon: "bi-exclamation-triangle-fill",
    },
    danger: {
      bg: "bg-red-50 dark:bg-red-900/20",
      border: "border-red-200 dark:border-red-800",
      text: "text-red-800 dark:text-red-300",
      icon: "bi-x-circle-fill",
    },
  };

  const currentStyle = typeStyles[type];
  const displayIcon = icon || currentStyle.icon;

  function handleDismiss() {
    visible = false;
    ondismiss?.();
  }
</script>

{#if visible}
  <div
    class={`
      flex items-start gap-3 p-4 rounded-lg border
      ${currentStyle.bg}
      ${currentStyle.border}
      ${currentStyle.text}
      ${customClass}
    `}
    role="alert"
  >
    {#if displayIcon}
      <i class={`bi ${displayIcon} text-lg flex-shrink-0 mt-0.5`}></i>
    {/if}

    <div class="flex-1 text-sm">
      {#if children}
        {@render children()}
      {/if}
    </div>

    {#if dismissible}
      <button
        onclick={handleDismiss}
        class="flex-shrink-0 ml-auto -mr-1 -mt-1 p-1.5 rounded-lg hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
        aria-label="Dismiss alert"
      >
        <i class="bi bi-x text-lg"></i>
      </button>
    {/if}
  </div>
{/if}
