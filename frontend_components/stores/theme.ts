import { writable } from "svelte/store";

type Theme = "light" | "dark";

// Check if we're in browser environment
const isBrowser = typeof window !== "undefined";

// Get initial theme from localStorage or system preference
function getInitialTheme(): Theme {
  if (!isBrowser) return "dark";
  
  const stored = localStorage.getItem("theme") as Theme | null;
  if (stored) return stored;
  
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

// Create theme store
function createThemeStore() {
  const { subscribe, set, update } = writable<Theme>(getInitialTheme());

  return {
    subscribe,
    set: (value: Theme) => {
      if (isBrowser) {
        localStorage.setItem("theme", value);
        document.documentElement.setAttribute("data-theme", value);
      }
      set(value);
    },
    toggle: () => {
      update((current) => {
        const next = current === "light" ? "dark" : "light";
        if (isBrowser) {
          localStorage.setItem("theme", next);
          document.documentElement.setAttribute("data-theme", next);
        }
        return next;
      });
    },
    init: () => {
      if (isBrowser) {
        const theme = getInitialTheme();
        document.documentElement.setAttribute("data-theme", theme);
        set(theme);
      }
    },
  };
}

export const theme = createThemeStore();
