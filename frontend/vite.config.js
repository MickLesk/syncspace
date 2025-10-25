import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    svelte({
      onwarn: (warning, handler) => {
        // Completely ignore all unused CSS selector warnings
        if (warning.code === 'css-unused-selector') return;
        
        // Ignore accessibility warnings
        if (warning.code && warning.code.startsWith('a11y_')) return;
        
        // Pass through other warnings
        handler(warning);
      }
    })
  ],
})
