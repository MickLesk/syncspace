<script>
  import { onMount, onDestroy } from "svelte";
  import Modal from "../ui/Modal.svelte";
  import { getFileIcon } from "../../utils/fileIcons";
  import api from "../../lib/api";

  let {
    visible = $bindable(false),
    file = $bindable(null),
    allFiles = [],
    currentIndex = 0,
  } = $props();

  let fileContent = $state("");
  let blobUrl = $state(null);
  let loading = $state(false);
  let error = $state(null);

  // Image Controls
  let zoomLevel = $state(1);
  let rotation = $state(0);
  let isFullscreen = $state(false);

  // Keyboard handler
  let keyboardHandler = null;

  // Cleanup blob URL when modal closes or file changes
  $effect(() => {
    if (!visible && blobUrl) {
      URL.revokeObjectURL(blobUrl);
      blobUrl = null;
      fileContent = "";
      error = null;
      resetControls();
    }
  });

  // Reset image controls
  function resetControls() {
    zoomLevel = 1;
    rotation = 0;
    isFullscreen = false;
  }

  // Zoom controls
  function zoomIn() {
    zoomLevel = Math.min(zoomLevel + 0.25, 5);
  }

  function zoomOut() {
    zoomLevel = Math.max(zoomLevel - 0.25, 0.25);
  }

  function resetZoom() {
    zoomLevel = 1;
  }

  // Rotation controls
  function rotateLeft() {
    rotation = (rotation - 90) % 360;
  }

  function rotateRight() {
    rotation = (rotation + 90) % 360;
  }

  // Navigation
  function goToPrevious() {
    if (allFiles.length > 0 && currentIndex > 0) {
      currentIndex--;
      file = allFiles[currentIndex];
      resetControls();
    }
  }

  function goToNext() {
    if (allFiles.length > 0 && currentIndex < allFiles.length - 1) {
      currentIndex++;
      file = allFiles[currentIndex];
      resetControls();
    }
  }

  // Keyboard shortcuts
  function setupKeyboardShortcuts() {
    keyboardHandler = (e) => {
      if (!visible) return;

      switch (e.key) {
        case "Escape":
          close();
          break;
        case "ArrowLeft":
          e.preventDefault();
          goToPrevious();
          break;
        case "ArrowRight":
          e.preventDefault();
          goToNext();
          break;
        case "+":
        case "=":
          e.preventDefault();
          zoomIn();
          break;
        case "-":
        case "_":
          e.preventDefault();
          zoomOut();
          break;
        case "0":
          e.preventDefault();
          resetZoom();
          break;
        case "r":
          e.preventDefault();
          rotateRight();
          break;
        case "f":
          e.preventDefault();
          toggleFullscreen();
          break;
      }
    };

    window.addEventListener("keydown", keyboardHandler);
  }

  function toggleFullscreen() {
    isFullscreen = !isFullscreen;
  }

  onMount(() => {
    setupKeyboardShortcuts();
  });

  onDestroy(() => {
    if (keyboardHandler) {
      window.removeEventListener("keydown", keyboardHandler);
    }
    if (blobUrl) {
      URL.revokeObjectURL(blobUrl);
    }
  });

  async function loadFileContent() {
    if (!file) return;

    loading = true;
    error = null;

    try {
      const backendPath = file.path || file.name;
      const blob = await api.files.download(backendPath);

      // Create blob URL for media files
      if (
        fileType === "image" ||
        fileType === "video" ||
        fileType === "audio" ||
        fileType === "pdf"
      ) {
        if (blobUrl) URL.revokeObjectURL(blobUrl);
        blobUrl = URL.createObjectURL(blob);
      } else if (fileType === "text") {
        // Read text content
        const text = await blob.text();
        fileContent = text.substring(0, 10000); // First 10k chars
      }
    } catch (err) {
      console.error("Failed to load file:", err);
      error = err.message || "Failed to load file preview";
    } finally {
      loading = false;
    }
  }

  // Load file when modal becomes visible
  $effect(() => {
    if (visible && file) {
      loadFileContent();
    }
  });

  function close() {
    visible = false;
  }

  function getFileType(filename) {
    if (!filename) return "other";
    const ext = filename.split(".").pop()?.toLowerCase() || "";

    // Images
    if (["jpg", "jpeg", "png", "gif", "webp", "bmp", "svg"].includes(ext)) {
      return "image";
    }

    // Videos
    if (["mp4", "avi", "mkv", "mov", "webm", "flv"].includes(ext)) {
      return "video";
    }

    // Audio
    if (["mp3", "wav", "flac", "aac", "ogg", "m4a"].includes(ext)) {
      return "audio";
    }

    // PDF
    if (ext === "pdf") {
      return "pdf";
    }

    // Text/Code
    if (
      [
        "txt",
        "md",
        "json",
        "js",
        "ts",
        "css",
        "html",
        "xml",
        "csv",
        "log",
        "py",
        "java",
        "c",
        "cpp",
        "rs",
        "go",
        "sh",
      ].includes(ext)
    ) {
      return "text";
    }

    return "other";
  }

  let fileType = $derived(file ? getFileType(file.name) : null);
</script>

{#if file}
  <Modal
    {visible}
    title={file?.name || "File Preview"}
    icon="eye"
    size="xl"
    on:close={close}
  >
    <div class="preview-container">
      {#if loading}
        <div class="flex justify-center items-center h-64">
          <span class="loading loading-spinner loading-lg text-primary"></span>
        </div>
      {:else if error}
        <div class="alert alert-error">
          <i class="bi bi-exclamation-triangle"></i>
          <span>{error}</span>
        </div>
      {:else if fileType === "image" && blobUrl}
        <div
          class="flex justify-center bg-slate-50 dark:bg-slate-800 rounded-lg p-4 relative overflow-hidden {isFullscreen
            ? 'fixed inset-0 z-[100] bg-black'
            : ''}"
        >
          <!-- Image Controls Toolbar -->
          <div
            class="absolute top-4 right-4 z-10 flex gap-2 bg-black/50 backdrop-blur-sm rounded-lg p-2"
          >
            <button
              onclick={zoomOut}
              class="btn btn-sm btn-circle glass text-white hover:bg-white/20"
              title="Zoom Out (-)"
            >
              <i class="bi bi-zoom-out"></i>
            </button>
            <button
              onclick={resetZoom}
              class="btn btn-sm glass text-white hover:bg-white/20"
              title="Reset Zoom (0)"
            >
              {Math.round(zoomLevel * 100)}%
            </button>
            <button
              onclick={zoomIn}
              class="btn btn-sm btn-circle glass text-white hover:bg-white/20"
              title="Zoom In (+)"
            >
              <i class="bi bi-zoom-in"></i>
            </button>
            <div class="divider divider-horizontal m-0"></div>
            <button
              onclick={rotateLeft}
              class="btn btn-sm btn-circle glass text-white hover:bg-white/20"
              title="Rotate Left"
            >
              <i class="bi bi-arrow-counterclockwise"></i>
            </button>
            <button
              onclick={rotateRight}
              class="btn btn-sm btn-circle glass text-white hover:bg-white/20"
              title="Rotate Right (R)"
            >
              <i class="bi bi-arrow-clockwise"></i>
            </button>
            <div class="divider divider-horizontal m-0"></div>
            <button
              onclick={toggleFullscreen}
              class="btn btn-sm btn-circle glass text-white hover:bg-white/20"
              title="Fullscreen (F)"
            >
              <i class="bi bi-{isFullscreen ? 'fullscreen-exit' : 'fullscreen'}"
              ></i>
            </button>
          </div>

          <!-- Navigation Arrows -->
          {#if allFiles.length > 1}
            <button
              onclick={goToPrevious}
              disabled={currentIndex === 0}
              class="absolute left-4 top-1/2 -translate-y-1/2 btn btn-circle glass text-white hover:bg-white/20 disabled:opacity-30"
              title="Previous (←)"
            >
              <i class="bi bi-chevron-left text-2xl"></i>
            </button>
            <button
              onclick={goToNext}
              disabled={currentIndex === allFiles.length - 1}
              class="absolute right-4 top-1/2 -translate-y-1/2 btn btn-circle glass text-white hover:bg-white/20 disabled:opacity-30"
              title="Next (→)"
            >
              <i class="bi bi-chevron-right text-2xl"></i>
            </button>
          {/if}

          <img
            src={blobUrl}
            alt={file?.name || "Image"}
            class="max-w-full {isFullscreen
              ? 'max-h-screen'
              : 'max-h-[70vh]'} object-contain rounded-lg shadow-lg transition-transform duration-200"
            style="transform: scale({zoomLevel}) rotate({rotation}deg);"
          />
        </div>
      {:else if fileType === "video" && blobUrl}
        <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
          <!-- svelte-ignore a11y_media_has_caption -->
          <video controls class="w-full max-h-[70vh] rounded-lg shadow-lg">
            <source
              src={blobUrl}
              type="video/{file?.name.split('.').pop() || 'mp4'}"
            />
            Your browser does not support the video tag.
          </video>
        </div>
      {:else if fileType === "audio" && blobUrl}
        <div class="flex flex-col items-center gap-4 p-8">
          <div class="text-6xl text-primary">
            <i class="bi bi-music-note-beamed"></i>
          </div>
          <audio controls class="w-full max-w-md">
            <source
              src={blobUrl}
              type="audio/{file?.name.split('.').pop() || 'mp3'}"
            />
            Your browser does not support the audio element.
          </audio>
        </div>
      {:else if fileType === "pdf" && blobUrl}
        <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
          <iframe
            src={blobUrl}
            title={file?.name || "PDF Document"}
            class="w-full h-[70vh] rounded-lg border-0"
          ></iframe>
        </div>
      {:else if fileType === "text" && fileContent}
        <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
          <div class="alert alert-info mb-4">
            <i class="bi bi-info-circle"></i>
            <span>Text file preview - showing first 10,000 characters</span>
          </div>
          <pre
            class="bg-white dark:bg-slate-900 p-4 rounded-lg overflow-auto max-h-[60vh] text-sm"><code
              >{fileContent}</code
            ></pre>
        </div>
      {:else}
        <div class="flex flex-col items-center gap-4 p-12">
          <div class="text-8xl opacity-30">
            <i class="bi {getFileIcon(file?.name || '')}"></i>
          </div>
          <h3 class="text-2xl font-bold">Preview not available</h3>
          <p class="text-base-content/60">
            No preview available for .{file?.name.split(".").pop() || "unknown"}
            files
          </p>
          <div class="stats shadow">
            <div class="stat">
              <div class="stat-title">File Size</div>
              <div class="stat-value text-xl">
                {file?.size ? Math.round(file.size / 1024) : 0} KB
              </div>
            </div>
            <div class="stat">
              <div class="stat-title">File Type</div>
              <div class="stat-value text-xl">
                .{file?.name.split(".").pop()?.toUpperCase() || "UNKNOWN"}
              </div>
            </div>
          </div>
          <a
            href={blobUrl || "#"}
            download={file?.name}
            class="btn btn-primary gap-2"
          >
            <i class="bi bi-download"></i>
            Download File
          </a>
        </div>
      {/if}
    </div>

    <div slot="actions">
      <!-- Keyboard Shortcuts Info -->
      <div class="text-xs text-base-content/60 mr-auto flex items-center gap-2">
        <i class="bi bi-keyboard"></i>
        <span>
          <kbd class="kbd kbd-xs">ESC</kbd> Close
          <kbd class="kbd kbd-xs ml-2">←→</kbd> Navigate
          <kbd class="kbd kbd-xs ml-2">+/-</kbd> Zoom
          <kbd class="kbd kbd-xs ml-2">R</kbd> Rotate
          <kbd class="kbd kbd-xs ml-2">F</kbd> Fullscreen
        </span>
      </div>

      <button class="btn btn-ghost" onclick={close}> Close </button>
      <a href={blobUrl} download={file?.name} class="btn btn-primary gap-2">
        <i class="bi bi-download"></i>
        Download
      </a>
    </div>
  </Modal>
{/if}

<style>
  .preview-container {
    min-height: 200px;
  }
</style>
