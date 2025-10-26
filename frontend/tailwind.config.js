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
          "primary": "#818cf8",           // Lighter Purple for better contrast
          "primary-content": "#0f172a",   // Dark text on primary
          "secondary": "#9333ea",         // Brighter Purple
          "secondary-content": "#ffffff", // White text on secondary
          "accent": "#fbbf24",            // Brighter Amber
          "accent-content": "#0f172a",    // Dark text on accent
          "neutral": "#374151",           // Gray
          "neutral-content": "#f3f4f6",   // Light text on neutral
          "base-100": "#0f172a",          // Darker background (slate-900)
          "base-200": "#1e293b",          // Dark surface (slate-800)
          "base-300": "#334155",          // Lighter dark (slate-700)
          "base-content": "#f1f5f9",      // Light text (slate-100)
          "info": "#60a5fa",              // Brighter Blue
          "info-content": "#0f172a",      // Dark text on info
          "success": "#34d399",           // Brighter Emerald
          "success-content": "#0f172a",   // Dark text on success
          "warning": "#fbbf24",           // Brighter Orange
          "warning-content": "#0f172a",   // Dark text on warning
          "error": "#f87171",             // Brighter Red
          "error-content": "#ffffff",     // White text on error
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
