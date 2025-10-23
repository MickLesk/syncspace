# Crystal Glass UI Components

Moderne, wiederverwendbare Svelte-Komponenten für SyncSpace mit Crystal Glass Design System.

## 🎨 Design Principles

- **Crystal Glass Aesthetic**: Transparente Backgrounds mit Blur-Effekten
- **Inter Font**: Moderne, saubere Typography
- **Material Expressive 3**: Vibrant Colors & Gradients
- **Smooth Animations**: 60 FPS cubic-bezier Transitions
- **Dark Mode Support**: Automatische Anpassung an System-Theme

---

## 📦 Verfügbare Komponenten

### 🔘 Button
Vielseitiger Button mit verschiedenen Stilen.

**Props:**
- `variant`: `"filled"` | `"outlined"` | `"text"` | `"glass"` | `"glass-primary"` (default: `"filled"`)
- `size`: `"small"` | `"medium"` | `"large"` (default: `"medium"`)
- `disabled`: boolean (default: `false`)
- `icon`: string - Bootstrap Icon class
- `fullWidth`: boolean (default: `false`)
- `onClick`: function

**Beispiel:**
```svelte
<Button variant="glass-primary" size="large" icon="bi-plus-lg">
  New File
</Button>
```

---

### 📊 StatCard
Statistik-Karte mit Gradient-Icon.

**Props:**
- `icon`: string - Bootstrap Icon (z.B. `"bi-files"`)
- `label`: string - Beschreibung
- `value`: string | number - Hauptwert
- `gradient`: string - CSS Gradient (default: purple-violet)
- `iconSize`: number (default: `28`)

**Beispiel:**
```svelte
<StatCard
  icon="bi-files"
  label="Total Files"
  value={1234}
  gradient="linear-gradient(135deg, #10b981, #34d399)"
/>
```

---

### 📈 ChartCard
Container für Charts und Diagramme.

**Props:**
- `title`: string (optional)
- `icon`: string - Bootstrap Icon (optional)

**Beispiel:**
```svelte
<ChartCard title="Usage Statistics" icon="bi-pie-chart-fill">
  <canvas bind:this={chartCanvas}></canvas>
</ChartCard>
```

---

### 🏷️ Badge
Kleine Labels für Status, Tags, etc.

**Props:**
- `variant`: `"default"` | `"success"` | `"warning"` | `"error"` | `"info"` | `"glass"`
- `size`: `"small"` | `"medium"` | `"large"` (default: `"medium"`)
- `icon`: string - Bootstrap Icon (optional)

**Beispiel:**
```svelte
<Badge variant="success" icon="bi-check-circle">
  Active
</Badge>
```

---

### 📊 ProgressBar
Fortschrittsbalken mit Animation.

**Props:**
- `value`: number - Aktueller Wert
- `max`: number - Maximum (default: `100`)
- `variant`: `"primary"` | `"success"` | `"warning"` | `"error"` | `"glass"`
- `size`: `"small"` | `"medium"` | `"large"` (default: `"medium"`)
- `showLabel`: boolean (default: `true`)
- `animated`: boolean - Shimmer-Effekt (default: `true`)

**Beispiel:**
```svelte
<ProgressBar
  value={750}
  max={1000}
  variant="success"
  showLabel={true}
/>
```

---

### 🗂️ TabBar
Tab-Navigation mit verschiedenen Stilen.

**Props:**
- `tabs`: Array von `{ id, label, icon? }`
- `activeTab`: string - ID des aktiven Tabs
- `onChange`: function(id) - Callback bei Tab-Wechsel
- `variant`: `"pills"` | `"underline"` | `"glass"` (default: `"pills"`)

**Beispiel:**
```svelte
<TabBar
  tabs={[
    { id: "files", label: "Files", icon: "bi-files" },
    { id: "images", label: "Images", icon: "bi-image" },
    { id: "videos", label: "Videos", icon: "bi-play-circle" }
  ]}
  activeTab={selectedTab}
  onChange={(id) => selectedTab = id}
  variant="glass"
/>
```

---

### ⏳ Spinner
Loading-Indikatoren.

**Props:**
- `size`: `"small"` | `"medium"` | `"large"` (default: `"medium"`)
- `variant`: `"circular"` | `"dots"` | `"bars"` (default: `"circular"`)

**Beispiel:**
```svelte
<Spinner variant="dots" size="large" />
```

---

### 🎫 Chip
Filter-Chips und Tags.

**Props:**
- `label`: string
- `icon`: string - Bootstrap Icon (optional)
- `selected`: boolean (default: `false`)
- `variant`: `"filter"` | `"tag"` | `"removable"` (default: `"filter"`)
- `size`: `"small"` | `"medium"` | `"large"` (default: `"medium"`)
- `onRemove`: function (nur bei `variant="removable"`)
- `onClick`: function

**Beispiel:**
```svelte
<Chip
  label="Images"
  icon="bi-image"
  selected={filter === "images"}
  onClick={() => filter = "images"}
/>

<Chip
  label="Important"
  variant="removable"
  onRemove={() => removeTag("important")}
/>
```

---

### 🎴 InfoCard
Flexible Informations-Karte.

**Props:**
- `title`: string (optional)
- `description`: string (optional)
- `variant`: `"default"` | `"bordered"` | `"glass"` | `"gradient"`
- `hoverable`: boolean - Hover-Animation (default: `false`)
- `clickable`: boolean - Macht Card klickbar (default: `false`)
- `padding`: `"small"` | `"medium"` | `"large"` (default: `"medium"`)
- `onClick`: function (nur wenn `clickable`)

**Beispiel:**
```svelte
<InfoCard
  title="Storage Usage"
  description="Your current storage statistics"
  variant="glass"
  hoverable={true}
>
  <ProgressBar value={75} max={100} variant="primary" />
</InfoCard>
```

---

### 💬 Dialog
Modale Dialoge (bereits vorhanden, refactored).

**Props:**
- `open`: boolean - Öffnet/schließt Dialog
- `title`: string
- `confirmText`: string (default: `"OK"`)
- `cancelText`: string (default: `"Abbrechen"`)
- `confirmVariant`: Button-Variant (default: `"filled"`)
- `showCancel`: boolean (default: `true`)
- `danger`: boolean - Danger-Styling (default: `false`)

**Events:**
- `on:confirm` - Bei Bestätigung
- `on:cancel` - Bei Abbruch

**Beispiel:**
```svelte
<Dialog
  bind:open={showDialog}
  title="Delete File?"
  confirmText="Delete"
  danger={true}
  on:confirm={handleDelete}
>
  <p>Are you sure you want to delete this file?</p>
</Dialog>
```

---

### ✏️ Input
Text-Eingabefeld mit Label (bereits vorhanden, refactored).

**Props:**
- `value`: string
- `label`: string
- `type`: string (default: `"text"`)
- `placeholder`: string
- `disabled`: boolean
- `error`: string - Fehlermeldung
- `required`: boolean
- `icon`: string - Leading icon

**Beispiel:**
```svelte
<Input
  bind:value={username}
  label="Username"
  icon="👤"
  required={true}
  error={usernameError}
/>
```

---

### 🔍 Icon
Icon-Wrapper (bereits vorhanden).

**Props:**
- `name`: string - Bootstrap Icon name
- `size`: number (default: `24`)
- `color`: string (optional)

**Beispiel:**
```svelte
<Icon name="files" size={32} color="#6366f1" />
```

---

## 🎯 CSS Utilities (Global)

Diese Utility-Classes sind in `app.css` verfügbar:

### Glass Effects
```svelte
<div class="glass-card">...</div>
<div class="glass-frosted">...</div>
<button class="glass-button">...</button>
<button class="glass-button-primary">...</button>
<input class="glass-input" />
<span class="glass-badge">...</span>
```

### Layouts
```svelte
<div class="page-header gradient-bg">...</div>
<div class="page-content">...</div>
```

### Animations
```svelte
<div class="gradient-bg">Animated Gradient Background</div>
```

---

## 🚀 Best Practices

1. **Konsistenz**: Nutze die vorgefertigten Komponenten statt eigene zu erstellen
2. **Varianten**: Wähle passende Variants (`glass`, `gradient`, etc.)
3. **Icons**: Verwende Bootstrap Icons (`bi-*`) für Konsistenz
4. **Gradients**: Nutze die vordefinierten Gradient-Kombinationen
5. **Spacing**: Halte dich an 8px Grid (8, 16, 24, 32, 40px)

---

## 📝 Beispiel-Layout

```svelte
<script>
  import Button from "../components/ui/Button.svelte";
  import StatCard from "../components/ui/StatCard.svelte";
  import TabBar from "../components/ui/TabBar.svelte";
  import InfoCard from "../components/ui/InfoCard.svelte";
  import Badge from "../components/ui/Badge.svelte";
</script>

<div class="page-header gradient-bg">
  <div class="header-glow"></div>
  <div class="header-content">
    <div class="icon-circle">
      <Icon name="dashboard" size={32} color="#fff" />
    </div>
    <h1>Dashboard</h1>
  </div>
</div>

<div class="page-content">
  <!-- Stats -->
  <div class="stats-grid">
    <StatCard
      icon="bi-files"
      label="Total Files"
      value={1234}
      gradient="linear-gradient(135deg, #6366f1, #8b5cf6)"
    />
    <StatCard
      icon="bi-hdd-fill"
      label="Storage Used"
      value="75 GB"
      gradient="linear-gradient(135deg, #10b981, #34d399)"
    />
  </div>

  <!-- Tabs -->
  <TabBar
    tabs={[
      { id: "overview", label: "Overview" },
      { id: "files", label: "Files", icon: "bi-files" },
      { id: "settings", label: "Settings", icon: "bi-gear" }
    ]}
    activeTab={currentTab}
    onChange={(id) => currentTab = id}
    variant="glass"
  />

  <!-- Content -->
  <InfoCard
    title="Recent Activity"
    variant="glass"
    hoverable={true}
  >
    <Badge variant="success" icon="bi-check">3 uploads</Badge>
    <Badge variant="info" icon="bi-download">2 downloads</Badge>
  </InfoCard>
</div>
```

---

## 🎨 Color Reference

**Primary Colors:**
- Primary: `#6366f1` (Indigo)
- Secondary: `#8b5cf6` (Purple)
- Accent: `#d946ef` (Pink)

**Semantic Colors:**
- Success: `#10b981` (Green)
- Warning: `#f59e0b` (Orange)
- Error: `#ef4444` (Red)
- Info: `#3b82f6` (Blue)

**Gradients:**
```css
linear-gradient(135deg, #6366f1, #8b5cf6) /* Primary */
linear-gradient(135deg, #10b981, #34d399) /* Success */
linear-gradient(135deg, #f59e0b, #fbbf24) /* Warning */
linear-gradient(135deg, #ef4444, #dc2626) /* Error */
```

---

## 📱 Responsive Design

Alle Komponenten sind responsive und passen sich automatisch an:
- Mobile: < 768px
- Tablet: 768px - 1024px
- Desktop: > 1024px

---

## 🌙 Dark Mode

Alle Komponenten unterstützen automatisch Dark Mode via:
```css
@media (prefers-color-scheme: dark) {
  /* Dark mode styles */
}
```
