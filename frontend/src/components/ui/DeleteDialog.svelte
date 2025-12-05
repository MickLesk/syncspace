<script>
  /**
   * Delete Confirmation Dialog
   *
   * Shows confirmation dialog with risk assessment for file/folder deletion.
   * Displays item count, total size, and irreversibility warning for trash.
   *
   * Features:
   * - Item count and type display
   * - Total size calculation
   * - Folder content warning
   * - Irreversibility warning for permanent delete
   * - Keyboard shortcuts (Enter to confirm, Esc to cancel)
   * - Accessibility (ARIA, focus management)
   *
   * @component
   * @example
   *   <DeleteDialog
   *     items={[file1, file2]}
   *     isPermanent={false}
   *     onConfirm={handleDelete}
   *     onCancel={() => {}}
   *   />
   */

  import { onMount } from "svelte";
  import { t, currentLang } from "../i18n.js";

  // Props
  let {
    items = [],
    isPermanent = false,
    onConfirm = () => {},
    onCancel = () => {},
  } = $props();

  // State
  let isOpen = true;
  let isDeleting = false;
  let focusableElements = [];
  let initialFocus = null;

  /**
   * Calculate statistics
   */
  function getStats() {
    const totalSize = items.reduce(
      (sum, item) => sum + (item.size_bytes || 0),
      0
    );
    const fileCount = items.filter((i) => i.type === "file").length;
    const folderCount = items.filter((i) => i.type === "folder").length;
    const hasSubfolders = items.some((i) => i.type === "folder");

    return {
      totalSize,
      fileCount,
      folderCount,
      hasSubfolders,
      itemCount: items.length,
    };
  }

  /**
   * Format file size for display
   */
  function formatSize(bytes) {
    const units = ["B", "KB", "MB", "GB"];
    let size = bytes;
    let unitIdx = 0;

    while (size >= 1024 && unitIdx < units.length - 1) {
      size /= 1024;
      unitIdx++;
    }

    return `${size.toFixed(1)} ${units[unitIdx]}`;
  }

  /**
   * Handle confirm
   */
  async function handleConfirm() {
    isDeleting = true;
    try {
      await onConfirm();
      isOpen = false;
    } catch (error) {
      console.error("Delete failed:", error);
      isDeleting = false;
    }
  }

  /**
   * Handle cancel
   */
  function handleCancel() {
    isOpen = false;
    onCancel();
  }

  /**
   * Handle keyboard shortcuts
   */
  function handleKeyDown(e) {
    if (!isOpen) return;

    switch (e.key) {
      case "Enter":
        e.preventDefault();
        handleConfirm();
        break;
      case "Escape":
        e.preventDefault();
        handleCancel();
        break;
    }
  }

  /**
   * Manage focus trap
   */
  function updateFocusableElements() {
    const dialog = document.querySelector('[role="alertdialog"]');
    if (!dialog) return;

    focusableElements = Array.from(
      dialog.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      )
    );

    if (focusableElements.length > 0) {
      initialFocus = focusableElements[focusableElements.length - 2]; // Cancel button
      setTimeout(() => initialFocus?.focus(), 0);
    }
  }

  /**
   * Handle tab key for focus trap
   */
  function handleTabKey(e) {
    if (!isOpen || focusableElements.length === 0) return;

    const focusedIndex = focusableElements.indexOf(document.activeElement);

    if (e.shiftKey) {
      // Shift+Tab
      if (focusedIndex <= 0) {
        e.preventDefault();
        focusableElements[focusableElements.length - 1].focus();
      }
    } else {
      // Tab
      if (focusedIndex >= focusableElements.length - 1) {
        e.preventDefault();
        focusableElements[0].focus();
      }
    }
  }

  onMount(() => {
    updateFocusableElements();
    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keydown", handleTabKey);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keydown", handleTabKey);
    };
  });

  let stats = $derived(getStats());
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center z-[9999] p-4"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-lg shadow-2xl max-w-md w-full border border-gray-200 dark:border-gray-700"
      role="alertdialog"
      aria-labelledby="dialog-title"
      aria-describedby="dialog-description"
    >
      <!-- Header -->
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h2
          id="dialog-title"
          class="text-lg font-bold text-gray-900 dark:text-white flex items-center gap-2"
        >
          <i
            class="bi bi-exclamation-triangle text-red-600 text-xl"
            aria-hidden="true"
          ></i>
          {isPermanent ? "Endgültig löschen?" : "Löschen?"}
        </h2>
      </div>

      <!-- Content -->
      <div class="px-6 py-4 space-y-4">
        <!-- Item count -->
        <div id="dialog-description" class="text-sm">
          <p class="text-gray-700 dark:text-gray-300 mb-3">
            {#if stats.itemCount === 1}
              <strong>1 Element</strong> wird gelöscht:
            {:else}
              <strong>{stats.itemCount} Elemente</strong> werden gelöscht:
            {/if}
          </p>

          <!-- Item breakdown -->
          <div
            class="space-y-1 text-sm text-gray-600 dark:text-gray-400 bg-gray-50 dark:bg-gray-800 rounded p-3 mb-3"
          >
            {#if stats.fileCount > 0}
              <div class="flex items-center gap-2">
                <i
                  class="bi bi-file text-green-600 dark:text-green-400"
                  aria-hidden="true"
                ></i>
                {stats.fileCount}
                {stats.fileCount === 1 ? "Datei" : "Dateien"}
              </div>
            {/if}

            {#if stats.folderCount > 0}
              <div class="flex items-center gap-2">
                <i
                  class="bi bi-folder text-yellow-600 dark:text-yellow-400"
                  aria-hidden="true"
                ></i>
                {stats.folderCount}
                {stats.folderCount === 1 ? "Ordner" : "Ordner"}
              </div>
            {/if}

            {#if stats.totalSize > 0}
              <div
                class="flex items-center gap-2 pt-2 border-t border-gray-200 dark:border-gray-700"
              >
                <i
                  class="bi bi-hdd text-gray-600 dark:text-gray-400"
                  aria-hidden="true"
                ></i>
                Gesamtgröße: <strong>{formatSize(stats.totalSize)}</strong>
              </div>
            {/if}
          </div>

          <!-- Warnings -->
          <div class="space-y-2">
            {#if stats.hasSubfolders}
              <div
                class="flex gap-2 text-sm text-amber-700 dark:text-amber-400 bg-amber-50 dark:bg-amber-900/20 rounded p-2"
              >
                <i
                  class="bi bi-exclamation-circle flex-shrink-0 mt-0.5"
                  aria-hidden="true"
                ></i>
                <span
                  >Ordner enthalten weitere Dateien und Unterordner, die
                  ebenfalls gelöscht werden.</span
                >
              </div>
            {/if}

            {#if isPermanent}
              <div
                class="flex gap-2 text-sm text-red-700 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded p-2"
              >
                <i
                  class="bi bi-exclamation-circle-fill flex-shrink-0 mt-0.5"
                  aria-hidden="true"
                ></i>
                <span
                  ><strong>Warnung:</strong> Diese Aktion kann nicht rückgängig gemacht
                  werden. Die Dateien werden endgültig gelöscht.</span
                >
              </div>
            {:else}
              <div
                class="flex gap-2 text-sm text-green-700 dark:text-green-400 bg-green-50 dark:bg-green-900/20 rounded p-2"
              >
                <i
                  class="bi bi-info-circle flex-shrink-0 mt-0.5"
                  aria-hidden="true"
                ></i>
                <span
                  >Gelöschte Dateien landen im Papierkorb und können von dort
                  wiederhergestellt werden.</span
                >
              </div>
            {/if}
          </div>
        </div>

        <!-- File list preview (max 5 items) -->
        {#if items.length > 0}
          <div
            class="max-h-32 overflow-y-auto bg-gray-50 dark:bg-gray-800 rounded p-2"
          >
            <div
              class="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-1"
            >
              Dateien ({items.length}):
            </div>
            <div class="space-y-1">
              {#each items.slice(0, 5) as item (item.id || item.path)}
                <div
                  class="text-xs text-gray-700 dark:text-gray-300 flex items-center gap-1 truncate"
                >
                  <i
                    class="bi {item.type === 'folder'
                      ? 'bi-folder'
                      : 'bi-file'} flex-shrink-0 text-gray-500 dark:text-gray-500"
                  />
                  <span class="truncate">{item.name || item.path}</span>
                </div>
              {/each}
              {#if items.length > 5}
                <div
                  class="text-xs text-gray-500 dark:text-gray-400 italic pt-1"
                >
                  ... und {items.length - 5} weitere
                </div>
              {/if}
            </div>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex gap-2 justify-end"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors"
          onclick={handleCancel}
          disabled={isDeleting}
        >
          Abbrechen
        </button>

        <button
          class="px-4 py-2 text-sm font-medium text-white bg-red-600 hover:bg-red-700 dark:bg-red-700 dark:hover:bg-red-600 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          onclick={handleConfirm}
          disabled={isDeleting}
        >
          {#if isDeleting}
            <i class="bi bi-hourglass-split animate-spin" aria-hidden="true"
            ></i>
            Wird gelöscht...
          {:else}
            <i class="bi bi-trash" aria-hidden="true"></i>
            {isPermanent ? "Endgültig löschen" : "In Papierkorb"}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  :global(.animate-spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
