<script>
  import { onMount } from "svelte";
  import api from "../../lib/api";

  let storageStats = $state({
    used: 0,
    total: 0,
    percentage: 0,
    byType: [],
  });
  let loading = $state(true);

  onMount(async () => {
    await loadStorageStats();
  });

  async function loadStorageStats() {
    try {
      loading = true;
      const data = await api.system.storage();

      // Transform backend response to UI format
      const used = data.used_bytes || 0;
      const total = data.total_bytes || 10 * 1024 * 1024 * 1024; // Default 10GB if not set
      const percentage = total > 0 ? Math.round((used / total) * 100) : 0;

      // Group files by type
      const byType = [];
      if (data.by_type) {
        const typeColors = {
          Documents: "bg-teal-500",
          Images: "bg-green-500",
          Videos: "bg-purple-500",
          Audio: "bg-yellow-500",
          Archives: "bg-orange-500",
          Other: "bg-gray-500",
        };

        for (const [type, info] of Object.entries(data.by_type)) {
          byType.push({
            type,
            size: info.total_bytes || 0,
            count: info.file_count || 0,
            color: typeColors[type] || "bg-gray-500",
          });
        }
      }

      storageStats = {
        used,
        total,
        percentage,
        byType:
          byType.length > 0
            ? byType
            : [
                { type: "Documents", size: 0, count: 0, color: "bg-teal-500" },
                { type: "Images", size: 0, count: 0, color: "bg-green-500" },
                { type: "Videos", size: 0, count: 0, color: "bg-purple-500" },
                { type: "Other", size: 0, count: 0, color: "bg-gray-500" },
              ],
      };
    } catch (error) {
      console.error("Failed to load storage stats:", error);
      // Fallback to empty data
      storageStats = {
        used: 0,
        total: 10 * 1024 * 1024 * 1024,
        percentage: 0,
        byType: [
          { type: "Documents", size: 0, count: 0, color: "bg-teal-500" },
          { type: "Images", size: 0, count: 0, color: "bg-green-500" },
          { type: "Videos", size: 0, count: 0, color: "bg-purple-500" },
          { type: "Other", size: 0, count: 0, color: "bg-gray-500" },
        ],
      };
    } finally {
      loading = false;
    }
  }

  function formatBytes(bytes) {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }
</script>

<div class="container mx-auto px-4 py-8 max-w-6xl page-fade-in">
  <div class="mb-8">
    <h1
      class="text-3xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-3"
    >
      <i
        class="bi bi-hdd text-primary-600 dark:text-primary-400"
        aria-hidden="true"
      ></i>
      Storage Management
    </h1>
    <p class="text-gray-600 dark:text-gray-400 mt-2">
      Monitor and manage your storage usage
    </p>
  </div>

  {#if loading}
    <div class="space-y-6">
      <div class="skeleton h-64 w-full rounded-2xl"></div>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        {#each Array(3) as _}
          <div class="skeleton h-32 w-full rounded-xl"></div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="space-y-6">
      <!-- Storage Overview -->
      <div
        class="card bg-white dark:bg-gray-800 shadow-xl rounded-2xl hover-scale"
      >
        <div class="card-body">
          <h2 class="card-title text-xl mb-6">
            <i class="bi bi-pie-chart text-primary-600" aria-hidden="true"></i>
            Storage Overview
          </h2>

          <div class="text-center mb-8">
            <div
              class="radial-progress text-primary bounce-in"
              style="--value:{storageStats.percentage}; --size:12rem; --thickness:1rem;"
            >
              <div class="text-center">
                <div class="text-4xl font-bold">{storageStats.percentage}%</div>
                <div class="text-sm text-gray-600 dark:text-gray-400">Used</div>
              </div>
            </div>
            <div class="mt-6">
              <p class="text-2xl font-bold text-gray-900 dark:text-gray-100">
                {formatBytes(storageStats.used)} / {formatBytes(
                  storageStats.total
                )}
              </p>
              <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                {formatBytes(storageStats.total - storageStats.used)} available
              </p>
            </div>
          </div>

          <!-- Progress Bar -->
          <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-4">
            <div
              class="bg-gradient-to-r from-primary-600 to-primary-400 h-4 rounded-full transition-all duration-500"
              style="width: {storageStats.percentage}%"
            ></div>
          </div>
        </div>
      </div>

      <!-- Storage by Type -->
      <div
        class="card bg-white dark:bg-gray-800 shadow-xl rounded-2xl hover-scale"
      >
        <div class="card-body">
          <h2 class="card-title text-xl mb-6">
            <i class="bi bi-list-ul text-primary-600" aria-hidden="true"></i>
            Storage by File Type
          </h2>

          <div class="space-y-4 list-stagger">
            {#each storageStats.byType as item}
              <div class="flex items-center gap-4">
                <div class="flex-1">
                  <div class="flex items-center justify-between mb-2">
                    <span
                      class="font-semibold text-gray-900 dark:text-gray-100"
                    >
                      {item.type}
                    </span>
                    <span class="text-sm text-gray-600 dark:text-gray-400">
                      {formatBytes(item.size)} • {item.count} files
                    </span>
                  </div>
                  <div
                    class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2"
                  >
                    <div
                      class="{item.color} h-2 rounded-full transition-all duration-500"
                      style="width: {(item.size / storageStats.used) * 100}%"
                    ></div>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Storage Actions -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <button class="btn btn-outline btn-lg rounded-xl">
          <i class="bi bi-trash3" aria-hidden="true"></i>
          Clean Up Storage
        </button>
        <button class="btn btn-outline btn-lg rounded-xl">
          <i class="bi bi-download" aria-hidden="true"></i>
          Export Storage Report
        </button>
      </div>
    </div>
  {/if}
</div>
