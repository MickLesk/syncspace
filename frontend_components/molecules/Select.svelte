<script lang="ts">
  interface Props {
    items: Array<{ id: string; name: string; value: string; grouped?: string }>;
    onchange?: (value: string) => void;
    placeholder?: string;
    disabled?: boolean;
  }

  let {
    items,
    onchange,
    placeholder = "Select...",
    disabled = false,
  }: Props = $props();
  let open = $state(false);
  let selected = $state<string | null>(null);

  function selectItem(value: string) {
    selected = value;
    onchange?.(value);
    open = false;
  }
</script>

<div class="relative w-full">
  <button
    {disabled}
    onclick={() => (open = !open)}
    class={`
      w-full px-4 py-2 border rounded-lg text-left flex items-center justify-between
      transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500
      ${disabled ? "bg-gray-100 cursor-not-allowed opacity-50" : "bg-white hover:bg-gray-50"}
      ${open ? "border-blue-500" : "border-gray-300"}
    `}
  >
    <span class={selected ? "text-gray-900" : "text-gray-500"}>
      {items.find((i) => i.value === selected)?.name || placeholder}
    </span>
    <i
      class={`bi bi-chevron-down transition-transform ${open ? "rotate-180" : ""}`}
    ></i>
  </button>

  {#if open}
    <div
      class="absolute top-full left-0 right-0 mt-1 bg-white border border-gray-300 rounded-lg shadow-lg z-10"
      onclick={(e) => e.stopPropagation()}
    >
      {#each items as item}
        <button
          onclick={() => selectItem(item.value)}
          class={`
            w-full px-4 py-2 text-left transition-colors hover:bg-blue-50
            ${selected === item.value ? "bg-blue-100 text-blue-900 font-semibold" : "text-gray-700"}
          `}
        >
          {item.name}
        </button>
      {/each}
    </div>
  {/if}
</div>

{#if open}
  <div class="fixed inset-0 z-0" onclick={() => (open = false)} />
{/if}
