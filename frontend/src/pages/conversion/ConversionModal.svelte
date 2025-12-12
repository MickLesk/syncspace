<script>
  import { language } from "../../stores/ui.js";
  import UIInput from "../../components/ui/UIInput.svelte";
  import UISelect from "../../components/ui/UISelect.svelte";
  import UIToggle from "../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../components/ui/UICheckbox.svelte";
  import UIModal from "../../components/ui/UIModal.svelte";
  import UIButton from "../../components/ui/UIButton.svelte";
  import { t } from "../../i18n.js";
  import { conversion } from "../../lib/api.js";
  import { currentPath } from "../../stores/files.js";

  let { formats, onclose, oncreated } = $props();

  let sourcePath = $state("");
  let targetFormat = $state("");
  let quality = $state(85);
  let resolution = $state("");
  let bitrate = $state("");
  let codec = $state("");
  let stripMetadata = $state(false);
  let loading = $state(false);
  let error = $state(null);

  // Categorize formats
  const imageFormats = $derived(formats.image || []);
  const documentFormats = $derived(formats.document || []);
  const videoFormats = $derived(formats.video || []);
  const audioFormats = $derived(formats.audio || []);

  async function handleSubmit() {
    if (!sourcePath || !targetFormat) {
      error = "Please select a file and target format";
      return;
    }

    loading = true;
    error = null;

    try {
      const options = {};
      if (quality) options.quality = quality;
      if (resolution) options.resolution = resolution;
      if (bitrate) options.bitrate = bitrate;
      if (codec) options.codec = codec;
      if (stripMetadata) options.strip_metadata = true;

      await conversion.createJob(sourcePath, targetFormat, options);
      oncreated();
    } catch (err) {
      error = err.message || "Failed to create conversion job";
      console.error("Create conversion error:", err);
    } finally {
      loading = false;
    }
  }

  function handleBackdropClick(e) {
    if (e.target === e.currentTarget) {
      onclose();
    }
  }
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
        <i class="bi bi-arrow-repeat mr-2"></i>
        {t($language, "conversion.convertFile")}
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

    <!-- Form -->
    <form
      onsubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
      class="space-y-4"
    >
      <!-- Source File -->
      <div class="form-control">
        <label for="sourcePath" class="label">
          <span class="label-text">{t($language, "conversion.sourceFile")}</span
          >
        </label>
        <input
          id="sourcePath"
          type="text"
          bind:value={sourcePath}
          placeholder="/path/to/file.png"
          class="input input-bordered w-full"
          required
          aria-describedby="sourcePath-help"
        />
        <div id="sourcePath-help" class="label">
          <span class="label-text-alt text-base-content/60">
            Enter the full path to the file you want to convert
          </span>
        </div>
      </div>

      <!-- Target Format -->
      <div class="form-control">
        <label for="targetFormat" class="label">
          <span class="label-text"
            >{t($language, "conversion.targetFormat")}</span
          >
        </label>
        <select
          id="targetFormat"
          bind:value={targetFormat}
          class="select select-bordered w-full"
          required
        >
          <option value="" disabled
            >{t($language, "conversion.selectFormat")}</option
          >

          {#if imageFormats.length > 0}
            <optgroup label={t($language, "conversion.imageFormats")}>
              {#each imageFormats as format}
                <option value={format}>{format.toUpperCase()}</option>
              {/each}
            </optgroup>
          {/if}

          {#if documentFormats.length > 0}
            <optgroup label={t($language, "conversion.documentFormats")}>
              {#each documentFormats as format}
                <option value={format}>{format.toUpperCase()}</option>
              {/each}
            </optgroup>
          {/if}

          {#if videoFormats.length > 0}
            <optgroup label={t($language, "conversion.videoFormats")}>
              {#each videoFormats as format}
                <option value={format}>{format.toUpperCase()}</option>
              {/each}
            </optgroup>
          {/if}

          {#if audioFormats.length > 0}
            <optgroup label={t($language, "conversion.audioFormats")}>
              {#each audioFormats as format}
                <option value={format}>{format.toUpperCase()}</option>
              {/each}
            </optgroup>
          {/if}
        </select>
      </div>

      <!-- Options (conditional based on format type) -->
      {#if targetFormat}
        <div class="divider text-sm">{t($language, "conversion.options")}</div>

        <!-- Image/Video Quality -->
        {#if imageFormats.includes(targetFormat) || videoFormats.includes(targetFormat)}
          <div class="form-control">
            <label for="quality" class="label">
              <span class="label-text"
                >{t($language, "conversion.quality")}</span
              >
              <span class="label-text-alt">{quality}%</span>
            </label>
            <input
              id="quality"
              type="range"
              min="1"
              max="100"
              bind:value={quality}
              class="range range-primary"
            />
          </div>
        {/if}

        <!-- Video Resolution -->
        {#if videoFormats.includes(targetFormat)}
          <div class="form-control">
            <label for="resolution" class="label">
              <span class="label-text"
                >{t($language, "conversion.resolution")}</span
              >
            </label>
            <select
              id="resolution"
              bind:value={resolution}
              class="select select-bordered select-sm"
            >
              <option value="">Original</option>
              <option value="1920x1080">1080p (1920x1080)</option>
              <option value="1280x720">720p (1280x720)</option>
              <option value="854x480">480p (854x480)</option>
              <option value="640x360">360p (640x360)</option>
            </select>
          </div>

          <div class="form-control">
            <UISelect
              label={t($language, "conversion.codec")}
              bind:value={codec}
              options={[
                { value: "", label: "Default" },
                { value: "libx264", label: "H.264" },
                { value: "libx265", label: "H.265 (HEVC)" },
                { value: "libvpx-vp9", label: "VP9" },
              ]}
            />
          </div>
        {/if}

        <!-- Audio Bitrate -->
        {#if audioFormats.includes(targetFormat)}
          <div class="form-control">
            <label for="bitrate" class="label">
              <span class="label-text"
                >{t($language, "conversion.bitrate")}</span
              >
            </label>
            <select
              id="bitrate"
              bind:value={bitrate}
              class="select select-bordered select-sm"
            >
              <option value="">Default</option>
              <option value="320k">320 kbps</option>
              <option value="256k">256 kbps</option>
              <option value="192k">192 kbps</option>
              <option value="128k">128 kbps</option>
              <option value="96k">96 kbps</option>
            </select>
          </div>
        {/if}

        <!-- Strip Metadata -->
        {#if imageFormats.includes(targetFormat)}
          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input
                type="checkbox"
                bind:checked={stripMetadata}
                class="checkbox checkbox-primary"
              />
              <span class="label-text"
                >{t($language, "conversion.stripMetadata")}</span
              >
            </label>
          </div>
        {/if}
      {/if}

      <!-- Actions -->
      <div class="modal-action">
        <button
          type="button"
          onclick={onclose}
          class="btn btn-ghost"
          disabled={loading}
        >
          {t($language, "cancel")}
        </button>
        <button
          type="submit"
          class="btn btn-primary"
          disabled={loading || !sourcePath || !targetFormat}
        >
          {#if loading}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            <i class="bi bi-arrow-repeat"></i>
          {/if}
          {t($language, "conversion.convertFile")}
        </button>
      </div>
    </form>
  </div>
</div>
