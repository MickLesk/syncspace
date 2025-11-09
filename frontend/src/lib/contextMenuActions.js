/**
 * Context Menu Actions Service
 * 
 * Manages all context menu actions for files and folders with proper
 * permission checking, file type detection, and action filtering.
 * 
 * Usage:
 *   const items = getContextMenuItems(file, selectionState, permissions);
 */

import api from './api.js';
import { toast } from '../stores/toast.js';

/**
 * Get context menu items based on file/folder and context
 */
export function getContextMenuItems(item, context = {}) {
  if (!item) return [];

  const { 
    isSelected = false,
    multiSelected = false,
    canEdit = true,
    canDelete = true,
    canShare = true,
    isTrashed = false,
  } = context;

  const items = [];

  // **Clipboard Operations** (top section)
  if (!isTrashed) {
    items.push({
      id: 'copy',
      label: 'Kopieren',
      icon: 'bi-files',
      shortcut: 'Ctrl+C',
      action: async () => {
        await copyToClipboard([item]);
      },
    });

    items.push({
      id: 'cut',
      label: 'Ausschneiden',
      icon: 'bi-scissors',
      shortcut: 'Ctrl+X',
      action: async () => {
        await cutToClipboard([item]);
      },
      disabled: !canEdit,
    });

    // Check if clipboard has content
    const hasClipboard = localStorage.getItem('clipboard_items');
    items.push({
      id: 'paste',
      label: 'Einfügen',
      icon: 'bi-clipboard',
      shortcut: 'Ctrl+V',
      action: async () => {
        await pasteFromClipboard();
      },
      disabled: !hasClipboard || !canEdit || item.type !== 'folder',
    });
  }

  // **Divider**
  items.push({ type: 'divider' });

  // **File Actions** (second section)
  if (!isTrashed) {
    if (item.type === 'file') {
      items.push({
        id: 'download',
        label: 'Herunterladen',
        icon: 'bi-download',
        shortcut: 'Shift+D',
        action: async () => {
          await downloadFile(item);
        },
      });

      items.push({
        id: 'preview',
        label: 'Vorschau',
        icon: 'bi-eye',
        shortcut: 'P',
        action: async () => {
          window.dispatchEvent(
            new CustomEvent('file-preview', { detail: item })
          );
        },
      });
    }

    if (item.type === 'folder') {
      items.push({
        id: 'new-file',
        label: 'Neue Datei',
        icon: 'bi-file-earmark-plus',
        action: async () => {
          window.dispatchEvent(
            new CustomEvent('new-file', { detail: item.path })
          );
        },
      });

      items.push({
        id: 'new-folder',
        label: 'Neuer Ordner',
        icon: 'bi-folder-plus',
        action: async () => {
          window.dispatchEvent(
            new CustomEvent('new-folder', { detail: item.path })
          );
        },
      });
    }

    // **Rename & Properties**
    if (canEdit) {
      items.push({
        id: 'rename',
        label: 'Umbenennen',
        icon: 'bi-pencil',
        shortcut: 'F2',
        action: async () => {
          window.dispatchEvent(
            new CustomEvent('rename-file', { detail: item })
          );
        },
      });
    }

    items.push({
      id: 'properties',
      label: 'Eigenschaften',
      icon: 'bi-info-circle',
      shortcut: 'Alt+Enter',
      action: async () => {
        window.dispatchEvent(
          new CustomEvent('show-properties', { detail: item })
        );
      },
    });
  }

  // **Divider**
  items.push({ type: 'divider' });

  // **Advanced Actions** (third section)
  if (!isTrashed) {
    // Compress/Archive
    if (item.type === 'file') {
      items.push({
        id: 'compress',
        label: 'Komprimieren',
        icon: 'bi-file-zip',
        action: async () => {
          await compressFile(item);
        },
        disabled: !canEdit,
      });
    }

    // Share
    if (canShare) {
      items.push({
        id: 'share',
        label: 'Teilen',
        icon: 'bi-share',
        shortcut: 'Shift+S',
        action: async () => {
          window.dispatchEvent(
            new CustomEvent('show-share', { detail: item })
          );
        },
      });
    }

    // Favorites
    const isFavorited = item.is_favorite;
    items.push({
      id: isFavorited ? 'unfavorite' : 'favorite',
      label: isFavorited ? 'Aus Favoriten entfernen' : 'Zu Favoriten hinzufügen',
      icon: isFavorited ? 'bi-star-fill' : 'bi-star',
      action: async () => {
        await toggleFavorite(item);
      },
    });

    // Tags
    items.push({
      id: 'tags',
      label: 'Tags',
      icon: 'bi-tags',
      action: async () => {
        window.dispatchEvent(
          new CustomEvent('show-tags', { detail: item })
        );
      },
    });

    // Version History
    if (item.type === 'file') {
      items.push({
        id: 'version-history',
        label: 'Versionsverlauf',
        icon: 'bi-clock-history',
        action: async () => {
          window.dispatchEvent(
            new CustomEvent('show-history', { detail: item })
          );
        },
      });
    }
  } else {
    // **Trash Actions**
    items.push({
      id: 'restore',
      label: 'Wiederherstellen',
      icon: 'bi-arrow-counterclockwise',
      action: async () => {
        await restoreFromTrash(item);
      },
    });
  }

  // **Divider**
  if (canDelete || isTrashed) {
    items.push({ type: 'divider' });

    // **Delete/Permanent Delete** (danger section)
    if (isTrashed) {
      items.push({
        id: 'delete-permanent',
        label: 'Endgültig löschen',
        icon: 'bi-trash-fill',
        action: async () => {
          await deletePermanent(item);
        },
        danger: true,
      });
    } else if (canDelete) {
      items.push({
        id: 'delete',
        label: 'In Papierkorb',
        icon: 'bi-trash',
        shortcut: 'Del',
        action: async () => {
          await deleteToTrash(item);
        },
        danger: true,
      });
    }
  }

  return items;
}

/**
 * Copy file(s) to clipboard
 */
async function copyToClipboard(files) {
  try {
    localStorage.setItem('clipboard_items', JSON.stringify({
      action: 'copy',
      items: files.map(f => ({
        path: f.path,
        type: f.type,
        name: f.name,
      })),
      timestamp: Date.now(),
    }));
    
    console.log(`✅ ${files.length} item(s) copied to clipboard`);
  } catch (error) {
    console.error('Failed to copy to clipboard:', error);
  }
}

/**
 * Cut file(s) to clipboard
 */
async function cutToClipboard(files) {
  try {
    localStorage.setItem('clipboard_items', JSON.stringify({
      action: 'cut',
      items: files.map(f => ({
        path: f.path,
        type: f.type,
        name: f.name,
      })),
      timestamp: Date.now(),
    }));
    
    console.log(`✅ ${files.length} item(s) cut to clipboard`);
  } catch (error) {
    console.error('Failed to cut to clipboard:', error);
  }
}

/**
 * Paste from clipboard
 */
async function pasteFromClipboard() {
  try {
    const clipboardData = JSON.parse(localStorage.getItem('clipboard_items'));
    if (!clipboardData) return;

    const { action, items } = clipboardData;

    if (action === 'copy') {
      // Copy files
      for (const item of items) {
        await api.files.copy(item.path, item.path + '_copy');
      }
    } else if (action === 'cut') {
      // Move files
      for (const item of items) {
        await api.files.move(item.path, item.path);
      }
      localStorage.removeItem('clipboard_items');
    }

    window.dispatchEvent(new CustomEvent('files-changed'));
  } catch (error) {
    console.error('Failed to paste from clipboard:', error);
  }
}

/**
 * Download file
 */
async function downloadFile(file) {
  try {
    const response = await api.files.download(file.path);
    const blob = await response.blob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = file.name;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('Failed to download file:', error);
  }
}

/**
 * Toggle favorite status
 */
async function toggleFavorite(item) {
  try {
    if (item.is_favorite) {
      await api.files.removeFavorite(item.path);
    } else {
      await api.files.addFavorite(item.path);
    }
    item.is_favorite = !item.is_favorite;
    window.dispatchEvent(new CustomEvent('files-changed'));
  } catch (error) {
    console.error('Failed to toggle favorite:', error);
  }
}

/**
 * Compress file
 */
async function compressFile(file) {
  try {
    const response = await api.batch.compress([file.path]);
    console.log('✅ File compressed:', response);
    window.dispatchEvent(new CustomEvent('files-changed'));
  } catch (error) {
    console.error('Failed to compress file:', error);
  }
}

/**
 * Delete to trash
 */
async function deleteToTrash(item) {
  try {
    const confirmed = confirm(
      `Soll "${item.name}" gelöscht werden?`
    );
    
    if (!confirmed) return;

    await api.files.delete(item.path);
    window.dispatchEvent(new CustomEvent('files-changed'));
  } catch (error) {
    console.error('Failed to delete file:', error);
  }
}

/**
 * Restore from trash
 */
async function restoreFromTrash(item) {
  try {
    await api.trash.restore(item.id);
    window.dispatchEvent(new CustomEvent('files-changed'));
  } catch (error) {
    console.error('Failed to restore file:', error);
  }
}

/**
 * Permanently delete
 */
async function deletePermanent(item) {
  try {
    const confirmed = confirm(
      `"${item.name}" wird endgültig gelöscht und kann nicht wiederhergestellt werden. Fortfahren?`
    );
    
    if (!confirmed) return;

    await api.trash.deletePermanent(item.id);
    window.dispatchEvent(new CustomEvent('files-changed'));
  } catch (error) {
    console.error('Failed to delete permanently:', error);
  }
}
