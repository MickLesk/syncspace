<script>
  import { onMount, onDestroy } from "svelte";
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons";
  import api from "../../lib/api";
  import { success, error as errorToast } from "../../stores/toast";
  import * as XLSX from "xlsx";
  import mammoth from "mammoth";
  import Prism from "prismjs";

  // Import Prism languages
  import "prismjs/components/prism-javascript";
  import "prismjs/components/prism-typescript";
  import "prismjs/components/prism-json";
  import "prismjs/components/prism-python";
  import "prismjs/components/prism-java";
  import "prismjs/components/prism-c";
  import "prismjs/components/prism-cpp";
  import "prismjs/components/prism-rust";
  import "prismjs/components/prism-go";
  import "prismjs/components/prism-bash";
  import "prismjs/components/prism-yaml";
  import "prismjs/components/prism-css";
  import "prismjs/components/prism-markup"; // HTML/XML

  // Import Prism theme
  import "prismjs/themes/prism-tomorrow.css";

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
  let excelData = $state(null);
  let activeSheet = $state(0);
  let nutrientContainer = $state(null);
  let nutrientInstance = $state(null);

  // Comments & Tags State
  let comments = $state([]);
  let tags = $state([]);
  let newComment = $state("");
  let newTag = $state("");

  // Helper function to format file size
  function formatFileSize(bytes) {
    if (!bytes || bytes === 0) return "0 KB";

    const kb = bytes / 1024;
    if (kb < 1024) return `${Math.round(kb)} KB`;

    const mb = kb / 1024;
    if (mb < 1024) return `${mb.toFixed(2)} MB`;

    const gb = mb / 1024;
    return `${gb.toFixed(2)} GB`;
  }

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
          "jsx",
          "tsx",
          "vue",
          "svelte",
        ].includes(ext)
      ) {
        previewType = "code";
        const text = await blob.text();
        let truncatedText = text.substring(0, 50000);

        // Detect language for syntax highlighting
        const languageMap = {
          js: "javascript",
          jsx: "javascript",
          ts: "typescript",
          tsx: "typescript",
          json: "json",
          py: "python",
          java: "java",
          c: "c",
          cpp: "cpp",
          rs: "rust",
          go: "go",
          sh: "bash",
          yaml: "yaml",
          yml: "yaml",
          css: "css",
          html: "markup",
          xml: "markup",
          svg: "markup",
          vue: "markup",
          svelte: "markup",
          md: "markdown",
          txt: "plaintext",
          log: "plaintext",
          csv: "plaintext",
        };

        const language = languageMap[ext] || "plaintext";

        // Format JSON with proper indentation
        if (ext === "json") {
          try {
            const jsonObj = JSON.parse(truncatedText);
            truncatedText = JSON.stringify(jsonObj, null, 2);
          } catch (e) {
            // If parsing fails, use original text
            console.warn("Failed to parse JSON:", e);
          }
        }

        // Apply syntax highlighting
        if (language !== "plaintext" && Prism.languages[language]) {
          previewUrl = Prism.highlight(
            truncatedText,
            Prism.languages[language],
            language
          );
        } else {
          previewUrl = truncatedText;
        }
      } else if (["xlsx", "xls", "xlsm", "xlsb", "ods"].includes(ext)) {
        // Excel files with Sheet.js
        previewType = "excel";
        const arrayBuffer = await blob.arrayBuffer();
        const workbook = XLSX.read(arrayBuffer, { type: "array" });

        excelData = {
          workbook: workbook,
          sheetNames: workbook.SheetNames,
        };
        activeSheet = 0; // Reset to first sheet
        renderExcelSheet(0);
        previewUrl = "excel-rendered";
      } else if (["docx", "doc"].includes(ext)) {
        // Word files with Mammoth.js
        previewType = "word";
        const arrayBuffer = await blob.arrayBuffer();
        const result = await mammoth.convertToHtml({ arrayBuffer });
        previewUrl = result.value; // HTML string
      } else if (["pptx", "ppt"].includes(ext)) {
        // PowerPoint files with Microsoft Office Viewer
        previewType = "office-viewer";
        previewUrl = URL.createObjectURL(blob);
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

  function renderExcelSheet(sheetIndex) {
    if (!excelData || !excelData.workbook) return;

    const sheetName = excelData.sheetNames[sheetIndex];
    const sheet = excelData.workbook.Sheets[sheetName];
    const htmlTable = XLSX.utils.sheet_to_html(sheet, {
      id: "excel-table",
      editable: false,
    });

    excelData = {
      ...excelData,
      html: htmlTable,
      currentSheet: sheetName,
    };
    activeSheet = sheetIndex;
  }

  function switchSheet(index) {
    renderExcelSheet(index);
  }

  async function copyToClipboard(text) {
    try {
      // Remove HTML tags for plain text copy
      const plainText = text.replace(/<[^>]*>/g, "");
      await navigator.clipboard.writeText(plainText);
      success("Code copied to clipboard!");
    } catch (err) {
      errorToast("Failed to copy code");
    }
  }

  async function loadMetadata() {
    // Load comments and tags from backend
    try {
      const fileId = file.id || file.name;
      // Try to load comments and tags from API
      try {
        const [commentsData, tagsData] = await Promise.all([
          api.files.getComments(fileId).catch(() => []),
          api.files.getTags(fileId).catch(() => []),
        ]);
        comments = commentsData || [];
        tags = tagsData || [];
      } catch (err) {
        // Graceful fallback
        comments = [];
        tags = [];
      }
    } catch (err) {
      console.error("Failed to load metadata:", err);
      comments = [];
      tags = [];
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
    if (!newComment.trim() || !file?.path) return;
    try {
      await api.comments.create(file.path, newComment);
      comments = [...comments, { text: newComment, timestamp: new Date() }];
      newComment = "";
      success("Comment added successfully");
    } catch (err) {
      errorToast("Failed to add comment: " + (err.message || err));
    }
  }

  async function addTag() {
    if (!newTag.trim() || !file?.path) return;
    try {
      await api.tags.add(file.path, newTag);
      tags = [...tags, newTag];
      newTag = "";
      success("Tag added successfully");
    } catch (err) {
      errorToast("Failed to add tag: " + (err.message || err));
    }
  }

  function toggleFullscreen() {
    isFullscreen = !isFullscreen;
  }

  onDestroy(() => {
    if (
      previewUrl &&
      !["text", "excel", "word", "excel-rendered"].includes(previewType)
    ) {
      URL.revokeObjectURL(previewUrl);
    }
    if (nutrientInstance) {
      nutrientInstance.unload(nutrientContainer);
    }
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
            {formatFileSize(file.size_bytes || file.size || 0)}
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
            <i class="bi bi-chevron-left" aria-hidden="true"></i>
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
            <i class="bi bi-chevron-right" aria-hidden="true"></i>
          </button>
          <div class="divider-v"></div>
        {/if}

        <button
          onclick={toggleFavorite}
          class="action-btn favorite"
          title="Toggle Favorite"
        >
          <i class="bi bi-star-fill" aria-hidden="true"></i>
        </button>
        <button
          onclick={downloadFile}
          class="action-btn download"
          title="Download"
        >
          <i class="bi bi-download" aria-hidden="true"></i>
        </button>
        <button onclick={renameFile} class="action-btn rename" title="Rename">
          <i class="bi bi-pencil-square" aria-hidden="true"></i>
        </button>
        <button onclick={moveFile} class="action-btn move" title="Move">
          <i class="bi bi-folder2-open" aria-hidden="true"></i>
        </button>
        <button onclick={copyFile} class="action-btn copy" title="Copy">
          <i class="bi bi-files" aria-hidden="true"></i>
        </button>
        <button onclick={shareFile} class="action-btn share" title="Share">
          <i class="bi bi-share-fill" aria-hidden="true"></i>
        </button>
        <button
          onclick={openVersionHistory}
          class="action-btn history"
          title="History"
        >
          <i class="bi bi-clock-history" aria-hidden="true"></i>
        </button>
        <button
          onclick={() => (showMetadata = !showMetadata)}
          class="action-btn metadata {showMetadata ? 'active' : ''}"
          title="Toggle Metadata"
        >
          <i class="bi bi-info-circle-fill" aria-hidden="true"></i>
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
        <button aria-label="Delete"><i class="bi bi-trash" aria-hidden="true"></i></button>
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
            <i class="bi bi-exclamation-triangle-fill" aria-hidden="true"></i>
            <p>{error}</p>
            <button onclick={downloadFile} class="btn-download">
              <i class="bi bi-download" aria-hidden="true"></i>
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
            <i class="bi bi-music-note-beamed file-type-icon" aria-hidden="true"></i>
            <audio src={previewUrl} controls class="preview-audio"></audio>
          </div>
        {:else if previewType === "pdf"}
          <div class="preview-pdf-container">
            <iframe src={previewUrl} title={file.name} class="preview-pdf"
            ></iframe>
          </div>
        {:else if previewType === "code"}
          <div class="preview-code-container">
            <div class="code-header">
              <div class="code-language">
                <i class="bi bi-code-slash" aria-hidden="true"></i>
                <span
                  >{file.name.split(".").pop()?.toUpperCase() || "TEXT"}</span
                >
              </div>
              <div class="code-actions">
                <button
                  class="code-action-btn"
                  onclick={() => copyToClipboard(previewUrl)}
                  title="Copy Code"
                >
                  <i class="bi bi-clipboard" aria-hidden="true"></i>
                </button>
              </div>
            </div>
            <pre class="preview-code"><code
                class="language-{file.name.split('.').pop()?.toLowerCase()}"
                >{@html previewUrl}</code
              ></pre>
          </div>
        {:else if previewType === "excel"}
          <div class="preview-excel-container">
            {#if excelData}
              <div class="excel-content">
                {@html excelData.html}
              </div>
            {/if}

            <!-- Sheet Tabs at Bottom (like Excel) -->
            {#if excelData && excelData.sheetNames.length > 0}
              <div class="excel-sheet-bar">
                <div class="sheet-navigation">
                  <button
                    class="nav-arrow"
                    disabled={activeSheet === 0}
                    onclick={() => switchSheet(activeSheet - 1)}
                    title="Previous Sheet"
                  >
                    <i class="bi bi-chevron-left" aria-hidden="true"></i>
                  </button>
                  <button
                    class="nav-arrow"
                    disabled={activeSheet === excelData.sheetNames.length - 1}
                    onclick={() => switchSheet(activeSheet + 1)}
                    title="Next Sheet"
                  >
                    <i class="bi bi-chevron-right" aria-hidden="true"></i>
                  </button>
                </div>
                <div class="excel-sheet-tabs">
                  {#each excelData.sheetNames as sheetName, index}
                    <button
                      class="sheet-tab {activeSheet === index ? 'active' : ''}"
                      onclick={() => switchSheet(index)}
                      title={sheetName}
                    >
                      <i class="bi bi-file-earmark-spreadsheet" aria-hidden="true"></i>
                      {sheetName}
                    </button>
                  {/each}
                </div>
                <div class="sheet-info">
                  Sheet {activeSheet + 1} of {excelData.sheetNames.length}
                </div>
              </div>
            {/if}
          </div>
        {:else if previewType === "word"}
          <div class="preview-word-container">
            <div class="word-toolbar">
              <i class="bi bi-file-earmark-word text-2xl text-blue-600" aria-hidden="true"></i>
              <span class="font-semibold">Word Document</span>
            </div>
            <div class="word-content">
              {@html previewUrl}
            </div>
          </div>
        {:else if previewType === "office-viewer"}
          <div class="preview-office-container">
            <div class="office-toolbar">
              <i class="bi bi-file-earmark-slides text-2xl text-orange-600" aria-hidden="true"></i>
              <span class="font-semibold">PowerPoint Presentation</span>
            </div>
            <iframe
              src="https://view.officeapps.live.com/op/embed.aspx?src={encodeURIComponent(
                previewUrl
              )}"
              class="office-iframe"
              title={file.name}
            ></iframe>
          </div>
        {:else}
          <div class="preview-unsupported">
            <i class="bi bi-file-earmark-x file-type-icon" aria-hidden="true"></i>
            <h4>Preview not available</h4>
            <p>
              {file.name.split(".").pop()?.toUpperCase() || "This"} files cannot
              be previewed
            </p>
            <button onclick={downloadFile} class="btn-download">
              <i class="bi bi-download" aria-hidden="true"></i>
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
              <i class="bi bi-info-circle" aria-hidden="true"></i>
              <span>Details</span>
            </button>
            <button
              class="metadata-tab {activeMetadataTab === 'comments'
                ? 'active'
                : ''}"
              onclick={() => (activeMetadataTab = "comments")}
            >
              <i class="bi bi-chat-dots" aria-hidden="true"></i>
              <span>Comments</span>
            </button>
            <button
              class="metadata-tab {activeMetadataTab === 'tags'
                ? 'active'
                : ''}"
              onclick={() => (activeMetadataTab = "tags")}
            >
              <i class="bi bi-tags" aria-hidden="true"></i>
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
                    {formatFileSize(file.size_bytes || file.size || 0)}
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
                      <i class="bi bi-chat-dots" aria-hidden="true"></i>
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
                    <i class="bi bi-send-fill" aria-hidden="true"></i>
                    Send
                  </button>
                </div>
              </div>
            {:else if activeMetadataTab === "tags"}
              <div class="tags-panel">
                <div class="tags-list">
                  {#if tags.length === 0}
                    <div class="empty-state">
                      <i class="bi bi-tags" aria-hidden="true"></i>
                      <p>No tags yet</p>
                    </div>
                  {:else}
                    {#each tags as tag}
                      <div class="tag-chip">
                        <i class="bi bi-tag-fill" aria-hidden="true"></i>
                        <span>{tag}</span>
                        <button class="tag-remove" aria-label="Remove tag">
                          <i class="bi bi-x" aria-hidden="true"></i>
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
                  <button aria-label="Add" onclick={addTag} class="btn-add"><i class="bi bi-plus-lg" aria-hidden="true"></i>
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

  /* Code Preview with Syntax Highlighting */
  .preview-code-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: #2d2d2d;
  }

  .code-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1.25rem;
    background: #1e1e1e;
    border-bottom: 1px solid #3d3d3d;
  }

  .code-language {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #569cd6;
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.5px;
  }

  .code-language i {
    font-size: 1.125rem;
  }

  .code-actions {
    display: flex;
    gap: 0.5rem;
  }

  .code-action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: 1px solid #3d3d3d;
    border-radius: 0.25rem;
    color: #cccccc;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .code-action-btn:hover {
    background: #3d3d3d;
    border-color: #569cd6;
    color: #569cd6;
  }

  .preview-code {
    margin: 0;
    padding: 1.5rem;
    flex: 1;
    overflow: auto;
    background: #1e1e1e;
    font-family: "Consolas", "Monaco", "Courier New", monospace;
    font-size: 0.875rem;
    line-height: 1.6;
    color: #d4d4d4;
  }

  .preview-code code {
    background: transparent !important;
    padding: 0 !important;
    border-radius: 0 !important;
    display: block;
  }

  /* Prism theme overrides for better dark mode */
  .preview-code :global(.token.comment),
  .preview-code :global(.token.prolog),
  .preview-code :global(.token.doctype),
  .preview-code :global(.token.cdata) {
    color: #6a9955;
  }

  .preview-code :global(.token.punctuation) {
    color: #d4d4d4;
  }

  .preview-code :global(.token.property),
  .preview-code :global(.token.tag),
  .preview-code :global(.token.boolean),
  .preview-code :global(.token.number),
  .preview-code :global(.token.constant),
  .preview-code :global(.token.symbol),
  .preview-code :global(.token.deleted) {
    color: #b5cea8;
  }

  .preview-code :global(.token.selector),
  .preview-code :global(.token.attr-name),
  .preview-code :global(.token.string),
  .preview-code :global(.token.char),
  .preview-code :global(.token.builtin),
  .preview-code :global(.token.inserted) {
    color: #ce9178;
  }

  .preview-code :global(.token.operator),
  .preview-code :global(.token.entity),
  .preview-code :global(.token.url),
  .preview-code :global(.language-css .token.string),
  .preview-code :global(.style .token.string) {
    color: #d4d4d4;
  }

  .preview-code :global(.token.atrule),
  .preview-code :global(.token.attr-value),
  .preview-code :global(.token.keyword) {
    color: #569cd6;
  }

  .preview-code :global(.token.function),
  .preview-code :global(.token.class-name) {
    color: #dcdcaa;
  }

  .preview-code :global(.token.regex),
  .preview-code :global(.token.important),
  .preview-code :global(.token.variable) {
    color: #9cdcfe;
  }

  /* Excel Preview */
  .preview-excel-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* Excel Sheet Bar (Bottom) */
  .excel-sheet-bar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0.75rem;
    background: var(--md-sys-color-surface-container-low);
    border-top: 1px solid var(--md-sys-color-outline-variant);
    min-height: 48px;
  }

  .sheet-navigation {
    display: flex;
    gap: 0.25rem;
    flex-shrink: 0;
  }

  .nav-arrow {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 0.25rem;
    color: var(--md-sys-color-on-surface-variant);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .nav-arrow:hover:not(:disabled) {
    background: var(--md-sys-color-surface-container-high);
    border-color: var(--md-sys-color-primary);
    color: var(--md-sys-color-primary);
  }

  .nav-arrow:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .excel-sheet-tabs {
    display: flex;
    gap: 0.125rem;
    overflow-x: auto;
    overflow-y: hidden;
    flex: 1;
    padding: 0 0.25rem;
    /* Custom scrollbar styling */
    scrollbar-width: thin;
    scrollbar-color: var(--md-sys-color-outline-variant) transparent;
  }

  .excel-sheet-tabs::-webkit-scrollbar {
    height: 6px;
  }

  .excel-sheet-tabs::-webkit-scrollbar-track {
    background: transparent;
  }

  .excel-sheet-tabs::-webkit-scrollbar-thumb {
    background: var(--md-sys-color-outline-variant);
    border-radius: 3px;
  }

  .excel-sheet-tabs::-webkit-scrollbar-thumb:hover {
    background: var(--md-sys-color-outline);
  }

  .sheet-tab {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.875rem;
    background: var(--md-sys-color-surface);
    border: 1px solid var(--md-sys-color-outline-variant);
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    color: var(--md-sys-color-on-surface-variant);
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
    flex-shrink: 0;
    font-weight: 500;
  }

  .sheet-tab i {
    font-size: 0.875rem;
    opacity: 0.7;
  }

  .sheet-tab:hover {
    background: var(--md-sys-color-surface-container-highest);
    border-color: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-surface);
    transform: translateY(-2px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  .sheet-tab:hover i {
    opacity: 1;
  }

  .sheet-tab.active {
    background: linear-gradient(
      135deg,
      var(--md-sys-color-primary),
      var(--md-sys-color-tertiary)
    );
    border-color: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
    font-weight: 600;
    box-shadow: 0 4px 12px rgba(103, 80, 164, 0.4);
    transform: translateY(-2px);
  }

  .sheet-tab.active i {
    opacity: 1;
  }

  .sheet-tab.active:hover {
    background: linear-gradient(
      135deg,
      var(--md-sys-color-primary),
      var(--md-sys-color-secondary)
    );
    transform: translateY(-3px);
    box-shadow: 0 6px 16px rgba(103, 80, 164, 0.5);
  }

  .sheet-info {
    display: flex;
    align-items: center;
    padding: 0.375rem 0.75rem;
    background: var(--md-sys-color-surface-container);
    border-radius: 0.375rem;
    font-size: 0.75rem;
    color: var(--md-sys-color-on-surface-variant);
    white-space: nowrap;
    flex-shrink: 0;
    font-weight: 500;
  }

  .excel-content {
    flex: 1;
    overflow: auto;
    padding: 1rem;
    background: var(--md-sys-color-surface);
  }

  .excel-content :global(table) {
    border-collapse: collapse;
    width: 100%;
    font-size: 0.875rem;
    background: white;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    color: #000 !important;
  }

  .excel-content :global(th),
  .excel-content :global(td) {
    border: 1px solid #e0e0e0;
    padding: 0.5rem 0.75rem;
    text-align: left;
    color: #000 !important;
    background: white;
  }

  .excel-content :global(th) {
    background: #f5f5f5 !important;
    color: #000 !important;
    font-weight: 600;
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .excel-content :global(tr:hover td) {
    background: #f9f9f9 !important;
  }

  /* Override any inline styles from Sheet.js */
  .excel-content :global(table *) {
    color: #000 !important;
  }

  .excel-content :global(a) {
    color: #1976d2 !important;
    text-decoration: underline;
  }

  /* Word Preview */
  .preview-word-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .word-toolbar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: var(--md-sys-color-surface-container);
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .word-content {
    flex: 1;
    overflow: auto;
    padding: 2rem;
    background: white;
    color: #000;
    font-family: "Calibri", "Arial", sans-serif;
    line-height: 1.6;
  }

  :global(.dark) .word-content {
    background: #1e1e1e;
    color: #e0e0e0;
  }

  .word-content :global(p) {
    margin-bottom: 1rem;
  }

  .word-content :global(h1),
  .word-content :global(h2),
  .word-content :global(h3) {
    margin-top: 1.5rem;
    margin-bottom: 0.75rem;
    font-weight: 600;
  }

  .word-content :global(img) {
    max-width: 100%;
    height: auto;
    margin: 1rem 0;
  }

  .word-content :global(table) {
    border-collapse: collapse;
    width: 100%;
    margin: 1rem 0;
  }

  .word-content :global(table td),
  .word-content :global(table th) {
    border: 1px solid #ddd;
    padding: 0.5rem;
  }

  /* Office Viewer (PowerPoint) */
  .preview-office-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .office-toolbar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: var(--md-sys-color-surface-container);
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .office-iframe {
    flex: 1;
    width: 100%;
    height: 100%;
    border: none;
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
