/**
 * SyncSpace Design System - Design Tokens v3
 * 
 * Zentrale Design-Tokens für das gesamte Projekt.
 * Diese Tokens werden sowohl in CSS (@theme) als auch in JavaScript verwendet.
 */

// =============================================================================
// COLOR TOKENS
// =============================================================================

export const colors = {
  // Primary Brand Colors (Gradient)
  primary: {
    50: 'oklch(0.97 0.02 265)',
    100: 'oklch(0.94 0.04 265)',
    200: 'oklch(0.88 0.08 265)',
    300: 'oklch(0.79 0.14 265)',
    400: 'oklch(0.68 0.20 265)',
    500: 'oklch(0.58 0.25 265)',  // Main brand color
    600: 'oklch(0.51 0.24 265)',
    700: 'oklch(0.44 0.20 265)',
    800: 'oklch(0.38 0.16 265)',
    900: 'oklch(0.32 0.12 265)',
    950: 'oklch(0.24 0.08 265)',
  },
  
  // Semantic Colors
  success: {
    50: 'oklch(0.98 0.02 156)',
    100: 'oklch(0.96 0.04 156)',
    200: 'oklch(0.92 0.08 156)',
    300: 'oklch(0.87 0.15 154)',
    400: 'oklch(0.79 0.21 152)',
    500: 'oklch(0.72 0.22 150)',
    600: 'oklch(0.63 0.19 149)',
    700: 'oklch(0.53 0.15 150)',
    800: 'oklch(0.45 0.12 151)',
    900: 'oklch(0.39 0.10 153)',
    950: 'oklch(0.27 0.07 153)',
  },
  
  warning: {
    50: 'oklch(0.99 0.03 95)',
    100: 'oklch(0.97 0.07 96)',
    200: 'oklch(0.94 0.13 96)',
    300: 'oklch(0.90 0.18 92)',
    400: 'oklch(0.85 0.20 84)',
    500: 'oklch(0.79 0.18 70)',
    600: 'oklch(0.67 0.18 58)',
    700: 'oklch(0.55 0.16 49)',
    800: 'oklch(0.47 0.14 46)',
    900: 'oklch(0.41 0.11 46)',
    950: 'oklch(0.28 0.08 46)',
  },
  
  error: {
    50: 'oklch(0.97 0.01 17)',
    100: 'oklch(0.94 0.03 18)',
    200: 'oklch(0.88 0.06 18)',
    300: 'oklch(0.81 0.11 20)',
    400: 'oklch(0.70 0.19 22)',
    500: 'oklch(0.64 0.24 25)',
    600: 'oklch(0.58 0.25 27)',
    700: 'oklch(0.50 0.21 28)',
    800: 'oklch(0.44 0.18 27)',
    900: 'oklch(0.40 0.14 26)',
    950: 'oklch(0.26 0.09 26)',
  },
  
  info: {
    50: 'oklch(0.98 0.01 237)',
    100: 'oklch(0.95 0.03 237)',
    200: 'oklch(0.90 0.06 231)',
    300: 'oklch(0.83 0.11 230)',
    400: 'oklch(0.75 0.16 233)',
    500: 'oklch(0.68 0.17 237)',
    600: 'oklch(0.59 0.16 242)',
    700: 'oklch(0.50 0.13 243)',
    800: 'oklch(0.44 0.11 241)',
    900: 'oklch(0.39 0.09 241)',
    950: 'oklch(0.29 0.07 243)',
  },
};

// =============================================================================
// GRADIENT TOKENS
// =============================================================================

export const gradients = {
  // Primary gradient (used in buttons, headers, etc.)
  primary: 'linear-gradient(135deg, oklch(0.60 0.20 265), oklch(0.52 0.22 300), oklch(0.65 0.20 330))',
  primaryHover: 'linear-gradient(135deg, oklch(0.55 0.22 265), oklch(0.48 0.24 300), oklch(0.60 0.22 330))',
  
  // Subtle gradients for backgrounds
  subtleLight: 'linear-gradient(135deg, oklch(0.99 0.01 265), oklch(0.98 0.02 300))',
  subtleDark: 'linear-gradient(135deg, oklch(0.18 0.02 265), oklch(0.15 0.02 300))',
  
  // Glass effect backgrounds
  glassLight: 'linear-gradient(135deg, rgba(255,255,255,0.9), rgba(255,255,255,0.7))',
  glassDark: 'linear-gradient(135deg, rgba(31,41,55,0.9), rgba(31,41,55,0.7))',
  
  // Success, warning, error gradients
  success: 'linear-gradient(135deg, oklch(0.72 0.22 150), oklch(0.65 0.20 160))',
  warning: 'linear-gradient(135deg, oklch(0.79 0.18 70), oklch(0.72 0.20 60))',
  error: 'linear-gradient(135deg, oklch(0.64 0.24 25), oklch(0.58 0.22 15))',
};

// =============================================================================
// SPACING TOKENS
// =============================================================================

export const spacing = {
  px: '1px',
  0: '0',
  0.5: '0.125rem',  // 2px
  1: '0.25rem',     // 4px
  1.5: '0.375rem',  // 6px
  2: '0.5rem',      // 8px
  2.5: '0.625rem',  // 10px
  3: '0.75rem',     // 12px
  3.5: '0.875rem',  // 14px
  4: '1rem',        // 16px
  5: '1.25rem',     // 20px
  6: '1.5rem',      // 24px
  7: '1.75rem',     // 28px
  8: '2rem',        // 32px
  9: '2.25rem',     // 36px
  10: '2.5rem',     // 40px
  11: '2.75rem',    // 44px
  12: '3rem',       // 48px
  14: '3.5rem',     // 56px
  16: '4rem',       // 64px
  20: '5rem',       // 80px
  24: '6rem',       // 96px
  28: '7rem',       // 112px
  32: '8rem',       // 128px
  36: '9rem',       // 144px
  40: '10rem',      // 160px
  44: '11rem',      // 176px
  48: '12rem',      // 192px
  52: '13rem',      // 208px
  56: '14rem',      // 224px
  60: '15rem',      // 240px
  64: '16rem',      // 256px
  72: '18rem',      // 288px
  80: '20rem',      // 320px
  96: '24rem',      // 384px
};

// =============================================================================
// BORDER RADIUS TOKENS
// =============================================================================

export const borderRadius = {
  none: '0',
  sm: '0.25rem',    // 4px
  DEFAULT: '0.375rem', // 6px
  md: '0.5rem',     // 8px
  lg: '0.75rem',    // 12px
  xl: '1rem',       // 16px
  '2xl': '1.25rem', // 20px
  '3xl': '1.5rem',  // 24px
  '4xl': '2rem',    // 32px
  full: '9999px',
};

// =============================================================================
// SHADOW TOKENS
// =============================================================================

export const shadows = {
  // Light mode shadows
  sm: '0 1px 2px 0 rgb(0 0 0 / 0.05)',
  DEFAULT: '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
  md: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
  lg: '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
  xl: '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)',
  '2xl': '0 25px 50px -12px rgb(0 0 0 / 0.25)',
  inner: 'inset 0 2px 4px 0 rgb(0 0 0 / 0.05)',
  none: 'none',
  
  // Glass effect shadows
  glass: '0 8px 32px 0 rgba(31, 38, 135, 0.15)',
  glassDark: '0 8px 32px 0 rgba(0, 0, 0, 0.3)',
  
  // Glow effects
  glowPrimary: '0 0 20px -5px oklch(0.58 0.25 265 / 0.4)',
  glowSuccess: '0 0 20px -5px oklch(0.72 0.22 150 / 0.4)',
  glowError: '0 0 20px -5px oklch(0.64 0.24 25 / 0.4)',
};

// =============================================================================
// TYPOGRAPHY TOKENS
// =============================================================================

export const typography = {
  fontFamily: {
    sans: 'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
    mono: 'JetBrains Mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
  },
  
  fontSize: {
    xs: ['0.75rem', { lineHeight: '1rem' }],
    sm: ['0.875rem', { lineHeight: '1.25rem' }],
    base: ['1rem', { lineHeight: '1.5rem' }],
    lg: ['1.125rem', { lineHeight: '1.75rem' }],
    xl: ['1.25rem', { lineHeight: '1.75rem' }],
    '2xl': ['1.5rem', { lineHeight: '2rem' }],
    '3xl': ['1.875rem', { lineHeight: '2.25rem' }],
    '4xl': ['2.25rem', { lineHeight: '2.5rem' }],
    '5xl': ['3rem', { lineHeight: '1' }],
    '6xl': ['3.75rem', { lineHeight: '1' }],
  },
  
  fontWeight: {
    thin: '100',
    extralight: '200',
    light: '300',
    normal: '400',
    medium: '500',
    semibold: '600',
    bold: '700',
    extrabold: '800',
    black: '900',
  },
  
  letterSpacing: {
    tighter: '-0.05em',
    tight: '-0.025em',
    normal: '0em',
    wide: '0.025em',
    wider: '0.05em',
    widest: '0.1em',
  },
};

// =============================================================================
// ANIMATION TOKENS
// =============================================================================

export const animations = {
  // Durations
  duration: {
    instant: '0ms',
    fast: '100ms',
    normal: '200ms',
    slow: '300ms',
    slower: '500ms',
    slowest: '700ms',
  },
  
  // Easing functions
  easing: {
    linear: 'linear',
    easeIn: 'cubic-bezier(0.4, 0, 1, 1)',
    easeOut: 'cubic-bezier(0, 0, 0.2, 1)',
    easeInOut: 'cubic-bezier(0.4, 0, 0.2, 1)',
    spring: 'cubic-bezier(0.175, 0.885, 0.32, 1.275)',
    bounce: 'cubic-bezier(0.68, -0.55, 0.265, 1.55)',
  },
  
  // Animation keyframe names
  keyframes: {
    fadeIn: 'fadeIn',
    fadeOut: 'fadeOut',
    slideUp: 'slideUp',
    slideDown: 'slideDown',
    slideLeft: 'slideLeft',
    slideRight: 'slideRight',
    scaleIn: 'scaleIn',
    scaleOut: 'scaleOut',
    shake: 'shake',
    pulse: 'pulse',
    spin: 'spin',
    bounce: 'bounce',
  },
};

// =============================================================================
// TRANSITIONS
// =============================================================================

export const transitions = {
  fast: 'all 100ms cubic-bezier(0.4, 0, 0.2, 1)',
  normal: 'all 200ms cubic-bezier(0.4, 0, 0.2, 1)',
  slow: 'all 300ms cubic-bezier(0.4, 0, 0.2, 1)',
  colors: 'color, background-color, border-color, fill, stroke 200ms cubic-bezier(0.4, 0, 0.2, 1)',
  transform: 'transform 200ms cubic-bezier(0.4, 0, 0.2, 1)',
  opacity: 'opacity 200ms cubic-bezier(0.4, 0, 0.2, 1)',
  shadow: 'box-shadow 200ms cubic-bezier(0.4, 0, 0.2, 1)',
};

// =============================================================================
// Z-INDEX TOKENS
// =============================================================================

export const zIndex = {
  hide: -1,
  base: 0,
  raised: 10,
  dropdown: 1000,
  sticky: 1100,
  overlay: 1200,
  modal: 1300,
  popover: 1400,
  toast: 1500,
  tooltip: 1600,
  max: 9999,
};

// =============================================================================
// BREAKPOINTS
// =============================================================================

export const breakpoints = {
  sm: '640px',
  md: '768px',
  lg: '1024px',
  xl: '1280px',
  '2xl': '1536px',
};

// =============================================================================
// ICON SIZES
// =============================================================================

export const iconSizes = {
  xs: '0.75rem',   // 12px
  sm: '1rem',      // 16px
  md: '1.25rem',   // 20px
  lg: '1.5rem',    // 24px
  xl: '2rem',      // 32px
  '2xl': '2.5rem', // 40px
  '3xl': '3rem',   // 48px
  '4xl': '4rem',   // 64px
};

// =============================================================================
// COMPONENT DEFAULTS
// =============================================================================

export const componentDefaults = {
  button: {
    height: {
      sm: '2rem',    // 32px
      md: '2.5rem',  // 40px
      lg: '3rem',    // 48px
    },
    padding: {
      sm: '0.75rem 1rem',
      md: '0.625rem 1.25rem',
      lg: '0.75rem 1.5rem',
    },
    fontSize: {
      sm: '0.75rem',
      md: '0.875rem',
      lg: '1rem',
    },
    borderRadius: '0.75rem',
  },
  
  input: {
    height: {
      sm: '2rem',
      md: '2.5rem',
      lg: '3rem',
    },
    padding: '0.625rem 1rem',
    borderRadius: '0.75rem',
    borderWidth: '1px',
  },
  
  card: {
    padding: {
      sm: '1rem',
      md: '1.5rem',
      lg: '2rem',
    },
    borderRadius: '1rem',
    borderWidth: '1px',
  },
  
  modal: {
    padding: '1.5rem',
    borderRadius: '1.25rem',
    maxWidth: {
      sm: '24rem',
      md: '28rem',
      lg: '32rem',
      xl: '36rem',
      '2xl': '42rem',
      '3xl': '48rem',
      full: '100%',
    },
  },
};

// =============================================================================
// EXPORT DEFAULT
// =============================================================================

export default {
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
};
