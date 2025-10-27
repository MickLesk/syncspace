<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../stores/toast";

  let trashedFiles = $state([]);
  let loading = $state(false);
  let selectedFiles = $state(new Set());
  let autoDeleteCountdown = $state(null);
  let countdownInterval = $state(null);
  let showDeleteModal = $state(false);
  let deleteTarget = $state(null); // 'single', 'selected', 'all'
  let deleteTargetFile = $state(null);

  // Placeholder - Backend has no trash endpoint yet
  onMount(() => {
    // Future: Load trashed files from backend
    // Mock data for demonstration
    trashedFiles = [
      {
        id: 1,
        name: "Document.pdf",
        originalPath: "/Documents/Work",
        size: 2048576,
        deletedAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
        autoDeleteDays: 28,
      },
      {
        id: 2,
        name: "Image.png",
        originalPath: "/Pictures/Vacation",
        size: 5242880,
        deletedAt: new Date(
          Date.now() - 15 * 24 * 60 * 60 * 1000
        ).toISOString(),
        autoDeleteDays: 15,
      },
    ];

    // Start auto-delete countdown
    startCountdown();

    return () => {
      if (countdownInterval) clearInterval(countdownInterval);
    };
  });

  function startCountdown() {
    // Update countdown every minute
    countdownInterval = setInterval(() => {
      trashedFiles = trashedFiles; // Trigger reactivity
    }, 60000);
  }

  function getDaysUntilAutoDelete(file) {
    const deletedDate = new Date(file.deletedAt);
    const autoDeleteDate = new Date(
      deletedDate.getTime() + file.autoDeleteDays * 24 * 60 * 60 * 1000
    );
    const now = new Date();
    const daysLeft = Math.ceil((autoDeleteDate.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
    return Math.max(0, daysLeft);
  }

  function getAutoDeleteBadge(file) {
    const daysLeft = getDaysUntilAutoDelete(file);
    if (daysLeft === 0) return { text: "Deletes today", class: "badge-error" };
    if (daysLeft <= 3)
      return { text: `${daysLeft} days left`, class: "badge-warning" };
    if (daysLeft <= 7)
      return { text: `${daysLeft} days left`, class: "badge-info" };
    return { text: `${daysLeft} days left`, class: "badge-ghost" };
  }

  function handleRestore(file) {
    success(`Restored: ${file.name}`);
    trashedFiles = trashedFiles.filter((f) => f.id !== file.id);
    selectedFiles.delete(file.id);
    selectedFiles = selectedFiles; // Trigger reactivity
  }

  function handleRestoreAll() {
    const count = selectedFiles.size || trashedFiles.length;
    const files =
      selectedFiles.size > 0
        ? trashedFiles.filter((f) => selectedFiles.has(f.id))
        : trashedFiles;

    success(`Restored ${count} file(s)`);

    if (selectedFiles.size > 0) {
      trashedFiles = trashedFiles.filter((f) => !selectedFiles.has(f.id));
      selectedFiles.clear();
      selectedFiles = selectedFiles; // Trigger reactivity
    } else {
      trashedFiles = [];
    }
  }

  function openDeleteModal(target, file = null) {
    deleteTarget = target;
    deleteTargetFile = file;
    showDeleteModal = true;
  }

  function closeDeleteModal() {
    showDeleteModal = false;
    deleteTarget = null;
    deleteTargetFile = null;
  }

  function confirmDelete() {
    if (deleteTarget === "single" && deleteTargetFile) {
      success(`Permanently deleted: ${deleteTargetFile.name}`);
      trashedFiles = trashedFiles.filter((f) => f.id !== deleteTargetFile.id);
      selectedFiles.delete(deleteTargetFile.id);
      selectedFiles = selectedFiles;
    } else if (deleteTarget === "selected" && selectedFiles.size > 0) {
      success(`Permanently deleted ${selectedFiles.size} file(s)`);
      trashedFiles = trashedFiles.filter((f) => !selectedFiles.has(f.id));
      selectedFiles.clear();
      selectedFiles = selectedFiles;
    } else if (deleteTarget === "all") {
      success("Trash emptied - all files permanently deleted");
      trashedFiles = [];
      selectedFiles.clear();
      selectedFiles = selectedFiles;
    }
    closeDeleteModal();
  }

  function toggleSelectFile(fileId) {
    if (selectedFiles.has(fileId)) {
      selectedFiles.delete(fileId);
    } else {
      selectedFiles.add(fileId);
    }
    selectedFiles = selectedFiles; // Trigger reactivity
  }

  function toggleSelectAll() {
    if (selectedFiles.size === trashedFiles.length) {
      selectedFiles.clear();
    } else {
      selectedFiles = new Set(trashedFiles.map((f) => f.id));
    }
    selectedFiles = selectedFiles; // Trigger reactivity
  }

  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return (bytes / Math.pow(k, i)).toFixed(2) + " " + sizes[i];
  }

  function formatDate(dateString) {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function getFileIcon(filename) {
    const ext = filename.split(".").pop().toLowerCase();
    const iconMap = {
      pdf: "bi-file-pdf-fill text-error",
      doc: "bi-file-word-fill text-primary",
      docx: "bi-file-word-fill text-primary",
      xls: "bi-file-excel-fill text-success",
      xlsx: "bi-file-excel-fill text-success",
      jpg: "bi-file-image-fill text-accent",
      jpeg: "bi-file-image-fill text-accent",
      png: "bi-file-image-fill text-accent",
      mp4: "bi-file-play-fill text-error",
      txt: "bi-file-text-fill",
    };
    return iconMap[ext] || "bi-file-earmark-fill text-base-content/60";
  }

  let totalSize = $derived(
    trashedFiles.reduce((sum, f) => sum + (f.size || 0), 0)
  );
</script>

<div class="trash-view">
  <!-- Header -->
  <div class="trash-header">
    <div class="header-top">
      <div>
        <h1 class="text-3xl font-bold">
          <i class="bi bi-trash3-fill"></i> Trash
        </h1>
        <p class="text-base-content/60 mt-1">
          {#if trashedFiles.length > 0}
            {trashedFiles.length} item(s) • {formatBytes(totalSize)} total
          {:else}
            No deleted files
          {/if}
        </p>
      </div>

      {#if trashedFiles.length > 0}
        <div class="header-actions">
          <button
            class="btn btn-success gap-2"
            onclick={handleRestoreAll}
            disabled={selectedFiles.size === 0 && trashedFiles.length === 0}
          >
            <i class="bi bi-arrow-counterclockwise"></i>
            Restore {selectedFiles.size > 0
              ? `Selected (${selectedFiles.size})`
              : "All"}
          </button>
          <button
            class="btn btn-error gap-2"
            onclick={() =>
              openDeleteModal(selectedFiles.size > 0 ? "selected" : "all")}
          >
            <i class="bi bi-x-circle"></i>
            {selectedFiles.size > 0
              ? `Delete Selected (${selectedFiles.size})`
              : "Empty Trash"}
          </button>
        </div>
      {/if}
    </div>

    <!-- Bulk Select Bar -->
    {#if trashedFiles.length > 0}
      <div class="bulk-select-bar">
        <label class="flex items-center gap-2 cursor-pointer">
          <input
            type="checkbox"
            class="checkbox checkbox-sm"
            checked={selectedFiles.size === trashedFiles.length &&
              trashedFiles.length > 0}
            indeterminate={selectedFiles.size > 0 &&
              selectedFiles.size < trashedFiles.length}
            onchange={toggleSelectAll}
          />
          <span class="text-sm">
            {selectedFiles.size > 0
              ? `${selectedFiles.size} selected`
              : "Select all"}
          </span>
        </label>

        {#if selectedFiles.size > 0}
          <button
            class="btn btn-ghost btn-sm gap-2"
            onclick={() => {
              selectedFiles.clear();
              selectedFiles = selectedFiles;
            }}
          >
            <i class="bi bi-x"></i>
            Clear selection
          </button>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Empty State -->
  {#if trashedFiles.length === 0}
    <div class="empty-state">
      <i class="bi bi-trash3 empty-icon"></i>
      <h3 class="text-xl font-semibold mt-4">Trash is Empty</h3>
      <p class="text-base-content/60 mt-2">
        Deleted files will appear here. You can restore them or permanently
        delete them.
      </p>
      <div class="alert alert-info mt-6 max-w-md">
        <i class="bi bi-info-circle-fill"></i>
        <div class="text-sm">
          <p class="font-semibold">Auto-delete in 30 days</p>
          <p class="opacity-80">
            Files are automatically deleted after 30 days in trash
          </p>
        </div>
      </div>
    </div>
  {:else}
    <!-- Trashed Files Table -->
    <div class="trash-table-container">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>
              <input
                type="checkbox"
                class="checkbox checkbox-sm"
                checked={selectedFiles.size === trashedFiles.length}
                onchange={toggleSelectAll}
              />
            </th>
            <th>Name</th>
            <th>Original Location</th>
            <th>Size</th>
            <th>Deleted</th>
            <th>Auto-delete</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each trashedFiles as file (file.id)}
            <tr class:selected={selectedFiles.has(file.id)}>
              <td>
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm"
                  checked={selectedFiles.has(file.id)}
                  onchange={() => toggleSelectFile(file.id)}
                />
              </td>
              <td>
                <div class="flex items-center gap-3">
                  <i class="{getFileIcon(file.name)} text-2xl"></i>
                  <span class="font-semibold">{file.name}</span>
                </div>
              </td>
              <td>
                <span class="font-mono text-sm text-base-content/60">
                  {file.originalPath || "/"}
                </span>
              </td>
              <td>
                <span class="badge badge-ghost badge-sm">
                  {formatBytes(file.size || 0)}
                </span>
              </td>
              <td>
                <span class="text-sm text-base-content/60">
                  {formatDate(file.deletedAt)}
                </span>
              </td>
              <td>
                <span class="badge {getAutoDeleteBadge(file).class} badge-sm">
                  <i class="bi bi-clock-history mr-1"></i>
                  {getAutoDeleteBadge(file).text}
                </span>
              </td>
              <td>
                <div class="flex gap-1">
                  <button
                    class="btn btn-success btn-sm gap-1"
                    onclick={() => handleRestore(file)}
                    title="Restore file"
                  >
                    <i class="bi bi-arrow-counterclockwise"></i>
                    Restore
                  </button>
                  <button
                    class="btn btn-error btn-sm gap-1"
                    onclick={() => openDeleteModal("single", file)}
                    title="Delete permanently"
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

<!-- Permanent Delete Warning Modal -->
{#if showDeleteModal}
  <div 
    class="modal modal-open" 
    onclick={closeDeleteModal}
    onkeydown={(e) => e.key === 'Escape' && closeDeleteModal()}
    role="button"
    tabindex="0"
  >
    <div 
      class="modal-box max-w-2xl" 
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="0"
    >
      <h3 class="font-bold text-lg mb-4 text-error">
        <i class="bi bi-exclamation-triangle-fill"></i>
        Permanent Delete Warning
      </h3>

      <!-- Warning Message -->
      <div class="alert alert-error mb-4">
        <i class="bi bi-exclamation-octagon-fill text-2xl"></i>
        <div>
          <p class="font-semibold">This action cannot be undone!</p>
          <p class="text-sm opacity-90">
            Files will be permanently deleted and cannot be recovered.
          </p>
        </div>
      </div>

      <!-- Delete Details -->
      <div class="delete-details">
        <p class="text-base-content/80 mb-3">
          You are about to permanently delete:
        </p>

        {#if deleteTarget === "single" && deleteTargetFile}
          <div class="file-preview">
            <i class="{getFileIcon(deleteTargetFile.name)} text-3xl"></i>
            <div>
              <p class="font-semibold">{deleteTargetFile.name}</p>
              <p class="text-sm text-base-content/60">
                {formatBytes(deleteTargetFile.size)} • {deleteTargetFile.originalPath}
              </p>
            </div>
          </div>
        {:else if deleteTarget === "selected"}
          <div class="stats shadow">
            <div class="stat">
              <div class="stat-title">Selected Files</div>
              <div class="stat-value text-error">{selectedFiles.size}</div>
              <div class="stat-desc">
                Total: {formatBytes(
                  trashedFiles
                    .filter((f) => selectedFiles.has(f.id))
                    .reduce((sum, f) => sum + f.size, 0)
                )}
              </div>
            </div>
          </div>
        {:else if deleteTarget === "all"}
          <div class="stats shadow">
            <div class="stat">
              <div class="stat-title">All Trashed Files</div>
              <div class="stat-value text-error">{trashedFiles.length}</div>
              <div class="stat-desc">
                Total: {formatBytes(totalSize)}
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Confirmation Checklist -->
      <div class="confirmation-checklist">
        <p class="font-semibold mb-2">Before you proceed:</p>
        <ul class="space-y-1 text-sm text-base-content/70">
          <li>
            <i class="bi bi-check2-circle text-warning"></i>
            I understand these files will be permanently deleted
          </li>
          <li>
            <i class="bi bi-check2-circle text-warning"></i>
            I have confirmed I don't need these files anymore
          </li>
          <li>
            <i class="bi bi-check2-circle text-warning"></i>
            I understand this action cannot be undone
          </li>
        </ul>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={closeDeleteModal}>Cancel</button>
        <button class="btn btn-error gap-2" onclick={confirmDelete}>
          <i class="bi bi-trash-fill"></i>
          Yes, Delete Permanently
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .trash-view {
    padding: 0;
    min-height: calc(100vh - 200px);
  }

  .trash-header {
    background: hsl(var(--b1));
    border-bottom: 1px solid hsl(var(--bc) / 0.1);
    padding: 2rem;
    margin: -2rem -2rem 2rem -2rem;
  }

  .header-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1.5rem;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  .bulk-select-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: hsl(var(--b2));
    border-radius: var(--rounded-btn);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    text-align: center;
    padding: 2rem;
  }

  .empty-icon {
    font-size: 5rem;
    color: hsl(var(--bc) / 0.2);
  }

  .trash-table-container {
    background: hsl(var(--b1));
    border-radius: var(--rounded-box);
    overflow: hidden;
    border: 1px solid hsl(var(--bc) / 0.1);
    margin: 0 2rem 2rem 2rem;
  }

  .table :where(th, td) {
    padding: 1rem;
  }

  .table tbody tr.selected {
    background: hsl(var(--p) / 0.1);
  }

  .table tbody tr:hover {
    background: hsl(var(--b2));
  }

  .table tbody tr.selected:hover {
    background: hsl(var(--p) / 0.15);
  }

  /* Delete Modal */
  .delete-details {
    background: hsl(var(--b2));
    padding: 1.5rem;
    border-radius: var(--rounded-box);
    margin-bottom: 1.5rem;
  }

  .file-preview {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: hsl(var(--b1));
    border-radius: var(--rounded-btn);
    border: 1px solid hsl(var(--bc) / 0.1);
  }

  .confirmation-checklist {
    background: hsl(var(--wa) / 0.1);
    padding: 1rem;
    border-radius: var(--rounded-box);
    border-left: 4px solid hsl(var(--wa));
    margin-bottom: 1.5rem;
  }

  .confirmation-checklist ul {
    list-style: none;
    padding-left: 0;
  }

  .confirmation-checklist li {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .confirmation-checklist li i {
    font-size: 1.25rem;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .trash-header {
      padding: 1.5rem;
      margin: -1.5rem -1.5rem 1.5rem -1.5rem;
    }

    .header-top {
      flex-direction: column;
      gap: 1rem;
    }

    .header-actions {
      width: 100%;
      flex-direction: column;
    }

    .trash-table-container {
      margin: 0 1rem 1rem 1rem;
    }

    .table {
      font-size: 0.875rem;
    }

    .table :where(th, td) {
      padding: 0.75rem 0.5rem;
    }
  }
</style>
