# SyncSpace Design System & Styleguide

## 🎨 Übersicht

Dieses Dokument beschreibt das einheitliche Design-System für SyncSpace. Alle Komponenten und Seiten sollten diese Richtlinien befolgen, um ein konsistentes UI zu gewährleisten.

---

## 🎨 Farbpalette

### Primary Brand Color (Grün)
Das primäre Grün wird als Akzentfarbe für aktive Elemente, Fortschrittsbalken und positive Aktionen verwendet.

```css
/* Primary Green */
--color-primary: #22c55e;           /* green-500 - Hauptfarbe */
--color-primary-dark: #16a34a;      /* green-600 - Hover/Active */
--color-primary-light: #4ade80;     /* green-400 - Highlights */
--color-primary-bg: #dcfce7;        /* green-100 - Light Mode Background */
--color-primary-bg-dark: rgba(34, 197, 94, 0.2); /* Dark Mode Background */
```

### Secondary Colors (Für verschiedene Bereiche)
```css
/* Blue - Files, Links */
--color-blue: #3b82f6;
--color-blue-bg: #dbeafe;
--color-blue-bg-dark: rgba(59, 130, 246, 0.2);

/* Amber - Warnings, Trash */
--color-amber: #f59e0b;
--color-amber-bg: #fef3c7;
--color-amber-bg-dark: rgba(245, 158, 11, 0.2);

/* Purple - Activity, Special Features */
--color-purple: #a855f7;
--color-purple-bg: #f3e8ff;
--color-purple-bg-dark: rgba(168, 85, 247, 0.2);

/* Red - Errors, Delete, Danger */
--color-red: #ef4444;
--color-red-bg: #fee2e2;
--color-red-bg-dark: rgba(239, 68, 68, 0.2);
```

### Neutral Grays
```css
/* Light Mode */
--gray-50: #f9fafb;    /* Page Background */
--gray-100: #f3f4f6;   /* Card Background hover */
--gray-200: #e5e7eb;   /* Borders, Dividers */
--gray-300: #d1d5db;   /* Disabled elements */
--gray-400: #9ca3af;   /* Placeholder text */
--gray-500: #6b7280;   /* Secondary text */
--gray-600: #4b5563;   /* Icons inactive */
--gray-700: #374151;   /* Body text */
--gray-800: #1f2937;   /* Headers */
--gray-900: #111827;   /* Primary text */

/* Dark Mode */
--dark-bg: #111827;          /* Page Background */
--dark-surface: #1f2937;     /* Card Background */
--dark-border: #374151;      /* Borders */
--dark-text: #f9fafb;        /* Primary text */
--dark-text-secondary: #9ca3af; /* Secondary text */
```

---

## 📐 Layout & Spacing

### Basis-Einheiten
```css
--space-1: 0.25rem;   /* 4px */
--space-2: 0.5rem;    /* 8px */
--space-3: 0.75rem;   /* 12px */
--space-4: 1rem;      /* 16px */
--space-5: 1.25rem;   /* 20px */
--space-6: 1.5rem;    /* 24px */
--space-8: 2rem;      /* 32px */
```

### Container
```css
.page-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 1.5rem;
}
```

### Grid-Systeme
```css
/* 2-Column Grid (Settings, Forms) */
.settings-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

/* 3-Column Grid (Quick Actions) */
.quick-actions {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

/* 4-Column Stats Grid */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 0.75rem;
}

/* Responsive: Mobile = 1 Column */
@media (max-width: 768px) {
  .settings-grid,
  .quick-actions,
  .stats-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 1024px) {
  .settings-grid {
    grid-template-columns: 1fr;
  }
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
```

---

## 🃏 Card Components

### Standard Card
```css
.card {
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 0.75rem;
  padding: 1.25rem;
  transition: all 0.2s ease;
}

:global(.dark) .card {
  background: #1f2937;
  border-color: #374151;
}

.card:hover {
  border-color: #22c55e;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}
```

### Setting Card (Kompakt)
```css
.setting-card {
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 0.75rem;
  overflow: hidden;
}

:global(.dark) .setting-card {
  background: #1f2937;
  border-color: #374151;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  border-bottom: 1px solid #f3f4f6;
}

:global(.dark) .card-header {
  border-bottom-color: #374151;
}

.card-content {
  padding: 1rem;
}
```

### Card Icon
```css
.card-icon {
  width: 36px;
  height: 36px;
  border-radius: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.125rem;
  flex-shrink: 0;
}

/* Icon Farb-Varianten */
.card-icon.green-icon {
  background: #dcfce7;
  color: #22c55e;
}
:global(.dark) .card-icon.green-icon {
  background: rgba(34, 197, 94, 0.2);
}

.card-icon.blue-icon {
  background: #dbeafe;
  color: #3b82f6;
}
:global(.dark) .card-icon.blue-icon {
  background: rgba(59, 130, 246, 0.2);
}

.card-icon.amber-icon {
  background: #fef3c7;
  color: #f59e0b;
}
:global(.dark) .card-icon.amber-icon {
  background: rgba(245, 158, 11, 0.2);
}

.card-icon.purple-icon {
  background: #f3e8ff;
  color: #a855f7;
}
:global(.dark) .card-icon.purple-icon {
  background: rgba(168, 85, 247, 0.2);
}

.card-icon.red-icon {
  background: #fee2e2;
  color: #ef4444;
}
:global(.dark) .card-icon.red-icon {
  background: rgba(239, 68, 68, 0.2);
}
```

### Stats Card (Mini)
```css
.stat-card {
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  padding: 0.75rem;
  text-align: center;
}

:global(.dark) .stat-card {
  background: rgba(55, 65, 81, 0.3);
  border-color: #4b5563;
}

.stat-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: #22c55e;
}

.stat-label {
  font-size: 0.6875rem;
  font-weight: 500;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.025em;
}

:global(.dark) .stat-label {
  color: #9ca3af;
}
```

---

## 🔘 Buttons

### Primary Button (Grün)
```css
.btn-primary {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.5rem 1rem;
  background: linear-gradient(135deg, #22c55e, #16a34a);
  color: white;
  border: none;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary:hover {
  background: linear-gradient(135deg, #16a34a, #15803d);
  box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
  transform: translateY(-1px);
}
```

### Outline Button
```css
.btn-outline {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.5rem 1rem;
  background: transparent;
  color: #374151;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-outline:hover {
  border-color: #22c55e;
  color: #22c55e;
  background: rgba(34, 197, 94, 0.05);
}

:global(.dark) .btn-outline {
  color: #d1d5db;
  border-color: #4b5563;
}

:global(.dark) .btn-outline:hover {
  border-color: #22c55e;
  color: #22c55e;
}
```

### Danger Button
```css
.btn-danger {
  background: linear-gradient(135deg, #ef4444, #dc2626);
  color: white;
}

.btn-danger:hover {
  background: linear-gradient(135deg, #dc2626, #b91c1c);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
}
```

### Icon Button
```css
.btn-icon {
  width: 32px;
  height: 32px;
  padding: 0;
  border-radius: 0.375rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #6b7280;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-icon:hover {
  background: #f3f4f6;
  color: #22c55e;
}

:global(.dark) .btn-icon:hover {
  background: #374151;
}
```

---

## 📝 Form Elements

### Input Fields
```css
.input-group {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.field-label {
  font-size: 0.6875rem;
  font-weight: 500;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.025em;
}

:global(.dark) .field-label {
  color: #9ca3af;
}

input[type="text"],
input[type="number"],
input[type="email"],
input[type="password"],
select,
textarea {
  padding: 0.5rem 0.75rem;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  background: white;
  color: #111827;
  transition: all 0.15s ease;
}

:global(.dark) input,
:global(.dark) select,
:global(.dark) textarea {
  background: #374151;
  border-color: #4b5563;
  color: #f9fafb;
}

input:focus,
select:focus,
textarea:focus {
  outline: none;
  border-color: #22c55e;
  box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
}
```

### Toggle Switch
```css
.toggle {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background: #d1d5db;
  border-radius: 24px;
  transition: all 0.3s ease;
}

.toggle-slider::before {
  content: "";
  position: absolute;
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background: white;
  border-radius: 50%;
  transition: all 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle input:checked + .toggle-slider {
  background: linear-gradient(135deg, #22c55e, #16a34a);
}

.toggle input:checked + .toggle-slider::before {
  transform: translateX(20px);
}

:global(.dark) .toggle-slider {
  background: #4b5563;
}
```

### Checkbox
```css
.checkbox-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.checkbox-item input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: #22c55e;
}

.checkbox-item span {
  font-size: 0.8125rem;
  color: #374151;
}

:global(.dark) .checkbox-item span {
  color: #d1d5db;
}
```

---

## 📊 Progress Bars

### Standard Progress Bar
```css
.progress-container {
  height: 8px;
  background: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
}

:global(.dark) .progress-container {
  background: #374151;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #22c55e, #16a34a);
  border-radius: 4px;
  transition: width 0.5s ease;
}
```

### Thin Progress Bar
```css
.progress-thin {
  height: 4px;
}
```

---

## 🔔 Badges & Tags

### Status Badge
```css
.badge {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 500;
  border-radius: 9999px;
}

.badge-success {
  background: #dcfce7;
  color: #16a34a;
}

:global(.dark) .badge-success {
  background: rgba(34, 197, 94, 0.2);
  color: #4ade80;
}

.badge-warning {
  background: #fef3c7;
  color: #d97706;
}

:global(.dark) .badge-warning {
  background: rgba(245, 158, 11, 0.2);
  color: #fbbf24;
}

.badge-error {
  background: #fee2e2;
  color: #dc2626;
}

:global(.dark) .badge-error {
  background: rgba(239, 68, 68, 0.2);
  color: #f87171;
}

.badge-info {
  background: #dbeafe;
  color: #2563eb;
}

:global(.dark) .badge-info {
  background: rgba(59, 130, 246, 0.2);
  color: #60a5fa;
}
```

### Notification Badge (Count)
```css
.notification-badge {
  min-width: 18px;
  height: 18px;
  padding: 0 0.375rem;
  font-size: 0.6875rem;
  font-weight: 600;
  background: #22c55e;
  color: white;
  border-radius: 9999px;
  display: flex;
  align-items: center;
  justify-content: center;
}
```

---

## 🎭 Animations

### Standard Transitions
```css
/* Schnelle UI-Interaktionen */
transition: all 0.15s ease;

/* Standard-Übergänge */
transition: all 0.2s ease;

/* Smooth Animationen */
transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
```

### Hover Scale
```css
.hover-scale:hover {
  transform: scale(1.02);
}

.hover-lift:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
}
```

### Spinner
```css
.spinner {
  width: 24px;
  height: 24px;
  border: 2px solid #e5e7eb;
  border-top-color: #22c55e;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
```

### Fade In
```css
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-8px); }
  to { opacity: 1; transform: translateY(0); }
}

.fade-in {
  animation: fadeIn 0.2s ease-out;
}
```

### Slide In
```css
@keyframes slideIn {
  from { transform: translateX(-100%); }
  to { transform: translateX(0); }
}

@keyframes slideOut {
  from { transform: translateX(0); }
  to { transform: translateX(-100%); }
}
```

---

## 📱 Responsive Breakpoints

```css
/* Mobile First */
@media (min-width: 640px) { /* sm */ }
@media (min-width: 768px) { /* md */ }
@media (min-width: 1024px) { /* lg */ }
@media (min-width: 1280px) { /* xl */ }
@media (min-width: 1536px) { /* 2xl */ }
```

---

## 🧩 Component Patterns

### Page Header
```html
<div class="page-header">
  <h1 class="page-title">
    <i class="bi bi-icon-name"></i>
    Page Title
  </h1>
  <p class="page-subtitle">Optional subtitle or description</p>
</div>
```

```css
.page-header {
  margin-bottom: 1.5rem;
}

.page-title {
  font-size: 1.5rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #111827;
}

:global(.dark) .page-title {
  color: #f9fafb;
}

.page-title i {
  color: #22c55e;
}

.page-subtitle {
  font-size: 0.875rem;
  color: #6b7280;
  margin-top: 0.25rem;
}

:global(.dark) .page-subtitle {
  color: #9ca3af;
}
```

### Section Header
```html
<div class="section-header">
  <h2 class="section-title">Section Title</h2>
  <div class="section-actions">
    <button class="btn-outline">Action</button>
  </div>
</div>
```

### Empty State
```html
<div class="empty-state">
  <i class="bi bi-inbox"></i>
  <h3>No items found</h3>
  <p>Description or call to action</p>
</div>
```

```css
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  text-align: center;
  color: #6b7280;
}

.empty-state i {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-state h3 {
  font-size: 1rem;
  font-weight: 600;
  color: #374151;
}

:global(.dark) .empty-state h3 {
  color: #d1d5db;
}
```

---

## 📋 Tables

```css
.table-container {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th {
  text-align: left;
  padding: 0.75rem 1rem;
  font-size: 0.6875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: #6b7280;
  background: #f9fafb;
  border-bottom: 1px solid #e5e7eb;
}

:global(.dark) th {
  background: #1f2937;
  color: #9ca3af;
  border-bottom-color: #374151;
}

td {
  padding: 0.75rem 1rem;
  font-size: 0.875rem;
  color: #374151;
  border-bottom: 1px solid #f3f4f6;
}

:global(.dark) td {
  color: #d1d5db;
  border-bottom-color: #374151;
}

tr:hover td {
  background: #f9fafb;
}

:global(.dark) tr:hover td {
  background: rgba(55, 65, 81, 0.3);
}
```

---

## 🖼️ Icons

Verwende **Bootstrap Icons** für alle Icons:

```html
<i class="bi bi-folder-fill"></i>
<i class="bi bi-gear-fill"></i>
<i class="bi bi-check-circle-fill"></i>
```

### Icon Größen
```css
.icon-sm { font-size: 0.875rem; }  /* 14px */
.icon-md { font-size: 1.125rem; }  /* 18px */
.icon-lg { font-size: 1.5rem; }    /* 24px */
.icon-xl { font-size: 2rem; }      /* 32px */
```

---

## ✅ Checkliste für neue Komponenten

1. [ ] Verwendet die definierten Farbvariablen
2. [ ] Hat Dark Mode Support (`:global(.dark)`)
3. [ ] Responsive auf Mobile (Breakpoints)
4. [ ] Hover/Focus-States definiert
5. [ ] Konsistente Spacing-Werte
6. [ ] Accessibility (aria-labels, focus-visible)
7. [ ] Smooth Transitions/Animations
8. [ ] Konsistente Border-Radius (0.5rem, 0.75rem)
9. [ ] Verwendet die Standard-Schriftgrößen

---

## 📁 Dateistruktur

```
frontend/src/
├── components/
│   ├── ui/           # Generic UI components
│   ├── navigation/   # Sidebar, Navbar
│   └── files/        # File-specific components
├── pages/
│   ├── Dashboard.svelte
│   ├── settings/     # Settings pages
│   └── ...
├── stores/           # Svelte stores
├── lib/              # Utilities, API
└── styles/
    └── app.css       # Global styles + Tailwind
```
