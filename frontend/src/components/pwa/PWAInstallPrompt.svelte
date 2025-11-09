<script>
  // PWAInstallPrompt.svelte - Handles PWA install prompts
  import { onMount } from 'svelte';
  import { offlineManager } from '../../stores/offlineManager';

  let installPromptEvent = null;
  let showInstallPrompt = $state(false);
  let isInstalled = $state(false);
  let isPWASupported = $state(false);

  onMount(() => {
    // Check if PWA is already installed
    if (window.matchMedia('(display-mode: standalone)').matches) {
      isInstalled = true;
    }

    // Check PWA support
    isPWASupported = 'serviceWorker' in navigator && 'PushManager' in window;

    // Handle beforeinstallprompt event
    window.addEventListener('beforeinstallprompt', (e) => {
      e.preventDefault();
      installPromptEvent = e;
      showInstallPrompt = true;

      console.log('[PWA] Install prompt available');
    });

    // Hide install button after installation
    window.addEventListener('appinstalled', () => {
      console.log('[PWA] App installed');
      isInstalled = true;
      showInstallPrompt = false;
      installPromptEvent = null;
    });

    // Initialize service worker and offline support
    if (isPWASupported) {
      offlineManager.registerServiceWorker();
      offlineManager.requestPersistentStorage();
    }
  });

  async function handleInstall() {
    if (!installPromptEvent) return;

    try {
      // Show the install prompt
      installPromptEvent.prompt();

      // Wait for user choice
      const { outcome } = await installPromptEvent.userChoice;

      if (outcome === 'accepted') {
        console.log('[PWA] User accepted install');
        showInstallPrompt = false;
      } else {
        console.log('[PWA] User dismissed install');
      }

      // Clear the event
      installPromptEvent = null;
    } catch (error) {
      console.error('[PWA] Install error:', error);
    }
  }

  function handleDismiss() {
    showInstallPrompt = false;
    // Could store dismissal preference if needed
  }
</script>

<!-- Install Prompt Banner -->
{#if showInstallPrompt && installPromptEvent && !isInstalled}
  <div class="sticky top-0 z-50 bg-gradient-to-r from-blue-600 to-blue-700 text-white px-4 py-4 shadow-lg">
    <div class="max-w-7xl mx-auto flex items-center justify-between gap-4">
      <!-- Content -->
      <div class="flex items-center gap-3 flex-1">
        <i class="bi bi-download text-2xl flex-shrink-0"></i>
        <div class="min-w-0">
          <p class="font-semibold text-sm md:text-base">
            Install SyncSpace
          </p>
          <p class="text-xs md:text-sm text-blue-100">
            Access your files offline and get app-like experience
          </p>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex gap-2 flex-shrink-0">
        <button
          on:click={handleInstall}
          class="px-4 py-2 bg-white text-blue-600 font-semibold rounded-lg hover:bg-blue-50 transition text-sm"
        >
          Install
        </button>
        <button
          on:click={handleDismiss}
          class="px-4 py-2 hover:bg-blue-500 rounded-lg transition text-sm font-medium"
        >
          Later
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- PWA Status Info (Debug mode) -->
{#if false}
  <!-- Hidden debug info -->
  <div class="text-xs text-gray-500 p-2">
    PWA Status: Supported={isPWASupported}, Installed={isInstalled}, HasPrompt={!!installPromptEvent}
  </div>
{/if}
