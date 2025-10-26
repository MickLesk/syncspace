<script><script>

  import { onMount } from 'svelte';  import { currentLang } from "../stores/ui.js";

  import api from '../lib/api.js';  import { t } from "../i18n.js";

  import { showToast } from '../stores/ui.js';  import EmptyState from "../components/ui/EmptyState.svelte";

</script>

  let shares = $state([]);

  let loading = $state(true);<div class="view-container">

  let viewMode = $state('grid'); // 'grid' or 'list'  <div class="page-content">

  let filterStatus = $state('all'); // 'all', 'active', 'expired'    <EmptyState

  let selectedShares = $state(new Set());      icon="🔗"

  let showQRModal = $state(false);      title={t($currentLang, "noSharedFiles")}

  let currentQRShare = $state(null);      description={t($currentLang, "shareWithOthers")}

    />

  onMount(async () => {  </div>

    await loadShares();</div>

  });

<style>

  async function loadShares() {  .view-container {

    try {    padding: 0;

      loading = true;    max-width: 1400px;

      const response = await api.get('/api/shares');    margin: 0 auto;

      shares = response.data || [];  }

    } catch (error) {

      console.error('Failed to load shares:', error);  .page-content {

      showToast('Failed to load shares', 'error');    padding: 0 32px;

    } finally {  }

      loading = false;</style>

    }
  }

  function getFilteredShares() {
    if (filterStatus === 'all') return shares;
    const now = new Date();
    if (filterStatus === 'active') {
      return shares.filter(s => !s.expires_at || new Date(s.expires_at) > now);
    }
    if (filterStatus === 'expired') {
      return shares.filter(s => s.expires_at && new Date(s.expires_at) <= now);
    }
    return shares;
  }

  function copyShareLink(share) {
    const link = `${window.location.origin}/share/${share.share_token}`;
    navigator.clipboard.writeText(link);
    showToast('Share link copied to clipboard!', 'success');
  }

  function showQRCode(share) {
    currentQRShare = share;
    showQRModal = true;
  }

  function closeQRModal() {
    showQRModal = false;
    currentQRShare = null;
  }

  function getShareLink(share) {
    return `${window.location.origin}/share/${share.share_token}`;
  }

  function getExpiryStatus(share) {
    if (!share.expires_at) return { text: 'Never expires', class: 'badge-success' };
    const expiryDate = new Date(share.expires_at);
    const now = new Date();
    const daysLeft = Math.ceil((expiryDate - now) / (1000 * 60 * 60 * 24));

    if (daysLeft < 0) return { text: 'Expired', class: 'badge-error' };
    if (daysLeft === 0) return { text: 'Expires today', class: 'badge-warning' };
    if (daysLeft <= 3) return { text: `${daysLeft} days left`, class: 'badge-warning' };
    return { text: `${daysLeft} days left`, class: 'badge-info' };
  }

  function getFileIcon(filename) {
    const ext = filename.split('.').pop().toLowerCase();
    const iconMap = {
      pdf: 'bi-file-pdf-fill text-error',
      doc: 'bi-file-word-fill text-primary',
      docx: 'bi-file-word-fill text-primary',
      xls: 'bi-file-excel-fill text-success',
      xlsx: 'bi-file-excel-fill text-success',
      ppt: 'bi-file-ppt-fill text-warning',
      pptx: 'bi-file-ppt-fill text-warning',
      zip: 'bi-file-zip-fill text-secondary',
      rar: 'bi-file-zip-fill text-secondary',
      jpg: 'bi-file-image-fill text-accent',
      jpeg: 'bi-file-image-fill text-accent',
      png: 'bi-file-image-fill text-accent',
      gif: 'bi-file-image-fill text-accent',
      mp4: 'bi-file-play-fill text-error',
      mp3: 'bi-file-music-fill text-warning',
      txt: 'bi-file-text-fill',
      md: 'bi-file-text-fill',
    };
    return iconMap[ext] || 'bi-file-earmark-fill';
  }

  async function deleteShare(shareId) {
    if (!confirm('Are you sure you want to delete this share?')) return;
    
    try {
      await api.delete(`/api/shares/${shareId}`);
      shares = shares.filter(s => s.id !== shareId);
      showToast('Share deleted successfully', 'success');
    } catch (error) {
      console.error('Failed to delete share:', error);
      showToast('Failed to delete share', 'error');
    }
  }

  function toggleSelectShare(shareId) {
    if (selectedShares.has(shareId)) {
      selectedShares.delete(shareId);
    } else {
      selectedShares.add(shareId);
    }
    selectedShares = selectedShares; // Trigger reactivity
  }

  async function bulkDeleteShares() {
    if (selectedShares.size === 0) return;
    if (!confirm(`Delete ${selectedShares.size} selected share(s)?`)) return;

    try {
      await Promise.all(
        Array.from(selectedShares).map(id => api.delete(`/api/shares/${id}`))
      );
      shares = shares.filter(s => !selectedShares.has(s.id));
      selectedShares.clear();
      selectedShares = selectedShares; // Trigger reactivity
      showToast('Selected shares deleted', 'success');
    } catch (error) {
      console.error('Failed to delete shares:', error);
      showToast('Failed to delete shares', 'error');
    }
  }

  function formatDate(dateString) {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function formatFileSize(bytes) {
    if (!bytes) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  }

  $: filteredShares = getFilteredShares();
</script>

<!-- Header -->
<div class="share-header">
  <div class="header-top">
    <div>
      <h1 class="text-3xl font-bold">
        <i class="bi bi-share-fill"></i> Shared Files
      </h1>
      <p class="text-base-content/60 mt-1">Manage your shared files and links</p>
    </div>
    <button class="btn btn-primary gap-2">
      <i class="bi bi-plus-lg"></i>
      Create Share
    </button>
  </div>

  <!-- Filters & View Controls -->
  <div class="header-controls">
    <!-- Filter Tabs -->
    <div class="join">
      <button 
        class="join-item btn btn-sm" 
        class:btn-active={filterStatus === 'all'}
        onclick={() => filterStatus = 'all'}>
        All Shares
      </button>
      <button 
        class="join-item btn btn-sm" 
        class:btn-active={filterStatus === 'active'}
        onclick={() => filterStatus = 'active'}>
        Active
      </button>
      <button 
        class="join-item btn btn-sm" 
        class:btn-active={filterStatus === 'expired'}
        onclick={() => filterStatus = 'expired'}>
        Expired
      </button>
    </div>

    <!-- View Mode Toggle -->
    <div class="join">
      <button 
        class="join-item btn btn-sm" 
        class:btn-active={viewMode === 'grid'}
        onclick={() => viewMode = 'grid'}>
        <i class="bi bi-grid-3x3-gap"></i>
      </button>
      <button 
        class="join-item btn btn-sm" 
        class:btn-active={viewMode === 'list'}
        onclick={() => viewMode = 'list'}>
        <i class="bi bi-list-ul"></i>
      </button>
    </div>

    <!-- Bulk Actions -->
    {#if selectedShares.size > 0}
      <div class="bulk-actions">
        <span class="text-sm text-base-content/60">{selectedShares.size} selected</span>
        <button class="btn btn-sm btn-error gap-2" onclick={bulkDeleteShares}>
          <i class="bi bi-trash"></i>
          Delete Selected
        </button>
      </div>
    {/if}
  </div>
</div>

<!-- Loading State -->
{#if loading}
  <div class="loading-container">
    <span class="loading loading-spinner loading-lg"></span>
    <p class="text-base-content/60 mt-4">Loading shares...</p>
  </div>
{:else if filteredShares.length === 0}
  <!-- Empty State -->
  <div class="empty-state">
    <i class="bi bi-share empty-icon"></i>
    <h3 class="text-xl font-semibold mt-4">No Shared Files</h3>
    <p class="text-base-content/60 mt-2">
      {filterStatus === 'all' 
        ? 'Create your first share to get started' 
        : `No ${filterStatus} shares found`}
    </p>
    <button class="btn btn-primary mt-4 gap-2">
      <i class="bi bi-plus-lg"></i>
      Create Share
    </button>
  </div>
{:else}
  <!-- Grid View -->
  {#if viewMode === 'grid'}
    <div class="shares-grid">
      {#each filteredShares as share (share.id)}
        <div class="share-card">
          <!-- Card Header with Checkbox -->
          <div class="card-header">
            <input 
              type="checkbox" 
              class="checkbox checkbox-sm"
              checked={selectedShares.has(share.id)}
              onchange={() => toggleSelectShare(share.id)} />
            {#if share.thumbnail_url}
              <img src={share.thumbnail_url} alt={share.filename} class="share-thumbnail" />
            {:else}
              <div class="share-icon">
                <i class="{getFileIcon(share.filename)} text-5xl"></i>
              </div>
            {/if}
          </div>

          <!-- Card Content -->
          <div class="card-content">
            <h3 class="share-filename" title={share.filename}>
              {share.filename}
            </h3>

            <!-- Share Stats -->
            <div class="share-stats">
              <div class="stat-item">
                <i class="bi bi-eye"></i>
                <span>{share.access_count || 0} views</span>
              </div>
              <div class="stat-item">
                <i class="bi bi-download"></i>
                <span>{share.download_count || 0} downloads</span>
              </div>
              <div class="stat-item">
                <i class="bi bi-hdd"></i>
                <span>{formatFileSize(share.file_size)}</span>
              </div>
            </div>

            <!-- Expiry Badge -->
            <div class="share-meta">
              <span class="badge {getExpiryStatus(share).class} badge-sm">
                {getExpiryStatus(share).text}
              </span>
              <span class="text-xs text-base-content/50">
                Created {formatDate(share.created_at)}
              </span>
            </div>

            <!-- Actions -->
            <div class="share-actions">
              <button 
                class="btn btn-sm btn-primary flex-1 gap-2"
                onclick={() => copyShareLink(share)}>
                <i class="bi bi-clipboard"></i>
                Copy Link
              </button>
              <button 
                class="btn btn-sm btn-ghost gap-2"
                onclick={() => showQRCode(share)}>
                <i class="bi bi-qr-code"></i>
              </button>
              <button 
                class="btn btn-sm btn-ghost btn-error gap-2"
                onclick={() => deleteShare(share.id)}>
                <i class="bi bi-trash"></i>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <!-- List View -->
    <div class="shares-table-container">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>
              <input type="checkbox" class="checkbox checkbox-sm" />
            </th>
            <th>File</th>
            <th>Views</th>
            <th>Downloads</th>
            <th>Size</th>
            <th>Status</th>
            <th>Created</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredShares as share (share.id)}
            <tr>
              <td>
                <input 
                  type="checkbox" 
                  class="checkbox checkbox-sm"
                  checked={selectedShares.has(share.id)}
                  onchange={() => toggleSelectShare(share.id)} />
              </td>
              <td>
                <div class="flex items-center gap-3">
                  <i class="{getFileIcon(share.filename)} text-2xl"></i>
                  <span class="font-medium">{share.filename}</span>
                </div>
              </td>
              <td>
                <div class="flex items-center gap-1">
                  <i class="bi bi-eye"></i>
                  {share.access_count || 0}
                </div>
              </td>
              <td>
                <div class="flex items-center gap-1">
                  <i class="bi bi-download"></i>
                  {share.download_count || 0}
                </div>
              </td>
              <td>{formatFileSize(share.file_size)}</td>
              <td>
                <span class="badge {getExpiryStatus(share).class} badge-sm">
                  {getExpiryStatus(share).text}
                </span>
              </td>
              <td>{formatDate(share.created_at)}</td>
              <td>
                <div class="flex gap-1">
                  <button 
                    class="btn btn-sm btn-ghost"
                    onclick={() => copyShareLink(share)}
                    title="Copy link">
                    <i class="bi bi-clipboard"></i>
                  </button>
                  <button 
                    class="btn btn-sm btn-ghost"
                    onclick={() => showQRCode(share)}
                    title="Show QR code">
                    <i class="bi bi-qr-code"></i>
                  </button>
                  <button 
                    class="btn btn-sm btn-ghost btn-error"
                    onclick={() => deleteShare(share.id)}
                    title="Delete">
                    <i class="bi bi-trash"></i>
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
{/if}

<!-- QR Code Modal -->
{#if showQRModal && currentQRShare}
  <div class="modal modal-open" onclick={closeQRModal}>
    <div class="modal-box" onclick={(e) => e.stopPropagation()}>
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-qr-code"></i> QR Code for {currentQRShare.filename}
      </h3>
      
      <!-- QR Code Container -->
      <div class="qr-container">
        <div class="qr-placeholder">
          <i class="bi bi-qr-code text-6xl text-base-content/20"></i>
          <p class="text-sm text-base-content/50 mt-2">QR Code generation requires backend implementation</p>
        </div>
      </div>

      <!-- Share Link -->
      <div class="share-link-box">
        <input 
          type="text" 
          class="input input-bordered w-full" 
          readonly 
          value={getShareLink(currentQRShare)} />
        <button 
          class="btn btn-primary mt-2 w-full gap-2"
          onclick={() => copyShareLink(currentQRShare)}>
          <i class="bi bi-clipboard"></i>
          Copy Link
        </button>
      </div>

      <div class="modal-action">
        <button class="btn" onclick={closeQRModal}>Close</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .share-header {
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

  .header-controls {
    display: flex;
    gap: 1rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .bulk-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    padding: 0.5rem 1rem;
    background: hsl(var(--b2));
    border-radius: var(--rounded-btn);
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    text-align: center;
  }

  .empty-icon {
    font-size: 4rem;
    color: hsl(var(--bc) / 0.2);
  }

  /* Grid View */
  .shares-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 1.5rem;
    padding: 1rem;
  }

  .share-card {
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    animation: slideEnter 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .share-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 10px 30px hsl(var(--bc) / 0.15);
    border-color: hsl(var(--p) / 0.3);
  }

  .card-header {
    position: relative;
    height: 200px;
    background: linear-gradient(135deg, hsl(var(--p) / 0.1) 0%, hsl(var(--s) / 0.1) 100%);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .card-header .checkbox {
    position: absolute;
    top: 1rem;
    left: 1rem;
    z-index: 10;
  }

  .share-thumbnail {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .share-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.6;
  }

  .card-content {
    padding: 1.5rem;
  }

  .share-filename {
    font-size: 1rem;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 1rem;
  }

  .share-stats {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
    padding: 0.75rem;
    background: hsl(var(--b2));
    border-radius: var(--rounded-btn);
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.7);
  }

  .stat-item i {
    font-size: 1rem;
  }

  .share-meta {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    gap: 0.5rem;
  }

  .share-actions {
    display: flex;
    gap: 0.5rem;
  }

  /* List View */
  .shares-table-container {
    background: hsl(var(--b1));
    border-radius: var(--rounded-box);
    overflow: hidden;
    border: 1px solid hsl(var(--bc) / 0.1);
  }

  .table :where(th, td) {
    padding: 1rem;
  }

  /* QR Modal */
  .qr-container {
    display: flex;
    justify-content: center;
    padding: 2rem;
    background: hsl(var(--b2));
    border-radius: var(--rounded-box);
    margin-bottom: 1rem;
  }

  .qr-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 200px;
  }

  .share-link-box {
    margin-top: 1rem;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .share-header {
      padding: 1.5rem;
      margin: -1.5rem -1.5rem 1.5rem -1.5rem;
    }

    .header-top {
      flex-direction: column;
      gap: 1rem;
    }

    .shares-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
      padding: 0.5rem;
    }

    .header-controls {
      width: 100%;
    }

    .join {
      flex: 1;
    }
  }
</style>
