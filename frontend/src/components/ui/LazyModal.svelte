<script>
  /**
   * LazyModal - Dynamic modal loader with Svelte 5 runes
   *
   * Usage:
   *   <LazyModal
   *     isOpen={showModal}
   *     component={BulkTaggingModal}
   *     props={{ files: selectedFiles }}
   *     on:close={() => showModal = false}
   *   />
   */

  let { isOpen = false, component, onClose, ...modalProps } = $props();

  let loadedComponent = $state(null);
  let isLoading = $state(false);
  let error = $state(null);

  // Load component when modal opens
  $effect(() => {
    if (isOpen && component && !loadedComponent) {
      isLoading = true;
      error = null;

      component()
        .then((module) => {
          loadedComponent = module.default;
          isLoading = false;
        })
        .catch((err) => {
          console.error("Failed to load modal:", err);
          error = err;
          isLoading = false;
        });
    }

    // Clean up when modal closes
    if (!isOpen) {
      // Optional: Keep loaded component in cache
      // loadedComponent = null;
    }
  });
</script>

{#if isOpen}
  {#if isLoading}
    <div class="modal modal-open">
      <div class="modal-box">
        <div class="flex items-center justify-center py-8">
          <div class="loading loading-spinner loading-lg"></div>
        </div>
      </div>
    </div>
  {:else if error}
    <div class="modal modal-open">
      <div class="modal-box">
        <div class="alert alert-error">
          <i class="bi bi-exclamation-triangle"></i>
          <span>Failed to load modal: {error.message}</span>
        </div>
        <div class="modal-action">
          <button class="btn" onclick={onClose}>Close</button>
        </div>
      </div>
    </div>
  {:else if loadedComponent}
    <svelte:component
      this={loadedComponent}
      {isOpen}
      {onClose}
      {...modalProps}
    />
  {/if}
{/if}
