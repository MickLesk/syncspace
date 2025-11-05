<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";
  import { formatFileSize, formatDate } from "../../lib/utils.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let trashItems = [];
  let loading = true;
  let error = null;
  let selectedItems = new Set();
  let searchQuery = "";
  let sortBy = "deleted_at";
  let sortOrder = "desc";

  // Filter and sort trash items
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

  // Select all filtered items
  function toggleSelectAll() {
    if (selectedItems.size === filteredItems.length) {
      selectedItems.clear();
    } else {
      filteredItems.forEach((item) => selectedItems.add(item.id));
    }
    selectedItems = selectedItems;
  }

  // Toggle individual item selection
  function toggleSelect(itemId) {
    if (selectedItems.has(itemId)) {
      selectedItems.delete(itemId);
    } else {
      selectedItems.add(itemId);
    }
    selectedItems = selectedItems;
  }

  // Load trash items
  async function loadTrash() {
    loading = true;
    error = null;
    try {
      const response = await api.trash.list();
      const data = await response.json();
      trashItems = Array.isArray(data) ? data : data.value || [];
      console.log("[TrashView] Loaded trash items:", trashItems);
    } catch (err) {
      console.error("[TrashView] Error loading trash:", err);
      error = err.message;
    } finally {
      loading = false;
    }
  }

  // Restore single item
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

  // Restore selected items
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

  // Permanently delete single item
  async function deleteItemPermanently(item) {
    if (
      !confirm(tr("trashConfirmDeleteSingle", item.file_name))
    )
      return;

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

  // Delete selected items permanently
  async function deleteSelectedPermanently() {
    const itemsToDelete = trashItems.filter((item) =>
      selectedItems.has(item.id)
    );
    if (
      !confirm(tr("trashConfirmDeleteMultiple", itemsToDelete.length))
    )
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

  // Empty entire trash
  async function emptyTrash() {
    if (
      !confirm(tr("trashConfirmEmpty", trashItems.length))
    )
      return;

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

  // Cleanup old items (>30 days)
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

  onMount(() => {
    loadTrash();
  });
</script>

<div class="trash-view">
  <!-- Header -->
  <div class="header">
    <div class="header-left">
      <h1>
        <svg
          class="icon"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
        {tr("trash")}
      </h1>
      <span class="item-count">
        {trashItems.length}
        {" "}
        {tr(trashItems.length === 1 ? "item" : "items")}
      </span>
    </div>

    <div class="header-actions">
      <button
        class="btn btn-sm"
        on:click={cleanupOldItems}
        disabled={trashItems.length === 0}
      >
        <svg
          class="icon-sm"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        {tr("trashCleanupOld")}
      </button>
      <button
        class="btn btn-sm btn-danger"
        on:click={emptyTrash}
        disabled={trashItems.length === 0}
      >
        <svg
          class="icon-sm"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
        {tr("emptyTrash")}
      </button>
    </div>
  </div>

  <!-- Toolbar -->
  {#if trashItems.length > 0}
    <div class="toolbar">
      <div class="toolbar-left">
        <label class="checkbox-label">
          <input
            type="checkbox"
            checked={selectedItems.size === filteredItems.length &&
              filteredItems.length > 0}
            on:change={toggleSelectAll}
          />
          <span>{tr("selectAll")}</span>
        </label>

        {#if selectedItems.size > 0}
          <span class="selected-count"
            >{selectedItems.size} {tr("selected")}</span
          >
          <button class="btn btn-sm btn-success" on:click={restoreSelected}>
            <svg
              class="icon-sm"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
            {tr("restoreFile")}
          </button>
          <button
            class="btn btn-sm btn-danger"
            on:click={deleteSelectedPermanently}
          >
            <svg
              class="icon-sm"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
            {tr("deleteForever")}
          </button>
        {/if}
      </div>

      <div class="toolbar-right">
        <input
          type="text"
          placeholder={tr("trashSearchPlaceholder")}
          class="search-input"
          bind:value={searchQuery}
        />

        <select class="sort-select" bind:value={sortBy}>
          <option value="deleted_at">{tr("dateDeleted")}</option>
          <option value="file_name">{tr("name")}</option>
          <option value="file_size">{tr("size")}</option>
        </select>

        <button
          class="btn-icon"
          on:click={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
        >
          <svg
            class="icon-sm"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            {#if sortOrder === "asc"}
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 15l7-7 7 7"
              />
            {:else}
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              />
            {/if}
          </svg>
        </button>
      </div>
    </div>
  {/if}

  <!-- Content -->
  <div class="content">
    {#if loading}
      <div class="space-y-4 p-6">
        {#each Array(6) as _}
          <div class="skeleton h-24 w-full rounded-xl"></div>
        {/each}
      </div>
    {:else if error}
      <div class="error">
        <svg
          class="icon-lg"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          />
        </svg>
  <h3>{tr("errorOccurred")}</h3>
        <p>{error}</p>
  <button class="btn" on:click={loadTrash}>{tr("tryAgain")}</button>
      </div>
    {:else if filteredItems.length === 0}
      <div class="empty">
        <svg
          class="icon-xl"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
        <h3>{searchQuery ? tr("noItemsFound") : tr("trashIsEmpty")}</h3>
        <p>
          {searchQuery
            ? tr("tryDifferentSearch")
            : tr("deletedFilesAppearHere")}
        </p>
      </div>
    {:else}
      <div class="trash-list">
        {#each filteredItems as item (item.id)}
          <div class="trash-item" class:selected={selectedItems.has(item.id)}>
            <label class="checkbox-container">
              <input
                type="checkbox"
                checked={selectedItems.has(item.id)}
                on:change={() => toggleSelect(item.id)}
              />
            </label>

            <div class="file-icon">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
                />
              </svg>
            </div>

            <div class="file-info">
              <div class="file-name">{item.file_name}</div>
              <div class="file-meta">
                <span>{formatFileSize(item.file_size)}</span>
                <span class="separator">•</span>
                <span>{tr("deletedOn")} {formatDate(item.deleted_at)}</span>
              </div>
            </div>

            <div class="actions">
              <button
                class="btn-icon"
                title={tr("restoreFile")}
                on:click={() => restoreItem(item)}
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                  />
                </svg>
              </button>
              <button
                class="btn-icon btn-danger"
                title={tr("deleteForever")}
                on:click={() => deleteItemPermanently(item)}
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .trash-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--surface-container);
  }

  /* Header */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    background: var(--surface-container-high);
    border-bottom: 1px solid var(--outline-variant);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .header h1 {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--on-surface);
  }

  .item-count {
    color: var(--on-surface-variant);
    font-size: 0.875rem;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Toolbar */
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: var(--surface-container);
    border-bottom: 1px solid var(--outline-variant);
    gap: 1rem;
  }

  .toolbar-left,
  .toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    user-select: none;
  }

  .selected-count {
    color: var(--primary);
    font-weight: 500;
    font-size: 0.875rem;
  }

  .search-input {
    padding: 0.5rem 1rem;
    border: 1px solid var(--outline);
    border-radius: 8px;
    background: var(--surface-container-highest);
    color: var(--on-surface);
    min-width: 200px;
  }

  .sort-select {
    padding: 0.5rem 1rem;
    border: 1px solid var(--outline);
    border-radius: 8px;
    background: var(--surface-container-highest);
    color: var(--on-surface);
    cursor: pointer;
  }

  /* Content */
  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem 2rem;
  }

  /* States */
  .error,
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 1rem;
    color: var(--on-surface-variant);
  }

  /* Trash List */
  .trash-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .trash-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: var(--surface-container-highest);
    border: 1px solid var(--outline-variant);
    border-radius: 12px;
    transition: all 0.2s ease;
  }

  .trash-item:hover {
    border-color: var(--outline);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .trash-item.selected {
    border-color: var(--primary);
    background: var(--primary-container);
  }

  .checkbox-container {
    display: flex;
    align-items: center;
  }

  .file-icon {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--surface-container);
    border-radius: 8px;
    color: var(--on-surface-variant);
  }

  .file-icon svg {
    width: 24px;
    height: 24px;
  }

  .file-info {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-weight: 500;
    color: var(--on-surface);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--on-surface-variant);
    margin-top: 0.25rem;
  }

  .separator {
    opacity: 0.5;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Buttons */
  .btn,
  .btn-icon {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 8px;
    background: var(--primary-container);
    color: var(--on-primary-container);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn:hover,
  .btn-icon:hover {
    background: var(--primary);
    color: var(--on-primary);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.875rem;
  }

  .btn-icon {
    padding: 0.5rem;
    background: transparent;
    color: var(--on-surface-variant);
  }

  .btn-icon:hover {
    background: var(--surface-container-highest);
    color: var(--on-surface);
  }

  .btn-success {
    background: var(--success-container);
    color: var(--on-success-container);
  }

  .btn-success:hover {
    background: var(--success);
    color: var(--on-success);
  }

  .btn-danger {
    background: var(--error-container);
    color: var(--on-error-container);
  }

  .btn-danger:hover {
    background: var(--error);
    color: var(--on-error);
  }

  /* Icons */
  .icon {
    width: 24px;
    height: 24px;
  }

  .icon-sm {
    width: 16px;
    height: 16px;
  }

  .icon-lg {
    width: 48px;
    height: 48px;
  }

  .icon-xl {
    width: 64px;
    height: 64px;
  }
</style>
