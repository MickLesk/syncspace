<script>
  import { onMount } from "svelte";
  import { activity } from "../../stores/activity";
  import { error as errorToast } from "../../stores/toast";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";

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

<PageWrapper gradient>
  <!-- Animated Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <!-- Page Header -->
  <div class="mb-8 relative z-10">
    <h1
      class="text-4xl font-bold gradient-text-primary mb-2 flex items-center gap-3"
    >
      <i class="bi bi-activity"></i>
      Activity Timeline
    </h1>
    <p class="text-base-content/70">Track all file operations and changes</p>
  </div>

  <!-- Stats -->
  {#if $activity.length > 0}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center">
            <div class="text-primary mb-3">
              <i class="bi bi-activity text-5xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">
              Total Events
            </div>
            <div class="text-4xl font-bold mb-2">{$activity.length}</div>
            <div class="text-xs text-base-content/50">All time</div>
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="gradient" hoverable>
        {#snippet children()}
          <div class="text-center">
            <div class="text-success mb-3">
              <i class="bi bi-calendar-check text-5xl"></i>
            </div>
            <div class="text-sm font-semibold text-base-content/60 mb-1">
              Today
            </div>
            <div class="text-4xl font-bold mb-2">{todayCount}</div>
            <div class="text-xs text-base-content/50">Recent activity</div>
          </div>
        {/snippet}
      </ModernCard>

      <ModernCard variant="glass" hoverable>
        {#snippet children()}
          <div
            class="text-center flex flex-col items-center justify-center h-full"
          >
            <ModernButton
              variant="danger"
              icon="trash-fill"
              onclick={handleClearAll}
              disabled={$activity.length === 0}
              fullWidth
            >
              Clear All History
            </ModernButton>
          </div>
        {/snippet}
      </ModernCard>
    </div>
  {/if}

  <!-- Filters & Search -->
  <ModernCard variant="glass" class="mb-6">
    {#snippet children()}
      <div class="flex flex-col md:flex-row gap-4">
        <!-- Filter Tabs -->
        <div
          role="tablist"
          class="flex flex-wrap gap-2 flex-1 glass-card-light p-2 rounded-lg"
        >
          {#each activityTypes as type}
            <button
              role="tab"
              class="px-3 py-2 text-sm rounded-md transition-all flex items-center gap-2 {selectedFilter ===
              type.value
                ? 'glass-card text-primary font-semibold shadow-md scale-105'
                : 'hover:glass-card-light'}"
              onclick={() => (selectedFilter = type.value)}
            >
              <i class="bi bi-{type.icon}"></i>
              {type.label}
            </button>
          {/each}
        </div>

        <!-- Search -->
        <div class="relative">
          <input
            type="text"
            placeholder="Search activities..."
            class="input input-bordered glass-input w-full md:w-64 pr-10"
            bind:value={searchQuery}
          />
          <button
            class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content/50"
          >
            <i class="bi bi-search"></i>
          </button>
        </div>
      </div>
    {/snippet}
  </ModernCard>

  <!-- Timeline -->
  {#if Object.keys(groupedActivities).length === 0}
    <ModernCard variant="glass" class="text-center py-16">
      {#snippet children()}
        <div class="animate-fade-in">
          <div class="text-primary/30 mb-6">
            <i class="bi bi-clock-history text-8xl"></i>
          </div>
          <h3 class="text-2xl font-bold mb-3">No Activity Yet</h3>
          <p class="text-base-content/60">File operations will appear here</p>
        </div>
      {/snippet}
    </ModernCard>
  {:else}
    {#each Object.entries(groupedActivities) as [dateLabel, activities], groupIndex}
      <div
        class="mb-8 animate-slide-up"
        style="animation-delay: {groupIndex * 100}ms;"
      >
        <!-- Date Badge -->
        <div class="flex items-center gap-3 mb-4">
          <div class="badge badge-glass-primary badge-lg font-bold">
            {dateLabel}
          </div>
          <div
            class="flex-1 h-px bg-gradient-to-r from-primary/50 to-transparent"
          ></div>
        </div>

        <!-- Timeline -->
        <div class="relative pl-8 border-l-2 border-primary/20 space-y-6">
          {#each activities as act, i}
            {@const config = typeConfig[act.type] || typeConfig.create}
            {@const colorMap = {
              success: "bg-success text-success-content",
              info: "bg-info text-info-content",
              error: "bg-error text-error-content",
              warning: "bg-warning text-warning-content",
              primary: "bg-primary text-primary-content",
              secondary: "bg-secondary text-secondary-content",
              accent: "bg-accent text-accent-content",
            }}
            <div
              class="relative animate-fade-in"
              style="animation-delay: {i * 50}ms;"
            >
              <!-- Timeline Icon -->
              <div
                class="absolute -left-[2.25rem] top-2 w-10 h-10 rounded-full {colorMap[
                  config.color
                ] || 'bg-primary'} flex items-center justify-center shadow-lg"
              >
                <i class="bi bi-{config.icon} text-lg"></i>
              </div>

              <!-- Time Stamp -->
              <div
                class="absolute -left-[10.5rem] top-2 text-xs text-base-content/50 text-right w-32 font-mono"
              >
                {formatTime(act.timestamp)}
              </div>

              <!-- Activity Card -->
              <ModernCard variant="glass" hoverable class="ml-4">
                {#snippet children()}
                  <div class="flex items-start justify-between gap-2">
                    <div class="flex-1">
                      <div class="badge badge-glass-{config.color} mb-2">
                        <i class="bi bi-{config.icon} mr-1"></i>
                        {config.label}
                      </div>
                      <h3 class="font-bold text-base">{act.filename}</h3>
                      {#if act.path}
                        <p class="text-xs font-mono text-base-content/50 mt-1">
                          {act.path}
                        </p>
                      {/if}
                      {#if act.details}
                        <p class="text-sm text-base-content/70 mt-2">
                          {act.details}
                        </p>
                      {/if}
                      <div class="text-xs text-base-content/40 mt-2 italic">
                        {getRelativeTime(act.timestamp)}
                      </div>
                    </div>
                  </div>
                {/snippet}
              </ModernCard>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</PageWrapper>

<style>
  /* ActivityView - using glassmorphism.css */
</style>


