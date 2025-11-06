<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Virtual List Component
   * Renders only visible items for better performance with large datasets
   */
  let {
    items = [],
    itemHeight = 60,
    visibleCount = 20,
    children,
    ...restProps
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let scrollTop = $state(0);
  let containerHeight = $state(0);

  // Calculate visible range using $derived
  let startIndex = $derived(
    Math.max(0, Math.floor(scrollTop / itemHeight) - 5)
  );
  let endIndex = $derived(
    Math.min(
      items.length,
      Math.ceil((scrollTop + containerHeight) / itemHeight) + 5
    )
  );
  let visibleItems = $derived(items.slice(startIndex, endIndex));
  let totalHeight = $derived(items.length * itemHeight);
  let offsetY = $derived(startIndex * itemHeight);

  function handleScroll(e) {
    scrollTop = e.target.scrollTop;
  }

  function handleResize(node) {
    const observer = new ResizeObserver((entries) => {
      containerHeight = entries[0].contentRect.height;
    });
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
      },
    };
  }
</script>

<div
  class="virtual-list-container"
  onscroll={handleScroll}
  use:handleResize
  {...restProps}
  style="overflow-y: auto; height: 100%;"
>
  <div
    class="virtual-list-spacer"
    style="height: {totalHeight}px; position: relative;"
  >
    <div
      class="virtual-list-content"
      style="position: absolute; top: {offsetY}px; left: 0; right: 0;"
    >
      {#each visibleItems as item, index (item.id || item.path || startIndex + index)}
        {@render children(item, startIndex + index)}
      {/each}
    </div>
  </div>
</div>

<style>
  .virtual-list-container {
    position: relative;
    will-change: scroll-position;
  }

  .virtual-list-content {
    will-change: transform;
  }
</style>
