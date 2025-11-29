<script>
  /**
   * Card Component v3 - SyncSpace Design System
   * 
   * Unified card with multiple variants, glass effects, and dark mode.
   * Supports headers, footers, and various content layouts.
   */
  import { cn } from "../../lib/design-system/utils.js";

  let {
    children,
    header = null,
    footer = null,
    title = "",
    subtitle = "",
    icon = "",
    variant = "default",
    padding = "normal",
    hoverable = false,
    clickable = false,
    selected = false,
    onclick = null,
    class: className = "",
    ...restProps
  } = $props();

  const baseClasses = "rounded-2xl transition-all duration-200";

  const variantClasses = {
    default: `
      bg-white dark:bg-gray-800
      border border-gray-200 dark:border-gray-700
      shadow-md
      dark:shadow-gray-900/30
    `,
    glass: `
      bg-white/90 dark:bg-gray-800/85
      backdrop-blur-xl
      border border-gray-200/60 dark:border-gray-700/50
      shadow-lg
      dark:shadow-gray-900/20
    `,
    gradient: `
      gradient-bg-primary
      text-white
      shadow-lg
      border-0
    `,
    elevated: `
      bg-white dark:bg-gray-800
      shadow-xl hover:shadow-2xl
      dark:shadow-gray-900/50
      border-0
    `,
    outline: `
      bg-transparent
      border-2 border-gray-200 dark:border-gray-700
    `,
    flat: `
      bg-gray-50 dark:bg-gray-800/50
      border-0
    `,
    ghost: `
      bg-transparent
      border border-transparent
      hover:border-gray-200 dark:hover:border-gray-700
      hover:bg-gray-50 dark:hover:bg-gray-800/50
    `,
  };

  const paddingClasses = {
    none: "",
    xs: "p-2",
    sm: "p-3",
    normal: "p-5",
    lg: "p-6",
    xl: "p-8",
  };

  const hoverClasses = hoverable ? `
    hover:shadow-lg hover:scale-[1.01] hover:-translate-y-0.5
    dark:hover:shadow-gray-900/40
  ` : "";

  const clickableClasses = clickable ? "cursor-pointer active:scale-[0.99]" : "";

  const selectedClasses = selected ? `
    ring-2 ring-primary-500 ring-offset-2
    dark:ring-offset-gray-900
  ` : "";

  const computedClasses = $derived(cn(
    baseClasses,
    variantClasses[variant] || variantClasses.default,
    paddingClasses[padding],
    hoverClasses,
    clickableClasses,
    selectedClasses,
    className
  ));
</script>

<div
  class={computedClasses}
  role={clickable ? "button" : undefined}
  tabindex={clickable ? 0 : undefined}
  {onclick}
  onkeydown={(e) => {
    if (clickable && (e.key === "Enter" || e.key === " ")) {
      e.preventDefault();
      onclick?.(e);
    }
  }}
  {...restProps}
>
  <!-- Header Section -->
  {#if header}
    <div class="card-header mb-4 -mx-5 -mt-5 px-5 py-4 border-b border-gray-100 dark:border-gray-700/50 rounded-t-2xl bg-gray-50/50 dark:bg-gray-900/30">
      {@render header()}
    </div>
  {:else if title || subtitle || icon}
    <div class="flex items-start gap-4 mb-4">
      {#if icon}
        <div class="flex-shrink-0 w-12 h-12 rounded-xl bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center text-primary-600 dark:text-primary-400 text-xl">
          <i class="bi bi-{icon}"></i>
        </div>
      {/if}
      <div class="flex-1 min-w-0">
        {#if title}
          <h3 class="text-lg font-bold text-gray-900 dark:text-white leading-tight">
            {title}
          </h3>
        {/if}
        {#if subtitle}
          <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
            {subtitle}
          </p>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Main Content -->
  <div class="card-content">
    {@render children()}
  </div>

  <!-- Footer Section -->
  {#if footer}
    <div class="card-footer mt-4 -mx-5 -mb-5 px-5 py-4 border-t border-gray-100 dark:border-gray-700/50 rounded-b-2xl bg-gray-50/50 dark:bg-gray-900/30">
      {@render footer()}
    </div>
  {/if}
</div>
