<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    value?: number;
    max?: number;
    variant?: "primary" | "success" | "danger" | "warning";
    size?: "sm" | "md" | "lg";
    showLabel?: boolean;
    striped?: boolean;
    animated?: boolean;
    class?: string;
    children?: Snippet;
  }

  let {
    value = 0,
    max = 100,
    variant = "primary",
    size = "md",
    showLabel = false,
    striped = false,
    animated = false,
    class: customClass = "",
    children,
  }: Props = $props();

  const percentage = $derived(Math.min(Math.max((value / max) * 100, 0), 100));

  const sizeClasses = {
    sm: "h-1",
    md: "h-2.5",
    lg: "h-4",
  };

  const variantClasses = {
    primary: "bg-blue-500",
    success: "bg-green-500",
    danger: "bg-red-500",
    warning: "bg-yellow-500",
  };
</script>

<div class={`w-full ${customClass}`}>
  {#if showLabel || children}
    <div class="flex justify-between mb-1">
      {#if children}
        <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
          {@render children()}
        </span>
      {/if}
      {#if showLabel}
        <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
          {percentage.toFixed(0)}%
        </span>
      {/if}
    </div>
  {/if}

  <div
    class={`
      w-full bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden
      ${sizeClasses[size]}
    `}
  >
    <div
      class={`
        h-full transition-all duration-300 ease-out
        ${variantClasses[variant]}
        ${striped ? "bg-stripes" : ""}
        ${animated && striped ? "bg-stripes-animated" : ""}
      `}
      style={`width: ${percentage}%`}
      role="progressbar"
      aria-valuenow={value}
      aria-valuemin="0"
      aria-valuemax={max}
    />
  </div>
</div>

<style>
  .bg-stripes {
    background-image: linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.15) 25%,
      transparent 25%,
      transparent 50%,
      rgba(255, 255, 255, 0.15) 50%,
      rgba(255, 255, 255, 0.15) 75%,
      transparent 75%,
      transparent
    );
    background-size: 1rem 1rem;
  }

  .bg-stripes-animated {
    animation: progress-stripes 1s linear infinite;
  }

  @keyframes progress-stripes {
    0% {
      background-position: 1rem 0;
    }
    100% {
      background-position: 0 0;
    }
  }
</style>
