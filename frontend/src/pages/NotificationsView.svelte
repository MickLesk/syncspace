<script>
  import { onMount } from "svelte";
  import { t } from "../i18n.js";
  import { currentLang } from "../stores/ui.js";
  import PageWrapper from "../components/PageWrapper.svelte";
  import ModernCard from "../components/ui/ModernCard.svelte";
  import ModernButton from "../components/ui/ModernButton.svelte";

  // Mock notifications data (should be from API in production)
  let notifications = $state([
    {
      id: 1,
      type: "success",
      icon: "check-circle-fill",
      title: "File uploaded successfully",
      message: "Document.pdf has been uploaded to /documents/",
      time: new Date(Date.now() - 2 * 60 * 1000),
      read: false,
      avatar: null,
    },
    {
      id: 2,
      type: "info",
      icon: "share-fill",
      title: "New share request",
      message: 'John Doe shared "Project Files" with you',
      time: new Date(Date.now() - 60 * 60 * 1000),
      read: false,
      avatar: "JD",
    },
    {
      id: 3,
      type: "warning",
      icon: "exclamation-triangle-fill",
      title: "Storage almost full",
      message: "85% of storage capacity used. Consider cleaning up old files.",
      time: new Date(Date.now() - 3 * 60 * 60 * 1000),
      read: true,
      avatar: null,
    },
    {
      id: 4,
      type: "error",
      icon: "x-circle-fill",
      title: "Upload failed",
      message: "Failed to upload large-file.zip. File exceeds size limit.",
      time: new Date(Date.now() - 6 * 60 * 60 * 1000),
      read: true,
      avatar: null,
    },
    {
      id: 5,
      type: "info",
      icon: "people-fill",
      title: "New user added",
      message: "Jane Smith was added to your workspace",
      time: new Date(Date.now() - 24 * 60 * 60 * 1000),
      read: true,
      avatar: "JS",
    },
  ]);

  let filterType = $state("all"); // all, unread, read
  let loading = $state(false);

  let filteredNotifications = $derived(() => {
    if (filterType === "unread") {
      return notifications.filter((n) => !n.read);
    } else if (filterType === "read") {
      return notifications.filter((n) => n.read);
    }
    return notifications;
  });

  let unreadCount = $derived(() => notifications.filter((n) => !n.read).length);

  function markAsRead(id) {
    notifications = notifications.map((n) =>
      n.id === id ? { ...n, read: true } : n
    );
  }

  function markAllAsRead() {
    notifications = notifications.map((n) => ({ ...n, read: true }));
  }

  function deleteNotification(id) {
    notifications = notifications.filter((n) => n.id !== id);
  }

  function clearAll() {
    if (confirm("Are you sure you want to delete all notifications?")) {
      notifications = [];
    }
  }

  function formatRelativeTime(date) {
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / (1000 * 60));
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffMins < 1) return "Just now";
    if (diffMins < 60)
      return `${diffMins} minute${diffMins > 1 ? "s" : ""} ago`;
    if (diffHours < 24)
      return `${diffHours} hour${diffHours > 1 ? "s" : ""} ago`;
    if (diffDays < 7) return `${diffDays} day${diffDays > 1 ? "s" : ""} ago`;

    return date.toLocaleDateString();
  }
</script>

<PageWrapper gradient>
  <!-- Animated Background Blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>
  <div class="blob blob-3"></div>

  <div class="relative z-10">
    <!-- Header -->
    <div class="flex justify-between items-start mb-8 gap-4 flex-wrap">
      <div class="flex-1 min-w-0">
        <h1
          class="text-4xl font-bold gradient-text-primary mb-2 flex items-center gap-3"
        >
          <i class="bi bi-bell-fill"></i>
          Notifications
        </h1>
        <p class="text-base-content/60 text-lg">
          {#if unreadCount() > 0}
            You have <span class="badge badge-glass-info badge-sm"
              >{unreadCount()}</span
            >
            unread notification{unreadCount() > 1 ? "s" : ""}
          {:else}
            You're all caught up! 🎉
          {/if}
        </p>
      </div>

      <div class="flex gap-2 flex-wrap">
        {#if unreadCount() > 0}
          <ModernButton
            variant="ghost"
            size="sm"
            icon="check-all"
            onclick={markAllAsRead}
          >
            Mark all as read
          </ModernButton>
        {/if}
        {#if notifications.length > 0}
          <ModernButton
            variant="danger"
            size="sm"
            icon="trash"
            onclick={clearAll}
          >
            Clear all
          </ModernButton>
        {/if}
      </div>
    </div>

    <!-- Filter Tabs -->
    <ModernCard variant="glass" class="mb-6">
      {#snippet children()}
        <div class="glass-card-light p-2 rounded-lg flex gap-2">
          <button
            class="flex-1 px-4 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200 {filterType ===
            'all'
              ? 'glass-card text-primary shadow-md scale-105'
              : 'hover:glass-card-light text-base-content/60 hover:text-base-content'}"
            onclick={() => (filterType = "all")}
          >
            <i class="bi bi-list-ul mr-2"></i>
            All <span class="badge badge-sm ml-2">{notifications.length}</span>
          </button>
          <button
            class="flex-1 px-4 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200 {filterType ===
            'unread'
              ? 'glass-card text-primary shadow-md scale-105'
              : 'hover:glass-card-light text-base-content/60 hover:text-base-content'}"
            onclick={() => (filterType = "unread")}
          >
            <i class="bi bi-envelope mr-2"></i>
            Unread <span class="badge badge-sm ml-2">{unreadCount()}</span>
          </button>
          <button
            class="flex-1 px-4 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200 {filterType ===
            'read'
              ? 'glass-card text-primary shadow-md scale-105'
              : 'hover:glass-card-light text-base-content/60 hover:text-base-content'}"
            onclick={() => (filterType = "read")}
          >
            <i class="bi bi-envelope-open mr-2"></i>
            Read
            <span class="badge badge-sm ml-2"
              >{notifications.length - unreadCount()}</span
            >
          </button>
        </div>
      {/snippet}
    </ModernCard>

    <!-- Notifications List -->
    <div class="space-y-3">
      {#if filteredNotifications().length > 0}
        {#each filteredNotifications() as notification, i}
          <div class="animate-slide-up" style="animation-delay: {i * 50}ms;">
            <ModernCard
              variant="glass"
              hoverable
              class={!notification.read ? "border-l-4 border-primary" : ""}
            >
              {#snippet children()}
                <div class="flex items-start gap-4">
                  <!-- Icon/Avatar -->
                  <div class="flex-shrink-0">
                    {#if notification.avatar}
                      <div
                        class="w-12 h-12 rounded-xl bg-gradient-to-br from-primary to-secondary flex items-center justify-center text-white font-bold text-sm"
                      >
                        {notification.avatar}
                      </div>
                    {:else}
                      <div
                        class="w-12 h-12 rounded-xl flex items-center justify-center text-xl
                        {notification.type === 'success'
                          ? 'bg-success/20 text-success'
                          : ''}
                        {notification.type === 'info'
                          ? 'bg-info/20 text-info'
                          : ''}
                        {notification.type === 'warning'
                          ? 'bg-warning/20 text-warning'
                          : ''}
                        {notification.type === 'error'
                          ? 'bg-error/20 text-error'
                          : ''}
                      "
                      >
                        <i class="bi bi-{notification.icon}"></i>
                      </div>
                    {/if}
                  </div>

                  <!-- Content -->
                  <div class="flex-1 min-w-0">
                    <div class="flex justify-between items-start gap-3 mb-2">
                      <h3
                        class="font-bold text-base-content {!notification.read
                          ? 'text-base'
                          : 'text-sm'}"
                      >
                        {notification.title}
                      </h3>
                      <span
                        class="badge badge-glass-info badge-sm flex-shrink-0"
                      >
                        <i class="bi bi-clock mr-1"></i>
                        {formatRelativeTime(notification.time)}
                      </span>
                    </div>
                    <p class="text-sm text-base-content/70">
                      {notification.message}
                    </p>
                  </div>

                  <!-- Actions -->
                  <div class="flex gap-1 flex-shrink-0">
                    {#if !notification.read}
                      <button
                        class="btn btn-ghost btn-sm btn-square"
                        onclick={() => markAsRead(notification.id)}
                        title="Mark as read"
                      >
                        <i class="bi bi-check"></i>
                      </button>
                    {/if}
                    <button
                      class="btn btn-ghost btn-sm btn-square text-error"
                      onclick={() => deleteNotification(notification.id)}
                      title="Delete"
                    >
                      <i class="bi bi-trash"></i>
                    </button>
                  </div>
                </div>
              {/snippet}
            </ModernCard>
          </div>
        {/each}
      {:else}
        <div class="animate-fade-in">
          <ModernCard variant="glass">
            {#snippet children()}
              <div class="text-center py-16">
                <i class="bi bi-bell-slash text-6xl opacity-20 mb-4 block"></i>
                <h3 class="text-xl font-bold mb-2">No notifications</h3>
                <p class="text-base-content/60">
                  {filterType === "all"
                    ? "You have no notifications"
                    : filterType === "unread"
                      ? "You have no unread notifications"
                      : "You have no read notifications"}
                </p>
              </div>
            {/snippet}
          </ModernCard>
        </div>
      {/if}
    </div>
  </div>
</PageWrapper>

<style>
  /* NotificationsView Styles */
</style>
