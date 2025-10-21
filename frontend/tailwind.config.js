/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#6750a4',
        secondary: '#625b71',
        tertiary: '#7d5260',
        surface: '#fffbfe',
        surfaceVariant: '#e7e0ec',
        outline: '#79747e',
        error: '#b3261e',
      }
    },
  },
  plugins: [],
}
