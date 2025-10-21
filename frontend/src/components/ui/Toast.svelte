<script>
  import { toasts, removeToast } from "../../stores/toast.js";

  function getIcon(type) {
    switch (type) {
      case "success":
        return "✅";
      case "error":
        return "❌";
      case "warning":
        return "⚠️";
      case "info":
        return "ℹ️";
      default:
        return "ℹ️";
    }
  }
</script>

<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <div
      class="toast toast-{toast.type}"
      on:click={() => removeToast(toast.id)}
      on:keydown={(e) => e.key === "Enter" && removeToast(toast.id)}
      role="button"
      aria-live="polite"
      tabindex="0"
    >
      <span class="toast-icon">{getIcon(toast.type)}</span>
      <span class="toast-message">{toast.message}</span>
      <button
        class="toast-close"
        on:click|stopPropagation={() => removeToast(toast.id)}
        aria-label="Schließen"
      >
        ✕
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 24px;
    right: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    z-index: 10000;
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    border-radius: 12px;
    background: var(--md-sys-color-surface);
    box-shadow: var(--md-elevation-3);
    min-width: 320px;
    max-width: 500px;
    pointer-events: auto;
    animation: slideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    cursor: pointer;
    border-left: 4px solid;
  }

  @keyframes slideIn {
    from {
      transform: translateX(400px);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  .toast-success {
    border-left-color: var(--md-sys-color-tertiary);
    background: var(--md-sys-color-tertiary-container);
    color: var(--md-sys-color-on-tertiary-container);
  }

  .toast-error {
    border-left-color: var(--md-sys-color-error);
    background: var(--md-sys-color-error-container);
    color: var(--md-sys-color-on-error-container);
  }

  .toast-warning {
    border-left-color: #f57c00;
    background: rgba(255, 152, 0, 0.15);
    color: var(--md-sys-color-on-surface);
  }

  .toast-info {
    border-left-color: var(--md-sys-color-primary);
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .toast-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .toast-message {
    flex: 1;
    font-size: 14px;
    font-weight: 500;
    line-height: 1.4;
    color: var(--md-sys-color-on-surface);
  }

  .toast-close {
    width: 24px;
    height: 24px;
    border: none;
    background: transparent;
    color: currentColor;
    cursor: pointer;
    font-size: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    opacity: 0.6;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .toast-close:hover {
    opacity: 1;
    background: rgba(0, 0, 0, 0.1);
  }

  @media (max-width: 640px) {
    .toast-container {
      left: 16px;
      right: 16px;
      bottom: 16px;
    }

    .toast {
      min-width: unset;
      width: 100%;
    }
  }
</style>
