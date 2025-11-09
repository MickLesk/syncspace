/**
 * Fuzzy Search Utility for SyncSpace
 * Provides client-side search with highlighting and scoring
 */

import Fuse from "fuse.js";

/**
 * Initialize a Fuse instance for file search
 * @param {Array} items - Items to search through
 * @param {Object} options - Fuse configuration
 * @returns {Fuse} Fuse instance
 */
export function createFileFuse(items, options = {}) {
  const defaultOptions = {
    keys: [
      { name: "name", weight: 10 }, // File names are most important
      { name: "file_path", weight: 5 },
      { name: "mime_type", weight: 1 },
    ],
    threshold: 0.3,
    minMatchCharLength: 2,
    ignoreLocation: true,
    useExtendedSearch: true,
    includeScore: true,
    includeMatches: true,
    isCaseSensitive: false,
    ...options,
  };

  return new Fuse(items, defaultOptions);
}

/**
 * Search files with fuzzy matching
 * @param {Fuse} fuse - Fuse instance
 * @param {string} query - Search query
 * @returns {Array} Search results with metadata
 */
export function fuzzySearch(fuse, query) {
  if (!query.trim()) {
    return [];
  }

  const results = fuse.search(query);

  return results.map((result) => ({
    ...result.item,
    score: result.score,
    matches: result.matches || [],
    highlightedName: highlightMatches(result.item.name, result.matches),
  }));
}

/**
 * Highlight matched text in a string
 * @param {string} text - Text to highlight
 * @param {Array} matches - Match information from Fuse
 * @returns {string} HTML with highlighting
 */
export function highlightMatches(text, matches = []) {
  if (!matches || matches.length === 0) {
    return text;
  }

  // Find matches for the text field
  const textMatch = matches.find((m) => m.key === "name");
  if (!textMatch || !textMatch.indices) {
    return text;
  }

  let highlighted = "";
  let lastIndex = 0;

  for (const [start, end] of textMatch.indices) {
    highlighted += text.substring(lastIndex, start);
    highlighted += `<mark class="font-bold bg-yellow-200 dark:bg-yellow-600">${text.substring(start, end + 1)}</mark>`;
    lastIndex = end + 1;
  }

  highlighted += text.substring(lastIndex);
  return highlighted;
}

/**
 * Get search summary
 * @param {Array} results - Search results
 * @param {number} total - Total items
 * @returns {Object} Summary statistics
 */
export function getSearchSummary(results, total) {
  return {
    resultCount: results.length,
    total: total,
    percentage: Math.round((results.length / total) * 100),
    isEmpty: results.length === 0,
  };
}

/**
 * Filter search results by type
 * @param {Array} results - Search results
 * @param {string} type - "files" | "folders" | "all"
 * @returns {Array} Filtered results
 */
export function filterResultsByType(results, type) {
  if (type === "all") return results;
  if (type === "files") return results.filter((r) => !r.is_directory);
  if (type === "folders") return results.filter((r) => r.is_directory);
  return results;
}

/**
 * Sort search results by relevance or other criteria
 * @param {Array} results - Search results
 * @param {string} sortBy - "relevance" | "name" | "modified" | "size"
 * @returns {Array} Sorted results
 */
export function sortSearchResults(results, sortBy = "relevance") {
  const sorted = [...results];

  switch (sortBy) {
    case "relevance":
      // Lower score = better match in Fuse
      return sorted.sort((a, b) => (a.score || 1) - (b.score || 1));
    case "name":
      return sorted.sort((a, b) => a.name.localeCompare(b.name));
    case "modified":
      return sorted.sort(
        (a, b) =>
          new Date(b.modified_at || 0) - new Date(a.modified_at || 0)
      );
    case "size":
      return sorted.sort((a, b) => (b.size_bytes || 0) - (a.size_bytes || 0));
    default:
      return sorted;
  }
}

/**
 * Debounced search function
 * @param {Function} searchFn - Search function to debounce
 * @param {number} delay - Delay in ms
 * @returns {Function} Debounced function
 */
export function debounceSearch(searchFn, delay = 300) {
  let timeoutId;
  return function debounced(...args) {
    clearTimeout(timeoutId);
    return new Promise((resolve) => {
      timeoutId = setTimeout(() => {
        resolve(searchFn(...args));
      }, delay);
    });
  };
}
