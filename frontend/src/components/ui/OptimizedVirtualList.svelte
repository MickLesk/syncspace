<script>
  import { virtualList } from "@sveltejs/svelte-virtual";

  let {
    items = [],
    itemHeight = 100,
    viewportHeight = 500,
    gap = 8,
    renderItem,
    loading = false,
    class: className = "",
    ondragstart = null,
    ondragend = null,
    ondrop = null,
  } = $props();

  let containerRef = $state(null);
  let scrollTop = $state(0);
  let visibleRange = $state({ start: 0, end: 0 });

  // Verwende virtualize logik
  $effect(() => {
    const totalHeight = items.length * (itemHeight + gap);
    const visibleCount = Math.ceil(viewportHeight / (itemHeight + gap)) + 2;
    const startIndex = Math.max(
      0,
      Math.floor(scrollTop / (itemHeight + gap)) - 1
    );
    const endIndex = Math.min(items.length, startIndex + visibleCount);

    visibleRange = { start: startIndex, end: endIndex };
  });

  const visibleItems = $derived(
    items.slice(visibleRange.start, visibleRange.end)
  );
  const offsetTop = $derived(visibleRange.start * (itemHeight + gap));
  const totalHeight = $derived(items.length * (itemHeight + gap));
</script>

<div
  bind:this={containerRef}
  class="virtual-list {className}"
  style="height: {viewportHeight}px; overflow-y: auto; position: relative;"
  onscroll={(e) => (scrollTop = e.currentTarget.scrollTop)}
>
  <!-- Spacer for items above viewport -->
  <div style="height: {offsetTop}px;"></div>

  <!-- Rendered items -->
  <div class="virtual-items">
    {#each visibleItems as item, i (item.id || i)}
      <div
        class="virtual-item"
        style="height: {itemHeight}px; margin-bottom: {gap}px; contain: layout;"
        draggable="true"
        ondragstart={(e) => ondragstart?.(item, e, visibleRange.start + i)}
        ondragend={(e) => ondragend?.(item, e, visibleRange.start + i)}
        ondrop={(e) => ondrop?.(item, e, visibleRange.start + i)}
      >
        <svelte:component this={renderItem} {item} />
      </div>
    {/each}
  </div>

  <!-- Spacer for items below viewport -->
  <div
    style="height: {Math.max(
      0,
      totalHeight -
        offsetTop -
        itemHeight * (visibleRange.end - visibleRange.start) -
        gap * (visibleRange.end - visibleRange.start)
    )}px;"
  ></div>

  {#if loading}
    <div
      class="absolute inset-0 flex items-center justify-center bg-white/50 dark:bg-gray-800/50"
    >
      <div
        class="animate-spin rounded-full h-8 w-8 border-b-2 border-green-500"
      ></div>
    </div>
  {/if}
</div>

<style>
  .virtual-list {
    contain: strict;
  }

  .virtual-items {
    contain: content;
  }

  .virtual-item {
    will-change: transform;
  }
</style>
