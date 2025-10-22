<script>
  import { onMount } from "svelte";
  import api from "../lib/api.js";

  let trashItems = $state([]);
  let loading = $state(false);
  let searchQuery = $state("");
  let sortBy = $state("deleted_at"); // deleted_at, name, size
  let sortOrder = $state("desc"); // asc, desc
  let selectedItems = $state(new Set());
  let stats = $state({ totalItems: 0, totalSize: 0 });

  onMount(() => {
    loadTrash();
  });

  async function loadTrash() {
    loading = true;
    try {
      const response = await api.listTrash();
      const data = await response.json();
      trashItems = data;
      calculateStats(data);
    } catch (error) {
      console.error("Failed to load trash:", error);
      trashItems = [];
    } finally {
      loading = false;
    }
  }

  function calculateStats(items) {
    stats = {
      totalItems: items.length,
      totalSize: items.reduce((sum, item) => sum + (item.size_bytes || 0), 0),
    };
  }

  function formatSize(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatDate(dateStr) {
    if (!dateStr) return "Unknown";
    const date = new Date(dateStr);
    return date.toLocaleString();
  }

  function getTimeRemaining(autoDeleteAt) {
    if (!autoDeleteAt) return "Never";
    const now = new Date();
    const deleteDate = new Date(autoDeleteAt);
    const diffMs = deleteDate - now;
    if (diffMs <= 0) return "Expired";
    const days = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    return `${days} day${days !== 1 ? "s" : ""}`;
  }

  // Filtering and sorting
  let filteredItems = $derived.by(() => {
    let items = trashItems;

    // Search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      items = items.filter((item) =>
        item.original_name.toLowerCase().includes(query)
      );
    }

    // Sort
    items = [...items].sort((a, b) => {
      let valA, valB;
      if (sortBy === "name") {
        valA = a.original_name.toLowerCase();
        valB = b.original_name.toLowerCase();
      } else if (sortBy === "size") {
        valA = a.size_bytes || 0;
        valB = b.size_bytes || 0;
      } else {
        // deleted_at
        valA = new Date(a.deleted_at || 0);
        valB = new Date(b.deleted_at || 0);
      }

      if (valA < valB) return sortOrder === "asc" ? -1 : 1;
      if (valA > valB) return sortOrder === "asc" ? 1 : -1;
      return 0;
    });

    return items;
  });

  function toggleSort(field) {
    if (sortBy === field) {
      sortOrder = sortOrder === "asc" ? "desc" : "asc";
    } else {
      sortBy = field;
      sortOrder = "desc";
    }
  }

  function toggleSelectAll() {
    if (selectedItems.size === filteredItems.length) {
      selectedItems.clear();
    } else {
      selectedItems = new Set(filteredItems.map((item) => item.id));
    }
  }

  function toggleSelect(itemId) {
    if (selectedItems.has(itemId)) {
      selectedItems.delete(itemId);
    } else {
      selectedItems.add(itemId);
    }
    selectedItems = selectedItems; // Trigger reactivity
  }

  async function restoreItem(item) {
    if (!confirm(`Restore "${item.original_name}"?`)) return;
    try {
      const response = await api.restoreTrash(item.original_path);
      if (response.ok) {
        await loadTrash();
      } else {
        alert("Failed to restore item");
      }
    } catch (error) {
      console.error("Failed to restore:", error);
      alert("Error: " + error.message);
    }
  }

  async function permanentDelete(item) {
    if (
      !confirm(
        `PERMANENTLY delete "${item.original_name}"? This cannot be undone!`
      )
    )
      return;
    try {
      const response = await api.permanentDeleteTrash(item.original_path);
      if (response.ok) {
        await loadTrash();
      } else {
        alert("Failed to delete item");
      }
    } catch (error) {
      console.error("Failed to delete:", error);
      alert("Error: " + error.message);
    }
  }

  async function restoreSelected() {
    if (selectedItems.size === 0) return;
    if (!confirm(`Restore ${selectedItems.size} item(s)?`)) return;

    const items = filteredItems.filter((item) => selectedItems.has(item.id));
    for (const item of items) {
      try {
        await api.restoreTrash(item.original_path);
      } catch (error) {
        console.error(`Failed to restore ${item.original_name}:`, error);
      }
    }
    selectedItems.clear();
    await loadTrash();
  }

  async function deleteSelected() {
    if (selectedItems.size === 0) return;
    if (
      !confirm(
        `PERMANENTLY delete ${selectedItems.size} item(s)? This cannot be undone!`
      )
    )
      return;

    const items = filteredItems.filter((item) => selectedItems.has(item.id));
    for (const item of items) {
      try {
        await api.permanentDeleteTrash(item.original_path);
      } catch (error) {
        console.error(`Failed to delete ${item.original_name}:`, error);
      }
    }
    selectedItems.clear();
    await loadTrash();
  }

  async function cleanupExpired() {
    if (!confirm("Delete all expired items?")) return;
    try {
      const response = await api.cleanupTrash();
      if (response.ok) {
        const result = await response.json();
        alert(`Cleaned up ${result.deleted_count} expired item(s)`);
        await loadTrash();
      }
    } catch (error) {
      console.error("Failed to cleanup:", error);
      alert("Error: " + error.message);
    }
  }

  async function emptyTrash() {
    if (
      !confirm("PERMANENTLY delete ALL items in trash? This cannot be undone!")
    )
      return;
    try {
      const response = await api.emptyTrash();
      if (response.ok) {
        const result = await response.json();
        alert(`Deleted ${result.deleted_count} item(s)`);
        await loadTrash();
      }
    } catch (error) {
      console.error("Failed to empty trash:", error);
      alert("Error: " + error.message);
    }
  }
</script>

<div class="trash-view">
  <div class="header">
    <h1>🗑️ Trash</h1>
    <div class="stats-bar">
      <div class="stat">
        <i class="bi bi-files"></i>
        <span>{stats.totalItems} item{stats.totalItems !== 1 ? "s" : ""}</span>
      </div>
      <div class="stat">
        <i class="bi bi-hdd"></i>
        <span>{formatSize(stats.totalSize)}</span>
      </div>
    </div>
  </div>

  <div class="toolbar">
    <div class="search-box">
      <i class="bi bi-search"></i>
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search trash..."
        class="search-input"
      />
    </div>

    <div class="actions">
      {#if selectedItems.size > 0}
        <button class="btn btn-success" onclick={restoreSelected}>
          <i class="bi bi-arrow-counterclockwise"></i>
          Restore ({selectedItems.size})
        </button>
        <button class="btn btn-danger" onclick={deleteSelected}>
          <i class="bi bi-trash3"></i>
          Delete ({selectedItems.size})
        </button>
      {/if}
      <button class="btn btn-outline" onclick={cleanupExpired}>
        <i class="bi bi-clock-history"></i>
        Cleanup Expired
      </button>
      <button class="btn btn-danger-outline" onclick={emptyTrash}>
        <i class="bi bi-trash3-fill"></i>
        Empty Trash
      </button>
    </div>
  </div>

  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading trash...</p>
    </div>
  {:else if trashItems.length === 0}
    <div class="empty-state">
      <i class="bi bi-trash3"></i>
      <h2>Trash is empty</h2>
      <p>Deleted files will appear here</p>
    </div>
  {:else if filteredItems.length === 0}
    <div class="empty-state">
      <i class="bi bi-search"></i>
      <h2>No results found</h2>
      <p>Try a different search term</p>
    </div>
  {:else}
    <div class="table-container">
      <table class="trash-table">
        <thead>
          <tr>
            <th class="checkbox-col">
              <input
                type="checkbox"
                checked={selectedItems.size === filteredItems.length &&
                  filteredItems.length > 0}
                onchange={toggleSelectAll}
              />
            </th>
            <th class="type-col">Type</th>
            <th class="name-col sortable" onclick={() => toggleSort("name")}>
              Name
              {#if sortBy === "name"}
                <i class="bi bi-arrow-{sortOrder === 'asc' ? 'up' : 'down'}"
                ></i>
              {/if}
            </th>
            <th class="path-col">Original Path</th>
            <th class="size-col sortable" onclick={() => toggleSort("size")}>
              Size
              {#if sortBy === "size"}
                <i class="bi bi-arrow-{sortOrder === 'asc' ? 'up' : 'down'}"
                ></i>
              {/if}
            </th>
            <th
              class="date-col sortable"
              onclick={() => toggleSort("deleted_at")}
            >
              Deleted
              {#if sortBy === "deleted_at"}
                <i class="bi bi-arrow-{sortOrder === 'asc' ? 'up' : 'down'}"
                ></i>
              {/if}
            </th>
            <th class="user-col">Deleted By</th>
            <th class="expiry-col">Auto-Delete</th>
            <th class="actions-col">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredItems as item (item.id)}
            <tr class:selected={selectedItems.has(item.id)}>
              <td>
                <input
                  type="checkbox"
                  checked={selectedItems.has(item.id)}
                  onchange={() => toggleSelect(item.id)}
                />
              </td>
              <td>
                <i
                  class="bi bi-{item.item_type === 'folder'
                    ? 'folder-fill'
                    : 'file-earmark'}"
                ></i>
              </td>
              <td class="name-cell">
                <span class="filename">{item.original_name}</span>
              </td>
              <td class="path-cell">
                <span class="path">{item.original_path || "/"}</span>
              </td>
              <td class="size-cell">{formatSize(item.size_bytes || 0)}</td>
              <td class="date-cell">{formatDate(item.deleted_at)}</td>
              <td class="user-cell">{item.deleted_by_username}</td>
              <td class="expiry-cell">
                <span
                  class="expiry-badge"
                  class:expired={getTimeRemaining(item.auto_delete_at) ===
                    "Expired"}
                >
                  {getTimeRemaining(item.auto_delete_at)}
                </span>
              </td>
              <td class="actions-cell">
                <button
                  class="btn-icon btn-restore"
                  onclick={() => restoreItem(item)}
                  title="Restore"
                >
                  <i class="bi bi-arrow-counterclockwise"></i>
                </button>
                <button
                  class="btn-icon btn-delete"
                  onclick={() => permanentDelete(item)}
                  title="Delete Permanently"
                >
                  <i class="bi bi-trash3"></i>
                </button>
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
    padding: 24px;
    max-width: 1400px;
    margin: 0 auto;
  }

  /* Header */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  h1 {
    font-size: 32px;
    font-weight: 600;
    margin: 0;
    color: var(--text-primary, #1a1a1a);
  }

  .stats-bar {
    display: flex;
    gap: 24px;
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--surface-variant, #f5f5f5);
    border-radius: 12px;
    font-size: 14px;
    color: var(--text-secondary, #666);
  }

  .stat i {
    font-size: 18px;
  }

  /* Toolbar */
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    margin-bottom: 24px;
    flex-wrap: wrap;
  }

  .search-box {
    position: relative;
    flex: 1;
    min-width: 200px;
    max-width: 400px;
  }

  .search-box i {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-secondary, #999);
  }

  .search-input {
    width: 100%;
    padding: 10px 12px 10px 40px;
    border: 1px solid var(--border-color, #e0e0e0);
    border-radius: 12px;
    font-size: 14px;
    transition: all 0.2s;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--primary-color, #6750a4);
    box-shadow: 0 0 0 3px rgba(103, 80, 164, 0.1);
  }

  .actions {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  /* Buttons */
  .btn {
    padding: 10px 16px;
    border-radius: 12px;
    font-size: 14px;
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
  }

  .btn-success {
    background: var(--success-color, #4caf50);
    color: white;
  }

  .btn-success:hover {
    background: #43a047;
  }

  .btn-danger {
    background: var(--error-color, #f44336);
    color: white;
  }

  .btn-danger:hover {
    background: #e53935;
  }

  .btn-outline {
    background: transparent;
    border: 1px solid var(--border-color, #e0e0e0);
    color: var(--text-primary, #1a1a1a);
  }

  .btn-outline:hover {
    background: var(--surface-variant, #f5f5f5);
  }

  .btn-danger-outline {
    background: transparent;
    border: 1px solid var(--error-color, #f44336);
    color: var(--error-color, #f44336);
  }

  .btn-danger-outline:hover {
    background: rgba(244, 67, 54, 0.1);
  }

  .btn-icon {
    width: 36px;
    height: 36px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    border: none;
    cursor: pointer;
    transition: all 0.2s;
    background: transparent;
  }

  .btn-restore {
    color: var(--success-color, #4caf50);
  }

  .btn-restore:hover {
    background: rgba(76, 175, 80, 0.1);
  }

  .btn-delete {
    color: var(--error-color, #f44336);
  }

  .btn-delete:hover {
    background: rgba(244, 67, 54, 0.1);
  }

  /* Loading */
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: var(--text-secondary, #666);
  }

  .spinner {
    width: 48px;
    height: 48px;
    border: 4px solid var(--surface-variant, #f5f5f5);
    border-top-color: var(--primary-color, #6750a4);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: 16px;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Empty State */
  .empty-state {
    text-align: center;
    padding: 60px 20px;
    color: var(--text-secondary, #666);
  }

  .empty-state i {
    font-size: 64px;
    margin-bottom: 16px;
    opacity: 0.3;
  }

  .empty-state h2 {
    font-size: 24px;
    margin: 0 0 8px 0;
    color: var(--text-primary, #1a1a1a);
  }

  .empty-state p {
    margin: 0;
    font-size: 14px;
  }

  /* Table */
  .table-container {
    background: white;
    border-radius: 16px;
    border: 1px solid var(--border-color, #e0e0e0);
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .trash-table {
    width: 100%;
    border-collapse: collapse;
  }

  thead {
    background: var(--surface-variant, #f5f5f5);
  }

  th {
    padding: 12px 16px;
    text-align: left;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary, #666);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  th.sortable {
    cursor: pointer;
    user-select: none;
  }

  th.sortable:hover {
    background: rgba(0, 0, 0, 0.03);
  }

  tbody tr {
    border-bottom: 1px solid var(--border-color, #f0f0f0);
    transition: background 0.2s;
  }

  tbody tr:hover {
    background: var(--surface-hover, #fafafa);
  }

  tbody tr.selected {
    background: rgba(103, 80, 164, 0.05);
  }

  td {
    padding: 12px 16px;
    font-size: 14px;
  }

  /* Column Widths */
  .checkbox-col {
    width: 40px;
  }

  .type-col {
    width: 50px;
  }

  .name-col {
    min-width: 200px;
  }

  .path-col {
    min-width: 150px;
    max-width: 250px;
  }

  .size-col {
    width: 100px;
  }

  .date-col {
    width: 160px;
  }

  .user-col {
    width: 120px;
  }

  .expiry-col {
    width: 120px;
  }

  .actions-col {
    width: 100px;
    text-align: right;
  }

  /* Table Cells */
  .name-cell .filename {
    font-weight: 500;
    color: var(--text-primary, #1a1a1a);
  }

  .path-cell .path {
    color: var(--text-secondary, #666);
    font-size: 13px;
    font-family: monospace;
  }

  .size-cell {
    color: var(--text-secondary, #666);
    font-variant-numeric: tabular-nums;
  }

  .date-cell {
    color: var(--text-secondary, #666);
    font-size: 13px;
  }

  .user-cell {
    color: var(--text-secondary, #666);
    font-size: 13px;
  }

  .expiry-cell {
    font-size: 13px;
  }

  .expiry-badge {
    padding: 4px 8px;
    border-radius: 6px;
    background: var(--info-bg, #e3f2fd);
    color: var(--info-color, #1976d2);
    font-weight: 500;
  }

  .expiry-badge.expired {
    background: var(--error-bg, #ffebee);
    color: var(--error-color, #f44336);
  }

  .actions-cell {
    display: flex;
    gap: 4px;
    justify-content: flex-end;
  }

  /* Responsive */
  @media (max-width: 1200px) {
    .path-col {
      display: none;
    }
  }

  @media (max-width: 900px) {
    .user-col,
    .expiry-col {
      display: none;
    }
  }

  @media (max-width: 600px) {
    .trash-view {
      padding: 16px;
    }

    .header {
      flex-direction: column;
      align-items: flex-start;
      gap: 16px;
    }

    .toolbar {
      flex-direction: column;
      align-items: stretch;
    }

    .search-box {
      max-width: none;
    }

    .actions {
      justify-content: stretch;
    }

    .btn {
      flex: 1;
      justify-content: center;
    }

    .size-col,
    .date-col {
      display: none;
    }
  }
</style>
