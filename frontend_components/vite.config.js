import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  server: {
    port: 5174,
    open: true,
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // Vendor chunk for dependencies
          vendor: ['svelte'],
          // Separate chunks for pages (lazy loaded)
          atoms: ['./atoms/Button.svelte', './atoms/Card.svelte', './atoms/Badge.svelte'],
        },
      },
    },
    // Optimize chunk size
    chunkSizeWarningLimit: 500,
    // Enable source maps for production debugging
    sourcemap: false,
    // Use esbuild for faster minification
    minify: 'esbuild',
  },
});
