# 🚀 Week 1 Quick Start Checklist

**Goal**: Foundation Setup - DaisyUI Integration & Component Migration

**Time Budget**: 20-25 hours  
**Status**: 🔴 Ready to Start

---

## Day 1: Setup & Configuration (4 hours)

### Morning (2h): DaisyUI Installation

- [ ] **Install Dependencies**

  ```powershell
  cd frontend
  npm install -D daisyui@latest
  npm install -D @tailwindcss/typography
  npm install -D postcss autoprefixer
  ```

- [ ] **Create Tailwind Config**

  - File: `tailwind.config.js`

  ```javascript
  export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
      extend: {
        fontFamily: {
          sans: ["Inter", "system-ui", "sans-serif"],
        },
      },
    },
    plugins: [require("daisyui"), require("@tailwindcss/typography")],
    daisyui: {
      themes: [
        {
          syncspace: {
            primary: "#667eea",
            secondary: "#764ba2",
            accent: "#f59e0b",
            neutral: "#1f2937",
            "base-100": "#ffffff",
            info: "#3b82f6",
            success: "#10b981",
            warning: "#f59e0b",
            error: "#ef4444",
          },
        },
        "dark",
        "light",
      ],
    },
  };
  ```

- [ ] **Create PostCSS Config**

  - File: `postcss.config.js`

  ```javascript
  export default {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  };
  ```

- [ ] **Update Main CSS**
  - File: `src/app.css` (add at top)
  ```css
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
  ```

### Afternoon (2h): Design Tokens

- [ ] **Create Design Tokens File**

  - File: `src/styles/design-tokens.css`
  - Copy current CSS variables
  - Add semantic tokens
  - Document usage

- [ ] **Test Theme Switching**
  - Add `data-theme="syncspace"` to `<html>` tag
  - Toggle to "dark" and verify
  - Test all color variables

---

## Day 2: Button & Input Components (5 hours)

### Morning (2.5h): Button Component

- [ ] **Create Button V2**

  - File: `src/components/ui/ButtonV2.svelte`
  - Base on DaisyUI `btn` classes
  - Props: variant, size, loading, disabled, icon
  - Variants: primary, secondary, accent, ghost, outline, error, success
  - Sizes: xs, sm, md, lg
  - Test all combinations

- [ ] **Migrate One View**
  - Pick SettingsView (smallest)
  - Replace `<Button>` with `<ButtonV2>`
  - Verify functionality
  - Compare before/after

### Afternoon (2.5h): Input Component

- [ ] **Create Input V2**

  - File: `src/components/ui/InputV2.svelte`
  - Base on DaisyUI `input` classes
  - Props: type, placeholder, value, error, disabled, icon
  - States: default, error, success, disabled
  - Sizes: sm, md, lg
  - Add validation states

- [ ] **Test Input in Form**
  - Create test form in SettingsView
  - Test validation
  - Test error states

---

## Day 3: Layout Components (5 hours)

### Morning (2.5h): Card Component

- [ ] **Create Card V2**

  - File: `src/components/ui/CardV2.svelte`
  - Base on DaisyUI `card` classes
  - Keep glassmorphism option
  - Props: glass, bordered, compact, hoverable
  - Slots: header, body, actions
  - Add hover lift effect

- [ ] **Migrate StatCard**
  - Combine with Card V2
  - Use in Dashboard/Storage views
  - Test stats display

### Afternoon (2.5h): Modal Component

- [ ] **Enhance Modal/Dialog**

  - Update existing Dialog.svelte
  - Add DaisyUI `modal` classes
  - Improve animations (scale + fade)
  - Add backdrop blur
  - Test keyboard (Escape to close)

- [ ] **Test in FilesView**
  - Use for file properties
  - Test ConfirmDialog still works

---

## Day 4: Navigation Redesign (6 hours)

### Morning (3h): App Header

- [ ] **Create AppHeader Component**

  - File: `src/components/layout/AppHeader.svelte`
  - DaisyUI `navbar` as base
  - Logo on left
  - Global search in center
  - User menu + theme toggle on right
  - Make sticky on scroll

- [ ] **Add to App.svelte**
  - Replace current header
  - Test on all pages
  - Verify responsive behavior

### Afternoon (3h): Sidebar Navigation

- [ ] **Create Sidebar Component**

  - File: `src/components/layout/Sidebar.svelte`
  - DaisyUI `menu` component
  - Icon + label for each route
  - Active state highlighting
  - Collapsible on mobile (drawer)
  - Storage usage at bottom

- [ ] **Integrate with App**
  - Add to App.svelte layout
  - Test navigation
  - Test mobile drawer

---

## Day 5: Polish & Testing (5 hours)

### Morning (2.5h): Component Gallery

- [ ] **Create Component Showcase**

  - File: `src/pages/ComponentGallery.svelte`
  - Show all new components
  - All variants + states
  - Copy code snippets
  - Test theme switching

- [ ] **Document Components**
  - Update COMPONENTS_README.md
  - Add migration guide
  - Screenshot examples

### Afternoon (2.5h): Migration Planning

- [ ] **Create Migration Matrix**

  - Spreadsheet of all components
  - Old vs new comparison
  - Migration priority
  - Breaking changes list

- [ ] **Test Build**

  ```powershell
  npm run build
  ```

  - Fix any errors
  - Check bundle size
  - Test production build

- [ ] **Create PR/Commit**
  - Commit: "feat: DaisyUI integration + new components"
  - Document changes
  - Tag as v2.0.0-alpha.1

---

## Success Criteria ✅

At end of Week 1, you should have:

- [x] DaisyUI fully configured and working
- [x] 5 new components (Button, Input, Card, Modal, Header)
- [x] New navigation (AppHeader + Sidebar)
- [x] At least 1 view fully migrated
- [x] Theme switching working
- [x] Component gallery page
- [x] Documentation updated
- [x] Clean build with no errors

---

## Quick Commands Reference

```powershell
# Development
npm run dev

# Build
npm run build

# Preview build
npm run preview

# Check bundle size
npm run build -- --mode production
```

---

## Troubleshooting

### Issue: Tailwind classes not working

**Solution**:

1. Check `tailwind.config.js` content paths
2. Restart dev server
3. Clear `.svelte-kit` cache

### Issue: DaisyUI theme not applying

**Solution**:

1. Add `data-theme="syncspace"` to html tag
2. Check theme name in config
3. Verify `@tailwind` directives in CSS

### Issue: Existing CSS conflicts

**Solution**:

1. Increase specificity with `@layer components`
2. Use `!important` sparingly
3. Gradually remove old CSS

---

## Notes for Next Week

After Week 1, focus on:

1. Migrate remaining 20 components
2. Rebuild FilesView with new components
3. Add file preview modal
4. Implement animations
5. Start on batch operations

---

**Created**: 24. Oktober 2025  
**Status**: 🔴 Ready to Start  
**Estimated Time**: 25 hours  
**Actual Time**: \_\_\_ hours (fill in as you go)
