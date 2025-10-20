# ⌨️ SyncSpace - Keyboard Shortcuts Reference

## Complete Keyboard Shortcut Guide

### File Operations

| Shortcut     | Action       | Context          | Description                                |
| ------------ | ------------ | ---------------- | ------------------------------------------ |
| **Ctrl + U** | Upload Files | Any view         | Opens file picker to upload files          |
| **Ctrl + N** | New Folder   | Files view       | Creates a new folder in current directory  |
| **F2**       | Rename       | 1 file selected  | Renames the currently selected file        |
| **Delete**   | Delete       | File(s) selected | Deletes selected file(s) with confirmation |
| **Ctrl + D** | Download     | 1 file selected  | Downloads the selected file _(planned)_    |

### Selection & Clipboard

| Shortcut     | Action     | Context          | Description                                    |
| ------------ | ---------- | ---------------- | ---------------------------------------------- |
| **Ctrl + A** | Select All | Files view       | Selects all files in current directory         |
| **Ctrl + C** | Copy       | File(s) selected | Copies selected files to clipboard             |
| **Ctrl + X** | Cut        | File(s) selected | Cuts selected files to clipboard               |
| **Ctrl + V** | Paste      | Any directory    | Pastes files from clipboard _(in development)_ |
| **Escape**   | Deselect   | Any selection    | Clears all selections and closes modals        |

### Navigation

| Shortcut      | Action | Context              | Description                               |
| ------------- | ------ | -------------------- | ----------------------------------------- |
| **Ctrl + F**  | Search | Any view             | Opens search view                         |
| **Ctrl + H**  | Home   | Any view             | Returns to home directory _(planned)_     |
| **Backspace** | Go Up  | Files view           | Navigate to parent directory _(planned)_  |
| **Enter**     | Open   | File/Folder selected | Opens folder or previews file _(planned)_ |

### View Controls

| Shortcut             | Action      | Context    | Description                                     |
| -------------------- | ----------- | ---------- | ----------------------------------------------- |
| **Ctrl + Shift + L** | Toggle View | Files view | Switches between grid and list view _(planned)_ |
| **Ctrl + Shift + D** | Dark Mode   | Any view   | Toggles dark/light theme _(planned)_            |
| **Ctrl + ,**         | Settings    | Any view   | Opens settings page _(planned)_                 |

### Advanced Operations

| Shortcut             | Action       | Context              | Description                           |
| -------------------- | ------------ | -------------------- | ------------------------------------- |
| **Ctrl + Shift + C** | Compress     | File(s) selected     | Creates ZIP archive _(planned)_       |
| **Ctrl + Shift + X** | Extract      | Archive selected     | Extracts archive contents _(planned)_ |
| **Ctrl + Shift + S** | Share Link   | 1 file selected      | Creates share link _(planned)_        |
| **Ctrl + B**         | Add Bookmark | File/Folder selected | Adds to favorites _(planned)_         |

### Preview & Quick Actions

| Shortcut             | Action         | Context         | Description                                     |
| -------------------- | -------------- | --------------- | ----------------------------------------------- |
| **Space**            | Quick Preview  | 1 file selected | Shows preview without opening modal _(planned)_ |
| **Shift + Delete**   | Delete Forever | Trash view      | Permanently deletes file _(planned)_            |
| **Ctrl + Z**         | Undo           | Any action      | Undoes last action _(planned)_                  |
| **Ctrl + Shift + Z** | Redo           | Any action      | Redoes last action _(planned)_                  |

---

## Context Menu Actions

**Right-click on file/folder** to access:

- 👁️ **Preview** - View file content
- ⬇️ **Download** - Download file
- ✏️ **Rename** - Rename file (or F2)
- 📋 **Copy** - Copy to clipboard
- ✂️ **Cut** - Cut to clipboard
- ⭐ **Favorite** - Add/remove from favorites
- 🗑️ **Delete** - Move to trash

---

## Global Shortcuts (Always Available)

| Shortcut     | Action         |
| ------------ | -------------- |
| **Ctrl + /** | Show this help |
| **Alt + 1**  | Files view     |
| **Alt + 2**  | Favorites view |
| **Alt + 3**  | Search view    |
| **Alt + 4**  | Trash view     |
| **Alt + 5**  | Peers view     |
| **Alt + 6**  | Settings view  |

---

## Multi-Select Mode

When multi-select is enabled:

- **Click** - Toggle selection
- **Shift + Click** - Range select _(planned)_
- **Ctrl + Click** - Add to selection _(planned)_
- **Delete** - Bulk delete
- **Ctrl + C/X** - Bulk copy/cut

---

## File Preview Shortcuts

When preview modal is open:

- **Escape** - Close preview
- **Left Arrow** - Previous file _(planned)_
- **Right Arrow** - Next file _(planned)_
- **+** - Zoom in (images) _(planned)_
- **-** - Zoom out (images) _(planned)_
- **0** - Reset zoom _(planned)_

---

## Search Shortcuts

In search view:

- **Ctrl + F** - Focus search input
- **Enter** - Execute search
- **Escape** - Clear search
- **Up/Down** - Navigate results _(planned)_

---

## Browser Compatibility Notes

### Windows/Linux

- Use **Ctrl** for all shortcuts

### macOS

- Replace **Ctrl** with **Cmd** (⌘)
- **Delete** may require **Fn + Delete**

---

## Implementation Status

### ✅ Fully Implemented (10)

1. Ctrl + U (Upload)
2. Ctrl + N (New Folder)
3. F2 (Rename)
4. Delete (Delete)
5. Ctrl + A (Select All)
6. Ctrl + C/X/V (Copy/Cut/Paste)
7. Escape (Deselect/Close)
8. Ctrl + F (Search)
9. Context Menu (Right-click)
10. Multi-select Mode Toggle

### 🚧 Partially Implemented (3)

- Copy/Cut/Paste (clipboard exists, paste not fully functional)
- Preview (works, but no keyboard navigation)
- Search (works, but no keyboard result navigation)

### 📋 Planned (15+)

- Advanced navigation (Enter, Backspace, Home)
- View toggles (Ctrl+Shift+L, Ctrl+Shift+D)
- Archive operations (Compress, Extract)
- Share links (Ctrl+Shift+S)
- Undo/Redo
- Quick preview (Space)
- Image zoom controls
- Range selection (Shift+Click)
- Alt+Number view switching
- Settings shortcut (Ctrl+,)

---

## Accessibility Features

### Screen Reader Support

- All buttons have proper ARIA labels
- File lists are keyboard navigable
- Form inputs are properly labeled

### High Contrast Mode

- Dark theme automatically adapts
- All colors meet WCAG AA standards
- Visual feedback for all interactions

### Reduced Motion

- Respects `prefers-reduced-motion`
- Animations can be disabled in settings _(planned)_

---

## Tips & Tricks

### Power User Shortcuts

1. **Quick Upload**: Press Ctrl+U anytime to start uploading
2. **Bulk Operations**: Enable multi-select, select files, then use keyboard shortcuts
3. **Context Menu**: Right-click is your friend - all actions available
4. **Favorites**: Star important folders for quick access
5. **Search**: Ctrl+F to find anything instantly

### Workflow Examples

**Organize Files:**

```
1. Ctrl+N - Create folder
2. Ctrl+A - Select all files
3. Drag to folder - Move files
```

**Bulk Delete:**

```
1. Enable multi-select mode (icon button)
2. Click files to select
3. Delete key - Confirm deletion
```

**Share Files:**

```
1. Right-click file
2. Select "Share"
3. Copy generated link
```

---

## Customization

### Planned Features

- Custom keyboard shortcut mapping
- Export/import shortcut preferences
- Conflict detection
- Shortcut cheat sheet overlay (Ctrl+/)

---

## Feedback

Found a shortcut not working? Want to suggest a new one?

Open an issue on GitHub or submit a pull request!

**Last Updated**: 2025-10-20
**Version**: 0.2.0
