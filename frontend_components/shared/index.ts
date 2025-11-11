// Material Design 3 Inspired Design System - Clean, Minimal, Expressive

// Button Types - Material 3 System
export type ButtonVariant = 'filled' | 'outlined' | 'text' | 'elevated' | 'tonal';
export type ButtonSize = 'sm' | 'md' | 'lg';
export type ButtonColor = 'primary' | 'secondary' | 'tertiary' | 'error' | 'success';

// Legacy types for backwards compatibility
export type BadgeVariant = 'primary' | 'secondary' | 'success' | 'danger' | 'warning' | 'info';
export type ToastType = 'success' | 'error' | 'warning' | 'info';
export type ToastPosition = 'top-left' | 'top-center' | 'top-right' | 'bottom-left' | 'bottom-center' | 'bottom-right';

// Elevation Levels (Material 3)
export type ElevationLevel = 0 | 1 | 2 | 3 | 4 | 5;

// Material 3 Surface Colors - Semantic layers
export const surface = {
  base: 'bg-white dark:bg-gray-950',
  container: 'bg-gray-50 dark:bg-gray-900',
  containerLow: 'bg-gray-100 dark:bg-gray-900/50',
  containerHigh: 'bg-gray-100 dark:bg-gray-800',
  containerHighest: 'bg-gray-200 dark:bg-gray-800/80',
  variant: 'bg-gray-100 dark:bg-gray-800',
  inverse: 'bg-gray-900 dark:bg-gray-100',
};

// State Layers - Material 3 opacity system
export const stateLayers = {
  hover: 'hover:bg-black/[0.08] dark:hover:bg-white/[0.08]',
  focus: 'focus:bg-black/[0.12] dark:focus:bg-white/[0.12]',
  pressed: 'active:bg-black/[0.12] dark:active:bg-white/[0.12]',
  dragged: 'bg-black/[0.16] dark:bg-white/[0.16]',
};

// Elevation System - Subtle shadows (Material 3)
export const elevation = {
  0: '', // No shadow
  1: 'shadow-[0_1px_2px_0_rgba(0,0,0,0.05)]',
  2: 'shadow-[0_1px_3px_0_rgba(0,0,0,0.1),0_1px_2px_-1px_rgba(0,0,0,0.1)]',
  3: 'shadow-[0_4px_6px_-1px_rgba(0,0,0,0.1),0_2px_4px_-2px_rgba(0,0,0,0.1)]',
  4: 'shadow-[0_10px_15px_-3px_rgba(0,0,0,0.1),0_4px_6px_-4px_rgba(0,0,0,0.1)]',
  5: 'shadow-[0_20px_25px_-5px_rgba(0,0,0,0.1),0_8px_10px_-6px_rgba(0,0,0,0.1)]',
};

// Color System - Clean semantic colors
export const colors = {
  primary: {
    default: 'bg-blue-600 dark:bg-blue-500',
    on: 'text-white',
    container: 'bg-blue-100 dark:bg-blue-950',
    onContainer: 'text-blue-900 dark:text-blue-100',
    hover: 'hover:bg-blue-700 dark:hover:bg-blue-600',
    focus: 'focus-visible:ring-2 focus-visible:ring-blue-600 focus-visible:ring-offset-2',
  },
  secondary: {
    default: 'bg-gray-600 dark:bg-gray-500',
    on: 'text-white',
    container: 'bg-gray-100 dark:bg-gray-900',
    onContainer: 'text-gray-900 dark:text-gray-100',
    hover: 'hover:bg-gray-700 dark:hover:bg-gray-600',
    focus: 'focus-visible:ring-2 focus-visible:ring-gray-600 focus-visible:ring-offset-2',
  },
  tertiary: {
    default: 'bg-purple-600 dark:bg-purple-500',
    on: 'text-white',
    container: 'bg-purple-100 dark:bg-purple-950',
    onContainer: 'text-purple-900 dark:text-purple-100',
    hover: 'hover:bg-purple-700 dark:hover:bg-purple-600',
    focus: 'focus-visible:ring-2 focus-visible:ring-purple-600 focus-visible:ring-offset-2',
  },
  error: {
    default: 'bg-red-600 dark:bg-red-500',
    on: 'text-white',
    container: 'bg-red-100 dark:bg-red-950',
    onContainer: 'text-red-900 dark:text-red-100',
    hover: 'hover:bg-red-700 dark:hover:bg-red-600',
    focus: 'focus-visible:ring-2 focus-visible:ring-red-600 focus-visible:ring-offset-2',
  },
  success: {
    default: 'bg-green-600 dark:bg-green-500',
    on: 'text-white',
    container: 'bg-green-100 dark:bg-green-950',
    onContainer: 'text-green-900 dark:text-green-100',
    hover: 'hover:bg-green-700 dark:hover:bg-green-600',
    focus: 'focus-visible:ring-2 focus-visible:ring-green-600 focus-visible:ring-offset-2',
  },
};

// Outline Styles
export const outline = {
  default: 'border border-gray-300 dark:border-gray-700',
  focus: 'focus-visible:border-blue-600 dark:focus-visible:border-blue-500',
  error: 'border-red-600 dark:border-red-500',
};

// Shape System - Material 3 rounded corners
export const shape = {
  none: 'rounded-none',
  xs: 'rounded-sm', // 2px
  sm: 'rounded', // 4px
  md: 'rounded-md', // 6px
  lg: 'rounded-lg', // 8px
  xl: 'rounded-xl', // 12px
  '2xl': 'rounded-2xl', // 16px
  '3xl': 'rounded-3xl', // 24px
  full: 'rounded-full',
};

// 🆕 M3 EXPRESSIVE: Extended Shape System
export const shapeExpressive = {
  // Extra-rounded corners (M3 Expressive Feature!)
  'extra-large': 'rounded-[28px]',
  'extra-extra-large': 'rounded-[32px]',
  'extra-extra-extra-large': 'rounded-[40px]',
  
  // Asymmetric corners for visual interest
  'top-rounded': 'rounded-t-xl rounded-b-none',
  'bottom-rounded': 'rounded-b-xl rounded-t-none',
  'left-rounded': 'rounded-l-xl rounded-r-none',
  'right-rounded': 'rounded-r-xl rounded-l-none',
  
  // Mix of rounded styles (M3 Expressive tactic)
  'top-extra-rounded': 'rounded-t-[28px] rounded-b-none',
  'mixed-rounded': 'rounded-tl-3xl rounded-br-3xl rounded-tr-md rounded-bl-md',
  
  // Squircle-style (iOS-inspired smooth curves)
  'squircle-sm': 'rounded-[16px]',
  'squircle-md': 'rounded-[20px]',
  'squircle-lg': 'rounded-[24px]',
  'squircle-xl': 'rounded-[32px]',
};

// 🆕 M3 EXPRESSIVE: Shape Morphing Animations
export const shapeMorph = {
  // Standard Material motion
  smooth: 'transition-all duration-400 ease-[cubic-bezier(0.4,0,0.2,1)]',
  
  // Spring physics for shape changes
  spring: 'transition-all duration-400 ease-[cubic-bezier(0.34,1.56,0.64,1)]',
  
  // Gentle morph
  gentle: 'transition-all duration-500 ease-[cubic-bezier(0.25,0.1,0.25,1)]',
  
  // Fast snap
  snap: 'transition-all duration-200 ease-[cubic-bezier(0.4,0,1,1)]',
};

// Size System - Material 3 touch targets (min 48px)
export const sizes = {
  sm: 'h-9 px-3 text-sm', // 36px height
  md: 'h-10 px-4 text-sm', // 40px height
  lg: 'h-12 px-6 text-base', // 48px height - Touch target
};

// Typography Scale
export const typography = {
  display: {
    large: 'text-5xl font-normal tracking-tight',
    medium: 'text-4xl font-normal',
    small: 'text-3xl font-normal',
  },
  headline: {
    large: 'text-2xl font-normal',
    medium: 'text-xl font-normal',
    small: 'text-lg font-normal',
  },
  title: {
    large: 'text-lg font-medium',
    medium: 'text-base font-medium',
    small: 'text-sm font-medium',
  },
  body: {
    large: 'text-base font-normal',
    medium: 'text-sm font-normal',
    small: 'text-xs font-normal',
  },
  label: {
    large: 'text-sm font-medium',
    medium: 'text-xs font-medium',
    small: 'text-xs font-medium',
  },
};

// 🆕 M3 EXPRESSIVE: Emphasized Typography
export const typographyEmphasized = {
  display: {
    large: 'text-5xl font-black tracking-tighter',      // 900 weight!
    medium: 'text-4xl font-extrabold tracking-tight',   // 800 weight
    small: 'text-3xl font-bold tracking-tight',         // 700 weight
  },
  headline: {
    large: 'text-2xl font-bold',                        // 700 weight
    medium: 'text-xl font-bold',
    small: 'text-lg font-bold',
  },
  title: {
    large: 'text-lg font-bold',                         // Emphasis upgrade
    medium: 'text-base font-bold',
    small: 'text-sm font-bold',
  },
  body: {
    large: 'text-base font-semibold',                   // 600 weight
    medium: 'text-sm font-semibold',
    small: 'text-xs font-semibold',
  },
  label: {
    large: 'text-sm font-bold',                         // Extra emphasis
    medium: 'text-xs font-bold',
    small: 'text-xs font-bold',
  },
};

// 🆕 M3 EXPRESSIVE: Typography Utilities
export const typographyUtils = {
  // Letter spacing variations
  spacing: {
    tighter: 'tracking-tighter',  // -0.05em
    tight: 'tracking-tight',      // -0.025em
    normal: 'tracking-normal',    // 0
    wide: 'tracking-wide',        // 0.025em
    wider: 'tracking-wider',      // 0.05em
  },
  
  // Line height variations
  leading: {
    tight: 'leading-tight',       // 1.25
    snug: 'leading-snug',         // 1.375
    normal: 'leading-normal',     // 1.5
    relaxed: 'leading-relaxed',   // 1.625
    loose: 'leading-loose',       // 2
  },
  
  // Text decoration for emphasis
  decoration: {
    underline: 'underline underline-offset-4 decoration-2',
    gradient: 'bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent',
  },
};

// Animation System - Subtle, purposeful
export const transitions = {
  fast: 'transition-all duration-100 ease-out',
  normal: 'transition-all duration-200 ease-in-out',
  slow: 'transition-all duration-300 ease-in-out',
  emphasis: 'transition-all duration-400 cubic-bezier(0.4, 0, 0.2, 1)', // Material motion
  // Legacy v1.0 (for backwards compatibility)
  smooth: 'transition-all duration-200 ease-in-out',
  spring: 'transition-all duration-300 cubic-bezier(0.34, 1.56, 0.64, 1)',
};

// 🆕 M3 EXPRESSIVE: Spring Motion Physics System
export const springs = {
  // SPATIAL SPRINGS - for position, scale, rotate (physics-based!)
  spatial: {
    bouncy: 'cubic-bezier(0.34, 1.56, 0.64, 1)',      // Heavy bounce - M3 Expressive signature!
    smooth: 'cubic-bezier(0.4, 0.0, 0.2, 1)',         // Material standard motion
    gentle: 'cubic-bezier(0.25, 0.1, 0.25, 1)',       // Subtle spring
    energetic: 'cubic-bezier(0.68, -0.55, 0.265, 1.55)', // Extra bouncy
  },
  
  // EFFECTS SPRINGS - for opacity, color, background (smooth!)
  effects: {
    fade: 'cubic-bezier(0.4, 0.0, 1, 1)',             // Fade in/out
    color: 'cubic-bezier(0.4, 0.0, 0.2, 1)',          // Color transitions
    glow: 'cubic-bezier(0.25, 0.46, 0.45, 0.94)',     // Glow effects
  },
  
  // DURATION SCALES - M3 Expressive timing
  duration: {
    instant: '100ms',   // Immediate feedback
    fast: '200ms',      // Quick interactions
    normal: '300ms',    // Standard animations
    slow: '400ms',      // Emphasized motion
    slower: '500ms',    // Hero moments
    slowest: '700ms',   // Special effects
  },
};

// 🆕 M3 EXPRESSIVE: Motion Presets (ready-to-use!)
export const motionPresets = {
  // Button hover/press
  buttonHover: `transition-all ${springs.duration.fast} ease-[${springs.spatial.smooth}]`,
  buttonPress: `transition-all ${springs.duration.instant} ease-[${springs.spatial.bouncy}]`,
  
  // Card interactions
  cardHover: `transition-all ${springs.duration.normal} ease-[${springs.spatial.gentle}]`,
  cardOpen: `transition-all ${springs.duration.slow} ease-[${springs.spatial.smooth}]`,
  
  // Modal/Dialog
  modalEnter: `transition-all ${springs.duration.slow} ease-[${springs.spatial.bouncy}]`,
  modalExit: `transition-all ${springs.duration.normal} ease-[${springs.effects.fade}]`,
  
  // Tooltips/Popovers
  tooltipEnter: `transition-all ${springs.duration.fast} ease-[${springs.spatial.gentle}]`,
  
  // Page transitions
  pageEnter: `transition-all ${springs.duration.slower} ease-[${springs.spatial.smooth}]`,
  
  // Micro animations
  microBounce: `transition-all ${springs.duration.normal} ease-[${springs.spatial.bouncy}]`,
  microFade: `transition-all ${springs.duration.fast} ease-[${springs.effects.fade}]`,
};

// Legacy v1.0 Design Tokens (Backwards Compatibility)
// These are kept for components that haven't been migrated to Material 3 yet

export const shadows = {
  sm: 'shadow-sm',
  md: 'shadow-md',
  lg: 'shadow-lg',
  xl: 'shadow-xl',
  '2xl': 'shadow-2xl',
};

export const radius = {
  sm: 'rounded-sm',
  md: 'rounded-md',
  lg: 'rounded-lg',
  xl: 'rounded-xl',
  '2xl': 'rounded-2xl',
  full: 'rounded-full',
};

export const colorMap = {
  primary: 'from-blue-500 to-blue-600',
  secondary: 'from-gray-500 to-gray-600',
  success: 'from-green-500 to-green-600',
  danger: 'from-red-500 to-red-600',
  warning: 'from-yellow-500 to-yellow-600',
  info: 'from-cyan-500 to-cyan-600',
};

export const colorText = {
  primary: 'text-blue-600 dark:text-blue-400',
  secondary: 'text-gray-600 dark:text-gray-400',
  success: 'text-green-600 dark:text-green-400',
  danger: 'text-red-600 dark:text-red-400',
  warning: 'text-yellow-600 dark:text-yellow-400',
  info: 'text-cyan-600 dark:text-cyan-400',
};

export const colorBorder = {
  primary: 'border-blue-500 dark:border-blue-400',
  secondary: 'border-gray-500 dark:border-gray-400',
  success: 'border-green-500 dark:border-green-400',
  danger: 'border-red-500 dark:border-red-400',
  warning: 'border-yellow-500 dark:border-yellow-400',
  info: 'border-cyan-500 dark:border-cyan-400',
};

export const glowEffects = {
  primary: 'shadow-[0_0_20px_rgba(59,130,246,0.5)]',
  secondary: 'shadow-[0_0_20px_rgba(107,114,128,0.5)]',
  success: 'shadow-[0_0_20px_rgba(34,197,94,0.5)]',
  danger: 'shadow-[0_0_20px_rgba(239,68,68,0.5)]',
  warning: 'shadow-[0_0_20px_rgba(234,179,8,0.5)]',
};

export const sizeMap = {
  xs: 'h-7 px-2 text-xs',
  sm: 'h-8 px-3 text-sm',
  md: 'h-10 px-4 text-sm',
  lg: 'h-11 px-5 text-base',
  xl: 'h-12 px-6 text-base',
};
