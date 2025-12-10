<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let compressing = $state(false);
  let analyzing = $state(false);
  let error = $state("");
  let success = $state("");

  // Algorithms and stats
  let algorithms = $state([]);
  let stats = $state(null);

  // Compress Modal
  let showCompressModal = $state(false);
  let compressForm = $state({
    files: "",
    algorithm: "gzip",
    level: 6,
    destination: "",
    deleteOriginals: false,
  });

  // Decompress Modal
  let showDecompressModal = $state(false);
  let decompressForm = $state({
    files: "",
    destination: "",
    deleteOriginals: false,
  });

  // Analysis Result
  let analysisResult = $state(null);
  let analyzePath = $state("");

  onMount(async () => {
    await Promise.all([loadAlgorithms(), loadStats()]);
  });

  async function loadAlgorithms() {
    try {
      const response = await api.compression.getAlgorithms();
      algorithms = response.algorithms || [];
    } catch (e) {
      console.error("Failed to load algorithms:", e);
    } finally {
      loading = false;
    }
  }

  async function loadStats() {
    try {
      stats = await api.compression.getStats();
    } catch (e) {
      console.error("Failed to load stats:", e);
    }
  }

  function openCompressModal() {
    compressForm = {
      files: "",
      algorithm: "zstd",
      level: 3,
      destination: "",
      deleteOriginals: false,
    };
    showCompressModal = true;
  }

  function openDecompressModal() {
    decompressForm = {
      files: "",
      destination: "",
      deleteOriginals: false,
    };
    showDecompressModal = true;
  }

  async function compressFiles() {
    const files = compressForm.files
      .split(",")
      .map((f) => f.trim())
      .filter(Boolean);
    if (files.length === 0) {
      error = "Please enter at least one file path";
      return;
    }

    compressing = true;
    error = "";
    try {
      const result = await api.compression.compress(
        files,
        compressForm.algorithm,
        {
          level: compressForm.level,
          destination: compressForm.destination || undefined,
          deleteOriginals: compressForm.deleteOriginals,
        }
      );
      success = `Compression job queued (Job ID: ${result.job_id})`;
      showCompressModal = false;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Failed to start compression";
    } finally {
      compressing = false;
    }
  }

  async function decompressFiles() {
    const files = decompressForm.files
      .split(",")
      .map((f) => f.trim())
      .filter(Boolean);
    if (files.length === 0) {
      error = "Please enter at least one file path";
      return;
    }

    compressing = true;
    error = "";
    try {
      const result = await api.compression.decompress(files, {
        destination: decompressForm.destination || undefined,
        deleteOriginals: decompressForm.deleteOriginals,
      });
      success = `Decompression job queued (Job ID: ${result.job_id})`;
      showDecompressModal = false;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Failed to start decompression";
    } finally {
      compressing = false;
    }
  }

  async function analyzeFile() {
    if (!analyzePath.trim()) {
      error = "Please enter a file path to analyze";
      return;
    }

    analyzing = true;
    error = "";
    analysisResult = null;
    try {
      analysisResult = await api.compression.analyze(analyzePath);
    } catch (e) {
      error = e.message || "Failed to analyze file";
    } finally {
      analyzing = false;
    }
  }

  function getSpeedBadge(speed) {
    switch (speed) {
      case "very_fast":
        return { class: "badge-success", label: "Very Fast" };
      case "fast":
        return { class: "badge-info", label: "Fast" };
      case "slow":
        return { class: "badge-warning", label: "Slow" };
      case "very_slow":
        return { class: "badge-error", label: "Very Slow" };
      default:
        return { class: "badge-ghost", label: speed };
    }
  }

  function getRatioBadge(ratio) {
    switch (ratio) {
      case "excellent":
        return { class: "badge-success", label: "Excellent" };
      case "best":
        return { class: "badge-success", label: "Best" };
      case "better":
        return { class: "badge-info", label: "Better" };
      case "good":
        return { class: "badge-info", label: "Good" };
      case "moderate":
        return { class: "badge-warning", label: "Moderate" };
      default:
        return { class: "badge-ghost", label: ratio };
    }
  }

  function formatBytes(bytes) {
    if (!bytes) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function closeCompressModal() {
    showCompressModal = false;
  }

  function closeDecompressModal() {
    showDecompressModal = false;
  }

  let selectedAlgorithm = $derived(
    algorithms.find((a) => a.id === compressForm.algorithm)
  );
</script>

<div class="compression-view p-6">
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-2xl font-bold flex items-center gap-2">
        <i class="bi bi-file-earmark-zip text-primary"></i>
        Compression Tools
      </h1>
      <p class="text-base-content/60 mt-1">
        Compress and decompress files using various algorithms
      </p>
    </div>
    <div class="flex gap-2">
      <button class="btn btn-outline" onclick={openDecompressModal}>
        <i class="bi bi-box-arrow-up"></i>
        Decompress
      </button>
      <button class="btn btn-primary" onclick={openCompressModal}>
        <i class="bi bi-box-arrow-down"></i>
        Compress
      </button>
    </div>
  </div>

  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-circle-fill"></i>
      <span>{error}</span>
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => (error = "")}
        aria-label="Dismiss"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if success}
    <div class="alert alert-success mb-4">
      <i class="bi bi-check-circle-fill"></i>
      <span>{success}</span>
    </div>
  {/if}

  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Algorithms Card -->
      <div class="card bg-base-200">
        <div class="card-body">
          <h3 class="card-title mb-4">
            <i class="bi bi-sliders text-primary"></i>
            Available Algorithms
          </h3>
          <div class="space-y-3">
            {#each algorithms as algo}
              {@const speedBadge = getSpeedBadge(algo.speed)}
              {@const ratioBadge = getRatioBadge(algo.ratio)}
              <div class="bg-base-100 rounded-lg p-4">
                <div class="flex justify-between items-start mb-2">
                  <div>
                    <h4 class="font-bold">{algo.name}</h4>
                    <p class="text-sm text-base-content/60">
                      {algo.description}
                    </p>
                  </div>
                  <code class="text-xs bg-base-300 px-2 py-1 rounded"
                    >{algo.extension}</code
                  >
                </div>
                <div class="flex gap-2 mt-2">
                  <span class="badge {speedBadge.class} badge-sm"
                    >{speedBadge.label}</span
                  >
                  <span class="badge {ratioBadge.class} badge-sm"
                    >{ratioBadge.label} ratio</span
                  >
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Analyze File Card -->
      <div class="card bg-base-200">
        <div class="card-body">
          <h3 class="card-title mb-4">
            <i class="bi bi-search text-primary"></i>
            Analyze Compression Potential
          </h3>
          <div class="form-control">
            <label class="label" for="analyze-path">
              <span class="label-text">File Path</span>
            </label>
            <div class="flex gap-2">
              <input
                id="analyze-path"
                type="text"
                class="input input-bordered flex-1"
                bind:value={analyzePath}
                placeholder="path/to/file.txt"
              />
              <button
                class="btn btn-primary"
                onclick={analyzeFile}
                disabled={analyzing}
              >
                {#if analyzing}
                  <span class="loading loading-spinner loading-sm"></span>
                {:else}
                  Analyze
                {/if}
              </button>
            </div>
          </div>

          {#if analysisResult}
            <div class="mt-4 bg-base-100 rounded-lg p-4">
              <h4 class="font-medium mb-3">Analysis Results</h4>
              <div class="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span class="text-base-content/60">Original Size:</span>
                  <p class="font-medium">
                    {formatBytes(analysisResult.original_size)}
                  </p>
                </div>
                <div>
                  <span class="text-base-content/60">Estimated Compressed:</span
                  >
                  <p class="font-medium">
                    {formatBytes(analysisResult.estimated_compressed_size)}
                  </p>
                </div>
                <div>
                  <span class="text-base-content/60">Estimated Savings:</span>
                  <p class="font-medium text-success">
                    {formatBytes(analysisResult.estimated_savings_bytes)}
                    ({analysisResult.estimated_savings_percent?.toFixed(1)}%)
                  </p>
                </div>
                <div>
                  <span class="text-base-content/60">Suggested Algorithm:</span>
                  <p class="font-medium">
                    {analysisResult.suggested_algorithm}
                  </p>
                </div>
              </div>
              <div class="mt-3 p-2 bg-base-200 rounded text-sm">
                <i class="bi bi-info-circle text-info"></i>
                {analysisResult.recommendation}
              </div>
              {#if analysisResult.is_already_compressed}
                <div
                  class="mt-2 p-2 bg-warning/20 rounded text-sm text-warning"
                >
                  <i class="bi bi-exclamation-triangle"></i>
                  File appears to be already compressed
                </div>
              {/if}
            </div>
          {/if}
        </div>
      </div>

      <!-- Statistics Card -->
      {#if stats}
        <div class="card bg-base-200 lg:col-span-2">
          <div class="card-body">
            <h3 class="card-title mb-4">
              <i class="bi bi-bar-chart text-primary"></i>
              Compression Statistics
            </h3>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div class="stat bg-base-100 rounded-lg">
                <div class="stat-title">Total Jobs</div>
                <div class="stat-value text-2xl">{stats.total_jobs || 0}</div>
              </div>
              <div class="stat bg-base-100 rounded-lg">
                <div class="stat-title">Completed</div>
                <div class="stat-value text-2xl text-success">
                  {stats.completed || 0}
                </div>
              </div>
              <div class="stat bg-base-100 rounded-lg">
                <div class="stat-title">Failed</div>
                <div class="stat-value text-2xl text-error">
                  {stats.failed || 0}
                </div>
              </div>
              <div class="stat bg-base-100 rounded-lg">
                <div class="stat-title">Space Saved</div>
                <div class="stat-value text-2xl">
                  {formatBytes(stats.total_bytes_saved || 0)}
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- Compress Modal -->
{#if showCompressModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeCompressModal}
      onkeydown={(e) => e.key === "Escape" && closeCompressModal()}
    ></div>
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-box-arrow-down text-primary"></i>
        Compress Files
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="compress-files">
            <span class="label-text">Files to Compress (comma-separated)</span>
          </label>
          <textarea
            id="compress-files"
            class="textarea textarea-bordered w-full h-24"
            bind:value={compressForm.files}
            placeholder="path/to/file1.txt, path/to/file2.log"
          ></textarea>
        </div>

        <div class="form-control">
          <label class="label" for="compress-algorithm">
            <span class="label-text">Algorithm</span>
          </label>
          <select
            id="compress-algorithm"
            class="select select-bordered w-full"
            bind:value={compressForm.algorithm}
          >
            {#each algorithms as algo}
              <option value={algo.id}>{algo.name} ({algo.extension})</option>
            {/each}
          </select>
          {#if selectedAlgorithm}
            <div class="label">
              <span class="label-text-alt text-base-content/60">
                {selectedAlgorithm.description}
              </span>
            </div>
          {/if}
        </div>

        <div class="form-control">
          <label class="label" for="compress-level">
            <span class="label-text"
              >Compression Level: {compressForm.level}</span
            >
          </label>
          <input
            id="compress-level"
            type="range"
            class="range range-primary"
            min="1"
            max="9"
            bind:value={compressForm.level}
          />
          <div class="flex justify-between text-xs text-base-content/60 px-1">
            <span>Fast (1)</span>
            <span>Balanced</span>
            <span>Best (9)</span>
          </div>
        </div>

        <div class="form-control">
          <label class="label" for="compress-destination">
            <span class="label-text">Destination (optional)</span>
          </label>
          <input
            id="compress-destination"
            type="text"
            class="input input-bordered w-full"
            bind:value={compressForm.destination}
            placeholder="Same as source"
          />
        </div>

        <div class="form-control">
          <label class="label cursor-pointer justify-start gap-3">
            <input
              type="checkbox"
              class="checkbox checkbox-primary"
              bind:checked={compressForm.deleteOriginals}
            />
            <span class="label-text"
              >Delete original files after compression</span
            >
          </label>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={closeCompressModal}
          >Cancel</button
        >
        <button
          class="btn btn-primary"
          onclick={compressFiles}
          disabled={compressing}
        >
          {#if compressing}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          Compress
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Decompress Modal -->
{#if showDecompressModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeDecompressModal}
      onkeydown={(e) => e.key === "Escape" && closeDecompressModal()}
    ></div>
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-box-arrow-up text-primary"></i>
        Decompress Files
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="decompress-files">
            <span class="label-text">Compressed Files (comma-separated)</span>
          </label>
          <textarea
            id="decompress-files"
            class="textarea textarea-bordered w-full h-24"
            bind:value={decompressForm.files}
            placeholder="path/to/file.gz, path/to/file.zst"
          ></textarea>
        </div>

        <div class="form-control">
          <label class="label" for="decompress-destination">
            <span class="label-text">Destination (optional)</span>
          </label>
          <input
            id="decompress-destination"
            type="text"
            class="input input-bordered w-full"
            bind:value={decompressForm.destination}
            placeholder="Same as source"
          />
        </div>

        <div class="form-control">
          <label class="label cursor-pointer justify-start gap-3">
            <input
              type="checkbox"
              class="checkbox checkbox-primary"
              bind:checked={decompressForm.deleteOriginals}
            />
            <span class="label-text"
              >Delete compressed files after decompression</span
            >
          </label>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={closeDecompressModal}
          >Cancel</button
        >
        <button
          class="btn btn-primary"
          onclick={decompressFiles}
          disabled={compressing}
        >
          {#if compressing}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          Decompress
        </button>
      </div>
    </div>
  </div>
{/if}
