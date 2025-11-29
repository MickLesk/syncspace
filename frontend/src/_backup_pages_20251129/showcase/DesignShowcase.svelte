<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import PageWrapper from "../../components/PageWrapper.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

  let loadingButton = $state(false);

  function handleClick() {
    loadingButton = true;
    setTimeout(() => {
      loadingButton = false;
    }, 2000);
  }
</script>

<PageWrapper>
  <div class="page-fade-in">
    <!-- Page Header -->
    <div class="glass-header mb-8">
      <div class="flex items-center gap-4">
        <div class="logo-circle">
          <i class="bi bi-palette-fill text-4xl"></i>
        </div>
        <div>
          <h1 class="text-4xl font-bold gradient-text mb-2">
            {tr("designSystemShowcase")}
          </h1>
          <p class="text-gray-600 dark:text-gray-400 text-lg">
            {tr("modernUIComponentsDescription")}
          </p>
        </div>
      </div>
    </div>

    <!-- Buttons Section -->
    <div class="mb-12">
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
        <i class="bi bi-cursor-fill text-blue-500 mr-2"></i>
        Buttons
      </h2>

      <ModernCard variant="glass" padding="large">
        {#snippet children()}
          <div class="space-y-6">
            <!-- Primary Buttons -->
            <div>
              <h3
                class="text-lg font-semibold mb-4 text-gray-700 dark:text-gray-300"
              >
                Primary Variants
              </h3>
              <div class="flex flex-wrap gap-3">
                <ModernButton variant="primary" size="sm"
                  >Small Primary</ModernButton
                >
                <ModernButton variant="primary" size="md"
                  >Medium Primary</ModernButton
                >
                <ModernButton variant="primary" size="lg"
                  >Large Primary</ModernButton
                >
                <ModernButton variant="primary" icon="star-fill"
                  >With Icon</ModernButton
                >
                <ModernButton
                  variant="primary"
                  icon="arrow-right"
                  iconPosition="right">Icon Right</ModernButton
                >
                <ModernButton
                  variant="primary"
                  loading={loadingButton}
                  onclick={handleClick}
                >
                  {loadingButton ? "Loading..." : "Click Me"}
                </ModernButton>
              </div>
            </div>

            <!-- Gradient Buttons -->
            <div>
              <h3
                class="text-lg font-semibold mb-4 text-gray-700 dark:text-gray-300"
              >
                Gradient Variants
              </h3>
              <div class="flex flex-wrap gap-3">
                <ModernButton variant="gradient" icon="sparkles"
                  >Gradient Button</ModernButton
                >
                <ModernButton
                  variant="gradient"
                  size="lg"
                  icon="rocket-takeoff-fill">Launch</ModernButton
                >
                <ModernButton variant="gradient" fullWidth
                  >Full Width Button</ModernButton
                >
              </div>
            </div>

            <!-- Other Variants -->
            <div>
              <h3
                class="text-lg font-semibold mb-4 text-gray-700 dark:text-gray-300"
              >
                Other Variants
              </h3>
              <div class="flex flex-wrap gap-3">
                <ModernButton variant="secondary">Secondary</ModernButton>
                <ModernButton variant="ghost">Ghost</ModernButton>
                <ModernButton variant="success" icon="check-circle-fill"
                  >Success</ModernButton
                >
                <ModernButton variant="danger" icon="trash-fill"
                  >Danger</ModernButton
                >
                <ModernButton variant="primary" disabled>Disabled</ModernButton>
              </div>
            </div>
          </div>
        {/snippet}
      </ModernCard>
    </div>

    <!-- Cards Section -->
    <div class="mb-12">
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
        <i class="bi bi-grid-3x3-gap-fill text-purple-500 mr-2"></i>
        Cards
      </h2>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- Default Card -->
        <ModernCard
          title="Default Card"
          subtitle="Standard card with border"
          icon="square-fill"
          variant="default"
        >
          {#snippet children()}
            <p class="text-gray-600 dark:text-gray-400">
              This is a default card with a clean, minimal design.
            </p>
          {/snippet}
        </ModernCard>

        <!-- Glass Card -->
        <ModernCard
          title="Glass Card"
          subtitle="Glassmorphism effect"
          icon="droplet-fill"
          variant="glass"
        >
          {#snippet children()}
            <p class="text-gray-600 dark:text-gray-400">
              This card has a beautiful glassmorphism effect with backdrop blur.
            </p>
          {/snippet}
        </ModernCard>

        <!-- Gradient Card -->
        <ModernCard
          title="Gradient Card"
          subtitle="Subtle gradient background"
          icon="palette-fill"
          variant="gradient"
        >
          {#snippet children()}
            <p class="text-gray-600 dark:text-gray-400">
              This card features a subtle gradient background.
            </p>
          {/snippet}
        </ModernCard>

        <!-- Elevated Card -->
        <ModernCard
          title="Elevated Card"
          subtitle="With shadow elevation"
          icon="layers-fill"
          variant="elevated"
        >
          {#snippet children()}
            <p class="text-gray-600 dark:text-gray-400">
              This card has an elevated shadow for depth.
            </p>
          {/snippet}
        </ModernCard>

        <!-- Hoverable Card -->
        <ModernCard
          title="Hoverable Card"
          subtitle="Hover for animation"
          icon="hand-index-thumb-fill"
          variant="glass"
          hoverable={true}
        >
          {#snippet children()}
            <p class="text-gray-600 dark:text-gray-400">
              Hover over this card to see the lift animation.
            </p>
          {/snippet}
        </ModernCard>

        <!-- Clickable Card -->
        <ModernCard
          title="Clickable Card"
          subtitle="Click me!"
          icon="cursor-fill"
          variant="gradient"
          clickable={true}
          hoverable={true}
          onclick={() => alert("Card clicked!")}
        >
          {#snippet children()}
            <p class="text-gray-600 dark:text-gray-400">
              This card is interactive and can trigger actions.
            </p>
          {/snippet}
        </ModernCard>
      </div>
    </div>

    <!-- Colors & Badges Section -->
    <div class="mb-12">
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
        <i class="bi bi-paint-bucket text-pink-500 mr-2"></i>
        Colors & Badges
      </h2>

      <ModernCard variant="glass">
        {#snippet children()}
          <div class="space-y-6">
            <!-- Gradient Texts -->
            <div>
              <h3
                class="text-lg font-semibold mb-4 text-gray-700 dark:text-gray-300"
              >
                Gradient Text
              </h3>
              <div class="space-y-2">
                <p class="text-3xl font-bold gradient-text">
                  Primary Gradient Text
                </p>
                <p class="text-3xl font-bold gradient-text-rainbow">
                  Rainbow Gradient Text
                </p>
                <p class="text-3xl font-bold gradient-text-warm">
                  Warm Gradient Text
                </p>
              </div>
            </div>

            <!-- Status Badges -->
            <div>
              <h3
                class="text-lg font-semibold mb-4 text-gray-700 dark:text-gray-300"
              >
                Status Badges
              </h3>
              <div class="flex flex-wrap gap-3">
                <span class="badge-glass-success">Success</span>
                <span class="badge-glass-warning">Warning</span>
                <span class="badge-glass-error">Error</span>
                <span class="badge-glass-info">Info</span>
              </div>
            </div>
          </div>
        {/snippet}
      </ModernCard>
    </div>

    <!-- Animations Section -->
    <div class="mb-12">
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
        <i class="bi bi-stars text-yellow-500 mr-2"></i>
        Animations
      </h2>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <ModernCard title="Slide Up Animation" variant="glass">
          {#snippet children()}
            <div
              class="animate-slide-up p-8 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-lg text-center"
            >
              <i class="bi bi-arrow-up-circle-fill text-4xl text-blue-500 mb-2"
              ></i>
              <p class="text-gray-600 dark:text-gray-400">Slides up on load</p>
            </div>
          {/snippet}
        </ModernCard>

        <ModernCard title="Float Animation" variant="glass">
          {#snippet children()}
            <div
              class="p-8 bg-gradient-to-br from-green-50 to-blue-50 dark:from-green-900/20 dark:to-blue-900/20 rounded-lg text-center"
            >
              <i
                class="bi bi-cloud-fill text-4xl text-blue-500 mb-2 animate-float"
              ></i>
              <p class="text-gray-600 dark:text-gray-400">
                Gentle floating effect
              </p>
            </div>
          {/snippet}
        </ModernCard>
      </div>
    </div>

    <!-- Background Blobs Preview -->
    <ModernCard
      title="Background Blobs"
      subtitle="Animated gradient background elements"
      variant="glass"
    >
      {#snippet children()}
        <div
          class="relative h-64 bg-gradient-to-br from-purple-100 to-pink-100 dark:from-purple-900/20 dark:to-pink-900/20 rounded-lg overflow-hidden"
        >
          <div class="blob blob-1"></div>
          <div class="blob blob-2"></div>
          <div class="blob blob-3"></div>
          <div class="relative z-10 flex items-center justify-center h-full">
            <p class="text-xl font-bold text-gray-700 dark:text-gray-300">
              Animated floating blobs in background
            </p>
          </div>
        </div>
      {/snippet}
    </ModernCard>
  </div>
</PageWrapper>
