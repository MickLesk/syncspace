<script>
  let { uploads = [] } = $props();
  
  function formatBytes(bytes) {
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 ** 2) return (bytes / 1024).toFixed(1) + " KB";
    if (bytes < 1024 ** 3) return (bytes / (1024 ** 2)).toFixed(1) + " MB";
    return (bytes / (1024 ** 3)).toFixed(1) + " GB";
  }
  
  function getStatusClass(status) {
    return status === "complete" ? "success" : status === "error" ? "error" : status === "cancelled" ? "warning" : "primary";
  }
</script>

{#if uploads.length > 0}
  <div class="upload-panel">
    <div class="upload-header">
      <h3 class="upload-title"><i class="bi bi-cloud-upload"></i> Uploads ({uploads.length})</h3>
      <button class="btn btn-ghost btn-sm">Clear All</button>
    </div>
    
    <div class="upload-list">
      {#each uploads as upload (upload.id)}
        <div class="upload-item">
          <div class="upload-icon">
            {#if upload.status === "complete"}<i class="bi bi-check-circle-fill text-success"></i>
            {:else if upload.status === "error"}<i class="bi bi-x-circle-fill text-error"></i>
            {:else if upload.status === "cancelled"}<i class="bi bi-dash-circle-fill text-warning"></i>
            {:else}<i class="bi bi-file-earmark-arrow-up text-primary"></i>{/if}
          </div>
          
          <div class="upload-details">
            <div class="upload-name">{upload.name}</div>
            <div class="upload-meta">
              {#if upload.status === "uploading"}
                {formatBytes(upload.progress * upload.size)} / {formatBytes(upload.size)} • {Math.round(upload.progress * 100)}%
              {:else if upload.status === "complete"}
                {formatBytes(upload.size)} • Complete
              {:else if upload.status === "error"}
                {upload.error || "Upload failed"}
              {:else}
                {formatBytes(upload.size)} • Cancelled
              {/if}
            </div>
            {#if upload.status === "uploading"}
              <progress class="progress progress-{getStatusClass(upload.status)} w-full" value={upload.progress * 100} max="100"></progress>
            {/if}
          </div>
          
          <div class="upload-actions">
            {#if upload.status === "uploading"}
              <button class="btn btn-ghost btn-xs btn-circle"><i class="bi bi-x-lg"></i></button>
            {:else if upload.status === "error"}
              <button class="btn btn-ghost btn-xs gap-1"><i class="bi bi-arrow-clockwise"></i> Retry</button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .upload-panel { position: fixed; bottom: 1rem; right: 1rem; width: 400px; max-height: 500px; background: hsl(var(--b1)); border: 1px solid hsl(var(--bc) / 0.2); border-radius: var(--rounded-box); box-shadow: 0 10px 40px hsl(var(--bc) / 0.2); z-index: 100; animation: slideUp 0.3s; }
  .upload-header { display: flex; justify-content: space-between; align-items: center; padding: 1rem; border-bottom: 1px solid hsl(var(--bc) / 0.1); }
  .upload-title { font-weight: 600; display: flex; align-items: center; gap: 0.5rem; margin: 0; }
  .upload-list { max-height: 400px; overflow-y: auto; padding: 0.5rem; }
  .upload-item { display: flex; gap: 0.75rem; padding: 0.75rem; border-radius: 0.5rem; transition: background 0.2s; }
  .upload-item:hover { background: hsl(var(--bc) / 0.05); }
  .upload-icon { font-size: 1.5rem; display: flex; align-items: center; }
  .upload-details { flex: 1; min-width: 0; }
  .upload-name { font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .upload-meta { font-size: 0.75rem; color: hsl(var(--bc) / 0.6); margin-top: 0.25rem; }
  .upload-actions { display: flex; align-items: center; }
  @keyframes slideUp { from { transform: translateY(100px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }
</style>

