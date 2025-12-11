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
            <div
              class="text-8xl font-bold bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-400 bg-clip-text text-transparent"
            >
              404
            </div>
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
                class="p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group backdrop-blur-sm bg-white/70 dark:bg-gray-800/70 border border-white/30 dark:border-white/10"
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
                class="p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group backdrop-blur-sm bg-white/70 dark:bg-gray-800/70 border border-white/30 dark:border-white/10"
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
                class="p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group backdrop-blur-sm bg-white/70 dark:bg-gray-800/70 border border-white/30 dark:border-white/10"
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
                class="p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group backdrop-blur-sm bg-white/70 dark:bg-gray-800/70 border border-white/30 dark:border-white/10"
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
