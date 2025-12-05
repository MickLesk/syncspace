<script>
  import { onMount } from "svelte";
  import { t } from "../../i18n.js";
  import { currentLang } from "../../stores/ui.js";
  import api from "../../lib/api.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Load notifications from API
  let notifications = $state([]);
  let loading = $state(true);

  onMount(async () => {
    await loadNotifications();
  });

  async function loadNotifications() {
    try {
      const data = await api.notifications?.list?.();
      if (!data) {
        throw new Error("Notifications API not available");
      }
      notifications = data.map((n) => ({
        ...n,
        time: new Date(n.created_at || n.time),
      }));
    } catch (error) {
      console.error("Failed to load notifications:", error);
      notifications = [];
    } finally {
      loading = false;
    }
  }

  let filterType = $state("all"); // all, unread, read

  let filteredNotifications = $derived(() => {
    if (filterType === "unread") {
      return notifications.filter((n) => !n.read);
    } else if (filterType === "read") {
      return notifications.filter((n) => n.read);
    }
    return notifications;
  });

  let unreadCount = $derived(() => notifications.filter((n) => !n.read).length);

  async function markAsRead(id) {
    try {
      await api.notifications?.markRead?.(id);
      notifications = notifications.map((n) =>
        n.id === id ? { ...n, read: true } : n
      );
    } catch (error) {
      console.error("Failed to mark as read:", error);
    }
  }

  async function markAllAsRead() {
    try {
      await api.notifications?.markAllRead?.();
      notifications = notifications.map((n) => ({ ...n, read: true }));
    } catch (error) {
      console.error("Failed to mark all as read:", error);
    }
  }

  async function deleteNotification(id) {
    try {
      await api.notifications?.delete?.(id);
      notifications = notifications.filter((n) => n.id !== id);
    } catch (error) {
      console.error("Failed to delete notification:", error);
    }
  }

  function clearAll() {
    if (confirm(tr("deleteAllNotificationsConfirm"))) {
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
  <div class="page-fade-in">
    <PageHeader
      title="Notifications"
      subtitle={unreadCount() > 0
        ? `You have ${unreadCount()} unread notification${unreadCount() > 1 ? "s" : ""}`
        : "You're all caught up! 🎉"}
      icon="bell-fill"
    >
      {#snippet actions()}
        {#if unreadCount() > 0}
          <ModernButton
            variant="secondary"
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
      {/snippet}
    </PageHeader>

    <!-- Filter Tabs -->
    <ModernCard variant="glass" class="mb-6">
      {#snippet children()}
        <div class="glass-card p-2 rounded-lg flex gap-2">
          <button
            class="flex-1 px-4 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200 {filterType ===
            'all'
              ? 'bg-white dark:bg-gray-800 text-primary-600 dark:text-primary-400 shadow-md scale-105'
              : 'hover:bg-white/50 dark:hover:bg-gray-800/50 text-gray-700 dark:text-gray-300'}"
            onclick={() => (filterType = "all")}
          >
            <i class="bi bi-list-ul mr-2" aria-hidden="true"></i>
            All
            <span class="badge-glass-info ml-2">{notifications.length}</span>
          </button>
          <button
            class="flex-1 px-4 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200 {filterType ===
            'unread'
              ? 'bg-white dark:bg-gray-800 text-primary-600 dark:text-primary-400 shadow-md scale-105'
              : 'hover:bg-white/50 dark:hover:bg-gray-800/50 text-gray-700 dark:text-gray-300'}"
            onclick={() => (filterType = "unread")}
          >
            <i class="bi bi-envelope mr-2" aria-hidden="true"></i>
            Unread
            <span class="badge-glass-warning ml-2">{unreadCount()}</span>
          </button>
          <button
            class="flex-1 px-4 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200 {filterType ===
            'read'
              ? 'bg-white dark:bg-gray-800 text-primary-600 dark:text-primary-400 shadow-md scale-105'
              : 'hover:bg-white/50 dark:hover:bg-gray-800/50 text-gray-700 dark:text-gray-300'}"
            onclick={() => (filterType = "read")}
          >
            <i class="bi bi-envelope-open mr-2" aria-hidden="true"></i>
            Read
            <span class="badge-glass-success ml-2"
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
                        class="w-12 h-12 rounded-xl gradient-bg-primary flex items-center justify-center text-white font-bold text-sm"
                      >
                        {notification.avatar}
                      </div>
                    {:else}
                      <div
                        class="w-12 h-12 rounded-xl flex items-center justify-center text-xl
                        {notification.type === 'success'
                          ? 'bg-green-500/20 text-green-500'
                          : ''}
                        {notification.type === 'info'
                          ? 'bg-cyan-500/20 text-cyan-500'
                          : ''}
                        {notification.type === 'warning'
                          ? 'bg-yellow-500/20 text-yellow-500'
                          : ''}
                        {notification.type === 'error'
                          ? 'bg-red-500/20 text-red-500'
                          : ''}"
                      >
                        <i class="bi bi-{notification.icon}" aria-hidden="true"
                        ></i>
                      </div>
                    {/if}
                  </div>

                  <!-- Content -->
                  <div class="flex-1 min-w-0">
                    <div class="flex justify-between items-start gap-3 mb-2">
                      <h3
                        class="font-bold text-gray-900 dark:text-gray-100 {!notification.read
                          ? 'text-base'
                          : 'text-sm'}"
                      >
                        {tr(notification.title)}
                      </h3>
                      <span class="badge-glass-info flex-shrink-0 bounce-in">
                        <i class="bi bi-clock mr-1" aria-hidden="true"></i>
                        {formatRelativeTime(notification.time)}
                      </span>
                    </div>
                    <p class="text-sm text-gray-700 dark:text-gray-300">
                      {notification.message}
                    </p>
                  </div>

                  <!-- Actions -->
                  <div class="flex gap-1 flex-shrink-0">
                    {#if !notification.read}
                      <ModernButton
                        variant="ghost"
                        size="sm"
                        icon="check"
                        onclick={() => markAsRead(notification.id)}
                      />
                    {/if}
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      icon="trash"
                      onclick={() => deleteNotification(notification.id)}
                      class="!text-red-600 dark:!text-red-400 hover:!bg-red-100 dark:hover:!bg-red-900/30"
                    />
                  </div>
                </div>
              {/snippet}
            </ModernCard>
          </div>
        {/each}
      {:else}
        <ModernCard variant="glass" padding="large">
          {#snippet children()}
            <div class="text-center animate-fade-in">
              <i
                class="bi bi-bell-slash text-6xl opacity-20 mb-4 block"
                aria-hidden="true"
              ></i>
              <h3
                class="text-xl font-bold mb-2 text-gray-900 dark:text-gray-100"
              >
                No notifications
              </h3>
              <p class="text-gray-600 dark:text-gray-400">
                {filterType === "all"
                  ? "You have no notifications"
                  : filterType === "unread"
                    ? "You have no unread notifications"
                    : "You have no read notifications"}
              </p>
            </div>
          {/snippet}
        </ModernCard>
      {/if}
    </div>
  </div>
</PageWrapper>
