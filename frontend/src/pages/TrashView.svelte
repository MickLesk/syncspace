<script><script><script>

  import { onMount } from 'svelte';

  import api from '../lib/api.js';  import { onMount } from 'svelte';  import { currentLang } from "../stores/ui.js";

  

  // Simple formatBytes function  import api from '../utils/api.js';  import { t } from "../i18n.js";

  function formatBytes(bytes) {

    if (bytes === 0) return '0 B';  import { formatBytes, formatDate } from '../utils/format.js';</script>

    const k = 1024;

    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];  

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];  let trashItems = $state([]);<div class="view-container">

  }

    let loading = $state(false);  <div class="page-header">

  // Simple formatDate function

  function formatDate(dateStr) {  let selectedItems = $state(new Set());    <h2>🗑️ {t($currentLang, "trash")}</h2>

    if (!dateStr) return '';

    const date = new Date(dateStr);  let sortBy = $state('deleted_at');    <button class="empty-trash-btn" disabled>

    const now = new Date();

    const diff = now - date;  let sortDesc = $state(true);      {t($currentLang, "emptyTrash")}

    const seconds = Math.floor(diff / 1000);

    const minutes = Math.floor(seconds / 60);  let searchTerm = $state('');    </button>

    const hours = Math.floor(minutes / 60);

    const days = Math.floor(hours / 24);  let filter = $state('all'); // all, files, folders  </div>

    

    if (seconds < 60) return 'Just now';  

    if (minutes < 60) return `${minutes}m ago`;

    if (hours < 24) return `${hours}h ago`;  let stats = $derived({  <div class="empty-state">

    if (days === 1) return 'Yesterday';

    if (days < 7) return `${days}d ago`;    total: trashItems.length,    <div class="empty-icon">🗑️</div>

    

    return date.toLocaleDateString();    files: trashItems.filter(item => item.item_type === 'file').length,    <h3>{t($currentLang, "trashIsEmpty")}</h3>

  }

      folders: trashItems.filter(item => item.item_type === 'folder').length,    <p>{t($currentLang, "deletedFilesStored")}</p>

  let trashItems = $state([]);

  let loading = $state(false);    totalSize: trashItems.reduce((sum, item) => sum + item.size_bytes, 0)  </div>

  let selectedItems = $state(new Set());

  let sortBy = $state('deleted_at');  });</div>

  let sortDesc = $state(true);

  let searchTerm = $state('');  

  let filter = $state('all');

    let filteredItems = $derived.by(() => {<style>

  let stats = $derived({

    total: trashItems.length,    let items = trashItems;  .view-container {

    files: trashItems.filter(item => item.item_type === 'file').length,

    folders: trashItems.filter(item => item.item_type === 'folder').length,        padding: 24px;

    totalSize: trashItems.reduce((sum, item) => sum + item.size_bytes, 0)

  });    // Filter by type    max-width: 1400px;

  

  let filteredItems = $derived.by(() => {    if (filter === 'files') {    margin: 0 auto;

    let items = trashItems;

          items = items.filter(item => item.item_type === 'file');  }

    if (filter === 'files') {

      items = items.filter(item => item.item_type === 'file');    } else if (filter === 'folders') {

    } else if (filter === 'folders') {

      items = items.filter(item => item.item_type === 'folder');      items = items.filter(item => item.item_type === 'folder');  .page-header {

    }

        }    display: flex;

    if (searchTerm.trim()) {

      const term = searchTerm.toLowerCase();        justify-content: space-between;

      items = items.filter(item => 

        item.original_name.toLowerCase().includes(term) ||    // Search filter    align-items: center;

        item.original_path.toLowerCase().includes(term)

      );    if (searchTerm.trim()) {    margin-bottom: 32px;

    }

          const term = searchTerm.toLowerCase();  }

    items = [...items].sort((a, b) => {

      let aVal = a[sortBy];      items = items.filter(item => 

      let bVal = b[sortBy];

              item.original_name.toLowerCase().includes(term) ||  h2 {

      if (sortBy === 'size_bytes') {

        aVal = parseInt(aVal);        item.original_path.toLowerCase().includes(term)    font-size: 28px;

        bVal = parseInt(bVal);

      }      );    font-weight: 500;

      

      if (sortDesc) {    }    color: var(--md-sys-color-on-surface);

        return aVal > bVal ? -1 : aVal < bVal ? 1 : 0;

      } else {        margin: 0;

        return aVal < bVal ? -1 : aVal > bVal ? 1 : 0;

      }    // Sort  }

    });

        items = [...items].sort((a, b) => {

    return items;

  });      let aVal = a[sortBy];  .empty-trash-btn {

  

  onMount(() => {      let bVal = b[sortBy];    padding: 10px 20px;

    loadTrash();

  });          background: var(--md-sys-color-error);

  

  async function loadTrash() {      if (sortBy === 'size_bytes') {    color: var(--md-sys-color-on-primary);

    loading = true;

    try {        aVal = parseInt(aVal);    border: none;

      const response = await api.listTrash();

      trashItems = await response.json();        bVal = parseInt(bVal);    border-radius: 20px;

    } catch (error) {

      console.error('Failed to load trash:', error);      }    font-size: 14px;

    } finally {

      loading = false;          font-weight: 500;

    }

  }      if (sortDesc) {    cursor: pointer;

  

  async function restoreItem(item) {        return aVal > bVal ? -1 : aVal < bVal ? 1 : 0;    transition: all 0.2s ease;

    if (!confirm(`Restore "${item.original_name}"?`)) return;

          } else {  }

    try {

      await api.restoreTrash(item.original_path);        return aVal < bVal ? -1 : aVal > bVal ? 1 : 0;

      await loadTrash();

    } catch (error) {      }  .empty-trash-btn:disabled {

      console.error('Failed to restore:', error);

    }    });    opacity: 0.38;

  }

          cursor: not-allowed;

  async function permanentDelete(item) {

    if (!confirm(`PERMANENTLY DELETE "${item.original_name}"?\n\nThis action cannot be undone!`)) return;    return items;  }

    

    try {  });

      await api.permanentDeleteTrash(item.original_path);

      await loadTrash();    .empty-state {

    } catch (error) {

      console.error('Failed to delete:', error);  onMount(() => {    text-align: center;

    }

  }    loadTrash();    padding: 80px 20px;

  

  async function restoreSelected() {  });  }

    if (selectedItems.size === 0) return;

    if (!confirm(`Restore ${selectedItems.size} item(s)?`)) return;  

    

    const items = trashItems.filter(item => selectedItems.has(item.id));  async function loadTrash() {  .empty-icon {

    for (const item of items) {

      try {    loading = true;    font-size: 80px;

        await api.restoreTrash(item.original_path);

      } catch (error) {    try {    margin-bottom: 24px;

        console.error('Failed to restore:', item.original_name, error);

      }      const response = await api.listTrash();    opacity: 0.5;

    }

          trashItems = await response.json();  }

    selectedItems.clear();

    await loadTrash();    } catch (error) {

  }

        console.error('Failed to load trash:', error);  h3 {

  async function deleteSelected() {

    if (selectedItems.size === 0) return;      alert('Failed to load trash');    font-size: 24px;

    if (!confirm(`PERMANENTLY DELETE ${selectedItems.size} item(s)?\n\nThis cannot be undone!`)) return;

        } finally {    font-weight: 500;

    const items = trashItems.filter(item => selectedItems.has(item.id));

    for (const item of items) {      loading = false;    color: var(--md-sys-color-on-surface);

      try {

        await api.permanentDeleteTrash(item.original_path);    }    margin-bottom: 8px;

      } catch (error) {

        console.error('Failed to delete:', item.original_name, error);  }  }

      }

    }  

    

    selectedItems.clear();  async function restoreItem(item) {  p {

    await loadTrash();

  }    if (!confirm(`Restore "${item.original_name}"?`)) return;    font-size: 16px;

  

  async function emptyTrash() {        color: var(--md-sys-color-on-surface-variant);

    if (!confirm('EMPTY ENTIRE TRASH?\n\nAll items will be PERMANENTLY DELETED!')) return;

        try {  }

    try {

      await api.emptyTrash();      await api.restoreTrash(item.original_path);</style>

      await loadTrash();

    } catch (error) {      await loadTrash();

      console.error('Failed to empty trash:', error);      alert('Item restored successfully');

    }    } catch (error) {

  }      console.error('Failed to restore:', error);

        alert('Failed to restore item');

  async function cleanupExpired() {    }

    try {  }

      await api.cleanupTrash();  

      await loadTrash();  async function permanentDelete(item) {

    } catch (error) {    if (!confirm(`PERMANENTLY DELETE "${item.original_name}"?\n\nThis action cannot be undone!`)) return;

      console.error('Failed to cleanup:', error);    

    }    try {

  }      await api.permanentDeleteTrash(item.original_path);

        await loadTrash();

  function toggleItem(id) {    } catch (error) {

    if (selectedItems.has(id)) {      console.error('Failed to delete:', error);

      selectedItems.delete(id);      alert('Failed to delete item');

    } else {    }

      selectedItems.add(id);  }

    }  

    selectedItems = selectedItems;  async function restoreSelected() {

  }    if (selectedItems.size === 0) {

        alert('No items selected');

  function toggleAll() {      return;

    if (selectedItems.size === filteredItems.length) {    }

      selectedItems.clear();    

    } else {    if (!confirm(`Restore ${selectedItems.size} item(s)?`)) return;

      selectedItems = new Set(filteredItems.map(item => item.id));    

    }    const items = trashItems.filter(item => selectedItems.has(item.id));

  }    let restored = 0;

      

  function changeSort(field) {    for (const item of items) {

    if (sortBy === field) {      try {

      sortDesc = !sortDesc;        await api.restoreTrash(item.original_path);

    } else {        restored++;

      sortBy = field;      } catch (error) {

      sortDesc = true;        console.error('Failed to restore:', item.original_name, error);

    }      }

  }    }

      

  function getIconForType(item) {    selectedItems.clear();

    if (item.item_type === 'folder') return 'folder-fill';    await loadTrash();

        alert(`Restored ${restored}/${items.length} item(s)`);

    const ext = item.original_name.split('.').pop()?.toLowerCase() || '';  }

    const iconMap = {  

      pdf: 'file-earmark-pdf-fill',  async function deleteSelected() {

      doc: 'file-earmark-word-fill',    if (selectedItems.size === 0) {

      docx: 'file-earmark-word-fill',      alert('No items selected');

      jpg: 'file-earmark-image-fill',      return;

      jpeg: 'file-earmark-image-fill',    }

      png: 'file-earmark-image-fill',    

      mp4: 'file-earmark-play-fill',    if (!confirm(`PERMANENTLY DELETE ${selectedItems.size} item(s)?\n\nThis action cannot be undone!`)) return;

      zip: 'file-earmark-zip-fill',    

    };    const items = trashItems.filter(item => selectedItems.has(item.id));

        let deleted = 0;

    return iconMap[ext] || 'file-earmark-fill';    

  }    for (const item of items) {

        try {

  function getDaysRemaining(item) {        await api.permanentDeleteTrash(item.original_path);

    if (!item.auto_delete_at) return null;        deleted++;

          } catch (error) {

    const deleteDate = new Date(item.auto_delete_at);        console.error('Failed to delete:', item.original_name, error);

    const now = new Date();      }

    const diff = deleteDate - now;    }

    const days = Math.ceil(diff / (1000 * 60 * 60 * 24));    

        selectedItems.clear();

    return days;    await loadTrash();

  }    alert(`Deleted ${deleted}/${items.length} item(s)`);

</script>  }

  

<div class="trash-view">  async function emptyTrash() {

  <div class="page-header">    if (!confirm('EMPTY ENTIRE TRASH?\n\nAll items will be PERMANENTLY DELETED!\n\nThis action cannot be undone!')) return;

    <div class="header-content">    

      <h1>    try {

        <i class="bi bi-trash3-fill"></i>      const response = await api.emptyTrash();

        Trash      const result = await response.json();

      </h1>      await loadTrash();

      <p class="subtitle">Deleted items are kept for 30 days</p>      alert(result.message || 'Trash emptied');

    </div>    } catch (error) {

          console.error('Failed to empty trash:', error);

    <div class="header-actions">      alert('Failed to empty trash');

      <button class="btn btn-secondary" onclick={cleanupExpired}>    }

        <i class="bi bi-clock-history"></i>  }

        Cleanup Expired  

      </button>  async function cleanupExpired() {

      <button class="btn btn-danger" onclick={emptyTrash} disabled={trashItems.length === 0}>    try {

        <i class="bi bi-trash-fill"></i>      const response = await api.cleanupTrash();

        Empty Trash      const result = await response.json();

      </button>      await loadTrash();

      <button class="btn btn-primary" onclick={loadTrash}>      alert(result.message || 'Cleanup completed');

        <i class="bi bi-arrow-clockwise"></i>    } catch (error) {

        Refresh      console.error('Failed to cleanup:', error);

      </button>      alert('Failed to cleanup expired items');

    </div>    }

  </div>  }

    

  <!-- Stats -->  function toggleItem(id) {

  <div class="stats-grid">    if (selectedItems.has(id)) {

    <div class="stat-card">      selectedItems.delete(id);

      <div class="stat-icon"><i class="bi bi-files"></i></div>    } else {

      <div class="stat-info">      selectedItems.add(id);

        <div class="stat-value">{stats.total}</div>    }

        <div class="stat-label">Total Items</div>    selectedItems = selectedItems;

      </div>  }

    </div>  

      function toggleAll() {

    <div class="stat-card">    if (selectedItems.size === filteredItems.length) {

      <div class="stat-icon"><i class="bi bi-file-earmark"></i></div>      selectedItems.clear();

      <div class="stat-info">    } else {

        <div class="stat-value">{stats.files}</div>      selectedItems = new Set(filteredItems.map(item => item.id));

        <div class="stat-label">Files</div>    }

      </div>  }

    </div>  

      function changeSort(field) {

    <div class="stat-card">    if (sortBy === field) {

      <div class="stat-icon"><i class="bi bi-folder"></i></div>      sortDesc = !sortDesc;

      <div class="stat-info">    } else {

        <div class="stat-value">{stats.folders}</div>      sortBy = field;

        <div class="stat-label">Folders</div>      sortDesc = true;

      </div>    }

    </div>  }

      

    <div class="stat-card">  function getIconForType(item) {

      <div class="stat-icon"><i class="bi bi-hdd"></i></div>    if (item.item_type === 'folder') return 'folder-fill';

      <div class="stat-info">    

        <div class="stat-value">{formatBytes(stats.totalSize)}</div>    const ext = item.original_name.split('.').pop()?.toLowerCase() || '';

        <div class="stat-label">Total Size</div>    const iconMap = {

      </div>      pdf: 'file-earmark-pdf-fill',

    </div>      doc: 'file-earmark-word-fill',

  </div>      docx: 'file-earmark-word-fill',

        xls: 'file-earmark-excel-fill',

  <!-- Controls -->      xlsx: 'file-earmark-excel-fill',

  <div class="controls-bar">      ppt: 'file-earmark-ppt-fill',

    <div class="search-box">      pptx: 'file-earmark-ppt-fill',

      <i class="bi bi-search"></i>      txt: 'file-earmark-text-fill',

      <input type="text" placeholder="Search trash..." bind:value={searchTerm} />      jpg: 'file-earmark-image-fill',

    </div>      jpeg: 'file-earmark-image-fill',

          png: 'file-earmark-image-fill',

    <div class="filter-buttons">      gif: 'file-earmark-image-fill',

      <button class="filter-btn" class:active={filter === 'all'} onclick={() => filter = 'all'}>      mp4: 'file-earmark-play-fill',

        All      mp3: 'file-earmark-music-fill',

      </button>      zip: 'file-earmark-zip-fill',

      <button class="filter-btn" class:active={filter === 'files'} onclick={() => filter = 'files'}>      rar: 'file-earmark-zip-fill',

        Files ({stats.files})    };

      </button>    

      <button class="filter-btn" class:active={filter === 'folders'} onclick={() => filter = 'folders'}>    return iconMap[ext] || 'file-earmark-fill';

        Folders ({stats.folders})  }

      </button>  

    </div>  function getDaysRemaining(item) {

        if (!item.auto_delete_at) return null;

    {#if selectedItems.size > 0}    

      <div class="bulk-actions">    const deleteDate = new Date(item.auto_delete_at);

        <span class="selection-count">{selectedItems.size} selected</span>    const now = new Date();

        <button class="btn btn-sm btn-success" onclick={restoreSelected}>    const diff = deleteDate - now;

          <i class="bi bi-arrow-clockwise"></i> Restore    const days = Math.ceil(diff / (1000 * 60 * 60 * 24));

        </button>    

        <button class="btn btn-sm btn-danger" onclick={deleteSelected}>    return days;

          <i class="bi bi-trash-fill"></i> Delete Forever  }

        </button></script>

      </div>

    {/if}<div class="trash-view">

  </div>  <div class="page-header">

      <div class="header-content">

  <!-- Table -->      <h1>

  {#if loading}        <i class="bi bi-trash3-fill"></i>

    <div class="loading-state">        Trash

      <div class="spinner"></div>      </h1>

      <p>Loading trash...</p>      <p class="subtitle">

    </div>        Deleted items are kept for 30 days before permanent removal

  {:else if filteredItems.length === 0}      </p>

    <div class="empty-state">    </div>

      <i class="bi bi-trash3"></i>    

      <h3>Trash is empty</h3>    <div class="header-actions">

      <p>Deleted items will appear here</p>      <button class="btn btn-secondary" onclick={() => cleanupExpired()}>

    </div>        <i class="bi bi-clock-history"></i>

  {:else}        Cleanup Expired

    <div class="trash-table-container">      </button>

      <table class="trash-table">      <button class="btn btn-danger" onclick={() => emptyTrash()} disabled={trashItems.length === 0}>

        <thead>        <i class="bi bi-trash-fill"></i>

          <tr>        Empty Trash

            <th style="width: 40px;">      </button>

              <input type="checkbox"       <button class="btn btn-primary" onclick={() => loadTrash()}>

                checked={selectedItems.size === filteredItems.length && filteredItems.length > 0}        <i class="bi bi-arrow-clockwise"></i>

                onchange={toggleAll} />        Refresh

            </th>      </button>

            <th style="width: 40px;"></th>    </div>

            <th onclick={() => changeSort('original_name')} class="sortable">  </div>

              Name {#if sortBy === 'original_name'}<i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>{/if}  

            </th>  <!-- Stats Cards -->

            <th onclick={() => changeSort('original_path')} class="sortable">  <div class="stats-grid">

              Location {#if sortBy === 'original_path'}<i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>{/if}    <div class="stat-card">

            </th>      <div class="stat-icon">

            <th onclick={() => changeSort('size_bytes')} class="sortable">        <i class="bi bi-files"></i>

              Size {#if sortBy === 'size_bytes'}<i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>{/if}      </div>

            </th>      <div class="stat-info">

            <th onclick={() => changeSort('deleted_at')} class="sortable">        <div class="stat-value">{stats.total}</div>

              Deleted {#if sortBy === 'deleted_at'}<i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>{/if}        <div class="stat-label">Total Items</div>

            </th>      </div>

            <th>Expires</th>    </div>

            <th style="width: 200px;">Actions</th>    

          </tr>    <div class="stat-card">

        </thead>      <div class="stat-icon">

        <tbody>        <i class="bi bi-file-earmark"></i>

          {#each filteredItems as item (item.id)}      </div>

            <tr class:selected={selectedItems.has(item.id)}>      <div class="stat-info">

              <td>        <div class="stat-value">{stats.files}</div>

                <input type="checkbox"         <div class="stat-label">Files</div>

                  checked={selectedItems.has(item.id)}      </div>

                  onchange={() => toggleItem(item.id)} />    </div>

              </td>    

              <td class="icon-cell">    <div class="stat-card">

                <i class="bi bi-{getIconForType(item)}"></i>      <div class="stat-icon">

              </td>        <i class="bi bi-folder"></i>

              <td class="name-cell">      </div>

                <div class="name-with-badge">      <div class="stat-info">

                  <span class="item-name">{item.original_name}</span>        <div class="stat-value">{stats.folders}</div>

                  {#if item.item_type === 'folder'}        <div class="stat-label">Folders</div>

                    <span class="badge badge-folder">Folder</span>      </div>

                  {/if}    </div>

                </div>    

              </td>    <div class="stat-card">

              <td class="path-cell">{item.original_path}</td>      <div class="stat-icon">

              <td>{formatBytes(item.size_bytes)}</td>        <i class="bi bi-hdd"></i>

              <td title={item.deleted_at}>{formatDate(item.deleted_at)}</td>      </div>

              <td>      <div class="stat-info">

                {#if getDaysRemaining(item) !== null}        <div class="stat-value">{formatBytes(stats.totalSize)}</div>

                  {@const days = getDaysRemaining(item)}        <div class="stat-label">Total Size</div>

                  <span class="expiry-badge" class:warning={days <= 7} class:danger={days <= 3}>      </div>

                    {days} day{days !== 1 ? 's' : ''}    </div>

                  </span>  </div>

                {:else}  

                  <span class="expiry-badge neutral">Never</span>  <!-- Filters and Search -->

                {/if}  <div class="controls-bar">

              </td>    <div class="search-box">

              <td class="action-cell">      <i class="bi bi-search"></i>

                <button class="action-btn restore-btn" onclick={() => restoreItem(item)}>      <input 

                  <i class="bi bi-arrow-counterclockwise"></i> Restore        type="text" 

                </button>        placeholder="Search trash..." 

                <button class="action-btn delete-btn" onclick={() => permanentDelete(item)}>        bind:value={searchTerm}

                  <i class="bi bi-x-circle"></i> Delete      />

                </button>    </div>

              </td>    

            </tr>    <div class="filter-buttons">

          {/each}      <button 

        </tbody>        class="filter-btn" 

      </table>        class:active={filter === 'all'}

    </div>        onclick={() => filter = 'all'}

  {/if}      >

</div>        All

      </button>

<style>      <button 

  .trash-view {        class="filter-btn" 

    padding: 24px;        class:active={filter === 'files'}

    max-width: 1600px;        onclick={() => filter = 'files'}

    margin: 0 auto;      >

  }        Files ({stats.files})

        </button>

  .page-header {      <button 

    display: flex;        class="filter-btn" 

    justify-content: space-between;        class:active={filter === 'folders'}

    align-items: flex-start;        onclick={() => filter = 'folders'}

    margin-bottom: 32px;      >

  }        Folders ({stats.folders})

        </button>

  .header-content h1 {    </div>

    margin: 0;    

    font-size: 32px;    {#if selectedItems.size > 0}

    font-weight: 600;      <div class="bulk-actions">

    color: #1f2937;        <span class="selection-count">{selectedItems.size} selected</span>

    display: flex;        <button class="btn btn-sm btn-success" onclick={restoreSelected}>

    align-items: center;          <i class="bi bi-arrow-clockwise"></i>

    gap: 12px;          Restore

  }        </button>

          <button class="btn btn-sm btn-danger" onclick={deleteSelected}>

  .header-content h1 i {          <i class="bi bi-trash-fill"></i>

    color: #ef4444;          Delete Forever

  }        </button>

        </div>

  .subtitle {    {/if}

    margin: 8px 0 0 0;  </div>

    color: #6b7280;  

    font-size: 14px;  <!-- Trash Table -->

  }  {#if loading}

      <div class="loading-state">

  .header-actions {      <div class="spinner"></div>

    display: flex;      <p>Loading trash...</p>

    gap: 12px;    </div>

  }  {:else if filteredItems.length === 0}

      <div class="empty-state">

  .stats-grid {      <i class="bi bi-trash3"></i>

    display: grid;      <h3>Trash is empty</h3>

    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));      <p>Deleted items will appear here</p>

    gap: 20px;    </div>

    margin-bottom: 32px;  {:else}

  }    <div class="trash-table-container">

        <table class="trash-table">

  .stat-card {        <thead>

    background: white;          <tr>

    border-radius: 16px;            <th style="width: 40px;">

    padding: 24px;              <input 

    display: flex;                type="checkbox" 

    align-items: center;                checked={selectedItems.size === filteredItems.length && filteredItems.length > 0}

    gap: 16px;                onchange={toggleAll}

    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);              />

  }            </th>

              <th style="width: 40px;"></th>

  .stat-icon {            <th onclick={() => changeSort('original_name')} class="sortable">

    width: 56px;              Name

    height: 56px;              {#if sortBy === 'original_name'}

    border-radius: 12px;                <i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>

    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);              {/if}

    display: flex;            </th>

    align-items: center;            <th onclick={() => changeSort('original_path')} class="sortable">

    justify-content: center;              Original Location

    color: white;              {#if sortBy === 'original_path'}

    font-size: 24px;                <i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>

  }              {/if}

              </th>

  .stat-value {            <th onclick={() => changeSort('size_bytes')} class="sortable">

    font-size: 28px;              Size

    font-weight: 700;              {#if sortBy === 'size_bytes'}

    color: #1f2937;                <i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>

  }              {/if}

              </th>

  .stat-label {            <th onclick={() => changeSort('deleted_at')} class="sortable">

    font-size: 14px;              Deleted

    color: #6b7280;              {#if sortBy === 'deleted_at'}

    margin-top: 4px;                <i class="bi bi-arrow-{sortDesc ? 'down' : 'up'}"></i>

  }              {/if}

              </th>

  .controls-bar {            <th>Expires In</th>

    background: white;            <th style="width: 200px;">Actions</th>

    border-radius: 16px;          </tr>

    padding: 16px 24px;        </thead>

    margin-bottom: 24px;        <tbody>

    display: flex;          {#each filteredItems as item (item.id)}

    align-items: center;            <tr class:selected={selectedItems.has(item.id)}>

    gap: 16px;              <td>

    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);                <input 

  }                  type="checkbox" 

                    checked={selectedItems.has(item.id)}

  .search-box {                  onchange={() => toggleItem(item.id)}

    flex: 1;                />

    position: relative;              </td>

    max-width: 400px;              <td class="icon-cell">

  }                <i class="bi bi-{getIconForType(item)}"></i>

                </td>

  .search-box i {              <td class="name-cell">

    position: absolute;                <div class="name-with-badge">

    left: 12px;                  <span class="item-name">{item.original_name}</span>

    top: 50%;                  {#if item.item_type === 'folder'}

    transform: translateY(-50%);                    <span class="badge badge-folder">Folder</span>

    color: #9ca3af;                  {/if}

  }                </div>

                </td>

  .search-box input {              <td class="path-cell">{item.original_path}</td>

    width: 100%;              <td>{formatBytes(item.size_bytes)}</td>

    padding: 10px 12px 10px 36px;              <td title={item.deleted_at}>{formatDate(item.deleted_at)}</td>

    border: 1px solid #e5e7eb;              <td>

    border-radius: 8px;                {#if getDaysRemaining(item) !== null}

    font-size: 14px;                  {@const days = getDaysRemaining(item)}

  }                  <span class="expiry-badge" class:warning={days <= 7} class:danger={days <= 3}>

                      {days} day{days !== 1 ? 's' : ''}

  .filter-btn {                  </span>

    padding: 8px 16px;                {:else}

    border: 1px solid #e5e7eb;                  <span class="expiry-badge neutral">Never</span>

    background: white;                {/if}

    border-radius: 8px;              </td>

    font-size: 14px;              <td class="action-cell">

    cursor: pointer;                <button 

  }                  class="action-btn restore-btn" 

                    onclick={() => restoreItem(item)}

  .filter-btn.active {                  title="Restore"

    background: #667eea;                >

    color: white;                  <i class="bi bi-arrow-counterclockwise"></i>

    border-color: #667eea;                  Restore

  }                </button>

                  <button 

  .bulk-actions {                  class="action-btn delete-btn" 

    display: flex;                  onclick={() => permanentDelete(item)}

    align-items: center;                  title="Delete Permanently"

    gap: 12px;                >

    padding-left: 16px;                  <i class="bi bi-x-circle"></i>

    border-left: 2px solid #e5e7eb;                  Delete

  }                </button>

                </td>

  .selection-count {            </tr>

    font-size: 14px;          {/each}

    font-weight: 600;        </tbody>

    color: #667eea;      </table>

  }    </div>

    {/if}

  .trash-table-container {</div>

    background: white;

    border-radius: 16px;<style>

    overflow: hidden;  .trash-view {

    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);    padding: 24px;

  }    max-width: 1600px;

      margin: 0 auto;

  .trash-table {  }

    width: 100%;  

    border-collapse: collapse;  .page-header {

  }    display: flex;

      justify-content: space-between;

  .trash-table thead {    align-items: flex-start;

    background: #f9fafb;    margin-bottom: 32px;

    border-bottom: 2px solid #e5e7eb;  }

  }  

    .header-content h1 {

  .trash-table th {    margin: 0;

    padding: 16px;    font-size: 32px;

    text-align: left;    font-weight: 600;

    font-size: 13px;    color: #1f2937;

    font-weight: 600;    display: flex;

    color: #6b7280;    align-items: center;

    text-transform: uppercase;    gap: 12px;

  }  }

    

  .trash-table th.sortable {  .header-content h1 i {

    cursor: pointer;    color: #ef4444;

    user-select: none;  }

  }  

    .subtitle {

  .trash-table tbody tr {    margin: 8px 0 0 0;

    border-bottom: 1px solid #e5e7eb;    color: #6b7280;

  }    font-size: 14px;

    }

  .trash-table tbody tr.selected {  

    background: #ede9fe;  .header-actions {

  }    display: flex;

      gap: 12px;

  .trash-table td {  }

    padding: 16px;  

    font-size: 14px;  .stats-grid {

    color: #374151;    display: grid;

  }    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));

      gap: 20px;

  .icon-cell {    margin-bottom: 32px;

    font-size: 24px;  }

    color: #667eea;  

  }  .stat-card {

      background: white;

  .name-with-badge {    border-radius: 16px;

    display: flex;    padding: 24px;

    align-items: center;    display: flex;

    gap: 8px;    align-items: center;

  }    gap: 16px;

      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);

  .item-name {    transition: transform 0.2s, box-shadow 0.2s;

    font-weight: 500;  }

  }  

    .stat-card:hover {

  .path-cell {    transform: translateY(-2px);

    color: #6b7280;    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);

    font-size: 13px;  }

  }  

    .stat-icon {

  .badge {    width: 56px;

    padding: 2px 8px;    height: 56px;

    border-radius: 6px;    border-radius: 12px;

    font-size: 11px;    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

    font-weight: 600;    display: flex;

  }    align-items: center;

      justify-content: center;

  .badge-folder {    color: white;

    background: #dbeafe;    font-size: 24px;

    color: #1e40af;  }

  }  

    .stat-info {

  .expiry-badge {    flex: 1;

    padding: 4px 10px;  }

    border-radius: 6px;  

    font-size: 12px;  .stat-value {

    font-weight: 600;    font-size: 28px;

  }    font-weight: 700;

      color: #1f2937;

  .expiry-badge.neutral {  }

    background: #f3f4f6;  

    color: #6b7280;  .stat-label {

  }    font-size: 14px;

      color: #6b7280;

  .expiry-badge.warning {    margin-top: 4px;

    background: #fef3c7;  }

    color: #92400e;  

  }  .controls-bar {

      background: white;

  .expiry-badge.danger {    border-radius: 16px;

    background: #fee2e2;    padding: 16px 24px;

    color: #991b1b;    margin-bottom: 24px;

  }    display: flex;

      align-items: center;

  .action-cell {    gap: 16px;

    display: flex;    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);

    gap: 8px;  }

  }  

    .search-box {

  .action-btn {    flex: 1;

    padding: 6px 12px;    position: relative;

    border: none;    max-width: 400px;

    border-radius: 6px;  }

    font-size: 13px;  

    font-weight: 500;  .search-box i {

    cursor: pointer;    position: absolute;

    display: flex;    left: 12px;

    align-items: center;    top: 50%;

    gap: 4px;    transform: translateY(-50%);

  }    color: #9ca3af;

    }

  .restore-btn {  

    background: #d1fae5;  .search-box input {

    color: #065f46;    width: 100%;

  }    padding: 10px 12px 10px 36px;

      border: 1px solid #e5e7eb;

  .delete-btn {    border-radius: 8px;

    background: #fee2e2;    font-size: 14px;

    color: #991b1b;    transition: border-color 0.2s;

  }  }

    

  .btn {  .search-box input:focus {

    padding: 10px 18px;    outline: none;

    border: none;    border-color: #667eea;

    border-radius: 8px;  }

    font-size: 14px;  

    font-weight: 500;  .filter-buttons {

    cursor: pointer;    display: flex;

    display: inline-flex;    gap: 8px;

    align-items: center;  }

    gap: 8px;  

  }  .filter-btn {

      padding: 8px 16px;

  .btn:disabled {    border: 1px solid #e5e7eb;

    opacity: 0.5;    background: white;

    cursor: not-allowed;    border-radius: 8px;

  }    font-size: 14px;

      cursor: pointer;

  .btn-primary {    transition: all 0.2s;

    background: #667eea;  }

    color: white;  

  }  .filter-btn:hover {

      background: #f3f4f6;

  .btn-secondary {  }

    background: #f3f4f6;  

    color: #374151;  .filter-btn.active {

  }    background: #667eea;

      color: white;

  .btn-danger {    border-color: #667eea;

    background: #ef4444;  }

    color: white;  

  }  .bulk-actions {

      display: flex;

  .btn-success {    align-items: center;

    background: #10b981;    gap: 12px;

    color: white;    padding-left: 16px;

  }    border-left: 2px solid #e5e7eb;

    }

  .btn-sm {  

    padding: 6px 12px;  .selection-count {

    font-size: 13px;    font-size: 14px;

  }    font-weight: 600;

      color: #667eea;

  .loading-state,  }

  .empty-state {  

    text-align: center;  .trash-table-container {

    padding: 80px 20px;    background: white;

    background: white;    border-radius: 16px;

    border-radius: 16px;    overflow: hidden;

    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);

  }  }

    

  .empty-state i {  .trash-table {

    font-size: 64px;    width: 100%;

    color: #d1d5db;    border-collapse: collapse;

    margin-bottom: 16px;  }

  }  

    .trash-table thead {

  .empty-state h3 {    background: #f9fafb;

    margin: 0 0 8px 0;    border-bottom: 2px solid #e5e7eb;

    font-size: 20px;  }

    color: #374151;  

  }  .trash-table th {

      padding: 16px;

  .empty-state p {    text-align: left;

    margin: 0;    font-size: 13px;

    color: #6b7280;    font-weight: 600;

  }    color: #6b7280;

      text-transform: uppercase;

  .spinner {    letter-spacing: 0.5px;

    width: 48px;  }

    height: 48px;  

    border: 4px solid #f3f4f6;  .trash-table th.sortable {

    border-top-color: #667eea;    cursor: pointer;

    border-radius: 50%;    user-select: none;

    animation: spin 0.8s linear infinite;    transition: background 0.2s;

    margin: 0 auto 16px;  }

  }  

    .trash-table th.sortable:hover {

  @keyframes spin {    background: #f3f4f6;

    to { transform: rotate(360deg); }  }

  }  

</style>  .trash-table tbody tr {

    border-bottom: 1px solid #e5e7eb;
    transition: background 0.2s;
  }
  
  .trash-table tbody tr:hover {
    background: #f9fafb;
  }
  
  .trash-table tbody tr.selected {
    background: #ede9fe;
  }
  
  .trash-table td {
    padding: 16px;
    font-size: 14px;
    color: #374151;
  }
  
  .icon-cell {
    font-size: 24px;
    color: #667eea;
  }
  
  .name-with-badge {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .item-name {
    font-weight: 500;
  }
  
  .path-cell {
    color: #6b7280;
    font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
    font-size: 13px;
  }
  
  .badge {
    padding: 2px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
  }
  
  .badge-folder {
    background: #dbeafe;
    color: #1e40af;
  }
  
  .expiry-badge {
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
  }
  
  .expiry-badge.neutral {
    background: #f3f4f6;
    color: #6b7280;
  }
  
  .expiry-badge.warning {
    background: #fef3c7;
    color: #92400e;
  }
  
  .expiry-badge.danger {
    background: #fee2e2;
    color: #991b1b;
  }
  
  .action-cell {
    display: flex;
    gap: 8px;
  }
  
  .action-btn {
    padding: 6px 12px;
    border: none;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 4px;
  }
  
  .restore-btn {
    background: #d1fae5;
    color: #065f46;
  }
  
  .restore-btn:hover {
    background: #a7f3d0;
  }
  
  .delete-btn {
    background: #fee2e2;
    color: #991b1b;
  }
  
  .delete-btn:hover {
    background: #fecaca;
  }
  
  .btn {
    padding: 10px 18px;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }
  
  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .btn-primary {
    background: #667eea;
    color: white;
  }
  
  .btn-primary:hover:not(:disabled) {
    background: #5568d3;
  }
  
  .btn-secondary {
    background: #f3f4f6;
    color: #374151;
  }
  
  .btn-secondary:hover {
    background: #e5e7eb;
  }
  
  .btn-danger {
    background: #ef4444;
    color: white;
  }
  
  .btn-danger:hover:not(:disabled) {
    background: #dc2626;
  }
  
  .btn-success {
    background: #10b981;
    color: white;
  }
  
  .btn-success:hover {
    background: #059669;
  }
  
  .btn-sm {
    padding: 6px 12px;
    font-size: 13px;
  }
  
  .loading-state,
  .empty-state {
    text-align: center;
    padding: 80px 20px;
    background: white;
    border-radius: 16px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  }
  
  .empty-state i {
    font-size: 64px;
    color: #d1d5db;
    margin-bottom: 16px;
  }
  
  .empty-state h3 {
    margin: 0 0 8px 0;
    font-size: 20px;
    color: #374151;
  }
  
  .empty-state p {
    margin: 0;
    color: #6b7280;
  }
  
  .spinner {
    width: 48px;
    height: 48px;
    border: 4px solid #f3f4f6;
    border-top-color: #667eea;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin: 0 auto 16px;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
