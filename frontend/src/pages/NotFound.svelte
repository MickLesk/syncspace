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
            <div class="inline-flex items-center justify-center w-32 h-32 rounded-full bg-primary-100 dark:bg-primary-900/30 mb-4">
              <i class="bi bi-file-earmark-x-fill text-6xl text-primary-600 dark:text-primary-400"></i>
            </div>
            <div class="text-8xl font-bold gradient-text-primary">404</div>
          </div>

          <!-- Error Message -->
          <h1 class="text-4xl font-bold mb-4 text-gray-900 dark:text-white">
            {tr("pageNotFound")}
          </h1>
          <p class="text-lg text-gray-600 dark:text-gray-400 mb-8 max-w-md mx-auto">
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
              <a href="#/files" class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group">
                <i class="bi bi-folder-fill text-2xl text-blue-500 group-hover:scale-110 transition-transform"></i>
                <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{tr("files")}</span>
              </a>
              <a href="#/favorites" class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group">
                <i class="bi bi-star-fill text-2xl text-yellow-500 group-hover:scale-110 transition-transform"></i>
                <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{tr("favorites")}</span>
              </a>
              <a href="#/recent" class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group">
                <i class="bi bi-clock-fill text-2xl text-green-500 group-hover:scale-110 transition-transform"></i>
                <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{tr("recent")}</span>
              </a>
              <a href="#/settings" class="glass-card p-4 rounded-xl hover:scale-105 transition-transform flex flex-col items-center gap-2 group">
                <i class="bi bi-gear-fill text-2xl text-purple-500 group-hover:scale-110 transition-transform"></i>
                <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{tr("settings")}</span>
              </a>
            </div>
          </div>
        {/snippet}
      </ModernCard>
    </div>
  </div>
</PageWrapper>

<style>
  .not-found-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
    overflow: hidden;
    padding: 1rem;
  }

  /* Animated Background Blobs */
  .blob {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.4;
    animation: float 20s infinite ease-in-out;
  }

  .blob-1 {
    width: 500px;
    height: 500px;
    background: linear-gradient(135deg, #667eea, #764ba2);
    top: -10%;
    left: -10%;
    animation-delay: 0s;
  }

  .blob-2 {
    width: 400px;
    height: 400px;
    background: linear-gradient(135deg, #f093fb, #f5576c);
    top: 40%;
    right: -5%;
    animation-delay: 5s;
  }

  .blob-3 {
    width: 350px;
    height: 350px;
    background: linear-gradient(135deg, #4facfe, #00f2fe);
    bottom: -10%;
    left: 30%;
    animation-delay: 10s;
  }

  /* Glassmorphism Card */
  .error-card {
    position: relative;
    z-index: 10;
    width: 100%;
    max-width: 600px;
    padding: 3rem 2.5rem;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px) saturate(180%);
    border-radius: 24px;
    box-shadow:
      0 8px 32px 0 rgba(31, 38, 135, 0.15),
      0 0 0 1px rgba(255, 255, 255, 0.18) inset;
    animation: slideUp 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  /* Dark mode card */
  :global(.dark) .error-card {
    background: rgba(17, 24, 39, 0.95);
    box-shadow:
      0 8px 32px 0 rgba(0, 0, 0, 0.3),
      0 0 0 1px rgba(255, 255, 255, 0.08) inset;
  }

  /* Error Icon */
  .error-icon-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .error-icon {
    width: 150px;
    height: 150px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(
      135deg,
      rgba(102, 126, 234, 0.1),
      rgba(118, 75, 162, 0.1)
    );
    border-radius: 50%;
    animation: pulse 3s ease-in-out infinite;
  }

  .error-code {
    position: absolute;
    font-size: 10rem;
    font-weight: 900;
    background: linear-gradient(135deg, #667eea, #764ba2);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    opacity: 0.05;
    z-index: -1;
    pointer-events: none;
  }

  /* Quick Links */
  .quick-link {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    background: white;
    border: 2px solid rgb(229 231 235);
    border-radius: 12px;
    transition: all 0.2s;
    text-decoration: none;
    color: rgb(55 65 81);
    font-size: 0.875rem;
    font-weight: 500;
  }

  :global(.dark) .quick-link {
    background: rgb(31 41 55);
    border-color: rgb(55 65 81);
    color: rgb(209 213 219);
  }

  .quick-link:hover {
    transform: translateY(-2px);
    border-color: rgb(191 219 254);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
  }

  :global(.dark) .quick-link:hover {
    border-color: rgb(59 130 246);
  }

  .quick-link i {
    font-size: 1.5rem;
  }

  /* Animations */
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(60px) scale(0.9);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  @keyframes float {
    0%,
    100% {
      transform: translate(0, 0) rotate(0deg);
    }
    33% {
      transform: translate(30px, -30px) rotate(120deg);
    }
    66% {
      transform: translate(-20px, 20px) rotate(240deg);
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
      opacity: 0.8;
    }
  }

  /* Responsive */
  @media (max-width: 640px) {
    .error-card {
      padding: 2rem 1.5rem;
    }

    .error-icon {
      width: 120px;
      height: 120px;
    }

    .error-icon i {
      font-size: 4rem !important;
    }

    .error-code {
      font-size: 8rem;
    }
  }
</style>

