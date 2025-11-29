<script>
  /**
   * Button Component v3 - SyncSpace Design System
   * 
   * Unified button with all variants, sizes, and states.
   * Supports icons, loading states, and dark mode.
   */
  import { cn } from "../../lib/design-system/utils.js";

  let {
    children = null,
    variant = "primary",
    size = "md",
    icon = "",
    iconRight = "",
    loading = false,
    disabled = false,
    fullWidth = false,
    type = "button",
    onclick = null,
    class: className = "",
    ...restProps
  } = $props();

  const baseClasses = `
    inline-flex items-center justify-center gap-2
    font-semibold rounded-xl
    transition-all duration-200
    focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2
    disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none
  `;

  const variantClasses = {
    primary: `
      bg-primary-500 hover:bg-primary-600 active:bg-primary-700
      text-white
      shadow-md hover:shadow-lg active:shadow-md
      focus-visible:ring-primary-500
      dark:shadow-primary-500/20
    `,
    secondary: `
      bg-gray-100 dark:bg-gray-700
      hover:bg-gray-200 dark:hover:bg-gray-600
      active:bg-gray-300 dark:active:bg-gray-500
      text-gray-900 dark:text-white
      border border-gray-200 dark:border-gray-600
      focus-visible:ring-gray-400
    `,
    ghost: `
      bg-transparent
      hover:bg-gray-100 dark:hover:bg-gray-800
      active:bg-gray-200 dark:active:bg-gray-700
      text-gray-700 dark:text-gray-300
      focus-visible:ring-gray-400
    `,
    danger: `
      bg-red-500 hover:bg-red-600 active:bg-red-700
      dark:bg-red-600 dark:hover:bg-red-700 dark:active:bg-red-800
      text-white
      shadow-md hover:shadow-lg
      focus-visible:ring-red-500
      dark:shadow-red-500/20
    `,
    success: `
      bg-green-500 hover:bg-green-600 active:bg-green-700
      dark:bg-green-600 dark:hover:bg-green-700 dark:active:bg-green-800
      text-white
      shadow-md hover:shadow-lg
      focus-visible:ring-green-500
      dark:shadow-green-500/20
    `,
    warning: `
      bg-amber-500 hover:bg-amber-600 active:bg-amber-700
      dark:bg-amber-600 dark:hover:bg-amber-700 dark:active:bg-amber-800
      text-white
      shadow-md hover:shadow-lg
      focus-visible:ring-amber-500
      dark:shadow-amber-500/20
    `,
    gradient: `
      gradient-bg-primary
      text-white
      shadow-lg hover:shadow-xl
      hover:scale-[1.02] active:scale-[0.98]
      focus-visible:ring-primary-500
    `,
    outline: `
      bg-transparent
      border-2 border-primary-500 dark:border-primary-400
      text-primary-600 dark:text-primary-400
      hover:bg-primary-50 dark:hover:bg-primary-900/20
      active:bg-primary-100 dark:active:bg-primary-900/30
      focus-visible:ring-primary-500
    `,
    glass: `
      glass-card
      text-gray-900 dark:text-white
      hover:bg-white/90 dark:hover:bg-gray-800/90
      focus-visible:ring-gray-400
    `,
  };

  const sizeClasses = {
    xs: "px-2 py-1 text-xs min-h-[28px]",
    sm: "px-3 py-1.5 text-sm min-h-[32px]",
    md: "px-4 py-2 text-sm min-h-[40px]",
    lg: "px-6 py-2.5 text-base min-h-[48px]",
    xl: "px-8 py-3 text-lg min-h-[56px]",
  };

  const iconSizes = {
    xs: "text-xs",
    sm: "text-sm",
    md: "text-base",
    lg: "text-lg",
    xl: "text-xl",
  };

  const computedClasses = $derived(cn(
    baseClasses,
    variantClasses[variant] || variantClasses.primary,
    sizeClasses[size] || sizeClasses.md,
    fullWidth && "w-full",
    className
  ));
</script>

<button
  {type}
  class={computedClasses}
  disabled={disabled || loading}
  {onclick}
  {...restProps}
>
  {#if loading}
    <svg
      class="animate-spin {iconSizes[size]}"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>
  {:else if icon}
    <i class="bi bi-{icon} {iconSizes[size]}"></i>
  {/if}

  {#if children}
    <span>
      {@render children()}
    </span>
  {/if}

  {#if iconRight && !loading}
    <i class="bi bi-{iconRight} {iconSizes[size]}"></i>
  {/if}
</button>
