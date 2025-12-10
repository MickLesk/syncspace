# SyncSpace Frontend Refactoring TODO

**Erstellt:** 10. Dezember 2025  
**Letzte Aktualisierung:** 10. Dezember 2025  
**Analyse basierend auf:** 184 Svelte-Dateien im Frontend, 23 Dateien im _backup

---

## ✅ Abgeschlossene Aufgaben

### 1. Ungenutzte Komponenten verschoben (36 Dateien)
**Ziel:** `_backup/unused_components/`

- AccessibleModal.svelte
- AdvancedFilePreviewModal.svelte
- AdvancedUploadUI.svelte
- BatchOperationDialog.svelte
- BatchOperationsToolbar.svelte
- BatchProgressDialog.svelte
- ChartCard.svelte
- Chip.svelte
- CollaborationPanel.svelte
- ConnectionStatusBadge.svelte
- ContextMenu.svelte
- DeleteDialog.svelte
- DestinationPicker.svelte
- DragDropList.svelte
- FilterPanel.svelte
- HelpDialog.svelte
- Icon.svelte
- InfoCard.svelte
- InputDialog.svelte
- KeyboardShortcutsPanel.svelte
- LazyImage.svelte
- MobileContextMenu.svelte
- MobileNav.svelte
- NotificationBell.svelte
- NotificationCenter.svelte
- OptimizedVirtualList.svelte
- PageHeader.svelte
- PreviewModal.svelte
- ResponsiveImage.svelte
- SearchBar.svelte
- SkeletonLoader.svelte
- SortingHeader.svelte
- StatCard.svelte
- TabBar.svelte
- TanStackVirtualList.svelte
- ThemeSwitcher.svelte

### 2. HelpView integriert
- `pages/user/HelpView.svelte` aus _backup kopiert
- Route in App.svelte hinzugefügt (`#help`)
- Nutzt reines Tailwind/DaisyUI (kein custom CSS!)

---

## 📊 Übersicht

| Kategorie | Status |
|-----------|--------|
| **Svelte 5 Migration** | ✅ Komplett (keine `export let`, keine `$:` gefunden) |
| **Tailwind CSS** | ⚠️ Teilweise (viele Dateien mit custom `<style>` Blöcken) |
| **Styleguide Konformität** | ⚠️ Inkonsistent |
| **DaisyUI Komponenten** | ⚠️ Gemischt mit custom CSS |
| **Backup Integration** | ❓ 23 Dateien zu prüfen |

---

## 🚨 Priorität 1: Massive CSS-Dateien (>400 Zeilen Style)

Diese Dateien haben enorme `<style>` Blöcke die zu Tailwind/DaisyUI migriert werden sollten:

| Datei | CSS Zeilen | Priorität |
|-------|-----------|-----------|
| `AppHeader.svelte` | 1399 | 🔴 KRITISCH |
| `FilePreviewPanel.svelte` | 1042 | 🔴 KRITISCH |
| `StorageAnalyticsView.svelte` | 718 | 🔴 HOCH |
| `AdminDashboardView.svelte` | 643 | 🔴 HOCH |
| `SmartFoldersView.svelte` | 629 | 🔴 HOCH |
| `TrashView.svelte` | 587 | 🟡 MITTEL |
| `SecuritySettings.svelte` | 514 | 🟡 MITTEL |
| `CommentsPanel.svelte` | 468 | 🟡 MITTEL |
| `Sidebar.svelte` | 467 | 🟡 MITTEL |
| `SecurityPolicySettings.svelte` | 467 | 🟡 MITTEL |
| `StorageSettings.svelte` | 467 | 🟡 MITTEL |
| `EnhancedUploadManager.svelte` | 438 | 🟡 MITTEL |

### Empfehlung:
1. Custom CSS → Tailwind Utilities ersetzen
2. Komplexe Komponenten → DaisyUI Klassen nutzen
3. Animationen → Tailwind `transition-*`, `animate-*` Klassen
4. Dark Mode → `dark:` Prefix statt `:global(.dark)`

---

## 📁 Komponenten-Inventar

### ✅ Aktiv genutzt (in App.svelte importiert)

**Navigation & Layout:**
- `Sidebar.svelte` - ⚠️ 467 Zeilen CSS
- `AppHeader.svelte` - 🔴 1399 Zeilen CSS
- `MobileBottomNav.svelte`
- `PageWrapper.svelte`

**File Views:**
- `FilesView.svelte`
- `SharedView.svelte`
- `FavoritesView.svelte`
- `SmartFoldersView.svelte` - ⚠️ 629 Zeilen CSS
- `RecentFilesView.svelte`
- `TrashView.svelte` - ⚠️ 587 Zeilen CSS

**Admin & System:**
- `AdminView.svelte` (Hub)
- `AdminDashboardView.svelte` - ⚠️ 643 Zeilen CSS
- `JobsDashboard.svelte`
- `JobsQueueView.svelte`
- `BackupView.svelte`
- `UsersView.svelte`
- `SystemHealthView.svelte`
- `StorageAnalyticsView.svelte` - ⚠️ 718 Zeilen CSS

**Settings:**
- `SettingsHub.svelte`
- `UserProfileView.svelte`
- `UserSettingsView.svelte`
- `SecurityView.svelte`

**Tools:**
- `DuplicatesView.svelte`
- `ArchivesView.svelte`
- `CompressionView.svelte`
- `TagCloudView.svelte`

**UI Components:**
- `Toast.svelte`
- `LoadingOverlay.svelte`
- `ErrorBoundary.svelte`
- `CommandPalette.svelte`
- `ShortcutsModal.svelte`
- `UploadQueue.svelte`
- `WebSocketStatus.svelte`

### ❓ Potentiell ungenutzt (nicht in App.svelte)

Komponenten in `components/ui/` die möglicherweise nicht verwendet werden:

**🔴 UNGENUTZT (0 Imports - Können gelöscht werden):**
```
AccessibleModal.svelte       - Ersetzt durch Modal.svelte mit ARIA
AdvancedFilePreviewModal.svelte - Wird nicht mehr benötigt
AdvancedUploadUI.svelte      - Upload läuft über EnhancedUploadManager
BatchOperationDialog.svelte  - Ersetzt durch andere Batch-Dialoge
BatchOperationsToolbar.svelte - Nicht integriert
BatchProgressDialog.svelte   - Nicht verwendet
ChartCard.svelte             - Chart wird direkt verwendet
Chip.svelte                  - Nicht genutzt, Badges stattdessen
CollaborationPanel.svelte    - Feature nicht aktiv
ConnectionStatusBadge.svelte - WebSocketStatus stattdessen
ContextMenu.svelte           - MobileContextMenu stattdessen?
DeleteDialog.svelte          - ConfirmDialog stattdessen
DestinationPicker.svelte     - Im ModalPortal integriert
DragDropList.svelte          - Nicht verwendet
FilterPanel.svelte           - Nicht integriert
HelpDialog.svelte            - Nicht verwendet
Icon.svelte                  - Bootstrap Icons direkt
InfoCard.svelte              - ModernCard stattdessen
InputDialog.svelte           - Alerts/Prompts stattdessen
KeyboardShortcutsPanel.svelte - Shortcuts in CommandPalette
LazyImage.svelte             - Nicht verwendet
MobileContextMenu.svelte     - Nicht integriert
MobileNav.svelte             - MobileBottomNav stattdessen
NotificationBell.svelte      - Notifications in Header
NotificationCenter.svelte    - Nicht verwendet
OptimizedVirtualList.svelte  - VirtualList stattdessen
PageHeader.svelte            - Inline Headers
PreviewModal.svelte          - FilePreviewPanel stattdessen
ResponsiveImage.svelte       - Nicht verwendet
SearchBar.svelte             - Suche im Header
SkeletonLoader.svelte        - LoadingState stattdessen
SortingHeader.svelte         - Inline Sorting
StatCard.svelte              - ModernCard stattdessen
TabBar.svelte                - DaisyUI Tabs direkt
TanStackVirtualList.svelte   - VirtualList stattdessen
ThemeSwitcher.svelte         - Im Header integriert
```

**✅ AKTIV GENUTZT:**
```
Modal.svelte                 - 3 Imports (FilesView, TagCloud, UsersSettings)
ModernCard.svelte            - 16+ Imports (überall!)
Toast.svelte                 - 1 Import (App.svelte - global)
LoadingState.svelte          - 8 Imports
EmptyState.svelte            - 9 Imports  
Chart.svelte                 - 1 Import (StorageView)
Loading.svelte               - 2 Imports
VirtualList.svelte           - 1 Import (FilesView)
ProgressBar.svelte           - Indirekt via Toast/Upload
ErrorBoundary.svelte         - 1 Import (App.svelte)
CommandPalette.svelte        - 1 Import (App.svelte)
ShortcutsModal.svelte        - 1 Import (App.svelte)
UploadQueue.svelte           - 1 Import (App.svelte)
AppHeader.svelte             - 1 Import (App.svelte)
ConfirmDialog.svelte         - Mehrere Imports
ModernButton.svelte          - Einige Imports
```

---

## 📦 _backup Integration

Dateien im `_backup/frontend/src/` die geprüft werden müssen:

### UI Komponenten (möglicherweise moderne Versionen):
| Datei | Status | Aktion |
|-------|--------|--------|
| `ui/Card.svelte` | ✅ Modern (Svelte 5, Tailwind) | **INTEGRIEREN** als `ModernCard.svelte` Alternative |
| `ui/Button.svelte` | ✅ Svelte 5 | **PRÜFEN** gegen `ModernButton.svelte` |
| `ui/Badge.svelte` | ✅ Svelte 5 | **INTEGRIEREN** wenn nicht vorhanden |
| `ui/Avatar.svelte` | ✅ Svelte 5 | **INTEGRIEREN** wenn nicht vorhanden |
| `ui/Dialog.svelte` | ✅ Svelte 5 | **PRÜFEN** gegen `Modal.svelte` |
| `ui/Input.svelte` | ✅ Svelte 5 | **INTEGRIEREN** als Standard |
| `ui/Spinner.svelte` | ✅ Svelte 5 | **PRÜFEN** gegen `Loading.svelte` |
| `ui/Header.svelte` | ✅ Svelte 5 | **PRÜFEN** gegen `AppHeader.svelte` |
| `ui/Breadcrumb.svelte` | ✅ Svelte 5 | **PRÜFEN** gegen `Breadcrumbs.svelte` |

### Feature Komponenten:
| Datei | Status | Aktion |
|-------|--------|--------|
| `files/FolderColorPicker.svelte` | ✅ Svelte 5 | **OPTIONAL** - Farben werden bereits inline in `FilesView.svelte` und `ModalPortal.svelte` gesetzt. Separate Komponente wäre sauberer. |
| `collaboration/CollaborationPanel.svelte` | ⚠️ Alt | **PRÜFEN** gegen vorhandene |
| `search/SearchFilters.svelte` | ⚠️ Alt | **PRÜFEN** gegen `FilterPanel.svelte` |

### Responsive Komponenten:
| Datei | Status | Aktion |
|-------|--------|--------|
| `responsive/MobileNavigation.svelte` | ⚠️ Prüfen | **VERGLEICHEN** mit `MobileBottomNav.svelte` |
| `responsive/ResponsiveFileGrid.svelte` | ⚠️ Prüfen | **INTEGRIEREN** falls besser |
| `responsive/ResponsiveLayout.svelte` | ⚠️ Prüfen | **PRÜFEN** ob nötig |
| `responsive/ResponsiveSidebar.svelte` | ⚠️ Prüfen | **VERGLEICHEN** mit `Sidebar.svelte` |

### Pages:
| Datei | Status | Aktion |
|-------|--------|--------|
| `FileStatisticsView.svelte` | ❌ Alt (Svelte 4 Imports) | **IGNORIEREN** |
| `SharedViewNew.svelte` | ⚠️ Prüfen | **VERGLEICHEN** mit `SharedView.svelte` |
| `settings/AdvancedPreferencesView.svelte` | ⚠️ Prüfen | **PRÜFEN** ob Features fehlen |
| `system/AdminConsoleView.svelte` | ⚠️ Prüfen | **VERGLEICHEN** mit `AdminView.svelte` |
| `user/HelpView.svelte` | ⚠️ Prüfen | **INTEGRIEREN** falls nicht vorhanden |
| `showcase/ComponentGallery.svelte` | 🎨 Dev-Tool | **INTEGRIEREN** für Design-Doku |
| `showcase/DesignShowcase.svelte` | 🎨 Dev-Tool | **INTEGRIEREN** für Design-Doku |

---

## 🎨 Styleguide Konformität

### Abweichungen vom STYLEGUIDE.md

1. **Farbvariablen nicht genutzt:**
   - Viele Dateien nutzen direkte Hex-Werte statt CSS Custom Properties
   - `#22c55e` sollte `var(--color-primary)` sein

2. **Dark Mode Implementierung:**
   - Sollte: `dark:` Tailwind Prefix
   - Ist oft: `:global(.dark) .class { }` in `<style>`

3. **Button Styles:**
   - Viele custom `.btn-*` Klassen statt DaisyUI `btn btn-primary`

4. **Card Komponenten:**
   - Inkonsistente Rounded-Corners (sollte `rounded-xl` sein)
   - Verschiedene Shadow-Styles

5. **Spacing:**
   - Inkonsistente Padding/Margin Werte
   - Sollte Tailwind Spacing Scale nutzen

---

## 📋 Aktionsplan

### Phase 1: Cleanup (Niedrige Priorität)
- [ ] Ungenutzte Komponenten identifizieren und markieren/löschen
- [ ] Duplicate Komponenten zusammenführen (z.B. `Modal.svelte` vs `AccessibleModal.svelte`)
- [ ] Backup-Komponenten bewerten und integrieren

### Phase 2: CSS Migration (Hohe Priorität)
- [ ] `AppHeader.svelte` - CSS zu Tailwind migrieren
- [ ] `FilePreviewPanel.svelte` - CSS zu Tailwind migrieren
- [ ] `StorageAnalyticsView.svelte` - CSS zu Tailwind migrieren
- [ ] `AdminDashboardView.svelte` - CSS zu Tailwind migrieren
- [ ] Settings-Dateien vereinheitlichen

### Phase 3: Styleguide Enforcement
- [ ] CSS Custom Properties einführen (`app.css`)
- [ ] Dark Mode auf `dark:` Prefix umstellen
- [ ] DaisyUI Komponenten konsequent nutzen
- [ ] Konsistente Spacing/Typography

### Phase 4: Feature Integration aus _backup
- [ ] `FolderColorPicker.svelte` integrieren
- [ ] `ComponentGallery.svelte` als Dev-Tool einbinden
- [ ] Responsive Komponenten prüfen und ggf. übernehmen

---

## 🔍 Detaillierte Komponentenanalyse

### Modals (5 verschiedene!)
```
components/ui/Modal.svelte              - Basis Modal
components/ui/AccessibleModal.svelte    - Mit ARIA
components/ui/PreviewModal.svelte       - File Preview
components/ui/ShortcutsModal.svelte     - Keyboard Shortcuts
components/modals/ModalPortal.svelte    - Portal Container
```
**Empfehlung:** Zu einer einzigen `Modal.svelte` mit Varianten konsolidieren

### Buttons (3 verschiedene!)
```
components/ui/ModernButton.svelte       - Modern Style
_backup/.../Button.svelte               - Tailwind Style
Inline <button class="btn...">          - DaisyUI
```
**Empfehlung:** `ModernButton.svelte` als Standard, DaisyUI für schnelle UI

### Loading States (4 verschiedene!)
```
components/ui/Loading.svelte            - Spinner
components/ui/LoadingState.svelte       - State + Message
components/ui/SkeletonLoader.svelte     - Skeleton
components/LoadingOverlay.svelte        - Fullscreen
```
**Empfehlung:** Behalten als verschiedene Use-Cases, aber Styling vereinheitlichen

### Charts (2 verschiedene!)
```
components/ui/Chart.svelte              - Generic
components/ui/ChartCard.svelte          - Card Wrapper
```
**Empfehlung:** OK so, Chart.svelte als Base, ChartCard als Wrapper

---

## 🏁 Nächste Schritte

1. **Sofort:** 35 ungenutzte UI-Komponenten löschen (siehe Liste oben)
2. **Diese Woche:** Top 5 CSS-Heavy Dateien zu Tailwind migrieren (`AppHeader`, `FilePreviewPanel`, etc.)
3. **Nächste Woche:** Backup-UI-Komponenten vergleichen - `Card.svelte` aus _backup ist sauberer als `ModernCard.svelte`
4. **Ongoing:** Bei jeder Code-Änderung Styleguide einhalten

---

## 📝 Zusammenfassung

| Kategorie | Anzahl | Aktion |
|-----------|--------|--------|
| **Ungenutzte Komponenten** | ~35 | LÖSCHEN |
| **CSS-Heavy Dateien (>400 Zeilen)** | 12 | MIGRIEREN zu Tailwind |
| **Svelte 5 Probleme** | 0 | ✅ Keine |
| **_backup Komponenten** | 23 | 5 INTEGRIEREN, Rest ignorieren |
| **Aktiv genutzte UI-Komponenten** | ~17 | BEHALTEN |

### Empfohlene Integrationen aus _backup:
1. `Card.svelte` - Sauberer als `ModernCard.svelte`, reines Tailwind
2. `HelpView.svelte` - Feature fehlt komplett
3. `ComponentGallery.svelte` - Nützlich für Design-Dokumentation
4. `DesignShowcase.svelte` - Für Style-Referenz

### Sofort löschbare Dateien:
Alle Komponenten in der "🔴 UNGENUTZT" Liste können sicher gelöscht werden.

