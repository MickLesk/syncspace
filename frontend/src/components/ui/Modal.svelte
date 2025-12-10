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
    tabindex="-1"
    aria-modal="true"
    aria-labelledby={title ? "modal-title" : undefined}
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === "Escape" && handleClose()}
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
      ]} w-full z-50 mx-4 rounded-xl border-2 border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-2xl overflow-hidden max-h-[90vh] flex flex-col animate-slideIn"
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
              class="w-12 h-12 rounded-xl bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center text-primary-600 dark:text-primary-400"
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
