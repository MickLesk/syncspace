<script>
  import { onMount } from "svelte";
  import { activity } from "../stores/activity";
  import { error as errorToast } from "../stores/toast";

  let groupedActivities = {};
  let selectedFilter = "all";
  let searchQuery = "";

  $: filteredActivities = filterActivities(
    $activity,
    selectedFilter,
    searchQuery
  );
  $: groupedActivities = groupByDate(filteredActivities);
  $: todayCount = activity.getToday().length;

  const activityTypes = [
    { value: "all", label: "All", icon: "list-ul" },
    { value: "upload", label: "Uploads", icon: "upload" },
    { value: "download", label: "Downloads", icon: "download" },
    { value: "delete", label: "Deletes", icon: "trash" },
    { value: "rename", label: "Renames", icon: "pencil" },
    { value: "move", label: "Moves", icon: "arrow-right" },
  ];

  const typeConfig = {
    upload: { label: "Uploaded", color: "success", icon: "upload" },
    download: { label: "Downloaded", color: "info", icon: "download" },
    delete: { label: "Deleted", color: "error", icon: "trash" },
    rename: { label: "Renamed", color: "warning", icon: "pencil" },
    create: { label: "Created", color: "primary", icon: "plus-circle" },
    move: { label: "Moved", color: "secondary", icon: "arrow-right" },
    share: { label: "Shared", color: "accent", icon: "share" },
    favorite: { label: "Favorited", color: "warning", icon: "star-fill" },
    unfavorite: { label: "Unfavorited", color: "neutral", icon: "star" },
  };

  onMount(async () => {
    await activity.load({ limit: 100 });
    const oldKey = "syncspace_activity";
    if (localStorage.getItem(oldKey)) {
      localStorage.removeItem(oldKey);
    }
  });

  function filterActivities(activities, filter, search) {
    let filtered = activities;

    if (filter !== "all") {
      filtered = filtered.filter((a) => a.type === filter);
    }

    if (search.trim()) {
      const query = search.toLowerCase();
      filtered = filtered.filter(
        (a) =>
          a.filename.toLowerCase().includes(query) ||
          a.path.toLowerCase().includes(query) ||
          (a.details && a.details.toLowerCase().includes(query))
      );
    }

    return filtered;
  }

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

  function formatDate(date) {
    return date.toLocaleDateString("en-US", {
      weekday: "short",
      month: "short",
      day: "numeric",
      year:
        date.getFullYear() !== new Date().getFullYear() ? "numeric" : undefined,
    });
  }

  function formatTime(timestamp) {
    return new Date(timestamp).toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

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

  function handleClearAll() {
    if (confirm("Clear all activity history?")) {
      activity.clear();
    }
  }
</script>

<div class="activity-view">
  <!-- Stats -->
  {#if $activity.length > 0}
    <div class="stats stats-vertical lg:stats-horizontal shadow mb-6 w-full">
      <div class="stat">
        <div class="stat-figure text-primary">
          <i class="bi bi-activity text-4xl"></i>
        </div>
        <div class="stat-title">Total Events</div>
        <div class="stat-value text-primary">{$activity.length}</div>
        <div class="stat-desc">All time</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-success">
          <i class="bi bi-calendar-check text-4xl"></i>
        </div>
        <div class="stat-title">Today</div>
        <div class="stat-value text-success">{todayCount}</div>
        <div class="stat-desc">Recent activity</div>
      </div>

      <div class="stat">
        <div class="stat-actions">
          <button
            class="btn btn-sm btn-error gap-2"
            on:click={handleClearAll}
            disabled={$activity.length === 0}
          >
            <i class="bi bi-trash-fill"></i>
            Clear All
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Filters & Search -->
  <div class="card bg-base-100 border border-base-300 shadow-sm mb-6">
    <div class="card-body p-4">
      <div class="flex flex-col md:flex-row gap-4">
        <!-- Filter Tabs -->
        <div role="tablist" class="tabs tabs-boxed flex-1">
          {#each activityTypes as type}
            <button
              role="tab"
              class="tab gap-2 {selectedFilter === type.value
                ? 'tab-active'
                : ''}"
              on:click={() => (selectedFilter = type.value)}
            >
              <i class="bi bi-{type.icon}"></i>
              {type.label}
            </button>
          {/each}
        </div>

        <!-- Search -->
        <div class="form-control">
          <div class="input-group">
            <input
              type="text"
              placeholder="Search activities..."
              class="input input-bordered"
              bind:value={searchQuery}
            />
            <button class="btn btn-square">
              <i class="bi bi-search"></i>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Timeline -->
  {#if Object.keys(groupedActivities).length === 0}
    <div class="hero min-h-[400px]">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <i class="bi bi-clock-history text-7xl text-base-300 mb-4"></i>
          <h1 class="text-3xl font-bold">No Activity Yet</h1>
          <p class="py-6">File operations will appear here</p>
        </div>
      </div>
    </div>
  {:else}
    {#each Object.entries(groupedActivities) as [dateLabel, activities]}
      <div class="mb-8">
        <!-- Date Badge -->
        <div class="flex items-center gap-3 mb-4">
          <div class="badge badge-primary badge-lg font-bold">{dateLabel}</div>
          <div class="flex-1 h-px bg-base-300"></div>
        </div>

        <!-- Timeline -->
        <ul class="timeline timeline-vertical">
          {#each activities as act, i}
            {@const config = typeConfig[act.type] || typeConfig.create}
            <li>
              {#if i > 0}
                <hr class="bg-{config.color}" />
              {/if}
              <div class="timeline-start timeline-box">
                <div class="text-xs opacity-70">
                  {formatTime(act.timestamp)}
                </div>
              </div>
              <div class="timeline-middle">
                <div class="avatar placeholder">
                  <div
                    class="bg-{config.color} text-{config.color}-content rounded-full w-10"
                  >
                    <i class="bi bi-{config.icon} text-xl"></i>
                  </div>
                </div>
              </div>
              <div class="timeline-end">
                <div
                  class="card bg-base-100 border border-base-300 shadow-sm hover:shadow-md transition-shadow"
                >
                  <div class="card-body p-4">
                    <div class="flex items-start justify-between gap-2">
                      <div class="flex-1">
                        <div
                          class="badge badge-{config.color} badge-sm mb-2 gap-1"
                        >
                          <i class="bi bi-{config.icon}"></i>
                          {config.label}
                        </div>
                        <h3 class="font-bold text-base">{act.filename}</h3>
                        {#if act.path}
                          <p class="text-xs font-mono opacity-70 mt-1">
                            {act.path}
                          </p>
                        {/if}
                        {#if act.details}
                          <p class="text-sm opacity-80 mt-2">{act.details}</p>
                        {/if}
                        <div class="text-xs opacity-60 mt-2 italic">
                          {getRelativeTime(act.timestamp)}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              {#if i < activities.length - 1}
                <hr class="bg-{config.color}" />
              {/if}
            </li>
          {/each}
        </ul>
      </div>
    {/each}
  {/if}
</div>

<style>
  .activity-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }

  /* Timeline customization */
  :global(.timeline-vertical li) {
    margin-bottom: 2rem;
  }

  :global(.timeline-box) {
    min-width: 80px;
  }

  /* Badge colors */
  :global(.badge-success) {
    background-color: oklch(var(--su));
    color: oklch(var(--suc));
  }

  :global(.badge-info) {
    background-color: oklch(var(--in));
    color: oklch(var(--inc));
  }

  :global(.badge-error) {
    background-color: oklch(var(--er));
    color: oklch(var(--erc));
  }

  :global(.badge-warning) {
    background-color: oklch(var(--wa));
    color: oklch(var(--wac));
  }

  :global(.badge-secondary) {
    background-color: oklch(var(--s));
    color: oklch(var(--sc));
  }

  :global(.badge-accent) {
    background-color: oklch(var(--a));
    color: oklch(var(--ac));
  }

  /* Avatar colors */
  :global(.bg-success) {
    background-color: oklch(var(--su));
  }

  :global(.bg-info) {
    background-color: oklch(var(--in));
  }

  :global(.bg-error) {
    background-color: oklch(var(--er));
  }

  :global(.bg-warning) {
    background-color: oklch(var(--wa));
  }

  :global(.bg-secondary) {
    background-color: oklch(var(--s));
  }

  :global(.bg-accent) {
    background-color: oklch(var(--a));
  }

  :global(.text-success-content) {
    color: oklch(var(--suc));
  }

  :global(.text-info-content) {
    color: oklch(var(--inc));
  }

  :global(.text-error-content) {
    color: oklch(var(--erc));
  }

  :global(.text-warning-content) {
    color: oklch(var(--wac));
  }

  :global(.text-secondary-content) {
    color: oklch(var(--sc));
  }

  :global(.text-accent-content) {
    color: oklch(var(--ac));
  }

  /* Timeline lines */
  :global(.timeline hr.bg-success) {
    background-color: oklch(var(--su));
  }

  :global(.timeline hr.bg-info) {
    background-color: oklch(var(--in));
  }

  :global(.timeline hr.bg-error) {
    background-color: oklch(var(--er));
  }

  :global(.timeline hr.bg-warning) {
    background-color: oklch(var(--wa));
  }

  :global(.timeline hr.bg-secondary) {
    background-color: oklch(var(--s));
  }

  :global(.timeline hr.bg-accent) {
    background-color: oklch(var(--a));
  }
</style>
