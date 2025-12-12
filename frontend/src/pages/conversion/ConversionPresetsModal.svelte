<script>
  import { onMount } from "svelte";
  import UIInput from "../../components/ui/UIInput.svelte";
  import UISelect from "../../components/ui/UISelect.svelte";
  import UIModal from "../../components/ui/UIModal.svelte";
  import UIButton from "../../components/ui/UIButton.svelte";
  import { language } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { conversion } from "../../lib/api.js";

  let { formats, onclose } = $props();

  let presets = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let showCreateForm = $state(false);
  let newPresetName = $state("");
  let newPresetFormat = $state("");

  onMount(async () => {
    await loadPresets();
  });

  async function loadPresets() {
    loading = true;
    error = null;
    try {
      presets = await conversion.listPresets();
    } catch (err) {
      error = err.message || "Failed to load presets";
      console.error("Load presets error:", err);
    } finally {
      loading = false;
    }
  }

  async function handleCreatePreset() {
    if (!newPresetName || !newPresetFormat) return;

    try {
      await conversion.createPreset(newPresetName, newPresetFormat, {});
      newPresetName = "";
      newPresetFormat = "";
      showCreateForm = false;
      await loadPresets();
    } catch (err) {
      error = err.message || "Failed to create preset";
      console.error("Create preset error:", err);
    }
  }

  async function handleDeletePreset(presetId, presetName) {
    if (!confirm(t($language, "conversion.confirmDeletePreset", presetName)))
      return;

    try {
      await conversion.deletePreset(presetId);
      await loadPresets();
    } catch (err) {
      error = err.message || "Failed to delete preset";
      console.error("Delete preset error:", err);
    }
  }

  function handleBackdropClick(e) {
    if (e.target === e.currentTarget) {
      onclose();
    }
  }

  // Categorize formats
  const allFormats = $derived([
    ...(formats.image || []),
    ...(formats.document || []),
    ...(formats.video || []),
    ...(formats.audio || []),
  ]);
</script>

<div
  class="modal modal-open"
  role="dialog"
  tabindex="-1"
  onclick={handleBackdropClick}
  onkeydown={(e) => e.key === "Escape" && onclose()}
>
  <div class="modal-box max-w-2xl" role="document">
    <!-- Header -->
    <div class="mb-4 flex items-center justify-between">
      <h3 class="text-xl font-bold text-base-content">
        <i class="bi bi-bookmark mr-2"></i>
        {t($language, "conversion.presets")}
      </h3>
      <button
        onclick={onclose}
        class="btn btn-circle btn-ghost btn-sm"
        aria-label="Close"
      >
        <i class="bi bi-x-lg"></i>
      </button>
    </div>

    <!-- Error -->
    {#if error}
      <div class="alert alert-error mb-4">
        <i class="bi bi-exclamation-triangle"></i>
        <span>{error}</span>
      </div>
    {/if}

    <!-- Create Preset Form -->
    {#if showCreateForm}
      <div class="card bg-base-200 mb-4 p-4">
        <div class="space-y-3">
          <div class="form-control">
            <label for="presetName" class="label">
              <span class="label-text"
                >{t($language, "conversion.presetName")}</span
              >
            </label>
            <input
              id="presetName"
              type="text"
              bind:value={newPresetName}
              placeholder="Web Optimized JPG"
              class="input input-bordered input-sm"
            />
          </div>

          <div class="form-control">
            <label for="presetFormat" class="label">
              <span class="label-text"
                >{t($language, "conversion.targetFormat")}</span
              >
            </label>
            <select
              id="presetFormat"
              bind:value={newPresetFormat}
              class="select select-bordered select-sm"
            >
              <option value="" disabled
                >{t($language, "conversion.selectFormat")}</option
              >
              {#each allFormats as format}
                <option value={format}>{format.toUpperCase()}</option>
              {/each}
            </select>
          </div>

          <div class="flex gap-2 justify-end">
            <button
              onclick={() => {
                showCreateForm = false;
                newPresetName = "";
                newPresetFormat = "";
              }}
              class="btn btn-ghost btn-sm"
            >
              {t($language, "cancel")}
            </button>
            <button
              onclick={handleCreatePreset}
              class="btn btn-primary btn-sm"
              disabled={!newPresetName || !newPresetFormat}
            >
              <i class="bi bi-check-lg"></i>
              {t($language, "conversion.savePreset")}
            </button>
          </div>
        </div>
      </div>
    {:else}
      <button
        onclick={() => (showCreateForm = true)}
        class="btn btn-primary btn-sm mb-4 w-full"
      >
        <i class="bi bi-plus-lg"></i>
        {t($language, "conversion.createPreset")}
      </button>
    {/if}

    <!-- Presets List -->
    <div class="space-y-2">
      {#if loading}
        <div class="flex justify-center py-8">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
      {:else if presets.length === 0}
        <div class="text-center py-8 text-base-content/60">
          <i class="bi bi-bookmark text-4xl mb-2 block"></i>
          <p>{t($language, "conversion.noJobs")}</p>
        </div>
      {:else}
        {#each presets as preset (preset.preset_id)}
          <div class="card bg-base-200 p-3">
            <div class="flex items-center justify-between">
              <div class="flex-1">
                <div class="font-semibold text-base-content">
                  {preset.name}
                </div>
                <div class="text-sm text-base-content/60">
                  {t($language, "conversion.targetFormat")}: {preset.target_format.toUpperCase()}
                  {#if preset.is_system}
                    <span class="badge badge-ghost badge-xs ml-2">System</span>
                  {/if}
                </div>
              </div>
              {#if !preset.is_system}
                <button
                  onclick={() =>
                    handleDeletePreset(preset.preset_id, preset.name)}
                  class="btn btn-error btn-xs"
                  aria-label={t($language, "conversion.deletePreset")}
                >
                  <i class="bi bi-trash"></i>
                </button>
              {/if}
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Actions -->
    <div class="modal-action">
      <button onclick={onclose} class="btn btn-ghost">
        {t($language, "close")}
      </button>
    </div>
  </div>
</div>
