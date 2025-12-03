/**
 * Command Palette Store
 * Manages command palette state, action registry, and keyboard shortcuts
 */

import { writable, derived, get } from "svelte/store";
import { currentView } from "./ui";
import { authToken } from "./auth";

// Command Palette visibility state
export const commandPaletteOpen = writable(false);

// Action registry
const createActionRegistry = () => {
  const { subscribe, set, update } = writable([]);

  return {
    subscribe,
    set,
    update,
    register: (action) => {
      update((actions) => {
        // Remove existing action with same id
        const filtered = actions.filter((a) => a.id !== action.id);
        return [...filtered, action];
      });
    },
    unregister: (actionId) => {
      update((actions) => actions.filter((a) => a.id !== actionId));
    },
    clear: () => set([]),
  };
};

export const actionRegistry = createActionRegistry();

// Recent commands history (last 10 commands)
export const recentCommands = writable([]);

// Command categories
export const commandCategories = {
  file: { name: "file", icon: "bi-file-earmark", color: "blue" },
  navigation: { name: "navigation", icon: "bi-compass", color: "green" },
  search: { name: "search", icon: "bi-search", color: "purple" },
  settings: { name: "settings", icon: "bi-gear", color: "gray" },
  view: { name: "view", icon: "bi-eye", color: "indigo" },
  help: { name: "help", icon: "bi-question-circle", color: "orange" },
};

// Filtered actions based on search query
export const filteredActions = derived(
  [actionRegistry, commandPaletteOpen],
  ([$actions, $isOpen]) => {
    if (!$isOpen) return [];
    return $actions.filter((action) => {
      // Filter by context (e.g., only show file actions if in files view)
      if (action.context && action.context !== get(currentView)) {
        return false;
      }
      // Filter by auth requirement
      if (action.requiresAuth && !get(authToken)) {
        return false;
      }
      return true;
    });
  }
);

/**
 * Register a command action
 * @param {Object} action - Action configuration
 * @param {string} action.id - Unique action ID
 * @param {string} action.label - Display label (i18n key or string)
 * @param {string} action.description - Short description
 * @param {string} action.category - Category (file, navigation, search, etc.)
 * @param {string} action.icon - Bootstrap icon class
 * @param {string[]} action.shortcuts - Keyboard shortcuts (e.g., ['Ctrl+S', 'Cmd+S'])
 * @param {Function} action.handler - Action handler function
 * @param {string} action.context - Context where action is available (optional)
 * @param {boolean} action.requiresAuth - Requires authentication (optional)
 */
export function registerCommand(action) {
  actionRegistry.register({
    id: action.id,
    label: action.label,
    description: action.description || "",
    category: action.category || "general",
    icon: action.icon || "bi-circle",
    shortcuts: action.shortcuts || [],
    handler: action.handler,
    context: action.context || null,
    requiresAuth: action.requiresAuth !== false,
    keywords: action.keywords || [],
  });
}

/**
 * Execute a command action
 */
export function executeCommand(actionId) {
  const actions = get(actionRegistry);
  const action = actions.find((a) => a.id === actionId);

  if (action && action.handler) {
    // Add to recent commands
    recentCommands.update((recent) => {
      const filtered = recent.filter((id) => id !== actionId);
      return [actionId, ...filtered].slice(0, 10);
    });

    // Execute handler
    action.handler();

    // Close palette
    commandPaletteOpen.set(false);
  }
}

/**
 * Fuzzy search actions
 */
export function searchActions(query) {
  if (!query || query.trim() === "") {
    return get(filteredActions);
  }

  const lowerQuery = query.toLowerCase();
  const actions = get(filteredActions);

  return actions
    .map((action) => {
      let score = 0;
      const labelLower = action.label.toLowerCase();
      const descriptionLower = (action.description || "").toLowerCase();
      const keywordsLower = (action.keywords || []).join(" ").toLowerCase();

      // Exact match bonus
      if (labelLower === lowerQuery) score += 100;
      if (labelLower.startsWith(lowerQuery)) score += 50;
      if (labelLower.includes(lowerQuery)) score += 25;

      // Description match
      if (descriptionLower.includes(lowerQuery)) score += 10;

      // Keywords match
      if (keywordsLower.includes(lowerQuery)) score += 15;

      // Fuzzy match
      const fuzzyScore = fuzzyMatch(lowerQuery, labelLower);
      score += fuzzyScore;

      return { action, score };
    })
    .filter(({ score }) => score > 0)
    .sort((a, b) => b.score - a.score)
    .map(({ action }) => action);
}

/**
 * Simple fuzzy matching algorithm
 */
function fuzzyMatch(query, text) {
  let score = 0;
  let queryIndex = 0;

  for (let i = 0; i < text.length && queryIndex < query.length; i++) {
    if (text[i] === query[queryIndex]) {
      score += 2;
      queryIndex++;
    }
  }

  // Bonus for matching all characters
  if (queryIndex === query.length) {
    score += 5;
  }

  return score;
}

/**
 * Toggle command palette
 */
export function toggleCommandPalette() {
  commandPaletteOpen.update((open) => !open);
}

/**
 * Open command palette
 */
export function openCommandPalette() {
  commandPaletteOpen.set(true);
}

/**
 * Close command palette
 */
export function closeCommandPalette() {
  commandPaletteOpen.set(false);
}
