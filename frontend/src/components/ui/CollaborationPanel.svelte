<script>
  import { onMount, onDestroy } from "svelte";
  import {
    locks,
    presence,
    activity,
    conflicts,
  } from "../../stores/collaboration.js";
  import { auth } from "../../stores/auth.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast.js";

  let { filePath = null, compact = false } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let showActivity = $state(false);
  let pollingInterval = null;

  $effect(() => {
    if (filePath) {
      loadCollaborationData();
    }
  });

  const filePresence = $derived(
    $presence.filter((p) => p.file_path === filePath)
  );
  const currentLock = $derived($locks.find((l) => l.file_path === filePath));
  const isLockedByMe = $derived(
    currentLock && currentLock.locked_by === $auth.user?.username
  );
  const isLockedByOther = $derived(currentLock && !isLockedByMe);
  const editingUsers = $derived(
    filePresence.filter((p) => p.activity_type === "editing")
  );

  onMount(() => {
    if (filePath) {
      loadCollaborationData();
      startPolling();
    }
  });

  onDestroy(() => {
    stopPolling();
  });

  async function loadCollaborationData() {
    if (!filePath) return;

    try {
      await locks.load(filePath);
      await presence.load(filePath);
    } catch (err) {
      console.error("Failed to load collaboration data:", err);
    }
  }

  function startPolling() {
    stopPolling();
    pollingInterval = setInterval(() => {
      if (filePath) {
        loadCollaborationData();
      }
    }, 10000); // Every 10 seconds
  }

  function stopPolling() {
    if (pollingInterval) {
      clearInterval(pollingInterval);
      pollingInterval = null;
    }
  }

  async function acquireLock() {
    if (!filePath) return;

    try {
      await locks.acquire(filePath, "exclusive", 300);
      success(tr("fileLockedSuccessfully"));
      await loadCollaborationData();
    } catch (err) {
      errorToast(err.message || tr("failedToLockFile"));
    }
  }

  async function releaseLock() {
    if (!currentLock) return;

    try {
      await locks.release(currentLock.id);
      success(tr("fileUnlockedSuccessfully"));
      await loadCollaborationData();
    } catch (err) {
      errorToast(tr("failedToUnlockFile"));
    }
  }

  async function updateMyPresence(activityType) {
    if (!filePath) return;

    try {
      await presence.update(filePath, activityType);
    } catch (err) {
      console.error("Failed to update presence:", err);
    }
  }

  async function loadActivity() {
    if (!filePath) return;

    try {
      await activity.loadForFile(filePath);
      showActivity = true;
    } catch (err) {
      errorToast(tr("failedToLoadActivity"));
    }
  }

  function formatTime(timestamp) {
    const date = new Date(timestamp);
    return date.toLocaleString();
  }

  function getActivityIcon(activityType) {
    const icons = {
      lock_acquired: "🔒",
      lock_released: "🔓",
      edit_started: "✏️",
      edit_saved: "💾",
      conflict_detected: "⚠️",
      conflict_resolved: "✅",
    };
    return icons[activityType] || "📝";
  }
</script>

<div class="collaboration-panel" class:compact>
  <!-- Lock Status -->
  {#if currentLock}
    <div
      class="rounded-lg shadow-lg mb-3 p-4 border {isLockedByMe
        ? 'bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800'
        : 'bg-amber-50 dark:bg-amber-900/20 border-amber-200 dark:border-amber-800'}"
    >
      <div class="flex items-center gap-2">
        <i
          class="bi bi-lock-fill {isLockedByMe
            ? 'text-blue-600 dark:text-blue-400'
            : 'text-amber-600 dark:text-amber-400'}"
        ></i>
        <div class="flex-1">
          <h3
            class="font-bold text-sm {isLockedByMe
              ? 'text-blue-900 dark:text-blue-100'
              : 'text-amber-900 dark:text-amber-100'}"
          >
            {isLockedByMe
              ? "You have locked this file"
              : `Locked by ${currentLock.locked_by}`}
          </h3>
          <div
            class="text-xs {isLockedByMe
              ? 'text-blue-700 dark:text-blue-300'
              : 'text-amber-700 dark:text-amber-300'}"
          >
            Expires: {formatTime(currentLock.expires_at)}
          </div>
        </div>
        {#if isLockedByMe}
          <button
            class="px-3 py-1.5 text-sm rounded-lg hover:bg-blue-100 dark:hover:bg-blue-800 text-blue-700 dark:text-blue-200 transition-colors flex items-center gap-2"
            onclick={releaseLock}
          >
            <i class="bi bi-unlock"></i>
            Unlock
          </button>
        {/if}
      </div>
    </div>
  {:else}
    <div class="mb-3">
      <button
        class="px-3 py-1.5 text-sm rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors flex items-center gap-2"
        onclick={acquireLock}
      >
        <i class="bi bi-lock"></i>
        Lock for Editing
      </button>
    </div>
  {/if}

  <!-- Active Users -->
  {#if filePresence.length > 0}
    <div
      class="bg-gray-50 dark:bg-gray-800 rounded-lg shadow-sm mb-3 border border-gray-200 dark:border-gray-700"
    >
      <div class="p-3">
        <h4
          class="text-sm font-bold mb-2 flex items-center gap-2 text-gray-900 dark:text-white"
        >
          <i class="bi bi-people-fill"></i>
          Active Users ({filePresence.length})
        </h4>

        <div class="space-y-2">
          {#each filePresence as user}
            <div class="flex items-center gap-2 text-sm">
              <div class="flex items-center justify-center">
                <div
                  class="bg-blue-600 dark:bg-blue-500 text-white rounded-full w-8 h-8 flex items-center justify-center"
                >
                  <span class="text-xs font-medium"
                    >{user.username.charAt(0).toUpperCase()}</span
                  >
                </div>
              </div>

              <div class="flex-1">
                <div class="font-medium text-gray-900 dark:text-white">
                  {user.username}
                </div>
                <div
                  class="text-xs text-gray-500 dark:text-gray-400 flex items-center gap-1"
                >
                  {#if user.activity_type === "editing"}
                    <span
                      class="w-2 h-2 bg-green-500 dark:bg-green-400 rounded-full"
                    ></span>
                    Editing
                  {:else if user.activity_type === "viewing"}
                    <span
                      class="w-2 h-2 bg-blue-500 dark:bg-blue-400 rounded-full"
                    ></span>
                    Viewing
                  {:else}
                    <span
                      class="w-2 h-2 bg-gray-400 dark:bg-gray-500 rounded-full"
                    ></span>
                    Idle
                  {/if}
                </div>
              </div>

              <div class="text-xs text-gray-400 dark:text-gray-500">
                {formatTime(user.last_seen)}
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <!-- Activity Toggle -->
  {#if !compact}
    <button
      class="px-3 py-1.5 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 transition-colors w-full flex items-center justify-center gap-2"
      onclick={() => (showActivity ? (showActivity = false) : loadActivity())}
    >
      <i class="bi bi-clock-history"></i>
      {showActivity ? "Hide" : "Show"} Activity
    </button>

    {#if showActivity}
      <div
        class="bg-gray-50 dark:bg-gray-800 rounded-lg shadow-sm mt-3 max-h-64 overflow-y-auto border border-gray-200 dark:border-gray-700"
      >
        <div class="p-3">
          <h4 class="text-sm font-bold mb-2 text-gray-900 dark:text-white">
            Recent Activity
          </h4>

          {#if $activity.length === 0}
            <div
              class="text-center text-sm text-gray-400 dark:text-gray-500 py-4"
            >
              No activity yet
            </div>
          {:else}
            <div class="space-y-2">
              {#each $activity as item}
                <div class="flex items-start gap-2 text-xs">
                  <span class="text-lg"
                    >{getActivityIcon(item.activity_type)}</span
                  >
                  <div class="flex-1">
                    <div class="font-medium text-gray-900 dark:text-white">
                      {item.username}
                    </div>
                    <div class="text-gray-500 dark:text-gray-400">
                      {item.activity_type.replace("_", " ")}
                    </div>
                    <div class="text-gray-400 dark:text-gray-500">
                      {formatTime(item.created_at)}
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    {/if}
  {/if}

  <!-- Conflict Warning -->
  {#if $conflicts.some((c) => c.file_path === filePath && c.status === "pending")}
    <div
      class="rounded-lg shadow-lg mt-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800"
    >
      <div class="flex items-center gap-2">
        <i
          class="bi bi-exclamation-triangle-fill text-red-600 dark:text-red-400"
        ></i>
        <div class="flex-1">
          <h4 class="font-bold text-sm text-red-900 dark:text-red-100">
            Edit Conflict Detected
          </h4>
          <div class="text-xs text-red-700 dark:text-red-300">
            This file has unresolved conflicts
          </div>
        </div>
        <button
          class="px-3 py-1.5 text-sm rounded-lg bg-red-600 dark:bg-red-500 text-white hover:bg-red-700 dark:hover:bg-red-600 transition-colors"
        >
          Resolve
        </button>
      </div>
    </div>
  {/if}
</div>
}
