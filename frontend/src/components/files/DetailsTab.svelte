<script>
  let { file } = $props();

  function formatBytes(bytes) {
    if (!bytes) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatDate(dateString) {
    if (!dateString) return "Unknown";
    return new Date(dateString).toLocaleString();
  }
</script>

<div class="space-y-4">
  <!-- Stats Cards -->
  <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
    <div class="stat">
      <div class="stat-figure text-primary">
        <i class="bi bi-file-earmark text-3xl"></i>
      </div>
      <div class="stat-title">File Name</div>
      <div class="stat-value text-lg break-all">{file?.name || "Unknown"}</div>
    </div>

    <div class="stat">
      <div class="stat-figure text-secondary">
        <i class="bi bi-hdd text-3xl"></i>
      </div>
      <div class="stat-title">File Size</div>
      <div class="stat-value text-lg">{formatBytes(file?.size)}</div>
      <div class="stat-desc">
        {file?.size ? Math.round(file.size / 1024) : 0} KB
      </div>
    </div>

    <div class="stat">
      <div class="stat-figure text-accent">
        <i class="bi bi-filetype-txt text-3xl"></i>
      </div>
      <div class="stat-title">File Type</div>
      <div class="stat-value text-lg">
        .{file?.name.split(".").pop()?.toUpperCase() || "?"}
      </div>
    </div>
  </div>

  <!-- Detailed Information -->
  <div class="card bg-base-200">
    <div class="card-body">
      <h3 class="card-title mb-4">
        <i class="bi bi-info-circle mr-2"></i>
        File Information
      </h3>
      <div class="space-y-3">
        <div class="flex justify-between py-2 border-b border-base-300">
          <span class="font-semibold">Full Path:</span>
          <span class="text-base-content/70 text-right break-all max-w-md">
            {file?.path || file?.name || "Unknown"}
          </span>
        </div>
        <div class="flex justify-between py-2 border-b border-base-300">
          <span class="font-semibold">MIME Type:</span>
          <span class="text-base-content/70">
            {file?.mime_type || "application/octet-stream"}
          </span>
        </div>
        <div class="flex justify-between py-2 border-b border-base-300">
          <span class="font-semibold">Created:</span>
          <span class="text-base-content/70">
            {formatDate(file?.created_at)}
          </span>
        </div>
        <div class="flex justify-between py-2 border-b border-base-300">
          <span class="font-semibold">Modified:</span>
          <span class="text-base-content/70">
            {formatDate(file?.updated_at)}
          </span>
        </div>
        {#if file?.owner_id}
          <div class="flex justify-between py-2">
            <span class="font-semibold">Owner ID:</span>
            <span class="text-base-content/70 font-mono text-xs">
              {file.owner_id.substring(0, 8)}...
            </span>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Permissions (if available) -->
  {#if file?.is_shared || file?.is_favorite}
    <div class="card bg-base-200">
      <div class="card-body">
        <h3 class="card-title mb-3">
          <i class="bi bi-shield-check mr-2"></i>
          Status
        </h3>
        <div class="flex flex-wrap gap-2">
          {#if file.is_shared}
            <div class="badge badge-primary badge-lg gap-2">
              <i class="bi bi-share"></i>
              Shared
            </div>
          {/if}
          {#if file.is_favorite}
            <div class="badge badge-warning badge-lg gap-2">
              <i class="bi bi-star-fill"></i>
              Favorite
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
