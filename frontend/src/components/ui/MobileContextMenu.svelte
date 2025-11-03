<script>
  import { touchGesture } from "../../lib/touchGestures.js";

  let {
    visible = false,
    x = 0,
    y = 0,
    items = [],
    onClose = () => {},
    target = null,
  } = $props();

  let menuElement = $state(null);
  let adjustedX = $state(0);
  let adjustedY = $state(0);

  // Detect if device is mobile
  let isMobile = $derived(
    typeof window !== "undefined" &&
      ("ontouchstart" in window || navigator.maxTouchPoints > 0)
  );

  $effect(() => {
    if (visible && menuElement) {
      // Adjust position to stay within viewport
      const rect = menuElement.getBoundingClientRect();
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;

      adjustedX = x;
      adjustedY = y;

      // Adjust horizontal position
      if (x + rect.width > viewportWidth) {
        adjustedX = viewportWidth - rect.width - 10;
      }

      // Adjust vertical position
      if (y + rect.height > viewportHeight) {
        adjustedY = viewportHeight - rect.height - 10;
      }

      // Ensure minimum margins
      adjustedX = Math.max(10, adjustedX);
      adjustedY = Math.max(10, adjustedY);
    }
  });

  function handleItemClick(item) {
    if (item.onClick) {
      item.onClick(target);
    }
    onClose();
  }

  function handleBackdropClick(e) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 {isMobile ? 'bg-black/50 backdrop-blur-sm' : ''}"
    onclick={handleBackdropClick}
  >
    <div
      bind:this={menuElement}
      class="context-menu {isMobile ? 'mobile' : 'desktop'}"
      style="left: {isMobile ? '50%' : adjustedX + 'px'}; 
             top: {isMobile ? 'auto' : adjustedY + 'px'}; 
             bottom: {isMobile ? '0' : 'auto'};
             transform: {isMobile ? 'translateX(-50%)' : 'none'};"
      role="menu"
      aria-orientation="vertical"
    >
      {#if isMobile}
        <!-- Mobile: Bottom Sheet Style -->
        <div class="mobile-handle">
          <div class="handle-bar"></div>
        </div>
      {/if}

      <div class="menu-items">
        {#each items as item}
          {#if item.divider}
            <div class="menu-divider"></div>
          {:else}
            <button
              class="menu-item"
              onclick={() => handleItemClick(item)}
              disabled={item.disabled}
            >
              {#if item.icon}
                <i class="bi bi-{item.icon} menu-icon"></i>
              {/if}
              <span class="menu-label">{item.label}</span>
              {#if item.shortcut && !isMobile}
                <span class="menu-shortcut">{item.shortcut}</span>
              {/if}
              {#if item.danger}
                <i class="bi bi-exclamation-triangle text-red-500 ml-auto"></i>
              {/if}
            </button>
          {/if}
        {/each}
      </div>

      {#if isMobile}
        <div class="mobile-footer">
          <button class="cancel-button" onclick={onClose}> Cancel </button>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    background: white;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    min-width: 200px;
    max-width: 90vw;
    z-index: 9999;
  }

  .context-menu.desktop {
    border-radius: 12px;
    border: 1px solid rgb(229 231 235);
  }

  .context-menu.mobile {
    border-radius: 20px 20px 0 0;
    width: 100%;
    max-width: 500px;
    animation: slideUp 0.3s ease-out;
  }

  @keyframes slideUp {
    from {
      transform: translateX(-50%) translateY(100%);
    }
    to {
      transform: translateX(-50%) translateY(0);
    }
  }

  .mobile-handle {
    padding: 12px 0 8px;
    display: flex;
    justify-content: center;
  }

  .handle-bar {
    width: 40px;
    height: 4px;
    background: rgb(209 213 219);
    border-radius: 2px;
  }

  .menu-items {
    padding: 8px 0;
    max-height: 60vh;
    overflow-y: auto;
  }

  .menu-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border: none;
    background: none;
    cursor: pointer;
    transition: background-color 0.2s;
    text-align: left;
    font-size: 14px;
    color: rgb(31 41 55);
  }

  .context-menu.mobile .menu-item {
    padding: 16px 20px;
    font-size: 16px;
  }

  .menu-item:hover:not(:disabled) {
    background: rgb(243 244 246);
  }

  .menu-item:active:not(:disabled) {
    background: rgb(229 231 235);
  }

  .menu-item:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .menu-icon {
    font-size: 16px;
    width: 20px;
    text-align: center;
    flex-shrink: 0;
  }

  .menu-label {
    flex: 1;
  }

  .menu-shortcut {
    font-size: 12px;
    color: rgb(107 114 128);
    font-family: monospace;
  }

  .menu-divider {
    height: 1px;
    background: rgb(229 231 235);
    margin: 4px 0;
  }

  .mobile-footer {
    border-top: 1px solid rgb(229 231 235);
    padding: 12px;
  }

  .cancel-button {
    width: 100%;
    padding: 14px;
    border: none;
    background: rgb(243 244 246);
    border-radius: 10px;
    font-size: 16px;
    font-weight: 600;
    color: rgb(31 41 55);
    cursor: pointer;
  }

  .cancel-button:active {
    background: rgb(229 231 235);
  }

  /* Dark mode */
  @media (prefers-color-scheme: dark) {
    .context-menu {
      background: rgb(31 41 55);
      border-color: rgb(55 65 81);
    }

    .handle-bar {
      background: rgb(75 85 99);
    }

    .menu-item {
      color: rgb(243 244 246);
    }

    .menu-item:hover:not(:disabled) {
      background: rgb(55 65 81);
    }

    .menu-item:active:not(:disabled) {
      background: rgb(75 85 99);
    }

    .menu-divider {
      background: rgb(55 65 81);
    }

    .cancel-button {
      background: rgb(55 65 81);
      color: rgb(243 244 246);
    }

    .cancel-button:active {
      background: rgb(75 85 99);
    }
  }
</style>
