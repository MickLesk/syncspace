<script>
  import { onMount, onDestroy } from "svelte";
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons";
  import api from "../../lib/api";
  import { success, error as errorToast } from "../../stores/toast";
  import PreviewTab from "./PreviewTab.svelte";
  import CommentsTab from "./CommentsTab.svelte";
  import TagsTab from "./TagsTab.svelte";
  import DetailsTab from "./DetailsTab.svelte";
  import MetadataTab from "./MetadataTab.svelte";

  let {
    visible = $bindable(false),
    file = $bindable(null),
    allFiles = [],
    currentIndex = 0,
  } = $props();
  let previewUrl = $state(null),
    previewType = $state(null),
    loading = $state(false),
    error = $state(null),
    activeTab = $state("preview"),
    isFullscreen = $state(false),
    isAnimating = $state(false);

  $effect(() => {
    if (visible) {
      // Trigger animation with a small delay
      setTimeout(() => {
        isAnimating = true;
      }, 10);
    } else {
      isAnimating = false;
      // Cleanup after animation completes
      setTimeout(() => {
        if (previewUrl && previewType !== "text")
          URL.revokeObjectURL(previewUrl);
        previewUrl = null;
        previewType = null;
        error = null;
        activeTab = "preview";
      }, 300);
    }
  });

  $effect(() => {
    if (visible && file) loadPreview();
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
      } else if (["stl", "obj"].includes(ext)) {
        previewType = "3d-model";
        previewUrl = URL.createObjectURL(blob);
      } else if (ext === "pdf") {
        previewType = "pdf";
        previewUrl = URL.createObjectURL(blob);
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
        ].includes(ext)
      ) {
        previewType = "text";
        const text = await blob.text();
        previewUrl = text.substring(0, 10000);
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

  function close() {
    visible = false;
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
      close();
      window.location.reload();
    } catch (err) {
      errorToast("Failed to delete file");
    }
  }

  function shareFile() {
    window.dispatchEvent(
      new CustomEvent("openShareModal", { detail: { file } })
    );
    close();
  }
  function openVersionHistory() {
    window.dispatchEvent(
      new CustomEvent("openVersionHistoryModal", { detail: { file } })
    );
    close();
  }

  function handleKeydown(e) {
    if (!visible) return;
    if (e.key === "Escape") {
      if (isFullscreen) {
        isFullscreen = false;
      } else {
        close();
      }
    }
    if (e.key === "ArrowLeft") navigatePrev();
    if (e.key === "ArrowRight") navigateNext();
    if (e.key === "f" && e.ctrlKey) {
      e.preventDefault();
      toggleFullscreen();
    }
  }

  function navigatePrev() {
    if (!allFiles || allFiles.length <= 1) return;
    const newIndex = currentIndex > 0 ? currentIndex - 1 : allFiles.length - 1;
    file = allFiles[newIndex];
    currentIndex = newIndex;
  }

  function navigateNext() {
    if (!allFiles || allFiles.length <= 1) return;
    const newIndex = currentIndex < allFiles.length - 1 ? currentIndex + 1 : 0;
    file = allFiles[newIndex];
    currentIndex = newIndex;
  }

  function toggleFullscreen() {
    isFullscreen = !isFullscreen;
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
  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });
  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
    if (previewUrl && previewType !== "text") URL.revokeObjectURL(previewUrl);
  });
</script>

{#if visible}
  <!-- Backdrop with blur - fades in -->
  <div
    class="fixed inset-0 bg-black/40 backdrop-blur-sm z-50 transition-opacity duration-300"
    class:opacity-0={!isAnimating}
    class:opacity-100={isAnimating}
    onclick={close}
    onkeydown={(e) => e.key === "Escape" && close()}
    role="button"
    tabindex="-1"
  ></div>

  <!-- Slide-in Panel from Right - slides in from off-screen -->
  <div
    class="fixed top-0 right-0 h-full bg-base-100 shadow-2xl z-50 flex flex-col transition-transform duration-300 ease-out {isFullscreen
      ? 'w-full'
      : 'w-[90vw] md:w-[60vw] lg:w-[50vw] xl:w-[45vw] max-w-[1200px]'}"
    class:translate-x-full={!isAnimating}
    class:translate-x-0={isAnimating}
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="0"
    style="box-shadow: -4px 0 24px rgba(0, 0, 0, 0.15);"
  >
    <!-- Collapse Button (Left Edge) -->
    <button
      onclick={close}
      class="absolute left-0 top-1/2 -translate-y-1/2 -translate-x-full bg-base-100 hover:bg-base-200 text-base-content rounded-l-lg shadow-lg px-2 py-8 transition-all duration-200 hover:px-3 group border-l border-t border-b border-base-300"
      title="Close Panel (ESC)"
      aria-label="Close preview panel"
    >
      <i
        class="bi bi-chevron-right text-xl group-hover:scale-110 transition-transform"
      ></i>
    </button>
    <!-- Header with gradient -->
    <div
      class="flex items-center justify-between gap-4 px-6 py-4 border-b border-base-300 bg-gradient-to-r from-base-100 to-base-200/50"
    >
      <div class="flex items-center gap-3 flex-1 min-w-0">
        <div
          class="w-12 h-12 rounded-xl bg-base-200 flex items-center justify-center shadow-sm"
        >
          <i
            class="bi {getFileIcon(file?.name || '')} text-2xl"
            style="color: {getFileIconColor(file?.name || '')}"
          ></i>
        </div>
        <div class="flex-1 min-w-0">
          <h2 class="text-lg font-semibold truncate">{file?.name || ""}</h2>
          <p class="text-sm text-base-content/60">
            {file?.size ? Math.round(file.size / 1024) : 0} KB
          </p>
        </div>
      </div>
      <div class="flex items-center gap-1">
        <!-- Navigation -->
        {#if allFiles && allFiles.length > 1}
          <button
            onclick={navigatePrev}
            class="btn btn-sm btn-ghost btn-square"
            title="Previous (←)"
            ><i class="bi bi-chevron-left" aria-hidden="true"></i></button
          >
          <span class="text-xs text-base-content/60 px-2"
            >{currentIndex + 1} / {allFiles.length}</span
          >
          <button
            onclick={navigateNext}
            class="btn btn-sm btn-ghost btn-square"
            title="Next (→)"
            ><i class="bi bi-chevron-right" aria-hidden="true"></i></button
          >
          <div class="divider divider-horizontal mx-1"></div>
        {/if}

        <!-- Actions -->
        <button
          onclick={toggleFavorite}
          class="btn btn-sm btn-ghost btn-square"
          title="Toggle Favorite"
          ><i class="bi bi-star" aria-hidden="true"></i></button
        >
        <button
          onclick={downloadFile}
          class="btn btn-sm btn-ghost btn-square"
          title="Download"
          ><i class="bi bi-download" aria-hidden="true"></i></button
        >
        <button
          onclick={renameFile}
          class="btn btn-sm btn-ghost btn-square"
          title="Rename"><i class="bi bi-pencil" aria-hidden="true"></i></button
        >
        <button
          onclick={moveFile}
          class="btn btn-sm btn-ghost btn-square"
          title="Move"
          ><i class="bi bi-folder2-open" aria-hidden="true"></i></button
        >
        <button
          onclick={copyFile}
          class="btn btn-sm btn-ghost btn-square"
          title="Copy"><i class="bi bi-files" aria-hidden="true"></i></button
        >
        <button
          onclick={shareFile}
          class="btn btn-sm btn-ghost btn-square"
          title="Share"><i class="bi bi-share" aria-hidden="true"></i></button
        >
        <button
          onclick={openVersionHistory}
          class="btn btn-sm btn-ghost btn-square"
          title="Version History"
          ><i class="bi bi-clock-history" aria-hidden="true"></i></button
        >
        <button
          onclick={toggleFullscreen}
          class="btn btn-sm btn-ghost btn-square"
          title="Fullscreen (Ctrl+F)"
          ><i
            class="bi bi-{isFullscreen
              ? 'fullscreen-exit'
              : 'arrows-fullscreen'}"
          ></i></button
        >
        <div class="divider divider-horizontal mx-1"></div>
        <button
          onclick={deleteFile}
          class="btn btn-sm btn-ghost btn-square text-error hover:bg-error/10"
          title="Delete"><i class="bi bi-trash" aria-hidden="true"></i></button
        >
        <button
          onclick={close}
          class="btn btn-sm btn-circle btn-ghost ml-2 hover:bg-error/10 hover:text-error"
          title="Close (ESC)"
          ><i class="bi bi-x-lg" aria-hidden="true"></i></button
        >
      </div>
    </div>

    <!-- Tabs with smoother styling -->
    <div
      class="tabs tabs-boxed bg-base-200/50 px-6 py-2 gap-1 border-b border-base-300"
    >
      <button
        class="tab transition-all duration-200 {activeTab === 'preview'
          ? 'tab-active'
          : ''}"
        onclick={() => (activeTab = "preview")}
        ><i class="bi bi-eye mr-2" aria-hidden="true"></i>Preview</button
      >
      <button
        class="tab transition-all duration-200 {activeTab === 'comments'
          ? 'tab-active'
          : ''}"
        onclick={() => (activeTab = "comments")}
        ><i class="bi bi-chat-dots mr-2" aria-hidden="true"></i>Comments</button
      >
      <button
        class="tab transition-all duration-200 {activeTab === 'tags'
          ? 'tab-active'
          : ''}"
        onclick={() => (activeTab = "tags")}
        ><i class="bi bi-tags mr-2" aria-hidden="true"></i>Tags</button
      >
      <button
        class="tab transition-all duration-200 {activeTab === 'details'
          ? 'tab-active'
          : ''}"
        onclick={() => (activeTab = "details")}
        ><i class="bi bi-info-circle mr-2" aria-hidden="true"
        ></i>Details</button
      >
      <button
        class="tab transition-all duration-200 {activeTab === 'metadata'
          ? 'tab-active'
          : ''}"
        onclick={() => (activeTab = "metadata")}
        ><i class="bi bi-card-list mr-2" aria-hidden="true"></i>Metadata</button
      >
    </div>

    <!-- Content Area with smooth scrolling -->
    <div class="flex-1 overflow-y-auto p-6 scroll-smooth">
      {#if activeTab === "preview"}<PreviewTab
          {file}
          {previewUrl}
          {previewType}
          {loading}
          {error}
        />
      {:else if activeTab === "comments"}<CommentsTab {file} />
      {:else if activeTab === "tags"}<TagsTab {file} />
      {:else if activeTab === "details"}<DetailsTab {file} />
      {:else if activeTab === "metadata"}<MetadataTab {file} />
      {/if}
    </div>

    <!-- Footer with keyboard shortcuts -->
    <div
      class="flex items-center justify-between px-6 py-3 border-t border-base-300 bg-gradient-to-r from-base-200/30 to-base-100"
    >
      <div class="text-xs text-base-content/60 flex gap-3">
        <span><kbd class="kbd kbd-xs">ESC</kbd> to close</span>
        {#if allFiles && allFiles.length > 1}
          <span
            ><kbd class="kbd kbd-xs">←</kbd><kbd class="kbd kbd-xs">→</kbd> navigate</span
          >
        {/if}
        <span><kbd class="kbd kbd-xs">Ctrl+F</kbd> fullscreen</span>
      </div>
      <div class="text-xs text-base-content/60">
        {#if allFiles && allFiles.length > 1}
          File: {currentIndex + 1} of {allFiles.length}
        {:else}
          <span class="opacity-50">Single file</span>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  /* Smooth slide-in animation */
  @keyframes slideInFromRight {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  /* Add custom scrollbar styling */
  .scroll-smooth::-webkit-scrollbar {
    width: 8px;
  }

  .scroll-smooth::-webkit-scrollbar-track {
    background: transparent;
  }

  .scroll-smooth::-webkit-scrollbar-thumb {
    background: oklch(var(--bc) / 0.2);
    border-radius: 4px;
  }

  .scroll-smooth::-webkit-scrollbar-thumb:hover {
    background: oklch(var(--bc) / 0.3);
  }
</style>
