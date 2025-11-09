# Mobile Responsive Design Guide

## Overview

SyncSpace now includes comprehensive mobile-first responsive design using Tailwind CSS breakpoints and custom responsive components.

## Breakpoints

- **Mobile**: < 640px (sm)
- **Tablet**: 640px - 1024px (md-lg)
- **Desktop**: ≥ 1024px (lg+)
- **Large Desktop**: ≥ 1280px (xl)

## Responsive Components

### 1. **ResponsiveLayout.svelte**

Main container that manages:

- Adaptive header with mobile menu toggle
- Collapsible sidebar (overlay on mobile, fixed on desktop)
- Main content area with proper overflow handling
- Bottom navigation for mobile
- Safe area insets for notched devices

```svelte
<ResponsiveLayout showSidebar={true}>
  <svelte:fragment slot="header">
    <h1>My App</h1>
  </svelte:fragment>

  <svelte:fragment slot="sidebar">
    <!-- Sidebar content -->
  </svelte:fragment>

  <svelte:fragment slot="content">
    <!-- Main content -->
  </svelte:fragment>
</ResponsiveLayout>
```

### 2. **ResponsiveFileGrid.svelte**

Adaptive file grid that changes columns based on screen size:

- Mobile (< 640px): 1-2 columns
- Tablet (640-1024px): 3 columns
- Desktop (1024-1280px): 4 columns
- Large Desktop (≥ 1280px): 6 columns

```svelte
<ResponsiveFileGrid {files} viewMode="grid">
  <svelte:fragment slot="fileCard" let:file>
    <!-- File card component -->
  </svelte:fragment>
</ResponsiveFileGrid>
```

### 3. **ResponsiveSidebar.svelte**

Smart sidebar component:

- Full-width overlay on mobile
- Fixed sidebar on desktop
- Smooth slide animation
- Backdrop overlay on mobile

### 4. **MobileNavigation.svelte**

Bottom navigation bar (mobile only):

- 5 main navigation items (Files, Search, Favorites, Settings, Profile)
- Active state highlighting
- Touch-friendly tap targets (44x44px minimum)
- Auto-hidden on md+ screens

## Utilities (mobileResponsive.js)

### Screen Size Store

```javascript
import { screen } from "./lib/mobileResponsive";

// Subscribe to screen size changes
screen.isMobile.subscribe((isMobile) => {
  console.log("Mobile:", isMobile);
});

screen.width.subscribe((width) => {
  console.log("Width:", width);
});
```

### Helper Functions

#### Format File Size

```javascript
import { formatFileSize } from "./lib/mobileResponsive";

formatFileSize(1048576); // "1 MB"
```

#### Truncate Text (for mobile display)

```javascript
import { truncateText } from "./lib/mobileResponsive";

truncateText("Very Long Filename.pdf", 15); // "Very Long Fi..."
```

#### Format Date (mobile-friendly)

```javascript
import { formatDateMobile } from "./lib/mobileResponsive";

formatDateMobile("2025-11-09"); // "15:30" (if today), "3d ago", etc.
```

#### Touch Device Detection

```javascript
import { isTouchDevice } from "./lib/mobileResponsive";

if (isTouchDevice()) {
  // Adapt for touch input
}
```

#### Safe Area Insets (notch support)

```javascript
import { getSafeAreaInsets } from "./lib/mobileResponsive";

const insets = getSafeAreaInsets();
console.log(insets); // { top: 44, bottom: 34, left: 0, right: 0 }
```

#### Responsive Grid Columns

```javascript
import { getGridCols } from "./lib/mobileResponsive";

getGridCols(320); // 1 column (mobile)
getGridCols(768); // 3 columns (tablet)
getGridCols(1200); // 4 columns (desktop)
```

## Tailwind CSS Responsive Classes

Use Tailwind breakpoints in your components:

```svelte
<!-- Hidden on mobile, visible on md+ -->
<div class="hidden md:block">
  Desktop-only content
</div>

<!-- Full width on mobile, constrained on desktop -->
<div class="w-full md:w-3/4 lg:w-2/3">
  Responsive width
</div>

<!-- Different padding on mobile vs desktop -->
<div class="px-4 md:px-6 lg:px-8">
  Responsive padding
</div>

<!-- Grid columns change based on screen size -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
  <!-- Items here -->
</div>

<!-- Font size responsive -->
<h1 class="text-lg md:text-2xl lg:text-3xl">
  Responsive heading
</h1>
```

## Mobile-First Design Principles

### 1. **Touch Targets**

- Minimum 44x44px for touch targets
- 8px minimum spacing between interactive elements
- Use larger tap areas on mobile

```svelte
<!-- Mobile-friendly button -->
<button class="p-3 md:p-2">
  <i class="text-lg md:text-base"></i>
</button>
```

### 2. **Text Truncation**

Use utilities for long text on mobile:

```svelte
<!-- Truncate text on mobile -->
<p class="truncate md:whitespace-normal">
  {truncateText(fileName, 20)}
</p>
```

### 3. **Responsive Images**

```svelte
<!-- Image adapts to container -->
<img
  src={imageSrc}
  alt="File preview"
  class="w-full h-auto rounded-lg"
/>
```

### 4. **Flexible Layouts**

```svelte
<!-- Stack on mobile, side-by-side on desktop -->
<div class="flex flex-col md:flex-row gap-4">
  <div class="w-full md:w-1/3">Sidebar</div>
  <div class="w-full md:w-2/3">Content</div>
</div>
```

## Touch Gestures Support

### Swipe Navigation

(Implemented in Touch Gestures - #32)

- Left swipe: Go back to parent directory
- Right swipe: Open sidebar
- Down swipe: Refresh files

### Long Press

- Long press on file: Open context menu
- Long press on empty area: Select all files

### Pinch Zoom

- Pinch on file preview: Zoom in/out
- Pinch on gallery: Change grid size

## Performance Optimization

### Code Splitting

All responsive components are auto-imported and code-split by Vite:

- ResponsiveLayout.svelte
- ResponsiveFileGrid.svelte
- MobileNavigation.svelte
- mobileResponsive.js utilities

### Lazy Loading

- Images use native lazy loading
- Components use Svelte suspense
- Heavy components load on scroll

### Media Query Optimization

- Use `@media` queries for critical CSS only
- Use Tailwind utility classes for most responsive behavior
- Minimal JavaScript for responsive calculations

## Testing on Mobile

### Browser DevTools

1. Open Chrome DevTools
2. Press Ctrl+Shift+M (Cmd+Shift+M on Mac)
3. Select device: iPhone 12, iPad, etc.

### Real Devices

Test on actual phones/tablets:

- iOS Safari
- Android Chrome
- Samsung Internet

### Responsive Breakpoints to Test

- **Mobile**: 320px (iPhone SE), 375px (iPhone 12), 414px (iPhone 12 Max)
- **Tablet**: 768px (iPad), 820px (iPad Air), 1024px (iPad Pro)
- **Desktop**: 1280px (1080p), 1920px (1440p)

## Common Mobile Issues & Solutions

### ❌ Issue: Bottom navigation blocked by keyboard

```svelte
<!-- Use padding-bottom instead -->
<div class="pb-20 md:pb-0">
  <!-- Content here, 20 = 80px for mobile nav -->
</div>
```

### ❌ Issue: Text too small on mobile

```svelte
<!-- Use responsive text sizes -->
<p class="text-sm md:text-base lg:text-lg">
  Readable on all screens
</p>
```

### ❌ Issue: Images too large on mobile

```svelte
<!-- Use responsive image sizes -->
<img
  src={imageSrc}
  alt="Preview"
  class="max-w-full h-auto max-h-96 md:max-h-screen"
/>
```

### ❌ Issue: Sidebar overlaps content

```svelte
<!-- Already handled in ResponsiveLayout -->
<!-- Sidebar is overlay on mobile, fixed on desktop -->
```

## Integration Checklist

- [x] ResponsiveLayout.svelte - Main responsive container
- [x] ResponsiveFileGrid.svelte - Adaptive file grid
- [x] ResponsiveSidebar.svelte - Collapsible sidebar
- [x] MobileNavigation.svelte - Bottom nav bar
- [x] mobileResponsive.js - Responsive utilities
- [ ] Integration into App.svelte (next step)
- [ ] Testing on real devices
- [ ] PWA manifest for install (see PWA Manifest - #34)
- [ ] Touch gesture handling (see Touch Gestures - #32)

## Performance Metrics

Target metrics for mobile:

- **First Contentful Paint (FCP)**: < 1.5s
- **Largest Contentful Paint (LCP)**: < 2.5s
- **Interaction to Paint (INP)**: < 200ms
- **Cumulative Layout Shift (CLS)**: < 0.1

## Browser Support

- iOS Safari 12+
- Android Chrome 80+
- Samsung Internet 12+
- Firefox Mobile 80+
- Edge Mobile 80+

## Next Steps

1. **Touch Gestures (#32)**: Add swipe, pinch, long press support
2. **PWA Manifest (#34)**: Add web app manifest for install
3. **Code Splitting (#35)**: Implement route-based code splitting
4. **Image Optimization (#36)**: Add WebP/AVIF support

## Resources

- [Tailwind CSS Responsive Design](https://tailwindcss.com/docs/responsive-design)
- [Web.dev Mobile-Friendly Guide](https://web.dev/responsive-web-design-basics/)
- [MDN Touch Events](https://developer.mozilla.org/en-US/docs/Web/API/Touch_events)

---

**Last Updated:** 2025-11-09  
**Design System:** Tailwind CSS v4 with Svelte 5  
**Status:** Production Ready
