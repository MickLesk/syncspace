<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { file = null, onClose = () => {} } = $props();

  function getPreviewType() {
    if (!file) return "none";
    const ext = file.name.split(".").pop().toLowerCase();
    if (["jpg", "jpeg", "png", "gif", "webp", "svg"].includes(ext))
      return "image";
    if (["mp4", "webm", "ogg"].includes(ext)) return "video";
    if (["mp3", "wav", "ogg"].includes(ext)) return "audio";
    if (["pdf"].includes(ext)) return "pdf";
    if (["txt", "md", "js", "ts", "json", "css", "html"].includes(ext))
      return "text";
    return "unsupported";
  }

  let previewType = $derived(getPreviewType());
</script>

{#if file}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="preview-overlay" onclick={onClose} role="presentation">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="preview-container"
      onclick={(e) => e.stopPropagation()}
      role="dialog" tabindex="0"
      tabindex="-1"
    >
      <div class="preview-header">
        <div class="preview-title">
          <i class="bi bi-file-earmark" aria-hidden="true"></i>
          <span>{file.name}</span>
        </div>
        <button
          class="w-8 h-8 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 flex items-center justify-center transition-colors"
          onclick={onClose}
          aria-label="Close preview"><i class="bi bi-x-lg" aria-hidden="true"></i></button
        >
      </div>

      <div class="preview-content">
        {#if previewType === "image"}
          <img src={file.url} alt={file.name} class="preview-image" />
        {:else if previewType === "video"}
          <!-- svelte-ignore a11y_media_has_caption -->
          <video src={file.url} controls class="preview-video"></video>
        {:else if previewType === "audio"}
          <audio src={file.url} controls class="preview-audio"></audio>
        {:else if previewType === "pdf"}
          <iframe src={file.url} class="preview-iframe" title={file.name}
          ></iframe>
        {:else if previewType === "text"}
          <pre class="preview-text">{file.content || tr("loading")}</pre>
        {:else}
          <div class="preview-unsupported">
            <i class="bi bi-file-earmark-x text-6xl opacity-30" aria-hidden="true"></i>
            <p>{tr("previewNotAvailable")}</p>
            <button
              class="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
              ><i class="bi bi-download" aria-hidden="true"></i> Download</button
            >
          </div>
        {/if}
      </div>

      <div class="preview-footer">
        <div class="preview-info">
          <span><i class="bi bi-hdd" aria-hidden="true"></i> {file.size || "Unknown size"}</span>
          <span
            ><i class="bi bi-calendar" aria-hidden="true"></i>
            {file.modified || "Unknown date"}</span
          >
        </div>
        <div class="preview-actions">
          <button
            class="px-3 py-1.5 text-sm bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200 transition-colors flex items-center gap-2"
            ><i class="bi bi-download" aria-hidden="true"></i> Download</button
          >
          <button
            class="px-3 py-1.5 text-sm bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200 transition-colors flex items-center gap-2"
            ><i class="bi bi-share" aria-hidden="true"></i> Share</button
          >
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .preview-overlay {
    position: fixed;
    inset: 0;
    z-index: 1000;
    background: hsl(var(--bc) / 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    backdrop-filter: blur(4px);
    animation: fadeIn 0.2s;
  }
  .preview-container {
    width: 90vw;
    max-width: 1200px;
    height: 90vh;
    background: hsl(var(--b1));
    border-radius: var(--rounded-box);
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 60px hsl(var(--bc) / 0.3);
  }
  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid hsl(var(--bc) / 0.1);
  }
  .preview-title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-weight: 600;
  }
  .preview-content {
    flex: 1;
    overflow: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1.5rem;
  }
  .preview-image,
  .preview-video {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    border-radius: 0.5rem;
  }
  .preview-audio {
    width: 100%;
    max-width: 600px;
  }
  .preview-iframe {
    width: 100%;
    height: 100%;
    border: none;
    border-radius: 0.5rem;
  }
  .preview-text {
    width: 100%;
    height: 100%;
    overflow: auto;
    background: hsl(var(--b2));
    padding: 1.5rem;
    border-radius: 0.5rem;
    font-family: monospace;
    font-size: 0.875rem;
  }
  .preview-unsupported {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }
  .preview-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-top: 1px solid hsl(var(--bc) / 0.1);
  }
  .preview-info {
    display: flex;
    gap: 1.5rem;
    color: hsl(var(--bc) / 0.6);
    font-size: 0.875rem;
  }
  .preview-info i {
    margin-right: 0.25rem;
  }
  .preview-actions {
    display: flex;
    gap: 0.5rem;
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
