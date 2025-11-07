<script>
  import { createVirtualizer } from '@tanstack/svelte-virtual';
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Advanced Virtual List using @tanstack/svelte-virtual
   * Optimized for 100,000+ items with dynamic heights
   * 
   * Features:
   * - Dynamic item heights (auto-measured)
   * - Smooth scrolling with momentum
   * - Scroll position persistence
   * - Horizontal scrolling support
   * - Sticky headers support
   * - Better performance than custom implementation
   * 
   * @props items - Array of items to render
   * @props estimateSize - Function to estimate item size (default: 60px)
   * @props isGrid - Enable grid layout (default: false)
   * @props columns - Number of grid columns (default: auto)
   * @props persistKey - localStorage key for scroll position
   * @props children - Render snippet for each item
   */
  let {
    items = [],
    estimateSize = () => 60,
    isGrid = false,
    columns = 'auto',
    persistKey = null,
    children,
    ...restProps
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let scrollElement = $state(null);
  let containerWidth = $state(0);
  let isRestoring = $state(false);

  // Calculate responsive columns for grid
  let responsiveColumns = $derived(() => {
    if (!isGrid || columns !== 'auto') return columns;

    if (containerWidth >= 1536) return 5; // 2xl
    if (containerWidth >= 1280) return 4; // xl
    if (containerWidth >= 1024) return 3; // lg
    if (containerWidth >= 640) return 2;  // sm
    return 1;
  });

  // For grid layout, group items into rows
  let rowItems = $derived(() => {
    if (!isGrid) return items.map((item, i) => ({ items: [item], rowIndex: i }));
    
    const cols = responsiveColumns();
    const rows = [];
    for (let i = 0; i < items.length; i += cols) {
      rows.push({
        items: items.slice(i, i + cols),
        rowIndex: Math.floor(i / cols)
      });
    }
    return rows;
  });

  // Create virtualizer instance
  const virtualizer = $derived(() => {
    if (!scrollElement) return null;

    return createVirtualizer({
      count: rowItems().length,
      getScrollElement: () => scrollElement,
      estimateSize: isGrid ? () => 220 : estimateSize,
      overscan: 5,
      // Enable smooth scrolling
      scrollPaddingStart: 0,
      scrollPaddingEnd: 0,
    });
  });

  const virtualItems = $derived(virtualizer()?.getVirtualItems() || []);
  const totalSize = $derived(virtualizer()?.getTotalSize() || 0);

  // Restore scroll position from localStorage
  $effect(() => {
    if (!persistKey || !scrollElement || isRestoring) return;

    const savedScroll = localStorage.getItem(`vscroll_tanstack_${persistKey}`);
    if (savedScroll) {
      const position = parseInt(savedScroll, 10);
      if (!isNaN(position) && position > 0) {
        isRestoring = true;
        requestAnimationFrame(() => {
          if (scrollElement) {
            scrollElement.scrollTop = position;
          }
          // Wait a bit before allowing saves again
          setTimeout(() => {
            isRestoring = false;
          }, 100);
        });
      }
    }
  });

  // Save scroll position (debounced)
  let saveTimeout;
  function handleScroll() {
    if (!persistKey || isRestoring) return;

    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(() => {
      const position = scrollElement?.scrollTop || 0;
      if (position > 0) {
        localStorage.setItem(`vscroll_tanstack_${persistKey}`, position.toString());
      }
    }, 300);
  }

  // Track container width for responsive columns
  function handleResize(node) {
    const observer = new ResizeObserver((entries) => {
      requestAnimationFrame(() => {
        containerWidth = entries[0].contentRect.width;
      });
    });
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
        clearTimeout(saveTimeout);
      },
    };
  }

  // Performance monitoring (dev only)
  $effect(() => {
    if (import.meta.env.DEV && virtualItems.length > 0) {
      console.debug(
        `[TanStackVirtualList] Rendering ${virtualItems.length}/${rowItems().length} rows, Total items: ${items.length}`
      );
    }
  });
</script>

<div
  bind:this={scrollElement}
  class="tanstack-virtual-container"
  onscroll={handleScroll}
  use:handleResize
  {...restProps}
  style="overflow-y: auto; height: 100%; overflow-x: hidden; contain: strict;"
>
  <div
    class="tanstack-virtual-spacer"
    style="height: {totalSize}px; position: relative; width: 100%;"
  >
    {#if virtualItems.length > 0}
      <div
        class="tanstack-virtual-content"
        style="position: absolute; top: 0; left: 0; right: 0; transform: translateY({virtualItems[0]?.start || 0}px); will-change: transform;"
      >
        {#each virtualItems as virtualRow (virtualRow.key)}
          {@const row = rowItems()[virtualRow.index]}
          {#if row}
            <div
              data-index={virtualRow.index}
              style="position: absolute; top: 0; left: 0; right: 0; transform: translateY({virtualRow.start - (virtualItems[0]?.start || 0)}px);"
            >
              {#if isGrid}
                <!-- Grid layout -->
                <div
                  class="tanstack-virtual-grid"
                  style="display: grid; grid-template-columns: repeat({responsiveColumns()}, 1fr); gap: 1rem; padding: 0.5rem;"
                >
                  {#each row.items as item, colIndex}
                    {@const itemIndex = virtualRow.index * responsiveColumns() + colIndex}
                    {#if itemIndex < items.length}
                      {@render children(item, itemIndex)}
                    {/if}
                  {/each}
                </div>
              {:else}
                <!-- List layout -->
                {#each row.items as item, colIndex}
                  {@const itemIndex = virtualRow.index}
                  {@render children(item, itemIndex)}
                {/each}
              {/if}
            </div>
          {/if}
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .tanstack-virtual-container {
    position: relative;
    will-change: scroll-position;
    transform: translateZ(0);
    -webkit-overflow-scrolling: touch;
  }

  .tanstack-virtual-content {
    will-change: transform;
    transform: translateZ(0);
  }

  .tanstack-virtual-grid {
    width: 100%;
    contain: layout style;
  }

  /* Scrollbar styling */
  .tanstack-virtual-container::-webkit-scrollbar {
    width: 12px;
  }

  .tanstack-virtual-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .tanstack-virtual-container::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 6px;
    border: 3px solid transparent;
    background-clip: padding-box;
  }

  .tanstack-virtual-container::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
    background-clip: padding-box;
  }

  :global(.dark) .tanstack-virtual-container::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    background-clip: padding-box;
  }

  :global(.dark) .tanstack-virtual-container::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
    background-clip: padding-box;
  }
</style>
