<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let { x = 0, y = 0, items = [], onClose = () => {} } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  function handleClick(item) {
    if (item.action) item.action();
    onClose();
  }
</script>

{#if items.length > 0}
  <div
    class="fixed z-[1000] min-w-[200px] bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-xl p-2 animate-scale-in"
    style="left: {x}px; top: {y}px;"
    role="menu"
    tabindex="0"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    {#each items as item}
      {#if item.divider}
        <div class="h-px bg-gray-200 dark:bg-gray-700 my-1"></div>
      {:else}
        <button
          class="flex items-center gap-3 w-full px-3 py-2 rounded-md border-none bg-transparent cursor-pointer text-sm transition-all duration-150 {item.danger
            ? 'text-red-500 hover:enabled:bg-red-500/10'
            : 'text-gray-900 dark:text-gray-100 hover:enabled:bg-gray-100 dark:hover:enabled:bg-gray-700'} disabled:opacity-40 disabled:cursor-not-allowed"
          onclick={() => handleClick(item)}
          disabled={item.disabled}
        >
          {#if item.icon}<i class="bi bi-{item.icon}" aria-hidden="true"
            ></i>{/if}
          <span>{item.label}</span>
          {#if item.shortcut}<kbd class="ml-auto text-xs opacity-60"
              >{item.shortcut}</kbd
            >{/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}
