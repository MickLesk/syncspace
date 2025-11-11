<script lang="ts">
  import {
    typographyEmphasized,
    springs,
    shapeExpressive,
  } from "../shared/index.js";
  import Button from "../atoms/Button.svelte";
  import Card from "../atoms/Card.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Avatar from "../atoms/Avatar.svelte";

  let currentSection = $state("welcome");
</script>

<div
  class="min-h-screen bg-gradient-to-br from-white via-blue-50 to-purple-50 dark:from-gray-950 dark:via-blue-950/20 dark:to-purple-950/20"
>
  <div class="max-w-7xl mx-auto px-4 py-12 sm:px-6 lg:px-8">
    <!-- Hero Section -->
    <div class="text-center mb-16 space-y-6">
      <div
        class="inline-flex items-center gap-3 px-5 py-2.5 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-full shadow-lg shadow-blue-500/30 mb-6"
      >
        <i class="bi bi-lightning-charge-fill text-xl"></i>
        <span class={`${typographyEmphasized.label.large} tracking-wider`}
          >SYNCSPACE COMPONENTS</span
        >
        <Badge
          size="xs"
          color="secondary"
          class="bg-white/20 text-white border-none">v2.0</Badge
        >
      </div>

      <h1
        class={`${typographyEmphasized.display.large} bg-gradient-to-r from-gray-900 via-blue-900 to-purple-900 dark:from-white dark:via-blue-200 dark:to-purple-200 bg-clip-text text-transparent mb-6`}
      >
        Built with Svelte 5 & Tailwind v4
      </h1>

      <p
        class="text-xl text-gray-600 dark:text-gray-400 max-w-3xl mx-auto leading-relaxed"
      >
        A comprehensive <strong class="text-gray-900 dark:text-white"
          >Material 3 Expressive</strong
        > component library featuring spring physics, bold typography, and emotionally
        impactful design.
      </p>

      <div class="flex flex-wrap justify-center gap-4 pt-6">
        <Button
          variant="filled"
          color="primary"
          size="lg"
          emphasized={true}
          shapeStyle="extra-rounded"
          motion="bouncy"
          onclick={() => (currentSection = "atoms")}
        >
          <i class="bi bi-box mr-2"></i>
          Explore Components
        </Button>
        <Button
          variant="outlined"
          color="primary"
          size="lg"
          shapeStyle="extra-rounded"
          onclick={() => (currentSection = "expressive")}
        >
          <i class="bi bi-stars mr-2"></i>
          M3 Expressive
        </Button>
      </div>
    </div>

    <!-- Key Features Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-16">
      <Card
        emphasized={true}
        shapeStyle="extra-rounded"
        surfaceTone="container"
        elevationLevel={2}
        class="hover:scale-105 transition-transform duration-300"
      >
        <div class="p-6 text-center space-y-3">
          <div
            class="w-16 h-16 mx-auto rounded-2xl bg-blue-500/10 dark:bg-blue-500/20 flex items-center justify-center"
          >
            <i
              class="bi bi-speedometer2 text-3xl text-blue-600 dark:text-blue-400"
            ></i>
          </div>
          <h3
            class={`${typographyEmphasized.title.large} text-gray-900 dark:text-white`}
          >
            Spring Physics
          </h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Fluid animations with cubic-bezier spring motion
          </p>
        </div>
      </Card>

      <Card
        emphasized={true}
        shapeStyle="extra-rounded"
        surfaceTone="container"
        elevationLevel={2}
        class="hover:scale-105 transition-transform duration-300"
      >
        <div class="p-6 text-center space-y-3">
          <div
            class="w-16 h-16 mx-auto rounded-2xl bg-purple-500/10 dark:bg-purple-500/20 flex items-center justify-center"
          >
            <i
              class="bi bi-type-bold text-3xl text-purple-600 dark:text-purple-400"
            ></i>
          </div>
          <h3
            class={`${typographyEmphasized.title.large} text-gray-900 dark:text-white`}
          >
            Bold Typography
          </h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Emphasized fonts (700-900) for emotional impact
          </p>
        </div>
      </Card>

      <Card
        emphasized={true}
        shapeStyle="extra-rounded"
        surfaceTone="container"
        elevationLevel={2}
        class="hover:scale-105 transition-transform duration-300"
      >
        <div class="p-6 text-center space-y-3">
          <div
            class="w-16 h-16 mx-auto rounded-2xl bg-teal-500/10 dark:bg-teal-500/20 flex items-center justify-center"
          >
            <i class="bi bi-square text-3xl text-teal-600 dark:text-teal-400"
            ></i>
          </div>
          <h3
            class={`${typographyEmphasized.title.large} text-gray-900 dark:text-white`}
          >
            Shape Variety
          </h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            13 expressive shapes including squircle & extra-rounded
          </p>
        </div>
      </Card>

      <Card
        emphasized={true}
        shapeStyle="extra-rounded"
        surfaceTone="container"
        elevationLevel={2}
        class="hover:scale-105 transition-transform duration-300"
      >
        <div class="p-6 text-center space-y-3">
          <div
            class="w-16 h-16 mx-auto rounded-2xl bg-green-500/10 dark:bg-green-500/20 flex items-center justify-center"
          >
            <i
              class="bi bi-check2-circle text-3xl text-green-600 dark:text-green-400"
            ></i>
          </div>
          <h3
            class={`${typographyEmphasized.title.large} text-gray-900 dark:text-white`}
          >
            Production Ready
          </h3>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Fully tested, accessible, and optimized components
          </p>
        </div>
      </Card>
    </div>

    <!-- Stats Section -->
    <div class="mb-16">
      <Card
        emphasized={true}
        shapeStyle="extra-rounded"
        surfaceTone="primary"
        elevationLevel={3}
        class="overflow-hidden"
      >
        <div class="p-8">
          <h2
            class={`${typographyEmphasized.headline.large} text-white text-center mb-8`}
          >
            Powered by Research & Innovation
          </h2>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div class="text-center">
              <div
                class={`${typographyEmphasized.display.medium} text-white mb-2`}
              >
                60+
              </div>
              <div class="text-blue-100">Components</div>
              <div class="text-sm text-blue-200 mt-1">
                Atoms, Molecules & Organisms
              </div>
            </div>
            <div class="text-center">
              <div
                class={`${typographyEmphasized.display.medium} text-white mb-2`}
              >
                4x
              </div>
              <div class="text-blue-100">Faster</div>
              <div class="text-sm text-blue-200 mt-1">
                UI Comprehension (Research-backed)
              </div>
            </div>
            <div class="text-center">
              <div
                class={`${typographyEmphasized.display.medium} text-white mb-2`}
              >
                100%
              </div>
              <div class="text-blue-100">Accessible</div>
              <div class="text-sm text-blue-200 mt-1">
                WCAG 2.1 AA Compliant
              </div>
            </div>
          </div>
        </div>
      </Card>
    </div>

    <!-- Component Categories -->
    <div class="space-y-6">
      <h2
        class={`${typographyEmphasized.headline.large} text-gray-900 dark:text-white text-center mb-8`}
      >
        Component Architecture
      </h2>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <!-- Atoms -->
        <Card
          emphasized={true}
          shapeStyle="extra-rounded"
          surfaceTone="base"
          elevationLevel={2}
          class="group cursor-pointer transition-all duration-300 hover:scale-[1.02] hover:-translate-y-1"
          onclick={() => (currentSection = "atoms")}
        >
          <div class="p-8 space-y-4">
            <div
              class="w-14 h-14 rounded-xl bg-gradient-to-br from-blue-500 to-blue-600 flex items-center justify-center"
            >
              <i class="bi bi-box text-2xl text-white"></i>
            </div>
            <h3
              class={`${typographyEmphasized.title.large} text-gray-900 dark:text-white`}
            >
              Atoms
            </h3>
            <p class="text-gray-600 dark:text-gray-400 text-sm">
              Basic building blocks: Buttons, Inputs, Badges, Avatars, Cards &
              more
            </p>
            <div
              class="flex items-center gap-2 text-blue-600 dark:text-blue-400 text-sm font-semibold"
            >
              <span>Explore Atoms</span>
              <i
                class="bi bi-arrow-right group-hover:translate-x-1 transition-transform"
              ></i>
            </div>
          </div>
        </Card>

        <!-- Molecules -->
        <Card
          emphasized={true}
          shapeStyle="extra-rounded"
          surfaceTone="base"
          elevationLevel={2}
          class="group cursor-pointer transition-all duration-300 hover:scale-[1.02] hover:-translate-y-1"
          onclick={() => (currentSection = "molecules")}
        >
          <div class="p-8 space-y-4">
            <div
              class="w-14 h-14 rounded-xl bg-gradient-to-br from-purple-500 to-purple-600 flex items-center justify-center"
            >
              <i class="bi bi-grid-3x3 text-2xl text-white"></i>
            </div>
            <h3
              class={`${typographyEmphasized.title.large} text-gray-900 dark:text-white`}
            >
              Molecules
            </h3>
            <p class="text-gray-600 dark:text-gray-400 text-sm">
              Composite components: Tabs, Dropdown, Tooltips, Breadcrumbs & more
            </p>
            <div
              class="flex items-center gap-2 text-purple-600 dark:text-purple-400 text-sm font-semibold"
            >
              <span>Explore Molecules</span>
              <i
                class="bi bi-arrow-right group-hover:translate-x-1 transition-transform"
              ></i>
            </div>
          </div>
        </Card>

        <!-- Organisms -->
        <Card
          emphasized={true}
          shapeStyle="extra-rounded"
          surfaceTone="base"
          elevationLevel={2}
          class="group cursor-pointer transition-all duration-300 hover:scale-[1.02] hover:-translate-y-1"
          onclick={() => (currentSection = "organisms")}
        >
          <div class="p-8 space-y-4">
            <div
              class="w-14 h-14 rounded-xl bg-gradient-to-br from-teal-500 to-teal-600 flex items-center justify-center"
            >
              <i class="bi bi-layers text-2xl text-white"></i>
            </div>
            <h3
              class={`${typographyEmphasized.title.large} text-gray-900 dark:text-white`}
            >
              Organisms
            </h3>
            <p class="text-gray-600 dark:text-gray-400 text-sm">
              Complex patterns: Modal, Navbar, Sidebar, FileViewer & more
            </p>
            <div
              class="flex items-center gap-2 text-teal-600 dark:text-teal-400 text-sm font-semibold"
            >
              <span>Explore Organisms</span>
              <i
                class="bi bi-arrow-right group-hover:translate-x-1 transition-transform"
              ></i>
            </div>
          </div>
        </Card>
      </div>
    </div>

    <!-- Footer CTA -->
    <div class="mt-16 text-center space-y-6">
      <p class="text-gray-600 dark:text-gray-400">
        Built with ❤️ using <strong class="text-gray-900 dark:text-white"
          >Svelte 5</strong
        >
        & <strong class="text-gray-900 dark:text-white">Tailwind CSS v4</strong>
      </p>
      <div class="flex justify-center gap-4">
        <Button
          variant="text"
          color="primary"
          href="https://github.com/MickLesk/syncspace"
        >
          <i class="bi bi-github mr-2"></i>
          View on GitHub
        </Button>
        <Button
          variant="text"
          color="primary"
          onclick={() => (currentSection = "expressive")}
        >
          <i class="bi bi-book mr-2"></i>
          Documentation
        </Button>
      </div>
    </div>
  </div>
</div>
