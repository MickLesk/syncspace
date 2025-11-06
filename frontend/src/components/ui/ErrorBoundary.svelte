<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { createEventDispatcher } from "svelte";

  let { error = null } = $props();

  const dispatch = createEventDispatcher();
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

{#if error}
  <div
    class="bg-red-100 dark:bg-red-900/40 border border-red-400 dark:border-red-700 text-red-800 dark:text-red-200 px-4 py-3 rounded relative mb-4"
    role="alert"
  >
    <strong class="font-bold">Error:</strong>
    <span class="block sm:inline"
      >{typeof error === "object" && error?.message
        ? error.message
        : String(error)}</span
    >
    <button
      class="absolute top-0 right-0 mt-2 mr-2 text-red-700 dark:text-red-200 hover:text-red-900"
      aria-label="Dismiss error"
      onclick={() => dispatch("dismiss")}
    >
      <i class="bi bi-x"></i>
    </button>
  </div>
{/if}
<slot />
