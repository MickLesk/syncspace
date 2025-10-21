<script>
  export let value = '';
  export let label = '';
  export let type = 'text';
  export let placeholder = '';
  export let disabled = false;
  export let error = '';
  export let required = false;
  export let icon = '';

  let focused = false;
  let filled = value.length > 0;

  $: filled = value.length > 0 || focused;
</script>

<div class="input-wrapper" class:focused class:filled class:error={error.length > 0}>
  {#if icon}
    <span class="leading-icon">{icon}</span>
  {/if}
  
  <div class="input-container">
    <input
      class="md-input"
      {type}
      {placeholder}
      {disabled}
      {required}
      bind:value
      on:focus={() => focused = true}
      on:blur={() => focused = false}
      on:input
      on:change
      id="input-{Math.random()}"
    />
    
    {#if label}
      <label class="input-label" for="input-{Math.random()}">
        {label}{#if required}<span class="required">*</span>{/if}
      </label>
    {/if}
  </div>

  {#if error}
    <span class="error-text">{error}</span>
  {/if}
</div>

<style>
  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
  }

  .input-container {
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .input-label {
    position: absolute;
    left: 16px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--md-sys-color-on-surface-variant);
    font-size: 16px;
    pointer-events: none;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    background: var(--md-sys-color-surface);
    padding: 0 4px;
  }

  .filled .input-label,
  .focused .input-label {
    top: 0;
    font-size: 12px;
    color: var(--md-sys-color-primary);
  }

  .error .input-label {
    color: var(--md-sys-color-error);
  }

  .md-input {
    width: 100%;
    height: 56px;
    padding: 16px;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 12px;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-family: 'Roboto', sans-serif;
    font-size: 16px;
    outline: none;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .md-input:focus {
    border-color: var(--md-sys-color-primary);
    border-width: 2px;
  }

  .md-input:disabled {
    opacity: 0.38;
    cursor: not-allowed;
  }

  .error .md-input {
    border-color: var(--md-sys-color-error);
  }

  .leading-icon {
    font-size: 24px;
    color: var(--md-sys-color-on-surface-variant);
    margin-left: 12px;
  }

  .error-text {
    position: absolute;
    bottom: -20px;
    left: 16px;
    font-size: 12px;
    color: var(--md-sys-color-error);
  }

  .required {
    color: var(--md-sys-color-error);
  }
</style>
