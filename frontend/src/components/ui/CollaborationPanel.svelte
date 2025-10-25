<script>
  import { onMount, onDestroy } from "svelte";
  import {
    locks,
    presence,
    activity,
    conflicts,
  } from "../../stores/collaboration.js";
  import { auth } from "../../stores/auth.js";
  import { success, error as errorToast } from "../../stores/toast.js";

  export let filePath = null;
  export let compact = false;

  let currentLock = null;
  let filePresence = [];
  let showActivity = false;
  let pollingInterval = null;

  $: if (filePath) {
    loadCollaborationData();
  }

  $: filePresence = $presence.filter((p) => p.file_path === filePath);
  $: currentLock = $locks.find((l) => l.file_path === filePath);
  $: isLockedByMe =
    currentLock && currentLock.locked_by === $auth.user?.username;
  $: isLockedByOther = currentLock && !isLockedByMe;
  $: editingUsers = filePresence.filter((p) => p.activity_type === "editing");

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
      success("File locked successfully");
      await loadCollaborationData();
    } catch (err) {
      errorToast(err.message || "Failed to lock file");
    }
  }

  async function releaseLock() {
    if (!currentLock) return;

    try {
      await locks.release(currentLock.id);
      success("File unlocked successfully");
      await loadCollaborationData();
    } catch (err) {
      errorToast("Failed to unlock file");
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
      errorToast("Failed to load activity");
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
      class="alert {isLockedByMe
        ? 'alert-info'
        : 'alert-warning'} shadow-lg mb-3"
    >
      <div class="flex items-center gap-2">
        <i class="bi bi-lock-fill"></i>
        <div class="flex-1">
          <h3 class="font-bold text-sm">
            {isLockedByMe
              ? "You have locked this file"
              : `Locked by ${currentLock.locked_by}`}
          </h3>
          <div class="text-xs opacity-70">
            Expires: {formatTime(currentLock.expires_at)}
          </div>
        </div>
        {#if isLockedByMe}
          <button class="btn btn-sm btn-ghost" on:click={releaseLock}>
            <i class="bi bi-unlock"></i>
            Unlock
          </button>
        {/if}
      </div>
    </div>
  {:else}
    <div class="mb-3">
      <button class="btn btn-sm btn-outline gap-2" on:click={acquireLock}>
        <i class="bi bi-lock"></i>
        Lock for Editing
      </button>
    </div>
  {/if}

  <!-- Active Users -->
  {#if filePresence.length > 0}
    <div class="card bg-base-200 shadow-sm mb-3">
      <div class="card-body p-3">
        <h4 class="card-title text-sm mb-2">
          <i class="bi bi-people-fill"></i>
          Active Users ({filePresence.length})
        </h4>

        <div class="space-y-2">
          {#each filePresence as user}
            <div class="flex items-center gap-2 text-sm">
              <div class="avatar placeholder">
                <div
                  class="bg-primary text-primary-content rounded-full w-8 h-8"
                >
                  <span class="text-xs"
                    >{user.username.charAt(0).toUpperCase()}</span
                  >
                </div>
              </div>

              <div class="flex-1">
                <div class="font-medium">{user.username}</div>
                <div class="text-xs opacity-70 flex items-center gap-1">
                  {#if user.activity_type === "editing"}
                    <span class="badge badge-success badge-xs"></span>
                    Editing
                  {:else if user.activity_type === "viewing"}
                    <span class="badge badge-info badge-xs"></span>
                    Viewing
                  {:else}
                    <span class="badge badge-ghost badge-xs"></span>
                    Idle
                  {/if}
                </div>
              </div>

              <div class="text-xs opacity-50">
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
      class="btn btn-sm btn-ghost w-full gap-2"
      on:click={() => (showActivity ? (showActivity = false) : loadActivity())}
    >
      <i class="bi bi-clock-history"></i>
      {showActivity ? "Hide" : "Show"} Activity
    </button>

    {#if showActivity}
      <div class="card bg-base-200 shadow-sm mt-3 max-h-64 overflow-y-auto">
        <div class="card-body p-3">
          <h4 class="card-title text-sm mb-2">Recent Activity</h4>

          {#if $activity.length === 0}
            <div class="text-center text-sm opacity-50 py-4">
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
                    <div class="font-medium">{item.username}</div>
                    <div class="opacity-70">
                      {item.activity_type.replace("_", " ")}
                    </div>
                    <div class="opacity-50">{formatTime(item.created_at)}</div>
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
    <div class="alert alert-error shadow-lg mt-3">
      <i class="bi bi-exclamation-triangle-fill"></i>
      <div class="flex-1">
        <h4 class="font-bold text-sm">Edit Conflict Detected</h4>
        <div class="text-xs">This file has unresolved conflicts</div>
      </div>
      <button class="btn btn-sm">Resolve</button>
    </div>
  {/if}
</div>
}

<style>
  .collaboration-panel {
    @apply rounded-lg;
  }

  .collaboration-panel.compact {
    @apply p-2;
  }

  .collaboration-panel:not(.compact) {
    @apply p-4;
  }
</style>
