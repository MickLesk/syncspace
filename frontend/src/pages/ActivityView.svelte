<script>
  import { onMount } from "svelte";
  import { activity } from "../stores/activity";
  import Icon from "../components/ui/Icon.svelte";

  let groupedActivities = {};
  let selectedFilter = "all";
  let searchQuery = "";

  $: filteredActivities = filterActivities(
    $activity,
    selectedFilter,
    searchQuery
  );
  $: groupedActivities = groupByDate(filteredActivities);

  onMount(async () => {
    // Load activities from backend
    await activity.load({ limit: 100 });
    
    // Cleanup old localStorage data if exists
    const oldKey = 'syncspace_activity';
    if (localStorage.getItem(oldKey)) {
      localStorage.removeItem(oldKey);
      console.log('Cleaned up old localStorage activities');
    }
  });

  /**
   * Filter activities by type and search
   */
  function filterActivities(activities, filter, search) {
    let filtered = activities;

    // Filter by type
    if (filter !== "all") {
      filtered = filtered.filter((a) => a.type === filter);
    }

    // Filter by search query
    if (search.trim()) {
      const query = search.toLowerCase();
      filtered = filtered.filter(
        (a) =>
          a.filename.toLowerCase().includes(query) ||
          a.path.toLowerCase().includes(query) ||
          a.details.toLowerCase().includes(query)
      );
    }

    return filtered;
  }

  /**
   * Group activities by date
   */
  function groupByDate(activities) {
    const groups = {};
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    for (const act of activities) {
      const date = new Date(act.timestamp);
      const dateKey = new Date(
        date.getFullYear(),
        date.getMonth(),
        date.getDate()
      );

      let label;
      if (dateKey.getTime() === today.getTime()) {
        label = "Today";
      } else if (dateKey.getTime() === yesterday.getTime()) {
        label = "Yesterday";
      } else {
        label = formatDate(date);
      }

      if (!groups[label]) {
        groups[label] = [];
      }
      groups[label].push(act);
    }

    return groups;
  }

  /**
   * Format date
   */
  function formatDate(date) {
    return date.toLocaleDateString("en-US", {
      weekday: "short",
      month: "short",
      day: "numeric",
      year:
        date.getFullYear() !== new Date().getFullYear() ? "numeric" : undefined,
    });
  }

  /**
   * Format time
   */
  function formatTime(timestamp) {
    return new Date(timestamp).toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  /**
   * Get relative time
   */
  function getRelativeTime(timestamp) {
    const now = Date.now();
    const diff = now - timestamp;

    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return "Just now";
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    if (days < 7) return `${days}d ago`;
    return formatDate(new Date(timestamp));
  }

  /**
   * Get activity type label
   */
  function getTypeLabel(type) {
    const labels = {
      upload: "Uploaded",
      download: "Downloaded",
      delete: "Deleted",
      rename: "Renamed",
      create: "Created",
      move: "Moved",
      share: "Shared",
      favorite: "Favorited",
      unfavorite: "Unfavorited",
    };
    return labels[type] || type;
  }

  /**
   * Clear all activities
   */
  function handleClearAll() {
    if (confirm("Clear all activity history?")) {
      activity.clear();
    }
  }

  /**
   * Get activity type color
   */
  function getTypeColor(type) {
    const colors = {
      upload: "#4CAF50",
      download: "#2196F3",
      delete: "#F44336",
      rename: "#FF9800",
      create: "#9C27B0",
      move: "#00BCD4",
      share: "#FF5722",
      favorite: "#FFC107",
      unfavorite: "#9E9E9E",
    };
    return colors[type] || "#607D8B";
  }
</script>

<div class="activity-timeline">
  <div class="timeline-header">
    <h2><Icon name="clock-history" size={24} /> Activity Timeline</h2>
    <button
      class="btn-clear"
      on:click={handleClearAll}
      disabled={$activity.length === 0}
    >
      <Icon name="trash" size={16} />
      Clear All
    </button>
  </div>

  <!-- Filters -->
  <div class="filter-bar">
    <div class="filter-chips">
      <button
        class="filter-chip"
        class:active={selectedFilter === "all"}
        on:click={() => (selectedFilter = "all")}
      >
        All
      </button>
      <button
        class="filter-chip"
        class:active={selectedFilter === "upload"}
        on:click={() => (selectedFilter = "upload")}
      >
        <span style="color: {getTypeColor('upload')}">⬆️</span> Uploads
      </button>
      <button
        class="filter-chip"
        class:active={selectedFilter === "download"}
        on:click={() => (selectedFilter = "download")}
      >
        <span style="color: {getTypeColor('download')}">⬇️</span> Downloads
      </button>
      <button
        class="filter-chip"
        class:active={selectedFilter === "delete"}
        on:click={() => (selectedFilter = "delete")}
      >
        <span style="color: {getTypeColor('delete')}">🗑️</span> Deletes
      </button>
      <button
        class="filter-chip"
        class:active={selectedFilter === "move"}
        on:click={() => (selectedFilter = "move")}
      >
        <span style="color: {getTypeColor('move')}">📦</span> Moves
      </button>
    </div>

    <input
      type="text"
      class="search-input"
      placeholder="Search activities..."
      bind:value={searchQuery}
    />
  </div>

  <!-- Stats Summary -->
  {#if $activity.length > 0}
    <div class="stats-summary">
      <div class="stat-item">
        <span class="stat-label">Total</span>
        <span class="stat-value">{$activity.length}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Today</span>
        <span class="stat-value">{activity.getToday().length}</span>
      </div>
    </div>
  {/if}

  <!-- Timeline Content -->
  <div class="timeline-content">
    {#if Object.keys(groupedActivities).length === 0}
      <div class="empty-state">
        <Icon name="clock-history" size={64} />
        <h3>No Activity Yet</h3>
        <p>File operations will appear here</p>
      </div>
    {:else}
      {#each Object.entries(groupedActivities) as [dateLabel, activities]}
        <div class="date-group">
          <div class="date-label">{dateLabel}</div>
          <div class="timeline-items">
            {#each activities as act}
              <div
                class="timeline-item"
                style="--accent-color: {getTypeColor(act.type)}"
              >
                <div class="timeline-marker">
                  <span class="activity-icon">{act.icon}</span>
                </div>
                <div class="timeline-content-item">
                  <div class="activity-header">
                    <span class="activity-type">{getTypeLabel(act.type)}</span>
                    <span class="activity-time"
                      >{formatTime(act.timestamp)}</span
                    >
                  </div>
                  <div class="activity-filename">{act.filename}</div>
                  {#if act.path}
                    <div class="activity-path">{act.path}</div>
                  {/if}
                  {#if act.details}
                    <div class="activity-details">{act.details}</div>
                  {/if}
                  <div class="activity-relative">
                    {getRelativeTime(act.timestamp)}
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .activity-timeline {
    padding: 32px;
    max-width: 1200px;
    margin: 0 auto;
  }

  .timeline-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 32px;
  }

  .timeline-header h2 {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 28px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin: 0;
  }

  .btn-clear {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 20px;
    border-radius: 20px;
    border: 1px solid var(--md-sys-color-outline);
    background: transparent;
    color: var(--md-sys-color-error);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-clear:hover:not(:disabled) {
    background: var(--md-sys-color-error-container);
    border-color: var(--md-sys-color-error);
  }

  .btn-clear:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .filter-bar {
    display: flex;
    gap: 16px;
    margin-bottom: 24px;
    flex-wrap: wrap;
  }

  .filter-chips {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    flex: 1;
  }

  .filter-chip {
    padding: 8px 16px;
    border-radius: 16px;
    border: 1px solid var(--md-sys-color-outline);
    background: transparent;
    color: var(--md-sys-color-on-surface);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .filter-chip:hover {
    background: var(--md-sys-color-surface-variant);
  }

  .filter-chip.active {
    background: var(--md-sys-color-primary-container);
    border-color: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary-container);
  }

  .search-input {
    padding: 10px 16px;
    border-radius: 20px;
    border: 1px solid var(--md-sys-color-outline);
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-size: 14px;
    min-width: 200px;
    transition: all 0.2s;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--md-sys-color-primary);
    box-shadow: 0 0 0 3px var(--md-sys-color-primary-container);
  }

  .stats-summary {
    display: flex;
    gap: 16px;
    margin-bottom: 24px;
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 16px 24px;
    background: var(--md-sys-color-surface);
    border-radius: 16px;
    box-shadow: var(--md-elevation-1);
  }

  .stat-label {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  .stat-value {
    font-size: 24px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .timeline-content {
    position: relative;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 64px 32px;
    color: var(--md-sys-color-on-surface-variant);
    text-align: center;
  }

  .empty-state h3 {
    font-size: 20px;
    font-weight: 500;
    margin: 16px 0 8px 0;
    color: var(--md-sys-color-on-surface);
  }

  .date-group {
    margin-bottom: 32px;
  }

  .date-label {
    font-size: 14px;
    font-weight: 600;
    color: var(--md-sys-color-primary);
    margin-bottom: 16px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .timeline-items {
    position: relative;
    padding-left: 32px;
    border-left: 2px solid var(--md-sys-color-outline-variant);
  }

  .timeline-item {
    position: relative;
    margin-bottom: 24px;
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-20px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .timeline-marker {
    position: absolute;
    left: -44px;
    top: 0;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: var(--accent-color, var(--md-sys-color-primary));
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 0 0 4px var(--md-sys-color-surface);
  }

  .activity-icon {
    font-size: 16px;
  }

  .timeline-content-item {
    background: var(--md-sys-color-surface);
    border-radius: 12px;
    padding: 16px;
    box-shadow: var(--md-elevation-1);
    transition: all 0.2s;
  }

  .timeline-item:hover .timeline-content-item {
    box-shadow: var(--md-elevation-2);
    transform: translateX(4px);
  }

  .activity-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .activity-type {
    font-size: 12px;
    font-weight: 600;
    color: var(--accent-color, var(--md-sys-color-primary));
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .activity-time {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .activity-filename {
    font-size: 16px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 4px;
  }

  .activity-path {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    font-family: "Courier New", monospace;
    margin-bottom: 4px;
  }

  .activity-details {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    margin-top: 8px;
  }

  .activity-relative {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    margin-top: 8px;
    font-style: italic;
  }
</style>
