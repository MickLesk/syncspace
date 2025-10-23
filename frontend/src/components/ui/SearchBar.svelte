<script>
  import { createEventDispatcher } from "svelte";

  export let value = "";
  export let placeholder = "Suchen...";
  export let fullWidth = false;
  // Svelte 5: allow passing callback props like `oninput` / `onclear`
  export let oninput = null;
  export let onclear = null;

  const dispatch = createEventDispatcher();
  let focused = false;

  function handleInput(e) {
    value = e.target.value;
    dispatch("input", value);
    if (typeof oninput === "function") oninput(value);
  }

  function handleClear() {
    value = "";
    dispatch("clear");
    if (typeof onclear === "function") onclear();
  }
</script>

<div class="search-bar" class:focused class:full-width={fullWidth}>
  <span class="search-icon">🔍</span>
  <input
    type="text"
    class="search-input"
    {placeholder}
    bind:value
    oninput={handleInput}
    onfocus={() => (focused = true)}
    onblur={() => (focused = false)}
  />
  {#if value}
    <button class="clear-button" onclick={handleClear} type="button">
      ✕
    </button>
  {/if}
</div>

<style>
  .search-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    height: 48px;
    padding: 0 16px;
    background: var(--md-sys-color-surface-variant);
    border-radius: 24px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    width: 320px;
    max-width: 100%;
  }

  .search-bar.full-width {
    width: 100%;
  }

  .search-bar.focused {
    background: var(--md-sys-color-surface);
    box-shadow: var(--md-elevation-2);
  }

  .search-icon {
    font-size: 20px;
    color: var(--md-sys-color-on-surface-variant);
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    border: none;
    background: transparent;
    color: var(--md-sys-color-on-surface);
    font-family: "Roboto", sans-serif;
    font-size: 16px;
    outline: none;
  }

  .search-input::placeholder {
    color: var(--md-sys-color-on-surface-variant);
  }

  .clear-button {
    width: 24px;
    height: 24px;
    border: none;
    background: var(--md-sys-color-on-surface-variant);
    color: var(--md-sys-color-surface);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .clear-button:hover {
    background: var(--md-sys-color-on-surface);
  }
</style>
