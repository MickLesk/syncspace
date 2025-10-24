<script>
  import { createEventDispatcher } from "svelte";

  export let visible = false;
  export let title = "";
  export let icon = "";
  export let size = "md"; // sm, md, lg, xl
  export let variant = "default"; // default, primary, success, warning, danger
  export let showCloseButton = true;

  const dispatch = createEventDispatcher();

  const sizeClasses = {
    sm: "max-w-sm",
    md: "max-w-2xl",
    lg: "max-w-4xl",
    xl: "max-w-6xl",
  };

  const variantGradients = {
    default: "from-primary/10 to-secondary/10",
    primary: "from-primary/20 to-accent/20",
    success: "from-success/10 to-info/10",
    warning: "from-warning/10 to-error/10",
    danger: "from-error/10 to-warning/10",
  };

  function handleClose() {
    dispatch("close");
  }

  function handleBackdropClick() {
    if (showCloseButton) {
      handleClose();
    }
  }
</script>

{#if visible}
  <dialog class="modal modal-open">
    <div
      class="modal-box {sizeClasses[size]} material-modal"
      class:modal-primary={variant === "primary"}
      class:modal-success={variant === "success"}
      class:modal-warning={variant === "warning"}
      class:modal-danger={variant === "danger"}
    >
      <!-- Header with gradient -->
      <div class="modal-header bg-gradient-to-r {variantGradients[variant]}">
        {#if showCloseButton}
          <button
            class="btn btn-sm btn-circle btn-ghost absolute right-3 top-3 hover:rotate-90 transition-transform duration-300"
            on:click={handleClose}
            aria-label="Close modal"
          >
            <i class="bi bi-x-lg text-lg"></i>
          </button>
        {/if}

        <h3 class="font-bold text-xl flex items-center gap-3">
          {#if icon}
            <div class="modal-icon">
              <i class="bi bi-{icon} text-2xl"></i>
            </div>
          {/if}
          <span>{title}</span>
        </h3>
      </div>

      <!-- Content -->
      <div class="modal-content">
        <slot />
      </div>

      <!-- Actions (if provided) -->
      {#if $$slots.actions}
        <div class="modal-actions">
          <slot name="actions" />
        </div>
      {/if}
    </div>

    <!-- Backdrop -->
    <form method="dialog" class="modal-backdrop backdrop-blur-sm">
      <button on:click={handleBackdropClick} aria-label="Close">close</button>
    </form>
  </dialog>
{/if}

<style>
  /* Material 3 Expressive Modal */
  .material-modal {
    padding: 0;
    border-radius: 28px;
    border: 1px solid hsl(var(--bc) / 0.1);
    box-shadow:
      0 8px 32px rgba(0, 0, 0, 0.12),
      0 2px 8px rgba(0, 0, 0, 0.08);
    backdrop-filter: blur(20px);
    overflow: hidden;
  }

  .modal-header {
    padding: 2rem 2rem 1.5rem;
    border-bottom: 1px solid hsl(var(--bc) / 0.08);
    position: relative;
  }

  .modal-icon {
    width: 48px;
    height: 48px;
    border-radius: 16px;
    background: hsl(var(--p) / 0.15);
    display: flex;
    align-items: center;
    justify-content: center;
    color: hsl(var(--p));
  }

  .modal-content {
    padding: 2rem;
  }

  .modal-actions {
    padding: 1.5rem 2rem;
    border-top: 1px solid hsl(var(--bc) / 0.08);
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    background: hsl(var(--b2));
  }

  /* Better text contrast in light mode */
  .modal-content :global(.text-base-content) {
    color: hsl(var(--bc)) !important;
  }

  .modal-content :global(.alert) {
    color: hsl(var(--bc));
  }

  /* Variant-specific icon colors */
  .modal-success .modal-icon {
    background: hsl(var(--su) / 0.15);
    color: hsl(var(--su));
  }

  .modal-warning .modal-icon {
    background: hsl(var(--wa) / 0.15);
    color: hsl(var(--wa));
  }

  .modal-danger .modal-icon {
    background: hsl(var(--er) / 0.15);
    color: hsl(var(--er));
  }

  /* Backdrop blur effect */
  .modal-backdrop {
    background: rgba(0, 0, 0, 0.5);
  }

  /* Close button hover effect */
  .btn-circle:hover {
    background: hsl(var(--bc) / 0.1);
  }

  /* Smooth animations */
  .material-modal {
    animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes modalSlideIn {
    from {
      opacity: 0;
      transform: translateY(-20px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Responsive */
  @media (max-width: 768px) {
    .modal-header {
      padding: 1.5rem 1.5rem 1rem;
    }

    .modal-content {
      padding: 1.5rem;
    }

    .modal-actions {
      padding: 1rem 1.5rem;
      flex-direction: column-reverse;
    }

    .modal-actions :global(.btn) {
      width: 100%;
    }
  }
</style>
