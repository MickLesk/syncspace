/**
 * Global Keyboard Shortcuts Handler for SyncSpace
 * 
 * Supported shortcuts:
 * - Ctrl+F: Focus search
 * - Ctrl+A: Select all files
 * - Delete: Delete selected files
 * - F2: Rename selected file
 * - Escape: Close modals/dialogs
 * - Ctrl+U: Open upload dialog
 * - Ctrl+D: Download selected files
 */

export class KeyboardShortcuts {
  constructor() {
    this.handlers = new Map();
    this.enabled = true;
  }

  /**
   * Register a keyboard shortcut handler
   * @param {string} key - Key combination (e.g., 'ctrl+f', 'delete', 'f2')
   * @param {Function} handler - Function to call when shortcut is triggered
   * @param {Object} options - Options like { preventDefault: true }
   */
  register(key, handler, options = {}) {
    this.handlers.set(key.toLowerCase(), { handler, options });
  }

  /**
   * Unregister a shortcut
   */
  unregister(key) {
    this.handlers.delete(key.toLowerCase());
  }

  /**
   * Handle keyboard event
   */
  handleKeyDown(event) {
    if (!this.enabled) return;

    // Don't intercept if user is typing in an input field
    const target = event.target;
    if (
      target.tagName === 'INPUT' ||
      target.tagName === 'TEXTAREA' ||
      target.isContentEditable
    ) {
      // Allow Escape to work even in input fields
      if (event.key !== 'Escape') return;
    }

    const key = this.normalizeKey(event);
    const binding = this.handlers.get(key);

    if (binding) {
      if (binding.options.preventDefault !== false) {
        event.preventDefault();
      }
      binding.handler(event);
    }
  }

  /**
   * Normalize key combination to string format
   */
  normalizeKey(event) {
    const parts = [];

    if (event.ctrlKey || event.metaKey) parts.push('ctrl');
    if (event.altKey) parts.push('alt');
    if (event.shiftKey) parts.push('shift');

    const key = event.key.toLowerCase();
    parts.push(key);

    return parts.join('+');
  }

  /**
   * Enable shortcuts
   */
  enable() {
    this.enabled = true;
  }

  /**
   * Disable shortcuts (useful when dialogs are open)
   */
  disable() {
    this.enabled = false;
  }

  /**
   * Initialize global event listener
   */
  init() {
    window.addEventListener('keydown', (e) => this.handleKeyDown(e));
  }

  /**
   * Get list of all registered shortcuts for help menu
   */
  getShortcuts() {
    return Array.from(this.handlers.entries()).map(([key, binding]) => ({
      key,
      description: binding.options.description || '',
    }));
  }
}

// Singleton instance
export const shortcuts = new KeyboardShortcuts();
