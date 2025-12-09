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
  <div class="trash-view">
    <!-- Header -->
    <div class="view-header">
      <h1 class="view-title">
        <i class="bi bi-trash3-fill"></i>
        {tr("trash")}
        <span class="item-count">{trashItems.length}</span>
      </h1>
      <div class="header-actions">
        <button
          class="btn-secondary"
          onclick={cleanupOldItems}
          disabled={loading || trashItems.length === 0}
        >
          <i class="bi bi-clock-history"></i>
          {tr("trashCleanupOld")}
        </button>
        <button
          class="btn-primary"
          onclick={emptyTrash}
          disabled={loading || trashItems.length === 0}
        >
          <i class="bi bi-trash3"></i>
          {tr("emptyTrash")}
        </button>
      </div>
    </div>

    {#if error}
      <div class="error-banner">
        <i class="bi bi-exclamation-triangle"></i> <span>{error}</span>
      </div>
    {/if}

    {#if loading}
      <div class="loading-container"><div class="spinner"></div></div>
    {:else}
      <!-- Quick Stats -->
      <div class="quick-stats">
        <div class="stat-card">
          <div class="stat-icon files-icon"><i class="bi bi-files"></i></div>
          <div class="stat-text">
            <h3>{trashItems.length}</h3>
            <p>{tr("files")}</p>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon size-icon"><i class="bi bi-hdd"></i></div>
          <div class="stat-text">
            <h3>{formatFileSize(totalSize)}</h3>
            <p>{tr("totalSize")}</p>
          </div>
        </div>
      </div>

      <!-- Table Card -->
      <div class="card table-card">
        <div class="card-header">
          <div class="card-icon"><i class="bi bi-trash3"></i></div>
          <h2>{tr("deletedFiles")}</h2>
          <span class="badge">{filteredItems.length}</span>
        </div>

        <!-- Toolbar -->
        {#if trashItems.length > 0}
          <div class="toolbar">
            <div class="toolbar-left">
              <label class="checkbox-wrapper">
                <input
                  type="checkbox"
                  checked={selectedItems.size === filteredItems.length &&
                    filteredItems.length > 0}
                  onchange={toggleSelectAll}
                />
                <span>{tr("selectAll")}</span>
              </label>
              {#if selectedItems.size > 0}
                <span class="selected-count"
                  >{selectedItems.size} {tr("selected")}</span
                >
                <button class="btn-sm success" onclick={restoreSelected}
                  ><i class="bi bi-arrow-counterclockwise"></i>
                  {tr("restoreFile")}</button
                >
                <button
                  class="btn-sm danger"
                  onclick={deleteSelectedPermanently}
                  ><i class="bi bi-x-lg"></i> {tr("deleteForever")}</button
                >
              {/if}
            </div>
            <div class="toolbar-right">
              <div class="search-box">
                <i class="bi bi-search"></i>
                <input
                  type="text"
                  placeholder={tr("trashSearchPlaceholder")}
                  bind:value={searchQuery}
                />
              </div>
              <select class="sort-select" bind:value={sortBy}>
                <option value="deleted_at">{tr("dateDeleted")}</option>
                <option value="file_name">{tr("name")}</option>
                <option value="file_size">{tr("size")}</option>
              </select>
              <button
                class="btn-icon"
                title={tr(
                  sortOrder === "asc" ? "sortDescending" : "sortAscending"
                )}
                onclick={() =>
                  (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
              >
                <i
                  class="bi {sortOrder === 'asc'
                    ? 'bi-sort-up'
                    : 'bi-sort-down'}"
                ></i>
              </button>
            </div>
          </div>
        {/if}

        <!-- Table or Empty State -->
        {#if filteredItems.length === 0}
          <div class="empty-state">
            <i class="bi bi-trash3"></i>
            <h3>{searchQuery ? tr("noItemsFound") : tr("trashIsEmpty")}</h3>
            <p>
              {searchQuery
                ? tr("tryDifferentSearch")
                : tr("deletedFilesAppearHere")}
            </p>
          </div>
        {:else}
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th class="col-checkbox"></th>
                  <th>{tr("name")}</th>
                  <th>{tr("size")}</th>
                  <th>{tr("dateDeleted")}</th>
                  <th>{tr("actions")}</th>
                </tr>
              </thead>
              <tbody>
                {#each filteredItems as item (item.id)}
                  <tr class:selected={selectedItems.has(item.id)}>
                    <td class="col-checkbox">
                      <input
                        type="checkbox"
                        checked={selectedItems.has(item.id)}
                        onchange={() => toggleSelect(item.id)}
                      />
                    </td>
                    <td class="name-cell">
                      <i class="bi {getFileIcon(item.file_name)} file-icon"></i>
                      <div class="file-info">
                        <span class="filename">{item.file_name}</span>
                        <span class="filepath">{item.original_path}</span>
                      </div>
                    </td>
                    <td class="mono">{formatFileSize(item.file_size)}</td>
                    <td class="muted">{getRelativeTime(item.deleted_at)}</td>
                    <td class="actions-cell">
                      <button
                        class="action-btn restore"
                        title={tr("restoreFile")}
                        onclick={() => restoreItem(item)}
                      >
                        <i class="bi bi-arrow-counterclockwise"></i>
                      </button>
                      <button
                        class="action-btn delete"
                        title={tr("deleteForever")}
                        onclick={() => deleteItemPermanently(item)}
                      >
                        <i class="bi bi-x-lg"></i>
                      </button>
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
  .trash-view {
    padding: 1.5rem;
  }

  /* Header */
  .view-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
    gap: 1rem;
  }
  .view-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #111827;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
  }
  :global(.dark) .view-title {
    color: #f9fafb;
  }

  .item-count {
    font-size: 0.875rem;
    font-weight: 600;
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    margin-left: 0.5rem;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Buttons */
  .btn-primary {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #22c55e;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-primary:hover:not(:disabled) {
    background: #16a34a;
  }
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: white;
    color: #374151;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }
  .btn-secondary:hover:not(:disabled) {
    background: #f3f4f6;
  }
  .btn-secondary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  :global(.dark) .btn-secondary {
    background: #374151;
    color: #e5e7eb;
    border-color: #4b5563;
  }
  :global(.dark) .btn-secondary:hover:not(:disabled) {
    background: #4b5563;
  }

  .btn-sm {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.625rem;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }
  .btn-sm.success {
    background: #dcfce7;
    color: #16a34a;
  }
  .btn-sm.success:hover {
    background: #22c55e;
    color: white;
  }
  .btn-sm.danger {
    background: #fee2e2;
    color: #dc2626;
  }
  .btn-sm.danger:hover {
    background: #dc2626;
    color: white;
  }
  :global(.dark) .btn-sm.success {
    background: rgba(34, 197, 94, 0.2);
  }
  :global(.dark) .btn-sm.danger {
    background: rgba(220, 38, 38, 0.2);
  }

  .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    color: #6b7280;
    cursor: pointer;
  }
  .btn-icon:hover {
    background: #f3f4f6;
    color: #111827;
  }
  :global(.dark) .btn-icon {
    background: #374151;
    border-color: #4b5563;
    color: #9ca3af;
  }
  :global(.dark) .btn-icon:hover {
    background: #4b5563;
    color: #f9fafb;
  }

  /* Error & Loading */
  .error-banner {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 0.5rem;
    color: #dc2626;
    margin-bottom: 1.5rem;
  }
  :global(.dark) .error-banner {
    background: rgba(220, 38, 38, 0.1);
    border-color: rgba(220, 38, 38, 0.3);
  }

  .loading-container {
    display: flex;
    justify-content: center;
    padding: 4rem;
  }
  .spinner {
    width: 36px;
    height: 36px;
    border: 3px solid #e5e7eb;
    border-top-color: #22c55e;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Quick Stats - same as StorageAnalytics */
  .quick-stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  .stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.25rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
  }
  :global(.dark) .stat-card {
    background: #1f2937;
    border-color: #374151;
  }
  .stat-icon {
    width: 40px;
    height: 40px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
  }
  .files-icon {
    background: #dcfce7;
    color: #16a34a;
  }
  .size-icon {
    background: #dbeafe;
    color: #2563eb;
  }
  :global(.dark) .files-icon {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }
  :global(.dark) .size-icon {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }
  .stat-text h3 {
    font-size: 1.25rem;
    font-weight: 700;
    color: #111827;
    margin: 0;
  }
  :global(.dark) .stat-text h3 {
    color: #f9fafb;
  }
  .stat-text p {
    font-size: 0.75rem;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin: 0.125rem 0 0 0;
  }
  :global(.dark) .stat-text p {
    color: #9ca3af;
  }

  /* Card */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    overflow: hidden;
  }
  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }

  .table-card .card-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }
  :global(.dark) .table-card .card-header {
    border-color: #374151;
  }
  .card-icon {
    width: 36px;
    height: 36px;
    background: #dcfce7;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
    color: #16a34a;
  }
  :global(.dark) .card-icon {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }
  .card-header h2 {
    flex: 1;
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }
  :global(.dark) .card-header h2 {
    color: #f9fafb;
  }
  .badge {
    padding: 0.125rem 0.5rem;
    background: #f3f4f6;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 500;
    color: #6b7280;
  }
  :global(.dark) .badge {
    background: #374151;
    color: #9ca3af;
  }

  /* Toolbar */
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.25rem;
    border-bottom: 1px solid #e5e7eb;
    flex-wrap: wrap;
    gap: 0.75rem;
  }
  :global(.dark) .toolbar {
    border-color: #374151;
  }
  .toolbar-left,
  .toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .checkbox-wrapper {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: #374151;
    cursor: pointer;
  }
  :global(.dark) .checkbox-wrapper {
    color: #d1d5db;
  }
  .selected-count {
    font-size: 0.875rem;
    font-weight: 500;
    color: #22c55e;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem 0.625rem;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
  }
  :global(.dark) .search-box {
    background: #111827;
    border-color: #374151;
  }
  .search-box i {
    color: #9ca3af;
    font-size: 0.875rem;
  }
  .search-box input {
    border: none;
    background: transparent;
    outline: none;
    color: #111827;
    width: 140px;
    font-size: 0.875rem;
  }
  :global(.dark) .search-box input {
    color: #f9fafb;
  }

  .sort-select {
    padding: 0.375rem 0.625rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    color: #374151;
    font-size: 0.875rem;
    cursor: pointer;
  }
  :global(.dark) .sort-select {
    background: #374151;
    border-color: #4b5563;
    color: #e5e7eb;
  }

  /* Table */
  .table-container {
    overflow-x: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
  }
  thead {
    background: #f9fafb;
  }
  :global(.dark) thead {
    background: #111827;
  }
  th {
    padding: 0.625rem 1rem;
    text-align: left;
    font-size: 0.75rem;
    font-weight: 600;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  :global(.dark) th {
    color: #9ca3af;
  }
  td {
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    color: #374151;
    border-top: 1px solid #e5e7eb;
  }
  :global(.dark) td {
    color: #d1d5db;
    border-color: #374151;
  }
  tr:hover td {
    background: #f9fafb;
  }
  :global(.dark) tr:hover td {
    background: rgba(255, 255, 255, 0.02);
  }
  tr.selected td {
    background: #dcfce7;
  }
  :global(.dark) tr.selected td {
    background: rgba(34, 197, 94, 0.1);
  }

  .col-checkbox {
    width: 40px;
  }
  .name-cell {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .file-icon {
    font-size: 1.125rem;
    color: #6b7280;
  }
  :global(.dark) .file-icon {
    color: #9ca3af;
  }
  .file-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .filename {
    font-weight: 500;
    color: #111827;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 300px;
  }
  :global(.dark) .filename {
    color: #f9fafb;
  }
  .filepath {
    font-size: 0.75rem;
    color: #9ca3af;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 300px;
  }
  .mono {
    font-family: ui-monospace, monospace;
  }
  .muted {
    color: #6b7280;
  }
  :global(.dark) .muted {
    color: #9ca3af;
  }

  .actions-cell {
    display: flex;
    gap: 0.375rem;
  }
  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    transition: all 0.15s;
  }
  .action-btn.restore {
    background: #dcfce7;
    color: #16a34a;
  }
  .action-btn.restore:hover {
    background: #22c55e;
    color: white;
  }
  .action-btn.delete {
    background: #fee2e2;
    color: #dc2626;
  }
  .action-btn.delete:hover {
    background: #dc2626;
    color: white;
  }
  :global(.dark) .action-btn.restore {
    background: rgba(34, 197, 94, 0.2);
  }
  :global(.dark) .action-btn.delete {
    background: rgba(220, 38, 38, 0.2);
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }
  .empty-state i {
    font-size: 3rem;
    color: #d1d5db;
    margin-bottom: 1rem;
  }
  :global(.dark) .empty-state i {
    color: #4b5563;
  }
  .empty-state h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
    margin: 0 0 0.25rem 0;
  }
  :global(.dark) .empty-state h3 {
    color: #e5e7eb;
  }
  .empty-state p {
    font-size: 0.875rem;
    color: #6b7280;
    margin: 0;
  }
  :global(.dark) .empty-state p {
    color: #9ca3af;
  }

  @media (max-width: 768px) {
    .trash-view {
      padding: 1rem;
    }
    .view-header {
      flex-direction: column;
      align-items: flex-start;
    }
    .toolbar {
      flex-direction: column;
      align-items: stretch;
    }
    .toolbar-left,
    .toolbar-right {
      width: 100%;
      justify-content: space-between;
    }
  }
</style>
