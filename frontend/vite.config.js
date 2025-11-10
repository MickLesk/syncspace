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
        manualChunks: {
          // Vendor chunks - separate large dependencies
          'vendor-svelte': ['svelte'],
          'vendor-icons': ['bootstrap-icons'],
          
          // UI components chunk
          'ui-components': [
            './src/components/ui/AppHeader.svelte',
            './src/components/ui/Modal.svelte',
            './src/components/ui/Toast.svelte',
            './src/components/navigation/Sidebar.svelte'
          ],
          
          // File management chunk
          'file-views': [
            './src/pages/files/FilesView.svelte',
            './src/pages/files/SharedView.svelte',
            './src/pages/files/RecentFilesView.svelte'
          ],
          
          // Admin/Settings chunk
          'admin-views': [
            './src/pages/user/UserSettingsView.svelte',
            './src/pages/user/UserProfileView.svelte',
            './src/pages/user/UsersView.svelte'
          ]
        }
      }
    },
    chunkSizeWarningLimit: 600, // Increase from default 500KB
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
  }
})
