<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    onFilesSelected,
    accept = "*",
    multiple = true,
    currentPath = "",
  } = $props();

  let isDragging = $state(false);
  let fileInput;
  let dragCounter = 0;

  function handleDragEnter(e) {
    e.preventDefault();
    dragCounter++;
    isDragging = true;
  }

  function handleDragLeave(e) {
    e.preventDefault();
    dragCounter--;
    if (dragCounter === 0) {
      isDragging = false;
    }
  }

  function handleDragOver(e) {
    e.preventDefault();
  }

  async function handleDrop(e) {
    e.preventDefault();
    isDragging = false;
    dragCounter = 0;

    const items = e.dataTransfer?.items;
    if (!items) return;

    const files = [];

    // Use webkitGetAsEntry API to handle folders
    for (let i = 0; i < items.length; i++) {
      const item = items[i].webkitGetAsEntry();
      if (item) {
        await traverseFileTree(item, "", files);
      }
    }

    if (files.length > 0) {
      onFilesSelected?.(files);
    }
  }

  // Recursively traverse file tree for folder uploads
  async function traverseFileTree(item, path, files) {
    if (item.isFile) {
      // Get the file
      const file = await new Promise((resolve) => {
        item.file((f) => {
          // Add relative path to file object
          Object.defineProperty(f, "relativePath", {
            value: path + f.name,
            writable: false,
          });
          resolve(f);
        });
      });
      files.push(file);
    } else if (item.isDirectory) {
      // Read directory contents
      const dirReader = item.createReader();
      const entries = await new Promise((resolve) => {
        dirReader.readEntries((entries) => resolve(entries));
      });

      // Recursively process subdirectories and files
      for (const entry of entries) {
        await traverseFileTree(entry, path + item.name + "/", files);
      }
    }
  }

  function handleFileSelect(e) {
    const files = Array.from(e.target.files || []);
    if (files.length > 0) {
      onFilesSelected?.(files);
    }
    // Reset input
    if (fileInput) fileInput.value = "";
  }

  function triggerFileInput() {
    fileInput?.click();
  }
</script>

<div
  role="region"
  aria-label="File upload drop zone"
  class="relative border-2 border-dashed rounded-lg p-8 transition-all min-h-[200px] flex items-center justify-center"
  class:border-green-500={isDragging}
  class:scale-[1.02]={isDragging}
  class:bg-green-50={isDragging}
  class:dark-bg-green-900={isDragging}
  class:border-gray-300={!isDragging}
  class:dark:border-gray-600={!isDragging}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondragover={handleDragOver}
  ondrop={handleDrop}
>
  <input
    bind:this={fileInput}
    type="file"
    {accept}
    {multiple}
    webkitdirectory={false}
    onchange={handleFileSelect}
    class="hidden"
    id="file-upload-input"
  />

  <div class="text-center">
    {#if isDragging}
      <i
        class="bi bi-download text-6xl text-green-500 mb-4 block animate-bounce"
        aria-hidden="true"
      ></i>
      <p class="text-xl font-semibold text-green-600 dark:text-green-400 mb-2">
        Drop files or folders here
      </p>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Release to upload to {currentPath || "root"}
      </p>
    {:else}
      <i
        class="bi bi-cloud-upload text-6xl text-gray-400 dark:text-gray-500 mb-4 block"
      ></i>
      <p class="text-lg font-semibold text-gray-700 dark:text-gray-200 mb-2">
        Drag & drop files or folders here
      </p>
      <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">or</p>
      <button
        type="button"
        class="btn btn-primary btn-sm"
        onclick={triggerFileInput}
      >
        <i class="bi bi-folder2-open" aria-hidden="true"></i>
        Browse Files
      </button>
    {/if}
  </div>
</div>
