<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { success, error } from "../../stores/toast.js";

  export let file = null;
  export let isOpen = false;
  export let onClose = () => {};

  let versions = [];
  let loading = false;
  let selectedVersions = { from: null, to: null };
  let diffContent = null;
  let showDiff = false;
  let restoreComment = "";
  let showRestoreModal = false;
  let versionToRestore = null;
  let lastFileId = null;

  // Only reload when file ID changes or modal opens
  $: if (isOpen && file?.id && file.id !== lastFileId) {
    lastFileId = file.id;
    loadVersions();
  }

  // Reset when modal closes
  $: if (!isOpen) {
    lastFileId = null;
    versions = [];
    selectedVersions = { from: null, to: null };
    diffContent = null;
    showDiff = false;
  }

  async function loadVersions() {
    if (!file?.id) return;

    loading = true;
    try {
      const response = await api.versions.list(file.id);
      versions = response.data || [];
    } catch (err) {
      error("Failed to load version history");
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function loadDiff(fromVersion, toVersion) {
    if (!fromVersion || !toVersion) return;

    try {
      const response = await api.versions.getDiff(fromVersion.id, toVersion.id);
      diffContent = response.data;
      showDiff = true;
    } catch (err) {
      error("Failed to load diff");
      console.error(err);
    }
  }

  async function restoreVersion(version) {
    if (!restoreComment.trim()) {
      error("Please provide a restore comment");
      return;
    }

    try {
      await api.versions.restore(version.id, { comment: restoreComment });
      success("File restored successfully");
      showRestoreModal = false;
      restoreComment = "";
      await loadVersions();
      onClose();
    } catch (err) {
      error("Failed to restore version");
      console.error(err);
    }
  }

  async function deleteVersion(version) {
    if (version.is_current) {
      error("Cannot delete current version");
      return;
    }

    if (!confirm("Are you sure you want to delete this version?")) {
      return;
    }

    try {
      await api.versions.delete(version.id);
      success("Version deleted successfully");
      await loadVersions();
    } catch (err) {
      error("Failed to delete version");
      console.error(err);
    }
  }

  async function downloadVersion(version) {
    try {
      const response = await api.versions.download(version.id);
      // Create download link
      const url = window.URL.createObjectURL(new Blob([response.data]));
      const a = document.createElement("a");
      a.href = url;
      a.download = `${file.name}_v${version.version_number}`;
      a.click();
      window.URL.revokeObjectURL(url);
    } catch (err) {
      error("Failed to download version");
      console.error(err);
    }
  }

  function formatDate(dateString) {
    return new Date(dateString).toLocaleString();
  }

  function formatSize(bytes) {
    const sizes = ["B", "KB", "MB", "GB"];
    if (bytes === 0) return "0 B";
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round((bytes / Math.pow(1024, i)) * 100) / 100 + " " + sizes[i];
  }

  function openRestoreModal(version) {
    versionToRestore = version;
    restoreComment = "";
    showRestoreModal = true;
  }

  function selectVersionForDiff(version, position) {
    selectedVersions[position] = version;
    if (selectedVersions.from && selectedVersions.to) {
      loadDiff(selectedVersions.from, selectedVersions.to);
    }
  }
</script>

<!-- Version History Modal Component -->
<!-- Shows file version history with diff view and restore capabilities -->

{#if isOpen}
  <div class="modal modal-open">
    <div class="modal-box w-11/12 max-w-6xl max-h-[90vh] flex flex-col">
      <div class="flex justify-between items-center mb-4">
        <h3 class="font-bold text-lg">Version History - {file?.name || ""}</h3>
        <button class="btn btn-sm btn-circle btn-ghost" on:click={onClose}
          >✕</button
        >
      </div>

      {#if loading}
        <div class="flex justify-center items-center py-8">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
      {:else}
        <div class="flex gap-4 flex-1 min-h-0">
          <!-- Version List -->
          <div class="w-1/2 flex flex-col">
            <div class="flex justify-between items-center mb-2">
              <h4 class="font-semibold">Versions ({versions.length})</h4>
              <div class="text-sm text-base-content/70">
                Select two versions to compare
              </div>
            </div>

            <div class="flex-1 overflow-y-auto border rounded-lg bg-base-100">
              {#each versions as version (version.id)}
                <div
                  class="border-b border-base-200 p-3 hover:bg-base-50 relative"
                >
                  <div class="flex justify-between items-start">
                    <div class="flex-1">
                      <div class="flex items-center gap-2 mb-1">
                        <span class="font-mono text-sm"
                          >v{version.version_number}</span
                        >
                        {#if version.is_current}
                          <span class="badge badge-primary badge-sm"
                            >Current</span
                          >
                        {/if}
                      </div>

                      <div class="text-sm text-base-content/70 mb-1">
                        {formatDate(version.created_at)} by {version.created_by}
                      </div>

                      <div class="text-sm text-base-content/70 mb-2">
                        Size: {formatSize(version.size_bytes)}
                      </div>

                      {#if version.comment}
                        <div class="text-sm bg-base-200 p-2 rounded mb-2">
                          {version.comment}
                        </div>
                      {/if}
                    </div>

                    <div class="dropdown dropdown-end">
                      <div
                        tabindex="0"
                        role="button"
                        class="btn btn-ghost btn-xs"
                      >
                        ⋮
                      </div>
                      <ul
                        class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
                      >
                        <li>
                          <button on:click={() => downloadVersion(version)}
                            >Download</button
                          >
                        </li>
                        {#if !version.is_current}
                          <li>
                            <button on:click={() => openRestoreModal(version)}
                              >Restore</button
                            >
                          </li>
                          <li>
                            <button
                              on:click={() => deleteVersion(version)}
                              class="text-error">Delete</button
                            >
                          </li>
                        {/if}
                      </ul>
                    </div>
                  </div>

                  <div class="flex gap-2 mt-2">
                    <button
                      class="btn btn-xs {selectedVersions.from?.id ===
                      version.id
                        ? 'btn-primary'
                        : 'btn-outline'}"
                      on:click={() => selectVersionForDiff(version, "from")}
                    >
                      From
                    </button>
                    <button
                      class="btn btn-xs {selectedVersions.to?.id === version.id
                        ? 'btn-primary'
                        : 'btn-outline'}"
                      on:click={() => selectVersionForDiff(version, "to")}
                    >
                      To
                    </button>
                  </div>
                </div>
              {/each}

              {#if versions.length === 0}
                <div class="p-8 text-center text-base-content/50">
                  No versions found
                </div>
              {/if}
            </div>
          </div>

          <!-- Diff View -->
          <div class="w-1/2 flex flex-col">
            <h4 class="font-semibold mb-2">Version Comparison</h4>

            {#if showDiff && diffContent}
              <div
                class="flex-1 overflow-y-auto border rounded-lg bg-base-100 p-4"
              >
                <div class="mb-4">
                  <div class="text-sm text-base-content/70">
                    Comparing v{selectedVersions.from?.version_number} → v{selectedVersions
                      .to?.version_number}
                  </div>
                  <div class="text-sm text-success">
                    +{diffContent.added_lines} additions
                  </div>
                  <div class="text-sm text-error">
                    -{diffContent.removed_lines} deletions
                  </div>
                </div>

                {#if diffContent.diff_content}
                  <pre
                    class="text-xs bg-base-200 p-3 rounded overflow-x-auto whitespace-pre-wrap">{diffContent.diff_content}</pre>
                {:else}
                  <div class="text-center text-base-content/50 py-8">
                    {#if diffContent.diff_type === "binary"}
                      Binary files cannot be compared
                    {:else}
                      No differences found
                    {/if}
                  </div>
                {/if}
              </div>
            {:else}
              <div
                class="flex-1 border rounded-lg bg-base-100 flex items-center justify-center"
              >
                <div class="text-center text-base-content/50">
                  <div class="text-4xl mb-2">📊</div>
                  <div>Select two versions to see differences</div>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <div class="modal-action">
        <button class="btn" on:click={onClose}>Close</button>
      </div>
    </div>
  </div>
{/if}

<!-- Restore Version Modal -->
{#if showRestoreModal && versionToRestore}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">Restore Version</h3>

      <div class="mb-4">
        <p>
          Are you sure you want to restore to version {versionToRestore.version_number}?
        </p>
        <p class="text-sm text-base-content/70 mt-1">
          This will create a new version with the content from v{versionToRestore.version_number}.
        </p>
      </div>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">Restore Comment *</span>
        </label>
        <textarea
          class="textarea textarea-bordered"
          placeholder="Reason for restoring this version..."
          bind:value={restoreComment}
          rows="3"
        ></textarea>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-primary"
          on:click={() => restoreVersion(versionToRestore)}
        >
          Restore Version
        </button>
        <button class="btn" on:click={() => (showRestoreModal = false)}
          >Cancel</button
        >
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-box {
    @apply bg-base-100;
  }

  .dropdown-content {
    @apply shadow-lg border border-base-200;
  }
</style>
