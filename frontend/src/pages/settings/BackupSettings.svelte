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
    class="bg-white dark:bg-slate-900 rounded-2xl shadow-xl hover:shadow-2xl transition-shadow p-6"
  >
    <h2 class="text-xl font-bold mb-6 flex items-center gap-2">
      <i class="bi bi-cloud-arrow-up-fill text-blue-600 dark:text-blue-400"></i>
      Backup Configuration
    </h2>

    <div class="space-y-6">
      <div
        class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-xl"
      >
        <div>
          <span
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
            >Automatic Backups</span
          >
          <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
            Automatically backup your data at scheduled intervals
          </p>
        </div>
        <div class="flex items-center gap-3">
          <span
            class="text-xs font-medium {autoBackup
              ? 'text-blue-600 dark:text-blue-400'
              : 'text-gray-400 dark:text-gray-600'}"
          >
            {autoBackup ? "ON" : "OFF"}
          </span>
          <button
            role="switch"
            aria-checked={autoBackup}
            class="relative inline-flex h-7 w-12 rounded-full transition-colors {autoBackup
              ? 'bg-blue-600'
              : 'bg-gray-300 dark:bg-gray-700'}"
            onclick={() => (autoBackup = !autoBackup)}
          >
            <span
              class="inline-block h-6 w-6 rounded-full bg-white shadow transition {autoBackup
                ? 'translate-x-5'
                : 'translate-x-0.5'} mt-0.5"
            ></span>
          </button>
        </div>
      </div>

      <div class="space-y-2">
        <label
          class="block text-sm font-medium text-gray-700 dark:text-gray-300"
          for="backup-frequency"
        >
          Backup Frequency
        </label>
        <select
          id="backup-frequency"
          class="w-full px-4 py-2 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 outline-none text-gray-900 dark:text-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
          bind:value={backupFrequency}
          disabled={!autoBackup}
        >
          <option value="hourly">Every Hour</option>
          <option value="daily">Daily</option>
          <option value="weekly">Weekly</option>
          <option value="monthly">Monthly</option>
        </select>
      </div>

      <div
        class="bg-gradient-to-br from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-xl p-6 border border-blue-200 dark:border-blue-800"
      >
        <div class="flex items-center gap-6">
          <div class="text-blue-600 dark:text-blue-400">
            <i class="bi bi-clock-history text-5xl"></i>
          </div>
          <div class="flex-1">
            <div
              class="text-sm text-gray-600 dark:text-gray-400 font-medium mb-1"
            >
              Last Backup
            </div>
            <div class="text-3xl font-bold text-blue-600 dark:text-blue-400">
              2 hours ago
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-500 mt-1">
              Next backup in 22 hours
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="flex justify-end mt-6">
      <button
        class="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-xl hover:from-blue-700 hover:to-purple-700 transition-all flex items-center gap-2 shadow-lg shadow-blue-500/25 text-sm"
        onclick={handleBackupNow}
      >
        <i class="bi bi-cloud-arrow-up"></i>
        Backup Now
      </button>
    </div>
  </div>

  <!-- Backup History -->
  <div
    class="bg-white dark:bg-slate-900 rounded-2xl shadow-xl hover:shadow-2xl transition-shadow p-6"
  >
    <h2 class="text-xl font-bold mb-6 flex items-center gap-2">
      <i class="bi bi-archive-fill text-purple-600 dark:text-purple-400"></i>
      Backup History
    </h2>

    <div class="space-y-3">
      {#each [{ date: "24.10.2025 02:00", size: "1.2 GB", status: "success" }, { date: "23.10.2025 02:00", size: "1.1 GB", status: "success" }, { date: "22.10.2025 02:00", size: "1.0 GB", status: "success" }] as backup}
        <div
          class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-750 transition-colors"
        >
          <div class="flex items-center gap-3">
            <i
              class="bi bi-check-circle-fill text-green-600 dark:text-green-400 text-xl"
            ></i>
            <div>
              <div class="font-semibold text-gray-900 dark:text-gray-100">
                {backup.date}
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                {backup.size}
              </div>
            </div>
          </div>
          <div class="flex gap-2">
            <button
              class="p-2 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors"
            >
              <i class="bi bi-download"></i>
            </button>
            <button
              class="p-2 text-red-600 dark:text-red-400 hover:bg-red-100 dark:hover:bg-red-900/30 rounded-lg transition-colors"
            >
              <i class="bi bi-trash"></i>
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
