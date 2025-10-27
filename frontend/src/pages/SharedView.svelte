<script>
  import { onMount } from "svelte";
  import api from "../lib/api.js";
  import PageWrapper from "../components/PageWrapper.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";
  import Loading from "../components/Loading.svelte";

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
    password: "",
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
        password: newShare.password || null,
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
        expires_at: selectedShare.expires_at || null,
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
      password: "",
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

<PageWrapper gradient>
  <!-- Animated Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Page Header -->
  <div class="flex justify-between items-start mb-8 relative z-10">
    <div>
      <h1
        class="text-4xl font-bold gradient-text-primary mb-2 flex items-center gap-3"
      >
        <i class="bi bi-share-fill"></i>
        Shared Files
      </h1>
      <p class="text-base-content/70">Manage file sharing and collaboration</p>
    </div>
    <ModernButton
      variant="gradient"
      icon="plus-circle"
      onclick={() => (showCreateModal = true)}
    >
      Create Share
    </ModernButton>
  </div>

  {#if error}
    <div class="glass-card-light border-l-4 border-error p-4 mb-6">
      <div class="flex gap-3">
        <i class="bi bi-exclamation-triangle text-error text-2xl"></i>
        <p class="font-semibold">{error}</p>
      </div>
    </div>
  {/if}

  {#if loading}
    <Loading />
  {:else if shares.length === 0}
    <ModernCard variant="glass" class="text-center py-16">
      {#snippet children()}
        <div class="animate-fade-in">
          <div class="text-primary/30 mb-6">
            <i class="bi bi-share text-8xl"></i>
          </div>
          <h2 class="text-2xl font-bold mb-3">No shares yet</h2>
          <p class="text-base-content/60 mb-6">
            Create your first share to collaborate with others
          </p>
          <ModernButton
            variant="gradient"
            icon="plus-circle"
            onclick={() => (showCreateModal = true)}
          >
            Create Share
          </ModernButton>
        </div>
      {/snippet}
    </ModernCard>
  {:else}
    <ModernCard variant="glass">
      {#snippet children()}
        <div class="overflow-x-auto">
          <table class="table table-zebra">
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
              {#each shares as share, i (share.id)}
                <tr
                  class:expired={isExpired(share.expires_at)}
                  class="animate-slide-up"
                  style="animation-delay: {i * 30}ms;"
                >
                  <td class="font-mono text-sm text-primary"
                    >{share.file_path}</td
                  >
                  <td>
                    <span
                      class="badge badge-glass-{share.permissions === 'read'
                        ? 'info'
                        : 'warning'}"
                    >
                      {share.permissions}
                    </span>
                  </td>
                  <td>{formatDate(share.expires_at)}</td>
                  <td>{formatDate(share.created_at)}</td>
                  <td>
                    {#if share.password_hash}
                      <i
                        class="bi bi-lock-fill text-warning text-lg"
                        title="Password protected"
                      ></i>
                    {:else}
                      <i
                        class="bi bi-unlock text-base-content/30 text-lg"
                        title="No password"
                      ></i>
                    {/if}
                  </td>
                  <td>
                    <div class="flex gap-2">
                      <ModernButton
                        variant="ghost"
                        size="sm"
                        icon="clipboard"
                        onclick={() => copyShareLink(share.id)}
                      >
                        Copy
                      </ModernButton>
                      <ModernButton
                        variant="ghost"
                        size="sm"
                        icon="pencil"
                        onclick={() => openEditModal(share)}
                      >
                        Edit
                      </ModernButton>
                      <ModernButton
                        variant="danger"
                        size="sm"
                        icon="trash"
                        onclick={() => openDeleteModal(share)}
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

{#if showCreateModal}
  <div class="modal modal-open">
    <div class="modal-box glass-card max-w-2xl">
      <h3
        class="font-bold text-2xl mb-6 gradient-text-primary flex items-center gap-2"
      >
        <i class="bi bi-plus-circle-fill"></i>
        Create New Share
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">File Path</span>
          </label>
          <input
            type="text"
            class="input input-bordered glass-input"
            bind:value={newShare.file_path}
            placeholder="/path/to/file"
          />
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">Permissions</span>
          </label>
          <select
            class="select select-bordered glass-input"
            bind:value={newShare.permissions}
          >
            <option value="read">Read Only</option>
            <option value="write">Read & Write</option>
          </select>
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">Expires At (Optional)</span>
          </label>
          <input
            type="datetime-local"
            class="input input-bordered glass-input"
            bind:value={newShare.expires_at}
          />
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">Password (Optional)</span>
          </label>
          <input
            type="password"
            class="input input-bordered glass-input"
            bind:value={newShare.password}
            placeholder="Leave empty for no password"
          />
        </div>
      </div>

      <div class="modal-action mt-6">
        <ModernButton
          variant="ghost"
          onclick={() => {
            showCreateModal = false;
            resetForm();
          }}
        >
          Cancel
        </ModernButton>
        <ModernButton
          variant="gradient"
          icon="plus-circle"
          onclick={handleCreateShare}
        >
          Create Share
        </ModernButton>
      </div>
    </div>
  </div>
{/if}

{#if showEditModal && selectedShare}
  <div class="modal modal-open">
    <div class="modal-box glass-card max-w-2xl">
      <h3
        class="font-bold text-2xl mb-6 gradient-text-primary flex items-center gap-2"
      >
        <i class="bi bi-pencil-fill"></i>
        Edit Share
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">File Path</span>
          </label>
          <input
            type="text"
            class="input input-bordered glass-input"
            value={selectedShare.file_path}
            disabled
          />
          <label class="label">
            <span class="label-text-alt opacity-70"
              >File path cannot be changed</span
            >
          </label>
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">Permissions</span>
          </label>
          <select
            class="select select-bordered glass-input"
            bind:value={selectedShare.permissions}
          >
            <option value="read">Read Only</option>
            <option value="write">Read & Write</option>
          </select>
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">Expires At</span>
          </label>
          <input
            type="datetime-local"
            class="input input-bordered glass-input"
            bind:value={selectedShare.expires_at}
          />
        </div>
      </div>

      <div class="modal-action mt-6">
        <ModernButton
          variant="ghost"
          onclick={() => {
            showEditModal = false;
            selectedShare = null;
          }}
        >
          Cancel
        </ModernButton>
        <ModernButton
          variant="primary"
          icon="check-circle"
          onclick={handleUpdateShare}
        >
          Save Changes
        </ModernButton>
      </div>
    </div>
  </div>
{/if}

{#if showDeleteModal && selectedShare}
  <div class="modal modal-open">
    <div class="modal-box glass-card">
      <h3 class="font-bold text-2xl mb-6 text-error flex items-center gap-2">
        <i class="bi bi-exclamation-triangle-fill"></i>
        Delete Share
      </h3>

      <div class="glass-card-light border-l-4 border-error p-4 mb-4">
        <p>
          Are you sure you want to delete the share for <strong
            class="font-mono text-primary">{selectedShare.file_path}</strong
          >?
        </p>
        <p class="text-sm text-base-content/70 mt-2">
          This action cannot be undone.
        </p>
      </div>

      <div class="modal-action">
        <ModernButton
          variant="ghost"
          onclick={() => {
            showDeleteModal = false;
            selectedShare = null;
          }}
        >
          Cancel
        </ModernButton>
        <ModernButton
          variant="danger"
          icon="trash-fill"
          onclick={handleDeleteShare}
        >
          Delete Share
        </ModernButton>
      </div>
    </div>
  </div>
{/if}

<style>
  .table tr.expired {
    opacity: 0.5;
    text-decoration: line-through;
  }

  .glass-input {
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(10px);
  }
</style>
