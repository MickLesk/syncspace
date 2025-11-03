<script>
  import { getFileIcon } from "../../utils/fileIcons";

  let { file, previewUrl, previewType, loading, error } = $props();
</script>

{#if loading}
  <div class="flex justify-center items-center h-96">
    <span class="loading loading-spinner loading-lg text-primary"></span>
  </div>
{:else if error}
  <div class="alert alert-error">
    <i class="bi bi-exclamation-triangle"></i>
    <span>{error}</span>
  </div>
{:else if previewType === "image"}
  <div class="flex justify-center bg-base-200 rounded-xl p-4">
    <img
      src={previewUrl}
      alt={file?.name}
      class="max-w-full max-h-[70vh] object-contain rounded-lg shadow-lg"
    />
  </div>
{:else if previewType === "video"}
  <div class="bg-base-200 rounded-xl p-4">
    <!-- svelte-ignore a11y_media_has_caption -->
    <video controls class="w-full max-h-[70vh] rounded-lg shadow-lg">
      <source src={previewUrl} type="video/{file?.name.split('.').pop()}" />
      Your browser does not support video playback.
    </video>
  </div>
{:else if previewType === "audio"}
  <div class="flex flex-col items-center gap-6 p-12">
    <i class="bi bi-music-note-beamed text-8xl text-primary"></i>
    <audio controls class="w-full max-w-md">
      <source src={previewUrl} type="audio/{file?.name.split('.').pop()}" />
      Your browser does not support audio playback.
    </audio>
  </div>
{:else if previewType === "pdf"}
  <div class="bg-base-200 rounded-xl p-4">
    <iframe
      src={previewUrl}
      title={file?.name}
      class="w-full h-[70vh] rounded-lg border-0"
    ></iframe>
  </div>
{:else if previewType === "text"}
  <div class="bg-base-200 rounded-xl p-4">
    <div class="alert alert-info mb-4">
      <i class="bi bi-info-circle"></i>
      <span>Text preview - first 10,000 characters</span>
    </div>
    <pre
      class="bg-base-100 p-4 rounded-lg overflow-auto max-h-[60vh] text-sm font-mono">{previewUrl}</pre>
  </div>
{:else}
  <div class="flex flex-col items-center gap-6 py-16">
    <i class="bi {getFileIcon(file?.name)} text-9xl opacity-20"></i>
    <div class="text-center">
      <h3 class="text-2xl font-bold mb-2">Preview not available</h3>
      <p class="text-base-content/60">
        .{file?.name.split(".").pop()?.toUpperCase()} files cannot be previewed
      </p>
    </div>
    <div class="stats shadow">
      <div class="stat place-items-center">
        <div class="stat-title">Size</div>
        <div class="stat-value text-primary">
          {file?.size ? Math.round(file.size / 1024) : 0} KB
        </div>
      </div>
    </div>
  </div>
{/if}
