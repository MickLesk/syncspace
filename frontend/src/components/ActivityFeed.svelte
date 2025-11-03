<script>
  import { onMount } from "svelte";
  import { websocketManager } from "@stores/websocket.js";

  let activity = [];

  onMount(() => {
    websocketManager.on("file_change", (event) => {
      activity.unshift({
        ...event,
        time: new Date().toLocaleTimeString(),
        id: Date.now() + Math.random(), // Unique ID for animations
      });
      if (activity.length > 50) activity.pop();
    });
  });

  function getEventIcon(kind) {
    const icons = {
      create: "bi-file-plus",
      modify: "bi-pencil",
      delete: "bi-trash",
      rename: "bi-cursor-text",
      move: "bi-folder-symlink",
    };
    return icons[kind] || "bi-file-earmark";
  }

  function getEventColor(kind) {
    const colors = {
      create: "text-green-500 bg-green-500/10",
      modify: "text-blue-500 bg-blue-500/10",
      delete: "text-red-500 bg-red-500/10",
      rename: "text-purple-500 bg-purple-500/10",
      move: "text-orange-500 bg-orange-500/10",
    };
    return colors[kind] || "text-gray-500 bg-gray-500/10";
  }
</script>

<div class="activity-feed-panel h-full flex flex-col">
  <!-- Modern Header with Gradient -->
  <div
    class="feed-header px-6 py-4 bg-gradient-to-r from-primary-500 to-secondary-500 text-white"
  >
    <div class="flex items-center gap-3">
      <div class="p-2 bg-white/20 rounded-lg backdrop-blur-sm">
        <i class="bi bi-activity text-xl"></i>
      </div>
      <div>
        <h3 class="font-bold text-lg">Activity Feed</h3>
        <p class="text-xs text-white/80">Real-time file operations</p>
      </div>
    </div>
  </div>

  <!-- Activity List with Smooth Scroll -->
  <div class="flex-1 overflow-y-auto p-4 space-y-2 custom-scrollbar">
    {#if activity.length === 0}
      <div
        class="flex flex-col items-center justify-center h-full text-center px-4"
      >
        <div
          class="w-20 h-20 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center mb-4"
        >
          <i
            class="bi bi-clock-history text-3xl text-gray-400 dark:text-gray-500"
          ></i>
        </div>
        <p class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-1">
          No recent activity
        </p>
        <p class="text-xs text-gray-500 dark:text-gray-500">
          File operations will appear here
        </p>
      </div>
    {:else}
      {#each activity as item (item.id)}
        <div
          class="activity-item group hover:scale-[1.02] transition-all duration-200"
          style="animation: slideInRight 0.3s ease-out;"
        >
          <div
            class="flex items-start gap-3 p-3 rounded-xl bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-all duration-200"
          >
            <!-- Icon with colored background -->
            <div
              class="flex-shrink-0 w-10 h-10 rounded-lg {getEventColor(
                item.kind
              )} flex items-center justify-center"
            >
              <i class="{getEventIcon(item.kind)} text-lg"></i>
            </div>

            <!-- Content -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 mb-1">
                <span
                  class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-gray-100"
                >
                  {item.kind || "change"}
                </span>
                <span class="text-xs text-gray-400 dark:text-gray-500">
                  {item.time}
                </span>
              </div>
              <p
                class="text-sm text-gray-700 dark:text-gray-300 truncate font-medium"
                title={item.path}
              >
                {item.path?.split("/").pop() || item.path}
              </p>
              {#if item.path?.includes("/")}
                <p
                  class="text-xs text-gray-500 dark:text-gray-400 truncate mt-0.5"
                  title={item.path}
                >
                  {item.path}
                </p>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <!-- Footer with Stats -->
  <div
    class="feed-footer px-4 py-3 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900"
  >
    <div class="flex items-center justify-between text-xs">
      <span class="text-gray-600 dark:text-gray-400">
        <i class="bi bi-clock-history"></i>
        {activity.length} recent events
      </span>
      {#if activity.length > 0}
        <button
          onclick={() => (activity = [])}
          class="text-primary-500 hover:text-primary-600 dark:text-primary-400 dark:hover:text-primary-300 font-medium transition-colors"
        >
          Clear all
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .activity-feed-panel {
    background: rgb(249 250 251);
  }

  :global(.dark) .activity-feed-panel {
    background: rgb(17 24 39);
  }

  /* Custom Scrollbar */
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }

  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }

  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(156, 163, 175, 0.3);
    border-radius: 3px;
  }

  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(156, 163, 175, 0.5);
  }

  :global(.dark) .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
  }

  :global(.dark) .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  /* Slide In Animation */
  @keyframes slideInRight {
    from {
      opacity: 0;
      transform: translateX(20px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
</style>
