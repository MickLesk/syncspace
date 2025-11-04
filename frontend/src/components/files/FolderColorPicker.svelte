<script>
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";

  let { visible = $bindable(false), folder = $bindable(null) } = $props();

  let selectedColor = $state("#3b82f6");
  let loading = $state(false);

  const presetColors = [
    "#ef4444", // red
    "#f97316", // orange
    "#f59e0b", // amber
    "#eab308", // yellow
    "#84cc16", // lime
    "#22c55e", // green
    "#10b981", // emerald
    "#14b8a6", // teal
    "#06b6d4", // cyan
    "#0ea5e9", // sky
    "#3b82f6", // blue
    "#6366f1", // indigo
    "#8b5cf6", // violet
    "#a855f7", // purple
    "#d946ef", // fuchsia
    "#ec4899", // pink
    "#f43f5e", // rose
    "#64748b", // slate
  ];

  $effect(() => {
    if (visible && folder) {
      loadCurrentColor();
    }
  });

  async function loadCurrentColor() {
    if (!folder?.path && !folder?.name) return;
    try {
      const folderPath = folder.path || folder.name;
      const data = await api.folderColors.get(folderPath);
      if (data.color) {
        selectedColor = data.color;
      }
    } catch (err) {
      console.log("No color set for folder");
    }
  }

  async function saveColor() {
    if (!folder?.path && !folder?.name) return;
    loading = true;
    try {
      const folderPath = folder.path || folder.name;
      await api.folderColors.set(folderPath, selectedColor);
      success("🎨 Folder color updated!");
      visible = false;
      // Trigger refresh of file list
      window.location.reload();
    } catch (err) {
      console.error("Failed to set folder color:", err);
      errorToast("Failed to set folder color");
    } finally {
      loading = false;
    }
  }

  async function removeColor() {
    if (!folder?.path && !folder?.name) return;
    if (!confirm("Remove folder color?")) return;
    loading = true;
    try {
      const folderPath = folder.path || folder.name;
      await api.folderColors.remove(folderPath);
      success("🗑️ Folder color removed!");
      visible = false;
      window.location.reload();
    } catch (err) {
      console.error("Failed to remove folder color:", err);
      errorToast("Failed to remove folder color");
    } finally {
      loading = false;
    }
  }

  function close() {
    visible = false;
  }
</script>

{#if visible}
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    onclick={close}
    onkeydown={(e) => e.key === "Escape" && close()}
    role="button"
    tabindex="-1"
  >
    <div
      class="bg-base-100 rounded-2xl shadow-2xl w-full max-w-md"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="0"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-base-300"
      >
        <div class="flex items-center gap-3">
          <i class="bi bi-folder text-2xl" style="color: {selectedColor}"></i>
          <div>
            <h2 class="text-lg font-semibold">Folder Color</h2>
            <p class="text-sm text-base-content/60 truncate max-w-xs">
              {folder?.name || ""}
            </p>
          </div>
        </div>
        <button
          onclick={close}
          class="btn btn-sm btn-circle btn-ghost"
          title="Close"
        >
          <i class="bi bi-x-lg"></i>
        </button>
      </div>

      <!-- Body -->
      <div class="p-6 space-y-4">
        <!-- Preview -->
        <div class="flex justify-center py-8">
          <i class="bi bi-folder-fill text-8xl" style="color: {selectedColor}"
          ></i>
        </div>

        <!-- Color Picker -->
        <div class="form-control">
          <label for="custom-color-picker" class="label">
            <span class="label-text font-semibold">Custom Color</span>
          </label>
          <div class="flex gap-2">
            <input
              id="custom-color-picker"
              type="color"
              bind:value={selectedColor}
              class="input input-bordered w-20 h-12 cursor-pointer p-1"
            />
            <input
              type="text"
              bind:value={selectedColor}
              placeholder="#3b82f6"
              class="input input-bordered flex-1 font-mono"
              pattern="^#[0-9A-Fa-f]{6}$"
            />
          </div>
        </div>

        <!-- Preset Colors -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">Preset Colors</span>
          </label>
          <div class="grid grid-cols-9 gap-2">
            {#each presetColors as color}
              <button
                onclick={() => (selectedColor = color)}
                class="w-10 h-10 rounded-lg transition-all hover:scale-110 border-2 {selectedColor ===
                color
                  ? 'border-base-content'
                  : 'border-transparent'}"
                style="background-color: {color}"
                title={color}
              ></button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-between px-6 py-4 border-t border-base-300 bg-base-200/30"
      >
        <button
          onclick={removeColor}
          class="btn btn-ghost btn-sm text-error gap-2"
          disabled={loading}
        >
          <i class="bi bi-trash"></i>
          Remove Color
        </button>
        <div class="flex gap-2">
          <button
            onclick={close}
            class="btn btn-ghost btn-sm"
            disabled={loading}
          >
            Cancel
          </button>
          <button
            onclick={saveColor}
            class="btn btn-primary btn-sm gap-2"
            disabled={loading}
          >
            {#if loading}
              <span class="loading loading-spinner loading-xs"></span>
            {:else}
              <i class="bi bi-check-lg"></i>
            {/if}
            Save Color
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
