<script>
  /**
   * File Context Menu Component
   *
   * Handles right-click menus for files with dynamic actions,
   * keyboard shortcuts (Shift+F10), and touch long-press support.
   *
   * Features:
   * - Dynamic menu items based on file type and permissions
   * - Keyboard navigation (arrows, enter, escape)
   * - Shift+F10 to open context menu
   * - Touch long-press support (500ms)
   * - Click-outside detection
   * - Viewport overflow handling
   *
   * @component
   * @example
   *   <FileContextMenu {item} {context} let:isOpen>
   *     <div on:contextmenu={showMenu}>
   *       File content...
   *     </div>
   *   </FileContextMenu>
   */

  import { onMount, onDestroy, tick } from "svelte";
  import { getContextMenuItems } from "../lib/contextMenuActions.js";
  import ContextMenu from "./ui/ContextMenu.svelte";

  // Props
  let { item = null, context = {}, onAction = () => {}, children } = $props();

  // State
  let isOpen = $state(false);
  let x = $state(0);
  let y = $state(0);
  let menuItems = $state([]);
  let selectedIndex = -1;
  let touchTimer = null;
  let touchStartX = 0;
  let touchStartY = 0;
  let targetElement = null;

  const TOUCH_HOLD_MS = 500;
  const TOUCH_THRESHOLD_PX = 10;

  /**
   * Update menu items when item or context changes
   */
  $effect(() => {
    if (item) {
      menuItems = getContextMenuItems(item, context);
    }
  });

  /**
   * Handle right-click on target element
   */
  function handleContextMenu(e) {
    e.preventDefault();
    e.stopPropagation();

    x = e.clientX;
    y = e.clientY;
    isOpen = true;
    selectedIndex = -1;
    targetElement = e.target;
  }

  /**
   * Handle keyboard shortcuts
   */
  function handleKeyDown(e) {
    if (!isOpen) {
      // Shift+F10 to open context menu
      if (e.shiftKey && e.key === "F10" && targetElement) {
        e.preventDefault();
        const rect = targetElement.getBoundingClientRect();
        x = rect.left + rect.width / 2;
        y = rect.top;
        isOpen = true;
        selectedIndex = 0;
        return;
      }

      // Direct shortcuts when menu is closed
      switch (e.key) {
        case "Delete":
          if (!context?.isTrashed) {
            e.preventDefault();
            // Find delete action
            const deleteAction = menuItems.find((m) => m.id === "delete");
            if (deleteAction?.action) deleteAction.action();
          }
          break;
        case "F2":
          if (context?.canEdit) {
            e.preventDefault();
            const renameAction = menuItems.find((m) => m.id === "rename");
            if (renameAction?.action) renameAction.action();
          }
          break;
      }
      return;
    }

    // Handle menu navigation
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, menuItems.length - 1);
        break;
      case "ArrowUp":
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case "Enter":
        e.preventDefault();
        if (selectedIndex >= 0 && !menuItems[selectedIndex]?.disabled) {
          const action = menuItems[selectedIndex];
          if (action.action) {
            action.action();
            isOpen = false;
          }
        }
        break;
      case "Escape":
        e.preventDefault();
        isOpen = false;
        selectedIndex = -1;
        break;
    }
  }

  /**
   * Handle touch long-press
   */
  function handleTouchStart(e) {
    touchStartX = e.touches[0].clientX;
    touchStartY = e.touches[0].clientY;
    targetElement = e.target;

    touchTimer = setTimeout(() => {
      x = e.touches[0].clientX;
      y = e.touches[0].clientY;
      isOpen = true;
      selectedIndex = -1;
      touchTimer = null;
    }, TOUCH_HOLD_MS);
  }

  /**
   * Handle touch end
   */
  function handleTouchEnd(e) {
    if (touchTimer) {
      clearTimeout(touchTimer);
      touchTimer = null;

      const dx = e.changedTouches[0].clientX - touchStartX;
      const dy = e.changedTouches[0].clientY - touchStartY;
      const distance = Math.sqrt(dx * dx + dy * dy);

      // If movement was small and touch was quick, treat as regular tap
      if (distance < TOUCH_THRESHOLD_PX) {
        // Regular tap - no special handling
      }
    }
  }

  /**
   * Close menu when clicking outside
   */
  function handleClickOutside(e) {
    if (isOpen && !e.target.closest('[role="menu"]')) {
      isOpen = false;
      selectedIndex = -1;
    }
  }

  /**
   * Handle menu item action
   */
  function handleMenuAction(action) {
    if (action?.action) {
      action.action();
      onAction?.(action);
    }
    isOpen = false;
  }

  onMount(() => {
    // Don't add global listeners - let parent handle context menu
    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("click", handleClickOutside);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("click", handleClickOutside);
      if (touchTimer) clearTimeout(touchTimer);
    };
  });

  onDestroy(() => {
    if (touchTimer) clearTimeout(touchTimer);
  });
</script>

<!-- Proxy events to parent -->
<div
  role="application"
  oncontextmenu={handleContextMenu}
  ontouchstart={handleTouchStart}
  ontouchend={handleTouchEnd}
>
  {@render children?.({ isOpen, x, y, menuItems })}
</div>

<!-- Context Menu -->
{#if isOpen}
  <ContextMenu
    {x}
    {y}
    items={menuItems}
    {context}
    onselect={(e) => handleMenuAction(e.detail)}
  />
{/if}
