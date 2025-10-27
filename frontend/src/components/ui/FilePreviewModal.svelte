<script>
  import Modal from "./Modal.svelte";
  import { getFileIcon } from "../../utils/fileIcons";

  let { visible = $bindable(false), file = $bindable(null) } = $props();

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
  let fileUrl = $derived(
    file
      ? `http://localhost:8080/api/files/${encodeURIComponent(file.path || file.name)}`
      : null
  );
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
      {#if fileType === "image"}
        <div class="flex justify-center bg-base-200 rounded-lg p-4">
          <img
            src={fileUrl}
            alt={file?.name || "Image"}
            class="max-w-full max-h-[70vh] object-contain rounded-lg shadow-lg"
          />
        </div>
      {:else if fileType === "video"}
        <div class="bg-base-200 rounded-lg p-4">
          <!-- svelte-ignore a11y_media_has_caption -->
          <video controls class="w-full max-h-[70vh] rounded-lg shadow-lg">
            <source
              src={fileUrl}
              type="video/{file?.name.split('.').pop() || 'mp4'}"
            />
            Your browser does not support the video tag.
          </video>
        </div>
      {:else if fileType === "audio"}
        <div class="flex flex-col items-center gap-4 p-8">
          <div class="text-6xl text-primary">
            <i class="bi bi-music-note-beamed"></i>
          </div>
          <audio controls class="w-full max-w-md">
            <source
              src={fileUrl}
              type="audio/{file?.name.split('.').pop() || 'mp3'}"
            />
            Your browser does not support the audio element.
          </audio>
        </div>
      {:else if fileType === "pdf"}
        <div class="bg-base-200 rounded-lg p-4">
          <iframe
            src={fileUrl}
            title={file?.name || "PDF Document"}
            class="w-full h-[70vh] rounded-lg border-0"
          ></iframe>
        </div>
      {:else if fileType === "text"}
        <div class="bg-base-200 rounded-lg p-4">
          <div class="alert alert-info mb-4">
            <i class="bi bi-info-circle"></i>
            <span>Text file preview - showing first 10,000 characters</span>
          </div>
          <iframe
            src={fileUrl}
            title={file?.name || "Text File"}
            class="w-full h-[60vh] rounded-lg border bg-base-100 p-4"
          ></iframe>
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
