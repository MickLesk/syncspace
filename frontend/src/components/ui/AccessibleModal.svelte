<script>
  /**
   * Accessible Modal Component
   * Implements WCAG 2.1 AA compliance with focus trapping and ARIA attributes
   */

  import { onMount, onDestroy } from "svelte";
  import { createFocusTrap } from "../../lib/keyboardNavigation.js";

  let {
    title = "Modal",
    subtitle = null,
    isOpen = false,
    onClose = null,
    onConfirm = null,
    confirmText = "Confirm",
    cancelText = "Cancel",
    isDangerous = false,
    showFooter = true,
    class: className = "",
    children,
  } = $props();

  let modalRef = $state(null);
  let focusTrap = $state(null);

  onMount(() => {
    if (modalRef) {
      focusTrap = createFocusTrap(modalRef);
      if (isOpen) {
        focusTrap.activate();
      }
    }

    // Handle ESC key
    function handleEscape(e) {
      if (e.key === "Escape" && isOpen) {
        onClose?.();
      }
    }

    document.addEventListener("keydown", handleEscape);

    return () => {
      document.removeEventListener("keydown", handleEscape);
      focusTrap?.deactivate();
    };
  });

  // React to isOpen changes
  $effect(() => {
    if (isOpen && focusTrap) {
      focusTrap.activate();
    } else if (focusTrap) {
      focusTrap.deactivate();
    }
  });

  function handleBackdropClick(e) {
    if (e.target === e.currentTarget) {
      onClose?.();
    }
  }
</script>

{#if isOpen}
  <!-- Backdrop with ARIA -->
  <div
    class="fixed inset-0 bg-black bg-opacity-50 z-40 transition-opacity duration-200"
    role="presentation"
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === "Escape" && onClose?.()}
  ></div>

  <!-- Modal -->
  <div
    bind:this={modalRef}
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-title"
    aria-describedby={subtitle ? "modal-description" : null}
    tabindex="0"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-md w-full max-h-screen overflow-y-auto {className}"
      role="document"
    >
      <!-- Header with ARIA attributes -->
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h2
          id="modal-title"
          class="text-xl font-bold text-gray-900 dark:text-gray-100"
        >
          {title}
        </h2>
        {#if subtitle}
          <p
            id="modal-description"
            class="mt-1 text-sm text-gray-600 dark:text-gray-400"
          >
            {subtitle}
          </p>
        {/if}
      </div>

      <!-- Content -->
      <div class="px-6 py-4">
        {@render children?.()}
      </div>

      <!-- Footer with accessible buttons -->
      {#if showFooter}
        <div
          class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex gap-2 justify-end"
        >
          <button
            type="button"
            class="px-4 py-2 rounded-lg border-2 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            onclick={onClose}
            aria-label="Cancel"
          >
            {cancelText}
          </button>

          {#if onConfirm}
            <button
              type="button"
              class="px-4 py-2 rounded-lg transition-colors"
              class:bg-green-500={!isDangerous}
              class:hover:bg-green-600={!isDangerous}
              class:bg-red-500={isDangerous}
              class:hover:bg-red-600={isDangerous}
              class:text-white={true}
              onclick={onConfirm}
              aria-label={confirmText}
              aria-describedby={isDangerous ? "danger-warning" : null}
            >
              {confirmText}
            </button>
          {/if}
        </div>

        {#if isDangerous}
          <div
            id="danger-warning"
            class="px-6 py-2 bg-red-50 dark:bg-red-900/30 border-t border-red-200 dark:border-red-800"
            role="alert"
          >
            <p class="text-sm text-red-600 dark:text-red-300">
              ⚠️ This action cannot be undone.
            </p>
          </div>
        {/if}
      {/if}
    </div>
  </div>
{/if}

<style>
  :global([role="dialog"]) {
    outline: none;
  }

  :global([role="dialog"]:focus-visible) {
    outline: 2px solid #3b82f6;
    outline-offset: -2px;
  }

  button:focus-visible {
    outline: 2px solid #3b82f6;
    outline-offset: 2px;
  }
</style>
