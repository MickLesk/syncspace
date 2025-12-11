import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { resolve } from 'path'

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
  build: {
    rollupOptions: {
      output: {
        assetFileNames: (assetInfo) => {
          // Keep font files in a fonts directory
          if (assetInfo.name && /\.(woff2?|eot|ttf|otf)$/.test(assetInfo.name)) {
            return 'fonts/[name]-[hash][extname]';
          }
          return 'assets/[name]-[hash][extname]';
        },
        manualChunks: (id) => {
          // Monaco Editor - separate chunk
          if (id.includes('node_modules/monaco-editor')) {
            return 'vendor-monaco';
          }
          
          // Vendor chunks - separate large dependencies
          if (id.includes('node_modules/svelte')) {
            return 'vendor-svelte';
          }
          if (id.includes('node_modules/bootstrap-icons')) {
            return 'vendor-icons';
          }
          
          // UI components chunk (shared across all views)
          if (id.includes('src/components/ui/') || id.includes('src/components/navigation/')) {
            return 'ui-components';
          }
          
          // Split pages by category for lazy loading
          // Each category becomes its own chunk that's loaded on-demand
          if (id.includes('src/pages/files/')) {
            return 'page-files';
          }
          if (id.includes('src/pages/admin/')) {
            return 'page-admin';
          }
          if (id.includes('src/pages/user/')) {
            return 'page-user';
          }
          if (id.includes('src/pages/system/')) {
            return 'page-system';
          }
          if (id.includes('src/pages/tools/')) {
            return 'page-tools';
          }
          if (id.includes('src/pages/settings/')) {
            return 'page-settings';
          }
          if (id.includes('src/pages/rbac/') || id.includes('src/pages/workflow/')) {
            return 'page-enterprise';
          }
          if (id.includes('src/pages/jobs/')) {
            return 'page-jobs';
          }
          if (id.includes('src/pages/analytics/')) {
            return 'page-analytics';
          }
          if (id.includes('src/pages/trash/')) {
            return 'page-trash';
          }
        }
      }
    },
    chunkSizeWarningLimit: 1000, // Increase for Monaco Editor
    sourcemap: false // Disable sourcemaps in production for smaller builds
  },
  resolve: {
    alias: {
      'bootstrap-icons/font/fonts': resolve(__dirname, 'node_modules/bootstrap-icons/font/fonts'),
      '@': resolve(__dirname, 'src'),
      '@stores': resolve(__dirname, 'src/stores'),
      '@components': resolve(__dirname, 'src/components'),
      '@lib': resolve(__dirname, 'src/lib')
    },
    extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.svelte']
  },
  // Monaco Editor Worker configuration
  optimizeDeps: {
    include: ['monaco-editor']
  }
})
