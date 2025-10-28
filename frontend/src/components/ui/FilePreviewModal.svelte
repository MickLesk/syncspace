<script>
  import { onMount } from "svelte";
  import Modal from "./Modal.svelte";
  import { getFileIcon } from "../../utils/fileIcons";
  import api from "../../lib/api";

  let { visible = $bindable(false), file = $bindable(null) } = $props();

  let fileContent = $state("");
  let blobUrl = $state(null);
  let loading = $state(false);
  let error = $state(null);

  // Cleanup blob URL when modal closes or file changes
  $effect(() => {
    if (!visible && blobUrl) {
      URL.revokeObjectURL(blobUrl);
      blobUrl = null;
      fileContent = "";
      error = null;
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
        <div class="flex justify-center bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
          <img
            src={blobUrl}
            alt={file?.name || "Image"}
            class="max-w-full max-h-[70vh] object-contain rounded-lg shadow-lg"
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
          <a href={fileUrl} download={file?.name} class="btn btn-primary gap-2">
            <i class="bi bi-download"></i>
            Download File
          </a>
        </div>
      {/if}
    </div>

    <div slot="actions">
      <button class="btn btn-ghost" onclick={close}> Close </button>
      <a href={fileUrl} download={file?.name} class="btn btn-primary gap-2">
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

