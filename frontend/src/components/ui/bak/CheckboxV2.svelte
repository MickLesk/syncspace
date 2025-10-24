<!--
  CheckboxV2.svelte - Modern Checkbox Component using DaisyUI
  
  A flexible checkbox with indeterminate state and custom styling.
  
  Props:
  - checked: boolean - Checked state
  - indeterminate: boolean - Indeterminate state (default: false)
  - label: string - Checkbox label
  - description: string - Additional description text
  - disabled: boolean - Disable checkbox
  - size: "xs" | "sm" | "md" | "lg" (default: "md")
  - color: "primary" | "secondary" | "accent" | "success" | "warning" | "error" (default: "primary")
  
  Events:
  - on:change - Fired when checkbox state changes
  
  Usage:
  <CheckboxV2 
    bind:checked={agreedToTerms}
    label="I agree to the terms and conditions"
    description="Please read our terms carefully"
  />
-->

<script>
  import { createEventDispatcher } from "svelte";

  export let checked = false;
  export let indeterminate = false;
  export let label = "";
  export let description = "";
  export let disabled = false;
  export let size = "md";
  export let color = "primary";

  let checkboxElement;
  const dispatch = createEventDispatcher();

  $: sizeClasses = {
    xs: "checkbox-xs",
    sm: "checkbox-sm",
    md: "checkbox-md",
    lg: "checkbox-lg",
  };

  $: colorClasses = {
    primary: "checkbox-primary",
    secondary: "checkbox-secondary",
    accent: "checkbox-accent",
    success: "checkbox-success",
    warning: "checkbox-warning",
    error: "checkbox-error",
  };

  $: computedClass = ["checkbox", sizeClasses[size], colorClasses[color]]
    .filter(Boolean)
    .join(" ");

  // Set indeterminate state
  $: if (checkboxElement) {
    checkboxElement.indeterminate = indeterminate;
  }

  function handleChange(e) {
    checked = e.target.checked;
    dispatch("change", { checked });
  }
</script>

<div class="form-control">
  <label
    class="label cursor-pointer justify-start gap-3"
    class:opacity-50={disabled}
  >
    <input
      bind:this={checkboxElement}
      type="checkbox"
      class={computedClass}
      {disabled}
      {checked}
      on:change={handleChange}
      on:click
      {...$$restProps}
    />

    {#if label || $$slots.default}
      <div class="flex flex-col gap-1">
        <span class="label-text font-medium">
          {#if $$slots.default}
            <slot />
          {:else}
            {label}
          {/if}
        </span>

        {#if description}
          <span class="label-text-alt opacity-70">
            {description}
          </span>
        {/if}
      </div>
    {/if}
  </label>
</div>

<style>
  .form-control {
    padding: 0;
  }

  .label {
    transition: all var(--duration-200) var(--ease-standard);
  }

  .label:hover:not(.opacity-50) {
    background: var(--color-surface-container-high);
    border-radius: var(--radius-lg);
    padding: var(--spacing-2) var(--spacing-3);
    margin: calc(var(--spacing-2) * -1) calc(var(--spacing-3) * -1);
  }

  input[type="checkbox"] {
    transition: all var(--duration-200) var(--ease-standard);
    flex-shrink: 0;
  }

  input[type="checkbox"]:focus {
    box-shadow: 0 0 0 4px rgba(var(--color-primary-rgb), 0.2);
  }

  .label-text {
    color: var(--color-on-surface);
    line-height: var(--line-height-normal);
  }

  .label-text-alt {
    font-size: var(--font-size-sm);
    color: var(--color-on-surface-variant);
  }
</style>
