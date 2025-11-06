<script>
  import { createEventDispatcher } from "svelte";
  import Icon from "./Icon.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    file = null,
    files = [],
    currentPath = "",
    onDownload = null,
    onShare = null,
    onCopy = null,
    onMove = null,
    onDelete = null,
  } = $props();

  const dispatch = createEventDispatcher();

  let currentIndex = $state(0);
  let previewUrl = $state(null);
  let previewType = $state(null);
  let loading = $state(true);
  let error = $state(null);

  $effect(() => {
    if (file) {
      currentIndex = files.findIndex((f) => f.name === file.name);
      loadPreview();
    }
  });

  async function loadPreview() {
    loading = true;
    error = null;

    try {
      // Build full path: if currentPath is "/", use just filename
      const fullPath =
        currentPath === "/" ? file.name : `${currentPath}${file.name}`;

      // Get token - try new format first, then fallback to old
      let token = localStorage.getItem("authToken");
      if (!token) {
        const authData = localStorage.getItem("auth");
        if (authData) {
          try {
            const parsed = JSON.parse(authData);
            token = parsed.token;
          } catch (e) {
            // ignore parse errors
          }
        }
      }

      if (!token) {
        throw new Error("Not authenticated - no token found in localStorage");
      }

      console.log(`[PreviewModal] Loading: ${fullPath}`);
      console.log(
        `[PreviewModal] Token available: ${token.substring(0, 20)}...`
      );

      const response = await fetch(
        `http://localhost:8080/api/file/${encodeURIComponent(fullPath)}`,
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );

      if (!response.ok) {
        throw new Error(tr("failedToLoadFile", response.status));
      }

      const blob = await response.blob();
      previewUrl = URL.createObjectURL(blob);

      // Determine preview type
      const ext = file.name.split(".").pop().toLowerCase();
      if (["jpg", "jpeg", "png", "gif", "webp", "svg", "bmp"].includes(ext)) {
        previewType = "image";
      } else if (["mp4", "webm", "ogg"].includes(ext)) {
        previewType = "video";
      } else if (ext === "pdf") {
        previewType = "pdf";
      } else if (
        ["txt", "md", "json", "js", "css", "html", "xml", "csv"].includes(ext)
      ) {
        previewType = "text";
        const text = await blob.text();
        previewUrl = text;
      } else {
        previewType = "unsupported";
      }

      loading = false;
    } catch (err) {
      console.error("Preview error:", err);
      error = tr("previewError", err.message);
      loading = false;
    }
  }

  function close() {
    if (previewUrl && previewType !== "text") {
      URL.revokeObjectURL(previewUrl);
    }
    dispatch("close");
  }

  function navigate(direction) {
    const previewableFiles = files.filter(
      (f) => !f.is_dir && isPreviewable(f.name)
    );
    let newIndex = currentIndex + direction;

    if (newIndex < 0) newIndex = previewableFiles.length - 1;
    if (newIndex >= previewableFiles.length) newIndex = 0;

    file = previewableFiles[newIndex];
  }

  function isPreviewable(filename) {
    const ext = filename.split(".").pop().toLowerCase();
    return [
      "jpg",
      "jpeg",
      "png",
      "gif",
      "webp",
      "svg",
      "bmp",
      "mp4",
      "webm",
      "ogg",
      "pdf",
      "txt",
      "md",
      "json",
      "js",
      "css",
      "html",
      "xml",
      "csv",
    ].includes(ext);
  }

  function handleKeydown(e) {
    if (e.key === "Escape") close();
    if (e.key === "ArrowLeft") navigate(-1);
    if (e.key === "ArrowRight") navigate(1);

    // Keyboard shortcuts for actions
    if (e.key === "c" && onCopy) {
      e.preventDefault();
      onCopy(file);
    }
    if (e.key === "m" && onMove) {
      e.preventDefault();
      onMove(file);
    }
    if (e.key === "s" && onShare) {
      e.preventDefault();
      onShare(file);
    }
    if (e.key === "d" && onDownload) {
      e.preventDefault();
      onDownload(file);
    }
    if (e.key === "Delete" && onDelete) {
      e.preventDefault();
      onDelete(file);
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if file}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="preview-overlay" onclick={close} role="presentation">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="preview-modal"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="-1"
    >
      <div class="preview-header">
        <div class="preview-title">
          <Icon name="file-earmark" size={20} />
          <span>{file.name}</span>
        </div>
        <div class="preview-actions">
          <button class="btn-nav" onclick={() => navigate(-1)} title="Previous">
            <Icon name="chevron-left" size={20} />
          </button>
          <button class="btn-nav" onclick={() => navigate(1)} title="Next">
            <Icon name="chevron-right" size={20} />
          </button>
          <button class="btn-close" onclick={close}>
            <Icon name="x" size={24} />
          </button>
        </div>
      </div>

      <!-- Action Toolbar -->
      <div class="action-toolbar">
        {#if onCopy}
          <button
            class="action-btn"
            onclick={() => onCopy(file)}
            title="{tr('copyFile')} (C)"
          >
            <i class="bi bi-files"></i>
            <span>{tr("copy")}</span>
            <kbd>C</kbd>
          </button>
        {/if}
        {#if onMove}
          <button
            class="action-btn"
            onclick={() => onMove(file)}
            title="{tr('moveFile')} (M)"
          >
            <i class="bi bi-folder-symlink"></i>
            <span>{tr("move")}</span>
            <kbd>M</kbd>
          </button>
        {/if}
        {#if onShare}
          <button
            class="action-btn"
            onclick={() => onShare(file)}
            title="{tr('share')} (S)"
          >
            <i class="bi bi-share"></i>
            <span>{tr("share")}</span>
            <kbd>S</kbd>
          </button>
        {/if}
        {#if onDownload}
          <button
            class="action-btn"
            onclick={() => onDownload(file)}
            title="{tr('download')} (D)"
          >
            <i class="bi bi-download"></i>
            <span>{tr("download")}</span>
            <kbd>D</kbd>
          </button>
        {/if}
        {#if onDelete}
          <button
            class="action-btn action-btn-danger"
            onclick={() => onDelete(file)}
            title="{tr('delete')} (Del)"
          >
            <i class="bi bi-trash"></i>
            <span>{tr("delete")}</span>
            <kbd>Del</kbd>
          </button>
        {/if}
      </div>

      <div class="preview-content">
        {#if loading}
          <div class="preview-loading">
            <div class="spinner"></div>
            <p>Loading preview...</p>
          </div>
        {:else if error}
          <div class="preview-error">
            <Icon name="exclamation-triangle" size={48} />
            <p>{error}</p>
          </div>
        {:else if previewType === "image"}
          <img src={previewUrl} alt={file.name} />
        {:else if previewType === "video"}
          <!-- svelte-ignore a11y_media_has_caption -->
          <video src={previewUrl} controls></video>
        {:else if previewType === "pdf"}
          <iframe src={previewUrl} title={file.name}></iframe>
        {:else if previewType === "text"}
          <pre><code>{previewUrl}</code></pre>
        {:else}
          <div class="preview-unsupported">
            <Icon name="file-earmark-x" size={64} />
            <p>Preview not available for this file type</p>
            <small
              >{file.name.split(".").pop().toUpperCase()} files cannot be previewed</small
            >
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .preview-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.85);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    backdrop-filter: blur(4px);
  }

  .preview-modal {
    background: var(--md-sys-color-surface);
    border-radius: 24px;
    max-width: 90vw;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    overflow: hidden;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
    background: var(--md-sys-color-surface-container);
  }

  .preview-title {
    display: flex;
    align-items: center;
    gap: 12px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    font-size: 16px;
  }

  .preview-actions {
    display: flex;
    gap: 8px;
  }

  .action-toolbar {
    display: flex;
    gap: 8px;
    padding: 12px 24px;
    background: var(--md-sys-color-surface-container-low);
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
    overflow-x: auto;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    background: var(--md-sys-color-surface-container-highest);
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 20px;
    color: var(--md-sys-color-on-surface);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .action-btn:hover {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  .action-btn-danger:hover {
    background: var(--md-sys-color-error-container);
    color: var(--md-sys-color-on-error-container);
    border-color: var(--md-sys-color-error);
  }

  .action-btn i {
    font-size: 16px;
  }

  .action-btn kbd {
    background: var(--md-sys-color-surface-container-low);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 11px;
    font-family: monospace;
    font-weight: 600;
    color: var(--md-sys-color-on-surface-variant);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
    margin-left: 4px;
  }

  .action-btn:hover kbd {
    background: var(--md-sys-color-surface);
    border-color: var(--md-sys-color-outline);
  }

  .btn-nav,
  .btn-close {
    background: transparent;
    border: none;
    color: var(--md-sys-color-on-surface);
    cursor: pointer;
    padding: 8px;
    border-radius: 50%;
    transition: background 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-nav:hover,
  .btn-close:hover {
    background: var(--md-sys-color-surface-container-highest);
  }

  .preview-content {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    overflow: auto;
    min-height: 400px;
  }

  .preview-content img {
    max-width: 100%;
    max-height: 70vh;
    object-fit: contain;
    border-radius: 12px;
  }

  .preview-content video {
    max-width: 100%;
    max-height: 70vh;
    border-radius: 12px;
  }

  .preview-content iframe {
    width: 80vw;
    height: 70vh;
    border: none;
    border-radius: 12px;
  }

  .preview-content pre {
    background: var(--md-sys-color-surface-container-low);
    padding: 16px;
    border-radius: 12px;
    max-width: 800px;
    max-height: 70vh;
    overflow: auto;
    font-family: "Courier New", monospace;
    font-size: 13px;
    line-height: 1.5;
    color: var(--md-sys-color-on-surface);
  }

  .preview-loading,
  .preview-error,
  .preview-unsupported {
    text-align: center;
    color: var(--md-sys-color-on-surface-variant);
  }

  .preview-loading .spinner {
    width: 48px;
    height: 48px;
    border: 4px solid var(--md-sys-color-outline);
    border-top-color: var(--md-sys-color-primary);
    border-radius: 50%;
    margin: 0 auto 16px;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .preview-error p,
  .preview-unsupported p {
    margin-top: 16px;
    font-size: 16px;
  }

  .preview-unsupported small {
    display: block;
    margin-top: 8px;
    font-size: 13px;
  }
</style>
