import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import("@sveltejs/vite-plugin-svelte").SvelteConfig} */
export default {
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  
  // Compiler options
  compilerOptions: {
    // Disable unused CSS selector warnings for icon libraries
    // (Bootstrap Icons has 2000+ selectors, we only use ~20)
    css: 'external'
  },
  
  // Suppress warnings
  onwarn: (warning, handler) => {
    // Ignore unused CSS selector warnings (common with icon libraries)
    if (warning.code === 'css-unused-selector') return;
    
    // Handle all other warnings normally
    handler(warning);
  }
}
