<script>
  import { success } from "../../stores/toast";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let storageLocation = $state("./data");
  let maxFileSize = $state("100");

  function handleClearCache() {
    success(tr("cacheCleared"));
  }
</script>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <!-- Storage Configuration -->
  <ModernCard variant="glass" hoverable>
    <div class="p-6">
      <h2
        class="text-xl font-bold mb-6 flex items-center gap-2 text-gray-900 dark:text-gray-100"
      >
        <i class="bi bi-hdd-fill text-primary-600 dark:text-primary-400"></i>
        {tr("manageBucketsDescription")}
      </h2>

      <div class="space-y-6">
        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
            for="storage-location"
          >
            {tr("totalStorage")}
          </label>
          <div
            class="flex items-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl"
          >
            <i class="bi bi-folder-fill text-gray-400 dark:text-gray-600"></i>
            <input
              type="text"
              id="storage-location"
              class="flex-1 bg-transparent outline-none text-gray-900 dark:text-gray-100 disabled:opacity-70"
              bind:value={storageLocation}
              disabled
            />
          </div>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            Path where files are stored
          </p>
        </div>

        <div class="space-y-2">
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300"
            for="max-file-size"
          >
            Max File Size (MB)
          </label>
          <div class="flex items-center gap-2">
            <i
              class="bi bi-file-earmark-arrow-up text-gray-400 dark:text-gray-600"
            ></i>
            <input
              type="number"
              id="max-file-size"
              class="glass-input flex-1"
              bind:value={maxFileSize}
            />
          </div>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            Maximum size for uploaded files
          </p>
        </div>

        <div
          class="flex justify-between items-center p-4 bg-gray-50 dark:bg-gray-800 rounded-xl"
        >
          <div>
            <div class="font-semibold text-gray-900 dark:text-gray-100">
              Cache Size
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              Currently using 245 MB
            </div>
          </div>
          <ModernButton variant="danger" onclick={handleClearCache} size="sm">
            <i class="bi bi-trash-fill mr-1"></i>
            Clear Cache
          </ModernButton>
        </div>
      </div>
    </div>
  </ModernCard>

  <!-- Storage Stats -->
  <ModernCard variant="glass" hoverable>
    <div class="p-6">
      <h2
        class="text-xl font-bold mb-6 flex items-center gap-2 text-gray-900 dark:text-gray-100"
      >
        <i class="bi bi-pie-chart-fill text-purple-600 dark:text-purple-400"
        ></i>
        Storage Usage
      </h2>

      <div class="space-y-4">
        <div
          class="bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 rounded-xl p-6 border border-blue-200 dark:border-blue-800"
        >
          <div class="flex items-center gap-6">
            <div class="text-blue-600 dark:text-blue-400">
              <i class="bi bi-hdd text-5xl"></i>
            </div>
            <div class="flex-1">
              <div
                class="text-sm text-gray-600 dark:text-gray-400 font-medium mb-1"
              >
                Total Space
              </div>
              <div class="text-3xl font-bold text-blue-600 dark:text-blue-400">
                50 GB
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-500 mt-1">
                Allocated storage
              </div>
            </div>
          </div>
        </div>

        <div
          class="bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/20 dark:to-purple-800/20 rounded-xl p-6 border border-purple-200 dark:border-purple-800"
        >
          <div class="flex items-center gap-6">
            <div class="text-purple-600 dark:text-purple-400">
              <i class="bi bi-files text-5xl"></i>
            </div>
            <div class="flex-1">
              <div
                class="text-sm text-gray-600 dark:text-gray-400 font-medium mb-1"
              >
                Used Space
              </div>
              <div
                class="text-3xl font-bold text-purple-600 dark:text-purple-400"
              >
                42.5 GB
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-500 mt-1">
                85% of total space
              </div>
            </div>
          </div>
        </div>

        <div
          class="bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-800/20 rounded-xl p-6 border border-green-200 dark:border-green-800"
        >
          <div class="flex items-center gap-6">
            <div class="text-green-600 dark:text-green-400">
              <i class="bi bi-check-circle text-5xl"></i>
            </div>
            <div class="flex-1">
              <div
                class="text-sm text-gray-600 dark:text-gray-400 font-medium mb-1"
              >
                Available
              </div>
              <div
                class="text-3xl font-bold text-green-600 dark:text-green-400"
              >
                7.5 GB
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-500 mt-1">
                Free space remaining
              </div>
            </div>
          </div>
        </div>

        <div class="mt-6">
          <div
            class="flex justify-between text-sm mb-2 text-gray-700 dark:text-gray-300"
          >
            <span>Storage Usage</span>
            <span class="font-semibold">85%</span>
          </div>
          <div
            class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 overflow-hidden"
          >
            <div
              class="bg-gradient-to-r from-blue-600 to-purple-600 h-3 rounded-full transition-all duration-500"
              style="width: 85%"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </ModernCard>
</div>
