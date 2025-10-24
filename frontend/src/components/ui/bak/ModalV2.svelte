<!--
  ModalV2.svelte - Modern Modal Component using DaisyUI
  
  A flexible modal dialog with animations, keyboard shortcuts, and focus management.
  
  Props:
  - open: boolean - Control modal visibility (default: false)
  - size: "sm" | "md" | "lg" | "xl" | "full" (default: "md")
  - closable: boolean - Show close button (default: true)
  - backdrop: boolean - Show backdrop overlay (default: true)
  - backdropBlur: boolean - Blur backdrop (default: true)
  - closeOnBackdrop: boolean - Close when clicking backdrop (default: true)
  - closeOnEsc: boolean - Close on ESC key (default: true)
  - title: string - Modal title
  - preventScroll: boolean - Prevent body scroll when open (default: true)
  
  Slots:
  - header: Custom header content
  - title: Modal title content
  - default: Main modal content
  - actions: Footer action buttons
  
  Events:
  - on:open - Fired when modal opens
  - on:close - Fired when modal closes
  - on:backdropClick - Fired when backdrop is clicked
  
  Usage:
  <ModalV2 bind:open={showModal} title="Confirm Action">
    <p>Are you sure you want to continue?</p>
    <svelte:fragment slot="actions">
      <ButtonV2 variant="ghost" on:click={() => showModal = false}>Cancel</ButtonV2>
      <ButtonV2 variant="primary">Confirm</ButtonV2>
    </svelte:fragment>
  </ModalV2>
-->

<script>
  import { onMount, onDestroy } from "svelte";
  import { fade, scale } from "svelte/transition";
  import { quintOut } from "svelte/easing";

  export let open = false;
  export let size = "md"; // sm, md, lg, xl, full
  export let closable = true;
  export let backdrop = true;
  export let backdropBlur = true;
  export let closeOnBackdrop = true;
  export let closeOnEsc = true;
  export let title = "";
  export let preventScroll = true;

  let modalElement;
  let previousFocus;

  $: sizeClasses = {
    sm: "modal-sm",
    md: "modal-md",
    lg: "modal-lg",
    xl: "modal-xl",
    full: "modal-full",
  };

  $: computedClass = [
    "modal-box",
    sizeClasses[size],
    backdropBlur ? "backdrop-blur" : "",
  ]
    .filter(Boolean)
    .join(" ");

  // Handle modal open/close
  $: if (open) {
    handleOpen();
  } else {
    handleClose();
  }

  function handleOpen() {
    // Store current focus
    previousFocus = document.activeElement;

    // Prevent body scroll
    if (preventScroll) {
      document.body.style.overflow = "hidden";
    }

    // Dispatch open event
    setTimeout(() => {
      modalElement?.dispatchEvent(new CustomEvent("open", { bubbles: true }));
      // Focus first focusable element
      focusFirstElement();
    }, 100);
  }

  function handleClose() {
    // Restore body scroll
    if (preventScroll) {
      document.body.style.overflow = "";
    }

    // Dispatch close event
    modalElement?.dispatchEvent(new CustomEvent("close", { bubbles: true }));

    // Restore previous focus
    previousFocus?.focus();
  }

  function closeModal() {
    open = false;
  }

  function handleBackdropClick() {
    if (closeOnBackdrop) {
      modalElement?.dispatchEvent(
        new CustomEvent("backdropClick", { bubbles: true })
      );
      closeModal();
    }
  }

  function handleEscape(e) {
    if (open && closeOnEsc && e.key === "Escape") {
      e.preventDefault();
      closeModal();
    }
  }

  function focusFirstElement() {
    const focusable = modalElement?.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    focusable?.[0]?.focus();
  }

  function trapFocus(e) {
    if (!open || !modalElement) return;

    const focusable = Array.from(
      modalElement.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      )
    );

    const firstFocusable = focusable[0];
    const lastFocusable = focusable[focusable.length - 1];

    if (e.key === "Tab") {
      if (e.shiftKey) {
        // Shift + Tab
        if (document.activeElement === firstFocusable) {
          e.preventDefault();
          lastFocusable?.focus();
        }
      } else {
        // Tab
        if (document.activeElement === lastFocusable) {
          e.preventDefault();
          firstFocusable?.focus();
        }
      }
    }
  }

  onMount(() => {
    document.addEventListener("keydown", handleEscape);
    document.addEventListener("keydown", trapFocus);
  });

  onDestroy(() => {
    document.removeEventListener("keydown", handleEscape);
    document.removeEventListener("keydown", trapFocus);
    // Clean up body scroll
    if (preventScroll) {
      document.body.style.overflow = "";
    }
  });
</script>

{#if open}
  <div class="modal modal-open">
    <!-- Backdrop -->
    {#if backdrop}
      <div
        class="modal-backdrop"
        class:backdrop-blur={backdropBlur}
        on:click={handleBackdropClick}
        on:keydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            handleBackdropClick();
          }
        }}
        transition:fade={{ duration: 200 }}
        role="button"
        tabindex="-1"
      ></div>
    {/if}

    <!-- Modal Box -->
    <div
      class={computedClass}
      bind:this={modalElement}
      transition:scale={{
        duration: 300,
        easing: quintOut,
        start: 0.9,
        opacity: 0,
      }}
      role="dialog"
      aria-modal="true"
      aria-labelledby={title ? "modal-title" : undefined}
    >
      <!-- Close Button -->
      {#if closable}
        <button
          class="btn btn-sm btn-circle btn-ghost absolute right-4 top-4 z-10"
          on:click={closeModal}
          aria-label="Close modal"
        >
          ✕
        </button>
      {/if}

      <!-- Custom Header or Title -->
      {#if $$slots.header}
        <div class="modal-header">
          <slot name="header" />
        </div>
      {:else if $$slots.title || title}
        <h3 id="modal-title" class="modal-title">
          {#if $$slots.title}
            <slot name="title" />
          {:else}
            {title}
          {/if}
        </h3>
      {/if}

      <!-- Modal Content -->
      <div class="modal-content">
        <slot />
      </div>

      <!-- Actions Footer -->
      {#if $$slots.actions}
        <div class="modal-actions">
          <slot name="actions" />
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  /* Modal Overlay */
  .modal {
    position: fixed;
    inset: 0;
    z-index: var(--z-modal);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-4);
  }

  /* Backdrop */
  .modal-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    cursor: pointer;
  }

  .modal-backdrop.backdrop-blur {
    backdrop-filter: blur(4px);
  }

  /* Modal Box Sizes */
  .modal-box {
    position: relative;
    z-index: calc(var(--z-modal) + 1);
    max-height: calc(100vh - var(--spacing-8));
    overflow-y: auto;
    background: var(--color-surface);
    border-radius: var(--radius-2xl);
    box-shadow: var(--shadow-2xl);
    padding: var(--spacing-6);
  }

  .modal-sm {
    width: 100%;
    max-width: 400px;
  }

  .modal-md {
    width: 100%;
    max-width: 600px;
  }

  .modal-lg {
    width: 100%;
    max-width: 800px;
  }

  .modal-xl {
    width: 100%;
    max-width: 1200px;
  }

  .modal-full {
    width: calc(100vw - var(--spacing-8));
    max-width: none;
    height: calc(100vh - var(--spacing-8));
    max-height: none;
  }

  /* Modal Header */
  .modal-header {
    margin-bottom: var(--spacing-4);
    padding-bottom: var(--spacing-3);
    border-bottom: 1px solid var(--color-outline);
  }

  /* Modal Title */
  .modal-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-on-surface);
    margin-bottom: var(--spacing-4);
    padding-right: var(--spacing-10); /* Space for close button */
  }

  /* Modal Content */
  .modal-content {
    flex: 1;
    color: var(--color-on-surface-variant);
    font-size: var(--font-size-base);
    line-height: var(--line-height-relaxed);
  }

  /* Modal Actions */
  .modal-actions {
    display: flex;
    gap: var(--spacing-3);
    margin-top: var(--spacing-6);
    padding-top: var(--spacing-4);
    border-top: 1px solid var(--color-outline);
    justify-content: flex-end;
  }

  /* Scrollbar Styling */
  .modal-box::-webkit-scrollbar {
    width: 8px;
  }

  .modal-box::-webkit-scrollbar-track {
    background: var(--color-surface-container-low);
    border-radius: var(--radius-full);
  }

  .modal-box::-webkit-scrollbar-thumb {
    background: var(--color-outline);
    border-radius: var(--radius-full);
  }

  .modal-box::-webkit-scrollbar-thumb:hover {
    background: var(--color-primary);
  }

  /* Responsive */
  @media (max-width: 768px) {
    .modal {
      padding: var(--spacing-2);
    }

    .modal-box {
      padding: var(--spacing-4);
      max-height: calc(100vh - var(--spacing-4));
    }

    .modal-sm,
    .modal-md,
    .modal-lg,
    .modal-xl {
      max-width: 100%;
    }

    .modal-full {
      width: 100vw;
      height: 100vh;
      max-height: 100vh;
      border-radius: 0;
    }

    .modal-actions {
      flex-direction: column;
    }

    .modal-actions > :global(*) {
      width: 100%;
    }
  }

  /* Animation Enhancement */
  .modal-box {
    animation: modalSlideIn var(--duration-300) var(--ease-emphasized);
  }

  @keyframes modalSlideIn {
    from {
      opacity: 0;
      transform: scale(0.9) translateY(20px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }
</style>
