<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    variant = "grid", // grid, list, table, spinner
    count = 6,
    message = "",
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  const displayMessage = $derived(message || tr("loading"));
</script>

{#if variant === "spinner"}
  <div
    class="loading-spinner-container flex flex-col items-center justify-center py-12"
  >
    <div
      class="loading loading-spinner loading-lg text-blue-500 dark:text-blue-400"
    ></div>
    {#if displayMessage}
      <p class="mt-4 text-gray-600 dark:text-gray-400">{displayMessage}</p>
    {/if}
  </div>
{:else if variant === "grid"}
  <div
    class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
  >
    {#each Array(count) as _, i (i)}
      <div
        class="skeleton-card bg-white dark:bg-gray-800 rounded-lg shadow p-4"
      >
        <div class="flex flex-col items-center gap-3">
          <div class="skeleton w-16 h-16 rounded-full"></div>
          <div class="w-full space-y-2">
            <div class="skeleton h-4 w-3/4 mx-auto"></div>
            <div class="skeleton h-3 w-1/2 mx-auto"></div>
          </div>
        </div>
      </div>
    {/each}
  </div>
{:else if variant === "list"}
  <div class="flex flex-col gap-2">
    {#each Array(count) as _, i (i)}
      <div
        class="skeleton-list-item bg-white dark:bg-gray-800 rounded-lg shadow p-3 flex items-center gap-4"
      >
        <div class="skeleton w-12 h-12 rounded-full flex-shrink-0"></div>
        <div class="flex-1 space-y-2">
          <div class="skeleton h-4 w-1/3"></div>
          <div class="skeleton h-3 w-1/2"></div>
        </div>
      </div>
    {/each}
  </div>
{:else if variant === "table"}
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
    <table class="w-full">
      <thead class="bg-gray-50 dark:bg-gray-700">
        <tr>
          <th class="px-4 py-3 text-left">
            <div class="skeleton h-4 w-24"></div>
          </th>
          <th class="px-4 py-3 text-left">
            <div class="skeleton h-4 w-20"></div>
          </th>
          <th class="px-4 py-3 text-left">
            <div class="skeleton h-4 w-32"></div>
          </th>
        </tr>
      </thead>
      <tbody>
        {#each Array(count) as _, i (i)}
          <tr class="border-t border-gray-200 dark:border-gray-700">
            <td class="px-4 py-3">
              <div class="skeleton h-4 w-full"></div>
            </td>
            <td class="px-4 py-3">
              <div class="skeleton h-4 w-16"></div>
            </td>
            <td class="px-4 py-3">
              <div class="skeleton h-4 w-24"></div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}

<style>
  .skeleton {
    background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: 0.25rem;
  }

  :global([data-theme="dark"]) .skeleton {
    background: linear-gradient(90deg, #374151 25%, #4b5563 50%, #374151 75%);
    background-size: 200% 100%;
  }

  @keyframes shimmer {
    0% {
      background-position: -200% 0;
    }
    100% {
      background-position: 200% 0;
    }
  }

  .skeleton-card,
  .skeleton-list-item {
    animation: fadeIn 0.3s ease-in;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .loading-spinner-container {
    min-height: 300px;
  }
</style>
