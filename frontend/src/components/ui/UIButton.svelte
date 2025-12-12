<!-- UIButton.svelte - Consistent button styles -->
<script>
  export let variant = "default"; // 'default', 'primary', 'danger', 'success', 'ghost'
  export let size = "md"; // 'xs', 'sm', 'md', 'lg', 'xl'
  export let icon = "";
  export let iconPosition = "left"; // 'left', 'right', 'only'
  export let loading = false;
  export let disabled = false;
  export let fullWidth = false;
  export let href = "";
  export let type = "button";

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  function handleClick(event) {
    if (!disabled && !loading) {
      dispatch("click", event);
    }
  }

  // Size classes
  const sizeClasses = {
    xs: "px-2 py-1 text-xs",
    sm: "px-3 py-1.5 text-sm",
    md: "px-4 py-2 text-sm",
    lg: "px-6 py-3 text-base",
    xl: "px-8 py-4 text-lg",
  };

  // Variant classes
  const variantClasses = {
    default:
      "bg-white dark:bg-slate-800 hover:bg-gray-50 dark:hover:bg-slate-700 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-slate-600",
    primary:
      "bg-blue-600 hover:bg-blue-700 text-white border border-transparent",
    danger: "bg-red-600 hover:bg-red-700 text-white border border-transparent",
    success:
      "bg-green-600 hover:bg-green-700 text-white border border-transparent",
    ghost:
      "bg-transparent hover:bg-gray-100 dark:hover:bg-slate-800 text-gray-700 dark:text-gray-300 border border-transparent",
  };

  // Disabled classes
  const disabledClasses = {
    default:
      "disabled:bg-gray-100 dark:disabled:bg-slate-700 disabled:text-gray-400 dark:disabled:text-slate-500 disabled:border-gray-200 dark:disabled:border-slate-600",
    primary: "disabled:bg-blue-400 dark:disabled:bg-blue-700",
    danger: "disabled:bg-red-400 dark:disabled:bg-red-700",
    success: "disabled:bg-green-400 dark:disabled:bg-green-700",
    ghost:
      "disabled:bg-transparent disabled:text-gray-400 dark:disabled:text-slate-500",
  };

  const baseClasses =
    "inline-flex items-center justify-center font-medium rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 disabled:cursor-not-allowed";
</script>

{#if href}
  <a
    {href}
    class="{baseClasses} {sizeClasses[size]} {variantClasses[
      variant
    ]} {fullWidth ? 'w-full' : ''}"
  >
    {#if icon && (iconPosition === "left" || iconPosition === "only")}
      <i
        class="bi bi-{icon} {iconPosition !== 'only' ? 'mr-2' : ''}"
        aria-hidden="true"
      ></i>
    {/if}

    {#if iconPosition !== "only"}
      <slot />
    {/if}

    {#if icon && iconPosition === "right"}
      <i class="bi bi-{icon} ml-2" aria-hidden="true"></i>
    {/if}
  </a>
{:else}
  <button
    {type}
    class="{baseClasses} {sizeClasses[size]} {variantClasses[
      variant
    ]} {disabledClasses[variant]} {fullWidth ? 'w-full' : ''}"
    disabled={disabled || loading}
    onclick={handleClick}
  >
    {#if loading}
      <div
        class="animate-spin rounded-full h-4 w-4 border-2 border-current border-t-transparent {iconPosition !==
        'only'
          ? 'mr-2'
          : ''}"
        aria-hidden="true"
      ></div>
    {:else if icon && (iconPosition === "left" || iconPosition === "only")}
      <i
        class="bi bi-{icon} {iconPosition !== 'only' ? 'mr-2' : ''}"
        aria-hidden="true"
      ></i>
    {/if}

    {#if !loading && iconPosition !== "only"}
      <slot />
    {/if}

    {#if !loading && icon && iconPosition === "right"}
      <i class="bi bi-{icon} ml-2" aria-hidden="true"></i>
    {/if}
  </button>
{/if}
