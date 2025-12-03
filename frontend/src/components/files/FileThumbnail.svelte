<script>
  import { onMount } from "svelte";
  import { getFileIcon, getFileIconColor } from "../../utils/fileIcons.js";
  import api from "../../lib/api.js";

  let { file = null } = $props();
  let { size = "md" } = $props(); // sm, md, lg
  let { showIcon = true } = $props();
  let { fallbackToIcon = true } = $props();

  let thumbnailUrl = null;
  let thumbnailError = false;
  let loading = false;
  let lastFilePath = null;

  // Size classes
  const sizeClasses = {
    sm: "w-8 h-8",
    md: "w-12 h-12",
    lg: "", // No size constraint for grid view - let icon determine size
  };

  const iconSizes = {
    sm: "text-lg",
    md: "text-2xl",
    lg: "!text-9xl", // Extra large with !important
  };

  // Image file extensions that support thumbnails
  const imageExtensions = [
    "jpg",
    "jpeg",
    "png",
    "gif",
    "webp",
    "bmp",
    "ico",
    "svg",
  ];

  $: isImage =
    file &&
    imageExtensions.some((ext) => file.name.toLowerCase().endsWith("." + ext));

  // Only load thumbnail when file path actually changes
  $: filePath = file ? (file.path || "") + file.name : null;
  $: if (filePath && filePath !== lastFilePath && isImage && !thumbnailError) {
    lastFilePath = filePath;
    loadThumbnail();
  }

  async function loadThumbnail() {
    if (!file || thumbnailError || loading) return;

    loading = true;
    try {
      // Use the file path to generate thumbnail URL
      const fullPath = file.path || file.name;
      thumbnailUrl = api.files.getThumbnailUrl(fullPath);

      // Test if thumbnail exists
      const response = await fetch(thumbnailUrl + "?t=" + Date.now(), {
        method: "HEAD",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("authToken")}`,
        },
      });

      if (!response.ok) {
        throw new Error("Thumbnail not available");
      }
    } catch (error) {
      console.log(
        `📸 Thumbnail not available for ${file.name}:`,
        error.message
      );
      thumbnailError = true;
      thumbnailUrl = null;
    } finally {
      loading = false;
    }
  }

  function handleThumbnailError() {
    console.log(`❌ Thumbnail load error for ${file.name}`);
    thumbnailError = true;
    thumbnailUrl = null;
  }
</script>

<div class="thumbnail-container {sizeClasses[size]} relative">
  {#if thumbnailUrl && !thumbnailError}
    <!-- Thumbnail Image -->
    <img
      src={thumbnailUrl}
      alt={file.name}
      class="w-full h-full object-cover rounded-lg shadow-sm"
      on:error={handleThumbnailError}
      loading="lazy"
    />

    <!-- Loading Overlay -->
    {#if loading}
      <div
        class="absolute inset-0 bg-gray-50 dark:bg-gray-800/80 rounded-lg flex items-center justify-center"
      >
        <div
          class="w-4 h-4 border-2 border-blue-200 dark:border-blue-900 border-t-blue-600 dark:border-t-blue-400 rounded-full animate-spin"
        ></div>
      </div>
    {/if}
  {:else if showIcon || fallbackToIcon}
    <!-- Fallback Icon -->
    <div class="w-full h-full rounded-lg flex items-center justify-center">
      <i
        class="bi bi-{getFileIcon(file?.name || '')} {getFileIconColor(
          file?.name || ''
        )} {iconSizes[size]}"
      ></i>
    </div>
  {:else}
    <!-- No thumbnail placeholder -->
    <div
      class="w-full h-full rounded-lg bg-gray-50 dark:bg-gray-800 flex items-center justify-center"
    >
      <span class="text-xs opacity-60">No preview</span>
    </div>
  {/if}

  <!-- Image Badge -->
  {#if isImage && size === "lg"}
    <div class="absolute top-1 right-1">
      <div
        class="px-1.5 py-0.5 text-xs bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 rounded flex items-center gap-1"
      >
        <i class="bi bi-image" aria-hidden="true"></i>
        IMG
      </div>
    </div>
  {/if}
</div>
