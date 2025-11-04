/// <reference types="svelte" />

declare namespace svelteHTML {
  // Add any custom HTML attributes here
  interface HTMLAttributes<T> {
    // Allow any attribute to exist
    [key: string]: any;
  }
}

// Svelte component types
declare module "*.svelte" {
  export { SvelteComponent as default } from "svelte";
}

// Vite client types
/// <reference types="vite/client" />
