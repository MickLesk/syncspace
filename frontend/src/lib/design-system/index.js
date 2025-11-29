/**
 * SyncSpace Design System v3
 * 
 * Central export for all design system utilities, tokens, and helpers.
 * Import this in components to access the entire design system.
 */

// Design Tokens
export { default as tokens } from './tokens.js';
export {
  colors,
  gradients,
  spacing,
  borderRadius,
  shadows,
  typography,
  animations,
  transitions,
  zIndex,
  breakpoints,
  iconSizes,
  componentDefaults,
} from './tokens.js';

// Utility Functions & Class Generators
export { default as utils } from './utils.js';
export {
  buttonClasses,
  cardClasses,
  inputClasses,
  badgeClasses,
  alertClasses,
  cn,
  getFileIcon,
  formatFileSize,
  formatDate,
  staggerDelay,
  animationClasses,
  isDarkMode,
  getCurrentTheme,
} from './utils.js';
