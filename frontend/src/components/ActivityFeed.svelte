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

<div class="activity-feed">
  <!-- Header -->
  <div class="feed-header">
    <div class="header-icon">
      <i class="bi bi-activity"></i>
    </div>
    <div class="header-text">
      <h3>Activity Feed</h3>
      <p>Real-time file operations</p>
    </div>
  </div>

  <!-- Activity List -->
  <div class="feed-content">
    {#if activity.length === 0}
      <div class="empty-state">
        <div class="empty-icon">
          <i class="bi bi-clock-history"></i>
        </div>
        <p class="empty-title">No recent activity</p>
        <p class="empty-desc">File operations will appear here</p>
      </div>
    {:else}
      {#each activity as item (item.id)}
        <div class="activity-item">
          <div class="item-icon {getEventColor(item.kind)}">
            <i class={getEventIcon(item.kind)}></i>
          </div>
          <div class="item-content">
            <div class="item-header">
              <span class="item-kind">{item.kind || "change"}</span>
              <span class="item-time">{item.time}</span>
            </div>
            <p class="item-file" title={item.path}>
              {item.path?.split("/").pop() || item.path}
            </p>
            {#if item.path?.includes("/")}
              <p class="item-path" title={item.path}>{item.path}</p>
            {/if}
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <!-- Footer -->
  <div class="feed-footer">
    <span class="event-count">
      <i class="bi bi-clock-history"></i>
      {activity.length} recent events
    </span>
    {#if activity.length > 0}
      <button class="clear-btn" onclick={clearActivity}>Clear all</button>
    {/if}
  </div>
</div>

<style>
  .activity-feed {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #f9fafb;
  }

  :global(.dark) .activity-feed {
    background: #111827;
  }

  /* Header */
  .feed-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    color: white;
  }

  .header-icon {
    width: 40px;
    height: 40px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
  }

  .header-text h3 {
    font-size: 1rem;
    font-weight: 600;
    margin: 0;
  }

  .header-text p {
    font-size: 0.75rem;
    margin: 0;
    opacity: 0.85;
  }

  /* Content */
  .feed-content {
    flex: 1;
    overflow-y: auto;
    padding: 0.75rem;
  }

  .feed-content::-webkit-scrollbar {
    width: 4px;
  }

  .feed-content::-webkit-scrollbar-thumb {
    background: rgba(34, 197, 94, 0.3);
    border-radius: 2px;
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 2rem;
    text-align: center;
  }

  .empty-icon {
    width: 64px;
    height: 64px;
    background: #f3f4f6;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 1rem;
  }

  :global(.dark) .empty-icon {
    background: #1f2937;
  }

  .empty-icon i {
    font-size: 1.75rem;
    color: #9ca3af;
  }

  .empty-title {
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
    margin: 0 0 0.25rem;
  }

  .empty-desc {
    font-size: 0.75rem;
    color: #9ca3af;
    margin: 0;
  }

  /* Activity Item */
  .activity-item {
    display: flex;
    gap: 0.75rem;
    padding: 0.75rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    margin-bottom: 0.5rem;
    transition: all 0.2s ease;
    animation: slideIn 0.3s ease-out;
  }

  :global(.dark) .activity-item {
    background: #1f2937;
    border-color: #374151;
  }

  .activity-item:hover {
    border-color: #22c55e;
    box-shadow: 0 2px 8px rgba(34, 197, 94, 0.1);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(20px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .item-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
    flex-shrink: 0;
  }

  .item-icon.green {
    background: #dcfce7;
    color: #22c55e;
  }
  .item-icon.blue {
    background: #dbeafe;
    color: #3b82f6;
  }
  .item-icon.red {
    background: #fee2e2;
    color: #ef4444;
  }
  .item-icon.purple {
    background: #f3e8ff;
    color: #a855f7;
  }
  .item-icon.amber {
    background: #fef3c7;
    color: #f59e0b;
  }

  :global(.dark) .item-icon.green {
    background: rgba(34, 197, 94, 0.2);
  }
  :global(.dark) .item-icon.blue {
    background: rgba(59, 130, 246, 0.2);
  }
  :global(.dark) .item-icon.red {
    background: rgba(239, 68, 68, 0.2);
  }
  :global(.dark) .item-icon.purple {
    background: rgba(168, 85, 247, 0.2);
  }
  :global(.dark) .item-icon.amber {
    background: rgba(245, 158, 11, 0.2);
  }

  .item-content {
    flex: 1;
    min-width: 0;
  }

  .item-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.25rem;
  }

  .item-kind {
    font-size: 0.6875rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #111827;
  }

  :global(.dark) .item-kind {
    color: #f9fafb;
  }

  .item-time {
    font-size: 0.6875rem;
    color: #9ca3af;
  }

  .item-file {
    font-size: 0.8125rem;
    font-weight: 500;
    color: #374151;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark) .item-file {
    color: #d1d5db;
  }

  .item-path {
    font-size: 0.6875rem;
    color: #6b7280;
    margin: 0.125rem 0 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark) .item-path {
    color: #9ca3af;
  }

  /* Footer */
  .feed-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-top: 1px solid #e5e7eb;
    background: #f9fafb;
  }

  :global(.dark) .feed-footer {
    background: #111827;
    border-top-color: #374151;
  }

  .event-count {
    font-size: 0.75rem;
    color: #6b7280;
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  :global(.dark) .event-count {
    color: #9ca3af;
  }

  .clear-btn {
    font-size: 0.75rem;
    font-weight: 500;
    color: #22c55e;
    background: none;
    border: none;
    cursor: pointer;
    transition: color 0.2s ease;
  }

  .clear-btn:hover {
    color: #16a34a;
  }
</style>
