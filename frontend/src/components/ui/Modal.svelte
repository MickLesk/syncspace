<script>
  import { createEventDispatcher, onMount, onDestroy } from "svelte";

  export let visible = false;
  export let title = "";
  export let icon = "";
  export let size = "md"; // sm, md, lg, xl, full
  export let variant = "default"; // default, primary, success, warning, danger
  export let showCloseButton = true;
  export let closeOnBackdrop = true;
  export let closeOnEscape = true;

  const dispatch = createEventDispatcher();

  let modalElement;

  const sizeClasses = {
    sm: "max-w-sm",
    md: "max-w-2xl",
    lg: "max-w-4xl",
    xl: "max-w-6xl",
    full: "w-11/12 max-w-7xl",
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
    if (showCloseButton && closeOnBackdrop) {
      handleClose();
    }
  }

  function handleEscape(e) {
    if (closeOnEscape && e.key === "Escape" && visible) {
      handleClose();
    }
  }

  // Focus trap
  function trapFocus(element) {
    const focusableElements = element.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    const firstFocusable = focusableElements[0];
    const lastFocusable = focusableElements[focusableElements.length - 1];

    function handleTab(e) {
      if (e.key === "Tab") {
        if (e.shiftKey) {
          if (document.activeElement === firstFocusable) {
            lastFocusable?.focus();
            e.preventDefault();
          }
        } else {
          if (document.activeElement === lastFocusable) {
            firstFocusable?.focus();
            e.preventDefault();
          }
        }
      }
    }

    element.addEventListener("keydown", handleTab);
    firstFocusable?.focus();

    return () => {
      element.removeEventListener("keydown", handleTab);
    };
  }

  onMount(() => {
    window.addEventListener("keydown", handleEscape);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleEscape);
    document.body.style.overflow = "";
  });

  $: if (visible && modalElement) {
    const cleanup = trapFocus(modalElement);
    document.body.style.overflow = "hidden";
  } else if (!visible) {
    document.body.style.overflow = "";
  }
</script>

{#if visible}
  <dialog
    class="modal modal-open"
    role="dialog"
    aria-modal="true"
    aria-labelledby={title ? "modal-title" : undefined}
  >
    <!-- Enhanced backdrop with blur -->
    <div
      class="modal-backdrop-enhanced"
      on:click={handleBackdropClick}
      on:keydown={(e) => e.key === "Enter" && handleBackdropClick()}
      role="button"
      tabindex="-1"
      aria-label="Close"
    ></div>

    <div
      bind:this={modalElement}
      class="modal-box {sizeClasses[size]} material-modal"
      class:modal-primary={variant === "primary"}
      class:modal-success={variant === "success"}
      class:modal-warning={variant === "warning"}
      class:modal-danger={variant === "danger"}
      on:click={(e) => e.stopPropagation()}
      on:keydown={() => {}}
      role="document"
      tabindex="-1"
    >
      <!-- Header with gradient -->
      <div class="modal-header bg-gradient-to-r {variantGradients[variant]}">
        {#if showCloseButton}
          <button
            class="modal-close-btn-new"
            on:click={handleClose}
            aria-label="Close modal"
            type="button"
          >
            <i class="bi bi-x-lg"></i>
          </button>
        {/if}

        <h3 id="modal-title" class="font-bold text-xl flex items-center gap-3">
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
  </dialog>
{/if}

<style>
  /* Material 3 Expressive Modal */
  .material-modal {
    padding: 0;
    border-radius: 1.5rem;
    border: 2px solid hsl(var(--bc) / 0.1);
    box-shadow:
      0 25px 50px -12px rgba(0, 0, 0, 0.25),
      0 0 0 1px rgba(0, 0, 0, 0.05);
    backdrop-filter: blur(20px);
    overflow: hidden;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .modal-backdrop-enhanced {
    position: fixed;
    inset: 0;
    background: hsl(var(--b1) / 0.7);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    animation: fadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 999;
  }

  .modal-header {
    padding: 1.5rem 2rem;
    border-bottom: 2px solid hsl(var(--bc) / 0.1);
    position: relative;
    flex-shrink: 0;
  }

  .modal-close-btn-new {
    position: absolute;
    right: 1.5rem;
    top: 1.5rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    padding: 0;
    border: none;
    background: transparent;
    color: hsl(var(--bc) / 0.5);
    border-radius: 0.75rem;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .modal-close-btn-new:hover {
    background: hsl(var(--bc) / 0.1);
    color: hsl(var(--bc));
    transform: rotate(90deg) scale(1.05);
  }

  .modal-close-btn-new:active {
    transform: rotate(90deg) scale(0.95);
  }

  .modal-close-btn-new i {
    font-size: 1.25rem;
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
    overflow-y: auto;
    flex: 1;
  }

  .modal-content::-webkit-scrollbar {
    width: 8px;
  }

  .modal-content::-webkit-scrollbar-track {
    background: hsl(var(--b2));
    border-radius: 4px;
  }

  .modal-content::-webkit-scrollbar-thumb {
    background: hsl(var(--bc) / 0.3);
    border-radius: 4px;
  }

  .modal-content::-webkit-scrollbar-thumb:hover {
    background: hsl(var(--bc) / 0.5);
  }

  .modal-actions {
    padding: 1.5rem 2rem;
    border-top: 2px solid hsl(var(--bc) / 0.1);
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
    background: hsl(var(--b2) / 0.5);
    flex-shrink: 0;
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

  /* Smooth animations */
  @keyframes modalSlideIn {
    from {
      opacity: 0;
      transform: translateY(-20px) scale(0.98);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Responsive */
  @media (max-width: 768px) {
    .material-modal {
      max-width: 95% !important;
      max-height: 95vh;
      border-radius: 1rem;
    }

    .modal-header {
      padding: 1rem 1.25rem;
    }

    .modal-content {
      padding: 1.25rem;
    }

    .modal-actions {
      padding: 1rem 1.25rem;
      flex-direction: column-reverse;
    }

    .modal-actions :global(.btn) {
      width: 100%;
    }

    .modal-close-btn-new {
      right: 1rem;
      top: 1rem;
      width: 2rem;
      height: 2rem;
    }

    .modal-close-btn-new i {
      font-size: 1rem;
    }
  }
</style>
