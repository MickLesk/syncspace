<script>
  import { onMount, onDestroy } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    visible = $bindable(false),
    title = "",
    icon = "",
    size = "md",
    variant = "default",
    showCloseButton = true,
    closeOnBackdrop = true,
    closeOnEscape = true,
    onclose = () => {},
    children,
    actions,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let modalElement = $state(undefined);

  const sizeClasses = {
    sm: "max-w-sm",
    md: "max-w-2xl",
    lg: "max-w-3xl",
    xl: "max-w-5xl",
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
    onclose();
  }

  function handleBackdropClick(e) {
    // Only close if clicking directly on backdrop, not on modal content
    if (e.target === e.currentTarget && showCloseButton && closeOnBackdrop) {
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

  $effect(() => {
    if (visible && modalElement) {
      const cleanup = trapFocus(modalElement);
      document.body.style.overflow = "hidden";
    } else if (!visible) {
      document.body.style.overflow = "";
    }
  });
</script>

{#if visible}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    role="dialog"
    tabindex="0"
    aria-modal="true"
    aria-labelledby={title ? "modal-title" : undefined}
    onclick={handleBackdropClick}
    onkeydown={(e = tabindex =
      "0" > e.key === "Escape" && handleBackdropClick(e))}
  >
    <!-- Enhanced backdrop with blur -->
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm animate-fadeIn z-40"
    ></div>

    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      bind:this={modalElement}
      class="relative {sizeClasses[
        size
      ]} w-full z-50 mx-4 rounded-2xl border-2 border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-2xl overflow-hidden max-h-[90vh] flex flex-col animate-slideIn"
      class:modal-primary={variant === "primary"}
      class:modal-success={variant === "success"}
      class:modal-warning={variant === "warning"}
      class:modal-danger={variant === "danger"}
      onclick={(e) => e.stopPropagation()}
      onkeydown={() => {}}
      role="document"
      tabindex="-1"
    >
      <!-- Header with gradient -->
      <div
        class="px-6 py-5 border-b-2 border-gray-200 dark:border-gray-700 flex-shrink-0 bg-gradient-to-r {variantGradients[
          variant
        ]}"
      >
        {#if showCloseButton}
          <button
            class="absolute right-4 top-4 w-10 h-10 flex items-center justify-center rounded-xl text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-white transition-all hover:rotate-90"
            aria-label="Close"
            onclick={handleClose}
            type="button"><i class="bi bi-x" aria-hidden="true"></i></button
          >
        {/if}

        <h3
          id="modal-title"
          class="font-bold text-xl flex items-center gap-3 text-gray-900 dark:text-white"
        >
          {#if icon}
            <div
              class="w-12 h-12 rounded-2xl bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center text-primary-600 dark:text-primary-400"
            >
              <i class="bi bi-{icon} text-2xl" aria-hidden="true"></i>
            </div>
          {/if}
          <span>{title}</span>
        </h3>
      </div>

      <!-- Content -->
      <div class="p-6 overflow-y-auto flex-1 text-gray-900 dark:text-gray-100">
        {@render children?.()}
      </div>

      <!-- Actions (if provided) -->
      {#if actions}
        <div
          class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50 flex justify-end gap-3"
        >
          {@render actions?.()}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  /* Material 3 Expressive Modal */
  .material-modal {
    padding: 0;
    border-radius: 1.5rem;
    border: 2px solid rgba(0, 0, 0, 0.1);
    background: white;
    box-shadow:
      0 25px 50px -12px rgba(0, 0, 0, 0.25),
      0 0 0 1px rgba(0, 0, 0, 0.05);
    overflow: hidden;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 1000;
  }

  :global(.dark) .material-modal {
    background: rgb(31 41 55);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .modal-backdrop-enhanced {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    animation: fadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 999;
  }

  .modal-header {
    padding: 1.5rem 2rem;
    border-bottom: 2px solid rgba(0, 0, 0, 0.1);
    position: relative;
    flex-shrink: 0;
  }

  :global(.dark) .modal-header {
    border-bottom-color: rgba(255, 255, 255, 0.1);
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
    color: rgba(0, 0, 0, 0.5);
    border-radius: 0.75rem;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 10;
  }

  :global(.dark) .modal-close-btn-new {
    color: rgba(255, 255, 255, 0.5);
  }

  .modal-close-btn-new:hover {
    background: rgba(0, 0, 0, 0.1);
    color: rgba(0, 0, 0, 1);
    transform: rotate(90deg) scale(1.05);
  }

  :global(.dark) .modal-close-btn-new:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 1);
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
    background: rgba(59, 130, 246, 0.15);
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgb(59, 130, 246);
  }

  .modal-content {
    padding: 2rem;
    overflow-y: auto;
    flex: 1;
    color: rgb(17 24 39);
  }

  :global(.dark) .modal-content {
    color: rgb(243 244 246);
  }

  .modal-content::-webkit-scrollbar {
    width: 8px;
  }

  .modal-content::-webkit-scrollbar-track {
    background: rgb(243 244 246);
    border-radius: 4px;
  }

  :global(.dark) .modal-content::-webkit-scrollbar-track {
    background: rgb(31 41 55);
  }

  .modal-content::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.3);
    border-radius: 4px;
  }

  :global(.dark) .modal-content::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.3);
  }

  .modal-content::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.5);
  }

  :global(.dark) .modal-content::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.5);
  }

  .modal-actions {
    padding: 1.5rem 2rem;
    border-top: 2px solid rgba(0, 0, 0, 0.1);
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
    background: rgba(243, 244, 246, 0.5);
    flex-shrink: 0;
  }

  :global(.dark) .modal-actions {
    border-top-color: rgba(255, 255, 255, 0.1);
    background: rgba(31, 41, 55, 0.5);
  }

  /* Variant-specific icon colors */
  .modal-success .modal-icon {
    background: rgba(34, 197, 94, 0.15);
    color: rgb(34, 197, 94);
  }

  .modal-warning .modal-icon {
    background: rgba(234, 179, 8, 0.15);
    color: rgb(234, 179, 8);
  }

  .modal-danger .modal-icon {
    background: rgba(239, 68, 68, 0.15);
    color: rgb(239, 68, 68);
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

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
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

    .modal-actions :global(button) {
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
