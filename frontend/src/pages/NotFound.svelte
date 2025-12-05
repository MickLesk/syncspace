<script>
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import PageWrapper from "../components/PageWrapper.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";

  function goHome() {
    window.location.hash = "#/files";
  }

  function goBack() {
    window.history.back();
  }
</script>

<PageWrapper fullHeight gradient>
  <div class="flex items-center justify-center min-h-screen">
    <div class="w-full max-w-2xl mx-auto text-center animate-slide-up">
      <ModernCard variant="glass" padding="large">
        {#snippet children()}
          <!-- 404 Icon -->
          <div class="mb-8">
            <div
              class="inline-flex items-center justify-center w-32 h-32 rounded-full bg-primary-100 dark:bg-primary-900/30 mb-4"
            >
              <i
                class="bi bi-file-earmark-x-fill text-6xl text-primary-600 dark:text-primary-400"
              ></i>
            </div>
            <div class="text-8xl font-bold gradient-text-primary">404</div>
          </div>

          <!-- Error Message -->
          <h1 class="text-4xl font-bold mb-4 text-gray-900 dark:text-white">
            {tr("pageNotFound")}
          </h1>
          <p
            class="text-lg text-gray-600 dark:text-gray-400 mb-8 max-w-md mx-auto"
          >
            {tr("pageNotFoundDescription")}
          </p>

          <!-- Action Buttons -->
          <div class="flex gap-4 flex-wrap justify-center mb-12">
            <ModernButton variant="gradient" icon="house-fill" onclick={goHome}>
              {tr("goHome")}
            </ModernButton>
            <ModernButton variant="ghost" icon="arrow-left" onclick={goBack}>
              {tr("goBack")}
            </ModernButton>
          </div>

          <!-- Quick Links -->
          <div class="pt-8 border-t border-gray-200 dark:border-gray-700">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
              {tr("quickLinks")}
            </p>
            <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
              <a
                href="#/files"
                class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group"
              >
                <i
                  class="bi bi-folder-fill text-2xl text-green-500 group-hover:scale-110 transition-transform"
                ></i>
                <span
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                  >{tr("files")}</span
                >
              </a>
              <a
                href="#/favorites"
                class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group"
              >
                <i
                  class="bi bi-star-fill text-2xl text-yellow-500 group-hover:scale-110 transition-transform"
                ></i>
                <span
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                  >{tr("favorites")}</span
                >
              </a>
              <a
                href="#/recent"
                class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group"
              >
                <i
                  class="bi bi-clock-fill text-2xl text-green-500 group-hover:scale-110 transition-transform"
                ></i>
                <span
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                  >{tr("recent")}</span
                >
              </a>
              <a
                href="#/settings"
                class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group"
              >
                <i
                  class="bi bi-gear-fill text-2xl text-purple-500 group-hover:scale-110 transition-transform"
                ></i>
                <span
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                  >{tr("settings")}</span
                >
              </a>
            </div>
          </div>
        {/snippet}
      </ModernCard>
    </div>
  </div>
</PageWrapper>

<style>
  /* Animation definitions */
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(40px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes pulse {
    0%,
    100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.05);
      opacity: 0.9;
    }
  }

  /* Global utility classes used in template */
  :global(.animate-slide-up) {
    animation: slideUp 0.6s ease-out;
  }

  :global(.gradient-text-primary) {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  :global(.glass-card) {
    backdrop-filter: blur(10px);
    background: rgba(255, 255, 255, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.3);
  }

  :global(.dark) :global(.glass-card) {
    background: rgba(45, 45, 60, 0.7);
    border-color: rgba(255, 255, 255, 0.1);
  }
</style>
