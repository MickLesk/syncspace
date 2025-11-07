<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Virtual List/Grid Component - Optimized for 10,000+ items
   * Renders only visible items for better performance with large datasets
   * Supports both list view and responsive grid layouts
   *
   * Performance targets:
   * - <16ms render time per frame (60fps)
   * - Support for 100,000+ items
   * - Minimal memory footprint (only visible items in DOM)
   * - Smooth scrolling with momentum
   *
   * @props items - Array of items to render
   * @props itemHeight - Height of each item/row in pixels (default: 60)
   * @props visibleCount - Number of visible items (informational, default: 20)
   * @props columns - Number of grid columns (default: 1 for list mode)
   * @props isGrid - Enable grid layout (default: false)
   * @props persistKey - localStorage key for scroll position persistence
   * @props overscan - Number of items to render outside viewport (default: 3)
   * @props children - Render snippet for each item
   */
  let {
    items = [],
    itemHeight = 60,
    visibleCount = 20,
    columns = 1,
    isGrid = false,
    persistKey = null,
    overscan = 3,
    children,
    ...restProps
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let scrollTop = $state(0);
  let containerHeight = $state(0);
  let containerWidth = $state(0);
  let isRestoring = $state(false);
  let rafId = $state(null);

  // Calculate responsive columns for grid layout
  let responsiveColumns = $derived(() => {
    if (!isGrid || columns > 1) return columns; // Use provided columns if specified

    // Auto-calculate responsive columns based on container width
    if (containerWidth >= 1536) return 5; // 2xl: 5 columns
    if (containerWidth >= 1280) return 4; // xl: 4 columns
    if (containerWidth >= 1024) return 3; // lg: 3 columns
    if (containerWidth >= 640) return 2; // sm: 2 columns
    return 1; // default: 1 column
  });

  // For grid: calculate rows, for list: each item is a row
  let effectiveColumns = $derived(isGrid ? responsiveColumns() : 1);
  let totalRows = $derived(Math.ceil(items.length / effectiveColumns));
  let rowHeight = $derived(itemHeight);

  // Calculate visible range with configurable overscan
  let startRowIndex = $derived(
    Math.max(0, Math.floor(scrollTop / rowHeight) - overscan)
  );
  let endRowIndex = $derived(
    Math.min(
      totalRows,
      Math.ceil((scrollTop + containerHeight) / rowHeight) + overscan
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
        // Use requestAnimationFrame for smoother restoration
        requestAnimationFrame(() => {
          const container = document.querySelector(".virtual-list-container");
          if (container) {
            container.scrollTop = position;
            scrollTop = position;
          }
          isRestoring = false;
        });
      }
    }
  });

  // Save scroll position to localStorage (debounced with RAF)
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

  // Optimized scroll handler with requestAnimationFrame
  function handleScroll(e) {
    if (rafId) cancelAnimationFrame(rafId);

    rafId = requestAnimationFrame(() => {
      scrollTop = e.target.scrollTop;
      rafId = null;
    });
  }

  // ResizeObserver for container dimensions
  function handleResize(node) {
    const observer = new ResizeObserver((entries) => {
      // Use requestAnimationFrame to batch updates
      requestAnimationFrame(() => {
        const rect = entries[0].contentRect;
        containerHeight = rect.height;
        containerWidth = rect.width;
      });
    });
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
        if (rafId) cancelAnimationFrame(rafId);
      },
    };
  }

  // Performance monitoring (dev mode only)
  $effect(() => {
    if (import.meta.env.DEV) {
      console.debug(
        `[VirtualList] Rendering ${visibleItems.length}/${items.length} items (rows ${startRowIndex}-${endRowIndex}/${totalRows})`
      );
    }
  });
</script>

<div
  class="virtual-list-container"
  onscroll={handleScroll}
  use:handleResize
  {...restProps}
  style="overflow-y: auto; height: 100%; overflow-x: hidden; contain: layout style paint;"
>
  <div
    class="virtual-list-spacer"
    style="height: {totalHeight}px; position: relative; will-change: contents;"
  >
    <div
      class="virtual-list-content"
      style="position: absolute; top: {offsetY}px; left: 0; right: 0; will-change: transform;"
    >
      {#if isGrid}
        <!-- Grid layout with responsive columns -->
        <div
          class="virtual-grid"
          style="display: grid; grid-template-columns: repeat({effectiveColumns}, 1fr); gap: 1rem; contain: layout style;"
        >
          {#each visibleItems as item, index (item.id || item.file_path || item.name || startIndex + index)}
            {@render children(item, startIndex + index)}
          {/each}
        </div>
      {:else}
        <!-- List layout -->
        <div class="virtual-list-items" style="contain: layout style;">
          {#each visibleItems as item, index (item.id || item.file_path || item.name || startIndex + index)}
            {@render children(item, startIndex + index)}
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .virtual-list-container {
    position: relative;
    will-change: scroll-position;
    /* Enable GPU acceleration */
    transform: translateZ(0);
    /* Improve scroll performance */
    -webkit-overflow-scrolling: touch;
    /* Contain layout for better performance */
    contain: strict;
  }

  .virtual-list-content {
    will-change: transform;
    /* GPU acceleration for smoother scrolling */
    transform: translateZ(0);
  }

  .virtual-grid,
  .virtual-list-items {
    width: 100%;
    /* Prevent layout shifts */
    contain: layout style;
  }

  /* Smooth scrollbar styling */
  .virtual-list-container::-webkit-scrollbar {
    width: 12px;
  }

  .virtual-list-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .virtual-list-container::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 6px;
    border: 3px solid transparent;
    background-clip: padding-box;
  }

  .virtual-list-container::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
    background-clip: padding-box;
  }

  /* Dark mode scrollbar */
  :global(.dark) .virtual-list-container::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    background-clip: padding-box;
  }

  :global(.dark) .virtual-list-container::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
    background-clip: padding-box;
  }
</style>
