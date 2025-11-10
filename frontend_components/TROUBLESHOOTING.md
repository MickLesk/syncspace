# Troubleshooting Guide - SyncSpace Component Library

## Problem: Cannot apply unknown utility class `bg-slate-900`

### Root Causes
1. **node_modules cache** - Old Tailwind CSS v3 files still in cache
2. **PostCSS not reloading** - CSS pipeline not properly reset
3. **Vite cache** - Dev server cache not cleared

### Solution

#### Option 1: Full Reset (Recommended)
```bash
cd /home/mick/Dokumente/GitHub/syncspace/frontend_components

# Run the reset script
bash RESET.sh

# Or manually:
rm -rf node_modules package-lock.json dist .vite
npm install --legacy-peer-deps
npm run dev
```

#### Option 2: Quick Clean
```bash
npm run dev -- --force
```

#### Option 3: Manual Steps
1. Stop dev server: `Ctrl+C`
2. Clean cache: `rm -rf dist .vite`
3. Reinstall: `npm install --legacy-peer-deps`
4. Start: `npm run dev`

---

## Configuration Files (Verified Correct)

### 1. postcss.config.js ✅
```javascript
export default {
  plugins: {
    '@tailwindcss/postcss': {},
  },
};
```

### 2. tailwind.config.js ✅
```javascript
export default {
  content: [
    './index.html',
    './App.svelte',
    './src/**/*.{svelte,js,ts}',
    './pages/**/*.{svelte,js,ts}',
    './atoms/**/*.{svelte,js,ts}',
    './molecules/**/*.{svelte,js,ts}',
    './organisms/**/*.{svelte,js,ts}',
  ],
  theme: { extend: {} },
  plugins: [],
};
```

### 3. src/app.css ✅
```css
@import "tailwindcss";
```

### 4. App.svelte ✅
```svelte
<script>
  import './src/app.css';
  // ... rest of script
</script>
```

---

## Common Issues & Fixes

### Issue: "Unknown utility class" errors
**Fix**: Run RESET.sh - clears old cache and reinstalls Tailwind v4

### Issue: Styles not loading
**Fix**: Make sure `src/app.css` is imported in `App.svelte`

### Issue: PostCSS errors
**Fix**: Verify `@tailwindcss/postcss` is in `node_modules`

### Issue: Vite not detecting changes
**Fix**: `npm run dev -- --force` or restart dev server

---

## Verification Steps

After running RESET.sh, verify:

1. **Check installation**:
   ```bash
   npm list @tailwindcss/postcss
   ```
   Should show: `@tailwindcss/postcss@^4.0.0`

2. **Start dev server**:
   ```bash
   npm run dev
   ```
   Should open on http://localhost:5174 without errors

3. **Check browser**:
   - Page should have dark theme (bg-slate-900)
   - All components should render
   - No CSS errors in console

---

## Environment Info

- Node.js: v25.1.0 (check with `node --version`)
- npm: 10.x+ (check with `npm --version`)
- Tailwind CSS: ^4.0.0
- PostCSS: ^8.4.0
- Svelte: ^5.0.0
- Vite: ^5.0.0

---

## Still Having Issues?

1. **Check package.json** - Ensure `@tailwindcss/postcss` is listed
2. **Check PostCSS config** - Must use `@tailwindcss/postcss` plugin
3. **Check Tailwind config** - Content paths must include `.svelte` files
4. **Clear browser cache** - F5 or Ctrl+Shift+R
5. **Check file permissions** - All files readable/writable

---

## Quick Reference Commands

```bash
# Full reset
bash RESET.sh

# Start dev (after reset)
npm run dev

# Build for production
npm run build

# Preview build
npm run preview

# Type check
npm run type-check

# Lint
npm run lint
```

---

**Last Updated**: 10. November 2025
**Status**: ✅ Troubleshooting guide complete
