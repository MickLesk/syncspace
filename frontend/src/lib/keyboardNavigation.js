/**
 * Keyboard Navigation & Shortcuts Manager
 * Handles keyboard events with proper event handling and shortcuts
 */

/**
 * Keyboard event types
 */
export const KEYBOARD_EVENTS = {
  // Navigation
  ARROW_UP: 'ArrowUp',
  ARROW_DOWN: 'ArrowDown',
  ARROW_LEFT: 'ArrowLeft',
  ARROW_RIGHT: 'ArrowRight',
  
  // Editing
  ENTER: 'Enter',
  ESCAPE: 'Escape',
  DELETE: 'Delete',
  BACKSPACE: 'Backspace',
  TAB: 'Tab',
  
  // Modifiers
  CTRL: 'Control',
  SHIFT: 'Shift',
  ALT: 'Alt',
  META: 'Meta', // Cmd on Mac
};

/**
 * Default keyboard shortcuts
 */
export const DEFAULT_SHORTCUTS = {
  // File operations
  COPY: { key: 'c', ctrl: true, name: 'Copy' },
  CUT: { key: 'x', ctrl: true, name: 'Cut' },
  PASTE: { key: 'v', ctrl: true, name: 'Paste' },
  DELETE: { key: 'Delete', name: 'Delete' },
  RENAME: { key: 'F2', name: 'Rename' },
  
  // Navigation
  UPLOAD: { key: 'u', ctrl: true, name: 'Upload' },
  REFRESH: { key: 'F5', name: 'Refresh' },
  SEARCH: { key: '/', ctrl: true, name: 'Quick Search' },
  
  // UI
  ESCAPE: { key: 'Escape', name: 'Close/Cancel' },
  TAB: { key: 'Tab', name: 'Navigate' },
  HELP: { key: '?', shift: true, name: 'Help/Shortcuts' },
};

/**
 * Create a keyboard shortcut handler
 * @param {Object} shortcuts - Custom shortcuts to merge with defaults
 * @returns {Object} Keyboard manager
 */
export function createKeyboardManager(shortcuts = {}) {
  const allShortcuts = { ...DEFAULT_SHORTCUTS, ...shortcuts };
  const listeners = new Map();

  /**
   * Register a keyboard event listener
   * @param {string} name - Shortcut name
   * @param {Function} handler - Callback function
   * @param {Object} options - Additional options
   */
  function on(name, handler, options = {}) {
    if (!listeners.has(name)) {
      listeners.set(name, []);
    }
    listeners.get(name).push({ handler, options });
  }

  /**
   * Unregister a keyboard event listener
   * @param {string} name - Shortcut name
   * @param {Function} handler - Callback function to remove
   */
  function off(name, handler) {
    if (listeners.has(name)) {
      const handlers = listeners.get(name);
      const index = handlers.findIndex((h) => h.handler === handler);
      if (index > -1) {
        handlers.splice(index, 1);
      }
    }
  }

  /**
   * Check if a keyboard event matches a shortcut
   * @param {KeyboardEvent} event - Keyboard event
   * @param {Object} shortcut - Shortcut definition
   * @returns {boolean} Whether event matches shortcut
   */
  function matchesShortcut(event, shortcut) {
    const keyMatches = event.key.toLowerCase() === shortcut.key.toLowerCase();
    const ctrlMatches = (event.ctrlKey || event.metaKey) === (shortcut.ctrl || false);
    const shiftMatches = event.shiftKey === (shortcut.shift || false);
    const altMatches = event.altKey === (shortcut.alt || false);

    return keyMatches && ctrlMatches && shiftMatches && altMatches;
  }

  /**
   * Handle keyboard event
   * @param {KeyboardEvent} event - Keyboard event
   */
  function handleKeyEvent(event) {
    // Find matching shortcut
    for (const [name, shortcut] of Object.entries(allShortcuts)) {
      if (matchesShortcut(event, shortcut)) {
        // Call all registered listeners for this shortcut
        if (listeners.has(name)) {
          const handlers = listeners.get(name);
          for (const { handler, options } of handlers) {
            if (options.preventDefault !== false) {
              event.preventDefault();
            }
            handler(event);
          }
        }
        break;
      }
    }
  }

  /**
   * Add global keyboard listener
   */
  function start() {
    document.addEventListener('keydown', handleKeyEvent);
  }

  /**
   * Remove global keyboard listener
   */
  function stop() {
    document.removeEventListener('keydown', handleKeyEvent);
  }

  /**
   * Get all registered shortcuts
   * @returns {Object} Shortcuts
   */
  function getShortcuts() {
    return allShortcuts;
  }

  /**
   * Get all shortcuts grouped by category
   * @returns {Object} Grouped shortcuts
   */
  function getGroupedShortcuts() {
    return {
      fileOperations: ['COPY', 'CUT', 'PASTE', 'DELETE', 'RENAME'],
      navigation: ['UPLOAD', 'REFRESH', 'SEARCH'],
      ui: ['ESCAPE', 'TAB', 'HELP'],
    };
  }

  return {
    on,
    off,
    handleKeyEvent,
    start,
    stop,
    getShortcuts,
    getGroupedShortcuts,
  };
}

/**
 * Create focus trap for modals
 * @param {HTMLElement} element - Modal element
 * @returns {Object} Focus trap manager
 */
export function createFocusTrap(element) {
  const focusableElements = [
    'button',
    '[href]',
    'input',
    'select',
    'textarea',
    '[tabindex]:not([tabindex="-1"])',
  ].join(',');

  let previouslyFocusedElement = null;

  function getFocusableElements() {
    return Array.from(element.querySelectorAll(focusableElements)).filter(
      (el) => !el.hasAttribute('disabled') && el.offsetParent !== null
    );
  }

  function activate() {
    previouslyFocusedElement = document.activeElement;

    const focusable = getFocusableElements();
    if (focusable.length === 0) return;

    // Focus first element
    focusable[0].focus();

    // Add tab listener
    element.addEventListener('keydown', handleTabKey);
  }

  function deactivate() {
    element.removeEventListener('keydown', handleTabKey);

    // Restore previous focus
    if (previouslyFocusedElement?.focus) {
      previouslyFocusedElement.focus();
    }
  }

  function handleTabKey(e) {
    if (e.key !== 'Tab') return;

    const focusable = getFocusableElements();
    const firstElement = focusable[0];
    const lastElement = focusable[focusable.length - 1];
    const activeElement = document.activeElement;

    // Shift + Tab on first element -> focus last
    if (e.shiftKey && activeElement === firstElement) {
      e.preventDefault();
      lastElement.focus();
      return;
    }

    // Tab on last element -> focus first
    if (!e.shiftKey && activeElement === lastElement) {
      e.preventDefault();
      firstElement.focus();
      return;
    }
  }

  return {
    activate,
    deactivate,
  };
}

/**
 * Create arrow key navigation
 * @param {Array<HTMLElement>} items - Items to navigate
 * @param {Object} options - Configuration
 * @returns {Object} Navigation manager
 */
export function createArrowNavigation(items, options = {}) {
  const {
    horizontal = false,
    wrap = true,
    startIndex = 0,
  } = options;

  let currentIndex = startIndex;

  function getCurrentItem() {
    return items[currentIndex];
  }

  function moveTo(index) {
    if (index < 0) {
      currentIndex = wrap ? items.length - 1 : 0;
    } else if (index >= items.length) {
      currentIndex = wrap ? 0 : items.length - 1;
    } else {
      currentIndex = index;
    }

    return getCurrentItem();
  }

  function moveNext() {
    return moveTo(currentIndex + 1);
  }

  function movePrevious() {
    return moveTo(currentIndex - 1);
  }

  function handleArrowKey(e) {
    if (!horizontal && (e.key === 'ArrowDown' || e.key === 'ArrowUp')) {
      e.preventDefault();
      const item =
        e.key === 'ArrowDown' ? moveNext() : movePrevious();
      item?.focus?.();
    } else if (horizontal && (e.key === 'ArrowLeft' || e.key === 'ArrowRight')) {
      e.preventDefault();
      const item =
        e.key === 'ArrowRight' ? moveNext() : movePrevious();
      item?.focus?.();
    }
  }

  return {
    getCurrentItem,
    moveTo,
    moveNext,
    movePrevious,
    handleArrowKey,
    getCurrentIndex: () => currentIndex,
  };
}

/**
 * Format keyboard shortcut for display
 * @param {Object} shortcut - Shortcut definition
 * @param {string} os - Operating system ('mac', 'windows', 'linux')
 * @returns {string} Formatted shortcut string
 */
export function formatShortcut(shortcut, os = 'windows') {
  const parts = [];

  if (shortcut.ctrl) {
    parts.push(os === 'mac' ? '⌘' : 'Ctrl');
  }
  if (shortcut.alt) {
    parts.push(os === 'mac' ? '⌥' : 'Alt');
  }
  if (shortcut.shift) {
    parts.push(os === 'mac' ? '⇧' : 'Shift');
  }

  parts.push(shortcut.key.toUpperCase());

  return parts.join(os === 'mac' ? '' : '+');
}
