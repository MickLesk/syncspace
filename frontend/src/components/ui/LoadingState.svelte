<script>
  /**
   * LoadingState Component - Pure Tailwind/DaisyUI
   * Skeleton loading states for various layouts
   */
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
  <div class="flex flex-col items-center justify-center py-12 min-h-[300px]">
    <span class="loading loading-spinner loading-lg text-primary"></span>
    {#if displayMessage}
      <p class="mt-4 text-gray-600 dark:text-gray-400">{displayMessage}</p>
    {/if}
  </div>
{:else if variant === "grid"}
  <div
    class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 animate-in fade-in duration-300"
  >
    {#each Array(count) as _, i (i)}
      <div class="card bg-base-100 shadow p-4">
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
  <div class="flex flex-col gap-2 animate-in fade-in duration-300">
    {#each Array(count) as _, i (i)}
      <div class="card bg-base-100 shadow p-3 flex-row items-center gap-4">
        <div class="skeleton w-12 h-12 rounded-full shrink-0"></div>
        <div class="flex-1 space-y-2">
          <div class="skeleton h-4 w-1/3"></div>
          <div class="skeleton h-3 w-1/2"></div>
        </div>
      </div>
    {/each}
  </div>
{:else if variant === "table"}
  <div class="card bg-base-100 shadow overflow-hidden">
    <table class="table w-full">
      <thead>
        <tr>
          <th><div class="skeleton h-4 w-24"></div></th>
          <th><div class="skeleton h-4 w-20"></div></th>
          <th><div class="skeleton h-4 w-32"></div></th>
        </tr>
      </thead>
      <tbody>
        {#each Array(count) as _, i (i)}
          <tr>
            <td><div class="skeleton h-4 w-full"></div></td>
            <td><div class="skeleton h-4 w-16"></div></td>
            <td><div class="skeleton h-4 w-24"></div></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}
