/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
      },
      colors: {
        // SyncSpace Brand Colors
        brand: {
          primary: '#667eea',
          secondary: '#764ba2',
        },
      },
    },
  },
  plugins: [
    require('daisyui'),
    require('@tailwindcss/typography'),
  ],
  daisyui: {
    themes: [
      {
        syncspace: {
          "primary": "#667eea",           // Brand Purple
          "secondary": "#764ba2",         // Deep Purple
          "accent": "#f59e0b",            // Amber
          "neutral": "#1f2937",           // Dark Gray
          "base-100": "#ffffff",          // White background
          "base-200": "#f9fafb",          // Light gray surface
          "base-300": "#f3f4f6",          // Lighter gray
          "info": "#3b82f6",              // Blue
          "success": "#10b981",           // Emerald
          "warning": "#f59e0b",           // Orange
          "error": "#ef4444",             // Red
        },
        "syncspace-dark": {
          "primary": "#667eea",           // Brand Purple
          "secondary": "#764ba2",         // Deep Purple
          "accent": "#f59e0b",            // Amber
          "neutral": "#374151",           // Gray
          "base-100": "#111827",          // Dark background
          "base-200": "#1f2937",          // Dark surface
          "base-300": "#374151",          // Lighter dark
          "info": "#3b82f6",              // Blue
          "success": "#10b981",           // Emerald
          "warning": "#f59e0b",           // Orange
          "error": "#ef4444",             // Red
        },
      },
      "light",
      "dark",
    ],
    base: true,
    styled: true,
    utils: true,
    logs: false,
  },
}
