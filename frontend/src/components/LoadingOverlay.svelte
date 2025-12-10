<script>
  import { loading } from "../stores/ui.js";

  // Subscribe to loading state (Svelte 5)
  let isLoading = $derived($loading.isLoading);
  let message = $derived($loading.message);
  let progress = $derived($loading.progress);

  // Calculate progress percentage
  let progressPercent = $derived(
    progress ? Math.round((progress.current / progress.total) * 100) : 0
  );
</script>

{#if isLoading}
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-[9999] animate-fadeIn"
    role="status"
    aria-live="polite"
  >
    <div
      class="bg-white dark:bg-gray-800 p-8 rounded-xl shadow-[0_8px_32px_rgba(0,0,0,0.2)] flex flex-col items-center gap-4 min-w-[280px] max-w-[400px]"
    >
      <!-- Spinner -->
      <div
        class="w-12 h-12 border-[3px] border-gray-200 dark:border-gray-700 border-t-green-500 rounded-full animate-spin"
      ></div>

      <!-- Loading Message -->
      <p
        class="text-[0.9375rem] font-medium text-gray-700 dark:text-gray-300 m-0 text-center"
      >
        {message}
      </p>

      <!-- Progress Bar (if available) -->
      {#if progress}
        <div class="w-full mt-2">
          <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded overflow-hidden">
            <div
              class="h-full bg-gradient-to-r from-green-500 to-green-600 rounded transition-[width] duration-300"
              style="width: {progressPercent}%"
            ></div>
          </div>
          <p
            class="text-[0.8125rem] text-gray-500 dark:text-gray-400 mt-2 mb-0 text-center"
          >
            {progress.current} / {progress.total} ({progressPercent}%)
          </p>
        </div>
      {/if}
    </div>
  </div>
{/if}
