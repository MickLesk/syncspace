<script>
  /**
   * Input Component v3 - SyncSpace Design System
   *
   * Unified input with multiple variants, sizes, and states.
   * Supports icons, validation, and dark mode.
   */
  import { cn } from "../../lib/design-system/utils.js";

  let {
    type = "text",
    value = $bindable(""),
    placeholder = "",
    label = "",
    hint = "",
    error = "",
    icon = "",
    iconRight = "",
    variant = "default",
    size = "md",
    disabled = false,
    required = false,
    readonly = false,
    class: className = "",
    inputClass = "",
    onchange = null,
    oninput = null,
    onfocus = null,
    onblur = null,
    ...restProps
  } = $props();

  let focused = $state(false);

  const wrapperClasses = "relative w-full";

  const labelClasses = `
    block text-sm font-medium mb-1.5
    text-gray-700 dark:text-gray-300
  `;

  const baseInputClasses = `
    w-full rounded-xl
    bg-white dark:bg-gray-800
    text-gray-900 dark:text-gray-100
    placeholder:text-gray-400 dark:placeholder:text-gray-500
    transition-all duration-200
    focus:outline-none
  `;

  const variantClasses = {
    default: `
      border border-gray-300 dark:border-gray-600
      hover:border-gray-400 dark:hover:border-gray-500
      focus:border-primary-500 dark:focus:border-primary-400
      focus:ring-2 focus:ring-primary-500/20
    `,
    glass: `
      bg-white/60 dark:bg-gray-800/60
      backdrop-blur-md
      border border-gray-200/50 dark:border-gray-700/50
      hover:border-gray-300/70 dark:hover:border-gray-600/70
      focus:border-primary-500 dark:focus:border-primary-400
      focus:ring-2 focus:ring-primary-500/20
    `,
    filled: `
      bg-gray-100 dark:bg-gray-700
      border-2 border-transparent
      hover:bg-gray-200/70 dark:hover:bg-gray-600/70
      focus:bg-white dark:focus:bg-gray-800
      focus:border-primary-500 dark:focus:border-primary-400
      focus:ring-0
    `,
    underline: `
      bg-transparent
      border-0 border-b-2 border-gray-300 dark:border-gray-600
      rounded-none
      hover:border-gray-400 dark:hover:border-gray-500
      focus:border-primary-500 dark:focus:border-primary-400
      focus:ring-0
    `,
  };

  const errorClasses = `
    border-red-500 dark:border-red-500
    hover:border-red-600 dark:hover:border-red-400
    focus:border-red-500 dark:focus:border-red-400
    focus:ring-2 focus:ring-red-500/20
  `;

  const sizeClasses = {
    sm: "px-3 py-1.5 text-sm min-h-[32px]",
    md: "px-4 py-2.5 text-sm min-h-[42px]",
    lg: "px-5 py-3 text-base min-h-[50px]",
  };

  const iconPaddingLeft = {
    sm: "pl-9",
    md: "pl-11",
    lg: "pl-12",
  };

  const iconPaddingRight = {
    sm: "pr-9",
    md: "pr-11",
    lg: "pr-12",
  };

  const iconPositionClasses = {
    sm: "top-1/2 -translate-y-1/2 text-sm",
    md: "top-1/2 -translate-y-1/2 text-base",
    lg: "top-1/2 -translate-y-1/2 text-lg",
  };

  const disabledClasses =
    "opacity-50 cursor-not-allowed bg-gray-100 dark:bg-gray-900";

  const computedInputClasses = $derived(
    cn(
      baseInputClasses,
      error ? errorClasses : variantClasses[variant] || variantClasses.default,
      sizeClasses[size] || sizeClasses.md,
      icon && (iconPaddingLeft[size] || iconPaddingLeft.md),
      iconRight && (iconPaddingRight[size] || iconPaddingRight.md),
      disabled && disabledClasses,
      inputClass
    )
  );

  function handleFocus(e) {
    focused = true;
    onfocus?.(e);
  }

  function handleBlur(e) {
    focused = false;
    onblur?.(e);
  }
</script>

<div class={cn(wrapperClasses, className)}>
  {#if label}
    <label class={labelClasses}>
      {label}
      {#if required}
        <span class="text-red-500 ml-0.5">*</span>
      {/if}
    </label>
  {/if}

  <div class="relative">
    {#if icon}
      <span
        class="absolute left-3 {iconPositionClasses[
          size
        ]} text-gray-400 dark:text-gray-500 pointer-events-none {focused
          ? 'text-primary-500 dark:text-primary-400'
          : ''}"
      >
        <i class="bi bi-{icon}"></i>
      </span>
    {/if}

    <input
      {type}
      bind:value
      {placeholder}
      {disabled}
      {required}
      {readonly}
      class={computedInputClasses}
      {onchange}
      {oninput}
      onfocus={handleFocus}
      onblur={handleBlur}
      aria-invalid={!!error}
      aria-describedby={error ? "input-error" : hint ? "input-hint" : undefined}
      {...restProps}
    />

    {#if iconRight}
      <span
        class="absolute right-3 {iconPositionClasses[
          size
        ]} text-gray-400 dark:text-gray-500 pointer-events-none"
      >
        <i class="bi bi-{iconRight}"></i>
      </span>
    {/if}
  </div>

  {#if error}
    <p
      id="input-error"
      class="mt-1.5 text-sm text-red-600 dark:text-red-400 flex items-center gap-1.5"
    >
      <i class="bi bi-exclamation-circle text-xs"></i>
      {error}
    </p>
  {:else if hint}
    <p id="input-hint" class="mt-1.5 text-sm text-gray-500 dark:text-gray-400">
      {hint}
    </p>
  {/if}
</div>
