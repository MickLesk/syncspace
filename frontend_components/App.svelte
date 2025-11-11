<script>
  import "./src/app.css";
  import { onMount } from "svelte";
  import { theme } from "./stores/theme.ts";
  import Home from "./pages/Home.svelte";
  import ThemeToggle from "./atoms/ThemeToggle.svelte";

  let currentPage = $state("home");
  let loadedComponents = $state({});
  let isLoading = $state(false);
  let loadingPage = $state("");

  // Initialize theme ASAP
  onMount(() => {
    theme.init();
    loadedComponents.home = Home;
  });

  // Lazy load page components on demand
  const componentLoaders = {
    home: () => Promise.resolve({ default: Home }),
    atoms: () => import("./pages/AtomsDemo.svelte"),
    molecules: () => import("./pages/MoleculesDemo.svelte"),
    organisms: () => import("./pages/OrganismsDemo.svelte"),
    playground: () => import("./pages/PlaygroundView.svelte"),
    dashboard: () => import("./pages/DashboardTemplate.svelte"),
  };

  async function loadComponent(pageKey) {
    // Prevent loading if already loaded or currently loading
    if (loadedComponents[pageKey] || loadingPage === pageKey) return;

    loadingPage = pageKey;
    isLoading = true;
    try {
      const module = await componentLoaders[pageKey]();
      loadedComponents = { ...loadedComponents, [pageKey]: module.default };
    } catch (error) {
      console.error(`Failed to load ${pageKey}:`, error);
    } finally {
      isLoading = false;
      loadingPage = "";
    }
  }

  // Auto-load component when page changes
  $effect(() => {
    if (
      currentPage &&
      !loadedComponents[currentPage] &&
      loadingPage !== currentPage
    ) {
      loadComponent(currentPage);
    }
  });

  const pages = {
    home: { label: "Home", icon: "bi-house-fill" },
    atoms: { label: "Atoms", icon: "bi-box" },
    molecules: { label: "Molecules", icon: "bi-grid-3x3" },
    organisms: { label: "Organisms", icon: "bi-layers" },
    playground: { label: "Playground", icon: "bi-code-slash", featured: true },
    dashboard: { label: "Dashboard", icon: "bi-speedometer2", featured: true },
  };
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-950 dark:to-gray-900"
>
  <!-- Modern Navigation -->
  <nav
    class="fixed top-0 left-0 right-0 bg-white/80 dark:bg-gray-900/80 backdrop-blur-lg border-b border-gray-200 dark:border-gray-800 z-50 shadow-sm"
  >
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <!-- Logo -->
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-600 to-purple-600 flex items-center justify-center shadow-lg shadow-blue-500/30"
          >
            <i class="bi bi-lightning-charge-fill text-white text-xl"></i>
          </div>
          <div>
            <h1
              class="text-lg font-extrabold bg-gradient-to-r from-gray-900 to-gray-600 dark:from-white dark:to-gray-300 bg-clip-text text-transparent"
            >
              SyncSpace
            </h1>
            <p
              class="text-[10px] text-gray-500 dark:text-gray-400 font-semibold tracking-wider"
            >
              COMPONENTS
            </p>
          </div>
        </div>

        <!-- Navigation Links -->
        <div class="flex items-center gap-1">
          {#each Object.entries(pages) as [key, page]}
            <button
              onclick={() => (currentPage = key)}
              class="group relative px-3 py-2 rounded-lg font-semibold text-sm transition-all duration-200 {currentPage ===
              key
                ? 'bg-blue-600 text-white shadow-lg shadow-blue-500/30 scale-105'
                : 'text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-800'}"
            >
              <span class="flex items-center gap-1.5">
                {#if page.icon}
                  <i class="{page.icon} text-base"></i>
                {/if}
                <span class="hidden lg:inline">{page.label}</span>
              </span>
              {#if page.featured}
                <span
                  class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full animate-pulse"
                ></span>
              {/if}
            </button>
          {/each}
          <div class="ml-2 pl-2 border-l border-gray-200 dark:border-gray-700">
            <ThemeToggle />
          </div>
        </div>
      </div>
    </div>
  </nav>

  <!-- Content with smooth transitions -->
  <div class="pt-16">
    {#if isLoading}
      <div class="flex items-center justify-center min-h-[calc(100vh-4rem)]">
        <div class="text-center">
          <div
            class="w-16 h-16 border-4 border-blue-600 border-t-transparent rounded-full animate-spin mx-auto mb-4"
          ></div>
          <p class="text-slate-600 dark:text-slate-400">Loading component...</p>
        </div>
      </div>
    {:else if loadedComponents[currentPage]}
      {@const Component = loadedComponents[currentPage]}
      <Component />
    {:else}
      <div class="flex items-center justify-center min-h-[calc(100vh-4rem)]">
        <p class="text-slate-600 dark:text-slate-400">Page not found</p>
      </div>
    {/if}
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }

  :global(*) {
    box-sizing: border-box;
  }
</style>
