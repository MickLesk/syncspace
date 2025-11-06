<script>
  import { success } from "../../stores/toast";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let autoBackup = $state(true);
  let backupFrequency = $state("daily");

  function handleBackupNow() {
    success(tr("backupInProgress"));
  }
</script>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <!-- Backup Configuration -->
  <ModernCard variant="glass" hoverable>
    <div class="p-6">
      <h2
        class="text-xl font-bold mb-6 flex items-center gap-2 text-gray-900 dark:text-gray-100"
      >
        <i
          class="bi bi-cloud-arrow-up-fill text-primary-600 dark:text-primary-400"
        ></i>
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
            <span class="badge-glass-{autoBackup ? 'success' : 'error'}">
              {autoBackup ? "ON" : "OFF"}
            </span>
            <button
              role="switch"
              aria-checked={autoBackup}
              class="relative inline-flex h-7 w-12 rounded-full transition-colors {autoBackup
                ? 'bg-primary-600'
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
            class="glass-input w-full disabled:opacity-50"
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
        <ModernButton variant="gradient" onclick={handleBackupNow}>
          <i class="bi bi-cloud-arrow-up mr-2"></i>
          Backup Now
        </ModernButton>
      </div>
    </div>
  </ModernCard>

  <!-- Backup History -->
  <ModernCard variant="glass" hoverable>
    <div class="p-6">
      <h2
        class="text-xl font-bold mb-6 flex items-center gap-2 text-gray-900 dark:text-gray-100"
      >
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
              <ModernButton variant="ghost" size="sm">
                <i class="bi bi-download"></i>
              </ModernButton>
              <ModernButton variant="danger" size="sm">
                <i class="bi bi-trash"></i>
              </ModernButton>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </ModernCard>
</div>
