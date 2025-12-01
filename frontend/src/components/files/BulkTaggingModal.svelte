<script>
  import { onMount } from "svelte";
  import { tags as tagsApi } from "../../lib/api.js";
  import { t } from "../../i18n.js";

  let { selectedFiles = [], onClose = null } = $props();

  let availableTags = $state([]);
  let selectedTags = $state(new Set());
  let newTagName = $state("");
  let newTagColor = $state("#3B82F6");
  let loading = $state(false);
  let error = $state(null);
  let taggingInProgress = $state(false);

  const predefinedColors = [
    "#3B82F6", // blue
    "#EF4444", // red
    "#10B981", // emerald
    "#F59E0B", // amber
    "#8B5CF6", // violet
    "#EC4899", // pink
    "#06B6D4", // cyan
    "#6B7280", // gray
  ];

  onMount(async () => {
    await loadAvailableTags();
  });

  async function loadAvailableTags() {
    loading = true;
    try {
      availableTags = await tagsApi.list();
    } catch (e) {
      console.error("Failed to load tags:", e);
      error = $t("bulkTagging.tagsLoadError");
    } finally {
      loading = false;
    }
  }

  function toggleTag(tagId) {
    if (selectedTags.has(tagId)) {
      selectedTags.delete(tagId);
    } else {
      selectedTags.add(tagId);
    }
    selectedTags = selectedTags;
  }

  async function createAndAddTag() {
    if (!newTagName.trim()) return;

    try {
      const newTag = await tagsApi.create({
        name: newTagName.trim(),
        color: newTagColor,
      });
      availableTags = [...availableTags, newTag];
      selectedTags.add(newTag.id);
      selectedTags = selectedTags;
      newTagName = "";
      newTagColor = "#3B82F6";
    } catch (e) {
      console.error("Failed to create tag:", e);
      error = $t("bulkTagging.tagCreateError");
    }
  }

  async function applyTags() {
    if (selectedTags.size === 0) {
      error = $t("bulkTagging.selectTagsError");
      return;
    }

    if (selectedFiles.length === 0) {
      error = $t("bulkTagging.selectFilesError");
      return;
    }

    taggingInProgress = true;
    error = null;
    const tagsToAdd = Array.from(selectedTags);

    try {
      let successCount = 0;
      let failureCount = 0;

      for (const file of selectedFiles) {
        try {
          for (const tagId of tagsToAdd) {
            await tagsApi.tagFile({
              file_path: file.path,
              tag_id: tagId,
            });
          }
          successCount++;
        } catch (e) {
          console.error(`Failed to tag ${file.name}:`, e);
          failureCount++;
        }
      }

      if (failureCount === 0) {
        // Success - close modal
        if (onClose) onClose(true);
      } else {
        error = $t("bulkTagging.partialError")
          .replace("{success}", successCount)
          .replace("{failure}", failureCount);
      }
    } catch (e) {
      console.error("Failed to apply tags:", e);
      error = $t("bulkTagging.applyError");
    } finally {
      taggingInProgress = false;
    }
  }

  function getTagColor(tagId) {
    const tag = availableTags.find((t) => t.id === tagId);
    return tag?.color || "#3B82F6";
  }

  function getTagName(tagId) {
    const tag = availableTags.find((t) => t.id === tagId);
    return tag?.name || "Unknown";
  }
</script>

<div class="modal modal-open">
  <div class="modal-box max-w-lg">
    <h3 class="font-bold text-lg flex items-center gap-2">
      <i class="bi bi-tags"></i>
      {$t("bulkTagging.title")}
    </h3>

    <!-- File Count -->
    <div class="my-4 p-3 bg-base-200/50 rounded-lg">
      <p class="text-sm font-medium">
        {$t("bulkTagging.filesSelected")}:
        <span class="text-primary font-bold">{selectedFiles.length}</span>
      </p>
      <p class="text-xs text-base-content/60 mt-1">
        {selectedFiles.map((f) => f.name).join(", ")}
      </p>
    </div>

    <!-- Error Alert -->
    {#if error}
      <div class="alert alert-error mb-4 gap-2">
        <i class="bi bi-exclamation-triangle"></i>
        <span class="text-sm">{error}</span>
        <button aria-label="Close" onclick={() => (error = null)} class="btn btn-ghost btn-sm ml-auto"><i class="bi bi-x-lg" aria-hidden="true"></i></button>
      </div>
    {/if}

    <!-- Available Tags -->
    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text font-medium"
          >{$t("bulkTagging.availableTags")}</span
        >
        <span class="badge badge-sm badge-primary">{availableTags.length}</span>
      </label>

      {#if loading}
        <div class="flex justify-center py-4">
          <span class="loading loading-spinner loading-sm text-primary"></span>
        </div>
      {:else if availableTags.length === 0}
        <p class="text-sm text-base-content/60 p-3 text-center">
          {$t("bulkTagging.noTags")}
        </p>
      {:else}
        <div class="flex flex-wrap gap-2">
          {#each availableTags as tag (tag.id)}
            <button
              onclick={() => toggleTag(tag.id)}
              class="badge badge-lg gap-2 cursor-pointer transition-all {selectedTags.has(
                tag.id
              )
                ? 'badge-outline ring-2 ring-offset-1'
                : 'badge-ghost'}"
              style="background-color: {selectedTags.has(tag.id)
                ? tag.color
                : 'transparent'}; color: {selectedTags.has(tag.id)
                ? 'white'
                : 'inherit'}; border-color: {tag.color};"
            >
              <span class="text-xs font-medium">{tag.name}</span>
              {#if selectedTags.has(tag.id)}
                <i class="bi bi-check-lg text-sm"></i>
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Create New Tag -->
    <div class="divider my-3">{$t("bulkTagging.or")}</div>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text font-medium text-sm"
          >{$t("bulkTagging.createNewTag")}</span
        >
      </label>

      <div class="flex gap-2">
        <input
          type="text"
          placeholder={$t("bulkTagging.tagNamePlaceholder")}
          bind:value={newTagName}
          class="input input-bordered input-sm flex-1"
        />

        <!-- Color Picker -->
        <div class="dropdown dropdown-end">
          <button
            class="btn btn-sm btn-outline"
            style="background-color: {newTagColor}; border-color: {newTagColor};"
            title={$t("bulkTagging.selectColor")}
          >
            <i class="bi bi-palette"></i>
          </button>
          <ul
            class="dropdown-content menu p-3 bg-base-100 border border-base-300 rounded-lg flex-row gap-2 flex-wrap"
          >
            {#each predefinedColors as color}
              <button
                onclick={() => (newTagColor = color)}
                class="w-8 h-8 rounded-full transition-transform hover:scale-110 {newTagColor ===
                color
                  ? 'ring-2 ring-offset-2'
                  : ''}"
                style="background-color: {color}; border-color: {color};"
                title={color}
              ></button>
            {/each}
          </ul>
        </div>

        <button
          onclick={createAndAddTag}
          disabled={!newTagName.trim()}
          class="btn btn-primary btn-sm gap-1"
        >
          <i class="bi bi-plus-lg"></i>
          {$t("add")}
        </button>
      </div>
    </div>

    <!-- Selected Tags Preview -->
    {#if selectedTags.size > 0}
      <div class="mb-4 p-3 bg-primary/10 rounded-lg border border-primary/30">
        <p class="text-xs font-medium mb-2">{$t("bulkTagging.toBeApplied")}:</p>
        <div class="flex flex-wrap gap-2">
          {#each Array.from(selectedTags) as tagId (tagId)}
            <div
              class="badge badge-sm gap-1"
              style="background-color: {getTagColor(tagId)}; color: white;"
            >
              <span>{getTagName(tagId)}</span>
              <button
                onclick={() => {
                  selectedTags.delete(tagId);
                  selectedTags = selectedTags;
                }}
                class="hover:opacity-70"
              >
                <i class="bi bi-x-lg text-xs"></i>
              </button>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Modal Actions -->
    <div class="modal-action gap-2">
      <button onclick={() => onClose?.(false)} class="btn btn-ghost">
        {$t("cancel")}
      </button>
      <button
        onclick={applyTags}
        disabled={selectedTags.size === 0 || taggingInProgress}
        class="btn btn-primary gap-2"
      >
        {#if taggingInProgress}
          <span class="loading loading-spinner loading-sm"></span>
        {:else}
          <i class="bi bi-check-lg"></i>
        {/if}
        {$t("bulkTagging.apply")} ({selectedTags.size})
      </button>
    </div>
  </div>
  <div
    class="modal-backdrop bg-black/50"
    onclick={() => onClose?.(false)}
  ></div>
</div>
