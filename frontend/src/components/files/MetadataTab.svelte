<script>
  import { onMount } from "svelte";
  import api from "../../lib/api";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n";

  let { file } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let metadata = $state(null);
  let loading = $state(true);
  let error = $state(null);

  onMount(async () => {
    await loadMetadata();
  });

  async function loadMetadata() {
    if (!file?.path && !file?.name) {
      loading = false;
      return;
    }

    loading = true;
    error = null;

    try {
      const filePath = file.path || file.name;
      metadata = await api.metadata.get(filePath);
    } catch (err) {
      console.error("Failed to load metadata:", err);
      error = err.message || "Failed to load metadata";
    } finally {
      loading = false;
    }
  }

  function formatMetadataValue(value) {
    if (typeof value === "boolean") {
      return value ? "Yes" : "No";
    }
    if (typeof value === "number") {
      return value.toLocaleString();
    }
    if (Array.isArray(value)) {
      return value.join(", ");
    }
    if (typeof value === "object") {
      return JSON.stringify(value);
    }
    return String(value);
  }

  function getMetadataIcon(key) {
    const iconMap = {
      // Image EXIF
      camera_make: "bi-camera",
      camera_model: "bi-camera-fill",
      date_taken: "bi-calendar",
      exposure_time: "bi-clock",
      f_number: "bi-aperture",
      iso: "bi-brightness-high",
      focal_length: "bi-zoom-in",
      width: "bi-arrows-expand",
      height: "bi-arrows-expand",
      gps_latitude: "bi-geo-alt",
      gps_longitude: "bi-geo-alt",
      gps_altitude: "bi-geo-alt-fill",
      orientation: "bi-aspect-ratio",
      software: "bi-gear",
      artist: "bi-person",
      copyright: "bi-c-circle",
      flash: "bi-lightning",
      white_balance: "bi-palette",
      lens_model: "bi-camera2",

      // Audio ID3
      title: "bi-music-note",
      artist: "bi-person",
      album: "bi-disc",
      year: "bi-calendar-event",
      genre: "bi-tags",
      track: "bi-hash",
      total_tracks: "bi-collection",
      disc: "bi-disc-fill",
      album_artist: "bi-people",
      duration_ms: "bi-clock",
      has_cover_art: "bi-image",

      // PDF
      author: "bi-person-badge",
      subject: "bi-bookmark",
      keywords: "bi-tags",
      creator: "bi-tools",
      producer: "bi-cpu",
      creation_date: "bi-calendar-plus",
      modification_date: "bi-calendar-check",
      page_count: "bi-file-text",
      pdf_version: "bi-file-pdf",

      // Generic
      created_timestamp: "bi-calendar-plus",
      modified_timestamp: "bi-calendar-check",
      accessed_timestamp: "bi-calendar-event",
      is_readonly: "bi-lock",
    };
    return iconMap[key] || "bi-info-circle";
  }

  function formatMetadataLabel(key) {
    const labelMap = {
      // Image EXIF
      camera_make: "Camera Make",
      camera_model: "Camera Model",
      date_taken: "Date Taken",
      exposure_time: "Exposure Time",
      f_number: "F-Number",
      iso: "ISO",
      focal_length: "Focal Length",
      width: "Width",
      height: "Height",
      gps_latitude: "GPS Latitude",
      gps_longitude: "GPS Longitude",
      gps_altitude: "GPS Altitude",
      orientation: "Orientation",
      software: "Software",
      artist: "Artist",
      copyright: "Copyright",
      flash: "Flash",
      white_balance: "White Balance",
      lens_model: "Lens Model",

      // Audio ID3
      title: "Title",
      album: "Album",
      year: "Year",
      genre: "Genre",
      track: "Track",
      total_tracks: "Total Tracks",
      disc: "Disc",
      album_artist: "Album Artist",
      duration_ms: "Duration (ms)",
      has_cover_art: "Has Cover Art",

      // PDF
      author: "Author",
      subject: "Subject",
      keywords: "Keywords",
      creator: "Creator",
      producer: "Producer",
      creation_date: "Creation Date",
      modification_date: "Modification Date",
      page_count: "Page Count",
      pdf_version: "PDF Version",

      // Generic
      created_timestamp: "Created",
      modified_timestamp: "Modified",
      accessed_timestamp: "Last Accessed",
      is_readonly: "Read Only",
    };
    return (
      labelMap[key] ||
      key.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase())
    );
  }

  function getMetadataTypeIcon(type) {
    switch (type) {
      case "image":
        return "bi-image";
      case "audio":
        return "bi-music-note-beamed";
      case "video":
        return "bi-film";
      case "document":
        return "bi-file-text";
      default:
        return "bi-file-earmark";
    }
  }

  function getMetadataTypeLabel(type) {
    switch (type) {
      case "image":
        return "Image Metadata (EXIF)";
      case "audio":
        return "Audio Metadata (ID3)";
      case "video":
        return "Video Metadata";
      case "document":
        return "Document Metadata";
      default:
        return "File Metadata";
    }
  }
</script>

<div class="max-h-[60vh] overflow-y-auto">
  {#if loading}
    <div class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner loading-lg text-primary mb-4"></span>
      <p class="text-base-content/60">Extracting metadata...</p>
    </div>
  {:else if error}
    <div class="alert alert-error">
      <i class="bi bi-exclamation-triangle" aria-hidden="true"></i>
      <span>{error}</span>
    </div>
  {:else if metadata}
    <!-- Header with metadata type -->
    <div class="flex items-center gap-3 mb-6">
      <div
        class="w-12 h-12 rounded-xl bg-gradient-to-br from-primary/20 to-secondary/20 flex items-center justify-center"
      >
        <i
          class="{getMetadataTypeIcon(
            metadata.metadata_type
          )} text-2xl text-primary"
        ></i>
      </div>
      <div>
        <h3 class="font-semibold text-lg">
          {getMetadataTypeLabel(metadata.metadata_type)}
        </h3>
        <p class="text-sm text-base-content/60">
          Extracted at {new Date(metadata.extracted_at).toLocaleString()}
        </p>
      </div>
    </div>

    <!-- Metadata entries -->
    {#if Object.keys(metadata.metadata).length > 0}
      <div class="space-y-2">
        {#each Object.entries(metadata.metadata) as [key, value]}
          <div
            class="flex items-start gap-3 p-3 bg-base-200 rounded-lg hover:bg-base-300 transition-colors"
          >
            <div
              class="w-8 h-8 rounded-lg bg-base-300 flex items-center justify-center flex-shrink-0"
            >
              <i class="{getMetadataIcon(key)} text-base-content/70"></i>
            </div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-base-content/70">
                {formatMetadataLabel(key)}
              </div>
              <div class="text-base font-semibold break-words">
                {formatMetadataValue(value)}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="text-center py-12">
        <i
          class="bi bi-file-earmark-x text-6xl text-base-content/20 mb-4"
          aria-hidden="true"
        ></i>
        <h4 class="text-lg font-semibold mb-2">No Metadata Found</h4>
        <p class="text-base-content/60">
          This file type doesn't contain extractable metadata.
        </p>
      </div>
    {/if}

    <!-- File info summary -->
    <div class="mt-6 pt-6 border-t border-base-300">
      <h4 class="text-sm font-semibold text-base-content/70 mb-3">
        File Information
      </h4>
      <div class="grid grid-cols-2 gap-3">
        <div class="stat bg-base-200 rounded-lg p-3">
          <div class="stat-title text-xs">File Size</div>
          <div class="text-base text-lg">
            {metadata.file_size
              ? (metadata.file_size / 1024 / 1024).toFixed(2)
              : 0} MB
          </div>
        </div>
        <div class="stat bg-base-200 rounded-lg p-3">
          <div class="stat-title text-xs">MIME Type</div>
          <div class="text-base text-sm truncate" title={metadata.mime_type}>
            {metadata.mime_type?.split("/").pop() || "unknown"}
          </div>
        </div>
      </div>
    </div>
  {:else}
    <div class="text-center py-12">
      <i
        class="bi bi-question-circle text-6xl text-base-content/20 mb-4"
        aria-hidden="true"
      ></i>
      <h4 class="text-lg font-semibold mb-2">No file selected</h4>
      <p class="text-base-content/60">Select a file to view its metadata.</p>
    </div>
  {/if}
</div>
