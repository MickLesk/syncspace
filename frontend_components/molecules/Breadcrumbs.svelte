<script lang="ts">
  interface Props {
    items: Array<{ label: string; value: string; href?: string }>;
    current?: string;
    separator?: string;
  }

  let { items, current, separator = "/" }: Props = $props();
</script>

<nav class="flex items-center gap-2" aria-label="breadcrumb">
  {#each items as item, i}
    <div class="flex items-center gap-2">
      {#if item.href}
        <a
          href={item.href}
          class="text-blue-600 hover:text-blue-800 hover:underline text-sm"
        >
          {item.label}
        </a>
      {:else}
        <span
          class={`text-sm ${current === item.value ? "text-gray-900 font-semibold" : "text-gray-600"}`}
        >
          {item.label}
        </span>
      {/if}
      {#if i < items.length - 1}
        <span class="text-gray-400">{separator}</span>
      {/if}
    </div>
  {/each}
</nav>
