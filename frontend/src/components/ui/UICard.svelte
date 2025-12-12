<!-- UICard.svelte - Consistent glass morphism cards -->
<script>
  export let title = "";
  export let subtitle = "";
  export let icon = "";
  export let padding = "p-6";
  export let hoverable = false;
  export let clickable = false;
  export let loading = false;
  export let error = "";

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  function handleClick() {
    if (clickable) {
      dispatch("click");
    }
  }
</script>

<!-- Use svelte:element to conditionally render button or div -->
<svelte:element
  this={clickable ? "button" : "div"}
  class="glass-card {hoverable ? 'glass-card-hover' : ''} {clickable
    ? 'cursor-pointer bg-transparent border-none w-full text-left'
    : ''} {padding} relative overflow-hidden"
  role={clickable ? "button" : undefined}
  onclick={clickable ? handleClick : undefined}
  onkeydown={clickable
    ? (e) => (e.key === "Enter" || e.key === " ") && handleClick()
    : undefined}
>
  <!-- Loading Overlay -->
  {#if loading}
    <div
      class="absolute inset-0 bg-white/50 dark:bg-slate-800/50 backdrop-blur-sm flex items-center justify-center z-10"
    >
      <div
        class="animate-spin rounded-full h-8 w-8 border-2 border-blue-600 border-t-transparent"
      ></div>
    </div>
  {/if}

  <!-- Error State -->
  {#if error}
    <div
      class="mb-4 p-3 rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800"
    >
      <div class="flex items-center">
        <i
          class="bi bi-exclamation-triangle text-red-600 dark:text-red-400 mr-2"
          aria-hidden="true"
        ></i>
        <span class="text-sm text-red-700 dark:text-red-300">{error}</span>
      </div>
    </div>
  {/if}

  <!-- Header -->
  {#if title || subtitle || icon}
    <div class="mb-4">
      <div class="flex items-center space-x-3">
        {#if icon}
          <div class="flex-shrink-0">
            <i
              class="bi bi-{icon} text-xl text-blue-600 dark:text-blue-400"
              aria-hidden="true"
            ></i>
          </div>
        {/if}

        <div class="flex-1 min-w-0">
          {#if title}
            <h3
              class="text-lg font-semibold text-gray-900 dark:text-white truncate"
            >
              {title}
            </h3>
          {/if}
          {#if subtitle}
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
              {subtitle}
            </p>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- Main Content -->
  <div class="relative">
    <slot />
  </div>
</svelte:element>
