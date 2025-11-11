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
  const { subscribe, set: internalSet, update } = writable<Theme>(getInitialTheme());

  function applyTheme(value: Theme) {
    if (isBrowser) {
      localStorage.setItem("theme", value);
      if (value === "dark") {
        document.documentElement.classList.add("dark");
      } else {
        document.documentElement.classList.remove("dark");
      }
    }
    internalSet(value);
  }

  return {
    subscribe,
    set: applyTheme,
    toggle: () => {
      update((current) => {
        const next = current === "light" ? "dark" : "light";
        applyTheme(next);
        return next;
      });
    },
    init: () => {
      const theme = getInitialTheme();
      applyTheme(theme);
    },
  };
}

export const theme = createThemeStore();
