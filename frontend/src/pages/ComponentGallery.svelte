<script>
  /**
   * Component Gallery - Tailwind v4 + HyperUI Showcase
   * Updated: October 27, 2025
   */
  import Card from "../components/ui/Card.svelte";

  let showModal = false;
</script>

<div class="max-w-7xl mx-auto px-4 py-8 space-y-12">
  <!-- Header -->
  <div class="text-center space-y-4">
    <h1
      class="text-4xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent"
    >
      Component Gallery
    </h1>
    <p class="text-lg text-gray-600 dark:text-gray-400">
      Tailwind CSS v4 + HyperUI Components Showcase
    </p>
  </div>

  <!-- Cards Section -->
  <section class="space-y-6">
    <h2 class="text-3xl font-bold text-gray-900 dark:text-white">Cards</h2>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <Card>
        {#snippet children()}
          <h3 class="text-xl font-semibold mb-2 text-gray-900 dark:text-white">
            Basic Card
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            A simple card with default styling and padding.
          </p>
        {/snippet}
      </Card>

      <Card hoverable>
        {#snippet children()}
          <h3 class="text-xl font-semibold mb-2 text-gray-900 dark:text-white">
            Hoverable Card
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            Hover over me to see the shadow effect!
          </p>
        {/snippet}
      </Card>

      <Card padding="lg" bordered={false}>
        {#snippet children()}
          <h3 class="text-xl font-semibold mb-2 text-gray-900 dark:text-white">
            No Border
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            Large padding without border.
          </p>
        {/snippet}
      </Card>
    </div>
  </section>

  <!-- Buttons Section -->
  <section class="space-y-6">
    <h2 class="text-3xl font-bold text-gray-900 dark:text-white">Buttons</h2>

    <Card>
      {#snippet children()}
        <h3 class="text-lg font-semibold mb-4 text-gray-900 dark:text-white">
          Button Variants
        </h3>
        <div class="flex flex-wrap gap-3">
          <button
            class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
          >
            Primary
          </button>
          <button
            class="px-4 py-2 bg-gray-200 text-gray-900 rounded-lg hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
          >
            Secondary
          </button>
          <button
            class="px-4 py-2 border-2 border-gray-300 rounded-lg hover:bg-gray-50 transition-colors dark:border-gray-600 dark:hover:bg-gray-800"
          >
            Outline
          </button>
          <button
            class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
          >
            Danger
          </button>
          <button
            class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
          >
            Success
          </button>
        </div>
      {/snippet}
    </Card>

    <Card>
      {#snippet children()}
        <h3 class="text-lg font-semibold mb-4 text-gray-900 dark:text-white">
          Button Sizes
        </h3>
        <div class="flex flex-wrap items-center gap-3">
          <button
            class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
          >
            Small
          </button>
          <button
            class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
          >
            Medium
          </button>
          <button
            class="px-6 py-3 text-lg bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
          >
            Large
          </button>
        </div>
      {/snippet}
    </Card>
  </section>

  <!-- Forms Section -->
  <section class="space-y-6">
    <h2 class="text-3xl font-bold text-gray-900 dark:text-white">
      Form Elements
    </h2>

    <Card>
      {#snippet children()}
        <h3 class="text-lg font-semibold mb-4 text-gray-900 dark:text-white">
          Inputs
        </h3>
        <div class="space-y-4 max-w-md">
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >
              Email Address
            </label>
            <input
              type="email"
              placeholder="you@example.com"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-800 dark:border-gray-700 dark:text-white"
            />
          </div>

          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >
              Password
            </label>
            <input
              type="password"
              placeholder="••••••••"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-800 dark:border-gray-700 dark:text-white"
            />
          </div>

          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >
              Message
            </label>
            <textarea
              rows="3"
              placeholder="Enter your message..."
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-800 dark:border-gray-700 dark:text-white"
            ></textarea>
          </div>
        </div>
      {/snippet}
    </Card>
  </section>

  <!-- Badges Section -->
  <section class="space-y-6">
    <h2 class="text-3xl font-bold text-gray-900 dark:text-white">
      Badges & Labels
    </h2>

    <Card>
      {#snippet children()}
        <h3 class="text-lg font-semibold mb-4 text-gray-900 dark:text-white">
          Status Badges
        </h3>
        <div class="flex flex-wrap gap-3">
          <span
            class="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm font-medium dark:bg-green-900 dark:text-green-200"
          >
            Active
          </span>
          <span
            class="px-3 py-1 bg-yellow-100 text-yellow-800 rounded-full text-sm font-medium dark:bg-yellow-900 dark:text-yellow-200"
          >
            Pending
          </span>
          <span
            class="px-3 py-1 bg-red-100 text-red-800 rounded-full text-sm font-medium dark:bg-red-900 dark:text-red-200"
          >
            Error
          </span>
          <span
            class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium dark:bg-blue-900 dark:text-blue-200"
          >
            Info
          </span>
          <span
            class="px-3 py-1 bg-gray-100 text-gray-800 rounded-full text-sm font-medium dark:bg-gray-700 dark:text-gray-200"
          >
            Default
          </span>
        </div>
      {/snippet}
    </Card>
  </section>

  <!-- Grid & Layout Section -->
  <section class="space-y-6">
    <h2 class="text-3xl font-bold text-gray-900 dark:text-white">Layouts</h2>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {#each Array(8) as _, i}
        <div
          class="bg-gradient-to-br from-indigo-500 to-purple-600 rounded-lg p-6 text-white"
        >
          <h4 class="text-lg font-semibold mb-2">Card {i + 1}</h4>
          <p class="text-indigo-100">Responsive grid layout</p>
        </div>
      {/each}
    </div>
  </section>

  <!-- Alerts Section -->
  <section class="space-y-6">
    <h2 class="text-3xl font-bold text-gray-900 dark:text-white">Alerts</h2>

    <div class="space-y-4">
      <div
        class="bg-green-50 border-l-4 border-green-500 p-4 rounded-r-lg dark:bg-green-900/20 dark:border-green-400"
      >
        <p class="text-green-800 dark:text-green-300 font-medium">
          Success! Your changes have been saved.
        </p>
      </div>

      <div
        class="bg-yellow-50 border-l-4 border-yellow-500 p-4 rounded-r-lg dark:bg-yellow-900/20 dark:border-yellow-400"
      >
        <p class="text-yellow-800 dark:text-yellow-300 font-medium">
          Warning: Please review your input.
        </p>
      </div>

      <div
        class="bg-red-50 border-l-4 border-red-500 p-4 rounded-r-lg dark:bg-red-900/20 dark:border-red-400"
      >
        <p class="text-red-800 dark:text-red-300 font-medium">
          Error: Something went wrong.
        </p>
      </div>

      <div
        class="bg-blue-50 border-l-4 border-blue-500 p-4 rounded-r-lg dark:bg-blue-900/20 dark:border-blue-400"
      >
        <p class="text-blue-800 dark:text-blue-300 font-medium">
          Info: New updates are available.
        </p>
      </div>
    </div>
  </section>
</div>
