<script>
  import { onMount } from "svelte";
  import api, { versions } from "../../lib/api.js";
  import { success, error } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    file = $bindable(null),
    isOpen = $bindable(false),
    onClose = () => {},
  } = $props();

  let timeline = $state(null);
  let loading = $state(false);
  let activeTab = $state("timeline"); // timeline, stats, diff
  let selectedVersions = $state({ from: null, to: null });
  let diffContent = $state(null);
  let loadingDiff = $state(false);
  let tagTemplates = $state([]);
  let showTagModal = $state(false);
  let tagVersion = $state(null);
  let newTag = $state({ name: "", color: "#3b82f6", description: "" });
  let showRestoreModal = $state(false);
  let restoreVersion = $state(null);
  let restoreComment = $state("");
  let lastFilePath = $state(null);

  // Load when modal opens with new file
  $effect(() => {
    const currentPath = file?.path || file?.file_path || file?.id;
    if (isOpen && currentPath && currentPath !== lastFilePath) {
      lastFilePath = currentPath;
      loadTimeline();
      loadTagTemplates();
    }
  });

  // Reset when modal closes
  $effect(() => {
    if (!isOpen) {
      lastFilePath = null;
      timeline = null;
      selectedVersions = { from: null, to: null };
      diffContent = null;
      activeTab = "timeline";
    }
  });

  async function loadTimeline() {
    const fileId = file?.id || file?.path || file?.file_path;
    if (!fileId) return;

    loading = true;
    try {
      const response = await versions.getTimeline(fileId);
      timeline = response.data;
    } catch (err) {
      error(tr("versions.failedToLoad"));
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function loadTagTemplates() {
    try {
      const response = await versions.getTagTemplates();
      tagTemplates = response.data || [];
    } catch (err) {
      console.error("Failed to load tag templates:", err);
    }
  }

  async function loadDiff() {
    if (!selectedVersions.from || !selectedVersions.to) return;

    const fileId = file?.id || file?.path || file?.file_path;
    loadingDiff = true;
    try {
      const response = await versions.getDiff(
        fileId,
        selectedVersions.from.version.id,
        selectedVersions.to.version.id
      );
      diffContent = response.data;
      activeTab = "diff";
    } catch (err) {
      error(tr("versions.failedToLoadDiff"));
      console.error(err);
    } finally {
      loadingDiff = false;
    }
  }

  async function handleRestore() {
    if (!restoreVersion || !restoreComment.trim()) {
      error(tr("versions.restoreCommentRequired"));
      return;
    }

    const fileId = file?.id || file?.path || file?.file_path;
    try {
      await versions.restore(fileId, restoreVersion.version.id, {
        comment: restoreComment,
      });
      success(tr("versions.restored"));
      showRestoreModal = false;
      restoreComment = "";
      restoreVersion = null;
      await loadTimeline();
    } catch (err) {
      error(tr("versions.restoreFailed"));
      console.error(err);
    }
  }

  async function handleDeleteVersion(version) {
    if (!confirm(tr("versions.confirmDelete"))) return;

    const fileId = file?.id || file?.path || file?.file_path;
    try {
      await versions.delete(fileId, version.version.id);
      success(tr("versions.deleted"));
      await loadTimeline();
    } catch (err) {
      error(tr("versions.deleteFailed"));
      console.error(err);
    }
  }

  async function handleTogglePin(version) {
    const fileId = file?.id || file?.path || file?.file_path;
    try {
      await versions.update(fileId, version.version.id, {
        is_pinned: !version.version.is_pinned,
      });
      await loadTimeline();
    } catch (err) {
      error(tr("versions.updateFailed"));
    }
  }

  async function handleToggleStar(version) {
    const fileId = file?.id || file?.path || file?.file_path;
    try {
      await versions.update(fileId, version.version.id, {
        is_starred: !version.version.is_starred,
      });
      await loadTimeline();
    } catch (err) {
      error(tr("versions.updateFailed"));
    }
  }

  async function handleAddTag() {
    if (!newTag.name.trim() || !tagVersion) return;

    const fileId = file?.id || file?.path || file?.file_path;
    try {
      await versions.addTag(fileId, tagVersion.version.id, {
        tag_name: newTag.name,
        tag_color: newTag.color,
        description: newTag.description || null,
      });
      success(tr("versions.tagAdded"));
      showTagModal = false;
      newTag = { name: "", color: "#3b82f6", description: "" };
      tagVersion = null;
      await loadTimeline();
    } catch (err) {
      if (err.message?.includes("409")) {
        error(tr("versions.tagExists"));
      } else {
        error(tr("versions.tagFailed"));
      }
    }
  }

  async function handleRemoveTag(version, tag) {
    const fileId = file?.id || file?.path || file?.file_path;
    try {
      await versions.removeTag(fileId, version.version.id, tag.id);
      await loadTimeline();
    } catch (err) {
      error(tr("versions.tagRemoveFailed"));
    }
  }

  function applyTagTemplate(template) {
    newTag.name = template.name;
    newTag.color = template.color;
    newTag.description = template.description || "";
  }

  function selectVersionForDiff(version, position) {
    selectedVersions[position] = version;
    if (selectedVersions.from && selectedVersions.to) {
      loadDiff();
    }
  }

  function formatDate(dateString) {
    return new Date(dateString).toLocaleString();
  }

  function formatSize(bytes) {
    if (!bytes || bytes === 0) return "0 B";
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return (bytes / Math.pow(1024, i)).toFixed(2) + " " + sizes[i];
  }

  function formatPercentage(value) {
    return value?.toFixed(1) + "%" || "0%";
  }

  function getRelativeTime(dateString) {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now - date;
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return tr("versions.today");
    if (diffDays === 1) return tr("versions.yesterday");
    if (diffDays < 7) return `${diffDays} ${tr("versions.daysAgo")}`;
    if (diffDays < 30)
      return `${Math.floor(diffDays / 7)} ${tr("versions.weeksAgo")}`;
    return `${Math.floor(diffDays / 30)} ${tr("versions.monthsAgo")}`;
  }
</script>

{#if isOpen}
  <!-- Modal Overlay -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <!-- Modal Content -->
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-2xl w-11/12 max-w-5xl max-h-[85vh] flex flex-col border border-gray-200 dark:border-gray-700"
    >
      <!-- Header -->
      <div
        class="flex justify-between items-center px-6 py-4 border-b border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
            <i
              class="bi bi-clock-history text-xl text-blue-600 dark:text-blue-400"
            ></i>
          </div>
          <div>
            <h3 class="font-bold text-lg text-gray-900 dark:text-white">
              {tr("versions.title")}
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {file?.name || file?.filename || ""}
            </p>
          </div>
        </div>
        <button
          class="w-8 h-8 rounded-full flex items-center justify-center hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400 transition-colors"
          onclick={onClose}
        >
          <i class="bi bi-x-lg"></i>
        </button>
      </div>

      <!-- Tabs -->
      <div class="flex border-b border-gray-200 dark:border-gray-700 px-6">
        <button
          class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {activeTab ===
          'timeline'
            ? 'border-blue-500 text-blue-600 dark:text-blue-400'
            : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
          onclick={() => (activeTab = "timeline")}
        >
          <i class="bi bi-clock-history mr-2"></i>{tr("versions.timeline")}
        </button>
        <button
          class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {activeTab ===
          'stats'
            ? 'border-blue-500 text-blue-600 dark:text-blue-400'
            : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
          onclick={() => (activeTab = "stats")}
        >
          <i class="bi bi-bar-chart mr-2"></i>{tr("versions.storage")}
        </button>
        <button
          class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {activeTab ===
          'diff'
            ? 'border-blue-500 text-blue-600 dark:text-blue-400'
            : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
          onclick={() => (activeTab = "diff")}
          disabled={!selectedVersions.from || !selectedVersions.to}
        >
          <i class="bi bi-file-diff mr-2"></i>{tr("versions.diff")}
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-hidden p-6">
        {#if loading}
          <div class="flex justify-center items-center h-full">
            <div
              class="w-12 h-12 border-4 border-blue-200 dark:border-blue-900 border-t-blue-600 dark:border-t-blue-400 rounded-full animate-spin"
            ></div>
          </div>
        {:else if activeTab === "timeline"}
          <!-- Timeline View -->
          <div class="flex gap-6 h-full">
            <!-- Version Timeline -->
            <div class="flex-1 overflow-y-auto pr-4">
              <div class="relative">
                <!-- Timeline Line -->
                <div
                  class="absolute left-5 top-0 bottom-0 w-0.5 bg-gray-200 dark:bg-gray-700"
                ></div>

                {#each timeline?.versions || [] as version, index}
                  <div class="relative flex gap-4 mb-6 last:mb-0">
                    <!-- Timeline Dot -->
                    <div class="relative z-10 flex-shrink-0">
                      <div
                        class="w-10 h-10 rounded-full flex items-center justify-center
                        {index === 0
                          ? 'bg-blue-500 text-white'
                          : 'bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600'}"
                      >
                        <span class="text-sm font-bold"
                          >v{version.version.version_number}</span
                        >
                      </div>
                    </div>

                    <!-- Version Card -->
                    <div
                      class="flex-1 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow"
                    >
                      <div class="flex justify-between items-start mb-2">
                        <div class="flex items-center gap-2">
                          <span
                            class="font-semibold text-gray-900 dark:text-white"
                          >
                            {tr("versions.version")}
                            {version.version.version_number}
                          </span>
                          {#if index === 0}
                            <span
                              class="px-2 py-0.5 text-xs font-medium bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 rounded-full"
                            >
                              {tr("versions.current")}
                            </span>
                          {/if}
                          {#if version.version.is_pinned}
                            <i
                              class="bi bi-pin-fill text-amber-500"
                              title="Pinned"
                            ></i>
                          {/if}
                          {#if version.version.is_starred}
                            <i
                              class="bi bi-star-fill text-yellow-500"
                              title="Starred"
                            ></i>
                          {/if}
                        </div>

                        <!-- Actions -->
                        <div class="flex items-center gap-1">
                          <button
                            class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400"
                            onclick={() => handleTogglePin(version)}
                            title={version.version.is_pinned
                              ? tr("versions.unpin")
                              : tr("versions.pin")}
                          >
                            <i
                              class="bi {version.version.is_pinned
                                ? 'bi-pin-fill'
                                : 'bi-pin'}"
                            ></i>
                          </button>
                          <button
                            class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400"
                            onclick={() => handleToggleStar(version)}
                            title={version.version.is_starred
                              ? tr("versions.unstar")
                              : tr("versions.star")}
                          >
                            <i
                              class="bi {version.version.is_starred
                                ? 'bi-star-fill text-yellow-500'
                                : 'bi-star'}"
                            ></i>
                          </button>
                          <button
                            class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400"
                            onclick={() => {
                              tagVersion = version;
                              showTagModal = true;
                            }}
                            title={tr("versions.addTag")}
                          >
                            <i class="bi bi-tag"></i>
                          </button>
                          {#if index !== 0}
                            <button
                              class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400"
                              onclick={() => {
                                restoreVersion = version;
                                showRestoreModal = true;
                              }}
                              title={tr("versions.restore")}
                            >
                              <i class="bi bi-arrow-counterclockwise"></i>
                            </button>
                            <button
                              class="p-1.5 rounded hover:bg-red-50 dark:hover:bg-red-900/20 text-red-500"
                              onclick={() => handleDeleteVersion(version)}
                              title={tr("versions.delete")}
                            >
                              <i class="bi bi-trash"></i>
                            </button>
                          {/if}
                        </div>
                      </div>

                      <!-- Tags -->
                      {#if version.tags?.length > 0}
                        <div class="flex flex-wrap gap-1 mb-2">
                          {#each version.tags as tag}
                            <span
                              class="inline-flex items-center gap-1 px-2 py-0.5 text-xs rounded-full text-white"
                              style="background-color: {tag.tag_color}"
                            >
                              {tag.tag_name}
                              <button
                                class="hover:bg-white/20 rounded-full p-0.5"
                                onclick={() => handleRemoveTag(version, tag)}
                              >
                                <i class="bi bi-x text-xs"></i>
                              </button>
                            </span>
                          {/each}
                        </div>
                      {/if}

                      <!-- Meta Info -->
                      <div
                        class="text-sm text-gray-500 dark:text-gray-400 mb-2"
                      >
                        <span
                          >{getRelativeTime(version.version.created_at)}</span
                        >
                        <span class="mx-1">•</span>
                        <span
                          >{version.created_by_name ||
                            version.version.created_by}</span
                        >
                        <span class="mx-1">•</span>
                        <span>{formatSize(version.version.original_size)}</span>
                        {#if version.version.is_compressed}
                          <span class="text-green-500 ml-1">
                            ({formatSize(version.version.compressed_size)}
                            {tr("versions.compressed")})
                          </span>
                        {/if}
                      </div>

                      <!-- Comment -->
                      {#if version.version.comment}
                        <p
                          class="text-sm text-gray-600 dark:text-gray-300 bg-gray-50 dark:bg-gray-700/50 rounded p-2"
                        >
                          {version.version.comment}
                        </p>
                      {/if}

                      <!-- Diff Selector -->
                      <div class="flex gap-2 mt-3">
                        <button
                          class="px-3 py-1 text-xs rounded-lg border transition-colors
                            {selectedVersions.from?.version.id ===
                          version.version.id
                            ? 'bg-blue-600 text-white border-blue-600'
                            : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-600'}"
                          onclick={() => selectVersionForDiff(version, "from")}
                        >
                          {tr("versions.compareFrom")}
                        </button>
                        <button
                          class="px-3 py-1 text-xs rounded-lg border transition-colors
                            {selectedVersions.to?.version.id ===
                          version.version.id
                            ? 'bg-blue-600 text-white border-blue-600'
                            : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-600'}"
                          onclick={() => selectVersionForDiff(version, "to")}
                        >
                          {tr("versions.compareTo")}
                        </button>
                      </div>
                    </div>
                  </div>
                {/each}

                {#if !timeline?.versions?.length}
                  <div
                    class="text-center py-12 text-gray-400 dark:text-gray-500"
                  >
                    <i class="bi bi-clock-history text-4xl mb-2"></i>
                    <p>{tr("versions.noVersions")}</p>
                  </div>
                {/if}
              </div>
            </div>
          </div>
        {:else if activeTab === "stats"}
          <!-- Storage Stats -->
          <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
            <div
              class="bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/30 dark:to-blue-800/30 rounded-xl p-4"
            >
              <div class="text-3xl font-bold text-blue-600 dark:text-blue-400">
                {timeline?.stats?.version_count || 0}
              </div>
              <div class="text-sm text-blue-700 dark:text-blue-300">
                {tr("versions.totalVersions")}
              </div>
            </div>
            <div
              class="bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/30 dark:to-purple-800/30 rounded-xl p-4"
            >
              <div
                class="text-3xl font-bold text-purple-600 dark:text-purple-400"
              >
                {formatSize(timeline?.stats?.total_original_size || 0)}
              </div>
              <div class="text-sm text-purple-700 dark:text-purple-300">
                {tr("versions.originalSize")}
              </div>
            </div>
            <div
              class="bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/30 dark:to-green-800/30 rounded-xl p-4"
            >
              <div
                class="text-3xl font-bold text-green-600 dark:text-green-400"
              >
                {formatSize(timeline?.stats?.storage_saved || 0)}
              </div>
              <div class="text-sm text-green-700 dark:text-green-300">
                {tr("versions.spaceSaved")}
              </div>
            </div>
            <div
              class="bg-gradient-to-br from-amber-50 to-amber-100 dark:from-amber-900/30 dark:to-amber-800/30 rounded-xl p-4"
            >
              <div
                class="text-3xl font-bold text-amber-600 dark:text-amber-400"
              >
                {formatPercentage(timeline?.stats?.compression_ratio)}
              </div>
              <div class="text-sm text-amber-700 dark:text-amber-300">
                {tr("versions.compressionRatio")}
              </div>
            </div>
          </div>

          <!-- Timeline Range -->
          {#if timeline?.stats}
            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
              <h4 class="font-semibold text-gray-900 dark:text-white mb-3">
                {tr("versions.versionHistory")}
              </h4>
              <div class="flex items-center gap-4">
                <div class="flex items-center gap-2">
                  <i class="bi bi-calendar-event text-gray-500"></i>
                  <span class="text-sm text-gray-600 dark:text-gray-300">
                    {tr("versions.firstVersion")}: {formatDate(
                      timeline.stats.first_version_at
                    )}
                  </span>
                </div>
                <div
                  class="flex-1 h-1 bg-gray-200 dark:bg-gray-700 rounded"
                ></div>
                <div class="flex items-center gap-2">
                  <span class="text-sm text-gray-600 dark:text-gray-300">
                    {tr("versions.lastVersion")}: {formatDate(
                      timeline.stats.last_version_at
                    )}
                  </span>
                  <i class="bi bi-calendar-check text-gray-500"></i>
                </div>
              </div>
            </div>
          {/if}
        {:else if activeTab === "diff"}
          <!-- Diff View -->
          {#if loadingDiff}
            <div class="flex justify-center items-center h-full">
              <div
                class="w-12 h-12 border-4 border-blue-200 dark:border-blue-900 border-t-blue-600 dark:border-t-blue-400 rounded-full animate-spin"
              ></div>
            </div>
          {:else if diffContent}
            <div class="h-full flex flex-col">
              <!-- Diff Header -->
              <div class="flex justify-between items-center mb-4">
                <div class="flex items-center gap-4">
                  <span
                    class="px-3 py-1 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 rounded-lg text-sm"
                  >
                    v{selectedVersions.from?.version.version_number}
                  </span>
                  <i class="bi bi-arrow-right text-gray-400"></i>
                  <span
                    class="px-3 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 rounded-lg text-sm"
                  >
                    v{selectedVersions.to?.version.version_number}
                  </span>
                </div>
                <div class="flex gap-4 text-sm">
                  <span class="text-green-600 dark:text-green-400">
                    <i class="bi bi-plus-lg mr-1"></i>+{diffContent.added_lines}
                    {tr("versions.additions")}
                  </span>
                  <span class="text-red-600 dark:text-red-400">
                    <i class="bi bi-dash-lg mr-1"
                    ></i>-{diffContent.removed_lines}
                    {tr("versions.deletions")}
                  </span>
                </div>
              </div>

              <!-- Diff Content -->
              <div
                class="flex-1 overflow-auto bg-gray-50 dark:bg-gray-800 rounded-lg p-4 font-mono text-sm"
              >
                {#if diffContent.diff_type === "binary"}
                  <div class="text-center text-gray-500 py-8">
                    <i class="bi bi-file-binary text-4xl mb-2"></i>
                    <p>{tr("versions.binaryNoDiff")}</p>
                  </div>
                {:else if diffContent.diff_content}
                  <pre
                    class="whitespace-pre-wrap">{#each diffContent.diff_content.split("\n") as line}<span
                        class={line.startsWith("+")
                          ? "text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20"
                          : line.startsWith("-")
                            ? "text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20"
                            : "text-gray-600 dark:text-gray-300"}>{line}</span
                      >
                    {/each}</pre>
                {:else}
                  <div class="text-center text-gray-500 py-8">
                    <i class="bi bi-check-circle text-4xl text-green-500 mb-2"
                    ></i>
                    <p>{tr("versions.noDifferences")}</p>
                  </div>
                {/if}
              </div>
            </div>
          {:else}
            <div
              class="flex flex-col items-center justify-center h-full text-gray-400 dark:text-gray-500"
            >
              <i class="bi bi-file-diff text-6xl mb-4"></i>
              <p class="text-lg">{tr("versions.selectTwoVersions")}</p>
              <p class="text-sm mt-2">{tr("versions.selectHint")}</p>
            </div>
          {/if}
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="flex justify-end gap-2 px-6 py-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          onclick={onClose}
        >
          {tr("common.close")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Add Tag Modal -->
{#if showTagModal && tagVersion}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-xl max-w-md w-full border border-gray-200 dark:border-gray-700"
    >
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h3 class="font-bold text-lg text-gray-900 dark:text-white">
          {tr("versions.addTag")} - v{tagVersion.version.version_number}
        </h3>
      </div>

      <div class="p-6 space-y-4">
        <!-- Tag Templates -->
        {#if tagTemplates.length > 0}
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-2"
            >
              {tr("versions.quickTags")}
            </label>
            <div class="flex flex-wrap gap-2">
              {#each tagTemplates as template}
                <button
                  class="px-3 py-1 text-xs rounded-full text-white transition-opacity hover:opacity-80"
                  style="background-color: {template.color}"
                  onclick={() => applyTagTemplate(template)}
                >
                  {template.name}
                </button>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Tag Name -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
          >
            {tr("versions.tagName")} *
          </label>
          <input
            type="text"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder={tr("versions.tagNamePlaceholder")}
            bind:value={newTag.name}
          />
        </div>

        <!-- Tag Color -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
          >
            {tr("versions.tagColor")}
          </label>
          <div class="flex items-center gap-3">
            <input
              type="color"
              class="w-10 h-10 rounded cursor-pointer"
              bind:value={newTag.color}
            />
            <input
              type="text"
              class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
              bind:value={newTag.color}
            />
          </div>
        </div>

        <!-- Description -->
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
          >
            {tr("versions.tagDescription")}
          </label>
          <textarea
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white resize-none"
            rows="2"
            placeholder={tr("versions.tagDescriptionPlaceholder")}
            bind:value={newTag.description}
          ></textarea>
        </div>
      </div>

      <div
        class="flex justify-end gap-2 px-6 py-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          onclick={() => {
            showTagModal = false;
            tagVersion = null;
          }}
        >
          {tr("common.cancel")}
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
          onclick={handleAddTag}
          disabled={!newTag.name.trim()}
        >
          {tr("versions.addTag")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Restore Confirmation Modal -->
{#if showRestoreModal && restoreVersion}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center bg-black/50 backdrop-blur-sm"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-xl max-w-md w-full border border-gray-200 dark:border-gray-700"
    >
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h3 class="font-bold text-lg text-gray-900 dark:text-white">
          {tr("versions.restoreTitle")}
        </h3>
      </div>

      <div class="p-6 space-y-4">
        <p class="text-gray-700 dark:text-gray-200">
          {tr("versions.restoreConfirm", restoreVersion.version.version_number)}
        </p>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          {tr("versions.restoreHint")}
        </p>

        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
          >
            {tr("versions.restoreComment")} *
          </label>
          <textarea
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white resize-none"
            rows="3"
            placeholder={tr("versions.restoreCommentPlaceholder")}
            bind:value={restoreComment}
          ></textarea>
        </div>
      </div>

      <div
        class="flex justify-end gap-2 px-6 py-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          onclick={() => {
            showRestoreModal = false;
            restoreVersion = null;
            restoreComment = "";
          }}
        >
          {tr("common.cancel")}
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
          onclick={handleRestore}
          disabled={!restoreComment.trim()}
        >
          {tr("versions.restore")}
        </button>
      </div>
    </div>
  </div>
{/if}
