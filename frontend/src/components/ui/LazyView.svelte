<script>
  /**
   * LazyView Component - Wrapper for dynamic imports with loading state
   *
   * Usage:
   *   <LazyView component={FilesView} />
   *
   * Where FilesView = () => import("./pages/files/FilesView.svelte")
   */

  let { component, ...props } = $props();

  let loadedComponent = $state(null);
  let isLoading = $state(true);
  let error = $state(null);

  // Load component dynamically
  $effect(() => {
    isLoading = true;
    error = null;
    loadedComponent = null;

    if (component) {
      component()
        .then((module) => {
          loadedComponent = module.default;
          isLoading = false;
        })
        .catch((err) => {
          console.error("Failed to load component:", err);
          error = err;
          isLoading = false;
        });
    }
  });
</script>

{#if isLoading}
  <div class="flex items-center justify-center h-full">
    <div class="text-center">
      <div class="loading loading-spinner loading-lg"></div>
      <p class="mt-4 text-sm opacity-70">Loading...</p>
    </div>
  </div>
{:else if error}
  <div class="flex items-center justify-center h-full">
    <div class="alert alert-error">
      <i class="bi bi-exclamation-triangle"></i>
      <span>Failed to load view: {error.message}</span>
    </div>
  </div>
{:else if loadedComponent}
  <svelte:component this={loadedComponent} {...props} />
{/if}
