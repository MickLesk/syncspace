# Touch Gestures Implementation Guide

## Overview

SyncSpace now includes comprehensive touch gesture support for mobile devices using native Touch Events API. No external libraries required.

## Supported Gestures

### 1. **Tap** (Quick press and release)

- Single-finger tap on any element
- Used for selection and basic interactions
- 300ms max duration for tap recognition

### 2. **Long Press** (Press and hold)

- Press and hold for 500ms (configurable)
- Motion threshold: < 10px to prevent accidental triggers
- Use case: Open context menu, select file

### 3. **Swipe** (Quick directional motion)

- **Swipe Left**: Navigate forward, close panel
- **Swipe Right**: Navigate back, open sidebar
- **Swipe Up**: Scroll up, refresh content
- **Swipe Down**: Scroll down, show pull-to-refresh
- Minimum distance: 50px (configurable)

### 4. **Pinch Zoom** (Multi-touch scaling)

- Two-finger pinch gesture
- Zoom in/out on images and previews
- Min scale: 0.5x, Max scale: 3x (configurable)

---

## Touch Gesture Handler Class

### Basic Usage

```javascript
import { TouchGestureHandler } from "./lib/touchGestures";

const element = document.querySelector("#myElement");
const handler = new TouchGestureHandler(element, {
  longPressDuration: 500,
  swipeThreshold: 50,
  tapThreshold: 10,
});

// Register event listeners
handler.on("tap", (data) => {
  console.log("Tapped at:", data.x, data.y);
});

handler.on("longPress", (data) => {
  console.log("Long press at:", data.x, data.y);
  // Open context menu, etc.
});

handler.on("swipeLeft", (data) => {
  console.log("Swiped left:", data.deltaX);
  // Navigate forward, close panel
});

handler.on("swipeRight", (data) => {
  console.log("Swiped right:", data.deltaX);
  // Navigate back, open sidebar
});

handler.on("swipeUp", (data) => {
  console.log("Swiped up:", data.deltaY);
  // Scroll up, refresh
});

handler.on("swipeDown", (data) => {
  console.log("Swiped down:", data.deltaY);
  // Scroll down, show pull-to-refresh
});

// Don't forget to clean up
handler.destroy();
```

### Event Data

Each gesture event receives data object:

```javascript
{
  x: number,              // X coordinate (for tap/long-press)
  y: number,              // Y coordinate (for tap/long-press)
  deltaX: number,         // Horizontal distance (for swipes)
  deltaY: number,         // Vertical distance (for swipes)
  duration: number,       // Duration in milliseconds
  target: HTMLElement,    // Target element
  originalEvent: Event    // Original touch event
}
```

---

## Svelte Action Usage

### Basic Action

```svelte
<script>
  import { touchGesture } from '$lib/touchGestures';

  function handleLongPress(data) {
    console.log('Long pressed at:', data.x, data.y);
  }

  function handleSwipe(data) {
    console.log('Swiped:', data.deltaX, data.deltaY);
  }
</script>

<div
  use:touchGesture={{
    onLongPress: handleLongPress,
    onSwipeLeft: handleSwipe,
    onSwipeRight: handleSwipe
  }}
>
  Touch me!
</div>
```

### File List with Gestures

```svelte
<script>
  import { touchGesture } from '$lib/touchGestures';
  import { currentPath } from '$stores/ui';

  function handleLongPress(data) {
    // Open context menu on long press
    showContextMenu(data);
  }

  function handleSwipeRight(data) {
    // Navigate to parent directory
    goToParent();
  }
</script>

<div
  class="file-list"
  use:touchGesture={{
    onLongPress: handleLongPress,
    onSwipeRight: handleSwipeRight
  }}
>
  {#each files as file}
    <div class="file-item">
      {file.name}
    </div>
  {/each}
</div>
```

### Image Preview with Pinch Zoom

```svelte
<script>
  import { PinchHandler } from '$lib/touchGestures';
  import { onMount } from 'svelte';

  let imageElement;
  let scale = 1;

  onMount(() => {
    const pinchHandler = new PinchHandler(imageElement, {
      minScale: 0.5,
      maxScale: 3,
      onPinch: (data) => {
        scale = data.scale;
        imageElement.style.transform = `scale(${scale})`;
      }
    });

    return () => pinchHandler.destroy();
  });
</script>

<div class="image-container">
  <img
    bind:this={imageElement}
    src={imageSrc}
    alt="Preview"
    style="transform: scale({scale}); transform-origin: center; transition: transform 0.1s"
  />
</div>
```

---

## Common Use Cases

### 1. **Navigate File Hierarchy**

```svelte
<script>
  import { touchGesture } from '$lib/touchGestures';
  import { fileHistory } from '$stores/fileHistory';

  function handleSwipeRight() {
    fileHistory.goBack();
  }

  function handleSwipeLeft() {
    fileHistory.goForward();
  }
</script>

<div use:touchGesture={{
  onSwipeRight: handleSwipeRight,
  onSwipeLeft: handleSwipeLeft
}}>
  <!-- File content -->
</div>
```

### 2. **Context Menu on Long Press**

```svelte
<script>
  import { touchGesture } from '$lib/touchGestures';

  let contextMenu = null;

  function handleLongPress(data) {
    contextMenu = {
      x: data.x,
      y: data.y,
      target: data.target
    };
  }
</script>

<div use:touchGesture={{ onLongPress: handleLongPress }}>
  <!-- Content -->
</div>

{#if contextMenu}
  <ContextMenu x={contextMenu.x} y={contextMenu.y} on:close={() => contextMenu = null} />
{/if}
```

### 3. **Pull-to-Refresh**

```svelte
<script>
  import { touchGesture } from '$lib/touchGestures';
  import { loadFiles } from '$lib/api';

  let isRefreshing = false;
  let pullDistance = 0;

  function handleSwipeDown(data) {
    if (data.deltaY > 100) {
      isRefreshing = true;
      loadFiles().finally(() => isRefreshing = false);
    }
  }
</script>

<div use:touchGesture={{ onSwipeDown: handleSwipeDown }} class="relative">
  {#if isRefreshing}
    <div class="py-4">
      <i class="bi bi-arrow-clockwise animate-spin"></i>
      Refreshing...
    </div>
  {/if}

  <!-- Content -->
</div>
```

### 4. **Multi-Select with Gestures**

```svelte
<script>
  import { touchGesture } from '$lib/touchGestures';

  let selectedFiles = new Set();
  let lastLongPressTarget = null;

  function handleTap(data) {
    // Single tap: select/deselect
    toggleSelection(data.target);
  }

  function handleLongPress(data) {
    // Long press: start multi-select mode
    lastLongPressTarget = data.target;
    enterMultiSelectMode();
  }
</script>

{#each files as file}
  <div
    use:touchGesture={{
      onTap: handleTap,
      onLongPress: handleLongPress
    }}
    class:selected={selectedFiles.has(file.id)}
  >
    {file.name}
  </div>
{/each}
```

### 5. **Sidebar Toggle**

```svelte
<script>
  import { touchGesture } from '$lib/touchGestures';

  let sidebarOpen = false;

  function handleSwipeRight(data) {
    if (!sidebarOpen && data.deltaX > 100) {
      sidebarOpen = true;
    }
  }

  function handleSwipeLeft(data) {
    if (sidebarOpen && data.deltaX < -100) {
      sidebarOpen = false;
    }
  }
</script>

<main use:touchGesture={{
  onSwipeRight: handleSwipeRight,
  onSwipeLeft: handleSwipeLeft
}}>
  {#if sidebarOpen}
    <aside class="sidebar">
      <!-- Sidebar content -->
    </aside>
  {/if}

  <!-- Main content -->
</main>
```

---

## Configuration Options

```javascript
new TouchGestureHandler(element, {
  // Long press duration in milliseconds (default: 500)
  longPressDuration: 500,

  // Minimum swipe distance in pixels (default: 50)
  swipeThreshold: 50,

  // Maximum distance for tap detection in pixels (default: 10)
  tapThreshold: 10,
});
```

---

## Preventing Default Touch Behavior

The touch gesture handler automatically prevents default behavior for swiping and long press. To manually prevent default:

```javascript
function handleSwipe(data) {
  data.originalEvent.preventDefault();
  // Your custom handling
}
```

---

## Browser Compatibility

- ✅ iOS Safari 10+
- ✅ Android Chrome 50+
- ✅ Samsung Internet 5+
- ✅ Firefox Mobile 50+
- ✅ Edge Mobile 15+

---

## Performance Tips

1. **Debounce Long-Running Operations**

   ```javascript
   import { debounce } from "$lib/utils";

   const handleSwipe = debounce((data) => {
     // Heavy operation
   }, 200);
   ```

2. **Cleanup Handlers**

   ```javascript
   onDestroy(() => {
     handler.destroy();
   });
   ```

3. **Use Passive Listeners**

   - Already implemented in TouchGestureHandler
   - Improves scrolling performance

4. **Combine with Throttle**

   ```javascript
   import { throttle } from "$lib/utils";

   const handleSwipeDown = throttle(refreshFiles, 1000);
   ```

---

## Accessibility Considerations

1. **Keyboard Alternatives** - Always provide keyboard shortcuts
2. **Visual Feedback** - Show visual indication of gesture recognition
3. **ARIA Labels** - Properly label interactive elements
4. **Focus Management** - Maintain focus for keyboard users

```svelte
<!-- Good: Supports both touch and keyboard -->
<button
  use:touchGesture={{ onLongPress: handleMenu }}
  on:contextmenu={handleMenu}
  aria-label="File options"
>
  {fileName}
</button>
```

---

## Gesture Detection Flowchart

```
Touch Event
    ↓
touchstart (record position, start timer)
    ↓
touchmove (check distance)
    ├→ Distance < tapThreshold → Clear timer
    └→ Distance > tapThreshold → Set isSwiping flag
    ↓
touchend
    ├→ Long timer not fired, distance < 10px → Tap
    ├→ isSwiping true → Determine swipe direction
    │   ├→ horizontal + deltaX > 50 → Swipe Left/Right
    │   └→ vertical + deltaY > 50 → Swipe Up/Down
    └→ Long timer fired → Long press already triggered
```

---

## Debugging Gestures

Enable debug logging:

```javascript
class TouchGestureHandler {
  // Add this property
  DEBUG = true;

  trigger(eventType, data) {
    if (this.DEBUG) {
      console.log(`[Gesture] ${eventType}:`, data);
    }
    // ... rest of code
  }
}
```

---

## Integration Checklist

- [x] TouchGestureHandler class - Full implementation
- [x] Svelte actions (touchGesture) - Svelte integration
- [x] Long press detection (500ms)
- [x] Swipe detection (4 directions)
- [x] Tap detection
- [x] Event data passing
- [x] Memory cleanup (destroy)
- [x] Passive event listeners
- [ ] PWA app integration (next: #34 PWA Manifest)
- [ ] Analytics tracking for gestures
- [ ] Performance monitoring

---

## Migration from Old Gesture System

If upgrading from previous touch system:

```javascript
// Old way
element.addEventListener('touchend', oldHandler);

// New way
import { touchGesture } from '$lib/touchGestures';

use:touchGesture={{
  onSwipeRight: handleSwipeRight,
  onSwipeLeft: handleSwipeLeft
}}
```

---

## Next Steps

1. **#34 PWA Manifest** - Add web app manifest for install
2. **#35 Code Splitting** - Implement route-based splitting
3. **#36 Image Optimization** - Add WebP/AVIF support
4. **Analytics** - Track gesture usage for UX improvements

---

**Last Updated:** 2025-11-09  
**Status:** Production Ready - Already Implemented  
**SyncSpace Version:** 1.0+
