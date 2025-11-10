# SyncSpace Component Library

A comprehensive, production-ready UI component library built with **Svelte 5** (runes), **TypeScript**, and **Tailwind CSS v4**. Designed following atomic design principles for maximum reusability and consistency.

## 🎯 Overview

This component library provides a complete set of building blocks for the SyncSpace application:

- **20+ Components** organized by complexity (atoms → molecules → organisms)
- **100% TypeScript** support with full prop validation
- **Tailwind CSS v4** for styling with centralized design tokens
- **Svelte 5 Runes** for optimal reactivity
- **Accessibility-first** design with semantic HTML
- **Extensively Customizable** through props and CSS classes

## 📁 Directory Structure

```
frontend_components/
├── atoms/                    # Basic building blocks
│   ├── Avatar.svelte        # User avatars with auto-colors
│   ├── Badge.svelte         # Status badges (6 variants)
│   ├── Button.svelte        # Buttons (7 variants, 5 sizes)
│   ├── Card.svelte          # Flexible card containers
│   ├── Checkbox.svelte      # Checkbox inputs
│   ├── Divider.svelte       # Visual separators
│   ├── Input.svelte         # Text input fields
│   ├── Label.svelte         # Form labels
│   ├── Toggle.svelte        # Switch toggles
│   └── index.ts             # Re-exports all atoms
├── molecules/               # Complex combinations
│   ├── Breadcrumbs.svelte   # Navigation breadcrumbs
│   ├── ContextMenu.svelte   # Right-click menus
│   ├── Filter.svelte        # Multi-select filters
│   ├── Select.svelte        # Dropdown selects
│   ├── Toast.svelte         # Toast notifications
│   └── index.ts             # Re-exports all molecules
├── organisms/               # Feature-complete components
│   ├── FileViewer.svelte    # Multi-type file preview
│   ├── Modal.svelte         # Modal dialogs
│   └── index.ts             # Re-exports all organisms
├── pages/                   # Demo and showcase pages
│   ├── AtomsDemo.svelte     # Atoms showcase
│   ├── DemoHome.svelte      # Home/overview page
│   ├── MoleculesDemo.svelte # Molecules showcase
│   ├── OrganismsDemo.svelte # Organisms showcase
│   └── index.ts             # Re-exports all pages
└── shared/                  # Design tokens & utilities
    └── index.ts             # Colors, sizes, transitions, types
```

## 🎨 Design System

### Color Palette

All components use a centralized color system with 7 variants:

- **primary** - Blue (#3B82F6) - Primary actions
- **secondary** - Slate (#64748B) - Secondary elements
- **danger** - Red (#EF4444) - Destructive actions
- **success** - Green (#10B981) - Positive feedback
- **warning** - Amber (#F59E0B) - Warnings/alerts
- **info** - Cyan (#06B6D4) - Information
- **ghost** - Transparent - Minimal styling

### Sizes

Consistent sizing across all interactive components:

```typescript
type ButtonSize = "xs" | "sm" | "md" | "lg" | "xl";

// Maps to: px, py, and text-size combinations
// xs:  px-2 py-1 text-xs
// sm:  px-3 py-2 text-sm
// md:  px-4 py-2 text-base
// lg:  px-6 py-3 text-lg
// xl:  px-8 py-4 text-xl
```

### Transitions

Consistent animation timings:

```typescript
fast:   150ms  // Quick interactions
normal: 300ms  // Default transitions
slow:   500ms  // Emphasized animations
```

## 📚 Component Documentation

### Atoms

#### Button

```svelte
<script>
  import { Button } from '../atoms/index.ts';
</script>

<Button
  variant="primary"           // 'primary' | 'secondary' | 'danger' | 'success' | 'warning' | 'ghost' | 'outline'
  size="md"                   // 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  disabled={false}            // boolean
  loading={false}             // boolean - shows spinner
  fullWidth={false}           // boolean
  icon="bi-star-fill"         // Bootstrap icon class (optional)
  href="/path"                // Link target (optional)
  on:click={handler}          // Click handler
>
  Click me
</Button>
```

#### Badge

```svelte
<Badge
  variant="primary"           // 6 variants
  size="md"                   // 'sm' | 'md' | 'lg'
  outline={false}             // boolean
  icon="bi-check-circle"      // Bootstrap icon (optional)
>
  Badge Text
</Badge>
```

#### Avatar

```svelte
<Avatar
  name="John Doe"             // Auto-generates initials & color
  size="md"                   // 'sm' | 'md' | 'lg'
  online={false}              // Shows online indicator
  icon="bi-person"            // Fallback icon
/>
```

#### Input

```svelte
<Input
  variant="primary"           // 'primary' | 'secondary' | 'danger'
  placeholder="Enter text..."
  disabled={false}
  error="Error message"       // Shows error state
  bind:value={inputValue}
/>
```

#### Checkbox

```svelte
<Checkbox
  label="Accept terms"
  variant="primary"           // 'primary' | 'secondary' | 'danger'
  size="md"                   // 'sm' | 'md' | 'lg'
  disabled={false}
  bind:checked={isChecked}
/>
```

#### Toggle

```svelte
<Toggle
  label="Enable notifications"
  variant="primary"           // 'primary' | 'success' | 'danger' | 'warning'
  disabled={false}
  bind:checked={isEnabled}
/>
```

#### Card

```svelte
<Card
  title="Card Title"
  description="Optional description"
  hoverable={false}           // Adds hover effect
  bordered={false}            // Shows border
  shadow="md"                 // 'sm' | 'md' | 'lg'
>
  <!-- Slot: main content -->
  <p>Card content goes here</p>

  <!-- Named slot: footer -->
  <div slot="footer">
    Footer content (optional)
  </div>
</Card>
```

### Molecules

#### Toast

```svelte
<script>
  import Toast from '../molecules/Toast.svelte';

  let toastComponent;

  function showMessage() {
    toastComponent.show('Success!', 'success', 3000);
    // Types: 'success' | 'error' | 'warning' | 'info'
    // Positions: 'top-left' | 'top-center' | 'top-right'
    //           | 'bottom-left' | 'bottom-center' | 'bottom-right'
  }
</script>

<Toast
  bind:this={toastComponent}
  position="bottom-right"
/>

<button on:click={showMessage}>Show Toast</button>
```

#### Breadcrumbs

```svelte
<Breadcrumbs
  items={[
    { label: 'Home', href: '/' },
    { label: 'Products', href: '/products' },
    { label: 'Electronics', href: '/products/electronics' }
  ]}
  current="iPhone 15"
  separator="/"              // Visual separator
/>
```

#### Filter

```svelte
<script>
  let selectedFilters = [];

  function handleChange(filters) {
    selectedFilters = filters;
  }
</script>

<Filter
  items={[
    { id: 'active', label: 'Active' },
    { id: 'archived', label: 'Archived' }
  ]}
  selected={selectedFilters}
  onChange={handleChange}
/>
```

#### Select

```svelte
<Select
  items={[
    { value: 'opt1', label: 'Option 1' },
    { value: 'opt2', label: 'Option 2' }
  ]}
  placeholder="Choose option..."
  disabled={false}
  bind:selected={selectedValue}
/>
```

#### ContextMenu

```svelte
<div on:contextmenu|preventDefault={handleRightClick}>
  Right-click me
</div>

<ContextMenu
  visible={menuVisible}
  x={mouseX}
  y={mouseY}
  items={[
    { id: 'edit', label: 'Edit', icon: 'bi-pencil' },
    { id: 'delete', label: 'Delete', icon: 'bi-trash', dangerous: true }
  ]}
  on:select={(e) => handleAction(e.detail)}
  on:close={() => menuVisible = false}
/>
```

### Organisms

#### Modal

```svelte
<script>
  let isOpen = false;
</script>

<Modal
  open={isOpen}
  title="Confirm Action"
  size="md"                   // 'sm' | 'md' | 'lg' | 'xl'
  closeButton={true}
  on:close={() => isOpen = false}
>
  <!-- Main content -->
  <p>Are you sure?</p>

  <!-- Footer actions -->
  <div slot="footer">
    <button on:click={() => isOpen = false}>Cancel</button>
    <button on:click={handleConfirm}>Confirm</button>
  </div>
</Modal>

<button on:click={() => isOpen = true}>Open Modal</button>
```

#### FileViewer

```svelte
<FileViewer
  file={{
    type: 'text',           // 'text' | 'code' | 'image' | 'pdf' | 'video' | 'audio' | 'file'
    name: 'document.txt',
    size: '2.4 KB',
    content: '...',         // For text/code files
    url: '...',             // For images/videos/audio/pdf
    language: 'javascript'  // Optional: for syntax highlighting
  }}
/>
```

## 🚀 Usage Examples

### Importing Components

```typescript
// Import individual components
import Button from "../atoms/Button.svelte";
import Badge from "../atoms/Badge.svelte";

// Or import from index (re-exports)
import { Button, Badge } from "../atoms/index.ts";

// Or import from molecules
import { Toast, Filter } from "../molecules/index.ts";

// Or import organisms
import { Modal, FileViewer } from "../organisms/index.ts";
```

### Building Complex UI

```svelte
<script>
  import { Button, Badge } from '../atoms/index.ts';
  import { Breadcrumbs, Toast } from '../molecules/index.ts';

  let toastComponent;

  function handleSave() {
    // Save logic
    toastComponent.show('Changes saved!', 'success', 3000);
  }
</script>

<Breadcrumbs items={navItems} current="Settings" />

<div class="flex gap-4 items-center">
  <h1>User Settings</h1>
  <Badge variant="success">Active</Badge>
</div>

<div class="flex gap-2 mt-8">
  <Button variant="primary" on:click={handleSave}>
    <i class="bi bi-check mr-2"></i>Save
  </Button>
  <Button variant="secondary">Cancel</Button>
</div>

<Toast bind:this={toastComponent} position="bottom-right" />
```

## 🎯 Component Variants Quick Reference

| Component | Variants                                                     |
| --------- | ------------------------------------------------------------ |
| Button    | primary, secondary, danger, success, warning, ghost, outline |
| Badge     | primary, secondary, danger, success, warning, info           |
| Toggle    | primary, success, danger, warning                            |
| Input     | primary, secondary, danger                                   |
| Checkbox  | primary, secondary, danger                                   |
| Avatar    | (auto-colors based on name)                                  |
| Toast     | success, error, warning, info                                |
| Modal     | sm, md, lg, xl                                               |

## 🔧 Customization

### Global Theme

Customize colors by modifying `/shared/index.ts`:

```typescript
export const colorMap = {
  primary: "from-blue-500 to-blue-600",
  secondary: "from-slate-500 to-slate-600",
  // ... customize as needed
};
```

### Component-Level Props

All components accept `class` prop for additional Tailwind classes:

```svelte
<Button class="custom-class">Button</Button>
<Badge class="shadow-lg">Badge</Badge>
```

### Svelte 5 Runes

Modify reactive state using Svelte 5 patterns:

```svelte
<script>
  import { Button } from '../atoms/index.ts';

  let count = $state(0);
  let doubled = $derived(count * 2);

  function increment() {
    count++;
  }
</script>

<Button on:click={increment}>Count: {count}</Button>
<p>Doubled: {doubled}</p>
```

## 📖 Demo Pages

The library includes 4 comprehensive demo pages:

1. **DemoHome** - Overview and statistics
2. **AtomsDemo** - Interactive showcase of all atoms
3. **MoleculesDemo** - Complex component interactions
4. **OrganismsDemo** - Feature-complete organisms

View these in your Svelte app routing to see all components in action.

## ✨ Features

- ✅ **Atomic Design** - Components organized by complexity
- ✅ **Full TypeScript** - Complete type safety
- ✅ **Svelte 5 Runes** - Modern reactive patterns
- ✅ **Tailwind v4** - Pure utility-first styling
- ✅ **Accessibility** - Semantic HTML with ARIA attributes
- ✅ **Responsive** - Mobile-first design
- ✅ **Customizable** - Props and CSS class support
- ✅ **Dark Mode Ready** - Built for theme support
- ✅ **Bootstrap Icons** - 2000+ included icons
- ✅ **Production Ready** - Thoroughly tested

## 🛠️ Development

### Adding New Components

1. Create `.svelte` file in appropriate directory (atoms/molecules/organisms)
2. Define TypeScript interfaces for props
3. Implement component using Svelte 5 runes
4. Use design tokens from `shared/index.ts`
5. Add to corresponding `index.ts` for re-export
6. Add demo to showcase pages

### Component Template

```svelte
<script lang="ts">
  interface Props {
    variant?: 'primary' | 'secondary';
    size?: 'sm' | 'md' | 'lg';
    class?: string;
  }

  let {
    variant = 'primary',
    size = 'md',
    class: className = '',
    ...rest
  }: Props = $props();
</script>

<button class={`base-classes ${className}`} {...rest}>
  <slot />
</button>
```

## 📝 License

Part of SyncSpace - Modern Self-Hosted File Synchronization Service

## 🤝 Contributing

When adding components:

1. Follow atomic design principles
2. Use TypeScript for all props
3. Include comprehensive prop documentation
4. Add demo showcase
5. Ensure accessibility compliance
6. Test all variants and edge cases

---

**Built with ❤️ using Svelte 5 & Tailwind CSS v4**
