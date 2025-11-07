<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Virtual List/Grid Component
   * Renders only visible items for better performance with large datasets
   * Supports both list view and responsive grid layouts
   *
   * @props items - Array of items to render
   * @props itemHeight - Height of each item/row in pixels (default: 60)
   * @props visibleCount - Number of visible items (informational, default: 20)
   * @props columns - Number of grid columns (default: 1 for list mode)
   * @props isGrid - Enable grid layout (default: false)
   * @props persistKey - localStorage key for scroll position persistence
   * @props children - Render snippet for each item
   */
  let {
    items = [],
    itemHeight = 60,
    visibleCount = 20,
    columns = 1,
    isGrid = false,
    persistKey = null,
    children,
    ...restProps
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let scrollTop = $state(0);
  let containerHeight = $state(0);
  let containerWidth = $state(0);
  let isRestoring = $state(false);

  // Calculate responsive columns for grid layout
  let responsiveColumns = $derived(() => {
    if (!isGrid || columns > 1) return columns; // Use provided columns if specified

    // Auto-calculate responsive columns based on container width
    if (containerWidth >= 1280) return 4; // xl: 4 columns
    if (containerWidth >= 1024) return 3; // lg: 3 columns
    if (containerWidth >= 640) return 2; // sm: 2 columns
    return 1; // default: 1 column
  });

  // For grid: calculate rows, for list: each item is a row
  let effectiveColumns = $derived(isGrid ? responsiveColumns() : 1);
  let totalRows = $derived(Math.ceil(items.length / effectiveColumns));
  let rowHeight = $derived(itemHeight);

  // Calculate visible range with buffer (2 rows before/after viewport)
  let startRowIndex = $derived(
    Math.max(0, Math.floor(scrollTop / rowHeight) - 2)
  );
  let endRowIndex = $derived(
    Math.min(
      totalRows,
      Math.ceil((scrollTop + containerHeight) / rowHeight) + 2
    )
  );

  // Convert row indices to item indices
  let startIndex = $derived(startRowIndex * effectiveColumns);
  let endIndex = $derived(
    Math.min(items.length, endRowIndex * effectiveColumns)
  );

  let visibleItems = $derived(items.slice(startIndex, endIndex));
  let totalHeight = $derived(totalRows * rowHeight);
  let offsetY = $derived(startRowIndex * rowHeight);

  // Restore scroll position from localStorage on mount
  $effect(() => {
    if (!persistKey || isRestoring) return;

    const savedScroll = localStorage.getItem(`vscroll_${persistKey}`);
    if (savedScroll) {
      const position = parseInt(savedScroll, 10);
      if (!isNaN(position) && position > 0) {
        isRestoring = true;
        // Use setTimeout to ensure DOM is ready
        setTimeout(() => {
          const container = document.querySelector(".virtual-list-container");
          if (container) {
            container.scrollTop = position;
            scrollTop = position;
          }
          isRestoring = false;
        }, 0);
      }
    }
  });

  // Save scroll position to localStorage (debounced)
  let saveTimeout;
  $effect(() => {
    if (!persistKey || isRestoring) return;

    // Trigger effect on scrollTop change
    const currentScroll = scrollTop;

    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(() => {
      if (currentScroll > 0) {
        localStorage.setItem(`vscroll_${persistKey}`, currentScroll.toString());
      }
    }, 300); // 300ms debounce

    return () => clearTimeout(saveTimeout);
  });

  function handleScroll(e) {
    scrollTop = e.target.scrollTop;
  }

  function handleResize(node) {
    const observer = new ResizeObserver((entries) => {
      const rect = entries[0].contentRect;
      containerHeight = rect.height;
      containerWidth = rect.width;
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
      {#if isGrid}
        <!-- Grid layout with responsive columns -->
        <div
          class="virtual-grid"
          style="display: grid; grid-template-columns: repeat({effectiveColumns}, 1fr); gap: 1rem;"
        >
          {#each visibleItems as item, index (item.id || item.file_path || item.name || startIndex + index)}
            {@render children(item, startIndex + index)}
          {/each}
        </div>
      {:else}
        <!-- List layout -->
        {#each visibleItems as item, index (item.id || item.file_path || item.name || startIndex + index)}
          {@render children(item, startIndex + index)}
        {/each}
      {/if}
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

  .virtual-grid {
    width: 100%;
  }
</style>
