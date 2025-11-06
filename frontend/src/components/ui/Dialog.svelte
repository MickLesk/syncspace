<script>
  import { createEventDispatcher } from "svelte";
  import Button from "./Button.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  export let open = false;
  export let title = "";
  export let confirmText = tr("ok");
  export let cancelText = tr("cancel");
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
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === "Escape" && handleCancel()}
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
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(8px);
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
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    border-radius: 24px;
    padding: 28px;
    min-width: 320px;
    max-width: 560px;
    max-height: 90vh;
    overflow: auto;
    animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes slideUp {
    from {
      transform: translateY(32px) scale(0.95);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .dialog.danger {
    border: 2px solid rgba(239, 68, 68, 0.5);
    box-shadow: 0 20px 60px rgba(239, 68, 68, 0.2);
  }

  .dialog-title {
    font-size: 24px;
    font-weight: 700;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 20px 0;
    letter-spacing: -0.5px;
  }

  .dialog-content {
    color: var(--md-sys-color-on-surface-variant);
    font-size: 14px;
    line-height: 1.6;
    margin-bottom: 28px;
    min-height: 40px;
  }

  .dialog-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }

  /* Dark Mode */
  :global(.dark) .dialog {
    background: rgba(30, 41, 59, 0.95);
    border-color: rgba(255, 255, 255, 0.1);
  }
</style>
