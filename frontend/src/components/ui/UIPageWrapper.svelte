<!-- UIPageWrapper.svelte - Consistent page layout for all views -->
<script>
  export let title = "";
  export let subtitle = "";
  export let showBackButton = false;
  export let onBack = null;
  export let actions = []; // Array of action buttons
  export let fullWidth = false;
  export let padding = true;

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  function handleBack() {
    if (onBack) {
      onBack();
    } else {
      dispatch("back");
    }
  }
</script>

<!-- Standard Page Container -->
<div
  class="min-h-screen bg-gradient-to-br from-blue-50/50 to-indigo-100/50 dark:from-slate-900 dark:to-slate-800"
>
  <div
    class="{fullWidth ? 'w-full' : 'max-w-7xl'} mx-auto {padding
      ? 'p-4 sm:p-6 lg:p-8'
      : ''}"
  >
    <!-- Page Header -->
    <div class="mb-6 sm:mb-8">
      <div class="flex items-center justify-between">
        <!-- Title Section -->
        <div class="flex items-center space-x-4">
          {#if showBackButton}
            <button
              onclick={handleBack}
              class="p-2 rounded-lg bg-white/70 dark:bg-slate-800/70 backdrop-blur-sm border border-gray-200/50 dark:border-slate-600/50 hover:bg-white dark:hover:bg-slate-700 transition-all duration-200 shadow-sm"
              aria-label="Zurück"
            >
              <i class="bi bi-arrow-left text-gray-600 dark:text-gray-300"></i>
            </button>
          {/if}

          <div>
            <h1
              class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-white"
            >
              {title}
            </h1>
            {#if subtitle}
              <p class="mt-1 text-sm text-gray-600 dark:text-gray-400">
                {subtitle}
              </p>
            {/if}
          </div>
        </div>

        <!-- Action Buttons -->
        {#if actions.length > 0}
          <div class="flex items-center space-x-2">
            {#each actions as action}
              <button
                onclick={action.onClick}
                class="flex items-center px-3 py-2 text-sm font-medium rounded-lg {action.variant ===
                'primary'
                  ? 'bg-blue-600 hover:bg-blue-700 text-white'
                  : action.variant === 'danger'
                    ? 'bg-red-600 hover:bg-red-700 text-white'
                    : 'bg-white/70 dark:bg-slate-800/70 hover:bg-white dark:hover:bg-slate-700 text-gray-700 dark:text-gray-300'} backdrop-blur-sm border {action.variant ===
                  'primary' || action.variant === 'danger'
                  ? 'border-transparent'
                  : 'border-gray-200/50 dark:border-slate-600/50'} transition-all duration-200 shadow-sm"
                disabled={action.disabled}
                aria-label={action.label}
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

    <!-- Main Content -->
    <div class="space-y-6">
      <slot />
    </div>
  </div>
</div>

<style>
  /* Consistent glassmorphism effect */
  :global(.glass-card) {
    background-color: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(4px);
    border: 1px solid rgba(229, 231, 235, 0.5);
    border-radius: 0.75rem;
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  }

  :global(.glass-card-hover) {
    background-color: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(4px);
    border: 1px solid rgba(229, 231, 235, 0.5);
    border-radius: 0.75rem;
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
    transition: all 0.2s;
  }

  :global(.glass-card-hover:hover) {
    background-color: rgba(255, 255, 255, 0.8);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  }

  /* Dark mode */
  @media (prefers-color-scheme: dark) {
    :global(.glass-card) {
      background-color: rgba(30, 41, 59, 0.7);
      border-color: rgba(71, 85, 105, 0.5);
    }

    :global(.glass-card-hover) {
      background-color: rgba(30, 41, 59, 0.7);
      border-color: rgba(71, 85, 105, 0.5);
    }

    :global(.glass-card-hover:hover) {
      background-color: rgba(30, 41, 59, 0.8);
    }
  }
</style>
