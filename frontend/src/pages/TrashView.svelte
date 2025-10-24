<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../stores/toast";

  let trashedFiles = [];
  let loading = false;

  // Placeholder - Backend has no trash endpoint yet
  onMount(() => {
    // Future: Load trashed files from backend
    trashedFiles = [];
  });

  function handleRestore(file) {
    success(`Restored: ${file.name}`);
    // Future: API call to restore
  }

  function handleDelete(file) {
    if (confirm(`Permanently delete "${file.name}"?`)) {
      success(`Deleted: ${file.name}`);
      // Future: API call to permanently delete
    }
  }

  function handleEmptyTrash() {
    if (confirm("Permanently delete all trashed files?")) {
      success("Trash emptied");
      trashedFiles = [];
    }
  }

  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return (bytes / Math.pow(k, i)).toFixed(2) + " " + sizes[i];
  }
</script>

<div class="trash-view">
  <!-- Controls -->
  {#if trashedFiles.length > 0}
    <div class="card bg-base-100 border border-base-300 shadow-sm mb-6">
      <div class="card-body p-4">
        <div class="flex justify-between items-center">
          <div class="text-sm">
            <span class="font-semibold">{trashedFiles.length}</span> items in trash
          </div>
          <button
            class="btn btn-error btn-sm gap-2"
            on:click={handleEmptyTrash}
          >
            <i class="bi bi-x-circle"></i>
            Empty Trash
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Empty State -->
  {#if trashedFiles.length === 0}
    <div class="hero min-h-[500px]">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <i class="bi bi-trash3 text-7xl text-base-300 mb-4"></i>
          <h1 class="text-3xl font-bold">Trash is Empty</h1>
          <p class="py-6">
            Deleted files will appear here. You can restore them or permanently
            delete them.
          </p>
          <div class="alert alert-info">
            <i class="bi bi-info-circle-fill"></i>
            <span class="text-sm">
              Note: Trash functionality requires backend support (not
              implemented yet)
            </span>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <!-- Trashed Files List -->
    <div class="overflow-x-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Name</th>
            <th>Original Location</th>
            <th>Size</th>
            <th>Deleted</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each trashedFiles as file}
            <tr>
              <td>
                <div class="flex items-center gap-2">
                  <i class="bi bi-file-earmark-fill text-error"></i>
                  <span class="font-semibold">{file.name}</span>
                </div>
              </td>
              <td>
                <span class="font-mono text-sm opacity-70"
                  >{file.originalPath || "/"}</span
                >
              </td>
              <td>
                <span class="badge badge-ghost"
                  >{formatBytes(file.size || 0)}</span
                >
              </td>
              <td>
                <span class="text-sm opacity-70">
                  {file.deletedAt
                    ? new Date(file.deletedAt).toLocaleString()
                    : "—"}
                </span>
              </td>
              <td>
                <div class="join">
                  <button
                    class="btn btn-success btn-sm join-item gap-1"
                    on:click={() => handleRestore(file)}
                  >
                    <i class="bi bi-arrow-counterclockwise"></i>
                    Restore
                  </button>
                  <button
                    class="btn btn-error btn-sm join-item gap-1"
                    on:click={() => handleDelete(file)}
                  >
                    <i class="bi bi-x-circle"></i>
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  .trash-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }
</style>
