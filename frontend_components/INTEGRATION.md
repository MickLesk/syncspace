# Component Library Deployment & Integration Guide

Complete guide for integrating the SyncSpace Component Library into your main application.

## 📦 Integration Steps

### Step 1: Verify Installation

Ensure your main SyncSpace frontend has required dependencies:

```bash
cd frontend
npm install svelte@^5.0.0 tailwindcss@^4.0.0 typescript@^5.0.0
npm list svelte tailwindcss
```

### Step 2: Update Tailwind Configuration

Ensure `frontend/tailwind.config.js` includes the component library:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{svelte,js,ts,jsx,tsx}",
    "../frontend_components/**/*.svelte", // Add this line
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

### Step 3: Update Vite Configuration

Ensure `frontend/vite.config.js` includes Svelte plugin:

```javascript
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      // Optional: Create alias for easier imports
      "@components": "/frontend_components",
    },
  },
});
```

### Step 4: Bootstrap Icons Setup

Add Bootstrap Icons to `frontend/index.html`:

```html
<head>
  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.0/font/bootstrap-icons.css"
  />
</head>
```

Or install locally:

```bash
npm install bootstrap-icons
```

Then import in your main CSS file:

```css
@import "bootstrap-icons/font/bootstrap-icons.css";
```

---

## 🎯 Using Components in Your App

### Basic Usage

```svelte
<!-- frontend/src/components/MyComponent.svelte -->
<script>
  import { Button, Badge } from '../../frontend_components/atoms/index.ts';
  import { Toast } from '../../frontend_components/molecules/index.ts';
  import { Modal } from '../../frontend_components/organisms/index.ts';

  let isModalOpen = false;
  let toastComponent;
</script>

<div class="space-y-4">
  <Badge variant="success">Active</Badge>
  <Button on:click={() => isModalOpen = true}>Open Modal</Button>
</div>

<Modal
  open={isModalOpen}
  title="Hello"
  on:close={() => isModalOpen = false}
>
  Content here
  <div slot="footer">
    <Button on:click={() => isModalOpen = false}>Close</Button>
  </div>
</Modal>

<Toast bind:this={toastComponent} position="bottom-right" />
```

### Alias Import (Optional)

If you set up the Vite alias, you can simplify imports:

```svelte
<script>
  import { Button, Badge } from '@components/atoms/index.ts';
  import { Toast } from '@components/molecules/index.ts';
  import { Modal } from '@components/organisms/index.ts';
</script>
```

---

## 🔌 Integration with Backend API

### Typical Pattern

```svelte
<script>
  import { Button } from '@components/atoms/index.ts';
  import { Toast } from '@components/molecules/index.ts';
  import * as api from '../lib/api.ts';

  let toastComponent;
  let isLoading = false;

  async function handleAction() {
    try {
      isLoading = true;
      const result = await api.files.upload(file);
      toastComponent.show('Success!', 'success', 3000);
    } catch (error) {
      toastComponent.show(error.message, 'error', 3000);
    } finally {
      isLoading = false;
    }
  }
</script>

<Button loading={isLoading} on:click={handleAction}>
  Save Changes
</Button>
<Toast bind:this={toastComponent} />
```

---

## 🎨 Theming & Customization

### Global Style Setup

Create `frontend/src/styles/theme.css`:

```css
/* Override component colors */
:root {
  --color-primary: #3b82f6;
  --color-secondary: #64748b;
  --color-danger: #ef4444;
  --color-success: #10b981;
  --color-warning: #f59e0b;
  --color-info: #06b6d4;
}

/* Dark mode support */
[data-theme="dark"] {
  --color-primary: #2563eb;
  --color-secondary: #475569;
  /* ... override for dark mode */
}
```

### Component Customization

Modify `frontend_components/shared/index.ts`:

```typescript
export const colorMap = {
  primary: "from-blue-500 to-blue-600",
  secondary: "from-slate-500 to-slate-600",
  // Customize colors here
};
```

---

## 📁 Recommended File Structure

```
frontend/
├── src/
│   ├── App.svelte                 # Main app
│   ├── lib/
│   │   ├── api.ts                 # Backend API client
│   │   ├── stores.ts              # Global stores
│   │   └── utils.ts               # Utilities
│   ├── pages/
│   │   ├── Dashboard.svelte
│   │   ├── Settings.svelte
│   │   ├── Files.svelte
│   │   └── ...
│   ├── components/
│   │   ├── FileViewer.svelte      # Your components using lib
│   │   ├── UserCard.svelte
│   │   └── ...
│   └── styles/
│       ├── app.css
│       └── theme.css
├── index.html
├── vite.config.js
├── tailwind.config.js
└── ...

frontend_components/              # Component library (at root)
├── atoms/
├── molecules/
├── organisms/
├── shared/
└── ...
```

---

## 🚀 Deployment Considerations

### Production Build

```bash
# Build frontend with components included
cd frontend
npm run build

# Or with specific commands:
vite build
```

### Bundle Size Optimization

Components are tree-shakeable. Only imported components are included:

```svelte
<!-- Only these are bundled -->
import { Button } from '@components/atoms/index.ts';
import { Toast } from '@components/molecules/index.ts';
```

### Performance Tips

1. **Lazy Load Pages**

   ```svelte
   <script>
     import { lazy } from 'svelte/transitions';
     const Modal = lazy(() => import('@components/organisms/Modal.svelte'));
   </script>
   ```

2. **Virtual Scrolling** for large lists

   ```svelte
   <script>
     import { virtualize } from 'svelte-virtual-scroll';
   </script>
   ```

3. **Code Splitting** - Vite handles automatically

---

## 🧪 Testing Components

### Unit Testing Example

```javascript
// frontend/tests/Button.test.js
import { render, screen } from "@testing-library/svelte";
import Button from "@components/atoms/Button.svelte";

describe("Button", () => {
  test("renders with primary variant", () => {
    render(Button, { props: { variant: "primary" } });
    const button = screen.getByRole("button");
    expect(button).toHaveClass("from-blue-500");
  });

  test("handles click event", async () => {
    const { component } = render(Button);
    const button = screen.getByRole("button");
    await button.click();
    expect(component.$on).toHaveBeenCalledWith("click");
  });
});
```

### Component Snapshot Testing

```javascript
// frontend/tests/Avatar.test.js
import Avatar from "@components/atoms/Avatar.svelte";

test("Avatar snapshot", () => {
  const { container } = render(Avatar, {
    props: { name: "John Doe" },
  });
  expect(container).toMatchSnapshot();
});
```

---

## 🔐 Type Safety

### TypeScript Configuration

Update `frontend/tsconfig.json`:

```json
{
  "compilerOptions": {
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "target": "ES2020",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "allowJs": true,
    "checkJs": false,
    "strict": true,
    "paths": {
      "@components/*": ["../frontend_components/*"]
    }
  }
}
```

### Component Type Checking

```svelte
<script lang="ts">
  import type { ButtonVariant, ButtonSize } from '@components/shared/index.ts';

  let variant: ButtonVariant = 'primary';
  let size: ButtonSize = 'md';
</script>
```

---

## 📝 Common Integration Patterns

### Form with Validation

```svelte
<script>
  import { Input, Checkbox, Button } from '@components/atoms/index.ts';
  import { Toast } from '@components/molecules/index.ts';
  import * as api from '../lib/api.ts';

  let form = {
    email: '',
    name: '',
    subscribe: false
  };
  let errors = {};
  let toastComponent;

  async function handleSubmit() {
    errors = validateForm(form);

    if (Object.keys(errors).length === 0) {
      try {
        await api.users.create(form);
        toastComponent.show('Account created!', 'success', 3000);
      } catch (error) {
        toastComponent.show(error.message, 'error', 3000);
      }
    }
  }

  function validateForm(data) {
    const errors = {};
    if (!data.email) errors.email = 'Email required';
    if (!data.name) errors.name = 'Name required';
    return errors;
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <Input
    label="Email"
    bind:value={form.email}
    error={errors.email}
  />
  <Input
    label="Name"
    bind:value={form.name}
    error={errors.name}
  />
  <Checkbox
    label="Subscribe to updates"
    bind:checked={form.subscribe}
  />
  <Button variant="primary">Create Account</Button>
</form>

<Toast bind:this={toastComponent} />
```

### File Upload with Progress

```svelte
<script>
  import { Button } from '@components/atoms/index.ts';
  import { Toast } from '@components/molecules/index.ts';
  import * as api from '../lib/api.ts';

  let uploadProgress = 0;
  let isUploading = false;
  let toastComponent;

  async function handleFileUpload(event) {
    const files = event.target.files;
    if (!files) return;

    for (const file of files) {
      try {
        isUploading = true;
        await api.files.upload(file, (progress) => {
          uploadProgress = progress;
        });
        toastComponent.show(`${file.name} uploaded!`, 'success', 2000);
      } catch (error) {
        toastComponent.show(`Failed to upload ${file.name}`, 'error', 3000);
      } finally {
        isUploading = false;
        uploadProgress = 0;
      }
    }
  }
</script>

<input
  type="file"
  multiple
  on:change={handleFileUpload}
  disabled={isUploading}
/>

{#if isUploading}
  <div class="w-full bg-slate-200 rounded">
    <div
      class="bg-blue-500 h-2 rounded"
      style="width: {uploadProgress}%"
    ></div>
  </div>
  <p>{uploadProgress}%</p>
{/if}

<Toast bind:this={toastComponent} />
```

---

## 🐛 Troubleshooting Integration

### Issue: Components Not Found

**Solution:** Check path resolution:

```javascript
// vite.config.js
export default {
  resolve: {
    alias: {
      "@components": new URL("../frontend_components", import.meta.url)
        .pathname,
    },
  },
};
```

### Issue: Styles Not Applying

**Solution:** Update Tailwind content paths:

```javascript
// tailwind.config.js
content: [
  './src/**/*.{svelte,js,ts}',
  '../frontend_components/**/*.svelte',  // Ensure this path
],
```

### Issue: Bootstrap Icons Not Showing

**Solution:** Verify CDN or local installation:

```html
<!-- Check index.html for CDN link -->
<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.0/font/bootstrap-icons.css"
/>
```

### Issue: TypeScript Errors

**Solution:** Update tsconfig.json paths:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@components/*": ["../frontend_components/*"]
    }
  }
}
```

---

## 📊 Verifying Integration

### Quick Verification Script

Create `frontend/verify-components.js`:

```javascript
import fs from "fs";
import path from "path";

const componentsPath = "../frontend_components";
const required = [
  "atoms/Button.svelte",
  "atoms/Badge.svelte",
  "molecules/Toast.svelte",
  "organisms/Modal.svelte",
  "shared/index.ts",
];

console.log("🔍 Verifying component library...\n");

let allFound = true;
for (const component of required) {
  const fullPath = path.join(componentsPath, component);
  const exists = fs.existsSync(fullPath);
  console.log(`${exists ? "✅" : "❌"} ${component}`);
  if (!exists) allFound = false;
}

console.log(
  `\n${allFound ? "✨ All components found!" : "❌ Some components missing"}`
);
```

Run:

```bash
node verify-components.js
```

---

## 🎯 Next Steps

1. **Review Components** - Check COMPONENTS.md for full API
2. **View Demos** - Import and display demo pages
3. **Create Custom** - Build application-specific components
4. **Test Integration** - Run your app with components
5. **Deploy** - Build and deploy to production

---

## 📞 Support Resources

- `README.md` - Component library overview
- `COMPONENTS.md` - Complete API reference
- `GETTING_STARTED.md` - Quick start guide
- `SUMMARY.md` - Library statistics
- `pages/DemoHome.svelte` - Interactive demos

---

**Integration Guide Version:** 1.0.0  
**Last Updated:** 2024
