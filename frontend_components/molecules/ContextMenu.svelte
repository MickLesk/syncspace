<script lang="ts">
  interface MenuItem {
    id: string;
    label: string;
    icon?: string;
    divider?: boolean;
    onclick?: () => void;
    dangerous?: boolean;
  }

  interface Props {
    items: MenuItem[];
    x?: number;
    y?: number;
    visible?: boolean;
  }

  let { items, x = 0, y = 0, visible = false }: Props = $props();

  let menuRef: HTMLElement | null = $state(null);

  function handleClick(item: MenuItem) {
    item.onclick?.();
    visible = false;
  }
</script>

{#if visible && menuRef}
  <div
    bind:this={menuRef}
    class="fixed bg-white rounded-lg shadow-xl border border-gray-200 z-50 py-1 min-w-max"
    style={`top: ${y}px; left: ${x}px;`}
    role="menu"
  >
    {#each items as item}
      {#if item.divider}
        <hr class="my-1 border-gray-200" />
      {:else}
        <button
          onclick={() => handleClick(item)}
          class={`
            w-full px-4 py-2 text-left text-sm flex items-center gap-2 hover:bg-gray-100
            transition-colors focus:outline-none
            ${item.dangerous ? "text-red-600 hover:bg-red-50" : "text-gray-700"}
          `}
          role="menuitem"
        >
          {#if item.icon}
            <i class={`bi ${item.icon}`}></i>
          {/if}
          {item.label}
        </button>
      {/if}
    {/each}
  </div>
{/if}
