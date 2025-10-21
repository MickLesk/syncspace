<script>
  import { createEventDispatcher } from "svelte";
  import Button from "./Button.svelte";

  export let open = false;
  export let title = "";
  export let confirmText = "OK";
  export let cancelText = "Abbrechen";
  export let confirmVariant = "filled";
  export let showCancel = true;
  export let danger = false;

  const dispatch = createEventDispatcher();

  function handleConfirm() {
    dispatch("confirm");
    open = false;
  }

  function handleCancel() {
    dispatch("cancel");
    open = false;
  }

  function handleBackdropClick(e) {
    if (e.target === e.currentTarget) {
      handleCancel();
    }
  }
</script>

{#if open}
  <div
    class="dialog-backdrop"
    on:click={handleBackdropClick}
    on:keydown={(e) => e.key === "Escape" && handleCancel()}
    role="button"
    tabindex="-1"
  >
    <div class="dialog" class:danger>
      {#if title}
        <h2 class="dialog-title">{title}</h2>
      {/if}

      <div class="dialog-content">
        <slot />
      </div>

      <div class="dialog-actions">
        {#if showCancel}
          <Button variant="text" onClick={handleCancel}>
            {cancelText}
          </Button>
        {/if}
        <Button variant={confirmVariant} onClick={handleConfirm}>
          {confirmText}
        </Button>
      </div>
    </div>
  </div>
{/if}

<style>
  .dialog-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .dialog {
    background: var(--md-sys-color-surface);
    border-radius: 28px;
    padding: 24px;
    min-width: 280px;
    max-width: 560px;
    max-height: 90vh;
    overflow: auto;
    box-shadow: var(--md-elevation-3);
    animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .dialog.danger {
    border: 2px solid var(--md-sys-color-error);
  }

  .dialog-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 16px 0;
  }

  .dialog-content {
    color: var(--md-sys-color-on-surface-variant);
    font-size: 14px;
    line-height: 1.5;
    margin-bottom: 24px;
    min-height: 40px;
  }

  .dialog-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
</style>
