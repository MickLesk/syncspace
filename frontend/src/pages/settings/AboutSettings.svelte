<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
</script>

<div class="max-w-2xl mx-auto">
  <ModernCard variant="glass" class="animate-slide-up">
    <div class="p-8">
      <!-- Logo Section -->
      <div class="text-center mb-8">
        <div
          class="w-24 h-24 mx-auto mb-6 gradient-bg-primary rounded-full flex items-center justify-center shadow-lg"
        >
          <i class="bi bi-cloud-fill text-5xl text-white"></i>
        </div>

        <h1 class="text-4xl font-bold mb-2 gradient-text">SyncSpace</h1>
        <p class="text-lg text-gray-600 dark:text-gray-400 font-medium">
          {tr("secureCloudStorage")}
        </p>
      </div>

      <!-- Version Info -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
        <div
          class="text-center p-4 bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 rounded-xl border border-blue-200 dark:border-blue-800"
        >
          <div
            class="text-sm font-semibold text-blue-600 dark:text-blue-400 mb-1"
          >
            {tr("version")}
          </div>
          <div class="text-2xl font-bold text-blue-700 dark:text-blue-300">
            1.0.0
          </div>
          <div class="text-xs text-blue-600/70 dark:text-blue-400/70 mt-1">
            {tr("latestRelease")}
          </div>
        </div>

        <div
          class="text-center p-4 bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/20 dark:to-purple-800/20 rounded-xl border border-purple-200 dark:border-purple-800"
        >
          <div
            class="text-sm font-semibold text-purple-600 dark:text-purple-400 mb-1"
          >
            {tr("license")}
          </div>
          <div class="text-2xl font-bold text-purple-700 dark:text-purple-300">
            MIT
          </div>
          <div class="text-xs text-purple-600/70 dark:text-purple-400/70 mt-1">
            {tr("openSource")}
          </div>
        </div>

        <div
          class="text-center p-4 bg-gradient-to-br from-pink-50 to-pink-100 dark:from-pink-900/20 dark:to-pink-800/20 rounded-xl border border-pink-200 dark:border-pink-800"
        >
          <div
            class="text-sm font-semibold text-pink-600 dark:text-pink-400 mb-1"
          >
            {tr("build")}
          </div>
          <div class="text-2xl font-bold text-pink-700 dark:text-pink-300">
            2025.10
          </div>
          <div class="text-xs text-pink-600/70 dark:text-pink-400/70 mt-1">
            {tr("october2025")}
          </div>
        </div>
      </div>

      <!-- Tech Stack -->
      <div class="border-t border-gray-200 dark:border-gray-700 pt-6 mb-6">
        <p class="text-center text-sm text-gray-600 dark:text-gray-400 mb-2">
          {tr("builtWith")}
        </p>
        <p class="text-center text-xs text-gray-500 dark:text-gray-500">
          {tr("copyright")}
        </p>
      </div>

      <!-- Action Buttons -->
      <div class="grid grid-cols-2 gap-3">
        <ModernButton variant="secondary">
          <i class="bi bi-github mr-2"></i>
          GitHub
        </ModernButton>
        <ModernButton variant="gradient">
          <i class="bi bi-book mr-2"></i>
          Documentation
        </ModernButton>
      </div>
    </div>
  </ModernCard>
</div>
