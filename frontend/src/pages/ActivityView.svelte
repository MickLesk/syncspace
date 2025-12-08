<script>
  import { onMount, onDestroy } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  import { activity } from "../stores/activity";
  import { error as errorToast } from "../stores/toast";
  import PageWrapper from "../components/PageWrapper.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";

  let groupedActivities = $state({});
  let selectedFilter = $state("all");
  let searchQuery = $state("");

  const filteredActivities = $derived(
    filterActivities($activity, selectedFilter, searchQuery)
  );
  const groupedActivitiesDerived = $derived(groupByDate(filteredActivities));
  const todayCount = $derived(activity.getToday().length);

  $effect(() => {
    groupedActivities = groupedActivitiesDerived;
  });

  const activityTypes = [
    { value: "all", label: "All", icon: "list-ul" },
    { value: "upload", label: "Uploads", icon: "upload" },
    { value: "download", label: "Downloads", icon: "download" },
    { value: "delete", label: "Deletes", icon: "trash" },
    { value: "rename", label: "Renames", icon: "pencil" },
    { value: "move", label: "Moves", icon: "arrow-right" },
    { value: "copy", label: "Copies", icon: "files" },
    { value: "share", label: "Shares", icon: "share" },
    { value: "auth", label: "Auth", icon: "shield-lock" },
    { value: "folder", label: "Folders", icon: "folder" },
  ];

  const typeConfig = {
    // File operations
    upload: { label: "Uploaded", color: "success", icon: "upload" },
    download: { label: "Downloaded", color: "info", icon: "download" },
    delete: { label: "Deleted", color: "error", icon: "trash" },
    rename: { label: "Renamed", color: "warning", icon: "pencil" },
    create: { label: "Created", color: "primary", icon: "plus-circle" },
    move: { label: "Moved", color: "secondary", icon: "arrow-right" },
    copy: { label: "Copied", color: "info", icon: "files" },
    // Sharing
    share: { label: "Shared", color: "accent", icon: "share-fill" },
    unshare: { label: "Unshared", color: "neutral", icon: "share" },
    // Favorites
    favorite: { label: "Favorited", color: "warning", icon: "star-fill" },
    unfavorite: { label: "Unfavorited", color: "neutral", icon: "star" },
    // Folder operations
    folder_create: {
      label: "Folder Created",
      color: "primary",
      icon: "folder-plus",
    },
    folder_delete: {
      label: "Folder Deleted",
      color: "error",
      icon: "folder-minus",
    },
    folder_rename: {
      label: "Folder Renamed",
      color: "warning",
      icon: "folder",
    },
    folder_move: {
      label: "Folder Moved",
      color: "secondary",
      icon: "folder-symlink",
    },
    folder_color: { label: "Color Changed", color: "accent", icon: "palette" },
    // Authentication
    login: { label: "Logged In", color: "success", icon: "box-arrow-in-right" },
    logout: { label: "Logged Out", color: "neutral", icon: "box-arrow-right" },
    password_change: {
      label: "Password Changed",
      color: "warning",
      icon: "key",
    },
    totp_enable: {
      label: "2FA Enabled",
      color: "success",
      icon: "shield-check",
    },
    totp_disable: { label: "2FA Disabled", color: "error", icon: "shield-x" },
    // Profile & Settings
    profile_update: {
      label: "Profile Updated",
      color: "info",
      icon: "person-gear",
    },
    settings_change: { label: "Settings Changed", color: "info", icon: "gear" },
    // Comments & Tags
    comment_add: {
      label: "Comment Added",
      color: "primary",
      icon: "chat-left-text",
    },
    comment_delete: {
      label: "Comment Deleted",
      color: "error",
      icon: "chat-left",
    },
    tag_add: { label: "Tag Added", color: "accent", icon: "tag-fill" },
    tag_remove: { label: "Tag Removed", color: "neutral", icon: "tag" },
    // Versions
    version_delete: {
      label: "Version Deleted",
      color: "error",
      icon: "clock-history",
    },
    version_restore: {
      label: "Version Restored",
      color: "success",
      icon: "arrow-counterclockwise",
    },
  };

  onMount(async () => {
    // Initialize WebSocket for real-time updates
    activity.init();

    // Load initial activities
    await activity.load({ limit: 100 });

    // Mark activity as visited - resets badge counter
    await markActivityVisited();

    // Clean up old localStorage key
    const oldKey = "syncspace_activity";
    if (localStorage.getItem(oldKey)) {
      localStorage.removeItem(oldKey);
    }
  });

  async function markActivityVisited() {
    try {
      const token = localStorage.getItem("authToken");
      if (!token) return;

      await fetch("http://localhost:8080/api/activity/mark-visited", {
        method: "PUT",
        headers: { Authorization: `Bearer ${token}` },
      });
    } catch (e) {
      console.error("Failed to mark activity as visited:", e);
    }
  }

  onDestroy(() => {
    // Disconnect WebSocket when component unmounts
    activity.disconnect();
  });

  function filterActivities(activities, filter, search) {
    let filtered = activities;

    if (filter !== "all") {
      // Group filters for combined categories
      const filterGroups = {
        auth: [
          "login",
          "logout",
          "password_change",
          "totp_enable",
          "totp_disable",
        ],
        folder: [
          "folder_create",
          "folder_delete",
          "folder_rename",
          "folder_move",
          "folder_color",
        ],
        share: ["share", "unshare"],
      };

      if (filterGroups[filter]) {
        filtered = filtered.filter((a) =>
          filterGroups[filter].includes(a.type)
        );
      } else {
        filtered = filtered.filter((a) => a.type === filter);
      }
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
  <div class="page-fade-in">
    <PageHeader
      title="Activity Timeline"
      subtitle="Track all file operations and changes"
      icon="activity"
    />

    <!-- Stats -->
    {#if $activity.length > 0}
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6 grid-stagger">
        <ModernCard variant="glass" hoverable class="hover-scale">
          {#snippet children()}
            <div class="text-center">
              <div class="text-primary-500 mb-3">
                <i class="bi bi-activity text-5xl bounce-in" aria-hidden="true"
                ></i>
              </div>
              <div
                class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
              >
                Total Events
              </div>
              <div
                class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2"
              >
                {$activity.length}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-500">
                All time
              </div>
            </div>
          {/snippet}
        </ModernCard>

        <ModernCard variant="glass" hoverable class="hover-scale">
          {#snippet children()}
            <div class="text-center">
              <div class="text-green-500 mb-3">
                <i
                  class="bi bi-calendar-check text-5xl bounce-in"
                  aria-hidden="true"
                ></i>
              </div>
              <div
                class="text-sm font-semibold text-gray-600 dark:text-gray-400 mb-1"
              >
                Today
              </div>
              <div
                class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2"
              >
                {todayCount}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-500">
                Recent activity
              </div>
            </div>
          {/snippet}
        </ModernCard>

        <ModernCard variant="glass" hoverable class="hover-scale">
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
            class="flex flex-wrap gap-2 flex-1 glass-card p-2 rounded-lg"
          >
            {#each activityTypes as type}
              <button
                role="tab"
                class="px-3 py-2 text-sm rounded-md transition-all flex items-center gap-2 hover-lift {selectedFilter ===
                type.value
                  ? 'bg-white dark:bg-gray-800 text-primary-600 dark:text-primary-400 font-semibold shadow-md scale-105'
                  : 'hover:bg-white/50 dark:hover:bg-gray-800/50 text-gray-700 dark:text-gray-300'}"
                onclick={() => (selectedFilter = type.value)}
              >
                <i class="bi bi-{type.icon}" aria-hidden="true"></i>
                {type.label}
              </button>
            {/each}
          </div>

          <!-- Search -->
          <div class="relative">
            <input
              type="text"
              placeholder="Search activities..."
              class="glass-input w-full md:w-64 pr-10"
              bind:value={searchQuery}
            />
            <button
              class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 dark:text-gray-400"
              aria-label="Search activities"
            >
              <i class="bi bi-search" aria-hidden="true"></i>
            </button>
          </div>
        </div>
      {/snippet}
    </ModernCard>

    <!-- Timeline -->
    {#if Object.keys(groupedActivities).length === 0}
      <ModernCard variant="glass" padding="large">
        {#snippet children()}
          <div class="text-center animate-fade-in">
            <div class="text-primary-500/30 mb-6">
              <i class="bi bi-clock-history text-8xl" aria-hidden="true"></i>
            </div>
            <h3
              class="text-2xl font-bold mb-3 text-gray-900 dark:text-gray-100"
            >
              No Activity Yet
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              File operations will appear here
            </p>
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
            <div class="badge-glass-primary text-base font-bold px-4 py-2">
              {dateLabel}
            </div>
            <div
              class="flex-1 h-px bg-gradient-to-r from-primary-500/50 to-transparent"
            ></div>
          </div>

          <!-- Timeline -->
          <div class="relative pl-8 border-l-2 border-primary-500/20 space-y-6">
            {#each activities as act, i}
              {@const config = typeConfig[act.type] || typeConfig.create}
              {@const colorMap = {
                success: "bg-green-500 text-white",
                info: "bg-green-500 text-white",
                error: "bg-red-500 text-white",
                warning: "bg-yellow-500 text-white",
                primary: "bg-primary-500 text-white",
                secondary: "bg-gray-500 text-white",
                accent: "bg-purple-500 text-white",
              }}
              <div
                class="relative animate-fade-in"
                style="animation-delay: {i * 50}ms;"
              >
                <!-- Timeline Icon -->
                <div
                  class="absolute -left-[2.25rem] top-2 w-10 h-10 rounded-full {colorMap[
                    config.color
                  ] ||
                    'bg-primary-500 text-white'} flex items-center justify-center shadow-lg"
                >
                  <i class="bi bi-{config.icon} text-lg" aria-hidden="true"></i>
                </div>

                <!-- Time Stamp -->
                <div
                  class="absolute -left-[10.5rem] top-2 text-xs text-gray-500 dark:text-gray-500 text-right w-32 font-mono"
                >
                  {formatTime(act.timestamp)}
                </div>

                <!-- Activity Card -->
                <ModernCard variant="glass" hoverable class="ml-4">
                  {#snippet children()}
                    <div class="flex items-start justify-between gap-2">
                      <div class="flex-1">
                        <div class="badge-glass-{config.color} mb-2">
                          <i class="bi bi-{config.icon} mr-1" aria-hidden="true"
                          ></i>
                          {config.label}
                        </div>
                        <h3
                          class="font-bold text-base text-gray-900 dark:text-gray-100"
                        >
                          {act.filename}
                        </h3>
                        {#if act.path}
                          <p
                            class="text-xs font-mono text-gray-500 dark:text-gray-500 mt-1"
                          >
                            {act.path}
                          </p>
                        {/if}
                        {#if act.details}
                          <p
                            class="text-sm text-gray-700 dark:text-gray-300 mt-2"
                          >
                            {act.details}
                          </p>
                        {/if}
                        <div
                          class="text-xs text-gray-400 dark:text-gray-600 mt-2 italic"
                        >
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
  </div>
</PageWrapper>
