<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";
  import { formatFileSize, formatDate } from "../../lib/utils.js";
  import PageWrapper from "../../components/PageWrapper.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let trashItems = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let selectedItems = $state(new Set());
  let searchQuery = $state("");
  let sortBy = $state("deleted_at");
  let sortOrder = $state("desc");

  const filteredItems = $derived(
    trashItems
      .filter((item) => {
        if (!searchQuery) return true;
        return item.file_name.toLowerCase().includes(searchQuery.toLowerCase());
      })
      .sort((a, b) => {
        const order = sortOrder === "asc" ? 1 : -1;
        if (sortBy === "deleted_at") {
          return (
            order *
            (new Date(a.deleted_at).getTime() -
              new Date(b.deleted_at).getTime())
          );
        } else if (sortBy === "file_name") {
          return order * a.file_name.localeCompare(b.file_name);
        } else if (sortBy === "file_size") {
          return order * (a.file_size - b.file_size);
        }
        return 0;
      })
  );

  const totalSize = $derived(
    trashItems.reduce((sum, item) => sum + (item.file_size || 0), 0)
  );

  function toggleSelectAll() {
    if (selectedItems.size === filteredItems.length) {
      selectedItems.clear();
    } else {
      filteredItems.forEach((item) => selectedItems.add(item.id));
    }
    selectedItems = selectedItems;
  }

  function toggleSelect(itemId) {
    if (selectedItems.has(itemId)) {
      selectedItems.delete(itemId);
    } else {
      selectedItems.add(itemId);
    }
    selectedItems = selectedItems;
  }

  async function loadTrash() {
    loading = true;
    error = null;
    try {
      const response = await api.trash.list();
      const data = await response.json();
      trashItems = Array.isArray(data) ? data : data.value || [];
    } catch (err) {
      console.error("[TrashView] Error loading trash:", err);
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function restoreItem(item) {
    try {
      const response = await api.trash.restore(item.original_path);
      if (response.ok) {
        await loadTrash();
        selectedItems.delete(item.id);
        selectedItems = selectedItems;
      }
    } catch (err) {
      console.error("[TrashView] Error restoring item:", err);
      alert(`${tr("trashRestoreFailed")}: ${err.message}`);
    }
  }

  async function restoreSelected() {
    const itemsToRestore = trashItems.filter((item) =>
      selectedItems.has(item.id)
    );
    let successCount = 0;
    for (const item of itemsToRestore) {
      try {
        const response = await api.trash.restore(item.original_path);
        if (response.ok) successCount++;
      } catch (err) {
        console.error("[TrashView] Error restoring item:", err);
      }
    }
    await loadTrash();
    selectedItems.clear();
    selectedItems = selectedItems;
    alert(tr("trashRestoreSummary", successCount, itemsToRestore.length));
  }

  async function deleteItemPermanently(item) {
    if (!confirm(tr("trashConfirmDeleteSingle", item.file_name))) return;
    try {
      const response = await api.trash.permanentDelete(item.original_path);
      if (response.ok) {
        await loadTrash();
        selectedItems.delete(item.id);
        selectedItems = selectedItems;
      }
    } catch (err) {
      console.error("[TrashView] Error deleting item:", err);
      alert(`${tr("trashDeleteFailed")}: ${err.message}`);
    }
  }

  async function deleteSelectedPermanently() {
    const itemsToDelete = trashItems.filter((item) =>
      selectedItems.has(item.id)
    );
    if (!confirm(tr("trashConfirmDeleteMultiple", itemsToDelete.length)))
      return;
    let successCount = 0;
    for (const item of itemsToDelete) {
      try {
        const response = await api.trash.permanentDelete(item.original_path);
        if (response.ok) successCount++;
      } catch (err) {
        console.error("[TrashView] Error deleting item:", err);
      }
    }
    await loadTrash();
    selectedItems.clear();
    selectedItems = selectedItems;
    alert(tr("trashDeleteSummary", successCount, itemsToDelete.length));
  }

  async function emptyTrash() {
    if (!confirm(tr("trashConfirmEmpty", trashItems.length))) return;
    try {
      const response = await api.trash.empty();
      if (response.ok) {
        await loadTrash();
        selectedItems.clear();
        selectedItems = selectedItems;
      }
    } catch (err) {
      console.error("[TrashView] Error emptying trash:", err);
      alert(`${tr("trashEmptyFailed")}: ${err.message}`);
    }
  }

  async function cleanupOldItems() {
    if (!confirm(tr("trashConfirmCleanup"))) return;
    try {
      const response = await api.trash.cleanup();
      if (response.ok) {
        await loadTrash();
      }
    } catch (err) {
      console.error("[TrashView] Error cleaning up trash:", err);
      alert(`${tr("trashCleanupFailed")}: ${err.message}`);
    }
  }

  function getFileIcon(fileName) {
    const ext = fileName.split(".").pop()?.toLowerCase();
    if (["jpg", "jpeg", "png", "gif", "webp", "svg"].includes(ext))
      return "bi-file-earmark-image";
    if (["mp4", "webm", "avi", "mov"].includes(ext))
      return "bi-file-earmark-play";
    if (["mp3", "wav", "ogg", "flac"].includes(ext))
      return "bi-file-earmark-music";
    if (["pdf"].includes(ext)) return "bi-file-earmark-pdf";
    if (["zip", "rar", "7z", "tar", "gz"].includes(ext))
      return "bi-file-earmark-zip";
    if (["doc", "docx"].includes(ext)) return "bi-file-earmark-word";
    if (["xls", "xlsx"].includes(ext)) return "bi-file-earmark-excel";
    if (["js", "ts", "py", "rs", "json", "html", "css"].includes(ext))
      return "bi-file-earmark-code";
    return "bi-file-earmark";
  }

  function getRelativeTime(date) {
    const now = new Date();
    const d = new Date(date);
    const diff = now - d;
    const mins = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (mins < 60) return `${mins} min`;
    if (hours < 24) return `${hours}h`;
    if (days < 30) return `${days}d`;
    return formatDate(date);
  }

  onMount(() => {
    loadTrash();
  });
</script>

<PageWrapper showSidebar={true}>
  <div class="p-6 max-md:p-4">
    <!-- Header -->
    <div class="flex justify-between items-center mb-6 flex-wrap gap-4 max-md:flex-col max-md:items-start">
      <h1 class="text-2xl font-bold text-base-content flex items-center gap-2 m-0">
        <i class="bi bi-trash3-fill"></i>
        {tr("trash")}
        <span class="text-sm font-semibold bg-gradient-to-br from-success to-green-600 text-white px-3 py-1 rounded-full ml-2">{trashItems.length}</span>
      </h1>
      <div class="flex gap-2">
        <button
          class="btn btn-ghost gap-2"
          onclick={cleanupOldItems}
          disabled={loading || trashItems.length === 0}
        >
          <i class="bi bi-clock-history"></i>
          {tr("trashCleanupOld")}
        </button>
        <button
          class="btn btn-success gap-2"
          onclick={emptyTrash}
          disabled={loading || trashItems.length === 0}
        >
          <i class="bi bi-trash3"></i>
          {tr("emptyTrash")}
        </button>
      </div>
    </div>

    {#if error}
      <div class="alert alert-error mb-6">
        <i class="bi bi-exclamation-triangle"></i>
        <span>{error}</span>
      </div>
    {/if}

    {#if loading}
      <div class="flex justify-center py-16">
        <span class="loading loading-spinner loading-lg text-success"></span>
      </div>
    {:else}
      <!-- Quick Stats -->
      <div class="grid grid-cols-[repeat(auto-fit,minmax(180px,1fr))] gap-4 mb-6">
        <div class="flex items-center gap-4 p-4 bg-base-100 border border-base-300 rounded-xl">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-success/10 text-success">
            <i class="bi bi-files"></i>
          </div>
          <div>
            <h3 class="text-xl font-bold text-base-content m-0">{trashItems.length}</h3>
            <p class="text-xs text-base-content/60 uppercase tracking-wide mt-0.5">{tr("files")}</p>
          </div>
        </div>
        <div class="flex items-center gap-4 p-4 bg-base-100 border border-base-300 rounded-xl">
          <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg bg-blue-500/10 text-blue-500">
            <i class="bi bi-hdd"></i>
          </div>
          <div>
            <h3 class="text-xl font-bold text-base-content m-0">{formatFileSize(totalSize)}</h3>
            <p class="text-xs text-base-content/60 uppercase tracking-wide mt-0.5">{tr("totalSize")}</p>
          </div>
        </div>
      </div>

      <!-- Table Card -->
      <div class="bg-base-100 border border-base-300 rounded-xl overflow-hidden">
        <div class="flex items-center gap-3 p-4 border-b border-base-300">
          <div class="w-9 h-9 bg-success/10 rounded-lg flex items-center justify-center text-base text-success">
            <i class="bi bi-trash3"></i>
          </div>
          <h2 class="flex-1 text-base font-semibold text-base-content m-0">{tr("deletedFiles")}</h2>
          <span class="badge badge-ghost">{filteredItems.length}</span>
        </div>

        <!-- Toolbar -->
        {#if trashItems.length > 0}
          <div class="flex justify-between items-center p-4 border-b border-base-300 flex-wrap gap-3 max-md:flex-col max-md:items-stretch">
            <div class="flex items-center gap-3 flex-wrap max-md:w-full max-md:justify-between">
              <label class="flex items-center gap-2 text-sm text-base-content cursor-pointer">
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm checkbox-success"
                  checked={selectedItems.size === filteredItems.length && filteredItems.length > 0}
                  onchange={toggleSelectAll}
                />
                <span>{tr("selectAll")}</span>
              </label>
              {#if selectedItems.size > 0}
                <span class="text-sm font-medium text-success">{selectedItems.size} {tr("selected")}</span>
                <button class="btn btn-xs btn-success btn-ghost" onclick={restoreSelected}>
                  <i class="bi bi-arrow-counterclockwise"></i>
                  {tr("restoreFile")}
                </button>
                <button class="btn btn-xs btn-error btn-ghost" onclick={deleteSelectedPermanently}>
                  <i class="bi bi-x-lg"></i>
                  {tr("deleteForever")}
                </button>
              {/if}
            </div>
            <div class="flex items-center gap-3 flex-wrap max-md:w-full max-md:justify-between">
              <div class="flex items-center gap-2 px-2.5 py-1.5 bg-base-200 border border-base-300 rounded-lg">
                <i class="bi bi-search text-sm text-base-content/40"></i>
                <input
                  type="text"
                  class="border-none bg-transparent outline-none text-base-content w-36 text-sm"
                  placeholder={tr("trashSearchPlaceholder")}
                  bind:value={searchQuery}
                />
              </div>
              <select class="select select-bordered select-sm" bind:value={sortBy}>
                <option value="deleted_at">{tr("dateDeleted")}</option>
                <option value="file_name">{tr("name")}</option>
                <option value="file_size">{tr("size")}</option>
              </select>
              <button
                class="btn btn-ghost btn-sm btn-square"
                title={tr(sortOrder === "asc" ? "sortDescending" : "sortAscending")}
                onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
              >
                <i class="bi {sortOrder === 'asc' ? 'bi-sort-up' : 'bi-sort-down'}"></i>
              </button>
            </div>
          </div>
        {/if}

        <!-- Table or Empty State -->
        {#if filteredItems.length === 0}
          <div class="flex flex-col items-center justify-center py-16 px-8 text-center">
            <i class="bi bi-trash3 text-5xl text-base-content/20 mb-4"></i>
            <h3 class="text-base font-semibold text-base-content mb-1">{searchQuery ? tr("noItemsFound") : tr("trashIsEmpty")}</h3>
            <p class="text-sm text-base-content/60">{searchQuery ? tr("tryDifferentSearch") : tr("deletedFilesAppearHere")}</p>
          </div>
        {:else}
          <div class="overflow-x-auto">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th class="w-10"></th>
                  <th>{tr("name")}</th>
                  <th>{tr("size")}</th>
                  <th>{tr("dateDeleted")}</th>
                  <th>{tr("actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each filteredItems as item (item.id)}
                  <tr class:bg-success/10={selectedItems.has(item.id)}>
                    <td>
                      <input
                        type="checkbox"
                        class="checkbox checkbox-sm checkbox-success"
                        checked={selectedItems.has(item.id)}
                        onchange={() => toggleSelect(item.id)}
                      />
                    </td>
                    <td>
                      <div class="flex items-center gap-3">
                        <i class="bi {getFileIcon(item.file_name)} text-lg text-base-content/60"></i>
                        <div class="flex flex-col min-w-0">
                          <span class="font-medium text-base-content truncate max-w-[300px]">{item.file_name}</span>
                          <span class="text-xs text-base-content/40 truncate max-w-[300px]">{item.original_path}</span>
                        </div>
                      </div>
                    </td>
                    <td class="font-mono text-sm">{formatFileSize(item.file_size)}</td>
                    <td class="text-base-content/60">{getRelativeTime(item.deleted_at)}</td>
                    <td>
                      <div class="flex gap-1.5">
                        <button
                          class="btn btn-xs btn-success btn-ghost btn-square"
                          title={tr("restoreFile")}
                          onclick={() => restoreItem(item)}
                        >
                          <i class="bi bi-arrow-counterclockwise"></i>
                        </button>
                        <button
                          class="btn btn-xs btn-error btn-ghost btn-square"
                          title={tr("deleteForever")}
                          onclick={() => deleteItemPermanently(item)}
                        >
                          <i class="bi bi-x-lg"></i>
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
    {/if}
  </div>
</PageWrapper>

<style>
  /* Minimal custom styles - keep only what Tailwind can't handle */
</style>
