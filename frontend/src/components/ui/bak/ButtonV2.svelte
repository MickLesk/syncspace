<script>
  /**
   * ButtonV2 - Modern button component using DaisyUI base
   * Features: Multiple variants, sizes, loading states, icons
   * Created: October 24, 2025
   */

  // Props
  export let variant = "default"; // default, primary, secondary, accent, ghost, outline, error, success, link
  export let size = "md"; // xs, sm, md, lg
  export let loading = false;
  export let disabled = false;
  export let type = "button";
  export let onClick = null;
  export let iconLeft = "";
  export let iconRight = "";
  export let glass = false; // Enable glassmorphism
  export let fullWidth = false;

  // DaisyUI button classes mapping
  const variantClasses = {
    default: "btn-neutral",
    primary: "btn-primary",
    secondary: "btn-secondary",
    accent: "btn-accent",
    ghost: "btn-ghost",
    outline: "btn-outline",
    error: "btn-error",
    success: "btn-success",
    link: "btn-link",
  };

  const sizeClasses = {
    xs: "btn-xs",
    sm: "btn-sm",
    md: "btn-md",
    lg: "btn-lg",
  };

  // Compute final class string
  $: computedClass = [
    "btn",
    variantClasses[variant] || variantClasses.default,
    sizeClasses[size] || sizeClasses.md,
    glass ? "glass" : "",
    fullWidth ? "btn-block" : "",
    loading ? "loading" : "",
  ]
    .filter(Boolean)
    .join(" ");

  function handleClick(e) {
    if (disabled || loading) {
      e.preventDefault();
      return;
    }
    if (onClick) {
      onClick(e);
    }
  }
</script>

<button
  {type}
  class={computedClass}
  disabled={disabled || loading}
  on:click={handleClick}
  {...$$restProps}
>
  {#if loading}
    <span class="loading loading-spinner loading-sm"></span>
  {:else if iconLeft}
    <slot name="icon-left">
      <i class="bi bi-{iconLeft}"></i>
    </slot>
  {/if}

  <slot />

  {#if iconRight && !loading}
    <slot name="icon-right">
      <i class="bi bi-{iconRight}"></i>
    </slot>
  {/if}
</button>

<style>
  /* Additional custom styles on top of DaisyUI */
  .btn.glass {
    background: var(--glass-background);
    backdrop-filter: blur(var(--glass-blur));
    -webkit-backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
  }

  .btn.glass:hover {
    background: rgba(255, 255, 255, 0.85);
    transform: translateY(-2px);
  }

  [data-theme="dark"] .btn.glass:hover,
  [data-theme="syncspace-dark"] .btn.glass:hover {
    background: rgba(31, 41, 55, 0.85);
  }

  /* Smooth transitions */
  .btn {
    transition: all var(--duration-200) var(--ease-standard);
  }

  /* Icon spacing */
  .btn i {
    font-size: 1em;
  }

  .btn.btn-xs i {
    font-size: 0.875em;
  }

  .btn.btn-lg i {
    font-size: 1.125em;
  }
</style>
