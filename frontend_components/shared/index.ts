// Shared utilities and types for component library

export type ButtonVariant = 'primary' | 'secondary' | 'danger' | 'success' | 'warning' | 'ghost' | 'outline';
export type ButtonSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl';
export type BadgeVariant = 'primary' | 'secondary' | 'success' | 'danger' | 'warning' | 'info';
export type ToastType = 'success' | 'error' | 'warning' | 'info';
export type ToastPosition = 'top-left' | 'top-center' | 'top-right' | 'bottom-left' | 'bottom-center' | 'bottom-right';

// Color utilities
export const colorMap = {
  primary: 'from-blue-500 to-blue-600',
  secondary: 'from-gray-500 to-gray-600',
  danger: 'from-red-500 to-red-600',
  success: 'from-green-500 to-green-600',
  warning: 'from-yellow-500 to-yellow-600',
  info: 'from-cyan-500 to-cyan-600',
};

export const colorBorder = {
  primary: 'border-blue-500',
  secondary: 'border-gray-500',
  danger: 'border-red-500',
  success: 'border-green-500',
  warning: 'border-yellow-500',
  info: 'border-cyan-500',
};

export const colorHover = {
  primary: 'hover:bg-blue-600',
  secondary: 'hover:bg-gray-600',
  danger: 'hover:bg-red-600',
  success: 'hover:bg-green-600',
  warning: 'hover:bg-yellow-600',
  info: 'hover:bg-cyan-600',
};

export const colorText = {
  primary: 'text-blue-600',
  secondary: 'text-gray-600',
  danger: 'text-red-600',
  success: 'text-green-600',
  warning: 'text-yellow-600',
  info: 'text-cyan-600',
};

export const sizeMap = {
  xs: 'px-2 py-1 text-xs',
  sm: 'px-3 py-2 text-sm',
  md: 'px-4 py-2 text-base',
  lg: 'px-6 py-3 text-lg',
  xl: 'px-8 py-4 text-xl',
};

// Animation utilities
export const transitions = {
  fast: 'transition-all duration-150',
  normal: 'transition-all duration-300',
  slow: 'transition-all duration-500',
};
