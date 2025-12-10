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
    class="fixed top-16 left-0 bottom-0 bg-black/10 dark:bg-black/40 backdrop-blur-sm z-[35] border-none p-0 cursor-pointer transition-all duration-300 animate-fade-in {isFullscreen ? 'right-0 bg-black/60' : 'right-[480px] xl:right-[480px] lg:right-[420px] md:right-[380px] sm:right-0'}"
    onclick={onClose}
    aria-label="Close preview"
  ></button>

  <!-- Preview Panel -->
  <div
    class="fixed top-16 right-0 h-[calc(100vh-4rem)] bg-base-100 border-l border-base-300 shadow-[-4px_0_24px_rgba(0,0,0,0.08)] dark:shadow-[-8px_0_32px_rgba(0,0,0,0.5)] z-40 flex flex-col overflow-hidden transition-all duration-300 animate-slide-in-right {isFullscreen ? 'left-0 w-screen max-w-full' : 'w-[480px] xl:w-[480px] lg:w-[420px] md:w-[380px] sm:w-screen max-w-[calc(100vw-280px)] sm:max-w-full sm:top-0'}"
  >
    <!-- Header with actions -->
    <div class="flex items-center justify-between p-4 px-5 border-b border-base-300 bg-base-200/50 gap-4 flex-wrap">
      <div class="flex items-center gap-3.5 min-w-0 flex-1">
        <i
          class="bi {getFileIcon(file.name)} text-[1.75rem]"
          style="color: {getFileIconColor(file.name)}"
        ></i>
        <div class="min-w-0 flex-1">
          <h3 class="m-0 text-base font-semibold text-base-content overflow-hidden text-ellipsis whitespace-nowrap">{file.name}</h3>
          <p class="mt-0.5 mb-0 text-[0.8125rem] text-base-content/60">
            {formatFileSize(file.size_bytes || file.size || 0)}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-1.5 flex-wrap">
        {#if allFiles.length > 1}
          <button
            onclick={navigatePrev}
            class="btn btn-ghost btn-sm btn-square text-base-content/60 hover:text-base-content hover:bg-base-300"
            disabled={currentIndex === 0}
            title="Previous (←)"
          >
            <i class="bi bi-chevron-left text-lg" aria-hidden="true"></i>
          </button>
          <span class="text-xs text-base-content/60 px-1">
            {currentIndex + 1} / {allFiles.length}
          </span>
          <button
            onclick={navigateNext}
            class="btn btn-ghost btn-sm btn-square text-base-content/60 hover:text-base-content hover:bg-base-300"
            disabled={currentIndex === allFiles.length - 1}
            title="Next (→)"
          >
            <i class="bi bi-chevron-right text-lg" aria-hidden="true"></i>
          </button>
          <div class="w-px h-6 bg-base-300 mx-1"></div>
        {/if}

        <button
          onclick={toggleFavorite}
          class="btn btn-ghost btn-sm btn-square text-amber-500 hover:bg-amber-500/10"
          title="Toggle Favorite"
        >
          <i class="bi bi-star-fill text-lg" aria-hidden="true"></i>
        </button>
        <button
          onclick={downloadFile}
          class="btn btn-ghost btn-sm btn-square text-green-500 hover:bg-green-500/10"
          title="Download"
        >
          <i class="bi bi-download text-lg" aria-hidden="true"></i>
        </button>
        <button onclick={renameFile} class="btn btn-ghost btn-sm btn-square text-blue-500 hover:bg-blue-500/10" title="Rename">
          <i class="bi bi-pencil-square text-lg" aria-hidden="true"></i>
        </button>
        <button onclick={moveFile} class="btn btn-ghost btn-sm btn-square text-purple-500 hover:bg-purple-500/10" title="Move">
          <i class="bi bi-folder2-open text-lg" aria-hidden="true"></i>
        </button>
        <button onclick={copyFile} class="btn btn-ghost btn-sm btn-square text-cyan-500 hover:bg-cyan-500/10" title="Copy">
          <i class="bi bi-files text-lg" aria-hidden="true"></i>
        </button>
        <button onclick={shareFile} class="btn btn-ghost btn-sm btn-square text-indigo-500 hover:bg-indigo-500/10" title="Share">
          <i class="bi bi-share-fill text-lg" aria-hidden="true"></i>
        </button>
        <button
          onclick={openVersionHistory}
          class="btn btn-ghost btn-sm btn-square text-slate-500 hover:bg-slate-500/10"
          title="History"
        >
          <i class="bi bi-clock-history text-lg" aria-hidden="true"></i>
        </button>
        <button
          onclick={() => (showMetadata = !showMetadata)}
          class="btn btn-ghost btn-sm btn-square text-amber-700 hover:bg-amber-700/10 {showMetadata ? 'bg-amber-700/15' : ''}"
          title="Toggle Metadata"
        >
          <i class="bi bi-info-circle-fill text-lg" aria-hidden="true"></i>
        </button>
        <button
          onclick={toggleFullscreen}
          class="btn btn-ghost btn-sm btn-square text-violet-600 hover:bg-violet-600/10 {isFullscreen ? 'bg-violet-600/15' : ''}"
          title={isFullscreen ? "Exit Fullscreen" : "Fullscreen"}
        >
          <i
            class="bi bi-{isFullscreen
              ? 'fullscreen-exit'
              : 'arrows-fullscreen'} text-lg"
          ></i>
        </button>
        <div class="w-px h-6 bg-base-300 mx-1"></div>
        <button class="btn btn-ghost btn-sm btn-square text-error hover:bg-error/10" aria-label="Delete" onclick={deleteFile}>
          <i class="bi bi-trash text-lg" aria-hidden="true"></i>
        </button>
      </div>
    </div>

    <!-- Preview Content with optional metadata sidebar -->
    <div class="flex flex-1 overflow-hidden {showMetadata ? '' : ''}">
      <!-- Main Preview Area -->
      <div class="flex-1 flex items-center justify-center p-8 overflow-auto bg-base-200/30">
        {#if loading}
          <div class="flex flex-col items-center justify-center gap-4 text-center text-base-content/60">
            <div class="loading loading-spinner loading-lg text-primary"></div>
            <p>Loading preview...</p>
          </div>
        {:else if error}
          <div class="flex flex-col items-center justify-center gap-4 text-center text-base-content/60">
            <i class="bi bi-exclamation-triangle-fill text-6xl opacity-30" aria-hidden="true"></i>
            <p>{error}</p>
            <button onclick={downloadFile} class="btn btn-primary gap-2">
              <i class="bi bi-download" aria-hidden="true"></i>
              Download File
            </button>
          </div>
        {:else if previewType === "image"}
          <div class="w-full h-full flex items-center justify-center">
            <img src={previewUrl} alt={file.name} class="max-w-full max-h-full object-contain rounded-lg shadow-lg" />
          </div>
        {:else if previewType === "video"}
          <div class="w-full h-full flex items-center justify-center">
            <video src={previewUrl} controls class="max-w-full max-h-full rounded-lg shadow-lg">
              <track kind="captions" />
            </video>
          </div>
        {:else if previewType === "audio"}
          <div class="flex flex-col items-center gap-8">
            <i class="bi bi-music-note-beamed text-6xl opacity-30" aria-hidden="true"></i>
            <audio src={previewUrl} controls class="w-full max-w-[500px]"></audio>
          </div>
        {:else if previewType === "pdf"}
          <div class="w-full h-full flex items-center justify-center">
            <iframe src={previewUrl} title={file.name} class="w-full h-full border-none rounded-lg"></iframe>
          </div>
        {:else if previewType === "code"}
          <div class="w-full h-full flex flex-col overflow-hidden bg-[#2d2d2d] rounded-lg">
            <div class="flex items-center justify-between px-5 py-3 bg-[#1e1e1e] border-b border-[#3d3d3d]">
              <div class="flex items-center gap-2 text-[#569cd6] text-[0.8125rem] font-semibold tracking-wide">
                <i class="bi bi-code-slash text-lg" aria-hidden="true"></i>
                <span>{file.name.split(".").pop()?.toUpperCase() || "TEXT"}</span>
              </div>
              <div class="flex gap-2">
                <button
                  class="w-8 h-8 flex items-center justify-center bg-transparent border border-[#3d3d3d] rounded text-[#cccccc] cursor-pointer hover:bg-[#3d3d3d] hover:border-[#569cd6] hover:text-[#569cd6] transition-all duration-200"
                  onclick={() => copyToClipboard(previewUrl)}
                  title="Copy Code"
                >
                  <i class="bi bi-clipboard" aria-hidden="true"></i>
                </button>
              </div>
            </div>
            <pre class="m-0 p-6 flex-1 overflow-auto bg-[#1e1e1e] font-mono text-sm leading-relaxed text-[#d4d4d4]"><code class="!bg-transparent !p-0 !rounded-none block">{@html previewUrl}</code></pre>
          </div>
        {:else if previewType === "excel"}
          <div class="w-full h-full flex flex-col overflow-hidden">
            {#if excelData}
              <div class="flex-1 overflow-auto p-4 bg-base-100 excel-content">
                {@html excelData.html}
              </div>
            {/if}

            <!-- Sheet Tabs at Bottom (like Excel) -->
            {#if excelData && excelData.sheetNames.length > 0}
              <div class="flex items-center gap-3 px-3 py-2 bg-base-200 border-t border-base-300 min-h-12">
                <div class="flex gap-1 flex-shrink-0">
                  <button
                    class="w-8 h-8 flex items-center justify-center bg-transparent border border-base-300 rounded text-base-content/60 cursor-pointer hover:bg-base-300 hover:border-primary hover:text-primary disabled:opacity-30 disabled:cursor-not-allowed transition-all duration-150"
                    disabled={activeSheet === 0}
                    onclick={() => switchSheet(activeSheet - 1)}
                    title="Previous Sheet"
                  >
                    <i class="bi bi-chevron-left" aria-hidden="true"></i>
                  </button>
                  <button
                    class="w-8 h-8 flex items-center justify-center bg-transparent border border-base-300 rounded text-base-content/60 cursor-pointer hover:bg-base-300 hover:border-primary hover:text-primary disabled:opacity-30 disabled:cursor-not-allowed transition-all duration-150"
                    disabled={activeSheet === excelData.sheetNames.length - 1}
                    onclick={() => switchSheet(activeSheet + 1)}
                    title="Next Sheet"
                  >
                    <i class="bi bi-chevron-right" aria-hidden="true"></i>
                  </button>
                </div>
                <div class="flex gap-0.5 overflow-x-auto flex-1 px-1 scrollbar-thin">
                  {#each excelData.sheetNames as sheetName, index}
                    <button
                      class="flex items-center gap-1.5 px-3.5 py-2 rounded-md text-[0.8125rem] font-medium whitespace-nowrap flex-shrink-0 transition-all duration-200 {activeSheet === index ? 'bg-gradient-to-br from-primary to-secondary text-primary-content shadow-md -translate-y-0.5' : 'bg-base-100 border border-base-300 text-base-content/60 hover:bg-base-300 hover:border-primary hover:text-base-content hover:-translate-y-0.5 hover:shadow-md'}"
                      onclick={() => switchSheet(index)}
                      title={sheetName}
                    >
                      <i class="bi bi-file-earmark-spreadsheet text-sm opacity-70" aria-hidden="true"></i>
                      {sheetName}
                    </button>
                  {/each}
                </div>
                <div class="flex items-center px-3 py-1.5 bg-base-300 rounded-md text-xs text-base-content/60 whitespace-nowrap flex-shrink-0 font-medium">
                  Sheet {activeSheet + 1} of {excelData.sheetNames.length}
                </div>
              </div>
            {/if}
          </div>
        {:else if previewType === "word"}
          <div class="w-full h-full flex flex-col overflow-hidden rounded-lg">
            <div class="flex items-center gap-3 px-6 py-4 bg-base-200 border-b border-base-300">
              <i class="bi bi-file-earmark-word text-2xl text-green-600" aria-hidden="true"></i>
              <span class="font-semibold">Word Document</span>
            </div>
            <div class="flex-1 overflow-auto p-8 bg-white dark:bg-[#1e1e1e] text-black dark:text-gray-200 font-[Calibri,Arial,sans-serif] leading-relaxed word-content">
              {@html previewUrl}
            </div>
          </div>
        {:else if previewType === "office-viewer"}
          <div class="w-full h-full flex flex-col overflow-hidden rounded-lg">
            <div class="flex items-center gap-3 px-6 py-4 bg-base-200 border-b border-base-300">
              <i class="bi bi-file-earmark-slides text-2xl text-orange-600" aria-hidden="true"></i>
              <span class="font-semibold">PowerPoint Presentation</span>
            </div>
            <iframe
              src="https://view.officeapps.live.com/op/embed.aspx?src={encodeURIComponent(previewUrl)}"
              class="flex-1 w-full h-full border-none"
              title={file.name}
            ></iframe>
          </div>
        {:else}
          <div class="flex flex-col items-center justify-center gap-4 text-center text-base-content/60">
            <i class="bi bi-file-earmark-x text-6xl opacity-30" aria-hidden="true"></i>
            <h4 class="text-lg font-medium">Preview not available</h4>
            <p>{file.name.split(".").pop()?.toUpperCase() || "This"} files cannot be previewed</p>
            <button onclick={downloadFile} class="btn btn-primary gap-2">
              <i class="bi bi-download" aria-hidden="true"></i>
              Download to View
            </button>
          </div>
        {/if}
      </div>

      <!-- Metadata Sidebar -->
      {#if showMetadata}
        <div class="w-80 lg:w-72 sm:w-full sm:max-h-[40vh] border-l border-base-300 flex flex-col bg-base-200/50">
          <div class="flex border-b border-base-300 bg-base-100">
            <button
              class="flex-1 flex flex-col items-center gap-1 py-3 px-2 bg-transparent border-none border-b-2 cursor-pointer transition-all duration-200 text-xs {activeMetadataTab === 'details' ? 'text-primary border-b-primary bg-base-200' : 'text-base-content/60 border-transparent hover:bg-base-200 hover:text-base-content'}"
              onclick={() => (activeMetadataTab = "details")}
            >
              <i class="bi bi-info-circle text-lg" aria-hidden="true"></i>
              <span>Details</span>
            </button>
            <button
              class="flex-1 flex flex-col items-center gap-1 py-3 px-2 bg-transparent border-none border-b-2 cursor-pointer transition-all duration-200 text-xs {activeMetadataTab === 'comments' ? 'text-primary border-b-primary bg-base-200' : 'text-base-content/60 border-transparent hover:bg-base-200 hover:text-base-content'}"
              onclick={() => (activeMetadataTab = "comments")}
            >
              <i class="bi bi-chat-dots text-lg" aria-hidden="true"></i>
              <span>Comments</span>
            </button>
            <button
              class="flex-1 flex flex-col items-center gap-1 py-3 px-2 bg-transparent border-none border-b-2 cursor-pointer transition-all duration-200 text-xs {activeMetadataTab === 'tags' ? 'text-primary border-b-primary bg-base-200' : 'text-base-content/60 border-transparent hover:bg-base-200 hover:text-base-content'}"
              onclick={() => (activeMetadataTab = "tags")}
            >
              <i class="bi bi-tags text-lg" aria-hidden="true"></i>
              <span>Tags</span>
            </button>
          </div>

          <div class="flex-1 overflow-y-auto p-4">
            {#if activeMetadataTab === "details"}
              <div class="flex flex-col gap-3">
                <div class="flex justify-between p-2.5 bg-base-300/50 rounded-lg gap-2">
                  <span class="font-medium text-base-content/60 text-[0.8125rem]">Name</span>
                  <span class="text-base-content text-[0.8125rem] text-right break-words">{file.name}</span>
                </div>
                <div class="flex justify-between p-2.5 bg-base-300/50 rounded-lg gap-2">
                  <span class="font-medium text-base-content/60 text-[0.8125rem]">Size</span>
                  <span class="text-base-content text-[0.8125rem] text-right break-words">
                    {formatFileSize(file.size_bytes || file.size || 0)}
                  </span>
                </div>
                <div class="flex justify-between p-2.5 bg-base-300/50 rounded-lg gap-2">
                  <span class="font-medium text-base-content/60 text-[0.8125rem]">Type</span>
                  <span class="text-base-content text-[0.8125rem] text-right break-words">
                    {file.name.split(".").pop()?.toUpperCase() || "Unknown"}
                  </span>
                </div>
                <div class="flex justify-between p-2.5 bg-base-300/50 rounded-lg gap-2">
                  <span class="font-medium text-base-content/60 text-[0.8125rem]">Modified</span>
                  <span class="text-base-content text-[0.8125rem] text-right break-words">
                    {file.modified_at ? new Date(file.modified_at).toLocaleString() : "Unknown"}
                  </span>
                </div>
                <div class="flex justify-between p-2.5 bg-base-300/50 rounded-lg gap-2">
                  <span class="font-medium text-base-content/60 text-[0.8125rem]">Owner</span>
                  <span class="text-base-content text-[0.8125rem] text-right break-words">{file.owner || "Me"}</span>
                </div>
              </div>
            {:else if activeMetadataTab === "comments"}
              <div class="flex flex-col gap-3">
                <div class="flex flex-col gap-3 mb-4">
                  {#if comments.length === 0}
                    <div class="flex flex-col items-center justify-center py-8 px-4 gap-2 text-base-content/60 opacity-60">
                      <i class="bi bi-chat-dots text-4xl" aria-hidden="true"></i>
                      <p>No comments yet</p>
                    </div>
                  {:else}
                    {#each comments as comment}
                      <div class="p-3 bg-base-300/50 rounded-lg border-l-[3px] border-l-primary">
                        <p class="m-0 mb-2 text-sm text-base-content">{comment.text}</p>
                        <span class="text-xs text-base-content/60">
                          {new Date(comment.timestamp).toLocaleString()}
                        </span>
                      </div>
                    {/each}
                  {/if}
                </div>
                <div class="flex gap-2 mt-auto">
                  <textarea
                    bind:value={newComment}
                    placeholder="Add a comment..."
                    rows="3"
                    class="textarea textarea-bordered flex-1 text-sm"
                  ></textarea>
                  <button onclick={addComment} class="btn btn-primary btn-sm gap-1.5">
                    <i class="bi bi-send-fill" aria-hidden="true"></i>
                    Send
                  </button>
                </div>
              </div>
            {:else if activeMetadataTab === "tags"}
              <div class="flex flex-col gap-3">
                <div class="flex flex-wrap gap-2 mb-4">
                  {#if tags.length === 0}
                    <div class="flex flex-col items-center justify-center py-8 px-4 gap-2 text-base-content/60 opacity-60 w-full">
                      <i class="bi bi-tags text-4xl" aria-hidden="true"></i>
                      <p>No tags yet</p>
                    </div>
                  {:else}
                    {#each tags as tag}
                      <div class="badge badge-secondary gap-1.5 py-3 px-2.5">
                        <i class="bi bi-tag-fill" aria-hidden="true"></i>
                        <span>{tag}</span>
                        <button class="btn btn-ghost btn-xs p-0.5 opacity-70 hover:opacity-100" aria-label="Remove tag">
                          <i class="bi bi-x" aria-hidden="true"></i>
                        </button>
                      </div>
                    {/each}
                  {/if}
                </div>
                <div class="flex gap-2 mt-auto">
                  <input
                    type="text"
                    bind:value={newTag}
                    placeholder="Add a tag..."
                    onkeydown={(e) => e.key === "Enter" && addTag()}
                    class="input input-bordered input-sm flex-1"
                  />
                  <button aria-label="Add" onclick={addTag} class="btn btn-primary btn-sm gap-1.5">
                    <i class="bi bi-plus-lg" aria-hidden="true"></i>
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
  /* Animations */
  @keyframes slide-in-right {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  :global(.animate-slide-in-right) {
    animation: slide-in-right 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
  }

  :global(.animate-fade-in) {
    animation: fade-in 0.3s ease-out;
  }

  /* Excel content table styling */
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

  .excel-content :global(table *) {
    color: #000 !important;
  }

  .excel-content :global(a) {
    color: #1976d2 !important;
    text-decoration: underline;
  }

  /* Word content styling */
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

  /* Scrollbar styling */
  .scrollbar-thin {
    scrollbar-width: thin;
    scrollbar-color: var(--fallback-bc, oklch(var(--bc)/0.3)) transparent;
  }

  .scrollbar-thin::-webkit-scrollbar {
    height: 6px;
  }

  .scrollbar-thin::-webkit-scrollbar-track {
    background: transparent;
  }

  .scrollbar-thin::-webkit-scrollbar-thumb {
    background: var(--fallback-bc, oklch(var(--bc)/0.3));
    border-radius: 3px;
  }
</style>
