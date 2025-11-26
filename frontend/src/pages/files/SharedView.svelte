<script>
  import { onMount } from "svelte";
  import api from "../../lib/api";
  import { success, error as errorToast } from "../../stores/toast";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import ShareModal from "../../components/sharing/ShareModal.svelte";
  import ShareAnalyticsView from "../sharing/ShareAnalyticsView.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let shares = $state([]);
  let loading = $state(true);
  let errorMsg = $state(null);
  let showShareModal = $state(false);
  let showEditModal = $state(false);
  let showDeleteModal = $state(false);
  let showAnalytics = $state(false);
  let analyticsShareId = $state(null);
  let selectedShare = $state(null);

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
      errorMsg = tr("failedToLoadShares");
      errorToast(tr("failedToLoadShares"));
      shares = [];
    } finally {
      loading = false;
    }
  }

  async function handleUpdateShare() {
    if (!selectedShare) return;

    try {
      const updates = {
        permissions: selectedShare.permission,
        expires_at: selectedShare.expires_at || null,
      };

      await api.shares.update(selectedShare.id, updates);
      success(tr("shareUpdatedSuccessfully"));
      showEditModal = false;
      selectedShare = null;
      await loadShares();
    } catch (err) {
      console.error("Failed to update share:", err);
      errorToast(tr("failedToUpdateShare"));
    }
  }

  async function handleDeleteShare() {
    if (!selectedShare) return;

    try {
      await api.shares.delete(selectedShare.id);
      success(tr("shareDeletedSuccessfully"));
      showDeleteModal = false;
      selectedShare = null;
      await loadShares();
    } catch (err) {
      console.error("Failed to delete share:", err);
      errorToast(tr("failedToDeleteShare"));
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

  function openAnalytics(shareId) {
    analyticsShareId = shareId;
    showAnalytics = true;
  }

  function closeAnalytics() {
    showAnalytics = false;
    analyticsShareId = null;
  }

  function formatDate(dateString) {
    if (!dateString) return tr("never");
    return new Date(dateString).toLocaleDateString();
  }

  async function copyShareLink(shareId) {
    const link = `${window.location.origin}/sharing/public/${shareId}`;
    try {
      await navigator.clipboard.writeText(link);
      success(tr("shareLinkCopiedToClipboard"));
    } catch (err) {
      errorToast(tr("failedToCopyLink"));
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
      title={tr("sharedFiles")}
      subtitle={tr("manageFileSharesAndCollaboration")}
      icon="share-fill"
    >
      {#snippet actions()}
        <ModernButton
          variant="gradient"
          icon="plus-circle"
          onclick={() => (showShareModal = true)}
          class="btn-pulse"
        >
          {tr("createShare")}
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
                  {tr("noSharesYet")}
                </h2>
                <p class="text-gray-600 dark:text-gray-400 mb-6">
                  {tr("createFirstShareToCollaborate")}
                </p>
                <ModernButton
                  variant="gradient"
                  icon="plus-circle"
                  onclick={() => (showShareModal = true)}
                >
                  {tr("createShare")}
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
                      >
                        {tr("filePath")}
                      </th>
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                      >
                        {tr("permissions")}
                      </th>
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                      >
                        {tr("expires")}
                      </th>
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                      >
                        {tr("created")}
                      </th>
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                      >
                        {tr("protected")}
                      </th>
                      <th
                        class="px-6 py-4 text-left text-sm font-semibold text-gray-900 dark:text-gray-100"
                      >
                        {tr("actions")}
                      </th>
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
                            class="badge-glass-{share.permission === 'read'
                              ? 'info'
                              : 'warning'}"
                          >
                            {share.permission}
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
                              {tr("copy")}
                            </ModernButton>
                            <ModernButton
                              variant="ghost"
                              size="sm"
                              icon="graph-up"
                              onclick={() => openAnalytics(share.id)}
                            >
                              {tr("analytics")}
                            </ModernButton>
                            <ModernButton
                              variant="secondary"
                              size="sm"
                              icon="pencil"
                              onclick={() => openEditModal(share)}
                            >
                              {tr("edit")}
                            </ModernButton>
                            <ModernButton
                              variant="danger"
                              size="sm"
                              icon="trash"
                              onclick={() => openDeleteModal(share)}
                            >
                              {tr("delete")}
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
                    <span class="font-semibold">{shares.length}</span>
                    {tr("totalShares", shares.length)}
                  </div>
                  <div class="flex gap-4 text-gray-600 dark:text-gray-400">
                    <div class="flex items-center gap-2">
                      <i class="bi bi-lock-fill text-yellow-500"></i>
                      <span
                        >{shares.filter((s) => s.password_hash).length}
                        {tr("protected")}</span
                      >
                    </div>
                    <div class="flex items-center gap-2">
                      <i class="bi bi-exclamation-triangle text-red-500"></i>
                      <span
                        >{shares.filter((s) => isExpired(s.expires_at)).length}
                        {tr("expired")}</span
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

  <!-- Share Modal (unified for create/edit) -->
  <ShareModal
    bind:isOpen={showShareModal}
    file={null}
    onClose={() => {
      showShareModal = false;
      loadShares();
    }}
  />

  <!-- Edit Share Modal -->
  {#if showEditModal && selectedShare}
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
            {tr("editShare")}
          </h2>
          <form
            onsubmit={(e) => {
              e.preventDefault();
              handleUpdateShare();
            }}
            class="space-y-5"
          >
            <div>
              <label
                for="edit_file_path"
                class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >
                {tr("filePath")}
              </label>
              <input
                id="edit_file_path"
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
                {tr("permissions")}
              </label>
              <select
                id="edit_permissions"
                bind:value={selectedShare.permission}
                class="glass-input w-full px-4 py-2.5 rounded-lg"
              >
                <option value="read">{tr("readOnly")}</option>
                <option value="write">{tr("readAndWrite")}</option>
              </select>
            </div>
            <div>
              <label
                for="edit_expires_at"
                class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              >
                {tr("expirationDateOptional")}
              </label>
              <input
                id="edit_expires_at"
                type="datetime-local"
                bind:value={selectedShare.expires_at}
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
                {tr("cancel")}
              </ModernButton>
              <ModernButton type="submit" variant="gradient" icon="save">
                {tr("saveChanges")}
              </ModernButton>
            </div>
          </form>
        {/snippet}
      </ModernCard>
    </div>
  {/if}

  <!-- Delete Confirmation Modal -->
  {#if showDeleteModal && selectedShare}
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
            {tr("deleteShare")}
          </h2>
          <div class="space-y-4">
            <div class="glass-card border-l-4 border-red-500 p-4">
              <div class="flex items-start gap-3">
                <i class="bi bi-exclamation-triangle-fill text-red-500 text-2xl"
                ></i>
                <div class="flex-1">
                  <p
                    class="font-semibold text-gray-900 dark:text-gray-100 mb-2"
                  >
                    {tr("areYouSureDeleteShare")}
                  </p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">
                    {tr("file")}:
                    <span
                      class="font-mono font-semibold text-primary-600 dark:text-primary-400"
                      >{selectedShare.file_path}</span
                    >
                  </p>
                  <p class="text-sm text-red-600 dark:text-red-400 mt-2">
                    {tr("thisActionCannotBeUndone")}
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
                {tr("cancel")}
              </ModernButton>
              <ModernButton
                variant="danger"
                icon="trash-fill"
                onclick={handleDeleteShare}
              >
                {tr("deleteShare")}
              </ModernButton>
            </div>
          </div>
        {/snippet}
      </ModernCard>
    </div>
  {/if}

  <!-- Analytics Modal -->
  {#if showAnalytics && analyticsShareId}
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4 modal-backdrop"
      onclick={closeAnalytics}
    >
      <div
        class="max-w-7xl w-full max-h-[90vh] overflow-y-auto bg-white dark:bg-gray-900 rounded-2xl shadow-2xl modal-content"
        onclick={(e) => e.stopPropagation()}
      >
        <div
          class="sticky top-0 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 p-4 flex items-center justify-between z-10"
        >
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            {tr("shareAnalytics")}
          </h2>
          <button
            onclick={closeAnalytics}
            class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
          >
            <i class="bi bi-x-lg text-2xl text-gray-600 dark:text-gray-400"></i>
          </button>
        </div>
        <ShareAnalyticsView shareId={analyticsShareId} />
      </div>
    </div>
  {/if}
</PageWrapper>
