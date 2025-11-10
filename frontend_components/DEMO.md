# 🚀 SyncSpace Component Library Demo

Standalone demo application für die SyncSpace Component Library.

## Quick Start (mit Node.js/npm installiert)

```bash
# Installiere Dependencies
npm install

# Starte Dev-Server (Port 5174)
npm run dev

# Build für Produktion
npm run build
```

## 📖 Komponenten-Überblick

Die Component Library besteht aus **17 production-ready Komponenten**:

### Atoms (10)

✅ **Button** - Interactive buttons with 7 variants  
✅ **Badge** - Status badges with 6 variants  
✅ **Input** - Text input fields  
✅ **Checkbox** - Checkbox inputs  
✅ **Toggle** - Switch toggles  
✅ **Avatar** - User avatars with auto-generated colors  
✅ **Card** - Flexible card containers  
✅ **Label** - Form labels with validation  
✅ **Divider** - Visual separators

### Molecules (5)

✅ **Breadcrumbs** - Navigation breadcrumbs  
✅ **Toast** - Toast notifications  
✅ **Filter** - Multi-select filters  
✅ **Select** - Dropdown selects  
✅ **ContextMenu** - Right-click context menus

### Organisms (2)

✅ **Modal** - Modal dialogs (4 sizes)  
✅ **FileViewer** - Multi-type file preview (8+ file types)

## 🎯 Was wir NICHT implementiert haben (bewusst weggelassen):

❌ **Radio Buttons** - Für SyncSpace nicht nötig  
❌ **Textareas** - Input-Komponente reicht aus  
❌ **Range Sliders** - Nicht für Dateimanagement nötig  
❌ **Tables/DataGrid** - Benötigt Custom-Lösung  
❌ **Navbar** - Wird vom Backend definiert  
❌ **Sidebar** - Wird vom Backend definiert  
❌ **Pagination** - Nicht kritisch für unseren Use-Case  
❌ **Alerts** - Toast reicht aus

**Optional (könnten später hinzugefügt werden):**

- Tabs
- Accordion
- Timeline
- Progress Bars

## 📚 Dokumentation

- **README.md** - Main overview
- **COMPONENTS.md** - Complete API reference
- **GETTING_STARTED.md** - Quick start guide
- **INTEGRATION.md** - Integration guide

## 🎨 Design System

**Colors (7 Variants):**

- Primary (Blue)
- Secondary (Slate)
- Danger (Red)
- Success (Green)
- Warning (Amber)
- Info (Cyan)
- Ghost (Transparent)

**Sizes (5 Levels):**

- XS, SM, MD (default), LG, XL

**Stack:**

- Svelte 5 (runes)
- TypeScript
- Tailwind CSS v4
- Bootstrap Icons

## 📝 Notes

Die Component Library ist eine eigenständige Bibliothek und kann in jede Svelte 5 Anwendung integriert werden. Siehe `INTEGRATION.md` für Details.

---

**Status:** ✅ Production Ready  
**Version:** 1.0.0
