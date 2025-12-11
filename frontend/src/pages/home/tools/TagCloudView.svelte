<script>
  import { onMount } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import PageWrapper from "../components/PageWrapper.svelte";
  import LoadingState from "../components/ui/LoadingState.svelte";
  import EmptyState from "../components/ui/EmptyState.svelte";
  import Modal from "../components/ui/Modal.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";
  import { tags as tagsApi } from "../lib/api.js";
  import { success, error as errorToast } from "../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let tags = $state([]);
  let error = $state(null);
  let selectedTags = $state(new Set());
  let selectedTagsArray = $state([]);

  // Tag detail modal
  let showTagModal = $state(false);
  let selectedTag = $state(null);
  let tagFiles = $state([]);
  let loadingTagFiles = $state(false);

  // Create tag modal
  let showCreateTagModal = $state(false);
  let newTagName = $state("");
  let newTagColor = $state("#10B981");
  let creatingTag = $state(false);

  // Cloud config
  let minFontSize = $state(16);
  let maxFontSize = $state(42);
  let colorMode = $state("category"); // 'category', 'usage', 'random'

  // Filter & sort
  let sortBy = $state("usage"); // 'usage', 'name', 'recent'
  let minUsage = $state(1);
  let searchQuery = $state("");

  const colorCategories = [
    "#10B981", // emerald (primary)
    "#3B82F6", // blue
    "#F59E0B", // amber
    "#8B5CF6", // violet
    "#EC4899", // pink
    "#06B6D4", // cyan
    "#EF4444", // red
    "#6B7280", // gray
  ];

  onMount(async () => {
    await loadTagCloud();
  });

  async function createTag() {
    if (!newTagName.trim()) {
      errorToast(tr("tagCloud.enterTagName"));
      return;
    }

    creatingTag = true;
    try {
      await tagsApi.create({ name: newTagName.trim(), color: newTagColor });
      success(tr("tagCloud.tagCreated"));
      showCreateTagModal = false;
      newTagName = "";
      newTagColor = "#10B981";
      await loadTagCloud();
    } catch (e) {
      console.error("Failed to create tag:", e);
      errorToast(tr("tagCloud.createError"));
    } finally {
      creatingTag = false;
    }
  }

  async function loadTagCloud() {
    loading = true;
    error = null;
    try {
      const allTags = await tagsApi.list();
      tags = allTags.map((tag, idx) => ({
        ...tag,
        usage: tag.file_count || 0,
        fontSize: calculateFontSize(tag.file_count || 0, allTags),
        color:
          colorMode === "random"
            ? colorCategories[
                Math.floor(Math.random() * colorCategories.length)
              ]
            : tag.color || colorCategories[idx % colorCategories.length],
        hue: Math.random() * 360,
      }));
    } catch (e) {
      console.error("Failed to load tag cloud:", e);
      error = tr("tagCloud.loadError");
      errorToast(error);
    } finally {
      loading = false;
    }
  }

  function calculateFontSize(count, allTags) {
    const maxCount = Math.max(...allTags.map((t) => t.file_count || 0), 1);
    const minCount = 1;
    return (
      minFontSize +
      ((count - minCount) / (maxCount - minCount)) * (maxFontSize - minFontSize)
    );
  }

  function getFilteredAndSortedTags() {
    let result = tags.filter((tag) => {
      const matchesSearch = tag.name
        .toLowerCase()
        .includes(searchQuery.toLowerCase());
      const meetsMinUsage = tag.usage >= minUsage;
      return matchesSearch && meetsMinUsage;
    });

    // Sort
    switch (sortBy) {
      case "name":
        result.sort((a, b) => a.name.localeCompare(b.name));
        break;
      case "recent":
        result.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
        break;
      case "usage":
      default:
        result.sort((a, b) => b.usage - a.usage);
        break;
    }

    return result;
  }

  function toggleTag(tag) {
    if (selectedTags.has(tag.id)) {
      selectedTags.delete(tag.id);
    } else {
      selectedTags.add(tag.id);
    }
    selectedTags = selectedTags;
    updateSelectedArray();
  }

  async function openTagModal(tag, e) {
    e.stopPropagation();
    selectedTag = tag;
    showTagModal = true;
    loadingTagFiles = true;

    try {
      // Load files associated with this tag
      const files = await tagsApi.getFiles(tag.id);
      tagFiles = files || [];
    } catch (err) {
      console.error("Failed to load tag files:", err);
      tagFiles = [];
    } finally {
      loadingTagFiles = false;
    }
  }

  function closeTagModal() {
    showTagModal = false;
    selectedTag = null;
    tagFiles = [];
  }

  function updateSelectedArray() {
    selectedTagsArray = Array.from(selectedTags)
      .map((id) => tags.find((t) => t.id === id))
      .filter(Boolean);
  }

  function clearSelection() {
    selectedTags.clear();
    selectedTags = selectedTags;
    selectedTagsArray = [];
  }

  function deleteTag(tag, e) {
    e.stopPropagation();
    if (!confirm(`${tr("tagCloud.deleteConfirm")} "${tag.name}"?`)) return;

    (async () => {
      try {
        await tagsApi.delete(tag.id);
        tags = tags.filter((t) => t.id !== tag.id);
        selectedTags.delete(tag.id);
        selectedTags = selectedTags;
        updateSelectedArray();
        success(`${tr("tagCloud.deleted")}: ${tag.name}`);
      } catch (e) {
        console.error("Failed to delete tag:", e);
        errorToast(tr("tagCloud.deleteError"));
      }
    })();
  }

  function getTagColor(tag) {
    if (colorMode === "random") {
      return (
        tag.color || colorCategories[tags.indexOf(tag) % colorCategories.length]
      );
    } else if (colorMode === "usage") {
      const maxCount = Math.max(...tags.map((t) => t.usage || 0), 1);
      const ratio = tag.usage / maxCount;
      if (ratio > 0.75) return "#10B981"; // emerald - high
      if (ratio > 0.5) return "#F59E0B"; // amber - medium
      if (ratio > 0.25) return "#3B82F6"; // blue - low
      return "#9CA3AF"; // gray - very low
    }
    return (
      tag.color || colorCategories[tags.indexOf(tag) % colorCategories.length]
    );
  }

  const displayTags = $derived(getFilteredAndSortedTags());
  const totalFiles = $derived(tags.reduce((sum, t) => sum + (t.usage || 0), 0));
</script>

<PageWrapper>
  <div class="container mx-auto px-4 py-8 max-w-6xl">
    <!-- Header -->
    <div class="mb-8 flex flex-wrap items-start justify-between gap-4">
      <div>
        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-2">
          <i class="bi bi-tags text-green-500 mr-3" aria-hidden="true"></i>{tr(
            "tagCloud.title"
          )}
        </h1>
        <p class="text-gray-600 dark:text-gray-400">
          {tr("tagCloud.description")} • {tags.length}
          {tr("tagCloud.tags")} • {totalFiles}
          {tr("tagCloud.files")}
        </p>
      </div>
      <button
        type="button"
        class="px-5 py-2.5 bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 text-white rounded-xl shadow-lg shadow-green-500/25 font-medium flex items-center gap-2 transition-all"
        onclick={() => (showCreateTagModal = true)}
      >
        <i class="bi bi-plus-lg" aria-hidden="true"></i>
        {tr("tagCloud.createTag")}
      </button>
    </div>

    <!-- Create Tag Modal -->
    <Modal
      visible={showCreateTagModal}
      title={tr("tagCloud.createTag")}
      icon="tag"
      size="sm"
      variant="primary"
      onclose={() => (showCreateTagModal = false)}
    >
      {#snippet children()}
        <div class="space-y-4">
          <div>
            <label
              for="newTagName"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              {tr("tagCloud.tagName")}
            </label>
            <input
              id="newTagName"
              type="text"
              bind:value={newTagName}
              placeholder={tr("tagCloud.tagNamePlaceholder")}
              class="w-full px-4 py-2 rounded-xl border-2 border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:border-green-500 focus:ring-2 focus:ring-green-500/20 outline-none"
            />
          </div>
          <div>
            <label
              for="newTagColor"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              {tr("tagCloud.tagColor")}
            </label>
            <div class="flex items-center gap-3">
              <input
                id="newTagColor"
                type="color"
                bind:value={newTagColor}
                class="w-12 h-12 rounded-lg border-2 border-gray-200 dark:border-gray-700 cursor-pointer"
              />
              <div class="flex flex-wrap gap-2">
                {#each colorCategories as color}
                  <button
                    type="button"
                    class="w-8 h-8 rounded-full transition-transform hover:scale-110 {newTagColor ===
                    color
                      ? 'ring-2 ring-offset-2 ring-green-500'
                      : ''}"
                    style="background-color: {color};"
                    onclick={() => (newTagColor = color)}
                    aria-label="Select color {color}"
                  ></button>
                {/each}
              </div>
            </div>
          </div>
          <div class="flex items-center gap-3 pt-2">
            <span
              class="px-4 py-2 rounded-full text-white font-medium"
              style="background-color: {newTagColor};"
            >
              {newTagName || tr("tagCloud.preview")}
            </span>
          </div>
        </div>
      {/snippet}
      {#snippet actions()}
        <button
          type="button"
          class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 transition-colors"
          onclick={() => (showCreateTagModal = false)}
        >
          {tr("cancel")}
        </button>
        <button
          type="button"
          class="px-5 py-2 bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 text-white rounded-xl shadow-lg shadow-green-500/25 font-medium flex items-center gap-2 transition-all disabled:opacity-50"
          onclick={createTag}
          disabled={creatingTag || !newTagName.trim()}
        >
          {#if creatingTag}
            <i class="bi bi-arrow-repeat animate-spin" aria-hidden="true"></i>
          {:else}
            <i class="bi bi-plus-lg" aria-hidden="true"></i>
          {/if}
          {tr("tagCloud.createTag")}
        </button>
      {/snippet}
    </Modal>

    {#if loading}
      <LoadingState />
    {:else if tags.length === 0}
      <EmptyState
        icon="bi-tags"
        title={tr("tagCloud.noTags")}
        description={tr("tagCloud.noTagsDescription")}
      />
    {:else}
      <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
        <!-- Cloud Container -->
        <div class="lg:col-span-3">
          <div
            class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8 border border-gray-200 dark:border-gray-700"
          >
            <!-- Controls -->
            <div
              class="mb-6 flex flex-wrap gap-3 items-center pb-6 border-b border-gray-200 dark:border-gray-700"
            >
              <div class="flex items-center gap-2">
                <label
                  for="color-mode-select"
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                  >{tr("tagCloud.colorMode")}:</label
                >
                <select
                  id="color-mode-select"
                  bind:value={colorMode}
                  class="px-3 py-1 rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white text-sm border border-gray-300 dark:border-gray-600"
                >
                  <option value="category">{tr("tagCloud.byCategory")}</option>
                  <option value="usage">{tr("tagCloud.byUsage")}</option>
                  <option value="random">{tr("tagCloud.random")}</option>
                </select>
              </div>

              <div class="flex items-center gap-2">
                <label
                  for="sort-by-select"
                  class="text-sm font-medium text-gray-700 dark:text-gray-300"
                  >{tr("tagCloud.sortBy")}:</label
                >
                <select
                  id="sort-by-select"
                  bind:value={sortBy}
                  class="px-3 py-1 rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white text-sm border border-gray-300 dark:border-gray-600"
                >
                  <option value="usage">{tr("tagCloud.byUsage")}</option>
                  <option value="name">{tr("tagCloud.byName")}</option>
                  <option value="recent">{tr("tagCloud.byRecent")}</option>
                </select>
              </div>
            </div>

            <!-- Tag Cloud -->
            <div
              class="flex flex-wrap gap-3 justify-center min-h-96 items-center"
            >
              {#each displayTags as tag (tag.id)}
                <button
                  class="px-4 py-2 rounded-full font-semibold transition-all duration-200 hover:scale-110 relative group cursor-pointer"
                  style="
                    font-size: {tag.fontSize}px;
                    background-color: {getTagColor(tag)};
                    color: white;
                    opacity: {selectedTags.has(tag.id) ? 1 : 0.7};
                    ring: {selectedTags.has(tag.id) ? '2px' : '0'};
                  "
                  class:ring-4={selectedTags.has(tag.id)}
                  class:ring-white={selectedTags.has(tag.id)}
                  onclick={() => toggleTag(tag)}
                  title="{tag.name} ({tag.usage} {tr('tagCloud.files')})"
                >
                  {tag.name}
                  <span class="text-xs opacity-75 ml-1">({tag.usage})</span>

                  <!-- Info Button on Hover -->
                  <div
                    class="absolute -top-2 -left-2 bg-green-500 rounded-full w-6 h-6 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer hover:bg-green-600"
                    onclick={(e) => openTagModal(tag, e)}
                    role="button"
                    tabindex="0"
                    title={tr("tagCloud.viewDetails")}
                    onkeydown={(e) => e.key === "Enter" && openTagModal(tag, e)}
                  >
                    <i class="bi bi-info text-white text-sm" aria-hidden="true"
                    ></i>
                  </div>

                  <!-- Delete Button on Hover (Admin) -->
                  <div
                    class="absolute -top-2 -right-2 bg-red-500 rounded-full w-6 h-6 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer hover:bg-red-600"
                    onclick={(e) => deleteTag(tag, e)}
                    role="button"
                    tabindex="0"
                    title={tr("tagCloud.deleteTag")}
                    onkeydown={(e) => e.key === "Enter" && deleteTag(tag, e)}
                  >
                    <i class="bi bi-x text-white text-sm" aria-hidden="true"
                    ></i>
                  </div>
                </button>
              {/each}
            </div>

            {#if displayTags.length === 0 && tags.length > 0}
              <div class="text-center py-12">
                <p class="text-gray-500 dark:text-gray-400">
                  {tr("tagCloud.noResults")}
                </p>
              </div>
            {/if}
          </div>
        </div>

        <!-- Sidebar: Filters & Selection -->
        <div class="space-y-4">
          <!-- Filter Card -->
          <div
            class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 border border-gray-200 dark:border-gray-700"
          >
            <h3
              class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
            >
              <i class="bi bi-funnel" aria-hidden="true"></i>{tr(
                "tagCloud.filters"
              )}
            </h3>

            <!-- Search -->
            <div class="mb-4">
              <div
                class="text-sm font-medium text-gray-700 dark:text-gray-300 block mb-2"
              >
                {tr("tagCloud.search")}
              </div>
              <input
                type="text"
                bind:value={searchQuery}
                placeholder={tr("tagCloud.searchPlaceholder")}
                class="w-full px-3 py-2 rounded-lg bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white text-sm"
              />
            </div>

            <!-- Min Usage -->
            <div class="mb-4">
              <div
                class="text-sm font-medium text-gray-700 dark:text-gray-300 block mb-2"
              >
                {tr("tagCloud.minUsage")}: {minUsage}
              </div>
              <input
                type="range"
                bind:value={minUsage}
                min="1"
                max={Math.max(...tags.map((t) => t.usage || 1))}
                class="w-full"
              />
            </div>

            <!-- Font Size Control -->
            <div class="mb-4">
              <div
                class="text-sm font-medium text-gray-700 dark:text-gray-300 block mb-2"
              >
                {tr("tagCloud.fontSize")}: {minFontSize}-{maxFontSize}px
              </div>
              <div class="flex gap-2">
                <input
                  type="range"
                  bind:value={minFontSize}
                  min="10"
                  max="30"
                  class="flex-1"
                />
                <input
                  type="range"
                  bind:value={maxFontSize}
                  min="30"
                  max="60"
                  class="flex-1"
                />
              </div>
            </div>

            <hr class="border-gray-200 dark:border-gray-700 my-4" />

            <p class="text-xs text-gray-500 dark:text-gray-400">
              {tr("tagCloud.displayingTags", displayTags.length, tags.length)}
            </p>
          </div>

          <!-- Selected Tags Card -->
          {#if selectedTagsArray.length > 0}
            <div
              class="bg-green-50 dark:bg-green-900/20 rounded-xl p-6 border border-green-200 dark:border-green-800"
            >
              <h3
                class="font-semibold text-gray-900 dark:text-white mb-3 flex items-center gap-2"
              >
                <i class="bi bi-check-circle" aria-hidden="true"></i>{tr(
                  "tagCloud.selected"
                )}: {selectedTagsArray.length}
              </h3>

              <div class="space-y-2 mb-4 max-h-48 overflow-y-auto">
                {#each selectedTagsArray as tag (tag.id)}
                  <div
                    class="bg-white dark:bg-gray-800 rounded-lg px-3 py-2 flex items-center justify-between"
                  >
                    <div class="flex items-center gap-2">
                      <div
                        class="w-3 h-3 rounded-full"
                        style="background-color: {getTagColor(tag)}"
                      ></div>
                      <span
                        class="text-sm font-medium text-gray-900 dark:text-white"
                        >{tag.name}</span
                      >
                    </div>
                    <button
                      onclick={() => toggleTag(tag)}
                      class="text-gray-500 hover:text-red-500 transition-colors"
                      aria-label="Remove tag {tag.name}"
                    >
                      <i class="bi bi-x" aria-hidden="true"></i>
                    </button>
                  </div>
                {/each}
              </div>

              <button
                onclick={clearSelection}
                class="w-full px-3 py-2 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
              >
                {tr("tagCloud.clearSelection")}
              </button>
            </div>
          {/if}

          <!-- Stats Card -->
          <div
            class="bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 rounded-xl p-6 border border-green-200 dark:border-green-800"
          >
            <h3
              class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
            >
              <i class="bi bi-bar-chart" aria-hidden="true"></i>{tr(
                "tagCloud.stats"
              )}
            </h3>
            <div class="space-y-3 text-sm">
              <div class="flex justify-between">
                <span class="text-gray-600 dark:text-gray-400"
                  >{tr("tagCloud.totalTags")}:</span
                >
                <span class="font-semibold text-gray-900 dark:text-white"
                  >{tags.length}</span
                >
              </div>
              <div class="flex justify-between">
                <span class="text-gray-600 dark:text-gray-400"
                  >{tr("tagCloud.totalFiles")}:</span
                >
                <span class="font-semibold text-gray-900 dark:text-white"
                  >{totalFiles}</span
                >
              </div>
              <div class="flex justify-between">
                <span class="text-gray-600 dark:text-gray-400"
                  >{tr("tagCloud.avgUsage")}:</span
                >
                <span class="font-semibold text-gray-900 dark:text-white">
                  {(totalFiles / Math.max(tags.length, 1)).toFixed(1)}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-600 dark:text-gray-400"
                  >{tr("tagCloud.mostUsed")}:</span
                >
                <span class="font-semibold text-gray-900 dark:text-white">
                  {tags[0]?.name || "—"} ({tags[0]?.usage || 0})
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</PageWrapper>

<!-- Tag Detail Modal -->
{#if showTagModal && selectedTag}
  <Modal title={selectedTag.name} onclose={closeTagModal}>
    <div class="space-y-6">
      <!-- Tag Info Header -->
      <div
        class="flex items-center gap-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-xl"
      >
        <div
          class="w-16 h-16 rounded-full flex items-center justify-center text-white text-2xl font-bold"
          style="background-color: {getTagColor(selectedTag)}"
        >
          <i class="bi bi-tag-fill" aria-hidden="true"></i>
        </div>
        <div class="flex-1">
          <h3 class="text-xl font-bold text-gray-900 dark:text-white">
            {selectedTag.name}
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            {selectedTag.usage}
            {tr("tagCloud.files")}
          </p>
          {#if selectedTag.created_at}
            <p class="text-sm text-gray-500 dark:text-gray-500">
              {tr("tagCloud.createdAt")}: {new Date(
                selectedTag.created_at
              ).toLocaleDateString()}
            </p>
          {/if}
        </div>
      </div>

      <!-- Files with this tag -->
      <div>
        <h4
          class="font-semibold text-gray-900 dark:text-white mb-3 flex items-center gap-2"
        >
          <i class="bi bi-files" aria-hidden="true"></i>
          {tr("tagCloud.filesWithTag")}
        </h4>

        {#if loadingTagFiles}
          <div class="flex items-center justify-center py-8">
            <div
              class="w-8 h-8 border-2 border-green-500 border-t-transparent rounded-full animate-spin"
            ></div>
          </div>
        {:else if tagFiles.length === 0}
          <div class="text-center py-8 text-gray-500 dark:text-gray-400">
            <i
              class="bi bi-folder2-open text-4xl opacity-50 mb-2"
              aria-hidden="true"
            ></i>
            <p>{tr("tagCloud.noFilesWithTag")}</p>
          </div>
        {:else}
          <div class="max-h-64 overflow-y-auto space-y-2">
            {#each tagFiles as file}
              <div
                class="flex items-center gap-3 p-3 bg-white dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600 hover:border-green-500 dark:hover:border-green-500 transition-colors"
              >
                <i
                  class="bi bi-file-earmark text-gray-400 dark:text-gray-500"
                  aria-hidden="true"
                ></i>
                <div class="flex-1 min-w-0">
                  <p class="font-medium text-gray-900 dark:text-white truncate">
                    {file.name || file.filename}
                  </p>
                  <p class="text-xs text-gray-500 dark:text-gray-400 truncate">
                    {file.path || file.file_path}
                  </p>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Actions -->
      <div
        class="flex justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700"
      >
        <ModernButton variant="secondary" onclick={closeTagModal}>
          {tr("close")}
        </ModernButton>
        <ModernButton
          variant="danger"
          onclick={(e) => {
            closeTagModal();
            deleteTag(selectedTag, e);
          }}
        >
          <i class="bi bi-trash mr-2" aria-hidden="true"></i>
          {tr("delete")}
        </ModernButton>
      </div>
    </div>
  </Modal>
{/if}
