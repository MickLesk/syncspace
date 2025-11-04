<script>
  import { onMount } from "svelte";
  import api from "../../lib/api";
  import { success, error as errorToast } from "../../stores/toast";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

  let shares = $state([]);
  let loading = $state(true);
  let errorMsg = $state(null);
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
      errorMsg = null;
      const response = await api.shares.list();
      shares = response || [];
    } catch (err) {
      console.error("Failed to load shares:", err);
      errorMsg = "Failed to load shares";
      errorToast("Failed to load shares");
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
      success("Share created successfully");
      showCreateModal = false;
      resetForm();
      await loadShares();
    } catch (err) {
      console.error("Failed to create share:", err);
      errorToast("Failed to create share");
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
      success("Share updated successfully");
      showEditModal = false;
      selectedShare = null;
      await loadShares();
    } catch (err) {
      console.error("Failed to update share:", err);
      errorToast("Failed to update share");
    }
  }

  async function handleDeleteShare() {
    if (!selectedShare) return;

    try {
      await api.shares.delete(selectedShare.id);
      success("Share deleted successfully");
      showDeleteModal = false;
      selectedShare = null;
      await loadShares();
    } catch (err) {
      console.error("Failed to delete share:", err);
      errorToast("Failed to delete share");
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

  async function copyShareLink(shareId) {
    const link = `${window.location.origin}/share/${shareId}`;
    try {
      await navigator.clipboard.writeText(link);
      success("Share link copied to clipboard");
    } catch (err) {
      errorToast("Failed to copy link");
    }
  }

  function isExpired(expiresAt) {
    if (!expiresAt) return false;
    return new Date(expiresAt) < new Date();
  }
</script>

<PageWrapper gradient>
  <div class="page-fade-in">
    <PageHeader
      title="Shared Files"
      subtitle="Manage file shares and collaboration"
      icon="share-fill"
    >
      {#snippet actions()}
        <ModernButton
          variant="gradient"
          icon="plus-circle"
          onclick={() => (showCreateModal = true)}
          class="btn-pulse"
        >
          Create Share
        </ModernButton>
      {/snippet}
    </PageHeader>

    {#if loading}
      <div class="space-y-4">
        {#each Array(5) as _}
          <div class="skeleton h-32 w-full rounded-xl"></div>
        {/each}
      </div>
    {:else}
      <div class="space-y-6">
        {#if errorMsg}
          <ModernCard variant="glass" padding="normal">
            {#snippet children()}
              <div
                class="flex items-center gap-3 border-l-4 border-red-500 pl-4"
              >
                <i class="bi bi-exclamation-triangle text-red-500 text-2xl"></i>
                <p class="font-semibold text-gray-900 dark:text-gray-100">
                  {errorMsg}
                </p>
              </div>
            {/snippet}
          </ModernCard>
        {/if}
        {#if shares.length === 0}
          <ModernCard variant="glass" padding="large">
            {#snippet children()}
              <div class="text-center py-8 animate-fade-in">
                <div class="text-8xl mb-6 opacity-30">
                  <i class="bi bi-share"></i>
                </div>
                <h2
                  class="text-2xl font-bold mb-3 text-gray-900 dark:text-gray-100"
                >
                  No shares yet
                </h2>
                <p class="text-gray-600 dark:text-gray-400 mb-6">
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
          <ModernCard variant="glass" padding="none">
            {#snippet children()}
              <div class="overflow-x-auto">
                <table class="w-full">
                  <thead>
                    <tr
                      class="border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50"
                    >
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                        >File Path</th
                      >
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                        >Permissions</th
                      >
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                        >Expires</th
                      >
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                        >Created</th
                      >
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                        >Protected</th
                      >
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                        >Actions</th
                      >
                    </tr>
                  </thead>
                  <tbody>
                    {#each shares as share, i (share.id)}
                      <tr
                        class="border-b border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors animate-slide-up"
                        class:opacity-50={isExpired(share.expires_at)}
                        style="animation-delay: {i * 30}ms;"
                      >
                        <td
                          class="px-6 py-4 font-mono text-sm text-primary-600 dark:text-primary-400"
                        >
                          {share.file_path}
                        </td>
                        <td class="px-6 py-4">
                          <span
                            class="badge-glass-{share.permissions === 'read'
                              ? 'info'
                              : 'warning'}"
                          >
                            {share.permissions}
                          </span>
                        </td>
                        <td
                          class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400"
                        >
                          {formatDate(share.expires_at)}
                        </td>
                        <td
                          class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400"
                        >
                          {formatDate(share.created_at)}
                        </td>
                        <td class="px-6 py-4">
                          {#if share.password_hash}
                            <i class="bi bi-lock-fill text-yellow-500 text-lg"
                            ></i>
                          {:else}
                            <i class="bi bi-unlock text-gray-400 text-lg"></i>
                          {/if}
                        </td>
                        <td class="px-6 py-4">
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
                              variant="secondary"
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
              <div
                class="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50 px-6 py-4"
              >
                <div class="flex items-center justify-between text-sm">
                  <div class="text-gray-600 dark:text-gray-400">
                    <span class="font-semibold">{shares.length}</span> total
                    share{shares.length !== 1 ? "s" : ""}
                  </div>
                  <div class="flex gap-4 text-gray-600 dark:text-gray-400">
                    <div class="flex items-center gap-2">
                      <i class="bi bi-lock-fill text-yellow-500"></i>
                      <span
                        >{shares.filter((s) => s.password_hash).length} protected</span
                      >
                    </div>
                    <div class="flex items-center gap-2">
                      <i class="bi bi-exclamation-triangle text-red-500"></i>
                      <span
                        >{shares.filter((s) => isExpired(s.expires_at)).length} expired</span
                      >
                    </div>
                  </div>
                </div>
              </div>
            {/snippet}
          </ModernCard>
        {/if}
      </div>
    {/if}
  </div>

  {#if showCreateModal}
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4 modal-backdrop"
    >
      <ModernCard
        variant="glass"
        padding="large"
        class="max-w-md w-full modal-content"
      >
        {#snippet children()}
          <h2 class="text-2xl font-bold mb-6 text-gray-900 dark:text-gray-100">
            Create Share
          </h2>
          <form
            onsubmit={(e) => {
              e.preventDefault();
              handleCreateShare();
            }}
            class="space-y-5"
          >
            <div>
              <label
                for="file_path"
                class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >
                File Path
              </label>
              <input
                id="file_path"
                type="text"
                bind:value={newShare.file_path}
                placeholder="/path/to/file.txt"
                class="glass-input w-full px-4 py-2.5 rounded-lg"
                required
              />
            </div>
            <div>
              <label
                for="permissions"
                class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >
                Permissions
              </label>
              <select
                id="permissions"
                bind:value={newShare.permissions}
                class="glass-input w-full px-4 py-2.5 rounded-lg"
              >
                <option value="read">Read Only</option>
                <option value="write">Read & Write</option>
              </select>
            </div>
            <div>
              <label
                for="expires_at"
                class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >
                Expiration Date (Optional)
              </label>
              <input
                id="expires_at"
                type="datetime-local"
                bind:value={newShare.expires_at}
                class="glass-input w-full px-4 py-2.5 rounded-lg"
              />
            </div>
            <div>
              <label
                for="password"
                class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >
                Password (Optional)
              </label>
              <input
                id="password"
                type="password"
                bind:value={newShare.password}
                placeholder="Leave empty for no password"
                class="glass-input w-full px-4 py-2.5 rounded-lg"
              />
            </div>
            <div class="flex justify-end gap-3 pt-4">
              <ModernButton
                variant="ghost"
                onclick={() => (showCreateModal = false)}
              >
                Cancel
              </ModernButton>
              <ModernButton type="submit" variant="gradient" icon="plus-circle">
                Create Share
              </ModernButton>
            </div>
          </form>
        {/snippet}
      </ModernCard>
    </div>
  {/if}

  {#if showEditModal}
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4 modal-backdrop"
    >
      <ModernCard
        variant="glass"
        padding="large"
        class="max-w-md w-full modal-content"
      >
        {#snippet children()}
          <h2 class="text-2xl font-bold mb-6 text-gray-900 dark:text-gray-100">
            Edit Share
          </h2>
          {#if selectedShare}
            <form
              onsubmit={(e) => {
                e.preventDefault();
                handleUpdateShare();
              }}
              class="space-y-5"
            >
              <div>
                <label
                  class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
                >
                  File Path
                </label>
                <input
                  type="text"
                  value={selectedShare.file_path}
                  disabled
                  class="glass-input w-full px-4 py-2.5 rounded-lg opacity-60"
                />
              </div>
              <div>
                <label
                  for="edit_permissions"
                  class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
                >
                  Permissions
                </label>
                <select
                  id="edit_permissions"
                  bind:value={selectedShare.permissions}
                  class="glass-input w-full px-4 py-2.5 rounded-lg"
                >
                  <option value="read">Read Only</option>
                  <option value="write">Read & Write</option>
                </select>
              </div>
              <div>
                <label
                  for="edit_expires_at"
                  class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
                >
                  Expiration Date (Optional)
                </label>
                <input
                  id="edit_expires_at"
                  type="datetime-local"
                  bind:value={selectedShare.expires_at}
                  class="glass-input w-full px-4 py-2.5 rounded-lg"
                />
              </div>
              <div>
                <label
                  for="edit_password"
                  class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
                >
                  Password (Leave empty to keep current)
                </label>
                <input
                  id="edit_password"
                  type="password"
                  bind:value={selectedShare.password}
                  placeholder="Enter new password or leave empty"
                  class="glass-input w-full px-4 py-2.5 rounded-lg"
                />
              </div>
              <div class="flex justify-end gap-3 pt-4">
                <ModernButton
                  variant="ghost"
                  onclick={() => {
                    showEditModal = false;
                    selectedShare = null;
                  }}
                >
                  Cancel
                </ModernButton>
                <ModernButton type="submit" variant="gradient" icon="save">
                  Save Changes
                </ModernButton>
              </div>
            </form>
          {/if}
        {/snippet}
      </ModernCard>
    </div>
  {/if}

  {#if showDeleteModal}
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4 modal-backdrop"
    >
      <ModernCard
        variant="glass"
        padding="large"
        class="max-w-md w-full modal-content"
      >
        {#snippet children()}
          <h2 class="text-2xl font-bold mb-6 text-gray-900 dark:text-gray-100">
            Delete Share
          </h2>
          {#if selectedShare}
            <div class="space-y-4">
              <div class="glass-card border-l-4 border-red-500 p-4">
                <div class="flex items-start gap-3">
                  <i
                    class="bi bi-exclamation-triangle-fill text-red-500 text-2xl"
                  ></i>
                  <div class="flex-1">
                    <p
                      class="font-semibold text-gray-900 dark:text-gray-100 mb-2"
                    >
                      Are you sure you want to delete this share?
                    </p>
                    <p class="text-sm text-gray-600 dark:text-gray-400">
                      File: <span
                        class="font-mono font-semibold text-primary-600 dark:text-primary-400"
                      >
                        {selectedShare.file_path}
                      </span>
                    </p>
                    <p class="text-sm text-red-600 dark:text-red-400 mt-2">
                      This action cannot be undone.
                    </p>
                  </div>
                </div>
              </div>
              <div class="flex justify-end gap-3 pt-2">
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
          {/if}
        {/snippet}
      </ModernCard>
    </div>
  {/if}
</PageWrapper>
