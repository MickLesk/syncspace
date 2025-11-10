# Getting Started with SyncSpace Component Library

Welcome to the SyncSpace Component Library! This guide will help you get started quickly.

## 📦 Quick Setup

### 1. Install Dependencies

The component library requires Svelte 5 and Tailwind CSS v4:

```bash
# In your main SyncSpace frontend directory
npm install svelte@^5.0.0 tailwindcss@^4.0.0 typescript@^5.0.0
```

### 2. Import Components

The most direct way to use components:

```svelte
<script>
  // Import individual components
  import Button from '../frontend_components/atoms/Button.svelte';
  import Badge from '../frontend_components/atoms/Badge.svelte';

  // Or use convenient index re-exports
  import { Button, Badge } from '../frontend_components/atoms/index.ts';
  import { Toast, Filter } from '../frontend_components/molecules/index.ts';
  import { Modal, FileViewer } from '../frontend_components/organisms/index.ts';
</script>

<!-- Use components -->
<Button variant="primary">Click me</Button>
<Badge variant="success">Active</Badge>
```

## 🎯 Component Categories

### Atoms (Basic Building Blocks)

Use for fundamental UI elements:

```svelte
<script>
  import { Button, Badge, Avatar, Input, Checkbox, Toggle } from '../frontend_components/atoms/index.ts';
</script>

<!-- Simple button -->
<Button>Basic Button</Button>

<!-- Input field -->
<Input placeholder="Enter text..." />

<!-- Checkbox selection -->
<Checkbox label="Accept terms" bind:checked={accepted} />

<!-- Status badge -->
<Badge variant="success">Approved</Badge>

<!-- User avatar -->
<Avatar name="John Doe" />
```

### Molecules (Complex Combinations)

Use for interactive UI patterns:

```svelte
<script>
  import { Breadcrumbs, Toast, Filter, Select, ContextMenu } from '../frontend_components/molecules/index.ts';
  import Button from '../frontend_components/atoms/Button.svelte';

  let toastComponent;
  let selectedFilters = [];
</script>

<!-- Navigation breadcrumbs -->
<Breadcrumbs
  items={[
    { label: 'Home', href: '/' },
    { label: 'Products', href: '/products' }
  ]}
  current="Details"
/>

<!-- Toast notifications -->
<Toast bind:this={toastComponent} position="bottom-right" />
<Button on:click={() => toastComponent.show('Success!', 'success', 3000)}>
  Show Toast
</Button>

<!-- Multi-select filters -->
<Filter
  items={[
    { id: 'active', label: 'Active' },
    { id: 'archived', label: 'Archived' }
  ]}
  selected={selectedFilters}
  onChange={(items) => selectedFilters = items}
/>

<!-- Dropdown select -->
<Select
  items={[
    { value: '1', label: 'Option 1' },
    { value: '2', label: 'Option 2' }
  ]}
  placeholder="Choose..."
/>
```

### Organisms (Feature-Complete Components)

Use for complex, full-featured UI:

```svelte
<script>
  import { Modal, FileViewer } from '../frontend_components/organisms/index.ts';
  import Button from '../frontend_components/atoms/Button.svelte';

  let isModalOpen = false;
</script>

<!-- Modal dialog -->
<Modal
  open={isModalOpen}
  title="Confirm Action"
  size="lg"
  on:close={() => isModalOpen = false}
>
  <p>Are you sure?</p>

  <div slot="footer">
    <Button variant="secondary" on:click={() => isModalOpen = false}>
      Cancel
    </Button>
    <Button variant="primary" on:click={handleConfirm}>
      Confirm
    </Button>
  </div>
</Modal>

<!-- File preview -->
<FileViewer
  file={{
    type: 'code',
    name: 'app.js',
    size: '2.4 KB',
    content: 'console.log("Hello");',
    language: 'javascript'
  }}
/>
```

## 🎨 Customization

### Using Props for Variants

Most components support multiple visual variants:

```svelte
<script>
  import Button from '../frontend_components/atoms/Button.svelte';
  import Badge from '../frontend_components/atoms/Badge.svelte';
</script>

<!-- Button variants -->
<Button variant="primary">Primary</Button>
<Button variant="secondary">Secondary</Button>
<Button variant="danger">Danger</Button>
<Button variant="success">Success</Button>
<Button variant="warning">Warning</Button>
<Button variant="ghost">Ghost</Button>
<Button variant="outline">Outline</Button>

<!-- Button sizes -->
<Button size="xs">Extra Small</Button>
<Button size="sm">Small</Button>
<Button size="md">Medium</Button>
<Button size="lg">Large</Button>
<Button size="xl">Extra Large</Button>

<!-- Badge variants -->
<Badge variant="primary">Primary</Badge>
<Badge variant="secondary">Secondary</Badge>
<Badge variant="danger">Danger</Badge>
<Badge variant="success">Success</Badge>
<Badge variant="warning">Warning</Badge>
<Badge variant="info">Info</Badge>

<!-- Outline badges -->
<Badge variant="primary" outline>Outline</Badge>
```

### Custom CSS Classes

Add Tailwind classes for additional customization:

```svelte
<Button class="shadow-lg uppercase tracking-wide">
  Custom Styled Button
</Button>

<Badge class="ring-2 ring-blue-300">
  Custom Badge
</Badge>
```

### Svelte 5 Runes for State

Use Svelte 5 runes for reactive state:

```svelte
<script>
  import Button from '../frontend_components/atoms/Button.svelte';

  let count = $state(0);
  let doubled = $derived(count * 2);

  function increment() {
    count++;
  }
</script>

<div>
  <p>Count: {count}</p>
  <p>Doubled: {doubled}</p>
  <Button on:click={increment}>Increment</Button>
</div>
```

## 📱 Responsive Design

All components are mobile-first and responsive:

```svelte
<script>
  import { Button } from '../frontend_components/atoms/index.ts';
</script>

<!-- Components adapt to screen size automatically -->
<div class="grid md:grid-cols-2 gap-4">
  <Button>Button 1</Button>
  <Button>Button 2</Button>
</div>
```

## 🌓 Dark Mode

Components work with Tailwind's dark mode. Set up in your app:

```html
<!-- In your main HTML file -->
<html data-theme="dark">
  <!-- Or toggle dynamically -->
</html>
```

```svelte
<script>
  function toggleDarkMode() {
    document.documentElement.setAttribute('data-theme',
      document.documentElement.getAttribute('data-theme') === 'dark' ? 'light' : 'dark'
    );
  }
</script>
```

## 🚀 Common Patterns

### Form with Validation

```svelte
<script>
  import { Input, Button, Label } from '../frontend_components/atoms/index.ts';
  import { Toast } from '../frontend_components/molecules/index.ts';

  let email = '';
  let toastComponent;

  function validateEmail(email) {
    return email.includes('@') && email.includes('.');
  }

  function handleSubmit() {
    if (!validateEmail(email)) {
      toastComponent.show('Invalid email', 'error', 3000);
      return;
    }

    toastComponent.show('Email valid!', 'success', 3000);
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <Label label="Email" required={true} />
  <Input
    type="email"
    bind:value={email}
    error={!validateEmail(email) && email ? 'Invalid email' : undefined}
    placeholder="Enter email..."
  />
  <Button>Submit</Button>
</form>

<Toast bind:this={toastComponent} position="bottom-right" />
```

### Settings with Toggles

```svelte
<script>
  import { Toggle, Card } from '../frontend_components/atoms/index.ts';
  import { Toast } from '../frontend_components/molecules/index.ts';

  let notifications = $state(true);
  let darkMode = $state(false);
  let newsletter = $state(false);
  let toastComponent;

  function handleSettingChange(setting, value) {
    toastComponent.show(`${setting} ${value ? 'enabled' : 'disabled'}`, 'info', 2000);
  }
</script>

<Card title="Settings" description="Manage your preferences">
  <div class="space-y-4">
    <Toggle
      label="Enable notifications"
      bind:checked={notifications}
      on:change={() => handleSettingChange('Notifications', notifications)}
    />
    <Toggle
      label="Dark mode"
      bind:checked={darkMode}
      on:change={() => handleSettingChange('Dark mode', darkMode)}
    />
    <Toggle
      label="Subscribe to newsletter"
      bind:checked={newsletter}
      on:change={() => handleSettingChange('Newsletter', newsletter)}
    />
  </div>
</Card>

<Toast bind:this={toastComponent} position="bottom-right" />
```

### Confirmation Dialog

```svelte
<script>
  import { Button } from '../frontend_components/atoms/index.ts';
  import { Modal } from '../frontend_components/organisms/index.ts';
  import { Toast } from '../frontend_components/molecules/index.ts';

  let showConfirm = false;
  let toastComponent;

  function handleDelete() {
    toastComponent.show('Item deleted successfully', 'success', 3000);
    showConfirm = false;
  }
</script>

<Button variant="danger" on:click={() => showConfirm = true}>
  Delete Item
</Button>

<Modal
  open={showConfirm}
  title="Delete Item"
  size="sm"
  on:close={() => showConfirm = false}
>
  <p class="text-slate-300">
    Are you sure you want to delete this item? This action cannot be undone.
  </p>

  <div slot="footer" class="flex gap-3 justify-end">
    <Button variant="secondary" size="sm" on:click={() => showConfirm = false}>
      Cancel
    </Button>
    <Button variant="danger" size="sm" on:click={handleDelete}>
      Delete
    </Button>
  </div>
</Modal>

<Toast bind:this={toastComponent} position="bottom-right" />
```

## 🔍 Demo Pages

The component library includes interactive demo pages:

```svelte
<script>
  import { DemoHome, AtomsDemo, MoleculesDemo, OrganismsDemo }
    from '../frontend_components/pages/index.ts';

  let currentPage = 'home';
</script>

{#if currentPage === 'home'}
  <DemoHome />
{:else if currentPage === 'atoms'}
  <AtomsDemo />
{:else if currentPage === 'molecules'}
  <MoleculesDemo />
{:else if currentPage === 'organisms'}
  <OrganismsDemo />
{/if}
```

## 📖 Full Documentation

For complete API documentation, see:

- `README.md` - Overview and features
- `COMPONENTS.md` - Detailed API reference for each component

## 💡 Best Practices

1. **Use Atoms for Basic UI** - Build complex components from simple atoms
2. **Leverage TypeScript** - Use the full type safety
3. **Centralize State** - Use Svelte 5 runes and stores for state management
4. **Follow Accessibility** - Components include a11y, use them properly
5. **Mobile-First** - Think mobile first when building layouts
6. **Consistent Theming** - Use design tokens from `shared/index.ts`

## 🐛 Troubleshooting

### Components Not Importing?

Ensure Svelte 5 is installed and configured:

```bash
npm install svelte@^5.0.0
```

### Styles Not Applying?

Make sure Tailwind CSS is configured in your project:

```javascript
// tailwind.config.js
module.exports = {
  content: ["./frontend_components/**/*.svelte"],
};
```

### Type Errors?

Ensure TypeScript is configured:

```bash
npm install typescript@^5.0.0
```

## 📞 Support

For issues or questions:

1. Check `COMPONENTS.md` for detailed API docs
2. Review demo pages for usage examples
3. Examine component source files in respective directories

## 🎉 Next Steps

1. **Review Components** - Check out demo pages to see all components
2. **Copy Patterns** - Use provided examples as templates
3. **Customize** - Modify colors and styles in `shared/index.ts`
4. **Build** - Create your own components following atomic design principles

---

Happy building! 🚀
