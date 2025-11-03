<script>
  /**
   * Modern Button Component v2 - Tailwind v4
   * Unified button with dark/light mode support
   */
  let {
    children = null,
    variant = "primary", // primary, secondary, ghost, danger, success, gradient
    size = "md", // sm, md, lg
    icon = "",
    iconPosition = "left", // left, right
    loading = false,
    disabled = false,
    fullWidth = false,
    onclick = null,
    type = "button",
    class: className = "",
    "aria-label": ariaLabel = "",
    ...restProps
  } = $props();

  const baseClasses =
    "inline-flex items-center justify-center gap-2 font-semibold rounded-xl transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2";

  const variantClasses = {
    primary:
      "bg-primary-500 hover:bg-primary-600 text-white focus:ring-primary-500 dark:focus:ring-primary-400 shadow-md hover:shadow-lg dark:shadow-primary-500/20",
    secondary:
      "bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-900 dark:text-white focus:ring-gray-400 dark:focus:ring-gray-500 border border-gray-300 dark:border-gray-600",
    ghost:
      "bg-transparent hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-300 focus:ring-gray-400 dark:focus:ring-gray-600",
    danger:
      "bg-red-500 hover:bg-red-600 dark:bg-red-600 dark:hover:bg-red-700 text-white focus:ring-red-500 dark:focus:ring-red-400 shadow-md hover:shadow-lg dark:shadow-red-500/20",
    success:
      "bg-green-500 hover:bg-green-600 dark:bg-green-600 dark:hover:bg-green-700 text-white focus:ring-green-500 dark:focus:ring-green-400 shadow-md hover:shadow-lg dark:shadow-green-500/20",
    gradient:
      "gradient-bg-primary text-white shadow-lg hover:shadow-xl hover:scale-105 focus:ring-primary-500 dark:focus:ring-primary-400 hover-glow",
  };

  const sizeClasses = {
    sm: "px-3 py-1.5 text-sm",
    md: "px-4 py-2.5 text-sm",
    lg: "px-6 py-3 text-base",
  };

  const disabledClasses = "opacity-50 cursor-not-allowed";
</script>

<button
  {type}
  class="{baseClasses} {variantClasses[variant]} {sizeClasses[size]} {fullWidth
    ? 'w-full'
    : ''} {disabled || loading ? disabledClasses : ''} {className}"
  disabled={disabled || loading}
  {onclick}
  aria-label={ariaLabel || undefined}
  {...restProps}
>
  {#if loading}
    <svg
      class="animate-spin w-4 h-4"
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
  {:else}
    {#if icon && iconPosition === "left"}
      <i class="bi bi-{icon}"></i>
    {/if}
    <span>
      {@render children()}
    </span>
    {#if icon && iconPosition === "right"}
      <i class="bi bi-{icon}"></i>
    {/if}
  {/if}
</button>
