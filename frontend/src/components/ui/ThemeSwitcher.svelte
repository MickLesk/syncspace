<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { auth } from "../../stores/auth.js";

  let currentTheme = $state("syncspace");
  let loading = $state(false);

  const themes = [
    { id: "syncspace", name: "Light", icon: "bi-sun-fill" },
    { id: "syncspace-dark", name: "Dark", icon: "bi-moon-fill" },
  ];

  // Map backend theme names to frontend theme names
  function backendToFrontendTheme(backendTheme) {
    if (backendTheme === "dark") return "syncspace-dark";
    return "syncspace"; // light or any other value defaults to light
  }

  // Map frontend theme names to backend theme names
  function frontendToBackendTheme(frontendTheme) {
    if (frontendTheme === "syncspace-dark") return "dark";
    return "light";
  }

  onMount(async () => {
    // Load theme from backend if logged in
    if ($auth.isLoggedIn) {
      await loadThemeFromBackend();
    } else {
      // Fallback to localStorage for non-logged in users
      const saved = localStorage.getItem("theme") || "syncspace";
      applyTheme(saved);
    }
  });

  async function loadThemeFromBackend() {
    try {
      const settings = await api.users.getSettings();
      if (settings && settings.theme) {
        const frontendTheme = backendToFrontendTheme(settings.theme);
        applyTheme(frontendTheme);
      }
    } catch (err) {
      // Silent fallback for 404 (endpoint not implemented) - this is expected
      if (err.message && err.message.includes("404")) {
        console.log(
          "[ThemeSwitcher] Backend endpoint not implemented, using localStorage"
        );
      } else {
        console.error("Failed to load theme from backend:", err);
      }
      // Fallback to localStorage
      const saved = localStorage.getItem("theme") || "syncspace";
      applyTheme(saved);
    }
  }

  function applyTheme(theme) {
    currentTheme = theme;
    const isDark = theme === "syncspace-dark";

    // Set data-theme attribute (for backwards compatibility)
    document.documentElement.setAttribute("data-theme", theme);

    // Set 'dark' class for Tailwind dark mode
    if (isDark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }

    // Still save to localStorage for instant load on next visit
    localStorage.setItem("theme", theme);
  }

  async function setTheme(theme) {
    loading = true;
    applyTheme(theme);

    // Save to backend if logged in
    if ($auth.isLoggedIn) {
      try {
        const backendTheme = frontendToBackendTheme(theme);
        await api.users.updateSettings({ theme: backendTheme });
      } catch (err) {
        console.error("Failed to save theme to backend:", err);
      }
    }
    loading = false;
  }

  async function toggleTheme() {
    const newTheme =
      currentTheme === "syncspace" ? "syncspace-dark" : "syncspace";
    await setTheme(newTheme);
  }
</script>

<button
  class="w-11 h-11 flex items-center justify-center rounded-xl bg-gray-100 hover:bg-gray-200 transition-all duration-300 border-2 border-gray-300 hover:border-gray-400"
  onclick={toggleTheme}
  title="Toggle theme"
>
  <i
    class="{currentTheme === 'syncspace'
      ? 'bi-moon'
      : 'bi-sun'} text-xl text-gray-700"
  ></i>
</button>

<style>
  button {
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }
  button:hover {
    transform: rotate(15deg) scale(1.1);
  }
  button i {
    transition: transform 0.3s ease;
  }
  button:hover i {
    transform: rotate(180deg);
  }
</style>
