<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = (key, ...args) => t($currentLang, key, ...args);

  // Props
  let { isOpen = $bindable(false), onClose = () => {} } = $props();

  const shortcuts = [
    {
      category: tr("shortcuts.navigation"),
      items: [
        { keys: ["Ctrl", "K"], description: tr("shortcuts.commandPalette") },
        { keys: ["Ctrl", "?"], description: tr("shortcuts.helpMenu") },
        { keys: ["Ctrl", "1"], description: tr("shortcuts.filesView") },
        { keys: ["Ctrl", "2"], description: tr("shortcuts.searchView") },
        { keys: ["Ctrl", "3"], description: tr("shortcuts.tagsView") },
        { keys: ["Ctrl", "4"], description: tr("shortcuts.favoritesView") },
        { keys: ["Ctrl", "5"], description: tr("shortcuts.trashView") },
      ],
    },
    {
      category: tr("shortcuts.fileOperations"),
      items: [
        { keys: ["Ctrl", "U"], description: tr("shortcuts.uploadFile") },
        {
          keys: ["Ctrl", "Shift", "N"],
          description: tr("shortcuts.newFolder"),
        },
        { keys: ["Ctrl", "D"], description: tr("shortcuts.deleteFile") },
        { keys: ["Ctrl", "C"], description: tr("shortcuts.copyFile") },
        { keys: ["Ctrl", "X"], description: tr("shortcuts.cutFile") },
        { keys: ["Ctrl", "V"], description: tr("shortcuts.pasteFile") },
        { keys: ["Del"], description: tr("shortcuts.softDelete") },
        {
          keys: ["Ctrl", "Shift", "Del"],
          description: tr("shortcuts.permanentDelete"),
        },
      ],
    },
    {
      category: tr("shortcuts.search"),
      items: [
        { keys: ["Ctrl", "F"], description: tr("shortcuts.quickSearch") },
        {
          keys: ["Ctrl", "Shift", "F"],
          description: tr("shortcuts.advancedSearch"),
        },
        {
          keys: ["Ctrl", "Alt", "F"],
          description: tr("shortcuts.fullTextSearch"),
        },
        { keys: ["Ctrl", "L"], description: tr("shortcuts.searchByTag") },
      ],
    },
    {
      category: tr("shortcuts.display"),
      items: [
        {
          keys: ["Ctrl", "Shift", "T"],
          description: tr("shortcuts.toggleTheme"),
        },
        { keys: ["Ctrl", "+"], description: tr("shortcuts.zoomIn") },
        { keys: ["Ctrl", "-"], description: tr("shortcuts.zoomOut") },
        { keys: ["Ctrl", "0"], description: tr("shortcuts.resetZoom") },
        { keys: ["F11"], description: tr("shortcuts.fullscreen") },
      ],
    },
    {
      category: tr("shortcuts.editor"),
      items: [
        { keys: ["Ctrl", "S"], description: tr("shortcuts.saveFile") },
        { keys: ["Ctrl", "Z"], description: tr("shortcuts.undo") },
        { keys: ["Ctrl", "Shift", "Z"], description: tr("shortcuts.redo") },
        { keys: ["Ctrl", "F"], description: tr("shortcuts.findInEditor") },
        { keys: ["Ctrl", "H"], description: tr("shortcuts.findReplace") },
      ],
    },
  ];
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/50 z-40 flex items-center justify-center p-4"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-hidden flex flex-col border border-gray-200 dark:border-gray-700"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700"
      >
        <h2
          class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-3"
        >
          <i class="bi bi-keyboard text-indigo-500" aria-hidden="true"></i>
          {tr("shortcuts.title")}
        </h2>
        <button
          onclick={() => {
            isOpen = false;
            onClose?.();
          }}
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition"
        >
          <i class="bi bi-x text-2xl" aria-hidden="true"></i>
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {#each shortcuts as section}
            <div>
              <h3
                class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
              >
                <span class="w-1 h-1 rounded-full bg-indigo-500"></span>
                {section.category}
              </h3>

              <div class="space-y-3">
                {#each section.items as shortcut}
                  <div class="flex items-center justify-between">
                    <p class="text-gray-700 dark:text-gray-300 text-sm">
                      {shortcut.description}
                    </p>
                    <div class="flex gap-1 flex-shrink-0">
                      {#each shortcut.keys as key}
                        <kbd
                          class="px-2 py-1 text-xs font-medium rounded-md bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 text-gray-900 dark:text-white"
                        >
                          {key}
                        </kbd>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/each}
        </div>

        <!-- Tip -->
        <div
          class="mt-8 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg"
        >
          <div class="flex gap-3">
            <i
              class="bi bi-lightbulb text-blue-600 dark:text-blue-400 text-lg flex-shrink-0 mt-0.5"
            ></i>
            <div>
              <p class="text-sm font-medium text-blue-900 dark:text-blue-200">
                {tr("shortcuts.tip")}
              </p>
              <p class="text-xs text-blue-800 dark:text-blue-300 mt-1">
                {tr("shortcuts.tipDesc")}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3"
      >
        <button
          onclick={() => {
            isOpen = false;
            onClose?.();
          }}
          class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition font-medium"
        >
          {tr("close")}
        </button>
      </div>
    </div>
  </div>
{/if}
