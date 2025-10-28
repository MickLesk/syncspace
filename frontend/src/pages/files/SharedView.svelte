<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { success, error as errorToast } from "../../stores/toast";
  import Loading from "../../components/Loading.svelte";
  import Modal from "../../components/ui/Modal.svelte";

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

{#if loading}
  <Loading />
{:else}
  <div
    class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-6"
  >
    <div class="blob blob-1"></div>
    <div class="blob blob-2"></div>
    <div class="blob blob-3"></div>
    <div class="relative z-10 max-w-7xl mx-auto">
      <div class="glass-card mb-6 p-6">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-3xl font-bold gradient-text mb-2">Shared Files</h1>
            <p class="text-gray-600 dark:text-gray-400">
              Manage file shares and collaboration
            </p>
          </div>
          <button
            onclick={() => (showCreateModal = true)}
            class="px-6 py-3 rounded-lg font-medium bg-gradient-to-r from-blue-500 to-purple-600 text-white hover:from-blue-600 hover:to-purple-700 shadow-lg hover:shadow-xl transition-all flex items-center gap-2"
            ><i class="bi bi-plus-circle"></i> Create Share</button
          >
        </div>
      </div>
      {#if errorMsg}
        <div class="glass-card-light border-l-4 border-red-500 p-4 mb-6">
          <div class="flex items-center gap-3">
            <i class="bi bi-exclamation-triangle text-red-500 text-2xl"></i>
            <p class="font-semibold text-gray-900 dark:text-gray-100">
              {errorMsg}
            </p>
          </div>
        </div>
      {/if}
      {#if shares.length === 0}
        <div class="glass-card text-center py-16">
          <div class="animate-fade-in">
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
            <button
              onclick={() => (showCreateModal = true)}
              class="px-6 py-3 rounded-lg font-medium bg-gradient-to-r from-blue-500 to-purple-600 text-white hover:from-blue-600 hover:to-purple-700 shadow-lg hover:shadow-xl transition-all inline-flex items-center gap-2"
              ><i class="bi bi-plus-circle"></i> Create Share</button
            >
          </div>
        </div>
      {:else}
        <div class="glass-card overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead
                ><tr
                  class="border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50"
                  ><th
                    class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                    >File Path</th
                  ><th
                    class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                    >Permissions</th
                  ><th
                    class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                    >Expires</th
                  ><th
                    class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                    >Created</th
                  ><th
                    class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                    >Protected</th
                  ><th
                    class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                    >Actions</th
                  ></tr
                ></thead
              ><tbody
                >{#each shares as share, i (share.id)}<tr
                    class="border-b border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors animate-slide-up"
                    class:opacity-50={isExpired(share.expires_at)}
                    style="animation-delay: {i * 30}ms;"
                    ><td
                      class="px-6 py-4 font-mono text-sm text-blue-600 dark:text-blue-400"
                      >{share.file_path}</td
                    ><td class="px-6 py-4"
                      ><span
                        class="px-3 py-1 rounded-full text-xs font-medium"
                        class:bg-blue-100={share.permissions === "read"}
                        class:text-blue-700={share.permissions === "read"}
                        class:dark:bg-blue-900={share.permissions === "read"}
                        class:dark:text-blue-300={share.permissions === "read"}
                        class:bg-yellow-100={share.permissions === "write"}
                        class:text-yellow-700={share.permissions === "write"}
                        class:dark:bg-yellow-900={share.permissions === "write"}
                        class:dark:text-yellow-300={share.permissions ===
                          "write"}>{share.permissions}</span
                      ></td
                    ><td
                      class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400"
                      >{formatDate(share.expires_at)}</td
                    ><td
                      class="px-6 py-4 text-sm text-gray-600 dark:text-gray-400"
                      >{formatDate(share.created_at)}</td
                    ><td class="px-6 py-4"
                      >{#if share.password_hash}<i
                          class="bi bi-lock-fill text-yellow-500 text-lg"
                        ></i>{:else}<i
                          class="bi bi-unlock text-gray-400 text-lg"
                        ></i>{/if}</td
                    ><td class="px-6 py-4"
                      ><div class="flex gap-2">
                        <button
                          onclick={() => copyShareLink(share.id)}
                          class="px-3 py-1.5 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 transition-all flex items-center gap-1"
                          ><i class="bi bi-clipboard"></i> Copy</button
                        ><button
                          onclick={() => openEditModal(share)}
                          class="px-3 py-1.5 rounded-lg text-sm font-medium text-blue-700 dark:text-blue-400 hover:bg-blue-100 dark:hover:bg-blue-900 transition-all flex items-center gap-1"
                          ><i class="bi bi-pencil"></i> Edit</button
                        ><button
                          onclick={() => openDeleteModal(share)}
                          class="px-3 py-1.5 rounded-lg text-sm font-medium text-red-700 dark:text-red-400 hover:bg-red-100 dark:hover:bg-red-900 transition-all flex items-center gap-1"
                          ><i class="bi bi-trash"></i> Delete</button
                        >
                      </div></td
                    ></tr
                  >{/each}</tbody
              >
            </table>
          </div>
          <div
            class="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50 px-6 py-4"
          >
            <div class="flex items-center justify-between text-sm">
              <div class="text-gray-600 dark:text-gray-400">
                <span class="font-semibold">{shares.length}</span> total share{shares.length !==
                1
                  ? "s"
                  : ""}
              </div>
              <div class="flex gap-4 text-gray-600 dark:text-gray-400">
                <div class="flex items-center gap-2">
                  <i class="bi bi-lock-fill text-yellow-500"></i><span
                    >{shares.filter((s) => s.password_hash).length} protected</span
                  >
                </div>
                <div class="flex items-center gap-2">
                  <i class="bi bi-exclamation-triangle text-red-500"></i><span
                    >{shares.filter((s) => isExpired(s.expires_at)).length} expired</span
                  >
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}

{#if showCreateModal}
  <Modal
    visible={showCreateModal}
    title="Create Share"
    size="md"
    onclose={() => (showCreateModal = false)}
    >{#snippet content()}<form
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
            >File Path</label
          ><input
            id="file_path"
            type="text"
            bind:value={newShare.file_path}
            placeholder="/path/to/file.txt"
            class="w-full px-4 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            required
          />
        </div>
        <div>
          <label
            for="permissions"
            class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
            >Permissions</label
          ><select
            id="permissions"
            bind:value={newShare.permissions}
            class="w-full px-4 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            ><option value="read">Read Only</option><option value="write"
              >Read & Write</option
            ></select
          >
        </div>
        <div>
          <label
            for="expires_at"
            class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
            >Expiration Date (Optional)</label
          ><input
            id="expires_at"
            type="datetime-local"
            bind:value={newShare.expires_at}
            class="w-full px-4 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
          />
        </div>
        <div>
          <label
            for="password"
            class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
            >Password (Optional)</label
          ><input
            id="password"
            type="password"
            bind:value={newShare.password}
            placeholder="Leave empty for no password"
            class="w-full px-4 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
          />
        </div>
        <div
          class="flex justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700"
        >
          <button
            type="button"
            onclick={() => (showCreateModal = false)}
            class="px-5 py-2.5 rounded-lg font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-all"
            >Cancel</button
          ><button
            type="submit"
            class="px-5 py-2.5 rounded-lg font-medium bg-gradient-to-r from-blue-500 to-purple-600 text-white hover:from-blue-600 hover:to-purple-700 shadow-lg hover:shadow-xl transition-all flex items-center gap-2"
            ><i class="bi bi-plus-circle"></i> Create Share</button
          >
        </div>
      </form>{/snippet}</Modal
  >
{/if}

{#if showEditModal}
  <Modal
    visible={showEditModal}
    title="Edit Share"
    size="md"
    onclose={() => (showEditModal = false)}
    >{#snippet content()}{#if selectedShare}<form
          onsubmit={(e) => {
            e.preventDefault();
            handleUpdateShare();
          }}
          class="space-y-5"
        >
          <div>
            <label
              class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >File Path</label
            ><input
              type="text"
              value={selectedShare.file_path}
              disabled
              class="w-full px-4 py-2.5 rounded-lg bg-gray-100 dark:bg-gray-900 border border-gray-300 dark:border-gray-600 text-gray-500 dark:text-gray-400"
            />
          </div>
          <div>
            <label
              for="edit_permissions"
              class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >Permissions</label
            ><select
              id="edit_permissions"
              bind:value={selectedShare.permissions}
              class="w-full px-4 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
              ><option value="read">Read Only</option><option value="write"
                >Read & Write</option
              ></select
            >
          </div>
          <div>
            <label
              for="edit_expires_at"
              class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >Expiration Date (Optional)</label
            ><input
              id="edit_expires_at"
              type="datetime-local"
              bind:value={selectedShare.expires_at}
              class="w-full px-4 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            />
          </div>
          <div>
            <label
              for="edit_password"
              class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >Password (Leave empty to keep current)</label
            ><input
              id="edit_password"
              type="password"
              bind:value={selectedShare.password}
              placeholder="Enter new password or leave empty"
              class="w-full px-4 py-2.5 rounded-lg bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            />
          </div>
          <div
            class="flex justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700"
          >
            <button
              type="button"
              onclick={() => {
                showEditModal = false;
                selectedShare = null;
              }}
              class="px-5 py-2.5 rounded-lg font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-all"
              >Cancel</button
            ><button
              type="submit"
              class="px-5 py-2.5 rounded-lg font-medium bg-gradient-to-r from-blue-500 to-purple-600 text-white hover:from-blue-600 hover:to-purple-700 shadow-lg hover:shadow-xl transition-all flex items-center gap-2"
              ><i class="bi bi-save"></i> Save Changes</button
            >
          </div>
        </form>{/if}{/snippet}</Modal
  >
{/if}

{#if showDeleteModal}
  <Modal
    visible={showDeleteModal}
    title="Delete Share"
    size="sm"
    onclose={() => (showDeleteModal = false)}
    >{#snippet content()}{#if selectedShare}<div class="space-y-4">
          <div class="glass-card-light border-l-4 border-red-500 p-4">
            <div class="flex items-start gap-3">
              <i class="bi bi-exclamation-triangle-fill text-red-500 text-2xl"
              ></i>
              <div class="flex-1">
                <p class="font-semibold text-gray-900 dark:text-gray-100 mb-2">
                  Are you sure you want to delete this share?
                </p>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  File: <span
                    class="font-mono font-semibold text-blue-600 dark:text-blue-400"
                    >{selectedShare.file_path}</span
                  >
                </p>
                <p class="text-sm text-red-600 dark:text-red-400 mt-2">
                  This action cannot be undone.
                </p>
              </div>
            </div>
          </div>
          <div class="flex justify-end gap-3 pt-2">
            <button
              type="button"
              onclick={() => {
                showDeleteModal = false;
                selectedShare = null;
              }}
              class="px-5 py-2.5 rounded-lg font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-all"
              >Cancel</button
            ><button
              onclick={handleDeleteShare}
              class="px-5 py-2.5 rounded-lg font-medium bg-gradient-to-r from-red-500 to-red-600 text-white hover:from-red-600 hover:to-red-700 shadow-lg hover:shadow-xl transition-all flex items-center gap-2"
              ><i class="bi bi-trash-fill"></i> Delete Share</button
            >
          </div>
        </div>{/if}{/snippet}</Modal
  >
{/if}


