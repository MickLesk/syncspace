<script>
  import { onMount, onDestroy } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { auth } from "../../stores/auth.js";

  const tr = (key, ...args) => t($currentLang, key, ...args);

  // Props
  let { isOpen = $bindable(false), onCommandSelect = () => {} } = $props();

  // State
  let searchQuery = $state("");
  let selectedIndex = $state(0);
  let inputElement = $state(null);

  // Available commands (categories)
  const commands = $derived.by(() => [
    {
      category: tr("commandPalette.file"),
      items: [
        {
          id: "new-folder",
          label: tr("commandPalette.newFolder"),
          description: tr("commandPalette.newFolderDesc"),
          icon: "bi-folder-plus",
          shortcut: "Ctrl+Shift+N",
          action: () => {
            onCommandSelect({ type: "new-folder" });
          },
        },
        {
          id: "upload-file",
          label: tr("commandPalette.uploadFile"),
          description: tr("commandPalette.uploadFileDesc"),
          icon: "bi-cloud-upload",
          shortcut: "Ctrl+U",
          action: () => {
            onCommandSelect({ type: "upload-file" });
          },
        },
        {
          id: "bulk-delete",
          label: tr("commandPalette.bulkDelete"),
          description: tr("commandPalette.bulkDeleteDesc"),
          icon: "bi-trash3",
          action: () => {
            onCommandSelect({ type: "bulk-delete" });
          },
        },
      ],
    },
    {
      category: tr("commandPalette.search"),
      items: [
        {
          id: "search-files",
          label: tr("commandPalette.searchFiles"),
          description: tr("commandPalette.searchFilesDesc"),
          icon: "bi-search",
          shortcut: "Ctrl+F",
          action: () => {
            onCommandSelect({ type: "search" });
          },
        },
        {
          id: "advanced-search",
          label: tr("commandPalette.advancedSearch"),
          description: tr("commandPalette.advancedSearchDesc"),
          icon: "bi-funnel",
          shortcut: "Ctrl+Shift+F",
          action: () => {
            onCommandSelect({ type: "advanced-search" });
          },
        },
        {
          id: "saved-searches",
          label: tr("commandPalette.savedSearches"),
          description: tr("commandPalette.savedSearchesDesc"),
          icon: "bi-bookmark",
          action: () => {
            onCommandSelect({ type: "saved-searches" });
          },
        },
      ],
    },
    {
      category: tr("commandPalette.view"),
      items: [
        {
          id: "files-view",
          label: tr("commandPalette.filesView"),
          description: tr("commandPalette.filesViewDesc"),
          icon: "bi-folder",
          shortcut: "Ctrl+1",
          action: () => {
            onCommandSelect({ type: "navigate", view: "files" });
          },
        },
        {
          id: "search-view",
          label: tr("commandPalette.searchView"),
          description: tr("commandPalette.searchViewDesc"),
          icon: "bi-search",
          shortcut: "Ctrl+2",
          action: () => {
            onCommandSelect({ type: "navigate", view: "search" });
          },
        },
        {
          id: "tag-cloud",
          label: tr("commandPalette.tagCloud"),
          description: tr("commandPalette.tagCloudDesc"),
          icon: "bi-tags",
          shortcut: "Ctrl+3",
          action: () => {
            onCommandSelect({ type: "navigate", view: "tag-cloud" });
          },
        },
        {
          id: "favorites-view",
          label: tr("commandPalette.favorites"),
          description: tr("commandPalette.favoritesDesc"),
          icon: "bi-star",
          shortcut: "Ctrl+4",
          action: () => {
            onCommandSelect({ type: "navigate", view: "favorites" });
          },
        },
        {
          id: "trash-view",
          label: tr("commandPalette.trash"),
          description: tr("commandPalette.trashDesc"),
          icon: "bi-trash",
          shortcut: "Ctrl+5",
          action: () => {
            onCommandSelect({ type: "navigate", view: "trash" });
          },
        },
        {
          id: "activity-view",
          label: tr("commandPalette.activity"),
          description: tr("commandPalette.activityDesc"),
          icon: "bi-clock-history",
          shortcut: "Ctrl+6",
          action: () => {
            onCommandSelect({ type: "navigate", view: "activity" });
          },
        },
      ],
    },
    {
      category: tr("commandPalette.settings"),
      items: [
        {
          id: "user-settings",
          label: tr("commandPalette.userSettings"),
          description: tr("commandPalette.userSettingsDesc"),
          icon: "bi-gear",
          action: () => {
            onCommandSelect({ type: "navigate", view: "settings" });
          },
        },
        {
          id: "theme-toggle",
          label: tr("commandPalette.toggleTheme"),
          description: tr("commandPalette.toggleThemeDesc"),
          icon: "bi-moon",
          shortcut: "Ctrl+Shift+T",
          action: () => {
            onCommandSelect({ type: "toggle-theme" });
          },
        },
        {
          id: "help",
          label: tr("commandPalette.help"),
          description: tr("commandPalette.helpDesc"),
          icon: "bi-question-circle",
          shortcut: "Ctrl+?",
          action: () => {
            onCommandSelect({ type: "help" });
          },
        },
      ],
    },
  ]);

  // Filter commands based on search query
  const filteredCommands = $derived.by(() => {
    const query = searchQuery.toLowerCase().trim();
    if (!query) return commands;

    return commands
      .map((category) => ({
        ...category,
        items: category.items.filter(
          (item) =>
            item.label.toLowerCase().includes(query) ||
            item.description.toLowerCase().includes(query) ||
            item.id.toLowerCase().includes(query)
        ),
      }))
      .filter((category) => category.items.length > 0);
  });

  // Get all filtered items in flat array for navigation
  const allFilteredItems = $derived(
    filteredCommands.flatMap((cat) => cat.items)
  );

  onMount(() => {
    if (isOpen) {
      setTimeout(() => inputElement?.focus(), 0);
    }

    const handleKeyDown = (e) => {
      // Global command palette trigger
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        isOpen = !isOpen;
      }

      if (!isOpen) return;

      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          selectedIndex = Math.min(
            selectedIndex + 1,
            allFilteredItems.length - 1
          );
          break;
        case "ArrowUp":
          e.preventDefault();
          selectedIndex = Math.max(selectedIndex - 1, 0);
          break;
        case "Enter":
          e.preventDefault();
          if (allFilteredItems[selectedIndex]) {
            allFilteredItems[selectedIndex].action?.();
            isOpen = false;
            searchQuery = "";
          }
          break;
        case "Escape":
          e.preventDefault();
          isOpen = false;
          searchQuery = "";
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  });

  // Reset index when filtered items change
  $effect(() => {
    selectedIndex = 0;
  });

  // Close when isOpen becomes false
  $effect(() => {
    if (isOpen) {
      searchQuery = "";
      selectedIndex = 0;
    }
  });
</script>

<!-- Backdrop -->
{#if isOpen}
  <div
    class="fixed inset-0 bg-black/50 z-40"
    onclick={() => (isOpen = false)}
  ></div>
{/if}

<!-- Command Palette Modal -->
{#if isOpen}
  <div
    class="fixed top-20 left-1/2 transform -translate-x-1/2 w-full max-w-2xl z-50"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl overflow-hidden border border-gray-200 dark:border-gray-700"
    >
      <!-- Search Input -->
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center gap-3">
          <i class="bi bi-search text-gray-400 text-lg"></i>
          <input
            bind:this={inputElement}
            bind:value={searchQuery}
            type="text"
            placeholder={tr("commandPalette.placeholder")}
            class="flex-1 bg-transparent text-lg outline-none text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500"
          />
          <span class="text-xs text-gray-400 dark:text-gray-500">
            ESC {tr("commandPalette.close")}
          </span>
        </div>
      </div>

      <!-- Commands List -->
      <div class="max-h-96 overflow-y-auto">
        {#if allFilteredItems.length === 0}
          <div class="p-8 text-center">
            <i
              class="bi bi-search text-3xl text-gray-300 dark:text-gray-600 mb-2 block"
            ></i>
            <p class="text-gray-500 dark:text-gray-400">
              {tr("commandPalette.noResults")}
            </p>
          </div>
        {:else}
          {#each filteredCommands as category}
            {#if category.items.length > 0}
              <div>
                <!-- Category Header -->
                <div
                  class="px-4 py-2 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase bg-gray-50 dark:bg-gray-700/50"
                >
                  {category.category}
                </div>

                <!-- Category Items -->
                {#each category.items as item, idx}
                  {@const globalIdx = allFilteredItems.indexOf(item)}
                  <button
                    class="w-full px-4 py-3 flex items-start gap-3 transition-colors {globalIdx ===
                    selectedIndex
                      ? 'bg-indigo-500 text-white'
                      : 'hover:bg-gray-50 dark:hover:bg-gray-700/50 text-gray-900 dark:text-white'}"
                    onclick={() => {
                      item.action?.();
                      isOpen = false;
                      searchQuery = "";
                    }}
                    onmouseenter={() => (selectedIndex = globalIdx)}
                  >
                    <!-- Icon -->
                    <div class="flex-shrink-0 mt-0.5">
                      <i
                        class="bi {item.icon} text-lg {globalIdx ===
                        selectedIndex
                          ? 'text-white'
                          : 'text-indigo-500'}"
                      ></i>
                    </div>

                    <!-- Content -->
                    <div class="flex-1 text-left">
                      <div class="font-medium text-sm">{item.label}</div>
                      <div class="text-xs opacity-75 mt-0.5">
                        {item.description}
                      </div>
                    </div>

                    <!-- Shortcut -->
                    {#if item.shortcut}
                      <div
                        class="flex-shrink-0 text-xs font-semibold px-2 py-1 rounded {globalIdx ===
                        selectedIndex
                          ? 'bg-white/20'
                          : 'bg-gray-100 dark:bg-gray-700'}"
                      >
                        {item.shortcut}
                      </div>
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          {/each}
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="px-4 py-2 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700/50 text-xs text-gray-500 dark:text-gray-400"
      >
        {tr("commandPalette.tip")}
      </div>
    </div>
  </div>
{/if}

<style>
  :global(.command-palette-portal) {
    z-index: 50;
  }
</style>
