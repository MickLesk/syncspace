<script lang="ts">
  import { Button } from "../atoms/index.ts";

  let currentSection = "overview";

  const sections = [
    { id: "overview", label: "Overview", icon: "bi-house" },
    { id: "atoms", label: "Atoms", icon: "bi-cube" },
    { id: "molecules", label: "Molecules", icon: "bi-boxes" },
    { id: "organisms", label: "Organisms", icon: "bi-diagram-3" },
  ];
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-900 to-slate-800">
  <!-- Header -->
  <header
    class="border-b border-slate-700 bg-slate-800/50 backdrop-blur-sm sticky top-0 z-40"
  >
    <div class="max-w-7xl mx-auto px-6 py-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-white">
            SyncSpace Component Library
          </h1>
          <p class="text-slate-400 mt-2">
            Modern, reusable UI components built with Svelte 5 & Tailwind v4
          </p>
        </div>
        <div class="flex items-center gap-4">
          <a
            href="https://hyperui.dev"
            target="_blank"
            rel="noreferrer"
            class="text-slate-400 hover:text-white transition-colors"
          >
            <i class="bi bi-box-arrow-up-right text-xl"></i>
          </a>
        </div>
      </div>
    </div>
  </header>

  <!-- Navigation -->
  <nav class="border-b border-slate-700 bg-slate-800/30">
    <div class="max-w-7xl mx-auto px-6">
      <div class="flex gap-8">
        {#each sections as section (section.id)}
          <button
            onclick={() => (currentSection = section.id)}
            class="px-4 py-4 text-slate-400 hover:text-white transition-colors border-b-2 border-transparent hover:border-blue-500 font-medium
              {currentSection === section.id
              ? 'text-white border-blue-500'
              : ''}"
          >
            <i class="bi {section.icon} mr-2"></i>
            {section.label}
          </button>
        {/each}
      </div>
    </div>
  </nav>

  <!-- Content -->
  <main class="max-w-7xl mx-auto px-6 py-12">
    {#if currentSection === "overview"}
      <div class="grid md:grid-cols-2 gap-8">
        <!-- Introduction -->
        <section class="bg-slate-700/40 border border-slate-600 rounded-xl p-8">
          <h2 class="text-2xl font-bold text-white mb-4">Welcome</h2>
          <p class="text-slate-300 mb-4">
            This is a comprehensive UI component library for SyncSpace, designed
            with modern design principles and built using Svelte 5 runes and
            Tailwind CSS v4.
          </p>
          <p class="text-slate-300 mb-6">
            Every component follows atomic design methodology, ensuring
            consistency and reusability across your application.
          </p>
          <div class="flex gap-4">
            <Button href="#atoms" variant="primary">View Atoms</Button>
            <Button href="#molecules" variant="secondary">View Molecules</Button
            >
          </div>
        </section>

        <!-- Statistics -->
        <section class="grid grid-cols-2 gap-4">
          <div
            class="bg-gradient-to-br from-blue-500/20 to-blue-600/10 border border-blue-400/30 rounded-xl p-6 text-center"
          >
            <div class="text-4xl font-bold text-blue-400">20+</div>
            <div class="text-slate-300 mt-2">Components</div>
          </div>
          <div
            class="bg-gradient-to-br from-purple-500/20 to-purple-600/10 border border-purple-400/30 rounded-xl p-6 text-center"
          >
            <div class="text-4xl font-bold text-purple-400">100%</div>
            <div class="text-slate-300 mt-2">Customizable</div>
          </div>
          <div
            class="bg-gradient-to-br from-green-500/20 to-green-600/10 border border-green-400/30 rounded-xl p-6 text-center"
          >
            <div class="text-4xl font-bold text-green-400">5+</div>
            <div class="text-slate-300 mt-2">Variants</div>
          </div>
          <div
            class="bg-gradient-to-br from-orange-500/20 to-orange-600/10 border border-orange-400/30 rounded-xl p-6 text-center"
          >
            <div class="text-4xl font-bold text-orange-400">Svelte 5</div>
            <div class="text-slate-300 mt-2">Runes</div>
          </div>
        </section>
      </div>

      <!-- Features Grid -->
      <section class="mt-12">
        <h3 class="text-2xl font-bold text-white mb-8">Key Features</h3>
        <div class="grid md:grid-cols-3 gap-6">
          {#each [{ icon: "bi-lightning-fill", title: "Performant", desc: "Built with Svelte 5 runes for optimal reactivity" }, { icon: "bi-palette-fill", title: "Themeable", desc: "Complete design token system for easy customization" }, { icon: "bi-shield-check", title: "Accessible", desc: "WCAG compliant with semantic HTML and ARIA" }, { icon: "bi-puzzle-fill", title: "Composable", desc: "Atomic design pattern for flexible composition" }, { icon: "bi-type", title: "Typed", desc: "Full TypeScript support for type safety" }, { icon: "bi-bug-fill", title: "Tested", desc: "Thoroughly tested edge cases and variants" }] as feature (feature.title)}
            <div
              class="bg-slate-700/40 border border-slate-600 rounded-lg p-6 hover:border-blue-500 transition-colors"
            >
              <i class="bi {feature.icon} text-2xl text-blue-400 mb-3"></i>
              <h4 class="text-lg font-semibold text-white mb-2">
                {feature.title}
              </h4>
              <p class="text-slate-400 text-sm">{feature.desc}</p>
            </div>
          {/each}
        </div>
      </section>
    {/if}

    {#if currentSection === "atoms"}
      <div class="text-slate-300">
        <h2 class="text-2xl font-bold text-white mb-2">Atoms</h2>
        <p class="text-slate-400 mb-8">
          Basic building blocks: Button, Badge, Input, Checkbox, and more.
        </p>
        <p
          class="text-amber-400 bg-amber-500/10 border border-amber-500/30 rounded-lg p-4"
        >
          <i class="bi bi-info-circle mr-2"></i>
          Component showcase pages coming soon. View individual component files in
          <code class="bg-slate-800 px-2 py-1 rounded">/atoms/</code> directory.
        </p>
      </div>
    {/if}

    {#if currentSection === "molecules"}
      <div class="text-slate-300">
        <h2 class="text-2xl font-bold text-white mb-2">Molecules</h2>
        <p class="text-slate-400 mb-8">
          Complex combinations: Breadcrumbs, Toast, Filter, Select, ContextMenu.
        </p>
        <p
          class="text-amber-400 bg-amber-500/10 border border-amber-500/30 rounded-lg p-4"
        >
          <i class="bi bi-info-circle mr-2"></i>
          Component showcase pages coming soon. View individual component files in
          <code class="bg-slate-800 px-2 py-1 rounded">/molecules/</code> directory.
        </p>
      </div>
    {/if}

    {#if currentSection === "organisms"}
      <div class="text-slate-300">
        <h2 class="text-2xl font-bold text-white mb-2">Organisms</h2>
        <p class="text-slate-400 mb-8">
          Complex features: Modal dialogs, File viewers, and more.
        </p>
        <p
          class="text-amber-400 bg-amber-500/10 border border-amber-500/30 rounded-lg p-4"
        >
          <i class="bi bi-info-circle mr-2"></i>
          Component showcase pages coming soon. View individual component files in
          <code class="bg-slate-800 px-2 py-1 rounded">/organisms/</code> directory.
        </p>
      </div>
    {/if}
  </main>

  <!-- Footer -->
  <footer class="border-t border-slate-700 bg-slate-800/50 mt-16 py-8">
    <div class="max-w-7xl mx-auto px-6 text-center text-slate-400">
      <p>
        SyncSpace Component Library • Built with <span class="text-red-500"
          >♥</span
        > using Svelte 5 & Tailwind CSS v4
      </p>
    </div>
  </footer>
</div>

<style>
  :global(body) {
    @apply bg-slate-900;
  }
</style>
