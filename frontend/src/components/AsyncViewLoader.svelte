<script>
  /**
   * Async View Loader Component
   * Shows loading state while async component loads
   *
   * Usage:
   * <AsyncViewLoader component={FilesView} let:Component>
   *   <svelte:component this={Component} />
   * </AsyncViewLoader>
   */

  import { onMount } from "svelte";

  let {
    component,
    fallback = null,
    error: errorComponent = null,
    ...restProps
  } = $props();

  let resolvedComponent = $state(null);
  let isLoading = $state(true);
  let error = $state(null);
  let loadTime = $state(0);

  onMount(async () => {
    const startTime = performance.now();

    try {
      // If component is already resolved (imported statically)
      if (component && !component.then) {
        resolvedComponent = component;
        isLoading = false;
      } else if (component && typeof component.then === "function") {
        // Component is a Promise (async import)
        resolvedComponent = await component;
        isLoading = false;
      } else {
        throw new Error("Invalid component");
      }

      loadTime = performance.now() - startTime;
      console.debug(`Component loaded in ${loadTime.toFixed(0)}ms`);
    } catch (err) {
      console.error("Failed to load async component:", err);
      error = err;
      isLoading = false;
    }
  });
</script>

{#if isLoading}
  {#if fallback}
    {@const FallbackComponent = fallback}
    <FallbackComponent />
  {:else}
    <div class="flex items-center justify-center h-full min-h-96">
      <div class="text-center">
        <div
          class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-500 mx-auto mb-4"
        ></div>
        <p class="text-gray-600 dark:text-gray-400">Loading...</p>
      </div>
    </div>
  {/if}
{:else if error}
  {#if errorComponent}
    {@const ErrorComp = errorComponent}
    <ErrorComp {error} />
  {:else}
    <div class="flex items-center justify-center h-full min-h-96">
      <div class="text-center text-red-600 dark:text-red-400">
        <i
          class="bi bi-exclamation-triangle-fill text-4xl mb-4"
          aria-hidden="true"
        ></i>
        <p class="font-semibold">Failed to load component</p>
        <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
          {error.message}
        </p>
        <button
          onclick={() => window.location.reload()}
          class="mt-4 px-4 py-2 bg-primary-500 text-white rounded hover:bg-primary-600"
        >
          Reload Page
        </button>
      </div>
    </div>
  {/if}
{:else if resolvedComponent}
  {@const ResolvedComp = resolvedComponent}
  <ResolvedComp {...restProps} />
{/if}
