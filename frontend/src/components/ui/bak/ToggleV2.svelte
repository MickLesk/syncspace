<!--
  ToggleV2.svelte - Modern Toggle Switch Component
  
  A sleek toggle switch with smooth animations and Material Design 3 styling.
  
  Props:
  - checked: boolean - Toggle state (required, use bind:checked)
  - label: string - Label text
  - description: string - Helper text below label
  - disabled: boolean - Disable the toggle
  - color: "primary" | "secondary" | "success" | "warning" | "error" (default: "primary")
  - size: "sm" | "md" | "lg" (default: "md")
  
  Usage:
  <ToggleV2 
    bind:checked={myVariable}
    label="Enable Feature"
    description="This enables a cool feature"
    color="success"
  />
-->

<script>
  export let checked = false;
  export let label = "";
  export let description = "";
  export let disabled = false;
  export let color = "primary";
  export let size = "md";

  $: colorClass = {
    primary: "toggle-primary",
    secondary: "toggle-secondary",
    success: "toggle-success",
    warning: "toggle-warning",
    error: "toggle-error",
  }[color];

  $: sizeClass = {
    sm: "toggle-sm",
    md: "toggle-md",
    lg: "toggle-lg",
  }[size];

  function handleToggle() {
    if (!disabled) {
      checked = !checked;
    }
  }
</script>

<div class="toggle-container">
  <label class="toggle-wrapper" class:disabled>
    <input
      type="checkbox"
      class="toggle {colorClass} {sizeClass}"
      bind:checked
      {disabled}
      on:change
    />
    {#if label || description}
      <div class="toggle-content">
        {#if label}
          <span class="toggle-label">{label}</span>
        {/if}
        {#if description}
          <span class="toggle-description">{description}</span>
        {/if}
      </div>
    {/if}
  </label>
</div>

<style>
  .toggle-container {
    width: 100%;
  }

  .toggle-wrapper {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    cursor: pointer;
    padding: 0.75rem;
    border-radius: 12px;
    transition: background-color 0.2s ease;
  }

  .toggle-wrapper:hover:not(.disabled) {
    background-color: var(--md-sys-color-surface-variant);
  }

  .toggle-wrapper.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toggle-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .toggle-label {
    font-size: 0.9375rem;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    line-height: 1.4;
  }

  .toggle-description {
    font-size: 0.8125rem;
    color: var(--md-sys-color-on-surface-variant);
    line-height: 1.4;
  }

  /* Modern Toggle Switch Styles */
  .toggle {
    appearance: none;
    width: 3rem;
    height: 1.5rem;
    background-color: var(--md-sys-color-surface-variant);
    border-radius: 9999px;
    position: relative;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    border: 2px solid var(--md-sys-color-outline);
    flex-shrink: 0;
  }

  .toggle::before {
    content: "";
    position: absolute;
    width: 1rem;
    height: 1rem;
    background-color: var(--md-sys-color-outline);
    border-radius: 50%;
    top: 50%;
    left: 0.125rem;
    transform: translateY(-50%);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .toggle:checked {
    background-color: var(--md-sys-color-primary);
    border-color: var(--md-sys-color-primary);
  }

  .toggle:checked::before {
    left: calc(100% - 1.125rem);
    background-color: var(--md-sys-color-on-primary);
  }

  /* Color Variants */
  .toggle-primary:checked {
    background-color: var(--md-sys-color-primary);
    border-color: var(--md-sys-color-primary);
  }

  .toggle-secondary:checked {
    background-color: var(--md-sys-color-secondary);
    border-color: var(--md-sys-color-secondary);
  }

  .toggle-success:checked {
    background-color: #10b981;
    border-color: #10b981;
  }

  .toggle-warning:checked {
    background-color: #f59e0b;
    border-color: #f59e0b;
  }

  .toggle-error:checked {
    background-color: #ef4444;
    border-color: #ef4444;
  }

  /* Size Variants */
  .toggle-sm {
    width: 2.5rem;
    height: 1.25rem;
  }

  .toggle-sm::before {
    width: 0.875rem;
    height: 0.875rem;
  }

  .toggle-sm:checked::before {
    left: calc(100% - 1rem);
  }

  .toggle-lg {
    width: 3.5rem;
    height: 1.75rem;
  }

  .toggle-lg::before {
    width: 1.25rem;
    height: 1.25rem;
  }

  .toggle-lg:checked::before {
    left: calc(100% - 1.375rem);
  }

  /* Disabled State */
  .toggle:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  /* Focus State */
  .toggle:focus-visible {
    outline: 2px solid var(--md-sys-color-primary);
    outline-offset: 2px;
  }

  /* Hover Effect */
  .toggle:not(:disabled):hover {
    box-shadow: 0 0 0 8px rgba(103, 80, 164, 0.1);
  }
</style>
