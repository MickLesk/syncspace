# Svelte 5 Event Handler Fixes

## Changes Made

### 1. **HTML Elements** (Native DOM elements)
- Changed `on:click=` to `onclick=`
- Changed `on:contextmenu=` to `oncontextmenu=`
- Changed `on:click|stopPropagation` to `onclick={(e) => e.stopPropagation()}`

### 2. **Files Fixed**
✅ `App.svelte` - Navigation button clicks
✅ `pages/DemoHome.svelte` - Section button clicks
✅ `molecules/Select.svelte` - Dropdown click handlers
✅ `organisms/Modal.svelte` - Modal backdrop clicks
✅ `pages/MoleculesDemo.svelte` - All demo buttons
✅ `pages/OrganismsDemo.svelte` - All demo buttons

### 3. **Custom Component Events** (Preserved)
- `on:close=` - Modal close event
- `on:select=` - Select/ContextMenu event
- These remain with `on:` prefix as they are custom component events, not DOM events

### 4. **Component Props** (Unchanged)
- `on:input`, `on:change`, `on:blur` in `Input.svelte`
- `on:change` in `Toggle.svelte` and `Checkbox.svelte`
- These are forwarded via `$$restProps` and work correctly

## Testing
Dev server running on http://localhost:5174
All components should render without event handler conflicts.

## Svelte 5 Event Handler Rules
- HTML native events: `onXXX={handler}` or `onXXX` attribute
- Custom component events: `on:eventName={handler}`
- Event modifiers (click outside): Manual handling with `e.stopPropagation()` etc.
