<script>
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import { DEFAULT_SHORTCUTS, formatShortcut } from "../../lib/keyboardNavigation.js";

  const tr = $derive((key, ...args) => t($currentLang, key, ...args));

  let { isOpen = false, onClose = null } = $props();

  let os = $state("windows");

  // Detect OS
  $effect(() => {
    const ua = navigator.userAgent;
    if (ua.includes("Mac")) os = "mac";
    else if (ua.includes("Linux")) os = "linux";
    else os = "windows";
  });

  const shortcuts = {
    fileOperations: {
      title: "File Operations",
      shortcuts: [
        { name: "Copy", key: DEFAULT_SHORTCUTS.COPY },
        { name: "Cut", key: DEFAULT_SHORTCUTS.CUT },
        { name: "Paste", key: DEFAULT_SHORTCUTS.PASTE },
        { name: "Delete", key: DEFAULT_SHORTCUTS.DELETE },
        { name: "Rename", key: DEFAULT_SHORTCUTS.RENAME },
      ],
    },
    navigation: {
      title: "Navigation",
      shortcuts: [
        { name: "Upload", key: DEFAULT_SHORTCUTS.UPLOAD },
        { name: "Refresh", key: DEFAULT_SHORTCUTS.REFRESH },
        { name: "Search", key: DEFAULT_SHORTCUTS.SEARCH },
      ],
    },
    ui: {
      title: "User Interface",
      shortcuts: [
        { name: "Close/Cancel", key: DEFAULT_SHORTCUTS.ESCAPE },
        { name: "Navigate", key: DEFAULT_SHORTCUTS.TAB },
        { name: "Help", key: DEFAULT_SHORTCUTS.HELP },
      ],
    },
  };
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="shortcuts-title"
    ondragover={(e) => e.preventDefault()}
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl max-w-2xl w-full max-h-screen overflow-y-auto"
    >
      <!-- Header -->
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center sticky top-0 bg-white dark:bg-gray-800">
        <h2 id="shortcuts-title" class="text-2xl font-bold text-gray-900 dark:text-gray-100">
          Keyboard Shortcuts
        </h2>
        <button
          type="button"
          onclick={onClose}
          class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded transition-colors"
          aria-label="Close"
        >
          <i class="bi bi-x-lg text-xl"></i>
        </button>
      </div>

      <!-- OS Selector -->
      <div class="px-6 py-3 border-b border-gray-200 dark:border-gray-700 flex gap-2">
        {#each ["windows", "mac", "linux"] as osOption}
          <button
            type="button"
            onclick={() => (os = osOption)}
            class="px-3 py-1 rounded text-sm font-medium transition-colors"
            class:bg-blue-500={os === osOption}
            class:text-white={os === osOption}
            class:bg-gray-200={os !== osOption}
            class:dark:bg-gray-700={os !== osOption}
            class:text-gray-900={os !== osOption}
            class:dark:text-gray-100={os !== osOption}
          >
            {osOption.charAt(0).toUpperCase() + osOption.slice(1)}
          </button>
        {/each}
      </div>

      <!-- Shortcuts by Category -->
      <div class="px-6 py-4 space-y-8">
        {#each Object.entries(shortcuts) as [_key, category]}
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3">
              {category.title}
            </h3>

            <div class="space-y-2">
              {#each category.shortcuts as item}
                <div
                  class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded"
                >
                  <span class="text-gray-700 dark:text-gray-300">{item.name}</span>
                  <kbd
                    class="px-3 py-1 bg-gray-200 dark:bg-gray-600 text-gray-900 dark:text-gray-100 rounded font-mono text-sm border border-gray-300 dark:border-gray-500"
                  >
                    {formatShortcut(item.key, os)}
                  </kbd>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>

      <!-- Tips Section -->
      <div
        class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-blue-50 dark:bg-blue-900/20"
      >
        <h4 class="font-semibold text-gray-900 dark:text-gray-100 mb-2">
          💡 Tips
        </h4>
        <ul class="text-sm text-gray-700 dark:text-gray-300 space-y-1">
          <li>• Use arrow keys to navigate through files in the file browser</li>
          <li>• Press Escape to close any dialog or cancel operations</li>
          <li>• In file preview, use arrow keys to browse through files</li>
          <li>• All keyboard shortcuts can be customized in Settings</li>
        </ul>
      </div>

      <!-- Footer -->
      <div class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  kbd {
    display: inline-block;
    white-space: nowrap;
  }
</style>
