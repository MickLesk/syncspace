<script>
  import { onMount } from "svelte";
  import { websocketManager } from "@stores/websocket.js";
  let activity = [];
  let isCollapsed = $state(false);

  onMount(() => {
    websocketManager.on("file_change", (event) => {
      activity.unshift({
        ...event,
        time: new Date().toLocaleTimeString(),
      });
      if (activity.length > 100) activity.pop();
    });

    // Auto-collapse on mobile
    if (window.innerWidth < 768) {
      isCollapsed = true;
    }
  });

  function getEventIcon(kind) {
    const icons = {
      create: "📄",
      modify: "✏️",
      delete: "🗑️",
      rename: "✒️",
      move: "📦",
    };
    return icons[kind] || "📁";
  }
</script>

<div
  class="activity-feed bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 h-full overflow-hidden transition-all duration-300"
  class:collapsed={isCollapsed}
>
  <div
    class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700"
  >
    <h3 class="font-bold text-lg text-gray-900 dark:text-gray-100">
      {#if !isCollapsed}Activity Feed{:else}Activity{/if}
    </h3>
    <button
      onclick={() => (isCollapsed = !isCollapsed)}
      class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded transition-colors"
      aria-label={isCollapsed
        ? "Expand activity feed"
        : "Collapse activity feed"}
    >
      <span class="text-xl">{isCollapsed ? "◀" : "▶"}</span>
    </button>
  </div>

  {#if !isCollapsed}
    <div class="overflow-y-auto p-4" style="max-height: calc(100% - 64px);">
      {#if activity.length === 0}
        <p class="text-sm text-gray-500 dark:text-gray-400 text-center py-8">
          No recent activity
        </p>
      {:else}
        <ul class="space-y-3">
          {#each activity as item}
            <li
              class="text-sm bg-gray-50 dark:bg-gray-900 p-3 rounded-lg border border-gray-200 dark:border-gray-700"
            >
              <div class="flex items-start gap-2">
                <span class="text-xl">{getEventIcon(item.kind)}</span>
                <div class="flex-1 min-w-0">
                  <p
                    class="font-semibold text-gray-900 dark:text-gray-100 text-xs mb-1"
                  >
                    {item.kind || "change"}
                  </p>
                  <p
                    class="text-xs text-blue-600 dark:text-blue-400 truncate"
                    title={item.path}
                  >
                    {item.path}
                  </p>
                  <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {item.time}
                  </p>
                </div>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}
</div>

<style>
  .activity-feed {
    width: 20rem;
  }

  .activity-feed.collapsed {
    width: 3rem;
  }

  @media (max-width: 768px) {
    .activity-feed {
      position: absolute;
      right: 0;
      top: 0;
      z-index: 20;
      width: 16rem;
      box-shadow: -4px 0 6px rgba(0, 0, 0, 0.1);
    }

    .activity-feed.collapsed {
      width: 0;
      border: none;
    }
  }
</style>
