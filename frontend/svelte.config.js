import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import("@sveltejs/vite-plugin-svelte").SvelteConfig} */
export default {
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  
  // Compiler options
  compilerOptions: {
    // Keep CSS scoped but ignore warnings
  },
  
  // Suppress warnings
  onwarn: (warning, handler) => {
    // Completely ignore all unused CSS selector warnings
    if (warning.code === 'css-unused-selector') return;
    
    // Ignore copilot-instructions.md warnings
    if (warning.filename && warning.filename.includes('copilot-instructions.md')) return;
    
    // Ignore accessibility warnings (can be re-enabled for testing)
    if (warning.code && warning.code.startsWith('a11y_')) return;
    
    // Pass through other warnings
    handler(warning);
  }
}
