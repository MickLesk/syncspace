<script>
  /**
   * ConfirmDialog Component
   * Styled confirmation dialog als Ersatz für native confirm()
   *
   * @component
   * @example
   * <ConfirmDialog
   *   bind:open={showConfirm}
   *   title="Delete File?"
   *   message="Are you sure you want to delete this file? This action cannot be undone."
   *   confirmText="Delete"
   *   cancelText="Cancel"
   *   variant="danger"
   *   on:confirm={handleDelete}
   *   on:cancel={handleCancel}
   * />
   */

  import { createEventDispatcher } from "svelte";
  import Icon from "./Icon.svelte";
  import Button from "./Button.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const dispatch = createEventDispatcher();

  /** @type {boolean} - Ob Dialog geöffnet ist */
  let {
    open = $bindable(false),
    title = "",
    message = "",
    confirmText = "",
    cancelText = "",
    variant = "default",
    icon = "",
    loading = false,
    content
  } = $props();

  // Derived default texts
  const defaultTitle = $derived(title || tr("confirmAction"));
  const defaultMessage = $derived(message || tr("areYouSure"));
  const defaultConfirmText = $derived(confirmText || tr("confirm"));
  const defaultCancelText = $derived(cancelText || tr("cancel"));

  // Auto-Icons basierend auf Variant
  const defaultIcon = $derived(
    variant === "danger"
      ? "exclamation-triangle-fill"
      : variant === "warning"
        ? "exclamation-circle-fill"
        : variant === "success"
          ? "check-circle-fill"
          : "question-circle-fill"
  );

  const displayIcon = $derived(icon || defaultIcon);

  function handleConfirm() {
    if (loading) return;
    dispatch("confirm");
  }

  function handleCancel() {
    if (loading) return;
    open = false;
    dispatch("cancel");
  }

  function handleBackdropClick(event) {
    if (event.target === event.currentTarget && !loading) {
      handleCancel();
    }
  }

  function handleKeydown(event) {
    if (event.key === "Escape" && !loading) {
      handleCancel();
    } else if (event.key === "Enter" && !loading) {
      handleConfirm();
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="confirm-dialog-backdrop"
    onclick={handleBackdropClick}
    onkeydown={handleKeydown}
    role="presentation"
  >
    <div
      class="confirm-dialog"
      class:variant-danger={variant === "danger"}
      class:variant-warning={variant === "warning"}
      class:variant-success={variant === "success"}
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeydown}
      role="alertdialog"
      aria-labelledby="confirm-dialog-title"
      aria-describedby="confirm-dialog-message"
      aria-modal="true"
      tabindex="-1"
    >
      <!-- Icon Circle -->
      <div class="dialog-icon-circle">
        <Icon name={displayIcon} size={32} />
      </div>

      <!-- Content -->
      <div class="dialog-content">
        <h2 id="confirm-dialog-title" class="dialog-title">{defaultTitle}</h2>
        <p id="confirm-dialog-message" class="dialog-message">{defaultMessage}</p>

        {@render content?.()}
      </div>

      <!-- Actions -->
      <div class="dialog-actions">
        <Button
          onClick={handleCancel}
          variant="outlined"
          size="medium"
          disabled={loading}
        >
          {defaultCancelText}
        </Button>
        <Button
          onClick={handleConfirm}
          variant={variant === "danger"
            ? "danger"
            : variant === "success"
              ? "success"
              : "filled"}
          size="medium"
          disabled={loading}
        >
          {#if loading}
            <div class="spinner"></div>
          {/if}
          {defaultConfirmText}
        </Button>
      </div>
    </div>
  </div>
{/if}

<style>
  .confirm-dialog-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    padding: 20px;
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .confirm-dialog {
    background: var(--md-sys-color-surface);
    border-radius: 24px;
    padding: 32px;
    max-width: 480px;
    width: 100%;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
    animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(30px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Icon Circle */
  .dialog-icon-circle {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(99, 102, 241, 0.12);
    color: var(--md-sys-color-primary);
    flex-shrink: 0;
  }

  .variant-danger .dialog-icon-circle {
    background: rgba(239, 68, 68, 0.12);
    color: var(--md-sys-color-error);
  }

  .variant-warning .dialog-icon-circle {
    background: rgba(245, 158, 11, 0.12);
    color: #f59e0b;
  }

  .variant-success .dialog-icon-circle {
    background: rgba(16, 185, 129, 0.12);
    color: var(--md-sys-color-success, #10b981);
  }

  /* Content */
  .dialog-content {
    text-align: center;
    width: 100%;
  }

  .dialog-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin: 0 0 12px 0;
    font-family:
      "Inter",
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      sans-serif;
  }

  .dialog-message {
    font-size: 16px;
    line-height: 1.5;
    color: var(--md-sys-color-on-surface-variant);
    margin: 0;
  }

  /* Actions */
  .dialog-actions {
    display: flex;
    gap: 12px;
    width: 100%;
    justify-content: flex-end;
  }

  .dialog-actions :global(button) {
    flex: 1;
    max-width: 150px;
  }

  /* Loading Spinner */
  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-right: 8px;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Dark Mode */
  :global([data-theme="dark"]) .confirm-dialog {
    background: rgba(30, 30, 38, 0.98);
  }

  /* Responsive */
  @media (max-width: 768px) {
    .confirm-dialog-backdrop {
      padding: 16px;
    }

    .confirm-dialog {
      padding: 24px;
      gap: 20px;
    }

    .dialog-icon-circle {
      width: 64px;
      height: 64px;
    }

    .dialog-title {
      font-size: 20px;
    }

    .dialog-message {
      font-size: 14px;
    }

    .dialog-actions {
      flex-direction: column-reverse;
      gap: 8px;
    }

    .dialog-actions :global(button) {
      max-width: none;
    }
  }
</style>
