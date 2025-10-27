<script>
  import { onMount } from "svelte";
  import api from "../lib/api.js";

  let shares = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let showCreateModal = $state(false);
  let showEditModal = $state(false);
  let showDeleteModal = $state(false);
  let selectedShare = $state(null);
  
  let newShare = $state({
    file_path: "",
    permissions: "read",
    expires_at: "",
    password: ""
  });

  onMount(async () => {
    await loadShares();
  });

  async function loadShares() {
    try {
      loading = true;
      error = null;
      const response = await api.shares.list();
      shares = response || [];
    } catch (err) {
      console.error("Failed to load shares:", err);
      error = "Failed to load shares";
      shares = [];
    } finally {
      loading = false;
    }
  }

  async function handleCreateShare() {
    try {
      const shareData = {
        file_path: newShare.file_path,
        permissions: newShare.permissions,
        expires_at: newShare.expires_at || null,
        password: newShare.password || null
      };
      
      await api.shares.create(shareData);
      showCreateModal = false;
      resetForm();
      await loadShares();
    } catch (err) {
      console.error("Failed to create share:", err);
      error = "Failed to create share";
    }
  }

  async function handleUpdateShare() {
    if (!selectedShare) return;
    
    try {
      const updates = {
        permissions: selectedShare.permissions,
        expires_at: selectedShare.expires_at || null
      };
      
      await api.shares.update(selectedShare.id, updates);
      showEditModal = false;
      selectedShare = null;
      await loadShares();
    } catch (err) {
      console.error("Failed to update share:", err);
      error = "Failed to update share";
    }
  }

  async function handleDeleteShare() {
    if (!selectedShare) return;
    
    try {
      await api.shares.delete(selectedShare.id);
      showDeleteModal = false;
      selectedShare = null;
      await loadShares();
    } catch (err) {
      console.error("Failed to delete share:", err);
      error = "Failed to delete share";
    }
  }

  function openEditModal(share) {
    selectedShare = { ...share };
    showEditModal = true;
  }

  function openDeleteModal(share) {
    selectedShare = share;
    showDeleteModal = true;
  }

  function resetForm() {
    newShare = {
      file_path: "",
      permissions: "read",
      expires_at: "",
      password: ""
    };
  }

  function formatDate(dateString) {
    if (!dateString) return "Never";
    return new Date(dateString).toLocaleDateString();
  }

  function copyShareLink(shareId) {
    const link = `${window.location.origin}/share/${shareId}`;
    navigator.clipboard.writeText(link);
  }

  function isExpired(expiresAt) {
    if (!expiresAt) return false;
    return new Date(expiresAt) < new Date();
  }
</script>

<div class="shared-view">
  <div class="page-header">
    <h1 class="page-title">Shared Files</h1>
    <button class="btn btn-primary" onclick={() => showCreateModal = true}>
      <i class="bi bi-plus-circle"></i>
      Create Share
    </button>
  </div>

  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-triangle"></i>
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="loading-container">
      <span class="loading loading-spinner loading-lg"></span>
      <p>Loading shares...</p>
    </div>
  {:else if shares.length === 0}
    <div class="empty-state">
      <i class="bi bi-share"></i>
      <h2>No shares yet</h2>
      <p>Create your first share to collaborate with others</p>
      <button class="btn btn-primary" onclick={() => showCreateModal = true}>
        Create Share
      </button>
    </div>
  {:else}
    <div class="shares-table-container">
      <table class="table">
        <thead>
          <tr>
            <th>File Path</th>
            <th>Permissions</th>
            <th>Expires</th>
            <th>Created</th>
            <th>Protected</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each shares as share (share.id)}
            <tr class:expired={isExpired(share.expires_at)}>
              <td class="file-path">{share.file_path}</td>
              <td>
                <span class="badge badge-{share.permissions === 'read' ? 'info' : 'warning'}">
                  {share.permissions}
                </span>
              </td>
              <td>{formatDate(share.expires_at)}</td>
              <td>{formatDate(share.created_at)}</td>
              <td>
                {#if share.password_hash}
                  <i class="bi bi-lock-fill text-warning"></i>
                {:else}
                  <i class="bi bi-unlock text-muted"></i>
                {/if}
              </td>
              <td class="actions">
                <button 
                  class="btn btn-sm btn-ghost"
                  onclick={() => copyShareLink(share.id)}
                  title="Copy Link"
                >
                  <i class="bi bi-clipboard"></i>
                </button>
                <button 
                  class="btn btn-sm btn-ghost"
                  onclick={() => openEditModal(share)}
                  title="Edit"
                >
                  <i class="bi bi-pencil"></i>
                </button>
                <button 
                  class="btn btn-sm btn-ghost btn-error"
                  onclick={() => openDeleteModal(share)}
                  title="Delete"
                >
                  <i class="bi bi-trash"></i>
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

{#if showCreateModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">Create New Share</h3>
      
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">File Path</span>
        </label>
        <input 
          type="text" 
          class="input input-bordered" 
          bind:value={newShare.file_path}
          placeholder="/path/to/file"
        />
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Permissions</span>
        </label>
        <select class="select select-bordered" bind:value={newShare.permissions}>
          <option value="read">Read Only</option>
          <option value="write">Read & Write</option>
        </select>
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Expires At (Optional)</span>
        </label>
        <input 
          type="datetime-local" 
          class="input input-bordered" 
          bind:value={newShare.expires_at}
        />
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Password (Optional)</span>
        </label>
        <input 
          type="password" 
          class="input input-bordered" 
          bind:value={newShare.password}
          placeholder="Leave empty for no password"
        />
      </div>

      <div class="modal-action">
        <button class="btn" onclick={() => { showCreateModal = false; resetForm(); }}>
          Cancel
        </button>
        <button class="btn btn-primary" onclick={handleCreateShare}>
          Create Share
        </button>
      </div>
    </div>
  </div>
{/if}

{#if showEditModal && selectedShare}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">Edit Share</h3>
      
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">File Path</span>
        </label>
        <input 
          type="text" 
          class="input input-bordered" 
          value={selectedShare.file_path}
          disabled
        />
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Permissions</span>
        </label>
        <select class="select select-bordered" bind:value={selectedShare.permissions}>
          <option value="read">Read Only</option>
          <option value="write">Read & Write</option>
        </select>
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Expires At</span>
        </label>
        <input 
          type="datetime-local" 
          class="input input-bordered" 
          bind:value={selectedShare.expires_at}
        />
      </div>

      <div class="modal-action">
        <button class="btn" onclick={() => { showEditModal = false; selectedShare = null; }}>
          Cancel
        </button>
        <button class="btn btn-primary" onclick={handleUpdateShare}>
          Save Changes
        </button>
      </div>
    </div>
  </div>
{/if}

{#if showDeleteModal && selectedShare}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">Delete Share</h3>
      <p class="mb-4">
        Are you sure you want to delete the share for <strong>{selectedShare.file_path}</strong>? 
        This action cannot be undone.
      </p>

      <div class="modal-action">
        <button class="btn" onclick={() => { showDeleteModal = false; selectedShare = null; }}>
          Cancel
        </button>
        <button class="btn btn-error" onclick={handleDeleteShare}>
          Delete
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .shared-view {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .page-title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--md-sys-color-on-surface);
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem;
    gap: 1rem;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem;
    text-align: center;
    gap: 1rem;
  }

  .empty-state i {
    font-size: 4rem;
    opacity: 0.3;
  }

  .empty-state h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0;
  }

  .empty-state p {
    color: var(--md-sys-color-on-surface-variant);
    margin: 0;
  }

  .shares-table-container {
    background: var(--md-sys-color-surface-container);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: var(--md-sys-elevation-1);
  }

  .table {
    width: 100%;
  }

  .table th {
    background: var(--md-sys-color-surface-container-high);
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 600;
    text-transform: uppercase;
    font-size: 0.75rem;
    letter-spacing: 0.5px;
    padding: 1rem;
  }

  .table td {
    padding: 1rem;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .table tr:last-child td {
    border-bottom: none;
  }

  .table tr.expired {
    opacity: 0.5;
  }

  .file-path {
    font-family: 'Courier New', monospace;
    color: var(--md-sys-color-primary);
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .badge {
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .badge-info {
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }

  .badge-warning {
    background: var(--md-sys-color-tertiary-container);
    color: var(--md-sys-color-on-tertiary-container);
  }

  .text-warning {
    color: var(--md-sys-color-tertiary);
  }

  .text-muted {
    opacity: 0.5;
  }
</style>
