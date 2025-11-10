<script>
  import "./src/app.css";
  import DemoHome from "./pages/DemoHome.svelte";
  import AtomsDemo from "./pages/AtomsDemo.svelte";
  import MoleculesDemo from "./pages/MoleculesDemo.svelte";
  import OrganismsDemo from "./pages/OrganismsDemo.svelte";
  import ThemeToggle from "./atoms/ThemeToggle.svelte";

  let currentPage = "home";

  const pages = {
    home: { component: DemoHome, label: "Home" },
    atoms: { component: AtomsDemo, label: "Atoms" },
    molecules: { component: MoleculesDemo, label: "Molecules" },
    organisms: { component: OrganismsDemo, label: "Organisms" },
  };
</script>

<div class="min-h-screen bg-white dark:bg-slate-900">
  <!-- Navigation -->
  <nav
    class="fixed top-0 left-0 right-0 bg-gray-100 dark:bg-slate-800 border-b border-gray-300 dark:border-slate-700 z-50"
  >
    <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
      <h1 class="text-2xl font-bold text-white">SyncSpace Components</h1>
      <div class="flex items-center gap-4">
        <div class="flex gap-4">
          {#each Object.entries(pages) as [key, page]}
            <button
              onclick={() => (currentPage = key)}
              class="px-4 py-2 rounded-lg font-medium transition-colors {currentPage ===
              key
                ? 'bg-blue-600 text-white'
                : 'bg-slate-700 text-slate-300 hover:bg-slate-600'}"
            >
              {page.label}
            </button>
          {/each}
        </div>
        <ThemeToggle />
      </div>
    </div>
  </nav>

  <!-- Content -->
  <div class="pt-20 bg-white dark:bg-slate-900 min-h-screen">
    {#key currentPage}
      <svelte:component this={pages[currentPage].component} />
    {/key}
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
