<script lang="ts">
  interface Props {
    items: Array<{ id: string; label: string }>;
    selected?: string[];
    onchange?: (selected: string[]) => void;
  }

  let { items, selected = [], onchange }: Props = $props();

  function toggleItem(id: string) {
    selected = selected.includes(id)
      ? selected.filter((s) => s !== id)
      : [...selected, id];
    onchange?.(selected);
  }
</script>

<div class="flex flex-wrap gap-2">
  {#each items as item}
    <button
      onclick={() => toggleItem(item.id)}
      class={`
        px-4 py-2 rounded-lg border-2 transition-all font-medium text-sm
        ${
          selected.includes(item.id)
            ? "border-blue-500 bg-blue-50 text-blue-700"
            : "border-gray-300 bg-white text-gray-700 hover:border-gray-400"
        }
      `}
    >
      {item.label}
    </button>
  {/each}
</div>
