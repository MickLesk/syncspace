<script lang="ts">
  import type { Snippet } from "svelte";

  interface Item {
    id: string;
    label: string;
    icon?: string;
    disabled?: boolean;
    divider?: boolean;
    danger?: boolean;
  }

  interface Props {
    items: Item[];
    open?: boolean;
    position?: "bottom-left" | "bottom-right" | "top-left" | "top-right";
    class?: string;
    onselect?: (itemId: string) => void;
    children?: Snippet;
  }

  let {
    items,
    open = $bindable(false),
    position = "bottom-left",
    class: customClass = "",
    onselect,
    children,
  }: Props = $props();

  const positionClasses = {
    "bottom-left": "top-full left-0 mt-2",
    "bottom-right": "top-full right-0 mt-2",
    "top-left": "bottom-full left-0 mb-2",
    "top-right": "bottom-full right-0 mb-2",
  };

  function handleSelect(item: Item) {
    if (item.disabled) return;
    onselect?.(item.id);
    open = false;
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest("[data-dropdown]")) {
      open = false;
    }
  }

  $effect(() => {
    if (open) {
      document.addEventListener("click", handleClickOutside);
      return () => document.removeEventListener("click", handleClickOutside);
    }
  });
</script>

<div class={`relative inline-block ${customClass}`} data-dropdown>
  <!-- Trigger -->
  <button
    type="button"
    onclick={() => (open = !open)}
    class="inline-flex items-center gap-2 px-4 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
  >
    {#if children}
      {@render children()}
    {/if}
    <i
      class={`bi bi-chevron-down text-gray-600 dark:text-gray-400 transition-transform ${
        open ? "rotate-180" : ""
      }`}
    ></i>
  </button>

  <!-- Dropdown Menu -->
  {#if open}
    <div
      class={`
        absolute z-50 min-w-[200px]
        bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700
        rounded-lg shadow-lg py-1
        ${positionClasses[position]}
      `}
    >
      {#each items as item (item.id)}
        {#if item.divider}
          <div class="my-1 border-t border-gray-200 dark:border-gray-700"></div>
        {:else}
          <button
            type="button"
            onclick={() => handleSelect(item)}
            disabled={item.disabled}
            class={`
              w-full flex items-center gap-3 px-4 py-2 text-left text-sm
              transition-colors
              ${
                item.disabled
                  ? "opacity-50 cursor-not-allowed"
                  : item.danger
                    ? "text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20"
                    : "text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
              }
            `}
          >
            {#if item.icon}
              <i class={`bi ${item.icon}`}></i>
            {/if}
            <span>{item.label}</span>
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>
