<script>
  import { onMount, onDestroy } from "svelte";
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons";
  import api from "../../lib/api";
  import { success, error as errorToast } from "../../stores/toast";
  import PreviewTab from "./PreviewTab.svelte";
  import CommentsTab from "./CommentsTab.svelte";
  import TagsTab from "./TagsTab.svelte";
  import DetailsTab from "./DetailsTab.svelte";

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
    isFullscreen = $state(false);

  $effect(() => {
    if (!visible) {
      if (previewUrl && previewType !== "text") URL.revokeObjectURL(previewUrl);
      previewUrl = null;
      previewType = null;
      error = null;
      activeTab = "preview";
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
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center {isFullscreen
      ? 'p-0'
      : 'p-4'}"
    onclick={close}
    onkeydown={(e) => e.key === "Escape" && close()}
    role="button"
    tabindex="-1"
  >
    <div
      class="bg-base-100 rounded-2xl shadow-2xl flex flex-col {isFullscreen
        ? 'w-full h-full rounded-none'
        : 'w-full max-w-5xl max-h-[90vh]'}"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="0"
    >
      <div
        class="flex items-center justify-between gap-4 px-6 py-4 border-b border-base-300"
      >
        <div class="flex items-center gap-3 flex-1 min-w-0">
          <i
            class="bi {getFileIcon(file?.name || '')} text-2xl"
            style="color: {getFileIconColor(file?.name || '')}"
          ></i>
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
              title="Previous (←)"><i class="bi bi-chevron-left"></i></button
            >
            <span class="text-xs text-base-content/60 px-2"
              >{currentIndex + 1} / {allFiles.length}</span
            >
            <button
              onclick={navigateNext}
              class="btn btn-sm btn-ghost btn-square"
              title="Next (→)"><i class="bi bi-chevron-right"></i></button
            >
            <div class="divider divider-horizontal mx-1"></div>
          {/if}

          <!-- Actions -->
          <button
            onclick={toggleFavorite}
            class="btn btn-sm btn-ghost btn-square"
            title="Toggle Favorite"><i class="bi bi-star"></i></button
          >
          <button
            onclick={downloadFile}
            class="btn btn-sm btn-ghost btn-square"
            title="Download"><i class="bi bi-download"></i></button
          >
          <button
            onclick={renameFile}
            class="btn btn-sm btn-ghost btn-square"
            title="Rename"><i class="bi bi-pencil"></i></button
          >
          <button
            onclick={moveFile}
            class="btn btn-sm btn-ghost btn-square"
            title="Move"><i class="bi bi-folder2-open"></i></button
          >
          <button
            onclick={copyFile}
            class="btn btn-sm btn-ghost btn-square"
            title="Copy"><i class="bi bi-files"></i></button
          >
          <button
            onclick={shareFile}
            class="btn btn-sm btn-ghost btn-square"
            title="Share"><i class="bi bi-share"></i></button
          >
          <button
            onclick={openVersionHistory}
            class="btn btn-sm btn-ghost btn-square"
            title="Version History"><i class="bi bi-clock-history"></i></button
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
            class="btn btn-sm btn-ghost btn-square text-error"
            title="Delete"><i class="bi bi-trash"></i></button
          >
          <button
            onclick={close}
            class="btn btn-sm btn-circle btn-ghost ml-2"
            title="Close (ESC)"><i class="bi bi-x-lg"></i></button
          >
        </div>
      </div>
      <div class="tabs tabs-boxed bg-base-200 px-6 py-2 gap-1">
        <button
          class="tab {activeTab === 'preview' ? 'tab-active' : ''}"
          onclick={() => (activeTab = "preview")}
          ><i class="bi bi-eye mr-2"></i>Preview</button
        >
        <button
          class="tab {activeTab === 'comments' ? 'tab-active' : ''}"
          onclick={() => (activeTab = "comments")}
          ><i class="bi bi-chat-dots mr-2"></i>Comments</button
        >
        <button
          class="tab {activeTab === 'tags' ? 'tab-active' : ''}"
          onclick={() => (activeTab = "tags")}
          ><i class="bi bi-tags mr-2"></i>Tags</button
        >
        <button
          class="tab {activeTab === 'details' ? 'tab-active' : ''}"
          onclick={() => (activeTab = "details")}
          ><i class="bi bi-info-circle mr-2"></i>Details</button
        >
      </div>
      <div class="flex-1 overflow-y-auto p-6">
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
        {/if}
      </div>
      <div
        class="flex items-center justify-between px-6 py-3 border-t border-base-300 bg-base-200/30"
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
          File: {currentIndex + 1} of {allFiles.length}
        </div>
      </div>
    </div>
  </div>
{/if}
