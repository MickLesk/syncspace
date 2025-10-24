<script>
  /**
   * InputV2 - Modern input component using DaisyUI base
   * Features: Multiple types, validation states, icons, labels
   * Created: October 24, 2025
   */

  // Props
  export let type = "text";
  export let value = "";
  export let placeholder = "";
  export let label = "";
  export let error = "";
  export let success = "";
  export let helpText = "";
  export let disabled = false;
  export let required = false;
  export let size = "md"; // xs, sm, md, lg
  export let iconLeft = "";
  export let iconRight = "";
  export let glass = false;
  export let maxlength = null;
  export let showCharCount = false;

  // DaisyUI size classes
  const sizeClasses = {
    xs: "input-xs",
    sm: "input-sm",
    md: "input-md",
    lg: "input-lg",
  };

  // Compute validation state
  $: hasError = !!error;
  $: hasSuccess = !!success && !hasError;

  // Compute final class string
  $: computedClass = [
    "input",
    "input-bordered",
    sizeClasses[size] || sizeClasses.md,
    hasError ? "input-error" : "",
    hasSuccess ? "input-success" : "",
    glass ? "glass-input" : "",
    "w-full",
  ]
    .filter(Boolean)
    .join(" ");

  // Character count
  $: charCount = value?.length || 0;
  $: showCount = showCharCount && maxlength;

  function handleInput(e) {
    value = e.target.value;
  }
</script>

<div class="form-control">
  {#if label}
    <label class="label" for={$$props.id}>
      <span class="label-text">
        {label}
        {#if required}
          <span class="text-error">*</span>
        {/if}
      </span>
      {#if showCount}
        <span class="label-text-alt text-base-content/60">
          {charCount}/{maxlength}
        </span>
      {/if}
    </label>
  {/if}

  <div
    class="input-wrapper"
    class:has-icon-left={iconLeft}
    class:has-icon-right={iconRight || hasError || hasSuccess}
  >
    {#if iconLeft}
      <div class="icon-left">
        <i class="bi bi-{iconLeft}"></i>
      </div>
    {/if}

    <input
      {type}
      {placeholder}
      {disabled}
      {required}
      {maxlength}
      class={computedClass}
      bind:value
      on:input={handleInput}
      on:focus
      on:blur
      on:change
      on:keydown
      on:keyup
      on:keypress
      {...$$restProps}
    />

    {#if iconRight || hasError || hasSuccess}
      <div class="icon-right">
        {#if hasError}
          <i class="bi bi-exclamation-circle-fill text-error"></i>
        {:else if hasSuccess}
          <i class="bi bi-check-circle-fill text-success"></i>
        {:else if iconRight}
          <i class="bi bi-{iconRight}"></i>
        {/if}
      </div>
    {/if}
  </div>

  {#if error}
    <label class="label">
      <span class="label-text-alt text-error">{error}</span>
    </label>
  {:else if success}
    <label class="label">
      <span class="label-text-alt text-success">{success}</span>
    </label>
  {:else if helpText}
    <label class="label">
      <span class="label-text-alt text-base-content/60">{helpText}</span>
    </label>
  {/if}
</div>

<style>
  .form-control {
    width: 100%;
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .input-wrapper.has-icon-left input {
    padding-left: 2.5rem;
  }

  .input-wrapper.has-icon-right input {
    padding-right: 2.5rem;
  }

  .icon-left,
  .icon-right {
    position: absolute;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 2.5rem;
    pointer-events: none;
    color: var(--color-on-surface-variant);
  }

  .icon-left {
    left: 0;
  }

  .icon-right {
    right: 0;
  }

  .icon-left i,
  .icon-right i {
    font-size: 1.125rem;
  }

  /* Glass effect for input */
  .input.glass-input {
    background: var(--glass-background);
    backdrop-filter: blur(var(--glass-blur));
    -webkit-backdrop-filter: blur(var(--glass-blur));
    border-color: var(--glass-border);
  }

  .input.glass-input:focus {
    background: rgba(255, 255, 255, 0.9);
  }

  [data-theme="dark"] .input.glass-input:focus,
  [data-theme="syncspace-dark"] .input.glass-input:focus {
    background: rgba(31, 41, 55, 0.9);
  }

  /* Smooth transitions */
  .input {
    transition: all var(--duration-200) var(--ease-standard);
  }

  /* Focus styles */
  .input:focus {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }

  /* Label styles */
  .label-text {
    font-weight: var(--font-weight-medium);
  }

  .label-text-alt {
    font-size: var(--font-size-xs);
  }

  /* Size-specific icon adjustments */
  .input-xs ~ .icon-left i,
  .input-xs ~ .icon-right i {
    font-size: 0.875rem;
  }

  .input-lg ~ .icon-left i,
  .input-lg ~ .icon-right i {
    font-size: 1.25rem;
  }
</style>
