<script>
  export let variant = 'filled'; // filled, outlined, text, tonal
  export let size = 'medium'; // small, medium, large
  export let disabled = false;
  export let icon = '';
  export let fullWidth = false;
  export let onClick = () => {};
</script>

<button
  class="md-button {variant} {size}"
  class:full-width={fullWidth}
  {disabled}
  type="button"
  on:click={onClick}
>
  {#if icon}
    <span class="icon">{icon}</span>
  {/if}
  <span class="label">
    <slot />
  </span>
</button>

<style>
  .md-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border: none;
    border-radius: 20px;
    font-family: 'Roboto', sans-serif;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    white-space: nowrap;
  }

  .md-button::before {
    content: '';
    position: absolute;
    inset: 0;
    background: currentColor;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .md-button:hover::before {
    opacity: 0.08;
  }

  .md-button:active::before {
    opacity: 0.12;
  }

  .md-button:disabled {
    cursor: not-allowed;
    opacity: 0.38;
  }

  .md-button:disabled::before {
    display: none;
  }

  /* Sizes */
  .small {
    height: 32px;
    padding: 0 16px;
    font-size: 13px;
  }

  .medium {
    height: 40px;
    padding: 0 24px;
    font-size: 14px;
  }

  .large {
    height: 48px;
    padding: 0 32px;
    font-size: 15px;
  }

  /* Variants */
  .filled {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
  }

  .filled:hover {
    box-shadow: var(--md-elevation-2);
  }

  .filled:active {
    box-shadow: var(--md-elevation-1);
  }

  .outlined {
    background: transparent;
    color: var(--md-sys-color-primary);
    border: 1px solid var(--md-sys-color-outline);
  }

  .text {
    background: transparent;
    color: var(--md-sys-color-primary);
    padding: 0 12px;
  }

  .tonal {
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
  }

  .full-width {
    width: 100%;
  }

  .icon {
    font-size: 18px;
    line-height: 1;
  }

  .label {
    position: relative;
    z-index: 1;
  }
</style>
