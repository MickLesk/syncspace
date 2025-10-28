<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../../stores/toast";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

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
    const daysLeft = Math.ceil(
      (autoDeleteDate.getTime() - now.getTime()) / (1000 * 60 * 60 * 24)
    );
    return Math.max(0, daysLeft);
  }

  function getAutoDeleteBadge(file) {
    const daysLeft = getDaysUntilAutoDelete(file);
    if (daysLeft === 0)
      return {
        text: "Deletes today",
        class:
          "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 border-red-200 dark:border-red-800",
      };
    if (daysLeft <= 3)
      return {
        text: `${daysLeft} days left`,
        class:
          "bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300 border-yellow-200 dark:border-yellow-800",
      };
    if (daysLeft <= 7)
      return {
        text: `${daysLeft} days left`,
        class:
          "bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 border-blue-200 dark:border-blue-800",
      };
    return {
      text: `${daysLeft} days left`,
      class:
        "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 border-gray-200 dark:border-gray-700",
    };
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

<PageWrapper gradient>
  <PageHeader
    title="Trash"
    subtitle={trashedFiles.length > 0
      ? `${trashedFiles.length} item(s) • ${formatBytes(totalSize)} total`
      : "No deleted files"}
    icon="trash3-fill"
  >
    {#snippet actions()}
      {#if trashedFiles.length > 0}
        <ModernButton
          variant="success"
          icon="arrow-counterclockwise"
          disabled={selectedFiles.size === 0 && trashedFiles.length === 0}
          onclick={handleRestoreAll}
        >
          Restore {selectedFiles.size > 0 ? `(${selectedFiles.size})` : "All"}
        </ModernButton>
        <ModernButton
          variant="danger"
          icon="x-circle"
          onclick={() =>
            openDeleteModal(selectedFiles.size > 0 ? "selected" : "all")}
        >
          Empty {selectedFiles.size > 0 ? `(${selectedFiles.size})` : "All"}
        </ModernButton>
      {/if}
    {/snippet}
  </PageHeader>

  <!-- Bulk Select Bar -->
  {#if trashedFiles.length > 0}
    <ModernCard variant="glass" padding="normal" class="mb-6">
      {#snippet children()}
        <div class="flex justify-between items-center">
          <label class="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              class="w-4 h-4 rounded border-2 border-gray-300 dark:border-gray-600 text-primary-600 focus:ring-2 focus:ring-primary-500 focus:ring-offset-0 bg-white dark:bg-gray-800 cursor-pointer"
              checked={selectedFiles.size === trashedFiles.length &&
                trashedFiles.length > 0}
              indeterminate={selectedFiles.size > 0 &&
                selectedFiles.size < trashedFiles.length}
              onchange={toggleSelectAll}
            />
            <span
              class="text-sm font-semibold text-gray-900 dark:text-gray-100"
            >
              {selectedFiles.size > 0
                ? `${selectedFiles.size} selected`
                : "Select all"}
            </span>
          </label>

          {#if selectedFiles.size > 0}
            <ModernButton
              variant="ghost"
              size="sm"
              icon="x"
              onclick={() => {
                selectedFiles.clear();
                selectedFiles = selectedFiles;
              }}
            >
              Clear
            </ModernButton>
          {/if}
        </div>
      {/snippet}
    </ModernCard>
  {/if}

  <!-- Empty State -->
  {#if trashedFiles.length === 0}
    <ModernCard variant="glass" class="text-center py-16">
      {#snippet children()}
        <div class="animate-fade-in">
          <div class="text-primary/30 mb-6">
            <i class="bi bi-trash3 text-8xl"></i>
          </div>
          <h3 class="text-2xl font-bold mb-3">Trash is Empty</h3>
          <p class="text-base-content/60 mb-6 max-w-md mx-auto">
            Deleted files will appear here. You can restore them or permanently
            delete them.
          </p>
          <div class="glass-card max-w-md mx-auto p-6 text-left">
            <div class="flex gap-3">
              <i class="bi bi-info-circle-fill text-info text-2xl flex-shrink-0"
              ></i>
              <div>
                <p class="font-semibold text-sm mb-1">Auto-delete in 30 days</p>
                <p class="text-xs text-base-content/70">
                  Files are automatically deleted after 30 days in trash
                </p>
              </div>
            </div>
          </div>
        </div>
      {/snippet}
    </ModernCard>
  {:else}
    <!-- Trashed Files Table -->
    <ModernCard variant="glass">
      {#snippet children()}
        <div class="overflow-x-auto">
          <table class="table w-full border-collapse-zebra">
            <thead>
              <tr>
                <th>
                  <input
                    type="checkbox"
                    class="w-4 h-4 rounded border-2 border-gray-300 dark:border-gray-600 text-primary-600 focus:ring-2 focus:ring-primary-500 focus:ring-offset-0 bg-white dark:bg-gray-800 cursor-pointer"
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
              {#each trashedFiles as file, i (file.id)}
                <tr
                  class:selected={selectedFiles.has(file.id)}
                  class="animate-slide-up"
                  style="animation-delay: {i * 30}ms;"
                >
                  <td>
                    <input
                      type="checkbox"
                      class="w-4 h-4 rounded border-2 border-gray-300 dark:border-gray-600 text-primary-600 focus:ring-2 focus:ring-primary-500 focus:ring-offset-0 bg-white dark:bg-gray-800 cursor-pointer"
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
                    <span
                      class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800"
                    >
                      {formatBytes(file.size || 0)}
                    </span>
                  </td>
                  <td>
                    <span class="text-sm text-base-content/60">
                      {formatDate(file.deletedAt)}
                    </span>
                  </td>
                  <td>
                    <span
                      class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border {getAutoDeleteBadge(
                        file
                      ).class}"
                    >
                      <i class="bi bi-clock-history mr-1"></i>
                      {getAutoDeleteBadge(file).text}
                    </span>
                  </td>
                  <td>
                    <div class="flex gap-2">
                      <ModernButton
                        variant="success"
                        size="sm"
                        icon="arrow-counterclockwise"
                        onclick={() => handleRestore(file)}
                      >
                        Restore
                      </ModernButton>
                      <ModernButton
                        variant="danger"
                        size="sm"
                        icon="x-circle"
                        onclick={() => openDeleteModal("single", file)}
                      >
                        Delete
                      </ModernButton>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/snippet}
    </ModernCard>
  {/if}
</PageWrapper>

<!-- Permanent Delete Warning Modal -->
{#if showDeleteModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    onclick={closeDeleteModal}
    onkeydown={(e) => e.key === "Escape" && closeDeleteModal()}
    role="button"
    tabindex="0"
  >
    <div
      class="relative max-w-2xl w-full bg-white dark:bg-gray-900 rounded-2xl shadow-xl p-6"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="0"
    >
      <h3 class="font-bold text-2xl mb-6 text-error flex items-center gap-2">
        <i class="bi bi-exclamation-triangle-fill"></i>
        Permanent Delete Warning
      </h3>

      <!-- Warning Alert -->
      <div class="glass-card border-l-4 border-error p-4 mb-6">
        <div class="flex gap-4">
          <i
            class="bi bi-exclamation-octagon-fill text-error text-3xl flex-shrink-0"
          ></i>
          <div>
            <p class="font-bold text-error mb-1">
              This action cannot be undone!
            </p>
            <p class="text-sm text-base-content/70">
              Files will be permanently deleted and cannot be recovered.
            </p>
          </div>
        </div>
      </div>

      <!-- Delete Details -->
      <div class="glass-card p-6 mb-6">
        <p class="text-base-content/80 mb-4 font-semibold">
          You are about to permanently delete:
        </p>

        {#if deleteTarget === "single" && deleteTargetFile}
          <div
            class="flex items-center gap-4 p-4 bg-base-100/50 rounded-lg border border-base-300"
          >
            <i class="{getFileIcon(deleteTargetFile.name)} text-4xl"></i>
            <div class="flex-1">
              <p class="font-bold">{deleteTargetFile.name}</p>
              <p class="text-sm text-base-content/60">
                {formatBytes(deleteTargetFile.size)} • {deleteTargetFile.originalPath}
              </p>
            </div>
          </div>
        {:else if deleteTarget === "selected"}
          <div
            class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 w-full"
          >
            <div class="flex items-center gap-6">
              <div class="text-red-600 dark:text-red-400">
                <i class="bi bi-files text-5xl"></i>
              </div>
              <div class="flex-1">
                <div
                  class="text-sm text-gray-600 dark:text-gray-400 font-medium mb-1"
                >
                  Selected Files
                </div>
                <div class="text-4xl font-bold text-red-600 dark:text-red-400">
                  {selectedFiles.size}
                </div>
                <div class="text-sm text-gray-500 dark:text-gray-500 mt-1">
                  Total: {formatBytes(
                    trashedFiles
                      .filter((f) => selectedFiles.has(f.id))
                      .reduce((sum, f) => sum + f.size, 0)
                  )}
                </div>
              </div>
            </div>
          </div>
        {:else if deleteTarget === "all"}
          <div
            class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 w-full"
          >
            <div class="flex items-center gap-6">
              <div class="text-red-600 dark:text-red-400">
                <i class="bi bi-trash3-fill text-5xl"></i>
              </div>
              <div class="flex-1">
                <div
                  class="text-sm text-gray-600 dark:text-gray-400 font-medium mb-1"
                >
                  All Trashed Files
                </div>
                <div class="text-4xl font-bold text-red-600 dark:text-red-400">
                  {trashedFiles.length}
                </div>
                <div class="text-sm text-gray-500 dark:text-gray-500 mt-1">
                  Total: {formatBytes(totalSize)}
                </div>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Confirmation Checklist -->
      <div class="glass-card border-l-4 border-warning p-4 mb-6">
        <p class="font-bold mb-3 flex items-center gap-2">
          <i class="bi bi-check2-all text-warning"></i>
          Before you proceed:
        </p>
        <ul class="space-y-2">
          <li class="flex items-center gap-2 text-sm">
            <i class="bi bi-check2-circle text-warning"></i>
            I understand these files will be permanently deleted
          </li>
          <li class="flex items-center gap-2 text-sm">
            <i class="bi bi-check2-circle text-warning"></i>
            I have confirmed I don't need these files anymore
          </li>
          <li class="flex items-center gap-2 text-sm">
            <i class="bi bi-check2-circle text-warning"></i>
            I understand this action cannot be undone
          </li>
        </ul>
      </div>

      <div class="flex gap-3 justify-end mt-6">
        <ModernButton variant="ghost" onclick={closeDeleteModal}>
          Cancel
        </ModernButton>
        <ModernButton
          variant="danger"
          icon="trash-fill"
          onclick={confirmDelete}
        >
          Yes, Delete Permanently
        </ModernButton>
      </div>
    </div>
  </div>
{/if}

<style>
  .table tbody tr.selected {
    background: rgb(59 130 246 / 0.1);
  }

  .table tbody tr:hover {
    background: rgb(243 244 246 / 0.5);
  }

  .table tbody tr.selected:hover {
    background: rgb(59 130 246 / 0.15);
  }

  :global(.dark) .table tbody tr:hover {
    background: rgb(31 41 55 / 0.5);
  }
</style>
