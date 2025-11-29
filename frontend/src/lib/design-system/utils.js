/**
 * SyncSpace Design System - Utility Classes & Helpers
 * 
 * Reusable CSS class generators and utility functions
 */

import tokens from './tokens.js';

// =============================================================================
// CSS CLASS GENERATORS
// =============================================================================

/**
 * Generate consistent button classes based on variant and size
 */
export function buttonClasses(variant = 'primary', size = 'md', options = {}) {
  const { fullWidth = false, disabled = false, loading = false } = options;
  
  const base = [
    'inline-flex items-center justify-center gap-2',
    'font-semibold rounded-xl',
    'transition-all duration-200',
    'focus:outline-none focus:ring-2 focus:ring-offset-2',
    'disabled:opacity-50 disabled:cursor-not-allowed',
  ];
  
  const variants = {
    primary: [
      'bg-primary-500 hover:bg-primary-600',
      'text-white',
      'shadow-md hover:shadow-lg',
      'focus:ring-primary-500',
      'dark:shadow-primary-500/20',
    ],
    secondary: [
      'bg-gray-200 dark:bg-gray-700',
      'hover:bg-gray-300 dark:hover:bg-gray-600',
      'text-gray-900 dark:text-white',
      'border border-gray-300 dark:border-gray-600',
      'focus:ring-gray-400',
    ],
    ghost: [
      'bg-transparent',
      'hover:bg-gray-100 dark:hover:bg-gray-800',
      'text-gray-700 dark:text-gray-300',
      'focus:ring-gray-400',
    ],
    danger: [
      'bg-red-500 hover:bg-red-600',
      'dark:bg-red-600 dark:hover:bg-red-700',
      'text-white',
      'shadow-md hover:shadow-lg',
      'focus:ring-red-500',
    ],
    success: [
      'bg-green-500 hover:bg-green-600',
      'dark:bg-green-600 dark:hover:bg-green-700',
      'text-white',
      'shadow-md hover:shadow-lg',
      'focus:ring-green-500',
    ],
    warning: [
      'bg-amber-500 hover:bg-amber-600',
      'dark:bg-amber-600 dark:hover:bg-amber-700',
      'text-white',
      'shadow-md hover:shadow-lg',
      'focus:ring-amber-500',
    ],
    gradient: [
      'gradient-bg-primary',
      'text-white',
      'shadow-lg hover:shadow-xl',
      'hover:scale-105',
      'focus:ring-primary-500',
    ],
    outline: [
      'bg-transparent',
      'border-2 border-primary-500',
      'text-primary-600 dark:text-primary-400',
      'hover:bg-primary-50 dark:hover:bg-primary-900/20',
      'focus:ring-primary-500',
    ],
  };
  
  const sizes = {
    xs: 'px-2.5 py-1 text-xs',
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2.5 text-sm',
    lg: 'px-6 py-3 text-base',
    xl: 'px-8 py-4 text-lg',
  };
  
  return [
    ...base,
    ...(variants[variant] || variants.primary),
    sizes[size] || sizes.md,
    fullWidth ? 'w-full' : '',
    (disabled || loading) ? 'opacity-50 cursor-not-allowed' : '',
  ].filter(Boolean).join(' ');
}

/**
 * Generate consistent card classes based on variant
 */
export function cardClasses(variant = 'default', options = {}) {
  const { hoverable = false, clickable = false, padding = 'normal' } = options;
  
  const base = 'rounded-2xl transition-all duration-200';
  
  const variants = {
    default: [
      'bg-white dark:bg-gray-800',
      'border border-gray-200 dark:border-gray-700',
      'shadow-md hover:shadow-lg',
      'dark:shadow-gray-900/50',
    ],
    glass: [
      'bg-white/95 dark:bg-gray-800/90',
      'backdrop-blur-xl',
      'border border-gray-200/80 dark:border-gray-700/50',
      'shadow-lg dark:shadow-gray-900/30',
    ],
    gradient: [
      'gradient-bg-primary',
      'text-white',
      'shadow-lg hover:shadow-xl',
    ],
    elevated: [
      'bg-white dark:bg-gray-800',
      'shadow-xl hover:shadow-2xl',
      'dark:shadow-gray-900/80',
    ],
    outline: [
      'bg-transparent',
      'border-2 border-gray-200 dark:border-gray-700',
    ],
    flat: [
      'bg-gray-50 dark:bg-gray-800/50',
      'border-0',
    ],
  };
  
  const paddings = {
    none: '',
    sm: 'p-4',
    normal: 'p-6',
    lg: 'p-8',
  };
  
  return [
    base,
    ...(variants[variant] || variants.default),
    paddings[padding],
    hoverable ? 'hover:scale-[1.02] hover:-translate-y-1' : '',
    clickable ? 'cursor-pointer' : '',
  ].filter(Boolean).join(' ');
}

/**
 * Generate consistent input classes
 */
export function inputClasses(variant = 'default', options = {}) {
  const { size = 'md', error = false, disabled = false } = options;
  
  const base = [
    'w-full rounded-xl',
    'bg-white dark:bg-gray-800',
    'border transition-all duration-200',
    'text-gray-900 dark:text-gray-100',
    'placeholder:text-gray-400 dark:placeholder:text-gray-500',
    'focus:outline-none focus:ring-2 focus:ring-offset-0',
  ];
  
  const variants = {
    default: [
      'border-gray-300 dark:border-gray-600',
      'hover:border-gray-400 dark:hover:border-gray-500',
      'focus:border-primary-500 focus:ring-primary-500/20',
    ],
    glass: [
      'bg-white/50 dark:bg-gray-800/50',
      'backdrop-blur-sm',
      'border-gray-200/50 dark:border-gray-700/50',
      'focus:border-primary-500 focus:ring-primary-500/20',
    ],
    error: [
      'border-red-500 dark:border-red-500',
      'focus:border-red-500 focus:ring-red-500/20',
    ],
  };
  
  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2.5 text-sm',
    lg: 'px-5 py-3 text-base',
  };
  
  return [
    ...base,
    ...(variants[error ? 'error' : variant] || variants.default),
    sizes[size] || sizes.md,
    disabled ? 'opacity-50 cursor-not-allowed bg-gray-100 dark:bg-gray-900' : '',
  ].filter(Boolean).join(' ');
}

/**
 * Generate badge classes
 */
export function badgeClasses(variant = 'default', size = 'md') {
  const base = 'inline-flex items-center gap-1 font-semibold rounded-full transition-colors';
  
  const variants = {
    default: 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300',
    primary: 'bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300',
    success: 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300',
    warning: 'bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300',
    error: 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300',
    info: 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300',
    // Glass variants
    glassDefault: 'bg-white/20 dark:bg-gray-800/40 backdrop-blur-sm border border-white/30 dark:border-gray-700/50',
    glassPrimary: 'bg-primary-500/20 dark:bg-primary-400/20 backdrop-blur-sm text-primary-700 dark:text-primary-300',
    glassSuccess: 'bg-green-500/20 dark:bg-green-400/20 backdrop-blur-sm text-green-700 dark:text-green-300',
    glassWarning: 'bg-amber-500/20 dark:bg-amber-400/20 backdrop-blur-sm text-amber-700 dark:text-amber-300',
    glassError: 'bg-red-500/20 dark:bg-red-400/20 backdrop-blur-sm text-red-700 dark:text-red-300',
  };
  
  const sizes = {
    sm: 'px-2 py-0.5 text-xs',
    md: 'px-2.5 py-1 text-xs',
    lg: 'px-3 py-1.5 text-sm',
  };
  
  return `${base} ${variants[variant] || variants.default} ${sizes[size] || sizes.md}`;
}

/**
 * Generate alert/notification classes
 */
export function alertClasses(variant = 'info') {
  const base = 'flex items-start gap-3 p-4 rounded-xl border';
  
  const variants = {
    info: 'bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800 text-blue-800 dark:text-blue-200',
    success: 'bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800 text-green-800 dark:text-green-200',
    warning: 'bg-amber-50 dark:bg-amber-900/20 border-amber-200 dark:border-amber-800 text-amber-800 dark:text-amber-200',
    error: 'bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800 text-red-800 dark:text-red-200',
  };
  
  return `${base} ${variants[variant] || variants.info}`;
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/**
 * Merge class names, filtering out falsy values
 */
export function cn(...classes) {
  return classes.filter(Boolean).join(' ');
}

/**
 * Get icon class based on file type
 */
export function getFileIcon(filename, isDirectory = false) {
  if (isDirectory) return 'folder-fill';
  
  const ext = filename?.split('.').pop()?.toLowerCase() || '';
  
  const iconMap = {
    // Images
    jpg: 'file-image', jpeg: 'file-image', png: 'file-image', gif: 'file-image',
    webp: 'file-image', svg: 'file-image', bmp: 'file-image', ico: 'file-image',
    
    // Documents
    pdf: 'file-pdf', doc: 'file-word', docx: 'file-word',
    xls: 'file-excel', xlsx: 'file-excel', csv: 'file-excel',
    ppt: 'file-ppt', pptx: 'file-ppt',
    txt: 'file-text', md: 'file-text', rtf: 'file-text',
    
    // Code
    js: 'file-code', ts: 'file-code', jsx: 'file-code', tsx: 'file-code',
    html: 'file-code', css: 'file-code', scss: 'file-code', sass: 'file-code',
    json: 'file-code', xml: 'file-code', yaml: 'file-code', yml: 'file-code',
    py: 'file-code', rb: 'file-code', php: 'file-code', java: 'file-code',
    c: 'file-code', cpp: 'file-code', h: 'file-code', rs: 'file-code',
    go: 'file-code', swift: 'file-code', kt: 'file-code',
    sh: 'terminal', bash: 'terminal', zsh: 'terminal',
    sql: 'database',
    
    // Media
    mp3: 'file-music', wav: 'file-music', flac: 'file-music', ogg: 'file-music',
    mp4: 'file-play', mov: 'file-play', avi: 'file-play', mkv: 'file-play',
    webm: 'file-play',
    
    // Archives
    zip: 'file-zip', rar: 'file-zip', '7z': 'file-zip', tar: 'file-zip',
    gz: 'file-zip', bz2: 'file-zip',
    
    // Other
    exe: 'file-binary', dll: 'file-binary', bin: 'file-binary',
    iso: 'disc', dmg: 'disc',
  };
  
  return iconMap[ext] || 'file-earmark';
}

/**
 * Format file size in human-readable format
 */
export function formatFileSize(bytes) {
  if (bytes === 0 || bytes === undefined || bytes === null) return '0 B';
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${units[i]}`;
}

/**
 * Format date in relative or absolute format
 */
export function formatDate(date, options = {}) {
  const { relative = false, format = 'short' } = options;
  
  if (!date) return '';
  
  const d = new Date(date);
  const now = new Date();
  const diff = now.getTime() - d.getTime();
  
  if (relative) {
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    
    if (seconds < 60) return 'Just now';
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    if (days < 7) return `${days}d ago`;
  }
  
  if (format === 'short') {
    return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  }
  
  return d.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

// =============================================================================
// ANIMATION HELPERS
// =============================================================================

/**
 * Get animation delay for staggered animations
 */
export function staggerDelay(index, baseDelay = 50) {
  return `animation-delay: ${index * baseDelay}ms`;
}

/**
 * Common animation classes
 */
export const animationClasses = {
  fadeIn: 'animate-fade-in',
  fadeOut: 'animate-fade-out',
  slideUp: 'animate-slide-up',
  slideDown: 'animate-slide-down',
  scaleIn: 'animate-scale-in',
  pulse: 'animate-pulse',
  spin: 'animate-spin',
  bounce: 'animate-bounce',
  shake: 'animate-shake',
};

// =============================================================================
// THEME HELPERS
// =============================================================================

/**
 * Check if dark mode is active
 */
export function isDarkMode() {
  if (typeof document === 'undefined') return false;
  return document.documentElement.classList.contains('dark');
}

/**
 * Get current theme
 */
export function getCurrentTheme() {
  if (typeof document === 'undefined') return 'light';
  return document.documentElement.classList.contains('dark') ? 'dark' : 'light';
}

// =============================================================================
// EXPORTS
// =============================================================================

export default {
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
};
