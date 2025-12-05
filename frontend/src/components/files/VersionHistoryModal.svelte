<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { success, error } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    file = $bindable(null),
    isOpen = $bindable(false),
    onClose = () => {},
  } = $props();

  let versions = $state([]);
  let loading = $state(false);
  let selectedVersions = $state({ from: null, to: null });
  let diffContent = $state(null);
  let showDiff = $state(false);
  let restoreComment = $state("");
  let showRestoreModal = $state(false);
  let versionToRestore = $state(null);
  let lastFilePath = $state(null);

  // Only reload when file path changes or modal opens
  $effect(() => {
    const currentPath = file?.path || file?.file_path;
    if (isOpen && currentPath && currentPath !== lastFilePath) {
      lastFilePath = currentPath;
      loadVersions();
    }
  });

  // Reset when modal closes
  $effect(() => {
    if (!isOpen) {
      lastFilePath = null;
      versions = [];
      selectedVersions = { from: null, to: null };
      diffContent = null;
      showDiff = false;
    }
  });

  async function loadVersions() {
    const filePath = file?.path || file?.file_path;
    if (!filePath) return;

    loading = true;
    try {
      // Use file path to load versions
      const encodedPath = encodeURIComponent(filePath);
      const response = await api.files.getVersions(encodedPath);
      versions = response.data || response || [];
      console.log(
        "[VersionHistory] Loaded",
        versions.length,
        "versions for",
        filePath
      );
    } catch (err) {
      error(tr("failedToLoadVersionHistory"));
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
      error(tr("failedToLoadDiff"));
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
  <!-- Modal Overlay -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <!-- Modal Content - Kompaktere Größe -->
    <div
      class="bg-white dark:bg-gray-900 rounded-lg shadow-xl w-11/12 max-w-3xl max-h-[80vh] flex flex-col border border-gray-200 dark:border-gray-700"
    >
      <div class="flex justify-between items-center mb-4 px-6 pt-6">
        <h3 class="font-bold text-lg text-gray-900 dark:text-white">
          Version History - {file?.name || ""}
        </h3>
        <button
          class="w-8 h-8 rounded-full flex items-center justify-center hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400 transition-colors"
          onclick={onClose}
        >
          ✕
        </button>
      </div>

      {#if loading}
        <div class="flex justify-center items-center py-8 px-6">
          <div
            class="w-12 h-12 border-4 border-green-200 dark:border-green-900 border-t-green-600 dark:border-t-green-400 rounded-full animate-spin"
          ></div>
        </div>
      {:else}
        <div class="flex gap-4 flex-1 min-h-0 px-6">
          <!-- Version List -->
          <div class="w-1/2 flex flex-col min-h-0">
            <div class="flex justify-between items-center mb-2">
              <h4 class="font-semibold text-gray-900 dark:text-white">
                Versions ({versions.length})
              </h4>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Select two versions to compare
              </div>
            </div>

            <div
              class="flex-1 overflow-y-auto border rounded-lg bg-white dark:bg-gray-900 max-h-[50vh]"
            >
              {#each versions as version (version.id)}
                <div
                  class="border-b border-gray-200 dark:border-gray-700 p-3 hover:bg-base-50 relative"
                >
                  <div class="flex justify-between items-start">
                    <div class="flex-1">
                      <div class="flex items-center gap-2 mb-1">
                        <span
                          class="font-mono text-sm text-gray-900 dark:text-white"
                          >v{version.version_number}</span
                        >
                        {#if version.is_current}
                          <span
                            class="px-2 py-0.5 text-xs font-medium bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200 rounded-full"
                            >Current</span
                          >
                        {/if}
                      </div>

                      <div
                        class="text-sm text-gray-500 dark:text-gray-400 mb-1"
                      >
                        {formatDate(version.created_at)} by {version.created_by}
                      </div>

                      <div
                        class="text-sm text-gray-500 dark:text-gray-400 mb-2"
                      >
                        Size: {formatSize(version.size_bytes)}
                      </div>

                      {#if version.comment}
                        <div
                          class="text-sm bg-gray-50 dark:bg-gray-800 p-2 rounded mb-2"
                        >
                          {version.comment}
                        </div>
                      {/if}
                    </div>

                    <div class="relative group">
                      <button
                        class="px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded text-gray-500 dark:text-gray-400 text-lg transition-colors"
                      >
                        ⋮
                      </button>
                      <ul
                        class="absolute right-0 top-full mt-1 z-10 hidden group-hover:block bg-white dark:bg-gray-900 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 w-52 py-1"
                      >
                        <li>
                          <button
                            onclick={() => downloadVersion(version)}
                            class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                          >
                            Download
                          </button>
                        </li>
                        {#if !version.is_current}
                          <li>
                            <button
                              onclick={() => openRestoreModal(version)}
                              class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                            >
                              Restore
                            </button>
                          </li>
                          <li>
                            <button
                              onclick={() => deleteVersion(version)}
                              class="w-full text-left px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                            >
                              Delete
                            </button>
                          </li>
                        {/if}
                      </ul>
                    </div>
                  </div>

                  <div class="flex gap-2 mt-2">
                    <button
                      class="px-3 py-1 text-xs rounded-lg border transition-colors {selectedVersions
                        .from?.id === version.id
                        ? 'bg-green-600 dark:bg-green-500 text-white border-green-600 dark:border-green-500'
                        : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
                      onclick={() => selectVersionForDiff(version, "from")}
                    >
                      From
                    </button>
                    <button
                      class="px-3 py-1 text-xs rounded-lg border transition-colors {selectedVersions
                        .to?.id === version.id
                        ? 'bg-green-600 dark:bg-green-500 text-white border-green-600 dark:border-green-500'
                        : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700'}"
                      onclick={() => selectVersionForDiff(version, "to")}
                    >
                      To
                    </button>
                  </div>
                </div>
              {/each}

              {#if versions.length === 0}
                <div class="p-8 text-center text-gray-400 dark:text-gray-500">
                  No versions found
                </div>
              {/if}
            </div>
          </div>

          <!-- Diff View -->
          <div class="w-1/2 flex flex-col min-h-0">
            <h4 class="font-semibold mb-2 text-gray-900 dark:text-white">
              Version Comparison
            </h4>

            {#if showDiff && diffContent}
              <div
                class="flex-1 overflow-y-auto border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-900 p-4 max-h-[50vh]"
              >
                <div class="mb-4">
                  <div class="text-sm text-gray-500 dark:text-gray-400">
                    Comparing v{selectedVersions.from?.version_number} → v{selectedVersions
                      .to?.version_number}
                  </div>
                  <div class="text-sm text-green-600 dark:text-green-400">
                    +{diffContent.added_lines} additions
                  </div>
                  <div class="text-sm text-red-600 dark:text-red-400">
                    -{diffContent.removed_lines} deletions
                  </div>
                </div>

                {#if diffContent.diff_content}
                  <pre
                    class="text-xs bg-gray-50 dark:bg-gray-800 p-3 rounded overflow-x-auto whitespace-pre-wrap text-gray-900 dark:text-white">{diffContent.diff_content}</pre>
                {:else}
                  <div
                    class="text-center text-gray-400 dark:text-gray-500 py-8"
                  >
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
                class="flex-1 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-900 flex items-center justify-center"
              >
                <div class="text-center text-gray-400 dark:text-gray-500">
                  <div class="text-4xl mb-2">📊</div>
                  <div>Select two versions to see differences</div>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <div
        class="flex justify-end gap-2 px-6 pb-6 pt-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          onclick={onClose}
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Restore Version Modal -->
{#if showRestoreModal && versionToRestore}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-lg shadow-xl max-w-md w-full border border-gray-200 dark:border-gray-700"
    >
      <h3
        class="font-bold text-lg mb-4 px-6 pt-6 text-gray-900 dark:text-white"
      >
        Restore Version
      </h3>

      <div class="mb-4 px-6">
        <p class="text-gray-700 dark:text-gray-200">
          Are you sure you want to restore to version {versionToRestore.version_number}?
        </p>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
          This will create a new version with the content from v{versionToRestore.version_number}.
        </p>
      </div>

      <div class="mb-4 px-6">
        <label class="block mb-2">
          <span class="text-sm font-medium text-gray-700 dark:text-gray-200"
            >Restore Comment *</span
          >
          <textarea
            class="mt-1 w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent transition-shadow"
            placeholder="Reason for restoring this version..."
            bind:value={restoreComment}
            rows="3"
          ></textarea>
        </label>
      </div>

      <div
        class="flex justify-end gap-2 px-6 pb-6 pt-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-green-600 dark:bg-green-500 rounded-lg hover:bg-green-700 dark:hover:bg-green-600 transition-colors"
          onclick={() => restoreVersion(versionToRestore)}
        >
          Restore Version
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          onclick={() => (showRestoreModal = false)}
        >
          Cancel
        </button>
      </div>
    </div>
  </div>
{/if}
