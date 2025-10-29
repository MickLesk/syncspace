<script>
  import { onMount } from "svelte";
  import { onFileEvent } from "../../stores/websocket.js";
  let activity = [];

  onMount(() => {
    onFileEvent((event) => {
      activity.unshift({
        ...event,
        time: new Date().toLocaleTimeString(),
      });
      if (activity.length > 100) activity.pop();
    });
  });
</script>

<div
  class="w-72 bg-base-100 border-l border-base-200 h-full overflow-y-auto p-4"
>
  <h3 class="font-bold mb-4 text-lg">Activity Feed</h3>
  <ul class="space-y-2">
    {#each activity as item}
      <li class="text-xs text-gray-700 dark:text-gray-300">
        <span class="font-semibold">[{item.time}]</span>
        {item.kind} <span class="text-blue-600">{item.path}</span>
      </li>
    {/each}
  </ul>
</div>
