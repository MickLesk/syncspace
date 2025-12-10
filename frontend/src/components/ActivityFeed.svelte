<script>
  import { onMount } from "svelte";
  import { websocketManager } from "@stores/websocket.js";

  let activity = $state([]);

  onMount(() => {
    websocketManager.on("file_change", (event) => {
      activity = [
        {
          ...event,
          time: new Date().toLocaleTimeString(),
          id: Date.now() + Math.random(),
        },
        ...activity.slice(0, 49),
      ];
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
      create: "green",
      modify: "blue",
      delete: "red",
      rename: "purple",
      move: "amber",
    };
    return colors[kind] || "gray";
  }

  function clearActivity() {
    activity = [];
  }
</script>

<div class="flex flex-col h-full bg-gray-50 dark:bg-gray-900">
  <!-- Header -->
  <div
    class="flex items-center gap-3 px-5 py-4 bg-gradient-to-r from-green-500 to-green-600 text-white"
  >
    <div
      class="w-10 h-10 bg-white/20 rounded-lg flex items-center justify-center text-xl"
    >
      <i class="bi bi-activity"></i>
    </div>
    <div>
      <h3 class="text-base font-semibold m-0">Activity Feed</h3>
      <p class="text-xs m-0 opacity-85">Real-time file operations</p>
    </div>
  </div>

  <!-- Activity List -->
  <div
    class="flex-1 overflow-y-auto p-3 scrollbar-thin scrollbar-thumb-green-500/30"
  >
    {#if activity.length === 0}
      <div
        class="flex flex-col items-center justify-center h-full p-8 text-center"
      >
        <div
          class="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-full flex items-center justify-center mb-4"
        >
          <i class="bi bi-clock-history text-3xl text-gray-400"></i>
        </div>
        <p class="text-sm font-medium text-gray-500 m-0 mb-1">
          No recent activity
        </p>
        <p class="text-xs text-gray-400 m-0">
          File operations will appear here
        </p>
      </div>
    {:else}
      {#each activity as item (item.id)}
        <div
          class="flex gap-3 p-3 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg mb-2 transition-all duration-200 hover:border-green-500 hover:shadow-[0_2px_8px_rgba(34,197,94,0.1)] animate-slideIn"
        >
          <div
            class="w-9 h-9 rounded-lg flex items-center justify-center text-base shrink-0 {getEventColor(
              item.kind
            ) === 'green'
              ? 'bg-green-100 dark:bg-green-500/20 text-green-500'
              : getEventColor(item.kind) === 'blue'
                ? 'bg-blue-100 dark:bg-blue-500/20 text-blue-500'
                : getEventColor(item.kind) === 'red'
                  ? 'bg-red-100 dark:bg-red-500/20 text-red-500'
                  : getEventColor(item.kind) === 'purple'
                    ? 'bg-purple-100 dark:bg-purple-500/20 text-purple-500'
                    : 'bg-amber-100 dark:bg-amber-500/20 text-amber-500'}"
          >
            <i class={getEventIcon(item.kind)}></i>
          </div>
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <span
                class="text-[0.6875rem] font-bold uppercase tracking-wide text-gray-900 dark:text-gray-50"
                >{item.kind || "change"}</span
              >
              <span class="text-[0.6875rem] text-gray-400">{item.time}</span>
            </div>
            <p
              class="text-[0.8125rem] font-medium text-gray-700 dark:text-gray-300 m-0 overflow-hidden text-ellipsis whitespace-nowrap"
              title={item.path}
            >
              {item.path?.split("/").pop() || item.path}
            </p>
            {#if item.path?.includes("/")}
              <p
                class="text-[0.6875rem] text-gray-500 dark:text-gray-400 mt-0.5 mb-0 overflow-hidden text-ellipsis whitespace-nowrap"
                title={item.path}
              >
                {item.path}
              </p>
            {/if}
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <!-- Footer -->
  <div
    class="flex items-center justify-between px-4 py-3 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900"
  >
    <span
      class="text-xs text-gray-500 dark:text-gray-400 flex items-center gap-1.5"
    >
      <i class="bi bi-clock-history"></i>
      {activity.length} recent events
    </span>
    {#if activity.length > 0}
      <button
        class="text-xs font-medium text-green-500 bg-transparent border-none cursor-pointer transition-colors duration-200 hover:text-green-600"
        onclick={clearActivity}>Clear all</button
      >
    {/if}
  </div>
</div>
