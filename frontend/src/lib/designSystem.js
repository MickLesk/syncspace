/**
 * SyncSpace Design System
 * Central design tokens and utilities for consistent UI
 */

// ==================== SPACING ====================
export const spacing = {
  xs: '0.5rem',    // 8px
  sm: '1rem',      // 16px
  md: '1.5rem',    // 24px
  lg: '2rem',      // 32px
  xl: '3rem',      // 48px
  '2xl': '4rem',   // 64px
  '3xl': '6rem',   // 96px
};

// ==================== COLORS ====================
export const colors = {
  // Brand colors
  primary: '#667eea',
  secondary: '#764ba2',
  accent: '#f093fb',
  
  // Semantic colors
  success: '#22c55e',
  warning: '#eab308',
  error: '#ef4444',
  info: '#3b82f6',
  
  // Gradients
  gradientStart: '#667eea',
  gradientMid: '#764ba2',
  gradientEnd: '#f093fb',
  
  // Glass effect
  glass: {
    light: 'rgba(255, 255, 255, 0.7)',
    dark: 'rgba(0, 0, 0, 0.3)',
    border: 'rgba(255, 255, 255, 0.18)',
  },
};

// ==================== TYPOGRAPHY ====================
export const typography = {
  fontFamily: {
    sans: 'Inter var, Inter, ui-sans-serif, system-ui, sans-serif',
    mono: 'ui-monospace, Cascadia Code, Source Code Pro, monospace',
  },
  
  fontSize: {
    '2xs': ['0.625rem', { lineHeight: '0.75rem' }],
    xs: ['0.75rem', { lineHeight: '1rem' }],
    sm: ['0.875rem', { lineHeight: '1.25rem' }],
    base: ['1rem', { lineHeight: '1.5rem' }],
    lg: ['1.125rem', { lineHeight: '1.75rem' }],
    xl: ['1.25rem', { lineHeight: '1.75rem' }],
    '2xl': ['1.5rem', { lineHeight: '2rem' }],
    '3xl': ['2rem', { lineHeight: '2.25rem' }],
    '4xl': ['2.5rem', { lineHeight: '2.75rem' }],
    '5xl': ['3rem', { lineHeight: '3.25rem' }],
  },
  
  fontWeight: {
    light: 300,
    normal: 400,
    medium: 500,
    semibold: 600,
    bold: 700,
    extrabold: 800,
  },
};

// ==================== BORDER RADIUS ====================
export const borderRadius = {
  none: '0',
  sm: '0.25rem',    // 4px
  md: '0.5rem',     // 8px
  lg: '0.75rem',    // 12px
  xl: '1rem',       // 16px
  '2xl': '1.5rem',  // 24px
  '3xl': '2rem',    // 32px
  '4xl': '2.5rem',  // 40px
  full: '9999px',
};

// ==================== SHADOWS ====================
export const shadows = {
  sm: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
  base: '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)',
  md: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
  lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
  xl: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)',
  '2xl': '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
  glass: '0 8px 32px 0 rgba(31, 38, 135, 0.15)',
  glassLg: '0 16px 48px 0 rgba(31, 38, 135, 0.2)',
};

// ==================== TRANSITIONS ====================
export const transitions = {
  fast: '150ms',
  normal: '200ms',
  slow: '300ms',
  slower: '500ms',
  
  // Easing functions
  easing: {
    default: 'cubic-bezier(0.4, 0, 0.2, 1)',
    in: 'cubic-bezier(0.4, 0, 1, 1)',
    out: 'cubic-bezier(0, 0, 0.2, 1)',
    inOut: 'cubic-bezier(0.4, 0, 0.2, 1)',
  },
};

// ==================== Z-INDEX ====================
export const zIndex = {
  dropdown: 1000,
  sticky: 1020,
  fixed: 1030,
  modalBackdrop: 1040,
  modal: 1050,
  popover: 1060,
  tooltip: 1070,
};

// ==================== BREAKPOINTS ====================
export const breakpoints = {
  sm: '640px',
  md: '768px',
  lg: '1024px',
  xl: '1280px',
  '2xl': '1536px',
};

// ==================== COMPONENT SIZES ====================
export const componentSizes = {
  button: {
    xs: 'px-2 py-1 text-xs',
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg',
    xl: 'px-8 py-4 text-xl',
  },
  
  input: {
    sm: 'px-3 py-1.5 text-sm h-9',
    md: 'px-4 py-2 text-base h-10',
    lg: 'px-5 py-3 text-lg h-12',
  },
  
  card: {
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8',
  },
  
  avatar: {
    xs: 'w-6 h-6',
    sm: 'w-8 h-8',
    md: 'w-10 h-10',
    lg: 'w-12 h-12',
    xl: 'w-16 h-16',
    '2xl': 'w-24 h-24',
  },
};

// ==================== UTILITIES ====================

/**
 * Generate Tailwind class string from object
 * @param {Object} classes - Conditional classes { 'class': condition }
 * @returns {string} - Space-separated class string
 */
export function classNames(...classes) {
  return classes.filter(Boolean).join(' ');
}

/**
 * Format file size to human-readable format
 * @param {number} bytes - File size in bytes
 * @returns {string} - Formatted size (e.g., "1.5 MB")
 */
export function formatBytes(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
}

/**
 * Format date to relative time (e.g., "2 hours ago")
 * @param {Date|string} date - Date to format
 * @returns {string} - Relative time string
 */
export function formatRelativeTime(date) {
  const now = new Date();
  const then = new Date(date);
  const diff = now - then;
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);
  const weeks = Math.floor(days / 7);
  const months = Math.floor(days / 30);
  const years = Math.floor(days / 365);

  if (seconds < 60) return 'just now';
  if (minutes < 60) return `${minutes} ${minutes === 1 ? 'minute' : 'minutes'} ago`;
  if (hours < 24) return `${hours} ${hours === 1 ? 'hour' : 'hours'} ago`;
  if (days < 7) return `${days} ${days === 1 ? 'day' : 'days'} ago`;
  if (weeks < 4) return `${weeks} ${weeks === 1 ? 'week' : 'weeks'} ago`;
  if (months < 12) return `${months} ${months === 1 ? 'month' : 'months'} ago`;
  return `${years} ${years === 1 ? 'year' : 'years'} ago`;
}

/**
 * Truncate text with ellipsis
 * @param {string} text - Text to truncate
 * @param {number} maxLength - Maximum length
 * @returns {string} - Truncated text
 */
export function truncateText(text, maxLength = 50) {
  if (text.length <= maxLength) return text;
  return `${text.slice(0, maxLength)}...`;
}

/**
 * Get file icon class based on file extension
 * @param {string} filename - File name
 * @returns {string} - Bootstrap icon class
 */
export function getFileIcon(filename) {
  const ext = filename.split('.').pop()?.toLowerCase();
  
  const iconMap = {
    // Documents
    pdf: 'file-earmark-pdf',
    doc: 'file-earmark-word',
    docx: 'file-earmark-word',
    txt: 'file-earmark-text',
    
    // Spreadsheets
    xls: 'file-earmark-excel',
    xlsx: 'file-earmark-excel',
    csv: 'file-earmark-spreadsheet',
    
    // Presentations
    ppt: 'file-earmark-ppt',
    pptx: 'file-earmark-ppt',
    
    // Images
    jpg: 'file-earmark-image',
    jpeg: 'file-earmark-image',
    png: 'file-earmark-image',
    gif: 'file-earmark-image',
    svg: 'file-earmark-image',
    webp: 'file-earmark-image',
    
    // Videos
    mp4: 'file-earmark-play',
    mov: 'file-earmark-play',
    avi: 'file-earmark-play',
    mkv: 'file-earmark-play',
    
    // Audio
    mp3: 'file-earmark-music',
    wav: 'file-earmark-music',
    flac: 'file-earmark-music',
    
    // Archives
    zip: 'file-earmark-zip',
    rar: 'file-earmark-zip',
    '7z': 'file-earmark-zip',
    tar: 'file-earmark-zip',
    gz: 'file-earmark-zip',
    
    // Code
    js: 'file-earmark-code',
    ts: 'file-earmark-code',
    jsx: 'file-earmark-code',
    tsx: 'file-earmark-code',
    html: 'file-earmark-code',
    css: 'file-earmark-code',
    scss: 'file-earmark-code',
    json: 'file-earmark-code',
    xml: 'file-earmark-code',
    yaml: 'file-earmark-code',
    yml: 'file-earmark-code',
    py: 'file-earmark-code',
    java: 'file-earmark-code',
    cpp: 'file-earmark-code',
    c: 'file-earmark-code',
    rs: 'file-earmark-code',
    go: 'file-earmark-code',
    
    // Default
    default: 'file-earmark',
  };
  
  return iconMap[ext] || iconMap.default;
}

/**
 * Get color for file type
 * @param {string} filename - File name
 * @returns {string} - Tailwind color class
 */
export function getFileColor(filename) {
  const ext = filename.split('.').pop()?.toLowerCase();
  
  const colorMap = {
    // Documents - Blue
    pdf: 'text-red-500',
    doc: 'text-blue-500',
    docx: 'text-blue-500',
    txt: 'text-gray-500',
    
    // Spreadsheets - Green
    xls: 'text-green-500',
    xlsx: 'text-green-500',
    csv: 'text-green-500',
    
    // Presentations - Orange
    ppt: 'text-orange-500',
    pptx: 'text-orange-500',
    
    // Images - Purple
    jpg: 'text-purple-500',
    jpeg: 'text-purple-500',
    png: 'text-purple-500',
    gif: 'text-purple-500',
    svg: 'text-purple-500',
    webp: 'text-purple-500',
    
    // Videos - Pink
    mp4: 'text-pink-500',
    mov: 'text-pink-500',
    avi: 'text-pink-500',
    mkv: 'text-pink-500',
    
    // Audio - Indigo
    mp3: 'text-indigo-500',
    wav: 'text-indigo-500',
    flac: 'text-indigo-500',
    
    // Archives - Yellow
    zip: 'text-yellow-500',
    rar: 'text-yellow-500',
    '7z': 'text-yellow-500',
    tar: 'text-yellow-500',
    gz: 'text-yellow-500',
    
    // Code - Cyan
    js: 'text-cyan-500',
    ts: 'text-cyan-500',
    jsx: 'text-cyan-500',
    tsx: 'text-cyan-500',
    html: 'text-cyan-500',
    css: 'text-cyan-500',
    py: 'text-cyan-500',
    
    // Default
    default: 'text-gray-500',
  };
  
  return colorMap[ext] || colorMap.default;
}

/**
 * Accent background helper for file avatars
 * @param {string} filename
 * @returns {string} - Tailwind utility classes for gradients/colors
 */
export function getFileAccent(filename = '') {
  const ext = filename.split('.').pop()?.toLowerCase();

  const accentMap = {
    pdf: 'bg-gradient-to-br from-rose-500 to-orange-500 text-white',
    doc: 'bg-gradient-to-br from-sky-500 to-blue-600 text-white',
    docx: 'bg-gradient-to-br from-sky-500 to-blue-600 text-white',
    xls: 'bg-gradient-to-br from-emerald-500 to-lime-500 text-white',
    xlsx: 'bg-gradient-to-br from-emerald-500 to-lime-500 text-white',
    ppt: 'bg-gradient-to-br from-amber-500 to-orange-500 text-white',
    pptx: 'bg-gradient-to-br from-amber-500 to-orange-500 text-white',
    jpg: 'bg-gradient-to-br from-fuchsia-500 to-purple-500 text-white',
    jpeg: 'bg-gradient-to-br from-fuchsia-500 to-purple-500 text-white',
    png: 'bg-gradient-to-br from-fuchsia-500 to-purple-500 text-white',
    svg: 'bg-gradient-to-br from-indigo-500 to-purple-500 text-white',
    gif: 'bg-gradient-to-br from-indigo-500 to-purple-500 text-white',
    mp4: 'bg-gradient-to-br from-purple-500 to-pink-500 text-white',
    mov: 'bg-gradient-to-br from-purple-500 to-pink-500 text-white',
    avi: 'bg-gradient-to-br from-purple-500 to-pink-500 text-white',
    zip: 'bg-gradient-to-br from-amber-500 to-yellow-500 text-slate-900',
    rar: 'bg-gradient-to-br from-amber-500 to-yellow-500 text-slate-900',
    '7z': 'bg-gradient-to-br from-amber-500 to-yellow-500 text-slate-900',
    tar: 'bg-gradient-to-br from-amber-500 to-yellow-500 text-slate-900',
    js: 'bg-gradient-to-br from-cyan-500 to-blue-500 text-slate-900',
    ts: 'bg-gradient-to-br from-cyan-500 to-blue-500 text-slate-900',
    jsx: 'bg-gradient-to-br from-cyan-500 to-blue-500 text-slate-900',
    tsx: 'bg-gradient-to-br from-cyan-500 to-blue-500 text-slate-900',
    html: 'bg-gradient-to-br from-amber-500 to-rose-500 text-slate-900',
    css: 'bg-gradient-to-br from-indigo-500 to-sky-500 text-white',
    json: 'bg-gradient-to-br from-slate-500 to-slate-700 text-white',
  };

  return (
    accentMap[ext] ||
    'bg-gradient-to-br from-slate-100 to-slate-200 text-slate-700 dark:from-slate-800/80 dark:to-slate-900/60 dark:text-slate-50'
  );
}

/**
 * Debounce function
 * @param {Function} func - Function to debounce
 * @param {number} wait - Wait time in ms
 * @returns {Function} - Debounced function
 */
export function debounce(func, wait = 300) {
  let timeout;
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}

/**
 * Throttle function
 * @param {Function} func - Function to throttle
 * @param {number} limit - Time limit in ms
 * @returns {Function} - Throttled function
 */
export function throttle(func, limit = 100) {
  let inThrottle;
  return function(...args) {
    if (!inThrottle) {
      func.apply(this, args);
      inThrottle = true;
      setTimeout(() => inThrottle = false, limit);
    }
  };
}

export default {
  spacing,
  colors,
  typography,
  borderRadius,
  shadows,
  transitions,
  zIndex,
  breakpoints,
  componentSizes,
  classNames,
  formatBytes,
  formatRelativeTime,
  truncateText,
  getFileIcon,
  getFileColor,
  debounce,
  throttle,
};
