<script>
  import { onMount } from "svelte";
  import { t } from "../i18n.js";
  import { currentLang } from "../stores/ui.js";

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

<div class="notifications-view">
  <div class="notifications-header">
    <div class="header-content">
      <h1 class="page-title">
        <i class="bi bi-bell-fill"></i>
        Notifications
      </h1>
      <p class="page-subtitle">
        {#if unreadCount() > 0}
          You have {unreadCount()} unread notification{unreadCount() > 1
            ? "s"
            : ""}
        {:else}
          You're all caught up!
        {/if}
      </p>
    </div>

    <div class="header-actions">
      {#if unreadCount() > 0}
        <button class="btn btn-ghost btn-sm gap-2" onclick={markAllAsRead}>
          <i class="bi bi-check-all"></i>
          Mark all as read
        </button>
      {/if}
      {#if notifications.length > 0}
        <button
          class="btn btn-ghost btn-sm gap-2 text-error"
          onclick={clearAll}
        >
          <i class="bi bi-trash"></i>
          Clear all
        </button>
      {/if}
    </div>
  </div>

  <!-- Filter Tabs -->
  <div class="filter-tabs">
    <button
      class="filter-tab"
      class:active={filterType === "all"}
      onclick={() => (filterType = "all")}
    >
      All ({notifications.length})
    </button>
    <button
      class="filter-tab"
      class:active={filterType === "unread"}
      onclick={() => (filterType = "unread")}
    >
      Unread ({unreadCount()})
    </button>
    <button
      class="filter-tab"
      class:active={filterType === "read"}
      onclick={() => (filterType = "read")}
    >
      Read ({notifications.length - unreadCount()})
    </button>
  </div>

  <!-- Notifications List -->
  <div class="notifications-list">
    {#if filteredNotifications().length > 0}
      {#each filteredNotifications() as notification}
        <div class="notification-card" class:unread={!notification.read}>
          <div
            class="notification-indicator"
            class:unread={!notification.read}
          ></div>

          <div class="notification-icon-wrapper {notification.type}">
            {#if notification.avatar}
              <div class="notification-avatar">
                {notification.avatar}
              </div>
            {:else}
              <i class="bi bi-{notification.icon}"></i>
            {/if}
          </div>

          <div class="notification-content">
            <div class="notification-header-row">
              <h3 class="notification-title">{notification.title}</h3>
              <span class="notification-time">
                <i class="bi bi-clock"></i>
                {formatRelativeTime(notification.time)}
              </span>
            </div>
            <p class="notification-message">{notification.message}</p>
          </div>

          <div class="notification-actions">
            {#if !notification.read}
              <button
                class="action-btn"
                onclick={() => markAsRead(notification.id)}
                title="Mark as read"
              >
                <i class="bi bi-check"></i>
              </button>
            {/if}
            <button
              class="action-btn"
              onclick={() => deleteNotification(notification.id)}
              title="Delete"
            >
              <i class="bi bi-trash"></i>
            </button>
          </div>
        </div>
      {/each}
    {:else}
      <div class="empty-state">
        <i class="bi bi-bell-slash text-6xl opacity-20"></i>
        <h3 class="empty-title">No notifications</h3>
        <p class="empty-message">
          {filterType === "all"
            ? "You have no notifications"
            : filterType === "unread"
              ? "You have no unread notifications"
              : "You have no read notifications"}
        </p>
      </div>
    {/if}
  </div>
</div>

<style>
  .notifications-view {
    padding: 2rem;
    max-width: 900px;
    margin: 0 auto;
  }

  .notifications-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 2rem;
    gap: 2rem;
  }

  .header-content {
    flex: 1;
  }

  .page-title {
    font-size: 2rem;
    font-weight: 800;
    color: hsl(var(--bc));
    margin: 0 0 0.5rem 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .page-title i {
    color: hsl(var(--p));
  }

  .page-subtitle {
    font-size: 1rem;
    color: hsl(var(--bc) / 0.6);
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  /* Filter Tabs */
  .filter-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    background: hsl(var(--b2));
    padding: 0.5rem;
    border-radius: 12px;
  }

  .filter-tab {
    padding: 0.5rem 1rem;
    border: none;
    background: transparent;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 600;
    color: hsl(var(--bc) / 0.7);
    cursor: pointer;
    transition: all 0.2s;
  }

  .filter-tab:hover {
    background: hsl(var(--b3));
    color: hsl(var(--bc));
  }

  .filter-tab.active {
    background: hsl(var(--p));
    color: hsl(var(--pc));
  }

  /* Notifications List */
  .notifications-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .notification-card {
    display: flex;
    align-items: start;
    gap: 1rem;
    padding: 1.25rem;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: 12px;
    transition: all 0.2s;
    position: relative;
    overflow: hidden;
  }

  .notification-card:hover {
    border-color: hsl(var(--bc) / 0.2);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .notification-card.unread {
    background: hsl(var(--p) / 0.03);
    border-color: hsl(var(--p) / 0.2);
  }

  .notification-indicator {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 4px;
    background: transparent;
    transition: background 0.2s;
  }

  .notification-indicator.unread {
    background: hsl(var(--p));
  }

  .notification-icon-wrapper {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    font-size: 1.25rem;
    margin-left: 0.5rem;
  }

  .notification-icon-wrapper.success {
    background: hsl(var(--su) / 0.15);
    color: hsl(var(--su));
  }

  .notification-icon-wrapper.info {
    background: hsl(var(--in) / 0.15);
    color: hsl(var(--in));
  }

  .notification-icon-wrapper.warning {
    background: hsl(var(--wa) / 0.15);
    color: hsl(var(--wa));
  }

  .notification-icon-wrapper.error {
    background: hsl(var(--er) / 0.15);
    color: hsl(var(--er));
  }

  .notification-avatar {
    width: 100%;
    height: 100%;
    border-radius: 12px;
    background: linear-gradient(135deg, hsl(var(--p)), hsl(var(--s)));
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    color: white;
    font-size: 0.875rem;
  }

  .notification-content {
    flex: 1;
    min-width: 0;
  }

  .notification-header-row {
    display: flex;
    justify-content: space-between;
    align-items: start;
    gap: 1rem;
    margin-bottom: 0.5rem;
  }

  .notification-title {
    font-size: 0.9375rem;
    font-weight: 600;
    color: hsl(var(--bc));
    margin: 0;
  }

  .notification-time {
    font-size: 0.75rem;
    color: hsl(var(--bc) / 0.5);
    display: flex;
    align-items: center;
    gap: 0.25rem;
    white-space: nowrap;
  }

  .notification-message {
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.7);
    margin: 0;
    line-height: 1.5;
  }

  .notification-actions {
    display: flex;
    gap: 0.25rem;
    flex-shrink: 0;
  }

  .action-btn {
    width: 2rem;
    height: 2rem;
    border: none;
    background: hsl(var(--bc) / 0.05);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: hsl(var(--bc) / 0.6);
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-btn:hover {
    background: hsl(var(--bc) / 0.1);
    color: hsl(var(--bc));
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }

  .empty-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: hsl(var(--bc));
    margin: 1rem 0 0.5rem;
  }

  .empty-message {
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.6);
    margin: 0;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .notifications-view {
      padding: 1rem;
    }

    .notifications-header {
      flex-direction: column;
      gap: 1rem;
    }

    .header-actions {
      width: 100%;
      justify-content: flex-start;
    }

    .notification-card {
      padding: 1rem;
    }

    .notification-icon-wrapper {
      width: 40px;
      height: 40px;
      font-size: 1rem;
    }

    .notification-header-row {
      flex-direction: column;
      gap: 0.25rem;
    }

    .notification-time {
      align-self: flex-start;
    }
  }
</style>
