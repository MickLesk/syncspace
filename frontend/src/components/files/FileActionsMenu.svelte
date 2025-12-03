<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    file,
    position = { x: 0, y: 0 },
    onClose,
    onRename,
    onDelete,
    onMove,
    onCopy,
    onShare,
    onDownload,
    onEdit,
    onVersionHistory,
    onPreview,
    onChangeFolderColor,
  } = $props();

  let menuRef;
  let adjustedPosition = $derived({ x: position.x, y: position.y });

  function handleClickOutside(e) {
    if (menuRef && !menuRef.contains(e.target)) {
      onClose?.();
    }
  }

  function handleAction(action) {
    action();
    onClose?.();
  }

  // Adjust position to keep menu in viewport
  $effect(() => {
    if (menuRef) {
      const rect = menuRef.getBoundingClientRect();
      const viewportHeight = window.innerHeight;
      const viewportWidth = window.innerWidth;

      let { x, y } = position;

      // Check if menu goes off bottom of screen
      if (y + rect.height > viewportHeight) {
        // Flip menu upward
        y = Math.max(10, y - rect.height);
      }

      // Check if menu goes off right of screen
      if (x + rect.width > viewportWidth) {
        // Move menu to the left
        x = Math.max(10, viewportWidth - rect.width - 10);
      }

      // Check if menu goes off top of screen
      if (y < 10) {
        y = 10;
      }

      // Check if menu goes off left of screen
      if (x < 10) {
        x = 10;
      }

      adjustedPosition = { x, y };
    }
  });

  $effect(() => {
    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  });
</script>

<div
  bind:this={menuRef}
  class="context-menu fixed bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 py-2 z-[9999] min-w-48"
  style="top: {adjustedPosition.y}px; left: {adjustedPosition.x}px;"
>
  <div class="px-3 py-2 border-b border-gray-200 dark:border-gray-700">
    <p class="font-semibold text-sm text-gray-700 dark:text-gray-200 truncate">
      {file.name}
    </p>
    <p class="text-xs text-gray-500 dark:text-gray-400">
      {file.is_directory ? "Folder" : "File"}
    </p>
  </div>

  <div class="py-1">
    {#if !file.is_directory && onPreview}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onPreview)}
      >
        <i class="bi bi-eye text-green-500" aria-hidden="true"></i>
        <span>{tr("preview")}</span>
      </button>
    {/if}

    {#if !file.is_directory && onDownload}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onDownload)}
      >
        <i class="bi bi-download text-green-600" aria-hidden="true"></i>
        <span>{tr("download")}</span>
      </button>
    {/if}

    {#if !file.is_directory && onEdit}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onEdit)}
      >
        <i class="bi bi-file-earmark-code text-blue-500" aria-hidden="true"></i>
        <span>{tr("editFile")}</span>
      </button>
    {/if}

    {#if onRename}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onRename)}
      >
        <i class="bi bi-pencil text-yellow-500" aria-hidden="true"></i>
        <span>{tr("rename")}</span>
      </button>
    {/if}

    {#if onMove}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onMove)}
      >
        <i class="bi bi-arrow-right-square text-purple-500" aria-hidden="true"
        ></i>
        <span>{tr("move")}</span>
      </button>
    {/if}

    {#if onCopy}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onCopy)}
      >
        <i class="bi bi-files text-cyan-500" aria-hidden="true"></i>
        <span>{tr("copy")}</span>
      </button>
    {/if}

    {#if file.is_directory && onChangeFolderColor}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onChangeFolderColor)}
      >
        <i class="bi bi-palette text-pink-500" aria-hidden="true"></i>
        <span>{tr("changeFolderColor")}</span>
      </button>
    {/if}

    <div class="divider my-1"></div>

    {#if !file.is_directory && onShare}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onShare)}
      >
        <i class="bi bi-share text-indigo-500" aria-hidden="true"></i>
        <span>{tr("share")}</span>
      </button>
    {/if}

    {#if !file.is_directory && onVersionHistory}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onVersionHistory)}
      >
        <i class="bi bi-clock-history text-teal-500" aria-hidden="true"></i>
        <span>Version History</span>
      </button>
    {/if}

    <div class="divider my-1"></div>

    {#if onDelete}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-red-50 dark:hover:bg-red-900/20 flex items-center gap-3 text-red-600 dark:text-red-400"
        onclick={() => handleAction(onDelete)}
      >
        <i class="bi bi-trash" aria-hidden="true"></i>
        <span>{tr("delete")}</span>
      </button>
    {/if}
  </div>
</div>

<style>
  .context-menu {
    animation: fadeIn 0.15s ease-out;
    max-width: 320px;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .menu-item {
    transition: all 0.1s ease;
    user-select: none;
  }

  .divider {
    height: 1px;
    background: currentColor;
    opacity: 0.1;
    margin: 0.25rem 0.5rem;
  }
</style>
