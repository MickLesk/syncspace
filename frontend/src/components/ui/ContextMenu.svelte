<script>
  import { createEventDispatcher } from "svelte";
  import Icon from "./Icon.svelte";

  export let visible = false;
  export let x = 0;
  export let y = 0;
  export let items = [];

  const dispatch = createEventDispatcher();

  function handleItemClick(item) {
    if (!item.disabled) {
      dispatch("select", item);
      visible = false;
    }
  }

  function handleClickOutside(event) {
    if (visible) {
      visible = false;
    }
  }

  $: if (visible && typeof window !== "undefined") {
    // Adjust position if menu would go off-screen
    const menuWidth = 200;
    const menuHeight = items.length * 40;

    if (x + menuWidth > window.innerWidth) {
      x = window.innerWidth - menuWidth - 10;
    }
    if (y + menuHeight > window.innerHeight) {
      y = window.innerHeight - menuHeight - 10;
    }
  }
</script>

<svelte:window
  on:click={handleClickOutside}
  on:contextmenu={handleClickOutside}
/>

{#if visible}
  <div
    class="context-menu"
    style="left: {x}px; top: {y}px;"
    on:click|stopPropagation
  >
    {#each items as item}
      {#if item.divider}
        <div class="context-menu-divider"></div>
      {:else}
        <button
          class="context-menu-item"
          class:disabled={item.disabled}
          on:click={() => handleItemClick(item)}
          disabled={item.disabled}
        >
          {#if item.icon}
            <Icon name={item.icon} size={16} />
          {/if}
          <span class="item-label">{item.label}</span>
          {#if item.shortcut}
            <span class="item-shortcut">{item.shortcut}</span>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 10000;
    background: var(--md-sys-color-surface-container);
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 12px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    min-width: 200px;
    padding: 8px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .context-menu-item {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 12px;
    border: none;
    background: transparent;
    color: var(--md-sys-color-on-surface);
    font-size: 14px;
    text-align: left;
    cursor: pointer;
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .context-menu-item:hover:not(.disabled) {
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
  }

  .context-menu-item.disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .item-label {
    flex: 1;
  }

  .item-shortcut {
    font-size: 12px;
    opacity: 0.6;
    font-family: "Roboto Mono", monospace;
  }

  .context-menu-divider {
    height: 1px;
    background: var(--md-sys-color-outline-variant);
    margin: 4px 0;
  }
</style>
