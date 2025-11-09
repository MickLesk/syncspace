<script>
  import { onMount, onDestroy } from "svelte";

  let {
    items = [],
    renderItem,
    onReorder = null,
    onDrop = null,
    onDragStart = null,
    onDragEnd = null,
    dropEffect = "move", // 'copy' | 'move'
    class: className = "",
  } = $props();

  let isDragging = $state(null);
  let dragOverIndex = $state(null);
  let dropZones = $state([]);

  function handleDragStart(e, index) {
    isDragging = index;
    e.dataTransfer.effectAllowed = dropEffect;
    e.dataTransfer.setData("application/json", JSON.stringify(items[index]));

    const img = new Image();
    img.src =
      "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='1' height='1'%3E%3C/svg%3E";
    e.dataTransfer.setDragImage(img, 0, 0);

    onDragStart?.(items[index], index);
  }

  function handleDragEnd(e, index) {
    isDragging = null;
    dragOverIndex = null;
    onDragEnd?.(items[index], index);
  }

  function handleDragOver(e, index) {
    e.preventDefault();
    e.dataTransfer.dropEffect = dropEffect;
    dragOverIndex = index;
  }

  function handleDragLeave(e) {
    dragOverIndex = null;
  }

  function handleDrop(e, index) {
    e.preventDefault();
    dragOverIndex = null;

    if (isDragging === null) return;

    try {
      const data = e.dataTransfer.getData("application/json");
      const draggedItem = JSON.parse(data);

      if (dropEffect === "move") {
        // Reorder items
        const newItems = [...items];
        newItems.splice(isDragging, 1);
        newItems.splice(index, 0, draggedItem);
        onReorder?.(newItems);
      } else if (dropEffect === "copy") {
        // Copy item
        onDrop?.(draggedItem, items[index], e);
      }
    } catch (err) {
      console.error("Drop error:", err);
    }

    isDragging = null;
  }

  onMount(() => {
    // Preload drag image for better performance
  });
</script>

<div class="drag-drop-list {className}">
  {#each items as item, index (item.id || index)}
    <div
      class="drag-drop-item"
      class:dragging={isDragging === index}
      class:drag-over={dragOverIndex === index}
      draggable="true"
      ondragstart={(e) => handleDragStart(e, index)}
      ondragend={(e) => handleDragEnd(e, index)}
      ondragover={(e) => handleDragOver(e, index)}
      ondragleave={handleDragLeave}
      ondrop={(e) => handleDrop(e, index)}
    >
      <!-- Drag Handle Indicator -->
      <div class="drag-handle">
        <i class="bi bi-grip-vertical"></i>
      </div>

      <!-- Item Content -->
      <div class="item-content">
        <svelte:component this={renderItem} {item} />
      </div>

      <!-- Drop Indicator -->
      {#if dragOverIndex === index && isDragging !== null}
        <div class="drop-indicator">
          <div class="drop-line"></div>
          <span class="drop-label">
            {dropEffect === "move" ? "Reorder" : "Copy"}
          </span>
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .drag-drop-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .drag-drop-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: white;
    border: 2px solid transparent;
    border-radius: 0.5rem;
    transition: all 0.2s ease;
    cursor: grab;
    position: relative;
    will-change: transform;
  }

  .drag-drop-item:active {
    cursor: grabbing;
  }

  .drag-drop-item.dragging {
    opacity: 0.5;
    transform: scale(0.95);
  }

  .drag-drop-item.drag-over {
    border-color: #3b82f6;
    background-color: #eff6ff;
    transform: translateX(4px);
  }

  .drag-drop-item:dark {
    background: #1f2937;
  }

  .drag-drop-item.drag-over:dark {
    background-color: #111827;
    border-color: #60a5fa;
  }

  .drag-handle {
    display: flex;
    align-items: center;
    justify-content: center;
    color: #9ca3af;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .drag-drop-item:hover .drag-handle {
    opacity: 1;
  }

  .drag-handle i {
    font-size: 1rem;
  }

  .item-content {
    flex: 1;
    min-width: 0;
  }

  .drop-indicator {
    position: absolute;
    left: 0;
    right: 0;
    top: -2px;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }

  .drop-line {
    height: 2px;
    background: #3b82f6;
    width: 100%;
    animation: pulse 0.6s ease-in-out infinite;
  }

  .drop-label {
    position: absolute;
    top: -1.5rem;
    background: #3b82f6;
    color: white;
    padding: 0.25rem 0.75rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    font-weight: 600;
    white-space: nowrap;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
