<script>
  import { onMount, onDestroy } from "svelte";
  import { notifications, unreadCount } from "../stores/notifications.js";

  let showDropdown = false;
  let recentNotifications = [];

  // Subscribe to notifications
  const unsubscribe = notifications.subscribe((value) => {
    recentNotifications = value.slice(0, 5); // Show last 5
  });

  onMount(async () => {
    // Load notifications on mount
    await notifications.load();
  });

  onDestroy(() => {
    unsubscribe();
  });

  function toggleDropdown() {
    showDropdown = !showDropdown;
  }

  async function markAsRead(notificationId) {
    await notifications.markRead(notificationId);
  }

  async function markAllAsRead() {
    for (const notif of recentNotifications) {
      if (!notif.is_read) {
        await notifications.markRead(notif.id);
      }
    }
  }

  function getNotificationIcon(type) {
    switch (type) {
      case "info":
        return "🔵";
      case "success":
        return "✅";
      case "warning":
        return "⚠️";
      case "error":
        return "❌";
      case "file_shared":
        return "📤";
      case "file_updated":
        return "📝";
      case "comment":
        return "💬";
      default:
        return "🔔";
    }
  }

  function formatTime(timestamp) {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now - date;

    if (diff < 60000) return "Just now";
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    return date.toLocaleDateString();
  }
</script>

<!-- Notification Bell -->
<div class="notification-bell">
  <button
    class="bell-button"
    onclick={toggleDropdown}
    aria-label="Notifications"
  >
    <span class="bell-icon">🔔</span>
    {#if $unreadCount > 0}
      <span class="badge">{$unreadCount}</span>
    {/if}
  </button>

  {#if showDropdown}
    <div class="dropdown">
      <div class="dropdown-header">
        <h3>Notifications</h3>
        {#if $unreadCount > 0}
          <button class="mark-all" onclick={markAllAsRead}>
            Mark all as read
          </button>
        {/if}
      </div>

      <div class="notification-list">
        {#if recentNotifications.length === 0}
          <div class="empty-state">
            <span>📭</span>
            <p>No notifications</p>
          </div>
        {:else}
          {#each recentNotifications as notif}
            <div
              class="notification-item"
              class:unread={!notif.is_read}
              onclick={() => markAsRead(notif.id)}
            >
              <div class="notif-icon">
                {getNotificationIcon(notif.notification_type)}
              </div>
              <div class="notif-content">
                <div class="notif-title">{notif.title}</div>
                <div class="notif-message">{notif.message}</div>
                <div class="notif-time">{formatTime(notif.created_at)}</div>
              </div>
              {#if !notif.is_read}
                <div class="unread-dot"></div>
              {/if}
            </div>
          {/each}
        {/if}
      </div>

      <div class="dropdown-footer">
        <a href="#/notifications">View all notifications</a>
      </div>
    </div>
  {/if}
</div>

<style>
  .notification-bell {
    position: relative;
  }

  .bell-button {
    position: relative;
    background: none;
    border: none;
    cursor: pointer;
    padding: 8px;
    border-radius: 8px;
    transition: background 0.2s;
  }

  .bell-button:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .bell-icon {
    font-size: 20px;
    display: block;
  }

  .badge {
    position: absolute;
    top: 4px;
    right: 4px;
    background: #ef4444;
    color: white;
    border-radius: 10px;
    padding: 2px 6px;
    font-size: 11px;
    font-weight: 600;
    min-width: 18px;
    text-align: center;
  }

  .dropdown {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    background: white;
    border-radius: 12px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    width: 360px;
    max-height: 500px;
    display: flex;
    flex-direction: column;
    z-index: 1000;
  }

  .dropdown-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid #e5e7eb;
  }

  .dropdown-header h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
  }

  .mark-all {
    background: none;
    border: none;
    color: #3b82f6;
    cursor: pointer;
    font-size: 13px;
    padding: 4px 8px;
  }

  .mark-all:hover {
    text-decoration: underline;
  }

  .notification-list {
    overflow-y: auto;
    max-height: 400px;
  }

  .empty-state {
    padding: 48px 24px;
    text-align: center;
    color: #9ca3af;
  }

  .empty-state span {
    font-size: 48px;
    display: block;
    margin-bottom: 8px;
  }

  .notification-item {
    display: flex;
    gap: 12px;
    padding: 12px 16px;
    cursor: pointer;
    transition: background 0.2s;
    position: relative;
  }

  .notification-item:hover {
    background: #f9fafb;
  }

  .notification-item.unread {
    background: #eff6ff;
  }

  .notif-icon {
    font-size: 24px;
    flex-shrink: 0;
  }

  .notif-content {
    flex: 1;
    min-width: 0;
  }

  .notif-title {
    font-weight: 600;
    font-size: 14px;
    margin-bottom: 2px;
    color: #111827;
  }

  .notif-message {
    font-size: 13px;
    color: #6b7280;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .notif-time {
    font-size: 12px;
    color: #9ca3af;
    margin-top: 4px;
  }

  .unread-dot {
    width: 8px;
    height: 8px;
    background: #3b82f6;
    border-radius: 50%;
    flex-shrink: 0;
    margin-top: 6px;
  }

  .dropdown-footer {
    border-top: 1px solid #e5e7eb;
    padding: 12px 16px;
    text-align: center;
  }

  .dropdown-footer a {
    color: #3b82f6;
    text-decoration: none;
    font-size: 14px;
    font-weight: 500;
  }

  .dropdown-footer a:hover {
    text-decoration: underline;
  }
</style>

