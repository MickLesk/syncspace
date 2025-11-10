# Component API Reference

Complete API documentation for all SyncSpace UI components.

## Table of Contents

- [Atoms](#atoms)
  - [Button](#button)
  - [Badge](#badge)
  - [Avatar](#avatar)
  - [Card](#card)
  - [Input](#input)
  - [Checkbox](#checkbox)
  - [Toggle](#toggle)
  - [Label](#label)
  - [Divider](#divider)
- [Molecules](#molecules)
  - [Breadcrumbs](#breadcrumbs)
  - [Toast](#toast)
  - [Filter](#filter)
  - [Select](#select)
  - [ContextMenu](#contextmenu)
- [Organisms](#organisms)
  - [Modal](#modal)
  - [FileViewer](#fileviewer)

---

## Atoms

### Button

A versatile button component with 7 variants, 5 sizes, and various states.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `'primary'\|'secondary'\|'danger'\|'success'\|'warning'\|'ghost'\|'outline'` | `'primary'` | Button style variant |
| `size` | `'xs'\|'sm'\|'md'\|'lg'\|'xl'` | `'md'` | Button size |
| `disabled` | `boolean` | `false` | Disable button interaction |
| `loading` | `boolean` | `false` | Show loading spinner |
| `fullWidth` | `boolean` | `false` | Stretch to full width |
| `icon` | `string` | `undefined` | Bootstrap icon class |
| `href` | `string` | `undefined` | Render as link instead of button |
| `class` | `string` | `''` | Additional Tailwind classes |

**Events:**

- `click` - Fired when button is clicked

**Slots:**

- `default` - Button content

**Example:**

```svelte
<Button variant="primary" size="lg" on:click={handleClick}>
  <i class="bi bi-save mr-2"></i>Save Changes
</Button>
```

---

### Badge

A small, colorful badge component for labels and status indicators.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `'primary'\|'secondary'\|'danger'\|'success'\|'warning'\|'info'` | `'primary'` | Badge color variant |
| `size` | `'sm'\|'md'\|'lg'` | `'md'` | Badge size |
| `outline` | `boolean` | `false` | Use outline style instead of filled |
| `icon` | `string` | `undefined` | Bootstrap icon class |
| `class` | `string` | `''` | Additional Tailwind classes |

**Slots:**

- `default` - Badge content

**Example:**

```svelte
<Badge variant="success" size="sm">
  <i class="bi bi-check-circle mr-1"></i>Active
</Badge>
```

---

### Avatar

User avatar component with auto-generated initials and color coding.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `string` | `undefined` | User name (generates initials + color) |
| `size` | `'sm'\|'md'\|'lg'` | `'md'` | Avatar size |
| `online` | `boolean` | `false` | Show online status indicator |
| `icon` | `string` | `'bi-person'` | Fallback Bootstrap icon |
| `class` | `string` | `''` | Additional Tailwind classes |

**Features:**

- Auto-generates initials from first and last name
- Assigns color based on first character hash
- Optional online status dot
- 6-color gradient palette

**Example:**

```svelte
<Avatar name="Alice Johnson" size="md" online={true} />
```

---

### Card

Flexible card container with optional header, footer, and interactive states.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `string` | `undefined` | Card header title |
| `description` | `string` | `undefined` | Card header description |
| `hoverable` | `boolean` | `false` | Add hover effect |
| `bordered` | `boolean` | `false` | Show card border |
| `shadow` | `'sm'\|'md'\|'lg'\|'xl'` | `'md'` | Shadow intensity |
| `footer` | `boolean\|string` | `false` | Show footer section |
| `class` | `string` | `''` | Additional Tailwind classes |

**Slots:**

- `default` - Main card content
- `footer` - Footer content (if footer prop enabled)

**Example:**

```svelte
<Card title="User Profile" hoverable={true}>
  <p>User information here</p>
  <div slot="footer">
    <Button size="sm">Edit</Button>
  </div>
</Card>
```

---

### Input

Text input field with validation states and multiple variants.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `'primary'\|'secondary'\|'danger'` | `'primary'` | Input variant (affects border color) |
| `placeholder` | `string` | `''` | Placeholder text |
| `value` | `string` | `''` | Input value |
| `disabled` | `boolean` | `false` | Disable input |
| `error` | `string` | `undefined` | Error message (shows error state) |
| `type` | `string` | `'text'` | Input type (text, email, password, etc.) |
| `class` | `string` | `''` | Additional Tailwind classes |

**Events:**

- `input` - Fired when value changes
- `change` - Fired when input loses focus
- `focus` - Fired when input gains focus
- `blur` - Fired when input loses focus

**Example:**

```svelte
<script>
  let email = '';
</script>

<Input
  type="email"
  placeholder="Enter email..."
  bind:value={email}
  error={!email.includes('@') ? 'Invalid email' : undefined}
/>
```

---

### Checkbox

Native checkbox with custom styling and variants.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `boolean` | `false` | Checkbox state |
| `label` | `string` | `undefined` | Label text |
| `variant` | `'primary'\|'secondary'\|'danger'` | `'primary'` | Checkbox variant |
| `size` | `'sm'\|'md'\|'lg'` | `'md'` | Checkbox size |
| `disabled` | `boolean` | `false` | Disable checkbox |
| `icon` | `string` | `undefined` | Bootstrap icon class |
| `class` | `string` | `''` | Additional Tailwind classes |

**Events:**

- `change` - Fired when checked state changes

**Example:**

```svelte
<script>
  let accepted = false;
</script>

<Checkbox
  label="I accept the terms"
  bind:checked={accepted}
/>
```

---

### Toggle

Switch-style toggle component for boolean options.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `boolean` | `false` | Toggle state |
| `label` | `string` | `undefined` | Label text |
| `variant` | `'primary'\|'success'\|'danger'\|'warning'` | `'primary'` | Toggle variant |
| `disabled` | `boolean` | `false` | Disable toggle |
| `class` | `string` | `''` | Additional Tailwind classes |

**Events:**

- `change` - Fired when toggle state changes

**Example:**

```svelte
<script>
  let darkMode = false;
</script>

<Toggle label="Dark mode" bind:checked={darkMode} />
```

---

### Label

Form label with optional validation states and hints.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `label` | `string` | `''` | Label text |
| `required` | `boolean` | `false` | Show required indicator |
| `error` | `string` | `undefined` | Error message |
| `hint` | `string` | `undefined` | Helper hint text |
| `class` | `string` | `''` | Additional Tailwind classes |

**Slots:**

- `default` - Form input element

**Example:**

```svelte
<Label label="Email Address" required={true} hint="We'll never share your email">
  <Input type="email" />
</Label>
```

---

### Divider

Visual separator component for horizontal or vertical divisions.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `'horizontal'\|'vertical'` | `'horizontal'` | Divider orientation |
| `color` | `string` | `'slate'` | Divider color (matches Tailwind) |
| `class` | `string` | `''` | Additional Tailwind classes |

**Example:**

```svelte
<div>Section 1</div>
<Divider variant="horizontal" color="slate" />
<div>Section 2</div>
```

---

## Molecules

### Breadcrumbs

Navigation breadcrumb component showing the current page hierarchy.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `BreadcrumbItem[]` | `[]` | Navigation path items |
| `current` | `string` | `undefined` | Current page name |
| `separator` | `string` | `'/'` | Separator between items |
| `class` | `string` | `''` | Additional Tailwind classes |

**Type:** `BreadcrumbItem`

```typescript
interface BreadcrumbItem {
  label: string;
  href?: string;
  value?: string;
}
```

**Example:**

```svelte
<Breadcrumbs
  items={[
    { label: 'Dashboard', href: '/' },
    { label: 'Settings', href: '/settings' }
  ]}
  current="Profile"
  separator=">"
/>
```

---

### Toast

Toast notification system with auto-dismiss and multiple positions.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `position` | Toast Position | `'bottom-right'` | Toast notification position |
| `toasts` | `Toast[]` | `[]` | Array of toast messages |

**Type:** Toast Position

```typescript
type ToastPosition =
  | "top-left"
  | "top-center"
  | "top-right"
  | "bottom-left"
  | "bottom-center"
  | "bottom-right";
```

**Type:** Toast Item

```typescript
interface Toast {
  id: string;
  message: string;
  type: "success" | "error" | "warning" | "info";
  duration?: number;
}
```

**Methods:**

- `show(message: string, type: ToastType, duration?: number)` - Show toast
- `hide(id: string)` - Hide specific toast

**Example:**

```svelte
<script>
  let toastComponent;
</script>

<Toast bind:this={toastComponent} position="top-right" />

<button on:click={() => toastComponent.show('Success!', 'success', 3000)}>
  Show Toast
</button>
```

---

### Filter

Multi-select filter button group component.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `FilterItem[]` | `[]` | Available filter options |
| `selected` | `string[]` | `[]` | Currently selected filters |
| `onChange` | `(items: string[]) => void` | `undefined` | Callback when selection changes |

**Type:** FilterItem

```typescript
interface FilterItem {
  id: string;
  label: string;
  icon?: string;
}
```

**Example:**

```svelte
<script>
  let selectedFilters = [];
</script>

<Filter
  items={[
    { id: 'active', label: 'Active' },
    { id: 'archived', label: 'Archived' }
  ]}
  selected={selectedFilters}
  onChange={(items) => selectedFilters = items}
/>
```

---

### Select

Dropdown select component with single selection.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `SelectItem[]` | `[]` | Available options |
| `selected` | `string` | `''` | Currently selected value |
| `placeholder` | `string` | `'Select...'` | Placeholder text |
| `disabled` | `boolean` | `false` | Disable select |
| `searchable` | `boolean` | `false` | Enable search/filter |

**Type:** SelectItem

```typescript
interface SelectItem {
  value: string;
  label: string;
  disabled?: boolean;
}
```

**Example:**

```svelte
<script>
  let selected = 'opt1';
</script>

<Select
  items={[
    { value: 'opt1', label: 'Option 1' },
    { value: 'opt2', label: 'Option 2' }
  ]}
  bind:selected
  placeholder="Choose..."
/>
```

---

### ContextMenu

Right-click context menu component for actions.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `visible` | `boolean` | `false` | Show/hide context menu |
| `x` | `number` | `0` | Mouse X position |
| `y` | `number` | `0` | Mouse Y position |
| `items` | `ContextMenuItem[]` | `[]` | Menu items |

**Type:** ContextMenuItem

```typescript
interface ContextMenuItem {
  id: string;
  label: string;
  icon?: string;
  divider?: boolean;
  dangerous?: boolean;
  onClick?: () => void;
}
```

**Events:**

- `select` - Fired when menu item selected (detail: item id)
- `close` - Fired when menu closes

**Example:**

```svelte
<script>
  let menuVisible = false;
  let x = 0, y = 0;

  function handleRightClick(event) {
    event.preventDefault();
    x = event.clientX;
    y = event.clientY;
    menuVisible = true;
  }
</script>

<div on:contextmenu={handleRightClick}>
  Right-click me
</div>

<ContextMenu
  visible={menuVisible}
  {x}
  {y}
  items={[
    { id: 'edit', label: 'Edit', icon: 'bi-pencil' },
    { id: 'delete', label: 'Delete', icon: 'bi-trash', dangerous: true }
  ]}
  on:select={(e) => console.log(e.detail)}
  on:close={() => menuVisible = false}
/>
```

---

## Organisms

### Modal

Full-featured modal dialog component with customizable sizes.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `boolean` | `false` | Show/hide modal |
| `title` | `string` | `''` | Modal title |
| `size` | `'sm'\|'md'\|'lg'\|'xl'` | `'md'` | Modal size |
| `closeButton` | `boolean` | `true` | Show close button |
| `class` | `string` | `''` | Additional Tailwind classes |

**Events:**

- `close` - Fired when modal closes

**Slots:**

- `default` - Modal content
- `footer` - Footer actions

**Size Reference:**

- `sm`: 28rem (448px)
- `md`: 32rem (512px)
- `lg`: 36rem (576px)
- `xl`: 42rem (672px)

**Example:**

```svelte
<script>
  let isOpen = false;
</script>

<Modal
  open={isOpen}
  title="Confirm Action"
  size="md"
  on:close={() => isOpen = false}
>
  <p>Are you sure you want to proceed?</p>

  <div slot="footer">
    <Button on:click={() => isOpen = false}>Cancel</Button>
    <Button variant="primary" on:click={handleConfirm}>
      Confirm
    </Button>
  </div>
</Modal>

<Button on:click={() => isOpen = true}>Open Modal</Button>
```

---

### FileViewer

Multi-format file preview component supporting 8+ file types.

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `file` | `FileObject` | `undefined` | File to display |

**Type:** FileObject

```typescript
interface FileObject {
  type: "text" | "code" | "image" | "pdf" | "video" | "audio" | "file";
  name: string;
  size: string;
  content?: string; // For text/code files
  url?: string; // For images/video/audio/pdf
  language?: string; // For syntax highlighting
  mimeType?: string; // MIME type
}
```

**Supported Formats:**

- **Text:** `.txt`, `.md`, `.json`, `.csv`
- **Code:** `.js`, `.ts`, `.jsx`, `.tsx`, `.py`, `.html`, `.css`, `.xml`
- **Images:** `.jpg`, `.png`, `.gif`, `.webp`, `.svg`, `.bmp`
- **Video:** `.mp4`, `.webm`, `.ogg`
- **Audio:** `.mp3`, `.wav`, `.ogg`
- **PDF:** `.pdf` (iframe preview)
- **Generic:** Any other file type

**Example:**

```svelte
<FileViewer
  file={{
    type: 'code',
    name: 'app.js',
    size: '2.4 KB',
    content: 'function main() { ... }',
    language: 'javascript'
  }}
/>
```

---

## Shared Utilities

Located in `shared/index.ts` - provides centralized design tokens and TypeScript types.

**Available Exports:**

```typescript
// Type definitions
export type ButtonVariant =
  | "primary"
  | "secondary"
  | "danger"
  | "success"
  | "warning"
  | "ghost"
  | "outline";
export type ButtonSize = "xs" | "sm" | "md" | "lg" | "xl";
export type BadgeVariant =
  | "primary"
  | "secondary"
  | "danger"
  | "success"
  | "warning"
  | "info";
export type ToastType = "success" | "error" | "warning" | "info";
export type ToastPosition =
  | "top-left"
  | "top-center"
  | "top-right"
  | "bottom-left"
  | "bottom-center"
  | "bottom-right";

// Color maps
export const colorMap: Record<ButtonVariant, string>;
export const colorBorder: Record<ButtonVariant, string>;
export const colorHover: Record<ButtonVariant, string>;
export const colorText: Record<ButtonVariant, string>;

// Size configurations
export const sizeMap: Record<
  ButtonSize,
  { px: string; py: string; textSize: string }
>;

// Animations
export const transitions = {
  fast: "150ms",
  normal: "300ms",
  slow: "500ms",
};
```

---

## Accessibility Notes

All components include:

- ✅ Semantic HTML elements
- ✅ ARIA attributes where appropriate
- ✅ Keyboard navigation support
- ✅ Focus management
- ✅ Color contrast compliance
- ✅ Screen reader friendly

---

## Performance Considerations

- Components use Svelte 5 runes for optimal reactivity
- Minimal re-renders through fine-grained reactivity
- CSS classes compiled at build time with Tailwind
- No unnecessary DOM mutations
- Event delegation where applicable

---

**Last Updated:** 2024  
**Version:** 1.0.0
