<script>
  import { success } from "../../stores/toast";

  let { file } = $props();
  let tags = $state([]);
  let showTagPicker = $state(false);
  let newTagName = $state("");
  let newTagColor = $state("#3b82f6");

  function createTag() {
    if (!newTagName.trim()) return;
    success("🏷️ Tags feature coming soon!");
    newTagName = "";
    newTagColor = "#3b82f6";
    showTagPicker = false;
  }
</script>

<div class="space-y-4">
  <!-- Current Tags -->
  <div class="card bg-base-200">
    <div class="card-body p-4">
      <h3 class="font-semibold mb-3 flex items-center gap-2">
        <i class="bi bi-tags"></i>
        Current Tags
      </h3>
      {#if tags.length === 0}
        <p class="text-sm text-base-content/60">No tags assigned yet.</p>
      {/if}
    </div>
  </div>

  <!-- Add Tag -->
  <div class="card bg-base-200">
    <div class="card-body p-4">
      <h3 class="font-semibold mb-3 flex items-center gap-2">
        <i class="bi bi-plus-circle"></i>
        Add Tag
      </h3>

      {#if showTagPicker}
        <div class="space-y-3">
          <div class="form-control">
            <label class="label">
              <span class="label-text">Tag Name</span>
            </label>
            <input
              type="text"
              bind:value={newTagName}
              placeholder="e.g. Important, Work, Personal"
              class="input input-bordered"
            />
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">Tag Color</span>
            </label>
            <div class="flex gap-2">
              <input
                type="color"
                bind:value={newTagColor}
                class="input input-bordered w-20 h-10 cursor-pointer p-1"
              />
              <input
                type="text"
                bind:value={newTagColor}
                placeholder="#3b82f6"
                class="input input-bordered flex-1"
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
      {:else}
        <button
          onclick={() => (showTagPicker = true)}
          class="btn btn-outline btn-sm gap-2"
        >
          <i class="bi bi-plus-circle"></i>
          Create New Tag
        </button>
      {/if}

      <div class="alert alert-info mt-4">
        <i class="bi bi-info-circle"></i>
        <span>Tags feature will be implemented with backend API</span>
      </div>
    </div>
  </div>
</div>
