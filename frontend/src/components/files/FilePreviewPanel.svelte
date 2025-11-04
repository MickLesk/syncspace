<script>
  import { onMount, onDestroy } from "svelte";
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons";
  import api from "../../lib/api";
  import { success, error as errorToast } from "../../stores/toast";

  let {
    file = $bindable(null),
    onClose = () => {},
    onNavigate = () => {},
    allFiles = [],
    currentIndex = 0,
  } = $props();

  let previewUrl = $state(null);
  let previewType = $state(null);
  let loading = $state(false);
  let error = $state(null);
  let activeMetadataTab = $state("details");
  let showMetadata = $state(false);
  let isFullscreen = $state(false);

  // Comments & Tags State
  let comments = $state([]);
  let tags = $state([]);
  let newComment = $state("");
  let newTag = $state("");

  $effect(() => {
    if (!file) {
      if (previewUrl && previewType !== "text") URL.revokeObjectURL(previewUrl);
      previewUrl = null;
      previewType = null;
      error = null;
    } else {
      loadPreview();
      loadMetadata();
    }
  });

  async function loadPreview() {
    loading = true;
    error = null;
    try {
      const filePath = file.path || file.name;
      const blob = await api.files.download(filePath);
      const ext = file.name.split(".").pop()?.toLowerCase() || "";

      if (["jpg", "jpeg", "png", "gif", "webp", "svg", "bmp"].includes(ext)) {
        previewType = "image";
        previewUrl = URL.createObjectURL(blob);
      } else if (["mp4", "webm", "ogg", "avi", "mkv", "mov"].includes(ext)) {
        previewType = "video";
        previewUrl = URL.createObjectURL(blob);
      } else if (["mp3", "wav", "flac", "aac", "ogg", "m4a"].includes(ext)) {
        previewType = "audio";
        previewUrl = URL.createObjectURL(blob);
      } else if (ext === "pdf") {
        previewType = "pdf";
        // Create proper PDF URL with type
        const pdfBlob = new Blob([blob], { type: "application/pdf" });
        previewUrl = URL.createObjectURL(pdfBlob);
      } else if (
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
          "yaml",
          "yml",
        ].includes(ext)
      ) {
        previewType = "text";
        const text = await blob.text();
        previewUrl = text.substring(0, 50000);
      } else {
        previewType = "unsupported";
      }
    } catch (err) {
      console.error("Preview error:", err);
      error = err.message || "Failed to load preview";
    } finally {
      loading = false;
    }
  }

  async function loadMetadata() {
    // Load comments and tags from backend
    try {
      // These endpoints should exist based on the backend structure
      const fileId = file.id || file.name;
      // Placeholder - implement when backend endpoints are ready
      comments = [];
      tags = [];
    } catch (err) {
      console.error("Failed to load metadata:", err);
    }
  }

  async function downloadFile() {
    if (!file) return;
    try {
      const blob = await api.files.download(file.path || file.name);
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = file.name;
      a.click();
      URL.revokeObjectURL(url);
      success("Download started");
    } catch (err) {
      errorToast("Failed to download file");
    }
  }

  async function deleteFile() {
    if (!file || !confirm(`Delete "${file.name}"?`)) return;
    try {
      await api.files.delete(file.path || file.name);
      success("File deleted");
      onClose();
      window.location.reload();
    } catch (err) {
      errorToast("Failed to delete file");
    }
  }

  function shareFile() {
    window.dispatchEvent(
      new CustomEvent("openShareModal", { detail: { file } })
    );
  }

  function openVersionHistory() {
    window.dispatchEvent(
      new CustomEvent("openVersionHistoryModal", { detail: { file } })
    );
  }

  function renameFile() {
    const newName = prompt("Enter new name:", file?.name);
    if (!newName || newName === file?.name) return;
    window.dispatchEvent(
      new CustomEvent("renameFile", { detail: { file, newName } })
    );
  }

  function moveFile() {
    window.dispatchEvent(
      new CustomEvent("openMoveModal", { detail: { files: [file] } })
    );
  }

  function copyFile() {
    window.dispatchEvent(new CustomEvent("copyFile", { detail: { file } }));
  }

  function toggleFavorite() {
    window.dispatchEvent(
      new CustomEvent("toggleFavorite", { detail: { file } })
    );
  }

  function navigatePrev() {
    if (currentIndex > 0) {
      onNavigate(currentIndex - 1);
    }
  }

  function navigateNext() {
    if (currentIndex < allFiles.length - 1) {
      onNavigate(currentIndex + 1);
    }
  }

  async function addComment() {
    if (!newComment.trim()) return;
    // TODO: Call backend API to add comment
    comments = [...comments, { text: newComment, timestamp: new Date() }];
    newComment = "";
  }

  async function addTag() {
    if (!newTag.trim()) return;
    // TODO: Call backend API to add tag
    tags = [...tags, newTag];
    newTag = "";
  }

  function toggleFullscreen() {
    isFullscreen = !isFullscreen;
  }

  onDestroy(() => {
    if (previewUrl && previewType !== "text") URL.revokeObjectURL(previewUrl);
  });
</script>

{#if file}
  <!-- Backdrop -->
  <button
    class="preview-backdrop {isFullscreen ? 'fullscreen' : ''}"
    onclick={onClose}
    aria-label="Close preview"
    style="animation: fadeIn 0.3s ease-out;"
  ></button>

  <!-- Preview Panel -->
  <div
    class="preview-panel {isFullscreen ? 'fullscreen' : ''}"
    style="animation: slideInRight 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);"
  >
    <!-- Header with actions -->
    <div class="preview-header">
      <div class="file-info">
        <i
          class="bi {getFileIcon(file.name)}"
          style="color: {getFileIconColor(file.name)}; font-size: 1.75rem;"
        ></i>
        <div class="file-details">
          <h3 class="file-name">{file.name}</h3>
          <p class="file-meta">
            {file.size ? Math.round(file.size / 1024) : 0} KB
          </p>
        </div>
      </div>

      <div class="preview-actions">
        {#if allFiles.length > 1}
          <button
            onclick={navigatePrev}
            class="action-btn"
            disabled={currentIndex === 0}
            title="Previous (←)"
          >
            <i class="bi bi-chevron-left"></i>
          </button>
          <span class="nav-counter">
            {currentIndex + 1} / {allFiles.length}
          </span>
          <button
            onclick={navigateNext}
            class="action-btn"
            disabled={currentIndex === allFiles.length - 1}
            title="Next (→)"
          >
            <i class="bi bi-chevron-right"></i>
          </button>
          <div class="divider-v"></div>
        {/if}

        <button
          onclick={toggleFavorite}
          class="action-btn favorite"
          title="Toggle Favorite"
        >
          <i class="bi bi-star-fill"></i>
        </button>
        <button
          onclick={downloadFile}
          class="action-btn download"
          title="Download"
        >
          <i class="bi bi-download"></i>
        </button>
        <button onclick={renameFile} class="action-btn rename" title="Rename">
          <i class="bi bi-pencil-square"></i>
        </button>
        <button onclick={moveFile} class="action-btn move" title="Move">
          <i class="bi bi-folder2-open"></i>
        </button>
        <button onclick={copyFile} class="action-btn copy" title="Copy">
          <i class="bi bi-files"></i>
        </button>
        <button onclick={shareFile} class="action-btn share" title="Share">
          <i class="bi bi-share-fill"></i>
        </button>
        <button
          onclick={openVersionHistory}
          class="action-btn history"
          title="History"
        >
          <i class="bi bi-clock-history"></i>
        </button>
        <button
          onclick={() => (showMetadata = !showMetadata)}
          class="action-btn metadata {showMetadata ? 'active' : ''}"
          title="Toggle Metadata"
        >
          <i class="bi bi-info-circle-fill"></i>
        </button>
        <button
          onclick={toggleFullscreen}
          class="action-btn fullscreen {isFullscreen ? 'active' : ''}"
          title={isFullscreen ? "Exit Fullscreen" : "Fullscreen"}
        >
          <i
            class="bi bi-{isFullscreen
              ? 'fullscreen-exit'
              : 'arrows-fullscreen'}"
          ></i>
        </button>
        <div class="divider-v"></div>
        <button onclick={deleteFile} class="action-btn delete" title="Delete">
          <i class="bi bi-trash3-fill"></i>
        </button>
        <button onclick={onClose} class="action-btn close" title="Close">
          <i class="bi bi-x-lg"></i>
        </button>
      </div>
    </div>

    <!-- Preview Content with optional metadata sidebar -->
    <div class="preview-body {showMetadata ? 'with-metadata' : ''}">
      <!-- Main Preview Area -->
      <div class="preview-content">
        {#if loading}
          <div class="preview-loading">
            <div class="spinner"></div>
            <p>Loading preview...</p>
          </div>
        {:else if error}
          <div class="preview-error">
            <i class="bi bi-exclamation-triangle-fill"></i>
            <p>{error}</p>
            <button onclick={downloadFile} class="btn-download">
              <i class="bi bi-download"></i>
              Download File
            </button>
          </div>
        {:else if previewType === "image"}
          <div class="preview-image-container">
            <img src={previewUrl} alt={file.name} class="preview-image" />
          </div>
        {:else if previewType === "video"}
          <div class="preview-video-container">
            <video src={previewUrl} controls class="preview-video">
              <track kind="captions" />
            </video>
          </div>
        {:else if previewType === "audio"}
          <div class="preview-audio-container">
            <i class="bi bi-music-note-beamed file-type-icon"></i>
            <audio src={previewUrl} controls class="preview-audio"></audio>
          </div>
        {:else if previewType === "pdf"}
          <div class="preview-pdf-container">
            <iframe src={previewUrl} title={file.name} class="preview-pdf"
            ></iframe>
          </div>
        {:else if previewType === "text"}
          <div class="preview-text-container">
            <pre class="preview-text"><code>{previewUrl}</code></pre>
          </div>
        {:else}
          <div class="preview-unsupported">
            <i class="bi bi-file-earmark-x file-type-icon"></i>
            <h4>Preview not available</h4>
            <p>
              {file.name.split(".").pop()?.toUpperCase() || "This"} files cannot
              be previewed
            </p>
            <button onclick={downloadFile} class="btn-download">
              <i class="bi bi-download"></i>
              Download to View
            </button>
          </div>
        {/if}
      </div>

      <!-- Metadata Sidebar -->
      {#if showMetadata}
        <div class="metadata-sidebar">
          <div class="metadata-tabs">
            <button
              class="metadata-tab {activeMetadataTab === 'details'
                ? 'active'
                : ''}"
              onclick={() => (activeMetadataTab = "details")}
            >
              <i class="bi bi-info-circle"></i>
              <span>Details</span>
            </button>
            <button
              class="metadata-tab {activeMetadataTab === 'comments'
                ? 'active'
                : ''}"
              onclick={() => (activeMetadataTab = "comments")}
            >
              <i class="bi bi-chat-dots"></i>
              <span>Comments</span>
            </button>
            <button
              class="metadata-tab {activeMetadataTab === 'tags'
                ? 'active'
                : ''}"
              onclick={() => (activeMetadataTab = "tags")}
            >
              <i class="bi bi-tags"></i>
              <span>Tags</span>
            </button>
          </div>

          <div class="metadata-content">
            {#if activeMetadataTab === "details"}
              <div class="details-panel">
                <div class="detail-row">
                  <span class="detail-label">Name</span>
                  <span class="detail-value">{file.name}</span>
                </div>
                <div class="detail-row">
                  <span class="detail-label">Size</span>
                  <span class="detail-value">
                    {file.size ? Math.round(file.size / 1024) : 0} KB
                  </span>
                </div>
                <div class="detail-row">
                  <span class="detail-label">Type</span>
                  <span class="detail-value">
                    {file.name.split(".").pop()?.toUpperCase() || "Unknown"}
                  </span>
                </div>
                <div class="detail-row">
                  <span class="detail-label">Modified</span>
                  <span class="detail-value">
                    {file.modified_at
                      ? new Date(file.modified_at).toLocaleString()
                      : "Unknown"}
                  </span>
                </div>
                <div class="detail-row">
                  <span class="detail-label">Owner</span>
                  <span class="detail-value">{file.owner || "Me"}</span>
                </div>
              </div>
            {:else if activeMetadataTab === "comments"}
              <div class="comments-panel">
                <div class="comments-list">
                  {#if comments.length === 0}
                    <div class="empty-state">
                      <i class="bi bi-chat-dots"></i>
                      <p>No comments yet</p>
                    </div>
                  {:else}
                    {#each comments as comment}
                      <div class="comment">
                        <p class="comment-text">{comment.text}</p>
                        <span class="comment-time">
                          {new Date(comment.timestamp).toLocaleString()}
                        </span>
                      </div>
                    {/each}
                  {/if}
                </div>
                <div class="comment-input">
                  <textarea
                    bind:value={newComment}
                    placeholder="Add a comment..."
                    rows="3"
                  ></textarea>
                  <button onclick={addComment} class="btn-add">
                    <i class="bi bi-send-fill"></i>
                    Send
                  </button>
                </div>
              </div>
            {:else if activeMetadataTab === "tags"}
              <div class="tags-panel">
                <div class="tags-list">
                  {#if tags.length === 0}
                    <div class="empty-state">
                      <i class="bi bi-tags"></i>
                      <p>No tags yet</p>
                    </div>
                  {:else}
                    {#each tags as tag}
                      <div class="tag-chip">
                        <i class="bi bi-tag-fill"></i>
                        <span>{tag}</span>
                        <button class="tag-remove" aria-label="Remove tag">
                          <i class="bi bi-x"></i>
                        </button>
                      </div>
                    {/each}
                  {/if}
                </div>
                <div class="tag-input">
                  <input
                    type="text"
                    bind:value={newTag}
                    placeholder="Add a tag..."
                    onkeydown={(e) => e.key === "Enter" && addTag()}
                  />
                  <button onclick={addTag} class="btn-add">
                    <i class="bi bi-plus-lg"></i>
                    Add
                  </button>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  /* Backdrop Overlay */
  .preview-backdrop {
    position: fixed;
    top: 4rem;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.2);
    backdrop-filter: blur(4px);
    z-index: 45;
    transition: background 0.3s ease;
    border: none;
    padding: 0;
    cursor: pointer;
  }

  :global(.dark) .preview-backdrop {
    background: rgba(0, 0, 0, 0.4);
  }

  .preview-backdrop.fullscreen {
    top: 4rem; /* Stay below header */
    background: rgba(0, 0, 0, 0.6);
  }

  /* Preview Panel - Similar to Activity Feed */
  .preview-panel {
    position: fixed;
    top: 4rem;
    right: 0;
    width: 50vw;
    min-width: 600px;
    max-width: 1200px;
    height: calc(100vh - 4rem);
    background: var(--md-sys-color-surface);
    box-shadow: -8px 0 32px rgba(0, 0, 0, 0.15);
    z-index: 50;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  :global(.dark) .preview-panel {
    box-shadow: -8px 0 32px rgba(0, 0, 0, 0.5);
  }

  /* Fullscreen Mode - stays below header */
  .preview-panel.fullscreen {
    top: 4rem; /* Keep below app header */
    left: 0;
    width: 100vw;
    max-width: 100vw;
    height: calc(100vh - 4rem); /* Account for header height */
  }

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
    background: var(--md-sys-color-surface-container-low);
    gap: 1rem;
    flex-wrap: wrap;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    min-width: 0;
    flex: 1;
  }

  .file-details {
    min-width: 0;
    flex: 1;
  }

  .file-name {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-meta {
    margin: 0.125rem 0 0;
    font-size: 0.8125rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .preview-actions {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
  }

  .action-btn {
    background: transparent;
    border: none;
    padding: 0.5rem;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .action-btn:hover {
    background: var(--md-sys-color-surface-container-highest);
    color: var(--md-sys-color-on-surface);
  }

  .action-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .action-btn.favorite {
    color: #ff9800;
  }
  .action-btn.favorite:hover {
    background: rgba(255, 152, 0, 0.1);
  }

  .action-btn.download {
    color: #4caf50;
  }
  .action-btn.download:hover {
    background: rgba(76, 175, 80, 0.1);
  }

  .action-btn.rename {
    color: #2196f3;
  }
  .action-btn.rename:hover {
    background: rgba(33, 150, 243, 0.1);
  }

  .action-btn.move {
    color: #9c27b0;
  }
  .action-btn.move:hover {
    background: rgba(156, 39, 176, 0.1);
  }

  .action-btn.copy {
    color: #00bcd4;
  }
  .action-btn.copy:hover {
    background: rgba(0, 188, 212, 0.1);
  }

  .action-btn.share {
    color: #3f51b5;
  }
  .action-btn.share:hover {
    background: rgba(63, 81, 181, 0.1);
  }

  .action-btn.history {
    color: #607d8b;
  }
  .action-btn.history:hover {
    background: rgba(96, 125, 139, 0.1);
  }

  .action-btn.metadata {
    color: #795548;
  }
  .action-btn.metadata:hover,
  .action-btn.metadata.active {
    background: rgba(121, 85, 72, 0.15);
    color: #795548;
  }

  .action-btn.fullscreen {
    color: #673ab7;
  }
  .action-btn.fullscreen:hover,
  .action-btn.fullscreen.active {
    background: rgba(103, 58, 183, 0.15);
    color: #673ab7;
  }

  .action-btn.delete {
    color: #f44336;
  }
  .action-btn.delete:hover {
    background: rgba(244, 67, 54, 0.1);
  }

  .action-btn.close {
    color: var(--md-sys-color-on-surface);
  }
  .action-btn.close:hover {
    background: var(--md-sys-color-error-container);
    color: var(--md-sys-color-error);
  }

  .nav-counter {
    font-size: 0.75rem;
    color: var(--md-sys-color-on-surface-variant);
    padding: 0 0.25rem;
  }

  .divider-v {
    width: 1px;
    height: 1.5rem;
    background: var(--md-sys-color-outline-variant);
    margin: 0 0.25rem;
  }

  .preview-body {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .preview-body.with-metadata {
    gap: 0;
  }

  .preview-content {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    overflow: auto;
    background: var(--md-sys-color-surface-container-lowest);
  }

  .preview-loading,
  .preview-error,
  .preview-unsupported {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    text-align: center;
    color: var(--md-sys-color-on-surface-variant);
  }

  .spinner {
    width: 3rem;
    height: 3rem;
    border: 3px solid var(--md-sys-color-outline);
    border-top-color: var(--md-sys-color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .file-type-icon {
    font-size: 4rem;
    opacity: 0.3;
  }

  .preview-image-container,
  .preview-video-container,
  .preview-pdf-container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .preview-image {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    border-radius: 0.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .preview-video {
    max-width: 100%;
    max-height: 100%;
    border-radius: 0.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .preview-audio-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;
  }

  .preview-audio {
    width: 100%;
    max-width: 500px;
  }

  .preview-pdf {
    width: 100%;
    height: 100%;
    border: none;
    border-radius: 0.5rem;
  }

  .preview-text-container {
    width: 100%;
    height: 100%;
    overflow: auto;
  }

  .preview-text {
    margin: 0;
    padding: 1.5rem;
    background: var(--md-sys-color-surface-container);
    border-radius: 0.5rem;
    font-family: "Consolas", "Monaco", "Courier New", monospace;
    font-size: 0.875rem;
    line-height: 1.6;
    white-space: pre-wrap;
    word-wrap: break-word;
    color: var(--md-sys-color-on-surface);
  }

  .btn-download {
    padding: 0.75rem 1.5rem;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    border: none;
    border-radius: 0.5rem;
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: background 0.2s ease;
  }

  .btn-download:hover {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  /* Metadata Sidebar */
  .metadata-sidebar {
    width: 320px;
    border-left: 1px solid var(--md-sys-color-outline-variant);
    display: flex;
    flex-direction: column;
    background: var(--md-sys-color-surface-container-low);
  }

  .metadata-tabs {
    display: flex;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
    background: var(--md-sys-color-surface);
  }

  .metadata-tab {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.75rem 0.5rem;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--md-sys-color-on-surface-variant);
    font-size: 0.75rem;
  }

  .metadata-tab i {
    font-size: 1.125rem;
  }

  .metadata-tab:hover {
    background: var(--md-sys-color-surface-container);
    color: var(--md-sys-color-on-surface);
  }

  .metadata-tab.active {
    color: var(--md-sys-color-primary);
    border-bottom-color: var(--md-sys-color-primary);
    background: var(--md-sys-color-surface-container);
  }

  .metadata-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }

  .details-panel,
  .comments-panel,
  .tags-panel {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    padding: 0.625rem;
    background: var(--md-sys-color-surface-container);
    border-radius: 0.5rem;
    gap: 0.5rem;
  }

  .detail-label {
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    font-size: 0.8125rem;
  }

  .detail-value {
    color: var(--md-sys-color-on-surface);
    font-size: 0.8125rem;
    text-align: right;
    word-break: break-word;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    gap: 0.5rem;
    color: var(--md-sys-color-on-surface-variant);
    opacity: 0.6;
  }

  .empty-state i {
    font-size: 2.5rem;
  }

  .comments-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .comment {
    padding: 0.75rem;
    background: var(--md-sys-color-surface-container);
    border-radius: 0.5rem;
    border-left: 3px solid var(--md-sys-color-primary);
  }

  .comment-text {
    margin: 0 0 0.5rem;
    font-size: 0.875rem;
    color: var(--md-sys-color-on-surface);
  }

  .comment-time {
    font-size: 0.75rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .comment-input,
  .tag-input {
    display: flex;
    gap: 0.5rem;
    margin-top: auto;
  }

  .comment-input textarea {
    flex: 1;
    padding: 0.625rem;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 0.5rem;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-family: inherit;
    font-size: 0.875rem;
    resize: vertical;
  }

  .tag-input input {
    flex: 1;
    padding: 0.625rem;
    border: 1px solid var(--md-sys-color-outline);
    border-radius: 0.5rem;
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-family: inherit;
    font-size: 0.875rem;
  }

  .btn-add {
    padding: 0.625rem 1rem;
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    border: none;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    transition: background 0.2s ease;
  }

  .btn-add:hover {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .tag-chip {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.625rem;
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
    border-radius: 1rem;
    font-size: 0.8125rem;
  }

  .tag-remove {
    background: transparent;
    border: none;
    padding: 0.125rem;
    cursor: pointer;
    color: inherit;
    opacity: 0.7;
    transition: opacity 0.2s ease;
  }

  .tag-remove:hover {
    opacity: 1;
  }

  /* Responsive Adjustments */
  @media (max-width: 1400px) {
    .preview-panel {
      width: 60vw;
      min-width: 500px;
    }
  }

  @media (max-width: 1200px) {
    .metadata-sidebar {
      width: 280px;
    }

    .preview-panel {
      width: 70vw;
      min-width: 400px;
    }
  }

  @media (max-width: 768px) {
    .preview-panel {
      width: 100vw;
      min-width: 100vw;
      top: 0;
    }

    .preview-backdrop {
      top: 0;
    }

    .preview-header {
      flex-direction: column;
      align-items: stretch;
    }

    .preview-actions {
      justify-content: space-between;
    }

    .preview-body.with-metadata {
      flex-direction: column;
    }

    .metadata-sidebar {
      width: 100%;
      max-height: 40vh;
    }
  }

  /* Animations */
  @keyframes slideInRight {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
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
