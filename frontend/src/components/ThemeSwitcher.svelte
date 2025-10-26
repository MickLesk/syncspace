<script>
  import { onMount } from "svelte";

  let currentTheme = $state("syncspace");

  const themes = [
    { id: "syncspace", name: "Light", icon: "bi-sun-fill" },
    { id: "syncspace-dark", name: "Dark", icon: "bi-moon-fill" },
  ];

  onMount(() => {
    const saved = localStorage.getItem("theme") || "syncspace";
    setTheme(saved);

    // Auto-detect system preference
    const darkModeQuery = window.matchMedia("(prefers-color-scheme: dark)");
    if (!localStorage.getItem("theme")) {
      setTheme(darkModeQuery.matches ? "syncspace-dark" : "syncspace");
    }

    darkModeQuery.addEventListener("change", (e) => {
      if (!localStorage.getItem("theme")) {
        setTheme(e.matches ? "syncspace-dark" : "syncspace");
      }
    });
  });

  function setTheme(theme) {
    currentTheme = theme;
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }

  function toggleTheme() {
    const newTheme =
      currentTheme === "syncspace" ? "syncspace-dark" : "syncspace";
    setTheme(newTheme);
  }
</script>

<button
  class="btn btn-ghost btn-circle"
  onclick={toggleTheme}
  title="Toggle theme"
>
  <i class="{currentTheme === 'syncspace' ? 'bi-moon' : 'bi-sun'} text-xl"></i>
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
