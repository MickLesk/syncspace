<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Modern Card Component v2 - Tailwind v4
   * Unified card with dark/light mode support
   */
  let {
    children,
    title = "",
    subtitle = "",
    icon = "",
    variant = "default", // default, glass, gradient, elevated
    padding = "normal", // none, small, normal, large
    hoverable = false,
    clickable = false,
    onclick = null,
    class: className = "",
    style = "",
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const baseClasses = "rounded-xl transition-all duration-200";

  const variantClasses = {
    default:
      "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 shadow-md hover:shadow-lg dark:shadow-gray-900/50",
    glass:
      "glass-card hover:shadow-glass-lg dark:bg-gray-800/50 dark:backdrop-blur-xl dark:border-gray-700/50",
    gradient:
      "gradient-bg-primary text-white shadow-lg hover:shadow-xl hover:scale-[1.02] dark:shadow-primary-500/30",
    elevated:
      "bg-white dark:bg-gray-800 shadow-xl hover:shadow-2xl dark:shadow-gray-900/80 border-0",
  };

  const paddingClasses = {
    none: "",
    small: "p-4",
    normal: "p-6",
    large: "p-8",
  };
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
  class="{baseClasses} {variantClasses[variant]} {paddingClasses[
    padding
  ]} {className}"
  class:cursor-pointer={clickable}
  class:hover:scale-[1.02]={hoverable}
  role={clickable ? "button" : undefined}
  tabindex={clickable ? 0 : undefined}
  {onclick}
  {style}
  onkeydown={(e) => {
    if (clickable && (e.key === "Enter" || e.key === " ")) {
      e.preventDefault();
      onclick?.(e);
    }
  }}
>
  {#if title || subtitle || icon}
    <div class="flex items-start gap-4 mb-4" class:mb-0={!title && !subtitle}>
      {#if icon}
        <div
          class="flex-shrink-0 w-12 h-12 rounded-xl bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center text-primary-600 dark:text-primary-400 text-xl"
        >
          <i class="bi bi-{icon}" aria-hidden="true"></i>
        </div>
      {/if}
      <div class="flex-1 min-w-0">
        {#if title}
          <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-1">
            {title}
          </h3>
        {/if}
        {#if subtitle}
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {subtitle}
          </p>
        {/if}
      </div>
    </div>
  {/if}

  <div class="card-content">
    {@render children()}
  </div>
</div>
