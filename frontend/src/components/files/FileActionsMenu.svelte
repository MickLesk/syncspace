<script>
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
    onVersionHistory,
    onPreview,
    onChangeFolderColor,
  } = $props();

  let menuRef;

  function handleClickOutside(e) {
    if (menuRef && !menuRef.contains(e.target)) {
      onClose?.();
    }
  }

  function handleAction(action) {
    action();
    onClose?.();
  }

  $effect(() => {
    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  });
</script>

<div
  bind:this={menuRef}
  class="context-menu fixed bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 py-2 z-50 min-w-48"
  style="top: {position.y}px; left: {position.x}px;"
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
        <i class="bi bi-eye text-blue-500"></i>
        <span>Preview</span>
      </button>
    {/if}

    {#if !file.is_directory && onDownload}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onDownload)}
      >
        <i class="bi bi-download text-green-500"></i>
        <span>Download</span>
      </button>
    {/if}

    {#if onRename}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onRename)}
      >
        <i class="bi bi-pencil text-yellow-500"></i>
        <span>Rename</span>
      </button>
    {/if}

    {#if onMove}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onMove)}
      >
        <i class="bi bi-arrow-right-square text-purple-500"></i>
        <span>Move</span>
      </button>
    {/if}

    {#if onCopy}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onCopy)}
      >
        <i class="bi bi-files text-cyan-500"></i>
        <span>Copy</span>
      </button>
    {/if}

    {#if file.is_directory && onChangeFolderColor}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onChangeFolderColor)}
      >
        <i class="bi bi-palette text-pink-500"></i>
        <span>Change Folder Color</span>
      </button>
    {/if}

    <div class="divider my-1"></div>

    {#if !file.is_directory && onShare}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onShare)}
      >
        <i class="bi bi-share text-indigo-500"></i>
        <span>Share</span>
      </button>
    {/if}

    {#if !file.is_directory && onVersionHistory}
      <button
        type="button"
        class="menu-item w-full px-4 py-2 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3"
        onclick={() => handleAction(onVersionHistory)}
      >
        <i class="bi bi-clock-history text-teal-500"></i>
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
        <i class="bi bi-trash"></i>
        <span>Delete</span>
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
