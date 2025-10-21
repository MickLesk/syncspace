<script>
  export let variant = "filled"; // filled, outlined, text
  export let type = "button";
  export let disabled = false;
  export let loading = false;
  export let icon = "";
</script>

<button class="md-button {variant}" {type} {disabled} on:click>
  {#if loading}
    <span class="spinner"></span>
  {:else if icon}
    <span class="icon">{icon}</span>
  {/if}
  <slot />
</button>

<style>
  .md-button {
    padding: 10px 24px;
    font-size: 14px;
    font-weight: 500;
    font-family: inherit;
    letter-spacing: 0.1px;
    border-radius: 100px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border: none;
    outline: none;
  }

  /* Filled Button */
  .md-button.filled {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    box-shadow: var(--md-elevation-1);
  }

  .md-button.filled:hover:not(:disabled) {
    box-shadow: var(--md-elevation-2);
  }

  /* Outlined Button */
  .md-button.outlined {
    background: transparent;
    color: var(--md-sys-color-primary);
    border: 1px solid var(--md-sys-color-outline);
  }

  .md-button.outlined:hover:not(:disabled) {
    background: var(--md-sys-color-primary-container);
  }

  /* Text Button */
  .md-button.text {
    background: transparent;
    color: var(--md-sys-color-primary);
  }

  .md-button.text:hover:not(:disabled) {
    background: var(--md-sys-color-primary-container);
  }

  .md-button:disabled {
    opacity: 0.38;
    cursor: not-allowed;
    box-shadow: none;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .icon {
    font-size: 18px;
  }
</style>
