<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";

  let { file } = $props();
  let allTags = $state([]);
  let fileTags = $state([]);
  let loading = $state(false);
  let showTagPicker = $state(false);
  let newTagName = $state("");
  let newTagColor = $state("#3b82f6");

  onMount(() => {
    loadTags();
  });

  async function loadTags() {
    loading = true;
    try {
      const data = await api.tags.list();
      allTags = Array.isArray(data) ? data : [];
      // Load file-specific tags when endpoint available
      if (file && (file.id || file.path || file.name)) {
        try {
          const fileId = file.id || file.path || file.name;
          const tags = await api.tags.getFileTags(fileId);
          fileTags = Array.isArray(tags) ? tags : [];
        } catch (err) {
          console.warn("Could not load file-specific tags:", err);
          fileTags = [];
        }
      }
    } catch (err) {
      console.error("Failed to load tags:", err);
      errorToast("Failed to load tags");
    } finally {
      loading = false;
    }
  }

  async function createTag() {
    if (!newTagName.trim()) return;
    try {
      await api.tags.create({ name: newTagName.trim(), color: newTagColor });
      success("🏷️ Tag created!");
      newTagName = "";
      newTagColor = "#3b82f6";
      showTagPicker = false;
      await loadTags();
    } catch (err) {
      console.error("Failed to create tag:", err);
      errorToast("Failed to create tag");
    }
  }

  async function assignTag(tag) {
    try {
      const fileId = file.path || file.name;
      await api.tags.tagFile({ file_id: fileId, tag_id: tag.id });
      success(`✅ Tagged with "${tag.name}"`);
      await loadTags();
    } catch (err) {
      console.error("Failed to assign tag:", err);
      errorToast("Failed to assign tag");
    }
  }

  async function removeTag(tag) {
    if (!confirm(`Remove tag "${tag.name}"?`)) return;
    try {
      const fileId = file.path || file.name;
      await api.tags.untagFile(`${fileId}_${tag.id}`);
      success(`🗑️ Tag "${tag.name}" removed`);
      await loadTags();
    } catch (err) {
      console.error("Failed to remove tag:", err);
      errorToast("Failed to remove tag");
    }
  }

  async function deleteTag(tag) {
    if (
      !confirm(`Delete tag "${tag.name}"? This will remove it from all files.`)
    )
      return;
    try {
      await api.tags.delete(tag.id);
      success(`🗑️ Tag "${tag.name}" deleted`);
      await loadTags();
    } catch (err) {
      console.error("Failed to delete tag:", err);
      errorToast("Failed to delete tag");
    }
  }
</script>

<div class="space-y-4">
  {#if loading}
    <!-- Loading State -->
    <div class="flex justify-center py-8">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else}
    <!-- File Tags -->
    {#if fileTags.length > 0}
      <div class="card bg-base-200">
        <div class="card-body p-4">
          <h3 class="font-semibold mb-3 flex items-center gap-2">
            <i class="bi bi-tags"></i>
            Tags on this file
          </h3>
          <div class="flex flex-wrap gap-2">
            {#each fileTags as tag}
              <div
                class="badge badge-lg gap-2 py-3 px-3"
                style="background-color: {tag.color ||
                  '#3b82f6'}; color: white;"
              >
                {tag.name}
                <button
                  onclick={() => removeTag(tag)}
                  class="btn btn-ghost btn-xs btn-circle text-white hover:bg-white/20"
                  title="Remove tag"
                >
                  <i class="bi bi-x"></i>
                </button>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}

    <!-- Available Tags -->
    <div class="card bg-base-200">
      <div class="card-body p-4">
        <div class="flex justify-between items-center mb-3">
          <h3 class="font-semibold flex items-center gap-2">
            <i class="bi bi-tag"></i>
            Available Tags
          </h3>
          {#if !showTagPicker}
            <button
              onclick={() => (showTagPicker = true)}
              class="btn btn-primary btn-sm gap-2"
            >
              <i class="bi bi-plus-circle"></i>
              New Tag
            </button>
          {/if}
        </div>

        {#if showTagPicker}
          <!-- Create Tag Form -->
          <div class="bg-base-300 rounded-lg p-4 mb-4 space-y-3">
            <div class="form-control">
              <label for="tag-name-input" class="label">
                <span class="label-text font-semibold">Tag Name</span>
              </label>
              <input
                id="tag-name-input"
                type="text"
                bind:value={newTagName}
                placeholder="e.g. Important, Work, Personal"
                class="input input-bordered"
                onkeydown={(e) => {
                  if (e.key === "Enter") createTag();
                }}
              />
            </div>
            <div class="form-control">
              <label for="tag-color-input" class="label">
                <span class="label-text font-semibold">Color</span>
              </label>
              <div class="flex gap-2">
                <input
                  id="tag-color-input"
                  type="color"
                  bind:value={newTagColor}
                  class="input input-bordered w-20 h-10 cursor-pointer p-1"
                />
                <input
                  type="text"
                  bind:value={newTagColor}
                  placeholder="#3b82f6"
                  class="input input-bordered flex-1 font-mono"
                  pattern="^#[0-9A-Fa-f]{6}$"
                />
              </div>
            </div>
            <div class="flex gap-2">
              <button
                onclick={() => {
                  showTagPicker = false;
                  newTagName = "";
                  newTagColor = "#3b82f6";
                }}
                class="btn btn-ghost btn-sm"
              >
                Cancel
              </button>
              <button
                onclick={createTag}
                class="btn btn-primary btn-sm gap-2"
                disabled={!newTagName.trim()}
              >
                <i class="bi bi-plus-circle"></i>
                Create Tag
              </button>
            </div>
          </div>
        {/if}

        {#if allTags.length === 0}
          <div class="text-center py-8 text-base-content/60">
            <i class="bi bi-tags text-5xl mb-3 block opacity-30"></i>
            <p class="text-sm">No tags created yet.</p>
            <p class="text-xs mt-1">Create your first tag to organize files!</p>
          </div>
        {:else}
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
            {#each allTags as tag}
              <div class="card bg-base-100 border border-base-300">
                <div
                  class="card-body p-3 flex-row items-center justify-between gap-2"
                >
                  <button
                    onclick={() => assignTag(tag)}
                    class="flex items-center gap-2 flex-1 min-w-0 text-left"
                  >
                    <div
                      class="w-4 h-4 rounded-full flex-shrink-0"
                      style="background-color: {tag.color || '#3b82f6'}"
                    ></div>
                    <span class="font-medium truncate">{tag.name}</span>
                  </button>
                  <button
                    onclick={() => deleteTag(tag)}
                    class="btn btn-ghost btn-xs btn-square text-error"
                    title="Delete tag"
                  >
                    <i class="bi bi-trash"></i>
                  </button>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
