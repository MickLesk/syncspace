<script>
  import { onMount, onDestroy } from "svelte";
  import LazyImage from "./LazyImage.svelte";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { file = null, files = [], onClose = null, isOpen = false } = $props();

  let currentIndex = $state(0);
  let currentFile = $derived(files[currentIndex] || file);
  let previewType = $state("unknown");
  let error = $state(null);

  onMount(() => {
    // Handle keyboard navigation
    function handleKeydown(e) {
      if (!isOpen) return;

      switch (e.key) {
        case "ArrowLeft":
          e.preventDefault();
          previousFile();
          break;
        case "ArrowRight":
          e.preventDefault();
          nextFile();
          break;
        case "Escape":
          e.preventDefault();
          onClose?.();
          break;
      }
    }

    document.addEventListener("keydown", handleKeydown);
    return () => document.removeEventListener("keydown", handleKeydown);
  });

  function getFileType(filename) {
    const ext = filename.split(".").pop()?.toLowerCase() || "";
    const imageExts = ["jpg", "jpeg", "png", "gif", "webp", "svg", "bmp"];
    const videoExts = ["mp4", "webm", "ogg", "mov", "avi"];
    const pdfExts = ["pdf"];
    const textExts = ["txt", "md", "json", "js", "css", "html", "xml", "csv"];
    const docExts = ["docx", "doc"];
    const sheetExts = ["xlsx", "xls"];

    if (imageExts.includes(ext)) return "image";
    if (videoExts.includes(ext)) return "video";
    if (pdfExts.includes(ext)) return "pdf";
    if (textExts.includes(ext)) return "text";
    if (docExts.includes(ext)) return "document";
    if (sheetExts.includes(ext)) return "spreadsheet";
    return "unknown";
  }

  function nextFile() {
    if (currentIndex < files.length - 1) {
      currentIndex++;
      error = null;
    }
  }

  function previousFile() {
    if (currentIndex > 0) {
      currentIndex--;
      error = null;
    }
  }

  $effect(() => {
    if (currentFile) {
      previewType = getFileType(currentFile.name);
    }
  });
</script>

{#if isOpen && currentFile}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-90 p-4"
    role="dialog"
    aria-modal="true"
    tabindex="0"
    ondragover={(e) => e.preventDefault()}
    ondrop={(e) => e.preventDefault()}
  >
    <!-- Close Button -->
    <button
      type="button"
      class="absolute top-4 right-4 p-2 text-white hover:bg-white/20 rounded-lg transition-colors z-50"
      aria-label="Close"
      onclick={onClose}
      title="Close (ESC)"><i class="bi bi-x" aria-hidden="true"></i></button
    >

    <!-- Main Preview Container -->
    <div
      class="flex flex-col items-center justify-center max-w-4xl max-h-screen"
      ondragover={(e) => e.preventDefault()}
      role="region"
      aria-label="File preview area"
    >
      <!-- Preview Content -->
      <div
        class="w-full max-h-96 overflow-auto rounded-lg bg-black flex items-center justify-center"
      >
        {#if error}
          <div class="text-red-400 text-center p-8">
            <i
              class="bi bi-exclamation-triangle-fill text-4xl block mb-2"
              aria-hidden="true"
            ></i>
            <p>{error}</p>
          </div>
        {:else if previewType === "image"}
          <LazyImage
            src={`/api/files/${currentFile.path || currentFile.name}`}
            alt={currentFile.name}
            class="max-w-full max-h-96 object-contain"
            loading="eager"
          />
        {:else if previewType === "video"}
          <video class="max-w-full max-h-96 object-contain" controls autoplay>
            <source
              src={`/api/files/${currentFile.path || currentFile.name}`}
              type={`video/${currentFile.name.split(".").pop()}`}
            />
            <track kind="captions" label="No captions available" />
            Your browser does not support the video tag.
          </video>
        {:else if previewType === "pdf"}
          <div class="text-white text-center p-8">
            <i class="bi bi-file-pdf text-4xl block mb-2" aria-hidden="true"
            ></i>
            <p>{currentFile.name}</p>
            <a
              href={`/api/files/${currentFile.path || currentFile.name}`}
              target="_blank"
              class="mt-4 inline-block px-4 py-2 bg-green-500 hover:bg-green-600 rounded transition-colors"
            >
              Open PDF
            </a>
          </div>
        {:else if previewType === "text"}
          <div
            class="text-white p-8 w-full max-h-96 overflow-auto bg-gray-900 rounded"
          >
            <p class="text-xs text-gray-400 mb-2">{currentFile.name}</p>
            <pre
              class="font-mono text-sm whitespace-pre-wrap">{currentFile.content ||
                "Loading..."}</pre>
          </div>
        {:else if previewType === "document"}
          <div class="text-white text-center p-8">
            <i class="bi bi-file-word text-4xl block mb-2" aria-hidden="true"
            ></i>
            <p>{currentFile.name}</p>
            <p class="text-sm text-gray-400 mt-2">
              Document preview coming soon
            </p>
          </div>
        {:else if previewType === "spreadsheet"}
          <div class="text-white text-center p-8">
            <i class="bi bi-file-excel text-4xl block mb-2" aria-hidden="true"
            ></i>
            <p>{currentFile.name}</p>
            <p class="text-sm text-gray-400 mt-2">
              Spreadsheet preview coming soon
            </p>
          </div>
        {:else}
          <div class="text-white text-center p-8">
            <i class="bi bi-file text-4xl block mb-2" aria-hidden="true"></i>
            <p>{currentFile.name}</p>
            <p class="text-sm text-gray-400 mt-2">Preview not available</p>
          </div>
        {/if}
      </div>

      <!-- File Info & Navigation -->
      <div class="mt-4 w-full">
        <!-- File Info -->
        <div class="text-white text-center text-sm mb-4">
          <p class="font-medium">{currentFile.name}</p>
          <p class="text-gray-400">
            {currentIndex + 1} / {files.length}
          </p>
        </div>

        <!-- Navigation Controls -->
        <div class="flex justify-center gap-4">
          <button
            type="button"
            disabled={currentIndex === 0}
            onclick={previousFile}
            class="px-4 py-2 bg-gray-700 hover:bg-gray-600 disabled:bg-gray-800 text-white rounded transition-colors"
          >
            <i class="bi bi-chevron-left" aria-hidden="true"></i> Previous
          </button>

          <button
            type="button"
            disabled={currentIndex === files.length - 1}
            onclick={nextFile}
            class="px-4 py-2 bg-gray-700 hover:bg-gray-600 disabled:bg-gray-800 text-white rounded transition-colors"
          >
            Next <i class="bi bi-chevron-right" aria-hidden="true"></i>
          </button>

          <button
            type="button"
            onclick={() => {
              const a = document.createElement("a");
              a.href = `/api/files/${currentFile.path || currentFile.name}`;
              a.download = currentFile.name;
              a.click();
            }}
            class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors"
          >
            <i class="bi bi-download" aria-hidden="true"></i> Download
          </button>
        </div>
      </div>
    </div>

    <!-- Keyboard Help Text -->
    <div class="absolute bottom-4 left-4 text-white text-sm text-gray-400">
      <p>← → Arrow keys | ESC to close</p>
    </div>
  </div>
{/if}

<style>
  :global([role="dialog"]) {
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>
