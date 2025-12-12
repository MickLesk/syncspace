<!-- UIModal.svelte - Consistent modal dialogs -->
<script>
  export let show = false;
  export let title = "";
  export let size = "md"; // 'sm', 'md', 'lg', 'xl', 'full'
  export let closable = true;
  export let actions = []; // Array of action buttons
  export let loading = false;

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  function close() {
    if (closable && !loading) {
      show = false;
      dispatch("close");
    }
  }

  function handleKeydown(event) {
    if (event.key === "Escape" && closable && !loading) {
      close();
    }
  }

  // Size classes
  const sizeClasses = {
    sm: "max-w-md",
    md: "max-w-lg",
    lg: "max-w-2xl",
    xl: "max-w-4xl",
    full: "max-w-7xl mx-4",
  };
</script>

{#if show}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    onclick={close}
    onkeydown={handleKeydown}
    role="button"
    tabindex="-1"
    aria-label="Modal schließen"
  >
    <!-- Modal Container -->
    <div
      class="w-full {sizeClasses[
        size
      ]} bg-white dark:bg-slate-800 rounded-xl shadow-2xl border border-gray-200/50 dark:border-slate-600/50 backdrop-blur-sm max-h-[90vh] flex flex-col"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.key === "Escape" && close()}
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby={title ? "modal-title" : undefined}
    >
      <!-- Loading Overlay -->
      {#if loading}
        <div
          class="absolute inset-0 bg-white/70 dark:bg-slate-800/70 backdrop-blur-sm flex items-center justify-center z-10 rounded-xl"
        >
          <div class="flex items-center space-x-3">
            <div
              class="animate-spin rounded-full h-6 w-6 border-2 border-blue-600 border-t-transparent"
            ></div>
            <span class="text-sm text-gray-600 dark:text-gray-400"
              >Laden...</span
            >
          </div>
        </div>
      {/if}

      <!-- Header -->
      {#if title || closable}
        <div
          class="flex items-center justify-between p-6 border-b border-gray-200/50 dark:border-slate-600/50"
        >
          <div class="flex-1">
            {#if title}
              <h2
                id="modal-title"
                class="text-xl font-semibold text-gray-900 dark:text-white"
              >
                {title}
              </h2>
            {/if}
          </div>

          {#if closable}
            <button
              onclick={close}
              class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors duration-200"
              aria-label="Modal schließen"
              disabled={loading}
            >
              <i
                class="bi bi-x text-xl text-gray-500 dark:text-gray-400"
                aria-hidden="true"
              ></i>
            </button>
          {/if}
        </div>
      {/if}

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6">
        <slot />
      </div>

      <!-- Footer Actions -->
      {#if actions.length > 0}
        <div
          class="flex items-center justify-end space-x-3 p-6 border-t border-gray-200/50 dark:border-slate-600/50"
        >
          {#each actions as action}
            <button
              onclick={action.onClick}
              disabled={action.disabled || loading}
              class="flex items-center px-4 py-2 text-sm font-medium rounded-lg transition-all duration-200 {action.variant ===
              'primary'
                ? 'bg-blue-600 hover:bg-blue-700 text-white disabled:bg-blue-400'
                : action.variant === 'danger'
                  ? 'bg-red-600 hover:bg-red-700 text-white disabled:bg-red-400'
                  : 'bg-gray-100 dark:bg-slate-700 hover:bg-gray-200 dark:hover:bg-slate-600 text-gray-700 dark:text-gray-300 disabled:bg-gray-50 dark:disabled:bg-slate-800'} disabled:cursor-not-allowed"
            >
              {#if action.icon}
                <i
                  class="bi bi-{action.icon} {action.label ? 'mr-2' : ''}"
                  aria-hidden="true"
                ></i>
              {/if}
              {action.label || ""}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}
