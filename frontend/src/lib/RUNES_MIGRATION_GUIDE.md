# Svelte 5 Runes Migration Guide

## Overview

This document provides a complete guide for migrating SyncSpace frontend components from Svelte 4 (reactive declarations) to Svelte 5 (runes).

## Migration Status

- ✅ **Already Using Runes**: App.svelte, StorageQuotaView.svelte, ActivityFeedView.svelte, FileStatisticsView.svelte, ThemeCustomizationView.svelte, BackupRestoreView.svelte
- 🔄 **Ready to Migrate**: All Wave 1-2 components (FilesView, Sidebar, AppHeader, FileContextMenu, etc.)
- 📋 **Pattern Library Below**: Copy-paste templates for common migration scenarios

---

## Quick Conversion Patterns

### 1. **Reactive Declaration** → **$state()**

#### Old Svelte 4:

```svelte
<script>
  let count = 0;
  let name = '';
  let items = [];
</script>
```

#### New Svelte 5:

```svelte
<script>
  let count = $state(0);
  let name = $state('');
  let items = $state([]);
</script>
```

---

### 2. **Reactive Expression** → **$derived()**

#### Old Svelte 4:

```svelte
<script>
  let count = 0;
  $: doubled = count * 2;
  $: computedValue = expensive(count);
</script>
```

#### New Svelte 5:

```svelte
<script>
  let count = $state(0);
  let doubled = $derived(count * 2);
  let computedValue = $derived(expensive(count));
</script>
```

---

### 3. **Multiple Dependencies** → **$derived.by()**

#### Old Svelte 4:

```svelte
<script>
  let x = 0;
  let y = 0;
  $: distance = {
    value: Math.sqrt(x * x + y * y),
    x,
    y
  };
</script>
```

#### New Svelte 5:

```svelte
<script>
  let x = $state(0);
  let y = $state(0);
  let distance = $derived.by(() => ({
    value: Math.sqrt(x * x + y * y),
    x,
    y
  }));
</script>
```

---

### 4. **Side Effects** → **$effect()**

#### Old Svelte 4:

```svelte
<script>
  import { onMount } from 'svelte';
  let data = [];

  onMount(async () => {
    data = await fetchData();
  });

  $: if (data.length > 0) {
    console.log('Data loaded:', data);
  }
</script>
```

#### New Svelte 5:

```svelte
<script>
  let data = $state([]);

  $effect(async () => {
    data = await fetchData();
  });

  $effect(() => {
    if (data.length > 0) {
      console.log('Data loaded:', data);
    }
  });
</script>
```

---

### 5. **Props with Defaults** → **Props with `$state()` Binding**

#### Old Svelte 4:

```svelte
<script>
  export let title = 'Default';
  export let count = 0;
</script>
```

#### New Svelte 5:

```svelte
<script>
  let { title = 'Default', count = 0 } = $props();
</script>
```

---

### 6. **Two-Way Binding** → **`bind:property`** (unchanged in usage, but with runes)

#### Old Svelte 4:

```svelte
<script>
  let value = 'hello';
</script>

<input bind:value />
```

#### New Svelte 5:

```svelte
<script>
  let value = $state('hello');
</script>

<input bind:value />
```

---

### 7. **Props Reactivity** → **Runes + $effect.pre()**

#### Old Svelte 4:

```svelte
<script>
  export let userId;
  let userData = {};

  $: if (userId) {
    fetchUser(userId).then(data => userData = data);
  }
</script>
```

#### New Svelte 5:

```svelte
<script>
  let { userId } = $props();
  let userData = $state({});

  $effect.pre(() => {
    if (userId) {
      fetchUser(userId).then(data => userData = data);
    }
  });
</script>
```

---

### 8. **Store Subscription** → **Keep unchanged** (stores remain the same)

#### Both Svelte 4 & 5:

```svelte
<script>
  import { myStore } from '../stores/myStore';
</script>

<!-- Auto-subscribe with $ prefix -->
<p>{$myStore.value}</p>
```

---

## Component Migration Checklist

For each Wave 1-2 component, follow this checklist:

- [ ] Replace `let x = ...` with `let x = $state(...)`
- [ ] Replace `$: ...` with `$derived()` or `$effect()`
- [ ] Replace `export let` with `let { ... } = $props()`
- [ ] Replace `onMount/onDestroy` with `$effect()`
- [ ] Update `<script>` to use new patterns
- [ ] Test component in browser
- [ ] Run error checks: `get_errors` tool
- [ ] Commit changes

---

## Real Component Examples

### FilesView.svelte Migration Template

#### Before (Svelte 4):

```svelte
<script>
  import { onMount, onDestroy } from 'svelte';
  import { currentDir } from './stores/files';

  export let sortBy = 'name';
  export let sortOrder = 'asc';

  let files = [];
  let isLoading = false;
  let selectedFiles = new Set();

  $: currentDirValue = $currentDir;
  $: sortedFiles = sortFiles(files, sortBy, sortOrder);

  $: if (currentDirValue) {
    loadFiles(currentDirValue);
  }

  async function loadFiles(dir) {
    isLoading = true;
    files = await api.files.list(dir);
    isLoading = false;
  }

  onMount(() => {
    console.log('FilesView mounted');
  });

  onDestroy(() => {
    console.log('FilesView destroyed');
  });
</script>
```

#### After (Svelte 5):

```svelte
<script>
  import { currentDir } from './stores/files';

  let { sortBy = 'name', sortOrder = 'asc' } = $props();

  let files = $state([]);
  let isLoading = $state(false);
  let selectedFiles = $state(new Set());

  let currentDirValue = $derived($currentDir);
  let sortedFiles = $derived(sortFiles(files, sortBy, sortOrder));

  $effect(() => {
    if (currentDirValue) {
      loadFiles(currentDirValue);
    }
  });

  async function loadFiles(dir) {
    isLoading = true;
    files = await api.files.list(dir);
    isLoading = false;
  }

  $effect(() => {
    console.log('FilesView mounted');
    return () => console.log('FilesView destroyed');
  });
</script>
```

---

### Sidebar.svelte Migration Template

#### Before (Svelte 4):

```svelte
<script>
  export let isOpen = true;
  export let width = 280;

  let isCollapsed = false;
  let animating = false;

  function toggle() {
    animating = true;
    isCollapsed = !isCollapsed;
    setTimeout(() => animating = false, 300);
  }

  $: collapsedWidth = isCollapsed ? 0 : width;
</script>
```

#### After (Svelte 5):

```svelte
<script>
  let { isOpen = true, width = 280 } = $props();

  let isCollapsed = $state(false);
  let animating = $state(false);

  function toggle() {
    animating = true;
    isCollapsed = !isCollapsed;
    setTimeout(() => animating = false, 300);
  }

  let collapsedWidth = $derived(isCollapsed ? 0 : width);
</script>
```

---

## Common Pitfalls & Solutions

### ❌ Pitfall 1: Mutating objects without reassignment

```svelte
<!-- WRONG - State won't update -->
<script>
  let obj = $state({ count: 0 });
  obj.count = 1; // ❌ No reactivity
</script>

<!-- RIGHT - Reassign object -->
<script>
  let obj = $state({ count: 0 });
  obj = { ...obj, count: 1 }; // ✅ Reactive
</script>
```

### ❌ Pitfall 2: Arrays mutation

```svelte
<!-- WRONG - Array methods don't trigger updates -->
<script>
  let items = $state([1, 2, 3]);
  items.push(4); // ❌ Not reactive
</script>

<!-- RIGHT - Reassign array -->
<script>
  let items = $state([1, 2, 3]);
  items = [...items, 4]; // ✅ Reactive
</script>
```

### ❌ Pitfall 3: Accessing other runes in class

```svelte
<!-- WRONG - Runes only work in component root -->
<script>
  class MyClass {
    value = $state(0); // ❌ Error!
  }
</script>

<!-- RIGHT - Use runes in component only -->
<script>
  let value = $state(0); // ✅ Correct
  class MyClass {
    doSomething() {
      // Access value from outer scope
      return value;
    }
  }
</script>
```

---

## Performance Tips

1. **Use `$derived()` for simple computations** (single expression)

   ```svelte
   let doubled = $derived(count * 2); // ✅ Efficient
   ```

2. **Use `$derived.by()` for complex logic**

   ```svelte
   let result = $derived.by(() => {
     const temp = expensiveCalc();
     return temp + count;
   });
   ```

3. **Minimize `$effect()` scope** - Only track needed dependencies

   ```svelte
   $effect(() => {
     console.log(count); // Only re-runs when count changes
   });
   ```

4. **Avoid nested `$effect()` calls** - Refactor to separate effects
   ```svelte
   // ✅ Two separate effects instead of nested
   $effect(() => { /* effect 1 */ });
   $effect(() => { /* effect 2 */ });
   ```

---

## Testing & Validation

After migrating a component:

```bash
# 1. Check for syntax errors
get_errors /path/to/component.svelte

# 2. Run component in browser
npm run dev

# 3. Test all props and state changes
# - Try binding props
# - Test user interactions
# - Verify derived values update correctly

# 4. Check performance
# - Use browser DevTools
# - Look for unnecessary re-renders
```

---

## Automated Migration Helpers

Run these tools to assist with migration:

```bash
# Find all old-style reactive declarations in a file
grep -n '\$:' frontend/src/components/*.svelte

# Find all export declarations (need $props())
grep -n '^  export let' frontend/src/components/*.svelte

# Find onMount/onDestroy (need $effect())
grep -n 'onMount\|onDestroy' frontend/src/components/*.svelte
```

---

## Migration Priority Queue

### 🔴 **High Priority** (Core components, used everywhere):

1. `FilesView.svelte` - Main file browser
2. `Sidebar.svelte` - Navigation
3. `AppHeader.svelte` - Top bar
4. `FileContextMenu.svelte` - Right-click menu
5. `BatchMoveDialog.svelte` - Batch operations

### 🟠 **Medium Priority** (Important features):

6. `AdvancedSearchFiltersPanel.svelte`
7. `SavedSearchesPanel.svelte`
8. UI components in `components/ui/`

### 🟡 **Low Priority** (Nice to have):

- Dev/showcase components
- Rarely-used modals
- Test/debug components

---

## Support & Questions

If you encounter issues during migration:

1. **Check the Pitfalls section** above first
2. **Review the templates** for your component type
3. **Test in browser** with `npm run dev`
4. **Run error checks** to catch syntax issues
5. **Refer to official docs**: https://svelte.dev/docs/runes

---

## Completion Tracking

After completing each component migration:

```bash
# 1. Document the change
git commit -m "Refactor: Migrate [ComponentName].svelte to Svelte 5 runes"

# 2. Test thoroughly
npm run dev  # Manual testing

# 3. Update this file with completed component
```

**Completed Migrations:**

- ✅ App.svelte (Wave 0 - Setup)
- ✅ StorageQuotaView.svelte (Wave 3 - #23)
- ✅ ActivityFeedView.svelte (Wave 3 - #27)
- ✅ FileStatisticsView.svelte (Wave 3 - #28)
- ✅ ThemeCustomizationView.svelte (Wave 3 - #26)
- ✅ BackupRestoreView.svelte (Wave 3 - #24)

**To Migrate (Pick any):**

- [ ] FilesView.svelte
- [ ] Sidebar.svelte
- [ ] AppHeader.svelte
- [ ] FileContextMenu.svelte
- [ ] BatchMoveDialog.svelte
- [ ] (And 50+ more Wave 1-2 components)

---

**Last Updated:** 2025-11-09  
**Guide Version:** 1.0  
**SyncSpace Frontend:** Svelte 5 ready
