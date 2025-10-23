<script>
  import { onMount } from "svelte";
  import { activity } from "../stores/activity";
  import Icon from "../components/ui/Icon.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import StatCard from "../components/ui/StatCard.svelte";
  import Chip from "../components/ui/Chip.svelte";
  import Button from "../components/ui/Button.svelte";

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

<div class="activity-view">
  <PageHeader 
    title="Activity Feed"
    subtitle=""
    icon="clock-history"
    gradient="blue"
  />

  <div class="page-content">
    <!-- Stats -->
    {#if $activity.length > 0}
      <div class="stats-grid">
        <StatCard
          icon="bi-activity"
          label="Total Events"
          value={$activity.length}
          gradient="linear-gradient(135deg, #6366f1, #8b5cf6)"
        />
        
        <StatCard
          icon="bi-calendar-check"
          label="Today"
          value={activity.getToday().length}
          gradient="linear-gradient(135deg, #10b981, #34d399)"
        />
      </div>
    {/if}

    <!-- Action Bar -->
    <div class="actions">
      <Button onClick={handleClearAll} disabled={$activity.length === 0} variant="danger">
        <Icon name="trash-fill" size={14} />
        <span>Clear All</span>
      </Button>
    </div>

    <!-- Filters -->
    <div class="filters">
      <div class="chips">
        <Chip label="All" selected={selectedFilter === "all"} onClick={() => (selectedFilter = "all")} variant="filter" />
        <Chip label="Uploads" icon="bi-upload" selected={selectedFilter === "upload"} onClick={() => (selectedFilter = "upload")} variant="filter" />
        <Chip label="Downloads" icon="bi-download" selected={selectedFilter === "download"} onClick={() => (selectedFilter = "download")} variant="filter" />
        <Chip label="Deletes" icon="bi-trash" selected={selectedFilter === "delete"} onClick={() => (selectedFilter = "delete")} variant="filter" />
        <Chip label="Moves" icon="bi-box-arrow-right" selected={selectedFilter === "move"} onClick={() => (selectedFilter = "move")} variant="filter" />
      </div>

      <input
        type="text"
        class="glass-input search"
        placeholder="Search activities..."
        bind:value={searchQuery}
      />
    </div>

    <!-- Timeline -->
    {#if Object.keys(groupedActivities).length === 0}
      <div class="empty">
        <Icon name="clock-history" size={64} />
        <h3>No Activity Yet</h3>
        <p>File operations will appear here</p>
      </div>
    {:else}
      {#each Object.entries(groupedActivities) as [dateLabel, activities]}
        <div class="group">
          <div class="date-label">{dateLabel}</div>
          <div class="timeline">
            {#each activities as act}
              <div class="activity-item" style="--color: {getTypeColor(act.type)}">
                <div class="marker">
                  <span class="icon">{act.icon}</span>
                </div>
                <div class="glass-card activity-card">
                  <div class="top">
                    <span class="type">{getTypeLabel(act.type)}</span>
                    <span class="time">{formatTime(act.timestamp)}</span>
                  </div>
                  <div class="name">{act.filename}</div>
                  {#if act.path}
                    <div class="path">{act.path}</div>
                  {/if}
                  {#if act.details}
                    <div class="details">{act.details}</div>
                  {/if}
                  <div class="rel">{getRelativeTime(act.timestamp)}</div>
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
  .activity-view {
    min-height: 100vh;
    background: var(--md-sys-color-surface);
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
    margin-bottom: 32px;
  }

  /* Actions */
  .actions {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 20px;
  }

  /* Filters */
  .filters {
    display: flex;
    gap: 16px;
    margin-bottom: 32px;
    flex-wrap: wrap;
  }

  .chips {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    flex: 1;
  }

  .search {
    min-width: 200px;
  }

  /* Empty State */
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 64px 32px;
    color: var(--md-sys-color-on-surface-variant);
    text-align: center;
  }

  .empty h3 {
    font-size: 20px;
    font-weight: 600;
    margin: 16px 0 8px 0;
    color: var(--md-sys-color-on-surface);
  }

  .empty p {
    font-size: 14px;
    margin: 0;
  }

  /* Timeline Groups */
  .group {
    margin-bottom: 40px;
  }

  .date-label {
    font-size: 13px;
    font-weight: 700;
    color: #6366f1;
    margin-bottom: 16px;
    text-transform: uppercase;
    letter-spacing: 1px;
    padding-left: 4px;
  }

  .timeline {
    position: relative;
    padding-left: 32px;
    border-left: 2px solid rgba(99, 102, 241, 0.15);
  }

  .activity-item {
    position: relative;
    margin-bottom: 20px;
    animation: slideIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-24px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  @keyframes float {
    0%,
    100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-10px);
    }
  }

  .marker {
    position: absolute;
    left: -43px;
    top: 0;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: var(--color, #6366f1);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow:
      0 0 0 4px var(--md-sys-color-surface),
      0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .icon {
    font-size: 16px;
  }

  .activity-card {
    padding: 16px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .activity-item:hover .activity-card {
    transform: translateX(6px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  }

  .top {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .type {
    font-size: 11px;
    font-weight: 700;
    color: var(--color, #6366f1);
    text-transform: uppercase;
    letter-spacing: 0.8px;
  }

  .time {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  .name {
    font-size: 15px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 6px;
    letter-spacing: -0.2px;
  }

  .path {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    font-family: "JetBrains Mono", "Courier New", monospace;
    margin-bottom: 6px;
    opacity: 0.8;
  }

  .details {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
    margin-top: 8px;
    line-height: 1.5;
  }

  .rel {
    font-size: 11px;
    color: var(--md-sys-color-on-surface-variant);
    margin-top: 8px;
    font-style: italic;
    opacity: 0.7;
  }

  /* Dark Mode */
  @media (prefers-color-scheme: dark) {
    .timeline {
      border-color: rgba(99, 102, 241, 0.2);
    }

    .date-label {
      color: #818cf8;
    }
  }

  /* Responsive */
  @media (max-width: 768px) {
    .stats-grid {
      grid-template-columns: 1fr;
      gap: 16px;
    }

    .filters {
      flex-direction: column;
    }

    .chips {
      width: 100%;
    }

    .search {
      width: 100%;
      min-width: unset;
    }

    .timeline {
      padding-left: 28px;
    }

    .marker {
      left: -40px;
      width: 28px;
      height: 28px;
    }

    .icon {
      font-size: 14px;
    }

    .activity-card {
      padding: 14px;
    }
  }
</style>
