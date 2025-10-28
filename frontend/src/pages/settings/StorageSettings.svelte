<script>
  import { success } from "../../stores/toast";

  let storageLocation = "./data";
  let maxFileSize = "100";

  function handleClearCache() {
    success("Cache cleared successfully!");
  }
</script>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <!-- Storage Configuration -->
  <div
    class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow"
  >
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-hdd-fill text-primary"></i>
        Storage Configuration
      </h2>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="storage-location">
            <span class="label-text">Storage Location</span>
          </label>
          <label class="input input-bordered flex items-center gap-2">
            <i class="bi bi-folder-fill opacity-70"></i>
            <input
              type="text"
              id="storage-location"
              class="grow"
              bind:value={storageLocation}
              disabled
            />
          </label>
          <label class="label">
            <span class="label-text-alt">Path where files are stored</span>
          </label>
        </div>

        <div class="form-control">
          <label class="label" for="max-file-size">
            <span class="label-text">Max File Size (MB)</span>
          </label>
          <label class="input input-bordered flex items-center gap-2">
            <i class="bi bi-file-earmark-arrow-up opacity-70"></i>
            <input
              type="number"
              id="max-file-size"
              class="grow"
              bind:value={maxFileSize}
            />
          </label>
          <label class="label">
            <span class="label-text-alt">Maximum size for uploaded files</span>
          </label>
        </div>

        <div
          class="flex justify-between items-center p-3 bg-slate-50 dark:bg-slate-800 rounded-lg"
        >
          <div>
            <div class="font-semibold">Cache Size</div>
            <div class="text-sm opacity-70">Currently using 245 MB</div>
          </div>
          <button class="btn btn-error btn-sm" onclick={handleClearCache}>
            <i class="bi bi-trash-fill"></i>
            Clear Cache
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Storage Stats -->
  <div
    class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow"
  >
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-pie-chart-fill text-info"></i>
        Storage Usage
      </h2>

      <div class="stats stats-vertical shadow w-full">
        <div class="stat">
          <div class="stat-figure text-primary">
            <i class="bi bi-hdd text-3xl"></i>
          </div>
          <div class="stat-title">Total Space</div>
          <div class="stat-value text-primary">50 GB</div>
          <div class="stat-desc">Allocated storage</div>
        </div>

        <div class="stat">
          <div class="stat-figure text-secondary">
            <i class="bi bi-files text-3xl"></i>
          </div>
          <div class="stat-title">Used Space</div>
          <div class="stat-value text-secondary">42.5 GB</div>
          <div class="stat-desc">85% of total space</div>
        </div>

        <div class="stat">
          <div class="stat-figure text-success">
            <i class="bi bi-check-circle text-3xl"></i>
          </div>
          <div class="stat-title">Available</div>
          <div class="stat-value text-success">7.5 GB</div>
          <div class="stat-desc">Free space remaining</div>
        </div>
      </div>

      <div class="mt-4">
        <div class="flex justify-between text-sm mb-2">
          <span>Storage Usage</span>
          <span class="font-semibold">85%</span>
        </div>
        <progress class="progress progress-primary w-full" value="85" max="100"
        ></progress>
      </div>
    </div>
  </div>
</div>

