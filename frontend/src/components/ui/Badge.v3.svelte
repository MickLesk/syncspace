<script>
  /**
   * Badge Component v3 - SyncSpace Design System
   * 
   * Unified badge/tag component with multiple variants and sizes.
   * Supports icons, removable state, and dark mode.
   */
  import { cn } from "../../lib/design-system/utils.js";

  let {
    children = null,
    text = "",
    variant = "default",
    size = "md",
    icon = "",
    iconRight = "",
    removable = false,
    dot = false,
    pill = true,
    onremove = null,
    onclick = null,
    class: className = "",
  } = $props();

  const baseClasses = `
    inline-flex items-center gap-1.5
    font-semibold
    transition-all duration-150
  `;

  const shapeClasses = pill ? "rounded-full" : "rounded-lg";

  const variantClasses = {
    default: "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300",
    primary: "bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300",
    secondary: "bg-gray-200 dark:bg-gray-600 text-gray-800 dark:text-gray-200",
    success: "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300",
    warning: "bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300",
    error: "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300",
    info: "bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300",
    // Solid variants
    solidPrimary: "bg-primary-500 text-white",
    solidSuccess: "bg-green-500 text-white",
    solidWarning: "bg-amber-500 text-white",
    solidError: "bg-red-500 text-white",
    solidInfo: "bg-blue-500 text-white",
    // Outline variants
    outlinePrimary: "bg-transparent border border-primary-500 text-primary-600 dark:text-primary-400",
    outlineSuccess: "bg-transparent border border-green-500 text-green-600 dark:text-green-400",
    outlineWarning: "bg-transparent border border-amber-500 text-amber-600 dark:text-amber-400",
    outlineError: "bg-transparent border border-red-500 text-red-600 dark:text-red-400",
    // Glass variants
    glass: "bg-white/20 dark:bg-gray-800/40 backdrop-blur-sm border border-white/30 dark:border-gray-700/50 text-gray-800 dark:text-gray-200",
    glassPrimary: "bg-primary-500/15 dark:bg-primary-400/20 backdrop-blur-sm text-primary-700 dark:text-primary-300 border border-primary-500/20",
    glassSuccess: "bg-green-500/15 dark:bg-green-400/20 backdrop-blur-sm text-green-700 dark:text-green-300 border border-green-500/20",
    glassWarning: "bg-amber-500/15 dark:bg-amber-400/20 backdrop-blur-sm text-amber-700 dark:text-amber-300 border border-amber-500/20",
    glassError: "bg-red-500/15 dark:bg-red-400/20 backdrop-blur-sm text-red-700 dark:text-red-300 border border-red-500/20",
    glassInfo: "bg-blue-500/15 dark:bg-blue-400/20 backdrop-blur-sm text-blue-700 dark:text-blue-300 border border-blue-500/20",
  };

  const sizeClasses = {
    xs: "px-1.5 py-0.5 text-[10px]",
    sm: "px-2 py-0.5 text-xs",
    md: "px-2.5 py-1 text-xs",
    lg: "px-3 py-1.5 text-sm",
    xl: "px-4 py-2 text-sm",
  };

  const iconSizeClasses = {
    xs: "text-[10px]",
    sm: "text-xs",
    md: "text-xs",
    lg: "text-sm",
    xl: "text-base",
  };

  const dotSizeClasses = {
    xs: "w-1.5 h-1.5",
    sm: "w-2 h-2",
    md: "w-2 h-2",
    lg: "w-2.5 h-2.5",
    xl: "w-3 h-3",
  };

  const clickableClasses = onclick ? "cursor-pointer hover:opacity-80 active:scale-95" : "";

  const computedClasses = $derived(cn(
    baseClasses,
    shapeClasses,
    variantClasses[variant] || variantClasses.default,
    sizeClasses[size] || sizeClasses.md,
    clickableClasses,
    className
  ));
</script>

<span
  class={computedClasses}
  role={onclick ? "button" : undefined}
  tabindex={onclick ? 0 : undefined}
  {onclick}
  onkeydown={(e) => {
    if (onclick && (e.key === "Enter" || e.key === " ")) {
      e.preventDefault();
      onclick(e);
    }
  }}
>
  {#if dot}
    <span class="rounded-full bg-current opacity-80 {dotSizeClasses[size]}"></span>
  {/if}

  {#if icon}
    <i class="bi bi-{icon} {iconSizeClasses[size]}"></i>
  {/if}

  {#if children}
    {@render children()}
  {:else if text}
    {text}
  {/if}

  {#if iconRight}
    <i class="bi bi-{iconRight} {iconSizeClasses[size]}"></i>
  {/if}

  {#if removable}
    <button
      type="button"
      class="ml-0.5 -mr-0.5 p-0.5 rounded-full hover:bg-black/10 dark:hover:bg-white/10 transition-colors"
      onclick={(e) => {
        e.stopPropagation();
        onremove?.(e);
      }}
      aria-label="Remove"
    >
      <i class="bi bi-x {iconSizeClasses[size]}"></i>
    </button>
  {/if}
</span>
