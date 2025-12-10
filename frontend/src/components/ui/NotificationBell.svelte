<script>
  import { onMount, onDestroy } from "svelte";
  import { notifications, unreadCount } from "../../stores/notifications.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let showDropdown = $state(false);
  let recentNotifications = $state([]);

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
<div class="relative">
  <button
    class="relative bg-transparent border-none cursor-pointer p-2 rounded-lg transition-colors duration-200 hover:bg-black/5 dark:hover:bg-white/10"
    onclick={toggleDropdown}
    aria-label="Notifications"
  >
    <span class="text-xl block">🔔</span>
    {#if $unreadCount > 0}
      <span
        class="absolute top-1 right-1 bg-red-500 text-white rounded-full px-1.5 py-0.5 text-[11px] font-semibold min-w-[18px] text-center"
        >{$unreadCount}</span
      >
    {/if}
  </button>

  {#if showDropdown}
    <div
      class="absolute top-[calc(100%+8px)] right-0 bg-white dark:bg-gray-800 rounded-xl shadow-lg w-[360px] max-h-[500px] flex flex-col z-[1000] border border-gray-200 dark:border-gray-700"
    >
      <div
        class="flex justify-between items-center px-4 py-4 border-b border-gray-200 dark:border-gray-700"
      >
        <h3 class="m-0 text-base font-semibold text-gray-900 dark:text-white">
          Notifications
        </h3>
        {#if $unreadCount > 0}
          <button
            class="bg-transparent border-none text-blue-500 cursor-pointer text-[13px] px-2 py-1 hover:underline"
            onclick={markAllAsRead}
          >
            Mark all as read
          </button>
        {/if}
      </div>

      <div class="overflow-y-auto max-h-[400px]">
        {#if recentNotifications.length === 0}
          <div class="py-12 px-6 text-center text-gray-400">
            <span class="text-5xl block mb-2">📭</span>
            <p>No notifications</p>
          </div>
        {:else}
          {#each recentNotifications as notif}
            <button
              class="w-full text-left flex gap-3 px-4 py-3 cursor-pointer transition-colors duration-200 relative border-none {notif.is_read
                ? 'bg-transparent hover:bg-gray-50 dark:hover:bg-gray-700/50'
                : 'bg-blue-50 dark:bg-blue-900/20 hover:bg-blue-100 dark:hover:bg-blue-900/30'}"
              onclick={() => markAsRead(notif.id)}
              type="button"
            >
              <div class="text-2xl shrink-0">
                {getNotificationIcon(notif.notification_type)}
              </div>
              <div class="flex-1 min-w-0">
                <div
                  class="font-semibold text-sm mb-0.5 text-gray-900 dark:text-white"
                >
                  {notif.title}
                </div>
                <div
                  class="text-[13px] text-gray-500 dark:text-gray-400 overflow-hidden text-ellipsis whitespace-nowrap"
                >
                  {notif.message}
                </div>
                <div class="text-xs text-gray-400 dark:text-gray-500 mt-1">
                  {formatTime(notif.created_at)}
                </div>
              </div>
              {#if !notif.is_read}
                <div
                  class="w-2 h-2 bg-blue-500 rounded-full shrink-0 mt-1.5"
                ></div>
              {/if}
            </button>
          {/each}
        {/if}
      </div>

      <div
        class="border-t border-gray-200 dark:border-gray-700 px-4 py-3 text-center"
      >
        <a
          href="#/notifications"
          class="text-blue-500 no-underline text-sm font-medium hover:underline"
          >View all notifications</a
        >
      </div>
    </div>
  {/if}
</div>
