<script>
  import { success } from "../../stores/toast";

  let autoBackup = true;
  let backupFrequency = "daily";

  function handleBackupNow() {
    success("Backup started successfully!");
  }
</script>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <!-- Backup Configuration -->
  <div
    class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow"
  >
    <div class="card-body">
      <h2 class="card-title mb-4">
        <i class="bi bi-cloud-arrow-up-fill text-info"></i>
        Backup Configuration
      </h2>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label cursor-pointer">
            <div>
              <span class="label-text font-medium">Automatic Backups</span>
              <p class="text-sm opacity-70">
                Automatically backup your data at scheduled intervals
              </p>
            </div>
            <div class="flex items-center gap-2">
              <span
                class="text-xs font-medium {autoBackup
                  ? 'text-info'
                  : 'text-base-content/40'}"
              >
                {autoBackup ? "ON" : "OFF"}
              </span>
              <input
                type="checkbox"
                class="toggle toggle-info"
                bind:checked={autoBackup}
              />
            </div>
          </label>
        </div>

        <div class="form-control">
          <label class="label" for="backup-frequency">
            <span class="label-text">Backup Frequency</span>
          </label>
          <select
            id="backup-frequency"
            class="select select-bordered w-full"
            bind:value={backupFrequency}
            disabled={!autoBackup}
          >
            <option value="hourly">Every Hour</option>
            <option value="daily">Daily</option>
            <option value="weekly">Weekly</option>
            <option value="monthly">Monthly</option>
          </select>
        </div>

        <div class="stats shadow w-full">
          <div class="stat">
            <div class="stat-figure text-primary">
              <i class="bi bi-clock-history text-3xl"></i>
            </div>
            <div class="stat-title">Last Backup</div>
            <div class="stat-value text-primary text-lg">2 hours ago</div>
            <div class="stat-desc">Next backup in 22 hours</div>
          </div>
        </div>
      </div>

      <div class="card-actions justify-end mt-4">
        <button class="btn btn-primary btn-sm" onclick={handleBackupNow}>
          <i class="bi bi-cloud-arrow-up"></i>
          Backup Now
        </button>
      </div>
    </div>
  </div>

  <!-- Backup History -->
  <div
    class="card bg-white dark:bg-slate-900 shadow-xl hover:shadow-2xl transition-shadow"
  >
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-archive-fill text-secondary"></i>
        Backup History
      </h2>

      <div class="space-y-3">
        {#each [{ date: "24.10.2025 02:00", size: "1.2 GB", status: "success" }, { date: "23.10.2025 02:00", size: "1.1 GB", status: "success" }, { date: "22.10.2025 02:00", size: "1.0 GB", status: "success" }] as backup}
          <div
            class="flex items-center justify-between p-3 bg-slate-50 dark:bg-slate-800 rounded-lg"
          >
            <div class="flex items-center gap-3">
              <i class="bi bi-check-circle-fill text-success text-xl"></i>
              <div>
                <div class="font-semibold">{backup.date}</div>
                <div class="text-sm opacity-70">{backup.size}</div>
              </div>
            </div>
            <div class="flex gap-2">
              <button class="btn btn-ghost btn-sm btn-circle">
                <i class="bi bi-download"></i>
              </button>
              <button class="btn btn-ghost btn-sm btn-circle text-error">
                <i class="bi bi-trash"></i>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>

