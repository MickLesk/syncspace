<script>
  import { onMount, onDestroy } from "svelte";
  import {
    commandPaletteOpen,
    closeCommandPalette,
    searchActions,
    executeCommand,
    recentCommands,
    actionRegistry,
    commandCategories,
  } from "../../stores/commandPalette";
  import { t, currentLang } from "../../lib/i18n";

  let searchQuery = $state("");
  let selectedIndex = $state(0);
  let inputElement;
  let actions = $state([]);
  let recentCommandIds = $state([]);

  // Reactive: Update actions when search query changes
  $effect(() => {
    if ($commandPaletteOpen) {
      actions = searchActions(searchQuery);
      selectedIndex = 0; // Reset selection
    }
  });

  // Reactive: Load recent commands
  $effect(() => {
    recentCommandIds = $recentCommands;
  });

  // Get recent actions
  function getRecentActions() {
    const allActions = $actionRegistry;
    return recentCommandIds
      .map((id) => allActions.find((a) => a.id === id))
      .filter(Boolean)
      .slice(0, 5);
  }

  // Handle keyboard events
  function handleKeydown(event) {
    if (!$commandPaletteOpen) return;

    switch (event.key) {
      case "Escape":
        event.preventDefault();
        closeCommandPalette();
        break;

      case "ArrowDown":
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, actions.length - 1);
        scrollToSelected();
        break;

      case "ArrowUp":
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        scrollToSelected();
        break;

      case "Enter":
        event.preventDefault();
        if (actions.length > 0 && selectedIndex < actions.length) {
          executeCommand(actions[selectedIndex].id);
        }
        break;
    }
  }

  // Scroll to selected item
  function scrollToSelected() {
    const listElement = document.getElementById("command-palette-list");
    const selectedElement = document.getElementById(
      `command-item-${selectedIndex}`
    );

    if (listElement && selectedElement) {
      const listRect = listElement.getBoundingClientRect();
      const selectedRect = selectedElement.getBoundingClientRect();

      if (selectedRect.bottom > listRect.bottom) {
        selectedElement.scrollIntoView({ block: "end", behavior: "smooth" });
      } else if (selectedRect.top < listRect.top) {
        selectedElement.scrollIntoView({ block: "start", behavior: "smooth" });
      }
    }
  }

  // Handle backdrop click
  function handleBackdropClick(event) {
    if (event.target === event.currentTarget) {
      closeCommandPalette();
    }
  }

  // Focus input when palette opens
  $effect(() => {
    if ($commandPaletteOpen && inputElement) {
      setTimeout(() => inputElement?.focus(), 50);
    }
  });

  // Reset on close
  $effect(() => {
    if (!$commandPaletteOpen) {
      searchQuery = "";
      selectedIndex = 0;
    }
  });

  // Global keyboard listener
  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });

  // Get category info
  function getCategoryInfo(categoryKey) {
    return commandCategories[categoryKey] || commandCategories.general;
  }

  // Format shortcut for display
  function formatShortcut(shortcut) {
    return shortcut
      .replace("Ctrl", "⌃")
      .replace("Cmd", "⌘")
      .replace("Alt", "⌥")
      .replace("Shift", "⇧");
  }
</script>

{#if $commandPaletteOpen}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 z-[9998] backdrop-blur-sm"
    onclick={handleBackdropClick}
    role="button"
    tabindex="-1"
    aria-label="Close command palette"
  ></div>

  <!-- Command Palette Modal -->
  <div
    class="fixed inset-x-0 top-20 mx-auto max-w-2xl z-[9999] px-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="command-palette-title"
    tabindex="0"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden"
    >
      <!-- Search Input -->
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <div class="relative">
          <i
            class="bi bi-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          ></i>
          <input
            bind:this={inputElement}
            bind:value={searchQuery}
            type="text"
            placeholder={t($currentLang, "commandPalettePlaceholder")}
            class="w-full pl-10 pr-4 py-2 bg-gray-50 dark:bg-gray-900 border border-gray-300 dark:border-gray-600 rounded-md text-gray-900 dark:text-gray-100 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-green-500"
          />
          <div
            class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-gray-400"
          >
            ESC {t($currentLang, "toClose")}
          </div>
        </div>
      </div>

      <!-- Results List -->
      <div
        id="command-palette-list"
        class="max-h-96 overflow-y-auto"
        role="listbox"
      >
        {#if searchQuery === "" && recentCommandIds.length > 0}
          <!-- Recent Commands -->
          <div class="p-2">
            <div
              class="px-3 py-2 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider"
            >
              {t($currentLang, "recentCommands")}
            </div>
            {#each getRecentActions() as action, index (action.id)}
              {@const categoryInfo = getCategoryInfo(action.category)}
              <button
                id="command-item-{index}"
                class="w-full text-left px-3 py-2 rounded-md transition-colors {selectedIndex ===
                index
                  ? 'bg-green-50 dark:bg-green-900/30'
                  : 'hover:bg-gray-50 dark:hover:bg-gray-700'}"
                onclick={() => executeCommand(action.id)}
                onmouseenter={() => (selectedIndex = index)}
                role="option"
                aria-selected={selectedIndex === index}
              >
                <div class="flex items-center gap-3">
                  <i
                    class="bi {categoryInfo.icon} text-{categoryInfo.color}-500 text-lg"
                  ></i>
                  <div class="flex-1 min-w-0">
                    <div class="font-medium text-gray-900 dark:text-gray-100">
                      {action.label}
                    </div>
                    {#if action.description}
                      <div
                        class="text-xs text-gray-500 dark:text-gray-400 truncate"
                      >
                        {action.description}
                      </div>
                    {/if}
                  </div>
                  {#if action.shortcuts && action.shortcuts.length > 0}
                    <div
                      class="text-xs font-mono text-gray-400 dark:text-gray-500"
                    >
                      {formatShortcut(action.shortcuts[0])}
                    </div>
                  {/if}
                </div>
              </button>
            {/each}
          </div>
        {/if}

        {#if actions.length > 0}
          <!-- Search Results -->
          <div class="p-2">
            {#if searchQuery !== ""}
              <div
                class="px-3 py-2 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider"
              >
                {t($currentLang, "searchResults")} ({actions.length})
              </div>
            {/if}
            {#each actions as action, index (action.id)}
              {@const categoryInfo = getCategoryInfo(action.category)}
              <button
                id="command-item-{index}"
                class="w-full text-left px-3 py-2 rounded-md transition-colors {selectedIndex ===
                index
                  ? 'bg-green-50 dark:bg-green-900/30'
                  : 'hover:bg-gray-50 dark:hover:bg-gray-700'}"
                onclick={() => executeCommand(action.id)}
                onmouseenter={() => (selectedIndex = index)}
                role="option"
                aria-selected={selectedIndex === index}
              >
                <div class="flex items-center gap-3">
                  <i
                    class="bi {categoryInfo.icon} text-{categoryInfo.color}-500 text-lg"
                  ></i>
                  <div class="flex-1 min-w-0">
                    <div class="font-medium text-gray-900 dark:text-gray-100">
                      {action.label}
                    </div>
                    {#if action.description}
                      <div
                        class="text-xs text-gray-500 dark:text-gray-400 truncate"
                      >
                        {action.description}
                      </div>
                    {/if}
                  </div>
                  {#if action.shortcuts && action.shortcuts.length > 0}
                    <div
                      class="text-xs font-mono text-gray-400 dark:text-gray-500"
                    >
                      {formatShortcut(action.shortcuts[0])}
                    </div>
                  {/if}
                </div>
              </button>
            {/each}
          </div>
        {:else if searchQuery !== ""}
          <!-- No Results -->
          <div class="p-8 text-center text-gray-500 dark:text-gray-400">
            <i class="bi bi-search text-4xl mb-2" aria-hidden="true"></i>
            <div class="font-medium">{t($currentLang, "noCommandsFound")}</div>
            <div class="text-sm">{t($currentLang, "tryDifferentSearch")}</div>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="px-4 py-3 bg-gray-50 dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700 flex items-center justify-between text-xs text-gray-500 dark:text-gray-400"
      >
        <div class="flex items-center gap-4">
          <span>
            <kbd
              class="px-2 py-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded"
              >↑↓</kbd
            >
            {t($currentLang, "navigate")}
          </span>
          <span>
            <kbd
              class="px-2 py-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded"
              >↵</kbd
            >
            {t($currentLang, "select")}
          </span>
          <span>
            <kbd
              class="px-2 py-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded"
              >ESC</kbd
            >
            {t($currentLang, "close")}
          </span>
        </div>
        <button
          onclick={() => {
            /* TODO: Open shortcuts cheatsheet */
          }}
          class="hover:text-green-500 transition-colors"
        >
          <i class="bi bi-keyboard" aria-hidden="true"></i>
          {t($currentLang, "viewShortcuts")}
        </button>
      </div>
    </div>
  </div>
{/if}
