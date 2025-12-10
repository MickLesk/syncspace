<script>
  /**
   * ConfirmDialog Component
   * Styled confirmation dialog als Ersatz für native confirm()
   * Pure Tailwind CSS - no custom styles
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
   *   onconfirm={handleDelete}
   *   oncancel={handleCancel}
   * />
   */

  import Icon from "./Icon.svelte";
  import ModernButton from "./ModernButton.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

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
    content,
    onconfirm = () => {},
    oncancel = () => {},
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
    onconfirm();
  }

  function handleCancel() {
    if (loading) return;
    open = false;
    oncancel();
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
    class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-[9999] p-5 animate-fade-in"
    onclick={handleBackdropClick}
    onkeydown={handleKeydown}
    role="presentation"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-3xl p-8 max-w-lg w-full shadow-2xl flex flex-col items-center gap-6 animate-slide-up"
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeydown}
      role="alertdialog"
      aria-labelledby="confirm-dialog-title"
      aria-describedby="confirm-dialog-message"
      aria-modal="true"
      tabindex="-1"
    >
      <!-- Icon Circle -->
      <div
        class="w-20 h-20 rounded-full flex items-center justify-center shrink-0 {variant ===
        'danger'
          ? 'bg-red-500/10 text-red-500'
          : variant === 'warning'
            ? 'bg-amber-500/10 text-amber-500'
            : variant === 'success'
              ? 'bg-green-500/10 text-green-500'
              : 'bg-primary-500/10 text-primary-500'}"
      >
        <Icon name={displayIcon} size={32} />
      </div>

      <!-- Content -->
      <div class="text-center w-full">
        <h2
          id="confirm-dialog-title"
          class="text-2xl font-semibold text-gray-900 dark:text-white mb-3"
        >
          {defaultTitle}
        </h2>
        <p
          id="confirm-dialog-message"
          class="text-base leading-relaxed text-gray-600 dark:text-gray-400"
        >
          {defaultMessage}
        </p>

        {@render content?.()}
      </div>

      <!-- Actions -->
      <div
        class="flex gap-3 w-full justify-end max-sm:flex-col-reverse max-sm:gap-2"
      >
        <ModernButton
          onclick={handleCancel}
          variant="secondary"
          size="md"
          disabled={loading}
          class="flex-1 max-w-[150px] max-sm:max-w-none"
        >
          {defaultCancelText}
        </ModernButton>
        <ModernButton
          onclick={handleConfirm}
          variant={variant === "danger"
            ? "danger"
            : variant === "success"
              ? "success"
              : "primary"}
          size="md"
          disabled={loading}
          {loading}
          class="flex-1 max-w-[150px] max-sm:max-w-none"
        >
          {defaultConfirmText}
        </ModernButton>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slide-up {
    from {
      opacity: 0;
      transform: translateY(30px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .animate-fade-in {
    animation: fade-in 0.2s ease-out;
  }

  .animate-slide-up {
    animation: slide-up 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }
</style>
