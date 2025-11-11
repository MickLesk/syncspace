<script>
  import "./src/app.css";
  import Home from "./pages/Home.svelte";
  import AtomsDemo from "./pages/AtomsDemo.svelte";
  import MoleculesDemo from "./pages/MoleculesDemo.svelte";
  import OrganismsDemo from "./pages/OrganismsDemo.svelte";
  import ExpressiveDemo from "./pages/ExpressiveDemo.svelte";
  import ThemeToggle from "./atoms/ThemeToggle.svelte";

  let currentPage = $state("home");

  const pages = {
    home: { component: Home, label: "Home", icon: "bi-house-fill" },
    atoms: { component: AtomsDemo, label: "Atoms", icon: "bi-box" },
    molecules: {
      component: MoleculesDemo,
      label: "Molecules",
      icon: "bi-grid-3x3",
    },
    organisms: {
      component: OrganismsDemo,
      label: "Organisms",
      icon: "bi-layers",
    },
    expressive: {
      component: ExpressiveDemo,
      label: "M3 Expressive",
      icon: "bi-stars",
      featured: true,
    },
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
        <div class="flex items-center gap-2">
          {#each Object.entries(pages) as [key, page]}
            <button
              onclick={() => (currentPage = key)}
              class="group relative px-4 py-2 rounded-lg font-semibold text-sm transition-all duration-200 {currentPage ===
              key
                ? 'bg-blue-600 text-white shadow-lg shadow-blue-500/30 scale-105'
                : 'text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-800'}"
            >
              <span class="flex items-center gap-2">
                {#if page.icon}
                  <i class={page.icon}></i>
                {/if}
                {page.label}
              </span>
              {#if page.featured}
                <span
                  class="absolute -top-1 -right-1 w-2 h-2 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full animate-pulse"
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
    {#if currentPage === "home"}
      <Home />
    {:else if currentPage === "atoms"}
      <AtomsDemo />
    {:else if currentPage === "molecules"}
      <MoleculesDemo />
    {:else if currentPage === "organisms"}
      <OrganismsDemo />
    {:else if currentPage === "expressive"}
      <ExpressiveDemo />
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
