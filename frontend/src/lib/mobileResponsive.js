// mobileResponsive.js - Mobile responsiveness utilities
import { writable, derived } from 'svelte/store';

// Tailwind breakpoints
export const breakpoints = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  '2xl': 1536,
};

// Create screen size store
export function createScreenSize() {
  const screenWidth = writable(typeof window !== 'undefined' ? window.innerWidth : 1024);
  const screenHeight = writable(typeof window !== 'undefined' ? window.innerHeight : 768);

  if (typeof window !== 'undefined') {
    const handleResize = () => {
      screenWidth.set(window.innerWidth);
      screenHeight.set(window.innerHeight);
    };

    window.addEventListener('resize', handleResize);

    return {
      width: screenWidth,
      height: screenHeight,
      isMobile: derived(screenWidth, ($w) => $w < breakpoints.md),
      isTablet: derived(screenWidth, ($w) => $w >= breakpoints.md && $w < breakpoints.lg),
      isDesktop: derived(screenWidth, ($w) => $w >= breakpoints.lg),
      isLargeDesktop: derived(screenWidth, ($w) => $w >= breakpoints.xl),
      destroy: () => window.removeEventListener('resize', handleResize),
    };
  }

  return {
    width: screenWidth,
    height: screenHeight,
    isMobile: derived(screenWidth, ($w) => $w < breakpoints.md),
    isTablet: derived(screenWidth, ($w) => $w >= breakpoints.md && $w < breakpoints.lg),
    isDesktop: derived(screenWidth, ($w) => $w >= breakpoints.lg),
    isLargeDesktop: derived(screenWidth, ($w) => $w >= breakpoints.xl),
    destroy: () => {},
  };
}

// Singleton instance
export const screen = createScreenSize();

/**
 * Media query hook for components
 * Usage: mediaQuery('(min-width: 768px)') returns true/false
 */
export function mediaQuery(query) {
  if (typeof window === 'undefined') return writable(false);

  const matches = writable(window.matchMedia(query).matches);

  if (typeof window !== 'undefined') {
    const mediaQueryList = window.matchMedia(query);
    const handler = (e) => matches.set(e.matches);

    mediaQueryList.addEventListener('change', handler);

    return {
      subscribe: matches.subscribe,
      destroy: () => mediaQueryList.removeEventListener('change', handler),
    };
  }

  return matches;
}

/**
 * Format bytes for display (mobile-friendly)
 */
export function formatFileSize(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

/**
 * Truncate text for mobile display
 */
export function truncateText(text, maxLength = 20) {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength - 3) + '...';
}

/**
 * Format date for mobile (compact)
 */
export function formatDateMobile(date) {
  const d = new Date(date);
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  if (d.toDateString() === today.toDateString()) {
    return d.toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit' });
  }

  if (d.toDateString() === yesterday.toDateString()) {
    return 'Yesterday';
  }

  const daysAgo = Math.floor((today - d) / (1000 * 60 * 60 * 24));
  if (daysAgo < 7) {
    return `${daysAgo}d ago`;
  }

  return d.toLocaleDateString('de-DE', { month: 'short', day: 'numeric' });
}

/**
 * Touch-friendly dimensions
 */
export const touchDimensions = {
  minTapSize: 44, // 44x44px minimum for touch targets
  minSpacing: 8, // Minimum spacing between touch targets
  iconSize: {
    sm: 16,
    md: 24,
    lg: 32,
  },
  padding: {
    mobile: 12,
    tablet: 16,
    desktop: 20,
  },
};

/**
 * Check if device supports touch
 */
export function isTouchDevice() {
  return (
    typeof window !== 'undefined' &&
    (navigator.maxTouchPoints > 0 || navigator.msMaxTouchPoints > 0)
  );
}

/**
 * Get safe area insets (for notch/etc)
 */
export function getSafeAreaInsets() {
  if (typeof window === 'undefined') return { top: 0, bottom: 0, left: 0, right: 0 };

  const style = getComputedStyle(document.documentElement);
  return {
    top: parseInt(style.getPropertyValue('--safe-area-inset-top')) || 0,
    bottom: parseInt(style.getPropertyValue('--safe-area-inset-bottom')) || 0,
    left: parseInt(style.getPropertyValue('--safe-area-inset-left')) || 0,
    right: parseInt(style.getPropertyValue('--safe-area-inset-right')) || 0,
  };
}

/**
 * Responsive grid columns
 */
export function getGridCols(screenWidth) {
  if (screenWidth < 640) return 1; // Mobile
  if (screenWidth < 768) return 2; // Small tablet
  if (screenWidth < 1024) return 3; // Tablet
  if (screenWidth < 1280) return 4; // Desktop
  return 6; // Large desktop
}

/**
 * Responsive font size
 */
export function getResponsiveFontSize(screenWidth, baseSize = 16) {
  if (screenWidth < 640) return Math.round(baseSize * 0.9); // 90% on mobile
  if (screenWidth < 1024) return baseSize; // 100% on tablet
  return Math.round(baseSize * 1.1); // 110% on desktop
}
