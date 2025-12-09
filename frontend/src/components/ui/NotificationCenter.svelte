<script>
  import { onMount } from "svelte";
  import { wsEvents } from "@stores/websocket.js";
  import ModernCard from "./ModernCard.svelte";
  import ModernButton from "./ModernButton.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { visible = false, onClose = () => {} } = $props();

  let notifications = $state([]);
  let filter = $state("all"); // 'all', 'files', 'system', 'errors'

  onMount(() => {
    const unsubscribe = wsEvents.subscribe((events) => {
      notifications = events.map((event, index) => ({
        id: index,
        type: getEventType(event),
        message: getEventMessage(event),
        timestamp: event.timestamp || new Date().toISOString(),
        icon: getEventIcon(event),
        data: event,
      }));
    });

    return () => unsubscribe();
  });

  function getEventType(event) {
    if (event.kind) return "file";
    if (event.error) return "error";
    return "system";
  }

  function getEventIcon(event) {
    if (event.kind === "created") return "bi-file-plus";
    if (event.kind === "modified") return "bi-pencil";
    if (event.kind === "deleted") return "bi-trash";
    if (event.kind === "renamed") return "bi-file-earmark-text";
    if (event.error) return "bi-exclamation-triangle";
    return "bi-info-circle";
  }

  function getEventMessage(event) {
    if (event.kind && event.path) {
      const fileName = event.path.split("/").pop();
      switch (event.kind) {
        case "created":
          return tr("fileCreated", fileName);
        case "modified":
          return tr("fileModified", fileName);
        case "deleted":
          return tr("fileDeleted", fileName);
        case "renamed":
          return `File renamed: ${fileName}`;
        default:
          return `File event: ${fileName}`;
      }
    }
    return event.message || "System notification";
  }

  function formatTime(timestamp) {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();

    if (diff < 60000) return "Just now";
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    return date.toLocaleDateString();
  }

  function clearAll() {
    notifications = [];
  }

  let filteredNotifications = $derived(
    filter === "all"
      ? notifications
      : notifications.filter((n) => n.type === filter)
  );
</script>

{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    onclick={() => onClose()}
    role="presentation"
  >
    <div
      class="bg-white dark:bg-gray-900 rounded-xl shadow-2xl max-w-2xl w-full max-h-[80vh] overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="notification-center-title"
      tabindex="-1"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <i class="bi bi-bell text-2xl text-primary-600 dark:text-primary-400"
           aria-hidden="true"></i>
          <div>
            <h2
              id="notification-center-title"
              class="text-xl font-bold text-gray-900 dark:text-gray-100"
            >
              Notifications
            </h2>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {filteredNotifications.length} notification{filteredNotifications.length !==
              1
                ? "s"
                : ""}
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2">
          {#if notifications.length > 0}
            <ModernButton variant="ghost" size="sm" onclick={clearAll}>
              <i class="bi bi-trash mr-1" aria-hidden="true"></i>
              Clear All
            </ModernButton>
          {/if}

          <button
            onclick={() => onClose()}
            class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            aria-label="Close notifications"
          >
            <i class="bi bi-x-lg text-xl text-gray-600 dark:text-gray-400" aria-hidden="true"></i>
          </button>
        </div>
      </div>

      <!-- Filter Tabs -->
      <div
        class="flex gap-2 p-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50"
      >
        <button
          onclick={() => (filter = "all")}
          class="px-4 py-2 rounded-lg font-medium transition-colors {filter ===
          'all'
            ? 'bg-primary-600 text-white'
            : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
        >
          All
        </button>
        <button
          onclick={() => (filter = "file")}
          class="px-4 py-2 rounded-lg font-medium transition-colors {filter ===
          'file'
            ? 'bg-primary-600 text-white'
            : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
        >
          Files
        </button>
        <button
          onclick={() => (filter = "system")}
          class="px-4 py-2 rounded-lg font-medium transition-colors {filter ===
          'system'
            ? 'bg-primary-600 text-white'
            : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
        >
          System
        </button>
        <button
          onclick={() => (filter = "error")}
          class="px-4 py-2 rounded-lg font-medium transition-colors {filter ===
          'error'
            ? 'bg-red-600 text-white'
            : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
        >
          Errors
        </button>
      </div>

      <!-- Notifications List -->
      <div class="overflow-y-auto max-h-[calc(80vh-200px)] p-4 space-y-2">
        {#if filteredNotifications.length === 0}
          <div class="text-center py-12">
            <i
              class="bi bi-inbox text-6xl text-gray-300 dark:text-gray-600 mb-4"
            ></i>
            <p class="text-gray-600 dark:text-gray-400">No notifications</p>
          </div>
        {:else}
          {#each filteredNotifications as notification (notification.id)}
            <div
              class="flex items-start gap-3 p-4 rounded-lg bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            >
              <div
                class="flex-shrink-0 w-10 h-10 rounded-full bg-primary-100 dark:bg-primary-900 flex items-center justify-center"
              >
                <i
                  class="bi {notification.icon} text-primary-600 dark:text-primary-400"
                ></i>
              </div>

              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-gray-900 dark:text-gray-100">
                  {notification.message}
                </p>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  {formatTime(notification.timestamp)}
                </p>
              </div>

              {#if notification.type === "error"}
                <span
                  class="px-2 py-1 text-xs font-medium bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-300 rounded"
                >
                  Error
                </span>
              {/if}
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}
