<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    icon?: string;
    title: string;
    description?: string;
    actionLabel?: string;
    onaction?: () => void;
    variant?: "default" | "info" | "warning" | "glass";
    class?: string;
    children?: Snippet;
  }

  let {
    icon = "bi-inbox",
    title,
    description,
    actionLabel,
    onaction,
    variant = "default",
    class: customClass = "",
    children,
  }: Props = $props();

  const variantClasses = {
    default:
      "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700",
    info: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800",
    warning:
      "bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800",
    glass: "bg-white/10 backdrop-blur-md border border-white/20",
  };

  const iconVariantClasses = {
    default: "text-gray-400 dark:text-gray-600",
    info: "text-blue-400 dark:text-blue-500",
    warning: "text-amber-500 dark:text-amber-600",
    glass: "text-white/60",
  };
</script>

<div
  class={`
  flex flex-col items-center justify-center text-center
  rounded-xl p-12 transition-all duration-200
  ${variantClasses[variant]}
  ${customClass}
`}
>
  <!-- Icon -->
  <div class="mb-4 relative">
    <div
      class="absolute inset-0 bg-gradient-to-br from-blue-500/20 to-purple-500/20 rounded-full blur-2xl scale-150"
    ></div>
    <i class="bi {icon} text-6xl {iconVariantClasses[variant]} relative"></i>
  </div>

  <!-- Title -->
  <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
    {title}
  </h3>

  <!-- Description -->
  {#if description}
    <p class="text-gray-600 dark:text-gray-400 mb-6 max-w-md">
      {description}
    </p>
  {/if}

  <!-- Custom content -->
  {#if children}
    <div class="mb-4">
      {@render children()}
    </div>
  {/if}

  <!-- Action button -->
  {#if actionLabel && onaction}
    <button
      onclick={onaction}
      class="
        px-6 py-2.5 bg-blue-500 hover:bg-blue-600
        text-white font-medium rounded-lg
        transition-all duration-200 shadow-lg shadow-blue-500/30
        hover:shadow-xl hover:scale-105 active:scale-95
      "
    >
      {actionLabel}
    </button>
  {/if}
</div>
