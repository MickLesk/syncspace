<script>
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons.js";

  let {
    file,
    selected = false,
    viewMode = "grid",
    onSelect,
    onOpen,
    onContextMenu,
  } = $props();

  // getFileIcon returns just icon name like "folder-fill", need to add "bi bi-"
  function getIconClass(file) {
    const iconName = getFileIcon(file.name, file.is_directory);
    return `bi bi-${iconName}`;
  }

  function formatFileSize(bytes) {
    if (!bytes) return "0 B";
    const units = ["B", "KB", "MB", "GB"];
    let size = bytes;
    let unitIndex = 0;
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  }

  function formatDate(dateString) {
    if (!dateString) return "Unknown";
    const date = new Date(dateString);
    return date.toLocaleDateString() + " " + date.toLocaleTimeString();
  }
</script>

{#if viewMode === "grid"}
  <button
    type="button"
    class="file-card-grid p-4 bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-lg dark:shadow-gray-900/50 text-left w-full transition-all border-2"
    class:border-blue-500={selected}
    class:dark:border-blue-400={selected}
    class:border-transparent={!selected}
    class:hover:border-blue-300={!selected}
    class:dark:hover:border-blue-600={!selected}
    onclick={(e) => {
      if (e.shiftKey || e.ctrlKey) {
        onSelect?.(file);
      } else {
        onOpen?.(file);
      }
    }}
    oncontextmenu={(e) => {
      e.preventDefault();
      onContextMenu?.(file, e);
    }}
  >
    <div class="flex flex-col items-center gap-3 text-center">
      <i class="{getIconClass(file)} text-5xl {getFileIconColor(file.name)}"
      ></i>
      <div class="w-full min-w-0">
        <p
          class="font-medium truncate text-gray-900 dark:text-gray-100"
          title={file.name}
        >
          {file.name}
        </p>
        {#if !file.is_directory}
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {formatFileSize(file.size_bytes)}
          </p>
        {:else}
          <p class="text-sm text-gray-500 dark:text-gray-400">Folder</p>
        {/if}
        {#if file.modified_at}
          <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">
            {formatDate(file.modified_at)}
          </p>
        {/if}
      </div>
    </div>
  </button>
{:else}
  <button
    type="button"
    class="file-card-list p-3 bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-md text-left w-full transition-all border-2 flex items-center gap-4"
    class:border-blue-500={selected}
    class:dark:border-blue-400={selected}
    class:border-transparent={!selected}
    class:hover:border-blue-300={!selected}
    class:dark:hover:border-blue-600={!selected}
    onclick={(e) => {
      if (e.shiftKey || e.ctrlKey) {
        onSelect?.(file);
      } else {
        onOpen?.(file);
      }
    }}
    oncontextmenu={(e) => {
      e.preventDefault();
      onContextMenu?.(file, e);
    }}
  >
    <i
      class="{getIconClass(file)} text-3xl flex-shrink-0 {getFileIconColor(
        file.name
      )}"
    ></i>
    <div class="flex-1 min-w-0">
      <p class="font-medium truncate text-gray-900 dark:text-gray-100">
        {file.name}
      </p>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        {#if !file.is_directory}
          {formatFileSize(file.size_bytes)} • {formatDate(file.modified_at)}
        {:else}
          Folder • {formatDate(file.modified_at)}
        {/if}
      </p>
    </div>
    {#if selected}
      <i
        class="bi bi-check-circle-fill text-2xl text-blue-500 dark:text-blue-400"
      ></i>
    {/if}
  </button>
{/if}

<style>
  .file-card-grid:hover,
  .file-card-list:hover {
    transform: translateY(-2px);
  }

  .file-card-grid,
  .file-card-list {
    cursor: pointer;
    user-select: none;
  }
</style>
